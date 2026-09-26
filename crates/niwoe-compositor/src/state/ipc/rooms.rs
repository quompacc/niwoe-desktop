use crate::state::NiwoeState;
use niwoe_config::rooms::RoomId;
use niwoe_ipc::{RoomChange, RoomMutationError, RoomSnapshot, ShellEvent};
use smithay::{reexports::wayland_server::protocol::wl_surface::WlSurface, utils::SERIAL_COUNTER};

impl NiwoeState {
    pub fn broadcast_rooms(&mut self) {
        let registry = self.workspaces.rooms();
        let snapshot = RoomSnapshot {
            revision: registry.definitions().revision,
            rooms: registry
                .definitions()
                .rooms
                .iter()
                .map(|room| niwoe_ipc::RoomEntry {
                    preferences: crate::room_registry::preferences_to_wire(&room.preferences),
                    id: room.id.0,
                    workspace: (registry.slot_for_room(room.id).expect("registered room") + 1)
                        as u8,
                    name: room.name.clone(),
                    description: room.description.clone(),
                    assignment: match room.assignment {
                        niwoe_config::rooms::AssignmentMode::Free => {
                            niwoe_ipc::RoomAssignment::Free
                        }
                        niwoe_config::rooms::AssignmentMode::Preferred => {
                            niwoe_ipc::RoomAssignment::Preferred
                        }
                        niwoe_config::rooms::AssignmentMode::Dedicated => {
                            niwoe_ipc::RoomAssignment::Dedicated
                        }
                    },
                })
                .collect(),
        };
        self.ipc.broadcast(&ShellEvent::RoomSnapshot { snapshot });
    }

    pub fn mutate_room(
        &mut self,
        request_id: String,
        expected_revision: u64,
        change: niwoe_ipc::RoomChange,
    ) {
        let change_effect = match &change {
            RoomChange::Create { .. } | RoomChange::CreateDetails { .. } => Some((None, None)),
            RoomChange::Delete { id, target_id } => {
                let registry = self.workspaces.rooms();
                Some((
                    registry.slot_for_room(RoomId(*id)),
                    registry.slot_for_room(RoomId(*target_id)),
                ))
            }
            _ => None,
        };
        let error = if request_id.len() > 128 || request_id.is_empty() {
            Some(RoomMutationError::Invalid)
        } else {
            self.workspaces
                .rooms_mut()
                .apply(expected_revision, change)
                .err()
        };
        if error.is_none() || error == Some(RoomMutationError::Durability) {
            match change_effect {
                Some((None, None)) => {
                    self.workspaces.append_room_space();
                    self.wm_workspaces.push(niwoe_wm::WmWorkspace::new());
                }
                Some((Some(source), Some(target))) => self.remove_room_runtime(source, target),
                _ => {}
            }
            self.broadcast_workspace();
            self.broadcast_window_snapshot();
        }
        self.broadcast_rooms();
        self.ipc.broadcast(&ShellEvent::RoomMutationResult {
            request_id,
            revision: self.workspaces.rooms().definitions().revision,
            error,
        });
    }

    fn remove_room_runtime(&mut self, source: usize, target: usize) {
        let windows: Vec<_> = self
            .workspaces
            .space_at(source)
            .elements()
            .cloned()
            .collect();
        let destination = target - usize::from(target > source);
        for window in &windows {
            super::super::assignment::preserve_manual(window);
            let floating = self.wm_workspaces[source].mode == niwoe_wm::WorkspaceMode::Floating
                || self.wm_workspaces[source].is_floating(window);
            self.wm_workspaces[source].remove_window(window);
            if floating {
                self.wm_workspaces[target].set_floating(window, true);
            } else {
                self.wm_workspaces[target].add_tiled(window.clone(), None);
            }
        }
        for minimized in self.minimized_windows.values_mut() {
            if minimized.workspace == source {
                super::super::assignment::preserve_manual(&minimized.window);
            }
            minimized.workspace = if minimized.workspace == source {
                destination
            } else {
                minimized.workspace - usize::from(minimized.workspace > source)
            };
        }
        self.workspaces
            .remove_room_space(source, target, &self.outputs);
        self.wm_workspaces.remove(source);
        self.workspace_output_state
            .remove_room_index(source, target);
        let serial = SERIAL_COUNTER.next_serial();
        self.set_keyboard_focus_with_decorations(Option::<WlSurface>::None, serial);
        self.broadcast_toplevel_focus_cleared();
        self.tile_workspace(destination);
        self.workspaces.space_at_mut(destination).refresh();
        self.mark_all_outputs_dirty("room-deleted");
    }
}
