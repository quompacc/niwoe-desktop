//! Persistent intent only; execution policies belong to P07/P09.
use serde::{Deserialize, Serialize};

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
