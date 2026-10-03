//! Static first-frame previews, matching the compositor's icon/size lookup.
//! One bounded worker on entry/size change; unchanged files share an Arc.
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use tiny_skia::Pixmap;

#[cfg(test)]
mod tests;
mod xcursor;

const FILE_LIMIT: usize = 4 * 1024 * 1024;
const METADATA_LIMIT: usize = 16 * 1024;
const INHERITANCE_LIMIT: usize = 8;

pub(crate) struct CursorRequest {
    pub themes: Vec<String>,
    pub size: u32,
    pub previous: CursorPreviews,
}

#[derive(Clone, Default)]
pub(crate) struct CursorPreviews {
    pub themes: Vec<String>,
    pub size: u32,
    pub entries: Vec<CursorPreview>,
}

#[derive(Clone)]
pub(crate) struct CursorPreview {
    pub label: String,
    pub image: Option<Arc<Pixmap>>,
    identity: Option<FileIdentity>,
}

#[derive(Clone, PartialEq, Eq)]
struct FileIdentity {
    path: PathBuf,
    length: u64,
    modified: SystemTime,
}

impl CursorPreviews {
    pub fn matches(&self, themes: &[String], size: u32) -> bool {
        self.themes == themes && self.size == size
    }

    pub fn load(themes: Vec<String>, size: u32, previous: Self) -> Self {
        Self::load_in_roots(themes, size, previous, &theme_roots())
    }

    fn load_in_roots(
        mut themes: Vec<String>,
        size: u32,
        previous: Self,
        roots: &[PathBuf],
    ) -> Self {
        themes.truncate(super::CURSOR_THEME_WIDGET_IDS.len());
        let entries = themes
            .iter()
            .map(|theme| {
                let path = ["left_ptr", "default", "arrow"]
                    .iter()
                    .find_map(|icon| find_icon(roots, theme, icon, &mut HashSet::new()));
                let identity = path.and_then(|path| {
                    let metadata = path.metadata().ok()?;
                    Some(FileIdentity {
                        path,
                        length: metadata.len(),
                        modified: metadata.modified().ok()?,
                    })
                });
                let old = previous
                    .themes
                    .iter()
                    .position(|name| name == theme)
                    .and_then(|i| previous.entries.get(i));
                let image = if previous.size == size
                    && identity.is_some()
                    && old.is_some_and(|entry| entry.identity == identity)
                {
                    old.and_then(|entry| entry.image.clone())
                } else {
                    identity.as_ref().and_then(|key| {
                        let bytes = read_bounded(&key.path, FILE_LIMIT)?;
                        xcursor::first_nearest_frame(&bytes, size).map(Arc::new)
                    })
                };
                CursorPreview {
                    label: theme_label(roots, theme),
                    image,
                    identity,
                }
            })
            .collect();
        Self {
            themes,
            size,
            entries,
        }
    }
}

pub(super) fn theme_roots() -> Vec<PathBuf> {
    if let Some(path) = std::env::var_os("XCURSOR_PATH") {
        return std::env::split_paths(&path).collect();
    }
    let home = std::env::var("HOME").unwrap_or_default();
    vec![
        PathBuf::from(home).join(".local/share/icons"),
        PathBuf::from("/usr/share/icons"),
        PathBuf::from("/usr/share/cursors/xorg-x11"),
    ]
}

fn read_bounded(path: &Path, limit: usize) -> Option<Vec<u8>> {
    let file = File::open(path).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1).read_to_end(&mut bytes).ok()?;
    (bytes.len() <= limit).then_some(bytes)
}

fn metadata(roots: &[PathBuf], theme: &str) -> Vec<String> {
    roots
        .iter()
        .filter_map(|root| {
            String::from_utf8(read_bounded(
                &root.join(theme).join("index.theme"),
                METADATA_LIMIT,
            )?)
            .ok()
        })
        .collect()
}

fn theme_label(roots: &[PathBuf], theme: &str) -> String {
    for text in metadata(roots, theme) {
        let mut in_section = false;
        let mut name = None;
        let mut localized = None;
        for line in text.lines().map(str::trim) {
            if line.starts_with('[') {
                in_section = line == "[Icon Theme]";
            }
            if !in_section {
                continue;
            }
            if let Some(value) = line.strip_prefix("Name[de]=") {
                localized = Some(value.trim());
            }
            if let Some(value) = line.strip_prefix("Name=") {
                name = Some(value.trim());
            }
        }
        if let Some(label) = localized.or(name).filter(|s| !s.is_empty()) {
            return label.to_string();
        }
    }
    theme.replace('_', " ")
}

fn find_icon(
    roots: &[PathBuf],
    theme: &str,
    icon: &str,
    visited: &mut HashSet<String>,
) -> Option<PathBuf> {
    if visited.len() >= INHERITANCE_LIMIT || !visited.insert(theme.to_string()) {
        return None;
    }
    for root in roots {
        let path = root.join(theme).join("cursors").join(icon);
        if path.is_file() {
            return Some(path);
        }
    }
    for text in metadata(roots, theme) {
        let inherited = text
            .lines()
            .filter_map(|line| {
                line.strip_prefix("Inherits")?
                    .trim_start()
                    .strip_prefix('=')?
                    .split([',', ';', ' ', '\t'])
                    .find(|name| !name.is_empty())
            })
            .next()
            .unwrap_or("default");
        if let Some(path) = find_icon(roots, inherited, icon, visited) {
            return Some(path);
        }
    }
    if theme != "default" {
        return find_icon(roots, "default", icon, visited);
    }
    None
}
