use std::collections::HashMap;
use tokio::process::Command;
use tracing::{debug, info, warn};
use zbus::zvariant::{Array, ObjectPath, OwnedValue, Signature, Value};

type Asv = HashMap<String, OwnedValue>;
const FILE_PICKER_ENV: &str = "NIWOE_FILE_PICKER";
const DEFAULT_FILE_PICKER: &str = "/usr/local/bin/niwoe-file-picker";

pub struct FileChooserImpl;

#[zbus::interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooserImpl {
    #[zbus(property)]
    fn version(&self) -> u32 {
        3
    }

    async fn open_file(
        &self,
        _handle: ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        title: &str,
        options: Asv,
    ) -> (u32, Asv) {
        let multiple = bool_opt(&options, "multiple");
        let directory = bool_opt(&options, "directory");
        debug!("OpenFile title={title:?} multiple={multiple} directory={directory}");

        let mut cmd = Command::new(file_picker_path());
        cmd.arg("--title").arg(title);
        if multiple {
            cmd.arg("--multiple");
        }
        if directory {
            cmd.arg("--directory");
        }
        forward_env(&mut cmd);

        match cmd.output().await {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let uris: Vec<String> = stdout
                    .lines()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(path_to_uri)
                    .collect();
                info!("OpenFile: {} file(s) selected", uris.len());
                (0, uris_asv(uris))
            }
            Ok(out) => {
                let code = out.status.code().unwrap_or(-1);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                info!("OpenFile: cancelled (exit={code}) stderr={stderr:?} stdout={stdout:?}");
                (1, Asv::new())
            }
            Err(e) => {
                warn!("OpenFile: picker error: {e}");
                (2, Asv::new())
            }
        }
    }

    async fn save_file(
        &self,
        _handle: ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        title: &str,
        options: Asv,
    ) -> (u32, Asv) {
        let current_name = str_opt(&options, "current_name").map(str::to_owned);
        debug!("SaveFile title={title:?}");

        let mut cmd = Command::new(file_picker_path());
        cmd.arg("--save").arg("--title").arg(title);
        if let Some(ref name) = current_name {
            cmd.arg("--filename").arg(name);
        }
        forward_env(&mut cmd);

        match cmd.output().await {
            Ok(out) if out.status.success() => {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_owned();
                if path.is_empty() {
                    return (1, Asv::new());
                }
                let uri = path_to_uri(&path);
                info!("SaveFile: selected {uri:?}");
                (0, uris_asv(vec![uri]))
            }
            Ok(_) => (1, Asv::new()),
            Err(e) => {
                warn!("SaveFile: picker error: {e}");
                (2, Asv::new())
            }
        }
    }

    async fn save_files(
        &self,
        _handle: ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        title: &str,
        options: Asv,
    ) -> (u32, Asv) {
        let Some(names) = save_names(&options) else {
            return (2, Asv::new());
        };
        debug!("SaveFiles title={title:?} (directory picker)");
        let mut cmd = Command::new(file_picker_path());
        cmd.arg("--directory").arg("--title").arg(title);
        forward_env(&mut cmd);

        match cmd.output().await {
            Ok(out) if out.status.success() => {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_owned();
                if path.is_empty() {
                    (1, Asv::new())
                } else {
                    let base = path_to_uri(&path);
                    let uris = names
                        .iter()
                        .map(|name| {
                            format!(
                                "{}/{}",
                                base.trim_end_matches('/'),
                                percent_encode_path(name)
                            )
                        })
                        .collect();
                    (0, uris_asv(uris))
                }
            }
            Ok(_) => (1, Asv::new()),
            Err(e) => {
                warn!("SaveFiles: picker error: {e}");
                (2, Asv::new())
            }
        }
    }
}

