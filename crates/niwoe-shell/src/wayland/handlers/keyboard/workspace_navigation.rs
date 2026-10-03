use super::*;

impl NiwoeShell {
    pub(super) fn workspace_navigation_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) {
        let page = crate::workspaces::page_size() as isize;
        let columns = niwoe_tokens::WorkspaceSwitcher::DEFAULT.columns as isize;
        match key {
            Keysym::Escape => {
                self.close_workspace_popup(CommitReason::Input);
            }
            Keysym::Return | Keysym::KP_Enter | Keysym::F2 => {
                if let Some(room) = self
                    .workspace_state
                    .rooms
                    .snapshot
                    .rooms
                    .get(self.workspace_state.selected)
                {
                    let workspace = room.workspace;
                    if key == Keysym::F2 {
                        self.open_room_editor(qh, workspace);
                    } else {
                        self.handle_workspace_click(
                            qh,
                            crate::ClickAction::SwitchWorkspace(workspace),
                        );
                    }
                }
                return;
            }
            Keysym::Left | Keysym::ISO_Left_Tab => self.workspace_state.select_relative(-1),
            Keysym::Right | Keysym::Tab => self.workspace_state.select_relative(1),
            Keysym::Up => self.workspace_state.select_relative(-columns),
            Keysym::Down => self.workspace_state.select_relative(columns),
            Keysym::Page_Up => self.workspace_state.select_relative(-page),
            Keysym::Page_Down => self.workspace_state.select_relative(page),
            Keysym::Home => self.workspace_state.selected = 0,
            Keysym::End => {
                self.workspace_state.selected = self
                    .workspace_state
                    .rooms
                    .snapshot
                    .rooms
                    .len()
                    .saturating_sub(1)
            }
            _ => return,
        }
        self.workspace_hover_idx = None;
        self.draw_workspace_popup(qh, RepaintReason::Keyboard);
    }
}
