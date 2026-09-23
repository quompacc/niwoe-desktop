//! Persistent room definitions. Runtime windows, outputs and PIDs never belong here.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub mod store;

pub const MAX_ROOMS: usize = 64;
pub const SCHEMA_VERSION: u32 = 1;
pub const LEGACY_ROOMS: usize = 9;
pub const MAX_NAME_CHARS: usize = 64;
pub const MAX_DESCRIPTION_CHARS: usize = 200;

/// Opaque identity; never derived from a name, position or process ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoomId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssignmentMode {
    Free,
    Preferred,
    Dedicated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub description: String,
    pub assignment: AssignmentMode,
}

/// Array order is presentation order, not a compositor Space index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rooms {
    pub schema_version: u32,
    pub revision: u64,
    pub next_id: u64,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomError {
    UnsupportedSchema(u32),
    Invalid(String),
    Conflict,
    Exhausted,
}

impl std::fmt::Display for RoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedSchema(v) => write!(f, "Nicht unterstütztes Raumschema: {v}"),
            Self::Invalid(message) => write!(f, "Ungültige Raumkonfiguration: {message}"),
            Self::Conflict => write!(f, "Raumkonfiguration wurde inzwischen geändert"),
            Self::Exhausted => write!(f, "Raum-ID oder Revision ausgeschöpft"),
        }
    }
}
impl std::error::Error for RoomError {}

impl Rooms {
    /// The original nine slots have no disk room file. Preserve their order and
    /// names once; callers load an existing document rather than recreating it.
    pub fn from_legacy_slots() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            revision: 0,
            next_id: LEGACY_ROOMS as u64 + 1,
            rooms: (1..=LEGACY_ROOMS as u64)
                .map(|id| Room {
                    id: RoomId(id),
                    name: format!("Raum {id}"),
                    description: String::new(),
                    assignment: AssignmentMode::Free,
                })
                .collect(),
        }
    }

    pub fn validate(&self) -> Result<(), RoomError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(RoomError::UnsupportedSchema(self.schema_version));
        }
        let invalid = |message: &str| RoomError::Invalid(message.into());
        if self.rooms.is_empty() || self.rooms.len() > MAX_ROOMS {
            return Err(invalid("Raumanzahl außerhalb 1–64"));
        }
        let mut ids = BTreeSet::new();
        for room in &self.rooms {
            if room.id.0 == 0 || room.id.0 >= self.next_id || !ids.insert(room.id) {
                return Err(invalid("doppelte oder ungültige Raum-ID/Zähler"));
            }
            if room.name.trim().is_empty()
                || room.name.chars().count() > MAX_NAME_CHARS
                || room.name.chars().any(char::is_control)
            {
                return Err(invalid(
                    "Name muss 1–64 Zeichen ohne Steuerzeichen enthalten",
                ));
            }
            if room.description.chars().count() > MAX_DESCRIPTION_CHARS
                || room
                    .description
                    .chars()
                    .any(|c| c.is_control() && c != '\n')
            {
                return Err(invalid(
                    "Beschreibung ist zu lang oder enthält Steuerzeichen",
                ));
            }
        }
        Ok(())
    }

    /// Build and validate a candidate. Publishing/persistence belongs to the
    /// sole compositor writer, never a shell-owned copy of this model.
    pub fn revised(
        &self,
        expected: u64,
        change: impl FnOnce(&mut Self),
    ) -> Result<Self, RoomError> {
        if expected != self.revision {
            return Err(RoomError::Conflict);
        }
        let mut next = self.clone();
        change(&mut next);
        if next.next_id < self.next_id {
            return Err(RoomError::Invalid("ID-Zähler darf nicht sinken".into()));
        }
        next.revision = self.revision.checked_add(1).ok_or(RoomError::Exhausted)?;
        next.validate_successor(self)?;
        Ok(next)
    }

    pub fn validate_successor(&self, previous: &Self) -> Result<(), RoomError> {
        self.validate()?;
        if self.revision
            != previous
                .revision
                .checked_add(1)
                .ok_or(RoomError::Exhausted)?
            || self.next_id < previous.next_id
        {
            return Err(RoomError::Conflict);
        }
        if self.rooms.iter().any(|room| {
            room.id.0 < previous.next_id && !previous.rooms.iter().any(|old| old.id == room.id)
        }) {
            return Err(RoomError::Invalid(
                "Gelöschte IDs dürfen nicht wiederverwendet werden".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
