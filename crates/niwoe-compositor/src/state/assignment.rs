//! Event-driven assignment, owned by the compositor and the actual Window.
//! No polling, launch-by-title heuristics, or work in the rendering loop.
use std::sync::Mutex;

use niwoe_wm::WorkspaceMode;
use smithay::{
    desktop::Window,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::SERIAL_COUNTER,
    wayland::{compositor::with_states, seat::WaylandFocus, shell::xdg::XdgToplevelSurfaceData},
};

use super::NiwoeState;
use crate::room_assignment::{choose_room, identity};

#[derive(Default)]
struct Assignment {
    settled: bool,
    manual: bool,
    presented: bool,
    launch_room: Option<niwoe_config::rooms::RoomId>,
}

fn tracking(window: &Window) -> &Mutex<Assignment> {
    window
        .user_data()
        .insert_if_missing(|| Mutex::new(Assignment::default()));
    window
        .user_data()
        .get::<Mutex<Assignment>>()
        .expect("assignment state")
}

pub(super) fn preserve_manual(window: &Window) {
    tracking(window).lock().unwrap().manual = true;
}

impl NiwoeState {
    fn assignment_window(&self, predicate: impl Fn(&Window) -> bool) -> Option<(usize, Window)> {
        (0..self.workspaces.count())
            .find_map(|slot| {
                self.workspaces
                    .space_at(slot)
                    .elements()
                    .find(|window| predicate(window))
                    .cloned()
                    .map(|window| (slot, window))
            })
            .or_else(|| {
                self.minimized_windows
                    .values()
                    .find(|entry| predicate(&entry.window))
                    .map(|entry| (entry.workspace, entry.window.clone()))
            })
    }

    pub(crate) fn assign_xdg_surface(&mut self, surface: &WlSurface) {
        if let Some((_, window)) = self
            .assignment_window(|window| window.wl_surface().is_some_and(|s| s.as_ref() == surface))
        {
            self.assign_window(&window);
        }
    }

    pub(crate) fn assign_x11_surface(&mut self, surface: &smithay::xwayland::X11Surface) {
        if let Some((_, window)) =
            self.assignment_window(|window| window.x11_surface() == Some(surface))
        {
            self.assign_window(&window);
        }
    }

    pub(crate) fn assign_window(&mut self, window: &Window) {
        let Some((source, _)) = self.assignment_window(|candidate| candidate == window) else {
            return;
        };
        let launch_room = if let Some(toplevel) = window.toplevel() {
            super::launch_intent::surface_room(toplevel.wl_surface())
        } else {
            window
                .x11_surface()
                .and_then(|surface| surface.startup_id())
                .and_then(|token| self.consume_launch_room(&token))
        };
        let (app, parent) = if let Some(toplevel) = window.toplevel() {
            let app = with_states(toplevel.wl_surface(), |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .and_then(|data| data.lock().unwrap().app_id.clone())
            })
            .and_then(|value| identity(value, true));
            let parent = toplevel.parent().and_then(|parent| {
                self.assignment_window(|w| w.wl_surface().is_some_and(|s| s.as_ref() == &parent))
            });
            (app, parent)
        } else if let Some(x11) = window.x11_surface() {
            if x11.is_override_redirect() {
                return;
            }
            let parent = x11.is_transient_for().and_then(|parent| {
                self.assignment_window(|w| w.x11_surface().is_some_and(|s| s.window_id() == parent))
            });
            (identity(x11.class(), false), parent)
        } else {
            return;
        };
        let registry = self.workspaces.rooms();
        let current = registry.room_at_slot(source).expect("mapped room");
        let mut assignment = tracking(window).lock().unwrap();
        assignment.launch_room = launch_room.or(assignment.launch_room);
        let explicit = if assignment.manual {
            Some(current)
        } else {
            assignment
                .launch_room
                .or(assignment.settled.then_some(current))
        };
        let target = choose_room(
            registry.definitions(),
            parent.and_then(|(slot, _)| registry.room_at_slot(slot)),
            explicit,
            app.as_ref(),
            current,
        );
        assignment.settled |= app.is_some();
        drop(assignment);
        let Some(target) = registry.slot_for_room(target) else {
            return;
        };
        if target == source {
            return;
        }
        if let Some(entry) = self
            .minimized_windows
            .values_mut()
            .find(|entry| &entry.window == window)
        {
            let floating = self.wm_workspaces[source].mode == WorkspaceMode::Floating
                || self.wm_workspaces[source].is_floating(window);
            self.wm_workspaces[source].remove_window(window);
            if floating {
                self.wm_workspaces[target].set_floating(window, true);
            } else {
                self.wm_workspaces[target].add_tiled(window.clone(), None);
            }
            entry.workspace = target;
            self.broadcast_window_snapshot();
            return;
        }
        let loc = self
            .workspaces
            .space_at(source)
            .element_location(window)
            .unwrap_or_default();
        let floating = window.x11_surface().is_some()
            || self.wm_workspaces[source].mode == WorkspaceMode::Floating
            || self.wm_workspaces[source].is_floating(window);
        self.wm_workspaces[source].remove_window(window);
        self.workspaces.space_at_mut(source).unmap_elem(window);
        self.workspaces
            .space_at_mut(target)
            .map_element(window.clone(), loc, false);
        if floating {
            self.wm_workspaces[target].set_floating(window, true);
        } else {
            self.wm_workspaces[target].add_tiled(window.clone(), None);
        }
        if target != self.current_workspace_index()
            && self.seat.get_keyboard().is_some_and(|keyboard| {
                keyboard.current_focus().as_ref() == window.wl_surface().as_deref()
            })
        {
            let focus = self
                .workspaces
                .space_at(self.current_workspace_index())
                .elements()
                .filter_map(|w| w.wl_surface().map(|s| s.into_owned()))
                .next_back();
            self.set_keyboard_focus_with_decorations(focus.clone(), SERIAL_COUNTER.next_serial());
            if let Some(focus) = focus {
                self.broadcast_toplevel_focused(&focus);
            } else {
                self.broadcast_toplevel_focus_cleared();
            }
        }
        self.tile_workspace(source);
        self.tile_workspace(target);
        self.workspaces.space_at_mut(source).refresh();
        self.workspaces.space_at_mut(target).refresh();
        self.mark_all_outputs_dirty("app-room-assignment");
        self.broadcast_window_snapshot();
    }

    /// Metadata and parent requests precede the first buffer. Focus only once,
    /// and only when the resolved destination is still the user's current room.
    pub(crate) fn present_assigned_xdg(&mut self, surface: &WlSurface) {
        let Some((slot, window)) =
            self.assignment_window(|w| w.toplevel().is_some_and(|t| t.wl_surface() == surface))
        else {
            return;
        };
        let mut assignment = tracking(&window).lock().unwrap();
        if assignment.presented {
            return;
        }
        assignment.presented = true;
        drop(assignment);
        if !self.lobby_active && slot == self.current_workspace_index() {
            self.set_keyboard_focus_with_decorations(
                Some(surface.clone()),
                SERIAL_COUNTER.next_serial(),
            );
            self.update_focused_output_from_surface(surface, "assigned-first-buffer");
            self.broadcast_toplevel_focused(surface);
        }
    }
}
