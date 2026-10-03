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
    for index in 0..ROOM_PAGE_SIZE {
        let rect = room_rect(index, 1920, 1032, ROOM_PAGE_SIZE, false);
        assert_eq!(
            hit_room(rect.x + 1, rect.y + 1, 1920, 1032, rooms.len(), 0, false),
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
        let views = list::view_rect(width, true);
        assert_eq!(views.x + views.width + C.outer_pad, width as i32);
        let (grid_x, _, card_width, _, _) = grid_geometry(width, 1032, 9);
        let grid_end = grid_x + C.room_columns * card_width + (C.room_columns - 1) * C.card_gap;
        assert!(grid_end + C.outer_pad <= toolbar.x);
    }
}

#[test]
fn room_pages_keep_card_hits_bound_to_stable_entries() {
    let width = 1920;
    let height = 1032;
    let first = room_rect(0, width, height, 4, false);
    assert_eq!(max_room_page(10), 1);
    assert_eq!(
        hit_room(first.x + 1, first.y + 1, width, height, 10, 1, false),
        Some(6)
    );
    assert_eq!(max_room_page(64), 10);
}

#[test]
fn six_card_pages_and_footer_remain_reachable_in_both_target_canvases() {
    for (width, height) in [(1920, 1032), (1366, 720)] {
        for count in [0, 1, 6, 9, 10, 64] {
            for page in 0..=max_room_page(count) {
                let start = page * ROOM_PAGE_SIZE;
                let visible = count.saturating_sub(start).min(ROOM_PAGE_SIZE);
                for index in 0..visible {
                    let area = room_rect(index, width, height, visible, false);
                    assert!(area.y + area.height <= pages::rect(width, height, true).y);
                    assert_eq!(
                        hit_room(
                            area.x + area.width / 2,
                            area.y + area.height / 2,
                            width,
                            height,
                            count,
                            page,
                            false
                        ),
                        Some(start + index)
                    );
                }
            }
        }
        for (back, action) in [(true, 6), (false, 7)] {
            let area = pages::rect(width, height, back);
            assert!(area.y + area.height <= height as i32);
            assert_eq!(
                pages::hit(area.x + 1, area.y + 1, width, height),
                Some(action)
            );
        }
    }
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
        &rooms,
        1,
        &counts,
        &[],
        None,
        None,
        0,
        &Default::default(),
        &crate::icons::IconCache::new(),
        &crate::hub_state::HubState::default(),
        &[],
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

#[test]
fn responsive_toolbar_and_list_rows_share_visible_non_overlapping_hits() {
    for (width, height) in [(1920, 1032), (1366, 720)] {
        let search = list::search_rect(width);
        assert!(search.width >= C.search_min_width);
        assert!(search.y + search.height <= list::grid_top(width));
        assert_eq!(list::hit_list(search.x + 1, search.y + 1, width), Some(4));
        let category = list::category_rect();
        assert_eq!(list::hit_list(category.x + 1, category.y + 1, width), None);
        for (view, action) in [(false, 8), (true, 9)] {
            let area = list::view_rect(width, view);
            assert_eq!(list::hit_list(area.x + 1, area.y + 1, width), Some(action));
            assert!(area.x >= list::new_rect(width).x + list::new_rect(width).width);
        }
        for page in 0..=max_room_page(64) {
            let count = (64 - page * ROOM_PAGE_SIZE).min(ROOM_PAGE_SIZE);
            for index in 0..count {
                let area = list::row_rect(index, width, height);
                assert!(area.height >= Controls::MIN_HEIGHT);
                assert!(area.y + area.height <= pages::rect(width, height, true).y);
                assert_eq!(
                    hit_room(
                        area.x + area.width / 2,
                        area.y + area.height / 2,
                        width,
                        height,
                        64,
                        page,
                        true
                    ),
                    Some(page * ROOM_PAGE_SIZE + index)
                );
            }
        }
    }
    assert!(!list::compact(1920));
    assert!(list::compact(1366));
}

#[test]
fn occupancy_counts_use_the_given_room_scope() {
    let rooms = rooms();
    let mut counts = [0; niwoe_config::rooms::MAX_ROOMS];
    counts[0] = 2;
    counts[8] = 3;
    assert_eq!(list::occupancy(&rooms, &counts), 2);
    assert_eq!(list::occupancy(&rooms[1..8], &counts), 0);
}
