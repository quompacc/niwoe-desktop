use smithay::{
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{IsAlive, SERIAL_COUNTER},
    wayland::seat::WaylandFocus,
};

use super::super::NiwoeState;

impl NiwoeState {
    /// Called after unmapping. Removing an unfocused/background window must
    /// never steal focus; a lock or layer focus remains owned by that surface.
    pub(crate) fn restore_focus_after_removed(
        &mut self,
        removed: &WlSurface,
        parent: Option<WlSurface>,
    ) {
        if self.lobby_active || self.lock_manager.is_locked_or_pending() {
            return;
        }
        let Some(keyboard) = self.seat.get_keyboard() else {
            return;
        };
        if keyboard
            .current_focus()
            .as_ref()
            .map(|target| target.surface())
            != Some(removed)
        {
            return;
        }
        let workspace = self.current_workspace_index();
        let candidates: Vec<_> = self
            .workspaces
            .space_at(workspace)
            .elements()
            .filter(|window| {
                !window
                    .x11_surface()
                    .is_some_and(|x11| x11.is_override_redirect())
            })
            .filter_map(|window| {
                let surface = window.wl_surface()?.into_owned();
                (surface.alive() && surface != *removed).then(|| (window.clone(), surface))
            })
            .collect();
        let target = candidates
            .iter()
            .find(|(_, surface)| Some(surface) == parent.as_ref())
            .or_else(|| candidates.last());
        let surface = target.map(|(window, surface)| {
            self.workspaces
                .space_at_mut(workspace)
                .raise_element(window, true);
            surface.clone()
        });
        self.set_keyboard_focus_with_decorations(surface.clone(), SERIAL_COUNTER.next_serial());
        if let Some(surface) = surface {
            self.update_focused_output_from_surface(&surface, "removed-window-focus");
            self.broadcast_toplevel_focused(&surface);
        } else {
            self.broadcast_toplevel_focus_cleared();
        }
        for window in self.workspaces.space_at(workspace).elements() {
            if let Some(toplevel) = window.toplevel() {
                toplevel.send_pending_configure();
            }
        }
        self.mark_all_outputs_dirty("removed-window-focus");
    }
}
