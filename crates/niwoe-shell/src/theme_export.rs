//! Stage the active NIWOE theme in private legacy desktop-config for GTK and
//! KDE/Qt applications — derived entirely from the central design source
//! ([`niwoe_config::ThemeConfig`] / `niwoe-tokens`), with **no third-party
//! theme dependency**.
//!
//! NIWOE serves `org.freedesktop.appearance color-scheme` over the Settings
//! portal, but most toolkits do not reconstruct a palette from it:
//!   - **KDE/Qt** (Breeze / `KColorScheme`, e.g. former defaults) read
//!     `kdeglobals` — generated here under `niwoe/toolkit`, never over KDE's file.
//!   - **GTK** apps read a *theme*. Rather than depend on a shipped theme like
//!     Adwaita-dark, NIWOE **generates its own GTK theme** from the tokens
//!     into `~/.local/share/themes/NIWOE/` (gtk-3.0 + gtk-4.0 `gtk.css` from
//!     the templates in `assets/gtk/*.css.in`). Shared GTK settings are untouched.
//!     Generated colours remain sourced from the single design pipeline.
//!
//! Files are staged at startup and theme switches. Applying them to external
//! applications requires a future scoped toolkit integration. The appearance
//! portal already publishes the theme; never change another desktop's files or
//! persistent GSettings keys to force application colours.

use std::fs;
use std::path::{Path, PathBuf};

use niwoe_config::{Color, ThemeConfig};

/// GTK theme name NIWOE installs + selects. Its CSS is generated from tokens.
const GTK_THEME_NAME: &str = "NIWOE";
const GTK3_TEMPLATE: &str = include_str!("../assets/gtk/gtk3.css.in");
const GTK4_TEMPLATE: &str = include_str!("../assets/gtk/gtk4.css.in");

/// Write private theme artifacts derived from `theme`. Never overwrites shared
/// toolkit settings or persistent GSettings. Problems are logged.
pub(crate) fn export_theme(theme: &ThemeConfig) {
    if let Some(cfg) = config_home() {
        write_toolkit_config(&cfg, theme);
    } else {
        tracing::warn!("theme_export: no config dir; skipping private toolkit files");
    }

    if let Some(data) = data_home() {
        let theme_dir = data.join("themes").join(GTK_THEME_NAME);
        write_file(&theme_dir.join("index.theme"), &index_theme(theme));
        write_file(
            &theme_dir.join("gtk-3.0").join("gtk.css"),
            &substitute_tokens(GTK3_TEMPLATE, theme),
        );
        write_file(
            &theme_dir.join("gtk-4.0").join("gtk.css"),
            &substitute_tokens(GTK4_TEMPLATE, theme),
        );
    } else {
        tracing::warn!("theme_export: no data dir; skipping generated GTK theme");
    }

    tracing::info!(
        "theme_export: private NIWOE theme staged (dark={})",
        !theme.appearance_is_light()
    );
}

fn write_toolkit_config(config_home: &Path, theme: &ThemeConfig) {
    let cfg = config_home.join("niwoe").join("toolkit");
    write_file(&cfg.join("kdeglobals"), &kdeglobals_contents(theme));
    let gtk_ini = gtk_settings_ini(theme);
    write_file(&cfg.join("gtk-3.0").join("settings.ini"), &gtk_ini);
    write_file(&cfg.join("gtk-4.0").join("settings.ini"), &gtk_ini);
    // Keep CSS beside the private toolkit settings. Never install it as the
    // user's global GTK override: that would also restyle KDE/GNOME sessions.
    write_file(
        &cfg.join("gtk-3.0").join("gtk.css"),
        &substitute_tokens(GTK3_TEMPLATE, theme),
    );
    write_file(
        &cfg.join("gtk-4.0").join("gtk.css"),
        &substitute_tokens(GTK4_TEMPLATE, theme),
    );
}

/// `~/.config` from `XDG_CONFIG_HOME` (must be absolute) or `HOME`.
fn config_home() -> Option<PathBuf> {
    abs_env("XDG_CONFIG_HOME").or_else(|| home().map(|h| h.join(".config")))
}

/// `~/.local/share` from `XDG_DATA_HOME` (must be absolute) or `HOME`.
fn data_home() -> Option<PathBuf> {
    abs_env("XDG_DATA_HOME").or_else(|| home().map(|h| h.join(".local").join("share")))
}

