fn settings_theme_from_config(config: &ThemeConfig) -> Theme {
    theme_from_config(config)
}

/// Content heading inside the shared native Control Center: page identity on
/// the left, persistent settings search on the right.
struct SettingsHeaderBar {
    width: i32,
    children: Vec<Box<dyn Widget>>,
}

impl Widget for SettingsHeaderBar {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            flex_shrink: 0.0,
            align_items: Some(AlignItems::Center),
            gap: UiSize {
                width: ui_length(SETTINGS_CHROME.header_gap as f32),
                height: ui_length(0.0_f32),
            },
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(SETTINGS_CHROME.header_height as f32),
            },
            padding: TaffyRect {
                left: ui_length(SETTINGS_CHROME.header_pad as f32),
                right: ui_length(SETTINGS_CHROME.header_pad as f32),
                top: ui_length(0.0_f32),
                bottom: ui_length(0.0_f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, _area: Rect, _canvas: &mut PixmapMut<'_>, _theme: &Theme, _state: WidgetState) {
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
}

struct SettingsTitle {
    width: i32,
    label: Box<str>,
}

impl Widget for SettingsTitle {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(SETTINGS_CHROME.heading_height as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _state: WidgetState) {
        let eyebrow_y = area.y + Typography::DEFAULT.caption_size as i32;
        paint_text(
            canvas,
            "CONTROL CENTER",
            area.x,
            eyebrow_y,
            Typography::DEFAULT.caption_size as f32,
            theme.palette.text_dim,
        );
        paint_text(
            canvas,
            &self.label,
            area.x,
            eyebrow_y + SETTINGS_CHROME.header_gap + Typography::DEFAULT.display_size as i32,
            Typography::DEFAULT.display_size as f32,
            theme.palette.text,
        );
    }
}

/// Search field filling the rest of the header. Type-to-filter; the actual
/// query lives in NiwoeShell.settings_search and is fed in via `query`.
struct SettingsSearchField {
    width: i32,
    query: Box<str>,
}

impl Widget for SettingsSearchField {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(SETTINGS_CHROME.search_height as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let label = if self.query.is_empty() {
            "Einstellungen durchsuchen…"
        } else {
            &self.query
        };
        niwoe_ui::widget::Component::new(niwoe_ui::widget::ComponentKind::Input, label, self.width)
            .paint(area, canvas, theme, state);
    }
}

/// Small dim all-caps group label inside the sidebar (e.g. "DARSTELLUNG").
struct SidebarSectionLabel {
    text: &'static str,
    width: i32,
    pad_top: i32,
}

impl Widget for SidebarSectionLabel {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length((self.pad_top + SETTINGS_CHROME.sidebar_section_height) as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _state: WidgetState) {
        paint_text(
            canvas,
            self.text,
            area.x + 16,
            area.y + area.height - 5,
            Typography::DEFAULT.caption_size as f32,
            theme.palette.text_dim,
        );
    }
}

fn settings_category_matches(
    cat: SettingsCategory,
    query: &str,
    available_themes: &[String],
    available_wallpapers: &[WallpaperEntry],
) -> bool {
    if query.is_empty()
        || cat.label().to_lowercase().contains(query)
        || cat.search_keywords().iter().any(|kw| kw.contains(query))
    {
        return true;
    }
    match cat {
        SettingsCategory::Theme => available_themes
            .iter()
            .any(|theme| theme.to_lowercase().contains(query)),
        SettingsCategory::Wallpaper => available_wallpapers
            .iter()
            .any(|wallpaper| wallpaper.display_name.to_lowercase().contains(query)),
        _ => false,
    }
}
