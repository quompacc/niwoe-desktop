//! Typed content groups. Existing Wizard actions own every draft mutation.
use super::*;
use niwoe_ui::{
    effect::{paint_text_pair, Symbol},
    widget::{ComponentState, SelectionKind, SelectionRow},
    Widget,
};

fn caption(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    text: &str,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    paint_text(
        pm,
        text,
        area.x,
        C.header_height + S.xl,
        Typography::DEFAULT.title_size as f32,
        p.text,
    );
}

fn note(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    y: i32,
    text: &str,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    paint_text(
        pm,
        &truncate_to_fit(text, area.width, Typography::DEFAULT.caption_size as f32),
        area.x,
        y,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
}

#[allow(clippy::too_many_arguments)] // Borrowed text and independent control state; no render-owned model.
fn choice(
    pm: &mut tiny_skia::PixmapMut<'_>,
    wizard: &Wizard,
    index: usize,
    title: &str,
    subtitle: &str,
    selected: bool,
    kind: SelectionKind,
    config: &niwoe_config::ThemeConfig,
) {
    let area = layout::control(
        pm.width(),
        pm.height(),
        index,
        wizard.control_count(),
        wizard.draft.step,
    );
    SelectionRow {
        title,
        subtitle,
        width: area.width,
        kind,
        state: ComponentState {
            selected,
            disabled: !wizard.enabled(index),
            focused: wizard.focus == index,
            ..Default::default()
        },
    }
    .paint(
        area,
        pm,
        &crate::ui::tokens::theme_from_config(config),
        niwoe_ui::WidgetState::Idle,
    );
}

fn information(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    symbol: Symbol,
    title: &str,
    detail: &str,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(pm, area, p.surface, Radius::DEFAULT.sm);
    outline(pm, area, p.border_subtle(), Controls::BORDER);
    let icon = Rect {
        x: area.x + S.lg,
        width: S.xxl,
        ..area
    };
    list::symbol(pm, icon, symbol, p.text_dim);
    let x = icon.x + icon.width + S.lg;
    paint_text_pair(
        pm,
        Rect {
            x,
            width: area.x + area.width - x - S.lg,
            ..area
        },
        title,
        detail,
        p.text,
        p.text_dim,
    );
}

fn welcome(pm: &mut tiny_skia::PixmapMut<'_>, config: &niwoe_config::ThemeConfig) {
    let area = layout::content(pm.width());
    caption(pm, area, "Dein Desktop, Schritt für Schritt", config);
    for (row, (symbol, title, detail)) in [
        (
            Symbol::User,
            "Bedienprofil",
            "Maus und Tastatur führen zu denselben Funktionen.",
        ),
        (
            Symbol::Room,
            "Räume",
            "Ordne Anwendungen nach deinem Arbeitskontext.",
        ),
        (
            Symbol::Panel,
            "Leiste",
            "Wähle die Module, die du im Alltag brauchst.",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        information(
            pm,
            Rect {
                y: layout::body_top() + row as i32 * (layout::W.information_height + C.card_gap),
                height: layout::W.information_height,
                ..area
            },
            symbol,
            title,
            detail,
            config,
        );
    }
    note(pm,area,layout::body_top()+3*(layout::W.information_height+C.card_gap)+S.xl,"Du kannst jederzeit schließen und später im Control Center → System → Übersicht fortsetzen.",config);
}

fn profile(pm: &mut tiny_skia::PixmapMut<'_>, wizard: &Wizard, config: &niwoe_config::ThemeConfig) {
    let area = layout::content(pm.width());
    caption(pm, area, "Wie möchtest du beginnen?", config);
    let selected = match wizard.draft.profile {
        niwoe_ipc::InteractionProfile::Preserve => 0,
        niwoe_ipc::InteractionProfile::Mouse => 1,
        niwoe_ipc::InteractionProfile::Keyboard => 2,
    };
    for (i, (title, detail)) in [
        (
            "Eigene Einstellungen erhalten",
            "Empfohlen für einen bereits eingerichteten Desktop.",
        ),
        (
            "Mit der Maus beginnen",
            "Bei einer neuen Leiste ist das Suchmodul sichtbar.",
        ),
        (
            "Mit der Tastatur beginnen",
            "Bei einer neuen Leiste ist das Suchmodul ausgeblendet; Super+Space öffnet den Hub.",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        choice(
            pm,
            wizard,
            i,
            title,
            detail,
            selected == i,
            SelectionKind::Radio,
            config,
        );
    }
    note(pm,area,layout::row_y(3)+S.lg,"Die Profilwahl setzt nur Startwerte für eine neue Leiste. Vorhandene Anpassungen haben Vorrang.",config);
}

fn rooms(pm: &mut tiny_skia::PixmapMut<'_>, wizard: &Wizard, config: &niwoe_config::ThemeConfig) {
    let left = layout::column(pm.width(), false);
    let right = layout::column(pm.width(), true);
    caption(pm, left, "Optionale Räume", config);
    caption(pm, right, "Vorhandene Apps", config);
    for (i, (title, description)) in crate::first_run::SUGGESTIONS.iter().enumerate() {
        let detail = if wizard.existing_suggestions[i] {
            "Bereits vorhanden · bleibt unverändert"
        } else if !wizard.room_selected(title) && wizard.draft.rooms.len() >= wizard.room_capacity {
            "Kein Platz für weitere Räume"
        } else {
            description
        };
        choice(
            pm,
            wizard,
            i,
            title,
            detail,
            wizard.room_selected(title),
            SelectionKind::Checkbox,
            config,
        );
    }
    native_control(
        pm,
        wizard,
        3,
        &format!(
            "Apps für: {} · Wechseln",
            crate::first_run::SUGGESTIONS[wizard.app_target].0
        ),
        config,
    );
    for (row, (label, reference)) in wizard
        .apps
        .iter()
        .skip(wizard.app_page * 4)
        .take(4)
        .enumerate()
    {
        let (name, detail) = match reference {
            niwoe_ipc::AppReference::Native(_) => (
                label.strip_suffix(" · Native").unwrap_or(label),
                "Native App",
            ),
            niwoe_ipc::AppReference::Xwayland(_) => (
                label.strip_suffix(" · XWayland").unwrap_or(label),
                "XWayland-Fenster",
            ),
        };
        let selected = wizard.draft.rooms.iter().any(|r| matches!(r,niwoe_ipc::RoomChange::Configure { name,preferences,.. } if name==crate::first_run::SUGGESTIONS[wizard.app_target].0 && preferences.apps.contains(reference)));
        choice(
            pm,
            wizard,
            6 + row,
            name,
            detail,
            selected,
            SelectionKind::Checkbox,
            config,
        );
    }
    if wizard.apps.is_empty() {
        note(
            pm,
            right,
            layout::body_top() + C.config_field_height + S.xxl,
            "Keine vorhandenen Apps im Katalog.",
            config,
        );
    }
    native_control(pm, wizard, 4, "Vorige Apps", config);
    native_control(
        pm,
        wizard,
        5,
        &format!(
            "Weitere Apps · {} / {}",
            wizard.app_page + 1,
            wizard.apps.len().saturating_sub(1) / 4 + 1
        ),
        config,
    );
    note(
        pm,
        left,
        layout::row_y(3) + S.lg,
        "Wähle zuerst den Raum für die App-Zuordnung.",
        config,
    );
    note(
        pm,
        left,
        layout::row_y(3) + S.lg + S.xl,
        "Die Auswahl bevorzugt den Raum bei späteren Appstarts.",
        config,
    );
    note(
        pm,
        right,
        layout::control(pm.width(), pm.height(), 5, wizard.control_count(), 2).y
            + C.config_field_height
            + S.xl,
        "Apps werden weder installiert noch automatisch gestartet.",
        config,
    );
}

fn panel(
    pm: &mut tiny_skia::PixmapMut<'_>,
    wizard: &Wizard,
    preview: Option<&Pixmap>,
    config: &niwoe_config::ThemeConfig,
) {
    let area = layout::content(pm.width());
    caption(pm, area, "Module und Reihenfolge", config);
    let modules = wizard.draft.panel.as_deref().unwrap_or(&[]);
    for (row, module) in crate::room_editor::panel::MODULES.into_iter().enumerate() {
        let position = modules.iter().position(|entry| *entry == module);
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
        choice(
            pm,
            wizard,
            row * 3,
            crate::room_editor::panel::label(module),
            &detail,
            position.is_some(),
            SelectionKind::Checkbox,
            config,
        );
        native_control(pm, wizard, row * 3 + 1, "Früher", config);
        native_control(pm, wizard, row * 3 + 2, "Später", config);
    }
    let p = crate::ui::tokens::theme_from_config(config).palette;
    paint_text(
        pm,
        "LEISTENVORSCHAU · ORIGINALGRÖSSE",
        area.x,
        layout::preview_y() - S.md,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    if let Some(preview) = preview {
        pm.draw_pixmap(
            area.x,
            layout::preview_y(),
            preview.as_ref(),
            &Default::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    }
    note(pm,area,layout::preview_y()+crate::PANEL_SURFACE_HEIGHT as i32+S.xl,"Die obere Leiste und diese Vorschau zeigen den Entwurf. Schließen oder Überspringen nimmt ihn zurück.",config);
}

fn ready(pm: &mut tiny_skia::PixmapMut<'_>, wizard: &Wizard, config: &niwoe_config::ThemeConfig) {
    let area = layout::content(pm.width());
    caption(pm, area, "Optional ausprobieren", config);
    let p = crate::ui::tokens::theme_from_config(config).palette;
    for (index, (symbol, title, detail)) in [
        (
            Symbol::Home,
            "Hub öffnen",
            "Super+Space · oder Hub-Schaltfläche in der Leiste",
        ),
        (
            Symbol::Room,
            "Ersten Raum wählen",
            "Super+1 · oder den ersten Raum in der Leiste wählen",
        ),
        (
            Symbol::System,
            "Systemdeck öffnen",
            "Super+Escape · oder Systemstatus in der Leiste",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let rect = layout::control(pm.width(), pm.height(), index, wizard.control_count(), 4);
        native_control(pm, wizard, index, "", config);
        let icon = Rect {
            x: rect.x + S.sm,
            width: S.xxl,
            ..rect
        };
        list::symbol(
            pm,
            icon,
            symbol,
            if wizard.enabled(index) {
                p.text_dim
            } else {
                p.text_disabled()
            },
        );
        let x = icon.x + icon.width + S.sm;
        paint_text_pair(
            pm,
            Rect {
                x,
                width: rect.width - (x - rect.x) - C.config_action_width - S.sm,
                ..rect
            },
            title,
            detail,
            if wizard.enabled(index) {
                p.text
            } else {
                p.text_disabled()
            },
            p.text_dim,
        );
        paint_text_centered(
            pm,
            if wizard.draft.practiced[index] {
                "Ausprobiert"
            } else {
                "Öffnen"
            },
            Rect {
                x: rect.x + rect.width - C.config_action_width,
                width: C.config_action_width,
                ..rect
            },
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
    }
    let profile = match wizard.draft.profile {
        niwoe_ipc::InteractionProfile::Preserve => "Eigene Einstellungen erhalten",
        niwoe_ipc::InteractionProfile::Mouse => "Mit der Maus beginnen",
        niwoe_ipc::InteractionProfile::Keyboard => "Mit der Tastatur beginnen",
    };
    let summary = format!(
        "{} zusätzliche Räume · {}",
        wizard.draft.rooms.len(),
        if wizard.draft.panel.is_some() {
            "Leistenentwurf"
        } else {
            "Leiste unverändert"
        }
    );
    information(
        pm,
        Rect {
            y: layout::row_y(3) + S.xl,
            height: layout::W.information_height,
            ..area
        },
        Symbol::List,
        "Deine Auswahl",
        &format!("{profile} · {summary}"),
        config,
    );
    note(pm,area,layout::row_y(3)+S.xl+layout::W.information_height+S.xl,"Übernehmen speichert deine Auswahl. Bestehende Räume bleiben erhalten; es werden keine Apps gestartet.",config);
    note(pm,area,layout::row_y(3)+S.xl+layout::W.information_height+S.xl*2,"Zurück zur Einführung: Control Center → System → Übersicht. Eigene Tastenkürzel haben Vorrang.",config);
}

pub(super) fn draw(
    pm: &mut tiny_skia::PixmapMut<'_>,
    wizard: &Wizard,
    preview: Option<&Pixmap>,
    config: &niwoe_config::ThemeConfig,
) {
    match wizard.draft.step {
        0 => welcome(pm, config),
        1 => profile(pm, wizard, config),
        2 => rooms(pm, wizard, config),
        3 => panel(pm, wizard, preview, config),
        _ => ready(pm, wizard, config),
    }
}
