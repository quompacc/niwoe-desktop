use niwoe_config::layouts::Geometry;
use smithay::{
    reexports::wayland_protocols::xdg::shell::server::xdg_toplevel,
    utils::{Logical, Rectangle},
    wayland::seat::WaylandFocus,
};

use super::super::{normal_window_workarea_from_output_geometry, NiwoeState};

fn fit_client(
    rect: Rectangle<i32, Logical>,
    area: Rectangle<i32, Logical>,
    (left, top, right, bottom): (i32, i32, i32, i32),
) -> Rectangle<i32, Logical> {
    let frame = Geometry {
        x: rect.loc.x - left,
        y: rect.loc.y - top,
        width: rect.size.w + left + right,
        height: rect.size.h + top + bottom,
    }
    .fit(&Geometry {
        x: area.loc.x,
        y: area.loc.y,
        width: area.size.w,
        height: area.size.h,
    });
    Rectangle::new(
        (frame.x + left, frame.y + top).into(),
        (
            (frame.width - left - right).max(1),
            (frame.height - top - bottom).max(1),
        )
            .into(),
    )
}

impl NiwoeState {
    /// Output changes only: bound normal frames to surviving workareas without
    /// changing focus/stacking. No timer or per-frame placement work.
    pub(crate) fn fit_normal_windows_to_live_workareas(&mut self) {
        let outputs = self.output_registry.list().to_vec();
        let Some(fallback) = outputs
            .iter()
            .find(|o| o.primary)
            .or_else(|| outputs.first())
        else {
            return;
        };
        for workspace in 0..self.workspaces.count() {
            let windows: Vec<_> = self
                .workspaces
                .space_at(workspace)
                .elements()
                .cloned()
                .collect();
            for window in windows {
                if window.x11_surface().is_some_and(|x| {
                    x.is_override_redirect() || x.is_fullscreen() || x.is_maximized()
                }) {
                    continue;
                }
                if window.toplevel().is_some_and(|t| {
                    t.with_pending_state(|s| {
                        s.states.contains(xdg_toplevel::State::Fullscreen)
                            || s.states.contains(xdg_toplevel::State::Maximized)
                    })
                }) {
                    continue;
                }
                let Some(loc) = self
                    .workspaces
                    .space_at(workspace)
                    .element_location(&window)
                else {
                    continue;
                };
                let Some(surface) = window.wl_surface() else {
                    continue;
                };
                let rect = Rectangle::new(loc, window.geometry().size);
                if rect.size.w <= 0 || rect.size.h <= 0 {
                    continue;
                }
                let output = outputs
                    .iter()
                    .filter_map(|o| {
                        let bounds = Rectangle::new(
                            (o.geometry.x, o.geometry.y).into(),
                            (o.geometry.width, o.geometry.height).into(),
                        );
                        rect.intersection(bounds).map(|intersection| {
                            (
                                o,
                                i64::from(intersection.size.w) * i64::from(intersection.size.h),
                            )
                        })
                    })
                    .max_by_key(|(_, area)| *area)
                    .map(|(o, _)| o)
                    .unwrap_or(fallback);
                let area = normal_window_workarea_from_output_geometry(output.geometry);
                let insets = self
                    .decoration_manager
                    .decoration_inset(&surface, &self.theme_manager.current().config.decorations);
                let fitted = fit_client(
                    rect,
                    Rectangle::new((area.x, area.y).into(), (area.width, area.height).into()),
                    insets,
                );
                if fitted == rect {
                    continue;
                }
                if let Some(toplevel) = window.toplevel() {
                    if fitted.size != rect.size {
                        toplevel.with_pending_state(|s| s.size = Some(fitted.size));
                        toplevel.send_pending_configure();
                    }
                } else if let Some(x11) = window.x11_surface() {
                    if let Err(error) = x11.configure(fitted) {
                        tracing::warn!(%error, "failed to fit X11 window after output change");
                        continue;
                    }
                }
                self.workspaces
                    .space_at_mut(workspace)
                    .relocate_element(&window, fitted.loc);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversized_frame_fits_below_panel_with_all_decorations() {
        let area = Rectangle::new((0, 48).into(), (1920, 1032).into());
        let rect = Rectangle::new((0, 0).into(), (3832, 2104).into());
        assert_eq!(
            fit_client(rect, area, (2, 34, 2, 2)),
            Rectangle::new((2, 82).into(), (1916, 996).into())
        );
    }

    #[test]
    fn visible_frame_keeps_its_geometry_on_negative_origin_output() {
        let area = Rectangle::new((-1920, 48).into(), (1920, 1032).into());
        let rect = Rectangle::new((-1700, 100).into(), (620, 400).into());
        assert_eq!(fit_client(rect, area, (2, 34, 2, 2)), rect);
    }

    #[test]
    fn partially_stranded_window_keeps_size_but_recovers_controls() {
        let area = Rectangle::new((0, 48).into(), (1920, 1032).into());
        let rect = Rectangle::new((1800, 900).into(), (620, 400).into());
        assert_eq!(
            fit_client(rect, area, (0, 0, 0, 0)),
            Rectangle::new((1300, 680).into(), (620, 400).into())
        );
    }
}
