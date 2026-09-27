#[test]
fn workspace_changed_clamps_workspace_range() {
    let mut active = 1u8;
    apply_workspace_changed(&mut active, 0);
    assert_eq!(active, 0);
    apply_workspace_changed(&mut active, 2);
    assert_eq!(active, 2);
    apply_workspace_changed(&mut active, 99);
    assert_eq!(active, niwoe_config::rooms::MAX_ROOMS as u8);
}

#[test]
fn lobby_snapshot_has_no_active_room_but_keeps_window_room_counts() {
    let mut active = 1u8;
    let mut windows = Vec::new();
    let mut counts = [0u16; niwoe_config::rooms::MAX_ROOMS];
    apply_full_window_snapshot(
        &mut active,
        &mut windows,
        &mut counts,
        0,
        vec![WindowSnapshotEntry {
            workspace: 1,
            id: "room-one-window".into(),
            title: "Background app".into(),
            minimized: false,
            app_id: None,
        }],
    );
    assert_eq!(active, 0);
    assert_eq!(counts[0], 1);
    assert_eq!(windows[0].workspace, 1);
}

#[test]
fn panel_global_activation_point_uses_top_panel_y() {
    assert_eq!(
        panel_global_activation_point((938.2, 20.7), Some(800)),
        crate::status_notifier::ActivationPoint { x: 938, y: 21 }
    );
    assert_eq!(
        panel_global_activation_point((938.2, 20.7), None),
        crate::status_notifier::ActivationPoint { x: 938, y: 21 }
    );
}

#[test]
fn full_snapshot_recalculates_counts_and_active_workspace() {
    let mut active = 1u8;
    let mut windows = Vec::new();
    let mut counts = [0u16; niwoe_config::rooms::MAX_ROOMS];
    apply_full_window_snapshot(
        &mut active,
        &mut windows,
        &mut counts,
        2,
        vec![
            WindowSnapshotEntry {
                workspace: 1,
                id: "id-1".into(),
                title: "A".into(),
                minimized: false,
                app_id: None,
            },
            WindowSnapshotEntry {
                workspace: 3,
                id: "id-2".into(),
                title: "B".into(),
                minimized: false,
                app_id: None,
            },
            WindowSnapshotEntry {
                workspace: 3,
                id: "id-3".into(),
                title: "C".into(),
                minimized: true,
                app_id: None,
            },
        ],
    );
    assert_eq!(active, 2);
    assert_eq!(counts[0], 1);
    assert_eq!(counts[2], 2);
    assert_eq!(windows.len(), 3);
    let occupied = compute_occupied_workspaces(&counts);
    assert!(occupied[0]);
    assert!(occupied[2]);
    assert!(!occupied[1]);
}

#[test]
fn empty_snapshot_marks_all_workspaces_empty() {
    let mut active = 5u8;
    let mut windows = vec![WindowInfo {
        id: "stale".into(),
        title: "stale".into(),
        workspace: 1,
        minimized: false,
        app_id: None,
    }];
    let mut counts = [3u16; niwoe_config::rooms::MAX_ROOMS];
    apply_full_window_snapshot(&mut active, &mut windows, &mut counts, 1, Vec::new());

    assert_eq!(active, 1);
    assert!(windows.is_empty());
    assert_eq!(counts, [0; niwoe_config::rooms::MAX_ROOMS]);
    assert_eq!(compute_occupied_workspaces(&counts), [false; niwoe_config::rooms::MAX_ROOMS]);
}

#[test]
fn snapshot_with_one_window_marks_single_workspace_occupied() {
    let mut active = 1u8;
    let mut windows = Vec::new();
    let mut counts = [0u16; niwoe_config::rooms::MAX_ROOMS];
    apply_full_window_snapshot(
        &mut active,
        &mut windows,
        &mut counts,
        1,
        vec![WindowSnapshotEntry {
            workspace: 1,
            id: "id-1".into(),
            title: "A".into(),
            minimized: false,
            app_id: None,
        }],
    );

    let occupied = compute_occupied_workspaces(&counts);
    assert!(occupied[0]);
    assert!(occupied[1..].iter().all(|v| !v));
}

