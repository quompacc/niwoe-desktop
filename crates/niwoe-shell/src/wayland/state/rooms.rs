impl NiwoeShell {
    pub(crate) fn open_new_room(&mut self, qh: &QueueHandle<Self>) {
        if self.workspace_state.rooms.begin_create() {
            self.room_configuration_id = Some(0);
            self.room_configuration_scroll_y = 0;
            self.room_configuration_save_pending = false;
            self.draw_launcher(qh, RepaintReason::Pointer);
        }
    }
    pub(crate) fn open_room_configuration(&mut self, qh: &QueueHandle<Self>, id: u64) {
        let Some(workspace) = self
            .workspace_state
            .rooms
            .snapshot
            .rooms
            .iter()
            .find(|room| room.id == id)
            .map(|room| room.workspace)
        else {
            return;
        };
        self.workspace_state.rooms.begin(workspace);
        if !self
            .workspace_state
            .rooms
            .edit
            .as_ref()
            .is_some_and(|edit| edit.id == id)
        {
            return;
        }
        self.room_configuration_id = Some(id);
        self.room_configuration_scroll_y = 0;
        self.room_configuration_save_pending = false;
        self.hovered_bento_idx = None;
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    pub(crate) fn return_to_room_management(&mut self, qh: &QueueHandle<Self>) {
        self.room_configuration_id = None;
        self.room_configuration_scroll_y = 0;
        self.room_configuration_save_pending = false;
        self.workspace_state.rooms.edit = None;
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    pub(crate) fn room_configuration_action(
        &mut self,
        qh: &QueueHandle<Self>,
        action: crate::room_management_view::ConfigurationAction,
    ) {
        use crate::room_management_view::ConfigurationAction;
        use crate::wayland::RoomEditAction;
        if self.workspace_state.rooms.pending.is_some() {
            return;
        }
        match action {
            ConfigurationAction::Back | ConfigurationAction::Cancel => {
                self.return_to_room_management(qh);
                return;
            }
            ConfigurationAction::Name | ConfigurationAction::Description => {
                if let Some(edit) = &mut self.workspace_state.rooms.edit {
                    edit.focus = if action == ConfigurationAction::Name { 0 } else { 5 };
                    edit.replace = true;
                    edit.confirm_delete = false;
                }
            }
            ConfigurationAction::Target | ConfigurationAction::Delete => {
                let edit_action = if action == ConfigurationAction::Target { RoomEditAction::Target } else { RoomEditAction::Delete };
                if !self.workspace_state.rooms.enabled(edit_action) { return; }
                if let Some(edit) = &mut self.workspace_state.rooms.edit {
                    edit.focus = if action == ConfigurationAction::Target { 6 } else { 7 };
                }
                if let Some(command) = self.workspace_state.rooms.request(edit_action) {
                    self.room_configuration_save_pending = true;
                    if !self.ipc.send(&command) {
                        self.workspace_state.rooms.pending = None;
                        self.room_configuration_save_pending = false;
                        self.workspace_state.rooms.message = "Keine Verbindung. Erneut versuchen.".into();
                    }
                }
            }
            ConfigurationAction::MoveEarlier
            | ConfigurationAction::MoveLater
            | ConfigurationAction::Save => {
                let (edit_action, focus) = match action {
                    ConfigurationAction::MoveEarlier => (RoomEditAction::Left, 1),
                    ConfigurationAction::MoveLater => (RoomEditAction::Right, 2),
                    _ => (RoomEditAction::Save, 3),
                };
                if let Some(edit) = &mut self.workspace_state.rooms.edit {
                    edit.focus = focus;
                }
                if !self.workspace_state.rooms.enabled(edit_action) {
                    self.draw_launcher(qh, RepaintReason::Pointer);
                    return;
                }
                if action == ConfigurationAction::Save {
                    let unchanged = self
                        .workspace_state
                        .rooms
                        .edit
                        .as_ref()
                        .is_some_and(|edit| {
                            self.workspace_state
                                .rooms
                                .snapshot
                                .rooms
                                .iter()
                                .find(|room| room.id == edit.id)
                                .is_some_and(|room| room.name == edit.name.trim() && room.description == edit.description)
                        });
                    if unchanged {
                        self.return_to_room_management(qh);
                        return;
                    }
                }
                if let Some(command) = self.workspace_state.rooms.request(edit_action) {
                    self.room_configuration_save_pending = action == ConfigurationAction::Save;
                    if !self.ipc.send(&command) {
                        self.workspace_state.rooms.pending = None;
                        self.room_configuration_save_pending = false;
                        self.workspace_state.rooms.message =
                            "Keine Verbindung. Erneut versuchen.".into();
                    }
                }
            }
        }
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    pub(crate) fn room_configuration_key(
        &mut self,
        qh: &QueueHandle<Self>,
        key: smithay_client_toolkit::seat::keyboard::Keysym,
    ) {
        use crate::room_management_view::ConfigurationAction;
        use smithay_client_toolkit::seat::keyboard::Keysym;
        if self.workspace_state.rooms.pending.is_some() {
            return;
        }
        if key == Keysym::Escape {
            self.return_to_room_management(qh);
            return;
        }
        if key == Keysym::Tab || key == Keysym::ISO_Left_Tab {
            use crate::wayland::RoomEditAction;
            let actions = [
                RoomEditAction::Name,
                RoomEditAction::Left,
                RoomEditAction::Right,
                RoomEditAction::Save,
                RoomEditAction::Cancel,
                RoomEditAction::Name,
                RoomEditAction::Target,
                RoomEditAction::Delete,
            ];
            if let Some(edit) = self.workspace_state.rooms.edit.as_ref() {
                let order = [0, 5, 1, 2, 6, 7, 3, 4];
                let step = if key == Keysym::Tab { 1 } else { order.len() - 1 };
                let mut position = order.iter().position(|i| *i == edit.focus).unwrap_or(0);
                let mut focus = edit.focus;
                for _ in 0..actions.len() {
                    position = (position + step) % order.len();
                    focus = order[position];
                    if self.workspace_state.rooms.enabled(actions[focus]) {
                        break;
                    }
                }
                self.workspace_state.rooms.edit.as_mut().unwrap().focus = focus;
                self.workspace_state.rooms.edit.as_mut().unwrap().replace = true;
                if focus == 6 || focus == 7 {
                    let (_, height) = self.launcher_content_size();
                    self.room_configuration_scroll_y = crate::room_management_view::max_configuration_scroll(height);
                } else if focus == 0 || focus == 5 {
                    self.room_configuration_scroll_y = 0;
                }
            }
        } else if key == Keysym::Return || key == Keysym::KP_Enter {
            let action = match self
                .workspace_state
                .rooms
                .edit
                .as_ref()
                .map(|edit| edit.focus)
            {
                Some(1) => ConfigurationAction::MoveEarlier,
                Some(2) => ConfigurationAction::MoveLater,
                Some(4) => ConfigurationAction::Cancel,
                Some(6) => ConfigurationAction::Target,
                Some(7) => ConfigurationAction::Delete,
                _ => ConfigurationAction::Save,
            };
            self.room_configuration_action(qh, action);
            return;
        } else if let Some(edit) = &mut self.workspace_state.rooms.edit {
            if edit.focus == 0 || edit.focus == 5 {
                let limit = if edit.focus == 0 { niwoe_config::rooms::MAX_NAME_CHARS } else { niwoe_config::rooms::MAX_DESCRIPTION_CHARS };
                let text = if edit.focus == 0 { &mut edit.name } else { &mut edit.description };
                if key == Keysym::BackSpace {
                    if edit.replace {
                        text.clear();
                    } else {
                        text.pop();
                    }
                    edit.replace = false;
                } else if let Some(ch) = key.key_char().filter(|ch| !ch.is_control()) {
                    if edit.replace {
                        text.clear();
                        edit.replace = false;
                    }
                    if text.chars().count() < limit {
                        text.push(ch);
                    }
                }
            }
        }
        self.draw_launcher(qh, RepaintReason::Keyboard);
    }

    pub(crate) fn open_room_editor(&mut self, qh: &QueueHandle<Self>, workspace: u8) {
        if !self.workspace_state.rooms.ready {
            return;
        }
        if !self.workspace_popup_open {
            self.toggle_workspace_popup(CommitReason::Input);
        }
        self.workspace_state.rooms.begin(workspace);
        self.draw_workspace_popup(qh, RepaintReason::Pointer);
    }

    pub(crate) fn room_edit_action(
        &mut self,
        qh: &QueueHandle<Self>,
        action: crate::wayland::RoomEditAction,
    ) {
        if let Some(command) = self.workspace_state.rooms.request(action) {
            if !self.ipc.send(&command) {
                self.workspace_state.rooms.pending = None;
                self.workspace_state.rooms.message = "Keine Verbindung. Erneut versuchen.".into();
            }
        }
        self.draw_workspace_popup(qh, RepaintReason::Pointer);
    }

    pub(crate) fn room_edit_key(
        &mut self,
        qh: &QueueHandle<Self>,
        key: smithay_client_toolkit::seat::keyboard::Keysym,
    ) {
        use crate::wayland::RoomEditAction;
        use smithay_client_toolkit::seat::keyboard::Keysym;
        if key == Keysym::Escape {
            self.workspace_state.rooms.edit = None;
            self.draw_workspace_popup(qh, RepaintReason::Keyboard);
            return;
        }
        if self.workspace_state.rooms.pending.is_some() {
            return;
        }
        if key == Keysym::Tab || key == Keysym::ISO_Left_Tab {
            let actions = [
                RoomEditAction::Name,
                RoomEditAction::Save,
                RoomEditAction::Cancel,
                RoomEditAction::Left,
                RoomEditAction::Right,
            ];
            let Some(edit) = self.workspace_state.rooms.edit.as_ref() else {
                return;
            };
            let mut focus = edit.focus;
            for _ in 0..actions.len() {
                focus = (focus
                    + if key == Keysym::Tab {
                        1
                    } else {
                        actions.len() - 1
                    })
                    % actions.len();
                if self.workspace_state.rooms.enabled(actions[focus]) {
                    break;
                }
            }
            self.workspace_state.rooms.edit.as_mut().unwrap().focus = focus;
            self.draw_workspace_popup(qh, RepaintReason::Keyboard);
            return;
        }
        let Some(edit) = self.workspace_state.rooms.edit.as_mut() else {
            return;
        };
        match key {
            Keysym::Return | Keysym::KP_Enter => {
                let action = match edit.focus {
                    2 => RoomEditAction::Cancel,
                    3 => RoomEditAction::Left,
                    4 => RoomEditAction::Right,
                    _ => RoomEditAction::Save,
                };
                self.room_edit_action(qh, action);
                return;
            }
            Keysym::BackSpace if edit.focus == 0 => {
                if edit.replace {
                    edit.name.clear();
                } else {
                    edit.name.pop();
                }
                edit.replace = false;
            }
            _ if edit.focus == 0 => {
                if let Some(ch) = key.key_char().filter(|ch| !ch.is_control()) {
                    if edit.replace {
                        edit.name.clear();
                        edit.replace = false;
                    }
                    if edit.name.chars().count() < niwoe_config::rooms::MAX_NAME_CHARS {
                        edit.name.push(ch);
                    }
                }
            }
            _ => {}
        }
        self.draw_workspace_popup(qh, RepaintReason::Keyboard);
    }
}
