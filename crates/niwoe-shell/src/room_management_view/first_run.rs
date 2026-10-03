//! Event-driven introduction. Geometry and drawing primitives are shared with P10.
use super::*;
use crate::first_run::Wizard;

pub(crate) fn control(width: u32, height: u32, index: usize, count: usize, step: u8) -> Rect {
    let base = count - 5;
    let x = C.outer_pad;
    let available = width as i32 - x * 2;
    if index >= base {
        let col = (index - base) as i32;
        let w = (available - S.md * 4) / 5;
        return Rect {
            x: x + col * (w + S.md),
            y: height as i32 - C.config_footer_height + S.md,
            width: w,
            height: C.config_field_height,
        };
    }
    let (row, col, columns) = if step == 3 {
        (index / 3, index % 3, 3)
    } else if step == 2 {
        (index / 2, index % 2, 2)
    } else {
        (index, 0, 1)
    };
    let w = (available - S.md * (columns - 1)) / columns;
    Rect {
        x: x + col as i32 * (w + S.md),
        y: C.header_height + S.xxl * 3 + row as i32 * (C.config_field_height + S.md),
        width: w,
        height: C.config_field_height,
    }
}

pub(crate) fn hit(x: i32, y: i32, width: u32, height: u32, wizard: &Wizard) -> Option<usize> {
    let count = wizard.control_count();
    (0..count).find(|&i| contains(control(width, height, i, count, wizard.draft.step), x, y))
}

pub(crate) fn draw(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    wizard: &Wizard,
    preview: Option<&Pixmap>,
    config: &niwoe_config::ThemeConfig,
) {
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
    let step = wizard.draft.step as usize;
    let titles = [
        "Willkommen bei NIWOE",
        "Dein Bedienprofil",
        "Räume und vorhandene Apps",
        "Deine Leiste",
        "Bereit für deinen Desktop",
    ];
    let explanations = [
        ["Eine kurze Einführung: Willkommen → Bedienprofil → Räume → Leiste → Fertig.",
         "Bis zum Abschluss bleiben deine Einstellungen unverändert. Optionale Schritte sind überspringbar.",
         "Abbrechen behält den Entwurf. Du findest ihn unter Einstellungen → Geräteinformationen → Einführung."],
        ["Maus und Tastatur bieten dieselben Funktionen. Das Profil setzt ausschließlich Leisten-Defaults.",
         "Nur bei frischer, unkonfigurierter Leiste: Maus zeigt Suche; Tastatur verwendet Super+Space.",
         "Bestehende persönliche Einstellungen haben Vorrang. Du kannst die Leiste im nächsten Schritt anpassen."],
        ["Ergänze optional Räume. Vorhandene Räume und Zuordnungen bleiben erhalten.",
         "Wähle reale Apps für einen ausgewählten Vorschlag: Preferred lenkt spätere Starts in diesen Raum.",
         "Keine Installation oder App-Starts. Layouts, Browser-Sitzungen und Terminalprozesse werden nicht wiederhergestellt."],
        ["Die Vorschau verwendet dieselbe Leiste wie dein Desktop. Wähle Module und Reihenfolge.",
         "Hub, Raumzugang und mittige Uhr bleiben erhalten. Systemstatus zeigt nur vorhandene Daten.",
         "Überspringen und Abbrechen nehmen die Vorschau zurück."],
        ["Optional ausprobieren: Die Schaltflächen öffnen echte Produktfunktionen. Standard-Shortcuts stehen daneben.",
         "Zurück zur Einführung: Systemdeck → Einstellungen → Geräteinformationen → Einführung.",
         "Eigene Tastenkürzel haben Vorrang. Übernehmen speichert den Entwurf; es startet keine Apps."],
    ];
    paint_text(
        &mut pm,
        &format!("{} · Schritt {} von 5", titles[step], step + 1),
        C.outer_pad,
        C.outer_pad + S.xxl,
        Typography::DEFAULT.display_size as f32,
        p.text,
    );
    for (i, text) in explanations[step].iter().enumerate() {
        paint_text(
            &mut pm,
            &truncate_to_fit(
                text,
                width as i32 - C.outer_pad * 2,
                Typography::DEFAULT.body_size as f32,
            ),
            C.outer_pad,
            C.outer_pad + S.xxl * 2 + i as i32 * S.xl,
            Typography::DEFAULT.body_size as f32,
            p.text_dim,
        );
    }
    let controls = wizard.controls();

    for (i, label) in controls.iter().enumerate() {
        let rect = control(width, height, i, controls.len(), wizard.draft.step);
        let enabled = wizard.enabled(i);
        fill(&mut pm, rect, p.surface_alt, Radius::DEFAULT.sm);
        outline(
            &mut pm,
            rect,
            if wizard.focus == i {
                p.accent
            } else {
                p.border
            },
            if wizard.focus == i {
                Controls::FOCUS_WIDTH
            } else {
                Controls::BORDER
            },
        );
        paint_text_left_centered(
            &mut pm,
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
    let message_y = height as i32 - C.config_footer_height - S.xl;
    paint_text(
        &mut pm,
        &truncate_to_fit(
            &wizard.message,
            width as i32 - C.outer_pad * 2,
            Typography::DEFAULT.caption_size as f32,
        ),
        C.outer_pad,
        message_y,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
    if step == 4 {
        let summary = format!(
            "Entwurf: {} zusätzliche Räume · {} · Abschluss erst nach bestätigter Speicherung",
            wizard.draft.rooms.len(),
            if wizard.draft.panel.is_some() {
                "Leistenentwurf"
            } else {
                "Leiste unverändert"
            }
        );
        paint_text(
            &mut pm,
            &truncate_to_fit(
                &summary,
                width as i32 - C.outer_pad * 2,
                Typography::DEFAULT.body_size as f32,
            ),
            C.outer_pad,
            message_y - S.xxl,
            Typography::DEFAULT.body_size as f32,
            p.text,
        );
    }
    if let Some(preview) = preview {
        let scale = (width as i32 - C.outer_pad * 2) as f32 / preview.width() as f32;
        let y = control(width, height, 9, controls.len(), 3).y + C.config_field_height + S.xl;
        pm.draw_pixmap(
            0,
            0,
            preview.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::from_row(scale, 0.0, 0.0, scale, C.outer_pad as f32, y as f32),
            None,
        );
    }
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
