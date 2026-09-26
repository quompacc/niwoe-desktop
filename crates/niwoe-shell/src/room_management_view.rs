//! Full-width room management surface from the binding Control Center mockup.
//! Values shown here come from the compositor room/window snapshots.

use crate::wayland::WindowInfo;
use niwoe_ipc::RoomEntry;
use niwoe_tokens::{Color, ControlCenter, Controls, Interaction, Radius, Spacing, Typography};
use niwoe_ui::{
    effect::{
        measure_text, paint_border, paint_fill, paint_text, rounded_rect_path, truncate_to_fit,
        ui_line_metrics,
    },
    paint::Rect,
};
use tiny_skia::Pixmap;

const C: ControlCenter = ControlCenter::DEFAULT;
const S: Spacing = Spacing::DEFAULT;
const ROOM_PAGE_SIZE: usize = C.room_columns as usize * C.room_page_rows as usize;

pub(crate) fn max_room_page(room_count: usize) -> usize {
    room_count.saturating_sub(1) / ROOM_PAGE_SIZE
}

fn alpha(color: Color, value: u8) -> Color {
    Color { a: value, ..color }
}

fn fill(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, color: Color, radius: i32) {
    if let Some(path) = rounded_rect_path(rect, radius) {
        paint_fill(pm, &path, color);
    }
}

fn outline(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, color: Color, width: i32) {
    if let Some(path) = rounded_rect_path(rect, Radius::DEFAULT.sm) {
        paint_border(pm, &path, color, width as f32);
    }
}

fn contains(rect: Rect, x: i32, y: i32) -> bool {
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}

fn centered_baseline(rect: Rect, size: f32) -> i32 {
    let (ascent, descent) = ui_line_metrics(size);
    (rect.y as f32 + (rect.height as f32 + ascent + descent) / 2.0).round() as i32
}

fn paint_text_left_centered(
    pm: &mut tiny_skia::PixmapMut<'_>,
    text: &str,
    x: i32,
    rect: Rect,
    size: f32,
    color: Color,
) {
    paint_text(pm, text, x, centered_baseline(rect, size), size, color);
}

fn paint_text_centered(
    pm: &mut tiny_skia::PixmapMut<'_>,
    text: &str,
    rect: Rect,
    size: f32,
    color: Color,
) {
    let width = measure_text(text, size).0;
    paint_text_left_centered(
        pm,
        text,
        rect.x + (rect.width - width) / 2,
        rect,
        size,
        color,
    );
}

fn back_rect(height: u32) -> Rect {
    Rect {
        x: C.outer_pad,
        y: height as i32 - C.config_footer_height + C.card_gap,
        width: C.sidebar_width - C.outer_pad * 2,
        height: C.config_field_height,
    }
}

fn right_rail_x(width: u32) -> i32 {
    width as i32 - C.right_rail_width - C.outer_pad
}

fn right_rail_toolbar_rect(width: u32) -> Rect {
    Rect {
        x: right_rail_x(width),
        y: C.header_height + (C.toolbar_height - C.filter_height) / 2,
        width: C.right_rail_width,
        height: C.filter_height,
    }
}

fn grid_geometry(width: u32, height: u32, room_count: usize) -> (i32, i32, i32, i32, usize) {
    let grid_x = C.sidebar_width + C.outer_pad;
    let grid_y = C.header_height + C.toolbar_height;
    let grid_right = right_rail_x(width) - C.outer_pad;
    let grid_width = (grid_right - grid_x).max(C.room_columns);
    let rows = room_count.max(1).div_ceil(C.room_columns as usize);
    let card_width = (grid_width - C.card_gap * (C.room_columns - 1)) / C.room_columns;
    let available_height = (height as i32 - grid_y - C.outer_pad).max(rows as i32);
    let card_height =
        (available_height - C.card_gap * (rows.saturating_sub(1) as i32)) / rows as i32;
    (grid_x, grid_y, card_width, card_height, rows)
}

fn room_rect(index: usize, width: u32, height: u32, room_count: usize) -> Rect {
    let (grid_x, grid_y, card_width, card_height, _) = grid_geometry(width, height, room_count);
    let column = index as i32 % C.room_columns;
    let row = index as i32 / C.room_columns;
    Rect {
        x: grid_x + column * (card_width + C.card_gap),
        y: grid_y + row * (card_height + C.card_gap),
        width: card_width,
        height: card_height,
    }
}

pub(crate) fn hit_back(x: i32, y: i32, height: u32) -> bool {
    contains(back_rect(height), x, y)
}

pub(crate) fn hit_new_room(x: i32, y: i32, width: u32) -> bool {
    contains(right_rail_toolbar_rect(width), x, y)
}

