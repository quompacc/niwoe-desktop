use super::*;

fn saved(count: usize) -> RestoreUi {
    RestoreUi {
        ready: true,
        revision: 1,
        results: (0..count)
            .map(|key| niwoe_ipc::LayoutResult {
                key: key as u32,
                label: format!("Fenster {key}"),
                message: "Ausstehend".into(),
                file: None,
            })
            .collect(),
        ..Default::default()
    }
}

#[test]
fn invalid_file_draft_is_rejected_before_sending_without_losing_selection() {
    let mut state = saved(1);
    state.file_key = Some(0);
    state.file = "relative-path".into();
    assert!(file_error(&state).is_some());
    assert_eq!(state.file, "relative-path");
    assert_eq!(state.file_key, Some(0));
    assert_eq!(state.results.len(), 1);
    state.file = std::env::temp_dir()
        .join("niwoe-owned-test")
        .to_string_lossy()
        .into();
    assert_eq!(file_error(&state), None);
    state.file.clear();
    assert_eq!(file_error(&state), None);
}

#[test]
fn hidden_page_actions_never_become_pointer_or_keyboard_targets() {
    let mut state = saved(12);
    state.file_key = Some(0);
    for (width, height) in [(1366, 720), (1920, 1032)] {
        for page in [Page::Files, Page::Restore] {
            let order = focus_order(&state, page, width, height);
            let hidden: &[usize] = if page == Page::Files {
                &[0, 1, 2, 3]
            } else {
                &[6, 7]
            };
            for &index in hidden {
                assert!(!enabled(&state, page, index, width, height));
                assert!(!order.contains(&index));
            }
        }
        let file = file_rect(width, height);
        // Restore may legitimately use this area for a read-only result row.
        assert!(!matches!(
            hit(
                &state,
                Page::Restore,
                file.x + S.sm,
                file.y + S.sm,
                width,
                height
            ),
            Some(6 | 7)
        ));
        assert_eq!(
            hit(
                &state,
                Page::Files,
                file.x + S.sm,
                file.y + S.sm,
                width,
                height
            ),
            Some(6)
        );
    }
}

#[test]
fn loading_missing_and_failure_have_distinct_capabilities_and_messages() {
    let mut state = RestoreUi::default();
    assert_eq!(
        focus_order(&state, Page::Files, 1366, 720),
        vec![30, 31, 32, 33]
    );
    assert_eq!(
        focus_order(&state, Page::Restore, 1366, 720),
        vec![30, 31, 32, 33]
    );
    assert!(message(&state).contains("geladen"));
    state.ready = true;
    state.message = "Kein lesbares Layout: No such file or directory (os error 2)".into();
    assert!(!failed(&state));
    assert!(message(&state).contains("noch kein Layout"));
    assert!(enabled(&state, Page::Restore, 0, 1366, 720));
    // A stale known revision must not enable restoration after deletion.
    state.revision = 7;
    assert!(!enabled(&state, Page::Restore, 1, 1366, 720));
    assert!(!enabled(&state, Page::Restore, 2, 1366, 720));
    state.message = "Kein lesbares Layout: Permission denied (os error 13)".into();
    assert!(failed(&state));
    assert!(message(&state).contains("Zugriffsrechte"));
    assert!(!message(&state).contains("noch kein"));
}

#[test]
fn absent_rows_invalid_selection_and_active_jobs_do_not_offer_file_edits() {
    let mut state = saved(1);
    state.file_key = Some(99);
    assert!(!enabled(&state, Page::Files, 6, 1366, 720));
    assert!(!enabled(&state, Page::Files, 7, 1366, 720));
    assert_eq!(
        focus_order(&state, Page::Files, 1366, 720),
        vec![30, 31, 32, 33, 8]
    );
    let empty_row = row_rect(Page::Files, 1366, 1);
    assert_eq!(
        hit(
            &state,
            Page::Files,
            empty_row.x + S.sm,
            empty_row.y + S.sm,
            1366,
            720
        ),
        None
    );
    state.file_key = Some(0);
    assert!(enabled(&state, Page::Files, 6, 1366, 720));
    state.running = true;
    for page in [Page::Files, Page::Restore] {
        for index in [0, 1, 2, 6, 7] {
            assert!(!enabled(&state, page, index, 1366, 720));
        }
    }
    assert!(enabled(&state, Page::Restore, 3, 1366, 720));
}

#[test]
fn paging_clamps_after_scale_or_result_changes_and_controls_fit() {
    for (width, height) in [(1366, 720), (1920, 1032), (7680, 4320)] {
        for page in [Page::Files, Page::Restore] {
            let mut state = saved(17);
            let count = page_size(page, width, height);
            assert!(count > 0 && count <= C.config_layout_page_rows);
            let last = row_rect(page, width, count - 1);
            let bottom = if page == Page::Files {
                file_rect(width, height).y - S.md
            } else {
                height as i32 - C.config_footer_height
            };
            assert!(last.y + last.height < bottom);
            for &index in page.buttons() {
                let r = control(page, width, index);
                assert!(r.width > 0 && r.x + r.width <= width as i32 - C.outer_pad);
                if enabled(&state, page, index, width, height) {
                    assert_eq!(
                        hit(
                            &state,
                            page,
                            r.x + r.width / 2,
                            r.y + r.height / 2,
                            width,
                            height
                        ),
                        Some(index)
                    );
                }
            }
            state.page = usize::MAX;
            let page_index = page_index(&state, page, width, height);
            assert_eq!(page_index, 16 / count);
            assert!(!enabled(&state, page, 5, width, height));
            state.results.truncate(1);
            assert_eq!(super::page_index(&state, page, width, height), 0);
            assert!(!enabled(&state, page, 4, width, height));
            assert!(!enabled(&state, page, 9, width, height));
        }
    }
}
