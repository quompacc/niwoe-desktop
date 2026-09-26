use super::*;

#[test]
fn new_form_fields_and_deletion_actions_remain_reachable_at_both_sizes() {
    for (width, height) in [(1920, 1032), (1366, 720)] {
        for scroll in [0, max_configuration_scroll(height)] {
            for (rect, action) in [
                (
                    description_rect(width, scroll),
                    ConfigurationAction::Description,
                ),
                (
                    deletion_rect(width, height, scroll, true),
                    ConfigurationAction::Target,
                ),
                (
                    deletion_rect(width, height, scroll, false),
                    ConfigurationAction::Delete,
                ),
            ] {
                assert!(rect.width > 0 && rect.x + rect.width <= width as i32);
                let y = rect.y + rect.height / 2;
                if y >= C.config_header_height + C.config_tabs_height
                    && y < height as i32 - C.config_footer_height
                {
                    assert_eq!(
                        hit_configuration(rect.x + rect.width / 2, y, width, height, scroll),
                        Some(action)
                    );
                }
            }
        }
        let target = deletion_rect(width, height, max_configuration_scroll(height), true);
        let delete = deletion_rect(width, height, max_configuration_scroll(height), false);
        assert!(target.x + target.width + C.card_gap <= delete.x);
        assert!(delete.y + delete.height <= height as i32 - C.config_footer_height);
    }
}

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
    let creating = std::env::var_os("NIWOE_PREVIEW_CREATE").is_some();
    let room = RoomEntry {
        preferences: Default::default(),
        id: 2,
        workspace: 2,
        name: if creating {
            "Neuer Raum"
        } else {
            "Design System"
        }
        .into(),
        description: String::new(),
        assignment: niwoe_ipc::RoomAssignment::Free,
    };
    let edit = Edit {
        id: if creating { 0 } else { 2 },
        revision: 1,
        name: room.name.clone(),
        description: room.description.clone(),
        assignment: room.assignment,
        delete_target: None,
        target_menu: std::env::var("NIWOE_PREVIEW_TARGET_MENU")
            .ok()
            .and_then(|s| s.parse().ok()),
        confirm_delete: false,
        creation_uncertain: false,
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
        &crate::room_editor::RoomUi::default().snapshot.rooms,
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
