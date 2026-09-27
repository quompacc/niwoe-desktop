use niwoe_ui::Widget;

use super::*;
use crate::{audio::AudioSnapshot, icons::IconCache, network::NetworkState};

#[test]
fn panel_chip_style_returns_correct_size() {
    let chip = PanelChip::new("test", "Test".into(), None, 58, false);
    let style = chip.style();
    assert_eq!(style.size.width, ui_length(58.0_f32));
    assert_eq!(style.size.height, ui_length(CHIP_H as f32));
}

#[test]
fn tray_chip_widths_match() {
    assert_eq!(SNI_W, TRAY_W);
    assert_eq!(SCREENSHOT_W, PanelTokens::DEFAULT.control_width as i32);
    const { assert!(TRAY_W < SCREENSHOT_W) };
}

#[test]
fn panel_window_chip_focus_window_id_returns_id() {
    let chip = PanelWindowChip {
        window_id: "win-1".into(),
        title: "Window".into(),
        focused: false,
        minimized: false,
        width: 100,
    };
    assert_eq!(chip.focus_window_id(), Some("win-1"));
}

#[test]
fn status_notifier_label_prefers_title_then_icon_then_service() {
    assert_eq!(
        status_notifier_label(&StatusNotifierItem {
            service: "org.example.Service".to_string(),
            title: Some("Dropbox".to_string()),
            icon_name: Some("cloud-sync".to_string()),
            menu_path: Some("/Menu".to_string()),
        }),
        "DR"
    );
    assert_eq!(
        status_notifier_label(&StatusNotifierItem {
            service: "org.example.Service".to_string(),
            title: None,
            icon_name: Some("cloud-sync".to_string()),
            menu_path: None,
        }),
        "CL"
    );
    assert_eq!(
        status_notifier_label(&StatusNotifierItem {
            service: "org.example.Service".to_string(),
            title: None,
            icon_name: None,
            menu_path: None,
        }),
        "SE"
    );
}

#[test]
fn status_icons_have_legible_native_size() {
    const { assert!(STATUS_ICON_SIZE >= 20) };
}

#[test]
fn status_icon_tint_preserves_alpha_and_uses_premultiplied_theme_color() {
    let mut pixmap = Pixmap::new(1, 1).expect("pixmap");
    pixmap.data_mut().copy_from_slice(&[0, 0, 0, 128]);
    tint_pixmap_premul(&mut pixmap, Color::rgb(100, 80, 60));
    assert_eq!(pixmap.data(), &[50, 40, 30, 128]);
}

#[test]
fn build_panel_widget_tree_root_has_three_children() {
    let icon_cache = IconCache::new();
    let network = NetworkState::Disconnected;
    let audio = AudioSnapshot::unavailable();
    let tree = build_panel_widget_tree(
        1920,
        &[],
        &[],
        &network,
        &audio,
        &[],
        false,
        false,
        &crate::battery::BatterySnapshot::default(),
        None,
        1,
        9,
        &[],
        &[false; niwoe_config::rooms::MAX_ROOMS],
        "12:34",
        &icon_cache,
        None,
        &crate::room_editor::panel::MODULES,
        &Theme::TOKYO_NIGHT_METRO,
    );
    // Floating-Island-Struktur: Root umschliesst die bar, die bar haelt die 3 Cluster.
    assert_eq!(tree.children().len(), 1, "root wraps the island bar");
    assert_eq!(
        tree.children()[0].children().len(),
        3,
        "bar holds left/center/right clusters"
    );
}

