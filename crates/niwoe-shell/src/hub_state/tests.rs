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
fn scroll_direction_matches_wayland_and_ignores_horizontal_frames() {
    assert_eq!(scroll_back(1, 15.0), Some(false));
    assert_eq!(scroll_back(-1, -15.0), Some(true));
    assert_eq!(scroll_back(0, 0.25), Some(false));
    assert_eq!(scroll_back(0, -0.25), Some(true));
    assert_eq!(scroll_back(0, 0.0), None);
}
