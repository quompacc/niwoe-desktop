use super::*;
fn rooms() -> Vec<RoomEntry> {
    (0..64)
        .map(|i| RoomEntry {
            id: i + 100,
            workspace: i as u8 + 1,
            name: format!("Raum {}", i + 1),
            description: "Kontext".into(),
            assignment: Default::default(),
            preferences: Default::default(),
        })
        .collect()
}
fn window(id: &str, workspace: u8) -> WindowInfo {
    WindowInfo {
        id: id.into(),
        workspace,
        title: format!("Dokument {id}"),
        app_id: Some("editor".into()),
        minimized: false,
    }
}
#[test]
fn bounded_static_generation_rejects_late_and_closed_responses() {
    let rooms = rooms();
    let windows: Vec<_> = (1..=64).map(|i| window(&i.to_string(), i)).collect();
    let mut hub = HubState::default();
    let first = hub.prepare(&rooms, &windows, "output1".into());
    assert_eq!(first.len(), PAGE_SIZE);
    assert!(hub.prepare(&rooms, &windows, "output1".into()).is_empty());
    assert!(hub.accept(&first[0].0, &first[0].1, 1, 1, &[0; 4]));
    assert_eq!(hub.images.len(), 1);
    for page in 0..16 {
        hub.page = page;
        hub.prepare(&rooms, &windows, "output1".into());
        assert!(hub.pending.len() <= PAGE_SIZE);
        assert!(hub.images.len() <= PAGE_SIZE);
    }
    assert!(!hub.accept(&first[1].0, &first[1].1, 1, 1, &[0; 4]));
    hub.clear();
    assert!(hub.images.is_empty());
    assert!(hub.pending.is_empty());
    assert!(!hub.accept(&first[0].0, &first[0].1, 1, 1, &[0; 4]));
}
#[test]
fn output_window_end_and_invalid_dimensions_invalidate() {
    let rooms = rooms();
    let windows = vec![window("one", 1)];
    let mut hub = HubState::default();
    let first = hub.prepare(&rooms, &windows, "scale1".into());
    let second = hub.prepare(&rooms, &windows, "scale2".into());
    assert!(!hub.accept(&first[0].0, "one", 1, 1, &[0; 4]));
    assert!(!hub.accept(&second[0].0, "one", u32::MAX, 1, &[0; 4]));
    hub.clear();
    let pending = hub.prepare(&rooms, &windows, "scale2".into());
    hub.prepare(&rooms, &[], "scale2".into());
    assert!(!hub.accept(&pending[0].0, "one", 1, 1, &[0; 4]));
}

#[test]
fn management_previews_are_bounded_and_reject_other_pages_minimized_and_ended_windows() {
    let rooms = rooms();
    let mut windows: Vec<_> = (1..=64).map(|i| window(&i.to_string(), i)).collect();
    let c = niwoe_tokens::ControlCenter::DEFAULT;
    let mut page = PreviewPage {
        index: 0,
        count: (c.room_columns * c.room_page_rows) as usize,
        max_width: c.room_preview_width,
        max_height: c.room_preview_height,
    };
    let mut state = HubState::default();
    let first = state.prepare_page(&rooms, &windows, "rooms".into(), page);
    assert_eq!(first.len(), 6);
    assert!(state
        .prepare_page(&rooms, &windows, "rooms".into(), page)
        .is_empty());
    assert!(state.accept(&first[0].0, &first[0].1, 1, 1, &[10, 20, 30, 0]));
    assert_eq!(state.images[&first[0].1].data(), &[30, 20, 10, 255]);
    for index in 1..=10 {
        page.index = index;
        let requests = state.prepare_page(&rooms, &windows, "rooms".into(), page);
        assert!(requests.len() <= 6);
        assert!(state.images.is_empty());
        assert!(!state.accept(&first[1].0, &first[1].1, 1, 1, &[0; 4]));
    }
    page.index = 0;
    let pending = state.prepare_page(&rooms, &windows, "rooms".into(), page);
    windows[0].minimized = true;
    state.prepare_page(&rooms, &windows, "rooms".into(), page);
    assert!(!state.accept(&pending[0].0, &pending[0].1, 1, 1, &[0; 4]));
    windows.clear();
    assert!(state
        .prepare_page(&rooms, &windows, "rooms".into(), page)
        .is_empty());
    assert!(state.images.is_empty());
    page.count = 7;
    assert!(state
        .prepare_page(&rooms, &windows, "rooms".into(), page)
        .is_empty());
    assert!(state.signature.is_none());
}
#[test]
fn search_preserves_room_ids_and_minimized_window_identity() {
    let mut rooms = rooms();
    rooms.swap(0, 63);
    let mut w = window("notes", 64);
    w.minimized = true;
    let hidden = Default::default();
    let rows = search(&rooms, &[w.clone()], &[], "Raum 64", &hidden);
    assert_eq!(rows[0].target, Target::Room(163));
    let rows = search(&rooms, &[w], &[], "NOTES", &hidden);
    assert_eq!(rows[0].target, Target::Window("notes".into()));
    assert!(rows[0].detail.contains("Minimiert"));
}

