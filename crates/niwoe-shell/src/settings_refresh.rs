//! One-shot Settings data refreshes kept off the Wayland event loop.

pub(crate) mod wallpaper;

use std::sync::mpsc::Sender;

use crate::settings_view::SettingsCategory;

pub(crate) enum SettingsData {
    SystemInfo(crate::sysinfo::SystemInfo),
    Users(crate::users::UserState),
    Printers(crate::printers::PrinterSnapshot),
    Audio(crate::audio::AudioSnapshot),
    Network(crate::network::NetworkLists),
    Bluetooth(crate::bluetooth::BluetoothSnapshot),
    DefaultApps(crate::default_apps::refresh::RefreshData),
    Wallpaper(wallpaper::WallpaperData),
    Cursor(crate::cursor::preview::CursorPreviews),
}

pub(crate) struct SettingsRefreshResult {
    pub category: SettingsCategory,
    pub data: SettingsData,
}

pub(crate) fn supports(category: SettingsCategory) -> bool {
    matches!(
        category,
        SettingsCategory::SystemOverview
            | SettingsCategory::Users
            | SettingsCategory::Printers
            | SettingsCategory::Sound
            | SettingsCategory::Network
            | SettingsCategory::Bluetooth
            | SettingsCategory::DefaultApps
            | SettingsCategory::Wallpaper
            | SettingsCategory::Cursor
    )
}

pub(crate) fn spawn(
    category: SettingsCategory,
    wallpaper: wallpaper::WallpaperRequest,
    cursor: crate::cursor::preview::CursorRequest,
    icon_config: (String, String),
    default_apps_change: Option<crate::default_apps::refresh::ChangeRequest>,
    tx: Sender<SettingsRefreshResult>,
) {
    if !supports(category) {
        return;
    }
    std::thread::spawn(move || {
        let data = match category {
            SettingsCategory::SystemOverview => {
                SettingsData::SystemInfo(crate::sysinfo::SystemInfo::gather())
            }
            SettingsCategory::Users => SettingsData::Users(crate::users::UserAccounts::gather()),
            SettingsCategory::Printers => {
                SettingsData::Printers(crate::printers::PrinterSnapshot::poll())
            }
            SettingsCategory::Sound => SettingsData::Audio(crate::audio::AudioSnapshot::poll()),
            SettingsCategory::Network => {
                SettingsData::Network(crate::network::NetworkLists::poll())
            }
            SettingsCategory::Bluetooth => {
                SettingsData::Bluetooth(crate::bluetooth::BluetoothSnapshot::poll())
            }
            SettingsCategory::DefaultApps => SettingsData::DefaultApps(
                crate::default_apps::refresh::RefreshData::load(&icon_config, default_apps_change),
            ),
            SettingsCategory::Wallpaper => {
                SettingsData::Wallpaper(wallpaper::WallpaperData::load(wallpaper))
            }
            SettingsCategory::Cursor => {
                SettingsData::Cursor(crate::cursor::preview::CursorPreviews::load(
                    cursor.themes,
                    cursor.size,
                    cursor.previous,
                ))
            }
            _ => return,
        };
        let _ = tx.send(SettingsRefreshResult { category, data });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_io_backed_pages_request_a_refresh() {
        assert!(supports(SettingsCategory::SystemOverview));
        assert!(supports(SettingsCategory::Users));
        assert!(supports(SettingsCategory::Printers));
        assert!(supports(SettingsCategory::Sound));
        assert!(supports(SettingsCategory::Network));
        assert!(supports(SettingsCategory::Bluetooth));
        assert!(supports(SettingsCategory::DefaultApps));
        assert!(supports(SettingsCategory::Wallpaper));
        assert!(!supports(SettingsCategory::Theme));
        assert!(supports(SettingsCategory::Cursor));
        assert!(!supports(SettingsCategory::Display));
    }
}
