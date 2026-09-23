use super::*;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static SERIAL: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "niwoe-rooms-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
    fn path(&self) -> PathBuf {
        self.0.join("rooms.toml")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn migration_is_idempotent_and_preserves_names_and_order_after_edits() {
    let f = Fixture::new();
    let original = store::load_or_initialize(&f.path()).unwrap();
    assert_eq!(original.rooms.len(), 9);
    assert_eq!(original.rooms[8].name, "Raum 9");
    let next = original
        .revised(0, |r| {
            r.rooms[0].name = "Arbeit".into();
            r.rooms.swap(0, 8);
        })
        .unwrap();
    store::save(&f.path(), &original, &next).unwrap();
    assert_eq!(store::load_or_initialize(&f.path()).unwrap(), next);
    assert_eq!(next.rooms[8].id, original.rooms[0].id);
}

#[test]
fn invalid_or_future_file_is_never_overwritten() {
    let f = Fixture::new();
    for contents in [
        "broken!",
        "schema_version = 999\nnew_field = true",
        "schema_version = 1\n",
    ] {
        fs::write(f.path(), contents).unwrap();
        assert!(store::load_or_initialize(&f.path()).is_err());
        assert_eq!(fs::read_to_string(f.path()).unwrap(), contents);
    }
}

#[test]
fn stale_save_retains_latest_file_and_failed_candidate_retains_memory() {
    let f = Fixture::new();
    let old = store::load_or_initialize(&f.path()).unwrap();
    let a = old.revised(0, |r| r.rooms[0].name = "A".into()).unwrap();
    let b = old.revised(0, |r| r.rooms[0].name = "B".into()).unwrap();
    store::save(&f.path(), &old, &a).unwrap();
    assert!(matches!(
        store::save(&f.path(), &old, &b),
        Err(store::StoreError::Invalid(RoomError::Conflict))
    ));
    assert_eq!(store::load(&f.path()).unwrap(), a);
    assert!(old.revised(0, |r| r.rooms.clear()).is_err());
    assert_eq!(old.rooms.len(), 9);
    assert!(matches!(old.revised(1, |_| {}), Err(RoomError::Conflict)));
}

#[test]
fn identity_counter_never_reuses_removed_ids() {
    let original = Rooms::from_legacy_slots();
    let removed = original
        .revised(0, |r| {
            r.rooms.remove(0);
        })
        .unwrap();
    assert!(removed
        .revised(1, |r| r.rooms.push(original.rooms[0].clone()))
        .is_err());
    assert!(removed.revised(1, |r| r.next_id = 2).is_err());
    assert!(original
        .revised(0, |r| r.rooms[0].id = r.rooms[1].id)
        .is_err());
}

#[test]
fn unicode_limits_empty_names_counts_and_schema_are_checked() {
    let old = Rooms::from_legacy_slots();
    assert!(old.revised(0, |r| r.rooms[0].name = "ä".repeat(64)).is_ok());
    for name in [" ".into(), "ä".repeat(65), "line\nbreak".into()] {
        assert!(old.revised(0, |r| r.rooms[0].name = name).is_err());
    }
    assert!(old
        .revised(0, |r| r.rooms[0].description = "ä".repeat(201))
        .is_err());
    assert!(old.revised(0, |r| r.schema_version = 2).is_err());
    let maximum = old
        .revised(0, |r| {
            for id in 10..=64 {
                r.rooms.push(Room {
                    id: RoomId(id),
                    name: format!("Room {id}"),
                    description: String::new(),
                    assignment: AssignmentMode::Free,
                });
            }
            r.next_id = 65;
        })
        .unwrap();
    assert_eq!(maximum.rooms.len(), MAX_ROOMS);
    assert!(maximum
        .revised(1, |r| {
            let mut extra = r.rooms[0].clone();
            extra.id = RoomId(65);
            r.rooms.push(extra);
            r.next_id = 66;
        })
        .is_err());
}

#[test]
fn concurrent_initialization_publishes_one_complete_document() {
    let f = Fixture::new();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let path = f.path();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                store::load_or_initialize(&path).unwrap()
            })
        })
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), Rooms::from_legacy_slots());
    }
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 1);
}

#[test]
fn invalid_save_leaves_file_byte_identical() {
    let f = Fixture::new();
    let old = store::load_or_initialize(&f.path()).unwrap();
    let before = fs::read(f.path()).unwrap();
    let mut invalid = old.clone();
    invalid.rooms.clear();
    assert!(store::save(&f.path(), &old, &invalid).is_err());
    assert_eq!(fs::read(f.path()).unwrap(), before);
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 1);
}

#[test]
fn oversized_file_is_preserved() {
    let f = Fixture::new();
    let bytes = vec![b' '; store::MAX_FILE_BYTES as usize + 1];
    fs::write(f.path(), &bytes).unwrap();
    assert!(store::load_or_initialize(&f.path()).is_err());
    assert_eq!(fs::read(f.path()).unwrap(), bytes);
}

#[cfg(unix)]
#[test]
fn published_file_is_private_and_symlink_is_not_followed() {
    use std::os::unix::{fs::symlink, fs::PermissionsExt};
    let f = Fixture::new();
    store::load_or_initialize(&f.path()).unwrap();
    assert_eq!(
        fs::metadata(f.path()).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let link = f.0.join("link.toml");
    symlink(f.path(), &link).unwrap();
    assert!(store::load_or_initialize(&link).is_err());
}
