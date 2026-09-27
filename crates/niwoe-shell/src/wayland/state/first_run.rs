impl NiwoeShell {
    pub(crate) fn open_first_run(&mut self, qh: &QueueHandle<Self>) {
        self.open_room_management(qh);
        self.workspace_state.rooms.panel.open = false;
        self.workspace_state.rooms.wizard.open = true;
        self.workspace_state.rooms.wizard.message = "Einführung wird geladen …".into();
        self.first_run_send(niwoe_ipc::FirstRunAction::Get, "open");
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    pub(crate) fn first_run_send(&mut self, action: niwoe_ipc::FirstRunAction, purpose: &str) {
        if let niwoe_ipc::FirstRunAction::SaveDraft { draft, .. } = &action {
            self.workspace_state.rooms.wizard.unsaved = true;
            if let Err(error) = draft.encode() {
                self.workspace_state.rooms.wizard.message = error;
                self.workspace_state.rooms.wizard.complete_after_save = false;
                return;
            }
        }
        let id = format!(
            "first-run-{purpose}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        let wizard = &mut self.workspace_state.rooms.wizard;
        if self.ipc.send(&ShellCommand::FirstRun {
            request_id: id.clone(),
            action,
        }) {
            wizard.pending = Some((id, Instant::now()));
        } else {
            wizard.pending = None;
            wizard.complete_after_save = false;
            wizard.message =
                "Keine Verbindung. Entwurf bleibt erhalten; neu laden und prüfen.".into();
        }
    }

    pub(crate) fn first_run_result(
        &mut self,
        id: &str,
        snapshot: Option<niwoe_ipc::FirstRunSnapshot>,
        error: Option<String>,
    ) {
        let wizard = &mut self.workspace_state.rooms.wizard;
        if !wizard
            .pending
            .as_ref()
            .is_some_and(|(pending, _)| pending == id)
        {
            return;
        }
        wizard.pending = None;
        let loading = id.starts_with("first-run-open-")
            || id.starts_with("first-run-login-")
            || id.starts_with("first-run-discard-");
        if let Some(state) = snapshot {
            if loading {
                let practiced = wizard.draft.practiced;
                let keep_local = wizard.unsaved
                    && !id.starts_with("first-run-discard-")
                    && state.draft.as_ref() != Some(&wizard.draft);
                if !keep_local {
                    wizard.unsaved = false;
                    wizard.draft =
                        state
                            .draft
                            .clone()
                            .unwrap_or_else(|| niwoe_ipc::FirstRunDraft {
                                rooms_revision: self.workspace_state.rooms.snapshot.revision,
                                panel_revision: self.workspace_state.rooms.panel.revision,
                                ..Default::default()
                            });
                }
                if state.draft.is_some() {
                    for (target, observed) in wizard.draft.practiced.iter_mut().zip(practiced) {
                        *target |= observed;
                    }
                }
                wizard.focus = usize::from(wizard.draft.step == 0);
                wizard.app_page = 0;
                wizard.app_target = 0;
                wizard.confirm_discard = false;
                wizard.existing_suggestions = crate::first_run::SUGGESTIONS.map(|(name, _)| {
                    self.workspace_state
                        .rooms
                        .snapshot
                        .rooms
                        .iter()
                        .any(|r| r.name == name)
                });
                wizard.room_capacity = niwoe_config::rooms::MAX_ROOMS
                    .saturating_sub(self.workspace_state.rooms.snapshot.rooms.len());
                let mut form = crate::room_editor::RoomUi::default();
                form.ready = true;
                form.begin_create();
                if let Some(edit) = &mut form.edit {
                    edit.catalog(&self.launcher_state.apps);
                    wizard.apps = edit.form.apps.clone();
                }
                if id.starts_with("first-run-login-") && state.fresh && !state.completed {
                    wizard.open = true;
                    self.room_management_open = true;
                    self.launcher_settings_open = false;
                    self.hub_search_active = false;
                    self.launcher_layer.set_exclusive_zone(0);
                    self.launcher_configured = false;
                    self.commit_surface(CommitSurfaceKind::Launcher, CommitReason::Input);
                }
            }
            self.workspace_state.rooms.wizard.state = Some(state);
        } else if error.is_some() {
            // A formerly valid snapshot cannot authorize controls after the
            // current document failed validation. Keep the local draft only.
            self.workspace_state.rooms.wizard.state = None;
        }
        if id.starts_with("first-run-login-")
            && error.is_some()
            && self.workspace_state.rooms.wizard.state.is_none()
        {
            self.workspace_state.rooms.wizard.open = true;
            self.room_management_open = true;
            self.launcher_settings_open = false;
            self.hub_search_active = false;
            self.launcher_layer.set_exclusive_zone(0);
            self.launcher_configured = false;
            self.commit_surface(CommitSurfaceKind::Launcher, CommitReason::Input);
        }
        let wizard = &mut self.workspace_state.rooms.wizard;
        if let Some(error) = error {
            wizard.message = error;
            wizard.complete_after_save = false;
        } else if id.starts_with("first-run-complete-") {
            wizard.unsaved = false;
            wizard.message =
                "Einrichtung dauerhaft gespeichert. Es wurden keine Apps gestartet.".into();
            wizard.complete_after_save = false;
            let was_open = wizard.open;
            wizard.open = false;
            wizard.preview_active = false;
            if was_open {
                self.room_management_open = false;
                self.workspace_state.rooms.panel.open = false;
            }
        } else {
            if !loading {
                wizard.unsaved = false;
            }
            wizard.message = if wizard.state.as_ref().is_some_and(|s| s.applying) {
                "Teilabschluss erkannt. Übernehmen setzt den gespeicherten Auftrag fort.".into()
            } else if wizard.unsaved {
                "Ungesicherter lokaler Entwurf bleibt erhalten. Nach Beheben des Speicherfehlers erneut versuchen.".into()
            } else if loading && wizard.state.as_ref().is_some_and(|s| s.draft.is_none()) {
                "Wähle Weiter. Deine bestehenden Einstellungen bleiben erhalten.".into()
            } else {
                "Entwurf getrennt von deinen Einstellungen gespeichert.".into()
            };
            if wizard.complete_after_save {
                wizard.complete_after_save = false;
                let expected_revision = wizard.revision();
                self.first_run_send(
                    niwoe_ipc::FirstRunAction::Complete { expected_revision },
                    "complete",
                );
            }
        }
        let wizard = &mut self.workspace_state.rooms.wizard;
        if !wizard.busy() && !wizard.enabled(wizard.focus) {
            wizard.focus = (0..wizard.control_count())
                .find(|&index| wizard.enabled(index))
                .unwrap_or(wizard.control_count() - 2);
        }
        self.first_run_preview();
        self.launcher_dirty = true;
    }

    fn first_run_preview(&mut self) {
        let rooms = &mut self.workspace_state.rooms;
        let active = rooms.wizard.open && rooms.wizard.draft.step == 3;
        if !active && !rooms.wizard.preview_active {
            return;
        }
        rooms.wizard.preview_active = active;
        rooms.panel.open = active;
        if rooms.panel.open {
            rooms.wizard.set_panel_default(&rooms.panel.saved);
            rooms.panel.draft = rooms
                .wizard
                .draft
                .panel
                .clone()
                .unwrap_or_else(|| rooms.panel.saved.clone());
        }
        self.panel_last_signature = None;
        self.panel_dirty = true;
    }

    pub(crate) fn first_run_action(&mut self, qh: &QueueHandle<Self>, index: usize) {
        use niwoe_ipc::{FirstRunAction, InteractionProfile, RoomChange};
        let rooms = &mut self.workspace_state.rooms;
        let w = &mut rooms.wizard;
        let count = w.control_count();
        let base = count - 5;
        if index >= count {
            return;
        }
        if index == base + 4 {
            w.complete_after_save = false;
            if !w.busy() && w.state.as_ref().is_some_and(|s| s.applying) && !w.confirm_discard {
                w.confirm_discard = true;
                w.message = "Erneut klicken: Restauftrag verwerfen. Bereits gespeicherte Räume/Leistenwerte bleiben; kein Abschluss wird bestätigt.".into();
                self.draw_launcher(qh, RepaintReason::Pointer);
                return;
            }
            if !w.busy() && w.state.is_some() {
                let expected_revision = w.revision();
                self.first_run_send(FirstRunAction::Discard { expected_revision }, "discard");
            } else {
                self.first_run_send(FirstRunAction::Get, "open");
            }
            return;
        }
        if !w.enabled(index) {
            return;
        }
        w.focus = index;
        if index == base + 3 {
            let action = FirstRunAction::SaveDraft {
                expected_revision: w.revision(),
                draft: w.draft.clone(),
            };
            let applying = w.busy() || w.state.as_ref().is_some_and(|s| s.applying);
            w.open = false;
            w.preview_active = false;
            rooms.panel.open = false;
            self.return_to_hub(qh);
            if !applying {
                self.first_run_send(action, "save");
            }
            return;
        }
        if w.state.is_none() {
            w.message = "Zustand nicht geladen. Neu laden; Original bleibt erhalten.".into();
            return;
        }
        if w.state.as_ref().is_some_and(|s| s.applying) {
            if index == base + 1 {
                let expected_revision = w.revision();
                self.first_run_send(FirstRunAction::Complete { expected_revision }, "complete");
            }
            return;
        }
        if index >= base {
            let step = w.draft.step;
            match index - base {
                0 if step > 0 => w.draft.step -= 1,
                1 if step < 4 => w.draft.step += 1,
                1 => w.complete_after_save = true,
                2 if (1..=3).contains(&step) => {
                    match step {
                        1 => {
                            w.draft.profile = InteractionProfile::Preserve;
                            w.draft.panel = None;
                        }
                        2 => w.draft.rooms.clear(),
                        3 => w.draft.panel = None,
                        _ => {}
                    }
                    w.draft.step += 1;
                }
                _ => return,
            }
            w.focus = usize::from(w.draft.step == 0);
            let draft = w.draft.clone();
            let expected_revision = w.revision();
            self.first_run_send(
                FirstRunAction::SaveDraft {
                    expected_revision,
                    draft,
                },
                "save",
            );
        } else {
            match w.draft.step {
                1 => {
                    w.draft.profile = [
                        InteractionProfile::Preserve,
                        InteractionProfile::Mouse,
                        InteractionProfile::Keyboard,
                    ][index];
                    // Profiles only seed an untouched fresh panel. Established preferences always win.
                    if w.state.as_ref().is_some_and(|s| s.fresh && !s.completed)
                        && rooms.panel.revision == 0
                    {
                        w.draft.panel = match index {
                            1 => Some(crate::room_editor::panel::MODULES.to_vec()),
                            2 => Some(
                                crate::room_editor::panel::MODULES
                                    .into_iter()
                                    .filter(|m| *m != niwoe_ipc::PanelModule::Search)
                                    .collect(),
                            ),
                            _ => None,
                        };
                    }
                }
                2 if index < 3 => {
                    let (name, description) = crate::first_run::SUGGESTIONS[index];
                    if let Some(position) = w.draft.rooms.iter().position(
                        |r| matches!(r, RoomChange::Configure { name: n, .. } if n == name),
                    ) {
                        w.draft.rooms.remove(position);
                    } else if rooms.snapshot.rooms.iter().any(|r| r.name == name) {
                        w.message =
                            "Dieser Raum ist bereits vorhanden und bleibt unverändert.".into();
                    } else {
                        w.draft.rooms.push(RoomChange::Configure {
                            id: None,
                            name: name.into(),
                            description: description.into(),
                            assignment: niwoe_ipc::RoomAssignment::Free,
                            preferences: Default::default(),
                            position: rooms.snapshot.rooms.len(),
                        });
                    }
                    // New rooms append in a stable order, even after toggling suggestions repeatedly.
                    for (i, change) in w.draft.rooms.iter_mut().enumerate() {
                        if let RoomChange::Configure { position, .. } = change {
                            *position = rooms.snapshot.rooms.len() + i;
                        }
                    }
                    w.app_target = index;
                }
                2 if index == 3 => w.app_target = (w.app_target + 1) % 3,
                2 if index == 4 => w.app_page = w.app_page.saturating_sub(1),
                2 if index == 5 => {
                    w.app_page = (w.app_page + 1).min(w.apps.len().saturating_sub(1) / 4)
                }
                2 => {
                    if let Some((_, app)) = w.apps.get(w.app_page * 4 + index - 6) {
                        if let Some(RoomChange::Configure { preferences, assignment, .. }) = w.draft.rooms.iter_mut().find(|r|
                            matches!(r, RoomChange::Configure { name, .. } if name == crate::first_run::SUGGESTIONS[w.app_target].0)) {
                            if let Some(i) = preferences.apps.iter().position(|r| r == app) { preferences.apps.remove(i); }
                            else if preferences.apps.len() < 64 { preferences.apps.push(app.clone()); }
                            *assignment = if preferences.apps.is_empty() { niwoe_ipc::RoomAssignment::Free } else { niwoe_ipc::RoomAssignment::Preferred };
                        } else { w.message = "Wähle zuerst den vorgeschlagenen Raum aus.".into(); }
                    }
                }
                3 => {
                    w.set_panel_default(&rooms.panel.saved);
                    let modules = w.draft.panel.as_mut().expect("initialized");
                    crate::room_editor::panel::change_module(modules, index);
                }
                4 => {
                    self.first_run_practice(qh, index);
                    return;
                }
                _ => {}
            }
            // Persist meaningful draft edits at their input boundary, never per
            // frame. A crash no longer loses choices within the current page.
            let w = &self.workspace_state.rooms.wizard;
            if w.draft.step != 2 || !(3..6).contains(&index) {
                let action = FirstRunAction::SaveDraft {
                    expected_revision: w.revision(),
                    draft: w.draft.clone(),
                };
                self.first_run_send(action, "save");
            }
        }
        self.first_run_preview();
        self.draw_panel(qh, RepaintReason::Pointer);
        self.draw_launcher(qh, RepaintReason::Pointer);
    }

    fn first_run_practice(&mut self, qh: &QueueHandle<Self>, index: usize) {
        self.workspace_state.rooms.wizard.open = false;
        self.workspace_state.rooms.wizard.preview_active = false;
        self.workspace_state.rooms.panel.open = false;
        self.return_to_hub(qh);
        let performed = match index {
            0 => self.launcher_state.open,
            1 => {
                if let Some(workspace) = self
                    .workspace_state
                    .rooms
                    .snapshot
                    .rooms
                    .first()
                    .map(|r| r.workspace)
                {
                    self.workspace_state.rooms.wizard.practice_room = Some(workspace);
                    self.handle_panel_click(qh, ClickAction::SwitchWorkspace(workspace));
                    self.close_launcher_after_launch(qh, RepaintReason::Pointer);
                    // Success is confirmed separately by the workspace snapshot, never by a sent command.
                }
                false
            }
            2 => {
                self.toggle_network_popup(CommitReason::Input);
                self.draw_network_popup(qh, RepaintReason::Pointer);
                self.network_popup_open
            }
            _ => false,
        };
        let w = &mut self.workspace_state.rooms.wizard;
        if index < 3 {
            w.draft.practiced[index] = performed;
        }
        let action = niwoe_ipc::FirstRunAction::SaveDraft {
            expected_revision: w.revision(),
            draft: w.draft.clone(),
        };
        self.first_run_send(action, "practice");
    }

    pub(crate) fn first_run_key(
        &mut self,
        qh: &QueueHandle<Self>,
        key: smithay_client_toolkit::seat::keyboard::Keysym,
    ) {
        use smithay_client_toolkit::seat::keyboard::Keysym;
        let w = &mut self.workspace_state.rooms.wizard;
        let count = w.control_count();
        match key {
            Keysym::Home | Keysym::End => {
                let backwards = key == Keysym::End;
                w.focus = if backwards { count - 1 } else { 0 };
                for _ in 0..count {
                    if w.enabled(w.focus) {
                        break;
                    }
                    w.focus = (w.focus + if backwards { count - 1 } else { 1 }) % count;
                }
            }
            Keysym::Tab | Keysym::ISO_Left_Tab | Keysym::Up | Keysym::Down => {
                let backwards = matches!(key, Keysym::ISO_Left_Tab | Keysym::Up);
                for _ in 0..count {
                    w.focus = (w.focus + if backwards { count - 1 } else { 1 }) % count;
                    if w.enabled(w.focus) {
                        break;
                    }
                }
            }
            Keysym::Escape => {
                self.first_run_action(qh, count - 2);
                return;
            }
            Keysym::Return | Keysym::KP_Enter | Keysym::space => {
                let index = w.focus;
                self.first_run_action(qh, index);
                return;
            }
            _ => {}
        }
        self.draw_launcher(qh, RepaintReason::Keyboard);
    }

    pub(crate) fn pause_first_run(&mut self) {
        let w = &mut self.workspace_state.rooms.wizard;
        if !w.open {
            return;
        }
        w.open = false;
        w.preview_active = false;
        self.workspace_state.rooms.panel.open = false;
        self.panel_dirty = true;
        self.panel_last_signature = None;
        if !w.busy() && w.state.is_some() && !w.state.as_ref().is_some_and(|s| s.applying) {
            let action = niwoe_ipc::FirstRunAction::SaveDraft {
                expected_revision: w.revision(),
                draft: w.draft.clone(),
            };
            self.first_run_send(action, "pause");
        }
    }

    fn observe_first_run(&mut self, event: &ShellEvent) {
        let workspace = match event {
            ShellEvent::WorkspaceChanged { workspace } => Some(*workspace),
            ShellEvent::WindowSnapshot {
                active_workspace, ..
            } => Some(*active_workspace),
            _ => None,
        };
        let w = &mut self.workspace_state.rooms.wizard;
        if w.practice_room.is_some() && w.practice_room == workspace {
            w.draft.practiced[1] = true;
            w.practice_room = None;
        }
    }
}
