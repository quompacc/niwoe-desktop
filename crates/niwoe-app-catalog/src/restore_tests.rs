use super::DesktopApp;

#[test]
fn documents_use_only_explicit_standalone_file_slots() {
    let app = |exec| {
        DesktopApp::parse(&format!("[Desktop Entry]\nType=Application\nName=Restore test\nExec={exec}\nStartupWMClass=RestoreTest\n")).unwrap()
    };
    let supported = app("/usr/bin/true --read %f");
    assert_eq!(supported.startup_wm_class.as_deref(), Some("RestoreTest"));
    assert!(supported.file_argv.is_some());
    for command in [
        "/usr/bin/true",
        "/usr/bin/true --read=%f",
        "/usr/bin/true %u",
        "/usr/bin/true %f %F",
        "%f /usr/bin/true",
    ] {
        assert!(app(command).file_argv.is_none(), "{command}");
    }
}

#[cfg(unix)]
#[test]
fn shell_characters_and_spaces_remain_a_single_literal_argument() {
    let path = std::env::temp_dir().join(format!(
        "niwoe-file-{}-$(echo unsafe) with spaces.txt",
        std::process::id()
    ));
    std::fs::write(&path, "test").unwrap();
    let app = DesktopApp::parse(
        "[Desktop Entry]\nType=Application\nName=Test\nExec=/usr/bin/true --read %f\n",
    )
    .unwrap();
    let (program, args) = app.restore_argv(path.to_str()).unwrap();
    assert_eq!(program, "/usr/bin/true");
    assert_eq!(
        args,
        vec!["--read".to_string(), path.to_str().unwrap().to_string()]
    );
    std::fs::remove_file(&path).unwrap();
    assert!(app.restore_argv(path.to_str()).is_err());
    let missing = DesktopApp::new(
        "Missing".into(),
        vec!["/does-not-exist-niwoe-p09".into()],
        false,
    );
    assert!(missing.restore_argv(None).is_err());
}

#[test]
fn duplicate_exec_cannot_replace_verified_program_for_file_launch() {
    let app = DesktopApp::parse("[Desktop Entry]\nType=Application\nName=Test\nExec=/usr/bin/true\nExec=/different-program %f\n").unwrap();
    assert_eq!(app.program, "/usr/bin/true");
    assert!(app.file_argv.is_none());
}
