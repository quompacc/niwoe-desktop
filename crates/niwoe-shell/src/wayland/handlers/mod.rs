use smithay_client_toolkit::{
    delegate_compositor, delegate_keyboard, delegate_layer, delegate_output, delegate_pointer,
    delegate_registry, delegate_seat, delegate_shm,
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
};

use super::NiwoeShell;

mod compositor;
mod keyboard;
mod layer;
mod output;
mod pointer;
mod pointer_state;
mod pointer_translate;
mod seat;
mod shm;
mod widget_dispatch;

delegate_compositor!(NiwoeShell);
delegate_output!(NiwoeShell);
delegate_shm!(NiwoeShell);
delegate_seat!(NiwoeShell);
delegate_keyboard!(NiwoeShell);
delegate_pointer!(NiwoeShell);
delegate_layer!(NiwoeShell);
delegate_registry!(NiwoeShell);

impl ProvidesRegistryState for NiwoeShell {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers![OutputState, SeatState];
}
