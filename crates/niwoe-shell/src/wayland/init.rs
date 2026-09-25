use std::{cell::RefCell, time::Instant};
pub(super) mod assets;
mod commit;
mod flags;
use niwoe_config::{NiwoeConfig, ThemeManager};
use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    reexports::{calloop::EventLoop, calloop_wayland_source::WaylandSource},
    registry::RegistryState,
    seat::SeatState,
    shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerShell},
    shm::{slot::SlotPool, Shm},
};
use tracing::{debug, info, warn};
use wayland_client::{globals::registry_queue_init, Connection, QueueHandle};
use wayland_protocols::ext::{
    image_capture_source::v1::client::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1,
    image_copy_capture::v1::client::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1,
};

use crate::{
    default_pinned_apps, launcher, network::NetworkController, panel, TextRenderer,
    AUDIO_POPUP_HEIGHT, AUDIO_POPUP_WIDTH, CALENDAR_POPUP_HEIGHT, CALENDAR_POPUP_RIGHT_MARGIN,
    CALENDAR_POPUP_WIDTH, LAUNCHER_HEIGHT, LAUNCHER_WIDTH, NETWORK_POPUP_HEIGHT,
    NETWORK_POPUP_RIGHT_MARGIN, NETWORK_POPUP_WIDTH, SHELL_POPUP_BOTTOM_MARGIN,
    THUMBNAIL_POPUP_HEIGHT, THUMBNAIL_POPUP_MAX_WIDTH, WORKSPACE_POPUP_HEIGHT,
    WORKSPACE_POPUP_LEFT_MARGIN, WORKSPACE_POPUP_WIDTH,
};

use super::{calendar::CalendarDisplayPolicy, CommitStats, IpcClient, NiwoeShell, SurfaceKind};

