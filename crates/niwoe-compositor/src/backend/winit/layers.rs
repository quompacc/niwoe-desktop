use std::time::Duration;

use smithay::backend::renderer::{
    element::{
        surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
        Kind,
    },
    gles::GlesRenderer,
};
use smithay::{
    desktop::{layer_map_for_output, LayerSurface},
    output::Output,
    utils::{Logical, Rectangle, Scale},
    wayland::shell::wlr_layer::Layer as WlrLayer,
};

use super::WinitRenderElements;

pub(super) type LayerRenderData = (LayerSurface, Rectangle<i32, Logical>);

fn is_upper_layer(namespace: &str, layer: WlrLayer) -> bool {
    crate::layer_order::upper_rank(namespace, layer).is_some()
}

pub(super) fn collect_layer_data(
    output: &Output,
    lower: &mut Vec<LayerRenderData>,
    upper: &mut Vec<LayerRenderData>,
) {
    let layer_map = layer_map_for_output(output);
    lower.clear();
    upper.clear();

    for layer_surface in layer_map.layers() {
        let geo = match layer_map.layer_geometry(layer_surface) {
            Some(geo) => geo,
            None => continue,
        };
        if is_upper_layer(layer_surface.namespace(), layer_surface.layer()) {
            upper.push((layer_surface.clone(), geo));
        } else {
            lower.push((layer_surface.clone(), geo));
        }
    }
    // Same stable front-to-back order as DRM and pointer hit-testing.
    upper.sort_by_key(|(s, _)| crate::layer_order::upper_rank(s.namespace(), s.layer()));
}

pub(super) fn render_layer_elements(
    renderer: &mut GlesRenderer,
    layer_data: &[LayerRenderData],
    scale: Scale<f64>,
    out: &mut Vec<WinitRenderElements>,
) {
    out.clear();
    for (layer, geo) in layer_data {
        let loc = geo.loc.to_f64().to_physical(scale).to_i32_round();
        let elements = render_elements_from_surface_tree::<
            GlesRenderer,
            WaylandSurfaceRenderElement<GlesRenderer>,
        >(
            renderer,
            layer.wl_surface(),
            loc,
            scale,
            1.0,
            Kind::Unspecified,
        );
        out.extend(elements.into_iter().map(WinitRenderElements::Layer));
    }
}

pub(super) fn send_layer_frames(
    output: &Output,
    time: Duration,
    lower_layer_data: &[LayerRenderData],
    upper_layer_data: &[LayerRenderData],
) {
    for (layer, _) in lower_layer_data.iter().chain(upper_layer_data.iter()) {
        layer.send_frame(output, time, Some(Duration::ZERO), |_, _| {
            Some(output.clone())
        });
    }
}

#[cfg(test)]
mod tests {
    use smithay::wayland::shell::wlr_layer::Layer as WlrLayer;

    use super::is_upper_layer;

    #[test]
    fn launcher_namespace_forces_upper_bucket() {
        assert!(is_upper_layer("niwoe-launcher", WlrLayer::Background));
        assert!(is_upper_layer("niwoe-launcher", WlrLayer::Bottom));
    }

    #[test]
    fn quick_settings_namespace_forces_upper_bucket() {
        assert!(is_upper_layer("niwoe-quick-settings", WlrLayer::Background));
        assert!(is_upper_layer("niwoe-quick-settings", WlrLayer::Bottom));
    }

    #[test]
    fn non_launcher_uses_layer_role() {
        assert!(is_upper_layer("other", WlrLayer::Top));
        assert!(is_upper_layer("other", WlrLayer::Overlay));
        assert!(!is_upper_layer("other", WlrLayer::Background));
        assert!(!is_upper_layer("other", WlrLayer::Bottom));
    }
}
