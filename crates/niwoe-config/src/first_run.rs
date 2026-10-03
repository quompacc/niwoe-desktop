//! Compositor-owned introduction state, separate from productive preferences.
//! A durable journal records both sides before the first productive write.
use crate::{panel_preferences::PanelPreferences, rooms::Rooms};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

const MAX_BYTES: u64 = 2 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Journal {
    pub rooms_before: Rooms,
    pub rooms_after: Rooms,
    pub panel_before: PanelPreferences,
    pub panel_after: PanelPreferences,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FirstRun {
    pub schema_version: u32,
    pub revision: u64,
    pub fresh: bool,
    pub completed: bool,
    /// Serialized, strictly validated IPC draft; never applied on startup.
    pub draft: Option<String>,
    pub journal: Option<Journal>,
}

impl FirstRun {
    /// Call under the room writer lock, BEFORE creating rooms.toml. Unknown
    /// pre-existing files and legacy directories conservatively mean established.
    pub fn initialize(directory: &Path) -> Result<Self, String> {
        let path = directory.join("first-run.toml");
        match Self::load(&path) {
            Ok(state) => return Ok(state),
            Err(error) if fs::symlink_metadata(&path).is_ok() => return Err(error),
            Err(_) => {}
        }
        let mut fresh = true;
        for entry in fs::read_dir(directory).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.file_name() != "rooms.lock" {
                fresh = false;
            }
        }
        if let Some(parent) = directory.parent() {
            match fs::symlink_metadata(parent.join("meridian")) {
                Ok(_) => fresh = false,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.to_string()),
            }
        }
        let state = Self {
            schema_version: 1,
            revision: 0,
            fresh,
            completed: false,
            draft: None,
            journal: None,
        };
        state.publish(&path, None)?;
        Ok(state)
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
        if !metadata.is_file() || metadata.len() > MAX_BYTES {
            return Err("Einführungszustand ist keine begrenzte reguläre Datei".into());
        }
        let mut text = String::new();
        fs::File::open(path)
            .map_err(|e| e.to_string())?
            .take(MAX_BYTES + 1)
            .read_to_string(&mut text)
            .map_err(|e| e.to_string())?;
        if text.len() as u64 > MAX_BYTES {
            return Err("Einführungszustand zu groß".into());
        }
        let state: Self =
            toml::from_str(&text).map_err(|e| format!("Einführungszustand beschädigt: {e}"))?;
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err("Unbekannte Version des Einführungszustands; Datei bleibt erhalten".into());
        }
        if self.draft.as_ref().is_some_and(|d| d.len() > 128 * 1024) {
            return Err("Einführungsentwurf zu groß".into());
        }
        if let Some(journal) = &self.journal {
            journal.rooms_before.validate().map_err(|e| e.to_string())?;
            if journal.rooms_after != journal.rooms_before {
                journal
                    .rooms_after
                    .validate_successor(&journal.rooms_before)
                    .map_err(|e| e.to_string())?;
            }
            journal.panel_before.validate()?;
            journal.panel_after.validate()?;
            if journal.panel_after != journal.panel_before
                && journal.panel_before.revision.checked_add(1)
                    != Some(journal.panel_after.revision)
            {
                return Err("Ungültige Leistenrevision im Abschlussjournal".into());
            }
        }
        Ok(())
    }

    pub fn save(&self, path: &Path, previous: &Self) -> Result<(), String> {
        if previous.revision.checked_add(1) != Some(self.revision) {
            return Err("Konflikt: Einführungsrevision".into());
        }
        self.publish(path, Some(previous))
    }

    fn publish(&self, path: &Path, previous: Option<&Self>) -> Result<(), String> {
        self.validate()?;
        let text = toml::to_string(self).map_err(|e| e.to_string())?;
        if text.len() as u64 > MAX_BYTES {
            return Err("Einführungszustand zu groß".into());
        }
        let parent = path.parent().ok_or("Zustandsverzeichnis fehlt")?;
        let temp = path.with_extension(format!(
            "p11-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let result = (|| {
            let mut options = fs::OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temp).map_err(|e| e.to_string())?;
            file.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            if let Some(previous) = previous {
                if Self::load(path)? != *previous {
                    return Err("Konflikt: Einführung wurde inzwischen geändert".into());
                }
                fs::rename(&temp, path).map_err(|e| e.to_string())?;
            } else {
                fs::hard_link(&temp, path).map_err(|e| e.to_string())?;
            }
            sync_directory(parent)
        })();
        let _ = fs::remove_file(temp);
        result
    }
}

/// Recovery confirms durable publication even if the previous reply was lost.
pub fn sync_document(path: &Path) -> Result<(), String> {
    fs::File::open(path)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    sync_directory(path.parent().ok_or("Zustandsverzeichnis fehlt")?)
}

fn sync_directory(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    fs::File::open(path)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

#[cfg(test)]
mod tests;