pub(crate) fn initialize(
    event_loop: &mut EventLoop<'_, NiwoeShell>,
) -> Result<(NiwoeShell, QueueHandle<NiwoeShell>), Box<dyn std::error::Error>> {
    let conn = Connection::connect_to_env()?;
    info!("Connected to Wayland display");
    let (globals, event_queue) = registry_queue_init(&conn)?;
    info!("Registry initialized");
    let qh = event_queue.handle();
    WaylandSource::new(conn.clone(), event_queue).insert(event_loop.handle())?;

    let screencopy_manager = globals
        .bind::<ExtImageCopyCaptureManagerV1, _, _>(&qh, 1..=1, ())
        .ok();
    let capture_source_manager = globals
        .bind::<ExtOutputImageCaptureSourceManagerV1, _, _>(&qh, 1..=1, ())
        .ok();

    let compositor = CompositorState::bind(&globals, &qh).expect("wl_compositor is not available");
    let layer_shell = LayerShell::bind(&globals, &qh).expect("wlr layer shell is not available");
    info!("Layer shell protocol bound");
    let shm = Shm::bind(&globals, &qh).expect("wl_shm is not available");

    let desktop_surface = compositor.create_surface(&qh);
    let desktop_layer = layer_shell.create_layer_surface(
        &qh,
        desktop_surface,
        Layer::Background,
        Some("niwoe-desktop"),
        None,
    );
    desktop_layer.set_anchor(Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT);
    desktop_layer.set_size(0, 0);
    desktop_layer.set_exclusive_zone(0);
    desktop_layer.set_keyboard_interactivity(KeyboardInteractivity::None);
    info!("Desktop background surface created without input buffer");

    let desktop_menu_surface = compositor.create_surface(&qh);
    let desktop_menu_layer = layer_shell.create_layer_surface(
        &qh,
        desktop_menu_surface,
        Layer::Overlay,
        Some("niwoe-desktop-menu"),
        None,
    );
    desktop_menu_layer.set_anchor(Anchor::TOP | Anchor::LEFT);
    desktop_menu_layer.set_size(
        crate::popup_surface_w(crate::context_menu::MENU_WIDTH as u32),
        crate::popup_surface_h(1),
    );
    desktop_menu_layer.set_exclusive_zone(0);
    desktop_menu_layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    info!("Desktop menu surface created");

    // Screenshot consent modal: centered overlay, grabs keyboard while shown so
    // Esc/Enter work. Mapped only when a consent request arrives.
    let consent_surface = compositor.create_surface(&qh);
    let consent_layer = layer_shell.create_layer_surface(
        &qh,
        consent_surface,
        Layer::Overlay,
        Some("niwoe-screenshot-consent"),
        None,
    );
    consent_layer.set_size(
        crate::screenshot_consent::MODAL_WIDTH as u32,
        crate::screenshot_consent::MODAL_HEIGHT as u32,
    );
    consent_layer.set_exclusive_zone(0);
    consent_layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    info!("Screenshot consent surface created");

    // Wi-Fi password modal: centered overlay, same lifecycle as the consent
    // modal. Mapped only while the user is entering a password.
    let wifi_modal_surface = compositor.create_surface(&qh);
    let wifi_modal_layer = layer_shell.create_layer_surface(
        &qh,
        wifi_modal_surface,
        Layer::Overlay,
        Some("niwoe-wifi-password"),
        None,
    );
    wifi_modal_layer.set_size(
        crate::wifi_password_modal::MODAL_WIDTH as u32,
        crate::wifi_password_modal::MODAL_HEIGHT as u32,
    );
    wifi_modal_layer.set_exclusive_zone(0);
    wifi_modal_layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    info!("Wi-Fi password modal surface created");

    // Screenshot region picker: fullscreen overlay anchored on all four
    // sides so the compositor sizes it to the output. Keyboard-exclusive
    // while mapped (Enter confirms, Esc cancels). Stays unmapped (no buffer)
    // until the first ScreenshotRegionRequest arrives.
    let region_picker_surface = compositor.create_surface(&qh);
    let region_picker_layer = layer_shell.create_layer_surface(
        &qh,
        region_picker_surface,
        Layer::Overlay,
        Some("niwoe-screenshot-region-picker"),
        None,
    );
    region_picker_layer.set_anchor(
        smithay_client_toolkit::shell::wlr_layer::Anchor::TOP
            | smithay_client_toolkit::shell::wlr_layer::Anchor::BOTTOM
            | smithay_client_toolkit::shell::wlr_layer::Anchor::LEFT
            | smithay_client_toolkit::shell::wlr_layer::Anchor::RIGHT,
    );
    // -1 = ignore every other layer's exclusive zone (panel etc.); the
    // picker is a true full-output overlay so the user can drag-select
    // across the panel area too.
    region_picker_layer.set_exclusive_zone(-1);
    region_picker_layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    info!("Screenshot region picker surface created");
    let panel_surface = compositor.create_surface(&qh);
    let panel =
        layer_shell.create_layer_surface(&qh, panel_surface, Layer::Top, Some("niwoe-panel"), None);
    panel.set_anchor(Anchor::TOP | Anchor::LEFT | Anchor::RIGHT);
    panel.set_size(0, crate::PANEL_SURFACE_HEIGHT);
    panel.set_exclusive_zone(crate::PANEL_SURFACE_HEIGHT as i32);
    panel.set_keyboard_interactivity(KeyboardInteractivity::None);
    info!("Panel surface created");

    let launcher_surface = compositor.create_surface(&qh);
    let launcher_layer = layer_shell.create_layer_surface(
        &qh,
        launcher_surface,
        Layer::Top,
        Some("niwoe-launcher"),
        None,
    );
    launcher_layer.set_anchor(Anchor::BOTTOM);
    launcher_layer.set_margin(crate::PANEL_POPUP_TOP_MARGIN, 0, 0, 0);
    launcher_layer.set_size(LAUNCHER_WIDTH, LAUNCHER_HEIGHT);
    launcher_layer.set_exclusive_zone(0);
    launcher_layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    debug!(
        "Launcher surface created: namespace=niwoe-launcher layer=Top anchor=Bottom size={}x{} margin_bottom={} exclusive_zone=0 keyboard_interactivity=Exclusive",
        LAUNCHER_WIDTH,
        LAUNCHER_HEIGHT,
        SHELL_POPUP_BOTTOM_MARGIN
    );

    let calendar_surface = compositor.create_surface(&qh);
    // Dedicated overlay namespace keeps compositor glass classification exact.
    let calendar_layer = layer_shell.create_layer_surface(
        &qh,
        calendar_surface,
        Layer::Overlay,
        Some("niwoe-calendar-popup"),
        None,
    );
    calendar_layer.set_anchor(Anchor::TOP | Anchor::RIGHT);
    let calendar_right = CALENDAR_POPUP_RIGHT_MARGIN;
    calendar_layer.set_margin(crate::PANEL_POPUP_TOP_MARGIN, calendar_right, 0, 0);
    calendar_layer.set_size(
        crate::popup_surface_w(CALENDAR_POPUP_WIDTH),
        crate::popup_surface_h(CALENDAR_POPUP_HEIGHT),
    );
    calendar_layer.set_exclusive_zone(0);
    calendar_layer.set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
    debug!(
        "Calendar popup surface created: namespace=niwoe-calendar-popup layer=Overlay anchor=Top|Right size={}x{} margin_bottom={} margin_right={} exclusive_zone=0 keyboard_interactivity=OnDemand",
        CALENDAR_POPUP_WIDTH,
        CALENDAR_POPUP_HEIGHT,
        SHELL_POPUP_BOTTOM_MARGIN,
        calendar_right
    );

    let workspace_surface = compositor.create_surface(&qh);
    let workspace_layer = layer_shell.create_layer_surface(
        &qh,
        workspace_surface,
        Layer::Overlay,
        Some("niwoe-workspace-popup"),
        None,
    );
    workspace_layer.set_anchor(Anchor::TOP | Anchor::LEFT);
    let workspace_left = WORKSPACE_POPUP_LEFT_MARGIN;
    workspace_layer.set_margin(crate::PANEL_POPUP_TOP_MARGIN, 0, 0, workspace_left);
    workspace_layer.set_size(
        crate::popup_surface_w(WORKSPACE_POPUP_WIDTH),
        crate::popup_surface_h(WORKSPACE_POPUP_HEIGHT),
    );
    workspace_layer.set_exclusive_zone(0);
    workspace_layer.set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
    debug!(
        "Workspace popup surface created: namespace=niwoe-workspace-popup layer=Overlay anchor=Top|Left size={}x{} margin_bottom={} margin_right={} exclusive_zone=0 keyboard_interactivity=OnDemand",
        WORKSPACE_POPUP_WIDTH,
        WORKSPACE_POPUP_HEIGHT,
        SHELL_POPUP_BOTTOM_MARGIN,
        WORKSPACE_POPUP_LEFT_MARGIN
    );

    let network_surface = compositor.create_surface(&qh);
    let network_layer = layer_shell.create_layer_surface(
        &qh,
        network_surface,
        Layer::Overlay,
        Some("niwoe-network-popup"),
        None,
    );
    network_layer.set_anchor(Anchor::TOP | Anchor::RIGHT);
    network_layer.set_margin(
        crate::PANEL_POPUP_TOP_MARGIN,
        NETWORK_POPUP_RIGHT_MARGIN,
        0,
        0,
    );
    network_layer.set_size(
        crate::popup_surface_w(NETWORK_POPUP_WIDTH),
        crate::popup_surface_h(NETWORK_POPUP_HEIGHT),
    );
    network_layer.set_exclusive_zone(0);
    network_layer.set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
    debug!(
        "Network popup surface created: namespace=niwoe-network-popup layer=Overlay anchor=Top|Right size={}x{} margin_bottom={} margin_right={} exclusive_zone=0 keyboard_interactivity=OnDemand",
        NETWORK_POPUP_WIDTH,
        NETWORK_POPUP_HEIGHT,
        SHELL_POPUP_BOTTOM_MARGIN,
        NETWORK_POPUP_RIGHT_MARGIN
    );

    // Phase A1.3: notification popup, anchored top-right, no keyboard
    // input (purely informational). Stays unmapped (1x1 commit) when
    // the notification queue is empty.
    let notification_surface = compositor.create_surface(&qh);
    let notification_layer = layer_shell.create_layer_surface(
        &qh,
        notification_surface,
        Layer::Overlay,
        Some("niwoe-notification"),
        None,
    );
    notification_layer.set_anchor(Anchor::TOP | Anchor::RIGHT);
    notification_layer.set_margin(
        crate::NOTIFICATION_TOP_MARGIN,
        crate::NOTIFICATION_RIGHT_MARGIN,
        0,
        0,
    );
    notification_layer.set_size(
        crate::popup_surface_w(crate::NOTIFICATION_WIDTH),
        crate::popup_surface_h(crate::NOTIFICATION_HEIGHT),
    );
    notification_layer.set_exclusive_zone(0);
    notification_layer.set_keyboard_interactivity(KeyboardInteractivity::None);
    debug!(
        "Notification surface created: namespace=niwoe-notification layer=Overlay anchor=Top|Right size={}x{} margin_top={} margin_right={} exclusive_zone=0 keyboard_interactivity=None",
        crate::NOTIFICATION_WIDTH,
        crate::NOTIFICATION_HEIGHT,
        crate::NOTIFICATION_TOP_MARGIN,
        crate::NOTIFICATION_RIGHT_MARGIN
    );

    let thumbnail_surface = compositor.create_surface(&qh);
    let thumbnail_layer = layer_shell.create_layer_surface(
        &qh,
        thumbnail_surface,
        Layer::Overlay,
        Some("niwoe-thumbnail-popup"),
        None,
    );
    thumbnail_layer.set_anchor(Anchor::TOP | Anchor::LEFT);
    thumbnail_layer.set_margin(crate::PANEL_POPUP_TOP_MARGIN, 0, 0, 0);
    thumbnail_layer.set_size(
        crate::popup_surface_w(THUMBNAIL_POPUP_MAX_WIDTH),
        crate::popup_surface_h(THUMBNAIL_POPUP_HEIGHT),
    );
    thumbnail_layer.set_exclusive_zone(0);
    thumbnail_layer.set_keyboard_interactivity(KeyboardInteractivity::None);

    let niwoe_config = NiwoeConfig::load();
    let mut theme_manager = ThemeManager::new();
    if !niwoe_config.general.theme.trim().is_empty()
        && niwoe_config.general.theme != theme_manager.current().name
    {
        if let Err(err) = theme_manager.set_theme(&niwoe_config.general.theme) {
            warn!(
                "Failed to load theme {:?} from config: {} — using current theme {:?}",
                niwoe_config.general.theme,
                err,
                theme_manager.current().name
            );
        }
    }
    let available_themes = theme_manager.available_themes();
    let theme = theme_manager.current().config.clone();
    info!("Theme loaded");

    if let Err(err) = conn.flush() {
        warn!("Failed to flush Wayland connection: {}", err);
    }
    info!("Wayland connection flushed, entering event loop");

    let font = TextRenderer::new(&theme.fonts.ui, 13);
    crate::font_resolve::apply_theme_ui_font(&theme);
    let pool = SlotPool::new(1024 * 1024 * 16, &shm)?;
    let launcher_apps = launcher::DesktopApp::load_system();

    let pinned_apps: Vec<panel::PinnedApp> = if niwoe_config.panel.pinned.is_empty() {
        default_pinned_apps()
    } else {
        niwoe_config
            .panel
            .pinned
            .iter()
            .map(|app| panel::PinnedApp {
                label: app.label.clone(),
                program: app.program.clone(),
                args: vec![],
                terminal: false,
                icon_name: app.icon.clone(),
            })
            .collect()
    };
    let icon_cache = assets::build_icon_cache(&theme, &pinned_apps);
    let mut network_controller = NetworkController::new();
    network_controller.poll();
    // Startup snapshots are cached; later Settings refreshes run off-thread.
    let printer_snapshot = crate::printers::PrinterSnapshot::poll();
    // The tick retries audio briefly when the session stack starts late.
    let audio_snapshot = crate::audio::AudioSnapshot::poll();
    let audio_settled = audio_snapshot.is_settled();
    let power_profile_init = crate::power_profile::current();
    let available_wallpapers = niwoe_config::NiwoeConfig::scan_wallpaper_dirs();
    let network_profiles = crate::network::list_saved_connections();
    let system_info = crate::sysinfo::SystemInfo::gather();
    let (settings_refresh_tx, settings_refresh_rx) = std::sync::mpsc::channel();
    let ipc_client = IpcClient::connect();

    let commit_stats_enabled = flags::enabled("NIWOE_SHELL_COMMIT_STATS");
    let render_stats_enabled = flags::enabled("NIWOE_SHELL_RENDER_STATS");

    let mut shell = include!("init/shell_state.rs");
    commit::commit_initial_surfaces(&mut shell);
    Ok((shell, qh))
}
