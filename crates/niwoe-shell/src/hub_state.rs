//! Bounded, generation-scoped Hub preview state. Images never survive closure.
use crate::wayland::WindowInfo;
use niwoe_ipc::RoomEntry;
use std::collections::HashMap;

pub(crate) const PAGE_SIZE: usize = niwoe_tokens::Hub::DEFAULT.room_columns as usize;
type PreviewSignature = (Vec<(u64, Option<String>)>, String, PreviewPage);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PreviewPage {
    pub index: usize,
    pub count: usize,
    pub max_width: u32,
    pub max_height: u32,
}
#[derive(Default)]
pub(crate) struct HubState {
    pub page: usize,
    pub selected: Option<usize>,
    pub images: HashMap<String, tiny_skia::Pixmap>,
    generation: u64,
    signature: Option<PreviewSignature>,
    pending: HashMap<String, String>,
}
impl HubState {
    pub fn clear(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.signature = None;
        self.pending.clear();
        self.images.clear();
    }
    pub fn prepare(
        &mut self,
        rooms: &[RoomEntry],
        windows: &[WindowInfo],
        context: String,
    ) -> Vec<(String, String)> {
        self.page = self.page.min(rooms.len().saturating_sub(1) / PAGE_SIZE);
        let h = niwoe_tokens::Hub::DEFAULT;
        self.prepare_page(
            rooms,
            windows,
            context,
            PreviewPage {
                index: self.page,
                count: PAGE_SIZE,
                max_width: h.preview_width,
                max_height: h.preview_height,
            },
        )
    }
    pub fn prepare_page(
        &mut self,
        rooms: &[RoomEntry],
        windows: &[WindowInfo],
        context: String,
        mut page: PreviewPage,
    ) -> Vec<(String, String)> {
        let c = niwoe_tokens::ControlCenter::DEFAULT;
        let max_height = if page.count == 1 {
            c.config_preview_capture_height
        } else {
            c.room_preview_height
        };
        if page.count == 0
            || page.count > (c.room_columns * c.room_page_rows) as usize
            || page.max_width == 0
            || page.max_height == 0
            || page.max_width > c.room_preview_width
            || page.max_height > max_height
        {
            self.clear();
            return Vec::new();
        }
        page.index = page.index.min(rooms.len().saturating_sub(1) / page.count);
        let visible: Vec<_> = rooms
            .iter()
            .skip(page.index * page.count)
            .take(page.count)
            .map(|r| (r.id, preview_window(r, windows).map(|w| w.id.clone())))
            .collect();
        if self.signature.as_ref() == Some(&(visible.clone(), context.clone(), page)) {
            return Vec::new();
        }
        self.clear();
        let requests: Vec<_> = visible
            .iter()
            .filter_map(|(_, id)| id.as_ref())
            .map(|id| {
                let request = format!("hub-{}-{}-{}", std::process::id(), self.generation, id);
                self.pending.insert(request.clone(), id.clone());
                (request, id.clone())
            })
            .collect();
        self.signature = Some((visible, context, page));
        requests
    }
    pub fn accept(
        &mut self,
        request: &str,
        id: &str,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> bool {
        if self.pending.get(request).map(String::as_str) != Some(id) {
            return false;
        }
        self.pending.remove(request);
        let Some((_, _, page)) = self.signature.as_ref() else {
            return false;
        };
        if width == 0
            || height == 0
            || width > page.max_width
            || height > page.max_height
            || data.len() != width as usize * height as usize * 4
        {
            return false;
        }
        let mut rgba = data.to_vec();
        for pixel in rgba.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
            pixel[3] = u8::MAX;
        }
        let Some(image) =
            tiny_skia::Pixmap::from_vec(rgba, tiny_skia::IntSize::from_wh(width, height).unwrap())
        else {
            return false;
        };
        self.images.insert(id.into(), image);
        true
    }
}
pub(crate) fn preview_window<'a>(
    room: &RoomEntry,
    windows: &'a [WindowInfo],
) -> Option<&'a WindowInfo> {
    windows
        .iter()
        .find(|w| w.workspace == room.workspace && !w.minimized)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Target {
    Room(u64),
    Window(String),
    App(String),
}
#[derive(Clone)]
pub(crate) struct ResultRow {
    pub target: Target,
    pub title: String,
    pub detail: String,
}
// Derived from the existing snapshots and app catalogue, never a second index.
pub(crate) fn search(
    rooms: &[RoomEntry],
    windows: &[WindowInfo],
    apps: &[crate::launcher::DesktopApp],
    query: &str,
    hidden: &std::collections::HashSet<String>,
) -> Vec<ResultRow> {
    let q = query.trim().to_lowercase();
    let matches = |s: &str| s.to_lowercase().contains(&q);
    let mut rows: Vec<_> = rooms
        .iter()
        .filter(|r| matches(&r.name))
        .map(|r| ResultRow {
            target: Target::Room(r.id),
            title: r.name.clone(),
            detail: format!("Raum · {}", r.description),
        })
        .collect();
    rows.extend(
        windows
            .iter()
            .filter(|w| matches(&w.title) || w.app_id.as_deref().is_some_and(matches))
            .map(|w| ResultRow {
                target: Target::Window(w.id.clone()),
                title: if w.title.trim().is_empty() {
                    w.app_id.clone().unwrap_or_else(|| "Fenster".into())
                } else {
                    w.title.clone()
                },
                detail: format!(
                    "Fenster · {}{}",
                    rooms
                        .iter()
                        .find(|r| r.workspace == w.workspace)
                        .map(|r| r.name.as_str())
                        .unwrap_or("Raum"),
                    if w.minimized { " · Minimiert" } else { "" }
                ),
            }),
    );
    rows.extend(
        apps.iter()
            .filter(|a| !hidden.contains(&a.program) && matches(&a.name))
            .map(|a| ResultRow {
                target: Target::App(app_identity(a)),
                title: a.name.clone(),
                detail: "Anwendung starten".into(),
            }),
    );
    rows
}

pub(crate) fn app_identity(app: &crate::launcher::DesktopApp) -> String {
    if app.desktop_id.is_empty() {
        app.program.clone()
    } else {
        app.desktop_id.clone()
    }
}

/// Wayland positive vertical axis means down/next; horizontal-only frames
/// carry a zero vertical axis and must leave the page unchanged.
pub(crate) fn scroll_back(discrete: i32, absolute: f64) -> Option<bool> {
    let value = if discrete != 0 {
        f64::from(discrete)
    } else {
        absolute
    };
    (value != 0.0).then_some(value < 0.0)
}

#[cfg(test)]
mod tests;