pub(crate) fn hit_room(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    room_count: usize,
    page: usize,
) -> Option<usize> {
    let page = page.min(max_room_page(room_count));
    let start = page * ROOM_PAGE_SIZE;
    let count = room_count.saturating_sub(start).min(ROOM_PAGE_SIZE);
    (0..count)
        .find(|&index| contains(room_rect(index, width, height, count), x, y))
        .map(|index| start + index)
}

fn sidebar_item(
    pm: &mut tiny_skia::PixmapMut<'_>,
    y: i32,
    label: &str,
    active: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let rect = Rect {
        x: C.outer_pad / 2,
        y,
        width: C.sidebar_width - C.outer_pad,
        height: C.sidebar_item_height,
    };
    if active {
        fill(
            pm,
            rect,
            Interaction::DEFAULT.selection(p.surface, p.accent, Interaction::SELECTION_ACTIVE),
            Radius::DEFAULT.sm,
        );
        fill(
            pm,
            Rect {
                x: rect.x,
                y: rect.y,
                width: Controls::FOCUS_WIDTH,
                height: rect.height,
            },
            p.accent,
            Radius::DEFAULT.none,
        );
    }
    paint_text_left_centered(
        pm,
        label,
        rect.x + S.xl,
        rect,
        Typography::DEFAULT.body_size as f32,
        if active { p.text } else { p.text_dim },
    );
}

