//! Bounded static thumbnails. Entry refresh runs once in the existing worker.
//! The shell lifetime cache shares pixels; path/size/mtime invalidate each image.
use niwoe_config::{NiwoeConfig, WallpaperEntry};
use std::{fs::File, io::Read, path::PathBuf, sync::Arc, time::SystemTime};
use tiny_skia::Pixmap;

const INPUT_LIMIT: u64 = 32 * 1024 * 1024;
const DIMENSION_LIMIT: u32 = 8192;
const DECODE_BUDGET: u64 = 128 * 1024 * 1024;

pub(crate) struct WallpaperRequest {
    pub selected: Option<String>,
    pub previous: Vec<WallpaperPreview>,
}

pub(crate) struct WallpaperData {
    pub requested_selected: Option<String>,
    pub catalog: Vec<WallpaperEntry>,
    pub previews: Vec<WallpaperPreview>,
}

#[derive(Clone)]
pub(crate) struct WallpaperPreview {
    pub path: String,
    pub image: Option<Arc<Pixmap>>,
    identity: Option<FileIdentity>,
    bounds: (u32, u32),
}

#[derive(Clone, PartialEq, Eq)]
struct FileIdentity {
    path: PathBuf,
    length: u64,
    modified: SystemTime,
}

impl WallpaperData {
    pub fn load(request: WallpaperRequest) -> Self {
        let catalog = NiwoeConfig::scan_wallpaper_dirs();
        let c = niwoe_tokens::Settings::DEFAULT;
        let previews = load_previews(
            &catalog,
            request.previous,
            (c.wallpaper_thumbnail_width, c.wallpaper_thumbnail_height),
        );
        Self {
            requested_selected: request.selected,
            catalog,
            previews,
        }
    }
}

fn load_previews(
    catalog: &[WallpaperEntry],
    previous: Vec<WallpaperPreview>,
    bounds: (u32, u32),
) -> Vec<WallpaperPreview> {
    catalog
        .iter()
        .take(crate::settings_view::WALLPAPER_WIDGET_IDS.len())
        .map(|entry| {
            let path = PathBuf::from(&entry.thumbnail_path);
            let identity = path.metadata().ok().and_then(|m| {
                Some(FileIdentity {
                    path,
                    length: m.len(),
                    modified: m.modified().ok()?,
                })
            });
            if let Some(cached) = previous.iter().find(|p| {
                p.path == entry.thumbnail_path && p.identity == identity && p.bounds == bounds
            }) {
                return cached.clone();
            }
            let image = identity
                .as_ref()
                .and_then(|key| decode(key, bounds))
                .map(Arc::new);
            WallpaperPreview {
                path: entry.thumbnail_path.clone(),
                image,
                identity,
                bounds,
            }
        })
        .collect()
}

fn decode(key: &FileIdentity, (width, height): (u32, u32)) -> Option<Pixmap> {
    if key.length > INPUT_LIMIT || width == 0 || height == 0 {
        return None;
    }
    let mut bytes = Vec::new();
    File::open(&key.path)
        .ok()?
        .take(INPUT_LIMIT + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.len() as u64 > INPUT_LIMIT {
        return None;
    }
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(DIMENSION_LIMIT);
    limits.max_image_height = Some(DIMENSION_LIMIT);
    // Decoder allocations are best-effort; strict dimension/input limits also
    // apply. This budget is not a bound on total process RSS or decoder scratch.
    limits.max_alloc = Some(DECODE_BUDGET);
    reader.limits(limits);
    let rgba = reader.decode().ok()?.thumbnail(width, height).to_rgba8();
    let mut result = Pixmap::new(rgba.width(), rgba.height())?;
    for (source, target) in rgba
        .as_raw()
        .as_chunks::<4>()
        .0
        .iter()
        .zip(result.data_mut().as_chunks_mut::<4>().0.iter_mut())
    {
        let alpha = u16::from(source[3]);
        for channel in 0..3 {
            target[channel] = (u16::from(source[channel]) * alpha / u16::from(u8::MAX)) as u8;
        }
        target[3] = source[3];
    }
    Some(result)
}

#[cfg(test)]
mod tests;
