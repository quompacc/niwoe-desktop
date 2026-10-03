struct AddAppRow {
    index: usize,
    name: Box<str>,
    accent: Color,
    row_width: i32,
    icon: Option<Pixmap>,
}

impl Widget for AddAppRow {
    fn id(&self) -> Option<&'static str> {
        PINNED_ADD_IDS.get(self.index).copied()
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.row_width as f32),
                height: ui_length(PINNED_ROW_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let bg = match state {
            WidgetState::Idle => theme.palette.surface_alt,
            WidgetState::Hovered => Interaction::DEFAULT.hover(theme.palette.surface_alt),
            WidgetState::Pressed => Interaction::DEFAULT.pressed(theme.palette.surface_alt),
        };
        if let Some(path) = rounded_rect_path(area, 0) {
            paint_fill(canvas, &path, bg);
        }
        let text_x = if let Some(ref icon) = self.icon {
            let iw = icon.width() as i32;
            let ih = icon.height() as i32;
            let ix = area.x + 6;
            let iy = area.y + (area.height - ih) / 2;
            canvas.draw_pixmap(
                ix,
                iy,
                icon.as_ref(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
            area.x + 6 + iw + 6
        } else {
            area.x + 10
        };
        paint_text(
            canvas,
            &self.name,
            text_x,
            area.y + area.height - 14,
            13.0,
            self.accent,
        );
    }
}
