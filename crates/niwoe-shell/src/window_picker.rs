//! Window identities, never app identities, drive the Hub's window actions.
use crate::wayland::WindowInfo;
use niwoe_ipc::RoomEntry;
use niwoe_tokens::{window_picker::WindowPicker as L, Controls, Radius, Spacing, Typography};
use niwoe_ui::{
    effect::{paint_border, paint_fill, paint_text, rounded_rect_path, truncate_to_fit},
    paint::Rect,
};

#[derive(Default)]
pub(crate) struct Picker {
    pub selected: usize,
    pub target_window: Option<String>,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Hit {
    Row(usize),
    Move(usize),
    Previous,
    Next,
    Back,
}

pub(crate) fn visible(selected: usize, count: usize) -> std::ops::Range<usize> {
    let start = selected.min(count.saturating_sub(1)) / L::ROWS * L::ROWS;
    start..(start + L::ROWS).min(count)
}

fn card(width: u32, height: u32) -> Rect {
    let card_width = L::WIDTH.min(width as i32 - Spacing::DEFAULT.md * 2);
    let height_card = L::HEADER + L::ROWS as i32 * L::ROW_HEIGHT + L::FOOTER;
    Rect {
        x: (width as i32 - card_width) / 2,
        y: (height as i32 - height_card) / 2,
        width: card_width,
        height: height_card,
    }
}

fn inside(r: Rect, x: i32, y: i32) -> bool {
    x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
}

pub(crate) fn hit(
    picker: &Picker,
    count: usize,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
) -> Option<Hit> {
    let rect = card(width, height);
    if !inside(rect, x, y) {
        return Some(Hit::Back);
    }
    if y < rect.y + L::HEADER {
        return None;
    }
    let row = ((y - rect.y - L::HEADER) / L::ROW_HEIGHT) as usize;
    if row < L::ROWS {
        let index = visible(picker.selected, count).start + row;
        return (index < count).then(|| {
            if picker.target_window.is_none() && x >= rect.x + rect.width - L::ACTION_WIDTH {
                Hit::Move(index)
            } else {
                Hit::Row(index)
            }
        });
    }
    Some(if x < rect.x + rect.width / 3 {
        Hit::Previous
    } else if x >= rect.x + rect.width * 2 / 3 {
        Hit::Next
    } else {
        Hit::Back
    })
}

fn fill(pm: &mut tiny_skia::PixmapMut<'_>, rect: Rect, color: niwoe_tokens::Color) {
    if let Some(path) = rounded_rect_path(rect, Radius::DEFAULT.md) {
        paint_fill(pm, &path, color);
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    picker: &Picker,
    windows: &[WindowInfo],
    rooms: &[RoomEntry],
    theme: &niwoe_config::ThemeConfig,
) {
    let Some(mut pm) = tiny_skia::PixmapMut::from_bytes(canvas, width, height) else {
        return;
    };
    let p = crate::ui::tokens::theme_from_config(theme).palette;
    let rect = card(width, height);
    fill(&mut pm, rect, p.surface);
    if let Some(path) = rounded_rect_path(rect, Radius::DEFAULT.md) {
        paint_border(&mut pm, &path, p.border, Controls::BORDER as f32);
    }
    let pad = Spacing::DEFAULT.md;
    let caption = Typography::DEFAULT.caption_size as f32;
    let body = Typography::DEFAULT.body_size as f32;
    let count = if picker.target_window.is_some() {
        rooms.len()
    } else {
        windows.len()
    };
    let heading = if let Some(id) = &picker.target_window {
        windows
            .iter()
            .find(|w| &w.id == id)
            .map(|w| format!("Verschieben: {}", title(w)))
            .unwrap_or_else(|| "Fenster wurde geschlossen".into())
    } else {
        format!("Alle Fenster · {count}")
    };
    paint_text(
        &mut pm,
        &truncate_to_fit(&heading, rect.width - pad * 2, body),
        rect.x + pad,
        rect.y + L::HEADER / 2,
        body,
        p.text,
    );
    for (row, index) in visible(picker.selected, count).enumerate() {
        let y = rect.y + L::HEADER + row as i32 * L::ROW_HEIGHT;
        let r = Rect {
            x: rect.x + pad,
            y,
            width: rect.width - pad * 2,
            height: L::ROW_HEIGHT,
        };
        if index == picker.selected.min(count.saturating_sub(1)) {
            fill(&mut pm, r, p.surface_alt);
            if let Some(path) = rounded_rect_path(r, Radius::DEFAULT.md) {
                paint_border(&mut pm, &path, p.accent, Controls::FOCUS_WIDTH as f32);
            }
        }
        let (label, detail) = if picker.target_window.is_some() {
            let room = &rooms[index];
            (room.name.clone(), format!("Raum {}", room.workspace))
        } else {
            let w = &windows[index];
            let room = rooms
                .iter()
                .find(|r| r.workspace == w.workspace)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| format!("Raum {}", w.workspace));
            (
                title(w).to_owned(),
                format!("{}{}", room, if w.minimized { " · minimiert" } else { "" }),
            )
        };
        let text_width = r.width
            - pad * 2
            - if picker.target_window.is_none() {
                L::ACTION_WIDTH
            } else {
                0
            };
        paint_text(
            &mut pm,
            &truncate_to_fit(&label, text_width, body),
            r.x + pad,
            y + L::ROW_HEIGHT / 2 - pad / 2,
            body,
            p.text,
        );
        paint_text(
            &mut pm,
            &truncate_to_fit(&detail, text_width, caption),
            r.x + pad,
            y + L::ROW_HEIGHT - pad / 2,
            caption,
            p.text_dim,
        );
        if picker.target_window.is_none() {
            paint_text(
                &mut pm,
                "Verschieben …",
                rect.x + rect.width - L::ACTION_WIDTH + pad,
                y + L::ROW_HEIGHT / 2,
                caption,
                p.text,
            );
        }
    }
    if count == 0 {
        paint_text(
            &mut pm,
            "Keine offenen Fenster",
            rect.x + pad,
            rect.y + L::HEADER + L::ROW_HEIGHT,
            body,
            p.text_dim,
        );
    }
    let y = rect.y + rect.height - L::FOOTER / 2;
    paint_text(&mut pm, "‹ Zurück", rect.x + pad, y, caption, p.text);
    paint_text(
        &mut pm,
        "Schließen",
        rect.x + rect.width / 3 + pad,
        y,
        caption,
        p.text,
    );
    paint_text(
        &mut pm,
        "Weiter ›",
        rect.x + rect.width * 2 / 3 + pad,
        y,
        caption,
        p.text,
    );
    if !picker.message.is_empty() {
        paint_text(
            &mut pm,
            &picker.message,
            rect.x + pad,
            rect.y + L::HEADER - pad / 2,
            caption,
            p.text_dim,
        );
    }
}

fn title(window: &WindowInfo) -> &str {
    if window.title.trim().is_empty() {
        window.app_id.as_deref().unwrap_or("Fenster ohne Titel")
    } else {
        &window.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pages_reach_last_window_and_room_and_clamp_after_close() {
        assert_eq!(visible(63, 64), 60..64);
        assert_eq!(visible(63, 1), 0..1);
        assert_eq!(visible(0, 0), 0..0);
        let picker = Picker {
            selected: 63,
            ..Default::default()
        };
        let r = card(880, 620);
        assert_eq!(
            hit(
                &picker,
                64,
                880,
                620,
                r.x + Spacing::DEFAULT.md,
                r.y + L::HEADER
            ),
            Some(Hit::Row(60))
        );
        assert_eq!(
            hit(
                &picker,
                64,
                880,
                620,
                r.x + r.width - Spacing::DEFAULT.md,
                r.y + L::HEADER
            ),
            Some(Hit::Move(60))
        );
    }

    #[test]
    fn native_preview_keeps_distinct_and_minimized_windows() {
        let windows: Vec<_> = (0..14)
            .map(|i| WindowInfo {
                id: format!("window-{i}"),
                title: format!("Dokument {} - mehrere Fenster derselben App", i + 1),
                workspace: (i % 3 + 1) as u8,
                minimized: i % 2 == 0,
                app_id: Some("org.example.Editor".into()),
            })
            .collect();
        let rooms: Vec<_> = (1..=64)
            .map(|i| RoomEntry {
                id: i,
                workspace: i as u8,
                name: format!("Arbeitsraum {i}"),
                description: String::new(),
                assignment: niwoe_ipc::RoomAssignment::Free,
                preferences: Default::default(),
            })
            .collect();
        let target = std::env::var_os("NIWOE_PICKER_ROOMS").is_some();
        let picker = Picker {
            selected: if target { 63 } else { 7 },
            target_window: target.then(|| windows[0].id.clone()),
            message: String::new(),
        };
        let mut canvas = vec![0; 880 * 620 * 4];
        draw(
            &mut canvas,
            880,
            620,
            &picker,
            &windows,
            &rooms,
            &niwoe_config::ThemeConfig::default(),
        );
        assert!(canvas.iter().any(|v| *v != 0));
        if let Ok(path) = std::env::var("NIWOE_PICKER_PREVIEW") {
            tiny_skia::Pixmap::from_vec(canvas, tiny_skia::IntSize::from_wh(880, 620).unwrap())
                .unwrap()
                .save_png(path)
                .unwrap();
        }
    }
}
