use crate::state::{window_list_entry, NiwoeState};
use smithay::{utils::SERIAL_COUNTER, wayland::seat::WaylandFocus};

impl NiwoeState {
    pub fn focus_window_by_id(&mut self, id: &str) {
        if self.lobby_active {
            let target = (0..self.workspaces.count()).find(|&idx| {
                self.workspaces.space_at(idx).elements().any(|window| {
                    window_list_entry(window).is_some_and(|(window_id, _)| window_id == id)
                })
            });
            if let Some(target) = target {
                self.switch_workspace(target);
            }
        }
        let idx = self.current_workspace_index();
        let mapped_window = self
            .workspaces
            .space_at(idx)
            .elements()
            .find(|window| {
                window_list_entry(window)
                    .map(|(window_id, _)| window_id == id)
                    .unwrap_or(false)
            })
            .cloned();
        if let Some(window) = mapped_window {
            self.workspaces
                .space_at_mut(idx)
                .raise_element(&window, true);

            if let Some(surface) = window.wl_surface().map(|surface| surface.into_owned()) {
                let serial = SERIAL_COUNTER.next_serial();
                self.set_keyboard_focus_with_decorations(Some(surface.clone()), serial);
                self.update_focused_output_from_surface(
                    &surface,
                    "keyboard-focus-ipc-focus-window",
                );
                self.broadcast_toplevel_focused(&surface);
            }

            self.workspaces.space_at(idx).elements().for_each(|window| {
                if let Some(toplevel) = window.toplevel() {
                    toplevel.send_pending_configure();
                }
            });
            return;
        }

        if let Some(minimized) = self.minimized_windows.remove(id) {
            if minimized.workspace != idx {
                tracing::warn!(
                    "focus-window requested minimized id on another workspace: id={} minimized_workspace={} active_workspace={}",
                    id,
                    minimized.workspace + 1,
                    idx + 1
                );
                self.minimized_windows.insert(id.to_string(), minimized);
                return;
            }

            if let Some(x11) = minimized.window.x11_surface() {
                if let Err(err) = x11.set_hidden(false) {
                    tracing::warn!(
                        "focus-window restore minimized x11 window: set_hidden(false) failed: {}",
                        err
                    );
                }
            }

            self.workspaces.space_at_mut(idx).map_element(
                minimized.window.clone(),
                minimized.restore_loc,
                true,
            );
            self.workspaces
                .space_at_mut(idx)
                .raise_element(&minimized.window, true);

            if let Some(surface) = minimized
                .window
                .wl_surface()
                .map(|surface| surface.into_owned())
            {
                let serial = SERIAL_COUNTER.next_serial();
                self.set_keyboard_focus_with_decorations(Some(surface.clone()), serial);
                self.update_focused_output_from_surface(
                    &surface,
                    "keyboard-focus-ipc-restore-minimized-window",
                );
                self.broadcast_toplevel_focused(&surface);
            }

            self.workspaces.space_at(idx).elements().for_each(|window| {
                if let Some(toplevel) = window.toplevel() {
                    toplevel.send_pending_configure();
                }
            });
            self.mark_all_outputs_dirty("ipc-restore-minimized-window");
            self.broadcast_window_snapshot();
            return;
        }

        tracing::warn!("focus-window requested unknown id: {}", id);
    }
}
