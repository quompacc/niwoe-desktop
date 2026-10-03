use super::*;
#[test]
fn legacy_thumbnail_command_defaults_to_uncorrelated() {
    let command = decode_command(
        r#"{"type":"capture-window-thumbnail","id":"window","max_width":200,"max_height":112}"#,
    )
    .unwrap();
    assert!(matches!(
        command,
        ShellCommand::CaptureWindowThumbnail {
            request_id: None,
            ..
        }
    ));
}
#[test]
fn correlated_thumbnail_and_lock_roundtrip() {
    for event in [
        ShellEvent::WindowThumbnail {
            request_id: Some("hub-7".into()),
            id: "window".into(),
            path: "/run/user/1000/niwoe-thumb-test.xrgb".into(),
            width: 100,
            height: 50,
        },
        ShellEvent::SessionLocked,
    ] {
        let bytes = encode_event(&event).unwrap();
        assert_eq!(
            decode_event(std::str::from_utf8(&bytes).unwrap()).unwrap(),
            event
        );
    }
}
