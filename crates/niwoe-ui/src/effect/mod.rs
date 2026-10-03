//! Visual effect building blocks.
//!
//! `rounded_rect_path` is a setup/resize helper and may allocate internally.
//! Cache the resulting `Path` if it is reused across frames.
//! `paint_border` is render-loop code and must remain allocation-free.
//! `paint_text` rasterizes glyphs on demand and allocates per glyph — known
//! trade-off, see text.rs.

mod border;
mod dominant_color;
mod fill;
mod heading;
mod image_contain;
mod image_cover;
pub use image_contain::paint_image_contain;
mod metro_surface;
mod radius;
mod symbol;
mod text;
mod text_roles;

pub use border::paint_border;
pub use dominant_color::dominant_color;
pub use fill::paint_fill;
pub use heading::paint_display_heading;
pub use image_cover::{image_cover_transform, paint_image_cover};
pub use metro_surface::paint_metro_surface;
pub use radius::rounded_rect_path;
pub use symbol::{symbol_icon, Symbol};
pub use text::{
    blend_text_sample, clear_ui_font, font_supports_text, measure_text, paint_text, set_ui_font,
    truncate_to_fit, ui_font, ui_line_metrics, TextInk,
};
pub use text_roles::{
    paint_focus, paint_text_pair, paint_title_pair, text_pair_layout, TextPairLayout,
};
