use super::*;

fn tree(width: u32, height: u32, category: SettingsCategory, page: usize) -> Box<dyn Widget> {
    let devices: Vec<_> = (0..8)
        .map(|i| AudioDevice {
            id: 5000 + i,
            name: format!("Gerät {i} {}", "Langer Gerätename ".repeat(20)),
            volume_percent: Some(84),
            muted: false,
            is_default: i == 0,
        })
        .collect();
    let audio = AudioSnapshot {
        service: AudioServiceState::Running,
        default_output: Some(devices[0].clone()),
        default_input: Some(devices[0].clone()),
        outputs: devices.clone(),
        inputs: devices,
    };
    let profiles: Vec<_> = (0..8)
        .map(|i| ConnectionProfile {
            name: format!("Profil {i} {}", "Langer Profilname ".repeat(20)),
            type_label: "Ethernet".into(),
            active: i == 0,
        })
        .collect();
    let wifi: Vec<_> = (0..10)
        .map(|i| WifiNetwork {
            ssid: format!("WLAN {i} {}", "Langer WLANname ".repeat(20)),
            signal: 99,
            secured: true,
            in_use: i == 0,
        })
        .collect();
    let bluetooth = BluetoothSnapshot {
        adapter_present: true,
        powered: true,
        scanning: false,
        device_status_available: true,
        devices: (0..8)
            .map(|i| crate::bluetooth::BluetoothDevice {
                address: format!("AA:BB:CC:DD:EE:{i:02}"),
                name: "Langer Bluetoothname ".repeat(20),
                paired: true,
                connected: i == 0,
            })
            .collect(),
    };
    build_settings_widget_tree(
        width,
        height,
        category,
        "",
        true,
        &[],
        "",
        &[],
        &[],
        0,
        None,
        WallpaperMode::Fill,
        24,
        &crate::cursor::preview::CursorPreviews::default(),
        &[],
        "",
        None,
        &[],
        &[],
        None,
        DisplayPages::default(),
        page,
        &PrinterSnapshot {
            service: PrinterServiceState::Unavailable,
            default_printer: None,
            list_available: false,
            default_available: false,
            printers: Vec::new(),
            job_count: None,
        },
        &audio,
        &SystemInfo::default(),
        &crate::users::UserState::default(),
        &NetworkState::Connected {
            kind: crate::network::ConnectionKind::Wifi { signal: Some(99) },
            connection_name: "Langer aktiver Verbindungsname ".repeat(20),
        },
        &profiles,
        crate::network::ListStatus {
            profiles_available: true,
            wifi_available: true,
        },
        &bluetooth,
        &wifi,
        false,
        &[],
        &IconCache::new(),
        None,
        None,
        &std::collections::HashMap::new(),
        None,
        0,
        &crate::default_apps::refresh::UiState::default(),
        &theme_from_config(&ThemeConfig::default()),
    )
}

#[test]
fn full_provider_lists_keep_every_original_action_visible_and_reachable_on_every_page() {
    for (width, height) in [(1920, 1032), (1366, 720)] {
        for (category, groups) in [
            (
                SettingsCategory::Network,
                vec![NETWORK_PROFILE_IDS, WIFI_NETWORK_IDS],
            ),
            (
                SettingsCategory::Sound,
                vec![AUDIO_OUTPUT_IDS, AUDIO_INPUT_IDS],
            ),
            (SettingsCategory::Bluetooth, vec![BT_DEVICE_IDS]),
        ] {
            let viewport = niwoe_ui::PixelSize { width, height };
            let body = height
                - SETTINGS_CHROME.header_height as u32
                - niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height as u32;
            let slots = provider_page_size(category, body);
            let count = groups.iter().map(|ids| ids.len()).max().unwrap();
            let pages = count.div_ceil(slots);
            let mut reached = std::collections::BTreeSet::new();
            for page in 0..pages {
                let root = tree(width, height, category, page);
                let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
                let targets =
                    crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
                for ids in &groups {
                    for index in provider_page_range(page, slots, count).filter(|i| *i < ids.len())
                    {
                        // Existing connected/default entries deliberately have no action.
                        if index == 0 {
                            assert!(!targets.iter().any(|(id, _)| *id == ids[0]));
                            continue;
                        }
                        let id = ids[index];
                        let (_, area) = targets
                            .iter()
                            .find(|(target, _)| *target == id)
                            .unwrap_or_else(|| {
                                panic!("{id} clipped at {width}x{height} page {page}")
                            });
                        assert!(area.x >= 0 && area.y >= 0);
                        assert!(
                            area.x + area.width <= width as i32
                                && area.y + area.height <= height as i32
                        );
                        let path = niwoe_ui::hit_test(
                            &layout,
                            niwoe_ui::PointerPosition {
                                x: area.x + area.width / 2,
                                y: area.y + area.height / 2,
                            },
                        )
                        .unwrap();
                        assert_eq!(
                            crate::widget_traversal::find_widget_at_path(root.as_ref(), &path)
                                .and_then(|w| w.id()),
                            Some(id)
                        );
                        assert!(crate::widget_action::action_for_id(id).is_some());
                        reached.insert(id);
                    }
                }
                assert_eq!(
                    targets
                        .iter()
                        .any(|(id, _)| *id == "provider-page-previous"),
                    page > 0
                );
                assert_eq!(
                    targets.iter().any(|(id, _)| *id == "provider-page-next"),
                    page + 1 < pages
                );
            }
            assert_eq!(
                reached.len(),
                groups.iter().map(|ids| ids.len() - 1).sum::<usize>()
            );
        }
    }
}

#[test]
fn paging_clamps_after_provider_refresh_and_counts_disclose_existing_caps() {
    assert_eq!(printer_job_count_label(Some(0)), "0 Druckaufträge");
    assert_eq!(printer_job_count_label(Some(1)), "1 Druckauftrag");
    assert_eq!(
        printer_job_count_label(None),
        "Druckaufträge nicht verfügbar"
    );
    assert_eq!(provider_page_range(999, 4, 8), 4..8);
    assert_eq!(provider_page_range(999, 4, 3), 0..3);
    assert_eq!(provider_page_range(999, 4, 0), 0..0);
    assert_eq!(provider_count_label(8, 12, "Geräte"), "8 von 12 Geräte");
    assert_eq!(provider_count_label(8, 8, "Geräte"), "8 Geräte");
    assert!(matches!(
        crate::widget_action::action_for_id("provider-page-next"),
        Some(crate::widget_action::WidgetAction::PageProviders { forward: true })
    ));
}