#[test]
fn snapshot_workspace_values_out_of_range_are_clamped_safely() {
    let mut active = 1u8;
    let mut windows = Vec::new();
    let mut counts = [0u16; niwoe_config::rooms::MAX_ROOMS];
    apply_full_window_snapshot(
        &mut active,
        &mut windows,
        &mut counts,
        42, // out of range active workspace
        vec![
            WindowSnapshotEntry {
                workspace: 0, // underflow case
                id: "id-1".into(),
                title: "A".into(),
                minimized: false,
                app_id: None,
            },
            WindowSnapshotEntry {
                workspace: 42, // overflow case
                id: "id-2".into(),
                title: "B".into(),
                minimized: true,
                app_id: None,
            },
        ],
    );

    assert_eq!(active, 42);
    assert_eq!(counts[0], 1);
    assert_eq!(counts[41], 1);
    let occupied = compute_occupied_workspaces(&counts);
    assert!(occupied[0]);
    assert!(occupied[41]);
    assert!(occupied[1..41].iter().all(|v| !v));
}

#[test]
fn window_opened_updates_or_inserts_without_crash() {
    let mut windows = vec![WindowInfo {
        id: "id-1".into(),
        title: "old".into(),
        workspace: 2,
        minimized: false,
        app_id: None,
    }];
    apply_window_opened_state(&mut windows, "id-1".into(), "new".into(), None);
    assert_eq!(windows[0].title, "new");
    assert_eq!(windows[0].workspace, 2);

    apply_window_opened_state(&mut windows, "id-2".into(), "B".into(), Some(3));
    assert_eq!(windows.len(), 2);
    assert_eq!(windows[1].workspace, 3);
}

#[test]
fn window_closed_is_safe_for_unknown_id() {
    let mut windows = vec![WindowInfo {
        id: "id-1".into(),
        title: "A".into(),
        workspace: 1,
        minimized: false,
        app_id: None,
    }];
    apply_window_closed_state(&mut windows, "missing");
    assert_eq!(windows.len(), 1);
}

#[test]
fn window_closed_removes_existing_window() {
    let mut windows = vec![WindowInfo {
        id: "id-1".into(),
        title: "A".into(),
        workspace: 1,
        minimized: false,
        app_id: None,
    }];
    apply_window_closed_state(&mut windows, "id-1");
    assert!(windows.is_empty());
}

#[test]
fn full_snapshot_preserves_workspace_on_window_entries() {
    let mut active = 1u8;
    let mut windows = Vec::new();
    let mut counts = [0u16; niwoe_config::rooms::MAX_ROOMS];
    apply_full_window_snapshot(
        &mut active,
        &mut windows,
        &mut counts,
        2,
        vec![
            WindowSnapshotEntry {
                workspace: 1,
                id: "id-1".into(),
                title: "A".into(),
                minimized: false,
                app_id: None,
            },
            WindowSnapshotEntry {
                workspace: 3,
                id: "id-2".into(),
                title: "B".into(),
                minimized: true,
                app_id: None,
            },
        ],
    );
    assert_eq!(windows[0].workspace, 1);
    assert_eq!(windows[1].workspace, 3);
    assert!(!windows[0].minimized);
    assert!(windows[1].minimized);
}

#[test]
fn stale_focused_window_id_is_cleared_when_no_window_matches() {
    let windows = vec![WindowInfo {
        id: "id-1".into(),
        title: "A".into(),
        workspace: 1,
        minimized: false,
        app_id: None,
    }];
    let mut focused = Some("missing".to_string());
    clear_stale_focused_window_id(&mut focused, &windows);
    assert_eq!(focused, None);
}

