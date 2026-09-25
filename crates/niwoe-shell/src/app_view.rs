//! Native search launcher. One row geometry for paint, pointer and keyboard.
use crate::{
    icons::{icon_image_to_pixmap, IconCache},
    launcher::{DesktopApp, LauncherCategory},
    panel::PinnedApp,
};
use niwoe_tokens::{Controls, Launcher, Radius, Spacing, Typography};
use niwoe_ui::{
    effect::{paint_border, paint_fill, paint_text, rounded_rect_path, truncate_to_fit},
    paint::Rect,
    style::Color,
};
use std::collections::HashSet;
use tiny_skia::{Pixmap, PixmapMut, PixmapPaint, Transform};
const L: Launcher = Launcher::HUB;
const S: Spacing = Spacing::DEFAULT;
const ROW: i32 = L.app_card_height + L.grid_gap;

pub(crate) fn collect_palette_apps<'a>(
    apps: &'a [DesktopApp],
    query: &str,
    hidden: &HashSet<String>,
    _category: LauncherCategory,
    pinned: &[PinnedApp],
) -> Vec<&'a DesktopApp> {
    let query = query.trim().to_lowercase();
    let mut matches: Vec<_> = apps
        .iter()
        .filter(|app| {
            !hidden.contains(&app.program)
                && query.split_whitespace().all(|word| {
                    app.name.to_lowercase().contains(word)
                        || app.program.to_lowercase().contains(word)
                })
        })
        .collect();
    matches.sort_by_cached_key(|app| {
        let name = app.name.to_lowercase();
        let rank = if !query.is_empty() && name == query {
            0
        } else if !query.is_empty() && name.starts_with(&query) {
            1
        } else if pinned.iter().any(|p| p.program == app.program) {
            2
        } else {
            3
        };
        (rank, name, app.program.clone())
    });
    matches
}
fn view_height(height: u32) -> i32 {
    (height as i32 - L.header_height - L.footer_height - S.md).max(0)
}
pub(crate) fn hit_app_row(x: i32, y: i32, scroll: i32, width: u32, height: u32) -> Option<usize> {
    if x < S.lg
        || x >= width as i32 - S.lg
        || y < L.header_height
        || y >= L.header_height + view_height(height)
    {
        return None;
    }
    let local = y - L.header_height + scroll;
    let row_y = L.header_height + (local / ROW) * ROW - scroll;
    if row_y < L.header_height || row_y + L.app_card_height > L.header_height + view_height(height)
    {
        return None;
    }
    (local >= 0 && local % ROW < L.app_card_height).then_some((local / ROW) as usize)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GridDirection {
    Left,
    Right,
    Up,
    Down,
}
pub(crate) fn next_grid_selection(
    current: Option<usize>,
    count: usize,
    direction: GridDirection,
) -> Option<usize> {
    if count == 0 {
        return None;
    }
    let current = current.unwrap_or(0).min(count - 1);
    Some(match direction {
        GridDirection::Up | GridDirection::Left => current.saturating_sub(1),
        GridDirection::Down | GridDirection::Right => (current + 1).min(count - 1),
    })
}
pub(crate) fn scroll_grid_selection_into_view(
    scroll: i32,
    selected: usize,
    count: usize,
    height: u32,
) -> i32 {
    let view = view_height(height);
    let top = selected as i32 * ROW;
    let bottom = top + L.app_card_height;
    let next = if top < scroll {
        top
    } else if bottom > scroll + view {
        bottom - view
    } else {
        scroll
    };
    next.clamp(0, (count as i32 * ROW - L.grid_gap - view).max(0))
}
pub(crate) fn max_scroll_for_palette(
    apps: &[DesktopApp],
    query: &str,
    hidden: &HashSet<String>,
    category: LauncherCategory,
    pinned: &[PinnedApp],
    height: u32,
) -> i32 {
    let count = collect_palette_apps(apps, query, hidden, category, pinned).len();
    (count as i32 * ROW - L.grid_gap - view_height(height)).max(0)
}
fn fill(pm: &mut PixmapMut<'_>, rect: Rect, color: Color, radius: i32) {
    if let Some(path) = rounded_rect_path(rect, radius) {
        paint_fill(pm, &path, color);
    }
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_command_palette(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    pinned: &[PinnedApp],
    apps: &[DesktopApp],
    category: LauncherCategory,
    query: &str,
    scroll: i32,
    selected: Option<usize>,
    _power: Option<(&str, f32)>,
    icons: &IconCache,
    hidden: &HashSet<String>,
    hovered: Option<usize>,
    _bento: Option<usize>,
    _settings: bool,
    _power_hover: Option<usize>,
    config: &niwoe_config::ThemeConfig,
) {
    if canvas.len() != width as usize * height as usize * 4 {
        return;
    }
    let Some(mut image) = Pixmap::new(width, height) else {
        return;
    };
    let theme = crate::ui::tokens::glass_theme_from_config(config);
    let p = theme.palette;
    let alpha = config
        .decorations
        .shell_surface_fill_alpha(niwoe_config::ThemeSurface::Launcher);
    let tint = config.glass_tint_color();
    image.fill(tiny_skia::Color::from_rgba8(tint.r, tint.g, tint.b, alpha));
    let body = Typography::DEFAULT.body_size as f32;
    let caption = Typography::DEFAULT.caption_size as f32;
    let mut pm = image.as_mut();
    let filtered = collect_palette_apps(apps, query, hidden, category, pinned);
    chrome::header(&mut pm, width, query, filtered.len(), config);
    let selected = selected.unwrap_or(0);
    if filtered.is_empty() {
        paint_text(
            &mut pm,
            if query.is_empty() {
                "Keine Anwendungen verfügbar"
            } else {
                "Keine Treffer"
            },
            S.xl,
            L.header_height + S.xxl,
            body,
            p.text_dim,
        );
    }
    // Paint visible rows only; icon lookup uses the existing warmed cache.
    for (index, app) in filtered.iter().enumerate() {
        let y = L.header_height + index as i32 * ROW - scroll;
        if y < L.header_height || y + L.app_card_height > L.header_height + view_height(height) {
            continue;
        }
        let rect = Rect {
            x: S.lg,
            y,
            width: width as i32 - S.lg * 2,
            height: L.app_card_height,
        };
        if index == selected || hovered == Some(index) {
            let color = if index == selected {
                niwoe_tokens::Interaction::DEFAULT.accent_idle(p.accent)
            } else {
                niwoe_tokens::Interaction::DEFAULT.neutral_hover
            };
            fill(&mut pm, rect, color, Radius::DEFAULT.md);
            if index == selected {
                fill(
                    &mut pm,
                    Rect {
                        x: rect.x,
                        y: rect.y + S.sm,
                        width: Controls::FOCUS_WIDTH,
                        height: rect.height - S.sm * 2,
                    },
                    p.accent,
                    0,
                );
                paint_text(
                    &mut pm,
                    "↵",
                    rect.x + rect.width - S.xxl,
                    y + S.xxl,
                    body,
                    p.accent,
                );
            }
        }
        let ix = rect.x + S.md;
        let iy = y + (L.app_card_height - L.app_icon_size) / 2;
        fill(
            &mut pm,
            Rect {
                x: ix - S.xs,
                y: iy - S.xs,
                width: L.app_icon_size + S.sm,
                height: L.app_icon_size + S.sm,
            },
            p.surface_alt,
            Radius::DEFAULT.md,
        );
        if let Some(icon) = app
            .icon_name
            .as_deref()
            .and_then(|name| icons.lookup(name, L.app_icon_size as u32))
            .and_then(icon_image_to_pixmap)
        {
            pm.draw_pixmap(
                ix,
                iy,
                icon.as_ref(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
        } else {
            let fallback = app.name.chars().next().unwrap_or('?').to_string();
            paint_text(&mut pm, &fallback, ix + S.sm, iy + S.xl, body, p.text_dim);
        }
        let tx = ix + L.app_icon_size + S.md;
        let text_width = rect.x + rect.width - S.xxl - S.lg - tx;
        let name = truncate_to_fit(&app.name, text_width, body);
        paint_text(&mut pm, &name, tx, y + S.xl, body, p.text);
        let detail = truncate_to_fit(chrome::detail(app), text_width, caption);
        paint_text(&mut pm, &detail, tx, y + S.xl + S.lg, caption, p.text_dim);
    }
    let footer_y = height as i32 - L.footer_height;
    chrome::footer(&mut pm, width, footer_y, config);
    if let Some(path) = rounded_rect_path(
        Rect {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        },
        Radius::DEFAULT.lg,
    ) {
        paint_border(&mut pm, &path, p.border, Controls::BORDER as f32);
    }
    for (rgba, bgra) in image
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .zip(canvas.as_chunks_mut::<4>().0)
    {
        bgra.copy_from_slice(&[rgba[2], rgba[1], rgba[0], rgba[3]]);
    }
}
#[path = "app_view_chrome.rs"]
mod chrome;

#[cfg(test)]
#[path = "app_view_tests.rs"]
mod tests;
