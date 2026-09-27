use super::*;

pub(crate) fn search_rect(width: u32) -> Rect {
    Rect {
        x: right_rail_x(width),
        y: C.outer_pad,
        width: C.right_rail_width,
        height: C.filter_height,
    }
}

pub(crate) fn filter_rect(index: usize) -> Rect {
    let widths = [
        C.filter_all_width,
        C.filter_status_width,
        C.filter_inactive_width,
        C.filter_sort_width,
    ];
    Rect {
        x: C.sidebar_width
            + C.outer_pad
            + widths[..index].iter().map(|w| *w + C.card_gap).sum::<i32>(),
        y: C.header_height + (C.toolbar_height - C.filter_height) / 2,
        width: widths[index],
        height: C.filter_height,
    }
}

pub(crate) fn hit_list(x: i32, y: i32, width: u32) -> Option<usize> {
    if contains(search_rect(width), x, y) {
        return Some(4);
    }
    if x >= right_rail_x(width)
        && (C.header_height + C.toolbar_height + S.xxl + S.lg
            ..C.header_height + C.toolbar_height + S.xxl * 2 + S.lg)
            .contains(&y)
    {
        return Some(5);
    }
    (0..4).find(|&index| contains(filter_rect(index), x, y))
}

pub(super) fn draw_list_controls(
    pm: &mut tiny_skia::PixmapMut<'_>,
    list: &crate::room_editor::list::ListUi,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let r = search_rect(pm.width());
    fill(pm, r, p.surface, Radius::DEFAULT.sm);
    outline(
        pm,
        r,
        if list.search_focus {
            p.accent
        } else {
            p.border
        },
        Controls::BORDER,
    );
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            &format!("Suchen: {}", list.query),
            r.width - S.md * 2,
            Typography::DEFAULT.caption_size as f32,
        ),
        r.x + S.md,
        r,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
    for (index, label) in [
        "Alle Räume",
        "Belegt",
        "Leer",
        if list.alphabetical {
            "Name ↑"
        } else {
            "Raumreihenfolge"
        },
    ]
    .iter()
    .enumerate()
    {
        chip(
            pm,
            filter_rect(index),
            label,
            if index == 3 {
                list.alphabetical
            } else {
                list.filter == index
            },
            config,
        );
        if list.focus == Some(index) {
            outline(pm, filter_rect(index), p.text, Controls::FOCUS_WIDTH);
        }
    }
}
