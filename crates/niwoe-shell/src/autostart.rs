use std::path::PathBuf;

pub fn launch_autostart_apps() {
    let dirs = autostart_dirs();
    let mut entries: Vec<(String, PathBuf)> = Vec::new();

    // Collect entries: user dir overrides system dir entries of the same filename
    for dir in &dirs {
        if let Ok(read) = std::fs::read_dir(dir) {
            for entry in read.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                    continue;
                }
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                if !entries.iter().any(|(n, _)| n == &name) {
                    entries.push((name, path));
                }
            }
        }
    }

    let wayland_display = std::env::var("WAYLAND_DISPLAY").unwrap_or_default();
    let xdg_runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_default();
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    for (name, path) in entries {
        match parse_desktop_file(&path, &desktop) {
            Some(spec) if !spec.disabled => {
                tracing::info!("autostart: launching {} ({})", name, spec.exec_argv[0]);
                let result = std::process::Command::new(&spec.exec_argv[0])
                    .args(&spec.exec_argv[1..])
                    .env("WAYLAND_DISPLAY", &wayland_display)
                    .env("XDG_RUNTIME_DIR", &xdg_runtime)
                    .spawn();
                if let Err(e) = result {
                    tracing::warn!("autostart: failed to launch {}: {}", name, e);
                }
            }
            Some(_) => {
                tracing::debug!("autostart: skipping {} (hidden or desktop excluded)", name);
            }
            None => {
                tracing::debug!("autostart: skipping {} (no Exec or not Application)", name);
            }
        }
    }
}

struct DesktopSpec {
    exec_argv: Vec<String>,
    disabled: bool,
}

fn parse_desktop_file(path: &std::path::Path, desktop: &str) -> Option<DesktopSpec> {
    let content = std::fs::read_to_string(path).ok()?;
    parse_desktop_entry(&content, desktop)
}

fn parse_desktop_entry(content: &str, desktop: &str) -> Option<DesktopSpec> {
    let mut in_entry = false;
    let mut exec: Option<String> = None;
    let mut hidden = false;
    let mut entry_type: Option<String> = None;
    let mut only_show_in = None;
    let mut not_show_in = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_entry {
            continue;
        }
        if let Some(val) = line.strip_prefix("Type=") {
            entry_type = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("Exec=") {
            exec = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("Hidden=") {
            hidden = val.trim().eq_ignore_ascii_case("true");
        } else if let Some(val) = line.strip_prefix("OnlyShowIn=") {
            only_show_in = Some(val);
        } else if let Some(val) = line.strip_prefix("NotShowIn=") {
            not_show_in = Some(val);
        }
    }

    // Only launch Application entries
    if entry_type.as_deref() != Some("Application") {
        return None;
    }
    let exec = exec?;
    let argv = parse_exec(&exec);
    if argv.is_empty() {
        return None;
    }

    Some(DesktopSpec {
        exec_argv: argv,
        disabled: hidden || !desktop_allowed(desktop, only_show_in, not_show_in),
    })
}

// XDG_CURRENT_DESKTOP is colon-separated; desktop entry lists use semicolons.
// NoDisplay only controls menu visibility and must not disable autostart.
fn desktop_allowed(desktop: &str, only: Option<&str>, not: Option<&str>) -> bool {
    let matches = |list: &str| {
        desktop
            .split(':')
            .filter(|name| !name.is_empty())
            .any(|name| list.split(';').any(|entry| entry == name))
    };
    only.is_none_or(matches) && !not.is_some_and(matches)
}

/// Parse a desktop file Exec= value into argv, stripping field codes.
fn parse_exec(exec: &str) -> Vec<String> {
    // Split respecting double-quoted strings, then strip %x field codes
    let mut args: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = exec.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => in_quotes = !in_quotes,
            '\\' if in_quotes => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        args.push(current);
    }

    // Remove field codes (%f, %F, %u, %U, %d, %D, %n, %N, %i, %c, %k, %v, %m)
    args.retain(|arg| !is_field_code(arg));
    args
}

fn is_field_code(s: &str) -> bool {
    matches!(
        s,
        "%f" | "%F" | "%u" | "%U" | "%d" | "%D" | "%n" | "%N" | "%i" | "%c" | "%k" | "%v" | "%m"
    )
}

fn autostart_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    // User-level first (higher priority)
    if let Ok(config) = std::env::var("XDG_CONFIG_HOME") {
        dirs.push(PathBuf::from(config).join("autostart"));
    } else if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".config/autostart"));
    }
    // System-level
    let system_dirs = std::env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_string());
    for d in system_dirs.split(':') {
        if !d.is_empty() {
            dirs.push(PathBuf::from(d).join("autostart"));
        }
    }
    dirs
}

#[cfg(test)]
mod tests {
    use super::parse_desktop_entry;

    fn enabled(keys: &str, desktop: &str) -> bool {
        let content = format!("[Desktop Entry]\nType=Application\nExec=example --flag\n{keys}");
        let spec = parse_desktop_entry(&content, desktop).expect("application entry");
        assert_eq!(spec.exec_argv, ["example", "--flag"]);
        !spec.disabled
    }

    #[test]
    fn excludes_foreign_desktop_autostarts() {
        assert!(!enabled("OnlyShowIn=KDE;GNOME;", "NIWOE"));
        assert!(enabled("OnlyShowIn=KDE;NIWOE;", "NIWOE"));
        assert!(!enabled("NotShowIn=NIWOE;", "NIWOE"));
        assert!(enabled("NotShowIn=KDE;", "NIWOE"));
    }

    #[test]
    fn matches_exact_names_in_desktop_lists() {
        assert!(enabled("OnlyShowIn=NIWOE;", "Other:NIWOE"));
        assert!(!enabled("NotShowIn=NIWOE;", "Other:NIWOE"));
        assert!(!enabled("OnlyShowIn=NIWOE;", "NiwoeExtra"));
        assert!(!enabled("OnlyShowIn=NIWOE;", "niwoe"));
        assert!(!enabled("OnlyShowIn=;", ":"));
        assert!(!enabled("OnlyShowIn=NIWOE;", ""));
        assert!(enabled("", ""));
    }

    #[test]
    fn hidden_disables_but_no_display_does_not() {
        assert!(enabled("NoDisplay=true\n", "NIWOE"));
        assert!(!enabled("Hidden=true\nNoDisplay=false\n", "NIWOE"));
        assert!(!enabled("NoDisplay=true\nHidden=true\n", "NIWOE"));
    }

    #[test]
    fn ignores_keys_in_other_groups() {
        assert!(enabled(
            "[Desktop Action other]\nOnlyShowIn=KDE;\nHidden=true",
            "NIWOE"
        ));
    }
}
