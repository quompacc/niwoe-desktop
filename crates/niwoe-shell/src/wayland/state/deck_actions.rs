impl NiwoeShell {
    pub(crate) fn start_deck_audio(&mut self, change: crate::deck_mutation::AudioChange) {
        if let Some(device) = self.audio_snapshot.default_output.as_ref() {
            self.deck_mutation
                .audio(crate::deck_mutation::AudioRequest {
                    device: device.id,
                    change,
                });
            self.network_dirty = true;
        }
    }

    fn poll_deck_mutations(&mut self, qh: &QueueHandle<Self>) {
        let audio = self.deck_mutation.poll_audio();
        let power = self.deck_mutation.power.poll();
        let changed = audio.is_some() || power.is_some();
        if let Some(Some(observed)) = audio {
            self.audio_snapshot = observed;
            self.draw_audio_popup(qh, RepaintReason::Clock);
            if self.volume_osd_open {
                self.draw_volume_osd(qh, RepaintReason::Clock);
            }
        }
        if let Some(Some(observed)) = power {
            self.power_profile = observed;
        }
        if changed {
            self.draw_network_popup(qh, RepaintReason::Clock);
            self.draw_panel(qh, RepaintReason::Clock);
        }
    }

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
            QuickSettingsHit::Bluetooth => {
                self.open_settings_category(qh, crate::settings_view::SettingsCategory::Bluetooth)
            }
            QuickSettingsHit::Display => {
                self.open_settings_category(qh, crate::settings_view::SettingsCategory::Display)
            }
            QuickSettingsHit::Settings => self
                .open_settings_category(qh, crate::settings_view::SettingsCategory::SystemOverview),
            QuickSettingsHit::Appearance => {
                self.open_settings_category(qh, crate::settings_view::SettingsCategory::Theme)
            }
            QuickSettingsHit::Network => {
                self.switch_network_tab(qh, crate::network_popup::NetworkTab::Wifi);
            }
            QuickSettingsHit::AudioMute => {
                self.start_deck_audio(crate::deck_mutation::AudioChange::Mute(
                    !self
                        .audio_snapshot
                        .default_output
                        .as_ref()
                        .is_some_and(|d| d.muted),
                ));
                self.draw_network_popup(qh, RepaintReason::Pointer);
            }
            QuickSettingsHit::Volume(volume) => {
                self.start_deck_audio(crate::deck_mutation::AudioChange::Volume(volume.min(100)));
                self.draw_network_popup(qh, RepaintReason::Pointer);
            }
            QuickSettingsHit::PowerProfile => {
                use crate::power_profile::PowerProfile;
                if let Some(current) = self.power_profile {
                    let idx = PowerProfile::ALL
                        .iter()
                        .position(|p| *p == current)
                        .unwrap_or(0);
                    self.deck_mutation
                        .power(PowerProfile::ALL[(idx + 1) % PowerProfile::ALL.len()]);
                    self.draw_network_popup(qh, RepaintReason::Pointer);
                }
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
