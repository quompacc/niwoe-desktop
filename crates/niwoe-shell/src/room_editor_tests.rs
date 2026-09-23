use super::*;
use crate::wayland::RoomEditAction;

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
