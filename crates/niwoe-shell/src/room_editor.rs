use niwoe_ipc::{RoomChange, RoomEntry, RoomSnapshot};
use std::time::{Duration, Instant};

pub(crate) struct Edit {
    pub id: u64,
    pub revision: u64,
    pub name: String,
    pub description: String,
    pub assignment: niwoe_ipc::RoomAssignment,
    pub delete_target: Option<u64>,
    pub target_menu: Option<u64>,
    pub confirm_delete: bool,
    pub creation_uncertain: bool,
    pub replace: bool,
    pub focus: usize,
}

pub(crate) struct RoomUi {
    pub snapshot: RoomSnapshot,
    pub ready: bool,
    pub edit: Option<Edit>,
    pub pending: Option<(String, Instant)>,
    pub message: String,
    serial: u64,
}

impl Default for RoomUi {
    fn default() -> Self {
        Self {
            snapshot: RoomSnapshot {
                revision: 0,
                rooms: (1..=9)
                    .map(|workspace| RoomEntry {
                        preferences: Default::default(),
                        id: workspace as u64,
                        workspace,
                        name: format!("Raum {workspace}"),
                        description: String::new(),
                        assignment: niwoe_ipc::RoomAssignment::Free,
                    })
                    .collect(),
            },
            ready: false,
            edit: None,
            pending: None,
            message: String::new(),
            serial: 0,
        }
    }
}

impl RoomUi {
    pub fn enabled(&self, action: crate::wayland::RoomEditAction) -> bool {
        use crate::wayland::RoomEditAction;
        if self.pending.is_some() {
            return false;
        }
        let Some(edit) = &self.edit else {
            return false;
        };
        if edit.id == 0 {
            if action == RoomEditAction::Save && edit.creation_uncertain {
                return false;
            }
            return !matches!(
                action,
                RoomEditAction::Left
                    | RoomEditAction::Right
                    | RoomEditAction::Delete
                    | RoomEditAction::Target
            );
        }
        let Some(position) = self.snapshot.rooms.iter().position(|r| r.id == edit.id) else {
            return false;
        };
        match action {
            RoomEditAction::Left => position > 0,
            RoomEditAction::Right => position + 1 < self.snapshot.rooms.len(),
            RoomEditAction::Delete | RoomEditAction::Target => self.snapshot.rooms.len() > 1,
            _ => true,
        }
    }

