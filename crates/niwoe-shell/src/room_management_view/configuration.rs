//! Native room configuration page. Only name and presentation order have a
//! persistent backend today; all other mockup sections remain visible as
//! explicit capability states.

use super::*;
use crate::room_editor::Edit;
use std::sync::OnceLock;
use tiny_skia::{PixmapPaint, Transform};

fn landscape() -> Option<&'static Pixmap> {
    static LANDSCAPE: OnceLock<Option<Pixmap>> = OnceLock::new();
    LANDSCAPE
        .get_or_init(|| {
            Pixmap::decode_png(include_bytes!(
                "../../../../assets/wallpapers/niwoe-alpine-dawn.png"
            ))
            .ok()
        })
        .as_ref()
}

fn draw_landscape(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect) {
    if let Some(image) = landscape() {
        pm.draw_pixmap(
            0,
            0,
            image.as_ref(),
            &PixmapPaint::default(),
            Transform::from_row(
                rect.width as f32 / image.width() as f32,
                0.0,
                0.0,
                rect.height as f32 / image.height() as f32,
                rect.x as f32,
                rect.y as f32,
            ),
            None,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConfigurationAction {
    Back,
    Name,
    MoveEarlier,
    MoveLater,
    Save,
    Cancel,
}

fn content_bounds(width: u32) -> (i32, i32, i32, i32) {
    let left = C.sidebar_width + C.outer_pad;
    let right = width as i32 - C.outer_pad;
    let available = (right - left - C.card_gap).max(S.xxl * 2);
    let left_width = (available * 3 / 5).max(S.xxl);
    (
        left,
        left_width,
        left + left_width + C.card_gap,
        available - left_width,
    )
}

fn body_top() -> i32 {
    C.config_header_height + C.config_tabs_height + C.outer_pad
}

fn body_height(height: u32) -> i32 {
    height as i32 - C.config_footer_height - C.outer_pad - body_top()
}

fn lower_height(height: u32) -> i32 {
    (body_height(height) - C.config_details_height - C.config_context_height - C.card_gap * 2)
        .max(C.config_lower_height.max(C.config_note_height))
}

fn preview_height(_height: u32) -> i32 {
    C.config_preview_height
}

fn note_height(height: u32) -> i32 {
    lower_height(height)
}

pub(crate) fn max_configuration_scroll(height: u32) -> i32 {
    let left_bottom =
        C.config_details_height + C.config_context_height + lower_height(height) + C.card_gap * 2;
    let right_bottom = preview_height(height) + note_height(height) + C.card_gap;
    let content_bottom = body_top() + left_bottom.max(right_bottom) + C.outer_pad;
    (content_bottom - (height as i32 - C.config_footer_height)).max(0)
}

fn footer_action_rect(width: u32, height: u32, save: bool) -> Rect {
    let save_x = width as i32 - C.outer_pad - C.config_save_width;
    Rect {
        x: if save {
            save_x
        } else {
            save_x - C.card_gap - C.config_action_width
        },
        y: height as i32 - C.config_footer_height + C.card_gap,
        width: if save {
            C.config_save_width
        } else {
            C.config_action_width
        },
        height: C.config_field_height,
    }
}

fn name_rect(width: u32, scroll_y: i32) -> Rect {
    let (left, left_width, _, _) = content_bounds(width);
    Rect {
        x: left + C.card_pad + S.xxl * 2,
        y: body_top() - scroll_y + S.xxl * 2,
        width: (left_width - C.card_pad * 3 - S.xxl * 2) / 2,
        height: C.config_field_height,
    }
}

fn order_rect(width: u32, scroll_y: i32, later: bool) -> Rect {
    let (left, left_width, _, _) = content_bounds(width);
    let right = left + left_width - C.card_pad;
    Rect {
        x: right
            - if later {
                C.config_action_width
            } else {
                C.config_action_width * 2 + C.card_gap
            },
        y: body_top() - scroll_y + C.config_details_height - C.card_pad - C.config_field_height,
        width: C.config_action_width,
        height: C.config_field_height,
    }
}

pub(crate) fn hit_configuration(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scroll_y: i32,
) -> Option<ConfigurationAction> {
    if contains(back_rect(height), x, y)
        || contains(
            Rect {
                x: C.sidebar_width + C.outer_pad,
                y: 0,
                width: C.right_rail_width,
                height: C.config_field_height,
            },
            x,
            y,
        )
    {
        return Some(ConfigurationAction::Back);
    }
    if contains(footer_action_rect(width, height, true), x, y) {
        return Some(ConfigurationAction::Save);
    }
    if contains(footer_action_rect(width, height, false), x, y) {
        return Some(ConfigurationAction::Cancel);
    }
    if y < C.config_header_height + C.config_tabs_height
        || y >= height as i32 - C.config_footer_height
    {
        return None;
    }
    if contains(name_rect(width, scroll_y), x, y) {
        return Some(ConfigurationAction::Name);
    }
    if contains(order_rect(width, scroll_y, false), x, y) {
        return Some(ConfigurationAction::MoveEarlier);
    }
    if contains(order_rect(width, scroll_y, true), x, y) {
        return Some(ConfigurationAction::MoveLater);
    }
    None
}

fn section(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, title: &str, p: niwoe_tokens::Palette) {
    fill(pm, rect, alpha(p.surface, C.card_alpha), Radius::DEFAULT.sm);
    outline(pm, rect, p.border, Controls::BORDER);
    paint_text(
        pm,
        title,
        rect.x + C.card_pad,
        rect.y + S.xl,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
}

fn row(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    title: &str,
    description: &str,
    p: niwoe_tokens::Palette,
) {
    fill(
        pm,
        rect,
        alpha(p.surface_alt, C.disabled_alpha),
        Radius::DEFAULT.sm,
    );
    outline(pm, rect, p.border, Controls::BORDER);
    let title_size = Typography::DEFAULT.body_size as f32;
    let note_size = Typography::DEFAULT.caption_size as f32;
    let (title_ascent, title_descent) = ui_line_metrics(title_size);
    let (note_ascent, note_descent) = ui_line_metrics(note_size);
    let text_height = title_ascent + title_descent + S.sm as f32 + note_ascent + note_descent;
    let text_top = rect.y as f32 + (rect.height as f32 - text_height) / 2.0;
    paint_text(
        pm,
        title,
        rect.x + S.md,
        (text_top + title_ascent).round() as i32,
        title_size,
        p.text_dim,
    );
    paint_text(
        pm,
        description,
        rect.x + S.md,
        (text_top + title_ascent + title_descent + S.sm as f32 + note_ascent).round() as i32,
        note_size,
        p.text_dim,
    );
}

include!("configuration/body.rs");

fn draw_chrome(
    pm: &mut tiny_skia::PixmapMut<'_>,
    room: &RoomEntry,
    edit: &Edit,
    message: &str,
    pending: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let height = pm.height();
    draw_sidebar(pm, height, true, config);
    let header = Rect {
        x: C.sidebar_width,
        y: 0,
        width: pm.width() as i32 - C.sidebar_width,
        height: C.config_header_height,
    };
    fill(pm, header, p.background, Radius::DEFAULT.none);
    draw_landscape(pm, header);
    fill(
        pm,
        header,
        alpha(p.background, C.header_tint_alpha),
        Radius::DEFAULT.none,
    );
    let x = header.x + C.outer_pad;
    paint_text(
        pm,
        &format!("Räume  ›  {}  ›  Konfiguration", room.name),
        x,
        C.outer_pad + S.md,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    paint_text(
        pm,
        "Raum konfigurieren",
        x,
        C.outer_pad + S.xxl * 2,
        Typography::DEFAULT.display_size as f32,
        p.text,
    );
    paint_text(
        pm,
        &truncate_to_fit(
            &format!(
                "Passe den Raum {} und seine verfügbaren Funktionen an.",
                room.name
            ),
            header.width - C.outer_pad * 2,
            Typography::DEFAULT.body_size as f32,
        ),
        x,
        C.outer_pad + S.xxl * 3 + S.lg,
        Typography::DEFAULT.body_size as f32,
        p.text_dim,
    );
    let tabs = Rect {
        x: C.sidebar_width,
        y: C.config_header_height,
        width: pm.width() as i32 - C.sidebar_width,
        height: C.config_tabs_height,
    };
    fill(pm, tabs, p.background, Radius::DEFAULT.none);
    let mut tab_x = tabs.x + C.outer_pad;
    for (label, selected) in [
        ("Allgemein", true),
        ("Apps", false),
        ("Dateien", false),
        ("Wiederherstellung", false),
        ("Automatisierung", false),
        ("Benachrichtigungen", false),
    ] {
        let width = measure_text(label, Typography::DEFAULT.caption_size as f32).0 + S.xxl;
        let tab = Rect {
            x: tab_x,
            y: tabs.y + S.sm,
            width,
            height: tabs.height - S.sm * 2,
        };
        fill(
            pm,
            tab,
            alpha(
                p.surface,
                if selected {
                    C.card_alpha
                } else {
                    C.disabled_alpha
                },
            ),
            Radius::DEFAULT.sm,
        );
        if selected {
            outline(pm, tab, p.accent, Controls::BORDER);
        }
        paint_text_centered(
            pm,
            label,
            tab,
            Typography::DEFAULT.caption_size as f32,
            if selected { p.text } else { p.text_dim },
        );
        tab_x += width + C.card_gap;
    }
    let footer = Rect {
        x: C.sidebar_width,
        y: pm.height() as i32 - C.config_footer_height,
        width: pm.width() as i32 - C.sidebar_width,
        height: C.config_footer_height,
    };
    fill(pm, footer, p.background, Radius::DEFAULT.none);
    paint_text_left_centered(
        pm,
        if message.is_empty() {
            "Name und Reihenfolge sind speicherbar."
        } else {
            message
        },
        footer.x + C.outer_pad,
        footer,
        Typography::DEFAULT.caption_size as f32,
        if message.contains("fehl") || message.contains("ungült") {
            p.error
        } else {
            p.text_dim
        },
    );
    for (save, label) in [(false, "Abbrechen"), (true, "Änderungen speichern")] {
        let action = footer_action_rect(pm.width(), pm.height(), save);
        fill(
            pm,
            action,
            if save && !pending {
                p.accent
            } else {
                p.surface
            },
            Radius::DEFAULT.sm,
        );
        outline(pm, action, p.border, Controls::BORDER);
        paint_text_centered(
            pm,
            label,
            action,
            Typography::DEFAULT.caption_size as f32,
            if save && !pending {
                p.on_accent()
            } else {
                p.text_dim
            },
        );
    }
    if edit.focus == 3 || edit.focus == 4 {
        let target = footer_action_rect(pm.width(), pm.height(), edit.focus == 3);
        outline(pm, target, p.accent, Controls::FOCUS_WIDTH);
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_room_configuration(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    room: &RoomEntry,
    edit: &Edit,
    order: usize,
    room_count: usize,
    windows: &[WindowInfo],
    message: &str,
    pending: bool,
    scroll_y: i32,
    config: &niwoe_config::ThemeConfig,
) {
    if canvas.len() != width as usize * height as usize * 4 {
        return;
    }
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
    draw_body(
        &mut pm, room, edit, order, room_count, windows, scroll_y, config,
    );
    draw_chrome(&mut pm, room, edit, message, pending, config);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_actions_follow_drawn_controls() {
        let width = 1920;
        let height = 1032;
        for (action, rect) in [
            (ConfigurationAction::Name, name_rect(width, 0)),
            (
                ConfigurationAction::MoveEarlier,
                order_rect(width, 0, false),
            ),
            (ConfigurationAction::MoveLater, order_rect(width, 0, true)),
            (
                ConfigurationAction::Save,
                footer_action_rect(width, height, true),
            ),
            (
                ConfigurationAction::Cancel,
                footer_action_rect(width, height, false),
            ),
        ] {
            assert_eq!(
                hit_configuration(rect.x + 1, rect.y + 1, width, height, 0),
                Some(action)
            );
        }
        assert_eq!(hit_configuration(1, 1, width, height, 0), None);
    }

    #[test]
    fn configuration_footer_and_context_have_readable_geometry() {
        let height = 1032;
        let back = back_rect(height);
        let cancel = footer_action_rect(1920, height, false);
        let save = footer_action_rect(1920, height, true);
        assert_eq!((back.y, back.height), (cancel.y, cancel.height));
        assert_eq!((cancel.y, cancel.height), (save.y, save.height));
        assert!(
            measure_text(
                "Änderungen speichern",
                Typography::DEFAULT.caption_size as f32
            )
            .0 + S.md * 2
                <= save.width
        );
        let row_height = (C.config_context_height - S.xxl - C.card_pad * 2 - C.card_gap * 2) / 3;
        assert!(row_height >= S.lg + S.xl + S.md);
        for height in [720, 1032, 1200] {
            assert_eq!(
                preview_height(height) + C.card_gap,
                C.config_details_height + C.config_context_height + C.card_gap * 2
            );
            assert_eq!(note_height(height), lower_height(height));
        }
    }

    #[test]
    fn configuration_renders_selected_room() {
        let room = RoomEntry {
            id: 2,
            workspace: 2,
            name: "Design System".into(),
            description: String::new(),
            assignment: niwoe_ipc::RoomAssignment::Free,
        };
        let edit = Edit {
            id: 2,
            revision: 1,
            name: room.name.clone(),
            replace: true,
            focus: 0,
        };
        let width = std::env::var("NIWOE_PREVIEW_WIDTH")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(1920);
        let height = std::env::var("NIWOE_PREVIEW_HEIGHT")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(1032);
        let mut canvas = vec![0; (width * height * 4) as usize];
        let scroll_y = std::env::var("NIWOE_PREVIEW_SCROLL")
            .ok()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(0);
        draw_room_configuration(
            &mut canvas,
            width,
            height,
            &room,
            &edit,
            1,
            9,
            &[],
            "",
            false,
            scroll_y,
            &niwoe_config::ThemeConfig::default(),
        );
        assert!(canvas.as_chunks::<4>().0.iter().any(|pixel| pixel[3] != 0));
        if let Ok(path) = std::env::var("NIWOE_ROOM_CONFIGURATION_PREVIEW") {
            for pixel in canvas.as_chunks_mut::<4>().0 {
                pixel.swap(0, 2);
            }
            Pixmap::from_vec(canvas, tiny_skia::IntSize::from_wh(width, height).unwrap())
                .unwrap()
                .save_png(path)
                .unwrap();
        }
    }
}
