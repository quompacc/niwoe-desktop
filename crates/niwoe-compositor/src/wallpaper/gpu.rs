use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::{
                texture::{TextureBuffer, TextureRenderElement},
                Kind,
            },
            gles::{GlesRenderer, GlesTexture},
            ImportMem,
        },
    },
    utils::{Buffer, Logical, Physical, Point, Rectangle, Scale, Size, Transform},
};
use tracing::{info, warn};

use super::WallpaperManager;

pub struct WallpaperGpuCache {
    buffer: TextureBuffer<GlesTexture>,
    output_size: (u32, u32),
    source_key: u64,
}

impl WallpaperGpuCache {
    fn needs_update(cache: &Option<Self>, manager: &WallpaperManager, w: u32, h: u32) -> bool {
        match cache {
            None => true,
            Some(cached) => {
                cached.output_size != (w, h) || cached.source_key != manager.source_key()
            }
        }
    }

    pub fn update(
        renderer: &mut GlesRenderer,
        cache: &mut Option<Self>,
        manager: &mut WallpaperManager,
        out_w: u32,
        out_h: u32,
    ) {
        if !Self::needs_update(cache, manager, out_w, out_h) {
            return;
        }

        let data = manager.compose_for_size(out_w, out_h);
        let size: Size<i32, Buffer> = (out_w as i32, out_h as i32).into();

        match renderer.import_memory(&data, Fourcc::Abgr8888, size, false) {
            Ok(texture) => {
                let buffer =
                    TextureBuffer::from_texture(renderer, texture, 1, Transform::Normal, None);
                *cache = Some(Self {
                    buffer,
                    output_size: (out_w, out_h),
                    source_key: manager.source_key(),
                });
                info!("Wallpaper texture uploaded ({}x{})", out_w, out_h);
            }
            Err(err) => {
                warn!("Wallpaper GPU upload failed: {err}");
                *cache = None;
            }
        }
    }

    pub fn render_element(&self) -> TextureRenderElement<GlesTexture> {
        self.render_element_at_scale(Scale::from(1.0))
    }

    /// Keep the cached physical output texture unchanged when UI scale changes.
    pub fn render_element_at_scale(&self, scale: Scale<f64>) -> TextureRenderElement<GlesTexture> {
        let physical =
            Size::<i32, Physical>::from((self.output_size.0 as i32, self.output_size.1 as i32));
        TextureRenderElement::from_texture_buffer(
            Point::<f64, Physical>::from((0.0, 0.0)),
            &self.buffer,
            Some(1.0),
            Some(Rectangle::<f64, Logical>::from_size(
                physical.to_f64().to_logical(1.0),
            )),
            Some(physical.to_f64().to_logical(scale).to_i32_round()),
            Kind::Unspecified,
        )
    }
}
