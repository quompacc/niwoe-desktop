//! Room page composition. Captures are owned by the shared generation cache.
use super::*;
use niwoe_ui::effect::{paint_focus, paint_text_pair, paint_title_pair, Symbol};

pub(super) struct Context<'a> {
    pub card: &'a RoomRenderContext<'a>,
    pub rooms: &'a [RoomEntry],
    pub all_rooms: &'a [RoomEntry],
    pub page: usize,
    pub list: &'a crate::room_editor::list::ListUi,
    pub hub: &'a crate::hub_state::HubState,
    pub apps: &'a [crate::launcher::DesktopApp],
}

mod rail;

pub(super) fn draw(
    pm: &mut tiny_skia::PixmapMut<'_>,
    width: u32,
    height: u32,
    context: &Context<'_>,
) {
    let start = context.page * ROOM_PAGE_SIZE;
    let count = context
        .rooms
        .len()
        .saturating_sub(start)
        .min(ROOM_PAGE_SIZE);
    for (slot, room) in context.rooms.iter().skip(start).take(count).enumerate() {
        let area = room_rect(slot, width, height, count, context.list.list_view);
        draw_card(pm, area, room, context, start + slot);
    }
    if context.rooms.is_empty() {
        let p = crate::ui::tokens::theme_from_config(context.card.config).palette;
        let area = Rect {
            x: C.sidebar_width + C.outer_pad,
            y: list::grid_top(width),
            width: right_rail_x(width) - C.outer_pad - C.sidebar_width - C.outer_pad,
            height: height as i32 - list::grid_top(width) - C.room_page_footer_height - C.outer_pad,
        };
        paint_text_pair(
            pm,
            Rect {
                x: area.x + C.card_pad,
                width: area.width - C.card_pad * 2,
                ..area
            },
            "Keine passenden Räume",
            "Passe den Filter oder Suchtext an.",
            p.text,
            p.text_dim,
        );
    }
    rail::draw(pm, width, context);
}

fn app_identities<'a>(room: &'a RoomEntry, windows: &'a [WindowInfo]) -> Vec<&'a str> {
    let mut ids: Vec<_> = windows
        .iter()
        .filter(|w| w.workspace == room.workspace)
        .filter_map(|w| w.app_id.as_deref())
        .collect();
    for reference in &room.preferences.apps {
        let (niwoe_ipc::AppReference::Native(id) | niwoe_ipc::AppReference::Xwayland(id)) =
            reference;
        ids.push(id);
    }
    let mut seen = std::collections::HashSet::new();
    ids.retain(|id| seen.insert(*id));
    ids
}

