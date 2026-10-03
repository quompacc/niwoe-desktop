use super::*;

mod apps;
mod general;
pub(crate) use apps::draw_apps;
pub(super) use general::draw_general;

pub(crate) const TABS: [&str; 5] = [
    "Allgemein",
    "Apps",
    "Dateien",
    "Wiederherstellung",
    "Benachrichtigungen · nicht verfügbar",
];

pub(crate) fn tab_rect(index: usize) -> Rect {
    let size = Typography::DEFAULT.caption_size as f32;
    Rect {
        x: C.sidebar_width
            + C.outer_pad
            + TABS[..index]
                .iter()
                .map(|label| measure_text(label, size).0 + S.xxl + S.xl + C.card_gap)
                .sum::<i32>(),
        y: C.config_header_height + S.sm,
        width: measure_text(TABS[index], size).0 + S.xxl + S.xl,
        height: C.config_tabs_height - S.sm * 2,
    }
}

pub(crate) fn hit_tab(x: i32, y: i32) -> Option<usize> {
    (0..4).find(|&i| contains(tab_rect(i), x, y))
}

pub(crate) fn rect(width: u32, scroll: i32, index: usize) -> Rect {
    let (left, left_width, _, _) = content_bounds(width);
    let top = body_top() - scroll;
    if index == 9 {
        return Rect {
            x: left + C.card_pad,
            y: name_rect(width, scroll).y - S.lg,
            width: S.xxl * 2,
            height: S.xxl * 2,
        };
    }
    let context_top = top + C.config_details_height + C.card_gap;
    let row_height = (C.config_context_height - S.xxl - C.card_pad * 2 - C.card_gap * 2) / 3;
    if (10..=11).contains(&index) || index == 8 {
        let row = if index == 8 { 2 } else { index - 10 };
        return Rect {
            x: left + C.card_pad,
            y: context_top + S.xxl + row as i32 * (row_height + C.card_gap),
            width: left_width - C.card_pad * 2,
            height: row_height,
        };
    }
    let lower_width = (left_width - C.card_gap) / 2;
    Rect {
        x: left
            + if index == 13 {
                lower_width + C.card_gap
            } else {
                0
            }
            + C.card_pad,
        y: context_top + C.config_context_height + C.card_gap + S.xxl,
        width: lower_width - C.card_pad * 2,
        height: C.config_field_height,
    }
}

pub(crate) fn app_rect(width: u32, index: usize) -> Rect {
    let left = C.sidebar_width + C.outer_pad;
    let available = width as i32 - left - C.outer_pad;
    let row = match index {
        14 => 0,
        15 | 16 => 1,
        20..=25 => 2 + (index - 20) / 2,
        _ => 0,
    };
    let half = (available - C.card_gap) / 2;
    Rect {
        x: left
            + if index == 16 || ((20..=25).contains(&index) && index % 2 == 1) {
                half + C.card_gap
            } else {
                0
            },
        y: body_top()
            + S.xxl
            + S.lg
            + if index >= 20 {
                2 * (C.config_field_height + S.md)
                    + (row - 2) as i32 * (Controls::TEXT_PAIR_HEIGHT + S.md)
            } else {
                row as i32 * (C.config_field_height + S.md)
            },
        width: if index == 15 || index == 16 || (20..=25).contains(&index) {
            half
        } else {
            available
        },
        height: if index >= 20 {
            Controls::TEXT_PAIR_HEIGHT
        } else {
            C.config_field_height
        },
    }
}
