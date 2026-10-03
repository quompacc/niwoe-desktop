use super::*;

#[test]
fn stored_hidden_from_menu_handler_keeps_name_without_becoming_a_choice() {
    let dir = std::env::temp_dir().join(format!("niwoe-mime-metadata-test-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let metadata = "[Desktop Entry]\nType=Application\nName=Installed Handler\nExec=viewer %U\nIcon=viewer\nMimeType=image/jpeg;\n";
    fs::write(dir.join("visible.desktop"), metadata).unwrap();
    fs::write(
        dir.join("handler.desktop"),
        metadata.to_owned() + "NoDisplay=true\n",
    )
    .unwrap();
    fs::write(
        dir.join("deleted.desktop"),
        metadata.to_owned() + "Hidden=true\n",
    )
    .unwrap();
    let index = MimeAppIndex::load_from_dirs(vec![dir.clone()]);
    assert_eq!(
        index.lookup("handler.desktop").unwrap().name,
        "Installed Handler"
    );
    assert!(index.lookup("deleted.desktop").is_none());
    assert_eq!(
        index
            .apps_for_mime("image/jpeg")
            .iter()
            .map(|app| app.desktop_id.as_str())
            .collect::<Vec<_>>(),
        vec!["visible.desktop"]
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn category_metadata_matches_per_variant() {
    for cat in DefaultAppCategory::ALL {
        assert!(!cat.label().is_empty(), "label missing for {:?}", cat);
        assert!(
            !cat.representative_mime().is_empty(),
            "representative mime missing for {:?}",
            cat
        );
        assert!(
            cat.all_mimes()
                .iter()
                .any(|m| *m == cat.representative_mime()),
            "all_mimes must include representative_mime for {:?}",
            cat
        );
        assert!(
            !cat.preferred_desktop_ids().is_empty(),
            "preferred desktop ids missing for {:?}",
            cat
        );
    }
}

#[test]
fn parse_minimal_desktop_entry_with_mimes() {
    let raw = r#"
[Desktop Entry]
Type=Application
Name=Demo Viewer
Exec=demo %U
Icon=demo
MimeType=image/png;image/jpeg;
"#;
    let path = std::path::PathBuf::from("/tmp/demo.desktop");
    std::fs::write(&path, raw).unwrap();
    let app = parse_mime_candidate(&path).expect("parsed");
    assert_eq!(app.desktop_id, "demo.desktop");
    assert_eq!(app.name, "Demo Viewer");
    assert_eq!(app.icon.as_deref(), Some("demo"));
    assert_eq!(
        app.mime_types,
        vec!["image/png".to_string(), "image/jpeg".to_string()]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn parse_hidden_entry_is_skipped() {
    let raw =
        "[Desktop Entry]\nType=Application\nName=Hidden\nMimeType=text/plain\nNoDisplay=true\n";
    let path = std::path::PathBuf::from("/tmp/hidden-default-apps-test.desktop");
    std::fs::write(&path, raw).unwrap();
    assert!(parse_mime_candidate(&path).is_none());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn index_apps_for_mime_returns_only_matching() {
    let mut idx = MimeAppIndex::default();
    idx.apps.push(MimeAppCandidate {
        desktop_id: "a.desktop".to_string(),
        name: "Alpha".to_string(),
        icon: None,
        mime_types: vec!["image/png".to_string()],
    });
    idx.apps.push(MimeAppCandidate {
        desktop_id: "b.desktop".to_string(),
        name: "Bravo".to_string(),
        icon: None,
        mime_types: vec!["text/plain".to_string()],
    });
    idx.by_mime.insert("image/png".to_string(), vec![0]);
    idx.by_mime.insert("text/plain".to_string(), vec![1]);
    let png = idx.apps_for_mime("image/png");
    assert_eq!(png.len(), 1);
    assert_eq!(png[0].desktop_id, "a.desktop");
    let none = idx.apps_for_mime("audio/mpeg");
    assert!(none.is_empty());
}
