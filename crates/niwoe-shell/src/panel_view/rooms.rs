const ROOM_IDS: [&str; niwoe_config::rooms::MAX_ROOMS] = [
    "panel-room-1",
    "panel-room-2",
    "panel-room-3",
    "panel-room-4",
    "panel-room-5",
    "panel-room-6",
    "panel-room-7",
    "panel-room-8",
    "panel-room-9",
    "panel-room-10",
    "panel-room-11",
    "panel-room-12",
    "panel-room-13",
    "panel-room-14",
    "panel-room-15",
    "panel-room-16",
    "panel-room-17",
    "panel-room-18",
    "panel-room-19",
    "panel-room-20",
    "panel-room-21",
    "panel-room-22",
    "panel-room-23",
    "panel-room-24",
    "panel-room-25",
    "panel-room-26",
    "panel-room-27",
    "panel-room-28",
    "panel-room-29",
    "panel-room-30",
    "panel-room-31",
    "panel-room-32",
    "panel-room-33",
    "panel-room-34",
    "panel-room-35",
    "panel-room-36",
    "panel-room-37",
    "panel-room-38",
    "panel-room-39",
    "panel-room-40",
    "panel-room-41",
    "panel-room-42",
    "panel-room-43",
    "panel-room-44",
    "panel-room-45",
    "panel-room-46",
    "panel-room-47",
    "panel-room-48",
    "panel-room-49",
    "panel-room-50",
    "panel-room-51",
    "panel-room-52",
    "panel-room-53",
    "panel-room-54",
    "panel-room-55",
    "panel-room-56",
    "panel-room-57",
    "panel-room-58",
    "panel-room-59",
    "panel-room-60",
    "panel-room-61",
    "panel-room-62",
    "panel-room-63",
    "panel-room-64",
];

/// Keep the panel rail stable: activation must never reorder or scroll room tabs.
/// Rooms beyond the available width remain reachable through the overflow control.
fn visible_rooms(total: u8, capacity: usize) -> std::ops::RangeInclusive<u8> {
    let total = total.clamp(1, ROOM_IDS.len() as u8);
    let count = capacity.clamp(1, total as usize) as u8;
    1..=count
}

struct RoomTab {
    workspace: u8,
    label: String,
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
        let mut label = self.label.clone();
        let limit = area.width
            - 2 * theme.spacing.sm
            - Typography::DEFAULT.caption_size as i32
            - theme.spacing.sm;
        if measure_text(&label, FONT_SIZE).0 > limit {
            while !label.is_empty() && measure_text(&format!("{label}…"), FONT_SIZE).0 > limit {
                label.pop();
            }
            label.push('…');
        }
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
                label: format!("Raum {workspace}"),
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
    fn visible_room_sequence_is_stable_and_bounded() {
        for capacity in [0, 1, 3, 9, 20] {
            let rooms = visible_rooms(9, capacity);
            assert_eq!(*rooms.start(), 1);
            assert!(*rooms.end() <= 9);
            assert_eq!(rooms.count(), capacity.clamp(1, 9));
        }
    }
}
