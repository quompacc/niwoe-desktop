//! Bounded local form state; no filesystem work, launches or persistent writes.
use niwoe_ipc::{AppReference, RoomAssignment, RoomRestore};

pub(crate) const ICONS: &[(&str, &str)] = &[
    ("", "Standard"),
    ("folder", "Ordner"),
    ("applications-development", "Entwicklung"),
    ("applications-office", "Büro"),
    ("applications-multimedia", "Medien"),
];

#[derive(Default)]
pub(crate) struct FormUi {
    pub tab: usize,
    pub page: usize,
    pub query: String,
    pub apps: Vec<(String, AppReference)>,
    pub error: String,
}

impl super::Edit {
    /// Traverse only controls with a visible, available action in this draft.
    pub fn focus_order(&self, room_count: usize) -> Vec<usize> {
        let mut order = vec![30, 31];
        if self.id != 0 {
            order.extend([32, 33]);
        }
        if self.form.tab == 1 {
            order.push(14);
            if self.form.page > 0 {
                order.push(15);
            }
            let rows = self.app_rows();
            if (self.form.page + 1) * 6 < rows.len() {
                order.push(16);
            }
            order.extend(20..20 + rows.len().saturating_sub(self.form.page * 6).min(6));
        } else {
            order.extend([9, 0, 5]);
            if self.id != 0 && self.position > 0 {
                order.push(1);
            }
            if self.id != 0 && self.position + 1 < room_count {
                order.push(2);
            }
            order.extend([10, 11]);
            if self.id != 0 {
                order.push(8);
            }
            order.push(12);
            if self.id != 0 {
                order.push(13);
                if room_count > 1 {
                    order.extend([6, 7]);
                }
            }
        }
        order.extend([3, 4]);
        order
    }

    pub fn catalog(&mut self, apps: &[crate::launcher::DesktopApp]) {
        self.form.apps.clear();
        for app in apps.iter().take(4096) {
            if let Some(id) = app.desktop_id.strip_suffix(".desktop") {
                self.form.apps.push((
                    format!("{} · Native", app.name),
                    AppReference::Native(id.into()),
                ));
            }
            if let Some(id) = &app.startup_wm_class {
                self.form.apps.push((
                    format!("{} · XWayland", app.name),
                    AppReference::Xwayland(id.clone()),
                ));
            }
        }
        for reference in &self.preferences.apps {
            if !self.form.apps.iter().any(|(_, r)| r == reference) {
                let id = match reference {
                    AppReference::Native(id) | AppReference::Xwayland(id) => id,
                };
                self.form
                    .apps
                    .push((format!("Nicht im Katalog: {id}"), reference.clone()));
            }
        }
    }

