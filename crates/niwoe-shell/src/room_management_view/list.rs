//! Responsive toolbar; shared geometry and native control roles.
use super::*;
use niwoe_ui::effect::{paint_focus, symbol_icon, Symbol};

fn widths() -> [i32; 4] {
    [
        C.filter_all_width,
        C.filter_status_width,
        C.filter_inactive_width,
        C.filter_sort_width,
    ]
}
pub(crate) fn filter_rect(index: usize) -> Rect {
    Rect {
        x: C.sidebar_width
            + C.outer_pad
            + widths()[..index]
                .iter()
                .map(|w| *w + C.card_gap)
                .sum::<i32>(),
        y: C.header_height + (C.toolbar_height - C.filter_height) / 2,
        width: widths()[index],
        height: C.filter_height,
    }
}
pub(crate) fn category_rect() -> Rect {
    let sort = filter_rect(3);
    Rect {
        x: sort.x + sort.width + C.card_gap,
        width: C.category_width,
        ..sort
    }
}
pub(crate) fn compact(width: u32) -> bool {
    let category = category_rect();
    category.x + category.width + C.card_gap + C.search_min_width
        > right_rail_x(width) - C.outer_pad
}
pub(crate) fn grid_top(width: u32) -> i32 {
    C.header_height
        + if compact(width) {
            C.compact_toolbar_height
        } else {
            C.toolbar_height
        }
}
pub(crate) fn search_rect(width: u32) -> Rect {
    let category = category_rect();
    let (x, y) = if compact(width) {
        (
            C.sidebar_width + C.outer_pad,
            grid_top(width) - C.filter_height - S.xs,
        )
    } else {
        (category.x + category.width + C.card_gap, category.y)
    };
    Rect {
        x,
        y,
        width: right_rail_x(width) - C.outer_pad - x,
        height: C.filter_height,
    }
}
pub(crate) fn new_rect(width: u32) -> Rect {
    Rect {
        x: right_rail_x(width),
        width: C.room_config_button_width,
        ..filter_rect(0)
    }
}
pub(crate) fn view_rect(width: u32, list_view: bool) -> Rect {
    let new = new_rect(width);
    Rect {
        x: new.x
            + new.width
            + C.card_gap
            + if list_view {
                C.room_view_width + C.card_gap
            } else {
                0
            },
        width: C.room_view_width,
        ..new
    }
}
pub(crate) fn quick_rect(width: u32, index: usize) -> Rect {
    Rect {
        x: right_rail_x(width) + S.sm,
        y: grid_top(width) + S.xxl + S.md + index as i32 * Controls::MIN_HEIGHT,
        width: C.right_rail_width - S.sm * 2,
        height: Controls::MIN_HEIGHT,
    }
}
pub(crate) fn row_rect(index: usize, width: u32, height: u32) -> Rect {
    let y = grid_top(width);
    let available = height as i32 - C.outer_pad - C.room_page_footer_height - y;
    let row_height = (available - C.card_gap * (ROOM_PAGE_SIZE as i32 - 1)) / ROOM_PAGE_SIZE as i32;
    Rect {
        x: C.sidebar_width + C.outer_pad,
        y: y + index as i32 * (row_height + C.card_gap),
        width: right_rail_x(width) - C.outer_pad - C.sidebar_width - C.outer_pad,
        height: row_height,
    }
}
pub(crate) fn hit_list(x: i32, y: i32, width: u32) -> Option<usize> {
    if contains(search_rect(width), x, y) {
        return Some(4);
    }
    if contains(quick_rect(width, 0), x, y) {
        return Some(5);
    }
    for (list_view, action) in [(false, 8), (true, 9)] {
        if contains(view_rect(width, list_view), x, y) {
            return Some(action);
        }
    }
    (0..4).find(|&index| contains(filter_rect(index), x, y))
}
#[derive(Default)]
pub(super) struct ControlState {
    pub enabled: bool,
    pub selected: bool,
    pub focused: bool,
    pub primary: bool,
}
pub(super) fn control(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    label: &str,
    state: ControlState,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let background = if state.enabled && state.primary {
        p.accent
    } else if state.selected {
        Interaction::DEFAULT.selected_tint(p.surface)
    } else {
        p.surface
    };
    let foreground = if state.enabled && state.primary {
        p.on_accent()
    } else if state.enabled {
        p.text
    } else {
        p.text_disabled()
    };
    fill(pm, area, background, Radius::DEFAULT.sm);
    outline(pm, area, p.border_control(), Controls::BORDER);
    paint_text_centered(
        pm,
        &truncate_to_fit(
            label,
            area.width - S.sm * 2,
            Typography::DEFAULT.caption_size as f32,
        ),
        area,
        Typography::DEFAULT.caption_size as f32,
        foreground,
    );
    if state.enabled && state.focused {
        paint_focus(
            pm,
            area,
            if state.primary {
                foreground
            } else {
                p.border_focus()
            },
            Radius::DEFAULT.sm,
        );
    }
}
pub(super) fn symbol(pm: &mut tiny_skia::PixmapMut<'_>, area: Rect, artwork: Symbol, color: Color) {
    if let Some(image) = symbol_icon(artwork, color, Controls::SYMBOL_SIZE) {
        pm.draw_pixmap(
            area.x + (area.width - image.width() as i32) / 2,
            area.y + (area.height - image.height() as i32) / 2,
            image.as_ref().as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    }
}
pub(crate) fn occupancy(
    rooms: &[RoomEntry],
    counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
) -> usize {
    rooms
        .iter()
        .filter(|room| {
            counts
                .get(room.workspace.saturating_sub(1) as usize)
                .is_some_and(|count| *count > 0)
        })
        .count()
}
pub(super) fn draw_list_controls(
    pm: &mut tiny_skia::PixmapMut<'_>,
    list: &crate::room_editor::list::ListUi,
    rooms: &[RoomEntry],
    counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    new_focused: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let occupied = occupancy(rooms, counts);
    for (index, label) in [
        format!("Alle · {}", rooms.len()),
        format!("Belegt · {occupied}"),
        format!("Leer · {}", rooms.len() - occupied),
        if list.alphabetical {
            "Name A–Z".into()
        } else {
            "Raumfolge".into()
        },
    ]
    .iter()
    .enumerate()
    {
        control(
            pm,
            filter_rect(index),
            label,
            ControlState {
                enabled: true,
                selected: index < 3 && list.filter == index,
                focused: list.focus == Some(index),
                ..Default::default()
            },
            config,
        );
    }
    control(
        pm,
        category_rect(),
        "Kategorien fehlen",
        ControlState::default(),
        config,
    );
    let area = search_rect(pm.width());
    fill(pm, area, p.surface, Radius::DEFAULT.sm);
    outline(pm, area, p.border_control(), Controls::BORDER);
    let icon = Rect {
        x: area.x + S.sm,
        width: S.xl,
        ..area
    };
    symbol(pm, icon, Symbol::Search, p.text_dim);
    let x = icon.x + icon.width + S.sm;
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            if list.query.is_empty() {
                "Räume filtern …"
            } else {
                &list.query
            },
            area.x + area.width - x - S.sm,
            Typography::DEFAULT.body_size as f32,
        ),
        x,
        area,
        Typography::DEFAULT.body_size as f32,
        p.text,
    );
    if list.search_focus {
        paint_focus(pm, area, p.border_focus(), Radius::DEFAULT.sm);
    }
    control(
        pm,
        new_rect(pm.width()),
        "+ Neuer Raum",
        ControlState {
            enabled: rooms.len() < niwoe_config::rooms::MAX_ROOMS,
            primary: true,
            focused: new_focused,
            ..Default::default()
        },
        config,
    );
    for (list_view, action, artwork) in [(false, 8, Symbol::Grid), (true, 9, Symbol::List)] {
        let area = view_rect(pm.width(), list_view);
        control(
            pm,
            area,
            "",
            ControlState {
                enabled: true,
                selected: list.list_view == list_view,
                focused: list.focus == Some(action),
                ..Default::default()
            },
            config,
        );
        symbol(
            pm,
            area,
            artwork,
            if list.list_view == list_view {
                p.accent
            } else {
                p.text_dim
            },
        );
    }
}
