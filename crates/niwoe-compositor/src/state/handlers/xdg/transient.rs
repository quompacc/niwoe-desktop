//! Follow the actual parent surface rather than the currently focused room.
use smithay::{
    desktop::Window,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::SERIAL_COUNTER,
    wayland::{seat::WaylandFocus, shell::xdg::ToplevelSurface},
};

use crate::state::NiwoeState;

fn mapped_window(state: &NiwoeState, surface: &WlSurface) -> Option<(usize, Window)> {
    (0..state.workspaces.count()).find_map(|index| {
        state
            .workspaces
            .space_at(index)
            .elements()
            .find(|w| w.wl_surface().is_some_and(|s| s.as_ref() == surface))
            .cloned()
            .map(|window| (index, window))
    })
}

pub(super) fn inherit_parent_workspace(state: &mut NiwoeState, surface: &ToplevelSurface) {
    let Some(parent) = surface.parent() else {
        return;
    };
    let target = mapped_window(state, &parent)
        .map(|(index, _)| index)
        .or_else(|| {
            state
                .minimized_windows
                .values()
                .find(|entry| {
                    entry
                        .window
                        .wl_surface()
                        .is_some_and(|s| s.as_ref() == &parent)
                })
                .map(|entry| entry.workspace)
        });
    let Some(target) = target else {
        return;
    };
    let child = surface.wl_surface();
    if let Some(entry) = state.minimized_windows.values_mut().find(|entry| {
        entry
            .window
            .wl_surface()
            .is_some_and(|s| s.as_ref() == child)
    }) {
        entry.workspace = target;
        state.broadcast_window_snapshot();
        return;
    }
    let Some((source, window)) = mapped_window(state, child) else {
        return;
    };
    if source == target {
        return;
    }
    let loc = state
        .workspaces
        .space_at(source)
        .element_location(&window)
        .unwrap_or_default();
    let floating = state.wm_workspaces[source].is_floating(&window);
    state.wm_workspaces[source].remove_window(&window);
    state.workspaces.space_at_mut(source).unmap_elem(&window);
    state
        .workspaces
        .space_at_mut(target)
        .map_element(window.clone(), loc, false);
    if floating {
        state.wm_workspaces[target].set_floating(&window, true);
    } else {
        state.wm_workspaces[target].add_tiled(window, None);
    }
    // new_toplevel precedes set_parent. Undo its temporary focus when the
    // parent belongs to a background room; never switch rooms to show a dialog.
    if target != state.current_workspace_index()
        && state
            .seat
            .get_keyboard()
            .is_some_and(|k| k.current_focus().as_ref() == Some(child))
    {
        let focus = state
            .workspaces
            .space_at(state.current_workspace_index())
            .elements()
            .filter_map(|w| w.wl_surface().map(|s| s.into_owned()))
            .next_back();
        state.set_keyboard_focus_with_decorations(focus.clone(), SERIAL_COUNTER.next_serial());
        if let Some(focus) = focus {
            state.broadcast_toplevel_focused(&focus);
        } else {
            state.broadcast_toplevel_focus_cleared();
        }
    }
    state.tile_workspace(source);
    state.tile_workspace(target);
    state.workspaces.space_at_mut(source).refresh();
    state.workspaces.space_at_mut(target).refresh();
    state.mark_all_outputs_dirty("xdg-parent-workspace");
    state.broadcast_window_snapshot();
}
