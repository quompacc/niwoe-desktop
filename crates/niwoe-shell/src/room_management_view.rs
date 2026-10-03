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
    crate::control_center::back_rect(height)
}

fn right_rail_x(width: u32) -> i32 {
    width as i32 - C.right_rail_width - C.outer_pad
}

fn right_rail_toolbar_rect(width: u32) -> Rect {
    list::new_rect(width)
}

fn grid_geometry(width: u32, height: u32, _room_count: usize) -> (i32, i32, i32, i32, usize) {
    let grid_x = C.sidebar_width + C.outer_pad;
    let grid_y = list::grid_top(width);
    let grid_right = right_rail_x(width) - C.outer_pad;
    let grid_width = (grid_right - grid_x).max(C.room_columns);
    let rows = C.room_page_rows as usize;
    let card_width = (grid_width - C.card_gap * (C.room_columns - 1)) / C.room_columns;
    let available_height =
        (height as i32 - grid_y - C.outer_pad - C.room_page_footer_height).max(rows as i32);
    let card_height =
        (available_height - C.card_gap * (rows.saturating_sub(1) as i32)) / rows as i32;
    (grid_x, grid_y, card_width, card_height, rows)
}

fn room_rect(index: usize, width: u32, height: u32, room_count: usize, list_view: bool) -> Rect {
    if list_view {
        return list::row_rect(index, width, height);
    }
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
    list_view: bool,
) -> Option<usize> {
    let page = page.min(max_room_page(room_count));
    let start = page * ROOM_PAGE_SIZE;
    let count = room_count.saturating_sub(start).min(ROOM_PAGE_SIZE);
    (0..count)
        .find(|&index| contains(room_rect(index, width, height, count, list_view), x, y))
        .map(|index| start + index)
}

include!("room_management_view/sidebar.rs");
pub(crate) mod first_run;
mod header;
pub(crate) mod list;
mod management;
pub(crate) mod pages;
pub(crate) mod panel;
use header::draw_header;

struct RoomRenderContext<'a> {
    preview: bool,
    icons: &'a crate::icons::IconCache,
    active_workspace: u8,
    window_counts: &'a [u16; niwoe_config::rooms::MAX_ROOMS],
    windows: &'a [WindowInfo],
    hovered_room: Option<usize>,
    keyboard_focus: Option<usize>,
    config: &'a niwoe_config::ThemeConfig,
}

// Room cards share the native management composition, including configuration previews.
mod configuration;
pub(crate) use configuration::{
    draw_room_configuration, hit_configuration, hit_target_menu, max_configuration_scroll,
    ConfigurationAction,
};
pub(crate) use configuration::{form, restore};
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_room_management(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    rooms: &[RoomEntry],
    all_rooms: &[RoomEntry],
    active_workspace: u8,
    window_counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    windows: &[WindowInfo],
    hovered_room: Option<usize>,
    keyboard_focus: Option<usize>,
    page: usize,
    list: &crate::room_editor::list::ListUi,
    icons: &crate::icons::IconCache,
    hub: &crate::hub_state::HubState,
    apps: &[crate::launcher::DesktopApp],
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
    draw_sidebar(
        &mut pm,
        height,
        crate::control_center::Page::Rooms,
        false,
        config,
    );
    draw_header(&mut pm, width, config);
    let page = page.min(max_room_page(rooms.len()));
    list::draw_list_controls(
        &mut pm,
        list,
        all_rooms,
        window_counts,
        keyboard_focus == Some(rooms.len()),
        config,
    );
    let context = RoomRenderContext {
        preview: false,
        icons,
        active_workspace,
        window_counts,
        windows,
        hovered_room,
        keyboard_focus,
        config,
    };
    let management = management::Context {
        card: &context,
        rooms,
        all_rooms,
        page,
        list,
        hub,
        apps,
    };
    management::draw(&mut pm, width, height, &management);
    pages::draw(&mut pm, rooms.len(), page, list.focus, config);
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
