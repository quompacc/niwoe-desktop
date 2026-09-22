// guard:allow-file: vector icon artwork uses a normalized 24-unit coordinate grid; colors and raster size come from tokens.
use super::*;
use std::cell::RefCell;
use tiny_skia::{LineCap, LineJoin, Paint, PathBuilder, Stroke};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Symbol {
    Wired(bool),
    Wifi(bool),
    Audio(bool),
    Battery(u8, bool),
}

type Entry = (Symbol, Color, Pixmap);
thread_local! { static CACHE: RefCell<Vec<Entry>> = const { RefCell::new(Vec::new()) }; }

/// Bounded, theme-keyed raster cache. No icon decoding or rerasterization on idle redraws.
pub(super) fn icon(symbol: Symbol, color: Color) -> Option<Pixmap> {
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, _, pm)) = cache.iter().find(|(s, c, _)| *s == symbol && *c == color) {
            return Some(pm.clone());
        }
        let mut pm = Pixmap::new(STATUS_ICON_SIZE, STATUS_ICON_SIZE)?;
        let mut path = PathBuilder::new();
        match symbol {
            Symbol::Wired(connected) => {
                path.move_to(3., 4.);
                path.line_to(21., 4.);
                path.line_to(21., 16.);
                path.line_to(3., 16.);
                path.close();
                path.move_to(12., 16.);
                path.line_to(12., 20.);
                path.move_to(8., 20.);
                path.line_to(16., 20.);
                if !connected {
                    path.move_to(8., 8.);
                    path.line_to(16., 12.);
                    path.move_to(16., 8.);
                    path.line_to(8., 12.);
                }
            }
            Symbol::Wifi(connected) => {
                path.move_to(2., 8.);
                path.quad_to(12., 0., 22., 8.);
                path.move_to(5., 12.);
                path.quad_to(12., 6., 19., 12.);
                path.move_to(8., 16.);
                path.quad_to(12., 12., 16., 16.);
                path.move_to(12., 20.);
                path.line_to(12., 20.1);
                if !connected {
                    path.move_to(3., 3.);
                    path.line_to(21., 21.);
                }
            }
            Symbol::Audio(muted) => {
                path.move_to(3., 9.);
                path.line_to(7., 9.);
                path.line_to(12., 5.);
                path.line_to(12., 19.);
                path.line_to(7., 15.);
                path.line_to(3., 15.);
                path.close();
                if muted {
                    path.move_to(17., 9.);
                    path.line_to(22., 15.);
                    path.move_to(22., 9.);
                    path.line_to(17., 15.);
                } else {
                    path.move_to(16., 8.);
                    path.quad_to(20., 12., 16., 16.);
                    path.move_to(19., 5.);
                    path.quad_to(25., 12., 19., 19.);
                }
            }
            Symbol::Battery(level, charging) => {
                path.move_to(2., 6.);
                path.line_to(20., 6.);
                path.line_to(20., 18.);
                path.line_to(2., 18.);
                path.close();
                path.move_to(23., 10.);
                path.line_to(23., 14.);
                if charging {
                    path.move_to(12., 8.);
                    path.line_to(9., 12.);
                    path.line_to(13., 12.);
                    path.line_to(10., 16.);
                } else {
                    for x in 0..level {
                        path.move_to(5. + f32::from(x) * 3., 9.);
                        path.line_to(5. + f32::from(x) * 3., 15.);
                    }
                }
            }
        }
        let mut paint = Paint {
            anti_alias: true,
            ..Paint::default()
        };
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        let stroke = Stroke {
            width: 1.5,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..Stroke::default()
        };
        pm.stroke_path(
            &path.finish()?,
            &paint,
            &stroke,
            Transform::from_scale(STATUS_ICON_SIZE as f32 / 24., STATUS_ICON_SIZE as f32 / 24.),
            None,
        );
        if cache.len() >= 32 {
            cache.remove(0);
        }
        cache.push((symbol, color, pm.clone()));
        Some(pm)
    })
}
