// A visible flyout dispatches sub-actions; its parent keeps the menu mapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesktopSelection {
    OpenSettings,
    Main(DesktopContextMenuAction),
    Sub(SettingsSubAction),
}

pub(crate) fn desktop_selection(open: bool, px: f64, py: f64) -> Option<DesktopSelection> {
    if open {
        if let Some(index) = submenu_hit_item_local(px, py) {
            return Some(DesktopSelection::Sub(submenu_items()[index].1));
        }
    }
    let action = desktop_item_list()[desktop_hit_item_local(px, py)?].1;
    Some(if action == DesktopContextMenuAction::Settings {
        DesktopSelection::OpenSettings
    } else {
        DesktopSelection::Main(action)
    })
}

#[cfg(test)]
mod selection_tests {
    use super::*;
    #[test]
    fn settings_parent_never_dispatches_an_unrelated_page() {
        for open in [false, true] {
            assert_eq!(
                desktop_selection(
                    open,
                    MENU_WIDTH as f64 / 2.0,
                    (VPAD + SETTINGS_ITEM_IDX as i32 * ITEM_H + ITEM_H / 2) as f64
                ),
                Some(DesktopSelection::OpenSettings)
            );
        }
    }
    #[test]
    fn flyout_hit_requires_visible_content_and_ignores_gap() {
        let x = (MENU_WIDTH + SUBMENU_GAP + SUBMENU_WIDTH / 2) as f64;
        let y = (VPAD + ITEM_H / 2) as f64;
        assert_eq!(desktop_selection(false, x, y), None);
        assert_eq!(
            desktop_selection(true, x, y),
            Some(DesktopSelection::Sub(SettingsSubAction::Display))
        );
        assert_eq!(
            desktop_selection(true, (MENU_WIDTH + SUBMENU_GAP / 2) as f64, y),
            None
        );
    }
}
