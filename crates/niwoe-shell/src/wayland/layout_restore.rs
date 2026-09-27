use super::NiwoeShell;
use niwoe_ipc::{LayoutAction, LayoutLaunch, LayoutNotice, ShellCommand};
mod input;

impl NiwoeShell {
    pub(crate) fn apply_layout_notice(&mut self, notice: LayoutNotice) {
        match notice {
            LayoutNotice::Status {
                revision,
                room_id,
                running,
                message,
                results,
            } => {
                if let Some(edit) = self
                    .workspace_state
                    .rooms
                    .edit
                    .as_mut()
                    .filter(|e| e.id == room_id)
                {
                    edit.restore.revision = revision;
                    edit.restore.running = running;
                    edit.restore.message = message.clone();
                    edit.restore.results = results.clone();
                }
                self.workspace_state.rooms.layouts.insert(
                    room_id,
                    LayoutNotice::Status {
                        room_id,
                        revision,
                        running,
                        message,
                        results,
                    },
                );
                self.launcher_dirty |= self.launcher_state.open;
            }
            LayoutNotice::Prepare {
                run,
                key,
                app,
                desktop_id,
                file,
            } => {
                let matches: Vec<_> = self
                    .launcher_state
                    .apps
                    .iter()
                    .filter(|candidate| {
                        if let Some(id) = &desktop_id {
                            return &candidate.desktop_id == id;
                        }
                        match &app {
                            niwoe_ipc::AppReference::Native(id) => {
                                candidate.desktop_id.strip_suffix(".desktop") == Some(id.as_str())
                            }
                            niwoe_ipc::AppReference::Xwayland(id) => {
                                candidate.startup_wm_class.as_ref() == Some(id)
                            }
                        }
                    })
                    .collect();
                let prepared = if matches.len() == 1 {
                    let app = matches[0];
                    app.restore_argv(file.as_deref())
                        .map(|(program, args)| LayoutLaunch {
                            desktop_id: app.desktop_id.clone(),
                            program,
                            args,
                            terminal: app.terminal,
                        })
                } else {
                    Err("App fehlt im Katalog oder ist nicht eindeutig zugeordnet".into())
                };
                let (launch, error) = match prepared {
                    Ok(l) => (Some(l), None),
                    Err(e) => (None, Some(e)),
                };
                self.ipc.send(&ShellCommand::Layout {
                    request_id: format!("prepare-{run}-{key}"),
                    action: LayoutAction::Prepared {
                        run,
                        key,
                        launch,
                        error,
                    },
                });
            }
        }
    }
}
