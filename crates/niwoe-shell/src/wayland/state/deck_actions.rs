impl NiwoeShell {
    pub(crate) fn launch_default_app(
        &mut self,
        category: Option<crate::default_apps::DefaultAppCategory>,
    ) {
        let app = match category {
            None => crate::launcher::terminal_program().map(|program| {
                crate::launcher::DesktopApp::new("Terminal".into(), vec![program], false)
            }),
            Some(category) => {
                let preferred = crate::default_apps::query_default(category.representative_mime());
                let apps = &self.launcher_state.apps;
                preferred
                    .as_ref()
                    .and_then(|id| apps.iter().find(|app| &app.desktop_id == id))
                    .or_else(|| {
                        category
                            .preferred_desktop_ids()
                            .iter()
                            .find_map(|id| apps.iter().find(|app| app.desktop_id == *id))
                    })
                    .cloned()
            }
        };
        if let Some(app) = app {
            crate::launcher::LauncherState::launch_desktop_app(app, &mut self.ipc);
        } else {
            tracing::warn!(?category, "no installed default application found");
        }
    }

    pub(crate) fn dispatch_deck_action(
        &mut self,
        qh: &QueueHandle<Self>,
        hit: crate::quick_settings_popup::QuickSettingsHit,
    ) {
        use crate::quick_settings_popup::QuickSettingsHit;
        match hit {
            QuickSettingsHit::Settings => self
                .open_settings_category(qh, crate::settings_view::SettingsCategory::SystemOverview),
            QuickSettingsHit::Appearance => {
                self.open_settings_category(qh, crate::settings_view::SettingsCategory::Theme)
            }
            QuickSettingsHit::Network => {
                self.switch_network_tab(qh, crate::network_popup::NetworkTab::Wifi);
            }
            QuickSettingsHit::AudioMute => {
                crate::audio::toggle_default_sink_mute();
                self.audio_snapshot = crate::audio::AudioSnapshot::poll();
                self.draw_network_popup(qh, RepaintReason::Pointer);
                self.draw_panel(qh, RepaintReason::Pointer);
            }
            QuickSettingsHit::Volume(volume) => {
                crate::audio::set_default_sink_volume(volume);
                self.audio_snapshot = crate::audio::AudioSnapshot::poll();
                self.draw_network_popup(qh, RepaintReason::Pointer);
                self.draw_panel(qh, RepaintReason::Pointer);
            }
            QuickSettingsHit::PowerProfile => {
                use crate::power_profile::{self, PowerProfile};
                let current = power_profile::current().unwrap_or(PowerProfile::Standard);
                let idx = PowerProfile::ALL
                    .iter()
                    .position(|profile| *profile == current)
                    .unwrap_or(0);
                let next = PowerProfile::ALL[(idx + 1) % PowerProfile::ALL.len()];
                if power_profile::set(next) {
                    self.power_profile = Some(next);
                }
                self.draw_network_popup(qh, RepaintReason::Pointer);
                self.draw_panel(qh, RepaintReason::Pointer);
            }
            QuickSettingsHit::Lock => {
                self.close_network_popup(crate::wayland::CommitReason::Input);
                if !self.ipc.send(&niwoe_ipc::ShellCommand::LockSession) {
                    tracing::warn!("quick settings: compositor lock IPC unavailable");
                }
            }
            QuickSettingsHit::PowerOff => {
                self.dispatch_widget_action(qh, crate::widget_action::WidgetAction::PowerOff);
                if self.network_popup_open {
                    self.draw_network_popup(qh, RepaintReason::Pointer);
                }
            }
            QuickSettingsHit::Card => {}
        }
    }
}
