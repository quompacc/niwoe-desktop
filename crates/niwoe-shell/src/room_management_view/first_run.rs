//! Five introduction pages using the Control Center's native component family.
//! Event-driven painting only; existing bounded font/symbol/panel caches apply.
use super::*;
use crate::first_run::Wizard;

mod body;
mod layout;
pub(crate) use layout::hit;

pub(crate) fn preview_width(width: u32) -> u32 {
    layout::content(width).width.max(0) as u32
}

const TITLES: [&str; 5] = [
    "Willkommen bei NIWOE",
    "Dein Bedienprofil",
    "Räume und Apps",
    "Deine Leiste",
    "Bereit für deinen Desktop",
];
const STEPS: [&str; 5] = ["Willkommen", "Bedienprofil", "Räume", "Leiste", "Fertig"];
const DETAILS: [&str; 5] = [
    "Richte deinen Desktop in wenigen Schritten ein. Deine Auswahl bleibt zunächst ein Entwurf.",
    "Wähle einen Ausgangspunkt. Deine vorhandenen persönlichen Einstellungen bleiben erhalten.",
    "Ergänze passende Räume und ordne ihnen vorhandene Apps zu. Alles ist optional.",
    "Wähle Module und ihre Reihenfolge. Hub, Räume und die mittige Uhr bleiben erreichbar.",
    "Probiere die wichtigsten Wege aus und prüfe deine Auswahl vor dem Übernehmen.",
];

fn native_control(
    pm: &mut tiny_skia::PixmapMut<'_>,
    wizard: &Wizard,
    index: usize,
    label: &str,
    config: &niwoe_config::ThemeConfig,
) {
    list::control(
        pm,
        layout::control(
            pm.width(),
            pm.height(),
            index,
            wizard.control_count(),
            wizard.draft.step,
        ),
        label,
        list::ControlState {
            enabled: wizard.enabled(index),
            focused: wizard.focus == index,
            primary: index == wizard.control_count() - 4,
            ..Default::default()
        },
        config,
    );
}

fn heading(pm: &mut tiny_skia::PixmapMut<'_>, wizard: &Wizard, config: &niwoe_config::ThemeConfig) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let area = layout::content(pm.width());
    let step = wizard.draft.step as usize;
    paint_text(
        pm,
        &format!("EINFÜHRUNG · SCHRITT {} VON 5", step + 1),
        area.x,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    niwoe_ui::effect::paint_display_heading(
        pm,
        TITLES[step],
        area.x,
        C.outer_pad + S.xxl * 2,
        Rect {
            height: C.header_height,
            ..area
        },
        p.text,
    );
    paint_text(
        pm,
        &truncate_to_fit(
            DETAILS[step],
            area.width,
            Typography::DEFAULT.body_size as f32,
        ),
        area.x,
        C.outer_pad + S.xxl * 3 + S.xs,
        Typography::DEFAULT.body_size as f32,
        p.text_dim,
    );
    let cell = (area.width - C.card_gap * 4) / 5;
    for (i, label) in STEPS.iter().enumerate() {
        let rect = Rect {
            x: area.x + i as i32 * (cell + C.card_gap),
            y: C.header_height - Controls::MIN_HEIGHT,
            width: cell,
            height: Controls::MIN_HEIGHT,
        };
        fill(
            pm,
            rect,
            if i == step {
                Interaction::DEFAULT.selected_tint(p.surface)
            } else {
                p.surface
            },
            Radius::DEFAULT.sm,
        );
        outline(
            pm,
            rect,
            if i == step {
                p.accent
            } else {
                p.border_subtle()
            },
            Controls::BORDER,
        );
        paint_text_centered(
            pm,
            label,
            rect,
            Typography::DEFAULT.caption_size as f32,
            if i == step { p.text } else { p.text_dim },
        );
    }
}

fn footer(pm: &mut tiny_skia::PixmapMut<'_>, wizard: &Wizard, config: &niwoe_config::ThemeConfig) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let area = layout::content(pm.width());
    let base = wizard.control_count() - 5;
    let labels = [
        "Zurück",
        if wizard.draft.step == 4 {
            "Einrichtung übernehmen"
        } else {
            "Weiter"
        },
        "Überspringen",
        "Schließen · Entwurf behalten",
        if wizard.state.is_none() || wizard.busy() {
            "Zustand neu laden"
        } else if wizard.state.as_ref().is_some_and(|s| s.applying) {
            if wizard.confirm_discard {
                "Restauftrag jetzt verwerfen"
            } else {
                "Restauftrag verwerfen …"
            }
        } else {
            "Entwurf verwerfen"
        },
    ];
    for (offset, label) in labels.iter().enumerate() {
        native_control(pm, wizard, base + offset, label, config);
    }
    let message = if wizard.message.is_empty() {
        "Deine Auswahl wird erst mit „Einrichtung übernehmen“ angewendet."
    } else {
        &wizard.message
    };
    paint_text(
        pm,
        &truncate_to_fit(message, area.width, Typography::DEFAULT.caption_size as f32),
        area.x,
        pm.height() as i32 - C.config_footer_height - S.xl,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
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
    heading(&mut pm, wizard, config);
    body::draw(&mut pm, wizard, preview, config);
    footer(&mut pm, wizard, config);
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
