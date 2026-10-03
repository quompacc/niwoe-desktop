//! Read-only package update status for the Settings "Updates" page.
//!
//! `apt list --upgradable` is a slow subprocess (seconds, and it can block on
//! an apt lock), so it must NEVER run on the single-threaded shell event loop
//! — doing so freezes the whole desktop. Instead it runs once in a background
//! thread and the result is cached; the page shows a placeholder until it
//! lands. The parser is unit-tested against fixture output.

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use smithay_client_toolkit::reexports::calloop::ping::Ping;

const MAX_LISTED: usize = 12;

type UpdateRows = Vec<(String, String)>;

static CACHE: OnceLock<Mutex<Option<UpdateRows>>> = OnceLock::new();
static STARTED: AtomicBool = AtomicBool::new(false);
static REFRESH_PING: OnceLock<Mutex<Ping>> = OnceLock::new();

/// Register the calloop ping the background query signals once its result
/// lands, so the event loop can redraw the Updates page without waiting for
/// the next user interaction. Called once at shell startup.
pub fn set_refresh_ping(ping: Ping) {
    let _ = REFRESH_PING.set(Mutex::new(ping));
}

fn cache() -> &'static Mutex<Option<UpdateRows>> {
    CACHE.get_or_init(|| Mutex::new(None))
}

/// Non-blocking: returns the cached rows, kicking off the background apt query
/// on first call. Never runs apt on the calling (event-loop) thread, so the
/// shell cannot freeze on it.
pub fn updates_rows() -> Vec<(String, String)> {
    if !STARTED.swap(true, Ordering::SeqCst) {
        std::thread::spawn(|| {
            let rows = query_blocking();
            if let Ok(mut guard) = cache().lock() {
                *guard = Some(rows);
            }
            if let Some(ping) = REFRESH_PING.get() {
                if let Ok(ping) = ping.lock() {
                    ping.ping();
                }
            }
        });
    }
    cache()
        .lock()
        .ok()
        .and_then(|guard| guard.clone())
        .unwrap_or_else(|| vec![("Status".to_string(), "wird ermittelt …".to_string())])
}

fn query_blocking() -> Vec<(String, String)> {
    #[cfg(target_os = "linux")]
    let provider = std::fs::read_to_string("/etc/os-release")
        .map(|value| provider_for(&value))
        .unwrap_or(Provider::Unsupported);
    #[cfg(not(target_os = "linux"))]
    let provider = Provider::Unsupported;
    if provider != Provider::Apt {
        return vec![
            (
                "Paketintegration".into(),
                if provider == Provider::Fedora {
                    "Für Fedora/RPM-Systeme noch nicht verfügbar"
                } else {
                    "Für dieses System noch nicht verfügbar"
                }
                .into(),
            ),
            (
                "Aktualisierungsstatus".into(),
                "Kann derzeit nicht ermittelt werden".into(),
            ),
        ];
    }
    match Command::new("apt")
        .env("LC_ALL", "C")
        .args(["list", "--upgradable"])
        .output()
    {
        Ok(out) if out.status.success() => {
            rows_from(&parse_upgradable(&String::from_utf8_lossy(&out.stdout)))
        }
        _ => vec![(
            "Paketstatus nicht verfügbar".into(),
            "Die lokale Paketliste konnte nicht gelesen werden.".into(),
        )],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Provider {
    Apt,
    Fedora,
    Unsupported,
}

fn provider_for(os_release: &str) -> Provider {
    let value = |key: &str| {
        os_release
            .lines()
            .filter_map(|line| line.split_once('='))
            .find(|(name, _)| *name == key)
            .map(|(_, value)| value.trim().trim_matches(['\'', '"']))
            .unwrap_or("")
    };
    match value("ID") {
        "debian" | "ubuntu" => Provider::Apt,
        "fedora" | "rhel" | "centos" => Provider::Fedora,
        "" => Provider::Unsupported,
        _ => {
            let like = value("ID_LIKE").split_whitespace().collect::<Vec<_>>();
            if like
                .iter()
                .any(|id| matches!(*id, "fedora" | "rhel" | "centos"))
            {
                Provider::Fedora
            } else if like.contains(&"debian") || like.contains(&"ubuntu") {
                Provider::Apt
            } else {
                Provider::Unsupported
            }
        }
    }
}

fn parse_upgradable(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter(|line| line.contains("upgradable from:"))
        .filter_map(|line| line.split('/').next())
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect()
}

fn rows_from(packages: &[String]) -> Vec<(String, String)> {
    if packages.is_empty() {
        return vec![(
            "Lokale Paketliste".into(),
            "Keine Aktualisierungen in dieser Liste gemeldet".into(),
        )];
    }
    let mut rows = vec![("Verfügbare Updates".to_string(), packages.len().to_string())];
    for pkg in packages.iter().take(MAX_LISTED) {
        rows.push((pkg.clone(), "aktualisierbar".to_string()));
    }
    if packages.len() > MAX_LISTED {
        rows.push((
            "…".to_string(),
            format!("+{} weitere", packages.len() - MAX_LISTED),
        ));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::{parse_upgradable, provider_for, rows_from, Provider};

    #[test]
    fn provider_uses_distribution_identity_and_never_apt_for_fedora() {
        for input in [
            "ID=fedora\nID_LIKE=debian",
            "ID=rhel",
            "ID=custom\nID_LIKE=\"fedora rhel\"",
        ] {
            assert_eq!(provider_for(input), Provider::Fedora);
        }
        for input in [
            "ID=debian",
            "ID=ubuntu",
            "ID=mint\nID_LIKE=\"ubuntu debian\"",
        ] {
            assert_eq!(provider_for(input), Provider::Apt);
        }
        for input in [
            "",
            "PRETTY_NAME=Ubuntu",
            "ID=arch",
            "ID=custom\nID_LIKE=notdebian",
        ] {
            assert_eq!(provider_for(input), Provider::Unsupported);
        }
    }

    #[test]
    fn parse_extracts_package_names() {
        let out = "Listing...\n\
            firefox/stable 120.0 amd64 [upgradable from: 119.0]\n\
            vim/stable 9.1 amd64 [upgradable from: 9.0]\n";
        assert_eq!(
            parse_upgradable(out),
            vec!["firefox".to_string(), "vim".to_string()]
        );
    }

    #[test]
    fn parse_ignores_header_and_blanks() {
        assert!(parse_upgradable("Listing...\n\n").is_empty());
    }

    #[test]
    fn rows_up_to_date_when_empty() {
        assert_eq!(
            rows_from(&[]),
            vec![(
                "Lokale Paketliste".into(),
                "Keine Aktualisierungen in dieser Liste gemeldet".into()
            )]
        );
    }

    #[test]
    fn rows_count_and_overflow() {
        let packages: Vec<String> = (0..15).map(|i| format!("pkg{i}")).collect();
        let rows = rows_from(&packages);
        assert_eq!(
            rows[0],
            ("Verfügbare Updates".to_string(), "15".to_string())
        );
        assert!(rows.iter().any(|(l, _)| l == "…"));
    }
}
