use niwoe_config::rooms::RoomId;
use smithay::{utils::SERIAL_COUNTER, wayland::seat::WaylandFocus};

use crate::state::{assignment::preserve_manual, window_list_entry, NiwoeState};

impl NiwoeState {
    pub(crate) fn move_window_to_room(&mut self, id: &str, room_id: u64) {
        let Some(target) = self.workspaces.rooms().slot_for_room(RoomId(room_id)) else {
            return;
        };
        if let Some(entry) = self.minimized_windows.get_mut(id) {
            preserve_manual(&entry.window);
            let source = entry.workspace;
            if source != target {
                let floating = self.wm_workspaces[source].mode == niwoe_wm::WorkspaceMode::Floating
                    || self.wm_workspaces[source].is_floating(&entry.window);
                self.wm_workspaces[source].remove_window(&entry.window);
                if floating {
                    self.wm_workspaces[target].set_floating(&entry.window, true);
                } else {
                    self.wm_workspaces[target].add_tiled(entry.window.clone(), None);
                }
            }
            entry.workspace = target;
            self.broadcast_window_snapshot();
            return;
        }
        let Some((source, window)) = (0..self.workspaces.count()).find_map(|source| {
            self.workspaces
                .space_at(source)
                .elements()
                .find(|w| window_list_entry(w).is_some_and(|(key, _)| key == id))
                .cloned()
                .map(|window| (source, window))
        }) else {
            return;
        };
        preserve_manual(&window);
        if source == target {
            return;
        }
        // A menu owns keyboard focus. Never focus the moved window merely to
        // identify it, nor dismiss an unrelated application's or layer's focus.
        if self.seat.get_keyboard().is_some_and(|k| {
            k.current_focus()
                .map(|target| target.into_surface())
                .as_ref()
                == window.wl_surface().as_deref()
        }) {
            self.set_keyboard_focus_with_decorations(None, SERIAL_COUNTER.next_serial());
            self.broadcast_toplevel_focus_cleared();
        }
        let loc = self
            .workspaces
            .space_at(source)
            .element_location(&window)
            .unwrap_or_default();
        let floating = self.wm_workspaces[source].mode == niwoe_wm::WorkspaceMode::Floating
            || self.wm_workspaces[source].is_floating(&window);
        self.wm_workspaces[source].remove_window(&window);
        self.workspaces.space_at_mut(source).unmap_elem(&window);
        if floating {
            self.wm_workspaces[target].set_floating(&window, true);
        } else {
            self.wm_workspaces[target].add_tiled(window.clone(), None);
        }
        self.workspaces
            .space_at_mut(target)
            .map_element(window, loc, false);
        self.tile_workspace(source);
        self.tile_workspace(target);
        self.workspaces.space_at_mut(source).refresh();
        self.workspaces.space_at_mut(target).refresh();
        self.mark_all_outputs_dirty("ipc-window-move");
        self.broadcast_window_snapshot();
    }
}
