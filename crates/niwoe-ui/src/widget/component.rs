//! Shared native component family. Uses the existing Widget and paint pipeline.
//! Paint only runs on caller invalidation; no timers, I/O or persistent caches.
//! Paths are bounded by the component rectangle, glyphs use the existing font cache.

use niwoe_tokens::{Controls, Interaction, Typography};
use taffy::prelude::{length, Size, Style};
use tiny_skia::PixmapMut;

use super::Widget;
use crate::{
    effect::{paint_border, paint_fill, paint_text, rounded_rect_path, truncate_to_fit},
    event::WidgetState,
    paint::Rect,
    style::Theme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Surface,
    Text,
    Button,
    Tab,
    Chip,
    Card,
    Input,
    Slider,
}

/// Orthogonal flags preserve focus/error while a pointer hovers or presses.
#[derive(Debug, Clone, Copy, Default)]
pub struct ComponentState {
    pub focused: bool,
    pub disabled: bool,
    pub error: bool,
    pub selected: bool,
}

pub struct Component<'a> {
    pub kind: ComponentKind,
    pub label: &'a str,
    pub width: i32,
    pub state: ComponentState,
    pub value: f32,
}

impl<'a> Component<'a> {
    pub fn new(kind: ComponentKind, label: &'a str, width: i32) -> Self {
        Self {
            kind,
            label,
            width,
            state: ComponentState::default(),
            value: 0.5,
        }
    }

    pub fn height(&self) -> i32 {
        match self.kind {
            ComponentKind::Card => Controls::CARD_HEIGHT,
            ComponentKind::Input => Controls::FORM_HEIGHT,
            _ => Controls::MIN_HEIGHT,
        }
    }

    /// Use from event dispatch as well as rendering: disabled controls cannot
    /// become keyboard targets or trigger actions.
    pub fn accepts_input(&self) -> bool {
        !self.state.disabled && !matches!(self.kind, ComponentKind::Surface | ComponentKind::Text)
    }
}

fn inset(area: Rect, pad: i32) -> Rect {
    Rect {
        x: area.x + pad,
        y: area.y + pad,
        width: (area.width - pad * 2).max(0),
        height: (area.height - pad * 2).max(0),
    }
}

impl Widget for Component<'_> {
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: length(self.width as f32),
                height: length(self.height() as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, pointer: WidgetState) {
        self.paint_scaled(area, canvas, theme, pointer, 1.0);
    }
}

impl Component<'_> {
    /// Rasterize at the output scale; glyphs are rendered at physical size,
    /// never enlarged from a low-resolution intermediate.
    pub fn paint_scaled(
        &self,
        area: Rect,
        canvas: &mut PixmapMut<'_>,
        theme: &Theme,
        pointer: WidgetState,
        scale: f32,
    ) {
        if !scale.is_finite() || scale <= 0.0 {
            return;
        }
        let px = |v: i32| (v as f32 * scale).round() as i32;
        let area = Rect {
            x: px(area.x),
            y: px(area.y),
            width: px(area.width),
            height: px(area.height),
        };
        let mut theme = *theme;
        theme.spacing.sm = px(theme.spacing.sm);
        theme.spacing.lg = px(theme.spacing.lg);
        theme.radius.sm = px(theme.radius.sm);
        theme.radius.md = px(theme.radius.md);
        theme.radius.lg = px(theme.radius.lg);
        let pal = theme.palette;
        let interactive = self.accepts_input();
        let selected = self.state.selected && interactive;
        let mut bg = if selected && self.kind == ComponentKind::Button {
            pal.accent
        } else {
            pal.surface_raised()
        };
        if interactive {
            bg = match pointer {
                WidgetState::Hovered => Interaction::DEFAULT.hover(bg),
                WidgetState::Pressed => Interaction::DEFAULT.pressed(bg),
                WidgetState::Idle => bg,
            };
        }
        let fg = if self.state.disabled {
            pal.text_disabled()
        } else if selected && self.kind == ComponentKind::Button {
            niwoe_tokens::contrast_text(bg)
        } else {
            pal.text
        };
        let outline = if self.state.error {
            pal.error
        } else if selected {
            pal.accent
        } else if interactive {
            pal.border_control()
        } else {
            pal.border_subtle()
        };
        let body = inset(area, px(Controls::BORDER));
        let radius = if self.kind == ComponentKind::Card {
            theme.radius.lg
        } else {
            theme.radius.sm
        };
        if self.kind != ComponentKind::Text {
            if let Some(path) = rounded_rect_path(body, radius) {
                paint_fill(canvas, &path, bg);
                paint_border(canvas, &path, outline, px(Controls::BORDER) as f32);
            }
        }
        if self.state.focused && interactive {
            if let Some(path) = rounded_rect_path(inset(area, px(Controls::FOCUS_INSET)), radius) {
                paint_border(
                    canvas,
                    &path,
                    if selected && self.kind == ComponentKind::Button {
                        fg
                    } else {
                        pal.border_focus()
                    },
                    px(Controls::FOCUS_WIDTH) as f32,
                );
            }
        }
        if self.kind == ComponentKind::Slider {
            let track = Rect {
                x: area.x + theme.spacing.lg,
                y: area.y + (area.height - px(Controls::TRACK_HEIGHT)) / 2,
                width: (area.width - theme.spacing.lg * 2).max(0),
                height: px(Controls::TRACK_HEIGHT),
            };
            if let Some(path) = rounded_rect_path(track, theme.radius.sm) {
                paint_fill(canvas, &path, pal.border_control());
            }
            let thumb = Rect {
                x: track.x
                    + ((track.width - px(Controls::THUMB_SIZE)).max(0) as f32
                        * self.value.clamp(0.0, 1.0)) as i32,
                y: area.y + (area.height - px(Controls::THUMB_SIZE)) / 2,
                width: px(Controls::THUMB_SIZE),
                height: px(Controls::THUMB_SIZE),
            };
            if let Some(path) = rounded_rect_path(thumb, theme.radius.md) {
                paint_fill(
                    canvas,
                    &path,
                    if self.state.disabled {
                        pal.text_disabled()
                    } else {
                        pal.accent
                    },
                );
            }
            return;
        }
        let size = Typography::DEFAULT.body_size as f32 * scale;
        let label = truncate_to_fit(self.label, area.width - theme.spacing.lg * 2, size);
        let (ascent, descent) = crate::effect::ui_line_metrics(size);
        let baseline = area.y + ((area.height as f32 + ascent + descent) / 2.0).round() as i32;
        paint_text(
            canvas,
            &label,
            area.x + theme.spacing.lg,
            baseline,
            size,
            fg,
        );
        if selected
            && matches!(
                self.kind,
                ComponentKind::Tab | ComponentKind::Chip | ComponentKind::Card
            )
        {
            let mark = Rect {
                x: body.x + theme.spacing.sm,
                y: body.y + body.height - px(Controls::FOCUS_INSET),
                width: body.width - theme.spacing.sm * 2,
                height: px(Controls::FOCUS_WIDTH),
            };
            if let Some(path) = rounded_rect_path(mark, theme.radius.none) {
                paint_fill(canvas, &path, pal.accent);
            }
        }
    }
}
