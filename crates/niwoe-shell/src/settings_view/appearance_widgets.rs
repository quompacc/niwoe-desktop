struct PinnedAppLabel {
    label: Box<str>,
    program: Box<str>,
    width: i32,
    icon: Option<Pixmap>,
}

impl Widget for PinnedAppLabel {
    fn id(&self) -> Option<&'static str> {
        None
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(PINNED_ROW_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _state: WidgetState) {
        if let Some(path) = rounded_rect_path(area, 0) {
            paint_fill(canvas, &path, theme.palette.surface);
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
            &self.label,
            text_x,
            area.y + 16,
            13.0,
            theme.palette.text,
        );
        paint_text(
            canvas,
            &self.program,
            text_x,
            area.y + 32,
            11.0,
            theme.palette.text_dim,
        );
    }
}

struct SettingsPlaceholder {
    width: i32,
    text: &'static str,
}

impl Widget for SettingsPlaceholder {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(60.0_f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _state: WidgetState) {
        paint_text(
            canvas,
            self.text,
            area.x + 20,
            area.y + 36,
            13.0,
            theme.palette.text_dim,
        );
    }
}
