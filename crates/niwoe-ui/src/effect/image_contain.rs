//! Fit a cached raster inside a destination while retaining its native size.
//! No decoding, allocation, animation or persistent state during painting.
use crate::Rect;
use tiny_skia::{Pixmap, PixmapMut, PixmapPaint, Transform};

pub fn paint_image_contain(canvas: &mut PixmapMut<'_>, image: &Pixmap, area: Rect) {
    if area.width <= 0 || area.height <= 0 {
        return;
    }
    let scale = (area.width as f32 / image.width() as f32)
        .min(area.height as f32 / image.height() as f32)
        .min(1.0);
    let x = area.x as f32 + (area.width as f32 - image.width() as f32 * scale) / 2.;
    let y = area.y as f32 + (area.height as f32 - image.height() as f32 * scale) / 2.;
    // Translate after scaling. draw_pixmap's x/y arguments would scale the
    // destination as well, moving a reduced raster outside its control.
    canvas.draw_pixmap(
        0,
        0,
        image.as_ref(),
        &PixmapPaint::default(),
        Transform::from_scale(scale, scale).post_translate(x, y),
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduced_and_native_images_stay_inside_the_nonzero_destination() {
        let area = Rect {
            x: 400,
            y: 200,
            width: 48,
            height: 48,
        };
        for (width, height) in [(128, 64), (24, 16)] {
            let mut source = Pixmap::new(width, height).unwrap();
            source.fill(tiny_skia::Color::WHITE);
            let mut target = Pixmap::new(600, 400).unwrap();
            paint_image_contain(&mut target.as_mut(), &source, area);
            let mut count = 0;
            for y in 0..target.height() {
                for x in 0..target.width() {
                    if target.pixel(x, y).unwrap().alpha() == 0 {
                        continue;
                    }
                    assert!((400..448).contains(&x), "escaped horizontally at {x}");
                    assert!((200..248).contains(&y), "escaped vertically at {y}");
                    count += 1;
                }
            }
            assert_eq!(count, if width == 128 { 48 * 24 } else { 24 * 16 });
        }
    }
}
