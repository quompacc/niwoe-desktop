use std::collections::HashMap;

use bitflags::bitflags;
use serde::Deserialize;

mod defaults;
mod parse;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const SUPER = 0x01;
        const SHIFT = 0x02;
        const CTRL  = 0x04;
        const ALT   = 0x08;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keybind {
    pub modifiers: Modifiers,
    pub keysym: u32,
}

impl Keybind {
    pub fn new(modifiers: Modifiers, keysym: u32) -> Self {
        Self { modifiers, keysym }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    SwitchWorkspace(usize),
    MoveToWorkspace(usize),
    ToggleTiling,
    ToggleFloating,
    ToggleFullscreen,
    LaunchTerminal,
    LaunchBrowser,
    LaunchFiles,
    SwapDirection { dx: i8, dy: i8 },
    FocusDirection { dx: i8, dy: i8 },
    CycleWorkspace(i8),
    CycleWindow(i8),
    ForceSplit(SplitDir),
    ResizeTile { dir: SplitDir, delta: f32 },
    CloseWindow,
    ToggleLauncher,
    ToggleQuickSettings,
    OpenSystemSettings,
    LockSession,
    ReloadConfig,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitDir {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct KeybindConfig {
    bindings: Vec<(Keybind, Action)>,
}

impl KeybindConfig {
    pub fn bindings(&self) -> &[(Keybind, Action)] {
        &self.bindings
    }

    pub fn find_action(&self, modifiers: Modifiers, keysym: u32) -> Option<&Action> {
        self.bindings
            .iter()
            .find(|(kb, _)| kb.modifiers == modifiers && kb.keysym == keysym)
            .map(|(_, action)| action)
    }

    pub fn from_map(raw: &HashMap<String, String>) -> Result<Self, String> {
        let mut bindings = Vec::new();

        for (key_str, action_str) in raw {
            let keybind = parse::parse_keybind(key_str)
                .map_err(|err| format!("invalid keybind {:?}: {}", key_str, err))?;
            let action = parse::parse_action(action_str)
                .map_err(|err| format!("invalid action {:?}: {}", action_str, err))?;
            bindings.push((keybind, action));
        }

        Ok(Self { bindings })
    }
}

impl Default for KeybindConfig {
    fn default() -> Self {
        let raw = defaults::default_bindings();
        match Self::from_map(&raw) {
            Ok(cfg) => cfg,
            Err(err) => {
                tracing::warn!("default keybind parse error (bug): {}", err);
                Self {
                    bindings: Vec::new(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct KeybindToml {
    pub keybinds: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Action, KeybindConfig, Modifiers};

    #[test]
    fn omarchy_shell_entry_points_are_distinct_and_parseable() {
        let cfg = KeybindConfig::default();
        for (mods, key, action) in [
            (Modifiers::SUPER, 0x20, Action::ToggleLauncher),
            (
                Modifiers::SUPER | Modifiers::ALT,
                0x20,
                Action::OpenSystemSettings,
            ),
            (Modifiers::SUPER, 0xff1b, Action::ToggleQuickSettings),
            (Modifiers::SUPER, 0x77, Action::CloseWindow),
            (
                Modifiers::SUPER | Modifiers::CTRL,
                0x6c,
                Action::LockSession,
            ),
        ] {
            assert_eq!(cfg.find_action(mods, key), Some(&action));
        }
        assert_eq!(cfg.find_action(Modifiers::SUPER, 0x71), None);
        assert_eq!(cfg.find_action(Modifiers::SUPER, 0x6c), None);
    }

    #[test]
    fn valid_keybinds_are_parsed() {
        let mut raw = HashMap::new();
        raw.insert("Super+T".to_string(), "toggle-tiling".to_string());
        raw.insert("Super+Space".to_string(), "toggle-launcher".to_string());
        let cfg = KeybindConfig::from_map(&raw).expect("valid keybinds");
        assert_eq!(cfg.bindings().len(), 2);
    }

    #[test]
    fn invalid_keybind_returns_controlled_error() {
        let mut raw = HashMap::new();
        raw.insert("Super+NotARealKey".to_string(), "toggle-tiling".to_string());
        let err = KeybindConfig::from_map(&raw).expect_err("must fail");
        assert!(err.contains("invalid keybind"));
    }

    #[test]
    fn defaults_include_workspace_switch_1_to_9() {
        let cfg = KeybindConfig::default();
        for (keysym, idx) in [
            (0x31, 0usize),
            (0x32, 1),
            (0x33, 2),
            (0x34, 3),
            (0x35, 4),
            (0x36, 5),
            (0x37, 6),
            (0x38, 7),
            (0x39, 8),
        ] {
            let action = cfg.find_action(Modifiers::SUPER, keysym);
            assert_eq!(action, Some(&Action::SwitchWorkspace(idx)));
        }
    }

    #[test]
    fn defaults_include_move_to_workspace_1_to_9() {
        let cfg = KeybindConfig::default();
        let mods = Modifiers::SUPER | Modifiers::SHIFT;
        for (keysym, idx) in [
            (0x31, 0usize),
            (0x32, 1),
            (0x33, 2),
            (0x34, 3),
            (0x35, 4),
            (0x36, 5),
            (0x37, 6),
            (0x38, 7),
            (0x39, 8),
        ] {
            let action = cfg.find_action(mods, keysym);
            assert_eq!(action, Some(&Action::MoveToWorkspace(idx)));
        }
    }

    #[test]
    fn reload_config_action_is_bindable() {
        let mut raw = HashMap::new();
        raw.insert("Super+R".to_string(), "reload-config".to_string());
        let cfg = KeybindConfig::from_map(&raw).expect("valid reload-config keybind");
        assert_eq!(
            cfg.find_action(Modifiers::SUPER, 0x72),
            Some(&Action::ReloadConfig)
        );
    }

    #[test]
    fn defaults_do_not_include_reload_config_binding() {
        let cfg = KeybindConfig::default();
        assert!(!cfg
            .bindings()
            .iter()
            .any(|(_, action)| matches!(action, Action::ReloadConfig)));
    }
}
