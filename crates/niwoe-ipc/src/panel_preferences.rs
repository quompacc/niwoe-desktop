use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PanelModule {
    Tray,
    Screenshot,
    Search,
    Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case")]
pub enum PanelAction {
    Get,
    Save {
        expected_revision: u64,
        modules: Vec<PanelModule>,
    },
}
