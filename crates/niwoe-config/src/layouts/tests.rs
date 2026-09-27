use super::*;

fn snapshot() -> Snapshot {
    Snapshot {
        revision: 1,
        schema_version: VERSION,
        room_id: RoomId(42),
        mode: RoomLayout::Tiling,
        entries: vec![Entry {
            key: 0,
            app: AppReference::Native("org.demo.Editor".into()),
            desktop_id: Some("org.demo.Editor.desktop".into()),
            title: "Document".into(),
            file: None,
            floating: false,
            geometry: Geometry {
                x: 100,
                y: 80,
                width: 600,
                height: 400,
            },
            output: None,
        }],
        tree: vec![Node::Window { key: 0 }],
    }
}

#[test]
fn matching_never_uses_start_order_or_cross_protocol_names() {
    let mut s = snapshot();
    let app = s.entries[0].app.clone();
    let mut second = s.entries[0].clone();
    second.key = 1;
    second.title = "Second".into();
    s.entries.push(second);
    let live = vec![
        (app.clone(), "Second".into()),
        (app.clone(), "Document".into()),
    ];
    assert_eq!(matching::unique_match(&s.entries[0], &s, &live), Some(1));
    assert_eq!(matching::unique_match(&s.entries[1], &s, &live), Some(0));
    let ambiguous = vec![
        (app.clone(), "Document".into()),
        (app.clone(), "Document".into()),
    ];
    assert_eq!(matching::unique_match(&s.entries[0], &s, &ambiguous), None);
    s.entries[1].title = "Document".into();
    assert_eq!(matching::unique_match(&s.entries[0], &s, &live), None);
    s.entries.pop();
    assert_eq!(
        matching::unique_match(
            &s.entries[0],
            &s,
            &[(
                AppReference::Xwayland("org.demo.Editor".into()),
                "Document".into()
            )]
        ),
        None
    );
    s.entries[0].file = Some("/tmp/document.txt".into());
    assert_eq!(
        matching::unique_match(&s.entries[0], &s, &[(app, "Unrelated document".into())]),
        None
    );
}

#[test]
fn roundtrip_has_stable_room_and_local_keys_only() {
    let original = snapshot();
    original.validate().unwrap();
    let text = toml::to_string(&original).unwrap();
    let decoded: Snapshot = toml::from_str(&text).unwrap();
    assert_eq!(decoded, original);
    assert!(!text.contains("window_id"));
}

#[test]
fn invalid_tree_and_incompatible_schema_are_rejected() {
    let mut s = snapshot();
    s.schema_version += 1;
    assert!(s.validate().is_err());
    s.schema_version = VERSION;
    s.tree.push(Node::Split {
        horizontal: true,
        ratio_millis: 500,
        left: 0,
        right: 0,
    });
    assert!(s.validate().is_err());
    s.tree = vec![Node::Window { key: 99 }];
    assert!(s.validate().is_err());
    s.tree = vec![Node::Split {
        horizontal: false,
        ratio_millis: 500,
        left: 1,
        right: 2,
    }];
    assert!(s.validate().is_err());
}

#[test]
fn limits_and_file_paths_are_validated_without_requiring_installation() {
    let mut s = snapshot();
    s.entries[0].file = Some("relative.txt".into());
    assert!(s.validate().is_err());
    s.entries[0].file = Some("/missing/document.txt".into());
    assert!(s.validate().is_ok());
    s.entries[0].desktop_id = Some("../../exec.desktop".into());
    assert!(s.validate().is_err());
    s.entries[0].desktop_id = None;
    s.entries = vec![s.entries[0].clone(); MAX_WINDOWS + 1];
    assert!(s.validate().is_err());
}

#[test]
fn geometry_is_reachable_for_extreme_inputs_and_smaller_scaled_outputs() {
    let area = Geometry {
        x: -1920,
        y: 48,
        width: 960,
        height: 492,
    };
    for g in [
        Geometry {
            x: i32::MAX,
            y: i32::MIN,
            width: i32::MAX,
            height: -1,
        },
        Geometry {
            x: 3000,
            y: 2000,
            width: 1200,
            height: 900,
        },
    ] {
        let fit = g.fit(&area);
        assert!(fit.x >= area.x && fit.y >= area.y);
        assert!(fit.width > 0 && fit.height > 0);
        assert!(fit.x + fit.width <= area.x + area.width);
        assert!(fit.y + fit.height <= area.y + area.height);
    }
}

#[test]
fn atomic_store_retains_invalid_data_and_reports_write_failure() {
    let dir = std::env::temp_dir().join(format!(
        "niwoe-layout-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("layout.toml");
    store::save(&path, &snapshot()).unwrap();
    assert_eq!(store::load(&path).unwrap(), snapshot());
    assert!(
        store::save(&path, &snapshot()).is_err(),
        "stale writer must conflict"
    );
    let mut next = snapshot();
    next.revision += 1;
    store::save(&path, &next).unwrap();
    assert_eq!(store::load(&path).unwrap(), next);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    for text in ["invalid = [", "schema_version = 99"] {
        std::fs::write(&path, text).unwrap();
        assert!(store::load(&path).is_err());
        assert!(store::save(&path, &snapshot()).is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), text);
    }
    assert!(store::save(&path.join("child"), &snapshot()).is_err());
    std::fs::remove_dir_all(dir).unwrap();
}
