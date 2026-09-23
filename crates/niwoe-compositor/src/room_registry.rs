//! Compositor-owned room identity. Presentation order never moves a Space.
use niwoe_config::rooms::{store, RoomError, RoomId, Rooms, LEGACY_ROOMS};
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
        let id = match &change {
            RoomChange::Rename { id, .. } | RoomChange::Move { id, .. } => *id,
        };
        let index = self
            .definitions
            .rooms
            .iter()
            .position(|r| r.id.0 == id)
            .ok_or(Error::Invalid)?;
        if matches!(&change, RoomChange::Move { position, .. } if *position >= self.slots.len()) {
            return Err(Error::Invalid);
        }
        let next = self
            .definitions
            .revised(expected, |rooms| match change {
                RoomChange::Rename { name, .. } => rooms.rooms[index].name = name,
                RoomChange::Move { position, .. } => {
                    let room = rooms.rooms.remove(index);
                    rooms.rooms.insert(position, room);
                }
            })
            .map_err(|_| Error::Invalid)?;
        let path = self.path.as_ref().ok_or(Error::Storage)?;
        match store::save(path, &self.definitions, &next) {
            Ok(()) => {
                self.definitions = next;
                Ok(())
            }
            Err(store::StoreError::PublishedButNotSynced(_)) => {
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

    /// Temporary nine-slot compatibility boundary. Do not accept a larger
    /// catalog until all legacy fixed-size shell/WM consumers have migrated.
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
        if definitions.rooms.len() != LEGACY_ROOMS {
            return Err("Dieser Übergangsstand unterstützt genau neun persistente Räume; Datei bleibt unverändert".into());
        }
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

#[cfg(test)]
#[path = "room_registry_tests.rs"]
mod tests;
