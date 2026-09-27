use crate::{PanelModule, RoomChange};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InteractionProfile {
    #[default]
    Preserve,
    Mouse,
    Keyboard,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FirstRunDraft {
    pub step: u8,
    pub profile: InteractionProfile,
    pub rooms_revision: u64,
    pub rooms: Vec<RoomChange>,
    pub panel_revision: u64,
    pub panel: Option<Vec<PanelModule>>,
    pub practiced: [bool; 3],
}

impl FirstRunDraft {
    pub fn validate(&self) -> Result<(), String> {
        if self.step > 4 || self.rooms.len() > 64 {
            return Err("Ungültiger Einführungsentwurf".into());
        }
        let mut ids = std::collections::HashSet::new();
        for change in &self.rooms {
            let RoomChange::Configure { id, .. } = change else {
                return Err("Einführung erlaubt nur ausdrückliche Raumkonfiguration".into());
            };
            if id.is_some_and(|id| id == 0 || !ids.insert(id)) {
                return Err("Raum ist mehrfach im Entwurf".into());
            }
        }
        if self.panel.as_ref().is_some_and(|modules| {
            modules.len() > 4
                || modules
                    .iter()
                    .enumerate()
                    .any(|(i, m)| modules[..i].contains(m))
        }) {
            return Err("Ungültiger Leistenentwurf".into());
        }
        Ok(())
    }
    pub fn encode(&self) -> Result<String, String> {
        self.validate()?;
        let text = serde_json::to_string(self).map_err(|e| e.to_string())?;
        if text.len() > 60 * 1024 {
            return Err("Entwurf zu groß".into());
        }
        Ok(text)
    }
    pub fn decode(text: &str) -> Result<Self, String> {
        if text.len() > 60 * 1024 {
            return Err("Entwurf zu groß".into());
        }
        let value: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        value.validate()?;
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case")]
pub enum FirstRunAction {
    Get,
    SaveDraft {
        expected_revision: u64,
        draft: FirstRunDraft,
    },
    Discard {
        expected_revision: u64,
    },
    Complete {
        expected_revision: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstRunSnapshot {
    pub revision: u64,
    pub fresh: bool,
    pub completed: bool,
    pub applying: bool,
    pub draft: Option<FirstRunDraft>,
}