#[test]
fn configuration_preview_selects_only_edited_room_and_rejects_previous_surface_generations() {
    let rooms = rooms();
    let mut windows = vec![window("first", 1), window("second", 2)];
    let c = niwoe_tokens::ControlCenter::DEFAULT;
    let mut state = HubState::default();
    let mut page = PreviewPage {
        index: 0,
        count: 6,
        max_width: c.room_preview_width,
        max_height: c.room_preview_height,
    };
    let management = state.prepare_page(&rooms, &windows, "output-1".into(), page);
    assert_eq!(management.len(), 2);
    page.count = 1;
    page.max_height = c.config_preview_capture_height;
    let selected = std::slice::from_ref(&rooms[1]);
    let configuration = state.prepare_page(selected, &windows, "output-1".into(), page);
    assert_eq!(configuration.len(), 1);
    assert_eq!(configuration[0].1, "second");
    assert!(state
        .prepare_page(selected, &windows, "output-1".into(), page)
        .is_empty());
    assert!(!state.accept(&management[1].0, "second", 1, 1, &[0; 4]));
    let pixels =
        [10, 20, 30, 0].repeat((c.room_preview_width * c.config_preview_capture_height) as usize);
    assert!(state.accept(
        &configuration[0].0,
        "second",
        c.room_preview_width,
        c.config_preview_capture_height,
        &pixels
    ));
    assert_eq!(state.images.len(), 1);
    assert_eq!(&state.images["second"].data()[..4], &[30, 20, 10, 255]);
    assert_eq!(state.images["second"].data().len(), pixels.len());
    windows[1].minimized = true;
    assert!(state
        .prepare_page(selected, &windows, "output-1".into(), page)
        .is_empty());
    assert!(state.images.is_empty());
    windows[1].minimized = false;
    let restored = state.prepare_page(selected, &windows, "output-1".into(), page);
    assert_eq!(restored.len(), 1);
    windows[1].workspace = 1;
    assert!(state
        .prepare_page(selected, &windows, "output-1".into(), page)
        .is_empty());
    assert!(!state.accept(&restored[0].0, "second", 1, 1, &[0; 4]));
    state.clear();
    assert!(state.images.is_empty());
    assert!(state.pending.is_empty());
    page.count = 6;
    assert!(state
        .prepare_page(&rooms, &windows, "output-1".into(), page)
        .is_empty());
    assert!(state.signature.is_none());
}

#[test]
fn scroll_direction_matches_wayland_and_ignores_horizontal_frames() {
    assert_eq!(scroll_back(1, 15.0), Some(false));
    assert_eq!(scroll_back(-1, -15.0), Some(true));
    assert_eq!(scroll_back(0, 0.25), Some(false));
    assert_eq!(scroll_back(0, -0.25), Some(true));
    assert_eq!(scroll_back(0, 0.0), None);
}
