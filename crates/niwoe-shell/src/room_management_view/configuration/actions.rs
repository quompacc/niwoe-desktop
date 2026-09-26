use super::*;

pub(super) fn description_rect(width: u32, scroll_y: i32) -> Rect {
    let name = name_rect(width, scroll_y);
    let (left, left_width, _, _) = content_bounds(width);
    Rect {
        x: name.x + name.width + C.card_gap,
        y: name.y,
        width: (left + left_width - C.card_pad - name.x - name.width - C.card_gap).max(S.xxl),
        height: C.config_field_height,
    }
}

pub(super) fn deletion_rect(width: u32, height: u32, scroll_y: i32, target: bool) -> Rect {
    let (_, _, right, right_width) = content_bounds(width);
    let delete_x = right + right_width - C.card_pad - C.config_action_width;
    Rect {
        x: if target { right + C.card_pad } else { delete_x },
        y: body_top() - scroll_y + preview_height(height) + C.card_gap + note_height(height)
            - C.card_pad
            - C.config_field_height,
        width: if target {
            delete_x - C.card_gap - right - C.card_pad
        } else {
            C.config_action_width
        },
        height: C.config_field_height,
    }
}

pub(super) fn draw_deletion(
    pm: &mut tiny_skia::PixmapMut<'_>,
    edit: &Edit,
    room_count: usize,
    target_name: &str,
    scroll_y: i32,
    p: niwoe_tokens::Palette,
) {
    if edit.id == 0 {
        return;
    }
    for (target, label, focus) in [
        (true, format!("Ziel: {target_name}"), 6),
        (
            false,
            if edit.confirm_delete {
                "Löschen bestätigen"
            } else {
                "Raum löschen"
            }
            .to_owned(),
            7,
        ),
    ] {
        let rect = deletion_rect(pm.width(), pm.height(), scroll_y, target);
        fill(pm, rect, p.surface_alt, Radius::DEFAULT.sm);
        outline(
            pm,
            rect,
            if edit.focus == focus {
                p.accent
            } else {
                p.border
            },
            if edit.focus == focus {
                Controls::FOCUS_WIDTH
            } else {
                Controls::BORDER
            },
        );
        paint_text_centered(
            pm,
            &truncate_to_fit(
                &label,
                rect.width - S.sm * 2,
                Typography::DEFAULT.caption_size as f32,
            ),
            rect,
            Typography::DEFAULT.caption_size as f32,
            if room_count > 1 { p.text } else { p.text_dim },
        );
    }
}
