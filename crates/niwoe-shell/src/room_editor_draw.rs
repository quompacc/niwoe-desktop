use super::*;
use crate::wayland::RoomEditAction;
use crate::{ClickAction, ClickZone, Painter, Rect, TextRenderer};
use std::cell::RefCell;

pub(crate) fn draw(
    p: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &niwoe_config::ThemeConfig,
    state: &RoomUi,
    clicks: &mut Vec<ClickZone>,
) {
    let Some(edit) = &state.edit else {
        return;
    };
    let spacing = niwoe_tokens::Spacing::DEFAULT;
    let pad = crate::popup_card::PAD_X;
    let width = crate::WORKSPACE_POPUP_WIDTH as i32 - 2 * pad;
    let top = crate::popup_card::BODY_TOP;
    let h = niwoe_tokens::Controls::MIN_HEIGHT;
    let mut name = edit.name.clone();
    if !edit.replace {
        if let Some(font) = font.borrow_mut().as_mut() {
            while font.measure_text(&name) > width - 2 * spacing.sm && !name.is_empty() {
                name.remove(0);
            }
        }
    }
    let rows = [
        (
            RoomEditAction::Name,
            Rect {
                x: pad,
                y: top,
                w: width,
                h,
            },
            name.as_str(),
        ),
        (
            RoomEditAction::Save,
            Rect {
                x: pad,
                y: top + h + spacing.sm,
                w: (width - spacing.sm) / 2,
                h,
            },
            "Speichern",
        ),
        (
            RoomEditAction::Cancel,
            Rect {
                x: pad + (width + spacing.sm) / 2,
                y: top + h + spacing.sm,
                w: (width - spacing.sm) / 2,
                h,
            },
            "Zurück",
        ),
        (
            RoomEditAction::Left,
            Rect {
                x: pad,
                y: top + 2 * (h + spacing.sm),
                w: (width - spacing.sm) / 2,
                h,
            },
            "Nach links",
        ),
        (
            RoomEditAction::Right,
            Rect {
                x: pad + (width + spacing.sm) / 2,
                y: top + 2 * (h + spacing.sm),
                w: (width - spacing.sm) / 2,
                h,
            },
            "Nach rechts",
        ),
    ];
    for (index, (action, rect, label)) in rows.into_iter().enumerate() {
        let enabled = state.enabled(action);
        let focused = edit.focus == index;
        p.roundish_rect_with_radius(
            rect,
            if focused {
                theme.colors.accent
            } else {
                theme.colors.text_dim
            },
            niwoe_tokens::Radius::DEFAULT.sm,
        );
        let border = if focused {
            niwoe_tokens::Controls::FOCUS_WIDTH
        } else {
            niwoe_tokens::Controls::BORDER
        };
        p.roundish_rect_with_radius(
            Rect {
                x: rect.x + border,
                y: rect.y + border,
                w: rect.w - 2 * border,
                h: rect.h - 2 * border,
            },
            theme.colors.surface,
            niwoe_tokens::Radius::DEFAULT.sm,
        );
        p.text_clipped(
            font,
            label,
            rect.x + spacing.sm,
            rect.y + (h + niwoe_tokens::Typography::DEFAULT.body_size as i32) / 2,
            rect.w - 2 * spacing.sm,
            if enabled {
                theme.colors.text
            } else {
                theme.colors.text_dim
            },
        );
        if enabled {
            clicks.push(ClickZone {
                id: Some(format!("room-edit-{index}")),
                rect,
                action: ClickAction::EditRoom(action),
            });
        }
    }
    let message = if state.message.is_empty() {
        "Enter: speichern · Esc: zurück"
    } else {
        &state.message
    };
    p.text_clipped(
        font,
        message,
        pad,
        top + 3 * (h + spacing.sm) + spacing.lg,
        width,
        theme.colors.text_dim,
    );
}
