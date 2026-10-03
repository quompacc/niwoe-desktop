//! Bounded introduction draft; productive writes belong to the compositor.
use niwoe_ipc::{FirstRunDraft, FirstRunSnapshot, PanelModule, RoomChange};

pub(crate) const SUGGESTIONS: [(&str, &str); 3] = [
    ("Arbeit", "Ein Kontext für deine Arbeit"),
    ("Privat", "Ein Kontext für persönliche Anwendungen"),
    ("Entwicklung", "Ein Kontext für Entwicklungswerkzeuge"),
];

/// Match P10's Super+1: resolve the first display position to its stable slot.
pub(crate) fn practice_workspace(rooms: &[niwoe_ipc::RoomEntry]) -> Option<u8> {
    rooms.first().map(|room| room.workspace)
}

#[derive(Default)]
pub(crate) struct Wizard {
    pub open: bool,
    pub state: Option<FirstRunSnapshot>,
    pub draft: FirstRunDraft,
    pub focus: usize,
    pub app_page: usize,
    pub app_target: usize,
    pub apps: Vec<(String, niwoe_ipc::AppReference)>,
    pub message: String,
    pub pending: Option<(String, std::time::Instant)>,
    pub complete_after_save: bool,
    pub practice_room: Option<u8>,
    pub existing_suggestions: [bool; 3],
    pub room_capacity: usize,
    pub confirm_discard: bool,
    pub preview_active: bool,
    pub unsaved: bool,
}

