//! Local draft controls. Cached icons only; cycling never persists or launches.
use super::*;
use niwoe_ui::effect::{paint_text_pair, Symbol};

pub(crate) fn draw_general(
    pm: &mut tiny_skia::PixmapMut<'_>,
    edit: &Edit,
    scroll: i32,
    icons: &crate::icons::IconCache,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let symbol = rect(pm.width(), scroll, 9);
    native_control(pm, symbol, "", edit.focus == 9, true, config);
    if let Some(image) = edit
        .preferences
        .icon
        .as_deref()
        .and_then(|id| icons.lookup(id, S.xxl as u32))
        .and_then(crate::icons::icon_image_to_pixmap)
    {
        pm.draw_pixmap(
            symbol.x + (symbol.width - image.width() as i32) / 2,
            symbol.y + (symbol.height - image.height() as i32) / 2,
            image.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    } else {
        super::super::super::list::symbol(pm, symbol, Symbol::Room, p.text_dim);
    }
    paint_text_centered(
        pm,
        "Symbol",
        Rect {
            y: symbol.y + symbol.height,
            height: S.xl,
            ..symbol
        },
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );

    let assignment = match edit.assignment {
        niwoe_ipc::RoomAssignment::Free => "Frei · alle Apps willkommen",
        niwoe_ipc::RoomAssignment::Preferred => {
            "Bevorzugt · ausgewählte Apps bevorzugen diesen Raum"
        }
        niwoe_ipc::RoomAssignment::Dedicated => "Dediziert · andere Apps bleiben erlaubt",
    };
    let restore = match edit.preferences.restore {
        niwoe_ipc::RoomRestore::Disabled => "Keine Vorgabe · manuell auslösen",
        niwoe_ipc::RoomRestore::LayoutOnly => "Nur Fenster anordnen · manuell auslösen",
        niwoe_ipc::RoomRestore::RelaunchApps => "Apps wieder öffnen · manuell bestätigen",
    };
    for (index, artwork, title, detail, hint, enabled) in [
        (
            10,
            Symbol::App,
            "App-Zuordnung",
            assignment,
            "Nächste Option",
            true,
        ),
        (
            11,
            Symbol::History,
            "Wiederherstellung",
            restore,
            "Nächste Option",
            true,
        ),
        (
            8,
            Symbol::Grid,
            "Gespeichertes Layout",
            "Fensterpositionen und Wiederöffnen verwalten",
            "Öffnen",
            edit.id != 0,
        ),
    ] {
        let area = rect(pm.width(), scroll, index);
        native_control(pm, area, "", edit.focus == index, enabled, config);
        let icon = Rect {
            x: area.x + S.sm,
            width: S.xl,
            ..area
        };
        let color = if enabled { p.text } else { p.text_disabled() };
        super::super::super::list::symbol(pm, icon, artwork, color);
        let action = Rect {
            x: area.x + area.width - C.config_action_width,
            width: C.config_action_width - S.sm,
            ..area
        };
        let x = icon.x + icon.width + S.sm;
        paint_text_pair(
            pm,
            Rect {
                x,
                width: action.x - x - S.sm,
                ..area
            },
            title,
            if enabled {
                detail
            } else {
                "Nach dem ersten Speichern verfügbar"
            },
            color,
            if enabled {
                p.text_dim
            } else {
                p.text_disabled()
            },
        );
        paint_text_centered(
            pm,
            hint,
            action,
            Typography::DEFAULT.caption_size as f32,
            if enabled { p.accent } else { p.text_disabled() },
        );
    }
    for (index, label) in [(12, "Apps auswählen"), (13, "Dateiverweise verwalten")] {
        native_control(
            pm,
            rect(pm.width(), scroll, index),
            label,
            edit.focus == index,
            index == 12 || edit.id != 0,
            config,
        );
    }
    if !edit.name_error.is_empty() {
        let name = name_rect(pm.width(), scroll);
        outline(pm, name, p.error, Controls::FOCUS_WIDTH);
        paint_text(
            pm,
            &truncate_to_fit(
                &edit.name_error,
                name.width,
                Typography::DEFAULT.caption_size as f32,
            ),
            name.x,
            name.y + name.height + S.md,
            Typography::DEFAULT.caption_size as f32,
            p.error,
        );
    }
}

pub(super) fn native_control(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    label: &str,
    focused: bool,
    enabled: bool,
    config: &niwoe_config::ThemeConfig,
) {
    super::super::super::list::control(
        pm,
        area,
        label,
        super::super::super::list::ControlState {
            enabled,
            focused,
            ..Default::default()
        },
        config,
    );
}
