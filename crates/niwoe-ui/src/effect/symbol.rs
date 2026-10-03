//! Native functional symbols, independent of UI font glyph coverage.
// guard:allow-file: native icon artwork uses a normalized 24-unit grid; color, raster size and cache bounds are supplied by tokens/callers.
use niwoe_tokens::{Color, Controls};
use std::{cell::RefCell, sync::Arc};
use tiny_skia::{LineCap, LineJoin, Paint, PathBuilder, Pixmap, Stroke, Transform};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    ChevronUp,
    ChevronDown,
    Lock,
    Check,
    App,
    Window,
    Room,
    Home,
    Folder,
    User,
    System,
    Panel,
    Download,
    History,
    List,
    Grid,
    Search,
    Settings,
    Monitor,
    Network,
    NetworkWired,
    Bluetooth,
    Speaker,
    Microphone,
    Printer,
}

type Entry = (Symbol, Color, u32, Arc<Pixmap>);
thread_local! { static CACHE: RefCell<Vec<Entry>> = const { RefCell::new(Vec::new()) }; }

/// Key: artwork/color/physical size. Different theme/scale cannot reuse old
/// pixels. FIFO eviction bounds total raster storage to 32 * 96 * 96 * 4 bytes;
/// the cache dies with its UI thread. Hits clone an Arc, never the pixel buffer.
pub fn symbol_icon(symbol: Symbol, color: Color, size: u32) -> Option<Arc<Pixmap>> {
    if size == 0 || size > Controls::SYMBOL_MAX_SIZE {
        return None;
    }
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, _, _, image)) = cache
            .iter()
            .find(|(s, c, z, _)| *s == symbol && *c == color && *z == size)
        {
            return Some(Arc::clone(image));
        }
        let mut image = Pixmap::new(size, size)?;
        let mut path = PathBuilder::new();
        match symbol {
            Symbol::Speaker => {
                path.move_to(3., 9.);
                path.line_to(7., 9.);
                path.line_to(12., 4.);
                path.line_to(12., 20.);
                path.line_to(7., 15.);
                path.line_to(3., 15.);
                path.close();
                path.move_to(16., 8.);
                path.cubic_to(19., 10., 19., 14., 16., 16.);
                path.move_to(19., 4.);
                path.cubic_to(25., 8., 25., 16., 19., 20.);
            }
            Symbol::Microphone => {
                path.move_to(8., 6.);
                path.cubic_to(8., 0.5, 16., 0.5, 16., 6.);
                path.line_to(16., 12.);
                path.cubic_to(16., 17.5, 8., 17.5, 8., 12.);
                path.close();
                path.move_to(5., 10.);
                path.line_to(5., 12.);
                path.cubic_to(5., 21., 19., 21., 19., 12.);
                path.line_to(19., 10.);
                path.move_to(12., 19.);
                path.line_to(12., 22.);
                path.move_to(8., 22.);
                path.line_to(16., 22.);
            }
            Symbol::Printer => {
                path.move_to(6., 8.);
                path.line_to(6., 2.);
                path.line_to(18., 2.);
                path.line_to(18., 8.);
                path.move_to(6., 18.);
                path.line_to(2., 18.);
                path.line_to(2., 8.);
                path.line_to(22., 8.);
                path.line_to(22., 18.);
                path.line_to(18., 18.);
                path.move_to(6., 14.);
                path.line_to(18., 14.);
                path.line_to(18., 22.);
                path.line_to(6., 22.);
                path.close();
                path.move_to(17., 11.);
                path.line_to(19., 11.);
            }
            Symbol::NetworkWired => {
                path.move_to(7., 2.);
                path.line_to(17., 2.);
                path.line_to(17., 9.);
                path.line_to(7., 9.);
                path.close();
                path.move_to(12., 9.);
                path.line_to(12., 14.);
                path.move_to(5., 18.);
                path.line_to(5., 14.);
                path.line_to(19., 14.);
                path.line_to(19., 18.);
                for x in [2., 16.] {
                    path.move_to(x, 18.);
                    path.line_to(x + 6., 18.);
                    path.line_to(x + 6., 22.);
                    path.line_to(x, 22.);
                    path.close();
                }
            }
            Symbol::Network => {
                path.move_to(3., 8.);
                path.cubic_to(8., 3., 16., 3., 21., 8.);
                path.move_to(6., 12.);
                path.cubic_to(10., 8., 14., 8., 18., 12.);
                path.move_to(9., 16.);
                path.cubic_to(11., 14., 13., 14., 15., 16.);
                path.move_to(12., 20.);
                path.line_to(12., 20.1);
            }
            Symbol::Bluetooth => {
                path.move_to(7., 7.);
                path.line_to(17., 17.);
                path.line_to(12., 22.);
                path.line_to(12., 2.);
                path.line_to(17., 7.);
                path.line_to(7., 17.);
            }
            Symbol::Monitor => {
                path.move_to(3., 3.);
                path.line_to(21., 3.);
                path.line_to(21., 16.);
                path.line_to(3., 16.);
                path.close();
                path.move_to(12., 16.);
                path.line_to(12., 21.);
                path.move_to(7., 21.);
                path.line_to(17., 21.);
            }
            Symbol::Grid => {
                for x in [3., 14.] {
                    for y in [3., 14.] {
                        path.move_to(x, y);
                        path.line_to(x + 7., y);
                        path.line_to(x + 7., y + 7.);
                        path.line_to(x, y + 7.);
                        path.close();
                    }
                }
            }
            Symbol::Search => {
                path.move_to(17., 10.);
                path.cubic_to(17., 0.7, 3., 0.7, 3., 10.);
                path.cubic_to(3., 19.3, 17., 19.3, 17., 10.);
                path.close();
                path.move_to(15., 15.);
                path.line_to(22., 22.);
            }
            Symbol::Home => {
                path.move_to(2., 11.);
                path.line_to(12., 3.);
                path.line_to(22., 11.);
                path.move_to(5., 9.);
                path.line_to(5., 21.);
                path.line_to(19., 21.);
                path.line_to(19., 9.);
                path.move_to(9., 21.);
                path.line_to(9., 14.);
                path.line_to(15., 14.);
                path.line_to(15., 21.);
            }
            Symbol::Folder => {
                path.move_to(3., 6.);
                path.line_to(9., 6.);
                path.line_to(12., 9.);
                path.line_to(21., 9.);
                path.line_to(21., 20.);
                path.line_to(3., 20.);
                path.close();
                path.move_to(3., 11.);
                path.line_to(21., 11.);
            }
            Symbol::User => {
                path.move_to(16., 7.);
                path.cubic_to(16., 1., 8., 1., 8., 7.);
                path.cubic_to(8., 13., 16., 13., 16., 7.);
                path.close();
                path.move_to(4., 21.);
                path.cubic_to(4., 11., 20., 11., 20., 21.);
            }
            Symbol::System => {
                path.move_to(12., 2.);
                path.line_to(21., 6.);
                path.line_to(21., 12.);
                path.cubic_to(21., 17., 15., 21., 12., 22.);
                path.cubic_to(9., 21., 3., 17., 3., 12.);
                path.line_to(3., 6.);
                path.close();
                path.move_to(8., 12.);
                path.line_to(11., 15.);
                path.line_to(16., 9.);
            }
            Symbol::Panel => {
                path.move_to(2., 5.);
                path.line_to(22., 5.);
                path.line_to(22., 19.);
                path.line_to(2., 19.);
                path.close();
                path.move_to(2., 10.);
                path.line_to(22., 10.);
                path.move_to(6., 7.5);
                path.line_to(10., 7.5);
            }
            Symbol::Download => {
                path.move_to(12., 2.);
                path.line_to(12., 16.);
                path.move_to(7., 11.);
                path.line_to(12., 16.);
                path.line_to(17., 11.);
                path.move_to(3., 16.);
                path.line_to(3., 21.);
                path.line_to(21., 21.);
                path.line_to(21., 16.);
            }
            Symbol::History => {
                path.move_to(4., 7.);
                path.cubic_to(13., -3., 25., 7., 19., 17.);
                path.cubic_to(15., 24., 4., 22., 3., 15.);
                path.move_to(3., 3.);
                path.line_to(3., 9.);
                path.line_to(9., 9.);
                path.move_to(12., 6.);
                path.line_to(12., 12.);
                path.line_to(16., 15.);
            }
            Symbol::List => {
                for y in [6., 12., 18.] {
                    path.move_to(3., y);
                    path.line_to(5., y);
                    path.move_to(9., y);
                    path.line_to(21., y);
                }
            }
            Symbol::Settings => {
                path.move_to(16., 12.);
                path.cubic_to(16., 6.5, 8., 6.5, 8., 12.);
                path.cubic_to(8., 17.5, 16., 17.5, 16., 12.);
                path.close();
                for (x, y, ex, ey) in [
                    (12., 2., 12., 5.),
                    (12., 19., 12., 22.),
                    (2., 12., 5., 12.),
                    (19., 12., 22., 12.),
                    (5., 5., 7., 7.),
                    (17., 17., 19., 19.),
                    (5., 19., 7., 17.),
                    (17., 7., 19., 5.),
                ] {
                    path.move_to(x, y);
                    path.line_to(ex, ey);
                }
            }
            Symbol::ChevronDown | Symbol::ChevronUp => {
                let (edge, middle) = if symbol == Symbol::ChevronDown {
                    (9., 15.)
                } else {
                    (15., 9.)
                };
                path.move_to(5., edge);
                path.line_to(12., middle);
                path.line_to(19., edge);
            }
            Symbol::Lock => {
                path.move_to(7., 10.);
                path.line_to(7., 7.);
                path.cubic_to(7., 1., 17., 1., 17., 7.);
                path.line_to(17., 10.);
                path.move_to(5., 10.);
                path.line_to(19., 10.);
                path.line_to(19., 21.);
                path.line_to(5., 21.);
                path.close();
                path.move_to(12., 14.);
                path.line_to(12., 17.);
            }
            Symbol::Check => {
                path.move_to(4., 12.);
                path.line_to(9., 17.);
                path.line_to(20., 6.);
            }
            Symbol::App | Symbol::Window => {
                path.move_to(3., 4.);
                path.line_to(21., 4.);
                path.line_to(21., 20.);
                path.line_to(3., 20.);
                path.close();
                path.move_to(3., 9.);
                path.line_to(21., 9.);
                if symbol == Symbol::App {
                    path.move_to(9., 9.);
                    path.line_to(9., 20.);
                }
            }
            Symbol::Room => {
                path.move_to(12., 2.);
                path.line_to(21., 7.);
                path.line_to(21., 17.);
                path.line_to(12., 22.);
                path.line_to(3., 17.);
                path.line_to(3., 7.);
                path.close();
                path.move_to(3., 7.);
                path.line_to(12., 12.);
                path.line_to(21., 7.);
                path.move_to(12., 12.);
                path.line_to(12., 22.);
            }
        }
        let mut paint = Paint {
            anti_alias: true,
            ..Paint::default()
        };
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        image.stroke_path(
            &path.finish()?,
            &paint,
            &Stroke {
                width: 1.5,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Stroke::default()
            },
            Transform::from_scale(size as f32 / 24., size as f32 / 24.),
            None,
        );
        let image = Arc::new(image);
        if cache.len() >= Controls::SYMBOL_CACHE_ENTRIES {
            cache.remove(0);
        }
        cache.push((symbol, color, size, Arc::clone(&image)));
        Some(image)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_symbols_render_without_font_glyphs_and_reuse_matching_rasters() {
        for symbol in [
            Symbol::ChevronUp,
            Symbol::ChevronDown,
            Symbol::Lock,
            Symbol::Check,
            Symbol::App,
            Symbol::Window,
            Symbol::Room,
            Symbol::Home,
            Symbol::Folder,
            Symbol::User,
            Symbol::System,
            Symbol::Panel,
            Symbol::Download,
            Symbol::History,
            Symbol::List,
            Symbol::Settings,
            Symbol::Network,
            Symbol::NetworkWired,
            Symbol::Bluetooth,
            Symbol::Speaker,
            Symbol::Microphone,
            Symbol::Printer,
        ] {
            for size in [Controls::SYMBOL_SIZE, 24, 32] {
                let icon = symbol_icon(symbol, niwoe_tokens::Palette::DARK.text, size).unwrap();
                assert!(icon.data().as_chunks::<4>().0.iter().any(|p| p[3] > 0));
                let repeated = symbol_icon(symbol, niwoe_tokens::Palette::DARK.text, size).unwrap();
                assert!(Arc::ptr_eq(&icon, &repeated));
            }
        }
    }

    #[test]
    fn color_size_and_cache_bounds_prevent_stale_or_unbounded_rasters() {
        CACHE.with(|c| c.borrow_mut().clear());
        let first = symbol_icon(
            Symbol::Lock,
            niwoe_tokens::Palette::DARK.text,
            Controls::SYMBOL_SIZE,
        )
        .unwrap();
        let changed = symbol_icon(
            Symbol::Lock,
            niwoe_tokens::Palette::DARK.accent,
            Controls::SYMBOL_SIZE,
        )
        .unwrap();
        assert!(!Arc::ptr_eq(&first, &changed));
        assert_ne!(first.data(), changed.data());
        for size in 1..=Controls::SYMBOL_MAX_SIZE {
            symbol_icon(Symbol::Lock, niwoe_tokens::Palette::DARK.text, size).unwrap();
        }
        CACHE.with(|c| assert!(c.borrow().len() <= Controls::SYMBOL_CACHE_ENTRIES));
        assert!(symbol_icon(Symbol::Lock, niwoe_tokens::Palette::DARK.text, 0).is_none());
        assert!(symbol_icon(
            Symbol::Lock,
            niwoe_tokens::Palette::DARK.text,
            Controls::SYMBOL_MAX_SIZE + 1
        )
        .is_none());
    }
}
