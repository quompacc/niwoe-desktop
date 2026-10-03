include!("content/theme.rs");
include!("content/wallpaper.rs");
include!("content/display.rs");
include!("content/pinned_apps.rs");
include!("content/default_apps.rs");
include!("content/system_overview.rs");
include!("content/network.rs");
include!("content/power.rs");
include!("content/users.rs");
include!("content/bluetooth.rs");
include!("content/updates.rs");
include!("content/sound.rs");
include!("content/printers.rs");
include!("content/cursor.rs");

struct SettingsContentContext<'a> {
    content_h: u32,
    content_w: u32,
    query: &'a str,
    available_themes: &'a [String],
    current_theme: &'a str,
    available_wallpapers: &'a [WallpaperEntry],
    wallpaper_thumbnails: &'a [crate::settings_refresh::wallpaper::WallpaperPreview],
    wallpaper_page: usize,
    current_wallpaper: Option<&'a str>,
    wallpaper_mode: WallpaperMode,
    cursor_size: u32,
    cursor_previews: &'a crate::cursor::preview::CursorPreviews,
    available_cursor_themes: &'a [String],
    current_cursor_theme: &'a str,
    idle_timeout_secs: Option<u64>,
    pinned_apps: &'a [PinnedApp],
    output_workspaces: &'a [OutputWorkspaceState],
    display_mode_dropdown_open: Option<usize>,
    display_pages: DisplayPages,
    provider_page: usize,
    printer_snapshot: &'a PrinterSnapshot,
    audio_snapshot: &'a AudioSnapshot,
    system_info: &'a SystemInfo,
    user_accounts: &'a crate::users::UserState,
    armed_power: Option<(&'a str, f32)>,
    network_state: &'a NetworkState,
    network_profiles: &'a [ConnectionProfile],
    network_list_status: crate::network::ListStatus,
    bluetooth_snapshot: &'a BluetoothSnapshot,
    wifi_networks: &'a [WifiNetwork],
    pinned_adding: bool,
    all_apps: &'a [DesktopApp],
    icon_cache: &'a IconCache,
    default_apps_index: Option<&'a crate::default_apps::MimeAppIndex>,
    default_apps_current:
        &'a std::collections::HashMap<crate::default_apps::DefaultAppCategory, String>,
    default_apps_picker_open: Option<crate::default_apps::DefaultAppCategory>,
    default_apps_page: usize,
    default_apps_status: &'a crate::default_apps::refresh::UiState,
    pal: &'a niwoe_ui::style::Palette,
}

#[allow(clippy::too_many_arguments)] // Existing flat settings-state boundary; refactor separately.
pub(crate) fn build_settings_widget_tree(
    width: u32,
    height: u32,
    selected: SettingsCategory,
    search: &str,
    back_to_rooms: bool,
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
    theme: &Theme,
) -> Box<dyn Widget> {
    let pal = theme.palette;

    let c = niwoe_tokens::ControlCenter::DEFAULT;
    let content_w = width.saturating_sub(c.sidebar_width as u32);
    let query = search.trim().to_lowercase();
    let effective_selected =
        effective_settings_category(selected, search, available_themes, available_wallpapers);
    let page = crate::control_center::Page::for_settings(effective_selected);
    let content_h = height.saturating_sub(
        (SETTINGS_CHROME.header_height + category_navigation_height(effective_selected)) as u32,
    );
    let sidebar = Box::new(crate::control_center::Sidebar::new(
        height,
        page,
        if back_to_rooms {
            "‹ Zurück"
        } else {
            "‹ Zurück zum Hub"
        },
    )) as Box<dyn Widget>;
    let content_ctx = SettingsContentContext {
        content_h,
        content_w,
        query: &query,
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
        armed_power,
        network_state,
        network_profiles,
        network_list_status,
        bluetooth_snapshot,
        wifi_networks,
        pinned_adding,
        all_apps,
        icon_cache,
        default_apps_index,
        default_apps_current,
        default_apps_picker_open,
        default_apps_page,
        default_apps_status,
        pal: &pal,
    };

    let content: Box<dyn Widget> = match effective_selected {
        SettingsCategory::Theme => build_theme_content(&content_ctx),
        SettingsCategory::Wallpaper => build_wallpaper_content(&content_ctx),
        SettingsCategory::Display => build_display_content(&content_ctx),
        SettingsCategory::PinnedApps => build_pinned_apps_content(&content_ctx),
        SettingsCategory::DefaultApps => build_default_apps_content(&content_ctx),
        SettingsCategory::SystemOverview => build_system_overview_content(&content_ctx),
        SettingsCategory::Network => build_network_content(&content_ctx),
        SettingsCategory::Power => build_power_content(&content_ctx),
        SettingsCategory::Users => build_users_content(&content_ctx),
        SettingsCategory::Bluetooth => build_bluetooth_content(&content_ctx),
        SettingsCategory::Updates => build_updates_content(&content_ctx),
        SettingsCategory::Sound => build_sound_content(&content_ctx),
        SettingsCategory::Printers => build_printers_content(&content_ctx),
        SettingsCategory::Cursor => build_cursor_content(&content_ctx),
    };

    let title_width = content_w as i32
        - SETTINGS_CHROME.header_pad * 2
        - SETTINGS_CHROME.search_width
        - SETTINGS_CHROME.header_gap;
    let header = Box::new(SettingsHeaderBar {
        width: content_w as i32,
        children: vec![
            Box::new(SettingsTitle {
                width: title_width,
                label: page.label().into(),
            }) as Box<dyn Widget>,
            Box::new(SettingsSearchField {
                width: SETTINGS_CHROME.search_width,
                query: search.into(),
            }) as Box<dyn Widget>,
        ],
    }) as Box<dyn Widget>;
    let navigation = build_category_navigation(content_w, effective_selected);
    let content_column = Box::new(Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Column,
            flex_shrink: 0.0,
            size: UiSize {
                width: ui_length(content_w as f32),
                height: ui_length(height as f32),
            },
            ..Default::default()
        },
        vec![header, navigation, content],
    )) as Box<dyn Widget>;
    Box::new(Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            size: UiSize {
                width: ui_length(width as f32),
                height: ui_length(height as f32),
            },
            ..Default::default()
        },
        vec![sidebar, content_column],
    ))
}
