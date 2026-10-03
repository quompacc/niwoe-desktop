mod calendar;
mod desktop_menu_focus;
mod handlers;
mod hub;
mod init;
mod ipc;
mod layout_restore;
mod panel_preview;
mod popup_placement;
mod render;
mod screencopy;
mod shell;
mod state;
mod time;
mod types;
mod window_picker;

pub use ipc::IpcClient;
pub use types::{ClickAction, ClickZone, Rect, RoomEditAction};

pub(crate) use init::initialize;
pub(crate) use shell::{CommitReason, CommitStats, CommitSurfaceKind, NiwoeShell, RepaintReason};
pub(crate) use types::{SurfaceKind, WindowInfo};
