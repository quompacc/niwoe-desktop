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
        paint_panel_control_background(area, canvas, state);
        let color = if self.active {
            theme.palette.accent
        } else {
            theme.palette.text_dim
        };
        let label = format!("Raum {}", self.workspace);
        let (width, _) = measure_text(&label, FONT_SIZE);
        paint_text(
            canvas,
            &label,
            area.x + (area.width - width) / 2,
            area.y + (area.height + FONT_SIZE as i32) / 2 - theme.spacing.xs,
            FONT_SIZE,
            color,
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
