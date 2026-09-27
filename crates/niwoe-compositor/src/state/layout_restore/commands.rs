use super::*;
use niwoe_config::layouts::store;
use niwoe_ipc::{LayoutAction, ShellCommand};

impl NiwoeState {
    pub(crate) fn layout_action(&mut self, request_id: String, action: LayoutAction) {
        if request_id.len() > 128 {
            return;
        }
        self.layout_restore.request_id = request_id;
        if self.lock_manager.is_locked_or_pending() {
            return;
        }
        let room = match &action {
            LayoutAction::Save { room_id, .. }
            | LayoutAction::Restore { room_id, .. }
            | LayoutAction::Cancel { room_id }
            | LayoutAction::Status { room_id }
            | LayoutAction::SetFile { room_id, .. } => Some(*room_id),
            LayoutAction::Prepared { .. } => None,
        };
        if room.is_some_and(|room| {
            self.workspaces
                .rooms()
                .slot_for_room(RoomId(room))
                .is_none()
        }) {
            return;
        }
        match action {
            LayoutAction::Status { room_id } => {
                let loaded = store::load(&path(room_id));
                let unchanged = loaded.as_ref().is_ok_and(|s| {
                    self.layout_restore.revisions.get(&room_id) == Some(&s.revision)
                });
                if let Some(notice) = self
                    .layout_restore
                    .statuses
                    .get(&room_id)
                    .filter(|_| unchanged)
                {
                    self.ipc.broadcast(&ShellEvent::Layout {
                        request_id: self.layout_restore.request_id.clone(),
                        notice: notice.clone(),
                    });
                } else {
                    match loaded {
                        Ok(s) => {
                            self.layout_restore.revisions.insert(room_id, s.revision);
                            self.layout_status(
                                room_id,
                                false,
                                "Gespeichertes Layout verfügbar",
                                result_rows(&s, &BTreeMap::new()),
                            );
                        }
                        Err(e) => self.layout_status(
                            room_id,
                            false,
                            format!("Kein lesbares Layout: {e}"),
                            vec![],
                        ),
                    }
                }
            }
            LayoutAction::Save {
                room_id,
                expected_revision,
            } => {
                if self.layout_restore.job.is_some() {
                    self.layout_status(
                        room_id,
                        true,
                        "Zuerst laufende Wiederherstellung abbrechen",
                        vec![],
                    );
                    return;
                }
                let saved = self.capture_layout(room_id).and_then(|mut saved| {
                    saved.snapshot.revision = expected_revision
                        .checked_add(1)
                        .ok_or("Layoutrevision ausgeschöpft")?;
                    store::save(&path(room_id), &saved.snapshot)?;
                    Ok(saved)
                });
                match saved {
                    Ok(saved) => {
                        self.layout_restore
                            .revisions
                            .insert(room_id, saved.snapshot.revision);
                        let rows = result_rows(&saved.snapshot, &BTreeMap::new());
                        self.layout_restore.saved.insert(room_id, saved);
                        self.layout_status(
                            room_id,
                            false,
                            "Layout gespeichert; Änderungen werden nach 2 s Ruhe gespeichert",
                            rows,
                        );
                    }
                    Err(e) => self.layout_status(
                        room_id,
                        false,
                        format!("Speichern fehlgeschlagen: {e}"),
                        vec![],
                    ),
                }
            }
            LayoutAction::SetFile {
                room_id,
                expected_revision,
                key,
                path: file,
            } => {
                let result = (|| {
                    if self.layout_restore.job.is_some() {
                        return Err("Wiederherstellung läuft".into());
                    }
                    let mut saved = store::load(&path(room_id))?;
                    if saved.revision != expected_revision {
                        return Err(
                            "Layoutkonflikt: inzwischen geändert; Ansicht erneut öffnen".into()
                        );
                    }
                    saved.revision = saved
                        .revision
                        .checked_add(1)
                        .ok_or("Layoutrevision ausgeschöpft")?;
                    let entry = saved
                        .entries
                        .iter_mut()
                        .find(|e| e.key == key)
                        .ok_or("Layouteintrag fehlt")?;
                    entry.file = file;
                    store::save(&path(room_id), &saved)?;
                    self.layout_restore
                        .revisions
                        .insert(room_id, saved.revision);
                    let rows = result_rows(&saved, &BTreeMap::new());
                    if let Some(memory) = self.layout_restore.saved.get_mut(&room_id) {
                        memory.snapshot = saved;
                    }
                    Ok::<_, String>(rows)
                })();
                let (message, rows) = match result {
                    Ok(rows) => ("Dateiverweis gespeichert".into(), rows),
                    Err(e) => (format!("Dateiverweis fehlgeschlagen: {e}"), vec![]),
                };
                self.layout_status(room_id, false, message, rows);
            }
            LayoutAction::Cancel { room_id } => {
                if self
                    .layout_restore
                    .job
                    .as_ref()
                    .is_some_and(|j| j.snapshot.room_id.0 == room_id)
                {
                    let mut job = self.layout_restore.job.take().unwrap();
                    for e in &job.snapshot.entries {
                        job.results.entry(e.key).or_insert_with(|| {
                            "Abgebrochen; bereits gestartete Apps bleiben geöffnet".into()
                        });
                    }
                    self.layout_status(
                        room_id,
                        false,
                        "Wiederherstellung abgebrochen",
                        result_rows(&job.snapshot, &job.results),
                    );
                }
            }
            LayoutAction::Restore { room_id, relaunch } => {
                self.begin_layout_restore(room_id, relaunch)
            }
            LayoutAction::Prepared {
                run,
                key,
                launch,
                error,
            } => {
                let Some(mut job) = self.layout_restore.job.take() else {
                    return;
                };
                if job.run != run
                    || !job.waiting.contains_key(&key)
                    || job.results.contains_key(&key)
                {
                    self.layout_restore.job = Some(job);
                    return;
                }
                // Accept each preparation once, even if a client replays it.
                if self
                    .layout_restore
                    .attempted
                    .contains(&(job.snapshot.room_id.0, key))
                {
                    self.layout_restore.job = Some(job);
                    return;
                }
                if let Some(launch) = launch.filter(|l| {
                    !l.program.is_empty()
                        && l.program.len() <= 4096
                        && l.args.len() <= 128
                        && l.args.iter().all(|a| a.len() <= 8192)
                }) {
                    if self.layout_restore.attempted.len()
                        >= niwoe_config::rooms::MAX_ROOMS * niwoe_config::layouts::MAX_WINDOWS
                    {
                        job.waiting.remove(&key);
                        job.results
                            .insert(key, "Sitzungslimit für App-Starts erreicht".into());
                        self.layout_restore.job = Some(job);
                        return;
                    }
                    self.layout_restore
                        .attempted
                        .insert((job.snapshot.room_id.0, key));
                    let entry = job
                        .snapshot
                        .entries
                        .iter_mut()
                        .find(|e| e.key == key)
                        .unwrap();
                    entry.desktop_id = Some(launch.desktop_id);
                    self.handle_shell_command(ShellCommand::LaunchApp {
                        program: launch.program,
                        args: launch.args,
                        terminal: launch.terminal,
                        room_id: Some(job.snapshot.room_id.0),
                    });
                } else {
                    job.waiting.remove(&key);
                    job.results.insert(
                        key,
                        error
                            .unwrap_or_else(|| "App nicht verfügbar".into())
                            .chars()
                            .take(300)
                            .collect(),
                    );
                }
                self.layout_restore.job = Some(job);
            }
        }
    }

