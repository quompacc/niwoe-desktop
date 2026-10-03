impl NiwoeShell {
    pub(crate) fn panel_form_preview(&self, width: u32) -> Option<tiny_skia::Pixmap> {
        let height = crate::PANEL_SURFACE_HEIGHT;
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
            None,
            self.workspace_state.rooms.panel.effective(),
            &self.theme,
            &|_| niwoe_ui::WidgetState::Idle,
            &mut Vec::new(),
        );
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        tiny_skia::Pixmap::from_vec(pixels, tiny_skia::IntSize::from_wh(width, height)?)
    }
    pub(crate) fn control_center_sidebar_click(
        &mut self,
        qh: &QueueHandle<Self>,
        x: i32,
        y: i32,
        height: u32,
    ) -> bool {
        let c = niwoe_tokens::ControlCenter::DEFAULT;
        let s = niwoe_tokens::Spacing::DEFAULT;
        if x < 0 || x >= c.sidebar_width {
            return false;
        }
        if self.workspace_state.rooms.pending.is_some()
            || self.workspace_state.rooms.panel.pending.is_some()
        {
            return true;
        }
        if y >= height as i32 - c.config_footer_height {
            if self.workspace_state.rooms.panel.open {
                self.panel_form_action(qh, 13);
            } else if self.room_configuration_id.is_some() {
                self.return_to_room_management(qh);
            } else {
                self.return_to_hub(qh);
            }
            return true;
        }
        let top = c.outer_pad + s.xxl * 2;
        if y >= top && y < top + crate::room_management_view::sidebar_row_height(height) * 7 {
            let index = (y - top) / crate::room_management_view::sidebar_row_height(height);
            use crate::settings_view::SettingsCategory as Category;
            match index {
                0 => self.return_to_hub(qh),
                1 => {
                    self.workspace_state.rooms.panel.open = false;
                    self.panel_dirty = true;
                    self.panel_last_signature = None;
                    self.return_to_room_management(qh);
                }
                2 if self.room_configuration_id.is_some() => {
                    self.room_form_tab(1);
                    self.draw_launcher(qh, RepaintReason::Pointer);
                }
                2 => self.open_settings_category(qh, Category::DefaultApps),
                3 if self.room_configuration_id.is_some() => {
                    self.room_form_tab(2);
                    self.draw_launcher(qh, RepaintReason::Pointer);
                }
                4 => self.open_settings_category(qh, Category::Users),
                5 => self.open_settings_category(qh, Category::SystemOverview),
                6 => self.open_panel_form(qh),
                _ => {}
            }
        } else {
            let top = top
                + crate::room_management_view::sidebar_row_height(height) * 7
                + c.sidebar_section_gap
                + s.xxl;
            if y >= top && y < top + crate::room_management_view::sidebar_row_height(height) * 4 {
                match (y - top) / crate::room_management_view::sidebar_row_height(height) {
                    0 => self.open_settings_category(
                        qh,
                        crate::settings_view::SettingsCategory::Updates,
                    ),
                    3 => self
                        .open_settings_category(qh, crate::settings_view::SettingsCategory::Cursor),
                    _ => {}
                }
            }
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
        if state.pending.is_some() {
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
                state.focus = (state.focus + if key == Keysym::Tab { 1 } else { 13 }) % 14;
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
