use super::*;

#[test]
fn focus_and_hits_exclude_unavailable_positions_and_pending_actions() {
    let mut state = PanelUi::default();
    assert!(!enabled(&state, 1));
    assert!(!enabled(&state, 11));
    assert!(!enabled(&state, 12));
    state.ready = true;
    assert!(enabled(&state, 12));
    crate::room_editor::panel::change_module(&mut state.draft, 0);
    assert!(!enabled(&state, 1));
    assert!(!enabled(&state, 2));
    let missing = control(C.canvas_min_width, C.canvas_min_height, 2);
    assert_eq!(
        hit(
            &state,
            missing.x + 1,
            missing.y + 1,
            C.canvas_min_width,
            C.canvas_min_height
        ),
        None
    );
    assert!(!focus_order(&state).contains(&2));
    state.pending = Some(("own-save".into(), std::time::Instant::now()));
    assert!(focus_order(&state).is_empty());
    assert!(!enabled(&state, 13));
}

#[test]
fn native_preview_and_all_controls_fit_the_minimum_canvas() {
    let w = C.canvas_min_width;
    let h = C.canvas_min_height;
    let footer = control(w, h, 12);
    assert!(preview_y() + crate::PANEL_SURFACE_HEIGHT as i32 + S.xl < footer.y);
    for index in 0..14 {
        let area = control(w, h, index);
        assert!(area.x >= C.sidebar_width && area.x + area.width <= w as i32 - C.outer_pad);
        assert!(area.y >= 0 && area.y + area.height <= h as i32);
    }
    let area = control(w, h, 0);
    let lines = niwoe_ui::effect::text_pair_layout(area);
    assert!(lines.height + 2 * (Controls::FOCUS_INSET + Controls::FOCUS_WIDTH / 2) <= area.height);
}
