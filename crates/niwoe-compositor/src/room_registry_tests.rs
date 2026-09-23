use super::*;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static SERIAL: AtomicU64 = AtomicU64::new(0);
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
fn unsupported_dynamic_count_does_not_rewrite_saved_rooms() {
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
    assert!(RoomRegistry::open(&f.0).is_err());
    assert_eq!(fs::read(&path).unwrap(), before);
}