    pub fn app_rows(&self) -> Vec<usize> {
        let query = self.form.query.to_lowercase();
        self.form
            .apps
            .iter()
            .enumerate()
            .filter(|(_, (label, _))| label.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn form_action(&mut self, control: usize) {
        self.focus = control;
        self.form.error.clear();
        match control {
            9 => {
                let index = ICONS
                    .iter()
                    .position(|(id, _)| Some(*id) == self.preferences.icon.as_deref())
                    .unwrap_or(0);
                let next = ICONS[(index + 1) % ICONS.len()].0;
                self.preferences.icon = (!next.is_empty()).then(|| next.into());
            }
            10 => {
                self.assignment = match self.assignment {
                    RoomAssignment::Free => RoomAssignment::Preferred,
                    RoomAssignment::Preferred => RoomAssignment::Dedicated,
                    RoomAssignment::Dedicated => RoomAssignment::Free,
                }
            }
            11 => {
                self.preferences.restore = match self.preferences.restore {
                    RoomRestore::Disabled => RoomRestore::LayoutOnly,
                    RoomRestore::LayoutOnly => RoomRestore::RelaunchApps,
                    RoomRestore::RelaunchApps => RoomRestore::Disabled,
                }
            }
            12 => self.form.tab = 1,
            15 => self.form.page = self.form.page.saturating_sub(1),
            16 => {
                self.form.page =
                    (self.form.page + 1).min(self.app_rows().len().saturating_sub(1) / 6)
            }
            20..=25 => {
                if let Some(index) = self.app_rows().get(self.form.page * 6 + control - 20) {
                    let reference = &self.form.apps[*index].1;
                    if let Some(index) = self.preferences.apps.iter().position(|r| r == reference) {
                        self.preferences.apps.remove(index);
                    } else if self.preferences.apps.len() < 64 {
                        self.preferences.apps.push(reference.clone());
                    } else {
                        self.form.error = "Maximal 64 App-Referenzen pro Raum.".into();
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyboard_traversal_skips_unsaved_actions_empty_rows_and_unavailable_pages() {
        let mut ui = super::super::RoomUi::default();
        ui.accept(ui.snapshot.clone());
        assert!(ui.begin_create());
        let edit = ui.edit.as_mut().unwrap();
        let order = edit.focus_order(9);
        for unavailable in [1, 2, 6, 7, 8, 13, 32, 33] {
            assert!(!order.contains(&unavailable));
        }
        assert!(order.contains(&9) && order.contains(&12));
        edit.form.tab = 1;
        edit.catalog(&[]);
        assert_eq!(edit.focus_order(9), vec![30, 31, 14, 3, 4]);
        edit.preferences
            .apps
            .push(AppReference::Native("missing".into()));
        edit.catalog(&[]);
        assert!(edit.focus_order(9).contains(&20));
        assert!(!edit.focus_order(9).contains(&21));
        edit.form.query = "no-match".into();
        assert!(!edit.focus_order(9).contains(&20));
        assert!(ui
            .snapshot
            .rooms
            .iter()
            .all(|room| room.preferences.apps.is_empty()));
    }

    #[test]
    fn duplicate_native_and_xwayland_choices_keep_independent_draft_selection() {
        let mut app =
            crate::launcher::DesktopApp::new("Editor".into(), vec!["editor".into()], false);
        app.desktop_id = "org.example.Editor.desktop".into();
        app.startup_wm_class = Some("EditorWindow".into());
        let mut ui = super::super::RoomUi::default();
        ui.accept(ui.snapshot.clone());
        ui.begin(1);
        let edit = ui.edit.as_mut().unwrap();
        edit.catalog(&[app]);
        edit.form.tab = 1;
        assert_eq!(edit.app_rows().len(), 2);
        assert!(edit.focus_order(9).contains(&21));
        assert!(!edit.focus_order(9).contains(&22));
        edit.form_action(20);
        edit.form_action(21);
        assert_eq!(edit.preferences.apps.len(), 2);
        edit.form_action(20);
        assert_eq!(
            edit.preferences.apps,
            vec![AppReference::Xwayland("EditorWindow".into())]
        );
        assert!(ui.snapshot.rooms[0].preferences.apps.is_empty());
    }

    #[test]
    fn missing_catalog_entries_remain_visible_removable_and_cancelled_locally() {
        let mut ui = super::super::RoomUi::default();
        ui.accept(ui.snapshot.clone());
        ui.begin(1);
        let edit = ui.edit.as_mut().unwrap();
        edit.preferences
            .apps
            .push(AppReference::Native("org.example.Gone".into()));
        edit.catalog(&[]);
        assert!(edit.form.apps[0].0.contains("Nicht im Katalog"));
        edit.form_action(20);
        assert!(edit.preferences.apps.is_empty());
        edit.form_action(9);
        edit.form_action(10);
        edit.form_action(11);
        assert_eq!(edit.preferences.icon.as_deref(), Some("folder"));
        assert_eq!(edit.assignment, RoomAssignment::Preferred);
        assert_eq!(edit.preferences.restore, RoomRestore::LayoutOnly);
        assert!(ui.snapshot.rooms[0].preferences.apps.is_empty());
        assert_eq!(ui.snapshot.rooms[0].preferences.icon, None);
    }
}