// Reject malformed names rather than allowing a client to escape the selected folder.
fn save_names(options: &Asv) -> Option<Vec<String>> {
    let files: Vec<Vec<u8>> = options.get("files")?.try_clone().ok()?.try_into().ok()?;
    if files.is_empty() {
        return None;
    }
    files
        .into_iter()
        .map(|bytes| {
            let name = std::str::from_utf8(bytes.strip_suffix(&[0])?).ok()?;
            if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\0']) {
                return None;
            }
            Some(name.to_owned())
        })
        .collect()
}

// Build a{sv} with a "uris" key holding an array of strings.
fn uris_asv(uris: Vec<String>) -> Asv {
    let mut m = Asv::new();
    let sig: Signature = "s".try_into().expect("valid sig");
    let mut arr = Array::new(&sig);
    for uri in uris {
        let _ = arr.append(Value::from(uri));
    }
    if let Ok(owned) = Value::Array(arr).try_to_owned() {
        m.insert("uris".into(), owned);
    }
    m
}

fn bool_opt(opts: &Asv, key: &str) -> bool {
    opts.get(key)
        .and_then(|v| bool::try_from(v).ok())
        .unwrap_or(false)
}

fn str_opt<'a>(opts: &'a Asv, key: &str) -> Option<&'a str> {
    opts.get(key).and_then(|v| <&str>::try_from(v).ok())
}

fn path_to_uri(path: &str) -> String {
    if path.starts_with("file://") {
        path.to_string()
    } else {
        format!("file://{}", percent_encode_path(path))
    }
}

fn file_picker_path() -> String {
    niwoe_config::environment::var(FILE_PICKER_ENV)
        .unwrap_or_else(|_| DEFAULT_FILE_PICKER.to_string())
}

fn percent_encode_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for byte in path.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn forward_env(cmd: &mut Command) {
    for var in ["WAYLAND_DISPLAY", "DISPLAY", "XDG_RUNTIME_DIR"] {
        if let Ok(val) = niwoe_config::environment::var(var) {
            cmd.env(var, val);
        }
    }
    // GTK3 GtkFileChooserDialog — never touches the portal.
    cmd.env("GDK_BACKEND", "wayland");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_result_has_portal_uri_array() {
        let result = uris_asv(vec![path_to_uri("/tmp/My file.txt")]);
        let uris: Vec<String> = result["uris"].try_clone().unwrap().try_into().unwrap();
        assert_eq!(uris, ["file:///tmp/My%20file.txt"]);
        assert_eq!(result.len(), 1);
    }

    fn files_options(names: &[&[u8]]) -> Asv {
        let files: Vec<Vec<u8>> = names.iter().map(|name| name.to_vec()).collect();
        HashMap::from([("files".into(), Value::from(files).try_to_owned().unwrap())])
    }

    #[test]
    fn save_names_preserve_order_and_spaces() {
        assert_eq!(
            save_names(&files_options(&[b"a b.txt\0", b"two.txt\0"])),
            Some(vec!["a b.txt".into(), "two.txt".into()])
        );
    }

    #[test]
    fn save_names_reject_invalid_paths_and_encoding() {
        for invalid in [
            b"../escape\0".as_slice(),
            b"/absolute\0",
            b"..\0",
            b".\0",
            b"\0",
            b"missing terminator",
            b"embedded\0nul\0",
            b"\xff\0",
        ] {
            assert!(save_names(&files_options(&[invalid])).is_none());
        }
        assert!(save_names(&files_options(&[])).is_none());
        assert!(save_names(&Asv::new()).is_none());
    }

    #[test]
    fn path_to_uri_preserves_existing_file_uri() {
        assert_eq!(
            path_to_uri("file:///tmp/already%20encoded"),
            "file:///tmp/already%20encoded"
        );
    }

    #[test]
    fn path_to_uri_percent_encodes_path_bytes() {
        assert_eq!(
            path_to_uri("/tmp/NIWOE Test/ä.png"),
            "file:///tmp/NIWOE%20Test/%C3%A4.png"
        );
    }
}
