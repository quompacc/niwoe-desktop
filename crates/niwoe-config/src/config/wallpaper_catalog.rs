//! Wallpaper identity is a verified package or an individual photo, never a folder.
use super::WallpaperEntry;
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

const CATALOG_LIMIT: usize = 40;
const SCAN_LIMIT: usize = 4096;
const DEPTH_LIMIT: usize = 5;

pub(super) fn scan(roots: &[PathBuf], current: Option<&str>) -> Vec<WallpaperEntry> {
    let mut entries = Vec::new();
    let mut remaining = SCAN_LIMIT;
    for root in roots {
        visit(root, DEPTH_LIMIT, &mut remaining, &mut entries);
    }
    let mut seen = BTreeSet::new();
    entries.retain(|entry| {
        seen.insert(
            std::fs::canonicalize(&entry.apply_path)
                .unwrap_or_else(|_| PathBuf::from(&entry.apply_path)),
        )
    });
    if let Some(path) = current.filter(|path| !path.is_empty()) {
        if !entries.iter().any(|entry| entry.apply_path == path) {
            entries.push(individual(Path::new(path)));
        }
    }
    entries.sort_by(|a, b| {
        a.display_name
            .cmp(&b.display_name)
            .then(a.apply_path.cmp(&b.apply_path))
    });
    // Keep an explicitly selected custom/missing file reachable beyond the cap.
    let selected = current.and_then(|path| {
        entries
            .iter()
            .find(|entry| entry.apply_path == path)
            .cloned()
    });
    entries.truncate(CATALOG_LIMIT);
    if let Some(selected) = selected {
        if !entries
            .iter()
            .any(|entry| entry.apply_path == selected.apply_path)
        {
            entries.pop();
            entries.push(selected);
            entries.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        }
    }
    entries
}

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .is_some_and(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "webp"
            )
        })
}

fn individual(path: &Path) -> WallpaperEntry {
    let path = path.to_string_lossy().into_owned();
    WallpaperEntry {
        display_name: super::wallpaper_entry_display_name(&path).replace('_', " "),
        apply_path: path.clone(),
        thumbnail_path: path,
    }
}

fn visit(dir: &Path, depth: usize, remaining: &mut usize, out: &mut Vec<WallpaperEntry>) {
    if depth == 0 || *remaining == 0 {
        return;
    }
    // KDE packages identify their image family explicitly. Their presentation
    // screenshot is not a wallpaper and must never be scanned as another entry.
    let images = dir.join("contents/images");
    if images.is_dir()
        && (dir.join("metadata.json").is_file() || dir.join("metadata.desktop").is_file())
    {
        if let Some(path) = pack_image(&images, remaining) {
            let path = path.to_string_lossy().into_owned();
            out.push(WallpaperEntry {
                display_name: super::wallpaper_dir_display_name(dir).replace('_', " "),
                apply_path: path.clone(),
                thumbnail_path: path,
            });
        }
        return;
    }
    let Ok(children) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<_> = children
        .flatten()
        .take(*remaining)
        .map(|entry| entry.path())
        .collect();
    paths.sort();
    for path in paths {
        if *remaining == 0 {
            break;
        }
        *remaining -= 1;
        if path.is_dir() {
            // Do not follow directory symlinks into unrelated user data/cycles.
            if !path.is_symlink() {
                visit(&path, depth - 1, remaining, out);
            }
        } else if path.is_file() && is_image(&path) && path.to_str().is_some() {
            out.push(individual(&path));
        }
    }
}

fn pack_image(dir: &Path, remaining: &mut usize) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    for entry in std::fs::read_dir(dir).ok()?.flatten().take(*remaining) {
        *remaining -= 1;
        let path = entry.path();
        if !path.is_file() || !is_image(&path) {
            continue;
        }
        let Some((width, height)) = resolution(&path) else {
            continue;
        };
        candidates.push((width >= height, u64::from(width) * u64::from(height), path));
    }
    candidates.sort();
    candidates.pop().map(|(_, _, path)| path)
}

fn resolution(path: &Path) -> Option<(u32, u32)> {
    let (width, height) = path.file_stem()?.to_str()?.split_once('x')?;
    let (width, height) = (width.parse().ok()?, height.parse().ok()?);
    (width > 0 && height > 0).then_some((width, height))
}

#[cfg(test)]
#[path = "wallpaper_catalog/tests.rs"]
mod tests;
