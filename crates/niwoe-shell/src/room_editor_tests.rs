use super::*;
use crate::wayland::RoomEditAction;

#[test]
fn uncertain_creation_cannot_be_retried_and_duplicated() {
    let mut ui = RoomUi::default();
    ui.accept(ui.snapshot.clone());
    ui.begin_create();
    ui.edit.as_mut().unwrap().name = "Test".into();
    ui.request(RoomEditAction::Save).unwrap();
    ui.pending.as_mut().unwrap().1 = Instant::now() - Duration::from_secs(11);
    assert!(ui.expire());
    assert!(!ui.enabled(RoomEditAction::Save));
    assert!(ui.request(RoomEditAction::Save).is_none());
    assert!(ui.enabled(RoomEditAction::Cancel));
}

#[test]
fn create_is_a_draft_until_explicit_save_and_cancel_has_no_mutation() {
    let mut ui = RoomUi::default();
    assert!(!ui.begin_create());
    ui.accept(ui.snapshot.clone());
    let before = ui.snapshot.clone();
    assert!(ui.begin_create());
    assert!(!ui.enabled(RoomEditAction::Delete));
    ui.edit.as_mut().unwrap().name = "Neuer Kontext".into();
    ui.edit.as_mut().unwrap().description = "Beschreibung".into();
    assert!(
        matches!(ui.request(RoomEditAction::Save), Some(niwoe_ipc::ShellCommand::MutateRoom {
        change: RoomChange::CreateDetails { name, description, .. }, ..
    }) if name == "Neuer Kontext" && description == "Beschreibung")
    );
    assert_eq!(ui.snapshot, before);
    assert!(ui.request(RoomEditAction::Cancel).is_none());
    assert!(ui.edit.is_some(), "pending creation cannot be cancelled");
    let request = ui.pending.as_ref().unwrap().0.clone();
    ui.result(&request, Some(niwoe_ipc::RoomMutationError::Storage));
    ui.request(RoomEditAction::Cancel);
    assert!(ui.edit.is_none());
    assert_eq!(ui.snapshot, before);
}

#[test]
fn description_and_name_use_one_revision_and_delete_requires_target_and_confirmation() {
    let mut ui = RoomUi::default();
    ui.accept(ui.snapshot.clone());
    ui.begin(1);
    ui.edit.as_mut().unwrap().description = "Arbeit".into();
    assert!(
        matches!(ui.request(RoomEditAction::Save), Some(niwoe_ipc::ShellCommand::MutateRoom {
        expected_revision: 0, change: RoomChange::UpdateDetails { id: 1, description, .. }, ..
    }) if description == "Arbeit")
    );
    let request = ui.pending.as_ref().unwrap().0.clone();
    ui.result(&request, None);
    assert!(ui.request(RoomEditAction::Delete).is_none());
    assert!(ui.request(RoomEditAction::Target).is_none());
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, None);
    ui.choose_target(2);
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, Some(2));
    assert!(ui.request(RoomEditAction::Delete).is_none());
    assert!(ui.edit.as_ref().unwrap().confirm_delete);
    ui.request(RoomEditAction::Target);
    assert!(!ui.edit.as_ref().unwrap().confirm_delete);
    ui.move_target_selection(1);
    ui.choose_target(ui.edit.as_ref().unwrap().target_menu.unwrap());
    assert!(ui.request(RoomEditAction::Delete).is_none());
    assert!(matches!(
        ui.request(RoomEditAction::Delete),
        Some(niwoe_ipc::ShellCommand::MutateRoom {
            change: RoomChange::Delete {
                id: 1,
                target_id: 3
            },
            ..
        })
    ));
}

#[test]
fn target_menu_does_not_change_destination_until_selection_and_survives_reordering() {
    let mut ui = RoomUi::default();
    ui.accept(ui.snapshot.clone());
    ui.begin(1);
    ui.request(RoomEditAction::Target);
    ui.move_target_selection(isize::MAX);
    assert_eq!(ui.edit.as_ref().unwrap().target_menu, Some(9));
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, None);
    ui.choose_target(1);
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, None);
    ui.choose_target(9);
    ui.request(RoomEditAction::Target);
    ui.move_target_selection(isize::MIN);
    assert_eq!(ui.edit.as_ref().unwrap().target_menu, Some(2));
    ui.request(RoomEditAction::Target); // Close without choosing.
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, Some(9));
    let mut snapshot = ui.snapshot.clone();
    snapshot.rooms.swap(1, 8);
    snapshot.revision += 1;
    assert!(ui.accept(snapshot.clone()));
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, Some(9));
    snapshot.rooms.retain(|r| r.id != 9);
    for (index, room) in snapshot.rooms.iter_mut().enumerate() {
        room.workspace = index as u8 + 1;
    }
    snapshot.revision += 1;
    ui.request(RoomEditAction::Target);
    assert!(ui.accept(snapshot));
    assert_eq!(ui.edit.as_ref().unwrap().delete_target, None);
    assert_eq!(ui.edit.as_ref().unwrap().target_menu, None);
    assert!(!ui.edit.as_ref().unwrap().confirm_delete);
    assert!(ui.request(RoomEditAction::Delete).is_none());
}

