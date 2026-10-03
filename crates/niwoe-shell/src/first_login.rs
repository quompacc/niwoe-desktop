use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use tracing::{info, warn};

const HUB_MARKER: &str = "first-login-hub-shown";

fn runtime_directory() -> Option<PathBuf> {
    std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|base| base.join("niwoe"))
}

fn session_marker_name() -> String {
    let session = std::env::var("XDG_SESSION_ID")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| std::env::var("WAYLAND_DISPLAY").ok())
        .unwrap_or_else(|| "current".to_string());
    let safe_session: String = session
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect();
    format!("{HUB_MARKER}-{safe_session}")
}

fn claim_marker(path: &Path) -> std::io::Result<bool> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            if let Err(error) = file.write_all(b"shown\n") {
                drop(file);
                let _ = std::fs::remove_file(path);
                return Err(error);
            }
            Ok(true)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(error) => Err(error),
    }
}

pub(crate) fn claim_first_login_hub() -> bool {
    let Some(path) = runtime_directory().map(|directory| directory.join(session_marker_name()))
    else {
        warn!("XDG_RUNTIME_DIR unavailable; skipping per-login Hub");
        return false;
    };
    match claim_marker(&path) {
        Ok(true) => {
            info!(path = %path.display(), "opening Hub once for this NIWOE login session");
            true
        }
        Ok(false) => false,
        Err(error) => {
            warn!(path = %path.display(), %error, "unable to persist per-login Hub state");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_is_claimed_exactly_once() {
        let unique = format!(
            "niwoe-first-login-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let directory = std::env::temp_dir().join(unique);
        let marker = directory.join(HUB_MARKER);
        assert!(claim_marker(&marker).unwrap());
        assert!(!claim_marker(&marker).unwrap());
        assert_eq!(std::fs::read_to_string(&marker).unwrap(), "shown\n");
        std::fs::remove_file(marker).unwrap();
        std::fs::remove_dir(directory).unwrap();
    }
}
