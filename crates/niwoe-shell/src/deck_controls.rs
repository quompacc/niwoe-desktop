use super::*;
use crate::panel_view::status_symbols::{self, Symbol};
use niwoe_tokens::{Controls, Spacing};

pub(super) fn draw(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    rect: Rect,
    state: &QuickSettingsState<'_>,
) {
    let (network_label, network_detail, network_symbol) = match state.network {
        NetworkState::Connected {
            kind: ConnectionKind::Wifi { .. },
            ..
        } => ("WLAN", "Verbunden", Symbol::Wifi(true)),
        NetworkState::Connected { .. } => ("Netzwerk", "Verbunden", Symbol::Wired(true)),
        NetworkState::Disconnected => ("WLAN", "Getrennt", Symbol::Wifi(false)),
        NetworkState::Offline => ("WLAN", "Fehlt", Symbol::Wifi(false)),
    };
    let bluetooth_ready = state.bluetooth.adapter_present && !state.bluetooth_pending;
    let bluetooth_detail = if state.bluetooth_pending {
        "Lädt …"
    } else if !state.bluetooth.adapter_present {
        "Fehlt"
    } else if state.bluetooth.powered {
        "Ein"
    } else {
        "Aus"
    };
    let profile_detail = match (state.power_status, state.power_profile) {
        (crate::deck_mutation::Status::Pending, _) => "Ändert …",
        (crate::deck_mutation::Status::Failed, _) => "Fehler",
        (_, Some(PowerProfile::Eco)) => "Eco",
        (_, Some(PowerProfile::Standard)) => "Standard",
        (_, Some(PowerProfile::Performance)) => "Maximal",
        (_, None) => "Fehlt",
    };
    let entries = [
        (
            network_label,
            network_detail,
            network_symbol,
            !matches!(state.network, NetworkState::Offline),
            &NETWORK,
        ),
        (
            "Bluetooth",
            bluetooth_detail,
            Symbol::Bluetooth,
            bluetooth_ready,
            &BLUETOOTH,
        ),
        ("Anzeige", "Öffnen", Symbol::Display, true, &DISPLAY),
        (
            "Leistung",
            profile_detail,
            Symbol::Performance,
            state.power_profile.is_some()
                && state.power_status != crate::deck_mutation::Status::Pending,
            &POWER_PROFILE,
        ),
    ];
    let gap = QuickSettings::DEFAULT.tile_gap;
    let count = entries.len() as i32;
    let width = (rect.w - gap * (count - 1)) / count;
    let size = Controls::MIN_HEIGHT + Spacing::DEFAULT.sm;
    for (index, (label, detail, symbol, enabled, slot)) in entries.into_iter().enumerate() {
        let cell = Rect {
            x: rect.x + index as i32 * (width + gap),
            w: width,
            ..rect
        };
        let circle = Rect {
            x: cell.x + (cell.w - size) / 2,
            y: cell.y,
            w: size,
            h: size,
        };
        painter.roundish_rect_with_radius(circle, glass_border_from_config(theme), size / 2);
        let border = Controls::BORDER;
        painter.roundish_rect_with_radius(
            Rect {
                x: circle.x + border,
                y: circle.y + border,
                w: circle.w - border * 2,
                h: circle.h - border * 2,
            },
            theme.colors.surface,
            size / 2,
        );
        draw_symbol(painter, theme, circle, symbol, enabled);
        for (value, baseline, dim) in [
            (label, circle.y + size + Spacing::DEFAULT.lg, !enabled),
            (detail, circle.y + size + Spacing::DEFAULT.xxl, true),
        ] {
            let x = cell.x + (cell.w - measure(font, value)).max(0) / 2;
            painter.text_clipped(
                font,
                value,
                x,
                baseline,
                cell.x + cell.w - x,
                if dim {
                    glass_dim_from_config(theme)
                } else {
                    glass_foreground_from_config(theme)
                },
            );
        }
        slot.with(|slot| slot.set(if enabled { cell } else { ZERO_RECT }));
    }
    draw_panel(painter, theme, rect);
}

pub(super) fn draw_symbol(
    painter: &mut Painter<'_>,
    theme: &ThemeConfig,
    rect: Rect,
    symbol: Symbol,
    enabled: bool,
) {
    let color = if enabled {
        glass_foreground_from_config(theme)
    } else {
        glass_dim_from_config(theme)
    };
    // Painter stores premultiplied BGRA. Swap the artwork color before using
    // tiny-skia's premultiplied RGBA compositor on that buffer. The existing
    // bounded icon cache also keys this color; no decode/conversion per frame.
    let color = niwoe_config::Color {
        r: color.b,
        b: color.r,
        ..color
    };
    let Some(icon) = status_symbols::icon(symbol, color) else {
        return;
    };
    let Some(mut target) =
        tiny_skia::PixmapMut::from_bytes(painter.data, painter.width as u32, painter.height as u32)
    else {
        return;
    };
    target.draw_pixmap(
        rect.x + (rect.w - icon.width() as i32) / 2,
        rect.y + (rect.h - icon.height() as i32) / 2,
        icon.as_ref(),
        &tiny_skia::PixmapPaint::default(),
        tiny_skia::Transform::identity(),
        None,
    );
}
