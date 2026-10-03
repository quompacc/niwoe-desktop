//! Private atomic single-writer storage; invalid existing data is never replaced.
use super::{Snapshot, MAX_BYTES};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub fn load(path: &Path) -> Result<Snapshot, String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.len() > MAX_BYTES {
        return Err("Layoutdatei ungültig oder zu groß".into());
    }
    let mut text = String::new();
    File::open(path)
        .and_then(|f| f.take(MAX_BYTES + 1).read_to_string(&mut text))
        .map_err(|e| e.to_string())?;
    if text.len() as u64 > MAX_BYTES {
        return Err("Layoutdatei zu groß".into());
    }
    let snapshot: Snapshot = toml::from_str(&text).map_err(|e| e.to_string())?;
    snapshot.validate()?;
    Ok(snapshot)
}

pub fn save(path: &Path, snapshot: &Snapshot) -> Result<(), String> {
    snapshot.validate()?;
    if !path.is_absolute() {
        return Err("Absoluter Layoutpfad erforderlich".into());
    }
    match fs::symlink_metadata(path) {
        Ok(_) => {
            let previous = load(path)?;
            if previous.revision.checked_add(1) != Some(snapshot.revision) {
                return Err("Layoutkonflikt: inzwischen geändert; Ansicht erneut öffnen".into());
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && snapshot.revision == 1 => {}
        Err(e) => return Err(e.to_string()),
    }
    let text = toml::to_string(snapshot).map_err(|e| e.to_string())?;
    if text.len() as u64 > MAX_BYTES {
        return Err("Layoutdatei zu groß".into());
    }
    let parent = path.parent().ok_or("Layoutpfad ohne Verzeichnis")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&temporary).map_err(|e| e.to_string())?;
    let result = (|| {
        file.write_all(text.as_bytes())?;
        file.sync_all()?;
        fs::rename(&temporary, path)?;
        #[cfg(unix)]
        File::open(parent)?.sync_all()?;
        Ok::<_, std::io::Error>(())
    })();
    let _ = fs::remove_file(temporary);
    result.map_err(|e| format!("Layout konnte nicht dauerhaft gespeichert werden: {e}"))
}