pub(super) fn draw_card(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    room: &RoomEntry,
    context: &Context<'_>,
    index: usize,
) {
    let card = context.card;
    let p = crate::ui::tokens::theme_from_config(card.config).palette;
    let active = !card.preview && room.workspace == card.active_workspace;
    let background = if active {
        Interaction::DEFAULT.selected_tint(p.surface)
    } else if card.hovered_room == Some(index) {
        Interaction::DEFAULT.neutral_hover
    } else {
        alpha(p.surface, C.card_alpha)
    };
    fill(pm, area, background, Radius::DEFAULT.sm);
    outline(
        pm,
        area,
        if active { p.border_control() } else { p.border },
        Controls::BORDER,
    );
    if card.keyboard_focus == Some(index) {
        paint_focus(pm, area, p.border_focus(), Radius::DEFAULT.sm);
    }
    if context.list.list_view {
        draw_row(pm, area, room, context);
        return;
    }
    let heading = Rect {
        x: area.x + S.md,
        y: area.y + S.md,
        width: area.width - S.md * 2,
        height: C.room_heading_height,
    };
    let icon = Rect {
        width: S.xxl,
        ..heading
    };
    fill(pm, icon, p.surface_alt, Radius::DEFAULT.sm);
    if let Some(image) = room
        .preferences
        .icon
        .as_deref()
        .and_then(|id| card.icons.lookup(id, S.xxl as u32))
        .and_then(crate::icons::icon_image_to_pixmap)
    {
        pm.draw_pixmap(
            icon.x,
            icon.y + (icon.height - image.height() as i32) / 2,
            image.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    } else {
        list::symbol(
            pm,
            icon,
            Symbol::Room,
            if active { p.accent } else { p.text_dim },
        );
    }
    let description = if room.id == 0 {
        "Lokaler Entwurf · noch nicht gespeichert".into()
    } else if room.description.trim().is_empty() {
        format!(
            "Raum {} · {}",
            room.workspace,
            if card.preview {
                "Vorschau"
            } else if active {
                "Aktiv"
            } else {
                "Inaktiv"
            }
        )
    } else {
        room.description.replace('\n', " ")
    };
    paint_title_pair(
        pm,
        Rect {
            x: icon.x + icon.width + S.sm,
            width: heading.width - icon.width - S.sm,
            ..heading
        },
        &room.name,
        &description,
        p.text,
        p.text_dim,
    );
    let footer = Rect {
        x: heading.x,
        y: area.y + area.height - S.md - C.filter_height,
        width: heading.width,
        height: C.filter_height,
    };
    let picture = Rect {
        x: heading.x,
        y: heading.y + heading.height + S.sm,
        width: heading.width,
        height: footer.y - S.sm - heading.y - heading.height - S.sm,
    };
    preview(pm, picture, room, card.windows, context.hub, card.config);
    let ids = app_identities(room, card.windows);
    let footer_icon = Rect {
        width: S.lg,
        ..footer
    };
    let icon_name = ids
        .first()
        .and_then(|id| {
            context.apps.iter().find(|app| {
                app.desktop_id
                    .trim_end_matches(".desktop")
                    .eq_ignore_ascii_case(id)
                    || app
                        .program
                        .rsplit('/')
                        .next()
                        .is_some_and(|name| name.eq_ignore_ascii_case(id))
            })
        })
        .and_then(|app| app.icon_name.as_deref());
    if let Some(image) = icon_name
        .and_then(|name| card.icons.lookup(name, Controls::SYMBOL_SIZE))
        .and_then(crate::icons::icon_image_to_pixmap)
    {
        pm.draw_pixmap(
            footer_icon.x,
            footer_icon.y + (footer_icon.height - image.height() as i32) / 2,
            image.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    } else {
        list::symbol(pm, footer_icon, Symbol::App, p.text_dim);
    }
    let occupied = card
        .window_counts
        .get(room.workspace.saturating_sub(1) as usize)
        .copied()
        .unwrap_or_default();
    paint_text_pair(
        pm,
        Rect {
            x: footer.x + S.lg + S.xs,
            width: footer.width - S.lg - S.xs - C.room_card_action_width - S.sm,
            ..footer
        },
        &format!("Apps · {}", ids.len()),
        &format!("Fenster · {occupied}"),
        p.text,
        p.text_dim,
    );
    if card.preview {
        paint_text_centered(
            pm,
            "Vorschau",
            Rect {
                x: footer.x + footer.width - C.room_card_action_width,
                width: C.room_card_action_width,
                ..footer
            },
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
    } else {
        action(pm, footer, card.config);
    }
}

fn action(pm: &mut tiny_skia::PixmapMut<'_>, area: Rect, config: &niwoe_config::ThemeConfig) {
    list::control(
        pm,
        Rect {
            x: area.x + area.width - C.room_card_action_width,
            width: C.room_card_action_width,
            ..area
        },
        "Konfigurieren",
        list::ControlState {
            enabled: true,
            ..Default::default()
        },
        config,
    );
}

fn draw_row(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    room: &RoomEntry,
    context: &Context<'_>,
) {
    let p = crate::ui::tokens::theme_from_config(context.card.config).palette;
    let picture = Rect {
        x: area.x + S.xs,
        y: area.y + S.xs,
        width: C.room_list_preview_width,
        height: area.height - S.xs * 2,
    };
    preview(
        pm,
        picture,
        room,
        context.card.windows,
        context.hub,
        context.card.config,
    );
    let heading = Rect {
        x: picture.x + picture.width + S.sm,
        width: area.width - picture.width - C.room_card_action_width - S.sm * 3,
        ..picture
    };
    let count = context
        .card
        .window_counts
        .get(room.workspace.saturating_sub(1) as usize)
        .copied()
        .unwrap_or_default();
    paint_text_pair(
        pm,
        heading,
        &room.name,
        &format!("Raum {} · {count} Fenster", room.workspace),
        p.text,
        p.text_dim,
    );
    action(
        pm,
        Rect {
            x: area.x + S.sm,
            y: picture.y,
            width: area.width - S.sm * 2,
            height: picture.height,
        },
        context.card.config,
    );
}

pub(super) fn preview(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    room: &RoomEntry,
    windows: &[WindowInfo],
    hub: &crate::hub_state::HubState,
    config: &niwoe_config::ThemeConfig,
) {
    if area.width <= 0 || area.height <= 0 {
        return;
    }
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(pm, area, p.surface_alt, Radius::DEFAULT.sm);
    let window = crate::hub_state::preview_window(room, windows);
    if let Some(image) = window.and_then(|window| hub.images.get(&window.id)) {
        let factor = (area.width as f32 / image.width() as f32)
            .min(area.height as f32 / image.height() as f32);
        let x = area.x as f32 + (area.width as f32 - image.width() as f32 * factor) / 2.;
        let y = area.y as f32 + (area.height as f32 - image.height() as f32 * factor) / 2.;
        pm.draw_pixmap(
            0,
            0,
            image.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::from_scale(factor, factor).post_translate(x, y),
            None,
        );
    } else {
        let minimized = window.is_none() && windows.iter().any(|w| w.workspace == room.workspace);
        let label = if room.id == 0 {
            "Noch nicht gespeichert"
        } else if minimized {
            "Fenster minimiert"
        } else if window.is_some() {
            "Vorschau nicht verfügbar"
        } else {
            "Raum ist leer"
        };
        paint_text_centered(
            pm,
            &truncate_to_fit(
                label,
                area.width - S.sm * 2,
                Typography::DEFAULT.caption_size as f32,
            ),
            area,
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
    }
}
