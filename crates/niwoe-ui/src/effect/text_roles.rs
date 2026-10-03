//! Font-metric layout shared by result rows and two-line form controls.
//! Computed only while painting an invalidated surface; no timer or cache.
use niwoe_tokens::{Color, Spacing, Typography};
use tiny_skia::PixmapMut;

use super::{paint_text, truncate_to_fit, ui_line_metrics};
use crate::Rect;

#[derive(Debug, Clone, Copy)]
pub struct TextPairLayout {
    pub primary_baseline: i32,
    pub secondary_baseline: i32,
    pub height: i32,
}

/// Keep the complete font line boxes apart, including accents and descenders.
pub fn text_pair_layout(area: Rect) -> TextPairLayout {
    pair_layout(area, Typography::DEFAULT.body_size as f32)
}

fn pair_layout(area: Rect, primary_size: f32) -> TextPairLayout {
    let typography = Typography::DEFAULT;
    let (pa, pd) = ui_line_metrics(primary_size);
    let (sa, sd) = ui_line_metrics(typography.caption_size as f32);
    // Round each side outwards; rounding the total alone can lose the gap
    // when the baseline itself is rounded upwards.
    let primary_height = pa.ceil() as i32 + (-pd).ceil() as i32;
    let secondary_height = sa.ceil() as i32 + (-sd).ceil() as i32;
    let height = primary_height + Spacing::DEFAULT.xs + secondary_height;
    let top = area.y + (area.height - height).max(0) / 2;
    TextPairLayout {
        primary_baseline: top + pa.ceil() as i32,
        secondary_baseline: top + primary_height + Spacing::DEFAULT.xs + sa.ceil() as i32,
        height,
    }
}

pub fn paint_text_pair(
    canvas: &mut PixmapMut<'_>,
    area: Rect,
    primary: &str,
    secondary: &str,
    primary_color: Color,
    secondary_color: Color,
) {
    paint_pair(
        canvas,
        area,
        primary,
        secondary,
        primary_color,
        secondary_color,
        Typography::DEFAULT.body_size as f32,
    );
}

/// Room/card titles share the same metric spacing as ordinary two-line rows.
pub fn paint_title_pair(
    canvas: &mut PixmapMut<'_>,
    area: Rect,
    primary: &str,
    secondary: &str,
    primary_color: Color,
    secondary_color: Color,
) {
    paint_pair(
        canvas,
        area,
        primary,
        secondary,
        primary_color,
        secondary_color,
        Typography::DEFAULT.title_size as f32,
    );
}

fn paint_pair(
    canvas: &mut PixmapMut<'_>,
    area: Rect,
    primary: &str,
    secondary: &str,
    primary_color: Color,
    secondary_color: Color,
    primary_size: f32,
) {
    let layout = pair_layout(area, primary_size);
    for (text, baseline, size, color) in [
        (
            primary,
            layout.primary_baseline,
            primary_size,
            primary_color,
        ),
        (
            secondary,
            layout.secondary_baseline,
            Typography::DEFAULT.caption_size as f32,
            secondary_color,
        ),
    ] {
        paint_text(
            canvas,
            &truncate_to_fit(text, area.width, size),
            area.x,
            baseline,
            size,
            color,
        );
    }
}

/// All native list/form focus rings use the same inset, width and color role.
pub fn paint_focus(canvas: &mut PixmapMut<'_>, area: Rect, color: Color, radius: i32) {
    let inset = niwoe_tokens::Controls::FOCUS_INSET;
    let area = Rect {
        x: area.x + inset,
        y: area.y + inset,
        width: area.width - inset * 2,
        height: area.height - inset * 2,
    };
    if let Some(path) = super::rounded_rect_path(area, radius) {
        super::paint_border(
            canvas,
            &path,
            color,
            niwoe_tokens::Controls::FOCUS_WIDTH as f32,
        );
    }
}
