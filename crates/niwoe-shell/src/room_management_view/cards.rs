fn draw_room_card(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    room: &RoomEntry,
    context: &RoomRenderContext<'_>,
    index: usize,
) {
    let p = crate::ui::tokens::theme_from_config(context.config).palette;
    let current = !context.preview && room.workspace == context.active_workspace;
    let occupied = context
        .window_counts
        .get(room.workspace.saturating_sub(1) as usize)
        .copied()
        .unwrap_or_default();
    let body = if current {
        Interaction::DEFAULT.selected_tint(p.surface)
    } else if context.hovered_room == Some(index) {
        Interaction::DEFAULT.neutral_hover
    } else {
        alpha(p.surface, C.card_alpha)
    };
    fill(pm, rect, body, Radius::DEFAULT.sm);
    outline(
        pm,
        rect,
        if current { p.border_control() } else { p.border },
        if current {
            Controls::FOCUS_WIDTH
        } else {
            Controls::BORDER
        },
    );
    if context.keyboard_focus == Some(index) {
        outline(pm, rect, p.text, Controls::FOCUS_WIDTH);
    }
    let x = rect.x + C.card_pad;
    let heading = Rect {
        x,
        y: rect.y + C.card_pad,
        width: rect.width - C.card_pad * 2,
        height: S.xxl,
    };
    if let Some(icon) = room
        .preferences
        .icon
        .as_deref()
        .and_then(|name| context.icons.lookup(name, S.xxl as u32))
        .and_then(crate::icons::icon_image_to_pixmap)
    {
        pm.draw_pixmap(
            x,
            heading.y,
            icon.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    } else {
        paint_text_left_centered(
            pm,
            "◇",
            x,
            heading,
            Typography::DEFAULT.title_size as f32,
            p.text_dim,
        );
    }
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            &room.name,
            rect.width - C.card_pad * 2 - S.xxl * 2,
            Typography::DEFAULT.title_size as f32,
        ),
        x + S.xxl,
        heading,
        Typography::DEFAULT.title_size as f32,
        p.text,
    );
    let compact = rect.height < C.card_pad * 2 + S.xxl + S.xl * 2 + C.config_field_height + S.md;
    let status = if compact && current {
        format!("Aktiv · {occupied} Fenster")
    } else if compact {
        format!("Inaktiv · {occupied} Fenster")
    } else if current {
        "Aktiver Raum".to_string()
    } else if occupied > 0 {
        format!("{occupied} offene Fenster")
    } else {
        "Inaktiv".to_string()
    };
    let status_row = Rect {
        x,
        y: heading.y + heading.height,
        width: heading.width,
        height: S.xl,
    };
    paint_text_left_centered(
        pm,
        &status,
        x + S.xxl,
        status_row,
        Typography::DEFAULT.caption_size as f32,
        if current || occupied > 0 {
            p.success
        } else {
            p.text_dim
        },
    );
    let footer = Rect {
        x,
        y: rect.y + rect.height - C.card_pad - C.config_field_height,
        width: rect.width - C.card_pad * 2,
        height: C.config_field_height,
    };
    let mut row_y = status_row.y + status_row.height + S.xl;
    let mut shown = 0;
    for window in context
        .windows
        .iter()
        .filter(|window| window.workspace == room.workspace && !window.title.trim().is_empty())
        .take(C.card_window_rows)
    {
        if row_y + S.md >= footer.y {
            break;
        }
        let title = truncate_to_fit(
            &window.title,
            rect.width - C.card_pad * 2,
            Typography::DEFAULT.caption_size as f32,
        );
        paint_text(
            pm,
            &title,
            x,
            row_y,
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
        row_y += S.xl;
        shown += 1;
    }
    if shown == 0 && row_y + S.md < footer.y {
        paint_text(
            pm,
            "Noch keine offenen Fenster",
            x,
            row_y,
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
    }
    paint_text_left_centered(
        pm,
        &format!("Raum {}", room.workspace),
        x,
        footer,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let action = Rect {
        x: rect.x + rect.width - C.card_pad - C.room_config_button_width,
        y: footer.y,
        width: C.room_config_button_width,
        height: footer.height,
    };
    let highlighted = context.hovered_room == Some(index) || context.keyboard_focus == Some(index);
    if !context.preview {
        fill(pm, action, p.surface_alt, Radius::DEFAULT.sm);
    }
    outline(
        pm,
        action,
        if highlighted { p.border_focus() } else { p.border },
        if highlighted {
            Controls::FOCUS_WIDTH
        } else {
            Controls::BORDER
        },
    );
    paint_text_centered(
        pm,
        if context.preview {
            "Vorschau"
        } else {
            "Konfigurieren  ›"
        },
        action,
        Typography::DEFAULT.body_size as f32,
        p.text,
    );
}
