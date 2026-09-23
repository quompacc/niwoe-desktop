use crate::state::NiwoeState;
use niwoe_ipc::{RoomMutationError, RoomSnapshot, ShellEvent};

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
                    id: room.id.0,
                    workspace: (registry.slot_for_room(room.id).expect("registered room") + 1)
                        as u8,
                    name: room.name.clone(),
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
        let error = if request_id.len() > 128 || request_id.is_empty() {
            Some(RoomMutationError::Invalid)
        } else {
            self.workspaces
                .rooms_mut()
                .apply(expected_revision, change)
                .err()
        };
        self.broadcast_rooms();
        self.ipc.broadcast(&ShellEvent::RoomMutationResult {
            request_id,
            revision: self.workspaces.rooms().definitions().revision,
            error,
        });
    }
}
