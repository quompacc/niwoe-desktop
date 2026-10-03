impl NiwoeShell {
    pub(crate) fn control_center_click(
        &mut self,
        qh: &QueueHandle<Self>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) {
        use crate::room_management_view as view;
        if self.control_center_sidebar_click(qh, x, y, height) {
            return;
        }
        if self.workspace_state.rooms.panel.open {
            if let Some(index) = view::panel::hit(&self.workspace_state.rooms.panel, x, y, width, height) {
                self.panel_form_action(qh, index);
            }
        } else if self.room_configuration_id.is_some() {
            if self.room_form_click(qh, x, y, width, height)
                || self.layout_click(qh, x, y, width, height)
                || self.room_target_menu_click(qh, x, y, width, height)
            {
                return;
            }
            if let Some(action) =
                view::hit_configuration(x, y, width, height, self.room_configuration_scroll_y)
            {
                self.room_configuration_action(qh, action);
            }
        } else if let Some(action) = view::pages::hit(x, y, width, height) {
            self.room_list_action(qh, action);
        } else if let Some(action) = view::list::hit_list(x, y, width) {
            self.room_list_action(qh, action);
        } else if view::hit_new_room(x, y, width) {
            self.open_new_room(qh);
        } else if view::hit_back(x, y, height) {
            self.return_to_hub(qh);
        } else {
            let rooms = self.visible_rooms();
            if let Some(index) = view::hit_room(
                x,
                y,
                width,
                height,
                rooms.len(),
                self.room_management_page,
                self.workspace_state.rooms.list.list_view,
            ) {
                self.open_room_configuration(qh, rooms[index].id);
            }
        }
    }
    pub(crate) fn visible_rooms(&self) -> Vec<niwoe_ipc::RoomEntry> {
        self.workspace_state.rooms.list.visible(
            &self.workspace_state.rooms.snapshot.rooms,
            &self.workspace_window_counts,
        )
    }

    pub(crate) fn room_list_action(&mut self, qh: &QueueHandle<Self>, action: usize) {
        if action == 5 {
            self.open_new_room(qh);
            return;
        }
        if matches!(action, 6 | 7) {
            let count = self.visible_rooms().len();
            if !crate::room_management_view::pages::enabled(
                action,
                count,
                self.room_management_page,
            ) {
                return;
            }
            self.room_management_page = if action == 6 {
                self.room_management_page.saturating_sub(1)
            } else {
                self.room_management_page.saturating_add(1)
            };
            self.workspace_state.rooms.list.search_focus = false;
            self.workspace_state.rooms.list.focus = Some(action);
            self.room_keyboard_focus = None;
            self.hovered_bento_idx = None;
            self.draw_launcher(qh, RepaintReason::Pointer);
            return;
        }
        let list = &mut self.workspace_state.rooms.list;
        list.search_focus = action == 4;
        list.focus = Some(action);
        match action {
            0..=2 => list.filter = action,
            3 => list.alphabetical = !list.alphabetical,
            8 => list.list_view = false,
            9 => list.list_view = true,
            _ => {}
        }
        if action <= 4 {
            self.room_management_page = 0;
        }
        self.room_keyboard_focus = None;
        self.hovered_bento_idx = None;
        self.draw_launcher(qh, RepaintReason::Pointer);
    }
    pub(crate) fn room_form_tab(&mut self, tab: usize) {
        if tab >= 4 {
            return;
        }
        if let Some(edit) = &mut self.workspace_state.rooms.edit {
            edit.form.tab = tab;
            edit.restore.open = false;
            edit.target_menu = None;
            edit.focus = 30 + tab;
            self.room_configuration_scroll_y = 0;
            if tab == 1 {
                edit.catalog(&self.launcher_state.apps);
            }
            if tab >= 2 {
                if edit.id == 0 {
                    edit.form.tab = 0;
                    self.workspace_state.rooms.message =
                        "Bitte den neuen Raum zuerst speichern.".into();
                } else {
                    self.open_layout_panel();
                }
            }
        }
    }

    pub(crate) fn room_form_click(
        &mut self,
        qh: &QueueHandle<Self>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> bool {
        use crate::room_management_view::{form, ConfigurationAction};
        if let Some(tab) = form::hit_tab(x, y) {
            self.room_configuration_action(qh, ConfigurationAction::Tab(tab));
            return true;
        }
        let Some(edit) = self.workspace_state.rooms.edit.as_ref() else {
            return false;
        };
        if edit.form.tab != 1 {
            return false;
        }
        let c = niwoe_tokens::ControlCenter::DEFAULT;
        if y < c.config_header_height + c.config_tabs_height
            || y >= height as i32 - c.config_footer_height
            || x < c.sidebar_width
        {
            return false;
        }
        for index in [14, 15, 16, 20, 21, 22, 23, 24, 25] {
            let r = form::app_rect(width, index);
            if x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height {
                self.room_configuration_action(qh, ConfigurationAction::Form(index));
                return true;
            }
        }
        true
    }

    pub(crate) fn room_form_key(
        &mut self,
        qh: &QueueHandle<Self>,
        key: smithay_client_toolkit::seat::keyboard::Keysym,
    ) -> bool {
        use crate::room_management_view::ConfigurationAction;
        use smithay_client_toolkit::seat::keyboard::Keysym;
        let Some(edit) = self.workspace_state.rooms.edit.as_ref() else {
            return false;
        };
        if edit.target_menu.is_some() {
            return false;
        }
        if edit.restore.open {
            return false;
        }
        let focus = edit.focus;
        if matches!(key, Keysym::Tab | Keysym::ISO_Left_Tab) {
            let order = edit.focus_order(self.workspace_state.rooms.snapshot.rooms.len());
            let index = order.iter().position(|i| *i == focus).unwrap_or(0);
            let next = order[(index
                + if key == Keysym::Tab {
                    1
                } else {
                    order.len() - 1
                })
                % order.len()];
            let edit = self.workspace_state.rooms.edit.as_mut().unwrap();
            edit.focus = next;
            edit.replace = true;
            if edit.form.tab == 0 {
                let (w, h) = self.launcher_content_size();
                let c = niwoe_tokens::ControlCenter::DEFAULT;
                if [8, 10, 11, 12, 13].contains(&next) {
                    let r = crate::room_management_view::form::rect(w, 0, next);
                    self.room_configuration_scroll_y = (r.y + r.height
                        - (h as i32 - c.config_footer_height - c.card_gap))
                        .max(0)
                        .min(crate::room_management_view::max_configuration_scroll(h));
                } else if next == 6 || next == 7 {
                    self.room_configuration_scroll_y =
                        crate::room_management_view::max_configuration_scroll(h);
                } else {
                    self.room_configuration_scroll_y = 0;
                }
            }
        } else if matches!(key, Keysym::Return | Keysym::KP_Enter | Keysym::space)
            && focus >= 8
            && focus != 14
        {
            if focus >= 30 {
                self.room_configuration_action(qh, ConfigurationAction::Tab(focus - 30));
            } else {
                self.room_configuration_action(qh, ConfigurationAction::Form(focus));
            }
        } else if focus == 14 {
            let edit = self.workspace_state.rooms.edit.as_mut().unwrap();
            if key == Keysym::BackSpace {
                edit.form.query.pop();
            } else if let Some(ch) = key.key_char().filter(|ch| !ch.is_control()) {
                if edit.form.query.chars().count() < 128 {
                    edit.form.query.push(ch);
                }
            } else {
                return false;
            }
            edit.form.page = 0;
        } else {
            return false;
        }
        self.draw_launcher(qh, RepaintReason::Keyboard);
        true
    }
}
