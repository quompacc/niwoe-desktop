impl NiwoeShell {
    pub(crate) fn open_hub_on_first_login(&mut self) {
        if !self.launcher_state.open {
            self.toggle_launcher();
        }
        self.first_run_send(niwoe_ipc::FirstRunAction::Get, "login");
    }

    pub(crate) fn open_room_management(&mut self, qh: &QueueHandle<Self>) {
        self.window_picker = None;
        if !self.launcher_state.open {
            self.toggle_launcher();
        }
        self.room_management_open = true;
        self.room_management_page = 0;
        self.room_configuration_id = None;
        self.room_configuration_scroll_y = 0;
        self.launcher_settings_open = false;
        self.settings_return_to_room_management = false;
        self.hub_search_active = false;
        self.search_query.clear();
        self.hovered_bento_idx = None;
        self.room_keyboard_focus = None;
        self.control_center_nav = Default::default();
        self.ui_preview_widget_state = None;
        self.launcher_layer
            .set_anchor(Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT);
        self.launcher_layer.set_margin(0, 0, 0, 0);
        self.launcher_layer.set_exclusive_zone(0);
        self.launcher_layer.set_size(0, 0);
        // A Settings -> introduction transition can retain this exact layer
        // size. Keep its usable buffer until a real configure arrives; waiting
        // unconditionally would leave the previous page painted indefinitely.
        self.launcher_dirty = true;
        self.commit_surface(CommitSurfaceKind::Launcher, CommitReason::Input);
        self.draw_panel(qh, RepaintReason::Pointer);
    }

    pub(crate) fn return_to_hub(&mut self, qh: &QueueHandle<Self>) {
        self.control_center_nav = Default::default();
        self.ui_preview_widget_state = None;
        self.launcher_settings_open = false;
        self.settings_return_to_room_management = false;
        self.workspace_state.rooms.panel.open = false;
        self.panel_last_signature = None;
        self.panel_dirty = true;
        self.room_management_open = false;
        self.room_configuration_id = None;
        self.room_configuration_save_pending = false;
        self.workspace_state.rooms.edit = None;
        self.hovered_bento_idx = None;
        self.room_keyboard_focus = None;
        self.launcher_layer
            .set_anchor(Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT);
        self.launcher_layer.set_margin(0, 0, 0, 0);
        self.launcher_layer.set_exclusive_zone(-1);
        self.launcher_layer.set_size(0, 0);
        self.launcher_configured = false;
        self.launcher_dirty = true;
        self.commit_surface(CommitSurfaceKind::Launcher, CommitReason::Input);
        self.draw_panel(qh, RepaintReason::Pointer);
    }

    pub(crate) fn open_system_settings_from_ipc(&mut self) {
        self.pause_first_run();
        if !self.launcher_state.open {
            self.toggle_launcher();
        }
        self.prepare_settings_category(crate::settings_view::SettingsCategory::SystemOverview);
        self.request_settings_refresh(crate::settings_view::SettingsCategory::SystemOverview);
        self.launcher_dirty = true;
        self.panel_dirty = true;
    }

    fn toggle_launcher(&mut self) {
        if self.launcher_state.open {
            self.pause_first_run();
        }
        self.window_picker = None;
        self.settings_return_to_room_management = false;
        let open_before = self.launcher_state.open;
        if !open_before && self.calendar_popup_open {
            self.close_calendar_popup(CommitReason::Input);
        }
        if !open_before && self.workspace_popup_open {
            self.close_workspace_popup(CommitReason::Input);
        }
        if !open_before && self.network_popup_open {
            self.close_network_popup(CommitReason::Input);
        }
        if !open_before && self.audio_popup_open {
            self.close_audio_popup(CommitReason::Input);
        }
        self.hub.clear();
        self.hub.page = 0;
        self.hub.selected = None;
        self.launcher_state.toggle();
        let open_after = self.launcher_state.open;
        if self.launcher_state.open {
            // Warm the grid app-icons on first open (deferred from startup).
            self.warm_launcher_icons();
            // Refresh the app list off-thread so newly-installed apps show up
            // without ever blocking the open (LAUNCH-2). The cached list renders
            // now; the fresh one swaps in within a tick.
            self.request_launcher_apps_refresh();
            // Fullscreen layer surface, transparent except for the launcher card.
            // This gives the software shadow room to render around the card
            // without moving the visible launcher away from the panel.
            self.launcher_layer
                .set_anchor(Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT);
            self.launcher_layer.set_margin(0, 0, 0, 0);
            self.launcher_layer.set_exclusive_zone(-1);
            self.launcher_layer.set_size(0, 0);
            self.launcher_layer
                .set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
            tracing::debug!(
                "launcher focus request: keyboard_interactivity=Exclusive (fullscreen)"
            );
            self.launcher_is_fullscreen = true;
            self.launcher_configured = false;
            self.commit_surface(CommitSurfaceKind::Launcher, CommitReason::Input);
            self.search_query.clear();
            self.hub_search_active = false;
            self.hovered_bento_idx = None;
            self.room_keyboard_focus = None;
            self.hovered_app_card_idx = None;
            self.app_view_scroll_y = 0;
            self.launcher_selected_idx = None;
        } else {
            self.launcher_is_fullscreen = false;
            self.launcher_settings_open = false;
            self.workspace_state.rooms.panel.open = false;
            self.panel_last_signature = None;
            self.panel_dirty = true;
            self.room_management_open = false;
            self.room_configuration_id = None;
            self.room_configuration_save_pending = false;
            self.workspace_state.rooms.edit = None;
            self.hub_search_active = false;
            self.settings_category = crate::settings_view::SettingsCategory::default();
            self.launcher_layer
                .set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
            tracing::debug!("launcher focus release: keyboard_interactivity=OnDemand");
        }
        // Force a panel re-commit on launcher toggle to avoid stale/missing panel attachment.
        self.panel_last_signature = None;
        self.launcher_dirty = true;
        self.panel_dirty = true;
        // Temporarily promoted to info! to diagnose the "launcher won't open"
        // report on real hardware (debug logging is off in prod). Revert once fixed.
        tracing::info!(
            "toggle_launcher: open_before={} open_after={} panel_configured={} launcher_configured={} launcher_size={}x{} panel_dirty={} launcher_dirty={} keyboard_focus={:?}",
            open_before,
            open_after,
            self.panel_configured,
            self.launcher_configured,
            self.launcher_width,
            self.launcher_height,
            self.panel_dirty,
            self.launcher_dirty,
            self.keyboard_focus
        );
    }

    pub(crate) fn open_settings_category(
        &mut self,
        qh: &QueueHandle<Self>,
        category: crate::settings_view::SettingsCategory,
    ) {
        if self.calendar_popup_open {
            self.close_calendar_popup(CommitReason::Input);
        }
        if self.workspace_popup_open {
            self.close_workspace_popup(CommitReason::Input);
        }
        if self.network_popup_open {
            self.close_network_popup(CommitReason::Input);
        }
        if self.audio_popup_open {
            self.close_audio_popup(CommitReason::Input);
        }
        if !self.launcher_state.open {
            self.toggle_launcher();
        }
        self.prepare_settings_category(category);
        self.display_mode_dropdown_open = None;
        self.display_pages = Default::default();
        self.request_settings_refresh(category);
        self.launcher_dirty = true;
        self.panel_dirty = true;
        self.draw_panel(qh, RepaintReason::Pointer);
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    fn prepare_settings_category(&mut self, category: crate::settings_view::SettingsCategory) {
        if !self.launcher_settings_open {
            self.settings_return_to_room_management = self.room_management_open;
        }
        if !self.room_management_open && !self.launcher_settings_open {
            // Every settings entry uses the same full work-area layer as rooms.
            self.launcher_layer
                .set_anchor(Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT);
            self.launcher_layer.set_margin(0, 0, 0, 0);
            self.launcher_layer.set_exclusive_zone(0);
            self.launcher_layer.set_size(0, 0);
            self.launcher_configured = false;
            self.commit_surface(CommitSurfaceKind::Launcher, CommitReason::Input);
        }
        self.room_management_open = false;
        // Keep a room/panel draft while visiting its settings; Escape restores
        // this exact origin rather than silently discarding it.
        self.control_center_nav = Default::default();
        self.ui_preview_widget_state = None;
        self.settings_search.clear();
        self.default_apps_picker_open = None;
        self.default_apps_page = 0;
        self.display_mode_dropdown_open = None;
        self.display_pages = Default::default();
        self.launcher_settings_open = true;
        self.settings_category = category;
        self.wallpaper_page = 0;
        self.provider_page = 0;
    }

    pub(crate) fn close_settings(&mut self, qh: &QueueHandle<Self>, reason: RepaintReason) {
        self.launcher_settings_open = false;
        self.ui_preview_widget_state = None;
        if self.settings_return_to_room_management {
            let configuration = self.room_configuration_id;
            let scroll = self.room_configuration_scroll_y;
            let page = self.room_management_page;
            self.open_room_management(qh);
            self.room_configuration_id = configuration;
            self.room_configuration_scroll_y = scroll;
            self.room_management_page = page;
            self.draw_launcher(qh, reason);
        } else {
            self.return_to_hub(qh);
        }
        self.settings_return_to_room_management = false;
    }

    fn open_sound_settings_from_tray(&mut self, reason: CommitReason) {
        if self.calendar_popup_open {
            self.close_calendar_popup(reason);
        }
        if self.workspace_popup_open {
            self.close_workspace_popup(reason);
        }
        if self.network_popup_open {
            self.close_network_popup(reason);
        }
        if self.audio_popup_open {
            self.close_audio_popup(reason);
        }
        if !self.launcher_state.open {
            self.toggle_launcher();
        }
        self.prepare_settings_category(crate::settings_view::SettingsCategory::Sound);
        self.request_settings_refresh(crate::settings_view::SettingsCategory::Sound);
        self.launcher_dirty = true;
        self.panel_dirty = true;
    }

    fn open_network_settings_from_tray(&mut self, reason: CommitReason) {
        if self.calendar_popup_open {
            self.close_calendar_popup(reason);
        }
        if self.workspace_popup_open {
            self.close_workspace_popup(reason);
        }
        if self.network_popup_open {
            self.close_network_popup(reason);
        }
        if self.audio_popup_open {
            self.close_audio_popup(reason);
        }
        if !self.launcher_state.open {
            self.toggle_launcher();
        }
        self.prepare_settings_category(crate::settings_view::SettingsCategory::Network);
        self.request_settings_refresh(crate::settings_view::SettingsCategory::Network);
        self.launcher_dirty = true;
        self.panel_dirty = true;
    }

    pub(super) fn toggle_calendar_popup(&mut self, reason: CommitReason) {
        if self.calendar_popup_open {
            self.close_calendar_popup(reason);
            return;
        }

        if self.network_popup_open {
            self.close_network_popup(reason);
        }
        if self.workspace_popup_open {
            self.close_workspace_popup(reason);
        }

        if self.launcher_state.open {
            self.launcher_state.close();
            self.launcher_layer
                .set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
            self.unmap_launcher(reason);
        }

        self.calendar_popup_open = true;
        self.calendar_layer.set_anchor(Anchor::TOP);
        self.calendar_layer
            .set_margin(crate::PANEL_POPUP_TOP_MARGIN, 0, 0, 0);
        self.calendar_layer.set_exclusive_zone(0);
        self.calendar_layer.set_size(
            crate::popup_surface_w(crate::CALENDAR_POPUP_WIDTH),
            crate::popup_surface_h(crate::CALENDAR_POPUP_HEIGHT),
        );
        self.calendar_layer
            .set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
        self.calendar_width = crate::popup_surface_w(crate::CALENDAR_POPUP_WIDTH);
        self.calendar_height = crate::popup_surface_h(crate::CALENDAR_POPUP_HEIGHT);
        self.calendar_dirty = true;
        tracing::debug!(
            "toggle_calendar_popup: open_after={} configured={} size={}x{} keyboard_focus={:?}",
            self.calendar_popup_open,
            self.calendar_configured,
            self.calendar_width,
            self.calendar_height,
            self.keyboard_focus
        );
    }

    pub(crate) fn close_calendar_popup(&mut self, reason: CommitReason) -> bool {
        if !self.calendar_popup_open {
            return false;
        }
        self.calendar_popup_open = false;
        self.calendar_layer
            .set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
        self.unmap_calendar_popup(reason);
        tracing::debug!(
            "close_calendar_popup: open_after={} configured={} keyboard_focus={:?}",
            self.calendar_popup_open,
            self.calendar_configured,
            self.keyboard_focus
        );
        true
    }

    pub(super) fn toggle_workspace_popup(&mut self, reason: CommitReason) {
        if self.workspace_popup_open {
            self.close_workspace_popup(reason);
            return;
        }

        if self.launcher_state.open {
            self.launcher_state.close();
            self.launcher_layer
                .set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
            self.unmap_launcher(reason);
        }

        if self.calendar_popup_open {
            self.close_calendar_popup(reason);
        }
        if self.network_popup_open {
            self.close_network_popup(reason);
        }

        self.workspace_state
            .select_active(self.panel_active_workspace());
        self.close_desktop_context_menu_from_ipc();
        self.workspace_popup_open = true;
        self.workspace_hover_idx = None;
        self.workspace_layer.set_anchor(Anchor::TOP | Anchor::LEFT);
        self.workspace_layer.set_margin(
            crate::PANEL_POPUP_TOP_MARGIN,
            0,
            0,
            self.workspace_popup_left_margin(),
        );
        self.workspace_layer.set_exclusive_zone(0);
        self.workspace_layer.set_size(
            crate::popup_surface_w(crate::WORKSPACE_POPUP_WIDTH),
            crate::popup_surface_h(crate::WORKSPACE_POPUP_HEIGHT),
        );
        self.workspace_layer
            .set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
        self.workspace_width = crate::popup_surface_w(crate::WORKSPACE_POPUP_WIDTH);
        self.workspace_height = crate::popup_surface_h(crate::WORKSPACE_POPUP_HEIGHT);
        self.workspace_dirty = true;
        tracing::debug!(
            "toggle_workspace_popup: open_after={} configured={} size={}x{} keyboard_focus={:?}",
            self.workspace_popup_open,
            self.workspace_configured,
            self.workspace_width,
            self.workspace_height,
            self.keyboard_focus
        );
    }

    pub(crate) fn close_workspace_popup(&mut self, reason: CommitReason) -> bool {
        if !self.workspace_popup_open {
            return false;
        }
        self.workspace_popup_open = false;
        self.workspace_state.rooms.edit = None;
        self.workspace_hover_idx = None;
        self.workspace_layer
            .set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
        self.unmap_workspace_popup(reason);
        tracing::debug!(
            "close_workspace_popup: open_after={} configured={} keyboard_focus={:?}",
            self.workspace_popup_open,
            self.workspace_configured,
            self.keyboard_focus
        );
        true
    }

    pub(super) fn toggle_network_popup(&mut self, reason: CommitReason) {
        self.pause_first_run();
        if self.network_popup_open {
            self.close_network_popup(reason);
            return;
        }
        if self.volume_osd_open {
            self.close_volume_osd(reason);
        }

        if self.launcher_state.open {
            self.launcher_state.close();
            self.launcher_layer
                .set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
            self.unmap_launcher(reason);
        }
        if self.calendar_popup_open {
            self.close_calendar_popup(reason);
        }
        if self.workspace_popup_open {
            self.close_workspace_popup(reason);
        }
        if self.audio_popup_open {
            self.close_audio_popup(reason);
        }

        self.network_popup_open = true;
        self.network_popup_tab = crate::network_popup::NetworkTab::Status;
        crate::quick_settings_popup::reset_keyboard_focus();
        // Refresh the live network state on open so the Status tab always shows
        // the current primary connection (e.g. right after connecting Wi-Fi or
        // unplugging the cable), not the last timer-polled snapshot.
        let network_poll_started = std::time::Instant::now();
        self.network_controller.poll();
        if network_poll_started.elapsed() >= std::time::Duration::from_millis(50) {
            tracing::warn!(
                elapsed_ms = network_poll_started.elapsed().as_millis(),
                "slow system deck network poll"
            );
        }
        self.request_settings_refresh(crate::settings_view::SettingsCategory::Bluetooth);
        self.network_layer.set_anchor(Anchor::TOP | Anchor::RIGHT);
        self.network_layer.set_margin(
            crate::PANEL_POPUP_TOP_MARGIN,
            crate::NETWORK_POPUP_RIGHT_MARGIN,
            0,
            0,
        );
        self.network_layer.set_exclusive_zone(0);
        self.network_layer.set_size(
            crate::popup_surface_w(crate::NETWORK_POPUP_WIDTH),
            crate::popup_surface_h(crate::NETWORK_POPUP_HEIGHT),
        );
        self.network_layer
            .set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
        self.network_width = crate::popup_surface_w(crate::NETWORK_POPUP_WIDTH);
        self.network_height = crate::popup_surface_h(crate::NETWORK_POPUP_HEIGHT);
        self.network_dirty = true;
        // If we just unmapped (transitioning from another shared-layer popup),
        // flush the pending state so the compositor sends a fresh configure
        // before any buffer commit. draw_network_popup will skip until the
        // configure handler flips network_configured back to true.
        if !self.network_configured {
            self.network_layer.commit();
        }
        tracing::debug!(
            "toggle_network_popup: open_after={} configured={} size={}x{} keyboard_focus={:?}",
            self.network_popup_open,
            self.network_configured,
            self.network_width,
            self.network_height,
            self.keyboard_focus
        );
    }
}
