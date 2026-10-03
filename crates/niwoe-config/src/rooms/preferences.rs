//! Persistent intent only; execution policies belong to P07/P09.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RoomPreferences {
    /// Freedesktop icon name, never a filesystem path or executable.
    pub icon: Option<String>,
    /// Ordered exact identities. Missing/uninstalled apps remain valid references.
    pub apps: Vec<AppReference>,
    pub layout: RoomLayout,
    pub restore: RoomRestore,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "kebab-case",
    deny_unknown_fields
)]
pub enum AppReference {
    Native(String),
    Xwayland(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RoomLayout {
    #[default]
    Tiling,
    Floating,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RoomRestore {
    #[default]
    Disabled,
    LayoutOnly,
    RelaunchApps,
}

impl RoomPreferences {
    pub fn validate(&self) -> Result<(), super::RoomError> {
        let invalid = || super::RoomError::Invalid("Ungültige Raumpräferenzen".into());
        if self.icon.as_ref().is_some_and(|icon| {
            icon.is_empty()
                || icon.len() > 128
                || !icon
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
                || icon.starts_with('.')
        }) {
            return Err(invalid());
        }
        if self.apps.len() > 64 {
            return Err(invalid());
        }
        let mut seen = BTreeSet::new();
        for app in &self.apps {
            let id = match app {
                AppReference::Native(id) | AppReference::Xwayland(id) => id,
            };
            if id.is_empty()
                || id.len() > 255
                || id.trim() != id
                || id.chars().any(char::is_control)
                || !seen.insert(app)
            {
                return Err(invalid());
            }
        }
        Ok(())
    }
}
