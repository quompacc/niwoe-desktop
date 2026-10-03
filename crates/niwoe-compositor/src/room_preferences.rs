//! Explicit adapter: config owns validation, IPC owns the wire representation.
use niwoe_config::rooms as config;
use niwoe_ipc as wire;

pub(super) fn preferences_from_wire(value: wire::RoomPreferences) -> config::RoomPreferences {
    config::RoomPreferences {
        icon: value.icon,
        apps: value
            .apps
            .into_iter()
            .map(|app| match app {
                wire::AppReference::Native(id) => config::AppReference::Native(id),
                wire::AppReference::Xwayland(id) => config::AppReference::Xwayland(id),
            })
            .collect(),
        layout: match value.layout {
            wire::RoomLayout::Tiling => config::RoomLayout::Tiling,
            wire::RoomLayout::Floating => config::RoomLayout::Floating,
        },
        restore: match value.restore {
            wire::RoomRestore::Disabled => config::RoomRestore::Disabled,
            wire::RoomRestore::LayoutOnly => config::RoomRestore::LayoutOnly,
            wire::RoomRestore::RelaunchApps => config::RoomRestore::RelaunchApps,
        },
    }
}

pub(crate) fn preferences_to_wire(value: &config::RoomPreferences) -> wire::RoomPreferences {
    wire::RoomPreferences {
        icon: value.icon.clone(),
        apps: value
            .apps
            .iter()
            .map(|app| match app {
                config::AppReference::Native(id) => wire::AppReference::Native(id.clone()),
                config::AppReference::Xwayland(id) => wire::AppReference::Xwayland(id.clone()),
            })
            .collect(),
        layout: match value.layout {
            config::RoomLayout::Tiling => wire::RoomLayout::Tiling,
            config::RoomLayout::Floating => wire::RoomLayout::Floating,
        },
        restore: match value.restore {
            config::RoomRestore::Disabled => wire::RoomRestore::Disabled,
            config::RoomRestore::LayoutOnly => wire::RoomRestore::LayoutOnly,
            config::RoomRestore::RelaunchApps => wire::RoomRestore::RelaunchApps,
        },
    }
}
