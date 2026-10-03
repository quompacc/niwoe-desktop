//! Immutable header artwork. Source decoded once; fitted rasters are bounded.
use niwoe_tokens::Artwork;
use std::{
    cell::RefCell,
    sync::{Arc, OnceLock},
};
use tiny_skia::{Pixmap, PixmapMut};

type Entry = (u32, u32, Arc<Pixmap>);
thread_local! { static CACHE: RefCell<Vec<Entry>> = const { RefCell::new(Vec::new()) }; }

fn source() -> Option<&'static Pixmap> {
    static SOURCE: OnceLock<Option<Pixmap>> = OnceLock::new();
    SOURCE
        .get_or_init(|| {
            Pixmap::decode_png(include_bytes!(
                "../../../assets/ui/niwoe-context-landscape-v1.png"
            ))
            .ok()
        })
        .as_ref()
}

fn fitted(width: u32, height: u32) -> Option<Arc<Pixmap>> {
    let bytes = width as usize * height as usize * 4;
    if width == 0
        || height == 0
        || width > Artwork::MAX_WIDTH
        || height > Artwork::MAX_HEIGHT
        || bytes > Artwork::CACHE_BYTES
    {
        return None;
    }
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, _, image)) = cache.iter().find(|(w, h, _)| (*w, *h) == (width, height)) {
            return Some(Arc::clone(image));
        }
        let mut image = Pixmap::new(width, height)?;
        niwoe_ui::effect::paint_image_cover(&mut image.as_mut(), source()?, Artwork::FOCAL_Y);
        while cache.len() >= Artwork::CACHE_ENTRIES
            || cache.iter().map(|(_, _, p)| p.data().len()).sum::<usize>() + bytes
                > Artwork::CACHE_BYTES
        {
            cache.remove(0);
        }
        let image = Arc::new(image);
        cache.push((width, height, Arc::clone(&image)));
        Some(image)
    })
}

/// Key: embedded asset identity and destination dimensions. Theme colors and
/// text remain caller-owned; no tinted wallpaper or per-frame capture.
pub(crate) fn paint(canvas: &mut PixmapMut<'_>, area: niwoe_ui::Rect) {
    if area.width <= 0 || area.height <= 0 {
        return;
    }
    if let Some(image) = fitted(area.width as u32, area.height as u32) {
        canvas.draw_pixmap(
            area.x,
            area.y,
            image.as_ref().as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_reuse_and_dimension_eviction_keep_header_storage_bounded() {
        CACHE.with(|c| c.borrow_mut().clear());
        let image = fitted(1400, 224).unwrap();
        assert!(Arc::ptr_eq(&image, &fitted(1400, 224).unwrap()));
        for width in [1118, 1672, 2000, 3000, 4000] {
            fitted(width, 512).unwrap();
        }
        CACHE.with(|c| {
            let entries = c.borrow();
            assert!(entries.len() <= Artwork::CACHE_ENTRIES);
            assert!(
                entries
                    .iter()
                    .map(|(_, _, p)| p.data().len())
                    .sum::<usize>()
                    <= Artwork::CACHE_BYTES
            );
        });
        assert!(fitted(Artwork::MAX_WIDTH + 1, 224).is_none());
        assert!(fitted(1400, Artwork::MAX_HEIGHT + 1).is_none());
    }
}
