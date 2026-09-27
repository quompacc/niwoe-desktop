use super::*;
use crate::room_editor::panel::{label, PanelUi, MODULES};

pub(crate) fn control(width: u32, height: u32, index: usize) -> Rect {
    let left = C.sidebar_width + C.outer_pad;
    let available = width as i32 - left - C.outer_pad;
    if index >= 12 {
        return Rect {
            x: width as i32
                - C.outer_pad
                - C.config_save_width
                - if index == 13 {
                    C.config_save_width + C.card_gap
                } else {
                    0
                },
            y: height as i32 - C.config_footer_height + C.card_gap,
            width: C.config_save_width,
            height: C.config_field_height,
        };
    }
    let row = index / 3;
    let col = index % 3;
    let main_width = available - C.config_action_width * 2 - C.card_gap * 2;
    Rect {
        x: left
            + match col {
                0 => 0,
                1 => main_width + C.card_gap,
                _ => main_width + C.config_action_width + C.card_gap * 2,
            },
        y: C.header_height + S.xxl * 2 + row as i32 * (C.config_field_height + S.xl),
        width: if col == 0 {
            main_width
        } else {
            C.config_action_width
        },
        height: C.config_field_height,
    }
}

pub(crate) fn hit(x: i32, y: i32, width: u32, height: u32) -> Option<usize> {
    (0..14).find(|&i| contains(control(width, height, i), x, y))
}

pub(crate) fn draw(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    state: &PanelUi,
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
    draw_sidebar(&mut pm, height, false, config);
    let x = C.sidebar_width + C.outer_pad;
    paint_text(
        &mut pm,
        "Leiste konfigurieren",
        x,
        C.outer_pad + S.xxl * 2,
        Typography::DEFAULT.display_size as f32,
        p.text,
    );
    for (row, text) in [
        "Live-Vorschau: Die obere Leiste verwendet deinen lokalen Entwurf.",
        "Hub, Räume und die mittige Uhr bleiben immer erreichbar.",
        "CPU/GPU/Sensoranzeigen sind ohne Provider nicht verfügbar.",
    ]
    .iter()
    .enumerate()
    {
        paint_text(
            &mut pm,
            text,
            x,
            C.outer_pad + S.xxl * 3 + row as i32 * S.xl,
            Typography::DEFAULT.body_size as f32,
            p.text_dim,
        );
    }
    for (row, module) in MODULES.iter().enumerate() {
        let position = state.draft.iter().position(|m| m == module);
        for col in 0..3 {
            let index = row * 3 + col;
            let r = control(width, height, index);
            let enabled = state.pending.is_none()
                && match col {
                    0 => true,
                    1 => position.is_some_and(|p| p > 0),
                    _ => position.is_some_and(|p| p + 1 < state.draft.len()),
                };
            let text = match col {
                0 => format!(
                    "{} {}{}",
                    if position.is_some() { "[✓]" } else { "[ ]" },
                    label(*module),
                    position.map_or(String::new(), |p| format!(" · Position {}", p + 1))
                ),
                1 => "← Früher".into(),
                _ => "Später →".into(),
            };
            fill(
                &mut pm,
                r,
                if enabled { p.surface_alt } else { p.surface },
                Radius::DEFAULT.sm,
            );
            outline(
                &mut pm,
                r,
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
                &mut pm,
                &truncate_to_fit(
                    &text,
                    r.width - S.md * 2,
                    Typography::DEFAULT.caption_size as f32,
                ),
                r.x + S.md,
                r,
                Typography::DEFAULT.caption_size as f32,
                if enabled { p.text } else { p.text_dim },
            );
        }
    }
    paint_text(
        &mut pm,
        &truncate_to_fit(
            &state.message,
            width as i32 - x - C.outer_pad,
            Typography::DEFAULT.caption_size as f32,
        ),
        x,
        control(width, height, 9).y + C.config_field_height + S.xxl,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
    if let Some(preview) = preview {
        let available = width as i32 - x - C.outer_pad;
        let scale = available as f32 / preview.width() as f32;
        let y = control(width, height, 9).y + C.config_field_height + S.xxl * 2;
        pm.draw_pixmap(
            0,
            0,
            preview.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::from_row(scale, 0.0, 0.0, scale, x as f32, y as f32),
            None,
        );
    }
    for (index, text) in [(12, "Änderungen speichern"), (13, "Abbrechen")] {
        let r = control(width, height, index);
        fill(&mut pm, r, p.surface_alt, Radius::DEFAULT.sm);
        outline(
            &mut pm,
            r,
            if state.focus == index {
                p.accent
            } else {
                p.border
            },
            Controls::FOCUS_WIDTH,
        );
        paint_text_centered(
            &mut pm,
            text,
            r,
            Typography::DEFAULT.caption_size as f32,
            if state.pending.is_none() {
                p.text
            } else {
                p.text_dim
            },
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
