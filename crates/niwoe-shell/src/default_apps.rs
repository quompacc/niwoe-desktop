//! Default application management — the data layer behind the Settings
//! "Standard-Apps" page.
//!
//! The Linux convention for "what app opens what MIME type" lives in
//! `~/.config/mimeapps.list` (user override) and `/usr/share/applications/`
//! (system-wide). The supported way to read/write it is the `xdg-mime` CLI:
//!
//!     xdg-mime query default <mime>          -> "<app>.desktop\n"
//!     xdg-mime default       <app> <mime>... -> updates mimeapps.list
//!
//! We wrap that with strong-typed categories (web browser, email, etc.) so
//! the settings UI doesn't have to think about individual MIME strings, plus
//! a sensible-defaults heuristic that picks an installed app for empty
//! categories.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const XDG_DATA_DIRS_DEFAULT: &str = "/usr/local/share:/usr/share";
pub(crate) const MAX_APPS_PER_CATEGORY: usize = 24;

/// One of the user-facing default-app categories the settings page exposes.
/// Each category is a cluster of related MIME types plus a single
/// "representative" MIME used to query the current default; setting a default
/// stamps the chosen app across every MIME in the cluster so e.g. picking an
/// image viewer applies to `.png`, `.jpg`, `.webp` etc. in one go.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultAppCategory {
    WebBrowser,
    Email,
    FileManager,
    TextEditor,
    ImageViewer,
    VideoPlayer,
    AudioPlayer,
    Pdf,
    Archive,
}