fn abs_env(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
}

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            tracing::warn!("theme_export: mkdir {:?}: {}", parent, e);
            return;
        }
    }
    backup_user_file(path);
    if let Err(e) = fs::write(path, contents) {
        tracing::warn!("theme_export: write {:?}: {}", path, e);
    }
}

/// Before NIWOE overwrites a pre-existing file for the first time, copy it
/// once to `<path>.bak` so hand-maintained toolkit configs are not lost
/// silently (P3-3, AUDIT_2026-08-19). Files NIWOE itself generated are
/// recognised by their header and skipped. Best-effort; never fails the write.
fn backup_user_file(path: &Path) {
    let Ok(existing) = fs::read_to_string(path) else {
        return; // nothing to back up
    };
    if is_niwoe_generated(&existing) {
        return;
    }
    let mut backup = std::ffi::OsString::from(path);
    backup.push(".bak");
    let backup = PathBuf::from(backup);
    if backup.exists() {
        return; // one-time backup already present
    }
    if let Err(e) = fs::copy(path, &backup) {
        tracing::warn!("theme_export: backup {:?} -> {:?}: {}", path, backup, e);
    } else {
        tracing::info!(
            "theme_export: backed up user file {:?} to {:?}",
            path,
            backup
        );
    }
}

/// Header heuristic for files written by this module (kdeglobals, gtk.css
/// templates, index.theme) — those never hold user edits, so they need no
/// backup before being regenerated.
fn is_niwoe_generated(contents: &str) -> bool {
    let head = contents.get(..512).unwrap_or(contents).to_ascii_lowercase();
    head.contains("generated") && head.contains("niwoe")
}

// ─── GTK theme ────────────────────────────────────────────────────────────────

/// Substitute the `@@TOKEN@@` placeholders in a GTK CSS template with the active
/// palette as `#rrggbb`. Longer names first so `@@SURFACE_ALT@@` is replaced
/// before `@@SURFACE@@` (defensive; the names are not substrings of each other).
fn substitute_tokens(template: &str, theme: &ThemeConfig) -> String {
    let c = &theme.colors;
    let d = &theme.decorations;
    let sel_fg = readable_on(c.accent, theme);
    template
        // Geometry tokens for the GTK CSD frame (border-radius / drop shadow) so
        // GTK apps render their OWN frame in the NIWOE shape — we don't draw it.
        .replace("@@RADIUS@@", &d.corner_radius.to_string())
        .replace("@@SHADOW_RADIUS@@", &d.shadow_radius.to_string())
        .replace("@@SHADOW_OFFSET@@", &d.shadow_offset_y.to_string())
        .replace("@@SHADOW_ALPHA@@", &format!("{:.2}", d.shadow_alpha))
        .replace("@@SURFACE_ALT@@", &c.surface_alt.to_hex())
        .replace("@@SURFACE@@", &c.surface.to_hex())
        .replace("@@ACCENT_ALT@@", &c.accent_alt.to_hex())
        .replace("@@ACCENT@@", &c.accent.to_hex())
        .replace("@@TEXT_DIM@@", &c.text_dim.to_hex())
        .replace("@@TEXT@@", &c.text.to_hex())
        .replace("@@BG@@", &c.background.to_hex())
        .replace("@@BORDER@@", &c.border.to_hex())
        .replace("@@ERROR@@", &c.error.to_hex())
        .replace("@@WARNING@@", &c.warning.to_hex())
        .replace("@@SUCCESS@@", &c.success.to_hex())
        .replace("@@SEL_FG@@", &sel_fg.to_hex())
}

fn index_theme(theme: &ThemeConfig) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=X-GNOME-Metatheme\n\
         Name={name}\n\
         Comment=Generated from niwoe-tokens — do not edit\n\n\
         [X-GNOME-Metatheme]\n\
         GtkTheme={name}\n\
         IconTheme={icons}\n",
        name = GTK_THEME_NAME,
        icons = app_icon_theme(theme),
    )
}

