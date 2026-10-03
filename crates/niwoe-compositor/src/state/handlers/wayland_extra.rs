use smithay::{
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Rectangle},
    wayland::{
        fractional_scale::FractionalScaleHandler,
        input_method::{InputMethodHandler, PopupSurface},
        xdg_activation::{
            XdgActivationHandler, XdgActivationState, XdgActivationToken, XdgActivationTokenData,
        },
    },
};

use crate::state::NiwoeState;

impl XdgActivationHandler for NiwoeState {
    fn activation_state(&mut self) -> &mut XdgActivationState {
        &mut self.xdg_activation_state
    }

    fn token_created(&mut self, _token: XdgActivationToken, data: XdgActivationTokenData) -> bool {
        tracing::debug!(
            "xdg-activation: token created (client_id={:?}, has_surface={})",
            data.client_id,
            data.surface.is_some()
        );
        self.prune_launch_tokens()
    }

    fn request_activation(
        &mut self,
        token: XdgActivationToken,
        _token_data: XdgActivationTokenData,
        surface: WlSurface,
    ) {
        self.apply_launch_activation(token.as_str(), &surface);
    }
}

impl FractionalScaleHandler for NiwoeState {
    fn new_fractional_scale(&mut self, _surface: WlSurface) {}
}

impl InputMethodHandler for NiwoeState {
    fn new_popup(&mut self, _surface: PopupSurface) {}

    fn popup_repositioned(&mut self, _surface: PopupSurface) {}

    fn dismiss_popup(&mut self, _surface: PopupSurface) {}

    fn parent_geometry(&self, _parent: &WlSurface) -> Rectangle<i32, Logical> {
        Rectangle::default()
    }
}