#[test]
fn draw_panel_ui_modifies_canvas_and_fills_clicks() {
    let width = 1024u32;
    let height = PANEL_HEIGHT;
    let mut canvas = vec![0u8; (width * height * 4) as usize];
    let icon_cache = IconCache::new();
    let network = NetworkState::Disconnected;
    let audio = AudioSnapshot::unavailable();
    let mut clicks = Vec::new();
    let state_fn = |_: &[usize]| WidgetState::Idle;

    draw_panel_ui(
        &mut canvas,
        width,
        height,
        &[PinnedApp {
            label: "Terminal".into(),
            program: "foot".into(),
            args: vec![],
            terminal: false,
            icon_name: None,
        }],
        &[],
        &network,
        &audio,
        &[],
        false,
        false,
        &crate::battery::BatterySnapshot::default(),
        None,
        1,
        9,
        &[],
        &[false; niwoe_config::rooms::MAX_ROOMS],
        "12:34",
        &icon_cache,
        None,
        &crate::room_editor::panel::MODULES,
        &niwoe_config::ThemeConfig::default(),
        &state_fn,
        &mut clicks,
    );

    assert!(canvas.iter().any(|byte| *byte != 0));
    // Like the deck, the shell must leave the compositor glass visible.
    assert_eq!(canvas[3], 0);
    if let Ok(path) = std::env::var("NIWOE_PANEL_PREVIEW") {
        let mut rgba = canvas.clone();
        for pixel in rgba.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        tiny_skia::Pixmap::from_vec(rgba, tiny_skia::IntSize::from_wh(width, height).unwrap())
            .unwrap()
            .save_png(path)
            .unwrap();
    }
    assert!(!clicks.is_empty());
    let rooms: Vec<_> = clicks
        .iter()
        .filter(|z| {
            z.id.as_deref()
                .is_some_and(|id| id.starts_with("panel-room-"))
        })
        .collect();
    assert!(!rooms.is_empty() && rooms.len() <= PanelTokens::DEFAULT.visible_rooms);
    assert!(rooms
        .iter()
        .any(|z| matches!(z.action, ClickAction::SwitchWorkspace(1))));
    assert!(clicks
        .iter()
        .any(|z| z.id.as_deref() == Some("panel-search")
            && matches!(z.action, ClickAction::OpenHubSearch)));
    assert!(clicks.iter().any(|z| {
        z.id.as_deref() == Some("panel-screenshot")
            && matches!(z.action, ClickAction::TakeScreenshot)
    }));
    assert!(
        clicks
            .iter()
            .all(|zone| !matches!(zone.action, ClickAction::LaunchPinnedApp(_))),
        "app shortcuts must not return to the panel"
    );
    let zone = |id| clicks.iter().find(|z| z.id.as_deref() == Some(id)).unwrap();
    let launcher = zone("panel-launcher");
    let status = zone("panel-status");
    let clock = zone("panel-clock");
    let workspace = zone("panel-workspace");
    assert!(launcher.rect.x + launcher.rect.w <= workspace.rect.x);
    assert!(workspace.rect.x + workspace.rect.w <= clock.rect.x);
    assert!(clock.rect.x + clock.rect.w <= status.rect.x);
    assert!((clock.rect.x * 2 + clock.rect.w - width as i32).abs() <= 1);
    assert!(clicks.iter().all(|z| z.rect.y >= 0
        && z.rect.y + z.rect.h <= height as i32
        && z.rect.x >= 0
        && z.rect.x + z.rect.w <= width as i32));
    assert_eq!(
        clicks
            .iter()
            .filter(|zone| zone.id.as_deref() == Some("panel-status"))
            .count(),
        1,
        "the system tray is one Quick Settings click target"
    );
    assert!(clicks.iter().all(|zone| !matches!(
        zone.id.as_deref(),
        Some("panel-network" | "panel-sound" | "panel-battery")
    )));
}

#[test]
fn panel_room_rail_uses_fixed_pages_and_keeps_active_room_visible() {
    let icons = IconCache::new();
    let audio = AudioSnapshot::unavailable();
    let room_ids = |active_workspace| {
        let tree = build_panel_widget_tree(
            1366,
            &[],
            &[],
            &NetworkState::Disconnected,
            &audio,
            &[],
            false,
            false,
            &crate::battery::BatterySnapshot::default(),
            None,
            active_workspace,
            9,
            &[],
            &[false; niwoe_config::rooms::MAX_ROOMS],
            "12:34",
            &icons,
            None,
            &crate::room_editor::panel::MODULES,
            &Theme::TOKYO_NIGHT_METRO,
        );
        let layout = compute_layout(
            &*tree,
            PixelSize {
                width: 1366,
                height: PANEL_HEIGHT,
            },
        )
        .unwrap();
        let mut zones = Vec::new();
        collect_click_zones(&*tree, &layout.root, 0, 0, &mut zones);
        zones
            .into_iter()
            .filter_map(|zone| zone.id.filter(|id| id.starts_with("panel-room-")))
            .collect::<Vec<_>>()
    };

    assert_eq!(room_ids(1), room_ids(4));
    assert!(room_ids(9).contains(&"panel-room-9".to_string()));
    assert_eq!(
        room_ids(1),
        vec![
            "panel-room-1",
            "panel-room-2",
            "panel-room-3",
            "panel-room-4"
        ]
    );
}

#[test]
fn panel_omits_room_overflow_when_every_room_fits() {
    let icons = IconCache::new();
    let tree = build_panel_widget_tree(
        1920,
        &[],
        &[],
        &NetworkState::Disconnected,
        &AudioSnapshot::unavailable(),
        &[],
        false,
        false,
        &crate::battery::BatterySnapshot::default(),
        None,
        4,
        4,
        &[],
        &[false; niwoe_config::rooms::MAX_ROOMS],
        "12:34",
        &icons,
        None,
        &crate::room_editor::panel::MODULES,
        &Theme::TOKYO_NIGHT_METRO,
    );
    let layout = compute_layout(
        &*tree,
        PixelSize {
            width: 1920,
            height: PANEL_HEIGHT,
        },
    )
    .unwrap();
    let mut zones = Vec::new();
    collect_click_zones(&*tree, &layout.root, 0, 0, &mut zones);

    assert!(zones
        .iter()
        .all(|zone| zone.id.as_deref() != Some("panel-workspace")));
    assert_eq!(
        zones
            .iter()
            .filter(|zone| zone
                .id
                .as_deref()
                .is_some_and(|id| id.starts_with("panel-room-")))
            .count(),
        4
    );
}

