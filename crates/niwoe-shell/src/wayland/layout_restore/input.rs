use super::*;
use crate::room_management_view::restore::{self, Page};
use crate::wayland::RepaintReason;
use smithay_client_toolkit::seat::keyboard::Keysym;
use wayland_client::QueueHandle;

impl NiwoeShell {
    pub(crate) fn open_layout_panel(&mut self) {
        let Some(edit) = self
            .workspace_state
            .rooms
            .edit
            .as_mut()
            .filter(|e| e.id != 0)
        else {
            return;
        };
        edit.restore.open = true;
        edit.restore.focus = 30 + edit.form.tab;
        if let Some(LayoutNotice::Status {
            revision,
            running,
            message,
            results,
            ..
        }) = self.workspace_state.rooms.layouts.get(&edit.id)
        {
            edit.restore.ready = true;
            edit.restore.revision = *revision;
            edit.restore.running = *running;
            edit.restore.message = message.clone();
            edit.restore.results = results.clone();
        }
        self.ipc.send(&ShellCommand::Layout {
            request_id: format!("status-{}", edit.id),
            action: LayoutAction::Status { room_id: edit.id },
        });
    }

    fn layout_control(&mut self, control: usize, width: u32, height: u32) {
        let Some(edit) = self.workspace_state.rooms.edit.as_mut() else {
            return;
        };
        let room_id = edit.id;
        let page = Page::from_tab(edit.form.tab);
        let state = &mut edit.restore;
        if !restore::enabled(state, page, control, width, height) {
            return;
        }
        state.focus = control;
        if control == 7 {
            if let Some(error) = restore::file_error(state) {
                state.message = error.into();
                return;
            }
        }
        state.page = restore::page_index(state, page, width, height);
        let count = restore::page_size(page, width, height);
        let action = match control {
            0 => Some(LayoutAction::Save {
                room_id,
                expected_revision: state.revision,
            }),
            1 | 2 => Some(LayoutAction::Restore {
                room_id,
                relaunch: control == 2,
            }),
            3 => Some(LayoutAction::Cancel { room_id }),
            4 => {
                state.page = state.page.saturating_sub(1);
                None
            }
            5 => {
                state.page = (state.page + 1).min(state.results.len().saturating_sub(1) / count);
                None
            }
            7 => state.file_key.map(|key| LayoutAction::SetFile {
                expected_revision: state.revision,
                room_id,
                key,
                path: (!state.file.is_empty()).then(|| state.file.clone()),
            }),
            i if i >= 8 && page == Page::Files => {
                state.file_key = state.results.get(state.page * count + i - 8).map(|r| r.key);
                state.file = state
                    .results
                    .get(state.page * count + i - 8)
                    .and_then(|r| r.file.clone())
                    .unwrap_or_default();
                None
            }
            _ => None,
        };
        if let Some(action) = action {
            let request_id = format!(
                "layout-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            );
            if !self.ipc.send(&ShellCommand::Layout { request_id, action }) {
                state.message = "Keine Verbindung; Aktion nicht bestätigt".into();
            }
        }
    }

    pub(crate) fn layout_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) -> bool {
        if key == Keysym::F6 {
            self.room_form_tab(3);
            self.draw_launcher(qh, RepaintReason::Keyboard);
            return true;
        }
        let (width, height) = self.launcher_content_size();
        let Some(edit) = self
            .workspace_state
            .rooms
            .edit
            .as_mut()
            .filter(|e| e.restore.open)
        else {
            return false;
        };
        let page = Page::from_tab(edit.form.tab);
        let state = &mut edit.restore;
        let order = restore::focus_order(state, page, width, height);
        let editable = restore::enabled(state, page, 6, width, height);
        match key {
            Keysym::Escape => {
                state.open = false;
                edit.form.tab = 0;
                edit.focus = 30;
            }
            Keysym::Tab | Keysym::ISO_Left_Tab => {
                let current = order.iter().position(|i| *i == state.focus);
                let next = match (current, key == Keysym::Tab) {
                    (Some(i), true) => (i + 1) % order.len(),
                    (Some(i), false) => (i + order.len() - 1) % order.len(),
                    (None, true) => 0,
                    (None, false) => order.len() - 1,
                };
                state.focus = order[next];
            }
            Keysym::Return | Keysym::KP_Enter => {
                let focus = state.focus;
                if (30..34).contains(&focus) {
                    self.room_form_tab(focus - 30);
                } else {
                    self.layout_control(focus, width, height);
                }
            }
            Keysym::BackSpace if state.focus == 6 && editable => {
                state.file.pop();
            }
            _ if state.focus == 6 && editable => {
                if let Some(ch) = key.key_char().filter(|c| !c.is_control()) {
                    if state.file.len() + ch.len_utf8() <= 4096 {
                        state.file.push(ch);
                    }
                }
            }
            _ => {}
        }
        self.draw_launcher(qh, RepaintReason::Keyboard);
        true
    }

    pub(crate) fn layout_click(
        &mut self,
        qh: &QueueHandle<Self>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> bool {
        if !self
            .workspace_state
            .rooms
            .edit
            .as_ref()
            .is_some_and(|e| e.restore.open)
        {
            return false;
        }
        let c = niwoe_tokens::ControlCenter::DEFAULT;
        if x < c.sidebar_width
            || y < c.config_header_height + c.config_tabs_height
            || y >= height as i32 - c.config_footer_height
        {
            return true;
        }
        let edit = self.workspace_state.rooms.edit.as_ref().unwrap();
        if let Some(control) = restore::hit(
            &edit.restore,
            Page::from_tab(edit.form.tab),
            x,
            y,
            width,
            height,
        ) {
            self.layout_control(control, width, height);
        }
        self.draw_launcher(qh, RepaintReason::Pointer);
        true
    }
}