fn draw_sidebar(
    pm: &mut tiny_skia::PixmapMut<'_>,
    height: u32,
    configuring: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(
        pm,
        Rect {
            x: 0,
            y: 0,
            width: C.sidebar_width,
            height: height as i32,
        },
        alpha(p.background, C.sidebar_alpha),
        Radius::DEFAULT.none,
    );
    paint_text(
        pm,
        "CONTROL CENTER",
        C.outer_pad,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let mut y = C.outer_pad + S.xxl * 2;
    for (label, active) in [
        ("Übersicht", false),
        ("Räume", true),
        ("Apps", false),
        ("Dateien", false),
        ("Automatisierung", false),
        ("Benutzer", false),
        ("System", false),
    ] {
        sidebar_item(pm, y, label, active, config);
        y += C.sidebar_item_height;
    }
    y += C.sidebar_section_gap;
    paint_text(
        pm,
        "WARTUNG",
        C.outer_pad,
        y,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    y += S.xxl;
    for label in ["Updates", "Backups", "Protokolle", "Einstellungen"] {
        sidebar_item(pm, y, label, false, config);
        y += C.sidebar_item_height;
    }
    let back = back_rect(height);
    outline(pm, back, p.border, Controls::BORDER);
    paint_text_centered(
        pm,
        if configuring {
            "‹  Zurück zu Räumen"
        } else {
            "‹  Zurück zur Übersicht"
        },
        back,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
}

fn draw_header(pm: &mut tiny_skia::PixmapMut<'_>, width: u32, config: &niwoe_config::ThemeConfig) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(
        pm,
        Rect {
            x: C.sidebar_width,
            y: 0,
            width: width as i32 - C.sidebar_width,
            height: C.header_height,
        },
        alpha(p.background, C.header_tint_alpha),
        Radius::DEFAULT.none,
    );
    let x = C.sidebar_width + C.outer_pad * 2;
    paint_text(
        pm,
        "RÄUME",
        x,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    paint_text(
        pm,
        "Räume verwalten",
        x,
        C.outer_pad + S.xxl * 2,
        Typography::DEFAULT.display_size as f32,
        p.text,
    );
    paint_text(
        pm,
        "Organisiere deine Arbeitsumgebungen. Räume bündeln Kontext und Fenster.",
        x,
        C.outer_pad + S.xxl * 3 + S.lg,
        Typography::DEFAULT.body_size as f32,
        p.text_dim,
    );
    let search = Rect {
        x: right_rail_x(width),
        y: C.outer_pad,
        width: C.right_rail_width,
        height: C.filter_height,
    };
    fill(
        pm,
        search,
        alpha(p.surface, C.card_alpha),
        Radius::DEFAULT.sm,
    );
    outline(pm, search, p.border, Controls::BORDER);
    paint_text_left_centered(
        pm,
        "⌕  Räume, Apps und Einstellungen",
        search.x + S.md,
        search,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
}

fn chip(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    text: &str,
    active: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(pm, rect, alpha(p.surface, C.card_alpha), Radius::DEFAULT.sm);
    outline(
        pm,
        rect,
        if active { p.accent } else { p.border },
        if active {
            Controls::FOCUS_WIDTH
        } else {
            Controls::BORDER
        },
    );
    paint_text_centered(
        pm,
        text,
        rect,
        Typography::DEFAULT.caption_size as f32,
        if active { p.text } else { p.text_dim },
    );
}

fn draw_toolbar(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rooms: &[RoomEntry],
    active_workspace: u8,
    window_counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    page: usize,
    new_focused: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let active = rooms
        .iter()
        .filter(|room| {
            room.workspace == active_workspace
                || window_counts
                    .get(room.workspace.saturating_sub(1) as usize)
                    .is_some_and(|count| *count > 0)
        })
        .count();
    let y = C.header_height + (C.toolbar_height - C.filter_height) / 2;
    let mut x = C.sidebar_width + C.outer_pad;
    for (label, width, selected) in [
        (
            format!("Alle Räume · {}", rooms.len()),
            C.filter_all_width,
            true,
        ),
        (format!("Aktiv · {active}"), C.filter_status_width, false),
        (
            format!("Inaktiv · {}", rooms.len().saturating_sub(active)),
            C.filter_inactive_width,
            false,
        ),
        (
            if rooms.len() > ROOM_PAGE_SIZE {
                format!("Seite {} / {}", page + 1, max_room_page(rooms.len()) + 1)
            } else {
                "Nach Name ⌄".to_string()
            },
            C.filter_sort_width,
            false,
        ),
    ] {
        chip(
            pm,
            Rect {
                x,
                y,
                width,
                height: C.filter_height,
            },
            &label,
            selected,
            config,
        );
        x += width + C.card_gap;
    }
    let add = right_rail_toolbar_rect(pm.width());
    fill(
        pm,
        add,
        if rooms.len() < niwoe_config::rooms::MAX_ROOMS {
            p.accent
        } else {
            alpha(p.surface, C.disabled_alpha)
        },
        Radius::DEFAULT.sm,
    );
    outline(
        pm,
        add,
        if new_focused { p.text } else { p.border },
        if new_focused {
            Controls::FOCUS_WIDTH
        } else {
            Controls::BORDER
        },
    );
    paint_text_centered(
        pm,
        "+  Neuer Raum",
        add,
        Typography::DEFAULT.caption_size as f32,
        if rooms.len() < niwoe_config::rooms::MAX_ROOMS {
            p.on_accent()
        } else {
            p.text_dim
        },
    );
}

struct RoomRenderContext<'a> {
    rooms: &'a [RoomEntry],
    active_workspace: u8,
    window_counts: &'a [u16; niwoe_config::rooms::MAX_ROOMS],
    windows: &'a [WindowInfo],
    hovered_room: Option<usize>,
    keyboard_focus: Option<usize>,
    page: usize,
    config: &'a niwoe_config::ThemeConfig,
}

include!("room_management_view/cards.rs");
mod configuration;
pub(crate) use configuration::{
    draw_room_configuration, hit_configuration, hit_target_menu, max_configuration_scroll,
    ConfigurationAction,
};
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_room_management(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    rooms: &[RoomEntry],
    active_workspace: u8,
    window_counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    windows: &[WindowInfo],
    hovered_room: Option<usize>,
    keyboard_focus: Option<usize>,
    page: usize,
    config: &niwoe_config::ThemeConfig,
) {
    if canvas.len() != width as usize * height as usize * 4 {
        return;
    }
    let Some(mut image) = Pixmap::new(width, height) else {
        return;
    };
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let mut pm = image.as_mut();
    fill(
        &mut pm,
        Rect {
            x: C.sidebar_width,
            y: C.header_height,
            width: width as i32 - C.sidebar_width,
            height: height as i32 - C.header_height,
        },
        alpha(p.background, C.content_alpha),
        Radius::DEFAULT.none,
    );
    draw_sidebar(&mut pm, height, false, config);
    draw_header(&mut pm, width, config);
    let page = page.min(max_room_page(rooms.len()));
    draw_toolbar(
        &mut pm,
        rooms,
        active_workspace,
        window_counts,
        page,
        keyboard_focus == Some(rooms.len()),
        config,
    );
    let context = RoomRenderContext {
        rooms,
        active_workspace,
        window_counts,
        windows,
        hovered_room,
        keyboard_focus,
        page,
        config,
    };
    draw_room_cards(&mut pm, width, height, &context);
    draw_right_rail(&mut pm, width, &context);
    for (rgba, bgra) in image
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .zip(canvas.as_chunks_mut::<4>().0)
    {
        bgra.copy_from_slice(&[rgba[2], rgba[1], rgba[0], rgba[3]]);
    }
}

#[cfg(test)]
#[path = "room_management_view/tests.rs"]
mod tests;