#[test]
fn module_visibility_and_order_keep_required_access_and_centered_clock() {
    use niwoe_ipc::PanelModule;
    let icons = IconCache::new();
    for width in [1366, 1920] {
        for modules in [
            vec![],
            vec![PanelModule::Status, PanelModule::Search],
            vec![PanelModule::Search, PanelModule::Status],
        ] {
            let tree = build_panel_widget_tree(
                width,
                &[],
                &[],
                &NetworkState::Disconnected,
                &AudioSnapshot::unavailable(),
                &[],
                false,
                false,
                &crate::battery::BatterySnapshot::default(),
                None,
                1,
                9,
                &[],
                &[false; niwoe_config::rooms::MAX_ROOMS],
                "12:34",
                &icons,
                None,
                &modules,
                &Theme::TOKYO_NIGHT_METRO,
            );
            let layout = compute_layout(
                &*tree,
                PixelSize {
                    width,
                    height: PANEL_HEIGHT,
                },
            )
            .unwrap();
            let mut zones = Vec::new();
            collect_click_zones(&*tree, &layout.root, 0, 0, &mut zones);
            for id in ["panel-launcher", "panel-clock", "panel-workspace"] {
                assert!(zones.iter().any(|z| z.id.as_deref() == Some(id)), "{id}");
            }
            let clock = zones
                .iter()
                .find(|z| z.id.as_deref() == Some("panel-clock"))
                .unwrap();
            assert!((clock.rect.x * 2 + clock.rect.w - width as i32).abs() <= 1);
            assert!(!zones
                .iter()
                .any(|z| z.id.as_deref() == Some("panel-screenshot")));
            if !modules.is_empty() {
                let search = zones
                    .iter()
                    .find(|z| z.id.as_deref() == Some("panel-search"))
                    .unwrap();
                let status = zones
                    .iter()
                    .find(|z| z.id.as_deref() == Some("panel-status"))
                    .unwrap();
                assert_eq!(
                    search.rect.x < status.rect.x,
                    modules[0] == PanelModule::Search
                );
            }
        }
    }
}

#[test]
fn panel_layout_keeps_clock_centered_and_controls_separate_across_viewports() {
    let icons = IconCache::new();
    let audio = AudioSnapshot::unavailable();
    for width in [1024, 1366, 1920] {
        for palette in [niwoe_tokens::Palette::DARK, niwoe_tokens::Palette::LIGHT] {
            let theme = Theme {
                palette,
                ..Theme::TOKYO_NIGHT_METRO
            };
            for active in [1, 5, 9] {
                let tree = build_panel_widget_tree(
                    width,
                    &[],
                    &[],
                    &NetworkState::Disconnected,
                    &audio,
                    &[],
                    false,
                    false,
                    &crate::battery::BatterySnapshot::default(),
                    None,
                    active,
                    9,
                    &[],
                    &[false; niwoe_config::rooms::MAX_ROOMS],
                    "19:18  Mi, 23. Sep",
                    &icons,
                    None,
                    &crate::room_editor::panel::MODULES,
                    &theme,
                );
                let layout = compute_layout(
                    &*tree,
                    PixelSize {
                        width,
                        height: PANEL_HEIGHT,
                    },
                )
                .unwrap();
                let mut zones = Vec::new();
                collect_click_zones(&*tree, &layout.root, 0, 0, &mut zones);
                let clock = zones
                    .iter()
                    .find(|z| z.id.as_deref() == Some("panel-clock"))
                    .unwrap();
                assert!((clock.rect.x * 2 + clock.rect.w - width as i32).abs() <= 1);
                let active_directly_visible = zones
                    .iter()
                    .any(|z| matches!(z.action, ClickAction::SwitchWorkspace(w) if w == active));
                let room_overflow_visible = zones
                    .iter()
                    .any(|z| matches!(z.action, ClickAction::ToggleWorkspacePopup));
                assert!(active_directly_visible || room_overflow_visible);
                for (index, zone) in zones.iter().enumerate() {
                    assert!(zone.rect.x >= 0 && zone.rect.x + zone.rect.w <= width as i32);
                    for other in zones.iter().skip(index + 1) {
                        assert!(
                            zone.rect.x + zone.rect.w <= other.rect.x
                                || other.rect.x + other.rect.w <= zone.rect.x,
                            "overlapping panel targets at {width}: {:?} / {:?}",
                            zone.id,
                            other.id
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn action_for_id_as_click_screenshot() {
    assert!(matches!(
        action_for_id_as_click("panel-screenshot"),
        Some(ClickAction::TakeScreenshot)
    ));
    assert!(matches!(
        action_for_id_as_click("panel-status"),
        Some(ClickAction::ToggleNetworkPopup)
    ));
    assert!(action_for_id_as_click("panel-sound").is_none());
}
