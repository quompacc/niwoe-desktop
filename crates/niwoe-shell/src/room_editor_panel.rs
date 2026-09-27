use niwoe_ipc::PanelModule;

pub(crate) const MODULES: [PanelModule; 4] = [
    PanelModule::Tray,
    PanelModule::Screenshot,
    PanelModule::Search,
    PanelModule::Status,
];
pub(crate) fn label(module: PanelModule) -> &'static str {
    match module {
        PanelModule::Tray => "Tray",
        PanelModule::Screenshot => "Bildschirmfoto",
        PanelModule::Search => "Suche",
        PanelModule::Status => "Systemstatus (Netzwerk, Audio, verfügbarer Akku)",
    }
}

pub(crate) struct PanelUi {
    pub open: bool,
    pub ready: bool,
    pub revision: u64,
    pub draft_revision: u64,
    pub saved: Vec<PanelModule>,
    pub draft: Vec<PanelModule>,
    pub focus: usize,
    pub pending: Option<(String, std::time::Instant)>,
    pub message: String,
}
impl Default for PanelUi {
    fn default() -> Self {
        Self {
            open: false,
            ready: false,
            revision: 0,
            draft_revision: 0,
            saved: MODULES.to_vec(),
            draft: MODULES.to_vec(),
            focus: 0,
            pending: None,
            message: String::new(),
        }
    }
}
impl PanelUi {
    pub fn accept(
        &mut self,
        id: &str,
        revision: u64,
        modules: Vec<PanelModule>,
        error: Option<String>,
    ) {
        let response = self
            .pending
            .as_ref()
            .is_some_and(|(pending, _)| pending == id);
        if revision < self.revision {
            // An unreadable document has no trustworthy revision. Preserve the
            // last known state, but do not hide a correlated storage error.
            if let Some(error) = error.filter(|_| response || self.open) {
                self.message = error;
                self.ready = false;
                if response {
                    self.pending = None;
                }
            }
            return;
        }
        if modules.len() > 4
            || modules
                .iter()
                .enumerate()
                .any(|(i, m)| modules[..i].contains(m))
        {
            return;
        }
        self.revision = revision;
        self.saved = modules;
        self.ready = error.is_none();
        if response {
            self.pending = None;
            // Only a confirmed response permits an explicit retry against the
            // newer revision. Unrelated broadcasts must not rebase the draft.
            self.draft_revision = revision;
        }
        if let Some(error) = error {
            self.message = error;
        } else if response {
            self.message = "Leiste dauerhaft gespeichert".into();
            self.open = false;
        } else if self.open && revision > self.draft_revision {
            self.message =
                "Leiste wurde geändert. Entwurf bleibt erhalten; Abbrechen lädt neu.".into();
        }
        if !self.open {
            self.draft = self.saved.clone();
            self.draft_revision = revision;
        }
    }
    pub fn effective(&self) -> &[PanelModule] {
        if self.open {
            &self.draft
        } else {
            &self.saved
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unreadable_document_reports_error_without_replacing_known_state() {
        let mut ui = PanelUi {
            open: true,
            revision: 5,
            draft_revision: 5,
            pending: Some(("save".into(), std::time::Instant::now())),
            ..Default::default()
        };
        ui.accept("save", 0, vec![], Some("Konfiguration nicht lesbar".into()));
        assert!(ui.pending.is_none());
        assert!(ui.message.contains("nicht lesbar"));
        assert_eq!(ui.saved, MODULES);
        assert_eq!(ui.revision, 5);
    }
    #[test]
    fn panel_ack_is_correlated_conflicts_keep_draft_and_closing_rolls_back_preview() {
        let mut ui = PanelUi {
            open: true,
            draft: vec![PanelModule::Search],
            pending: Some(("mine".into(), std::time::Instant::now())),
            ..Default::default()
        };
        ui.accept("other", 1, vec![PanelModule::Tray], None);
        assert!(ui.open && ui.pending.is_some());
        assert_eq!(
            ui.draft_revision, 0,
            "broadcast must preserve conflict detection"
        );
        assert_eq!(ui.effective(), &[PanelModule::Search]);
        ui.accept("mine", 1, vec![PanelModule::Tray], Some("Konflikt".into()));
        assert!(ui.open && ui.pending.is_none());
        assert_eq!(
            ui.draft_revision, 1,
            "explicit retry follows acknowledged conflict"
        );
        ui.open = false;
        assert_eq!(ui.effective(), &[PanelModule::Tray]);
        ui.pending = Some(("saved".into(), std::time::Instant::now()));
        ui.open = true;
        ui.accept("saved", 2, vec![], None);
        assert!(!ui.open && ui.saved.is_empty());
    }
}
