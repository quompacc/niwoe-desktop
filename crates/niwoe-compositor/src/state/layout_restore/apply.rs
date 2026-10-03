use super::*;
use niwoe_config::{
    layouts::{Geometry, Node},
    rooms::RoomLayout,
};
use niwoe_wm::{tiling::LayoutNode, SplitDir, WorkspaceMode};
use smithay::{reexports::wayland_protocols::xdg::shell::server::xdg_toplevel, utils::Rectangle};

impl NiwoeState {
    pub(super) fn apply_restored_layout(&mut self, job: &mut Job) {
        let Some(slot) = self.workspaces.rooms().slot_for_room(job.snapshot.room_id) else {
            return;
        };
        self.wm_workspaces[slot].mode = if job.snapshot.mode == RoomLayout::Floating {
            WorkspaceMode::Floating
        } else {
            WorkspaceMode::Tiling
        };
        for e in &job.snapshot.entries {
            let Some(window) = job.bound.get(&e.key) else {
                continue;
            };
            let Some((id, _)) = super::super::utils::window_list_entry(window) else {
                continue;
            };
            self.move_window_to_room(&id, job.snapshot.room_id.0);
            self.maximize_restore_locations.remove(&id);
            if let Some(m) = self.minimized_windows.remove(&id) {
                if let Some(x11) = m.window.x11_surface() {
                    let _ = x11.set_hidden(false);
                }
                self.workspaces
                    .space_at_mut(slot)
                    .map_element(m.window, m.restore_loc, false);
            }
            self.wm_workspaces[slot].set_floating(window, e.floating);
            let Some(output) = e
                .output
                .as_ref()
                .and_then(|o| {
                    self.output_registry
                        .list()
                        .iter()
                        .find(|live| live.name == o.name)
                })
                .or_else(|| self.output_registry.list().iter().find(|o| o.primary))
                .or_else(|| self.output_registry.list().first())
            else {
                continue;
            };
            let area = super::super::normal_window_workarea_from_output_geometry(output.geometry);
            let workarea = Geometry {
                x: area.x,
                y: area.y,
                width: area.width,
                height: area.height,
            };
            let mut g = e.geometry.clone();
            if let Some(old) = &e.output {
                g.x = workarea
                    .x
                    .saturating_add(g.x.saturating_sub(old.workarea.x));
                g.y = workarea
                    .y
                    .saturating_add(g.y.saturating_sub(old.workarea.y));
            }
            // Existing configured window minimum, then clamp to available area.
            g.width = g.width.max(niwoe_config::Decorations::RESTORE_MIN_WIDTH);
            g.height = g.height.max(niwoe_config::Decorations::RESTORE_MIN_HEIGHT);
            let g = g.fit(&workarea);
            let rect = Rectangle::new((g.x, g.y).into(), (g.width, g.height).into());
            if let Some(toplevel) = window.toplevel() {
                self.decoration_manager
                    .set_fullscreen(toplevel.wl_surface(), false);
                self.decoration_manager
                    .set_maximized(toplevel.wl_surface(), false);
                toplevel.with_pending_state(|s| {
                    s.states.unset(xdg_toplevel::State::Fullscreen);
                    s.states.unset(xdg_toplevel::State::Maximized);
                    for state in [
                        xdg_toplevel::State::TiledLeft,
                        xdg_toplevel::State::TiledRight,
                        xdg_toplevel::State::TiledTop,
                        xdg_toplevel::State::TiledBottom,
                    ] {
                        s.states.unset(state);
                    }
                    s.size = Some(rect.size);
                });
                self.decoration_manager
                    .set_tiled(toplevel.wl_surface(), false);
                toplevel.send_pending_configure();
            } else if let Some(x11) = window.x11_surface() {
                let _ = x11.set_fullscreen(false);
                let _ = x11.set_maximized(false);
                let _ = x11.configure(rect);
            }
            self.workspaces
                .space_at_mut(slot)
                .map_element(window.clone(), rect.loc, false);
            job.results.insert(e.key, "Fenster angeordnet".into());
        }
        let nodes = job
            .snapshot
            .tree
            .iter()
            .map(|node| match node {
                Node::Window { key } => LayoutNode::Window(job.bound.get(key).cloned()),
                Node::Split {
                    horizontal,
                    ratio_millis,
                    left,
                    right,
                } => LayoutNode::Split {
                    dir: if *horizontal {
                        SplitDir::Horizontal
                    } else {
                        SplitDir::Vertical
                    },
                    ratio: *ratio_millis as f32 / 1000.0,
                    left: *left,
                    right: *right,
                },
            })
            .collect();
        let extra = self.wm_workspaces[slot].tiled_windows();
        self.wm_workspaces[slot].restore_layout(nodes);
        for w in extra {
            if !self.wm_workspaces[slot].tiled_windows().contains(&w) {
                self.wm_workspaces[slot].add_tiled(w, None);
            }
        }
        self.tile_workspace(slot);
        // Preserve proven session bindings and explicit files across repeated
        // restores and a later Save, without arming writes over partial results.
        self.layout_restore.saved.insert(
            job.snapshot.room_id.0,
            Saved {
                snapshot: job.snapshot.clone(),
                bindings: job.bound.clone(),
                changed: None,
                armed: false,
            },
        );
        self.mark_all_outputs_dirty("layout-restore");
        self.broadcast_window_snapshot();
    }
}