#[test]
fn editing_waits_for_snapshot_and_ack_and_never_optimistically_changes_panel() {
    let mut ui = RoomUi::default();
    ui.begin(1);
    assert!(ui.edit.is_none());
    assert!(ui.accept(ui.snapshot.clone()));
    ui.begin(1);
    ui.edit.as_mut().unwrap().name = "Arbeit".into();
    let command = ui.request(RoomEditAction::Save).unwrap();
    let niwoe_ipc::ShellCommand::MutateRoom {
        request_id,
        expected_revision,
        change,
    } = command
    else {
        panic!("mutation");
    };
    assert_eq!(expected_revision, 0);
    assert_eq!(
        change,
        RoomChange::Rename {
            id: 1,
            name: "Arbeit".into()
        }
    );
    assert_eq!(ui.snapshot.rooms[0].name, "Raum 1");
    assert!(ui.request(RoomEditAction::Save).is_none());
    ui.result("unrelated", None);
    assert!(ui.pending.is_some());
    ui.result(&request_id, Some(niwoe_ipc::RoomMutationError::Storage));
    assert!(ui.pending.is_none());
    assert_eq!(ui.snapshot.rooms[0].name, "Raum 1");
    assert_eq!(ui.edit.as_ref().unwrap().name, "Arbeit");
}

#[test]
fn concurrent_snapshot_does_not_silently_rebase_an_open_name_edit() {
    let mut ui = RoomUi::default();
    ui.accept(ui.snapshot.clone());
    ui.begin(1);
    ui.edit.as_mut().unwrap().name = "Mein Entwurf".into();
    let mut changed = ui.snapshot.clone();
    changed.revision = 1;
    changed.rooms[0].name = "Anderer Entwurf".into();
    ui.accept(changed);
    assert!(matches!(
        ui.request(RoomEditAction::Save),
        Some(niwoe_ipc::ShellCommand::MutateRoom {
            expected_revision: 0,
            ..
        })
    ));
}

#[test]
fn invalid_snapshots_cannot_rebind_click_targets() {
    let mut ui = RoomUi::default();
    let mut bad = ui.snapshot.clone();
    bad.rooms[1].workspace = 1;
    assert!(!ui.accept(bad));
    let mut ordered = ui.snapshot.clone();
    ordered.revision = 5;
    ordered.rooms.reverse();
    assert!(ui.accept(ordered.clone()));
    ui.begin(1);
    assert_eq!(ui.edit.as_ref().unwrap().id, 1);
    ordered.revision = 4;
    assert!(!ui.accept(ordered));
}

#[test]
fn accepts_compact_dynamic_slots_and_rejects_gaps() {
    let mut ui = RoomUi::default();
    let mut snapshot = ui.snapshot.clone();
    snapshot.rooms.push(RoomEntry {
        id: 10,
        workspace: 10,
        name: "Raum 10".into(),
        description: String::new(),
        assignment: niwoe_ipc::RoomAssignment::Free,
    });
    assert!(ui.accept(snapshot.clone()));
    assert_eq!(ui.snapshot.rooms.len(), 10);
    snapshot.revision += 1;
    snapshot.rooms[9].workspace = 11;
    assert!(!ui.accept(snapshot));
}

#[test]
fn moves_preserve_id_and_timeout_requests_refresh_instead_of_retrying() {
    let mut ui = RoomUi::default();
    ui.accept(ui.snapshot.clone());
    ui.begin(1);
    assert!(ui.request(RoomEditAction::Left).is_none());
    let command = ui.request(RoomEditAction::Right).unwrap();
    assert!(matches!(
        command,
        niwoe_ipc::ShellCommand::MutateRoom {
            change: RoomChange::Move { id: 1, position: 1 },
            ..
        }
    ));
    ui.pending.as_mut().unwrap().1 = Instant::now() - Duration::from_secs(11);
    assert!(ui.expire());
    assert!(ui.pending.is_none());
    assert!(!ui.expire());
}

#[test]
fn editor_buttons_stay_within_popup_and_do_not_overlap() {
    let mut ui = RoomUi::default();
    ui.accept(ui.snapshot.clone());
    ui.begin(1);
    let mut pixels =
        vec![0; (crate::WORKSPACE_POPUP_WIDTH * crate::WORKSPACE_POPUP_HEIGHT * 4) as usize];
    let mut painter = crate::Painter::new(
        &mut pixels,
        crate::WORKSPACE_POPUP_WIDTH as i32,
        crate::WORKSPACE_POPUP_HEIGHT as i32,
    );
    let font = std::cell::RefCell::new(crate::TextRenderer::new(
        "sans",
        niwoe_tokens::Typography::DEFAULT.body_size.into(),
    ));
    let mut clicks = Vec::new();
    crate::popup_card::draw_card_body(&mut painter, &niwoe_config::ThemeConfig::default());
    crate::popup_card::draw_card_title(
        &mut painter,
        &font,
        &niwoe_config::ThemeConfig::default(),
        "Raum bearbeiten",
    );
    draw(
        &mut painter,
        &font,
        &niwoe_config::ThemeConfig::default(),
        &ui,
        &mut clicks,
    );
    assert_eq!(clicks.len(), 4); // The first room cannot move further left.
    for (i, zone) in clicks.iter().enumerate() {
        let a = zone.rect;
        assert!(
            a.x >= 0
                && a.y >= 0
                && a.x + a.w <= crate::WORKSPACE_POPUP_WIDTH as i32
                && a.y + a.h <= crate::WORKSPACE_POPUP_HEIGHT as i32
        );
        for b in clicks.iter().skip(i + 1).map(|z| z.rect) {
            assert!(a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y);
        }
    }
    if let Some(path) = std::env::var_os("NIWOE_ROOM_EDITOR_EVIDENCE") {
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        image::save_buffer(
            std::path::Path::new(&path),
            &pixels,
            crate::WORKSPACE_POPUP_WIDTH,
            crate::WORKSPACE_POPUP_HEIGHT,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
}
