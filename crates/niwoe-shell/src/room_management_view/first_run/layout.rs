//! Stable action indices with separate page, navigation and exit groups.
use super::*;

pub(super) const W: niwoe_tokens::FirstRun = niwoe_tokens::FirstRun::DEFAULT;
pub(crate) fn content(width: u32) -> Rect {
    let available = (width as i32 - C.outer_pad * 2).max(0);
    let width = available.min(W.content_max_width);
    Rect {
        x: (available - width) / 2 + C.outer_pad,
        y: 0,
        width,
        height: 0,
    }
}
pub(super) fn column(width: u32, right: bool) -> Rect {
    let body = content(width);
    let cell = (body.width - W.column_gap) / 2;
    Rect {
        x: body.x + if right { cell + W.column_gap } else { 0 },
        width: cell,
        ..body
    }
}
pub(super) fn body_top() -> i32 {
    C.header_height + S.xxl + S.lg
}
pub(super) fn row_y(row: usize) -> i32 {
    body_top() + row as i32 * (C.panel_module_height + C.card_gap)
}
pub(super) fn preview_y() -> i32 {
    row_y(3) + C.panel_module_height + S.xxl + S.md
}

pub(crate) fn control(width: u32, height: u32, index: usize, count: usize, step: u8) -> Rect {
    let base = count - 5;
    let body = content(width);
    let right = body.x + body.width;
    if index >= base {
        let y = height as i32 - C.config_footer_height + C.card_gap;
        let primary_x = right - W.primary_width;
        let back_x = primary_x - C.card_gap - C.config_action_width;
        return match index - base {
            0 => Rect {
                x: back_x,
                y,
                width: C.config_action_width,
                height: C.config_field_height,
            },
            1 => Rect {
                x: primary_x,
                y,
                width: W.primary_width,
                height: C.config_field_height,
            },
            2 => Rect {
                x: back_x - C.card_gap - C.config_action_width,
                y,
                width: C.config_action_width,
                height: C.config_field_height,
            },
            3 => Rect {
                x: body.x,
                y,
                width: W.exit_width,
                height: C.config_field_height,
            },
            _ => Rect {
                x: right - W.exit_width,
                y: C.outer_pad,
                width: W.exit_width,
                height: C.config_field_height,
            },
        };
    }
    if step == 3 {
        let col = index % 3;
        let main = body.width - C.config_action_width * 2 - C.card_gap * 2;
        return Rect {
            x: body.x
                + match col {
                    0 => 0,
                    1 => main + C.card_gap,
                    _ => main + C.config_action_width + C.card_gap * 2,
                },
            y: row_y(index / 3),
            width: if col == 0 {
                main
            } else {
                C.config_action_width
            },
            height: C.panel_module_height,
        };
    }
    if step == 2 {
        let side = column(width, index >= 3);
        if index < 3 {
            return Rect {
                y: row_y(index),
                height: C.panel_module_height,
                ..side
            };
        }
        if index == 3 {
            return Rect {
                y: body_top(),
                height: C.config_field_height,
                ..side
            };
        }
        if index >= 6 {
            return Rect {
                y: body_top()
                    + C.config_field_height
                    + C.card_gap
                    + (index - 6) as i32 * (C.panel_module_height + C.card_gap),
                height: C.panel_module_height,
                ..side
            };
        }
        let cell = (side.width - C.card_gap) / 2;
        return Rect {
            x: side.x + if index == 5 { cell + C.card_gap } else { 0 },
            y: body_top()
                + C.config_field_height
                + C.card_gap
                + 4 * (C.panel_module_height + C.card_gap),
            width: cell,
            height: C.config_field_height,
        };
    }
    Rect {
        y: row_y(index),
        height: C.panel_module_height,
        ..body
    }
}
pub(crate) fn hit(x: i32, y: i32, width: u32, height: u32, wizard: &Wizard) -> Option<usize> {
    let count = wizard.control_count();
    (0..count).find(|&i| {
        wizard.enabled(i) && contains(control(width, height, i, count, wizard.draft.step), x, y)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_five_pages_fit_without_overlapping_action_regions() {
        for (width, height) in [(1366, 720), (1920, 1032), (3840, 2112)] {
            let mut wizard = Wizard {
                state: Some(niwoe_ipc::FirstRunSnapshot {
                    revision: 0,
                    fresh: false,
                    completed: false,
                    applying: false,
                    draft: None,
                }),
                ..Default::default()
            };
            wizard.apps = (0..4)
                .map(|i| {
                    (
                        format!("App {i}"),
                        niwoe_ipc::AppReference::Native(format!("own-{i}")),
                    )
                })
                .collect();
            for step in 0..=4 {
                wizard.draft.step = step;
                let count = wizard.control_count();
                let areas: Vec<_> = (0..count)
                    .map(|i| control(width, height, i, count, step))
                    .collect();
                for (i, area) in areas.iter().enumerate() {
                    assert!(area.x >= 0 && area.x + area.width <= width as i32);
                    assert!(area.y >= 0 && area.y + area.height <= height as i32);
                    assert!(area.width > 0 && area.height >= Controls::MIN_HEIGHT);
                    for other in &areas[i + 1..] {
                        assert!(
                            area.x + area.width <= other.x
                                || other.x + other.width <= area.x
                                || area.y + area.height <= other.y
                                || other.y + other.height <= area.y
                        );
                    }
                    assert_eq!(
                        hit(
                            area.x + area.width / 2,
                            area.y + area.height / 2,
                            width,
                            height,
                            &wizard
                        ),
                        wizard.enabled(i).then_some(i)
                    );
                }
            }
            assert!(
                preview_y() + crate::PANEL_SURFACE_HEIGHT as i32 + S.xl
                    < height as i32 - C.config_footer_height - S.xl
            );
        }
    }
    #[test]
    fn missing_state_keeps_only_safe_exit_and_reload_hittable() {
        let wizard = Wizard::default();
        let count = wizard.control_count();
        for i in 0..count {
            let area = control(1366, 720, i, count, 0);
            assert_eq!(
                hit(
                    area.x + area.width / 2,
                    area.y + area.height / 2,
                    1366,
                    720,
                    &wizard
                ),
                [count - 2, count - 1].contains(&i).then_some(i)
            );
        }
    }
}