/// `~/.config/gtk-{3,4}.0/settings.ini` selecting the generated NIWOE theme.
pub(crate) fn gtk_settings_ini(theme: &ThemeConfig) -> String {
    let prefer_dark = u8::from(!theme.appearance_is_light());
    format!(
        "[Settings]\n\
         gtk-theme-name={theme_name}\n\
         gtk-application-prefer-dark-theme={prefer_dark}\n\
         gtk-icon-theme-name={icons}\n\
         gtk-font-name={font}\n\
         gtk-cursor-theme-name={cursor}\n\
         gtk-cursor-theme-size={cursor_size}\n",
        theme_name = GTK_THEME_NAME,
        icons = app_icon_theme(theme),
        font = theme.fonts.ui,
        cursor = theme.cursor.theme,
        cursor_size = theme.cursor.size,
    )
}

// ─── kdeglobals (KDE/Qt) ──────────────────────────────────────────────────────

/// `kdeglobals` colour scheme; KColorScheme reads the `[Colors:*]` groups
/// directly, so no shipped `*.colors` file is needed.
pub(crate) fn kdeglobals_contents(theme: &ThemeConfig) -> String {
    let c = &theme.colors;
    let scheme = if theme.appearance_is_light() {
        "NiwoeLight"
    } else {
        "NiwoeDark"
    };
    let sel_fg = readable_on(c.accent, theme);

    let mut s = String::new();
    s.push_str("# Generated by NIWOE (theme_export). Managed file.\n\n");
    push_color_group(&mut s, "Colors:Window", c.surface, theme);
    push_color_group(&mut s, "Colors:View", c.background, theme);
    push_color_group(&mut s, "Colors:Button", c.surface_alt, theme);
    push_color_group(&mut s, "Colors:Tooltip", c.surface_alt, theme);
    push_color_group(&mut s, "Colors:Complementary", c.background, theme);

    s.push_str("[Colors:Selection]\n");
    s.push_str(&format!("BackgroundNormal={}\n", triplet(c.accent)));
    s.push_str(&format!("ForegroundNormal={}\n", triplet(sel_fg)));
    s.push_str(&format!("ForegroundInactive={}\n", triplet(sel_fg)));
    s.push_str(&format!("DecorationFocus={}\n", triplet(c.accent)));
    s.push_str(&format!("DecorationHover={}\n", triplet(c.accent)));
    s.push('\n');

    s.push_str("[General]\n");
    s.push_str(&format!("ColorScheme={scheme}\n\n"));
    s.push_str("[Icons]\n");
    s.push_str(&format!("Theme={}\n\n", app_icon_theme(theme)));
    s.push_str("[KDE]\n");
    s.push_str("widgetStyle=Breeze\n");
    s
}

fn push_color_group(out: &mut String, group: &str, bg: Color, theme: &ThemeConfig) {
    let c = &theme.colors;
    out.push_str(&format!("[{group}]\n"));
    out.push_str(&format!("BackgroundNormal={}\n", triplet(bg)));
    out.push_str(&format!("BackgroundAlternate={}\n", triplet(c.surface_alt)));
    out.push_str(&format!("ForegroundNormal={}\n", triplet(c.text)));
    out.push_str(&format!("ForegroundInactive={}\n", triplet(c.text_dim)));
    out.push_str(&format!("ForegroundActive={}\n", triplet(c.accent)));
    out.push_str(&format!("ForegroundLink={}\n", triplet(c.accent)));
    out.push_str(&format!("ForegroundVisited={}\n", triplet(c.accent_alt)));
    out.push_str(&format!("ForegroundNegative={}\n", triplet(c.error)));
    out.push_str(&format!("ForegroundNeutral={}\n", triplet(c.warning)));
    out.push_str(&format!("ForegroundPositive={}\n", triplet(c.success)));
    out.push_str(&format!("DecorationFocus={}\n", triplet(c.accent)));
    out.push_str(&format!("DecorationHover={}\n", triplet(c.accent)));
    out.push('\n');
}

// ─── helpers ──────────────────────────────────────────────────────────────────

/// The icon theme exported to OTHER apps (GTK / Qt / gsettings). NIWOE's own
/// panel uses `theme.icons.theme` directly because it needs the recolourable
/// `-symbolic` set ("Papirus") — the Papirus-Dark/-Light variants lack those.
/// Apps want the full-colour dark/light variant instead, so map the base set to
/// it. Non-Papirus themes are exported unchanged.
fn app_icon_theme(theme: &ThemeConfig) -> String {
    let base = theme.icons.theme.trim();
    if base.eq_ignore_ascii_case("Papirus") {
        if theme.appearance_is_light() {
            "Papirus-Light".to_string()
        } else {
            "Papirus-Dark".to_string()
        }
    } else {
        base.to_string()
    }
}

