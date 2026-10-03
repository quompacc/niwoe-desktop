use super::*;

#[test]
fn search_results_use_cached_app_icons_in_the_bgra_output() {
    let mut app = crate::launcher::DesktopApp::new("Editor".into(), vec!["editor".into()], false);
    app.desktop_id = "org.example.Editor.desktop".into();
    app.icon_name = Some("editor-fixture".into());
    let mut cache = crate::icons::IconCache::new_for_theme("missing-test-theme", "#ffffff");
    let pixel = [17, 61, 193, 255];
    cache.insert_loaded(
        "editor-fixture".into(),
        H.app_icon_size as u32,
        Some(crate::icons::IconImage {
            width: H.app_icon_size as u32,
            height: H.app_icon_size as u32,
            bgra: pixel.repeat((H.app_icon_size * H.app_icon_size) as usize),
        }),
    );
    let rows = [crate::hub_state::ResultRow {
        target: crate::hub_state::Target::App(app.desktop_id.clone()),
        title: "Editor mit langem Namen".into(),
        detail: "Anwendung starten".into(),
    }];
    let mut canvas = vec![0; (H.width * H.height * 4) as usize];
    draw_search(
        &mut canvas,
        H.width as u32,
        H.height as u32,
        "editor",
        &rows,
        0,
        &[app],
        &[],
        &cache,
        &niwoe_config::ThemeConfig::default(),
    );
    let x = H.outer_pad + S.lg + H.app_icon_size / 2;
    let y = H.header_height + (H.search_row_height - S.xs) / 2;
    let offset = ((y * H.width + x) * 4) as usize;
    assert_eq!(&canvas[offset..offset + 4], &pixel);
}
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

#[test]
fn room_preview_and_app_strip_leave_the_status_footer_clear() {
    for slot in 0..H.room_columns as usize {
        let card = room_rect(slot, H.width as u32);
        let preview = preview_rect(card);
        assert!(preview.y > card.y + H.card_pad + H.room_icon_size + S.md);
        let app_bottom = preview.y + preview.height + S.sm + H.app_icon_size;
        let footer_top = card.y + card.height - H.card_pad - S.xxl;
        assert!(app_bottom + S.xs <= footer_top);
        assert_eq!(preview.height, H.preview_height as i32);
    }
}
