use std::cell::{Cell, RefCell};

use niwoe_config::ThemeConfig;
use niwoe_tokens::{Interaction, QuickSettings, Radius};

use crate::{
    audio::AudioSnapshot,
    battery::BatterySnapshot,
    network::{ConnectionKind, NetworkState},
    popup_card::draw_card_body,
    power_profile::PowerProfile,
    ui::tokens::{glass_border_from_config, glass_dim_from_config, glass_foreground_from_config},
    Painter, Rect, TextRenderer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickSettingsHit {
    Card,
    Settings,
    Network,
    AudioMute,
    Volume(u8),
    Appearance,
    PowerProfile,
    Lock,
    PowerOff,
}

const ZERO_RECT: Rect = Rect {
    x: 0,
    y: 0,
    w: 0,
    h: 0,
};

thread_local! {
    static SETTINGS: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static NETWORK: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static AUDIO_MUTE: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static VOLUME: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static APPEARANCE: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static POWER_PROFILE: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static LOCK: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static POWER_OFF: Cell<Rect> = const { Cell::new(ZERO_RECT) };
}

pub struct QuickSettingsState<'a> {
    pub network: &'a NetworkState,
    pub audio: &'a AudioSnapshot,
    pub battery: &'a BatterySnapshot,
    pub power_profile: Option<PowerProfile>,
    pub theme_name: &'a str,
    pub power_armed: bool,
}

pub fn draw(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    state: QuickSettingsState<'_>,
) {
    let q = QuickSettings::DEFAULT;
    let gap = niwoe_tokens::Spacing::DEFAULT;
    draw_card_body(painter, theme);
    text(painter, font, theme, "System", q.outer_pad, gap.xxl, false);
    let settings = Rect {
        x: q.width - q.outer_pad - q.footer_height * 3,
        y: gap.md,
        w: q.footer_height * 3,
        h: niwoe_tokens::Controls::MIN_HEIGHT,
    };
    draw_control(painter, font, theme, settings, "Einstellungen", false);
    SETTINGS.with(|slot| slot.set(settings));
    let width = q.width - q.outer_pad * 2;
    let audio = Rect {
        x: q.outer_pad,
        y: q.header_height + q.section_gap,
        w: width,
        h: q.audio_height,
    };
    draw_panel(painter, theme, audio);
    let device = state.audio.default_output.as_ref();
    let muted = device.is_none_or(|d| d.muted);
    let mute = Rect {
        x: audio.x + gap.md,
        y: audio.y + gap.md,
        w: niwoe_tokens::Controls::MIN_HEIGHT * 2,
        h: niwoe_tokens::Controls::MIN_HEIGHT,
    };
    draw_control(
        painter,
        font,
        theme,
        mute,
        if muted { "Stumm" } else { "Ton an" },
        muted,
    );
    AUDIO_MUTE.with(|slot| slot.set(if device.is_some() { mute } else { ZERO_RECT }));
    let tx = mute.x + mute.w + gap.md;
    text(
        painter,
        font,
        theme,
        "Lautstärke",
        tx,
        audio.y + gap.xl,
        false,
    );
    text(
        painter,
        font,
        theme,
        &device
            .map(|d| fit(&d.name, 24))
            .unwrap_or_else(|| "Nicht verfügbar".into()),
        tx,
        audio.y + gap.xl + gap.lg,
        true,
    );
    let volume = device.and_then(|d| d.volume_percent);
    let slider = Rect {
        x: audio.x + gap.lg,
        y: audio.y + q.audio_height - gap.xxl - gap.md,
        w: audio.w - gap.lg * 2 - niwoe_tokens::Controls::MIN_HEIGHT * 2,
        h: niwoe_tokens::Controls::MIN_HEIGHT,
    };
    draw_slider(painter, theme, slider, volume.unwrap_or(0), muted);
    VOLUME.with(|slot| slot.set(if volume.is_some() { slider } else { ZERO_RECT }));
    text(
        painter,
        font,
        theme,
        &volume
            .map(|v| format!("{v}%"))
            .unwrap_or_else(|| "—".into()),
        slider.x + slider.w + gap.md,
        slider.y + gap.xl,
        false,
    );
    let tile_y = audio.y + audio.h + q.section_gap;
    let tile = Rect {
        x: q.outer_pad,
        y: tile_y,
        w: (width - q.tile_gap) / 2,
        h: q.tile_height,
    };
    let profile = Rect {
        x: tile.x + tile.w + q.tile_gap,
        ..tile
    };
    draw_network_tile(painter, font, theme, tile, state.network);
    draw_profile_tile(painter, font, theme, profile, state.power_profile);
    NETWORK.with(|slot| {
        slot.set(if matches!(state.network, NetworkState::Offline) {
            ZERO_RECT
        } else {
            tile
        })
    });
    POWER_PROFILE.with(|slot| {
        slot.set(if state.power_profile.is_some() {
            profile
        } else {
            ZERO_RECT
        })
    });
    let status = Rect {
        x: q.outer_pad,
        y: tile_y + q.tile_height + q.section_gap,
        w: width,
        h: q.status_height,
    };
    draw_panel(painter, theme, status);
    let appearance = Rect {
        w: status.w / 2,
        ..status
    };
    draw_status_cell(
        painter,
        font,
        theme,
        appearance,
        "Darstellung",
        if state.theme_name.to_ascii_lowercase().contains("dark") {
            "Dunkel"
        } else {
            "Hell"
        },
    );
    APPEARANCE.with(|slot| slot.set(appearance));
    let battery = Rect {
        x: status.x + appearance.w,
        w: status.w - appearance.w,
        ..status
    };
    draw_status_cell(
        painter,
        font,
        theme,
        battery,
        "Energie",
        &if state.battery.present {
            format!("{}% Akku", state.battery.capacity)
        } else {
            "Netzbetrieb".into()
        },
    );
    let lock = Rect {
        x: q.outer_pad,
        y: status.y + status.h + q.section_gap,
        w: (width - q.tile_gap) / 2,
        h: q.footer_height,
    };
    let power = Rect {
        x: lock.x + lock.w + q.tile_gap,
        ..lock
    };
    draw_control(painter, font, theme, lock, "Sperren", false);
    draw_control(
        painter,
        font,
        theme,
        power,
        if state.power_armed {
            "Bestätigen"
        } else {
            "Ausschalten"
        },
        state.power_armed,
    );
    LOCK.with(|slot| slot.set(lock));
    POWER_OFF.with(|slot| slot.set(power));
    draw_keyboard_focus(painter, theme);
}

