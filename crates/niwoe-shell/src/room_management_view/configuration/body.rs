#[allow(clippy::too_many_arguments)]
fn draw_body(
    pm: &mut tiny_skia::PixmapMut<'_>,
    room: &RoomEntry,
    edit: &Edit,
    order: usize,
    room_count: usize,
    rooms: &[RoomEntry],
    windows: &[WindowInfo],
    scroll_y: i32,
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
    outline(
        pm,
        name,
        if edit.focus == 0 { p.accent } else { p.border },
        Controls::BORDER,
    );
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
    fill(
        pm,
        description,
        p.surface_alt,
        Radius::DEFAULT.sm,
    );
    outline(pm, description, if edit.focus == 5 { p.accent } else { p.border }, Controls::BORDER);
    paint_text_left_centered(
        pm,
        &truncate_to_fit(&edit.description.replace('\n', " "), description.width - S.md * 2, Typography::DEFAULT.caption_size as f32),
        description.x + S.md,
        description,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
    paint_text(
        pm,
        &if edit.id == 0 { "Neuer Raum · noch nicht gespeichert".to_owned() } else { format!(
            "Raum {} · Position {} von {}",
            room.workspace,
            order + 1,
            room_count
        ) },
        details.x + C.card_pad,
        details.y + details.height - C.card_pad - S.md,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    for (later, label, allowed) in [
        (false, "← Früher", edit.id != 0 && order > 0),
        (true, "Später →", edit.id != 0 && order + 1 < room_count),
    ] {
        let action = order_rect(pm.width(), scroll_y, later);
        fill(
            pm,
            action,
            alpha(
                p.surface_alt,
                if allowed {
                    C.card_alpha
                } else {
                    C.disabled_alpha
                },
            ),
            Radius::DEFAULT.sm,
        );
        let focused = edit.focus == if later { 2 } else { 1 };
        outline(
            pm,
            action,
            if focused { p.accent } else { p.border },
            if focused {
                Controls::FOCUS_WIDTH
            } else {
                Controls::BORDER
            },
        );
        paint_text_centered(
            pm,
            label,
            action,
            Typography::DEFAULT.caption_size as f32,
            if allowed { p.text } else { p.text_dim },
        );
    }

    let context = Rect {
        x: left,
        y: details.y + details.height + C.card_gap,
        width: left_width,
        height: C.config_context_height,
    };
    section(pm, context, "KONTEXT & WIEDERHERSTELLUNG", p);
    let rows_top = context.y + S.xxl;
    let row_height = (context.height - S.xxl - C.card_pad * 2 - C.card_gap * 2) / 3;
    for (index, (title, note)) in [
        (
            "Fensterlayout merken",
            "Manuell im Tab Wiederherstellung · kein automatischer Login-Restore",
        ),
        (
            "Dateien wiederherstellen",
            "Explizite Dateiverweise im Tab Wiederherstellung · unterstützte Apps nötig",
        ),
        (
            "Terminal-Sitzungen fortsetzen",
            "Noch nicht verfügbar · keine Sitzungswiederherstellung",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        row(
            pm,
            Rect {
                x: context.x + C.card_pad,
                y: rows_top + index as i32 * (row_height + C.card_gap),
                width: context.width - C.card_pad * 2,
                height: row_height,
            },
            title,
            note,
            p,
        );
    }

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
    section(pm, apps, "START-APPS", p);
    section(pm, rules, "AUTOMATISIERUNGSREGELN", p);
    let assignment_note = match room.assignment {
        niwoe_ipc::RoomAssignment::Free => "Freie Zuordnung: alle Apps willkommen.",
        niwoe_ipc::RoomAssignment::Preferred => "Zugeordnete Apps bevorzugen diesen Raum.",
        niwoe_ipc::RoomAssignment::Dedicated => "Aufgabenraum: andere Apps bleiben erlaubt.",
    };
    for (rect, lines) in [
        (
            apps,
            [
                assignment_note,
                "Noch keine Start-Apps konfigurierbar.",
            ],
        ),
        (
            rules,
            [
                "Regeln folgen mit dem Automatisierungsdienst.",
                "Beim Raumwechsel läuft keine Regel.",
            ],
        ),
    ] {
        paint_centered_card_lines(pm, rect, &lines, p);
    }

    let preview = Rect {
        x: right,
        y: top,
        width: right_width,
        height: preview_height(pm.height()),
    };
    section(pm, preview, "VORSCHAU", p);
    let viewport = Rect {
        x: preview.x + C.card_pad,
        y: preview.y + S.xxl,
        width: preview.width - C.card_pad * 2,
        height: preview.height - S.xxl - C.card_pad,
    };
    fill(pm, viewport, p.background, Radius::DEFAULT.sm);
    draw_landscape(pm, viewport);
    fill(
        pm,
        viewport,
        alpha(p.background, C.header_tint_alpha),
        Radius::DEFAULT.sm,
    );
    outline(pm, viewport, p.border, Controls::BORDER);
    paint_text(
        pm,
        &truncate_to_fit(
            &edit.name,
            viewport.width - S.xl * 2,
            Typography::DEFAULT.title_size as f32,
        ),
        viewport.x + S.xl,
        viewport.y + S.xxl,
        Typography::DEFAULT.title_size as f32,
        p.text,
    );
    paint_text(
        pm,
        "Aktuell offene Fenster in diesem Raum",
        viewport.x + S.xl,
        viewport.y + S.xxl * 2,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let mut titles = windows
        .iter()
        .filter(|window| window.workspace == room.workspace && !window.title.trim().is_empty())
        .take(C.card_window_rows)
        .peekable();
    if titles.peek().is_none() {
        paint_text(
            pm,
            "Noch keine offenen Fenster",
            viewport.x + S.xl,
            viewport.y + S.xxl * 3 + S.lg,
            Typography::DEFAULT.body_size as f32,
            p.text_dim,
        );
    }
    for (index, window) in titles.enumerate() {
        let window_rect = Rect {
            x: viewport.x + S.xl,
            y: viewport.y + S.xxl * 3 + index as i32 * (S.xxl + S.lg),
            width: viewport.width - S.xl * 2,
            height: S.xxl + S.sm,
        };
        fill(
            pm,
            window_rect,
            alpha(p.surface, C.card_alpha),
            Radius::DEFAULT.sm,
        );
        outline(pm, window_rect, p.border, Controls::BORDER);
        paint_text_left_centered(
            pm,
            &truncate_to_fit(
                &window.title,
                window_rect.width - S.md * 2,
                Typography::DEFAULT.caption_size as f32,
            ),
            window_rect.x + S.md,
            window_rect,
            Typography::DEFAULT.caption_size as f32,
            p.text,
        );
    }
    let note = Rect {
        x: right,
        y: preview.y + preview.height + C.card_gap,
        width: right_width,
        height: note_height(pm.height()),
    };
    section(pm, note, if edit.id == 0 { "DEIN KONTEXT BLEIBT BEI DIR" } else { "RAUM LÖSCHEN" }, p);
    paint_centered_card_lines(
        pm,
        Rect { height: note.height - C.config_field_height - C.card_gap - if edit.id == 0 { 0 } else { S.xxl }, ..note },
        if edit.id == 0 { &[
        "NIWOE organisiert Räume und Fenster.",
        "Apps und Dateien bleiben in ihren Anwendungen.",
        "Weitere Restore-Funktionen folgen mit Backend.",
        ] } else if room_count == 1 { &[
            "Der letzte Raum kann nicht gelöscht werden.",
            "Erstelle zuerst einen weiteren Raum.",
        ] } else { &[
            "Offene Fenster wechseln in den gewählten Raum.",
            "Anwendungen bleiben geöffnet.",
        ] },
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
    let first_baseline = content_top as f32
        + ((content_bottom - content_top) as f32 - group_height) / 2.0
        + ascent;
    for (index, line) in lines.iter().enumerate() {
        paint_text(
            pm,
            &truncate_to_fit(
                line,
                rect.width - C.card_pad * 2,
                size,
            ),
            rect.x + C.card_pad,
            (first_baseline + index as f32 * S.xxl as f32).round() as i32,
            size,
            p.text_dim,
        );
    }
}
