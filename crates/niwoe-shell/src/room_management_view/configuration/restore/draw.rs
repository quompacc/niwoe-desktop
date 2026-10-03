//! Event-driven composition of existing layout notices; no capture or I/O.
use super::*;
use niwoe_ui::effect::{paint_focus, paint_text_pair, Symbol};

pub(crate) fn draw(
    pm: &mut tiny_skia::PixmapMut<'_>,
    state: &RestoreUi,
    page: Page,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let width = pm.width();
    let height = pm.height();
    let left = C.sidebar_width + C.outer_pad;
    let available = width as i32 - left - C.outer_pad;
    let (title, subtitle) = match page {
        Page::Files => ("Dateiverweise für diesen Raum", "Verknüpfe eine Datei mit einem gespeicherten Fenster. Einträge stammen aus dem Raumlayout."),
        Page::Restore => ("Gespeichertes Raumlayout", "Nur anordnen verschiebt vorhandene Fenster. Apps öffnen sich nur auf deinen ausdrücklichen Aufruf."),
    };
    for (text, y, size, color) in [
        (
            title,
            body_top() + S.xl,
            Typography::DEFAULT.title_size,
            p.text,
        ),
        (
            subtitle,
            body_top() + S.xxl + S.sm,
            Typography::DEFAULT.caption_size,
            p.text_dim,
        ),
    ] {
        paint_text(
            pm,
            &truncate_to_fit(text, available, size as f32),
            left,
            y,
            size as f32,
            color,
        );
    }
    for &index in page.buttons() {
        let label = if index == 5 {
            format!(
                "{} · Seite {} / {}",
                BUTTONS[index],
                page_index(state, page, width, height) + 1,
                state.results.len().saturating_sub(1) / page_size(page, width, height) + 1
            )
        } else {
            BUTTONS[index].to_owned()
        };
        super::super::super::list::control(
            pm,
            control(page, width, index),
            &label,
            super::super::super::list::ControlState {
                enabled: enabled(state, page, index, width, height),
                primary: index == 0,
                focused: state.focus == index,
                ..Default::default()
            },
            config,
        );
    }
    let status = status_rect(page, width);
    fill(pm, status, p.surface, Radius::DEFAULT.sm);
    paint_text_pair(pm, Rect { x: status.x + S.sm, width: status.width - S.sm * 2, ..status },
        &truncate_to_fit(message(state), status.width - S.sm * 2, Typography::DEFAULT.body_size as f32),
        match page {
            Page::Files => "Verweis speichern bestätigt die Datei separat. Leer lassen entfernt den bestehenden Verweis.",
            Page::Restore => "Gespeichert werden Fensterpositionen. Browser-Sitzungen, Terminalprozesse und ungespeicherte Inhalte gehören nicht dazu.",
        }, if failed(state) { p.error } else { p.text }, p.text_dim);

    let count = page_size(page, width, height);
    let current = page_index(state, page, width, height);
    for (i, result) in state
        .results
        .iter()
        .skip(current * count)
        .take(count)
        .enumerate()
    {
        let area = row_rect(page, width, i);
        let selected = page == Page::Files && state.file_key == Some(result.key);
        fill(
            pm,
            area,
            if selected {
                Interaction::DEFAULT.selected_tint(p.surface)
            } else {
                p.surface
            },
            Radius::DEFAULT.sm,
        );
        outline(
            pm,
            area,
            if selected {
                p.accent
            } else {
                p.border_control()
            },
            Controls::BORDER,
        );
        let icon = Rect {
            x: area.x + S.sm,
            width: S.xl,
            ..area
        };
        super::super::super::list::symbol(
            pm,
            icon,
            if page == Page::Files {
                Symbol::Folder
            } else {
                Symbol::History
            },
            p.text_dim,
        );
        let secondary = match page {
            Page::Files => result
                .file
                .as_deref()
                .unwrap_or("Noch kein Dateiverweis · Eintrag auswählen"),
            Page::Restore => result.message.as_str(),
        };
        let text_x = icon.x + icon.width + S.sm;
        paint_text_pair(
            pm,
            Rect {
                x: text_x,
                width: area.x + area.width - text_x - S.sm,
                ..area
            },
            &result.label,
            secondary,
            p.text,
            p.text_dim,
        );
        if state.focus == 8 + i && enabled(state, page, 8 + i, width, height) {
            paint_focus(pm, area, p.border_focus(), Radius::DEFAULT.sm);
        }
    }
    if state.results.is_empty() {
        let area = row_rect(page, width, 0);
        let (title, detail) = if !state.ready {
            (
                "Layout wird geladen",
                "Die vorhandenen Einträge werden nach der Antwort angezeigt.",
            )
        } else if failed(state) {
            (
                "Kein nutzbares Layout",
                "Die Meldung oben erklärt den Fehler. Öffne die Ansicht nach der Korrektur erneut.",
            )
        } else if state.revision == 0 {
            (
                "Noch keine gespeicherten Fenster",
                if page == Page::Files {
                    "Öffne Wiederherstellung und speichere das Layout deiner Fenster in diesem Raum."
                } else {
                    "Öffne Fenster in diesem Raum und wähle Layout speichern."
                },
            )
        } else {
            (
                "Dieses Layout enthält keine Fenster",
                if page == Page::Files {
                    "Öffne Fenster in diesem Raum und speichere ihr Layout unter Wiederherstellung."
                } else {
                    "Öffne Fenster in diesem Raum und wähle Layout speichern."
                },
            )
        };
        paint_text_pair(pm, area, title, detail, p.text, p.text_dim);
    }
    if page == Page::Files {
        draw_file(pm, state, config);
    }
}

fn draw_file(
    pm: &mut tiny_skia::PixmapMut<'_>,
    state: &RestoreUi,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let width = pm.width();
    let height = pm.height();
    let file = file_rect(width, height);
    paint_text(
        pm,
        "Dateiverweis · absoluter Pfad · leer = entfernen",
        file.x,
        file.y - S.md,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let editable = enabled(state, Page::Files, 6, width, height);
    super::super::super::list::control(
        pm,
        file,
        "",
        super::super::super::list::ControlState {
            enabled: editable,
            focused: state.focus == 6,
            ..Default::default()
        },
        config,
    );
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            if !state.file.is_empty() {
                &state.file
            } else if editable {
                "Absoluten Dateipfad eingeben …"
            } else {
                "Zuerst einen gespeicherten Fenstereintrag auswählen"
            },
            file.width - S.sm * 2,
            Typography::DEFAULT.body_size as f32,
        ),
        file.x + S.sm,
        file,
        Typography::DEFAULT.body_size as f32,
        if editable { p.text } else { p.text_disabled() },
    );
    super::super::super::list::control(
        pm,
        save_file_rect(width, height),
        "Verweis speichern",
        super::super::super::list::ControlState {
            enabled: enabled(state, Page::Files, 7, width, height),
            primary: true,
            focused: state.focus == 7,
            ..Default::default()
        },
        config,
    );
}
