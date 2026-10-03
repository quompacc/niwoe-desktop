use super::*;
use niwoe_config::{
    layouts::{Geometry, Node, OutputHint, MAX_WINDOWS, VERSION},
    rooms::RoomLayout,
};
use niwoe_wm::{tiling::LayoutNode, SplitDir, WorkspaceMode};

impl NiwoeState {
    pub(super) fn capture_layout(&self, room: u64) -> Result<Saved, String> {
        let slot = self
            .workspaces
            .rooms()
            .slot_for_room(RoomId(room))
            .ok_or("Raum nicht vorhanden")?;
        let space = self.workspaces.space_at(slot);
        let mut windows: Vec<_> = space
            .elements()
            .cloned()
            .chain(
                self.minimized_windows
                    .values()
                    .filter(|m| m.workspace == slot)
                    .map(|m| m.window.clone()),
            )
            .collect();
        windows.sort_by_key(|w| super::super::utils::window_list_entry(w).map(|(id, _)| id));
        if windows.len() > MAX_WINDOWS {
            return Err("Höchstens 128 Fenster pro Layout".into());
        }
        let previous = self.layout_restore.saved.get(&room);
        let mut entries = Vec::new();
        let mut bindings = BTreeMap::new();
        let mut next_key = previous
            .and_then(|s| s.snapshot.entries.iter().map(|e| e.key).max())
            .map_or(Some(0), |key| key.checked_add(1));
        for window in &windows {
            let Some(app) = app(window) else {
                return Err("Fenster ohne App-Identität: Speichern nicht möglich".into());
            };
            let loc = space
                .element_location(window)
                .or_else(|| {
                    self.minimized_windows
                        .values()
                        .find(|m| &m.window == window)
                        .map(|m| m.restore_loc)
                })
                .unwrap_or_default();
            let size = window.geometry().size;
            let output = self
                .output_registry
                .list()
                .iter()
                .find(|o| {
                    loc.x >= o.geometry.x
                        && loc.y >= o.geometry.y
                        && (loc.x as i64) < o.geometry.x as i64 + o.geometry.width as i64
                        && (loc.y as i64) < o.geometry.y as i64 + o.geometry.height as i64
                })
                .or_else(|| self.output_registry.list().first())
                .map(|o| {
                    let area =
                        super::super::normal_window_workarea_from_output_geometry(o.geometry);
                    OutputHint {
                        name: o.name.clone(),
                        scale_millis: (o.scale * 1000.0) as u32,
                        workarea: Geometry {
                            x: area.x,
                            y: area.y,
                            width: area.width,
                            height: area.height,
                        },
                    }
                });
            let old = previous.and_then(|p| {
                p.bindings
                    .iter()
                    .find(|(_, w)| *w == window)
                    .and_then(|(key, _)| p.snapshot.entries.iter().find(|e| e.key == *key))
            });
            let key = if let Some(old) = old {
                old.key
            } else {
                let key = next_key.ok_or("Layoutschlüssel ausgeschöpft")?;
                next_key = key.checked_add(1);
                key
            };
            entries.push(Entry {
                key,
                app,
                desktop_id: old.and_then(|e| e.desktop_id.clone()),
                title: super::super::utils::window_list_entry(window)
                    .map(|(_, t)| t)
                    .unwrap_or_default(),
                file: old.and_then(|e| e.file.clone()),
                floating: self.wm_workspaces[slot].is_floating(window),
                geometry: Geometry {
                    x: loc.x,
                    y: loc.y,
                    width: size.w,
                    height: size.h,
                },
                output,
            });
            bindings.insert(key, window.clone());
        }
        let mut tree = Vec::new();
        for node in self.wm_workspaces[slot].layout_snapshot() {
            tree.push(match node {
                LayoutNode::Window(window) => Node::Window {
                    key: *bindings
                        .iter()
                        .find(|(_, w)| **w == window)
                        .map(|(key, _)| key)
                        .ok_or("Layout enthält nicht erfasstes Fenster")?,
                },
                LayoutNode::Split {
                    dir,
                    ratio,
                    left,
                    right,
                } => Node::Split {
                    horizontal: dir == SplitDir::Horizontal,
                    ratio_millis: (ratio * 1000.0).round() as u16,
                    left,
                    right,
                },
            });
        }
        let snapshot = Snapshot {
            revision: previous.map_or(0, |s| s.snapshot.revision),
            schema_version: VERSION,
            room_id: RoomId(room),
            mode: if self.wm_workspaces[slot].mode == WorkspaceMode::Floating {
                RoomLayout::Floating
            } else {
                RoomLayout::Tiling
            },
            entries,
            tree,
        };
        snapshot.validate()?;
        Ok(Saved {
            snapshot,
            bindings,
            armed: true,
            changed: None,
        })
    }
}
