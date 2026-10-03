fn build_cursor_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    let labels = [
        "Klein · 16 px",
        "Normal · 24 px",
        "Groß · 32 px",
        "Sehr groß · 48 px",
    ];
    let size_chips = crate::cursor::CURSOR_SIZE_OPTIONS
        .iter()
        .zip(labels)
        .map(|((px, id, _), label)| {
            Box::new(SettingsControl {
                id: Some(id),
                kind: niwoe_ui::widget::ComponentKind::Chip,
                disabled: false,
                label: label.into(),
                selected: *px == ctx.cursor_size,
                width: c.cursor_size_width,
            }) as Box<dyn Widget>
        })
        .collect();
    rows.push(Box::new(Container::row(c.option_gap, size_chips)));
    if ctx.available_cursor_themes.is_empty() {
        rows.push(Box::new(SettingsPlaceholder {
            width: row_w,
            text: "Keine Mauszeiger-Themen installiert.",
        }));
    } else {
        let width = (row_w - c.option_gap * (c.cursor_grid_columns - 1) as i32)
            / c.cursor_grid_columns as i32;
        let current = ctx
            .cursor_previews
            .matches(ctx.available_cursor_themes, ctx.cursor_size);
        let cards: Vec<Box<dyn Widget>> = ctx
            .available_cursor_themes
            .iter()
            .take(crate::cursor::CURSOR_THEME_WIDGET_IDS.len())
            .enumerate()
            .map(|(index, name)| {
                let entry = current
                    .then(|| ctx.cursor_previews.entries.get(index))
                    .flatten();
                let selected = name == ctx.current_cursor_theme;
                Box::new(CursorThemeRow {
                    index,
                    name: entry.map(|e| e.label.as_str()).unwrap_or(name).into(),
                    image: entry.and_then(|e| e.image.clone()),
                    status: if entry.is_none() {
                        "Vorschau wird geladen …"
                    } else if entry.is_some_and(|e| e.image.is_none()) {
                        "Vorschau nicht verfügbar"
                    } else if selected {
                        "Ausgewählt"
                    } else {
                        "Mauszeiger-Thema"
                    },
                    is_selected: selected,
                    row_width: width,
                }) as Box<dyn Widget>
            })
            .collect();
        let mut pending = cards.into_iter();
        loop {
            let group: Vec<_> = pending.by_ref().take(c.cursor_grid_columns).collect();
            if group.is_empty() {
                break;
            }
            rows.push(Box::new(Container::row(c.option_gap, group)));
        }
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Mauszeiger",
        "Größe und Darstellung wählen. Die Vorschau zeigt das echte Zeigerbild.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}

struct CursorThemeRow {
    index: usize,
    name: Box<str>,
    image: Option<std::sync::Arc<Pixmap>>,
    status: &'static str,
    is_selected: bool,
    row_width: i32,
}
impl Widget for CursorThemeRow {
    fn id(&self) -> Option<&'static str> {
        crate::cursor::CURSOR_THEME_WIDGET_IDS
            .get(self.index)
            .copied()
    }
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.row_width as f32),
                height: ui_length(SETTINGS_CHROME.cursor_row_height as f32),
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
        let background = match state {
            WidgetState::Idle => base,
            WidgetState::Hovered => Interaction::DEFAULT.hover(base),
            WidgetState::Pressed => Interaction::DEFAULT.pressed(base),
        };
        if let Some(path) = rounded_rect_path(area, theme.radius.sm) {
            paint_fill(canvas, &path, background);
            paint_border(
                canvas,
                &path,
                theme.palette.border_subtle(),
                niwoe_tokens::Controls::BORDER as f32,
            );
        }
        let c = SETTINGS_CHROME;
        let side = c.cursor_preview_side;
        let x = area.x + c.option_gap;
        if let Some(image) = &self.image {
            niwoe_ui::effect::paint_image_contain(
                canvas,
                image,
                Rect {
                    x,
                    y: area.y + (area.height - side) / 2,
                    width: side,
                    height: side,
                },
            );
        }
        let text_x = x + side + c.group_gap;
        niwoe_ui::effect::paint_text_pair(
            canvas,
            Rect {
                x: text_x,
                y: area.y,
                width: (area.x + area.width - c.option_gap - text_x).max(0),
                height: area.height,
            },
            &self.name,
            self.status,
            theme.palette.text,
            theme.palette.text_dim,
        );
    }
}
