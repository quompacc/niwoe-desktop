struct WallpaperRow {
    index: usize,
    display_name: Box<str>,
    thumbnail: Option<std::sync::Arc<Pixmap>>,
    status: &'static str,
    is_selected: bool,
    row_width: i32,
}

impl Widget for WallpaperRow {
    fn id(&self) -> Option<&'static str> {
        WALLPAPER_WIDGET_IDS.get(self.index).copied()
    }
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.row_width as f32),
                height: ui_length(SETTINGS_CHROME.wallpaper_row_height as f32),
            },
            ..Default::default()
        }
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let base = if self.is_selected {
            Interaction::DEFAULT.selected_tint(theme.palette.surface)
        } else {
            theme.palette.surface
        };
        let color = match state {
            WidgetState::Idle => base,
            WidgetState::Hovered => Interaction::DEFAULT.hover(base),
            WidgetState::Pressed => Interaction::DEFAULT.pressed(base),
        };
        if let Some(path) = rounded_rect_path(area, theme.radius.sm) {
            paint_fill(canvas, &path, color);
            paint_border(
                canvas,
                &path,
                theme.palette.border_subtle(),
                niwoe_tokens::Controls::BORDER as f32,
            );
        }
        let c = SETTINGS_CHROME;
        let thumbnail = Rect {
            x: area.x + c.option_gap,
            y: area.y + (area.height - c.wallpaper_thumbnail_height as i32) / 2,
            width: c.wallpaper_thumbnail_width as i32,
            height: c.wallpaper_thumbnail_height as i32,
        };
        if let Some(image) = &self.thumbnail {
            niwoe_ui::effect::paint_image_contain(canvas, image, thumbnail);
        } else if let Some(path) = rounded_rect_path(thumbnail, theme.radius.sm) {
            paint_fill(canvas, &path, theme.palette.surface_alt);
        }
        let x = thumbnail.x + thumbnail.width + c.group_gap;
        niwoe_ui::effect::paint_text_pair(
            canvas,
            Rect {
                x,
                y: area.y,
                width: (area.x + area.width - c.option_gap - x).max(0),
                height: area.height,
            },
            &self.display_name,
            self.status,
            theme.palette.text,
            theme.palette.text_dim,
        );
    }
}

struct WallpaperBrowseRow {
    row_width: i32,
}
impl Widget for WallpaperBrowseRow {
    fn id(&self) -> Option<&'static str> {
        Some("wallpaper-browse")
    }
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.row_width as f32),
                height: ui_length(SETTINGS_CHROME.wallpaper_browse_height as f32),
            },
            ..Default::default()
        }
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let c = SETTINGS_CHROME;
        niwoe_ui::widget::Component::new(
            niwoe_ui::widget::ComponentKind::Button,
            "",
            self.row_width,
        )
        .paint(area, canvas, theme, state);
        let side = niwoe_tokens::Spacing::DEFAULT.xl;
        if let Some(icon) = niwoe_ui::effect::symbol_icon(
            niwoe_ui::effect::Symbol::Folder,
            theme.palette.text,
            side as u32,
        ) {
            niwoe_ui::effect::paint_image_contain(
                canvas,
                &icon,
                Rect {
                    x: area.x + c.group_pad,
                    y: area.y + (area.height - side) / 2,
                    width: side,
                    height: side,
                },
            );
        }
        let x = area.x + c.group_pad + side + c.group_gap;
        niwoe_ui::effect::paint_text_pair(
            canvas,
            Rect {
                x,
                y: area.y,
                width: (area.x + area.width - c.group_pad - x).max(0),
                height: area.height,
            },
            "Eigenes Bild auswählen …",
            "Öffnet die Dateiauswahl",
            theme.palette.text,
            theme.palette.text_dim,
        );
    }
}

struct SettingsPageLabel {
    label: String,
    width: i32,
}
impl Widget for SettingsPageLabel {
    fn style(&self) -> WidgetStyle {
        niwoe_ui::widget::Component::new(
            niwoe_ui::widget::ComponentKind::Text,
            &self.label,
            self.width,
        )
        .style()
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        niwoe_ui::widget::Component::new(
            niwoe_ui::widget::ComponentKind::Text,
            &self.label,
            self.width,
        )
        .paint(area, canvas, theme, state);
    }
}

pub(crate) fn wallpaper_page_size(content_height: u32) -> usize {
    let c = SETTINGS_CHROME;
    let reserved =
        niwoe_tokens::Controls::MIN_HEIGHT * 2 + c.wallpaper_browse_height + c.option_gap * 3;
    let room = settings_group_body_height(content_height).saturating_sub(reserved as u32);
    let rows =
        ((room + c.option_gap as u32) / (c.wallpaper_row_height + c.option_gap) as u32).max(1);
    rows as usize * c.wallpaper_grid_columns
}

pub(crate) fn wallpaper_visible_indices(entries: &[WallpaperEntry], query: &str) -> Vec<usize> {
    let query = query.trim().to_lowercase();
    let hit = !query.is_empty()
        && entries
            .iter()
            .any(|entry| entry.display_name.to_lowercase().contains(&query));
    entries
        .iter()
        .enumerate()
        .take(WALLPAPER_WIDGET_IDS.len())
        .filter(|(_, entry)| !hit || entry.display_name.to_lowercase().contains(&query))
        .map(|(index, _)| index)
        .collect()
}
