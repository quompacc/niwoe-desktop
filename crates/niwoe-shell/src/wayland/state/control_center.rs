impl NiwoeShell {
    pub(crate) fn effective_settings_category(&self) -> crate::settings_view::SettingsCategory {
        crate::settings_view::effective_settings_category(
            self.settings_category,
            &self.settings_search,
            &self.available_themes,
            &self.available_wallpapers,
        )
    }

    pub(crate) fn edit_settings_search(&mut self, edit: impl FnOnce(&mut String)) {
        let previous = self.effective_settings_category();
        edit(&mut self.settings_search);
        self.wallpaper_page = 0;
        self.provider_page = 0;
        self.control_center_nav.widget_focus = None;
        let current = self.effective_settings_category();
        if current != previous {
            self.default_apps_picker_open = None;
            self.default_apps_page = 0;
            self.display_mode_dropdown_open = None;
            self.display_pages = Default::default();
            self.ui_preview_widget_state = None;
            // Only category transitions request I/O; typing within the same
            // page never starts repeated queries after its worker completes.
            self.request_settings_refresh(current);
        }
    }

    pub(crate) fn control_center_visible(&self) -> bool {
        self.room_management_open || self.launcher_settings_open
    }

    pub(crate) fn control_center_page(&self) -> crate::control_center::Page {
        use crate::control_center::Page;
        if self.launcher_settings_open {
            Page::for_settings(self.effective_settings_category())
        } else if self.workspace_state.rooms.panel.open {
            Page::Panel
        } else {
            Page::Rooms
        }
    }

    pub(crate) fn navigate_control_center_page(
        &mut self,
        qh: &QueueHandle<Self>,
        page: crate::control_center::Page,
    ) {
        use crate::{control_center::Page, settings_view::SettingsCategory as Category};
        if self.workspace_state.rooms.pending.is_some()
            || self.workspace_state.rooms.panel.pending.is_some()
        {
            return;
        }
        self.control_center_nav = Default::default();
        self.ui_preview_widget_state = None;
        self.settings_search.clear();
        match page {
            Page::Rooms => {
                self.workspace_state.rooms.panel.open = false;
                self.panel_last_signature = None;
                self.panel_dirty = true;
                self.workspace_state.rooms.edit = None;
                self.open_room_management(qh);
            }
            Page::Apps => self.open_settings_category(qh, Category::DefaultApps),
            Page::Users => self.open_settings_category(qh, Category::Users),
            Page::System => self.open_settings_category(qh, Category::SystemOverview),
            Page::Panel => {
                if !self.room_management_open {
                    self.open_room_management(qh);
                }
                if !self.workspace_state.rooms.panel.open {
                    self.open_panel_form(qh);
                }
            }
            Page::Updates => self.open_settings_category(qh, Category::Updates),
            Page::Settings => self.open_settings_category(qh, Category::Wallpaper),
        }
    }

    pub(crate) fn settings_widget_tree(
        &self,
        width: u32,
        height: u32,
    ) -> Box<dyn niwoe_ui::Widget> {
        crate::settings_view::build_settings_widget_tree(
            width,
            height,
            self.settings_category,
            &self.settings_search,
            self.settings_return_to_room_management,
            &self.available_themes,
            &self.theme_name,
            &self.available_wallpapers,
            &self.wallpaper_thumbnails,
            self.wallpaper_page,
            self.wallpaper_path.as_deref(),
            self.wallpaper_mode,
            self.cursor_size,
            &self.cursor_previews,
            &self.available_cursor_themes,
            &self.cursor_theme,
            self.idle_timeout_secs,
            &self.pinned_apps,
            &self.output_workspaces,
            self.display_mode_dropdown_open,
            self.display_pages,
            self.provider_page,
            &self.printer_snapshot,
            &self.audio_snapshot,
            &self.system_info,
            &self.user_accounts,
            self.network_controller.state(),
            &self.network_profiles,
            self.network_list_status,
            &self.bluetooth_snapshot,
            &self.wifi_networks,
            self.settings_pinned_adding,
            &self.launcher_state.apps,
            &self.icon_cache,
            self.armed_power
                .as_ref()
                .map(|(id, at)| (id.as_str(), at.elapsed().as_secs_f32())),
            self.default_apps_index.as_ref(),
            &self.default_apps_current,
            self.default_apps_picker_open,
            self.default_apps_page,
            &self.default_apps_status,
            &crate::ui::tokens::theme_from_config(&self.theme),
        )
    }
}