fn draw_network_tile(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    rect: Rect,
    network: &NetworkState,
) {
    draw_panel(painter, theme, rect);
    let (label, detail, active) = match network {
        NetworkState::Connected {
            kind,
            connection_name,
        } => {
            let label = match kind {
                ConnectionKind::Ethernet => "Ethernet",
                ConnectionKind::Wifi { .. } => "WLAN",
                ConnectionKind::Vpn => "VPN",
                ConnectionKind::Other => "Netzwerk",
            };
            (label, fit(connection_name, 19), true)
        }
        NetworkState::Disconnected => ("Netzwerk", "Getrennt".to_string(), false),
        NetworkState::Offline => ("Netzwerk", "Nicht verfügbar".to_string(), false),
    };
    let gap = niwoe_tokens::Spacing::DEFAULT;
    text(
        painter,
        font,
        theme,
        label,
        rect.x + gap.md,
        rect.y + gap.xl,
        false,
    );
    let color = glass_dim_from_config(theme);
    painter.text_clipped(
        font,
        &detail,
        rect.x + gap.md,
        rect.y + gap.xl + gap.xl,
        rect.w - gap.md * 2,
        color,
    );
    let _ = active;
}

fn draw_profile_tile(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    rect: Rect,
    profile: Option<PowerProfile>,
) {
    let gap = niwoe_tokens::Spacing::DEFAULT;
    draw_panel(painter, theme, rect);
    text(
        painter,
        font,
        theme,
        "Leistung",
        rect.x + gap.md,
        rect.y + gap.xl,
        false,
    );
    painter.text_clipped(
        font,
        profile
            .map(PowerProfile::label)
            .unwrap_or("Nicht verfügbar"),
        rect.x + gap.md,
        rect.y + gap.xl + gap.xl,
        rect.w - gap.md * 2,
        glass_dim_from_config(theme),
    );
}

