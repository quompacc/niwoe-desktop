impl NiwoeShell {
    pub(crate) fn panel_screenshot_icon(
        icons: &crate::icons::IconCache,
    ) -> Option<tiny_skia::Pixmap> {
        icons
            .lookup(
                "camera-photo-symbolic",
                niwoe_tokens::Panel::DEFAULT.status_icon_size as u32,
            )
            .and_then(crate::icons::icon_image_to_pixmap)
    }
    pub(crate) fn panel_form_preview(
        &mut self,
        width: u32,
    ) -> Option<std::sync::Arc<tiny_skia::Pixmap>> {
        use crate::wayland::panel_preview::{PanelPreviewCache, PreviewKey};
        let width = PanelPreviewCache::fitted_width(width);
        let height = crate::PANEL_SURFACE_HEIGHT;
        let key = PreviewKey {
            width,
            active_workspace: self.panel_active_workspace(),
            rooms: self
                .workspace_state
                .rooms
                .snapshot
                .rooms
                .iter()
                .map(|room| (room.id, room.workspace, room.name.clone()))
                .collect(),
            occupied: self.occupied_workspaces,
            clock: self.last_clock.clone(),
            network_icon: self.network_controller.state().icon_name(),
            audio_icon: self.audio_snapshot.icon_name(),
            audio_label: self.audio_snapshot.panel_label(),
            notifier_items: self
                .status_notifier_items
                .iter()
                .map(|item| {
                    (
                        item.service.clone(),
                        item.title.clone(),
                        item.icon_name.clone(),
                        item.menu_path.clone(),
                    )
                })
                .collect(),
            battery: self.battery_snapshot.clone(),
            modules: self.workspace_state.rooms.panel.effective().to_vec(),
            theme: self.theme_render_signature(),
            icon_loader: self.icon_cache.loader_config(),
            icon_generation: self.icon_cache.generation(),
        };
        if let Some(image) = self.panel_preview.get(&key) {
            return Some(image);
        }
        let size = tiny_skia::IntSize::from_wh(width, height)?;
        let mut pixels = vec![0; width as usize * height as usize * 4];
        crate::panel_view::draw_panel_ui(
            &mut pixels,
            width,
            height,
            &self.pinned_apps,
            &self.panel_window_entries(self.panel_active_workspace()),
            self.network_controller.state(),
            &self.audio_snapshot,
            &self.status_notifier_items,
            false,
            false,
            &self.battery_snapshot,
            self.power_profile,
            self.panel_active_workspace(),
            self.workspace_state.rooms.snapshot.rooms.len() as u8,
            &self.workspace_state.rooms.snapshot.rooms,
            &self.occupied_workspaces,
            &self.last_clock,
            &self.icon_cache,
            Self::panel_screenshot_icon(&self.icon_cache),
            self.workspace_state.rooms.panel.effective(),
            &self.theme,
            &|_| niwoe_ui::WidgetState::Idle,
            &mut Vec::new(),
        );
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        let image = tiny_skia::Pixmap::from_vec(pixels, size)?;
        Some(self.panel_preview.insert(key, image))
    }
    pub(crate) fn control_center_sidebar_click(
        &mut self,
        qh: &QueueHandle<Self>,
        x: i32,
        y: i32,
        height: u32,
    ) -> bool {
        let c = niwoe_tokens::ControlCenter::DEFAULT;
        if x < 0 || x >= c.sidebar_width {
            return false;
        }
        if self.workspace_state.rooms.pending.is_some()
            || self.workspace_state.rooms.panel.pending.is_some()
        {
            return true;
        }
        let back = crate::control_center::back_rect(height);
        if x >= back.x && x < back.x + back.width && y >= back.y && y < back.y + back.height {
            if self.workspace_state.rooms.panel.open {
                self.panel_form_action(qh, 13);
            } else if self.room_configuration_id.is_some() {
                self.return_to_room_management(qh);
            } else {
                self.return_to_hub(qh);
            }
        } else if let Some(page) = crate::control_center::hit(height, x, y) {
            self.navigate_control_center_page(qh, page);
        }
        true
    }
    pub(crate) fn open_panel_form(&mut self, qh: &QueueHandle<Self>) {
        self.room_configuration_id = None;
        self.workspace_state.rooms.edit = None;
        let state = &mut self.workspace_state.rooms.panel;
        state.open = true;
        state.draft = state.saved.clone();
        state.draft_revision = state.revision;
        state.focus = 0;
        state.message.clear();
        self.ipc.send(&niwoe_ipc::ShellCommand::PanelPreferences {
            request_id: "panel-read".into(),
            action: niwoe_ipc::PanelAction::Get,
        });
        self.draw_launcher(qh, RepaintReason::Pointer);
    }
    pub(crate) fn panel_form_action(&mut self, qh: &QueueHandle<Self>, index: usize) {
        let state = &mut self.workspace_state.rooms.panel;
        if !crate::room_management_view::panel::enabled(state, index) {
            return;
        }
        state.focus = index;
        match index {
            0..=11 => crate::room_editor::panel::change_module(&mut state.draft, index),
            12 => {
                let id = format!(
                    "panel-{}-{}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos()
                );
                if self.ipc.send(&niwoe_ipc::ShellCommand::PanelPreferences {
                    request_id: id.clone(),
                    action: niwoe_ipc::PanelAction::Save {
                        expected_revision: state.draft_revision,
                        modules: state.draft.clone(),
                    },
                }) {
                    state.pending = Some((id, std::time::Instant::now()));
                    state.message = "Wird gespeichert …".into();
                } else {
                    state.message = "Keine Verbindung; nicht gespeichert.".into();
                }
            }
            13 => {
                state.open = false;
                state.draft = state.saved.clone();
                self.panel_preview.clear();
            }
            _ => {}
        }
        self.panel_last_signature = None;
        self.panel_dirty = true;
        self.draw_launcher(qh, RepaintReason::Pointer);
    }
    pub(crate) fn panel_form_key(
        &mut self,
        qh: &QueueHandle<Self>,
        key: smithay_client_toolkit::seat::keyboard::Keysym,
    ) -> bool {
        use smithay_client_toolkit::seat::keyboard::Keysym;
        if key == Keysym::F7 {
            self.open_panel_form(qh);
            return true;
        }
        if !self.workspace_state.rooms.panel.open {
            return false;
        }
        let state = &mut self.workspace_state.rooms.panel;
        match key {
            Keysym::Escape => self.panel_form_action(qh, 13),
            Keysym::Tab | Keysym::ISO_Left_Tab => {
                let order = crate::room_management_view::panel::focus_order(state);
                if let Some(position) = order.iter().position(|index| *index == state.focus) {
                    let next = if key == Keysym::Tab {
                        position + 1
                    } else {
                        position + order.len() - 1
                    };
                    state.focus = order[next % order.len()];
                } else if let Some(first) = order.first() {
                    state.focus = *first;
                }
                self.draw_launcher(qh, RepaintReason::Keyboard);
            }
            Keysym::Return | Keysym::KP_Enter | Keysym::space => {
                let focus = state.focus;
                self.panel_form_action(qh, focus);
            }
            _ => {}
        }
        true
    }
}
