use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomEntry {
    pub id: u64,
    /// Stable one-based Space slot for legacy window/output consumers.
    pub workspace: u8,
    pub name: String,
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
    Rename { id: u64, name: String },
    Move { id: u64, position: usize },
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
}
