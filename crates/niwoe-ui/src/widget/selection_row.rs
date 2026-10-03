//! Borrowed native checkbox/radio row; no I/O, timers or retained raster.
//! Check artwork uses the bounded symbol cache; fonts use the existing cache.
use super::{ComponentState, Widget};
use crate::{effect, ui_length, Rect, Theme, UiSize, WidgetState, WidgetStyle};
use niwoe_tokens::{Controls, Interaction, Spacing};
use tiny_skia::{PathBuilder, PixmapMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionKind {
    Checkbox,
    Radio,
}

pub struct SelectionRow<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub width: i32,
    pub kind: SelectionKind,
    pub state: ComponentState,
}

impl Widget for SelectionRow<'_> {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(Controls::TEXT_PAIR_HEIGHT as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, pointer: WidgetState) {
        let p = theme.palette;
        let s = Spacing::DEFAULT;
        let selected = self.state.selected;
        let base = if selected {
            Interaction::DEFAULT.selected_tint(p.surface)
        } else {
            p.surface
        };
        let background = match (self.state.disabled, pointer) {
            (false, WidgetState::Hovered) => Interaction::DEFAULT.hover(base),
            (false, WidgetState::Pressed) => Interaction::DEFAULT.pressed(base),
            _ => base,
        };
        if let Some(path) = effect::rounded_rect_path(area, theme.radius.sm) {
            effect::paint_fill(canvas, &path, background);
            effect::paint_border(canvas, &path, p.border_control(), Controls::BORDER as f32);
        }
        let mark = Rect {
            x: area.x + s.sm,
            y: area.y + (area.height - s.lg) / 2,
            width: s.lg,
            height: s.lg,
        };
        let radius = if self.kind == SelectionKind::Radio {
            mark.width / 2
        } else {
            theme.radius.sm
        };
        let mark_color = if self.state.disabled {
            p.text_disabled()
        } else {
            p.accent
        };
        if let Some(path) = effect::rounded_rect_path(mark, radius) {
            effect::paint_fill(
                canvas,
                &path,
                if selected { mark_color } else { p.surface_alt },
            );
            effect::paint_border(canvas, &path, p.border_control(), Controls::BORDER as f32);
        }
        if selected {
            if self.kind == SelectionKind::Radio {
                if let Some(dot) = PathBuilder::from_circle(
                    (mark.x + mark.width / 2) as f32,
                    (mark.y + mark.height / 2) as f32,
                    (s.xs / 2) as f32,
                ) {
                    effect::paint_fill(canvas, &dot, p.on_accent());
                }
            } else if let Some(check) =
                effect::symbol_icon(effect::Symbol::Check, p.on_accent(), Controls::SYMBOL_SIZE)
            {
                canvas.draw_pixmap(
                    mark.x,
                    mark.y,
                    check.as_ref().as_ref(),
                    &Default::default(),
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        }
        let x = mark.x + mark.width + s.sm;
        effect::paint_text_pair(
            canvas,
            Rect {
                x,
                width: area.x + area.width - x - s.sm,
                ..area
            },
            self.title,
            self.subtitle,
            if self.state.disabled {
                p.text_disabled()
            } else {
                p.text
            },
            if self.state.disabled {
                p.text_disabled()
            } else {
                p.text_dim
            },
        );
        if self.state.focused && !self.state.disabled {
            effect::paint_focus(canvas, area, p.border_focus(), theme.radius.sm);
        }
    }
}
