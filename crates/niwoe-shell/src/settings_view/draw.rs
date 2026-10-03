#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_settings_launcher(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    selected: SettingsCategory,
    search: &str,
    back_to_rooms: bool,
    focused_widget: Option<&'static str>,
    available_themes: &[String],
    current_theme: &str,
    available_wallpapers: &[WallpaperEntry],
    wallpaper_thumbnails: &[crate::settings_refresh::wallpaper::WallpaperPreview],
    wallpaper_page: usize,
    current_wallpaper: Option<&str>,
    wallpaper_mode: WallpaperMode,
    cursor_size: u32,
    cursor_previews: &crate::cursor::preview::CursorPreviews,
    available_cursor_themes: &[String],
    current_cursor_theme: &str,
    idle_timeout_secs: Option<u64>,
    pinned_apps: &[PinnedApp],
    output_workspaces: &[OutputWorkspaceState],
    display_mode_dropdown_open: Option<usize>,
    display_pages: DisplayPages,
    provider_page: usize,
    printer_snapshot: &PrinterSnapshot,
    audio_snapshot: &AudioSnapshot,
    system_info: &SystemInfo,
    user_accounts: &crate::users::UserState,
    network_state: &NetworkState,
    network_profiles: &[ConnectionProfile],
    network_list_status: crate::network::ListStatus,
    bluetooth_snapshot: &BluetoothSnapshot,
    wifi_networks: &[WifiNetwork],
    pinned_adding: bool,
    all_apps: &[DesktopApp],
    icon_cache: &IconCache,
    armed_power: Option<(&str, f32)>,
    default_apps_index: Option<&crate::default_apps::MimeAppIndex>,
    default_apps_current: &std::collections::HashMap<
        crate::default_apps::DefaultAppCategory,
        String,
    >,
    default_apps_picker_open: Option<crate::default_apps::DefaultAppCategory>,
    default_apps_page: usize,
    default_apps_status: &crate::default_apps::refresh::UiState,
    theme_config: &ThemeConfig,
    state_fn: &dyn Fn(&[usize]) -> WidgetState,
) {
    let expected = (width as usize)
        .saturating_mul(height as usize)
        .saturating_mul(4);
    if canvas.len() != expected {
        return;
    }
    let Some(mut pixmap) = Pixmap::new(width, height) else {
        return;
    };
    let theme = settings_theme_from_config(theme_config);
    pixmap.fill(tiny_skia::Color::from_rgba8(
        theme.palette.background.r,
        theme.palette.background.g,
        theme.palette.background.b,
        niwoe_tokens::ControlCenter::DEFAULT.content_alpha,
    ));
    let root = build_settings_widget_tree(
        width,
        height,
        selected,
        search,
        back_to_rooms,
        available_themes,
        current_theme,
        available_wallpapers,
        wallpaper_thumbnails,
        wallpaper_page,
        current_wallpaper,
        wallpaper_mode,
        cursor_size,
        cursor_previews,
        available_cursor_themes,
        current_cursor_theme,
        idle_timeout_secs,
        pinned_apps,
        output_workspaces,
        display_mode_dropdown_open,
        display_pages,
        provider_page,
        printer_snapshot,
        audio_snapshot,
        system_info,
        user_accounts,
        network_state,
        network_profiles,
        network_list_status,
        bluetooth_snapshot,
        wifi_networks,
        pinned_adding,
        all_apps,
        icon_cache,
        armed_power,
        default_apps_index,
        default_apps_current,
        default_apps_picker_open,
        default_apps_page,
        default_apps_status,
        &theme,
    );
    if let Ok(layout) = niwoe_ui::compute_layout(&*root, niwoe_ui::PixelSize { width, height }) {
        let mut pm = pixmap.as_mut();
        let _ = niwoe_ui::render(&*root, &layout, &mut pm, &theme, state_fn);
        if let Some(id) = focused_widget {
            if let Some((_, area)) = crate::widget_traversal::focus_targets(
                root.as_ref(),
                &layout,
                niwoe_ui::PixelSize { width, height },
            )
            .into_iter()
            .find(|(candidate, _)| *candidate == id)
            {
                niwoe_ui::effect::paint_focus(
                    &mut pm,
                    area,
                    theme.palette.border_focus(),
                    theme.radius.sm,
                );
            }
        }
    }
    blit_rgba_to_argb(pixmap.data(), canvas);
}

fn printer_service_message(service: PrinterServiceState) -> &'static str {
    match service {
        PrinterServiceState::Running => "",
        PrinterServiceState::Stopped => "Der CUPS-Druckdienst läuft nicht.",
        PrinterServiceState::Unavailable => "Der Druckstatus konnte nicht gelesen werden.",
    }
}

fn blit_rgba_to_argb(src: &[u8], dst: &mut [u8]) {
    if src.len() != dst.len() || !src.len().is_multiple_of(4) {
        return;
    }
    for (s, d) in src
        .as_chunks::<4>()
        .0
        .iter()
        .zip(dst.as_chunks_mut::<4>().0.iter_mut())
    {
        d[0] = s[2];
        d[1] = s[1];
        d[2] = s[0];
        d[3] = s[3];
    }
}
