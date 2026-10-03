use super::{CommitReason, NiwoeShell, RepaintReason};
use crate::hub_state::{self, Target, PAGE_SIZE};
use smithay_client_toolkit::seat::{keyboard::Keysym, pointer::PointerEventKind};
use wayland_client::QueueHandle;
impl NiwoeShell {
    pub(crate) fn hub_visible(&self) -> bool {
        self.launcher_state.open
            && !self.room_management_open
            && !self.launcher_settings_open
            && self.window_picker.is_none()
    }
    pub(crate) fn hub_selection_target(&self) -> Option<Target> {
        let index = self.hub.selected?;
        if !self.hub_visible() {
            return None;
        }
        if self.hub_search_active {
            self.hub_results().get(index).map(|r| r.target.clone())
        } else {
            self.workspace_state
                .rooms
                .snapshot
                .rooms
                .get(index)
                .map(|r| Target::Room(r.id))
        }
    }
    pub(crate) fn reconcile_hub_selection(&mut self, target: Option<Target>) {
        let Some(target) = target else {
            return;
        };
        if self.hub_search_active {
            let rows = self.hub_results();
            self.hub.selected = rows
                .iter()
                .position(|r| r.target == target)
                .or_else(|| (!rows.is_empty()).then_some(0));
        } else if let Target::Room(id) = target {
            self.hub.selected = self
                .workspace_state
                .rooms
                .snapshot
                .rooms
                .iter()
                .position(|r| r.id == id);
            if let Some(index) = self.hub.selected {
                self.hub.page = index / PAGE_SIZE;
            }
        }
    }
    pub(crate) fn hub_results(&self) -> Vec<hub_state::ResultRow> {
        hub_state::search(
            &self.workspace_state.rooms.snapshot.rooms,
            &self.windows,
            &self.launcher_state.apps,
            &self.search_query,
            &self.hidden_execs,
        )
    }
    pub(crate) fn prepare_hub(&mut self) {
        if !self.hub_visible() || self.hub_search_active {
            self.hub.clear();
            return;
        }
        let context = format!(
            "{:?}:{:?}",
            self.launcher_content_size(),
            self.focused_output_id
        );
        for (request_id, id) in self.hub.prepare(
            &self.workspace_state.rooms.snapshot.rooms,
            &self.windows,
            context,
        ) {
            let h = niwoe_tokens::Hub::DEFAULT;
            self.ipc
                .send(&niwoe_ipc::ShellCommand::CaptureWindowThumbnail {
                    request_id: Some(request_id),
                    id,
                    max_width: h.preview_width,
                    max_height: h.preview_height,
                });
        }
    }
    pub(crate) fn activate_hub_target(&mut self, qh: &QueueHandle<Self>, target: Target) {
        use niwoe_ipc::ShellCommand;
        match target {
            Target::Room(id) => {
                if let Some(room) = self
                    .workspace_state
                    .rooms
                    .snapshot
                    .rooms
                    .iter()
                    .find(|r| r.id == id)
                {
                    self.ipc.send(&ShellCommand::SwitchWorkspace {
                        workspace: room.workspace,
                    });
                } else {
                    return;
                }
            }
            Target::Window(id) => {
                if let Some(window) = self.windows.iter().find(|w| w.id == id) {
                    self.ipc.send(&ShellCommand::SwitchWorkspace {
                        workspace: window.workspace,
                    });
                    self.ipc.send(&ShellCommand::FocusWindow { id });
                } else {
                    return;
                }
            }
            Target::App(id) => {
                if let Some(app) = self
                    .launcher_state
                    .apps
                    .iter()
                    .find(|app| hub_state::app_identity(app) == id)
                    .cloned()
                {
                    crate::launcher::LauncherState::launch_desktop_app(app, &mut self.ipc);
                } else {
                    return;
                }
            }
        }
        self.close_launcher_after_launch(qh, RepaintReason::Keyboard);
    }
    pub(crate) fn hub_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) -> bool {
        if !self.hub_visible() {
            return false;
        }
        if key == Keysym::F6 {
            return false;
        }
        if key == Keysym::Escape {
            if self.hub_search_active {
                self.hub_search_active = false;
                self.search_query.clear();
                self.hub.selected = None;
            } else {
                self.close_launcher_after_launch(qh, RepaintReason::Keyboard);
                return true;
            }
        } else if key == Keysym::BackSpace {
            self.search_query.pop();
            self.hub.selected = None;
        } else if matches!(key, Keysym::Return | Keysym::KP_Enter) {
            if self.hub_search_active {
                if let Some(row) = self.hub_results().get(self.hub.selected.unwrap_or(0)) {
                    self.activate_hub_target(qh, row.target.clone());
                    return true;
                }
            } else {
                let rooms = &self.workspace_state.rooms.snapshot.rooms;
                let index = self.hub.selected.unwrap_or(self.hub.page * PAGE_SIZE);
                if index == rooms.len() {
                    self.open_room_management(qh);
                    return true;
                }
                if let Some(room) = rooms.get(index) {
                    self.activate_hub_target(qh, Target::Room(room.id));
                    return true;
                }
            }
        } else if matches!(
            key,
            Keysym::Left
                | Keysym::Right
                | Keysym::Up
                | Keysym::Down
                | Keysym::Tab
                | Keysym::ISO_Left_Tab
                | Keysym::Home
                | Keysym::End
                | Keysym::Page_Up
                | Keysym::Page_Down
        ) {
            let count = if self.hub_search_active {
                self.hub_results().len()
            } else {
                self.workspace_state.rooms.snapshot.rooms.len() + 1
            };
            let last = count.saturating_sub(1);
            let current = self.hub.selected.unwrap_or(self.hub.page * PAGE_SIZE);
            let back = matches!(
                key,
                Keysym::Left | Keysym::Up | Keysym::ISO_Left_Tab | Keysym::Page_Up
            );
            let step = if matches!(key, Keysym::Page_Up | Keysym::Page_Down) {
                PAGE_SIZE
            } else {
                1
            };
            let index = match key {
                Keysym::Home => 0,
                Keysym::End => {
                    if self.hub_search_active {
                        last
                    } else {
                        last.saturating_sub(1)
                    }
                }
                _ if self.hub.selected.is_none() => {
                    if back {
                        last
                    } else {
                        0
                    }
                }
                _ if back => current.checked_sub(step).unwrap_or(last),
                _ => {
                    if current + step > last {
                        0
                    } else {
                        current + step
                    }
                }
            };
            self.hub.selected = Some(index);
            if !self.hub_search_active {
                self.hub.page = index.min(last.saturating_sub(1)) / PAGE_SIZE;
            }
        } else if let Some(c) = key.key_char().filter(|c| !c.is_control()) {
            self.hub_search_active = true;
            self.search_query.push(c);
            self.hub.selected = None;
        } else {
            return true;
        }
        self.draw_launcher(qh, RepaintReason::Keyboard);
        true
    }
    pub(crate) fn hub_pointer(
        &mut self,
        qh: &QueueHandle<Self>,
        event: &PointerEventKind,
        pos: (f64, f64),
        width: u32,
        height: u32,
    ) -> bool {
        if !self.hub_visible() {
            return false;
        }
        let (x, y) = (pos.0 as i32, pos.1 as i32);
        match event {
            PointerEventKind::Motion { .. } => {
                let hovered = if self.hub_search_active {
                    None
                } else {
                    crate::hub_view::hit_room(
                        x,
                        y,
                        width,
                        self.workspace_state
                            .rooms
                            .snapshot
                            .rooms
                            .len()
                            .saturating_sub(self.hub.page * PAGE_SIZE),
                    )
                };
                if self.hovered_bento_idx == hovered {
                    return true;
                }
                self.hovered_bento_idx = hovered;
            }
            PointerEventKind::Axis { vertical, .. } => {
                let Some(back) = hub_state::scroll_back(vertical.discrete, vertical.absolute)
                else {
                    return true;
                };
                if self.hub_search_active {
                    let count = self.hub_results().len();
                    let index = self.hub.selected.unwrap_or(0);
                    self.hub.selected = Some(if back {
                        index.saturating_sub(PAGE_SIZE)
                    } else {
                        (index + PAGE_SIZE).min(count.saturating_sub(1))
                    });
                } else {
                    let max = self
                        .workspace_state
                        .rooms
                        .snapshot
                        .rooms
                        .len()
                        .saturating_sub(1)
                        / PAGE_SIZE;
                    self.hub.page = if back {
                        self.hub.page.saturating_sub(1)
                    } else {
                        (self.hub.page + 1).min(max)
                    };
                    self.hub.selected = Some(self.hub.page * PAGE_SIZE);
                }
            }
            PointerEventKind::Release { button: 0x110, .. } => {
                if crate::hub_view::hit_close(x, y, width) {
                    self.close_launcher_after_launch(qh, RepaintReason::Pointer);
                    return true;
                }
                if crate::hub_view::hit_manage_rooms(x, y, width) {
                    self.open_room_management(qh);
                    return true;
                }
                let rooms = &self.workspace_state.rooms.snapshot.rooms;
                let target = if self.hub_search_active {
                    let rows = self.hub_results();
                    crate::hub_view::hit_search(
                        x,
                        y,
                        width,
                        height,
                        self.hub.selected.unwrap_or(0),
                        rows.len(),
                    )
                    .and_then(|i| rows.get(i))
                    .map(|r| r.target.clone())
                } else if let Some(back) = crate::hub_view::hit_page(x, y, width) {
                    let max = rooms.len().saturating_sub(1) / PAGE_SIZE;
                    self.hub.page = if back {
                        self.hub.page.saturating_sub(1)
                    } else {
                        (self.hub.page + 1).min(max)
                    };
                    self.hub.selected = Some(self.hub.page * PAGE_SIZE);
                    None
                } else if let Some(slot) = crate::hub_view::hit_room(
                    x,
                    y,
                    width,
                    rooms.len().saturating_sub(self.hub.page * PAGE_SIZE),
                ) {
                    rooms.get(self.hub.page * PAGE_SIZE + slot).map(|room| {
                        if crate::hub_view::hit_preview(x, y, width, slot) {
                            if let Some(w) = hub_state::preview_window(room, &self.windows) {
                                return Target::Window(w.id.clone());
                            }
                        }
                        Target::Room(room.id)
                    })
                } else {
                    None
                };
                if let Some(target) = target {
                    self.activate_hub_target(qh, target);
                    return true;
                }
            }
            _ => return true,
        }
        self.draw_launcher(qh, RepaintReason::Pointer);
        true
    }
    pub(crate) fn receive_hub_thumbnail(
        &mut self,
        request: Option<String>,
        id: String,
        path: String,
        width: u32,
        height: u32,
    ) {
        if request
            .as_deref()
            .is_some_and(|r| !r.starts_with(&format!("hub-{}-", std::process::id())))
        {
            return;
        }
        // Only consume compositor-owned files in this user's runtime directory.
        let p = std::path::Path::new(&path);
        let runtime = std::env::var_os("XDG_RUNTIME_DIR").map(std::path::PathBuf::from);
        if p.parent() != runtime.as_deref()
            || !p
                .file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("niwoe-thumb-"))
        {
            return;
        }
        let data = std::fs::symlink_metadata(p)
            .ok()
            .filter(|m| m.file_type().is_file() && m.len() <= 512 * 288 * 4)
            .and_then(|_| std::fs::read(p).ok());
        let _ = std::fs::remove_file(p);
        let wanted = if self.thumbnail_popup_open {
            self.thumbnail_popup_window_ids.clone()
        } else {
            self.thumbnail_hover_app_idx
                .and_then(|index| self.pinned_apps.get(index))
                .map(|app| {
                    super::state::pinned_app_window_ids(
                        app,
                        &self.windows,
                        self.panel_active_workspace(),
                    )
                })
                .unwrap_or_default()
                .into_iter()
                .take(crate::THUMBNAIL_MAX_WINDOWS)
                .collect()
        };
        if let Some(data) = data {
            if let Some(request) = request {
                if self.hub_visible() && self.hub.accept(&request, &id, width, height, &data) {
                    self.launcher_dirty = true;
                }
            } else if wanted.contains(&id)
                && width > 0
                && height > 0
                && width <= 512
                && height <= 288
                && data.len() == width as usize * height as usize * 4
            {
                self.thumbnail_cache.retain(|key, _| wanted.contains(key));
                self.thumbnail_cache.insert(id, (width, height, data));
                self.thumbnail_dirty = true;
            }
        }
    }

    pub(crate) fn discard_hub_on_lock(&mut self) {
        self.hub.clear();
        self.thumbnail_cache.clear();
        self.launcher_state.close();
        self.launcher_layer.set_keyboard_interactivity(
            smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::OnDemand,
        );
        self.unmap_launcher(CommitReason::Input);
        self.close_thumbnail_popup(CommitReason::Input);
    }
}
