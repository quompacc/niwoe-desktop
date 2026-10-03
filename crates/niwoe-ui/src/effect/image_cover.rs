//! Proportional image cover with an explicit vertical focal position.
use crate::Rect;
use tiny_skia::{Pixmap, PixmapMut, PixmapPaint, Transform};

pub fn image_cover_transform(image: &Pixmap, area: Rect, focal_y: f32) -> Option<Transform> {
    if area.width <= 0 || area.height <= 0 || !focal_y.is_finite() {
        return None;
    }
    let scale =
        (area.width as f32 / image.width() as f32).max(area.height as f32 / image.height() as f32);
    let x = area.x as f32 + (area.width as f32 - image.width() as f32 * scale) / 2.;
    let y = area.y as f32
        + (area.height as f32 - image.height() as f32 * scale) * focal_y.clamp(0., 1.);
    Some(Transform::from_scale(scale, scale).post_translate(x, y))
}

/// Caller provides a canvas/clip matching the destination; no decoding or cache.
pub fn paint_image_cover(canvas: &mut PixmapMut<'_>, image: &Pixmap, focal_y: f32) {
    let area = Rect {
        x: 0,
        y: 0,
        width: canvas.width() as i32,
        height: canvas.height() as i32,
    };
    if let Some(transform) = image_cover_transform(image, area, focal_y) {
        canvas.draw_pixmap(
            0,
            0,
            image.as_ref(),
            &PixmapPaint::default(),
            transform,
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_and_tall_crops_preserve_aspect_and_cover_destination() {
        let source = Pixmap::new(2172, 724).unwrap();
        for (width, height) in [(1400, 224), (1696, 192), (1118, 132), (300, 400)] {
            let rect = Rect {
                x: 12,
                y: 8,
                width,
                height,
            };
            let t = image_cover_transform(&source, rect, niwoe_tokens::Artwork::FOCAL_Y).unwrap();
            assert_eq!(t.sx, t.sy);
            assert_eq!(t.kx, 0.);
            assert_eq!(t.ky, 0.);
            assert!(t.tx <= rect.x as f32 && t.ty <= rect.y as f32);
            assert!(t.tx + source.width() as f32 * t.sx >= (rect.x + width) as f32);
            assert!(t.ty + source.height() as f32 * t.sy >= (rect.y + height) as f32);
        }
    }
}
