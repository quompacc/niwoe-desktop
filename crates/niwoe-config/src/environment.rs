//! Read-only compatibility for user overrides. A present new value, including
//! an empty or non-Unicode value, always wins. Internal IPC credentials and
//! inherited DRM file descriptors deliberately do not use this compatibility.
use std::{env, ffi::OsString};

pub fn var_os(name: &str) -> Option<OsString> {
    lookup(name, |key| env::var_os(key))
}

pub fn var(name: &str) -> Result<String, env::VarError> {
    var_os(name)
        .ok_or(env::VarError::NotPresent)?
        .into_string()
        .map_err(env::VarError::NotUnicode)
}

fn lookup(name: &str, get: impl Fn(&str) -> Option<OsString>) -> Option<OsString> {
    get(name).or_else(|| {
        let suffix = name.strip_prefix("NIWOE_")?;
        get(&format!("MERIDIAN_{suffix}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_override_wins_even_when_empty() {
        let values = |key: &str| match key {
            "NIWOE_THEME_DIR" => Some(OsString::new()),
            "MERIDIAN_THEME_DIR" => Some("legacy".into()),
            _ => None,
        };
        assert_eq!(lookup("NIWOE_THEME_DIR", values), Some(OsString::new()));
    }

    #[test]
    fn only_absent_new_override_falls_back() {
        let values = |key: &str| (key == "MERIDIAN_THEME_DIR").then(|| "legacy".into());
        assert_eq!(lookup("NIWOE_THEME_DIR", values), Some("legacy".into()));
        assert_eq!(lookup("XDG_THEME_DIR", values), None);
    }
}
