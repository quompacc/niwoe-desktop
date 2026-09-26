use std::{
    path::PathBuf,
    process::{Child, Command},
    time::Duration,
};

use niwoe_compositor::{
    backend::{drm::init_drm, winit::init_winit},
    protocols::xwayland::start_xwayland,
    state::NiwoeState,
};
use smithay::reexports::{
    calloop::{
        generic::Generic,
        timer::{TimeoutAction, Timer},
        EventLoop, Interest, Mode, PostAction,
    },
    wayland_server::Display,
};
use tracing::{info, warn};

const SHELL_RESTART_MIN_DELAY: Duration = Duration::from_secs(2);
const SHELL_RESTART_MAX_DELAY: Duration = Duration::from_secs(30);
const SHELL_STABLE_AFTER: Duration = Duration::from_secs(10);
const IPC_TOKEN_ENV: &str = "NIWOE_IPC_TOKEN";

struct ShellWatchdog {
    child: Option<Child>,
    last_start: std::time::Instant,
    restart_delay: Duration,
    shutting_down: bool,
    wayland_display: String,
    shell_binary: PathBuf,
    ipc_token: String,
}

impl ShellWatchdog {
    fn new(wayland_display: String, ipc_token: String) -> Self {
        let shell_binary = find_shell_binary();
        info!("niwoe-shell binary: {:?}", shell_binary);
        Self {
            child: None,
            last_start: std::time::Instant::now() - Duration::from_secs(5),
            restart_delay: SHELL_RESTART_MIN_DELAY,
            shutting_down: false,
            wayland_display,
            shell_binary,
            ipc_token,
        }
    }

    fn start(&mut self) {
        if self.shutting_down {
            return;
        }
        info!(
            "starting niwoe-shell: {:?} (WAYLAND_DISPLAY={})",
            self.shell_binary, self.wayland_display
        );
        match Command::new(&self.shell_binary)
            .env("WAYLAND_DISPLAY", &self.wayland_display)
            // Pass DISPLAY so X11 apps the shell launches reach XWayland. The
            // compositor sets DISPLAY in its own env once XWayland signals Ready;
            // fall back to :0 (where a fresh session's XWayland lands) if the
            // shell is spawned before that event.
            .env(
                "DISPLAY",
                std::env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string()),
            )
            .env(IPC_TOKEN_ENV, &self.ipc_token)
            .env(
                "XDG_RUNTIME_DIR",
                std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| {
                    // SAFETY: `geteuid` has no preconditions and simply reads the caller's effective uid.
                    format!("/run/user/{}", unsafe { libc::geteuid() })
                }),
            )
            .env("XDG_SESSION_TYPE", "wayland")
            .env("XDG_CURRENT_DESKTOP", "NIWOE")
            .env("XDG_SESSION_DESKTOP", "niwoe")
            .env("DESKTOP_SESSION", "niwoe")
            .spawn()
        {
            Ok(child) => {
                info!("niwoe-shell started (pid {})", child.id());
                self.child = Some(child);
                self.last_start = std::time::Instant::now();
            }
            Err(err) => {
                warn!(
                    "failed to start niwoe-shell {:?}: {}",
                    self.shell_binary, err
                );
                self.bump_restart_delay();
            }
        }
    }

    fn watch(&mut self) {
        if self.shutting_down {
            return;
        }

        if let Some(child) = self.child.as_mut() {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let uptime = self.last_start.elapsed();
                    if uptime >= SHELL_STABLE_AFTER {
                        self.restart_delay = SHELL_RESTART_MIN_DELAY;
                    } else {
                        self.bump_restart_delay();
                    }
                    info!(
                        "niwoe-shell exited: {} (uptime {:?}, next restart in {:?})",
                        status, uptime, self.restart_delay
                    );
                    self.child = None;
                }
                Ok(None) => return,
                Err(err) => {
                    warn!("niwoe-shell wait error: {}", err);
                    self.child = None;
                }
            }
        }

        if self.child.is_none() && self.last_start.elapsed() >= self.restart_delay {
            self.start();
        }
    }

    fn bump_restart_delay(&mut self) {
        self.restart_delay = next_restart_delay(self.restart_delay);
    }

    fn stop(&mut self) {
        self.shutting_down = true;
        if let Some(mut child) = self.child.take() {
            info!("stopping niwoe-shell (pid {})", child.id());
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl Drop for ShellWatchdog {
    fn drop(&mut self) {
        self.stop();
    }
}

fn find_shell_binary() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("niwoe-shell");
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    PathBuf::from("niwoe-shell")
}

fn next_restart_delay(current: Duration) -> Duration {
    let doubled = current.saturating_mul(2);
    doubled.clamp(SHELL_RESTART_MIN_DELAY, SHELL_RESTART_MAX_DELAY)
}

fn env_flag_enabled(name: &str) -> bool {
    niwoe_compositor::environment::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Session managers may discard stdout; stderr retains backend diagnostics.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let mut event_loop: EventLoop<'static, NiwoeState> = EventLoop::try_new()?;
    let display: Display<NiwoeState> = Display::new()?;
    let mut state = NiwoeState::new(&mut event_loop, display)?;

    let in_session = std::env::var("WAYLAND_DISPLAY").is_ok() || std::env::var("DISPLAY").is_ok();

    if in_session {
        info!("Detected parent display – using winit backend");
        init_winit(&mut event_loop, &mut state)?;
    } else {
        info!("No parent display – using DRM/KMS backend");
        init_drm(&mut event_loop, &mut state)?;
    }

    info!("NIWOE running on socket: {:?}", state.socket_name);
    let socket_name = state.socket_name.to_string_lossy().to_string();
    // SAFETY: process-global env mutation is intentionally performed during compositor startup.
    unsafe { std::env::set_var("WAYLAND_DISPLAY", &state.socket_name) };

    start_xwayland(&mut state);

    let shell_disabled =
        env_flag_enabled("NIWOE_DRM_DISABLE_SHELL") || env_flag_enabled("NIWOE_NO_SHELL");

    if shell_disabled {
        info!("shell auto-start disabled by env (NIWOE_DRM_DISABLE_SHELL or NIWOE_NO_SHELL)");
    }

    if let Some(listener) = state.ipc.event_listener_clone() {
        event_loop.handle().insert_source(
            Generic::new(listener, Interest::READ, Mode::Level),
            |_, _, state| {
                state.poll_ipc();
                Ok(PostAction::Continue)
            },
        )?;
    } else {
        warn!("NIWOE IPC readiness source unavailable; using timer polling only");
    }

    // Retain a low-frequency fallback for already-connected clients and for
    // platforms where listener readiness cannot be registered. New UI-runtime
    // connections are handled immediately by the source above.
    event_loop.handle().insert_source(
        Timer::from_duration(Duration::from_millis(100)),
        |_, _, state| {
            state.poll_ipc();
            TimeoutAction::ToDuration(Duration::from_millis(100))
        },
    )?;

    if !shell_disabled {
        let mut watchdog = ShellWatchdog::new(socket_name, state.ipc.auth_token().to_string());
        watchdog.start();
        event_loop.handle().insert_source(
            Timer::from_duration(Duration::from_secs(2)),
            move |_, _, _| {
                watchdog.watch();
                TimeoutAction::ToDuration(Duration::from_secs(2))
            },
        )?;
    }

    event_loop.run(None, &mut state, move |_| {})?;
    Ok(())
}
