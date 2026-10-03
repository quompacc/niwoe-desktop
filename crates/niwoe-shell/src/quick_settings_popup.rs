use std::cell::{Cell, RefCell};

use niwoe_config::ThemeConfig;
use niwoe_tokens::{Interaction, QuickSettings};

use crate::{
    audio::AudioSnapshot,
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
    Room,
    Logout,
    Network,
    Bluetooth,
    Display,
    AudioMute,
    Volume(u8),
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
    static BLUETOOTH: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static DISPLAY: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static AUDIO_MUTE: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static VOLUME: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static ROOM: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static POWER_PROFILE: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static LOCK: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static LOGOUT: Cell<Rect> = const { Cell::new(ZERO_RECT) };
    static POWER_OFF: Cell<Rect> = const { Cell::new(ZERO_RECT) };
}

pub struct QuickSettingsState<'a> {
    pub network: &'a NetworkState,
    pub audio: &'a AudioSnapshot,
    pub bluetooth: &'a crate::bluetooth::BluetoothSnapshot,
    pub bluetooth_pending: bool,
    pub volume_preview: Option<u8>,
    pub audio_status: crate::deck_mutation::Status,
    pub power_status: crate::deck_mutation::Status,
    pub power_profile: Option<PowerProfile>,
    pub room_name: &'a str,
    pub power_armed: bool,
    pub logout_armed: bool,
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
    text(
        painter,
        font,
        theme,
        "System Deck",
        q.outer_pad,
        gap.xxl,
        false,
    );
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
    let audio_ready =
        device.is_some() && state.audio_status != crate::deck_mutation::Status::Pending;
    let mute = Rect {
        x: audio.x,
        y: audio.y,
        w: niwoe_tokens::Controls::MIN_HEIGHT,
        h: niwoe_tokens::Controls::MIN_HEIGHT,
    };
    deck_controls::draw_symbol(
        painter,
        theme,
        mute,
        crate::panel_view::status_symbols::Symbol::Audio(muted),
        audio_ready,
    );
    AUDIO_MUTE.with(|slot| slot.set(if audio_ready { mute } else { ZERO_RECT }));
    painter.text_clipped(
        font,
        &match state.audio_status {
            _ if state.volume_preview.is_some() => "Loslassen zum Ãœbernehmen".into(),
            crate::deck_mutation::Status::Pending => "Wird geändert …".into(),
            crate::deck_mutation::Status::Failed => "Nicht übernommen – erneut versuchen".into(),
            crate::deck_mutation::Status::Idle => device
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "Nicht verfügbar".into()),
        },
        audio.x,
        audio.y + mute.h + gap.lg,
        audio.w,
        glass_dim_from_config(theme),
    );
    let volume = state
        .volume_preview
        .or_else(|| device.and_then(|d| d.volume_percent));
    let slider = Rect {
        x: mute.x + mute.w + gap.md,
        y: audio.y,
        w: audio.w - mute.w - gap.md * 2 - niwoe_tokens::Controls::MIN_HEIGHT - gap.lg,
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
    let controls = Rect {
        x: q.outer_pad,
        y: tile_y,
        w: width,
        h: q.tile_height,
    };
    deck_controls::draw(painter, font, theme, controls, &state);
    let status = Rect {
        x: q.outer_pad,
        y: tile_y + q.tile_height + q.section_gap,
        w: width,
        h: q.status_height,
    };
    draw_panel(painter, theme, status);
    draw_status_cell(
        painter,
        font,
        theme,
        status,
        "Aktueller Raum",
        state.room_name,
    );
    text(
        painter,
        font,
        theme,
        "›",
        status.x + status.w - gap.xl,
        status.y + gap.xl,
        false,
    );
    ROOM.with(|slot| slot.set(status));
    let action_width = (width - q.tile_gap * 2) / 3;
    let lock = Rect {
        x: q.outer_pad,
        y: status.y + status.h + q.section_gap,
        w: action_width,
        h: q.footer_height,
    };
    let logout = Rect {
        x: lock.x + lock.w + q.tile_gap,
        ..lock
    };
    let power = Rect {
        x: logout.x + logout.w + q.tile_gap,
        w: width - action_width * 2 - q.tile_gap * 2,
        ..logout
    };
    draw_control(painter, font, theme, lock, "Sperren", false);
    draw_control(
        painter,
        font,
        theme,
        logout,
        if state.logout_armed {
            "Bestätigen"
        } else {
            "Abmelden"
        },
        state.logout_armed,
    );
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
    LOGOUT.with(|slot| slot.set(logout));
    POWER_OFF.with(|slot| slot.set(power));
    let settings = Rect {
        x: q.outer_pad,
        y: lock.y + lock.h + q.section_gap,
        w: width,
        h: q.footer_height,
    };
    draw_control(
        painter,
        font,
        theme,
        settings,
        "Systemeinstellungen …",
        false,
    );
    SETTINGS.with(|slot| slot.set(settings));
    draw_keyboard_focus(painter, theme);
}

fn draw_status_cell(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    rect: Rect,
    label: &str,
    value: &str,
) {
    let gap = niwoe_tokens::Spacing::DEFAULT;
    text(
        painter,
        font,
        theme,
        label,
        rect.x + gap.md,
        rect.y + gap.lg,
        false,
    );
    painter.text_clipped(
        font,
        value,
        rect.x + gap.md,
        rect.y + gap.lg * 2,
        rect.w - gap.md * 2 - gap.xl,
        glass_dim_from_config(theme),
    );
}

fn draw_panel(painter: &mut Painter<'_>, theme: &ThemeConfig, rect: Rect) {
    let border = glass_border_from_config(theme);
    painter.rect(
        Rect {
            x: rect.x,
            y: rect.y + rect.h - niwoe_tokens::Controls::BORDER,
            w: rect.w,
            h: niwoe_tokens::Controls::BORDER,
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
        (&ROOM, QuickSettingsHit::Room),
        (&NETWORK, QuickSettingsHit::Network),
        (&BLUETOOTH, QuickSettingsHit::Bluetooth),
        (&DISPLAY, QuickSettingsHit::Display),
        (&AUDIO_MUTE, QuickSettingsHit::AudioMute),
        (&POWER_PROFILE, QuickSettingsHit::PowerProfile),
        (&LOCK, QuickSettingsHit::Lock),
        (&LOGOUT, QuickSettingsHit::Logout),
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

#[path = "deck_controls.rs"]
mod deck_controls;

include!("deck_tests.rs");
