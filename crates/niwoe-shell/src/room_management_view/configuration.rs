//! Native room configuration, backed by revision-checked room mutations.

use super::*;
use crate::room_editor::Edit;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConfigurationAction {
    Restore,
    Tab(usize),
    Form(usize),
    Back,
    Name,
    MoveEarlier,
    MoveLater,
    Save,
    Cancel,
    Description,
    Delete,
    Target,
}

mod actions;
pub(crate) mod form;
pub(crate) mod restore;
use actions::{deletion_rect, description_rect, draw_deletion};
mod target_menu;
pub(crate) use target_menu::hit_target_menu;

fn content_bounds(width: u32) -> (i32, i32, i32, i32) {
    let left = C.sidebar_width + C.outer_pad;
    let right = width as i32 - C.outer_pad;
    let available = (right - left - C.card_gap).max(S.xxl * 2);
    let left_width = (available * 3 / 5).max(S.xxl);
    (
        left,
        left_width,
        left + left_width + C.card_gap,
        available - left_width,
    )
}

fn body_top() -> i32 {
    C.config_header_height + C.config_tabs_height + C.outer_pad
}

fn body_height(height: u32) -> i32 {
    height as i32 - C.config_footer_height - C.outer_pad - body_top()
}

fn lower_height(height: u32) -> i32 {
    (body_height(height) - C.config_details_height - C.config_context_height - C.card_gap * 2)
        .max(C.config_lower_height.max(C.config_note_height))
}

fn preview_height(height: u32) -> i32 {
    C.config_preview_height
        .min(body_height(height).max(C.config_details_height))
}

fn note_height(height: u32) -> i32 {
    lower_height(height)
}

pub(crate) fn max_configuration_scroll(height: u32) -> i32 {
    let left_bottom =
        C.config_details_height + C.config_context_height + lower_height(height) + C.card_gap * 2;
    let right_bottom = preview_height(height) + note_height(height) + C.card_gap;
    let content_bottom = body_top() + left_bottom.max(right_bottom) + C.outer_pad;
    (content_bottom - (height as i32 - C.config_footer_height)).max(0)
}

fn footer_action_rect(width: u32, height: u32, save: bool) -> Rect {
    let save_x = width as i32 - C.outer_pad - C.config_save_width;
    Rect {
        x: if save {
            save_x
        } else {
            save_x - C.card_gap - C.config_action_width
        },
        y: height as i32 - C.config_footer_height + C.card_gap,
        width: if save {
            C.config_save_width
        } else {
            C.config_action_width
        },
        height: C.config_field_height,
    }
}

fn name_rect(width: u32, scroll_y: i32) -> Rect {
    let (left, left_width, _, _) = content_bounds(width);
    Rect {
        x: left + C.card_pad + S.xxl * 2 + S.md,
        y: body_top() - scroll_y + S.xxl * 2,
        width: (left_width - C.card_pad * 3 - S.xxl * 2 - S.md) / 2,
        height: C.config_field_height,
    }
}

fn order_rect(width: u32, scroll_y: i32, later: bool) -> Rect {
    let (left, left_width, _, _) = content_bounds(width);
    let right = left + left_width - C.card_pad;
    Rect {
        x: right
            - if later {
                C.config_action_width
            } else {
                C.config_action_width * 2 + C.card_gap
            },
        y: body_top() - scroll_y + C.config_details_height - C.card_pad - C.config_field_height,
        width: C.config_action_width,
        height: C.config_field_height,
    }
}

pub(crate) fn hit_configuration(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scroll_y: i32,
) -> Option<ConfigurationAction> {
    if let Some(tab) = form::hit_tab(x, y) {
        return Some(ConfigurationAction::Tab(tab));
    }
    if restore::hit_tab(x, y) {
        return Some(ConfigurationAction::Restore);
    }
    if contains(back_rect(height), x, y)
        || contains(
            Rect {
                x: C.sidebar_width + C.outer_pad,
                y: 0,
                width: C.right_rail_width,
                height: C.config_field_height,
            },
            x,
            y,
        )
    {
        return Some(ConfigurationAction::Back);
    }
    if contains(footer_action_rect(width, height, true), x, y) {
        return Some(ConfigurationAction::Save);
    }
    if contains(footer_action_rect(width, height, false), x, y) {
        return Some(ConfigurationAction::Cancel);
    }
    if y < C.config_header_height + C.config_tabs_height
        || y >= height as i32 - C.config_footer_height
    {
        return None;
    }
    for index in [9, 10, 11, 8, 12, 13] {
        if contains(form::rect(width, scroll_y, index), x, y) {
            return Some(ConfigurationAction::Form(index));
        }
    }
    if contains(name_rect(width, scroll_y), x, y) {
        return Some(ConfigurationAction::Name);
    }
    if contains(description_rect(width, scroll_y), x, y) {
        return Some(ConfigurationAction::Description);
    }
    for (target, action) in [
        (true, ConfigurationAction::Target),
        (false, ConfigurationAction::Delete),
    ] {
        if contains(deletion_rect(width, height, scroll_y, target), x, y) {
            return Some(action);
        }
    }
    if contains(order_rect(width, scroll_y, false), x, y) {
        return Some(ConfigurationAction::MoveEarlier);
    }
    if contains(order_rect(width, scroll_y, true), x, y) {
        return Some(ConfigurationAction::MoveLater);
    }
    None
}

fn section(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, title: &str, p: niwoe_tokens::Palette) {
    fill(pm, rect, alpha(p.surface, C.card_alpha), Radius::DEFAULT.sm);
    outline(pm, rect, p.border, Controls::BORDER);
    paint_text(
        pm,
        title,
        rect.x + C.card_pad,
        rect.y + S.xl,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
}

include!("configuration/body.rs");

mod chrome;

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_room_configuration(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    room: &RoomEntry,
    edit: &Edit,
    order: usize,
    room_count: usize,
    rooms: &[RoomEntry],
    windows: &[WindowInfo],
    message: &str,
    pending: bool,
    scroll_y: i32,
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
    let mut pm = image.as_mut();
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(
        &mut pm,
        Rect {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        },
        p.background,
        Radius::DEFAULT.none,
    );
    if edit.restore.open {
        restore::draw(
            &mut pm,
            &edit.restore,
            restore::Page::from_tab(edit.form.tab),
            config,
        );
    } else if edit.form.tab == 1 {
        form::draw_apps(&mut pm, edit, icons, apps, config);
    } else {
        draw_body(
            &mut pm, room, edit, order, room_count, rooms, windows, scroll_y, icons, hub, apps,
            config,
        );
        form::draw_general(&mut pm, edit, scroll_y, icons, config);
        target_menu::draw(&mut pm, edit, rooms, scroll_y, p);
    }
    chrome::draw(
        &mut pm,
        room,
        edit,
        message,
        pending || edit.creation_uncertain,
        config,
    );
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
mod tests;
