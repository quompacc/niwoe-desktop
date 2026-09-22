use super::*;

fn app(name: &str, program: &str, terminal: bool) -> DesktopApp {
    DesktopApp::new(name.into(), vec![program.into()], terminal)
}

#[test]
fn search_matches_words_across_name_and_program_and_keeps_terminal_apps() {
    let apps = vec![
        app("Editor", "code", false),
        app("Terminal Editor", "nvim", true),
        app("Browser", "firefox", false),
    ];
    let hidden = HashSet::new();
    let found = collect_palette_apps(&apps, "editor nvim", &hidden, LauncherCategory::All, &[]);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].program, "nvim");
    let mut hidden = HashSet::new();
    hidden.insert("nvim".into());
    assert!(collect_palette_apps(&apps, "nvim", &hidden, LauncherCategory::All, &[]).is_empty());
}

#[test]
fn search_order_is_stable_and_exact_names_rank_first() {
    let apps = vec![
        app("Terminal Editor", "nvim", true),
        app("Editor", "code", false),
        app("A Editor", "aedit", false),
    ];
    let hidden = HashSet::new();
    let names = |apps: &[DesktopApp]| {
        collect_palette_apps(apps, "editor", &hidden, LauncherCategory::All, &[])
            .into_iter()
            .map(|a| a.name.clone())
            .collect::<Vec<_>>()
    };
    let mut reversed = apps.clone();
    reversed.reverse();
    assert_eq!(names(&apps), names(&reversed));
    assert_eq!(names(&apps)[0], "Editor");
}

#[test]
fn result_hit_testing_excludes_search_footer_and_row_gaps() {
    assert_eq!(hit_app_row(24, L.header_height, 0, 640, 480), Some(0));
    assert_eq!(hit_app_row(24, L.header_height + ROW, 0, 640, 480), Some(1));
    assert_eq!(
        hit_app_row(24, L.header_height + L.app_card_height, 0, 640, 480),
        None
    );
    assert_eq!(hit_app_row(24, L.header_height - 1, 0, 640, 480), None);
    assert_eq!(hit_app_row(24, 450, 0, 640, 480), None);
    assert_eq!(hit_app_row(630, 100, 0, 640, 480), None);
}

#[test]
fn selection_remains_visible_on_small_outputs() {
    assert_eq!(next_grid_selection(None, 4, GridDirection::Down), Some(1));
    assert_eq!(next_grid_selection(Some(0), 4, GridDirection::Up), Some(0));
    assert_eq!(
        next_grid_selection(Some(3), 4, GridDirection::Down),
        Some(3)
    );
    assert_eq!(next_grid_selection(None, 0, GridDirection::Down), None);
    for height in [280, 384, 480] {
        let scroll = scroll_grid_selection_into_view(0, 19, 20, height);
        let y = L.header_height + 19 * ROW - scroll;
        assert!(y >= L.header_height);
        assert!(y + L.app_card_height <= L.header_height + view_height(height));
    }
}

#[test]
fn native_launcher_renders_real_results_and_empty_state() {
    let config = niwoe_config::ThemeConfig::default();
    let icons = IconCache::new();
    let apps = vec![
        app("Firefox", "firefox", false),
        app("Dateien", "dolphin", false),
        app("Terminal", "konsole", false),
    ];
    let mut canvas = vec![0; (L.width * L.height * 4) as usize];
    for query in ["", "kein Treffer"] {
        draw_command_palette(
            &mut canvas,
            L.width as u32,
            L.height as u32,
            &[],
            &apps,
            LauncherCategory::All,
            query,
            0,
            None,
            None,
            &icons,
            &HashSet::new(),
            None,
            None,
            false,
            None,
            &config,
        );
        assert!(canvas.as_chunks::<4>().0.iter().any(|p| p[3] != 0));
        if query.is_empty() {
            if let Ok(path) = std::env::var("NIWOE_LAUNCHER_PREVIEW") {
                let mut rgba = canvas.clone();
                for p in rgba.as_chunks_mut::<4>().0 {
                    p.swap(0, 2);
                }
                Pixmap::from_vec(
                    rgba,
                    tiny_skia::IntSize::from_wh(L.width as u32, L.height as u32).unwrap(),
                )
                .unwrap()
                .save_png(path)
                .unwrap();
            }
        }
    }
}