fn draw_status_cell(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    rect: Rect,
    label: &str,
    value: &str,
) {
    text(painter, font, theme, label, rect.x + 14, rect.y + 28, false);
    text(painter, font, theme, value, rect.x + 14, rect.y + 49, true);
}

fn draw_panel(painter: &mut Painter<'_>, theme: &ThemeConfig, rect: Rect) {
    painter.roundish_rect_with_radius(rect, Interaction::DEFAULT.neutral_hover, Radius::DEFAULT.lg);
    let border = glass_border_from_config(theme);
    painter.rect(
        Rect {
            x: rect.x + Radius::DEFAULT.lg,
            y: rect.y,
            w: rect.w - Radius::DEFAULT.lg * 2,
            h: 1,
        },
        border,
    );
}

fn draw_control(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    rect: Rect,
    label: &str,
    active: bool,
) {
    painter.roundish_rect_with_radius(
        rect,
        if active {
            Interaction::DEFAULT.accent_idle(theme.colors.accent)
        } else {
            Interaction::DEFAULT.neutral_hover
        },
        QuickSettings::DEFAULT.control_radius,
    );
    let width = measure(font, label);
    text(
        painter,
        font,
        theme,
        label,
        rect.x + (rect.w - width) / 2,
        rect.y + (rect.h + 10) / 2,
        false,
    );
}

fn draw_slider(
    painter: &mut Painter<'_>,
    theme: &ThemeConfig,
    hit_rect: Rect,
    volume: u8,
    muted: bool,
) {
    let q = QuickSettings::DEFAULT;
    let bar = Rect {
        x: hit_rect.x,
        y: hit_rect.y + (hit_rect.h - q.slider_height) / 2,
        w: hit_rect.w,
        h: q.slider_height,
    };
    painter.roundish_rect_with_radius(bar, glass_border_from_config(theme), q.slider_height);
    let fill = Rect {
        w: bar.w * i32::from(volume.min(100)) / 100,
        ..bar
    };
    if fill.w > 0 {
        painter.roundish_rect_with_radius(
            fill,
            if muted {
                glass_dim_from_config(theme)
            } else {
                theme.colors.accent
            },
            q.slider_height,
        );
    }
    let thumb_size = q.slider_thumb_size;
    let thumb_center = bar.x + bar.w * i32::from(volume.min(100)) / 100;
    painter.roundish_rect_with_radius(
        Rect {
            x: (thumb_center - thumb_size / 2)
                .clamp(bar.x - thumb_size / 2, bar.x + bar.w - thumb_size / 2),
            y: hit_rect.y + (hit_rect.h - thumb_size) / 2,
            w: thumb_size,
            h: thumb_size,
        },
        if muted {
            glass_dim_from_config(theme)
        } else {
            theme.colors.accent
        },
        thumb_size,
    );
}

fn text(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    value: &str,
    x: i32,
    baseline: i32,
    dim: bool,
) {
    let color = if dim {
        glass_dim_from_config(theme)
    } else {
        glass_foreground_from_config(theme)
    };
    painter.text_clipped(font, value, x, baseline, painter.width - x - 8, color);
}

fn measure(font: &RefCell<Option<TextRenderer>>, value: &str) -> i32 {
    font.borrow_mut()
        .as_mut()
        .map(|renderer| renderer.measure_text(value))
        .unwrap_or(value.chars().count() as i32 * 8)
}

fn fit(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_string();
    }
    let mut out: String = value.chars().take(max.saturating_sub(1)).collect();
    out.push('\u{2026}');
    out
}

pub fn hit_test(width: u32, height: u32, x: f64, y: f64) -> Option<QuickSettingsHit> {
    if !(Rect {
        x: 0,
        y: 0,
        w: width as i32,
        h: height as i32,
    })
    .contains(x, y)
    {
        return None;
    }
    for (slot, hit) in [
        (&SETTINGS, QuickSettingsHit::Settings),
        (&NETWORK, QuickSettingsHit::Network),
        (&AUDIO_MUTE, QuickSettingsHit::AudioMute),
        (&APPEARANCE, QuickSettingsHit::Appearance),
        (&POWER_PROFILE, QuickSettingsHit::PowerProfile),
        (&LOCK, QuickSettingsHit::Lock),
        (&POWER_OFF, QuickSettingsHit::PowerOff),
    ] {
        if slot.with(Cell::get).contains(x, y) {
            return Some(hit);
        }
    }
    let volume = VOLUME.with(Cell::get);
    if volume.contains(x, y) {
        return Some(QuickSettingsHit::Volume(volume_from_slider_x(volume, x)));
    }
    Some(QuickSettingsHit::Card)
}

