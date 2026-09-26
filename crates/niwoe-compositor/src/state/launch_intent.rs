//! Bounded, single-use launch correlation. No PID/app-ID guessing or polling.
use std::{sync::Mutex, time::Duration};

use niwoe_config::rooms::RoomId;
use smithay::{
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::{compositor::with_states, xdg_activation::XdgActivationTokenData},
};

use super::NiwoeState;

const LIFETIME: Duration = Duration::from_secs(60);
const MAX_PENDING: usize = 128;

#[derive(Clone, Copy)]
struct LaunchRoom(RoomId);

#[derive(Default)]
struct SurfaceIntent(Mutex<Option<RoomId>>);

pub(super) fn surface_room(surface: &WlSurface) -> Option<RoomId> {
    with_states(surface, |states| {
        states
            .data_map
            .get::<SurfaceIntent>()
            .and_then(|intent| *intent.0.lock().unwrap())
    })
}

impl NiwoeState {
    pub(crate) fn prune_launch_tokens(&mut self) -> bool {
        self.xdg_activation_state
            .retain_tokens(|_, data| data.timestamp.elapsed() < LIFETIME);
        self.xdg_activation_state.tokens().count() < MAX_PENDING
    }

    pub(crate) fn launch_room_token(&mut self, room: RoomId) -> Option<String> {
        if self.workspaces.rooms().slot_for_room(room).is_none() || !self.prune_launch_tokens() {
            return None;
        }
        let data = XdgActivationTokenData::default();
        data.user_data.insert_if_missing(|| LaunchRoom(room));
        Some(
            self.xdg_activation_state
                .create_external_token(data)
                .0
                .as_str()
                .to_owned(),
        )
    }

    pub(super) fn consume_launch_room(&mut self, token: &str) -> Option<RoomId> {
        let token = token.to_owned().into();
        let room = self
            .xdg_activation_state
            .data_for_token(&token)
            .and_then(|data| {
                valid_room(data, |room| {
                    self.workspaces.rooms().slot_for_room(room).is_some()
                })
            });
        self.xdg_activation_state.remove_token(&token);
        room
    }

    pub(crate) fn apply_launch_activation(&mut self, token: &str, surface: &WlSurface) {
        let Some(room) = self.consume_launch_room(token) else {
            return;
        };
        with_states(surface, |states| {
            states.data_map.insert_if_missing(SurfaceIntent::default);
            *states
                .data_map
                .get::<SurfaceIntent>()
                .unwrap()
                .0
                .lock()
                .unwrap() = Some(room);
        });
        // Also covers activation before a toplevel exists: assignment reads the
        // surface metadata again on the normal lifecycle callbacks.
        self.assign_xdg_surface(surface);
    }
}

fn valid_room(
    data: &XdgActivationTokenData,
    exists: impl FnOnce(RoomId) -> bool,
) -> Option<RoomId> {
    let room = data.user_data.get::<LaunchRoom>()?.0;
    (data.timestamp.elapsed() < LIFETIME && exists(room)).then_some(room)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_live_server_intents_for_existing_rooms_are_accepted() {
        let mut data = XdgActivationTokenData {
            app_id: Some("not-a-launch-identity".into()),
            ..Default::default()
        };
        assert_eq!(valid_room(&data, |_| true), None);
        data.user_data.insert_if_missing(|| LaunchRoom(RoomId(42)));
        assert_eq!(valid_room(&data, |_| true), Some(RoomId(42)));
        assert_eq!(valid_room(&data, |_| false), None);
        data.timestamp -= LIFETIME;
        assert_eq!(valid_room(&data, |_| true), None);
    }
}
