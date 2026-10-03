use crate::state::NiwoeState;
use niwoe_config::{
    first_run::{sync_document, FirstRun, Journal},
    panel_preferences::PanelPreferences,
};
use niwoe_ipc::{FirstRunAction, FirstRunDraft, FirstRunSnapshot, ShellEvent};
use std::path::Path;

fn panel_modules(
    modules: &[niwoe_ipc::PanelModule],
) -> Vec<niwoe_config::panel_preferences::PanelModule> {
    use niwoe_config::panel_preferences::PanelModule as C;
    use niwoe_ipc::PanelModule as W;
    modules
        .iter()
        .map(|m| match m {
            W::Tray => C::Tray,
            W::Screenshot => C::Screenshot,
            W::Search => C::Search,
            W::Status => C::Status,
        })
        .collect()
}

fn advance(state: &mut FirstRun) -> Result<(), String> {
    state.revision = state
        .revision
        .checked_add(1)
        .ok_or("Einführungsrevision erschöpft")?;
    Ok(())
}

impl NiwoeState {
    pub(super) fn first_run(&mut self, request_id: String, action: FirstRunAction) {
        let directory = niwoe_config::config_directory();
        let path = directory.join("first-run.toml");
        let old_count = self.workspaces.rooms().slot_count();
        let result = (|| -> Result<(), String> {
            if request_id.is_empty() || request_id.len() > 128 {
                return Err("Ungültige Anfrage".into());
            }
            let old = FirstRun::load(&path)?;
            let expected = match &action {
                FirstRunAction::Get => return Ok(()),
                FirstRunAction::SaveDraft {
                    expected_revision, ..
                }
                | FirstRunAction::Discard { expected_revision }
                | FirstRunAction::Complete { expected_revision } => *expected_revision,
            };
            if expected != old.revision {
                return Err("Konflikt: Einführung wurde geändert. Zustand neu laden.".into());
            }
            let mut next = old.clone();
            match action {
                FirstRunAction::Get => unreachable!(),
                FirstRunAction::SaveDraft { draft, .. } => {
                    if old.journal.is_some() {
                        return Err(
                            "Teilabschluss zuerst fortsetzen; Entwurf bleibt erhalten".into()
                        );
                    }
                    // Validate against the existing room command implementation before recording.
                    self.first_run_journal(&draft, &directory)?;
                    next.draft = Some(draft.encode()?);
                }
                FirstRunAction::Discard { .. } => {
                    // Explicit abandonment keeps every productive document and
                    // the previous completion marker. It never claims rollback
                    // or completion of the interrupted run.
                    next.draft = None;
                    next.journal = None;
                }
                FirstRunAction::Complete { .. } => {
                    if next.journal.is_none() {
                        let draft = FirstRunDraft::decode(
                            next.draft.as_deref().ok_or("Kein gespeicherter Entwurf")?,
                        )?;
                        next.journal = Some(self.first_run_journal(&draft, &directory)?);
                        advance(&mut next)?;
                        next.save(&path, &old)?;
                    }
                    self.apply_first_run_journal(
                        next.journal.as_ref().expect("recorded"),
                        &directory,
                    )?;
                    let previous = next.clone();
                    next.completed = true;
                    next.draft = None;
                    next.journal = None;
                    advance(&mut next)?;
                    next.save(&path, &previous)?;
                    return Ok(());
                }
            }
            advance(&mut next)?;
            next.save(&path, &old)
        })();
        // A published-but-not-synced room write still updates runtime exactly once.
        let new_count = self.workspaces.rooms().slot_count();
        for _ in old_count..new_count {
            self.workspaces.append_room_space();
            self.wm_workspaces.push(niwoe_wm::WmWorkspace::new());
        }
        self.broadcast_rooms();
        self.broadcast_workspace();
        self.broadcast_window_snapshot();
        self.panel_preferences("first-run-panel".into(), niwoe_ipc::PanelAction::Get);
        let loaded = FirstRun::load(&path).and_then(|state| {
            let draft = state
                .draft
                .as_deref()
                .map(FirstRunDraft::decode)
                .transpose()?;
            Ok(FirstRunSnapshot {
                revision: state.revision,
                fresh: state.fresh,
                completed: state.completed,
                applying: state.journal.is_some(),
                draft,
            })
        });
        let error = result.err().or_else(|| loaded.as_ref().err().cloned());
        self.ipc.broadcast(&ShellEvent::FirstRun {
            request_id,
            snapshot: loaded.ok(),
            error,
        });
    }

