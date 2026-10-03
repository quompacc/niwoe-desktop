use crate::{
    control_center::{Page, PAGES},
    wayland::{NiwoeShell, RepaintReason},
};
use smithay_client_toolkit::seat::keyboard::Keysym;
use wayland_client::QueueHandle;

fn next_page(current: usize, backwards: bool) -> usize {
    (current + if backwards { PAGES.len() - 1 } else { 1 }) % PAGES.len()
}

#[derive(Debug, PartialEq, Eq)]
enum Dismiss {
    AppPicker,
    DisplayModes,
    Page,
}

fn escape_target(app_picker: bool, display_modes: bool) -> Dismiss {
    if app_picker {
        Dismiss::AppPicker
    } else if display_modes {
        Dismiss::DisplayModes
    } else {
        Dismiss::Page
    }
}

impl NiwoeShell {
    pub(super) fn control_center_key(&mut self, qh: &QueueHandle<Self>, key: Keysym) -> bool {
        if !self.control_center_visible() {
            return false;
        }
        if key == Keysym::F7 {
            self.navigate_control_center_page(qh, Page::Panel);
            return true;
        }
        if key == Keysym::F8 {
            self.control_center_nav.widget_focus = None;
            self.control_center_nav.sidebar_focus = PAGES
                .iter()
                .position(|page| *page == self.control_center_page());
            self.draw_launcher(qh, RepaintReason::Keyboard);
            return true;
        }
        if let Some(index) = self.control_center_nav.sidebar_focus {
            match key {
                Keysym::Tab | Keysym::ISO_Left_Tab => {
                    self.control_center_nav.sidebar_focus = None;
                    self.control_center_nav.widget_focus =
                        self.launcher_settings_open.then_some("show-tile-view");
                }
                Keysym::Up | Keysym::Down | Keysym::Left | Keysym::Right => {
                    self.control_center_nav.sidebar_focus =
                        Some(next_page(index, matches!(key, Keysym::Up | Keysym::Left)));
                    self.draw_launcher(qh, RepaintReason::Keyboard);
                    return true;
                }
                Keysym::Return | Keysym::KP_Enter | Keysym::space => {
                    self.navigate_control_center_page(qh, PAGES[index]);
                    return true;
                }
                Keysym::Escape => {
                    self.control_center_nav.sidebar_focus = None;
                    self.draw_launcher(qh, RepaintReason::Keyboard);
                    return true;
                }
                _ => return true,
            }
        }
        if !self.launcher_settings_open {
            return false;
        }
        if key == Keysym::space && self.control_center_nav.widget_focus.is_none() {
            return false;
        }
        if key == Keysym::Escape {
            match escape_target(
                self.default_apps_picker_open.is_some(),
                self.display_mode_dropdown_open.is_some(),
            ) {
                Dismiss::AppPicker => self.dispatch_widget_action(
                    qh,
                    crate::widget_action::WidgetAction::DefaultAppsClosePicker,
                ),
                Dismiss::DisplayModes => {
                    self.display_mode_dropdown_open = None;
                    self.draw_launcher(qh, RepaintReason::Keyboard);
                }
                Dismiss::Page => return false,
            }
            return true;
        }
        if !matches!(
            key,
            Keysym::Tab
                | Keysym::ISO_Left_Tab
                | Keysym::Down
                | Keysym::Up
                | Keysym::Return
                | Keysym::KP_Enter
                | Keysym::space
        ) {
            return false;
        }
        let (width, height) = self.launcher_content_size();
        let tree = self.settings_widget_tree(width, height);
        let viewport = niwoe_ui::PixelSize { width, height };
        let Ok(layout) = niwoe_ui::compute_layout(tree.as_ref(), viewport) else {
            return true;
        };
        let targets = crate::widget_traversal::focus_targets(tree.as_ref(), &layout, viewport);
        if targets.is_empty() {
            self.control_center_nav.widget_focus = None;
            return true;
        }
        let current = self
            .control_center_nav
            .widget_focus
            .and_then(|id| targets.iter().position(|(candidate, _)| *candidate == id));
        if matches!(key, Keysym::Return | Keysym::KP_Enter | Keysym::space) {
            if let Some(action) =
                current.and_then(|index| crate::widget_action::action_for_id(targets[index].0))
            {
                self.dispatch_widget_action(qh, action);
            }
        } else {
            let backwards = matches!(key, Keysym::ISO_Left_Tab | Keysym::Up);
            let next = match current {
                Some(index) => {
                    (index + if backwards { targets.len() - 1 } else { 1 }) % targets.len()
                }
                None if backwards => targets.len() - 1,
                None => 0,
            };
            self.control_center_nav.widget_focus = Some(targets[next].0);
        }
        self.draw_launcher(qh, RepaintReason::Keyboard);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escape_dismisses_transient_selection_before_the_settings_page() {
        assert_eq!(escape_target(true, false), Dismiss::AppPicker);
        assert_eq!(escape_target(false, true), Dismiss::DisplayModes);
        assert_eq!(escape_target(true, true), Dismiss::AppPicker);
        assert_eq!(escape_target(false, false), Dismiss::Page);
    }
    #[test]
    fn keyboard_traverses_every_visible_page_and_wraps_in_both_directions() {
        for index in 0..PAGES.len() {
            assert_eq!(next_page(next_page(index, false), true), index);
        }
        assert_eq!(PAGES[next_page(0, true)], Page::Updates);
        assert_eq!(PAGES[next_page(PAGES.len() - 1, false)], Page::Rooms);
    }
}
