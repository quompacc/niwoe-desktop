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
    rooms: &[RoomEntry],
    scroll_y: i32,
    p: niwoe_tokens::Palette,
) {
    if edit.id == 0 {
        return;
    }
    let target_rect = deletion_rect(pm.width(), pm.height(), scroll_y, true);
    paint_text(
        pm,
        "Fenster verschieben nach …",
        target_rect.x,
        target_rect.y - S.sm,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let target_name = edit
        .delete_target
        .and_then(|id| rooms.iter().find(|r| r.id == id))
        .map_or_else(
            || "Raum auswählen".to_owned(),
            |r| format!("{} · {}", r.workspace, r.name),
        );
    for (target, label, focus) in [
        (true, target_name, 6),
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
                rect.width - if target { S.xxl * 2 } else { S.sm * 2 },
                Typography::DEFAULT.caption_size as f32,
            ),
            rect,
            Typography::DEFAULT.caption_size as f32,
            if room_count > 1 { p.text } else { p.text_dim },
        );
        if target {
            let x = rect.x + rect.width - S.lg;
            let y = rect.y + rect.height / 2;
            let direction = if edit.target_menu.is_some() { -1 } else { 1 };
            let mut path = tiny_skia::PathBuilder::new();
            path.move_to((x - S.xs) as f32, (y - direction * S.xs / 2) as f32);
            path.line_to(x as f32, (y + direction * S.xs / 2) as f32);
            path.line_to((x + S.xs) as f32, (y - direction * S.xs / 2) as f32);
            if let Some(path) = path.finish() {
                paint_border(pm, &path, p.text_dim, Controls::BORDER as f32);
            }
        }
    }
}