#[test]
fn resolve_shell_theme_from_config_applies_cursor_and_wallpaper_overrides() {
    let config = NiwoeConfig {
        general: GeneralConfig {
            theme: "light".to_string(),
            idle_timeout_secs: None,
        },
        cursor: Some(niwoe_config::CursorConfig {
            theme: "niwoe".to_string(),
            size: 30,
        }),
        wallpaper: Some(WallpaperConfig {
            path: "".to_string(),
            mode: WallpaperMode::Tile,
        }),
        ..Default::default()
    };

    let (name, theme, _available) =
        resolve_shell_theme_from_config(&config).expect("resolve theme");
    assert_eq!(name, "dark");
    assert_eq!(config.general.theme, "light");
    assert_eq!(theme.cursor.size, 30);
    assert_eq!(theme.cursor.theme, "niwoe");
    assert_eq!(
        theme.wallpaper.as_ref().map(|w| w.mode),
        Some(WallpaperMode::Tile)
    );
}

#[test]
fn resolve_shell_theme_from_config_preserves_unused_legacy_theme() {
    let config = NiwoeConfig {
        general: GeneralConfig {
            theme: "definitely-not-a-theme".to_string(),
            idle_timeout_secs: None,
        },
        ..Default::default()
    };
    let (name, _, _) = resolve_shell_theme_from_config(&config).expect("alpha theme");
    assert_eq!(name, "dark");
    assert_eq!(config.general.theme, "definitely-not-a-theme");
}

#[test]
fn panel_theme_signature_changes_when_theme_changes() {
    let config = NiwoeConfig::default();
    let (_name, mut theme, _available) =
        resolve_shell_theme_from_config(&config).expect("resolve theme");
    let sig_a = panel_theme_signature(&theme);
    theme.colors.accent = niwoe_config::Color::rgb(0, 0, 0);
    let sig_b = panel_theme_signature(&theme);
    assert_ne!(sig_a, sig_b);
}

#[test]
fn panel_theme_signature_changes_when_surface_alt_changes() {
    let config = NiwoeConfig::default();
    let (_name, mut theme, _available) =
        resolve_shell_theme_from_config(&config).expect("resolve theme");
    let sig_a = panel_theme_signature(&theme);
    theme.colors.surface_alt = niwoe_config::Color::rgb(0, 0, 0);
    let sig_b = panel_theme_signature(&theme);
    assert_ne!(sig_a, sig_b);
}

#[test]
fn panel_theme_signature_changes_when_border_changes() {
    let config = NiwoeConfig::default();
    let (_name, mut theme, _available) =
        resolve_shell_theme_from_config(&config).expect("resolve theme");
    let sig_a = panel_theme_signature(&theme);
    theme.colors.border = niwoe_config::Color::rgb(0, 0, 0);
    let sig_b = panel_theme_signature(&theme);
    assert_ne!(sig_a, sig_b);
}

#[test]
fn output_workspace_snapshot_with_two_outputs_is_stored() {
    let mut focused_output_id = None;
    let mut output_workspaces = Vec::new();
    let mut output_workspace_state_available = false;
    let mut workspace_indicator_dirty = false;

    apply_output_workspace_snapshot_state(
        &mut focused_output_id,
        &mut output_workspaces,
        &mut output_workspace_state_available,
        &mut workspace_indicator_dirty,
        Some(2),
        vec![
            OutputWorkspaceState {
                output_id: 1,
                output_name: Some("eDP-1".to_string()),
                active_workspace: 2,
                primary: true,
                focused: false,
                ..Default::default()
            },
            OutputWorkspaceState {
                output_id: 2,
                output_name: Some("HDMI-A-1".to_string()),
                active_workspace: 4,
                primary: false,
                focused: true,
                ..Default::default()
            },
        ],
    );

    assert_eq!(focused_output_id, Some(2));
    assert_eq!(output_workspaces.len(), 2);
    assert!(output_workspace_state_available);
    assert!(workspace_indicator_dirty);
}
