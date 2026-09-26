//! Compositor-owned room identity. Presentation order never moves a Space.
use niwoe_config::rooms::{store, AssignmentMode, Room, RoomError, RoomId, Rooms, MAX_ROOMS};
use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
};

pub struct RoomRegistry {
    definitions: Rooms,
    slots: Vec<RoomId>,
    // OS-owned lock is released even after a crash. Never unlink the lock file:
    // another process may already be waiting on the same inode.
    _writer: Option<File>,
    path: Option<PathBuf>,
}

impl RoomRegistry {
    pub fn apply(
        &mut self,
        expected: u64,
        change: niwoe_ipc::RoomChange,
    ) -> Result<(), niwoe_ipc::RoomMutationError> {
        use niwoe_ipc::{RoomChange, RoomMutationError as Error};
        if expected != self.definitions.revision {
            return Err(Error::Conflict);
        }
        let index = match &change {
            RoomChange::Create { .. } | RoomChange::CreateDetails { .. } => None,
            RoomChange::Rename { id, .. }
            | RoomChange::UpdateDetails { id, .. }
            | RoomChange::SetDescription { id, .. }
            | RoomChange::SetAssignment { id, .. }
            | RoomChange::Move { id, .. }
            | RoomChange::Delete { id, .. } => Some(
                self.definitions
                    .rooms
                    .iter()
                    .position(|r| r.id.0 == *id)
                    .ok_or(Error::Invalid)?,
            ),
        };
        if matches!(&change, RoomChange::Move { position, .. } if *position >= self.slots.len()) {
            return Err(Error::Invalid);
        }
        if let RoomChange::Delete { id, target_id } = &change {
            if id == target_id
                || self.slots.len() <= 1
                || self.slot_for_room(RoomId(*target_id)).is_none()
            {
                return Err(Error::Invalid);
            }
        }
        if matches!(
            &change,
            RoomChange::Create { .. } | RoomChange::CreateDetails { .. }
        ) && (self.slots.len() >= MAX_ROOMS || self.definitions.next_id == u64::MAX)
        {
            return Err(Error::Invalid);
        }
        let created_id = self.definitions.next_id;
        let next = self
            .definitions
            .revised(expected, |rooms| match change {
                RoomChange::Create { name } => {
                    rooms.rooms.push(Room {
                        id: RoomId(created_id),
                        name,
                        description: String::new(),
                        assignment: AssignmentMode::Free,
                    });
                    rooms.next_id += 1;
                }
                RoomChange::CreateDetails {
                    name,
                    description,
                    assignment,
                } => {
                    rooms.rooms.push(Room {
                        id: RoomId(created_id),
                        name,
                        description,
                        assignment: assignment_mode(assignment),
                    });
                    rooms.next_id += 1;
                }
                RoomChange::UpdateDetails {
                    name, description, ..
                } => {
                    let room = &mut rooms.rooms[index.expect("validated room")];
                    room.name = name;
                    room.description = description;
                }
                RoomChange::Rename { name, .. } => {
                    rooms.rooms[index.expect("validated room")].name = name
                }
                RoomChange::SetDescription { description, .. } => {
                    rooms.rooms[index.expect("validated room")].description = description;
                }
                RoomChange::SetAssignment { assignment, .. } => {
                    rooms.rooms[index.expect("validated room")].assignment = match assignment {
                        niwoe_ipc::RoomAssignment::Free => AssignmentMode::Free,
                        niwoe_ipc::RoomAssignment::Preferred => AssignmentMode::Preferred,
                        niwoe_ipc::RoomAssignment::Dedicated => AssignmentMode::Dedicated,
                    };
                }
                RoomChange::Move { position, .. } => {
                    let room = rooms.rooms.remove(index.expect("validated room"));
                    rooms.rooms.insert(position, room);
                }
                RoomChange::Delete { .. } => {
                    rooms.rooms.remove(index.expect("validated room"));
                }
            })
            .map_err(|_| Error::Invalid)?;
        let path = self.path.as_ref().ok_or(Error::Storage)?;
        match store::save(path, &self.definitions, &next) {
            Ok(()) => {
                self.sync_slots(&next);
                self.definitions = next;
                Ok(())
            }
            Err(store::StoreError::PublishedButNotSynced(_)) => {
                self.sync_slots(&next);
                self.definitions = next;
                Err(Error::Durability)
            }
            Err(store::StoreError::Invalid(RoomError::Conflict)) => Err(Error::Conflict),
            Err(error) => {
                tracing::warn!(%error, "room mutation could not be persisted");
                Err(Error::Storage)
            }
        }
    }

    fn sync_slots(&mut self, next: &Rooms) {
        self.slots
            .retain(|id| next.rooms.iter().any(|room| room.id == *id));
        for room in &next.rooms {
            if !self.slots.contains(&room.id) {
                self.slots.push(room.id);
            }
        }
    }

    pub fn from_definitions(definitions: Rooms) -> Result<Self, RoomError> {
        definitions.validate()?;
        let mut slots: Vec<_> = definitions.rooms.iter().map(|room| room.id).collect();
        slots.sort_unstable();
        Ok(Self {
            definitions,
            slots,
            _writer: None,
            path: None,
        })
    }

    pub fn open(directory: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        fs::create_dir_all(directory)?;
        let lock_path = directory.join("rooms.lock");
        if fs::symlink_metadata(&lock_path).is_ok_and(|m| !m.is_file()) {
            return Err("Raumsperre ist keine reguläre Datei".into());
        }
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let writer = options.open(lock_path)?;
        writer
            .try_lock()
            .map_err(|e| format!("Raumkonfiguration bereits gesperrt oder nicht sperrbar: {e}"))?;
        let definitions = store::load_or_initialize(&directory.join("rooms.toml"))?;
        let mut registry = Self::from_definitions(definitions)?;
        registry._writer = Some(writer);
        registry.path = Some(directory.join("rooms.toml"));
        Ok(registry)
    }

    pub fn definitions(&self) -> &Rooms {
        &self.definitions
    }
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
    pub fn room_at_slot(&self, slot: usize) -> Option<RoomId> {
        self.slots.get(slot).copied()
    }
    pub fn slot_for_room(&self, id: RoomId) -> Option<usize> {
        self.slots.iter().position(|candidate| *candidate == id)
    }
    pub fn slot_at_position(&self, position: usize) -> Option<usize> {
        self.definitions
            .rooms
            .get(position)
            .and_then(|room| self.slot_for_room(room.id))
    }
}

fn assignment_mode(value: niwoe_ipc::RoomAssignment) -> AssignmentMode {
    match value {
        niwoe_ipc::RoomAssignment::Free => AssignmentMode::Free,
        niwoe_ipc::RoomAssignment::Preferred => AssignmentMode::Preferred,
        niwoe_ipc::RoomAssignment::Dedicated => AssignmentMode::Dedicated,
    }
}

#[cfg(test)]
#[path = "room_registry_tests.rs"]
mod tests;
