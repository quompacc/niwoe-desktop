// Modifier dragging reuses the same move/resize grabs as decoration input.
// No new grab lifecycle, timers or render work is introduced.
fn modifier_drag_hit(
    state: &NiwoeState,
    location: Point<f64, Logical>,
    button: u32,
    under_is_layer_surface: bool,
) -> Option<HitInfo> {
    if under_is_layer_surface || state.lock_manager.is_locked_or_pending() {
        return None;
    }
    if !state.seat.get_keyboard()?.modifier_state().logo {
        return None;
    }
    let space = state.workspaces.active_space();
    let (window, _) = space.element_under(location)?;
    // element_under returns the render origin, which includes the client's
    // CSD/shadow offset. Grabs relocate the mapped window geometry instead.
    let origin = space.element_location(window)?;
    if window.x11_surface().is_some_and(|x| x.is_override_redirect()) {
        return None;
    }
    let (_, fullscreen) = started_move_grab_window_states(window);
    if fullscreen {
        return None;
    }
    let hit = match button {
        0x110 => DecorationHit::TitleBar,
        0x111 => {
            let size = window.geometry().size;
            DecorationHit::Resize(modifier_resize_corner(
                location.x - f64::from(origin.x),
                location.y - f64::from(origin.y),
                size.w,
                size.h,
            ))
        }
        _ => return None,
    };
    Some((window.clone(), hit, origin, None))
}

fn modifier_resize_corner(x: f64, y: f64, w: i32, h: i32) -> DecorationResizeEdge {
    match (x < f64::from(w) / 2.0, y < f64::from(h) / 2.0) {
        (true, true) => DecorationResizeEdge::TopLeft,
        (false, true) => DecorationResizeEdge::TopRight,
        (true, false) => DecorationResizeEdge::BottomLeft,
        (false, false) => DecorationResizeEdge::BottomRight,
    }
}

#[cfg(test)]
mod modifier_drag_tests {
    use super::*;

    #[test]
    fn resize_uses_nearest_corner_including_center_boundary() {
        for (x, y, expected) in [
            (0.0, 0.0, DecorationResizeEdge::TopLeft),
            (99.0, 0.0, DecorationResizeEdge::TopRight),
            (0.0, 79.0, DecorationResizeEdge::BottomLeft),
            (99.0, 79.0, DecorationResizeEdge::BottomRight),
            (50.0, 40.0, DecorationResizeEdge::BottomRight),
        ] {
            assert_eq!(modifier_resize_corner(x, y, 100, 80), expected);
        }
    }
}
