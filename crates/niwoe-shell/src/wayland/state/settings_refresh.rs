impl NiwoeShell {
    pub(crate) fn request_settings_refresh(
        &mut self,
        category: crate::settings_view::SettingsCategory,
    ) {
        self.request_settings_refresh_with_default_apps(category, None);
    }

    pub(crate) fn start_default_apps_change(
        &mut self,
        request: crate::default_apps::refresh::ChangeRequest,
    ) {
        if self.default_apps_index.is_none() {
            return;
        }
        self.request_settings_refresh_with_default_apps(
            crate::settings_view::SettingsCategory::DefaultApps,
            Some(request),
        );
    }

    fn request_settings_refresh_with_default_apps(
        &mut self,
        category: crate::settings_view::SettingsCategory,
        change: Option<crate::default_apps::refresh::ChangeRequest>,
    ) {
        if !crate::settings_refresh::supports(category) {
            return;
        }
        if !self.settings_refresh_inflight.insert(category) {
            return;
        }
        if category == crate::settings_view::SettingsCategory::DefaultApps {
            self.default_apps_status.start(change.is_some());
            self.control_center_nav.widget_focus = None;
        }
        crate::settings_refresh::spawn(
            category,
            crate::settings_refresh::wallpaper::WallpaperRequest {
                selected: self.wallpaper_path.clone(),
                previous: self.wallpaper_thumbnails.clone(),
            },
            crate::cursor::preview::CursorRequest {
                themes: self.available_cursor_themes.clone(),
                size: self.cursor_size,
                previous: self.cursor_previews.clone(),
            },
            self.icon_cache.loader_config(),
            change,
            self.settings_refresh_tx.clone(),
        );
    }

    fn poll_settings_refresh(&mut self, qh: &QueueHandle<Self>) {
        while let Ok(result) = self.settings_refresh_rx.try_recv() {
            self.settings_refresh_inflight.remove(&result.category);
            match result.data {
                crate::settings_refresh::SettingsData::SystemInfo(value) => {
                    self.system_info = value;
                }
                crate::settings_refresh::SettingsData::Users(value) => {
                    self.user_accounts = value;
                }
                crate::settings_refresh::SettingsData::Printers(value) => {
                    self.printer_snapshot = value;
                }
                crate::settings_refresh::SettingsData::Audio(value) => {
                    self.audio_settled = value.is_settled();
                    self.audio_snapshot = value;
                }
                crate::settings_refresh::SettingsData::Network(lists) => {
                    self.network_profiles = lists.profiles;
                    self.wifi_networks = lists.wifi;
                    self.network_list_status = lists.status;
                    if self.network_popup_open
                        && self.network_popup_tab == crate::network_popup::NetworkTab::Wifi
                    {
                        self.draw_network_popup(qh, RepaintReason::Ipc);
                    }
                }
                crate::settings_refresh::SettingsData::Bluetooth(value) => {
                    self.bluetooth_snapshot = value;
                    if self.network_popup_open
                        && self.network_popup_tab == crate::network_popup::NetworkTab::Status
                    {
                        self.draw_network_popup(qh, RepaintReason::Ipc);
                    }
                }
                crate::settings_refresh::SettingsData::DefaultApps(value) => {
                    if value.outcome.as_ref().is_some_and(|outcome| {
                        !matches!(outcome, crate::default_apps::refresh::ChangeOutcome::Failed)
                    }) {
                        self.default_apps_picker_open = None;
                        self.default_apps_page = 0;
                    }
                    self.control_center_nav.widget_focus = None;
                    self.default_apps_status
                        .finish(value.failed_reads, value.outcome.as_ref());
                    self.default_apps_index = Some(value.index);
                    self.default_apps_current = value.current;
                }
                crate::settings_refresh::SettingsData::Wallpaper(value) => {
                    if value.requested_selected != self.wallpaper_path {
                        self.request_settings_refresh(result.category);
                        continue;
                    }
                    let changed = self
                        .available_wallpapers
                        .iter()
                        .map(|e| &e.apply_path)
                        .ne(value.catalog.iter().map(|e| &e.apply_path));
                    if changed {
                        self.wallpaper_page = 0;
                        self.control_center_nav.widget_focus = None;
                        self.ui_preview_widget_state = None;
                    }
                    self.available_wallpapers = value.catalog;
                    self.wallpaper_thumbnails = value.previews;
                }
                crate::settings_refresh::SettingsData::Cursor(value) => {
                    if !value.matches(&self.available_cursor_themes, self.cursor_size) {
                        if self.launcher_settings_open
                            && self.effective_settings_category() == result.category
                        {
                            self.request_settings_refresh(result.category);
                        }
                        continue;
                    }
                    self.cursor_previews = value;
                }
            }
            if self.launcher_state.open
                && self.launcher_settings_open
                && self.effective_settings_category() == result.category
            {
                self.draw_launcher(qh, RepaintReason::Ipc);
            }
        }
    }
}
