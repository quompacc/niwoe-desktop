use super::*;

#[test]
fn all_power_and_idle_actions_fit_and_are_reachable_on_the_minimum_canvas() {
    for (width, height) in [(1920, 1032), (1366, 720)] {
        let viewport = niwoe_ui::PixelSize { width, height };
        let root = test_tree(width, height, SettingsCategory::Power, &[], 0);
        let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
        let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
        for id in IDLE_TIMEOUT_OPTIONS.iter().map(|(_, id, _)| *id).chain([
            "power-sleep",
            "power-lock",
            "power-logout",
            "power-restart",
            "power-off",
        ]) {
            let (_, area) = targets
                .iter()
                .find(|(target, _)| *target == id)
                .unwrap_or_else(|| panic!("{id} is clipped at {width}x{height}"));
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
                    .and_then(|widget| widget.id()),
                Some(id)
            );
        }
    }
}

#[test]
fn display_modes_keep_original_indices_and_all_targets_fit_every_page() {
    let outputs: Vec<_> = (0..16)
        .map(|output| OutputWorkspaceState {
            output_id: output + 1,
            output_name: Some(format!("connector-{output}")),
            primary: output == 0,
            modes: (0..64)
                .map(|mode| OutputModeState {
                    width: if mode == 3 { 0 } else { 1920 + mode },
                    height: 1080,
                    refresh_millihz: Some(60000),
                    current: mode == 0,
                    preferred: mode == 1,
                })
                .collect(),
            ..Default::default()
        })
        .collect();
    for (width, height) in [(1920, 1032), (1366, 720)] {
        let viewport = niwoe_ui::PixelSize { width, height };
        let content_height = height
            - SETTINGS_CHROME.header_height as u32
            - niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height as u32;
        let slots = display_mode_page_size(content_height);
        for output in 0..outputs.len() {
            let indices = display_mode_indices(&outputs[output]);
            assert!(!indices.contains(&3));
            let pages = indices.len().div_ceil(slots);
            let mut reached = std::collections::BTreeSet::new();
            for page in 0..pages {
                let root = test_tree_with_display(
                    width,
                    height,
                    SettingsCategory::Display,
                    &[],
                    0,
                    &outputs,
                    Some(output),
                    DisplayPages {
                        output,
                        modes: page,
                    },
                    None,
                );
                let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
                let targets =
                    crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
                for &mode in indices.iter().skip(page * slots).take(slots) {
                    let id = display_mode_option_id(output, mode).unwrap();
                    let (_, area) = targets
                        .iter()
                        .find(|(target, _)| *target == id)
                        .unwrap_or_else(|| {
                            panic!("{id} is clipped at {width}x{height}, page {page}")
                        });
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
                            .and_then(|widget| widget.id()),
                        Some(id)
                    );
                    assert_eq!(
                        crate::widget_action::action_for_id(id),
                        Some(crate::widget_action::WidgetAction::SetOutputMode {
                            output_index: output,
                            mode_index: mode
                        })
                    );
                    reached.insert(mode);
                }
                for (id, enabled) in [
                    ("display-modes-previous", page > 0),
                    ("display-modes-next", page + 1 < pages),
                    ("display-outputs-previous", output > 0),
                    ("display-outputs-next", output + 1 < outputs.len()),
                ] {
                    assert_eq!(
                        targets.iter().any(|(target, _)| *target == id),
                        enabled,
                        "{id} output={output}, page={page}, {width}x{height}"
                    );
                }
            }
            assert_eq!(reached.into_iter().collect::<Vec<_>>(), indices);
        }
    }
}

#[test]
fn empty_display_snapshot_and_oversized_page_positions_are_safe() {
    for outputs in [Vec::new(), vec![OutputWorkspaceState::default()]] {
        let root = test_tree_with_display(
            1366,
            720,
            SettingsCategory::Display,
            &[],
            0,
            &outputs,
            Some(usize::MAX),
            DisplayPages {
                output: usize::MAX,
                modes: usize::MAX,
            },
            None,
        );
        let viewport = niwoe_ui::PixelSize {
            width: 1366,
            height: 720,
        };
        let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
        let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
        assert!(!targets
            .iter()
            .any(|(id, _)| id.starts_with("display-mode-")));
    }
}

