use smithay_client_toolkit::seat::keyboard::Keysym;
use wayland_client::QueueHandle;

use crate::wayland::{ClickAction, NiwoeShell, RepaintReason};

fn next_focus(current: Option<usize>, count: usize, backwards: bool) -> Option<usize> {
    if count == 0 {
        return None;
    }
    Some(match current.filter(|&index| index < count) {
        Some(0) if backwards => count - 1,
        Some(index) if backwards => index - 1,
        Some(index) => (index + 1) % count,
        None if backwards => count - 1,
        None => 0,
    })
}

impl NiwoeShell {
    pub(super) fn hub_navigation_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) -> bool {
        let room_count = self
            .workspace_state
            .rooms
            .snapshot
            .rooms
            .len()
            .min(niwoe_tokens::Hub::DEFAULT.room_columns as usize);
        let backwards = matches!(key, Keysym::ISO_Left_Tab | Keysym::Left | Keysym::Up);
        if matches!(
            key,
            Keysym::Tab
                | Keysym::ISO_Left_Tab
                | Keysym::Left
                | Keysym::Right
                | Keysym::Up
                | Keysym::Down
        ) {
            self.room_keyboard_focus =
                next_focus(self.room_keyboard_focus, room_count + 1, backwards);
            self.draw_launcher(qh, RepaintReason::Keyboard);
            return true;
        }
        if matches!(key, Keysym::Return | Keysym::KP_Enter) {
            match self.room_keyboard_focus {
                Some(index) if index == room_count => self.open_room_management(qh),
                Some(index) => {
                    if let Some(workspace) = self
                        .workspace_state
                        .rooms
                        .snapshot
                        .rooms
                        .get(index)
                        .map(|room| room.workspace)
                    {
                        self.handle_panel_click(qh, ClickAction::SwitchWorkspace(workspace));
                        self.close_launcher_after_launch(qh, RepaintReason::Keyboard);
                    }
                }
                None => {}
            }
            return true;
        }
        false
    }

    pub(super) fn room_management_navigation_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) {
        let room_count = self.workspace_state.rooms.snapshot.rooms.len();
        let backwards = matches!(key, Keysym::ISO_Left_Tab | Keysym::Left | Keysym::Up);
        if matches!(
            key,
            Keysym::Tab
                | Keysym::ISO_Left_Tab
                | Keysym::Left
                | Keysym::Right
                | Keysym::Up
                | Keysym::Down
        ) {
            let count = room_count + usize::from(room_count < niwoe_config::rooms::MAX_ROOMS);
            self.room_keyboard_focus = next_focus(self.room_keyboard_focus, count, backwards);
            if let Some(index) = self.room_keyboard_focus {
                self.room_management_page = index.min(room_count.saturating_sub(1))
                    / (niwoe_tokens::ControlCenter::DEFAULT.room_columns as usize
                        * niwoe_tokens::ControlCenter::DEFAULT.room_page_rows as usize);
            }
            self.draw_launcher(qh, RepaintReason::Keyboard);
        } else if matches!(key, Keysym::Return | Keysym::KP_Enter) {
            if self.room_keyboard_focus == Some(room_count) {
                self.open_new_room(qh);
                return;
            }
            if let Some(id) = self
                .room_keyboard_focus
                .and_then(|index| self.workspace_state.rooms.snapshot.rooms.get(index))
                .map(|room| room.id)
            {
                self.open_room_configuration(qh, id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::next_focus;

    #[test]
    fn room_focus_cycles_and_wraps() {
        assert_eq!(next_focus(None, 4, false), Some(0));
        assert_eq!(next_focus(Some(3), 4, false), Some(0));
        assert_eq!(next_focus(None, 4, true), Some(3));
        assert_eq!(next_focus(Some(0), 4, true), Some(3));
        assert_eq!(next_focus(None, 0, false), None);
    }
}
