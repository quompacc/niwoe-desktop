use crate::ShellCommand;

impl ShellCommand {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Layout { .. } => "layout",
            Self::FirstRun { .. } => "first-run",
            Self::PanelPreferences { .. } => "panel-preferences",
            Self::RequestRoomSnapshot => "request-room-snapshot",
            Self::MutateRoom { .. } => "mutate-room",
            Self::Authenticate { .. } => "authenticate",
            Self::SwitchWorkspace { .. } => "switch-workspace",
            Self::ToggleLauncher => "toggle-launcher",
            Self::ToggleQuickSettings => "toggle-quick-settings",
            Self::OpenSystemSettings => "open-system-settings",
            Self::SettingsRefresh => "settings-refresh",
            Self::AppearanceRefresh => "appearance-refresh",
            Self::AppearanceThemeSet { .. } => "appearance-theme-set",
            Self::AppearanceWallpaperSet { .. } => "appearance-wallpaper-set",
            Self::AppearanceWallpaperModeSet { .. } => "appearance-wallpaper-mode-set",
            Self::QuickSettingsNetworkRefresh => "quick-settings-network-refresh",
            Self::QuickSettingsNetworkConnect { .. } => "quick-settings-network-connect",
            Self::QuickSettingsNetworkDisconnect => "quick-settings-network-disconnect",
            Self::AudioVolumeSet { .. } => "audio-volume-set",
            Self::AudioMuteToggle => "audio-mute-toggle",
            Self::PowerProfileSet { .. } => "power-profile-set",
            Self::FocusWindow { .. } => "focus-window",
            Self::MoveWindowToRoom { .. } => "move-window-to-room",
            Self::LaunchApp { .. } => "launch-app",
            Self::LockSession => "lock-session",
            Self::PowerPrepareSleep => "power-prepare-sleep",
            Self::PowerResume => "power-resume",
            Self::ReloadConfig => "reload-config",
            Self::Quit => "quit",
            Self::CaptureWindowThumbnail { .. } => "capture-window-thumbnail",
            Self::ScreenshotConsentResponse { .. } => "screenshot-consent-response",
            Self::ScreenshotRegionResponse { .. } => "screenshot-region-response",
        }
    }
}
