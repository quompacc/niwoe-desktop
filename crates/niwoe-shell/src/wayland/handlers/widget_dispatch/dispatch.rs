impl NiwoeShell {
    pub(crate) fn dispatch_widget_action(
        &mut self,
        qh: &QueueHandle<NiwoeShell>,
        action: WidgetAction,
    ) {
        // Cancel an armed power button on any non-power action — the user
        // changed their mind by clicking somewhere else. Power actions are
        // skipped here; their own handler arms or consumes the armed state.
        let is_power = matches!(
            action,
            WidgetAction::PowerOff
                | WidgetAction::PowerRestart
                | WidgetAction::PowerSleep
                | WidgetAction::PowerLock
                | WidgetAction::PowerLogout
        );
        if !is_power && self.armed_power.is_some() {
            self.armed_power = None;
            self.draw_launcher(qh, RepaintReason::Pointer);
        }

        match action {
            WidgetAction::OpenFirstRun => self.open_first_run(qh),
            WidgetAction::OpenControlCenter(page) => self.navigate_control_center_page(qh, page),
            WidgetAction::LaunchApp { .. } | WidgetAction::LaunchExec(_) => {
                self.dispatch_launch_action(qh, action);
            }
            WidgetAction::ToggleCalendar
            | WidgetAction::ToggleNetworkPopup
            | WidgetAction::ToggleWorkspacePopup => self.dispatch_popup_action(qh, action),
            WidgetAction::ToggleSettings
            | WidgetAction::SetSettingsCategory(_)
            | WidgetAction::ApplyThemeByIndex(_)
            | WidgetAction::ApplyWallpaperByIndex(_)
            | WidgetAction::PageWallpaper { .. }
            | WidgetAction::SetWallpaperMode(_)
            | WidgetAction::SetCursorSize(_)
            | WidgetAction::ApplyCursorThemeByIndex(_)
            | WidgetAction::SetIdleTimeout(_)
            | WidgetAction::SetDefaultSinkVolume(_)
            | WidgetAction::ToggleDefaultSinkMute
            | WidgetAction::SetDefaultAudioOutput(_)
            | WidgetAction::SetDefaultAudioInput(_)
            | WidgetAction::ActivateConnection(_)
            | WidgetAction::WifiConnect(_)
            | WidgetAction::ToggleBluetoothPower
            | WidgetAction::ToggleBluetoothScan
            | WidgetAction::BluetoothDevice(_)
            | WidgetAction::BrowseWallpaper
            | WidgetAction::SetPrimaryOutput(_)
            | WidgetAction::CycleOutputScale(_)
            | WidgetAction::CycleOutputTransform(_)
            | WidgetAction::ToggleOutputModeDropdown(_)
            | WidgetAction::PageDisplay { .. }
            | WidgetAction::PageProviders { .. }
            | WidgetAction::SetOutputMode { .. }
            | WidgetAction::DefaultAppsAutoSet
            | WidgetAction::DefaultAppsTogglePicker(_)
            | WidgetAction::DefaultAppsPick { .. }
            | WidgetAction::DefaultAppsClosePicker => self.dispatch_settings_action(qh, action),
            WidgetAction::PageDefaultApps { forward } => self.page_default_apps(qh, forward),
            WidgetAction::PowerOff
            | WidgetAction::PowerRestart
            | WidgetAction::PowerSleep
            | WidgetAction::PowerLock
            | WidgetAction::PowerLogout => self.dispatch_power_action(qh, action),
            WidgetAction::PinnedMoveUp(_)
            | WidgetAction::PinnedMoveDown(_)
            | WidgetAction::PinnedRemove(_)
            | WidgetAction::PinnedOpenAdd
            | WidgetAction::PinnedCloseAdd
            | WidgetAction::PinnedAddApp(_) => self.dispatch_pinned_action(qh, action),
        }
    }

    fn dispatch_launch_action(&mut self, qh: &QueueHandle<NiwoeShell>, action: WidgetAction) {
        match action {
            WidgetAction::LaunchApp { program, args } => {
                if let Err(err) = std::process::Command::new(&program).args(&args).spawn() {
                    tracing::warn!("launch failed: {:?}", err);
                }
            }
            WidgetAction::LaunchExec(exec) => {
                if let Err(err) = std::process::Command::new(&exec).spawn() {
                    tracing::warn!("launch failed: {:?}", err);
                }
            }
            _ => unreachable!("non launch action routed to launch dispatcher"),
        }
        // Mirror handle_launcher_click's behavior: any launch via a widget
        // click should dismiss the launcher so the new window is not occluded.
        // No-op when launcher is closed (close_launcher_after_launch checks).
        self.close_launcher_after_launch(qh, RepaintReason::Pointer);
    }

    fn dispatch_popup_action(&mut self, qh: &QueueHandle<NiwoeShell>, action: WidgetAction) {
        match action {
            WidgetAction::ToggleCalendar => {
                self.toggle_calendar_popup(CommitReason::Input);
                self.draw_panel(qh, RepaintReason::Pointer);
                if self.calendar_popup_open {
                    self.draw_calendar_popup(qh, RepaintReason::Pointer);
                }
            }
            WidgetAction::ToggleNetworkPopup => {
                self.toggle_network_popup(CommitReason::Input);
                self.draw_panel(qh, RepaintReason::Pointer);
                if self.network_popup_open {
                    self.draw_network_popup(qh, RepaintReason::Pointer);
                }
            }
            WidgetAction::ToggleWorkspacePopup => {
                self.toggle_workspace_popup(CommitReason::Input);
                self.draw_panel(qh, RepaintReason::Pointer);
                if self.workspace_popup_open {
                    self.draw_workspace_popup(qh, RepaintReason::Pointer);
                }
            }
            _ => unreachable!("non popup action routed to popup dispatcher"),
        }
    }

    fn dispatch_settings_action(&mut self, qh: &QueueHandle<NiwoeShell>, action: WidgetAction) {
        match action {
            WidgetAction::ToggleSettings => {
                if self.launcher_settings_open {
                    self.close_settings(qh, RepaintReason::Pointer);
                } else {
                    self.settings_search.clear();
                    self.open_settings_category(qh, self.settings_category);
                }
            }
            WidgetAction::SetSettingsCategory(cat) => {
                self.settings_search.clear();
                self.control_center_nav.sidebar_focus = None;
                self.control_center_nav.widget_focus = Some(cat.chip_id());
                self.ui_preview_widget_state = None;
                self.settings_category = cat;
                self.wallpaper_page = 0;
                self.provider_page = 0;
                self.display_mode_dropdown_open = None;
                self.display_pages = crate::settings_view::DisplayPages::default();
                if cat == crate::settings_view::SettingsCategory::Network {
                    self.wifi_password_prompt = None;
                    self.wifi_password_input.clear();
                }
                if cat == crate::settings_view::SettingsCategory::DefaultApps {
                    self.default_apps_picker_open = None;
                    self.default_apps_page = 0;
                }
                self.request_settings_refresh(cat);
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ApplyThemeByIndex(idx) => {
                if let Some(name) = self.available_themes.get(idx).cloned() {
                    self.apply_theme(qh, name);
                }
            }
            WidgetAction::PageWallpaper { forward } => {
                let (_, height) = self.launcher_content_size();
                let c = niwoe_tokens::Settings::DEFAULT;
                let body = height.saturating_sub(
                    (c.header_height + niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height)
                        as u32,
                );
                let slots = crate::settings_view::wallpaper_page_size(body);
                let count = crate::settings_view::wallpaper_visible_indices(
                    &self.available_wallpapers,
                    &self.settings_search,
                )
                .len();
                let last = count.div_ceil(slots).saturating_sub(1);
                let current = self.wallpaper_page.min(last);
                self.wallpaper_page = if forward {
                    current.saturating_add(1).min(last)
                } else {
                    current.saturating_sub(1)
                };
                self.control_center_nav.widget_focus = Some(if self.wallpaper_page == last {
                    "wallpaper-page-previous"
                } else {
                    "wallpaper-page-next"
                });
                self.ui_preview_widget_state = None;
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ApplyWallpaperByIndex(idx) => {
                if let Some(entry) = self.available_wallpapers.get(idx) {
                    let path = entry.apply_path.clone();
                    let mode = self.wallpaper_mode;
                    self.apply_wallpaper(qh, path, mode);
                }
            }
            WidgetAction::SetWallpaperMode(mode) => {
                self.wallpaper_mode = mode;
                if let Some(path) = self.wallpaper_path.clone() {
                    self.apply_wallpaper(qh, path, mode);
                } else {
                    self.draw_launcher(qh, RepaintReason::Pointer);
                }
            }
            WidgetAction::SetCursorSize(size) => {
                if self.cursor_size != size {
                    self.cursor_size = size;
                    self.request_settings_refresh(crate::settings_view::SettingsCategory::Cursor);
                    // Persist alongside the current theme, then ask the
                    // compositor to reload so the live cursor updates at once.
                    niwoe_config::NiwoeConfig::save_cursor(&self.cursor_theme, size);
                    self.ipc.send(&niwoe_ipc::ShellCommand::ReloadConfig);
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ApplyCursorThemeByIndex(idx) => {
                if let Some(name) = self.available_cursor_themes.get(idx).cloned() {
                    if self.cursor_theme != name {
                        self.cursor_theme = name.clone();
                        // Persist alongside the current size, then reload so the
                        // compositor swaps the live cursor theme at once.
                        niwoe_config::NiwoeConfig::save_cursor(&name, self.cursor_size);
                        self.ipc.send(&niwoe_ipc::ShellCommand::ReloadConfig);
                    }
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::SetIdleTimeout(secs) => {
                if self.idle_timeout_secs != secs {
                    self.idle_timeout_secs = secs;
                    // Persist to [general] and reload so the compositor picks up
                    // the new idle blanking timeout (or disables it) at once.
                    niwoe_config::NiwoeConfig::save_idle_timeout(secs);
                    self.ipc.send(&niwoe_ipc::ShellCommand::ReloadConfig);
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::SetDefaultSinkVolume(percent) => {
                // System state, not NIWOE config: drive wpctl directly, then
                // re-poll so the page reflects the real new level.
                crate::audio::set_default_sink_volume(percent);
                self.audio_snapshot = crate::audio::AudioSnapshot::poll();
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ToggleDefaultSinkMute => {
                crate::audio::toggle_default_sink_mute();
                self.audio_snapshot = crate::audio::AudioSnapshot::poll();
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::SetDefaultAudioOutput(idx) => {
                // Look up the wpctl id by position in the current snapshot, make
                // it the default sink, then re-poll so the page reflects it.
                if let Some(device) = self.audio_snapshot.outputs.get(idx) {
                    crate::audio::set_default_device(device.id);
                    self.audio_snapshot = crate::audio::AudioSnapshot::poll();
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::SetDefaultAudioInput(idx) => {
                if let Some(device) = self.audio_snapshot.inputs.get(idx) {
                    crate::audio::set_default_device(device.id);
                    self.audio_snapshot = crate::audio::AudioSnapshot::poll();
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ActivateConnection(idx) => {
                // Bringing a link up can block for seconds (DHCP/auth), so the
                // activation runs on a background thread inside
                // activate_connection — never on the event loop. Optimistically
                // mark the chosen profile active so the click gives feedback;
                // re-entering the Network page re-lists the real state. We only
                // flip the clicked row (not the others), so a still-active VPN
                // is never falsely hidden if activation is slow or fails.
                if let Some(profile) = self.network_profiles.get_mut(idx) {
                    let name = profile.name.clone();
                    profile.active = true;
                    crate::network::activate_connection(&name);
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::WifiConnect(idx) => {
                if let Some(net) = self.wifi_networks.get(idx) {
                    let ssid = net.ssid.clone();
                    // A secured network with no matching saved profile needs a
                    // password: open the centered password modal (the single,
                    // unified password UI). Open or already-known networks
                    // connect straight away (off-thread).
                    let known = self.network_profiles.iter().any(|p| p.name == ssid);
                    if net.secured && !known {
                        self.open_wifi_password_modal(ssid);
                        self.draw_wifi_modal(qh, RepaintReason::Pointer);
                    } else {
                        crate::network::connect_wifi(&ssid, None);
                    }
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ToggleBluetoothPower => {
                // Drive bluetoothctl off-thread, then optimistically flip the
                // cached flag so the toggle gives feedback; re-entering the page
                // re-polls the real state.
                let turn_on = !self.bluetooth_snapshot.powered;
                crate::bluetooth::set_power(turn_on);
                self.bluetooth_snapshot.powered = turn_on;
                if !turn_on {
                    self.bluetooth_snapshot.scanning = false;
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::ToggleBluetoothScan => {
                // One-shot timed discovery (off-thread). Optimistically show the
                // scanning state; re-entering the page re-polls real discovery.
                crate::bluetooth::start_scan();
                self.bluetooth_snapshot.scanning = true;
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::BluetoothDevice(idx) => {
                // Already-paired device connects; an unknown device pairs (which
                // also trusts + connects). Both block on a remote handshake, so
                // they run off-thread inside the bluetooth helpers.
                if let Some(dev) = self.bluetooth_snapshot.devices.get(idx) {
                    if dev.paired {
                        crate::bluetooth::connect_device(&dev.address);
                    } else {
                        crate::bluetooth::pair_device(&dev.address);
                    }
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::BrowseWallpaper => {
                // Close launcher so the file dialog opens in the foreground.
                self.close_launcher_after_launch(qh, RepaintReason::Pointer);
                self.spawn_file_picker();
            }
            WidgetAction::SetPrimaryOutput(idx) => {
                if let Some(output) = self.output_workspaces.get(idx).cloned() {
                    let Some(name) = output.output_name else {
                        tracing::warn!(
                            "cannot set primary output without output name: output_id={}",
                            output.output_id
                        );
                        return;
                    };
                    niwoe_config::NiwoeConfig::save_primary_output(&name);
                    for state in &mut self.output_workspaces {
                        state.primary = state.output_name.as_deref() == Some(name.as_str());
                    }
                    self.ipc.send(&niwoe_ipc::ShellCommand::ReloadConfig);
                    self.draw_launcher(qh, RepaintReason::Pointer);
                }
            }
            WidgetAction::CycleOutputScale(idx) => {
                if let Some(output) = self.output_workspaces.get(idx).cloned() {
                    let Some(name) = output.output_name else {
                        return;
                    };
                    // Advance to the next scale in the cycle (wrap around).
                    let cur = output.scale_millis as f64 / 1000.0;
                    let cycle = crate::settings_view::DISPLAY_SCALE_CYCLE;
                    let pos = cycle
                        .iter()
                        .position(|s| (s - cur).abs() < 0.001)
                        .unwrap_or(usize::MAX);
                    let next = cycle[pos.wrapping_add(1) % cycle.len()];
                    niwoe_config::NiwoeConfig::save_output_scale(&name, next);
                    if let Some(state) = self.output_workspaces.get_mut(idx) {
                        state.scale_millis = (next * 1000.0).round() as u32;
                    }
                    self.ipc.send(&niwoe_ipc::ShellCommand::ReloadConfig);
                    self.draw_launcher(qh, RepaintReason::Pointer);
                }
            }
            WidgetAction::CycleOutputTransform(idx) => {
                if let Some(output) = self.output_workspaces.get(idx).cloned() {
                    let Some(name) = output.output_name else {
                        return;
                    };
                    let next = next_output_transform(output.transform.as_deref());
                    niwoe_config::NiwoeConfig::save_output_transform(&name, next);
                    if let Some(state) = self.output_workspaces.get_mut(idx) {
                        state.transform = next.map(|s| s.to_string());
                    }
                    self.ipc.send(&niwoe_ipc::ShellCommand::ReloadConfig);
                    self.draw_launcher(qh, RepaintReason::Pointer);
                }
            }
            WidgetAction::PageProviders { forward } => self.page_providers(qh, forward),
            WidgetAction::PageDisplay { modes, forward } => self.page_display(qh, modes, forward),
            WidgetAction::ToggleOutputModeDropdown(idx) => {
                self.display_pages.modes = 0;
                self.display_mode_dropdown_open = if self.display_mode_dropdown_open == Some(idx) {
                    None
                } else {
                    Some(idx)
                };
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::SetOutputMode {
                output_index,
                mode_index,
            } => {
                self.apply_output_mode_selection(qh, output_index, mode_index);
            }
            WidgetAction::DefaultAppsAutoSet => {
                self.start_default_apps_change(
                    crate::default_apps::refresh::ChangeRequest::FillEmpty,
                );
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::DefaultAppsClosePicker => {
                tracing::info!("default_apps: close picker (back)");
                self.default_apps_picker_open = None;
                self.default_apps_page = 0;
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::DefaultAppsTogglePicker(idx) => {
                tracing::info!("default_apps: toggle picker idx={}", idx);
                let Some(cat) = crate::default_apps::DefaultAppCategory::ALL
                    .get(idx)
                    .copied()
                else {
                    return;
                };
                self.default_apps_page = 0;
                self.default_apps_picker_open = match self.default_apps_picker_open {
                    Some(open) if open == cat => None,
                    _ => Some(cat),
                };
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            WidgetAction::DefaultAppsPick { cat_idx, app_idx } => {
                tracing::info!("default_apps: pick cat_idx={} app_idx={}", cat_idx, app_idx);
                let Some(cat) = crate::default_apps::DefaultAppCategory::ALL
                    .get(cat_idx)
                    .copied()
                else {
                    return;
                };
                let Some(index) = self.default_apps_index.as_ref() else {
                    return;
                };
                let apps = index.apps_for_mime(cat.representative_mime());
                let Some(app) = apps.get(app_idx) else {
                    return;
                };
                if app_idx >= crate::default_apps::MAX_APPS_PER_CATEGORY {
                    return;
                }
                let request = crate::default_apps::refresh::ChangeRequest::Pick {
                    category: cat,
                    desktop_id: app.desktop_id.clone(),
                };
                self.start_default_apps_change(request);
                self.draw_launcher(qh, RepaintReason::Pointer);
            }
            _ => unreachable!("non settings action routed to settings dispatcher"),
        }
    }
}
