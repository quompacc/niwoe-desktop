//! One product-panel raster, rebuilt only when a painted input changes.
//! Maximum retention: 4096 × native panel height × four bytes, until closure.
use std::sync::Arc;

use super::shell::ThemeRenderSignature;

type NotifierKey = (String, Option<String>, Option<String>, Option<String>);

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct PreviewKey {
    pub width: u32,
    pub active_workspace: u8,
    pub rooms: Vec<(u64, u8, String)>,
    pub occupied: [bool; niwoe_config::rooms::MAX_ROOMS],
    pub clock: String,
    pub network_icon: &'static str,
    pub audio_icon: &'static str,
    pub audio_label: String,
    pub notifier_items: Vec<NotifierKey>,
    pub battery: crate::battery::BatterySnapshot,
    pub modules: Vec<niwoe_ipc::PanelModule>,
    pub theme: ThemeRenderSignature,
    pub icon_loader: (String, String),
    pub icon_generation: u64,
}

#[derive(Default)]
pub(crate) struct PanelPreviewCache {
    entry: Option<(PreviewKey, Arc<tiny_skia::Pixmap>)>,
}

impl PanelPreviewCache {
    pub(crate) fn fitted_width(width: u32) -> u32 {
        width.min(niwoe_tokens::ControlCenter::DEFAULT.panel_preview_max_width)
    }

    pub(crate) fn get(&self, key: &PreviewKey) -> Option<Arc<tiny_skia::Pixmap>> {
        self.entry
            .as_ref()
            .filter(|(cached, _)| cached == key)
            .map(|(_, image)| Arc::clone(image))
    }

    pub(crate) fn insert(
        &mut self,
        key: PreviewKey,
        image: tiny_skia::Pixmap,
    ) -> Arc<tiny_skia::Pixmap> {
        let image = Arc::new(image);
        self.entry = Some((key, Arc::clone(&image)));
        image
    }

    pub(crate) fn clear(&mut self) {
        self.entry = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key() -> PreviewKey {
        PreviewKey {
            width: 1366,
            active_workspace: 0,
            rooms: vec![(1, 1, "Raum 1".into())],
            occupied: [false; niwoe_config::rooms::MAX_ROOMS],
            clock: "12:00".into(),
            network_icon: "network-offline",
            audio_icon: "audio-volume-muted",
            audio_label: String::new(),
            notifier_items: Vec::new(),
            battery: Default::default(),
            modules: crate::room_editor::panel::MODULES.to_vec(),
            theme: ThemeRenderSignature {
                font_ui: "Sans".into(),
                colors: [0; 44],
            },
            icon_loader: ("Adwaita".into(), "symbolic".into()),
            icon_generation: 0,
        }
    }

    #[test]
    fn unchanged_preview_reuses_pixels_and_close_releases_them() {
        let key = key();
        let mut cache = PanelPreviewCache::default();
        let image = cache.insert(
            key.clone(),
            tiny_skia::Pixmap::new(key.width, crate::PANEL_SURFACE_HEIGHT).unwrap(),
        );
        assert!(Arc::ptr_eq(&image, &cache.get(&key).unwrap()));
        cache.clear();
        assert!(cache.get(&key).is_none());
        assert_eq!(Arc::strong_count(&image), 1);
    }

    #[test]
    fn painted_inputs_invalidate_and_replacement_retains_one_raster() {
        let key = key();
        let mut cache = PanelPreviewCache::default();
        let old = cache.insert(
            key.clone(),
            tiny_skia::Pixmap::new(key.width, crate::PANEL_SURFACE_HEIGHT).unwrap(),
        );
        let mut changes = vec![key.clone(); 7];
        changes[0].width -= 1;
        changes[1].modules.pop();
        changes[2].clock = "12:01".into();
        changes[3].battery.present = true;
        changes[4].icon_generation += 1;
        changes[5].theme.font_ui = "Serif".into();
        changes[6].rooms[0].2 = "Anderer Name".into();
        for next in changes {
            assert!(cache.get(&next).is_none());
        }
        let mut next = key.clone();
        next.modules.pop();
        cache.insert(
            next.clone(),
            tiny_skia::Pixmap::new(next.width, crate::PANEL_SURFACE_HEIGHT).unwrap(),
        );
        assert!(cache.get(&key).is_none());
        assert_eq!(Arc::strong_count(&old), 1);
    }

    #[test]
    fn ultrawide_preview_is_bounded_before_allocation() {
        let cap = niwoe_tokens::ControlCenter::DEFAULT.panel_preview_max_width;
        assert_eq!(PanelPreviewCache::fitted_width(u32::MAX), cap);
        assert_eq!(PanelPreviewCache::fitted_width(1366), 1366);
        assert_eq!(PanelPreviewCache::fitted_width(0), 0);
    }
}
