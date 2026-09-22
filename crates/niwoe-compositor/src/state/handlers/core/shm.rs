use smithay::wayland::shm::{ShmHandler, ShmState};

use super::super::super::NiwoeState;

impl ShmHandler for NiwoeState {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}