#[test]
fn app_categories_and_twelve_cursor_themes_are_visible_and_pointer_reachable() {
    for category in [SettingsCategory::DefaultApps, SettingsCategory::Cursor] {
        for (width, height) in [(1920, 1032), (1366, 720)] {
            let viewport = niwoe_ui::PixelSize { width, height };
            let root = test_tree(width, height, category, &[], 0);
            let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
            let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
            let ids: Vec<_> = if category == SettingsCategory::Cursor {
                crate::cursor::CURSOR_THEME_WIDGET_IDS.to_vec()
            } else {
                (0..crate::default_apps::DefaultAppCategory::ALL.len())
                    .map(|i| default_apps_pick_id(i).unwrap())
                    .collect()
            };
            for id in ids {
                let (_, area) = targets
                    .iter()
                    .find(|(target, _)| *target == id)
                    .unwrap_or_else(|| panic!("{id} is clipped at {width}x{height}"));
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
                        .and_then(|widget| widget.id()),
                    Some(id),
                );
            }
        }
    }
}

fn test_tree(
    width: u32,
    height: u32,
    category: SettingsCategory,
    wallpapers: &[WallpaperEntry],
    page: usize,
) -> Box<dyn Widget> {
    test_tree_with_display(
        width,
        height,
        category,
        wallpapers,
        page,
        &[],
        None,
        DisplayPages::default(),
        None,
    )
}

#[allow(clippy::too_many_arguments)] // Fixture for the existing flat settings-state boundary.
fn test_tree_with_display(
    width: u32,
    height: u32,
    category: SettingsCategory,
    wallpapers: &[WallpaperEntry],
    page: usize,
    outputs: &[OutputWorkspaceState],
    dropdown: Option<usize>,
    display_pages: DisplayPages,
    apps: Option<DefaultAppsFixture<'_>>,
) -> Box<dyn Widget> {
    let printer = PrinterSnapshot {
        service: PrinterServiceState::Unavailable,
        default_printer: None,
        list_available: false,
        default_available: false,
        printers: Vec::new(),
        job_count: None,
    };
    let audio = AudioSnapshot::unavailable();
    let system = SystemInfo::default();
    let network = NetworkState::Disconnected;
    let bluetooth = BluetoothSnapshot::default();
    let icons = IconCache::new();
    let index = crate::default_apps::MimeAppIndex::default();
    let current = std::collections::HashMap::new();
    let apps = apps.unwrap_or(DefaultAppsFixture {
        index: &index,
        current: &current,
        open: None,
        page: 0,
        busy: false,
    });
    let theme = theme_from_config(&ThemeConfig::default());
    let cursor_themes = (0..12).map(|i| format!("Theme {i}")).collect::<Vec<_>>();
    build_settings_widget_tree(
        width,
        height,
        category,
        "",
        true,
        &[],
        "",
        wallpapers,
        &[],
        page,
        None,
        WallpaperMode::Fill,
        24,
        &crate::cursor::preview::CursorPreviews::default(),
        &cursor_themes,
        "",
        None,
        &[],
        outputs,
        dropdown,
        display_pages,
        0,
        &printer,
        &audio,
        &system,
        &crate::users::UserState::default(),
        &network,
        &[],
        crate::network::ListStatus::default(),
        &bluetooth,
        &[],
        false,
        &[],
        &icons,
        None,
        Some(apps.index),
        apps.current,
        apps.open,
        apps.page,
        &crate::default_apps::refresh::UiState {
            busy: apps.busy,
            ..Default::default()
        },
        &theme,
    )
}

struct DefaultAppsFixture<'a> {
    index: &'a crate::default_apps::MimeAppIndex,
    current: &'a std::collections::HashMap<crate::default_apps::DefaultAppCategory, String>,
    open: Option<crate::default_apps::DefaultAppCategory>,
    page: usize,
    busy: bool,
}

