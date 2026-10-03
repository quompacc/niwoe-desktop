//! Bounded page controls, with one geometry shared by drawing and hit testing.
use super::*;

pub(crate) fn rect(width: u32, height: u32, back: bool) -> Rect {
    let rail = right_rail_x(width) - C.outer_pad;
    Rect {
        x: rail
            - if back {
                C.filter_all_width * 2 + C.card_gap
            } else {
                C.filter_all_width
            },
        y: height as i32 - C.outer_pad - C.filter_height,
        width: C.filter_all_width,
        height: C.filter_height,
    }
}

pub(crate) fn hit(x: i32, y: i32, width: u32, height: u32) -> Option<usize> {
    [(true, 6), (false, 7)]
        .into_iter()
        .find_map(|(back, action)| contains(rect(width, height, back), x, y).then_some(action))
}

pub(crate) fn enabled(action: usize, count: usize, page: usize) -> bool {
    match action {
        6 => page > 0,
        7 => page < max_room_page(count),
        _ => true,
    }
}

pub(super) fn draw(
    pm: &mut tiny_skia::PixmapMut<'_>,
    count: usize,
    page: usize,
    focus: Option<usize>,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    for (back, action, label) in [(true, 6, "Zurück"), (false, 7, "Weiter")] {
        let area = rect(pm.width(), pm.height(), back);
        let available = enabled(action, count, page);
        fill(pm, area, p.surface, Radius::DEFAULT.sm);
        outline(pm, area, p.border, Controls::BORDER);
        paint_text_centered(
            pm,
            label,
            area,
            Typography::DEFAULT.body_size as f32,
            if available { p.text } else { p.text_disabled() },
        );
        if available && focus == Some(action) {
            niwoe_ui::effect::paint_focus(pm, area, p.border_focus(), Radius::DEFAULT.sm);
        }
    }
    let area = Rect {
        x: C.sidebar_width + C.outer_pad,
        y: rect(pm.width(), pm.height(), true).y,
        width: rect(pm.width(), pm.height(), true).x - C.sidebar_width - C.outer_pad - S.md,
        height: C.filter_height,
    };
    paint_text_left_centered(
        pm,
        &format!(
            "Seite {} von {} · {} Räume im Filter",
            page + 1,
            max_room_page(count) + 1,
            count
        ),
        area.x,
        area,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
}
