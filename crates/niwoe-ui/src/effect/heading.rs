//! Fixed serif display role. The embedded face is parsed once per process;
//! immutable glyph coverage is cached by character at the central role size.
//! FIFO bounds are 128 entries / 512 KiB, with Arc hits and no bitmap copies.
use std::{
    cell::RefCell,
    collections::VecDeque,
    sync::{Arc, OnceLock},
};

use fontdue::{Font, FontSettings, Metrics};
use niwoe_tokens::{Color, Typography};
use tiny_skia::PixmapMut;

use super::{blend_text_sample, TextInk};
use crate::paint::Rect;

struct Glyph {
    metrics: Metrics,
    coverage: Vec<u8>,
}

#[derive(Default)]
struct Cache {
    entries: VecDeque<(char, Arc<Glyph>)>,
    bytes: usize,
}

impl Cache {
    fn get(&mut self, ch: char) -> Arc<Glyph> {
        if let Some((_, glyph)) = self.entries.iter().find(|(key, _)| *key == ch) {
            return Arc::clone(glyph);
        }
        static FONT: OnceLock<Font> = OnceLock::new();
        let font = FONT.get_or_init(|| {
            Font::from_bytes(
                niwoe_tokens::font::NOTO_SERIF_REGULAR,
                FontSettings::default(),
            )
            .expect("embedded Noto Serif parses")
        });
        let role = Typography::DEFAULT;
        let (metrics, coverage) = font.rasterize(ch, role.serif_display_size as f32);
        let glyph = Arc::new(Glyph { metrics, coverage });
        let bytes = glyph.coverage.len();
        if bytes <= role.heading_cache_bytes {
            while self.entries.len() >= role.heading_cache_entries
                || self.bytes + bytes > role.heading_cache_bytes
            {
                let Some((_, oldest)) = self.entries.pop_front() else {
                    break;
                };
                self.bytes -= oldest.coverage.len();
            }
            self.bytes += bytes;
            self.entries.push_back((ch, Arc::clone(&glyph)));
        }
        glyph
    }
}

thread_local! { static CACHE: RefCell<Cache> = RefCell::default(); }

/// Paint within the supplied page region; coordinates use a baseline.
pub fn paint_display_heading(
    canvas: &mut PixmapMut<'_>,
    text: &str,
    x: i32,
    baseline: i32,
    clip: Rect,
    color: Color,
) {
    let width = canvas.width() as i32;
    let height = canvas.height() as i32;
    let ink = TextInk::new(color);
    let mut pen = x as f32;
    for ch in text.chars() {
        let glyph = CACHE.with(|cache| cache.borrow_mut().get(ch));
        let m = &glyph.metrics;
        let left = pen.round() as i32 + m.xmin;
        let top = baseline - m.ymin - m.height as i32;
        for row in 0..m.height {
            let py = top + row as i32;
            if py < clip.y.max(0) || py >= (clip.y + clip.height).min(height) {
                continue;
            }
            for col in 0..m.width {
                let px = left + col as i32;
                if px < clip.x.max(0) || px >= (clip.x + clip.width).min(width) {
                    continue;
                }
                let at = (py as usize * width as usize + px as usize) * 4;
                blend_text_sample(
                    &mut canvas.data_mut()[at..at + 4],
                    ink,
                    glyph.coverage[row * m.width + col],
                    [0, 1, 2],
                );
            }
        }
        pen += m.advance_width;
        if pen >= (clip.x + clip.width) as f32 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heading_coverage_is_reused_and_bounded() {
        let mut cache = Cache::default();
        let first = cache.get('Ä');
        assert!(Arc::ptr_eq(&first, &cache.get('Ä')));
        for ch in ' '..='ӿ' {
            cache.get(ch);
        }
        assert!(cache.entries.len() <= Typography::DEFAULT.heading_cache_entries);
        assert!(cache.bytes <= Typography::DEFAULT.heading_cache_bytes);
    }
    #[test]
    fn heading_cannot_paint_outside_its_region() {
        let mut image = tiny_skia::Pixmap::new(160, 80).unwrap();
        let clip = Rect {
            x: 20,
            y: 10,
            width: 60,
            height: 50,
        };
        paint_display_heading(
            &mut image.as_mut(),
            "Ärbeit",
            0,
            40,
            clip,
            niwoe_tokens::Palette::DARK.text,
        );
        let mut count = 0;
        for y in 0..80 {
            for x in 0..160 {
                if image.pixel(x, y).unwrap().alpha() > 0 {
                    count += 1;
                    assert!((20..80).contains(&x) && (10..60).contains(&y));
                }
            }
        }
        assert!(count > 0);
    }
}
