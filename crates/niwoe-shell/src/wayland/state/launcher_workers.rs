impl NiwoeShell {
    /// Warm the launcher grid's app icons (deferred from startup so the panel
    /// appears immediately). Runs once per cache build; the first launcher open
    /// pays the decode cost instead of every login.
    /// Kick an OFF-THREAD warm of the launcher grid icons (LAUNCH-3). Decoding
    /// every app icon at two sizes on the event-loop thread froze the launcher
    /// for ~0.5s on each open (and again after every background app refresh),
    /// which showed up as input lag / bursty scrolling. The worker decodes with
    /// a throwaway loader and posts ready buffers to `launcher_icons_rx`, which
    /// `tick()` drains via `poll_launcher_icons_warm`. No-op if already warmed
    /// or a warm is already in flight.
    fn warm_launcher_icons(&mut self) {
        if self.launcher_icons_warmed || self.launcher_icons_rx.is_some() {
            return;
        }
        let mut names: Vec<String> = self
            .launcher_state
            .apps
            .iter()
            .filter_map(|app| app.icon_name.clone())
            .filter(|name| !name.is_empty())
            .collect();
        names.extend(
            self.workspace_state
                .rooms
                .snapshot
                .rooms
                .iter()
                .filter_map(|r| r.preferences.icon.clone()),
        );
        names.extend(
            crate::room_editor::form::ICONS
                .iter()
                .filter(|(id, _)| !id.is_empty())
                .map(|(id, _)| id.to_string()),
        );
        names.extend(self.windows.iter().filter_map(|w| w.app_id.clone()));
        names.sort();
        names.dedup();
        if names.is_empty() {
            self.launcher_icons_warmed = true;
            return;
        }
        let (theme_name, symbolic_color) = self.icon_cache.loader_config();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let batch = crate::icons::IconCache::load_batch(
                &theme_name,
                &symbolic_color,
                &names,
                &[22, 24, 32],
            );
            let _ = tx.send(batch);
        });
        self.launcher_icons_rx = Some(rx);
        // Mark warmed now so re-opens before results arrive don't re-spawn; the
        // results are applied when the worker finishes.
        self.launcher_icons_warmed = true;
    }

    /// Apply a finished off-thread icon warm, if one has arrived. Called from
    /// `tick()`; cheap `try_recv`, never blocks. Redraws the launcher so the
    /// freshly-decoded icons appear the moment they land.
    fn poll_launcher_icons_warm(&mut self, qh: &QueueHandle<Self>) {
        let Some(rx) = self.launcher_icons_rx.as_ref() else {
            return;
        };
        match rx.try_recv() {
            Ok(batch) => {
                self.launcher_icons_rx = None;
                for (name, size, image) in batch {
                    self.icon_cache.insert_loaded(name, size, image);
                }
                if self.launcher_state.open {
                    self.draw_launcher(qh, RepaintReason::Ipc);
                }
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.launcher_icons_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }
    }

    /// Kick a background rescan of the desktop-entry app list (LAUNCH-2).
    /// Scanning every applications dir is hundreds of fs reads + TryExec stats;
    /// doing it on the event-loop thread froze the UI on every launcher open.
    /// The worker thread posts the fresh list to `launcher_apps_rx`, which
    /// `tick()` swaps in. No-op if a rescan is already in flight.
    pub(crate) fn request_launcher_apps_refresh(&mut self) {
        if self.launcher_apps_rx.is_some() {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(crate::launcher::DesktopApp::load_system());
        });
        self.launcher_apps_rx = Some(rx);
    }

    pub(crate) fn refresh_quick_settings_network(&mut self) {
        self.network_controller.poll();
        self.network_profiles = crate::network::list_saved_connections();
        self.wifi_networks = crate::network::scan_wifi_networks();
        self.network_list_status = crate::network::ListStatus::default();
    }

    /// Apply a finished background app-list rescan, if one has arrived. Called
    /// from `tick()`; cheap `try_recv`, never blocks.
    fn poll_launcher_apps_refresh(&mut self, qh: &QueueHandle<Self>) {
        let Some(rx) = self.launcher_apps_rx.as_ref() else {
            return;
        };
        match rx.try_recv() {
            Ok(apps) => {
                self.launcher_apps_rx = None;
                self.launcher_state.apps = apps;
                // New app list → the old warm is stale. Cancel any in-flight
                // warm and re-request so the fresh apps get their icons.
                self.launcher_icons_warmed = false;
                self.launcher_icons_rx = None;
                if self.launcher_state.open {
                    self.warm_launcher_icons();
                    self.draw_launcher(qh, RepaintReason::Ipc);
                }
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.launcher_apps_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }
    }
}
