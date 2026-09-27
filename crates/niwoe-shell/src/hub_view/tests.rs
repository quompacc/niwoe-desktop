use super::*;

#[test]
fn room_hit_test_uses_same_four_card_geometry() {
    let width = H.width as u32;
    for index in 0..4 {
        let rect = room_rect(index, width);
        assert_eq!(hit_room(rect.x + 1, rect.y + 1, width, 9), Some(index));
    }
    assert_eq!(hit_room(H.outer_pad, H.outer_pad, width, 9), None);
}

#[test]
fn hub_renders_room_and_capability_overview() {
    let rooms: Vec<_> = ["Entwicklung", "Recherche", "Konstruktion", "Kommunikation"]
        .into_iter()
        .enumerate()
        .map(|(index, name)| RoomEntry {
            preferences: Default::default(),
            id: index as u64 + 1,
            workspace: index as u8 + 1,
            name: name.into(),
            description: String::new(),
            assignment: niwoe_ipc::RoomAssignment::Free,
        })
        .collect();
    let windows = vec![WindowInfo {
        id: "preview".into(),
        title: "Raumkonzept.md".into(),
        workspace: 1,
        minimized: false,
        app_id: None,
    }];
    let system = SystemInfo {
        os_name: "Fedora Linux".into(),
        hostname: "niwoe".into(),
        kernel: "Linux".into(),
        uptime: "2 Stunden".into(),
        cpu: "Test CPU".into(),
        memory: "4.0 GiB / 8.0 GiB belegt".into(),
    };
    let width = std::env::var("NIWOE_PREVIEW_WIDTH")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(H.width as u32);
    let height = std::env::var("NIWOE_PREVIEW_HEIGHT")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(H.height as u32);
    let mut canvas = vec![0; (width * height * 4) as usize];
    let mut counts = [0; niwoe_config::rooms::MAX_ROOMS];
    counts[..4].copy_from_slice(&[3, 2, 1, 4]);
    draw_hub(
        &mut canvas,
        width,
        height,
        &rooms,
        1,
        &counts,
        &windows,
        &system,
        None,
        None,
        &crate::hub_state::HubState::default(),
        &[],
        &crate::icons::IconCache::new(),
        &niwoe_config::ThemeConfig::default(),
    );
    assert!(canvas.as_chunks::<4>().0.iter().any(|pixel| pixel[3] != 0));
    if let Ok(path) = std::env::var("NIWOE_HUB_PREVIEW") {
        for pixel in canvas.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        Pixmap::from_vec(canvas, tiny_skia::IntSize::from_wh(width, height).unwrap())
            .unwrap()
            .save_png(path)
            .unwrap();
    }
}
