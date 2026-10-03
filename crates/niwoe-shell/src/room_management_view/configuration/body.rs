#[allow(clippy::too_many_arguments)]
fn draw_body(
    pm: &mut tiny_skia::PixmapMut<'_>,
    room: &RoomEntry,
    edit: &Edit,
    _order: usize,
    room_count: usize,
    rooms: &[RoomEntry],
    windows: &[WindowInfo],
    scroll_y: i32,
    icons: &crate::icons::IconCache,
    hub: &crate::hub_state::HubState,
    catalog: &[crate::launcher::DesktopApp],
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let (left, left_width, right, right_width) = content_bounds(pm.width());
    let top = body_top() - scroll_y;
    let details = Rect {
        x: left,
        y: top,
        width: left_width,
        height: C.config_details_height,
    };
    section(pm, details, "RAUMDETAILS", p);
    let name = name_rect(pm.width(), scroll_y);
    paint_text(
        pm,
        "Name des Raums",
        name.x,
        name.y - S.sm,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    fill(pm, name, p.surface_alt, Radius::DEFAULT.sm);
    outline(pm, name, p.border_control(), Controls::BORDER);
    if edit.focus == 0 {
        niwoe_ui::effect::paint_focus(pm, name, p.border_focus(), Radius::DEFAULT.sm);
    }
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            &edit.name,
            name.width - S.md * 2,
            Typography::DEFAULT.body_size as f32,
        ),
        name.x + S.md,
        name,
        Typography::DEFAULT.body_size as f32,
        p.text,
    );
    let description = description_rect(pm.width(), scroll_y);
    paint_text(
        pm,
        "Beschreibung (optional)",
        description.x,
        description.y - S.sm,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    fill(pm, description, p.surface_alt, Radius::DEFAULT.sm);
    outline(pm, description, p.border_control(), Controls::BORDER);
    if edit.focus == 5 {
        niwoe_ui::effect::paint_focus(pm, description, p.border_focus(), Radius::DEFAULT.sm);
    }
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            &edit.description.replace('\n', " "),
            description.width - S.md * 2,
            Typography::DEFAULT.caption_size as f32,
        ),
        description.x + S.md,
        description,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
    paint_text(
        pm,
        &if edit.id == 0 {
            "Neuer Raum · noch nicht gespeichert".to_owned()
        } else {
            format!(
                "Raum {} · Position {} von {}",
                room.workspace,
                edit.position + 1,
                room_count
            )
        },
        details.x + C.card_pad,
        details.y + details.height - C.card_pad - S.md,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    for (later, label, allowed) in [
        (false, "Früher", edit.id != 0 && edit.position > 0),
        (
            true,
            "Später",
            edit.id != 0 && edit.position + 1 < room_count,
        ),
    ] {
        let action = order_rect(pm.width(), scroll_y, later);
        super::list::control(
            pm,
            action,
            label,
            super::list::ControlState {
                enabled: allowed,
                focused: edit.focus == if later { 2 } else { 1 },
                ..Default::default()
            },
            config,
        );
    }

    let context = Rect {
        x: left,
        y: details.y + details.height + C.card_gap,
        width: left_width,
        height: C.config_context_height,
    };
    section(pm, context, "KONTEXT & WIEDERHERSTELLUNG", p);
    let lower_top = context.y + context.height + C.card_gap;
    let lower_width = (left_width - C.card_gap) / 2;
    let apps = Rect {
        x: left,
        y: lower_top,
        width: lower_width,
        height: lower_height(pm.height()),
    };
    let rules = Rect {
        x: left + lower_width + C.card_gap,
        y: lower_top,
        width: left_width - lower_width - C.card_gap,
        height: lower_height(pm.height()),
    };
    section(pm, apps, "APPS", p);
    section(pm, rules, "DATEIVERWEISE", p);
    for (rect, lines) in [
        (
            apps,
            [
                "Kein Autostart beim Login oder Raumwechsel.",
                "App-Präferenzen steuern die Fensterzuordnung.",
            ],
        ),
        (
            rules,
            [
                "Nur ausdrücklich gespeicherte lokale Dateien.",
                "Keine Browser-/Terminal-Sitzungen.",
            ],
        ),
    ] {
        paint_centered_card_lines(
            pm,
            Rect {
                y: rect.y + S.xxl,
                height: rect.height - S.xxl,
                ..rect
            },
            &lines,
            p,
        );
    }
    let preview = Rect {
        x: right,
        y: top,
        width: right_width,
        height: preview_height(pm.height()),
    };
    section(pm, preview, "VORSCHAU · RAUMKARTE", p);
    let viewport = Rect {
        x: preview.x + C.card_pad,
        y: preview.y + S.xxl,
        width: preview.width - C.card_pad * 2,
        height: preview.height - S.xxl - C.card_pad,
    };
    let draft = RoomEntry {
        name: if edit.name.trim().is_empty() {
            room.name.clone()
        } else {
            edit.name.clone()
        },
        description: edit.description.clone(),
        assignment: edit.assignment,
        preferences: edit.preferences.clone(),
        ..room.clone()
    };
    let mut counts = [0; niwoe_config::rooms::MAX_ROOMS];
    if room.workspace > 0 {
        counts[room.workspace as usize - 1] = windows
            .iter()
            .filter(|w| w.workspace == room.workspace)
            .count()
            .min(u16::MAX as usize) as u16;
    }
    let context = RoomRenderContext {
        preview: true,
        icons,
        active_workspace: room.workspace,
        window_counts: &counts,
        windows,
        hovered_room: None,
        keyboard_focus: None,
        config,
    };
    let list = crate::room_editor::list::ListUi::default();
    super::management::draw_card(
        pm,
        viewport,
        &draft,
        &super::management::Context {
            card: &context,
            rooms,
            all_rooms: rooms,
            page: 0,
            list: &list,
            hub,
            apps: catalog,
        },
        0,
    );
    let note = Rect {
        x: right,
        y: preview.y + preview.height + C.card_gap,
        width: right_width,
        height: note_height(pm.height()),
    };
    section(
        pm,
        note,
        if edit.id == 0 {
            "DEIN KONTEXT BLEIBT BEI DIR"
        } else {
            "RAUM LÖSCHEN"
        },
        p,
    );
    paint_centered_card_lines(
        pm,
        Rect {
            height: note.height
                - C.config_field_height
                - C.card_gap
                - if edit.id == 0 { 0 } else { S.xxl },
            ..note
        },
        if edit.id == 0 {
            &[
                "NIWOE organisiert Räume und Fenster.",
                "Apps und Dateien bleiben in ihren Anwendungen.",
                "Wiederherstellung wird ausdrücklich ausgelöst.",
            ]
        } else if room_count == 1 {
            &[
                "Der letzte Raum kann nicht gelöscht werden.",
                "Erstelle zuerst einen weiteren Raum.",
            ]
        } else {
            &[
                "Offene Fenster wechseln in den gewählten Raum.",
                "Anwendungen bleiben geöffnet.",
            ]
        },
        p,
    );
    draw_deletion(pm, edit, room_count, rooms, scroll_y, p);
}

fn paint_centered_card_lines(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    lines: &[&str],
    p: niwoe_tokens::Palette,
) {
    let size = Typography::DEFAULT.caption_size as f32;
    let (ascent, descent) = ui_line_metrics(size);
    let content_top = rect.y + S.xxl;
    let content_bottom = rect.y + rect.height - C.card_pad;
    let group_height = ascent + descent + S.xxl as f32 * lines.len().saturating_sub(1) as f32;
    let first_baseline =
        content_top as f32 + ((content_bottom - content_top) as f32 - group_height) / 2.0 + ascent;
    for (index, line) in lines.iter().enumerate() {
        paint_text(
            pm,
            &truncate_to_fit(line, rect.width - C.card_pad * 2, size),
            rect.x + C.card_pad,
            (first_baseline + index as f32 * S.xxl as f32).round() as i32,
            size,
            p.text_dim,
        );
    }
}
