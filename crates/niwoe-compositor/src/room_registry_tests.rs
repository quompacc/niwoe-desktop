use super::*;
use niwoe_config::rooms::LEGACY_ROOMS;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static SERIAL: AtomicU64 = AtomicU64::new(0);

#[test]
fn complete_form_is_validated_and_persisted_atomically() {
    use niwoe_ipc::{RoomAssignment, RoomChange, RoomMutationError};
    let f = Fixture::new();
    let mut registry = RoomRegistry::open(&f.0).unwrap();
    let original = registry.definitions().clone();
    assert_eq!(
        registry.apply(
            0,
            RoomChange::UpdateDetails {
                id: 1,
                name: "Nicht teilweise speichern".into(),
                description: "x".repeat(201),
            }
        ),
        Err(RoomMutationError::Invalid)
    );
    assert_eq!(registry.definitions(), &original);
    registry
        .apply(
            0,
            RoomChange::CreateDetails {
                name: "Kontext".into(),
                description: "Beschreibung".into(),
                assignment: RoomAssignment::Preferred,
            },
        )
        .unwrap();
    assert_eq!(registry.slot_for_room(RoomId(10)), Some(9));
    registry
        .apply(
            1,
            RoomChange::UpdateDetails {
                id: 10,
                name: "Neu".into(),
                description: "Zusammen gespeichert".into(),
            },
        )
        .unwrap();
    let expected = registry.definitions().clone();
    assert_eq!(expected.revision, 2);
    assert_eq!(
        expected.rooms.last().unwrap().assignment,
        AssignmentMode::Preferred
    );
    drop(registry);
    assert_eq!(RoomRegistry::open(&f.0).unwrap().definitions(), &expected);
}
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let directory = std::env::temp_dir().join(format!(
            "niwoe-room-registry-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).unwrap();
        Self(directory)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn rename_and_move_persist_without_changing_slots_and_reject_stale_writes() {
    use niwoe_ipc::{RoomChange, RoomMutationError};
    let f = Fixture::new();
    let mut registry = RoomRegistry::open(&f.0).unwrap();
    registry
        .apply(
            0,
            RoomChange::Rename {
                id: 1,
                name: "Entwicklung".into(),
            },
        )
        .unwrap();
    assert_eq!(
        registry.apply(0, RoomChange::Move { id: 1, position: 8 }),
        Err(RoomMutationError::Conflict)
    );
    registry
        .apply(1, RoomChange::Move { id: 1, position: 8 })
        .unwrap();
    assert_eq!(registry.slot_for_room(RoomId(1)), Some(0));
    assert_eq!(registry.slot_at_position(8), Some(0));
    let snapshot = registry.definitions().clone();
    assert_eq!(snapshot.rooms[8].name, "Entwicklung");
    assert!(registry
        .apply(
            2,
            RoomChange::Rename {
                id: 1,
                name: " ".into()
            }
        )
        .is_err());
    assert!(registry
        .apply(
            2,
            RoomChange::Move {
                id: 999,
                position: 0
            }
        )
        .is_err());
    assert!(registry
        .apply(2, RoomChange::Move { id: 1, position: 9 })
        .is_err());
    assert_eq!(registry.definitions(), &snapshot);
    drop(registry);
    assert_eq!(RoomRegistry::open(&f.0).unwrap().definitions(), &snapshot);
}

#[test]
fn failed_disk_write_never_publishes_candidate_in_memory() {
    let f = Fixture::new();
    let mut registry = RoomRegistry::open(&f.0).unwrap();
    let before = registry.definitions().clone();
    let path = f.0.join("rooms.toml");
    fs::remove_file(&path).unwrap();
    fs::create_dir(&path).unwrap();
    assert_eq!(
        registry.apply(
            0,
            niwoe_ipc::RoomChange::Rename {
                id: 1,
                name: "Unbestätigt".into()
            }
        ),
        Err(niwoe_ipc::RoomMutationError::Storage)
    );
    assert_eq!(registry.definitions(), &before);
}

#[test]
fn display_order_and_name_do_not_change_slot_identity() {
    let original = Rooms::from_legacy_slots();
    let next = original
        .revised(0, |r| {
            r.rooms.reverse();
            r.rooms[0].name = "Recherche".into();
        })
        .unwrap();
    let registry = RoomRegistry::from_definitions(next).unwrap();
    assert_eq!(registry.slot_at_position(0), Some(8));
    for slot in 0..LEGACY_ROOMS {
        let id = RoomId(slot as u64 + 1);
        assert_eq!(registry.room_at_slot(slot), Some(id));
        assert_eq!(registry.slot_for_room(id), Some(slot));
    }
    assert_eq!(registry.slot_for_room(RoomId(999)), None);
    assert_eq!(registry.room_at_slot(9), None);
    assert_eq!(registry.slot_at_position(9), None);
}

#[test]
fn writer_lock_is_exclusive_and_released_on_drop() {
    let f = Fixture::new();
    let first = RoomRegistry::open(&f.0).unwrap();
    assert!(RoomRegistry::open(&f.0).is_err());
    let before = fs::read(f.0.join("rooms.toml")).unwrap();
    drop(first);
    let reopened = RoomRegistry::open(&f.0).unwrap();
    assert_eq!(reopened.definitions(), &Rooms::from_legacy_slots());
    assert_eq!(fs::read(f.0.join("rooms.toml")).unwrap(), before);
}

#[test]
fn damaged_file_is_preserved_and_error_releases_lock() {
    let f = Fixture::new();
    let path = f.0.join("rooms.toml");
    fs::write(&path, "not toml!").unwrap();
    assert!(RoomRegistry::open(&f.0).is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), "not toml!");
    fs::remove_file(&path).unwrap();
    assert!(RoomRegistry::open(&f.0).is_ok());
}

#[test]
fn dynamic_count_reopens_without_rewriting_saved_rooms() {
    let f = Fixture::new();
    let path = f.0.join("rooms.toml");
    let old = store::load_or_initialize(&path).unwrap();
    let next = old
        .revised(0, |r| {
            r.rooms.pop();
        })
        .unwrap();
    store::save(&path, &old, &next).unwrap();
    let before = fs::read(&path).unwrap();
    let registry = RoomRegistry::open(&f.0).unwrap();
    assert_eq!(registry.slot_count(), LEGACY_ROOMS - 1);
    assert_eq!(fs::read(&path).unwrap(), before);
}

#[test]
fn create_delete_keep_stable_ids_and_compact_slots() {
    use niwoe_ipc::{RoomChange, RoomMutationError};
    let f = Fixture::new();
    let mut registry = RoomRegistry::open(&f.0).unwrap();
    registry
        .apply(0, RoomChange::Create { name: "Neu".into() })
        .unwrap();
    assert_eq!(registry.slot_for_room(RoomId(10)), Some(9));
    assert_eq!(registry.slot_count(), 10);
    assert_eq!(
        registry.apply(
            0,
            RoomChange::Delete {
                id: 2,
                target_id: 1
            }
        ),
        Err(RoomMutationError::Conflict)
    );
    registry
        .apply(
            1,
            RoomChange::Delete {
                id: 2,
                target_id: 1,
            },
        )
        .unwrap();
    assert_eq!(registry.slot_for_room(RoomId(2)), None);
    assert_eq!(registry.slot_for_room(RoomId(10)), Some(8));
    assert_eq!(registry.slot_count(), 9);
    assert_eq!(
        registry.apply(
            2,
            RoomChange::Delete {
                id: 1,
                target_id: 1
            }
        ),
        Err(RoomMutationError::Invalid)
    );
    drop(registry);
    let reopened = RoomRegistry::open(&f.0).unwrap();
    assert_eq!(reopened.slot_for_room(RoomId(10)), Some(8));
    assert_eq!(reopened.definitions().next_id, 11);
}

#[test]
fn room_metadata_is_validated_and_persisted() {
    use niwoe_ipc::{RoomAssignment, RoomChange, RoomMutationError};
    let f = Fixture::new();
    let mut registry = RoomRegistry::open(&f.0).unwrap();
    assert_eq!(
        registry.apply(
            0,
            RoomChange::SetDescription {
                id: 1,
                description: "x".repeat(201),
            }
        ),
        Err(RoomMutationError::Invalid)
    );
    registry
        .apply(
            0,
            RoomChange::SetDescription {
                id: 1,
                description: "Entwicklung".into(),
            },
        )
        .unwrap();
    registry
        .apply(
            1,
            RoomChange::SetAssignment {
                id: 1,
                assignment: RoomAssignment::Preferred,
            },
        )
        .unwrap();
    drop(registry);
    let reopened = RoomRegistry::open(&f.0).unwrap();
    assert_eq!(reopened.definitions().rooms[0].description, "Entwicklung");
    assert_eq!(
        reopened.definitions().rooms[0].assignment,
        AssignmentMode::Preferred
    );
}