    pub fn accept(&mut self, snapshot: RoomSnapshot) -> bool {
        let ids: std::collections::HashSet<_> = snapshot.rooms.iter().map(|r| r.id).collect();
        let slots: std::collections::HashSet<_> =
            snapshot.rooms.iter().map(|r| r.workspace).collect();
        let count = snapshot.rooms.len();
        if !(1..=niwoe_config::rooms::MAX_ROOMS).contains(&count)
            || ids.len() != count
            || slots.len() != count
            || snapshot.rooms.iter().any(|r| {
                r.id == 0
                    || !(1..=count as u8).contains(&r.workspace)
                    || r.name.trim().is_empty()
                    || r.name.chars().count() > 64
                    || r.name.chars().any(char::is_control)
                    || r.description.chars().count() > niwoe_config::rooms::MAX_DESCRIPTION_CHARS
                    || r.description.chars().any(|c| c.is_control() && c != '\n')
                    || !valid_preferences(&r.preferences)
            })
            || self.ready && snapshot.revision < self.snapshot.revision
        {
            return false;
        }
        self.ready = true;
        if let Some(edit) = &mut self.edit {
            if edit.delete_target.is_some_and(|id| !ids.contains(&id)) {
                edit.delete_target = None;
                edit.confirm_delete = false;
            }
            if edit.target_menu.is_some_and(|id| !ids.contains(&id)) {
                edit.target_menu = None;
            }
        }
        self.snapshot = snapshot;
        true
    }
    pub fn begin(&mut self, workspace: u8) {
        if !self.ready || self.pending.is_some() {
            return;
        }
        if let Some(room) = self
            .snapshot
            .rooms
            .iter()
            .find(|r| r.workspace == workspace)
        {
            self.edit = Some(Edit {
                id: room.id,
                revision: self.snapshot.revision,
                name: room.name.clone(),
                description: room.description.clone(),
                assignment: room.assignment,
                delete_target: None,
                target_menu: None,
                confirm_delete: false,
                creation_uncertain: false,
                replace: true,
                focus: 0,
            });
            self.message.clear();
        }
    }
    pub fn begin_create(&mut self) -> bool {
        if !self.ready
            || self.pending.is_some()
            || self.snapshot.rooms.len() >= niwoe_config::rooms::MAX_ROOMS
        {
            return false;
        }
        self.edit = Some(Edit {
            id: 0,
            revision: self.snapshot.revision,
            name: String::new(),
            description: String::new(),
            assignment: niwoe_ipc::RoomAssignment::Free,
            delete_target: None,
            target_menu: None,
            confirm_delete: false,
            creation_uncertain: false,
            replace: true,
            focus: 0,
        });
        self.message.clear();
        true
    }
    pub fn request(
        &mut self,
        action: crate::wayland::RoomEditAction,
    ) -> Option<niwoe_ipc::ShellCommand> {
        use crate::wayland::RoomEditAction;
        if self.pending.is_some() {
            return None;
        }
        if action == RoomEditAction::Cancel {
            self.edit = None;
            self.message.clear();
            return None;
        }
        let edit = self.edit.as_mut()?;
        if action == RoomEditAction::Save && edit.creation_uncertain {
            return None;
        }
        if action == RoomEditAction::Name {
            edit.focus = 0;
            return None;
        }
        let change = match action {
            RoomEditAction::Delete => {
                let Some(target_id) = edit.delete_target else {
                    self.message =
                        "Bitte wählen, in welchen Raum die offenen Fenster wechseln sollen.".into();
                    return None;
                };
                if !edit.confirm_delete {
                    edit.confirm_delete = true;
                    let target = self
                        .snapshot
                        .rooms
                        .iter()
                        .find(|room| room.id == target_id)?;
                    self.message = format!(
                        "Raum löschen bestätigen: Offene Fenster wechseln nach „{}“.",
                        target.name
                    );
                    return None;
                }
                RoomChange::Delete {
                    id: edit.id,
                    target_id,
                }
            }
            RoomEditAction::Target => {
                edit.target_menu = if edit.target_menu.is_some() {
                    None
                } else {
                    edit.delete_target.or_else(|| {
                        self.snapshot
                            .rooms
                            .iter()
                            .find(|r| r.id != edit.id)
                            .map(|r| r.id)
                    })
                };
                edit.confirm_delete = false;
                self.message.clear();
                return None;
            }
            RoomEditAction::Save if edit.id == 0 => RoomChange::CreateDetails {
                name: edit.name.trim().to_owned(),
                description: edit.description.clone(),
                assignment: edit.assignment,
            },
            RoomEditAction::Save
                if self
                    .snapshot
                    .rooms
                    .iter()
                    .find(|r| r.id == edit.id)
                    .is_some_and(|r| r.description != edit.description) =>
            {
                RoomChange::UpdateDetails {
                    id: edit.id,
                    name: edit.name.trim().to_owned(),
                    description: edit.description.clone(),
                }
            }
            RoomEditAction::Save => RoomChange::Rename {
                id: edit.id,
                name: edit.name.trim().to_owned(),
            },
            RoomEditAction::Left | RoomEditAction::Right => {
                let position = self.snapshot.rooms.iter().position(|r| r.id == edit.id)?;
                if self.snapshot.rooms[position].name != edit.name.trim()
                    || self.snapshot.rooms[position].description != edit.description
                {
                    self.message = "Änderungen bitte zuerst speichern.".into();
                    return None;
                }
                let next = if action == RoomEditAction::Left {
                    position.checked_sub(1)?
                } else {
                    (position + 1 < self.snapshot.rooms.len()).then_some(position + 1)?
                };
                RoomChange::Move {
                    id: edit.id,
                    position: next,
                }
            }
            _ => return None,
        };
        self.serial += 1;
        let request_id = format!(
            "room-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            self.serial
        );
        self.pending = Some((request_id.clone(), Instant::now()));
        self.message = "Wird gespeichert …".into();
        Some(niwoe_ipc::ShellCommand::MutateRoom {
            request_id,
            expected_revision: edit.revision,
            change,
        })
    }
    pub fn result(&mut self, id: &str, error: Option<niwoe_ipc::RoomMutationError>) {
        if !self
            .pending
            .as_ref()
            .is_some_and(|(pending, _)| pending == id)
        {
            return;
        }
        self.pending = None;
        if let Some(edit) = &mut self.edit {
            edit.revision = self.snapshot.revision;
            edit.confirm_delete = false;
            if edit.id == 0 && error == Some(niwoe_ipc::RoomMutationError::Durability) {
                edit.creation_uncertain = true;
            }
        }
        self.message = match error {
            None => "Gespeichert",
            Some(niwoe_ipc::RoomMutationError::Conflict) => {
                "Zwischenzeitlich geändert. Erneut versuchen."
            }
            Some(niwoe_ipc::RoomMutationError::Invalid) => {
                "Ungültige Raumdaten oder Zielraum nicht verfügbar."
            }
            Some(niwoe_ipc::RoomMutationError::Storage) => {
                "Speichern fehlgeschlagen. Erneut versuchen."
            }
            Some(niwoe_ipc::RoomMutationError::Durability) => {
                "Gespeichert; Datenträgerprüfung fehlgeschlagen."
            }
        }
        .into();
    }
    pub fn expire(&mut self) -> bool {
        if self
            .pending
            .as_ref()
            .is_some_and(|(_, at)| at.elapsed() >= Duration::from_secs(10))
        {
            self.pending = None;
            self.message = "Keine Bestätigung. Stand wird neu geladen.".into();
            if let Some(edit) = self.edit.as_mut().filter(|edit| edit.id == 0) {
                edit.creation_uncertain = true;
                self.message =
                    "Keine Bestätigung. Zurück zur Raumliste und Ergebnis prüfen.".into();
            }
            return true;
        }
        false
    }
}

