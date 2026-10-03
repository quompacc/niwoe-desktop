use niwoe_ui::{Widget, WidgetPath};

/// Derive keyboard targets from the same layout and IDs as pointer dispatch.
/// Only fully visible actionable widgets can take focus; clipped/offscreen rows
/// and disabled controls without an action ID cannot activate by keyboard.
pub(crate) fn focus_targets(
    root: &dyn Widget,
    layout: &niwoe_ui::paint::LayoutTree,
    viewport: niwoe_ui::PixelSize,
) -> Vec<(&'static str, niwoe_ui::Rect)> {
    let mut targets = Vec::new();
    collect_targets(
        root,
        &layout.root,
        (0, 0),
        niwoe_ui::Rect {
            x: 0,
            y: 0,
            width: viewport.width as i32,
            height: viewport.height as i32,
        },
        &mut targets,
    );
    targets
}

fn collect_targets(
    widget: &dyn Widget,
    node: &niwoe_ui::paint::LayoutNode,
    parent: (i32, i32),
    clip: niwoe_ui::Rect,
    targets: &mut Vec<(&'static str, niwoe_ui::Rect)>,
) {
    let area = niwoe_ui::Rect {
        x: parent.0 + node.rect.x,
        y: parent.1 + node.rect.y,
        ..node.rect
    };
    let left = area.x.max(clip.x);
    let top = area.y.max(clip.y);
    let right = (area.x + area.width).min(clip.x + clip.width);
    let bottom = (area.y + area.height).min(clip.y + clip.height);
    if left >= right || top >= bottom {
        return;
    }
    if area.x == left
        && area.y == top
        && area.x + area.width == right
        && area.y + area.height == bottom
    {
        if let Some(id) = widget
            .id()
            .filter(|id| crate::widget_action::action_for_id(id).is_some())
        {
            targets.push((id, area));
        }
    }
    let clip = niwoe_ui::Rect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    };
    for (child, layout) in widget.children().iter().zip(&node.children) {
        collect_targets(child.as_ref(), layout, (area.x, area.y), clip, targets);
    }
}

pub(crate) fn find_widget_at_path<'a>(
    root: &'a dyn Widget,
    path: &WidgetPath,
) -> Option<&'a dyn Widget> {
    let mut current = root;
    for &index in path.as_slice() {
        let children = current.children();
        if index >= children.len() {
            return None;
        }
        current = &*children[index];
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use niwoe_ui::{
        style::Palette,
        widget::{Button, Container},
        Widget, WidgetPath,
    };

    use super::find_widget_at_path;

    #[test]
    fn keyboard_targets_match_pointer_hits_and_exclude_disabled_or_clipped_rows() {
        struct Row(Option<&'static str>);
        impl Widget for Row {
            fn id(&self) -> Option<&'static str> {
                self.0
            }
            fn style(&self) -> niwoe_ui::WidgetStyle {
                niwoe_ui::WidgetStyle {
                    flex_shrink: 0.0,
                    size: niwoe_ui::UiSize {
                        width: niwoe_ui::ui_length(100.0_f32),
                        height: niwoe_ui::ui_length(32.0_f32),
                    },
                    ..Default::default()
                }
            }
            fn paint(
                &self,
                _: niwoe_ui::Rect,
                _: &mut tiny_skia::PixmapMut<'_>,
                _: &niwoe_ui::Theme,
                _: niwoe_ui::WidgetState,
            ) {
            }
        }
        let viewport = niwoe_ui::PixelSize {
            width: 120,
            height: 64,
        };
        let root = Container::new(
            niwoe_ui::WidgetStyle {
                flex_direction: niwoe_ui::FlexDirection::Column,
                size: niwoe_ui::UiSize {
                    width: niwoe_ui::ui_length(120.0_f32),
                    height: niwoe_ui::ui_length(64.0_f32),
                },
                ..Default::default()
            },
            vec![
                Box::new(Row(Some("power-off"))),
                Box::new(Row(None)),
                Box::new(Row(Some("power-logout"))),
            ],
        );
        let layout = niwoe_ui::compute_layout(&root, viewport).unwrap();
        let targets = super::focus_targets(&root, &layout, viewport);
        assert_eq!(targets.len(), 1);
        for (id, area) in targets {
            let path = niwoe_ui::hit_test(
                &layout,
                niwoe_ui::PointerPosition {
                    x: area.x + area.width / 2,
                    y: area.y + area.height / 2,
                },
            )
            .unwrap();
            assert_eq!(
                find_widget_at_path(&root, &path).and_then(|widget| widget.id()),
                Some(id)
            );
        }
    }

    #[test]
    fn find_widget_at_path_empty_returns_root() {
        let root = Container::leaf(Default::default());
        let path = WidgetPath::from_vec(vec![]);
        let found = find_widget_at_path(&root, &path);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), None);
    }

    #[test]
    fn find_widget_at_path_out_of_bounds_returns_none() {
        let root = Container::leaf(Default::default());
        let path = WidgetPath::from_vec(vec![0]);
        let found = find_widget_at_path(&root, &path);
        assert!(found.is_none());
    }

    #[test]
    fn find_widget_at_path_smoke_finds_apps_switch() {
        let pal = Palette::TOKYO_NIGHT_METRO;
        let footer_left = vec![
            Box::new(Button::with_id("apps-switch", "Apps", pal.accent, 144, 48))
                as Box<dyn Widget>,
        ];
        let footer_right = vec![
            Box::new(Button::with_id("power-off", "Off", pal.error, 48, 48)) as Box<dyn Widget>,
        ];
        let footer = Container::footer_row(880, 56, 28, 8, footer_left, footer_right);
        let root = Container::centered_viewport(480, 360, vec![Box::new(footer)]);
        // Tree: root (centered_viewport) -> footer (footer_row) -> left_cluster -> apps-switch button
        // path: [0, 0, 0]
        let path = WidgetPath::from_vec(vec![0, 0, 0]);
        let found = find_widget_at_path(&root, &path);
        assert!(found.is_some());
        assert_eq!(found.and_then(|w| w.id()), Some("apps-switch"));
    }
}
