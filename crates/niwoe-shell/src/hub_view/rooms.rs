#[allow(clippy::too_many_arguments)]
fn draw_rooms(
    pm: &mut tiny_skia::PixmapMut<'_>,
    width: u32,
    rooms: &[RoomEntry],
    active_workspace: u8,
    window_counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    hovered_room: Option<usize>,
    keyboard_focus: Option<usize>,
    hub: &crate::hub_state::HubState,
    windows: &[WindowInfo],
    apps: &[crate::launcher::DesktopApp],
    icons: &crate::icons::IconCache,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let body = Typography::DEFAULT.body_size as f32;
    let caption = Typography::DEFAULT.caption_size as f32;
    for (index, room) in rooms.iter().take(H.room_columns as usize).enumerate() {
        let rect = room_rect(index, width);
        let active = room.workspace == active_workspace;
        let fill_color = if active {
            niwoe_tokens::Interaction::DEFAULT.selected_tint(p.surface)
        } else if hovered_room == Some(index) {
            niwoe_tokens::Interaction::DEFAULT.neutral_hover
        } else {
            alpha(p.surface, H.card_alpha)
        };
        fill(pm, rect, fill_color, Radius::DEFAULT.md);
        outline(
            pm,
            rect,
            if active { p.border_control() } else { p.border },
            if active {
                Controls::FOCUS_WIDTH
            } else {
                Controls::BORDER
            },
        );
        if keyboard_focus == Some(index) {
            niwoe_ui::effect::paint_focus(pm, rect, p.border_focus(), Radius::DEFAULT.md);
        }
        let icon = Rect {
            x: rect.x + H.card_pad,
            y: rect.y + H.card_pad,
            width: H.room_icon_size,
            height: H.room_icon_size,
        };
        fill(
            pm,
            icon,
            alpha(p.surface_alt, H.card_alpha),
            Radius::DEFAULT.sm,
        );
        if let Some(image) = niwoe_ui::effect::symbol_icon(
            niwoe_ui::effect::Symbol::Room,
            if active { p.accent } else { p.text_dim },
            H.app_icon_size as u32,
        ) {
            pm.draw_pixmap(
                icon.x + (icon.width - H.app_icon_size) / 2,
                icon.y + (icon.height - H.app_icon_size) / 2,
                image.as_ref().as_ref(),
                &tiny_skia::PixmapPaint::default(),
                tiny_skia::Transform::identity(),
                None,
            );
        }
        if let Some(image) = room
            .preferences
            .icon
            .as_deref()
            .and_then(|name| icons.lookup(name, S.xxl as u32))
            .and_then(crate::icons::icon_image_to_pixmap)
        {
            fill(
                pm,
                icon,
                alpha(p.surface_alt, H.card_alpha),
                Radius::DEFAULT.sm,
            );
            pm.draw_pixmap(
                icon.x + (icon.width - image.width() as i32) / 2,
                icon.y + (icon.height - image.height() as i32) / 2,
                image.as_ref(),
                &tiny_skia::PixmapPaint::default(),
                tiny_skia::Transform::identity(),
                None,
            );
        }
        let name = truncate_to_fit(
            &room.name,
            rect.width - H.room_icon_size - H.card_pad * 3,
            Typography::DEFAULT.title_size as f32,
        );
        paint_text_left_centered(
            pm,
            &name,
            icon.x + icon.width + S.md,
            icon,
            Typography::DEFAULT.title_size as f32,
            p.text,
        );
        draw_room_preview(pm, rect, room, windows, hub, apps, icons, config);
        let count = window_counts
            .get(room.workspace.saturating_sub(1) as usize)
            .copied()
            .unwrap_or_default();
        let detail = if active {
            format!("Aktiv · {count} offene Fenster")
        } else {
            format!("{count} offene Fenster")
        };
        let footer = Rect {
            x: rect.x + H.card_pad,
            y: rect.y + rect.height - H.card_pad - S.xxl,
            width: rect.width - H.card_pad * 2,
            height: S.xxl,
        };
        paint_text_left_centered(
            pm,
            &detail,
            rect.x + H.card_pad + S.lg,
            footer,
            caption,
            p.text_dim,
        );
        fill(
            pm,
            Rect {
                x: rect.x + H.card_pad,
                y: footer.y + (footer.height - H.status_dot_size) / 2,
                width: H.status_dot_size,
                height: H.status_dot_size,
            },
            if count > 0 { p.text } else { p.text_dim },
            Radius::DEFAULT.lg,
        );
        paint_text_right_centered(
            pm,
            "›",
            rect.x + rect.width - H.card_pad,
            footer,
            body,
            p.text,
        );
    }
}
