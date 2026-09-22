use smithay_client_toolkit::shm::{Shm, ShmHandler};

use crate::wayland::NiwoeShell;

impl ShmHandler for NiwoeShell {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}
