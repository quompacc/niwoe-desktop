//! Single-writer store: only the compositor may publish changes. No silent
//! fallback on malformed/newer files. First publication never overwrites a file.
use super::{RoomError, Rooms};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const MAX_FILE_BYTES: u64 = 128 * 1024;
static TEMP_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
    /// Rename/publication succeeded, directory flush failed. Reload the file;
    /// do not claim the old document is still authoritative or blindly retry.
    PublishedButNotSynced(std::io::Error),
    Invalid(RoomError),
    Format(String),
}
impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Raumdatei: {e}"),
            Self::PublishedButNotSynced(e) => write!(
                f,
                "Raumdatei veröffentlicht, Verzeichnis nicht synchronisiert: {e}"
            ),
            Self::Invalid(e) => e.fmt(f),
            Self::Format(e) => write!(f, "Raumdatei: {e}"),
        }
    }
}
impl std::error::Error for StoreError {}
impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<RoomError> for StoreError {
    fn from(e: RoomError) -> Self {
        Self::Invalid(e)
    }
}

pub fn load(path: &Path) -> Result<Rooms, StoreError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.len() > MAX_FILE_BYTES {
        return Err(StoreError::Format("keine reguläre begrenzte Datei".into()));
    }
    let mut text = String::new();
    File::open(path)?
        .take(MAX_FILE_BYTES + 1)
        .read_to_string(&mut text)?;
    if text.len() as u64 > MAX_FILE_BYTES {
        return Err(StoreError::Format("Datei zu groß".into()));
    }
    // Inspect the version before parsing the current strict schema.
    let value: toml::Value =
        toml::from_str(&text).map_err(|e| StoreError::Format(e.to_string()))?;
    if let Some(version) = value
        .get("schema_version")
        .and_then(toml::Value::as_integer)
    {
        if version != super::SCHEMA_VERSION as i64 {
            return Err(StoreError::Format(format!(
                "Nicht unterstütztes Raumschema: {version}"
            )));
        }
    }
    let rooms: Rooms = value
        .try_into()
        .map_err(|e: toml::de::Error| StoreError::Format(e.to_string()))?;
    rooms.validate()?;
    Ok(rooms)
}

/// The legacy slots live in memory: there is no legacy room document to
/// overwrite or back up. If a file already exists, even an invalid one, retain it.
pub fn load_or_initialize(path: &Path) -> Result<Rooms, StoreError> {
    match load(path) {
        Err(StoreError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
            let rooms = Rooms::from_legacy_slots();
            let temporary = write_temporary(path, &rooms)?;
            match fs::hard_link(&temporary.0, path) {
                Ok(()) => {
                    sync_parent(path)?;
                    Ok(rooms)
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => load(path),
                Err(e) => Err(e.into()),
            }
        }
        result => result,
    }
}

/// Requires one responsible writer. Expected content protects against stale
/// candidates/external edits, but is not a multi-process locking protocol.
pub fn save(path: &Path, previous: &Rooms, next: &Rooms) -> Result<(), StoreError> {
    next.validate_successor(previous)?;
    if load(path)? != *previous {
        return Err(RoomError::Conflict.into());
    }
    let temporary = write_temporary(path, next)?;
    fs::rename(&temporary.0, path)?;
    sync_parent(path)?;
    Ok(())
}

struct Temporary(PathBuf);
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn write_temporary(path: &Path, rooms: &Rooms) -> Result<Temporary, StoreError> {
    rooms.validate()?;
    if !path.is_absolute() {
        return Err(StoreError::Format(
            "Absoluter Dateipfad erforderlich".into(),
        ));
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| StoreError::Format("Absoluter Dateipfad erforderlich".into()))?;
    fs::create_dir_all(parent)?;
    let text = toml::to_string(rooms).map_err(|e| StoreError::Format(e.to_string()))?;
    if text.len() as u64 > MAX_FILE_BYTES {
        return Err(StoreError::Format("Datei zu groß".into()));
    }
    let temp = parent.join(format!(
        ".rooms-{}-{}.tmp",
        std::process::id(),
        TEMP_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&temp)?;
    let temporary = Temporary(temp);
    file.write_all(text.as_bytes())?;
    file.sync_all()?;
    Ok(temporary)
}

fn sync_parent(path: &Path) -> Result<(), StoreError> {
    #[cfg(unix)]
    File::open(path.parent().expect("validated parent"))
        .and_then(|directory| directory.sync_all())
        .map_err(StoreError::PublishedButNotSynced)?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}
