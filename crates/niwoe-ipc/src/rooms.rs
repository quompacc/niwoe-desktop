use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomEntry {
    pub id: u64,
    /// Stable one-based Space slot for legacy window/output consumers.
    pub workspace: u8,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub assignment: RoomAssignment,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RoomAssignment {
    #[default]
    Free,
    Preferred,
    Dedicated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomSnapshot {
    pub revision: u64,
    /// Presentation order only; never use this index as a Space slot.
    pub rooms: Vec<RoomEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case")]
pub enum RoomChange {
    Create { name: String },
    Rename { id: u64, name: String },
    SetDescription { id: u64, description: String },
    SetAssignment { id: u64, assignment: RoomAssignment },
    Move { id: u64, position: usize },
    Delete { id: u64, target_id: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RoomMutationError {
    Invalid,
    Conflict,
    Storage,
    Durability,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mutation_and_snapshot_wire_roundtrip() {
        let command = crate::ShellCommand::MutateRoom {
            request_id: "edit-1".into(),
            expected_revision: 7,
            change: RoomChange::Rename {
                id: 42,
                name: "Büro".into(),
            },
        };
        assert_eq!(
            crate::decode_command(
                std::str::from_utf8(&crate::encode_command(&command).unwrap()).unwrap()
            )
            .unwrap(),
            command
        );
        let event = crate::ShellEvent::RoomSnapshot {
            snapshot: RoomSnapshot {
                revision: 8,
                rooms: vec![RoomEntry {
                    id: 42,
                    workspace: 1,
                    name: "Büro".into(),
                    description: String::new(),
                    assignment: RoomAssignment::Free,
                }],
            },
        };
        assert_eq!(
            crate::decode_event(
                std::str::from_utf8(&crate::encode_event(&event).unwrap()).unwrap()
            )
            .unwrap(),
            event
        );
    }

    #[test]
    fn create_and_delete_wire_roundtrip() {
        for change in [
            RoomChange::Create {
                name: "Neuer Raum".into(),
            },
            RoomChange::Delete {
                id: 12,
                target_id: 1,
            },
            RoomChange::SetDescription {
                id: 1,
                description: "Arbeit".into(),
            },
            RoomChange::SetAssignment {
                id: 1,
                assignment: RoomAssignment::Preferred,
            },
        ] {
            let command = crate::ShellCommand::MutateRoom {
                request_id: "room-edit".into(),
                expected_revision: 4,
                change,
            };
            let wire = crate::encode_command(&command).unwrap();
            assert_eq!(
                crate::decode_command(std::str::from_utf8(&wire).unwrap()).unwrap(),
                command
            );
        }
    }
}
