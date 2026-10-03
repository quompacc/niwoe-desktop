use super::*;
use niwoe_config::layouts::store;

impl NiwoeState {
    pub(crate) fn poll_layout_restore(&mut self) {
        let now = Instant::now();
        if self.layout_restore.next_poll.is_some_and(|t| now < t) {
            return;
        }
        self.layout_restore.next_poll = Some(now + Duration::from_millis(500));
        let exists = |room: &u64| {
            self.workspaces
                .rooms()
                .slot_for_room(RoomId(*room))
                .is_some()
        };
        self.layout_restore.saved.retain(|room, _| exists(room));
        self.layout_restore.statuses.retain(|room, _| exists(room));
        self.layout_restore.revisions.retain(|room, _| exists(room));
        self.layout_restore
            .attempted
            .retain(|(room, _)| exists(room));
        if self
            .layout_restore
            .job
            .as_ref()
            .is_some_and(|job| !exists(&job.snapshot.room_id.0))
        {
            self.layout_restore.job = None;
        }
        if self.lock_manager.is_locked_or_pending() {
            if let Some(job) = self.layout_restore.job.take() {
                self.layout_restore.request_id = job.request_id.clone();
                self.layout_status(
                    job.snapshot.room_id.0,
                    false,
                    "Beim Sperren abgebrochen; gestartete Apps bleiben geöffnet",
                    result_rows(&job.snapshot, &job.results),
                );
            }
            return;
        }
        if let Some(mut job) = self.layout_restore.job.take() {
            self.layout_restore.request_id = job.request_id.clone();
            let windows = self.layout_windows();
            let mut changed = false;
            for key in job
                .bound
                .iter()
                .filter(|(_, w)| !windows.contains(w))
                .map(|(key, _)| *key)
                .collect::<Vec<_>>()
            {
                job.bound.remove(&key);
                job.results.insert(
                    key,
                    "Fenster während der Wiederherstellung geschlossen".into(),
                );
                changed = true;
            }
            for key in job.waiting.keys().copied().collect::<Vec<_>>() {
                let e = job.snapshot.entries.iter().find(|e| e.key == key).unwrap();
                if let Some(w) = match_entry(e, &job.snapshot, &windows) {
                    job.bound.insert(key, w);
                    job.waiting.remove(&key);
                    changed = true;
                } else if now >= job.deadline
                    || now.duration_since(job.waiting[&key]) >= Duration::from_secs(15)
                {
                    job.waiting.remove(&key);
                    job.results.insert(
                        key,
                        "Kein eindeutig zuordenbares Fenster (Deadline / Weiterleitung)".into(),
                    );
                    changed = true;
                }
            }
            if now >= job.deadline {
                for key in job.queue.drain(..) {
                    job.results
                        .insert(key, "Gesamtdeadline erreicht; nicht gestartet".into());
                }
                changed = true;
            }
            while job.waiting.len() < 2 && !job.queue.is_empty() {
                let key = job.queue.remove(0);
                let e = job.snapshot.entries.iter().find(|e| e.key == key).unwrap();
                job.waiting.insert(key, now);
                self.ipc.broadcast(&ShellEvent::Layout {
                    request_id: job.request_id.clone(),
                    notice: LayoutNotice::Prepare {
                        run: job.run,
                        key,
                        app: match &e.app {
                            AppReference::Native(s) => niwoe_ipc::AppReference::Native(s.clone()),
                            AppReference::Xwayland(s) => {
                                niwoe_ipc::AppReference::Xwayland(s.clone())
                            }
                        },
                        desktop_id: e.desktop_id.clone(),
                        file: e.file.clone(),
                    },
                });
                changed = true;
            }
            let done = job.waiting.is_empty() && job.queue.is_empty();
            if changed || done {
                self.apply_restored_layout(&mut job);
                let complete = job
                    .results
                    .values()
                    .filter(|result| result.as_str() == "Fenster angeordnet")
                    .count();
                let message = if done {
                    format!(
                        "Beendet: {complete} angeordnet, {} nicht wiederhergestellt",
                        job.snapshot.entries.len() - complete
                    )
                } else {
                    "Apps werden geöffnet; Abbrechen bleibt möglich".into()
                };
                self.layout_status(
                    job.snapshot.room_id.0,
                    !done,
                    message,
                    result_rows(&job.snapshot, &job.results),
                );
            }
            if !done {
                self.layout_restore.job = Some(job);
            }
            return;
        }
        // Only explicitly saved rooms are armed, never login/restore snapshots.
        self.layout_restore.request_id.clear();
        for room in self
            .layout_restore
            .saved
            .iter()
            .filter_map(|(room, saved)| saved.armed.then_some(*room))
            .collect::<Vec<_>>()
        {
            let Ok(mut current) = self.capture_layout(room) else {
                continue;
            };
            let previous = &self.layout_restore.saved[&room];
            if current.snapshot != previous.snapshot {
                current.changed = Some(now);
                self.layout_restore.saved.insert(room, current);
            } else if previous
                .changed
                .is_some_and(|t| now.duration_since(t) >= Duration::from_secs(2))
            {
                let mut next = previous.snapshot.clone();
                let result = next
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| "Layoutrevision ausgeschöpft".to_string())
                    .and_then(|revision| {
                        next.revision = revision;
                        store::save(&path(room), &next)
                    });
                self.layout_restore.saved.get_mut(&room).unwrap().changed = None;
                if result.is_ok() {
                    self.layout_restore.revisions.insert(room, next.revision);
                    self.layout_restore.saved.get_mut(&room).unwrap().snapshot = next.clone();
                    self.layout_status(
                        room,
                        false,
                        "Layoutänderungen gespeichert",
                        result_rows(&next, &BTreeMap::new()),
                    );
                }
                if let Err(e) = result {
                    self.layout_restore.saved.remove(&room);
                    self.layout_status(
                        room,
                        false,
                        format!("Automatisches Speichern gestoppt: {e}"),
                        vec![],
                    );
                }
            }
        }
    }
}
