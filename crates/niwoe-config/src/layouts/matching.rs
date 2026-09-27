use super::{Entry, Snapshot};
use crate::rooms::AppReference;

pub fn unique_match(
    entry: &Entry,
    snapshot: &Snapshot,
    live: &[(AppReference, String)],
) -> Option<usize> {
    let same_app: Vec<_> = live
        .iter()
        .enumerate()
        .filter(|(_, (app, _))| app == &entry.app)
        .collect();
    let saved_count = snapshot
        .entries
        .iter()
        .filter(|e| e.app == entry.app)
        .count();
    if entry.file.is_none() && saved_count == 1 && same_app.len() == 1 {
        return Some(same_app[0].0);
    }
    if entry.title.is_empty()
        || snapshot
            .entries
            .iter()
            .filter(|e| e.app == entry.app && e.title == entry.title)
            .count()
            != 1
    {
        return None;
    }
    let titles: Vec<_> = same_app
        .into_iter()
        .filter(|(_, (_, title))| title == &entry.title)
        .collect();
    (titles.len() == 1).then(|| titles[0].0)
}
