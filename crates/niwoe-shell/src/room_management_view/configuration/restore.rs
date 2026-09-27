//! Uses existing Control Center geometry, text, focus and surface tokens.
use super::*;
use crate::room_editor::RestoreUi;

const BUTTONS: [&str; 6] = [
    "Layout speichern",
    "Nur anordnen",
    "Apps wieder öffnen",
    "Abbrechen",
    "Vorige Ergebnisse",
    "Weitere Ergebnisse",
];

pub(crate) fn hit_tab(x: i32, y: i32) -> bool {
    let size = Typography::DEFAULT.caption_size as f32;
    let start = C.sidebar_width
        + C.outer_pad
        + ["Allgemein", "Apps", "Dateien"]
            .iter()
            .map(|s| measure_text(s, size).0 + S.xxl + C.card_gap)
            .sum::<i32>();
    contains(
        Rect {
            x: start,
            y: C.config_header_height + S.sm,
            width: measure_text("Wiederherstellung", size).0 + S.xxl,
            height: C.config_tabs_height - S.sm * 2,
        },
        x,
        y,
    )
}

fn control(width: u32, index: usize) -> Rect {
    let left = C.sidebar_width + C.outer_pad;
    let available = width as i32 - left - C.outer_pad;
    let columns = C.room_columns;
    Rect {
        x: left + index as i32 % columns * ((available + C.card_gap) / columns),
        y: body_top() + index as i32 / columns * (C.config_field_height + S.md),
        width: (available - C.card_gap * (columns - 1)) / columns,
        height: C.config_field_height,
    }
}

fn text_top(width: u32) -> i32 {
    let last = control(width, BUTTONS.len() - 1);
    last.y + last.height + S.xl
}
fn results_top(width: u32) -> i32 {
    text_top(width) + S.xl * 5
}

pub(crate) fn page_size(width: u32, height: u32) -> usize {
    ((height as i32
        - C.config_footer_height
        - C.outer_pad
        - results_top(width)
        - C.config_field_height * 2
        - S.xl)
        / C.config_field_height)
        .max(1) as usize
}

fn file_rect(width: u32, height: u32) -> Rect {
    Rect {
        x: C.sidebar_width + C.outer_pad,
        y: height as i32 - C.config_footer_height - C.outer_pad - C.config_field_height,
        width: width as i32
            - C.sidebar_width
            - C.outer_pad * 2
            - C.config_action_width
            - C.card_gap,
        height: C.config_field_height,
    }
}

pub(crate) fn enabled(state: &RestoreUi, control: usize, width: u32, height: u32) -> bool {
    match control {
        0..=2 => !state.running,
        3 => state.running,
        4 => state.page > 0,
        5 => (state.page + 1) * page_size(width, height) < state.results.len(),
        7 => state.file_key.is_some() && !state.running,
        _ => true,
    }
}

pub(crate) fn hit(x: i32, y: i32, width: u32, height: u32) -> Option<usize> {
    if let Some(i) = (0..BUTTONS.len()).find(|i| contains(control(width, *i), x, y)) {
        return Some(i);
    }
    let file = file_rect(width, height);
    if contains(file, x, y) {
        return Some(6);
    }
    if contains(
        Rect {
            x: file.x + file.width + C.card_gap,
            width: C.config_action_width,
            ..file
        },
        x,
        y,
    ) {
        return Some(7);
    }
    if x >= file.x && x < width as i32 - C.outer_pad && y >= results_top(width) {
        let row = (y - results_top(width)) / C.config_field_height;
        if (row as usize) < page_size(width, height) {
            return Some(8 + row as usize);
        }
    }
    None
}

