// Existing provider categories live inside the shared Control Center content.
pub(crate) fn effective_settings_category(
    selected: SettingsCategory,
    search: &str,
    themes: &[String],
    wallpapers: &[WallpaperEntry],
) -> SettingsCategory {
    // Historical pinning data stays intact, but the current panel has no app
    // shortcuts. Old category requests use the active Apps page instead.
    let selected = if selected == SettingsCategory::PinnedApps {
        SettingsCategory::DefaultApps
    } else {
        selected
    };
    let query = search.trim().to_lowercase();
    if query.is_empty() || settings_category_matches(selected, &query, themes, wallpapers) {
        selected
    } else {
        SettingsCategory::ALL
            .iter()
            .flat_map(|categories| categories.iter())
            .copied()
            .find(|cat| settings_category_matches(*cat, &query, themes, wallpapers))
            .unwrap_or(selected)
    }
}

fn page_categories(page: crate::control_center::Page) -> &'static [SettingsCategory] {
    use crate::control_center::Page;
    use SettingsCategory as Category;
    match page {
        Page::Apps => &[Category::DefaultApps],
        Page::Users => &[Category::Users],
        Page::Updates => &[Category::Updates],
        Page::Settings => &[Category::Wallpaper, Category::Cursor, Category::Display],
        _ => &[
            Category::SystemOverview,
            Category::Network,
            Category::Bluetooth,
            Category::Sound,
            Category::Printers,
            Category::Power,
        ],
    }
}

struct CategoryTab {
    category: SettingsCategory,
    selected: bool,
    width: i32,
}
impl CategoryTab {
    fn component(&self) -> niwoe_ui::widget::Component<'static> {
        use niwoe_ui::widget::{Component, ComponentKind};
        let mut component = Component::new(ComponentKind::Tab, self.category.label(), self.width);
        component.state.selected = self.selected;
        component
    }
}
impl Widget for CategoryTab {
    fn id(&self) -> Option<&'static str> {
        Some(self.category.chip_id())
    }
    fn style(&self) -> WidgetStyle {
        self.component().style()
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        self.component().paint(area, canvas, theme, state);
    }
}

fn build_category_navigation(width: u32, selected: SettingsCategory) -> Box<dyn Widget> {
    let page = crate::control_center::Page::for_settings(selected);
    let categories = page_categories(page);
    let c = niwoe_tokens::ControlCenter::DEFAULT;
    let s = niwoe_tokens::Spacing::DEFAULT;
    let tab_width = (width as i32 - c.outer_pad * 2 - s.sm * (categories.len() as i32 - 1))
        / categories.len() as i32;
    let children: Vec<Box<dyn Widget>> = categories
        .iter()
        .filter(|_| categories.len() > 1)
        .map(|category| {
            Box::new(CategoryTab {
                category: *category,
                selected: *category == selected,
                width: tab_width,
            }) as Box<dyn Widget>
        })
        .collect();
    Box::new(Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            flex_shrink: 0.0,
            align_items: Some(AlignItems::Center),
            size: UiSize {
                width: ui_length(width as f32),
                height: ui_length(category_navigation_height(selected) as f32),
            },
            gap: UiSize {
                width: ui_length(s.sm as f32),
                height: ui_length(0.0_f32),
            },
            padding: TaffyRect {
                left: ui_length(c.outer_pad as f32),
                right: ui_length(c.outer_pad as f32),
                ..WidgetStyle::default().padding
            },
            ..Default::default()
        },
        children,
    ))
}

pub(crate) fn category_navigation_height(selected: SettingsCategory) -> i32 {
    if page_categories(crate::control_center::Page::for_settings(selected)).len() > 1 {
        niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height
    } else {
        0
    }
}

#[cfg(test)]
mod navigation_tests {
    #[test]
    fn historical_pinning_requests_use_the_active_apps_page() {
        assert_eq!(
            super::effective_settings_category(super::SettingsCategory::PinnedApps, "", &[], &[]),
            super::SettingsCategory::DefaultApps
        );
    }
    #[test]
    fn single_page_sections_do_not_repeat_the_sidebar_as_a_tab() {
        for category in [SettingsCategory::DefaultApps, SettingsCategory::Users, SettingsCategory::Updates] {
            assert_eq!(category_navigation_height(category), 0);
            let tree = build_category_navigation(1118, category);
            assert!(tree.children().is_empty());
        }
        assert!(category_navigation_height(SettingsCategory::Display) > 0);
        assert!(category_navigation_height(SettingsCategory::Network) > 0);
    }
    use super::*;
    #[test]
    fn content_tabs_keep_all_active_providers_reachable_without_pin_or_theme_selection() {
        use crate::control_center::Page;
        let exposed: Vec<_> = [
            Page::Apps,
            Page::Users,
            Page::System,
            Page::Settings,
            Page::Updates,
        ]
        .into_iter()
        .flat_map(page_categories)
        .copied()
        .collect();
        assert_eq!(exposed.len(), 12);
        for category in SettingsCategory::ALL.iter().flat_map(|group| group.iter()) {
            assert!(exposed.contains(category));
        }
        assert!(!exposed.contains(&SettingsCategory::Theme));
        assert!(!exposed.contains(&SettingsCategory::PinnedApps));
    }

    #[test]
    fn search_updates_category_and_frame_identity_together() {
        let category = effective_settings_category(SettingsCategory::DefaultApps, "wlan", &[], &[]);
        assert_eq!(category, SettingsCategory::Network);
        assert_eq!(
            crate::control_center::Page::for_settings(category),
            crate::control_center::Page::System
        );
        assert_eq!(
            effective_settings_category(category, "no-matching-provider", &[], &[]),
            category
        );
    }
}
