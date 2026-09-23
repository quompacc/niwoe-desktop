impl NiwoeShell {
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