/// Reuse config validation at the snapshot boundary; no second shell policy.
/// This conversion runs on room events only, never on the rendering path.
fn valid_preferences(preferences: &niwoe_ipc::RoomPreferences) -> bool {
    serde_json::to_value(preferences)
        .and_then(serde_json::from_value::<niwoe_config::rooms::RoomPreferences>)
        .is_ok_and(|preferences| preferences.validate().is_ok())
}

impl RoomUi {
    pub fn move_target_selection(&mut self, delta: isize) {
        if self.pending.is_some() {
            return;
        }
        let Some(edit) = &mut self.edit else {
            return;
        };
        let Some(selected) = edit.target_menu else {
            return;
        };
        let targets: Vec<_> = self
            .snapshot
            .rooms
            .iter()
            .filter(|r| r.id != edit.id)
            .collect();
        let Some(index) = targets.iter().position(|r| r.id == selected) else {
            return;
        };
        let next = index.saturating_add_signed(delta).min(targets.len() - 1);
        edit.target_menu = Some(targets[next].id);
    }

    pub fn choose_target(&mut self, id: u64) {
        if self.pending.is_some() {
            return;
        }
        let Some(edit) = &mut self.edit else {
            return;
        };
        if edit.target_menu.is_none()
            || id == edit.id
            || !self.snapshot.rooms.iter().any(|r| r.id == id)
        {
            return;
        }
        edit.delete_target = Some(id);
        edit.target_menu = None;
        edit.confirm_delete = false;
        self.message.clear();
    }
}

#[path = "room_editor_draw.rs"]
mod draw;
pub(crate) use draw::draw;

#[cfg(test)]
#[path = "room_editor_tests.rs"]
mod tests;
