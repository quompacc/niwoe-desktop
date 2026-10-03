use super::*;
use crate::default_apps::MimeAppCandidate;

#[derive(Default)]
struct Fake {
    values: HashMap<String, String>,
    failures: HashSet<String>,
    writes: Vec<(String, Vec<String>)>,
    reject: bool,
    partial: bool,
}
impl Backend for Fake {
    fn query(&mut self, mime: &str) -> Result<Option<String>, ()> {
        if self.failures.contains(mime) {
            Err(())
        } else {
            Ok(self.values.get(mime).cloned())
        }
    }
    fn set(&mut self, id: &str, mimes: &[&str]) -> bool {
        self.writes
            .push((id.into(), mimes.iter().map(|s| (*s).into()).collect()));
        if self.reject {
            return false;
        }
        for mime in mimes
            .iter()
            .take(if self.partial { 1 } else { mimes.len() })
        {
            self.values.insert((*mime).into(), id.into());
        }
        true
    }
}
fn index() -> MimeAppIndex {
    let mut index = MimeAppIndex::default();
    index.apps.push(MimeAppCandidate {
        desktop_id: "org.kde.kwrite.desktop".into(),
        name: "KWrite".into(),
        icon: None,
        mime_types: vec!["text/plain".into()],
    });
    index.by_mime.insert("text/plain".into(), vec![0]);
    index
}
fn pick() -> ChangeRequest {
    ChangeRequest::Pick {
        category: DefaultAppCategory::TextEditor,
        desktop_id: "org.kde.kwrite.desktop".into(),
    }
}

#[test]
fn saved_requires_the_whole_mime_cluster_to_round_trip() {
    let mut backend = Fake::default();
    assert_eq!(apply(&index(), pick(), &mut backend), ChangeOutcome::Saved);
    assert_eq!(backend.writes.len(), 1);
    for mime in DefaultAppCategory::TextEditor.all_mimes() {
        assert_eq!(
            backend.values.get(*mime).map(String::as_str),
            Some("org.kde.kwrite.desktop")
        );
    }
    let (current, failed) = snapshot(&mut backend);
    assert_eq!(
        current
            .get(&DefaultAppCategory::TextEditor)
            .map(String::as_str),
        Some("org.kde.kwrite.desktop")
    );
    assert!(failed.is_empty());
}

#[test]
fn rejected_or_partial_writes_never_report_success_or_invent_a_snapshot() {
    for partial in [false, true] {
        let mut backend = Fake {
            reject: !partial,
            partial,
            ..Default::default()
        };
        backend
            .values
            .insert("text/plain".into(), "previous.desktop".into());
        assert_eq!(apply(&index(), pick(), &mut backend), ChangeOutcome::Failed);
        let (current, failed) = snapshot(&mut backend);
        assert!(failed.is_empty());
        assert_eq!(
            current
                .get(&DefaultAppCategory::TextEditor)
                .map(String::as_str),
            Some(if partial {
                "org.kde.kwrite.desktop"
            } else {
                "previous.desktop"
            })
        );
    }
}

#[test]
fn missing_cached_candidate_and_failed_query_do_not_write_defaults() {
    let mut backend = Fake::default();
    assert_eq!(
        apply(&MimeAppIndex::default(), pick(), &mut backend),
        ChangeOutcome::Failed
    );
    assert!(backend.writes.is_empty());
    backend.failures.insert("text/plain".into());
    assert_eq!(
        apply(&index(), ChangeRequest::FillEmpty, &mut backend),
        ChangeOutcome::Failed
    );
    assert!(backend.writes.is_empty());
    let (current, failed) = snapshot(&mut backend);
    assert!(!current.contains_key(&DefaultAppCategory::TextEditor));
    assert!(failed.contains(&DefaultAppCategory::TextEditor));
}

#[test]
fn filling_keeps_existing_choices_and_reports_missing_candidates_honestly() {
    let mut backend = Fake::default();
    backend
        .values
        .insert("x-scheme-handler/https".into(), "existing.desktop".into());
    assert_eq!(
        apply(&index(), ChangeRequest::FillEmpty, &mut backend),
        ChangeOutcome::Filled {
            changed: 1,
            unavailable: 7
        }
    );
    assert_eq!(backend.writes.len(), 1);
    assert_eq!(backend.values["x-scheme-handler/https"], "existing.desktop");
}

#[test]
fn read_failures_remain_distinct_from_empty_and_finish_clears_busy() {
    let mut ui = UiState::default();
    ui.start(true);
    assert!(ui.busy);
    ui.finish(
        HashSet::from([DefaultAppCategory::TextEditor]),
        Some(&ChangeOutcome::Saved),
    );
    assert!(!ui.busy);
    assert!(ui.message.contains("nicht vollständig geladen"));
    assert!(ui.failed_reads.contains(&DefaultAppCategory::TextEditor));
    ui.start(false);
    ui.finish(HashSet::new(), None);
    assert!(ui.message.is_empty());
    assert!(ui.failed_reads.is_empty());
}