impl DefaultAppCategory {
    pub const ALL: &'static [DefaultAppCategory] = &[
        Self::WebBrowser,
        Self::Email,
        Self::FileManager,
        Self::TextEditor,
        Self::ImageViewer,
        Self::VideoPlayer,
        Self::AudioPlayer,
        Self::Pdf,
        Self::Archive,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::WebBrowser => "Web-Browser",
            Self::Email => "E-Mail",
            Self::FileManager => "Dateimanager",
            Self::TextEditor => "Texteditor",
            Self::ImageViewer => "Bildbetrachter",
            Self::VideoPlayer => "Videoplayer",
            Self::AudioPlayer => "Audioplayer",
            Self::Pdf => "PDF",
            Self::Archive => "Archive",
        }
    }

    /// The single MIME used to query the current default for this category.
    /// (Setting writes to every MIME in `all_mimes`.)
    pub fn representative_mime(self) -> &'static str {
        match self {
            Self::WebBrowser => "x-scheme-handler/https",
            Self::Email => "x-scheme-handler/mailto",
            Self::FileManager => "inode/directory",
            Self::TextEditor => "text/plain",
            Self::ImageViewer => "image/jpeg",
            Self::VideoPlayer => "video/mp4",
            Self::AudioPlayer => "audio/mpeg",
            Self::Pdf => "application/pdf",
            Self::Archive => "application/zip",
        }
    }

    /// Every MIME type stamped when the user picks an app for this category.
    /// Picking an image viewer should cover png/jpeg/gif/webp/etc. in one go;
    /// the user shouldn't have to set each one separately.
    pub fn all_mimes(self) -> &'static [&'static str] {
        match self {
            Self::WebBrowser => &[
                "x-scheme-handler/http",
                "x-scheme-handler/https",
                "x-scheme-handler/about",
                "x-scheme-handler/unknown",
                "text/html",
                "application/xhtml+xml",
            ],
            Self::Email => &["x-scheme-handler/mailto"],
            Self::FileManager => &["inode/directory"],
            Self::TextEditor => &["text/plain", "text/markdown", "text/x-readme", "text/x-log"],
            Self::ImageViewer => &[
                "image/jpeg",
                "image/png",
                "image/gif",
                "image/webp",
                "image/svg+xml",
                "image/bmp",
                "image/tiff",
                "image/heif",
                "image/heic",
            ],
            Self::VideoPlayer => &[
                "video/mp4",
                "video/x-matroska",
                "video/webm",
                "video/quicktime",
                "video/x-msvideo",
                "video/mpeg",
                "video/ogg",
            ],
            Self::AudioPlayer => &[
                "audio/mpeg",
                "audio/flac",
                "audio/ogg",
                "audio/wav",
                "audio/x-wav",
                "audio/aac",
                "audio/mp4",
                "audio/x-vorbis+ogg",
                "audio/opus",
            ],
            Self::Pdf => &["application/pdf"],
            Self::Archive => &[
                "application/zip",
                "application/x-tar",
                "application/x-rar",
                "application/vnd.rar",
                "application/x-7z-compressed",
                "application/gzip",
                "application/x-bzip2",
                "application/x-xz",
            ],
        }
    }

    /// Heuristic ranking of known-good apps per category, by preference.
    /// First installed match wins when sensible-defaults runs.
    pub fn preferred_desktop_ids(self) -> &'static [&'static str] {
        match self {
            Self::WebBrowser => &[
                "firefox.desktop",
                "org.mozilla.firefox.desktop",
                "firefox-esr.desktop",
                "chromium.desktop",
                "google-chrome.desktop",
                "brave-browser.desktop",
                "vivaldi-stable.desktop",
                "org.gnome.Epiphany.desktop",
                "org.kde.falkon.desktop",
            ],
            Self::Email => &[
                "thunderbird.desktop",
                "org.mozilla.Thunderbird.desktop",
                "org.kde.kmail2.desktop",
                "org.gnome.Geary.desktop",
                "evolution.desktop",
            ],
            Self::FileManager => &[
                "org.gnome.Nautilus.desktop",
                "org.kde.dolphin.desktop",
                "thunar.desktop",
                "pcmanfm.desktop",
                "nemo.desktop",
            ],
            Self::TextEditor => &[
                "org.gnome.TextEditor.desktop",
                "org.gnome.gedit.desktop",
                "code.desktop",
                "code-oss.desktop",
                "org.kde.kate.desktop",
                "org.kde.kwrite.desktop",
                "mousepad.desktop",
                "xed.desktop",
            ],
            Self::ImageViewer => &[
                "org.gnome.Loupe.desktop",
                "org.gnome.eog.desktop",
                "org.kde.gwenview.desktop",
                "feh.desktop",
                "qimgv.desktop",
            ],
            Self::VideoPlayer => &[
                "mpv.desktop",
                "io.mpv.Mpv.desktop",
                "org.videolan.VLC.desktop",
                "vlc.desktop",
                "org.gnome.Totem.desktop",
                "org.kde.haruna.desktop",
                "org.kde.dragonplayer.desktop",
            ],
            Self::AudioPlayer => &[
                "io.bassi.Amberol.desktop",
                "org.gnome.Rhythmbox3.desktop",
                "rhythmbox.desktop",
                "org.kde.elisa.desktop",
                "audacious.desktop",
                "org.videolan.VLC.desktop",
                "vlc.desktop",
                "mpv.desktop",
            ],
            Self::Pdf => &[
                "org.gnome.Evince.desktop",
                "org.kde.okular.desktop",
                "xreader.desktop",
                "qpdfview.desktop",
                "atril.desktop",
            ],
            Self::Archive => &[
                "org.gnome.FileRoller.desktop",
                "file-roller.desktop",
                "org.kde.ark.desktop",
                "xarchiver.desktop",
            ],
        }
    }
}

/// A `.desktop` file that declares it handles at least one MIME type. The
/// settings UI shows a dropdown of these per category; the user picks one
/// and `set_default_for_mimes` stamps it onto every MIME of the category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MimeAppCandidate {
    /// e.g. `firefox.desktop`. Identifies the app inside `xdg-mime default`.
    pub desktop_id: String,
    /// Human-readable name (`Name=` field).
    pub name: String,
    /// `Icon=` value, when present.
    pub icon: Option<String>,
    /// All MIME types this app advertises (`MimeType=...`).
    pub mime_types: Vec<String>,
}

/// Snapshot of every desktop entry on the system that handles at least one
/// MIME type, indexed for quick "apps for MIME" lookup. Built once and reused
/// across UI ticks; `xdg-mime` reads + writes still go to disk per call.
#[derive(Debug, Clone, Default)]
pub struct MimeAppIndex {
    apps: Vec<MimeAppCandidate>,
    by_mime: HashMap<String, Vec<usize>>,
    previews: HashMap<String, std::sync::Arc<tiny_skia::Pixmap>>,
}

impl MimeAppIndex {
    pub fn load_system() -> Self {
        Self::load_from_dirs(desktop_app_dirs())
    }

