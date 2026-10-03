//! Serialized refresh/write requests and verified outcomes for the Settings worker.
use super::{
    command::{Backend, XdgMime},
    DefaultAppCategory, MimeAppIndex, MAX_APPS_PER_CATEGORY,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub(crate) enum ChangeRequest {
    Pick {
        category: DefaultAppCategory,
        desktop_id: String,
    },
    FillEmpty,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ChangeOutcome {
    Saved,
    Filled { changed: usize, unavailable: usize },
    Failed,
}

#[derive(Default)]
pub(crate) struct UiState {
    pub busy: bool,
    pub message: String,
    pub failed_reads: HashSet<DefaultAppCategory>,
}

impl UiState {
    pub(crate) fn start(&mut self, writing: bool) {
        self.busy = true;
        self.message = if writing {
            "Zuordnungen werden gespeichert und geprüft…"
        } else {
            "Installierte Apps und Zuordnungen werden geladen…"
        }
        .into();
    }

    pub(crate) fn finish(
        &mut self,
        failed_reads: HashSet<DefaultAppCategory>,
        outcome: Option<&ChangeOutcome>,
    ) {
        self.busy = false;
        self.message = if !failed_reads.is_empty() {
            "Zuordnungen konnten nicht vollständig geladen werden. Seite erneut öffnen.".into()
        } else {
            match outcome {
                Some(ChangeOutcome::Saved) => "Zuordnung gespeichert und geprüft.".into(),
                Some(ChangeOutcome::Filled {
                    changed,
                    unavailable,
                }) => format!(
                    "{changed} leere Zuordnungen ergänzt · {unavailable} ohne passende App."
                ),
                Some(ChangeOutcome::Failed) => {
                    "Zuordnung nicht vollständig übernommen. Angezeigt wird der gelesene Stand."
                        .into()
                }
                None => String::new(),
            }
        };
        self.failed_reads = failed_reads;
    }
}

pub(crate) struct RefreshData {
    pub index: MimeAppIndex,
    pub current: HashMap<DefaultAppCategory, String>,
    pub failed_reads: HashSet<DefaultAppCategory>,
    pub outcome: Option<ChangeOutcome>,
}

impl RefreshData {
    pub(crate) fn load(icon_config: &(String, String), request: Option<ChangeRequest>) -> Self {
        let mut index = MimeAppIndex::load_system();
        let mut backend = XdgMime::new();
        let outcome = request.map(|request| apply(&index, request, &mut backend));
        let (current, failed_reads) = snapshot(&mut backend);
        index.prepare_previews(icon_config, &current);
        Self {
            index,
            current,
            failed_reads,
            outcome,
        }
    }
}

fn snapshot(
    backend: &mut impl Backend,
) -> (
    HashMap<DefaultAppCategory, String>,
    HashSet<DefaultAppCategory>,
) {
    let mut current = HashMap::new();
    let mut failed = HashSet::new();
    for &category in DefaultAppCategory::ALL {
        match backend.query(category.representative_mime()) {
            Ok(Some(id)) => {
                current.insert(category, id);
            }
            Ok(None) => {}
            Err(()) => {
                failed.insert(category);
            }
        }
    }
    (current, failed)
}

fn verified_write(backend: &mut impl Backend, category: DefaultAppCategory, id: &str) -> bool {
    backend.set(id, category.all_mimes())
        && category.all_mimes().iter().all(|mime| {
            backend
                .query(mime)
                .is_ok_and(|actual| actual.as_deref() == Some(id))
        })
}

fn apply(
    index: &MimeAppIndex,
    request: ChangeRequest,
    backend: &mut impl Backend,
) -> ChangeOutcome {
    match request {
        ChangeRequest::Pick {
            category,
            desktop_id,
        } => {
            // Resolve again after the click: a cached candidate may have been removed.
            let valid = index
                .apps_for_mime(category.representative_mime())
                .into_iter()
                .take(MAX_APPS_PER_CATEGORY)
                .any(|app| app.desktop_id == desktop_id);
            if valid && verified_write(backend, category, &desktop_id) {
                ChangeOutcome::Saved
            } else {
                ChangeOutcome::Failed
            }
        }
        ChangeRequest::FillEmpty => {
            let mut changed = 0;
            let mut unavailable = 0;
            let mut failed = false;
            for &category in DefaultAppCategory::ALL {
                match backend.query(category.representative_mime()) {
                    Ok(Some(_)) => continue,
                    Err(()) => {
                        failed = true;
                        continue;
                    }
                    Ok(None) => {}
                }
                let candidates = index.apps_for_mime(category.representative_mime());
                let chosen = category
                    .preferred_desktop_ids()
                    .iter()
                    .find(|id| candidates.iter().any(|app| app.desktop_id == **id));
                match chosen {
                    Some(id) if verified_write(backend, category, id) => changed += 1,
                    Some(_) => failed = true,
                    None => unavailable += 1,
                }
            }
            if failed {
                ChangeOutcome::Failed
            } else {
                ChangeOutcome::Filled {
                    changed,
                    unavailable,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
