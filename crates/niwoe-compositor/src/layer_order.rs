//! Shared front-to-back order for shell rendering and pointer hit-testing.
use smithay::wayland::shell::wlr_layer::Layer;

/// Keep the established DRM order: Overlay first, then Top and shell surfaces
/// whose cached role temporarily fell back during unmap/remap. Equal ranks
/// retain LayerMap iteration order, never the reverse order of layer_under().
pub(crate) fn upper_rank(namespace: &str, layer: Layer) -> Option<u8> {
    let shell_overlay = matches!(
        namespace,
        "niwoe-launcher"
            | "niwoe-quick-settings"
            | "niwoe-calendar-popup"
            | "niwoe-workspace-popup"
            | "niwoe-network-popup"
            | "niwoe-notification"
            | "niwoe-thumbnail-popup"
            | "niwoe-desktop-menu"
            | "niwoe-screenshot-consent"
            | "niwoe-screenshot-region-picker"
    );
    match layer {
        Layer::Overlay => Some(0),
        Layer::Top => Some(1),
        _ if shell_overlay => Some(1),
        _ => None,
    }
}

/// Match the stable render sort without allocating/sorting on pointer motion.
/// A transparent input region or hidden surface must not hide the next hit.
pub(crate) fn first_upper_hit<T, R, I: Iterator<Item = T>>(
    layers: impl Fn() -> I,
    rank: impl Fn(&T) -> Option<u8>,
    mut hit: impl FnMut(T) -> Option<R>,
) -> Option<R> {
    for priority in [0, 1] {
        for layer in layers().filter(|layer| rank(layer) == Some(priority)) {
            if let Some(result) = hit(layer) {
                return Some(result);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Surface {
        name: &'static str,
        layer: Layer,
        accepts: bool,
        bounds: (i32, i32, i32, i32),
    }

    fn target(surfaces: &[Surface], x: i32, y: i32) -> Option<&'static str> {
        first_upper_hit(
            || surfaces.iter(),
            |s| upper_rank(s.name, s.layer),
            |s| {
                let (left, top, width, height) = s.bounds;
                (s.accepts && x >= left && y >= top && x < left + width && y < top + height)
                    .then_some(s.name)
            },
        )
    }

    #[test]
    fn visible_consent_over_settings_receives_both_buttons() {
        // Same stable LayerMap order used by the render lists. layer_under's
        // reverse search used to pick the large settings surface instead.
        let surfaces = [
            Surface {
                name: "niwoe-screenshot-consent",
                layer: Layer::Overlay,
                accepts: true,
                bounds: (200, 200, 400, 200),
            },
            Surface {
                name: "niwoe-launcher",
                layer: Layer::Overlay,
                accepts: true,
                bounds: (0, 0, 880, 620),
            },
        ];
        assert_eq!(
            target(&surfaces, 300, 350),
            Some("niwoe-screenshot-consent")
        );
        assert_eq!(
            target(&surfaces, 500, 350),
            Some("niwoe-screenshot-consent")
        );
        assert_eq!(target(&surfaces, 100, 100), Some("niwoe-launcher"));
    }

    #[test]
    fn hidden_or_input_transparent_surface_does_not_mask_next_target() {
        let surfaces = [
            Surface {
                name: "niwoe-screenshot-consent",
                layer: Layer::Overlay,
                accepts: false,
                bounds: (0, 0, 500, 500),
            },
            Surface {
                name: "niwoe-launcher",
                layer: Layer::Overlay,
                accepts: true,
                bounds: (0, 0, 500, 500),
            },
        ];
        assert_eq!(target(&surfaces, 50, 50), Some("niwoe-launcher"));
    }

    #[test]
    fn overlay_beats_top_and_stale_shell_role_stays_above_windows() {
        let surfaces = [
            Surface {
                name: "panel",
                layer: Layer::Top,
                accepts: true,
                bounds: (0, 0, 500, 500),
            },
            Surface {
                name: "niwoe-screenshot-consent",
                layer: Layer::Overlay,
                accepts: true,
                bounds: (0, 0, 500, 500),
            },
        ];
        assert_eq!(target(&surfaces, 50, 50), Some("niwoe-screenshot-consent"));
        assert_eq!(
            upper_rank("niwoe-screenshot-consent", Layer::Background),
            Some(1)
        );
        assert_eq!(upper_rank("wallpaper", Layer::Background), None);
    }
}
