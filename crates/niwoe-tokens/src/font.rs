//! The embedded UI typeface, owned here once so every desktop component
//! (shell, ui widgets, lock screen, polkit agent) references a single copy
//! instead of embedding its own. See the design-tokens audit (2026-06-04).
//!
//! Login and the boot splash deliberately use their own brand fonts
//! (DejaVu + Italianno, via niwoe-compass-render) and are not affected.

/// Adwaita Sans Regular — the NIWOE desktop UI typeface.
pub const ADWAITA_SANS_REGULAR: &[u8] = include_bytes!("../assets/fonts/AdwaitaSans-Regular.ttf");

/// Noto Serif Regular 2.015, SIL OFL 1.1; only for large page headings.
pub const NOTO_SERIF_REGULAR: &[u8] = include_bytes!("../assets/fonts/NotoSerif-Regular.ttf");

/// Required desktop labels/symbols. A resolved face lacking these uses the
/// embedded face consistently for metrics and painting in both shell paths.
pub const REQUIRED_UI_GLYPHS: &str = "ÄÖÜ äöü ß € – … ← → ✓";
