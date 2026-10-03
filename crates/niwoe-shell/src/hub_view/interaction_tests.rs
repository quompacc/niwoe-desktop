use super::*;
#[test]
fn search_hit_test_tracks_last_page_without_aliasing_rows() {
    let height = H.height as u32;
    let rows = search_rows(height);
    let selected = 63;
    let first = selected / rows * rows;
    assert_eq!(
        hit_search(
            H.outer_pad,
            H.header_height,
            H.width as u32,
            height,
            selected,
            64
        ),
        Some(first)
    );
    assert_eq!(
        hit_search(
            H.outer_pad,
            H.header_height - 1,
            H.width as u32,
            height,
            selected,
            64
        ),
        None
    );
    assert_eq!(
        hit_search(
            H.outer_pad,
            H.header_height + rows as i32 * H.search_row_height,
            H.width as u32,
            height,
            selected,
            64
        ),
        None
    );
}
#[test]
fn preview_click_and_room_click_share_the_card_geometry() {
    for slot in 0..H.room_columns as usize {
        let card = room_rect(slot, H.width as u32);
        let preview = preview_rect(card);
        assert!(hit_preview(preview.x, preview.y, H.width as u32, slot));
        assert!(!hit_preview(
            card.x + H.card_pad,
            card.y + H.card_pad,
            H.width as u32,
            slot
        ));
        assert_eq!(
            hit_room(preview.x, preview.y, H.width as u32, 64),
            Some(slot)
        );
    }
}
