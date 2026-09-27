//! Manual bounded restore controller. Login and workspace switches never call it.
use super::NiwoeState;
use niwoe_config::{
    layouts::{Entry, Snapshot},
    rooms::{AppReference, RoomId},
};
use niwoe_ipc::{LayoutNotice, LayoutResult, ShellEvent};
use smithay::desktop::Window;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    time::{Duration, Instant},
};

mod apply;
mod capture;
mod commands;
mod progress;

pub(crate) struct Saved {
    snapshot: Snapshot,
    bindings: BTreeMap<u32, Window>,
    changed: Option<Instant>,
    armed: bool,
}

pub(crate) struct Job {
    request_id: String,
    run: u64,
    snapshot: Snapshot,
    bound: BTreeMap<u32, Window>,
    results: BTreeMap<u32, String>,
    waiting: BTreeMap<u32, Instant>,
    queue: Vec<u32>,
    deadline: Instant,
}

#[derive(Default)]
pub(crate) struct Controller {
    revisions: BTreeMap<u64, u64>,
    request_id: String,
    saved: BTreeMap<u64, Saved>,
    job: Option<Job>,
    attempted: BTreeSet<(u64, u32)>,
    statuses: BTreeMap<u64, LayoutNotice>,
    serial: u64,
    next_poll: Option<Instant>,
}

fn path(room: u64) -> PathBuf {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        });
    base.join("niwoe/layouts").join(format!("room-{room}.toml"))
}

fn app(window: &Window) -> Option<AppReference> {
    super::utils::window_app_id(window)
        .filter(|id| !id.is_empty())
        .map(|id| {
            if window.x11_surface().is_some() {
                AppReference::Xwayland(id)
            } else {
                AppReference::Native(id)
            }
        })
}

impl NiwoeState {
    fn layout_status(
        &mut self,
        room: u64,
        running: bool,
        message: impl Into<String>,
        results: Vec<LayoutResult>,
    ) {
        let notice = LayoutNotice::Status {
            revision: self
                .layout_restore
                .revisions
                .get(&room)
                .copied()
                .unwrap_or(0),
            room_id: room,
            running,
            message: message.into(),
            results,
        };
        self.layout_restore.statuses.insert(room, notice.clone());
        self.ipc.broadcast(&ShellEvent::Layout {
            request_id: self.layout_restore.request_id.clone(),
            notice,
        });
    }

    fn layout_windows(&self) -> Vec<Window> {
        (0..self.workspaces.count())
            .flat_map(|i| self.workspaces.space_at(i).elements().cloned())
            .chain(self.minimized_windows.values().map(|m| m.window.clone()))
            .collect()
    }
}

fn result_rows(snapshot: &Snapshot, results: &BTreeMap<u32, String>) -> Vec<LayoutResult> {
    snapshot
        .entries
        .iter()
        .map(|e| LayoutResult {
            file: e.file.clone(),
            key: e.key,
            label: {
                let app = match &e.app {
                    AppReference::Native(s) | AppReference::Xwayland(s) => s,
                };
                if e.title.is_empty() {
                    app.clone()
                } else {
                    format!("{} · {app}", e.title)
                }
            },
            message: results
                .get(&e.key)
                .cloned()
                .unwrap_or_else(|| "Ausstehend".into()),
        })
        .collect()
}

/// Match across sessions only when both sides are unambiguous. Never by order.
fn match_entry(entry: &Entry, snapshot: &Snapshot, windows: &[Window]) -> Option<Window> {
    let candidates: Vec<_> = windows
        .iter()
        .filter_map(|w| Some((w, (app(w)?, super::utils::window_list_entry(w)?.1))))
        .collect();
    let metadata: Vec<_> = candidates
        .iter()
        .map(|(_, identity)| identity.clone())
        .collect();
    niwoe_config::layouts::matching::unique_match(entry, snapshot, &metadata)
        .map(|index| candidates[index].0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_binding_rows_distinguish_documents_of_the_same_app() {
        let entry = Entry {
            key: 0,
            app: AppReference::Native("org.example.Editor".into()),
            desktop_id: None,
            title: "First document".into(),
            file: None,
            floating: true,
            geometry: niwoe_config::layouts::Geometry {
                x: 0,
                y: 0,
                width: 400,
                height: 300,
            },
            output: None,
        };
        let mut second = entry.clone();
        second.key = 1;
        second.title = "Second document".into();
        let mut untitled = entry.clone();
        untitled.key = 2;
        untitled.title.clear();
        let snapshot = Snapshot {
            revision: 1,
            schema_version: niwoe_config::layouts::VERSION,
            room_id: RoomId(1),
            mode: niwoe_config::rooms::RoomLayout::Floating,
            entries: vec![entry, second, untitled],
            tree: vec![],
        };
        let rows = result_rows(&snapshot, &BTreeMap::new());
        assert!(rows[0].label.starts_with("First document"));
        assert!(rows[1].label.starts_with("Second document"));
        assert_eq!(rows[2].label, "org.example.Editor");
        assert_eq!(
            rows.iter().map(|r| r.key).collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
    }
}
