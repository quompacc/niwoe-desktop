const ROOM_IDS: [&str; 9] = [
    "panel-room-1",
    "panel-room-2",
    "panel-room-3",
    "panel-room-4",
    "panel-room-5",
    "panel-room-6",
    "panel-room-7",
    "panel-room-8",
    "panel-room-9",
];

/// Keep the active room visible, with neighbors and a persistent overflow menu.
/// Labels reflect actual workspace slots until persistent room names exist.
fn visible_rooms(active: u8, total: u8, capacity: usize) -> std::ops::RangeInclusive<u8> {
    let total = total.clamp(1, ROOM_IDS.len() as u8);
    let count = capacity.clamp(1, total as usize) as u8;
    let start = active
        .clamp(1, total)
        .saturating_sub(count / 2)
        .max(1)
        .min(total - count + 1);
    start..=start + count - 1
}

struct RoomTab {
    workspace: u8,
    active: bool,
}

impl Widget for RoomTab {
    fn id(&self) -> Option<&'static str> {
        ROOM_IDS
            .get(self.workspace.saturating_sub(1) as usize)
            .copied()
    }

    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            size: UiSize {
                width: ui_length(PanelTokens::DEFAULT.room_width as f32),
                height: ui_length(CHIP_H as f32),
            },
            flex_shrink: 0.0,
            ..Default::default()
        }
    }

    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        paint_panel_control_background(area, canvas, theme, state);
        let color = if self.active {
            theme.palette.accent
        } else {
            theme.palette.text
        };
        let label = format!("Raum {}", self.workspace);
        let (width, text_height) = measure_text(&label, FONT_SIZE);
        let symbol_size = Typography::DEFAULT.caption_size as i32;
        let content_width = symbol_size + theme.spacing.sm + width;
        let content_x = area.x + (area.width - content_width) / 2;
        let symbol = Rect {
            x: content_x,
            y: area.y + (area.height - symbol_size) / 2,
            width: symbol_size,
            height: symbol_size,
        };
        if self.active {
            if let Some(path) = rounded_rect_path(area, niwoe_tokens::Radius::DEFAULT.sm) {
                paint_fill(
                    canvas,
                    &path,
                    Color::rgba(color.r, color.g, color.b, PanelTokens::DEFAULT.active_alpha),
                );
            }
        }
        if let Some(path) = rounded_rect_path(symbol, niwoe_tokens::Radius::DEFAULT.sm) {
            niwoe_ui::effect::paint_border(
                canvas,
                &path,
                color,
                niwoe_tokens::Controls::BORDER as f32,
            );
        }
        paint_text(
            canvas,
            &label,
            content_x + symbol_size + theme.spacing.sm,
            // Fixed "Raum 1..9" labels have no descenders. Center their actual
            // measured ink height rather than subtracting a spacing token.
            area.y + (area.height + text_height) / 2,
            FONT_SIZE,
            theme.palette.text,
        );
        if self.active {
            if let Some(path) = rounded_rect_path(area, niwoe_tokens::Radius::DEFAULT.sm) {
                // Match the mockup's outlined active room, without adding another titlebar.
                niwoe_ui::effect::paint_border(
                    canvas,
                    &path,
                    color,
                    niwoe_tokens::Controls::BORDER as f32,
                );
            }
        }
    }
}

#[cfg(test)]
mod room_tests {
    use super::*;
    #[test]
    fn room_label_ink_is_vertically_centered() {
        let theme = Theme::TOKYO_NIGHT_METRO;
        let area = Rect {
            x: 0,
            y: 0,
            width: PanelTokens::DEFAULT.room_width as i32,
            height: CHIP_H,
        };
        for workspace in 1..=9 {
            let mut image = Pixmap::new(area.width as u32, area.height as u32).unwrap();
            RoomTab {
                workspace,
                active: false,
            }
            .paint(area, &mut image.as_mut(), &theme, WidgetState::Idle);
            let rows: Vec<_> = image
                .data()
                .as_chunks::<4>()
                .0
                .iter()
                .enumerate()
                // Include antialiased ink, excluding the symbol on the left.
                .filter(|(index, pixel)| {
                    pixel[3] > 0 && *index as i32 % area.width > area.width / 2
                })
                .map(|(index, _)| index as i32 / area.width)
                .collect();
            let top = *rows.iter().min().expect("visible text ink");
            let bottom = *rows.iter().max().unwrap();
            assert!(
                (top + bottom + 1 - area.height).abs() <= 2,
                "room {workspace}: ink {top}..{bottom} is not centered"
            );
        }
    }

    #[test]
    fn active_room_stays_visible_at_both_edges_and_in_the_middle() {
        for capacity in [0, 1, 3, 9, 20] {
            for active in 1..=9 {
                let rooms = visible_rooms(active, 9, capacity);
                assert!(rooms.contains(&active));
                assert!(*rooms.start() >= 1 && *rooms.end() <= 9);
                assert!(rooms.count() <= capacity.clamp(1, 9));
            }
        }
    }
}
