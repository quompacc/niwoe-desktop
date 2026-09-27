//! Versioned independent panel document; does not rewrite config.toml or foreign settings.
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PanelModule {
    Tray,
    Screenshot,
    Search,
    Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PanelPreferences {
    pub schema_version: u32,
    pub revision: u64,
    pub modules: Vec<PanelModule>,
}

impl Default for PanelPreferences {
    fn default() -> Self {
        Self {
            schema_version: 1,
            revision: 0,
            modules: vec![
                PanelModule::Tray,
                PanelModule::Screenshot,
                PanelModule::Search,
                PanelModule::Status,
            ],
        }
    }
}

impl PanelPreferences {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.modules.len() > 4
            || self
                .modules
                .iter()
                .enumerate()
                .any(|(i, m)| self.modules[..i].contains(m))
        {
            return Err("Ungültige oder unbekannte Leistenkonfiguration".into());
        }
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        match fs::symlink_metadata(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Ok(m) if !m.is_file() || m.len() > 4096 => {
                return Err("Leistenkonfiguration ist keine kleine reguläre Datei".into())
            }
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let result: Self = toml::from_str(&text).map_err(|e| e.to_string())?;
        result.validate()?;
        Ok(result)
    }

    /// Called only by the compositor writer. Success follows both fsyncs.
    pub fn save(&self, path: &Path, expected: u64) -> Result<(), String> {
        self.validate()?;
        let previous = Self::load(path)?;
        if previous.revision != expected || expected.checked_add(1) != Some(self.revision) {
            return Err("Konflikt: Leiste wurde zwischenzeitlich geändert. Entwurf prüfen und erneut speichern.".into());
        }
        let parent = path.parent().ok_or("Konfigurationsverzeichnis fehlt")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let temp = path.with_extension(format!(
            "p10-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let result = (|| -> Result<(), String> {
            let mut options = fs::OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temp).map_err(|e| e.to_string())?;
            file.write_all(toml::to_string(self).map_err(|e| e.to_string())?.as_bytes())
                .map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            if Self::load(path)? != previous {
                return Err("Konflikt: Konfiguration geändert".into());
            }
            fs::rename(&temp, path).map_err(|e| e.to_string())?;
            #[cfg(unix)]
            fs::File::open(parent)
                .and_then(|f| f.sync_all())
                .map_err(|e| format!("Dauerhafte Speicherung nicht bestätigt: {e}"))?;
            Ok(())
        })();
        let _ = fs::remove_file(temp);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn panel_save_conflict_and_invalid_data_preserve_previous_document() {
        let directory = std::env::temp_dir().join(format!(
            "niwoe-panel-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let path = directory.join("panel.toml");
        let next = PanelPreferences {
            revision: 1,
            modules: vec![PanelModule::Status, PanelModule::Search],
            ..Default::default()
        };
        next.save(&path, 0).unwrap();
        assert_eq!(PanelPreferences::load(&path).unwrap(), next);
        let bytes = fs::read(&path).unwrap();
        assert!(next.save(&path, 0).is_err());
        let mut bad = next.clone();
        bad.revision = 2;
        bad.modules.push(PanelModule::Search);
        assert!(bad.save(&path, 1).is_err());
        assert_eq!(fs::read(&path).unwrap(), bytes);
        fs::write(&path, "unknown=true").unwrap();
        assert!(next.save(&path, 0).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "unknown=true");
        fs::remove_dir_all(directory).unwrap();
    }
}
