use std::borrow::Cow;

use smithay::{
    backend::input::KeyState,
    input::{
        keyboard::{KeyboardTarget, KeysymHandle, ModifiersState},
        Seat,
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{IsAlive, Serial},
    wayland::seat::WaylandFocus,
    xwayland::X11Surface,
};

use super::NiwoeState;

/// Keep the existing surface identity while dispatching X11 keyboard focus
/// through Smithay's ICCCM-aware target (SetInputFocus / WM_TAKE_FOCUS).
#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardFocusTarget {
    surface: WlSurface,
    x11: Option<X11Surface>,
}

impl KeyboardFocusTarget {
    pub fn surface(&self) -> &WlSurface {
        &self.surface
    }

    pub fn into_surface(self) -> WlSurface {
        self.surface
    }

    fn target(&self) -> &dyn KeyboardTarget<NiwoeState> {
        match &self.x11 {
            Some(surface) => surface,
            None => &self.surface,
        }
    }
}

impl NiwoeState {
    pub(super) fn keyboard_target(&self, surface: WlSurface) -> KeyboardFocusTarget {
        // Lock and layer surfaces remain native. Never resolve an application
        // target while the compositor owns a pending or acquired session lock.
        let x11 = if self.lock_manager.is_locked_or_pending() {
            None
        } else {
            (0..self.workspaces.count()).find_map(|index| {
                self.workspaces
                    .space_at(index)
                    .elements()
                    .find_map(|window| {
                        window
                            .x11_surface()
                            .filter(|x11| x11.wl_surface().as_ref() == Some(&surface))
                            .cloned()
                    })
            })
        };
        KeyboardFocusTarget { surface, x11 }
    }
}

impl From<WlSurface> for KeyboardFocusTarget {
    fn from(surface: WlSurface) -> Self {
        Self { surface, x11: None }
    }
}

impl From<smithay::desktop::PopupKind> for KeyboardFocusTarget {
    fn from(popup: smithay::desktop::PopupKind) -> Self {
        popup.wl_surface().clone().into()
    }
}

impl From<KeyboardFocusTarget> for WlSurface {
    fn from(target: KeyboardFocusTarget) -> Self {
        target.surface
    }
}

impl IsAlive for KeyboardFocusTarget {
    fn alive(&self) -> bool {
        self.surface.alive()
    }
}

impl WaylandFocus for KeyboardFocusTarget {
    fn wl_surface(&self) -> Option<Cow<'_, WlSurface>> {
        Some(Cow::Borrowed(&self.surface))
    }
}

impl KeyboardTarget<NiwoeState> for KeyboardFocusTarget {
    fn enter(
        &self,
        seat: &Seat<NiwoeState>,
        data: &mut NiwoeState,
        keys: Vec<KeysymHandle<'_>>,
        serial: Serial,
    ) {
        self.target().enter(seat, data, keys, serial);
    }

    fn leave(&self, seat: &Seat<NiwoeState>, data: &mut NiwoeState, serial: Serial) {
        self.target().leave(seat, data, serial);
    }

    fn key(
        &self,
        seat: &Seat<NiwoeState>,
        data: &mut NiwoeState,
        key: KeysymHandle<'_>,
        state: KeyState,
        serial: Serial,
        time: u32,
    ) {
        self.target().key(seat, data, key, state, serial, time);
    }

    fn modifiers(
        &self,
        seat: &Seat<NiwoeState>,
        data: &mut NiwoeState,
        modifiers: ModifiersState,
        serial: Serial,
    ) {
        self.target().modifiers(seat, data, modifiers, serial);
    }
}
