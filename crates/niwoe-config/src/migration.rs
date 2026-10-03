//! One-time, non-destructive migration of explicitly owned configuration.
//! No toolkit output, global desktop settings, caches or live sockets are copied.
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

/// NIWOE files win individually; configuration documents are never merged.
/// Failed validation preserves the source and backup and publishes no target.
pub fn migrate_legacy_at(config_home: &Path) -> Result<(), String> {
    let old = config_home.join("meridian");
    let new = config_home.join("niwoe");
    if !old.try_exists().map_err(|e| e.to_string())? {
        return Ok(());
    }
    reject_symlink(&old)?;
    for name in ["config.toml", "hidden_apps.txt"] {
        migrate_file(&old, &new, Path::new(name))?;
    }
    migrate_legacy_themes_at(config_home)
}

/// User-installed themes may also live below XDG_DATA_HOME.
pub fn migrate_legacy_themes_at(base: &Path) -> Result<(), String> {
    let old = base.join("meridian");
    let new = base.join("niwoe");
    let themes = old.join("themes");
    if themes.try_exists().map_err(|e| e.to_string())? {
        reject_symlink(&old)?;
        walk_themes(&old, &new, Path::new("themes"))?;
    }
    Ok(())
}

fn reject_symlink(path: &Path) -> Result<(), String> {
    if fs::symlink_metadata(path)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_symlink()
    {
        return Err(format!("refusing migration symlink: {}", path.display()));
    }
    Ok(())
}

fn walk_themes(old: &Path, new: &Path, relative: &Path) -> Result<(), String> {
    let directory = old.join(relative);
    reject_symlink(&directory)?;
    for entry in fs::read_dir(directory).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let relative = relative.join(entry.file_name());
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_dir() {
            walk_themes(old, new, &relative)?;
        } else {
            migrate_file(old, new, &relative)?;
        }
    }
    Ok(())
}

fn migrate_file(old: &Path, new: &Path, relative: &Path) -> Result<(), String> {
    let source = old.join(relative);
    let destination = new.join(relative);
    // symlink_metadata also detects dangling destinations: never replace them.
    if fs::symlink_metadata(&destination).is_ok() {
        return Ok(());
    }
    let metadata = match fs::symlink_metadata(&source) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.to_string()),
    };
    if !metadata.is_file() || metadata.len() > MAX_FILE_BYTES {
        return Err(format!(
            "invalid legacy configuration file: {}",
            source.display()
        ));
    }
    let bytes = fs::read(&source).map_err(|e| e.to_string())?;
    let backup = old.join("niwoe-migration-backup").join(relative);
    publish_once(&backup, &bytes).map_err(|e| e.to_string())?;
    if relative == Path::new("config.toml") {
        let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
        crate::config::NiwoeConfig::parse(text)?;
    } else if relative == Path::new("hidden_apps.txt") {
        let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
        if text.contains('\0') {
            return Err("hidden apps contain NUL".into());
        }
    } else if relative
        .file_name()
        .is_some_and(|name| name == "theme.toml")
    {
        let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
        toml::from_str::<crate::ThemeConfig>(text).map_err(|e| e.to_string())?;
    }
    publish_once(&destination, &bytes).map_err(|e| e.to_string())
}

/// Publish a fully written private file without replacing any existing entry.
fn publish_once(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path.parent().expect("migration path has a parent");
    // Refuse symlink ancestors so migration cannot write outside the XDG tree.
    for ancestor in parent.ancestors() {
        if let Ok(metadata) = fs::symlink_metadata(ancestor) {
            if metadata.file_type().is_symlink() {
                return Err(io::Error::other(
                    "migration destination has symlink ancestor",
                ));
            }
        }
    }
    fs::create_dir_all(parent)?;
    let temporary: PathBuf = parent.join(format!(
        ".niwoe-migrate-{}-{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&temporary)?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        match fs::hard_link(&temporary, path) {
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            result => result,
        }
    })();
    drop(file);
    let _ = fs::remove_file(temporary);
    result
}

#[cfg(test)]
#[path = "migration_tests.rs"]
mod tests;