impl Wizard {
    pub fn report_error(&mut self, error: String) {
        self.complete_after_save = false;
        self.message = if self.state.is_none() {
            "Die Einführung konnte nicht geladen werden. Zustand neu laden; vorhandene Einstellungen bleiben erhalten.".into()
        } else {
            error
        };
    }

    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }
    pub fn revision(&self) -> u64 {
        self.state.as_ref().map_or(0, |s| s.revision)
    }
    pub fn room_selected(&self, name: &str) -> bool {
        self.draft
            .rooms
            .iter()
            .any(|r| matches!(r, RoomChange::Configure { name: n, .. } if n == name))
    }
    #[cfg(test)]
    pub fn controls(&self) -> Vec<String> {
        let mut controls = match self.draft.step {
            0 => vec![],
            1 => [
                "Eigene Einstellungen erhalten",
                "Maus: Suchmodul als Standard sichtbar",
                "Tastatur: Suchmodul als Standard ausgeblendet",
            ]
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let selected = match self.draft.profile {
                    niwoe_ipc::InteractionProfile::Preserve => 0,
                    niwoe_ipc::InteractionProfile::Mouse => 1,
                    niwoe_ipc::InteractionProfile::Keyboard => 2,
                };
                format!("{} {s}", if selected == i { "[✓]" } else { "[ ]" })
            })
            .collect(),
            2 => {
                let mut rows: Vec<_> = SUGGESTIONS
                    .iter()
                    .enumerate()
                    .map(|(i, (name, _))| {
                        format!(
                            "{} Raum {name} {}",
                            if self.room_selected(name) {
                                "[✓]"
                            } else {
                                "[ ]"
                            },
                            if self.existing_suggestions[i] {
                                "bereits vorhanden"
                            } else {
                                "ergänzen"
                            }
                        )
                    })
                    .collect();
                rows.push(format!(
                    "Apps zuordnen für: {}",
                    SUGGESTIONS[self.app_target].0
                ));
                rows.push("Vorherige Apps".into());
                rows.push("Weitere Apps".into());
                for (label, reference) in self.apps.iter().skip(self.app_page * 4).take(4) {
                    let selected = self.draft.rooms.iter().any(|r| matches!(r,
                        RoomChange::Configure { name, preferences, .. }
                        if name == SUGGESTIONS[self.app_target].0 && preferences.apps.contains(reference)));
                    rows.push(format!("{} {label}", if selected { "[✓]" } else { "[ ]" }));
                }
                rows
            }
            3 => {
                let modules = self.draft.panel.as_deref().unwrap_or(&[]);
                let mut rows = Vec::new();
                for module in crate::room_editor::panel::MODULES {
                    rows.push(format!(
                        "{} {}",
                        if modules.contains(&module) {
                            "[✓]"
                        } else {
                            "[ ]"
                        },
                        crate::room_editor::panel::label(module)
                    ));
                    rows.push("← Früher".into());
                    rows.push("Später →".into());
                }
                rows
            }
            _ => [
                "Hub ausprobieren · Super+Space · Panel-Hub",
                "Ersten Raum wählen · Super+1 · Raumleiste",
                "Systemdeck öffnen · Super+Escape · Systemstatus",
            ]
            .iter()
            .enumerate()
            .map(|(i, label)| {
                format!(
                    "{} {label}",
                    if self.draft.practiced[i] {
                        "[✓]"
                    } else {
                        "[ ]"
                    }
                )
            })
            .collect(),
        };
        controls.extend([
            "Zurück".into(),
            if self.draft.step == 4 {
                "Einrichtung übernehmen".into()
            } else {
                "Weiter".into()
            },
            "Schritt überspringen".into(),
            "Abbrechen · Entwurf behalten".into(),
            if self.state.as_ref().is_some_and(|s| s.applying) {
                if self.confirm_discard {
                    "Teilabschluss jetzt verwerfen"
                } else {
                    "Teilabschluss verwerfen …"
                }
            } else {
                "Entwurf verwerfen / neu laden"
            }
            .into(),
        ]);
        controls
    }

    pub fn set_panel_default(&mut self, modules: &[PanelModule]) {
        if self.draft.panel.is_none() {
            self.draft.panel = Some(modules.to_vec());
        }
    }

    pub fn enabled(&self, index: usize) -> bool {
        let base = self.control_count() - 5;
        if index == base + 4 || index == base + 3 {
            return true;
        }
        if self.busy() || self.state.is_none() {
            return false;
        }
        if self.state.as_ref().is_some_and(|s| s.applying) {
            return index == base + 1;
        }
        if index >= base {
            return match index - base {
                0 => self.draft.step > 0,
                2 => (1..=3).contains(&self.draft.step),
                _ => true,
            };
        }
        if self.draft.step == 2 {
            if index < 3 {
                return !self.existing_suggestions[index]
                    && (self.room_selected(SUGGESTIONS[index].0)
                        || self.draft.rooms.len() < self.room_capacity);
            }
            if index == 4 {
                return self.app_page > 0;
            }
            if index == 5 {
                return (self.app_page + 1) * 4 < self.apps.len();
            }
            if index >= 6 {
                return self.room_selected(SUGGESTIONS[self.app_target].0);
            }
        }
        if self.draft.step == 3 && !index.is_multiple_of(3) {
            let modules = self.draft.panel.as_deref().unwrap_or(&[]);
            let module = crate::room_editor::panel::MODULES[index / 3];
            return modules.iter().position(|m| *m == module).is_some_and(|p| {
                if index % 3 == 1 {
                    p > 0
                } else {
                    p + 1 < modules.len()
                }
            });
        }
        true
    }

    pub fn control_count(&self) -> usize {
        5 + match self.draft.step {
            0 => 0,
            1 | 4 => 3,
            2 => 6 + self.apps.len().saturating_sub(self.app_page * 4).min(4),
            _ => 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unreadable_state_preserves_local_draft_and_only_offers_exit_or_reload() {
        let mut wizard = Wizard::default();
        wizard.draft.profile = niwoe_ipc::InteractionProfile::Keyboard;
        wizard.draft.step = 3;
        wizard.complete_after_save = true;
        let original = wizard.draft.clone();
        wizard.report_error("expected value at line 1 column 1".into());
        assert_eq!(wizard.draft, original);
        assert!(!wizard.complete_after_save);
        assert!(wizard
            .message
            .starts_with("Die Einführung konnte nicht geladen werden."));
        assert!(!wizard.message.contains("expected value"));
        let base = wizard.control_count() - 5;
        for index in 0..wizard.control_count() {
            assert_eq!(
                wizard.enabled(index),
                index == base + 3 || index == base + 4
            );
        }
        wizard.state = Some(FirstRunSnapshot {
            revision: 5,
            fresh: false,
            completed: false,
            applying: true,
            draft: Some(original),
        });
        let conflict = "Konflikt: Einführung wurde geändert. Zustand neu laden.";
        wizard.report_error(conflict.into());
        assert_eq!(wizard.message, conflict);
        assert!(wizard.enabled(base + 1));
    }

    #[test]
    fn practice_shortcut_target_survives_room_reordering() {
        let rooms = [2, 1].map(|workspace| niwoe_ipc::RoomEntry {
            id: u64::from(workspace) + 30,
            workspace,
            name: "Existing room".into(),
            description: String::new(),
            assignment: Default::default(),
            preferences: Default::default(),
        });
        assert_eq!(practice_workspace(&rooms), Some(2));
        assert_eq!(practice_workspace(&rooms[1..]), Some(1));
        assert_eq!(practice_workspace(&[]), None);
    }

    #[test]
    fn incomplete_or_uncertain_state_only_exposes_safe_exit_and_reload() {
        let mut w = Wizard::default();
        for step in 0..=4 {
            w.draft.step = step;
            let base = w.control_count() - 5;
            for i in 0..w.control_count() {
                assert_eq!(w.enabled(i), i == base + 3 || i == base + 4);
            }
        }
        w.state = Some(FirstRunSnapshot {
            revision: 5,
            fresh: false,
            completed: true,
            applying: true,
            draft: Some(w.draft.clone()),
        });
        let base = w.control_count() - 5;
        for i in 0..w.control_count() {
            assert_eq!(w.enabled(i), [base + 1, base + 3, base + 4].contains(&i));
        }
        w.pending = Some(("completion".into(), std::time::Instant::now()));
        assert!(!w.enabled(base + 1));
        assert!(w.enabled(base + 3), "pending I/O must never trap the user");
    }

    #[test]
    fn existing_room_and_capacity_are_not_presented_as_active_creation() {
        let mut w = Wizard {
            existing_suggestions: [true, false, false],
            room_capacity: 0,
            state: Some(FirstRunSnapshot {
                revision: 0,
                fresh: false,
                completed: false,
                applying: false,
                draft: None,
            }),
            ..Default::default()
        };
        w.draft.step = 2;
        assert!((0..3).all(|i| !w.enabled(i)));
        w.room_capacity = 1;
        assert!(!w.enabled(0));
        assert!(w.enabled(1));
        assert!(!w.enabled(4));
        assert!(!w.enabled(5));
        for step in 0..=4 {
            w.draft.step = step;
            assert_eq!(w.controls().len(), w.control_count());
        }
    }
}
