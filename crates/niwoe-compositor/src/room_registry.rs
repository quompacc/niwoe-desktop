//! Compositor-owned room identity. Presentation order never moves a Space.
use niwoe_config::rooms::{store, RoomError, RoomId, Rooms, LEGACY_ROOMS};
use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

pub struct RoomRegistry {
    definitions: Rooms,
    slots: Vec<RoomId>,
    // OS-owned lock is released even after a crash. Never unlink the lock file:
    // another process may already be waiting on the same inode.
    _writer: Option<File>,
}

impl RoomRegistry {
    pub fn from_definitions(definitions: Rooms) -> Result<Self, RoomError> {
        definitions.validate()?;
        let mut slots: Vec<_> = definitions.rooms.iter().map(|room| room.id).collect();
        slots.sort_unstable();
        Ok(Self {
            definitions,
            slots,
            _writer: None,
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