#[test]
fn twenty_four_app_choices_keep_original_actions_and_fit_every_page() {
    let dir = std::env::temp_dir().join(format!("niwoe-mime-layout-test-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    for i in 0..30 {
        std::fs::write(dir.join(format!("choice-{i:02}.desktop")),format!("[Desktop Entry]\nType=Application\nName=Choice {i:02} with a long descriptive application name\nExec=fixture %U\nMimeType=text/plain;\n")).unwrap();
    }
    let index = crate::default_apps::MimeAppIndex::load_from_dirs(vec![dir.clone()]);
    let current = std::collections::HashMap::from([(
        crate::default_apps::DefaultAppCategory::TextEditor,
        "choice-23.desktop".into(),
    )]);
    for (width, height) in [(1920, 1032), (1366, 720)] {
        let viewport = niwoe_ui::PixelSize { width, height };
        let content_height = height
            - SETTINGS_CHROME.header_height as u32
            - category_navigation_height(SettingsCategory::DefaultApps) as u32;
        let slots = default_apps_page_size(content_height);
        let pages = 24_usize.div_ceil(slots);
        let mut reached = std::collections::BTreeSet::new();
        for page in 0..pages {
            let root = test_tree_with_display(
                width,
                height,
                SettingsCategory::DefaultApps,
                &[],
                0,
                &[],
                None,
                DisplayPages::default(),
                Some(DefaultAppsFixture {
                    index: &index,
                    current: &current,
                    open: Some(crate::default_apps::DefaultAppCategory::TextEditor),
                    page,
                    busy: false,
                }),
            );
            let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
            let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
            for app in page * slots..((page + 1) * slots).min(24) {
                let id = default_apps_set_id(3, app).unwrap();
                let (_, area) = targets
                    .iter()
                    .find(|(target, _)| *target == id)
                    .unwrap_or_else(|| panic!("{id} clipped at {width}x{height}"));
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
                        .and_then(|widget| widget.id()),
                    Some(id)
                );
                assert_eq!(
                    crate::widget_action::action_for_id(id),
                    Some(crate::widget_action::WidgetAction::DefaultAppsPick {
                        cat_idx: 3,
                        app_idx: app
                    })
                );
                reached.insert(app);
            }
            assert_eq!(
                targets.iter().any(|(id, _)| *id == "default-apps-previous"),
                page > 0
            );
            assert_eq!(
                targets.iter().any(|(id, _)| *id == "default-apps-next"),
                page + 1 < pages
            );
            assert!(targets.iter().any(|(id, _)| *id == "default-apps-back"));
        }
        assert_eq!(reached, (0..24).collect());
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn default_app_writes_are_not_focusable_while_busy_but_back_remains_available() {
    let dir = std::env::temp_dir().join(format!("niwoe-mime-busy-test-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(
        dir.join("choice.desktop"),
        "[Desktop Entry]\nType=Application\nName=Choice\nExec=true\nMimeType=text/plain;\n",
    )
    .unwrap();
    let index = crate::default_apps::MimeAppIndex::load_from_dirs(vec![dir.clone()]);
    let current = std::collections::HashMap::new();
    for (width, height) in [(1920, 1032), (1366, 720)] {
        for open in [
            None,
            Some(crate::default_apps::DefaultAppCategory::TextEditor),
        ] {
            let root = test_tree_with_display(
                width,
                height,
                SettingsCategory::DefaultApps,
                &[],
                0,
                &[],
                None,
                DisplayPages::default(),
                Some(DefaultAppsFixture {
                    index: &index,
                    current: &current,
                    open,
                    page: 0,
                    busy: true,
                }),
            );
            let viewport = niwoe_ui::PixelSize { width, height };
            let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
            let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
            assert!(!targets
                .iter()
                .any(|(id, _)| *id == "default-apps-auto" || id.starts_with("default-apps-set-")));
            if open.is_some() {
                assert!(targets.iter().any(|(id, _)| *id == "default-apps-back"));
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn all_forty_wallpapers_have_pointer_and_keyboard_targets_across_pages() {
    let wallpapers: Vec<_> = (0..40)
        .map(|index| WallpaperEntry {
            display_name: format!("Bild {index:02} mit langem Katalognamen"),
            apply_path: format!("/test/{index}.png"),
            thumbnail_path: format!("/test/{index}.png"),
        })
        .collect();
    for (width, height) in [(1920, 1032), (1366, 720)] {
        let viewport = niwoe_ui::PixelSize { width, height };
        let body_height = height
            - SETTINGS_CHROME.header_height as u32
            - niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height as u32;
        let slots = wallpaper_page_size(body_height);
        let mut reached = std::collections::BTreeSet::new();
        for page in 0..wallpapers.len().div_ceil(slots) {
            let root = test_tree(
                width,
                height,
                SettingsCategory::Wallpaper,
                &wallpapers,
                page,
            );
            let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
            let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
            for (index, &id) in WALLPAPER_WIDGET_IDS
                .iter()
                .enumerate()
                .take(((page + 1) * slots).min(wallpapers.len()))
                .skip(page * slots)
            {
                let (_, area) = targets
                    .iter()
                    .find(|(target, _)| *target == id)
                    .unwrap_or_else(|| panic!("{id} is clipped at {width}x{height}, page {page}"));
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
                        .and_then(|widget| widget.id()),
                    Some(id)
                );
                reached.insert(index);
            }
            assert_eq!(
                targets
                    .iter()
                    .any(|(id, _)| *id == "wallpaper-page-previous"),
                page > 0
            );
            assert_eq!(
                targets.iter().any(|(id, _)| *id == "wallpaper-page-next"),
                (page + 1) * slots < wallpapers.len()
            );
        }
        assert_eq!(reached.len(), wallpapers.len());
    }
}
