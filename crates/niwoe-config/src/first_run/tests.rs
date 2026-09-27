use super::*;

struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "niwoe-p11-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(path.join("niwoe")).unwrap();
        Self(path)
    }
    fn directory(&self) -> std::path::PathBuf {
        self.0.join("niwoe")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn origin_is_decided_before_rooms_and_survives_restarts() {
    let f = Fixture::new();
    let state = FirstRun::initialize(&f.directory()).unwrap();
    assert!(state.fresh && !state.completed);
    fs::write(f.directory().join("rooms.toml"), "later initialization").unwrap();
    assert_eq!(FirstRun::initialize(&f.directory()).unwrap(), state);
}

#[test]
fn existing_and_legacy_profiles_are_never_fresh_from_missing_marker() {
    for legacy in [false, true] {
        let f = Fixture::new();
        if legacy {
            fs::create_dir(f.0.join("meridian")).unwrap();
        } else {
            fs::write(f.directory().join("config.toml"), "individual").unwrap();
        }
        assert!(!FirstRun::initialize(&f.directory()).unwrap().fresh);
    }
}

#[test]
fn corrupt_unknown_and_stale_state_remain_untouched() {
    let f = Fixture::new();
    let path = f.directory().join("first-run.toml");
    let old = FirstRun::initialize(&f.directory()).unwrap();
    let mut next = old.clone();
    next.revision += 1;
    next.completed = true;
    next.save(&path, &old).unwrap();
    assert!(next.save(&path, &old).is_err());
    assert!(FirstRun::load(&path).unwrap().completed);
    for text in [
        "broken = [",
        "schema_version = 99\nrevision = 0\nfresh = true\ncompleted = false",
    ] {
        fs::write(&path, text).unwrap();
        assert!(FirstRun::initialize(&f.directory()).is_err());
        assert!(next.save(&path, &old).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), text);
    }
}
