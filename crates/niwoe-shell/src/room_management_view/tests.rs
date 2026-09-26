use super::*;

fn rooms() -> Vec<RoomEntry> {
    (1..=9)
        .map(|workspace| RoomEntry {
            preferences: Default::default(),
            id: workspace as u64,
            workspace,
            name: format!("Raum {workspace}"),
            description: String::new(),
            assignment: niwoe_ipc::RoomAssignment::Free,
        })
        .collect()
}

#[test]
fn room_grid_and_back_hit_regions_match_draw_geometry() {
    let rooms = rooms();
    for index in 0..rooms.len() {
        let rect = room_rect(index, 1920, 1032, rooms.len());
        assert_eq!(
            hit_room(rect.x + 1, rect.y + 1, 1920, 1032, rooms.len(), 0),
            Some(index)
        );
    }
    let back = back_rect(1032);
    assert!(hit_back(back.x + 1, back.y + 1, 1032));
}

#[test]
fn right_toolbar_and_cards_share_one_rail_edge() {
    for width in [1280, 1600, 1920] {
        let toolbar = right_rail_toolbar_rect(width);
        assert_eq!(toolbar.x, right_rail_x(width));
        assert_eq!(toolbar.x + toolbar.width + C.outer_pad, width as i32);
        let (grid_x, _, card_width, _, _) = grid_geometry(width, 1032, 9);
        let grid_end = grid_x + C.room_columns * card_width + (C.room_columns - 1) * C.card_gap;
        assert!(grid_end + C.outer_pad <= toolbar.x);
    }
}

#[test]
fn room_pages_keep_card_hits_bound_to_stable_entries() {
    let width = 1920;
    let height = 1032;
    let first = room_rect(0, width, height, 1);
    assert_eq!(max_room_page(10), 1);
    assert_eq!(
        hit_room(first.x + 1, first.y + 1, width, height, 10, 1),
        Some(9)
    );
    assert_eq!(max_room_page(64), 7);
}

#[test]
fn management_view_renders_real_room_surface() {
    let rooms = rooms();
    let width = std::env::var("NIWOE_PREVIEW_WIDTH")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(1920);
    let height = std::env::var("NIWOE_PREVIEW_HEIGHT")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(1032);
    let mut canvas = vec![0; (width * height * 4) as usize];
    let mut counts = [0; niwoe_config::rooms::MAX_ROOMS];
    counts[0] = 1;
    draw_room_management(
        &mut canvas,
        width,
        height,
        &rooms,
        1,
        &counts,
        &[],
        None,
        None,
        0,
        &niwoe_config::ThemeConfig::default(),
    );
    assert!(canvas.as_chunks::<4>().0.iter().any(|pixel| pixel[3] != 0));
    if let Ok(path) = std::env::var("NIWOE_ROOM_MANAGEMENT_PREVIEW") {
        for pixel in canvas.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        Pixmap::from_vec(canvas, tiny_skia::IntSize::from_wh(width, height).unwrap())
            .unwrap()
            .save_png(path)
            .unwrap();
    }
}