    pub(crate) fn load_from_dirs(dirs: Vec<PathBuf>) -> Self {
        let mut apps = Vec::new();
        let mut by_mime: HashMap<String, Vec<usize>> = HashMap::new();
        for dir in dirs {
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.extension().map(|e| e == "desktop").unwrap_or(false) {
                    continue;
                }
                let Some((app, selectable)) = entry::parse_mime_entry(&path) else {
                    continue;
                };
                if app.mime_types.is_empty() {
                    continue;
                }
                // Dedupe by desktop_id (user override > /usr/local > /usr).
                if apps
                    .iter()
                    .any(|a: &MimeAppCandidate| a.desktop_id == app.desktop_id)
                {
                    continue;
                }
                let idx = apps.len();
                if selectable {
                    for mime in &app.mime_types {
                        by_mime.entry(mime.clone()).or_default().push(idx);
                    }
                }
                apps.push(app);
            }
        }
        Self {
            apps,
            by_mime,
            previews: HashMap::new(),
        }
    }

    /// All candidate apps that advertise `mime` in their `MimeType=` line,
    /// sorted by display name (alphabetical, case-insensitive).
    pub fn apps_for_mime(&self, mime: &str) -> Vec<&MimeAppCandidate> {
        let mut out: Vec<&MimeAppCandidate> = self
            .by_mime
            .get(mime)
            .map(|idxs| idxs.iter().filter_map(|i| self.apps.get(*i)).collect())
            .unwrap_or_default();
        out.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
                .then_with(|| a.desktop_id.cmp(&b.desktop_id))
        });
        out
    }

    /// Lookup the displayed app candidate behind a `desktop_id`.
    pub fn lookup(&self, desktop_id: &str) -> Option<&MimeAppCandidate> {
        self.apps.iter().find(|a| a.desktop_id == desktop_id)
    }

    pub(crate) fn app_icon(
        &self,
        app: &MimeAppCandidate,
    ) -> Option<std::sync::Arc<tiny_skia::Pixmap>> {
        app.icon
            .as_ref()
            .and_then(|name| self.previews.get(name))
            .cloned()
    }
}

/// Pick a file-manager command line for `inode/directory`. Probes a
/// curated list of known managers in `/usr/bin`; falls back to `gio open`
/// (which honours mimeapps.list via GIO). Returns `(program, args)` ready
/// for `ShellCommand::LaunchApp`; the path argument is the user's home
/// directory.
pub fn pick_file_manager() -> (String, Vec<String>) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    // pkg installs to /usr/local/bin on FreeBSD; Linux distros use /usr/bin.
    // Probe both so the file manager is found regardless of prefix.
    let bindirs = ["/usr/local/bin", "/usr/bin"];
    // GTK file managers only (the desktop is GTK-based); Nemo is the shipped
    // default so it leads. No KDE/Dolphin — that pulls the whole KIO stack.
    for fm in ["nemo", "nautilus", "thunar", "pcmanfm", "caja"] {
        if bindirs
            .iter()
            .any(|dir| std::path::Path::new(dir).join(fm).exists())
        {
            return (fm.to_string(), vec![home]);
        }
    }
    ("gio".to_string(), vec!["open".to_string(), home])
}

/// Existing quick-start lookup; Settings reads use the checked worker backend.
pub fn query_default(mime: &str) -> Option<String> {
    let out = std::process::Command::new("xdg-mime")
        .env("LC_ALL", "C")
        .args(["query", "default", mime])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let id = String::from_utf8(out.stdout).ok()?.trim().to_string();
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

fn desktop_app_dirs() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(home) = std::env::var_os("HOME") {
        let mut user = PathBuf::from(home);
        user.push(".local/share/applications");
        out.push(user);
    }
    // Treat a present-but-EMPTY XDG_DATA_DIRS like an unset one: otherwise the
    // system applications/ dirs are skipped, the app index is nearly empty, and
    // every default renders as a raw `foo.desktop` string ("not recognized").
    let data_dirs = std::env::var("XDG_DATA_DIRS")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| XDG_DATA_DIRS_DEFAULT.to_string());
    for dir in data_dirs.split(':').filter(|s| !s.is_empty()) {
        let mut path = PathBuf::from(dir);
        path.push("applications");
        out.push(path);
    }
    out
}

mod entry;
#[cfg(test)]
use entry::parse_mime_candidate;
mod command;
mod previews;
pub(crate) mod refresh;
#[cfg(test)]
mod tests;
