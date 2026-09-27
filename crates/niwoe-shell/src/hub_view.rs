//! Room-first Hub. Its layout follows the binding Hub mockup while every
//! visible value comes from shell state or is labelled as unavailable.
use crate::{sysinfo::SystemInfo, wayland::WindowInfo};
use niwoe_ipc::RoomEntry;
use niwoe_tokens::{Color, Controls, Hub, Radius, Spacing, Typography};
use niwoe_ui::{
    effect::{paint_border, paint_fill, paint_text, rounded_rect_path, truncate_to_fit},
    paint::Rect,
};
use tiny_skia::Pixmap;

const H: Hub = Hub::DEFAULT;
const S: Spacing = Spacing::DEFAULT;

fn alpha(color: Color, value: u8) -> Color {
    Color { a: value, ..color }
}

fn fill(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, color: Color, radius: i32) {
    if let Some(path) = rounded_rect_path(rect, radius) {
        paint_fill(pm, &path, color);
    }
}

fn outline(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, color: Color, width: i32) {
    if let Some(path) = rounded_rect_path(rect, Radius::DEFAULT.md) {
        paint_border(pm, &path, color, width as f32);
    }
}

include!("hub_view/alignment.rs");

fn inner_width(width: u32) -> i32 {
    width as i32 - H.outer_pad * 2
}

fn room_rect(index: usize, width: u32) -> Rect {
    let available = inner_width(width) - H.card_gap * (H.room_columns - 1);
    let card_width = available / H.room_columns;
    Rect {
        x: H.outer_pad + index as i32 * (card_width + H.card_gap),
        y: H.header_height,
        width: card_width,
        height: H.room_height,
    }
}

fn lower_height(height: u32) -> i32 {
    let top = H.header_height + H.room_height + H.section_gap;
    let footer_height = Controls::MIN_HEIGHT + S.sm;
    H.lower_height
        .min((height as i32 - top - footer_height).max(0))
}

fn lower_rect(index: usize, width: u32, height: u32) -> Rect {
    let available = inner_width(width) - H.card_gap * (H.lower_columns - 1);
    let card_width = available / H.lower_columns;
    Rect {
        x: H.outer_pad + index as i32 * (card_width + H.card_gap),
        y: H.header_height + H.room_height + H.section_gap,
        width: card_width,
        height: lower_height(height),
    }
}

