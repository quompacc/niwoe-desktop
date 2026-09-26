use super::{NiwoeShell, RepaintReason};
use crate::window_picker::{Hit, Picker};
use niwoe_ipc::ShellCommand;
use niwoe_tokens::window_picker::WindowPicker as L;
use smithay_client_toolkit::seat::{keyboard::Keysym, pointer::PointerEventKind};
use wayland_client::QueueHandle;

impl NiwoeShell {
    fn picker_action(&mut self, qh: &QueueHandle<Self>, hit: Hit) {
        let Some(mut picker) = self.window_picker.take() else {
            return;
        };
        let count = if picker.target_window.is_some() {
            self.workspace_state.rooms.snapshot.rooms.len()
        } else {
            self.windows.len()
        };
        picker.selected = picker.selected.min(count.saturating_sub(1));
        match hit {
            Hit::Back => {
                if picker.target_window.take().is_some() {
                    picker.selected = 0;
                    self.window_picker = Some(picker);
                }
                self.draw_launcher(qh, RepaintReason::Pointer);
                return;
            }
            Hit::Previous => picker.selected = picker.selected.saturating_sub(L::ROWS),
            Hit::Next => picker.selected = (picker.selected + L::ROWS).min(count.saturating_sub(1)),
            Hit::Move(index) => {
                if let Some(window) = self.windows.get(index) {
                    picker.target_window = Some(window.id.clone());
                    picker.selected = 0;
                }
            }
            Hit::Row(index) => {
                if let Some(id) = &picker.target_window {
                    if self.windows.iter().any(|w| &w.id == id) {
                        if let Some(room) = self.workspace_state.rooms.snapshot.rooms.get(index) {
                            if self.ipc.send(&ShellCommand::MoveWindowToRoom {
                                id: id.clone(),
                                room_id: room.id,
                            }) {
                                picker.target_window = None;
                                picker.selected = 0;
                                picker.message = "Verschieben angefordert".into();
                            } else {
                                picker.message = "Verbindung unterbrochen".into();
                            }
                        }
                    } else {
                        picker.target_window = None;
                        picker.message = "Fenster wurde geschlossen".into();
                    }
                } else if let Some(window) = self.windows.get(index).cloned() {
                    self.ipc.send(&ShellCommand::SwitchWorkspace {
                        workspace: window.workspace,
                    });
                    self.ipc.send(&ShellCommand::FocusWindow { id: window.id });
                    self.close_launcher_after_launch(qh, RepaintReason::Pointer);
                    return;
                }
            }
        }
        self.window_picker = Some(picker);
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    pub(crate) fn window_picker_pointer(
        &mut self,
        qh: &QueueHandle<Self>,
        event: &PointerEventKind,
        pos: (f64, f64),
        width: u32,
        height: u32,
    ) -> bool {
        if let Some(picker) = &self.window_picker {
            // Switching rooms must wait until the implicit pointer grab ends.
            // Consume the release here so it cannot activate the Hub beneath.
            if let PointerEventKind::Release { button, .. } = event {
                let count = if picker.target_window.is_some() {
                    self.workspace_state.rooms.snapshot.rooms.len()
                } else {
                    self.windows.len()
                };
                if let Some(mut hit) = crate::window_picker::hit(
                    picker,
                    count,
                    width,
                    height,
                    pos.0 as i32,
                    pos.1 as i32,
                ) {
                    if *button == 0x111 && picker.target_window.is_none() {
                        if let Hit::Row(index) = hit {
                            hit = Hit::Move(index);
                        }
                    }
                    if *button == 0x110 || *button == 0x111 {
                        self.picker_action(qh, hit);
                    }
                }
            }
            return true;
        }
        if !self.room_management_open
            && !self.launcher_settings_open
            && !self.hub_search_active
            && matches!(event, PointerEventKind::Release { button: 0x110, .. })
            && crate::hub_view::hit_windows(pos.0 as i32, pos.1 as i32, width, height)
        {
            self.window_picker = Some(Picker::default());
            self.draw_launcher(qh, RepaintReason::Pointer);
            return true;
        }
        false
    }

    pub(crate) fn window_picker_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) -> bool {
        if self.window_picker.is_none() {
            if key == Keysym::F6 && !self.hub_search_active {
                self.window_picker = Some(Picker::default());
                self.draw_launcher(qh, RepaintReason::Keyboard);
                return true;
            }
            return false;
        }
        let picker = self.window_picker.as_mut().unwrap();
        let count = if picker.target_window.is_some() {
            self.workspace_state.rooms.snapshot.rooms.len()
        } else {
            self.windows.len()
        };
        picker.selected = picker.selected.min(count.saturating_sub(1));
        let action = match key {
            Keysym::Escape => Some(Hit::Back),
            Keysym::Return | Keysym::KP_Enter => Some(Hit::Row(picker.selected)),
            Keysym::F2 if picker.target_window.is_none() => Some(Hit::Move(picker.selected)),
            Keysym::Page_Up => Some(Hit::Previous),
            Keysym::Page_Down => Some(Hit::Next),
            Keysym::Up => {
                picker.selected = picker.selected.saturating_sub(1);
                None
            }
            Keysym::Down | Keysym::Tab => {
                picker.selected = (picker.selected + 1).min(count.saturating_sub(1));
                None
            }
            Keysym::Home => {
                picker.selected = 0;
                None
            }
            Keysym::End => {
                picker.selected = count.saturating_sub(1);
                None
            }
            _ => None,
        };
        if let Some(action) = action {
            self.picker_action(qh, action);
        } else {
            self.draw_launcher(qh, RepaintReason::Keyboard);
        }
        true
    }
}
