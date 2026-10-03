use std::{
    io::{self, Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    time::{Duration, Instant},
};

use niwoe_ipc::{ShellCommand, ShellEvent};
use tracing::{debug, warn};

// Room storage permits 128 KiB. Allow the complete snapshot plus its JSON
// envelope/metadata; keep each event and incomplete line bounded independently.
const IPC_MAX_BUFFER_BYTES: usize = 256 * 1024;

pub struct IpcClient {
    stream: Option<UnixStream>,
    buffer: Vec<u8>,
    last_attempt: Instant,
}

impl IpcClient {
    pub(crate) fn connect() -> Self {
        let mut client = Self {
            stream: None,
            buffer: Vec::new(),
            last_attempt: Instant::now() - Duration::from_secs(5),
        };
        client.reconnect();
        client
    }

    pub(crate) fn should_reconnect(&self) -> bool {
        self.stream.is_none() && self.last_attempt.elapsed() >= Duration::from_secs(2)
    }

    pub(crate) fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    pub(crate) fn event_stream_clone(&self) -> Option<UnixStream> {
        self.stream
            .as_ref()
            .and_then(|stream| stream.try_clone().ok())
    }

    pub(crate) fn reconnect(&mut self) {
        self.last_attempt = Instant::now();
        match UnixStream::connect(niwoe_ipc::socket_path()) {
            Ok(mut stream) => {
                let Some(auth) = shell_auth_command() else {
                    warn!(
                        "niwoe IPC auth token missing; shell will not connect to compositor control socket"
                    );
                    let _ = stream.shutdown(Shutdown::Both);
                    return;
                };
                match niwoe_ipc::encode_command(&auth).and_then(|bytes| stream.write_all(&bytes)) {
                    Ok(()) => {}
                    Err(err) => {
                        warn!("failed to authenticate niwoe IPC client: {}", err);
                        let _ = stream.shutdown(Shutdown::Both);
                        return;
                    }
                }
                if let Err(err) = niwoe_ipc::encode_command(&ShellCommand::RequestRoomSnapshot)
                    .and_then(|bytes| stream.write_all(&bytes))
                {
                    warn!("failed to request room snapshot: {}", err);
                    return;
                }
                if let Err(err) = stream.set_nonblocking(true) {
                    warn!("failed to set niwoe IPC nonblocking: {}", err);
                }
                self.stream = Some(stream);
            }
            Err(err) => {
                debug!("niwoe IPC unavailable: {}", err);
            }
        }
    }

    pub(crate) fn poll(&mut self) -> Vec<ShellEvent> {
        let mut out = Vec::new();
        let Some(stream) = self.stream.as_mut() else {
            return out;
        };

        let mut tmp = [0_u8; 4096];
        loop {
            match stream.read(&mut tmp) {
                Ok(0) => {
                    self.disconnect();
                    break;
                }
                Ok(n) => {
                    self.buffer.extend_from_slice(&tmp[..n]);
                    if !drain_event_lines(&mut self.buffer, &mut out) {
                        warn!(
                            "niwoe IPC read buffer exceeded limit ({} bytes), reconnecting",
                            IPC_MAX_BUFFER_BYTES
                        );
                        self.buffer.clear();
                        self.disconnect();
                        break;
                    }
                    if out.len() >= 128 {
                        break;
                    }
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                Err(err) => {
                    warn!("niwoe IPC read failed: {}", err);
                    self.disconnect();
                    break;
                }
            }
        }

        out
    }

    pub fn send(&mut self, command: &ShellCommand) -> bool {
        if self.stream.is_none() {
            self.reconnect();
        }

        let Some(stream) = self.stream.as_mut() else {
            return false;
        };

        let Ok(bytes) = niwoe_ipc::encode_command(command) else {
            return false;
        };

        match stream.write_all(&bytes) {
            Ok(()) => true,
            Err(err) => {
                warn!("niwoe IPC write failed: {}", err);
                self.disconnect();
                false
            }
        }
    }

    fn disconnect(&mut self) {
        if let Some(stream) = self.stream.take() {
            let _ = stream.shutdown(Shutdown::Both);
        }
    }
}

// Apply the size limit to each frame, not a burst of valid snapshots. Parse
// incrementally so a 64-room refresh cannot discard adjacent control events.
fn drain_event_lines(buffer: &mut Vec<u8>, out: &mut Vec<ShellEvent>) -> bool {
    while let Some(pos) = buffer.iter().position(|byte| *byte == b'\n') {
        if pos > IPC_MAX_BUFFER_BYTES {
            return false;
        }
        let line = buffer.drain(..=pos).collect::<Vec<_>>();
        if let Some(event) = parse_event_line(String::from_utf8_lossy(&line).trim()) {
            out.push(event);
        }
    }
    buffer.len() <= IPC_MAX_BUFFER_BYTES
}

fn shell_auth_command() -> Option<ShellCommand> {
    std::env::var(niwoe_ipc::IPC_TOKEN_ENV)
        .ok()
        .filter(|token| !token.is_empty())
        .map(|token| ShellCommand::Authenticate {
            role: "shell".to_string(),
            token,
        })
}

fn parse_event_line(line: &str) -> Option<ShellEvent> {
    if line.is_empty() {
        return None;
    }
    if let Ok(event) = niwoe_ipc::decode_event(line) {
        return Some(event);
    }

    let mut parts = line.splitn(3, ' ');
    let event = match parts.next()? {
        "workspace-changed" => parts
            .next()
            .and_then(|workspace| workspace.parse().ok())
            .map(|workspace| ShellEvent::WorkspaceChanged { workspace }),
        "window-opened" => {
            let id = parts.next()?.to_string();
            let title = parts.next().unwrap_or("").to_string();
            Some(ShellEvent::WindowOpened { id, title })
        }
        "window-closed" => Some(ShellEvent::WindowClosed {
            id: parts.next()?.to_string(),
        }),
        "window-focused" => Some(ShellEvent::WindowFocused {
            id: parts.next()?.to_string(),
        }),
        "window-focus-cleared" => Some(ShellEvent::WindowFocusCleared),
        "config-reloaded" => parts
            .next()
            .and_then(|value| value.parse().ok())
            .map(|success| ShellEvent::ConfigReloaded { success }),
        _ => None,
    };
    if event.is_some() {
        warn!(
            "deprecated legacy IPC text event format received; migrate sender to JSON ShellEvent"
        );
    }
    event
}

#[cfg(test)]
mod tests {
    use std::{
        io::Write,
        os::unix::net::UnixStream,
        time::{Duration, Instant},
    };

    use niwoe_ipc::ShellEvent;

    use super::{parse_event_line, IpcClient, IPC_MAX_BUFFER_BYTES};

    #[test]
    fn valid_snapshot_burst_larger_than_buffer_limit_keeps_every_event() {
        let (mut writer, reader) = UnixStream::pair().unwrap();
        reader.set_nonblocking(true).unwrap();
        let mut client = IpcClient {
            stream: Some(reader),
            buffer: Vec::new(),
            last_attempt: Instant::now(),
        };
        let event = ShellEvent::WindowOpened {
            id: "window".into(),
            title: "x".repeat(8192),
        };
        let line = niwoe_ipc::encode_event(&event).unwrap();
        let payload = line.repeat(40);
        assert!(payload.len() > IPC_MAX_BUFFER_BYTES);
        // The producer can stream a burst larger than the socket or parser
        // buffer; polling must preserve every complete frame.
        let producer = std::thread::spawn(move || {
            let _ = writer.write_all(&payload);
        });
        std::thread::sleep(Duration::from_millis(20));
        let mut events = Vec::new();
        let deadline = Instant::now() + Duration::from_secs(2);
        while events.len() < 40 && client.stream.is_some() && Instant::now() < deadline {
            events.extend(client.poll());
            std::thread::yield_now();
        }
        producer.join().unwrap();
        assert_eq!(events.len(), 40);
        assert!(events.iter().all(|received| received == &event));
        assert!(client.buffer.is_empty());
    }

    #[test]
    fn valid_64_room_snapshot_with_unicode_metadata_fits_event_budget() {
        let event = ShellEvent::RoomSnapshot {
            snapshot: niwoe_ipc::RoomSnapshot {
                revision: 1,
                rooms: (1..=64)
                    .map(|id| niwoe_ipc::RoomEntry {
                        id,
                        workspace: id as u8,
                        name: "🗂".repeat(niwoe_config::rooms::MAX_NAME_CHARS),
                        description: "🗂".repeat(niwoe_config::rooms::MAX_DESCRIPTION_CHARS),
                        assignment: Default::default(),
                        preferences: Default::default(),
                    })
                    .collect(),
            },
        };
        let line = niwoe_ipc::encode_event(&event).unwrap();
        assert!(line.len() > 64 * 1024);
        let mut buffer = line;
        let mut received = Vec::new();
        assert!(super::drain_event_lines(&mut buffer, &mut received));
        assert_eq!(received, vec![event]);
        assert!(buffer.is_empty());
    }

    #[test]
    fn oversized_incomplete_line_disconnects_and_clears_buffer() {
        let (mut writer, reader) = UnixStream::pair().expect("stream pair");
        reader.set_nonblocking(true).expect("set nonblocking");

        let mut client = IpcClient {
            stream: Some(reader),
            buffer: Vec::new(),
            last_attempt: Instant::now() - Duration::from_secs(5),
        };

        // Write the oversized payload from a separate thread. A blocking
        // write_all from this thread would deadlock on platforms whose unix
        // socket send buffer is no larger than IPC_MAX_BUFFER_BYTES (FreeBSD's
        // net.local.stream.sendspace defaults to exactly 64 KiB), because the
        // reader is only drained by client.poll() below. Once poll() trips the
        // overflow guard and shuts the socket down, the writer sees a broken
        // pipe — expected, so the result is ignored.
        let payload = vec![b'x'; IPC_MAX_BUFFER_BYTES + 1];
        let writer_thread = std::thread::spawn(move || {
            let _ = writer.write_all(&payload);
        });

        // The kernel may dribble the payload through a small socket buffer, so a
        // single poll() can return before the buffer crosses the limit. Poll
        // until the overflow guard disconnects (bounded so a real failure still
        // ends the test).
        let mut events = Vec::new();
        for _ in 0..200 {
            events = client.poll();
            if !client.is_connected() {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }

        let _ = writer_thread.join();
        assert!(events.is_empty());
        assert!(!client.is_connected());
        assert!(client.buffer.is_empty());
    }

    #[test]
    fn json_event_parsing_remains_preferred() {
        let line = r#"{"type":"workspace-changed","workspace":4}"#;
        assert_eq!(
            parse_event_line(line),
            Some(ShellEvent::WorkspaceChanged { workspace: 4 })
        );
    }

    #[test]
    fn legacy_workspace_changed_event_still_parses() {
        assert_eq!(
            parse_event_line("workspace-changed 3"),
            Some(ShellEvent::WorkspaceChanged { workspace: 3 })
        );
    }

    #[test]
    fn legacy_window_focus_cleared_event_still_parses() {
        assert_eq!(
            parse_event_line("window-focus-cleared"),
            Some(ShellEvent::WindowFocusCleared)
        );
    }

    #[test]
    fn invalid_line_is_ignored() {
        assert_eq!(parse_event_line("not-a-real-event"), None);
    }
}
