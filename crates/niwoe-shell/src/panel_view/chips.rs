struct PanelDivider;

impl Widget for PanelDivider {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(DIVIDER_W as f32),
                height: ui_length(CHIP_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _state: WidgetState) {
        let pal = theme.palette;
        let col = Color::rgba(pal.text.r, pal.text.g, pal.text.b, SEGMENT_DIVIDER_OPACITY);
        let line_height = theme.spacing.md * 2 + ACCENT_LINE_H;
        let line = Rect {
            x: area.x + DIVIDER_W / 2,
            y: area.y + (area.height - line_height) / 2,
            width: 1,
            height: line_height,
        };
        if let Some(path) = rounded_rect_path(line, 0) {
            paint_fill(canvas, &path, col);
        }
    }
}

struct PanelChip {
    id: &'static str,
    label: Box<str>,
    icon: Option<Pixmap>,
    width: i32,
    active: bool,
}

struct PanelStatusIcon {
    show_label: bool,
    label: Box<str>,
    icon: Option<Pixmap>,
}

impl Widget for PanelStatusIcon {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length((TRAY_W + if self.show_label { LAUNCHER_W } else { 0 }) as f32),
                height: ui_length(CHIP_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _state: WidgetState) {
        if let Some(ref icon) = self.icon {
            let x = area.x + (TRAY_W - icon.width() as i32) / 2;
            let y = area.y + (area.height - icon.height() as i32) / 2;
            canvas.draw_pixmap(
                x,
                y,
                icon.as_ref(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
        } else {
            let (text_w, _) = measure_text(&self.label, FONT_SIZE);
            paint_text(
                canvas,
                &self.label,
                area.x + (area.width - text_w) / 2,
                area.y + area.height / 2 + theme.spacing.xs,
                FONT_SIZE,
                theme.palette.text,
            );
        }
        if self.show_label {
            paint_text(
                canvas,
                &self.label,
                area.x + TRAY_W,
                area.y + area.height / 2 + theme.spacing.xs,
                FONT_SIZE,
                theme.palette.text,
            );
        }
    }
}

struct PanelStatusGroup {
    active: bool,
    children: Vec<Box<dyn Widget>>,
}

impl Widget for PanelStatusGroup {
    fn id(&self) -> Option<&'static str> {
        Some("panel-status")
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            align_items: Some(AlignItems::Center),
            size: UiSize {
                width: ui_length(
                    (TRAY_W * self.children.len() as i32
                        + if self.children.len() == 3 {
                            LAUNCHER_W
                        } else {
                            0
                        }) as f32,
                ),
                height: ui_length(CHIP_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let highlight = if self.active {
            Some(Interaction::DEFAULT.accent_idle(theme.palette.accent))
        } else {
            match state {
                WidgetState::Idle => None,
                WidgetState::Hovered => Some(Interaction::DEFAULT.neutral_hover),
                WidgetState::Pressed => Some(Interaction::DEFAULT.neutral_pressed),
            }
        };
        if let Some(color) = highlight {
            if let Some(path) = rounded_rect_path(area, CHIP_HL_RADIUS) {
                paint_fill(canvas, &path, color);
            }
        }
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
}

impl PanelChip {
    fn new(
        id: &'static str,
        label: Box<str>,
        icon: Option<Pixmap>,
        width: i32,
        active: bool,
    ) -> Self {
        Self {
            id,
            label,
            icon,
            width,
            active,
        }
    }
}

impl Widget for PanelChip {
    fn id(&self) -> Option<&'static str> {
        Some(self.id)
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(self.width as f32),
                height: ui_length(CHIP_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let highlight = if self.active {
            Some(Interaction::DEFAULT.accent_idle(theme.palette.accent))
        } else {
            match state {
                WidgetState::Idle => None,
                WidgetState::Hovered => Some(Interaction::DEFAULT.neutral_hover),
                WidgetState::Pressed => Some(Interaction::DEFAULT.neutral_pressed),
            }
        };
        if let Some(color) = highlight {
            if let Some(ref path) = rounded_rect_path(area, CHIP_HL_RADIUS) {
                paint_fill(canvas, path, color);
            }
        }

        if let Some(ref icon) = self.icon {
            let iw = icon.width() as i32;
            let ih = icon.height() as i32;
            let x = area.x + (area.width - iw) / 2;
            let y = area.y + (area.height - ih) / 2;
            canvas.draw_pixmap(
                x,
                y,
                icon.as_ref(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
        } else {
            let (text_w, _) = measure_text(&self.label, FONT_SIZE);
            let tx = area.x + (area.width - text_w) / 2;
            let ty = area.y + area.height / 2 + theme.spacing.xs;
            paint_text(canvas, &self.label, tx, ty, FONT_SIZE, theme.palette.text);
        }
    }
}

struct PanelWorkspaceChip {
    hidden_count: u8,
    active_hidden: bool,
}

impl Widget for PanelWorkspaceChip {
    fn id(&self) -> Option<&'static str> {
        Some("panel-workspace")
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(WS_W as f32),
                height: ui_length(CHIP_H as f32),
            },
            flex_shrink: 0.0,
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        paint_panel_control_background(area, canvas, theme, state);
        let label = format!("+{}", self.hidden_count);
        let (width, _) = measure_text(&label, FONT_SIZE);
        if self.active_hidden {
            if let Some(path) = rounded_rect_path(area, niwoe_tokens::Radius::DEFAULT.sm) {
                paint_fill(
                    canvas,
                    &path,
                    Color::rgba(
                        theme.palette.accent.r,
                        theme.palette.accent.g,
                        theme.palette.accent.b,
                        PanelTokens::DEFAULT.active_alpha,
                    ),
                );
                niwoe_ui::effect::paint_border(
                    canvas,
                    &path,
                    theme.palette.accent,
                    niwoe_tokens::Controls::BORDER as f32,
                );
            }
        }
        paint_text(
            canvas,
            &label,
            area.x + (area.width - width) / 2,
            area.y + area.height / 2 + theme.spacing.xs,
            FONT_SIZE,
            if self.active_hidden {
                theme.palette.accent
            } else {
                theme.palette.text
            },
        );
    }
}
struct PanelClockChip {
    value: Box<str>,
}

impl Widget for PanelClockChip {
    fn id(&self) -> Option<&'static str> {
        Some("panel-clock")
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(CLOCK_W as f32),
                height: ui_length(CHIP_H as f32),
            },
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        paint_panel_control_background(area, canvas, theme, state);
        let (time, date) = self.value.split_once("  ").unwrap_or((&self.value, ""));
        let time_size = Typography::DEFAULT.body_size as f32;
        let (time_w, time_height) = measure_text(time, time_size);
        let (date_w, _) = measure_text(date, FONT_SIZE);
        let gap = if date.is_empty() { 0 } else { theme.spacing.lg };
        let x = area.x + (area.width - time_w - date_w - gap) / 2;
        let baseline = area.y + area.height / 2 + theme.spacing.xs;
        paint_text(canvas, date, x, baseline, FONT_SIZE, theme.palette.text);
        paint_text(
            canvas,
            time,
            x + date_w + gap,
            area.y + (area.height + time_height) / 2,
            time_size,
            theme.palette.text,
        );
    }
}

fn paint_panel_control_background(
    area: Rect,
    canvas: &mut PixmapMut<'_>,
    theme: &Theme,
    state: WidgetState,
) {
    let color = match state {
        WidgetState::Idle => None,
        WidgetState::Hovered => Some(Color::rgba(
            theme.palette.surface_alt.r,
            theme.palette.surface_alt.g,
            theme.palette.surface_alt.b,
            PanelTokens::DEFAULT.hover_alpha,
        )),
        WidgetState::Pressed => Some(Color::rgba(
            theme.palette.surface_alt.r,
            theme.palette.surface_alt.g,
            theme.palette.surface_alt.b,
            PanelTokens::DEFAULT.pressed_alpha,
        )),
    };
    if let Some(color) = color {
        if let Some(path) = rounded_rect_path(area, CHIP_HL_RADIUS) {
            paint_fill(canvas, &path, color);
        }
    }
}

// PanelPinnedChip
