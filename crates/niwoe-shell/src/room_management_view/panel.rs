//! Native module controls and a cached preview in product text geometry.
use super::*;
use crate::room_editor::panel::{label, PanelUi, MODULES};
use niwoe_ui::effect::{paint_focus, paint_text_pair, Symbol};

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
        y: C.header_height + S.xxl + row as i32 * (C.panel_module_height + C.card_gap),
        width: if col == 0 {
            main_width
        } else {
            C.config_action_width
        },
        height: C.panel_module_height,
    }
}

pub(crate) fn enabled(state: &PanelUi, index: usize) -> bool {
    if state.pending.is_some() {
        return false;
    }
    match index {
        0..=11 => {
            let position = state
                .draft
                .iter()
                .position(|module| *module == MODULES[index / 3]);
            match index % 3 {
                0 => true,
                1 => position.is_some_and(|p| p > 0),
                _ => position.is_some_and(|p| p + 1 < state.draft.len()),
            }
        }
        12 => state.ready,
        13 => true,
        _ => false,
    }
}

pub(crate) fn focus_order(state: &PanelUi) -> Vec<usize> {
    (0..14).filter(|index| enabled(state, *index)).collect()
}

pub(crate) fn hit(state: &PanelUi, x: i32, y: i32, width: u32, height: u32) -> Option<usize> {
    focus_order(state)
        .into_iter()
        .find(|&i| contains(control(width, height, i), x, y))
}

fn preview_y() -> i32 {
    control(C.canvas_min_width, C.canvas_min_height, 9).y + C.panel_module_height + S.xxl * 2
}

fn native_control(
    pm: &mut tiny_skia::PixmapMut<'_>,
    state: &PanelUi,
    index: usize,
    text: &str,
    config: &niwoe_config::ThemeConfig,
) {
    list::control(
        pm,
        control(pm.width(), pm.height(), index),
        text,
        list::ControlState {
            enabled: enabled(state, index),
            focused: state.focus == index,
            primary: index == 12,
            ..Default::default()
        },
        config,
    );
}

fn draw_module(
    pm: &mut tiny_skia::PixmapMut<'_>,
    state: &PanelUi,
    row: usize,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let module = MODULES[row];
    let position = state.draft.iter().position(|entry| *entry == module);
    let index = row * 3;
    let area = control(pm.width(), pm.height(), index);
    fill(
        pm,
        area,
        if position.is_some() {
            Interaction::DEFAULT.selected_tint(p.surface)
        } else {
            p.surface
        },
        Radius::DEFAULT.sm,
    );
    outline(pm, area, p.border_control(), Controls::BORDER);
    let checkbox = Rect {
        x: area.x + S.sm,
        y: area.y + (area.height - S.lg) / 2,
        width: S.lg,
        height: S.lg,
    };
    fill(pm, checkbox, p.surface_alt, Radius::DEFAULT.sm);
    outline(pm, checkbox, p.border_control(), Controls::BORDER);
    if position.is_some() {
        list::symbol(pm, checkbox, Symbol::Check, p.text);
    }
    let detail = match module {
        niwoe_ipc::PanelModule::Tray => "Symbole und Menüs geöffneter Apps",
        niwoe_ipc::PanelModule::Screenshot => "Bildschirmfoto aufnehmen",
        niwoe_ipc::PanelModule::Search => "Hub und Suche öffnen",
        niwoe_ipc::PanelModule::Status => "Netzwerk, Lautstärke und vorhandener Akku",
    };
    let detail = position.map_or_else(
        || format!("Ausgeblendet · {detail}"),
        |p| format!("Position {} · {detail}", p + 1),
    );
    let x = checkbox.x + checkbox.width + S.sm;
    paint_text_pair(
        pm,
        Rect {
            x,
            width: area.width - (x - area.x) - S.sm,
            ..area
        },
        label(module),
        &detail,
        if enabled(state, index) {
            p.text
        } else {
            p.text_disabled()
        },
        p.text_dim,
    );
    if enabled(state, index) && state.focus == index {
        paint_focus(pm, area, p.border_focus(), Radius::DEFAULT.sm);
    }
    native_control(pm, state, index + 1, "Früher", config);
    native_control(pm, state, index + 2, "Später", config);
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
    draw_sidebar(
        &mut pm,
        height,
        crate::control_center::Page::Panel,
        true,
        config,
    );
    let x = C.sidebar_width + C.outer_pad;
    let available = width as i32 - x - C.outer_pad;
    let heading = Rect {
        x,
        y: 0,
        width: available,
        height: C.header_height,
    };
    paint_text(
        &mut pm,
        "LEISTE",
        x,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    niwoe_ui::effect::paint_display_heading(
        &mut pm,
        "Leiste konfigurieren",
        x,
        C.outer_pad + S.xxl * 2,
        heading,
        p.text,
    );
    for (row, text) in [
        "Die obere Leiste zeigt deinen Entwurf. Speichern übernimmt die Änderungen.",
        "Hub, Räume und die mittige Uhr bleiben immer erreichbar.",
    ]
    .iter()
    .enumerate()
    {
        paint_text(
            &mut pm,
            &truncate_to_fit(text, available, Typography::DEFAULT.body_size as f32),
            x,
            C.outer_pad + S.xxl * 3 + S.lg + row as i32 * S.xl,
            Typography::DEFAULT.body_size as f32,
            p.text_dim,
        );
    }
    for row in 0..MODULES.len() {
        draw_module(&mut pm, state, row, config);
    }
    let caption = if available as u32 > C.panel_preview_max_width {
        "LEISTENVORSCHAU · ORIGINALGRÖSSE · AUSSCHNITT"
    } else {
        "LEISTENVORSCHAU · ORIGINALGRÖSSE"
    };
    paint_text(
        &mut pm,
        caption,
        x,
        preview_y() - S.md,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    if let Some(preview) = preview {
        pm.draw_pixmap(
            x,
            preview_y(),
            preview.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    }
    paint_text(
        &mut pm,
        "CPU-, GPU- und Sensoranzeigen sind noch nicht verfügbar.",
        x,
        preview_y() + crate::PANEL_SURFACE_HEIGHT as i32 + S.xl,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let footer = control(width, height, 13);
    let message = if !state.ready && state.message.is_empty() {
        "Leisteneinstellungen werden geladen …"
    } else if state.message.is_empty() {
        "Lokaler Entwurf · Abbrechen verwirft die Änderungen."
    } else {
        &state.message
    };
    paint_text_left_centered(
        &mut pm,
        &truncate_to_fit(
            message,
            footer.x - x - C.card_gap,
            Typography::DEFAULT.caption_size as f32,
        ),
        x,
        footer,
        Typography::DEFAULT.caption_size as f32,
        if state.message.contains("nicht gespeichert") || state.message.contains("Konflikt") {
            p.error
        } else {
            p.text_dim
        },
    );
    native_control(&mut pm, state, 12, "Änderungen speichern", config);
    native_control(&mut pm, state, 13, "Abbrechen", config);
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

#[cfg(test)]
mod tests;
