//! Bounded Xcursor reader: validate offsets before copying the selected frame.
use tiny_skia::{IntSize, Pixmap};

const IMAGE_TYPE: u32 = 0xfffd_0002;
const TOC_LIMIT: usize = 4096;
const RASTER_LIMIT: u32 = 128;

fn word(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?,
    ))
}

pub(super) fn first_nearest_frame(bytes: &[u8], size: u32) -> Option<Pixmap> {
    if bytes.get(..4)? != b"Xcur" {
        return None;
    }
    let header = word(bytes, 4)? as usize;
    let count = word(bytes, 12)? as usize;
    if header < 16 || count > TOC_LIMIT {
        return None;
    }
    bytes.get(header..header.checked_add(count.checked_mul(12)?)?)?;
    let mut nearest: Option<(u32, u32, u32, &[u8])> = None;
    for index in 0..count {
        let toc = header + index * 12;
        if word(bytes, toc)? != IMAGE_TYPE {
            continue;
        }
        let pos = word(bytes, toc + 8)? as usize;
        let frame = bytes.get(pos..)?;
        if word(frame, 0)? != 36 || word(frame, 4)? != IMAGE_TYPE || word(frame, 12)? != 1 {
            return None;
        }
        let nominal = word(frame, 8)?;
        let width = word(frame, 16)?;
        let height = word(frame, 20)?;
        if width == 0
            || height == 0
            || width > 0x7fff
            || height > 0x7fff
            || word(frame, 24)? > width
            || word(frame, 28)? > height
        {
            return None;
        }
        let length = (width as usize)
            .checked_mul(height as usize)?
            .checked_mul(4)?;
        let pixels = frame.get(36..36usize.checked_add(length)?)?;
        let distance = nominal.abs_diff(size);
        if nearest.as_ref().is_none_or(|(best, ..)| distance < *best) {
            nearest = Some((distance, width, height, pixels));
        }
    }
    let (_, width, height, pixels) = nearest?;
    if width > RASTER_LIMIT || height > RASTER_LIMIT {
        return None;
    }
    // Match the compositor's pixels_rgba byte order; source pixels are premultiplied.
    Pixmap::from_vec(pixels.to_vec(), IntSize::from_wh(width, height)?)
}