/// Map a card-local pointer x to the current slider value. During a drag the
/// pointer may leave the bar horizontally, so the value is clamped at 0/100.
pub fn volume_from_x(x: f64) -> Option<u8> {
    let volume = VOLUME.with(Cell::get);
    (volume.w > 0).then(|| volume_from_slider_x(volume, x))
}

fn volume_from_slider_x(volume: Rect, x: f64) -> u8 {
    let fraction = ((x - f64::from(volume.x)) / f64::from(volume.w)).clamp(0.0, 1.0);
    (fraction * 100.0).round() as u8
}

include!("deck_keyboard.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_controls_are_not_clickable_or_keyboard_targets() {
        let q = QuickSettings::DEFAULT;
        let mut canvas = vec![0; (q.width * q.height * 4) as usize];
        let font = RefCell::new(TextRenderer::new(
            "sans",
            niwoe_tokens::Typography::DEFAULT.body_size.into(),
        ));
        let theme = ThemeConfig::default();
        reset_keyboard_focus();
        draw(
            &mut Painter::new(&mut canvas, q.width, q.height),
            &font,
            &theme,
            QuickSettingsState {
                network: &NetworkState::Offline,
                audio: &AudioSnapshot::unavailable(),
                battery: &BatterySnapshot::default(),
                power_profile: None,
                theme_name: "dark",
                power_armed: false,
            },
        );
        let targets = focus_targets();
        assert!(!targets.iter().any(|(_, action)| matches!(
            action,
            QuickSettingsHit::AudioMute
                | QuickSettingsHit::Volume(_)
                | QuickSettingsHit::PowerProfile
                | QuickSettingsHit::Network
        )));
        for (rect, action) in &targets {
            assert!(
                rect.x >= 0
                    && rect.y >= 0
                    && rect.x + rect.w <= q.width
                    && rect.y + rect.h <= q.height
            );
            assert_eq!(
                hit_test(
                    q.width as u32,
                    q.height as u32,
                    f64::from(rect.x + rect.w / 2),
                    f64::from(rect.y + rect.h / 2)
                ),
                Some(*action)
            );
        }
        for (_, action) in &targets {
            focus_next(false);
            assert_eq!(focused_action(), Some(*action));
        }
        focus_next(false);
        assert_eq!(focused_action(), targets.first().map(|(_, action)| *action));
        if let Ok(path) = std::env::var("NIWOE_DECK_PREVIEW") {
            // The live compositor supplies the glass surface below this transparent layer.
            // Show the foreground against its theme tint in the standalone preview.
            let background = theme.glass_tint_color();
            for pixel in canvas.as_chunks_mut::<4>().0 {
                let inverse = 255 - u16::from(pixel[3]);
                for (channel, base) in
                    pixel[..3]
                        .iter_mut()
                        .zip([background.b, background.g, background.r])
                {
                    *channel = (u16::from(*channel) + u16::from(base) * inverse / 255) as u8;
                }
                pixel[3] = 255;
                pixel.swap(0, 2);
            }
            tiny_skia::Pixmap::from_vec(
                canvas,
                tiny_skia::IntSize::from_wh(q.width as u32, q.height as u32).unwrap(),
            )
            .unwrap()
            .save_png(path)
            .unwrap();
        }
        reset_keyboard_focus();
    }

    #[test]
    fn outside_is_not_a_hit() {
        assert_eq!(hit_test(384, 468, -1.0, 10.0), None);
    }

    #[test]
    fn slider_x_maps_and_clamps_to_percent() {
        let slider = Rect {
            x: 100,
            y: 0,
            w: 200,
            h: 28,
        };
        assert_eq!(volume_from_slider_x(slider, 100.0), 0);
        assert_eq!(volume_from_slider_x(slider, 200.0), 50);
        assert_eq!(volume_from_slider_x(slider, 300.0), 100);
        assert_eq!(volume_from_slider_x(slider, 40.0), 0);
        assert_eq!(volume_from_slider_x(slider, 500.0), 100);
    }
}