/// `"r,g,b"` decimal triplet — the format KConfig expects for colour keys.
fn triplet(c: Color) -> String {
    format!("{},{},{}", c.r, c.g, c.b)
}

/// Pick whichever of the theme's text/background colour reads better on `bg`
/// (used for foregrounds painted on the accent, e.g. selection text).
fn readable_on(bg: Color, theme: &ThemeConfig) -> Color {
    let lum = 0.299 * bg.r as f32 + 0.587 * bg.g as f32 + 0.114 * bg.b as f32;
    if lum > 140.0 {
        theme.colors.background
    } else {
        theme.colors.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use niwoe_config::ThemeConfig;

    fn light_theme() -> ThemeConfig {
        let mut t = ThemeConfig::default();
        t.colors.background = Color::rgb(0xf2, 0xf2, 0xf2);
        t.colors.surface = Color::rgb(0xff, 0xff, 0xff);
        t.colors.text = Color::rgb(0x1a, 0x1a, 0x1a);
        t
    }

    #[test]
    fn triplet_formats_decimal_rgb() {
        assert_eq!(triplet(Color::rgb(0x20, 0x25, 0x2b)), "32,37,43");
        assert_eq!(triplet(Color::rgb(255, 255, 255)), "255,255,255");
    }

    #[test]
    fn readable_on_picks_contrasting_role() {
        let dark = ThemeConfig::default();
        assert_eq!(
            readable_on(Color::rgb(0xff, 0xff, 0xff), &dark),
            dark.colors.background
        );
        assert_eq!(
            readable_on(Color::rgb(0x10, 0x10, 0x10), &dark),
            dark.colors.text
        );
    }

    #[test]
    fn kdeglobals_dark_has_scheme_groups_and_breeze() {
        let s = kdeglobals_contents(&ThemeConfig::default());
        assert!(s.contains("[Colors:Window]"));
        assert!(s.contains("ColorScheme=NiwoeDark"));
        assert!(s.contains("widgetStyle=Breeze"));
        assert!(s.contains("BackgroundNormal=25,34,26")); // surface 0x20252b
    }

    #[test]
    fn kdeglobals_light_switches_scheme_name() {
        assert!(kdeglobals_contents(&light_theme()).contains("ColorScheme=NiwoeLight"));
    }

    #[test]
    fn gtk_ini_selects_niwoe_theme_and_dark_flag() {
        let s = gtk_settings_ini(&ThemeConfig::default());
        assert!(s.contains("gtk-theme-name=NIWOE"));
        assert!(s.contains("gtk-application-prefer-dark-theme=1"));
        // No third-party GTK *theme* dependency. (The Adwaita *cursor* is fine
        // and expected via gtk-cursor-theme-name.)
        assert!(!s.contains("Adwaita-dark"));
        assert!(s.contains("gtk-cursor-theme-name=Adwaita"));
    }

    #[test]
    fn gtk_ini_light_clears_dark_flag() {
        let s = gtk_settings_ini(&light_theme());
        assert!(s.contains("gtk-theme-name=NIWOE"));
        assert!(s.contains("gtk-application-prefer-dark-theme=0"));
    }

    #[test]
    fn gtk_css_substitutes_all_tokens_with_hex() {
        let css3 = substitute_tokens(GTK3_TEMPLATE, &ThemeConfig::default());
        let css4 = substitute_tokens(GTK4_TEMPLATE, &ThemeConfig::default());
        // No placeholder may survive substitution.
        assert!(!css3.contains("@@"), "unsubstituted token in gtk3 css");
        assert!(!css4.contains("@@"), "unsubstituted token in gtk4 css");
        // Tokens resolved to the dark palette hexes.
        assert!(css3.contains("#101710")); // background
        assert!(css3.contains("#d6b35b")); // accent
                                           // libadwaita named colour wired from tokens.
        assert!(css4.contains("@define-color window_bg_color #101710"));
        assert!(css4.contains("@define-color accent_bg_color #d6b35b"));
    }

    #[test]
    fn config_gtk_css_carries_libadwaita_named_colours() {
        // The per-user override written to ~/.config/gtk-4.0/gtk.css must define
        // the libadwaita named colours from the tokens (the recolour surface),
        // otherwise libadwaita apps stay on their built-in dark.
        let css4 = substitute_tokens(GTK4_TEMPLATE, &ThemeConfig::default());
        assert!(css4.contains("@define-color window_bg_color #101710"));
        assert!(css4.contains("@define-color headerbar_bg_color"));
        assert!(css4.contains("@define-color accent_bg_color #d6b35b"));
    }

    #[test]
    fn index_theme_names_niwoe() {
        let s = index_theme(&ThemeConfig::default());
        assert!(s.contains("Name=NIWOE"));
        assert!(s.contains("GtkTheme=NIWOE"));
    }

    #[test]
    fn app_icon_theme_maps_papirus_to_dark_light_variant() {
        // Panel keeps "Papirus" (symbolic); apps get the full-colour variant.
        let mut dark = ThemeConfig::default();
        dark.icons.theme = "Papirus".to_string();
        assert_eq!(app_icon_theme(&dark), "Papirus-Dark");

        let mut light = light_theme();
        light.icons.theme = "Papirus".to_string();
        assert_eq!(app_icon_theme(&light), "Papirus-Light");

        // Non-Papirus themes pass through unchanged.
        let mut custom = ThemeConfig::default();
        custom.icons.theme = "Adwaita".to_string();
        assert_eq!(app_icon_theme(&custom), "Adwaita");
    }

    #[test]
    fn toolkit_export_preserves_other_desktop_settings() {
        let dir = std::env::temp_dir().join(format!(
            "niwoe-toolkit-isolation-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let files = [
            "kdeglobals",
            "gtk-3.0/settings.ini",
            "gtk-4.0/settings.ini",
            "gtk-3.0/gtk.css",
            "gtk-4.0/gtk.css",
        ];
        for file in files {
            let path = dir.join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "existing desktop settings\n").unwrap();
        }
        for theme in [ThemeConfig::default(), light_theme()] {
            write_toolkit_config(&dir, &theme);
            for file in files {
                assert_eq!(
                    fs::read_to_string(dir.join(file)).unwrap(),
                    "existing desktop settings\n"
                );
                assert!(dir.join("niwoe/toolkit").join(file).is_file());
                assert!(!dir.join(format!("{file}.bak")).exists());
            }
        }
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn generated_header_is_recognised() {
        assert!(is_niwoe_generated(
            "# Generated by NIWOE (theme_export). Managed file.\n"
        ));
        assert!(is_niwoe_generated(
            "/*\n * NIWOE GTK3 theme — GENERATED from niwoe-tokens ...\n */\n"
        ));
        // Hand-maintained user configs must not match.
        assert!(!is_niwoe_generated("[Settings]\ngtk-theme-name=Adwaita\n"));
    }

    #[test]
    fn backup_user_file_is_one_time_and_skips_generated() {
        let dir =
            std::env::temp_dir().join(format!("niwoe-theme-export-test-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("temp dir");

        // User file -> gets a .bak on first overwrite, kept afterwards.
        let user = dir.join("settings.ini");
        fs::write(&user, "[Settings]\ngtk-theme-name=Adwaita\n").expect("write user file");
        backup_user_file(&user);
        let backup = dir.join("settings.ini.bak");
        assert_eq!(
            fs::read_to_string(&backup).expect("backup"),
            "[Settings]\ngtk-theme-name=Adwaita\n"
        );
        // Second run must not clobber the existing backup.
        fs::write(&user, "[Settings]\ngtk-theme-name=NIWOE\n").expect("rewrite");
        backup_user_file(&user);
        assert_eq!(
            fs::read_to_string(&backup).expect("backup"),
            "[Settings]\ngtk-theme-name=Adwaita\n"
        );

        // NIWOE-generated file -> no backup.
        let generated = dir.join("kdeglobals");
        fs::write(
            &generated,
            "# Generated by NIWOE (theme_export). Managed file.\n",
        )
        .expect("write generated file");
        backup_user_file(&generated);
        assert!(!dir.join("kdeglobals.bak").exists());

        let _ = fs::remove_dir_all(&dir);
    }
}