    fn first_run_journal(
        &self,
        draft: &FirstRunDraft,
        directory: &Path,
    ) -> Result<Journal, String> {
        draft.validate()?;
        let before = self.workspaces.rooms().definitions().clone();
        let mut after = before.clone();
        if !draft.rooms.is_empty() {
            if draft.rooms_revision != before.revision {
                return Err("Konflikt: Räume wurden geändert. Entwurf bleibt erhalten.".into());
            }
            for change in &draft.rooms {
                let registry = crate::room_registry::RoomRegistry::from_definitions(after.clone())
                    .map_err(|e| e.to_string())?;
                after = registry
                    .prepare(after.revision, change.clone())
                    .map_err(|e| format!("Raumentwurf ungültig: {e:?}"))?;
            }
            after.revision = before
                .revision
                .checked_add(1)
                .ok_or("Raumrevision erschöpft")?;
            after
                .validate_successor(&before)
                .map_err(|e| e.to_string())?;
        }
        let panel_before = PanelPreferences::load(&directory.join("panel.toml"))?;
        let mut panel_after = panel_before.clone();
        if let Some(modules) = &draft.panel {
            if draft.panel_revision != panel_before.revision {
                return Err("Konflikt: Leiste wurde geändert. Entwurf bleibt erhalten.".into());
            }
            panel_after.modules = panel_modules(modules);
            if panel_after.modules != panel_before.modules {
                panel_after.revision = panel_before
                    .revision
                    .checked_add(1)
                    .ok_or("Leistenrevision erschöpft")?;
            }
        }
        panel_after.validate()?;
        Ok(Journal {
            rooms_before: before,
            rooms_after: after,
            panel_before,
            panel_after,
        })
    }

    fn apply_first_run_journal(
        &mut self,
        journal: &Journal,
        directory: &Path,
    ) -> Result<(), String> {
        let rooms_path = directory.join("rooms.toml");
        let panel_path = directory.join("panel.toml");
        let rooms = niwoe_config::rooms::store::load(&rooms_path).map_err(|e| e.to_string())?;
        let panel = PanelPreferences::load(&panel_path)?;
        if (rooms != journal.rooms_before && rooms != journal.rooms_after)
            || (panel != journal.panel_before && panel != journal.panel_after)
            || self.workspaces.rooms().definitions() != &rooms
        {
            return Err("Konflikt im Teilabschluss: gespeicherte Räume/Leiste weichen ab. Originale und Journal bleiben erhalten.".into());
        }
        if rooms != journal.rooms_after {
            self.workspaces
                .rooms_mut()
                .publish(journal.rooms_after.clone())
                .map_err(|e| {
                    format!("Räume nicht dauerhaft bestätigt: {e:?}. Abschluss fortsetzen.")
                })?;
        }
        sync_document(&rooms_path)?;
        if panel != journal.panel_after {
            journal.panel_after.save(&panel_path, panel.revision)
                .map_err(|error| format!("Leiste nicht gespeichert: {error}. Teilabschluss nach Beheben des Fehlers fortsetzen."))?;
        }
        if panel_path.try_exists().map_err(|e| e.to_string())? {
            sync_document(&panel_path)?;
        }
        if niwoe_config::rooms::store::load(&rooms_path).map_err(|e| e.to_string())?
            != journal.rooms_after
            || PanelPreferences::load(&panel_path)? != journal.panel_after
        {
            return Err("Abschlusszustand nicht bestätigt".into());
        }
        Ok(())
    }
}
