use crate::state::NiwoeState;
use smithay::{desktop::Window, utils::SERIAL_COUNTER, wayland::seat::WaylandFocus};

fn windows(state: &NiwoeState) -> Vec<Window> {
    if state.lobby_active {
        return Vec::new();
    }
    state
        .workspaces
        .space_at(state.current_workspace_index())
        .elements()
        .filter(|w| !w.x11_surface().is_some_and(|x| x.is_override_redirect()))
        .cloned()
        .collect()
}

fn focus(state: &mut NiwoeState, window: Window) {
    let Some(surface) = window.wl_surface().map(|s| s.into_owned()) else {
        return;
    };
    let index = state.current_workspace_index();
    state
        .workspaces
        .space_at_mut(index)
        .raise_element(&window, true);
    state.set_keyboard_focus_with_decorations(Some(surface.clone()), SERIAL_COUNTER.next_serial());
    state.update_focused_output_from_surface(&surface, "keyboard-navigation");
    state.broadcast_toplevel_focused(&surface);
    for window in state.workspaces.space_at(index).elements() {
        if let Some(toplevel) = window.toplevel() {
            toplevel.send_pending_configure();
        }
    }
    state.mark_all_outputs_dirty("keyboard-navigation");
}

pub(super) fn cycle_window(state: &mut NiwoeState, step: i8) {
    let mut candidates = windows(state);
    // Stable surface identity prevents raising a window from reordering Alt+Tab.
    candidates.sort_by_key(|w| w.wl_surface().map(|s| crate::state::window_id(&s)));
    if candidates.is_empty() {
        return;
    }
    let current = super::focused_window_for_close(state);
    let index = current
        .as_ref()
        .and_then(|w| candidates.iter().position(|c| c == w));
    let next = index
        .map(|i| (i as i32 + i32::from(step)).rem_euclid(candidates.len() as i32) as usize)
        .unwrap_or(0);
    focus(state, candidates[next].clone());
}

fn directional_score(from: (i64, i64), to: (i64, i64), dx: i8, dy: i8) -> Option<i64> {
    let x = to.0 - from.0;
    let y = to.1 - from.1;
    let along = x * i64::from(dx) + y * i64::from(dy);
    let across = (x * i64::from(dy) - y * i64::from(dx)).abs();
    (along > 0).then_some(
        along
            .saturating_mul(along)
            .saturating_add(across.saturating_mul(across)),
    )
}

fn directional_target(state: &NiwoeState, dx: i8, dy: i8) -> Option<Window> {
    let current = super::focused_window_for_close(state)?;
    let space = state.workspaces.space_at(state.current_workspace_index());
    let center = |w: &Window| {
        space.element_location(w).map(|p| {
            let size = w.geometry().size;
            (
                i64::from(p.x) * 2 + i64::from(size.w),
                i64::from(p.y) * 2 + i64::from(size.h),
            )
        })
    };
    let origin = center(&current)?;
    windows(state)
        .into_iter()
        .filter(|w| w != &current)
        .filter_map(|w| directional_score(origin, center(&w)?, dx, dy).map(|score| (score, w)))
        .min_by_key(|(score, _)| *score)
        .map(|(_, w)| w)
}

pub(super) fn focus_direction(state: &mut NiwoeState, dx: i8, dy: i8) {
    if let Some(target) = directional_target(state, dx, dy) {
        focus(state, target);
    } else if super::focused_window_for_close(state).is_none() {
        cycle_window(state, 1);
    }
}

pub(super) fn swap_direction(state: &mut NiwoeState, dx: i8, dy: i8) {
    let Some(current) = super::focused_window_for_close(state) else {
        return;
    };
    let Some(target) = directional_target(state, dx, dy) else {
        return;
    };
    let index = state.current_workspace_index();
    if state.wm_workspaces[index].swap_windows(&current, &target) {
        state.tile_workspace(index);
        state.mark_all_outputs_dirty("keyboard-swap-tiles");
        state.broadcast_window_snapshot();
    }
}

pub(super) fn toggle_fullscreen(state: &mut NiwoeState) {
    use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State;
    use smithay::{wayland::shell::xdg::XdgShellHandler, xwayland::XwmHandler};
    let Some(window) = super::focused_window_for_close(state) else {
        return;
    };
    let index = state.current_workspace_index();
    if let Some(surface) = window.toplevel().cloned() {
        let key = format!(
            "keyboard-fullscreen:{}",
            crate::state::window_id(surface.wl_surface())
        );
        let fullscreen = surface.with_pending_state(|s| s.states.contains(State::Fullscreen));
        if fullscreen {
            XdgShellHandler::unfullscreen_request(state, surface.clone());
            if let Some(restore) = state.maximize_restore_locations.remove(&key) {
                surface.with_pending_state(|s| s.size = restore.client_size);
                state
                    .workspaces
                    .space_at_mut(index)
                    .map_element(window, restore.client_loc, true);
                surface.send_pending_configure();
            }
            state.tile_workspace(index);
        } else {
            if let Some(origin) = state.workspaces.space_at(index).element_location(&window) {
                state.maximize_restore_locations.insert(
                    key,
                    crate::state::MaximizeRestoreGeometry::new(
                        origin,
                        Some(window.geometry().size),
                    ),
                );
            }
            XdgShellHandler::fullscreen_request(state, surface, None);
        }
    } else if let Some(surface) = window.x11_surface().cloned() {
        if let Some(xwm) = surface.xwm_id() {
            if surface.is_fullscreen() {
                XwmHandler::unfullscreen_request(state, xwm, surface);
                state.tile_workspace(index);
            } else {
                XwmHandler::fullscreen_request(state, xwm, surface);
            }
        }
    }
    state.mark_all_outputs_dirty("keyboard-fullscreen");
}

pub(super) fn toggle_floating(state: &mut NiwoeState) {
    let Some(window) = super::focused_window_for_close(state) else {
        return;
    };
    let index = state.current_workspace_index();
    if state.wm_workspaces[index].mode == niwoe_wm::WorkspaceMode::Floating {
        state.wm_workspaces[index].mode = niwoe_wm::WorkspaceMode::Tiling;
        let all = windows(state);
        state.wm_workspaces[index].rebuild_tiling_from(all.into_iter());
    }
    let floating = !state.wm_workspaces[index].is_floating(&window);
    state.wm_workspaces[index].set_floating(&window, floating);
    if let Some(surface) = window.wl_surface() {
        state.decoration_manager.set_tiled(&surface, !floating);
    }
    if floating {
        if let Some(toplevel) = window.toplevel() {
            toplevel.with_pending_state(|pending| {
                crate::state::clear_tiled_toplevel_states(pending);
                // A floating client chooses its size; retaining the last tile
                // size makes a later focus configure undo client-side resizing.
                pending.size = None;
            });
            toplevel.send_pending_configure();
        }
    }
    state.tile_workspace(index);
    state.mark_all_outputs_dirty("keyboard-toggle-floating");
    state.broadcast_window_snapshot();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directional_focus_rejects_opposite_and_same_center_candidates() {
        assert_eq!(directional_score((0, 0), (-10, 0), 1, 0), None);
        assert_eq!(directional_score((0, 0), (0, 0), 1, 0), None);
        assert_eq!(directional_score((0, 0), (0, -10), 0, -1), Some(100));
        assert!(
            directional_score((0, 0), (10, 0), 1, 0) < directional_score((0, 0), (10, 10), 1, 0)
        );
    }
}
