use super::*;

pub(crate) const TABS: [&str; 5] = [
    "Allgemein",
    "Apps",
    "Dateien",
    "Wiederherstellung",
    "Benachrichtigungen · nicht verfügbar",
];

pub(crate) fn tab_rect(index: usize) -> Rect {
    let size = Typography::DEFAULT.caption_size as f32;
    Rect {
        x: C.sidebar_width
            + C.outer_pad
            + TABS[..index]
                .iter()
                .map(|label| measure_text(label, size).0 + S.xxl + C.card_gap)
                .sum::<i32>(),
        y: C.config_header_height + S.sm,
        width: measure_text(TABS[index], size).0 + S.xxl,
        height: C.config_tabs_height - S.sm * 2,
    }
}

pub(crate) fn hit_tab(x: i32, y: i32) -> Option<usize> {
    (0..4).find(|&i| contains(tab_rect(i), x, y))
}

pub(crate) fn rect(width: u32, scroll: i32, index: usize) -> Rect {
    let (left, left_width, _, _) = content_bounds(width);
    let top = body_top() - scroll;
    if index == 9 {
        return Rect {
            x: left + C.card_pad,
            y: name_rect(width, scroll).y,
            width: S.xxl * 2 - C.card_pad,
            height: C.config_field_height,
        };
    }
    let context_top = top + C.config_details_height + C.card_gap;
    let row_height = (C.config_context_height - S.xxl - C.card_pad * 2 - C.card_gap * 2) / 3;
    if (10..=11).contains(&index) || index == 8 {
        let row = if index == 8 { 2 } else { index - 10 };
        return Rect {
            x: left + C.card_pad,
            y: context_top + S.xxl + row as i32 * (row_height + C.card_gap),
            width: left_width - C.card_pad * 2,
            height: row_height,
        };
    }
    let lower_width = (left_width - C.card_gap) / 2;
    Rect {
        x: left
            + if index == 13 {
                lower_width + C.card_gap
            } else {
                0
            }
            + C.card_pad,
        y: context_top + C.config_context_height + C.card_gap + S.xxl,
        width: lower_width - C.card_pad * 2,
        height: C.config_field_height,
    }
}

pub(crate) fn app_rect(width: u32, index: usize) -> Rect {
    let left = C.sidebar_width + C.outer_pad;
    let available = width as i32 - left - C.outer_pad;
    let row = match index {
        14 => 0,
        15 | 16 => 1,
        20..=25 => index - 18,
        _ => 0,
    };
    let half = (available - C.card_gap) / 2;
    Rect {
        x: left + if index == 16 { half + C.card_gap } else { 0 },
        y: body_top() + S.xxl * 2 + row as i32 * (C.config_field_height + C.card_gap),
        width: if index == 15 || index == 16 {
            half
        } else {
            available
        },
        height: C.config_field_height,
    }
}

fn button(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    label: &str,
    focus: bool,
    enabled: bool,
    p: niwoe_tokens::Palette,
) {
    fill(
        pm,
        rect,
        if enabled { p.surface_alt } else { p.surface },
        Radius::DEFAULT.sm,
    );
    outline(
        pm,
        rect,
        if focus { p.accent } else { p.border },
        if focus {
            Controls::FOCUS_WIDTH
        } else {
            Controls::BORDER
        },
    );
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            label,
            rect.width - S.md * 2,
            Typography::DEFAULT.caption_size as f32,
        ),
        rect.x + S.md,
        rect,
        Typography::DEFAULT.caption_size as f32,
        if enabled { p.text } else { p.text_dim },
    );
}

pub(super) fn draw_general(
    pm: &mut tiny_skia::PixmapMut<'_>,
    edit: &Edit,
    scroll: i32,
    p: niwoe_tokens::Palette,
) {
    let assignment = match edit.assignment {
        niwoe_ipc::RoomAssignment::Free => "Zuordnung: Frei · alle Apps willkommen",
        niwoe_ipc::RoomAssignment::Preferred => {
            "Zuordnung: Bevorzugt · ausgewählte Apps bevorzugen diesen Raum"
        }
        niwoe_ipc::RoomAssignment::Dedicated => {
            "Zuordnung: Dediziert · andere Apps bleiben erlaubt"
        }
    };
    let restore = match edit.preferences.restore {
        niwoe_ipc::RoomRestore::Disabled => "Wiederherstellung: Keine Vorgabe",
        niwoe_ipc::RoomRestore::LayoutOnly => "Wiederherstellung: Nur Fenster anordnen",
        niwoe_ipc::RoomRestore::RelaunchApps => {
            "Wiederherstellung: Apps ausdrücklich wieder öffnen"
        }
    };
    for (index, label) in [
        (9, "Icon"),
        (10, assignment),
        (11, restore),
        (8, "Layout und Wiederherstellung öffnen →"),
        (12, "App-Zuordnung konfigurieren →"),
        (13, "Explizite Dateiverweise öffnen →"),
    ] {
        button(
            pm,
            rect(pm.width(), scroll, index),
            label,
            edit.focus == index,
            true,
            p,
        );
    }
    if !edit.name_error.is_empty() {
        let name = name_rect(pm.width(), scroll);
        outline(pm, name, p.error, Controls::FOCUS_WIDTH);
        paint_text(
            pm,
            &edit.name_error,
            name.x,
            name.y + name.height + S.md,
            Typography::DEFAULT.caption_size as f32,
            p.error,
        );
    }
}

pub(crate) fn draw_apps(pm: &mut tiny_skia::PixmapMut<'_>, edit: &Edit, p: niwoe_tokens::Palette) {
    let x = C.sidebar_width + C.outer_pad;
    paint_text(
        pm,
        "App-Präferenzen · keine automatischen Starts",
        x,
        body_top() + S.xl,
        Typography::DEFAULT.title_size as f32,
        p.text,
    );
    paint_text(pm,"Native App-ID und XWayland-Klasse sind getrennte Identitäten. Fehlende Apps bleiben entfernbar.",x,body_top()+S.xxl+S.md,Typography::DEFAULT.caption_size as f32,p.text_dim);
    let rows = edit.app_rows();
    for (index, label) in [
        (14, format!("Suchen: {}", edit.form.query)),
        (15, "← Vorige Apps".into()),
        (
            16,
            format!(
                "Weitere Apps → · Seite {} / {}",
                edit.form.page + 1,
                rows.len().saturating_sub(1) / 6 + 1
            ),
        ),
    ] {
        button(
            pm,
            app_rect(pm.width(), index),
            &label,
            edit.focus == index,
            match index {
                15 => edit.form.page > 0,
                16 => (edit.form.page + 1) * 6 < rows.len(),
                _ => true,
            },
            p,
        );
    }
    for (row, index) in rows.iter().skip(edit.form.page * 6).take(6).enumerate() {
        let (label, reference) = &edit.form.apps[*index];
        let selected = edit.preferences.apps.contains(reference);
        button(
            pm,
            app_rect(pm.width(), 20 + row),
            &format!("{} {label}", if selected { "[✓]" } else { "[ ]" }),
            edit.focus == 20 + row,
            true,
            p,
        );
    }
    let note = if !edit.form.error.is_empty() {
        edit.form.error.as_str()
    } else if rows.is_empty() {
        "Keine passenden Apps im Katalog."
    } else {
        "Auswahl wird erst mit Änderungen speichern übernommen."
    };
    paint_text(
        pm,
        note,
        x,
        app_rect(pm.width(), 25).y + C.config_field_height + S.xl,
        Typography::DEFAULT.caption_size as f32,
        if edit.form.error.is_empty() {
            p.text_dim
        } else {
            p.error
        },
    );
}
