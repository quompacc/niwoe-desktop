//! Page-specific capabilities and shared geometry for saved room layouts.
use super::*;
use crate::room_editor::RestoreUi;
mod draw;
pub(crate) use draw::draw;
#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Files,
    Restore,
}
impl Page {
    pub(crate) fn from_tab(tab: usize) -> Self {
        if tab == 2 {
            Self::Files
        } else {
            Self::Restore
        }
    }
    fn buttons(self) -> &'static [usize] {
        match self {
            Self::Files => &[4, 5],
            Self::Restore => &[0, 1, 2, 3, 4, 5],
        }
    }
}
const BUTTONS: [&str; 6] = [
    "Layout speichern",
    "Nur anordnen",
    "Apps wieder öffnen",
    "Wiederherstellung abbrechen",
    "Vorige Einträge",
    "Weitere Einträge",
];
pub(crate) fn hit_tab(x: i32, y: i32) -> bool {
    contains(form::tab_rect(3), x, y)
}
fn control(page: Page, width: u32, index: usize) -> Rect {
    let left = C.sidebar_width + C.outer_pad;
    let available = width as i32 - left - C.outer_pad;
    let (columns, position) = match page {
        Page::Files => (2, index.saturating_sub(4)),
        Page::Restore => (C.room_columns, index),
    };
    Rect {
        x: left + position as i32 % columns * ((available + C.card_gap) / columns),
        y: body_top() + S.xxl + S.lg + position as i32 / columns * (C.config_field_height + S.md),
        width: (available - C.card_gap * (columns - 1)) / columns,
        height: C.config_field_height,
    }
}
fn status_rect(page: Page, width: u32) -> Rect {
    let last = control(page, width, 5);
    Rect {
        x: C.sidebar_width + C.outer_pad,
        y: last.y + last.height + S.md,
        width: width as i32 - C.sidebar_width - C.outer_pad * 2,
        height: S.xxl * 2,
    }
}
fn results_top(page: Page, width: u32) -> i32 {
    let status = status_rect(page, width);
    status.y + status.height + S.md
}
fn file_rect(width: u32, height: u32) -> Rect {
    Rect {
        x: C.sidebar_width + C.outer_pad,
        y: height as i32 - C.config_footer_height - C.outer_pad - C.config_field_height,
        width: width as i32
            - C.sidebar_width
            - C.outer_pad * 2
            - C.config_action_width
            - C.card_gap,
        height: C.config_field_height,
    }
}
fn save_file_rect(width: u32, height: u32) -> Rect {
    let file = file_rect(width, height);
    Rect {
        x: file.x + file.width + C.card_gap,
        width: C.config_action_width,
        ..file
    }
}
fn row_rect(page: Page, width: u32, row: usize) -> Rect {
    Rect {
        x: C.sidebar_width + C.outer_pad,
        y: results_top(page, width) + row as i32 * (C.config_field_height + S.xs),
        width: width as i32 - C.sidebar_width - C.outer_pad * 2,
        height: C.config_field_height,
    }
}
pub(crate) fn page_size(page: Page, width: u32, height: u32) -> usize {
    let bottom = match page {
        Page::Files => file_rect(width, height).y - S.md - S.xs,
        Page::Restore => height as i32 - C.config_footer_height - C.outer_pad,
    };
    (((bottom - results_top(page, width)) / (C.config_field_height + S.xs)).max(1) as usize)
        .min(C.config_layout_page_rows)
}
pub(crate) fn page_index(state: &RestoreUi, page: Page, width: u32, height: u32) -> usize {
    state
        .page
        .min(state.results.len().saturating_sub(1) / page_size(page, width, height))
}
pub(crate) fn enabled(
    state: &RestoreUi,
    page: Page,
    index: usize,
    width: u32,
    height: u32,
) -> bool {
    let count = page_size(page, width, height);
    let current = page_index(state, page, width, height);
    match index {
        0 => page == Page::Restore && state.ready && !state.running,
        1 | 2 => {
            page == Page::Restore
                && state.ready
                && state.revision > 0
                && !state.running
                && !state.message.starts_with("Kein lesbares Layout")
        }
        3 => page == Page::Restore && state.running,
        4 => state.ready && current > 0,
        5 => state.ready && (current + 1) * count < state.results.len(),
        6 | 7 => {
            page == Page::Files
                && state.ready
                && !state.running
                && state
                    .results
                    .iter()
                    .any(|row| Some(row.key) == state.file_key)
        }
        i if i >= 8 => {
            state.ready && i - 8 < count && state.results.get(current * count + i - 8).is_some()
        }
        _ => false,
    }
}
pub(crate) fn focus_order(state: &RestoreUi, page: Page, width: u32, height: u32) -> Vec<usize> {
    let mut order = vec![30, 31, 32, 33];
    order.extend(
        (0..8 + page_size(page, width, height)).filter(|i| enabled(state, page, *i, width, height)),
    );
    order
}
pub(crate) fn hit(
    state: &RestoreUi,
    page: Page,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Option<usize> {
    let button = page
        .buttons()
        .iter()
        .copied()
        .find(|i| contains(control(page, width, *i), x, y));
    let field = if page == Page::Files && contains(file_rect(width, height), x, y) {
        Some(6)
    } else if page == Page::Files && contains(save_file_rect(width, height), x, y) {
        Some(7)
    } else {
        None
    };
    button
        .or(field)
        .or_else(|| {
            (0..page_size(page, width, height))
                .find(|row| contains(row_rect(page, width, *row), x, y))
                .map(|row| 8 + row)
        })
        .filter(|i| enabled(state, page, *i, width, height))
}
fn missing(state: &RestoreUi) -> bool {
    state.message.starts_with("Kein lesbares Layout") && state.message.contains("(os error 2)")
}
pub(crate) fn file_error(state: &RestoreUi) -> Option<&'static str> {
    (!state.file.is_empty() && !std::path::Path::new(&state.file).is_absolute())
        .then_some("Dateiverweis fehlgeschlagen: Bitte einen absoluten Dateipfad eingeben.")
}
fn message(state: &RestoreUi) -> &str {
    if !state.ready {
        "Gespeichertes Layout wird geladen …"
    } else if missing(state) {
        "Für diesen Raum ist noch kein Layout gespeichert."
    } else if state.message.starts_with("Kein lesbares Layout") {
        "Layout konnte nicht geladen werden. Prüfe Zugriffsrechte und gespeicherten Stand."
    } else {
        &state.message
    }
}
fn failed(state: &RestoreUi) -> bool {
    !missing(state)
        && (state.message.contains("fehlgeschlagen")
            || state.message.starts_with("Kein lesbares Layout")
            || state
                .message
                .starts_with("Automatisches Speichern gestoppt"))
}