include!("hub_view/hit.rs");

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_hub(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    rooms: &[RoomEntry],
    active_workspace: u8,
    window_counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    windows: &[WindowInfo],
    system: &SystemInfo,
    hovered_room: Option<usize>,
    keyboard_focus: Option<usize>,
    hub: &crate::hub_state::HubState,
    apps: &[crate::launcher::DesktopApp],
    icons: &crate::icons::IconCache,
    config: &niwoe_config::ThemeConfig,
) {
    if canvas.len() != width as usize * height as usize * 4 {
        return;
    }
    let Some(mut image) = Pixmap::new(width, height) else {
        return;
    };
    let theme = crate::ui::tokens::glass_theme_from_config(config);
    let p = theme.palette;
    let surface_alpha = config
        .decorations
        .shell_surface_fill_alpha(niwoe_config::ThemeSurface::Launcher);
    let tint = config.glass_tint_color();
    image.fill(tiny_skia::Color::from_rgba8(
        tint.r,
        tint.g,
        tint.b,
        surface_alpha,
    ));
    let mut pm = image.as_mut();
    draw_header(
        &mut pm,
        width,
        rooms.len(),
        active_workspace == 0,
        keyboard_focus == Some(rooms.len()),
        config,
    );
    draw_rooms(
        &mut pm,
        width,
        &rooms[hub
            .page
            .saturating_mul(H.room_columns as usize)
            .min(rooms.len())..],
        active_workspace,
        window_counts,
        hovered_room,
        keyboard_focus.and_then(|i| i.checked_sub(hub.page * H.room_columns as usize)),
        hub,
        windows,
        apps,
        icons,
        config,
    );
    draw_pages(&mut pm, width, hub.page, rooms.len(), config);
    draw_recent(&mut pm, lower_rect(0, width, height), windows, config);
    draw_shortcuts(&mut pm, lower_rect(1, width, height), config);
    draw_system(&mut pm, lower_rect(2, width, height), system, config);
    let footer_top = H.header_height + H.room_height + H.section_gap + lower_height(height);
    let footer_rect = Rect {
        x: 0,
        y: footer_top,
        width: width as i32,
        height: (height as i32 - footer_top).max(0),
    };
    let footer = if active_workspace == 0 {
        "Wähle einen Raum, um deinen Kontext zu öffnen."
    } else {
        "Räume schaffen Klarheit."
    };
    paint_text_centered(
        &mut pm,
        footer,
        footer_rect,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    outline(
        &mut pm,
        Rect {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        },
        p.border,
        Controls::BORDER,
    );
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

fn draw_header(
    pm: &mut tiny_skia::PixmapMut<'_>,
    width: u32,
    room_count: usize,
    lobby_active: bool,
    manage_focused: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let title = Typography::DEFAULT.display_size as f32;
    let body = Typography::DEFAULT.body_size as f32;
    let caption = Typography::DEFAULT.caption_size as f32;
    paint_text(
        pm,
        "HUB",
        H.outer_pad,
        H.outer_pad + S.lg,
        caption,
        p.accent,
    );
    paint_text(
        pm,
        "Deine Arbeit.",
        H.outer_pad,
        H.outer_pad + S.xxl * 2,
        title,
        p.text,
    );
    paint_text(
        pm,
        "In ihrem Kontext.",
        H.outer_pad,
        H.outer_pad + S.xxl * 3,
        title,
        p.text,
    );
    paint_text(
        pm,
        "Räume für fokussiertes Arbeiten. Alles an seinem Platz.",
        H.outer_pad,
        H.outer_pad + S.xxl * 4,
        body,
        p.text_dim,
    );
    paint_text(
        pm,
        &if lobby_active {
            format!("LOGE · {room_count} RÄUME")
        } else {
            format!("RÄUME · {room_count}")
        },
        H.outer_pad,
        H.header_height - S.md,
        caption,
        p.accent,
    );
    let close = Rect {
        x: width as i32 - H.outer_pad - H.close_width,
        y: H.outer_pad,
        width: H.close_width,
        height: Controls::MIN_HEIGHT,
    };
    let manage = Rect {
        x: close.x - H.card_gap - H.manage_width,
        y: close.y,
        width: H.manage_width,
        height: close.height,
    };
    fill(
        pm,
        manage,
        alpha(p.surface_alt, H.quiet_alpha),
        Radius::DEFAULT.sm,
    );
    outline(
        pm,
        manage,
        if manage_focused { p.accent } else { p.border },
        if manage_focused {
            Controls::FOCUS_WIDTH
        } else {
            Controls::BORDER
        },
    );
    paint_text_centered(pm, "Räume verwalten", manage, caption, p.text);
    fill(
        pm,
        close,
        alpha(p.surface_alt, H.quiet_alpha),
        Radius::DEFAULT.sm,
    );
    outline(pm, close, p.border, Controls::BORDER);
    paint_text_centered(pm, "Esc  Schließen", close, caption, p.text_dim);
}

include!("hub_view/rooms.rs");

fn section_card(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    title: &str,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(pm, rect, alpha(p.surface, H.card_alpha), Radius::DEFAULT.md);
    outline(pm, rect, p.border, Controls::BORDER);
    paint_text_left_centered(
        pm,
        title,
        rect.x + H.card_pad,
        Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: S.xxl + S.lg,
        },
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
}

fn draw_recent(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    windows: &[WindowInfo],
    config: &niwoe_config::ThemeConfig,
) {
    section_card(pm, rect, "FENSTER · ALLE ANZEIGEN ›", config);
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let caption = Typography::DEFAULT.caption_size as f32;
    let rows = windows
        .iter()
        .filter(|w| !w.title.trim().is_empty())
        .take(4);
    let mut count = 0;
    for (index, window) in rows.enumerate() {
        count += 1;
        let row = Rect {
            x: rect.x,
            y: rect.y + S.xxl + S.lg + index as i32 * S.xxl,
            width: rect.width,
            height: S.xxl,
        };
        let title = truncate_to_fit(&window.title, rect.width - H.card_pad * 2 - S.xxl, caption);
        paint_text_left_centered(pm, &title, rect.x + H.card_pad, row, caption, p.text);
        paint_text_right_centered(
            pm,
            "›",
            rect.x + rect.width - H.card_pad,
            row,
            caption,
            p.text_dim,
        );
    }
    if count == 0 {
        let row = Rect {
            x: rect.x,
            y: rect.y + S.xxl + S.lg,
            width: rect.width,
            height: S.xxl,
        };
        paint_text_left_centered(
            pm,
            "Noch keine offenen Fenster",
            rect.x + H.card_pad,
            row,
            caption,
            p.text_dim,
        );
    }
}

fn draw_shortcuts(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    config: &niwoe_config::ThemeConfig,
) {
    section_card(pm, rect, "SCHNELLHILFE", config);
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let caption = Typography::DEFAULT.caption_size as f32;
    for (index, (key, description)) in [
        ("Tippen", "Suche starten"),
        ("↑ / ↓", "Treffer auswählen"),
        ("Enter", "Auswahl öffnen"),
        ("Esc", "Zurück oder schließen"),
    ]
    .into_iter()
    .enumerate()
    {
        let row = Rect {
            x: rect.x,
            y: rect.y + S.xxl + S.lg + index as i32 * S.xxl,
            width: rect.width,
            height: S.xxl,
        };
        paint_text_left_centered(pm, key, rect.x + H.card_pad, row, caption, p.text);
        paint_text_left_centered(
            pm,
            description,
            rect.x + rect.width / 3,
            row,
            caption,
            p.text_dim,
        );
    }
}

fn draw_system(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    system: &SystemInfo,
    config: &niwoe_config::ThemeConfig,
) {
    section_card(pm, rect, "SYSTEMZUSTAND", config);
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let ready_row = Rect {
        x: rect.x,
        y: rect.y + S.xxl + S.lg,
        width: rect.width,
        height: S.xxl,
    };
    fill(
        pm,
        Rect {
            x: rect.x + H.card_pad,
            y: ready_row.y + (ready_row.height - H.status_dot_size - S.xs) / 2,
            width: H.status_dot_size + S.xs,
            height: H.status_dot_size + S.xs,
        },
        p.success,
        Radius::DEFAULT.lg,
    );
    paint_text_left_centered(
        pm,
        "System bereit",
        rect.x + H.card_pad + S.xl,
        ready_row,
        Typography::DEFAULT.body_size as f32,
        p.text,
    );
    for (index, (label, value)) in [
        ("System", system.os_name.as_str()),
        ("Speicher", system.memory.as_str()),
        ("Laufzeit", system.uptime.as_str()),
    ]
    .into_iter()
    .enumerate()
    {
        let row = Rect {
            x: rect.x,
            y: ready_row.y + ready_row.height + index as i32 * S.xxl,
            width: rect.width,
            height: S.xxl,
        };
        paint_text_left_centered(
            pm,
            label,
            rect.x + H.card_pad,
            row,
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
        let value = truncate_to_fit(
            value,
            rect.width / 2,
            Typography::DEFAULT.caption_size as f32,
        );
        paint_text_left_centered(
            pm,
            &value,
            rect.x + rect.width / 2,
            row,
            Typography::DEFAULT.caption_size as f32,
            p.text,
        );
    }
}

#[cfg(test)]
mod interaction_tests;
#[cfg(test)]
mod tests;
include!("hub_view/search.rs");
include!("hub_view/previews.rs");
