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
    read_document(path).map(|(rooms, _, _)| rooms)
}

fn read_document(path: &Path) -> Result<(Rooms, String, bool), StoreError> {
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
        if version != 1 && version != super::SCHEMA_VERSION as i64 {
            return Err(StoreError::Format(format!(
                "Nicht unterstütztes Raumschema: {version}"
            )));
        }
    }
    let mut rooms: Rooms = value
        .try_into()
        .map_err(|e: toml::de::Error| StoreError::Format(e.to_string()))?;
    let legacy = rooms.schema_version == 1;
    rooms.schema_version = super::SCHEMA_VERSION;
    rooms.validate()?;
    Ok((rooms, text, legacy))
}

/// The legacy slots live in memory: there is no legacy room document to
/// overwrite or back up. If a file already exists, even an invalid one, retain it.
pub fn load_or_initialize(path: &Path) -> Result<Rooms, StoreError> {
    match read_document(path) {
        Ok((rooms, original, legacy)) => {
            if legacy {
                migrate(path, &rooms, &original)?;
            }
            Ok(rooms)
        }
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
        Err(error) => Err(error),
    }
}

/// Caller owns the writer lock. Backup publication precedes schema replacement;
/// a retry accepts only the exact same, regular backup file.
fn migrate(path: &Path, rooms: &Rooms, original: &str) -> Result<(), StoreError> {
    let backup = path.with_extension("toml.v1.bak");
    let temporary = write_bytes(path, original)?;
    match fs::hard_link(&temporary.0, &backup) {
        Ok(()) => sync_parent(path)?,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            if !fs::symlink_metadata(&backup)?.is_file()
                || fs::read(&backup)? != original.as_bytes()
            {
                return Err(StoreError::Format(
                    "Schemasicherung stimmt nicht überein".into(),
                ));
            }
        }
        Err(e) => return Err(e.into()),
    }
    let next = write_temporary(path, rooms)?;
    if fs::read(path)? != original.as_bytes() {
        return Err(RoomError::Conflict.into());
    }
    fs::rename(&next.0, path)?;
    sync_parent(path)
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
    let text = toml::to_string(rooms).map_err(|e| StoreError::Format(e.to_string()))?;
    write_bytes(path, &text)
}

fn write_bytes(path: &Path, text: &str) -> Result<Temporary, StoreError> {
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
