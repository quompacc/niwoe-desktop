//! Geometry and opacity tokens for the room-first Hub.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hub {
    pub min_canvas_width: u32,
    pub min_canvas_height: u32,
    pub preview_width: u32,
    pub preview_height: u32,
    pub search_row_height: i32,
    pub app_icon_size: i32,
    pub width: i32,
    pub height: i32,
    pub outer_pad: i32,
    pub hero_text_inset: i32,
    pub header_height: i32,
    pub close_width: i32,
    pub manage_width: i32,
    pub room_columns: i32,
    pub room_height: i32,
    pub lower_height: i32,
    pub lower_columns: i32,
    pub section_gap: i32,
    pub card_gap: i32,
    pub card_pad: i32,
    pub room_icon_size: i32,
    pub status_dot_size: i32,
    pub search_height: i32,
    pub card_alpha: u8,
    pub quiet_alpha: u8,
}

impl Hub {
    pub const DEFAULT: Hub = Hub {
        min_canvas_width: 1366,
        min_canvas_height: 768,
        preview_width: 288,
        preview_height: 112,
        search_row_height: 52,
        app_icon_size: 24,
        width: 1400,
        height: 832,
        outer_pad: 32,
        hero_text_inset: 112,
        header_height: 256,
        close_width: 112,
        manage_width: 152,
        room_columns: 4,
        room_height: 280,
        lower_height: 176,
        lower_columns: 3,
        section_gap: 24,
        card_gap: 16,
        card_pad: 16,
        room_icon_size: 44,
        status_dot_size: 8,
        search_height: 52,
        card_alpha: 255,
        quiet_alpha: 92,
    };
}

impl Hub {
    /// Preserve the four-card composition on small logical HiDPI viewports.
    /// Rendering and inverse pointer mapping use these same canvas dimensions.
    /// Transient/extreme aspect ratios cannot grow either axis beyond twice
    /// the canonical Hub extent; normal supported viewports never hit this cap.
    pub fn canvas_size(self, width: u32, height: u32) -> (u32, u32) {
        let scale = (width as f64 / self.min_canvas_width as f64)
            .min(height as f64 / self.min_canvas_height as f64)
            .min(1.0);
        if scale <= 0.0 {
            return (width, height);
        }
        (
            ((width as f64 / scale).ceil() as u32).min(self.width as u32 * 2),
            ((height as f64 / scale).ceil() as u32).min(self.height as u32 * 2),
        )
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl crate::Launcher {
    /// Shared by shell rendering/input and compositor glass. Preserve the
    /// normal centered Hub; on short viewports keep its controls below the panel.
    pub fn fitted_rect(self, width: u32, height: u32) -> (i32, i32, u32, u32) {
        let width = width.max(1);
        let height = height.max(1);
        let w = (self.width as u32).min(width);
        let h = (self.height as u32).min(height);
        let reserved = if self == Self::HUB {
            crate::Panel::DEFAULT.height.min(height - 1)
        } else {
            0
        };
        let y = ((height - h) / 2).max(reserved);
        (((width - w) / 2) as i32, y as i32, w, h.min(height - y))
    }
}

#[cfg(test)]
mod tests {
    use super::Hub;

    #[test]
    fn hidpi_canvas_preserves_composition_and_pointer_coordinates() {
        let hub = Hub::DEFAULT;
        for (width, height) in [(1400, 832), (1366, 768), (1280, 720), (960, 540)] {
            let (cw, ch) = hub.canvas_size(width, height);
            assert!(cw >= hub.min_canvas_width && ch >= hub.min_canvas_height);
            let bottom = hub.header_height + hub.room_height + hub.section_gap + hub.lower_height;
            assert!(bottom < ch as i32);
            let point = (
                hub.outer_pad + hub.card_pad,
                hub.header_height + hub.card_pad,
            );
            let physical = (
                point.0 as f64 * width as f64 / cw as f64,
                point.1 as f64 * height as f64 / ch as f64,
            );
            assert!(
                (physical.0 * cw as f64 / width as f64 - point.0 as f64).abs()
                    < f64::EPSILON * cw as f64
            );
            assert!(
                (physical.1 * ch as f64 / height as f64 - point.1 as f64).abs()
                    < f64::EPSILON * ch as f64
            );
        }
        assert_eq!(hub.canvas_size(1400, 832), (1400, 832));
        assert_eq!(hub.canvas_size(1366, 768), (1366, 768));
        for (w, h) in [(1, 832), (1400, 1), (1, 1)] {
            let (cw, ch) = hub.canvas_size(w, h);
            assert!(cw <= hub.width as u32 * 2 && ch <= hub.height as u32 * 2);
        }
    }
}
