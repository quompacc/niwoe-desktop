use super::*;

// A bounded list is rendered only on input/snapshot repaint; no timer or new assets.
fn rows(
    width: u32,
    height: u32,
    scroll: i32,
    edit: &Edit,
    rooms: &[RoomEntry],
) -> Vec<(Rect, u64)> {
    let Some(selected) = edit.target_menu else {
        return Vec::new();
    };
    let targets: Vec<_> = rooms.iter().filter(|room| room.id != edit.id).collect();
    let Some(index) = targets.iter().position(|r| r.id == selected) else {
        return Vec::new();
    };
    let trigger = deletion_rect(width, height, scroll, true);
    let available = (trigger.y - S.xxl - C.config_header_height - C.config_tabs_height).max(0)
        / C.config_field_height;
    let count = targets
        .len()
        .min(C.config_target_rows)
        .min(available.saturating_sub(1).max(0) as usize);
    if count == 0 {
        return Vec::new();
    }
    let start = index.saturating_sub(count - 1);
    let top = trigger.y - S.xxl - count as i32 * C.config_field_height;
    targets
        .iter()
        .enumerate()
        .skip(start)
        .take(count)
        .map(|(i, room)| {
            (
                Rect {
                    x: trigger.x,
                    y: top + (i - start) as i32 * C.config_field_height,
                    width: trigger.width,
                    height: C.config_field_height,
                },
                room.id,
            )
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn hit_target_menu(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scroll: i32,
    edit: &Edit,
    rooms: &[RoomEntry],
) -> Option<u64> {
    if y < C.config_header_height + C.config_tabs_height
        || y >= height as i32 - C.config_footer_height
    {
        return None;
    }
    rows(width, height, scroll, edit, rooms)
        .into_iter()
        .find_map(|(rect, id)| contains(rect, x, y).then_some(id))
}

pub(super) fn draw(
    pm: &mut tiny_skia::PixmapMut<'_>,
    edit: &Edit,
    rooms: &[RoomEntry],
    scroll: i32,
    p: niwoe_tokens::Palette,
) {
    let size = Typography::DEFAULT.caption_size as f32;
    let items = rows(pm.width(), pm.height(), scroll, edit, rooms);
    if let Some((first, id)) = items.first() {
        let targets: Vec<_> = rooms.iter().filter(|r| r.id != edit.id).collect();
        let start = targets.iter().position(|r| r.id == *id).unwrap_or(0) + 1;
        let header = Rect {
            y: first.y - C.config_field_height,
            ..*first
        };
        fill(pm, header, p.surface, Radius::DEFAULT.none);
        outline(pm, header, p.border, Controls::BORDER);
        let label = format!(
            "↑↓ / Mausrad · {}–{} von {}",
            start,
            start + items.len() - 1,
            targets.len()
        );
        paint_text_left_centered(
            pm,
            &truncate_to_fit(&label, header.width - S.md * 2, size),
            header.x + S.md,
            header,
            size,
            p.text_dim,
        );
    }
    for (rect, id) in items {
        let Some(room) = rooms.iter().find(|r| r.id == id) else {
            continue;
        };
        let selected = edit.target_menu == Some(id);
        fill(
            pm,
            rect,
            if selected { p.surface_alt } else { p.surface },
            Radius::DEFAULT.none,
        );
        outline(
            pm,
            rect,
            if selected { p.accent } else { p.border },
            Controls::BORDER,
        );
        let label = format!("{} · {}", room.workspace, room.name);
        paint_text_left_centered(
            pm,
            &truncate_to_fit(&label, rect.width - S.md * 2, size),
            rect.x + S.md,
            rect,
            size,
            p.text,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_other_rooms_can_be_reached_without_overlapping_chrome() {
        let mut ui = crate::room_editor::RoomUi::default();
        ui.snapshot.rooms = (1..=64)
            .map(|id| RoomEntry {
                preferences: Default::default(),
                id,
                workspace: id as u8,
                name: format!("Raum {id}"),
                description: String::new(),
                assignment: niwoe_ipc::RoomAssignment::Free,
            })
            .collect();
        ui.accept(ui.snapshot.clone());
        ui.begin(32);
        for (width, height) in [(1920, 1032), (1366, 720)] {
            let scroll = max_configuration_scroll(height);
            for id in (1..=64).filter(|id| *id != 32) {
                let edit = ui.edit.as_mut().unwrap();
                edit.target_menu = Some(id);
                let items = rows(width, height, scroll, edit, &ui.snapshot.rooms);
                assert!(items.len() <= C.config_target_rows);
                assert!(items.iter().any(|(_, candidate)| *candidate == id));
                for (rect, candidate) in items {
                    assert_ne!(candidate, 32);
                    assert!(rect.y >= C.config_header_height + C.config_tabs_height);
                    assert!(rect.y + rect.height < height as i32 - C.config_footer_height);
                    assert_eq!(
                        hit_target_menu(
                            rect.x + 1,
                            rect.y + 1,
                            width,
                            height,
                            scroll,
                            edit,
                            &ui.snapshot.rooms
                        ),
                        Some(candidate)
                    );
                }
            }
        }
    }
}
