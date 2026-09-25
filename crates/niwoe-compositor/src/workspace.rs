use smithay::{
    desktop::{space::SpaceElement, Space, Window},
    output::Output,
    utils::{Logical, Point, Rectangle},
};

/// Per-workspace window spaces plus the active index.
///
/// Generic over the space element so the workspace/window-lifecycle logic can
/// be unit-tested with a mock element (the production type is `Window`, which
/// needs a live Wayland/X11 surface and cannot be built in tests). Only the
/// active workspace is rendered; all of them are searched when a window must
/// be located regardless of which workspace is currently shown.
pub struct WorkspaceManager<E: SpaceElement = Window> {
    spaces: Vec<Space<E>>,
    rooms: crate::room_registry::RoomRegistry,
    pub active: usize,
}

impl<E: SpaceElement + PartialEq> Default for WorkspaceManager<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: SpaceElement + PartialEq> WorkspaceManager<E> {
    pub fn new() -> Self {
        Self::with_rooms(
            crate::room_registry::RoomRegistry::from_definitions(
                niwoe_config::rooms::Rooms::from_legacy_slots(),
            )
            .expect("built-in legacy rooms are valid"),
        )
    }

    pub fn with_rooms(rooms: crate::room_registry::RoomRegistry) -> Self {
        Self {
            spaces: (0..rooms.slot_count()).map(|_| Space::default()).collect(),
            rooms,
            active: 0,
        }
    }

    pub fn rooms_mut(&mut self) -> &mut crate::room_registry::RoomRegistry {
        &mut self.rooms
    }

    pub fn rooms(&self) -> &crate::room_registry::RoomRegistry {
        &self.rooms
    }

    pub fn find_element_room<F>(&self, pred: F) -> Option<(niwoe_config::rooms::RoomId, &E)>
    where
        F: Fn(&E) -> bool,
    {
        self.find_element_workspace(pred)
            .and_then(|(slot, element)| self.rooms.room_at_slot(slot).map(|id| (id, element)))
    }

    pub fn count(&self) -> usize {
        self.spaces.len()
    }

    /// Keep the runtime Spaces aligned with the registry's persisted slot order.
    pub fn append_room_space(&mut self) {
        self.spaces.push(Space::default());
    }

    /// Move every mapped element to the chosen room, then compact slot indices.
    /// The registry has already validated and persisted the corresponding edit.
    pub fn remove_room_space(&mut self, source: usize, target: usize, outputs: &[Output])
    where
        E: Clone,
    {
        debug_assert!(source < self.spaces.len() && target < self.spaces.len() && source != target);
        let elements: Vec<_> = self.spaces[source]
            .elements()
            .map(|element| {
                (
                    element.clone(),
                    self.spaces[source]
                        .element_location(element)
                        .unwrap_or_default(),
                )
            })
            .collect();
        for (element, location) in elements {
            self.spaces[source].unmap_elem(&element);
            self.spaces[target].map_element(element, location, false);
        }
        for space in &mut self.spaces {
            for output in outputs {
                space.unmap_output(output);
            }
        }
        self.spaces.remove(source);
        self.active = if self.active == source {
            target - usize::from(target > source)
        } else {
            self.active - usize::from(self.active > source)
        };
        for output in outputs {
            self.spaces[self.active].map_output(output, (0, 0));
        }
    }

    pub fn active_space(&self) -> &Space<E> {
        &self.spaces[self.active]
    }

    pub fn active_space_mut(&mut self) -> &mut Space<E> {
        &mut self.spaces[self.active]
    }

    pub fn space_at(&self, idx: usize) -> &Space<E> {
        &self.spaces[idx]
    }

    pub fn space_at_mut(&mut self, idx: usize) -> &mut Space<E> {
        &mut self.spaces[idx]
    }

    /// Locate an element and its workspace index by predicate, searching
    /// **every** workspace (not just the active one). Window-lifecycle cleanup
    /// must use this so a window that lives on a non-active workspace is not
    /// missed (audit XW-1: active-only search left taskbar ghosts).
    pub fn find_element_workspace<F>(&self, pred: F) -> Option<(usize, &E)>
    where
        F: Fn(&E) -> bool,
    {
        (0..self.spaces.len()).find_map(|ws| {
            // `elements()` yields `&E`, so `find` hands the closure `&&E`;
            // deref once for `pred`.
            self.spaces[ws]
                .elements()
                .find(|candidate| pred(&**candidate))
                .map(|e| (ws, e))
        })
    }

    /// Rescue windows stranded off every live output (e.g. after a monitor
    /// is unplugged) by moving them onto `fallback`. `outputs` are the
    /// surviving output regions in global logical coords. Returns how many
    /// windows were relocated. No-op without outputs (nothing to clamp onto).
    /// Runs across every workspace, not just the active one.
    pub fn reclamp_offscreen_windows(
        &mut self,
        outputs: &[Rectangle<i32, Logical>],
        fallback: Point<i32, Logical>,
    ) -> usize
    where
        E: Clone,
    {
        if outputs.is_empty() {
            return 0;
        }
        let mut moved = 0;
        for space in &mut self.spaces {
            let strays: Vec<E> = space
                .elements()
                .filter(|w| match space.element_location(w) {
                    Some(loc) => {
                        let rect = Rectangle::new(loc, w.geometry().size);
                        !outputs.iter().any(|o| o.overlaps(rect))
                    }
                    None => false,
                })
                .cloned()
                .collect();
            for w in strays {
                space.map_element(w, fallback, false);
                moved += 1;
            }
        }
        moved
    }

