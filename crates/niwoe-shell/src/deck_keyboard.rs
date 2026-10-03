// Included by quick_settings_popup: keyboard and pointer use the same hit slots.
thread_local! {
    static KEYBOARD_FOCUS: Cell<Option<QuickSettingsHit>> = const { Cell::new(None) };
}

fn focus_targets() -> Vec<(Rect, QuickSettingsHit)> {
    [
        (AUDIO_MUTE.with(Cell::get), QuickSettingsHit::AudioMute),
        (VOLUME.with(Cell::get), QuickSettingsHit::Volume(0)),
        (NETWORK.with(Cell::get), QuickSettingsHit::Network),
        (BLUETOOTH.with(Cell::get), QuickSettingsHit::Bluetooth),
        (DISPLAY.with(Cell::get), QuickSettingsHit::Display),
        (
            POWER_PROFILE.with(Cell::get),
            QuickSettingsHit::PowerProfile,
        ),
        (ROOM.with(Cell::get), QuickSettingsHit::Room),
        (LOCK.with(Cell::get), QuickSettingsHit::Lock),
        (LOGOUT.with(Cell::get), QuickSettingsHit::Logout),
        (POWER_OFF.with(Cell::get), QuickSettingsHit::PowerOff),
        (SETTINGS.with(Cell::get), QuickSettingsHit::Settings),
    ]
    .into_iter()
    .filter(|(rect, _)| rect.w > 0 && rect.h > 0)
    .collect()
}

pub(crate) fn reset_keyboard_focus() {
    KEYBOARD_FOCUS.with(|s| s.set(None));
}

pub(crate) fn focus_next(reverse: bool) {
    let targets = focus_targets();
    let count = targets.len();
    if count == 0 {
        return;
    }
    KEYBOARD_FOCUS.with(|s| {
        let index = s
            .get()
            .and_then(|current| targets.iter().position(|(_, action)| *action == current));
        let next = match (index, reverse) {
            (None, false) => 0,
            (None, true) => count - 1,
            (Some(i), false) => (i + 1) % count,
            (Some(i), true) => (i + count - 1) % count,
        };
        s.set(Some(targets[next].1));
    });
}

pub(crate) fn focused_action() -> Option<QuickSettingsHit> {
    let current = KEYBOARD_FOCUS.with(Cell::get)?;
    focus_targets()
        .iter()
        .find(|(_, action)| *action == current)
        .map(|(_, action)| *action)
}

fn draw_keyboard_focus(painter: &mut Painter<'_>, theme: &ThemeConfig) {
    let Some(current) = focused_action() else {
        return;
    };
    let Some((rect, _)) = focus_targets()
        .into_iter()
        .find(|(_, action)| *action == current)
    else {
        return;
    };
    let line = niwoe_tokens::Controls::FOCUS_WIDTH;
    for edge in [
        Rect { h: line, ..rect },
        Rect {
            y: rect.y + rect.h - line,
            h: line,
            ..rect
        },
        Rect { w: line, ..rect },
        Rect {
            x: rect.x + rect.w - line,
            w: line,
            ..rect
        },
    ] {
        painter.rect(edge, theme.colors.accent);
    }
}