pub(crate) fn draw(pm: &mut tiny_skia::PixmapMut<'_>, state: &RestoreUi, p: niwoe_tokens::Palette) {
    let size = Typography::DEFAULT.body_size as f32;
    for (i, label) in BUTTONS.iter().enumerate() {
        let rect = control(pm.width(), i);
        let available = enabled(state, i, pm.width(), pm.height());
        fill(
            pm,
            rect,
            if available { p.surface_alt } else { p.surface },
            Radius::DEFAULT.sm,
        );
        outline(
            pm,
            rect,
            if state.focus == i { p.accent } else { p.border },
            if state.focus == i {
                Controls::FOCUS_WIDTH
            } else {
                Controls::BORDER
            },
        );
        paint_text_centered(
            pm,
            label,
            rect,
            size,
            if available { p.text } else { p.text_dim },
        );
    }
    let left = C.sidebar_width + C.outer_pad;
    let width = pm.width() as i32 - left - C.outer_pad;
    for (index, text) in [
        "Manuell: Nur anordnen öffnet keine Apps. Apps wieder öffnen ist ausdrücklich optional.",
        "Keine Browser-Sitzungen, Terminalprozesse oder ungespeicherten Dokumente.",
        "Dateien nur über unterstützte App-Aufrufe. Mehrdeutige Fenster werden gemeldet.",
        "Nach Speichern: Änderungen nach 2 s Ruhe; nach Login/Restore erneut Speichern zum Aktivieren.",
        state.message.as_str(),
    ]
    .iter()
    .enumerate()
    {
        paint_text(
            pm,
            &truncate_to_fit(text, width, size),
            left,
            text_top(pm.width()) + index as i32 * S.xl,
            size,
            if index == 4
                && (state.message.contains("fehlgeschlagen")
                    || state.message.starts_with("Kein lesbares Layout")
                    || state.message.starts_with("Automatisches Speichern gestoppt"))
            {
                p.error
            } else {
                p.text
            },
        );
    }
    let count = page_size(pm.width(), pm.height());
    for (i, result) in state
        .results
        .iter()
        .skip(state.page * count)
        .take(count)
        .enumerate()
    {
        let rect = Rect {
            x: left,
            y: results_top(pm.width()) + i as i32 * C.config_field_height,
            width,
            height: C.config_field_height,
        };
        if state.file_key == Some(result.key) || state.focus == 8 + i {
            outline(pm, rect, p.accent, Controls::FOCUS_WIDTH);
        }
        paint_text_left_centered(
            pm,
            &truncate_to_fit(
                &format!(
                    "{} · {}",
                    truncate_to_fit(&result.label, C.right_rail_width, size),
                    result.message
                ),
                width - S.md,
                size,
            ),
            left + S.sm,
            rect,
            size,
            match result.message.as_str() {
                "Fenster angeordnet" => p.success,
                "Ausstehend" => p.text_dim,
                _ => p.warning,
            },
        );
    }
    let file = file_rect(pm.width(), pm.height());
    paint_text(
        pm,
        "Dateiverweis: Ergebnis auswählen, absoluten Pfad eingeben (leer = entfernen)",
        left,
        file.y - S.md,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    for (index, rect, label) in [
        (6, file, state.file.as_str()),
        (
            7,
            Rect {
                x: file.x + file.width + C.card_gap,
                width: C.config_action_width,
                ..file
            },
            "Datei speichern",
        ),
    ] {
        let available = enabled(state, index, pm.width(), pm.height());
        fill(
            pm,
            rect,
            if available { p.surface_alt } else { p.surface },
            Radius::DEFAULT.sm,
        );
        outline(
            pm,
            rect,
            if state.focus == index {
                p.accent
            } else {
                p.border
            },
            if state.focus == index {
                Controls::FOCUS_WIDTH
            } else {
                Controls::BORDER
            },
        );
        paint_text_left_centered(
            pm,
            &truncate_to_fit(label, rect.width - S.md, size),
            rect.x + S.sm,
            rect,
            size,
            if available { p.text } else { p.text_dim },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_restore_actions_are_disabled() {
        let mut state = RestoreUi::default();
        for control in [3, 4, 5, 7] {
            assert!(!enabled(&state, control, 1366, 768));
        }
        state.file_key = Some(1);
        assert!(enabled(&state, 7, 1366, 768));
        state.running = true;
        assert!(enabled(&state, 3, 1366, 768));
        for control in [0, 1, 2, 7] {
            assert!(!enabled(&state, control, 1366, 768));
        }
    }

    #[test]
    fn restore_controls_and_paged_results_fit_supported_canvases() {
        for (width, height) in [
            (1366, 768),
            (1920, 1032),
            niwoe_tokens::Hub::DEFAULT.canvas_size(960, 540),
        ] {
            for i in 0..BUTTONS.len() {
                let r = control(width, i);
                assert!(r.width > 0 && r.x + r.width <= width as i32);
                assert_eq!(
                    hit(r.x + r.width / 2, r.y + r.height / 2, width, height),
                    Some(i)
                );
            }
            let file = file_rect(width, height);
            assert!(
                results_top(width) + page_size(width, height) as i32 * C.config_field_height
                    < file.y - S.md
            );
            assert_eq!(hit(file.x + S.sm, file.y + S.sm, width, height), Some(6));
            assert!(page_size(width, height) > 0);
        }
    }
}
