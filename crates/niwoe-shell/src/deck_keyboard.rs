// Included by quick_settings_popup: keyboard and pointer use the same hit slots.
thread_local! {
    static KEYBOARD_FOCUS: Cell<Option<usize>> = const { Cell::new(None) };
}

fn focus_targets() -> Vec<(Rect, QuickSettingsHit)> {
    [
        (AUDIO_MUTE.with(Cell::get), QuickSettingsHit::AudioMute),
        (VOLUME.with(Cell::get), QuickSettingsHit::Volume(0)),
        (NETWORK.with(Cell::get), QuickSettingsHit::Network),
        (POWER_PROFILE.with(Cell::get), QuickSettingsHit::PowerProfile),
        (APPEARANCE.with(Cell::get), QuickSettingsHit::Appearance),
        (SETTINGS.with(Cell::get), QuickSettingsHit::Settings),
        (LOCK.with(Cell::get), QuickSettingsHit::Lock),
        (POWER_OFF.with(Cell::get), QuickSettingsHit::PowerOff),
    ].into_iter().filter(|(rect, _)| rect.w > 0 && rect.h > 0).collect()
}

pub(crate) fn reset_keyboard_focus() {
    KEYBOARD_FOCUS.with(|s| s.set(None));
}

pub(crate) fn focus_next(reverse: bool) {
    let count = focus_targets().len();
    if count == 0 { return; }
    KEYBOARD_FOCUS.with(|s| {
        let next = match (s.get(), reverse) {
            (None, false) => 0,
            (None, true) => count - 1,
            (Some(i), false) => (i + 1) % count,
            (Some(i), true) => (i + count - 1) % count,
        };
        s.set(Some(next));
    });
}

pub(crate) fn focused_action() -> Option<QuickSettingsHit> {
    focus_targets().get(KEYBOARD_FOCUS.with(Cell::get)?).map(|(_, action)| *action)
}

fn draw_keyboard_focus(painter: &mut Painter<'_>, theme: &ThemeConfig) {
    let Some(index) = KEYBOARD_FOCUS.with(Cell::get) else { return; };
    let Some((rect, _)) = focus_targets().get(index).copied() else { return; };
    let line = niwoe_tokens::Controls::FOCUS_WIDTH;
    for edge in [
        Rect { h: line, ..rect },
        Rect { y: rect.y + rect.h - line, h: line, ..rect },
        Rect { w: line, ..rect },
        Rect { x: rect.x + rect.w - line, w: line, ..rect },
    ] { painter.rect(edge, theme.colors.accent); }
}
