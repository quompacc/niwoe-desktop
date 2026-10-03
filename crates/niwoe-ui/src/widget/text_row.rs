//! Shared settings row. Text and cached artwork are prepared during tree build.
use super::Widget;
use crate::{effect, ui_length, Rect, Theme, UiSize, WidgetState, WidgetStyle};
use niwoe_tokens::{Controls, Interaction, Spacing};
use std::sync::Arc;
use tiny_skia::{Pixmap, PixmapMut};

pub struct TextRow {
    pub id: Option<&'static str>,
    pub title: Box<str>,
    pub subtitle: Box<str>,
    pub width: i32,
    pub selected: bool,
    pub leading: Option<Arc<Pixmap>>,
    pub trailing: Option<Arc<Pixmap>>,
}

impl TextRow {
    pub fn new(title: impl Into<Box<str>>, subtitle: impl Into<Box<str>>, width: i32) -> Self {
        Self {
            id: None,
            title: title.into(),
            subtitle: subtitle.into(),
            width,
            selected: false,
            leading: None,
            trailing: None,
        }
    }
}

impl Widget for TextRow {
    fn id(&self) -> Option<&'static str> {
        self.id
    }
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(Controls::TEXT_PAIR_HEIGHT as f32),
            },
            ..Default::default()
        }
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let base = if self.selected {
            Interaction::DEFAULT.selected_tint(theme.palette.surface)
        } else {
            theme.palette.surface
        };
        let color = match (self.id.is_some(), state) {
            (true, WidgetState::Hovered) => Interaction::DEFAULT.hover(base),
            (true, WidgetState::Pressed) => Interaction::DEFAULT.pressed(base),
            _ => base,
        };
        if let Some(path) = effect::rounded_rect_path(area, theme.radius.sm) {
            effect::paint_fill(canvas, &path, color);
            effect::paint_border(
                canvas,
                &path,
                theme.palette.border_subtle(),
                Controls::BORDER as f32,
            );
        }
        let s = Spacing::DEFAULT;
        let side = s.xl;
        let mut x = area.x + s.lg;
        let mut right = area.x + area.width - s.lg;
        if let Some(icon) = &self.leading {
            effect::paint_image_contain(
                canvas,
                icon,
                Rect {
                    x,
                    y: area.y + (area.height - side) / 2,
                    width: side,
                    height: side,
                },
            );
            x += side + s.md;
        }
        if let Some(icon) = &self.trailing {
            effect::paint_image_contain(
                canvas,
                icon,
                Rect {
                    x: right - side,
                    y: area.y + (area.height - side) / 2,
                    width: side,
                    height: side,
                },
            );
            right -= side + s.md;
        }
        effect::paint_text_pair(
            canvas,
            Rect {
                x,
                y: area.y,
                width: (right - x).max(0),
                height: area.height,
            },
            &self.title,
            &self.subtitle,
            theme.palette.text,
            theme.palette.text_dim,
        );
    }
}
