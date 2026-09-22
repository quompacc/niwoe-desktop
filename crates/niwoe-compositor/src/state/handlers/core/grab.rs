use smithay::{
    input::{pointer::GrabStartData as PointerGrabStartData, Seat},
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Resource},
    utils::Serial,
};

use super::super::super::NiwoeState;

pub(crate) fn check_grab(
    seat: &Seat<NiwoeState>,
    surface: &WlSurface,
    serial: Serial,
) -> Option<PointerGrabStartData<NiwoeState>> {
    let pointer = seat.get_pointer()?;
    if !pointer.has_grab(serial) {
        return None;
    }
    let start_data = pointer.grab_start_data()?;
    let (focus, _) = start_data.focus.as_ref()?;
    if !focus.id().same_client_as(&surface.id()) {
        return None;
    }
    Some(start_data)
}