    fn can_target_workspace(&self, idx: usize) -> bool {
        idx < self.spaces.len() && idx != self.active
    }

    /// Switch active workspace. Returns (old, new) if a switch occurred.
    pub fn try_switch(&mut self, idx: usize) -> Option<(usize, usize)> {
        if !self.can_target_workspace(idx) {
            return None;
        }
        let old = self.active;
        self.active = idx;
        Some((old, idx))
    }

    /// Move a window from the active workspace to `target`.
    pub fn move_window_to(&mut self, window: E, target: usize) {
        if !self.can_target_workspace(target) {
            return;
        }
        let active = self.active;
        let loc: Point<i32, Logical> = self.spaces[active]
            .element_location(&window)
            .unwrap_or_default();
        self.spaces[active].unmap_elem(&window);
        self.spaces[target].map_element(window, loc, false);
    }

    /// Remap all tracked outputs from `old` workspace to `new` workspace.
    pub fn remap_outputs(&mut self, outputs: &[Output], old: usize, new: usize) {
        for output in outputs {
            self.spaces[old].unmap_output(output);
            self.spaces[new].map_output(output, (0, 0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WorkspaceManager;
    use smithay::{
        desktop::space::SpaceElement,
        output::Output,
        utils::{IsAlive, Logical, Point, Rectangle},
    };

    /// Minimal SpaceElement for testing workspace/window bookkeeping without a
    /// live Wayland surface (which the real `Window` requires).
    #[derive(Clone, PartialEq, Debug)]
    struct MockWindow {
        id: u32,
    }

    impl IsAlive for MockWindow {
        fn alive(&self) -> bool {
            true
        }
    }

    impl SpaceElement for MockWindow {
        fn bbox(&self) -> Rectangle<i32, Logical> {
            Rectangle::new((0, 0).into(), (10, 10).into())
        }
        fn is_in_input_region(&self, _point: &Point<f64, Logical>) -> bool {
            false
        }
        fn set_activate(&self, _activated: bool) {}
        fn output_enter(&self, _output: &Output, _overlap: Rectangle<i32, Logical>) {}
        fn output_leave(&self, _output: &Output) {}
    }

    fn manager() -> WorkspaceManager<MockWindow> {
        WorkspaceManager::new()
    }

    #[test]
    fn try_switch_ignores_invalid_target() {
        let mut manager = manager();
        let old = manager.active;
        assert_eq!(manager.try_switch(99), None);
        assert_eq!(manager.active, old);
    }

    #[test]
    fn try_switch_ignores_current_workspace() {
        let mut manager = manager();
        let old = manager.active;
        assert_eq!(manager.try_switch(old), None);
        assert_eq!(manager.active, old);
    }

    #[test]
    fn try_switch_updates_active_workspace_on_valid_target() {
        let mut manager = manager();
        assert_eq!(manager.active, 0);
        assert_eq!(manager.try_switch(2), Some((0, 2)));
        assert_eq!(manager.active, 2);
    }

    #[test]
    fn move_guards_share_same_target_validation() {
        let manager = manager();
        assert!(!manager.can_target_workspace(0));
        assert!(!manager.can_target_workspace(99));
        assert!(manager.can_target_workspace(1));
    }

    // ── Window-lifecycle search/move semantics (audit XW-1 / GR-1 kernel) ──

    #[test]
    fn find_element_workspace_finds_window_on_non_active_workspace() {
        let mut m = manager();
        // Map a window into workspace 3 while active is 0.
        m.space_at_mut(3)
            .map_element(MockWindow { id: 7 }, (0, 0), false);
        // The all-workspace search must find it (active-only would miss it —
        // the bug XW-1 fixed).
        let found = m.find_element_workspace(|w| w.id == 7);
        assert_eq!(found.map(|(ws, w)| (ws, w.id)), Some((3, 7)));
    }

    #[test]
    fn find_element_workspace_is_none_for_absent_or_empty() {
        let mut m = manager();
        assert!(m.find_element_workspace(|_| true).is_none());
        m.space_at_mut(1)
            .map_element(MockWindow { id: 1 }, (0, 0), false);
        assert!(m.find_element_workspace(|w| w.id == 99).is_none());
    }

    #[test]
    fn window_room_identity_uses_space_slot_not_display_position() {
        use niwoe_config::rooms::{RoomId, Rooms};
        let mut definitions = Rooms::from_legacy_slots();
        definitions.rooms.reverse();
        definitions.rooms[8].name = "Arbeit".into();
        let registry = crate::room_registry::RoomRegistry::from_definitions(definitions).unwrap();
        let mut m = WorkspaceManager::with_rooms(registry);
        let window = MockWindow { id: 42 };
        m.space_at_mut(0)
            .map_element(window.clone(), (12, 34), false);
        assert_eq!(
            m.find_element_room(|w| w.id == 42).map(|(id, _)| id),
            Some(RoomId(1))
        );
        m.move_window_to(window.clone(), 8);
        assert_eq!(
            m.find_element_room(|w| w.id == 42).map(|(id, _)| id),
            Some(RoomId(9))
        );
        assert_eq!(
            m.space_at(8).element_location(&window),
            Some((12, 34).into())
        );
        assert_eq!(m.active, 0);
        assert!(m.space_at(0).elements().next().is_none());
    }

    #[test]
    fn move_window_to_relocates_and_leaves_no_duplicate() {
        let mut m = manager();
        let w = MockWindow { id: 1 };
        m.active_space_mut().map_element(w.clone(), (5, 5), false); // workspace 0
        assert_eq!(
            m.find_element_workspace(|x| x.id == 1).map(|(ws, _)| ws),
            Some(0)
        );
        m.move_window_to(w, 2);
        // Now present in workspace 2 only — not duplicated across workspaces.
        assert_eq!(
            m.find_element_workspace(|x| x.id == 1).map(|(ws, _)| ws),
            Some(2)
        );
        assert!(m.space_at(0).elements().next().is_none());
    }

    #[test]
    fn move_window_to_rejects_active_and_out_of_range() {
        let mut m = manager();
        let w = MockWindow { id: 5 };
        m.active_space_mut().map_element(w.clone(), (0, 0), false);
        m.move_window_to(w.clone(), 0); // active -> no-op
        m.move_window_to(w, 99); // out of range -> no-op
        assert_eq!(
            m.find_element_workspace(|x| x.id == 5).map(|(ws, _)| ws),
            Some(0)
        );
    }

    #[test]
    fn removing_room_moves_windows_and_keeps_other_room_identity() {
        let mut m = manager();
        let moved = MockWindow { id: 5 };
        let later = MockWindow { id: 6 };
        m.space_at_mut(2)
            .map_element(moved.clone(), (13, 17), false);
        m.space_at_mut(5)
            .map_element(later.clone(), (29, 31), false);
        m.active = 5;
        m.remove_room_space(2, 0, &[]);
        assert_eq!(m.count(), 8);
        assert_eq!(m.active, 4);
        assert_eq!(
            m.find_element_workspace(|w| w.id == 5)
                .map(|(slot, _)| slot),
            Some(0)
        );
        assert_eq!(
            m.find_element_workspace(|w| w.id == 6)
                .map(|(slot, _)| slot),
            Some(4)
        );
        assert_eq!(
            m.space_at(0).element_location(&moved),
            Some((13, 17).into())
        );
    }

    fn output(x: i32, w: i32) -> Rectangle<i32, Logical> {
        Rectangle::new((x, 0).into(), (w, 1080).into())
    }

    #[test]
    fn reclamp_moves_window_stranded_off_all_outputs() {
        let mut m = manager();
        // Window at x=2500 was on a now-removed second monitor; only the
        // 0..1920 output survives.
        m.space_at_mut(0)
            .map_element(MockWindow { id: 1 }, (2500, 100), false);
        let moved = m.reclamp_offscreen_windows(&[output(0, 1920)], (0, 0).into());
        assert_eq!(moved, 1);
        assert_eq!(
            m.space_at(0).element_location(&MockWindow { id: 1 }),
            Some((0, 0).into())
        );
    }

    #[test]
    fn reclamp_leaves_onscreen_windows_untouched() {
        let mut m = manager();
        m.space_at_mut(0)
            .map_element(MockWindow { id: 2 }, (100, 100), false);
        let moved = m.reclamp_offscreen_windows(&[output(0, 1920)], (0, 0).into());
        assert_eq!(moved, 0);
        assert_eq!(
            m.space_at(0).element_location(&MockWindow { id: 2 }),
            Some((100, 100).into())
        );
    }

    #[test]
    fn reclamp_runs_across_all_workspaces() {
        let mut m = manager();
        m.space_at_mut(0)
            .map_element(MockWindow { id: 1 }, (3000, 0), false);
        m.space_at_mut(4)
            .map_element(MockWindow { id: 2 }, (3000, 0), false);
        assert_eq!(
            m.reclamp_offscreen_windows(&[output(0, 1920)], (10, 10).into()),
            2
        );
    }

    #[test]
    fn reclamp_without_outputs_is_noop() {
        let mut m = manager();
        m.space_at_mut(0)
            .map_element(MockWindow { id: 1 }, (3000, 0), false);
        assert_eq!(m.reclamp_offscreen_windows(&[], (0, 0).into()), 0);
        assert_eq!(
            m.space_at(0).element_location(&MockWindow { id: 1 }),
            Some((3000, 0).into())
        );
    }
}
