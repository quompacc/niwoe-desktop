//! Initial room selection. Identifiers are exact and protocol-specific;
//! neither titles, case folding nor desktop-file suffix guesses are identities.
use niwoe_config::rooms::{AppReference, AssignmentMode, RoomId, Rooms};

pub(crate) fn choose_room(
    rooms: &Rooms,
    parent: Option<RoomId>,
    explicit: Option<RoomId>,
    identity: Option<&AppReference>,
    fallback: RoomId,
) -> RoomId {
    let exists = |id: &RoomId| rooms.rooms.iter().any(|room| room.id == *id);
    parent
        .filter(exists)
        .or_else(|| explicit.filter(exists))
        .or_else(|| {
            let identity = identity?;
            rooms
                .rooms
                .iter()
                .find(|room| {
                    room.assignment != AssignmentMode::Free
                        && room.preferences.apps.contains(identity)
                })
                .map(|room| room.id)
        })
        .unwrap_or(fallback)
}

/// Empty metadata means unresolved, not an application rule.
pub(crate) fn identity(value: String, native: bool) -> Option<AppReference> {
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        return None;
    }
    Some(if native {
        AppReference::Native(value)
    } else {
        AppReference::Xwayland(value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rooms() -> Rooms {
        let mut rooms = Rooms::from_legacy_slots();
        for room in &mut rooms.rooms[1..3] {
            room.assignment = AssignmentMode::Preferred;
            room.preferences.apps = vec![AppReference::Native("org.example.App".into())];
        }
        rooms.rooms[2].assignment = AssignmentMode::Dedicated;
        rooms
    }

    #[test]
    fn priorities_and_saved_order_use_stable_ids() {
        let mut rooms = rooms();
        let app = identity("org.example.App".into(), true);
        let choose = |rooms: &Rooms, parent, explicit| {
            choose_room(rooms, parent, explicit, app.as_ref(), RoomId(1))
        };
        assert_eq!(choose(&rooms, None, None), RoomId(2));
        rooms.rooms.swap(1, 2);
        assert_eq!(choose(&rooms, None, None), RoomId(3));
        assert_eq!(choose(&rooms, None, Some(RoomId(4))), RoomId(4));
        assert_eq!(choose(&rooms, Some(RoomId(5)), Some(RoomId(4))), RoomId(5));
        assert_eq!(
            choose(&rooms, Some(RoomId(99)), Some(RoomId(99))),
            RoomId(3)
        );
    }

    #[test]
    fn identities_are_exact_and_protocol_specific() {
        let mut rooms = rooms();
        for app in [
            None,
            identity("org.example.app".into(), true),
            identity("org.example.App".into(), false),
        ] {
            assert_eq!(
                choose_room(&rooms, None, None, app.as_ref(), RoomId(1)),
                RoomId(1)
            );
        }
        rooms.rooms[0]
            .preferences
            .apps
            .push(AppReference::Native("org.example.App".into()));
        let app = identity("org.example.App".into(), true);
        assert_eq!(
            choose_room(&rooms, None, None, app.as_ref(), RoomId(1)),
            RoomId(2)
        );
        rooms.rooms[3].assignment = AssignmentMode::Dedicated;
        rooms.rooms[3]
            .preferences
            .apps
            .push(AppReference::Xwayland("Example".into()));
        let x11 = identity("Example".into(), false);
        assert_eq!(
            choose_room(&rooms, None, None, x11.as_ref(), RoomId(1)),
            RoomId(4)
        );
        // Dedicated is not an admission filter, including for unknown apps.
        assert_eq!(choose_room(&rooms, None, None, None, RoomId(4)), RoomId(4));
        assert!(identity(String::new(), true).is_none());
        assert!(identity(" Example".into(), false).is_none());
    }
}
