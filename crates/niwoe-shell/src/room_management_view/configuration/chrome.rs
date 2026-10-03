//! Fixed heading, tabs and footer; drawn after the scrollable content.
use super::*;
use niwoe_ui::effect::Symbol;

pub(super) fn draw(
    pm: &mut tiny_skia::PixmapMut<'_>,
    room: &RoomEntry,
    edit: &Edit,
    message: &str,
    pending: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let height = pm.height();
    draw_sidebar(pm, height, crate::control_center::Page::Rooms, true, config);
    super::super::header::draw_configuration(pm, room, edit, config);
    let tabs = Rect {
        x: C.sidebar_width,
        y: C.config_header_height,
        width: pm.width() as i32 - C.sidebar_width,
        height: C.config_tabs_height,
    };
    fill(pm, tabs, p.background, Radius::DEFAULT.none);
    for (index, label) in form::TABS.iter().enumerate() {
        let tab = form::tab_rect(index);
        super::super::list::control(
            pm,
            tab,
            "",
            super::super::list::ControlState {
                enabled: index < 2 || (index < 4 && edit.id != 0),
                selected: edit.form.tab == index,
                focused: if edit.restore.open {
                    edit.restore.focus
                } else {
                    edit.focus
                } == 30 + index,
                ..Default::default()
            },
            config,
        );
        let enabled = index < 2 || (index < 4 && edit.id != 0);
        let color = if !enabled { p.text_disabled() } else { p.text };
        let icon = Rect {
            x: tab.x + S.sm,
            width: S.xl,
            ..tab
        };
        super::super::list::symbol(
            pm,
            icon,
            [
                Symbol::Settings,
                Symbol::App,
                Symbol::Folder,
                Symbol::History,
                Symbol::Lock,
            ][index],
            color,
        );
        paint_text_left_centered(
            pm,
            label,
            icon.x + icon.width + S.sm,
            tab,
            Typography::DEFAULT.caption_size as f32,
            color,
        );
    }
    let footer = Rect {
        x: C.sidebar_width,
        y: pm.height() as i32 - C.config_footer_height,
        width: pm.width() as i32 - C.sidebar_width,
        height: C.config_footer_height,
    };
    fill(pm, footer, p.background, Radius::DEFAULT.none);
    let note = if !message.is_empty() {
        message
    } else if edit.restore.open {
        "Esc: Allgemein · Tab/Enter: Bedienung · Layoutaktionen werden separat bestätigt."
    } else if edit.form.tab == 1 && !edit.form.error.is_empty() {
        &edit.form.error
    } else if edit.focus == 9 {
        "Symbol wechseln: Enter · Änderungen bleiben im Entwurf."
    } else {
        "Lokaler Entwurf · Änderungen gemeinsam speichern."
    };
    let note_width = if edit.restore.open {
        footer.width - C.outer_pad * 2
    } else {
        footer_action_rect(pm.width(), pm.height(), false).x - footer.x - C.outer_pad * 2
    };
    paint_text_left_centered(
        pm,
        &truncate_to_fit(note, note_width, Typography::DEFAULT.caption_size as f32),
        footer.x + C.outer_pad,
        footer,
        Typography::DEFAULT.caption_size as f32,
        if note.contains("fehl") || note.contains("Ungült") || !edit.form.error.is_empty() {
            p.error
        } else {
            p.text_dim
        },
    );
    if edit.restore.open {
        return;
    }
    for (save, label) in [(false, "Abbrechen"), (true, "Änderungen speichern")] {
        let action = footer_action_rect(pm.width(), pm.height(), save);
        super::super::list::control(
            pm,
            action,
            label,
            super::super::list::ControlState {
                enabled: !pending,
                primary: save,
                focused: edit.focus == if save { 3 } else { 4 },
                ..Default::default()
            },
            config,
        );
    }
}