    fn begin_layout_restore(&mut self, room: u64, relaunch: bool) {
        if self.layout_restore.job.is_some() {
            self.layout_status(room, true, "Eine Wiederherstellung läuft bereits", vec![]);
            return;
        }
        if self
            .workspaces
            .rooms()
            .slot_for_room(RoomId(room))
            .is_none()
        {
            return;
        }
        let snapshot = match store::load(&path(room)) {
            Ok(s) if s.room_id.0 == room => s,
            Ok(_) => {
                self.layout_status(
                    room,
                    false,
                    "Raum-ID im Layout stimmt nicht überein",
                    vec![],
                );
                return;
            }
            Err(e) => {
                self.layout_status(
                    room,
                    false,
                    format!("Wiederherstellung fehlgeschlagen: {e}"),
                    vec![],
                );
                return;
            }
        };
        // Never autosave a partial restore over the user's saved intent.
        self.layout_restore
            .revisions
            .insert(room, snapshot.revision);
        let previous = self.layout_restore.saved.remove(&room);
        let windows = self.layout_windows();
        let mut bound = BTreeMap::new();
        let mut results = BTreeMap::new();
        let mut queue = Vec::new();
        for entry in &snapshot.entries {
            let matched = previous
                .as_ref()
                .filter(|s| {
                    s.snapshot.entries.iter().any(|old| {
                        old.key == entry.key && old.app == entry.app && old.file == entry.file
                    })
                })
                .and_then(|s| s.bindings.get(&entry.key))
                .filter(|w| windows.contains(w))
                .cloned()
                .or_else(|| match_entry(entry, &snapshot, &windows));
            if let Some(w) = matched.filter(|w| !bound.values().any(|old| old == w)) {
                bound.insert(entry.key, w);
            } else if windows.iter().any(|w| app(w).as_ref() == Some(&entry.app)) {
                results.insert(
                    entry.key,
                    "Mehrdeutig / Single-Instance: kein zusätzlicher Start".into(),
                );
            } else if !relaunch {
                results.insert(entry.key, "Fenster fehlt; nur Layout angefordert".into());
            } else if self.layout_restore.attempted.contains(&(room, entry.key)) {
                results.insert(
                    entry.key,
                    "Bereits versucht; kein erneuter Start in dieser Sitzung".into(),
                );
            } else if snapshot
                .entries
                .iter()
                .filter(|e| e.app == entry.app)
                .count()
                > 1
            {
                results.insert(
                    entry.key,
                    "Mehrere Fenster derselben App: manuell öffnen und erneut anordnen".into(),
                );
            } else if entry
                .file
                .as_ref()
                .is_some_and(|f| !std::path::Path::new(f).is_file())
            {
                results.insert(entry.key, "Datei fehlt; App nicht gestartet".into());
            } else {
                queue.push(entry.key);
            }
        }
        self.layout_restore.serial += 1;
        let mut job = Job {
            request_id: self.layout_restore.request_id.clone(),
            run: self.layout_restore.serial,
            snapshot,
            bound,
            results,
            queue,
            waiting: BTreeMap::new(),
            deadline: Instant::now() + Duration::from_secs(45),
        };
        self.apply_restored_layout(&mut job);
        self.layout_status(
            room,
            true,
            "Wiederherstellung läuft (höchstens 45 s)",
            result_rows(&job.snapshot, &job.results),
        );
        self.layout_restore.job = Some(job);
        self.layout_restore.next_poll = None;
        self.poll_layout_restore();
    }
}
