use niwoe_ipc::{RoomChange, RoomEntry, RoomSnapshot};
use std::time::{Duration, Instant};

pub(crate) struct Edit {
    pub id: u64,
    pub revision: u64,
    pub name: String,
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
        let Some(position) = self.snapshot.rooms.iter().position(|r| r.id == edit.id) else {
            return false;
        };
        match action {
            RoomEditAction::Left => position > 0,
            RoomEditAction::Right => position + 1 < self.snapshot.rooms.len(),
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
            })
            || self.ready && snapshot.revision < self.snapshot.revision
        {
            return false;
        }
        self.ready = true;
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
                replace: true,
                focus: 0,
            });
            self.message.clear();
        }
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
        if action == RoomEditAction::Name {
            edit.focus = 0;
            return None;
        }
        let change = match action {
            RoomEditAction::Save => RoomChange::Rename {
                id: edit.id,
                name: edit.name.trim().to_owned(),
            },
            RoomEditAction::Left | RoomEditAction::Right => {
                let position = self.snapshot.rooms.iter().position(|r| r.id == edit.id)?;
                if self.snapshot.rooms[position].name != edit.name.trim() {
                    self.message = "Namen bitte zuerst speichern.".into();
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
        }
        self.message = match error {
            None => "Gespeichert",
            Some(niwoe_ipc::RoomMutationError::Conflict) => {
                "Zwischenzeitlich geändert. Erneut versuchen."
            }
            Some(niwoe_ipc::RoomMutationError::Invalid) => "Bitte einen gültigen Namen eingeben.",
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
            return true;
        }
        false
    }
}

#[path = "room_editor_draw.rs"]
mod draw;
pub(crate) use draw::draw;

#[cfg(test)]
#[path = "room_editor_tests.rs"]
mod tests;
