//! Shared Control Center navigation for room pages and existing settings providers.
//! Draw only on invalidation. Native symbols use the bounded shared artwork cache.
use niwoe_tokens::{ControlCenter, Controls, Interaction, Radius, Spacing, Typography};
use niwoe_ui::{
    effect::{
        paint_border, paint_fill, paint_text, symbol_icon, truncate_to_fit, ui_line_metrics, Symbol,
    },
    ui_length, Position, Rect, TaffyRect, Theme, UiSize, Widget, WidgetState, WidgetStyle,
};
use tiny_skia::{PixmapMut, PixmapPaint, Transform};

const C: ControlCenter = ControlCenter::DEFAULT;
const S: Spacing = Spacing::DEFAULT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Page {
    Rooms,
    Apps,
    Users,
    System,
    Panel,
    Updates,
    Settings,
}

#[derive(Default)]
pub(crate) struct Navigation {
    pub(crate) sidebar_focus: Option<usize>,
    pub(crate) widget_focus: Option<&'static str>,
}

pub(crate) const PAGES: [Page; 7] = [
    Page::Rooms,
    Page::Apps,
    Page::Settings,
    Page::Panel,
    Page::System,
    Page::Users,
    Page::Updates,
];
const DESKTOP_START: usize = 2;

impl Page {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Rooms => "Räume",
            Self::Apps => "Apps",
            Self::Users => "Benutzer",
            Self::System => "System & Geräte",
            Self::Panel => "Leiste",
            Self::Updates => "Updates",
            Self::Settings => "Darstellung",
        }
    }
    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Rooms => "cc-rooms",
            Self::Apps => "cc-apps",
            Self::Users => "cc-users",
            Self::System => "cc-system",
            Self::Panel => "cc-panel",
            Self::Updates => "cc-updates",
            Self::Settings => "cc-settings",
        }
    }
    fn icon(self) -> Symbol {
        match self {
            Self::Rooms => Symbol::Room,
            Self::Apps => Symbol::App,
            Self::Users => Symbol::User,
            Self::System => Symbol::System,
            Self::Panel => Symbol::Panel,
            Self::Updates => Symbol::Download,
            Self::Settings => Symbol::Settings,
        }
    }
    pub(crate) fn for_settings(category: crate::settings_view::SettingsCategory) -> Self {
        use crate::settings_view::SettingsCategory as Category;
        match category {
            Category::DefaultApps | Category::PinnedApps => Self::Apps,
            Category::Users => Self::Users,
            Category::Updates => Self::Updates,
            Category::Wallpaper | Category::Cursor | Category::Display | Category::Theme => {
                Self::Settings
            }
            _ => Self::System,
        }
    }
}

pub(crate) fn row_height(height: u32) -> i32 {
    ((height as i32 - C.config_footer_height - C.outer_pad - S.xxl * 3 - C.sidebar_section_gap)
        / PAGES.len() as i32)
        .clamp(C.config_field_height, C.sidebar_item_height)
}

pub(crate) fn entry_rect(height: u32, index: usize) -> Rect {
    Rect {
        x: C.outer_pad / 2,
        y: C.outer_pad
            + S.xxl * 2
            + index as i32 * row_height(height)
            + if index >= DESKTOP_START {
                C.sidebar_section_gap + S.xxl
            } else {
                0
            },
        width: C.sidebar_width - C.outer_pad,
        height: row_height(height),
    }
}

pub(crate) fn back_rect(height: u32) -> Rect {
    Rect {
        x: C.outer_pad,
        y: height as i32 - C.config_footer_height + C.card_gap,
        width: C.sidebar_width - C.outer_pad * 2,
        height: C.config_field_height,
    }
}

pub(crate) fn hit(height: u32, x: i32, y: i32) -> Option<Page> {
    PAGES.into_iter().enumerate().find_map(|(index, page)| {
        let r = entry_rect(height, index);
        (x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height).then_some(page)
    })
}

fn fill(canvas: &mut PixmapMut<'_>, rect: Rect, color: niwoe_tokens::Color, radius: i32) {
    if let Some(path) = niwoe_ui::rounded_rect_path(rect, radius) {
        paint_fill(canvas, &path, color);
    }
}

fn text(canvas: &mut PixmapMut<'_>, area: Rect, label: &str, color: niwoe_tokens::Color) {
    let size = Typography::DEFAULT.body_size as f32;
    let (ascent, descent) = ui_line_metrics(size);
    paint_text(
        canvas,
        &truncate_to_fit(label, area.width, size),
        area.x,
        (area.y as f32 + (area.height as f32 + ascent + descent) / 2.0).round() as i32,
        size,
        color,
    );
}

struct Entry {
    rect: Rect,
    page: Option<Page>,
    selected: bool,
    enabled: bool,
    back_label: &'static str,
}
impl Widget for Entry {
    fn id(&self) -> Option<&'static str> {
        self.enabled
            .then(|| self.page.map_or("show-tile-view", Page::id))
    }
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            position: Position::Absolute,
            inset: TaffyRect {
                left: ui_length(self.rect.x as f32),
                top: ui_length(self.rect.y as f32),
                ..WidgetStyle::default().inset
            },
            size: UiSize {
                width: ui_length(self.rect.width as f32),
                height: ui_length(self.rect.height as f32),
            },
            ..Default::default()
        }
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let p = theme.palette;
        let hot = self.enabled && state != WidgetState::Idle;
        let color = if !self.enabled {
            p.text_disabled()
        } else {
            p.text
        };
        if self.selected || hot {
            let bg = if state == WidgetState::Pressed {
                Interaction::DEFAULT.pressed(p.surface)
            } else if self.selected {
                Interaction::DEFAULT.selected_tint(p.surface)
            } else {
                Interaction::DEFAULT.hover(p.surface)
            };
            fill(canvas, area, bg, theme.radius.sm);
        }
        if self.page.is_none() {
            if let Some(path) = niwoe_ui::rounded_rect_path(area, theme.radius.sm) {
                paint_border(canvas, &path, p.border_control(), Controls::BORDER as f32);
            }
            text(
                canvas,
                Rect {
                    x: area.x + S.md,
                    width: area.width - S.md * 2,
                    ..area
                },
                self.back_label,
                color,
            );
        } else if let Some(page) = self.page {
            let size = Controls::SYMBOL_SIZE as i32;
            if let Some(icon) = symbol_icon(page.icon(), color, size as u32) {
                canvas.draw_pixmap(
                    area.x + S.lg,
                    area.y + (area.height - size) / 2,
                    icon.as_ref().as_ref(),
                    &PixmapPaint::default(),
                    Transform::identity(),
                    None,
                );
            }
            let x = area.x + S.lg + size + S.md;
            let text_area = Rect {
                x,
                width: area.x + area.width - S.md - x,
                ..area
            };
            if !self.enabled {
                niwoe_ui::effect::paint_text_pair(
                    canvas,
                    text_area,
                    page.label(),
                    "Nicht verfügbar",
                    color,
                    color,
                );
            } else {
                text(canvas, text_area, page.label(), color);
            }
        }
        // Keyboard focus is painted by the common launcher overlay. Hover
        // only changes the fill and must not masquerade as keyboard focus.
    }
}

pub(crate) struct Sidebar {
    height: u32,
    children: Vec<Box<dyn Widget>>,
}
impl Sidebar {
    pub(crate) fn new(height: u32, active: Page, back_label: &'static str) -> Self {
        let mut children: Vec<Box<dyn Widget>> = PAGES
            .into_iter()
            .enumerate()
            .map(|(i, page)| {
                Box::new(Entry {
                    rect: entry_rect(height, i),
                    page: Some(page),
                    selected: page == active,
                    enabled: true,
                    back_label: "",
                }) as Box<dyn Widget>
            })
            .collect();
        children.push(Box::new(Entry {
            rect: back_rect(height),
            page: None,
            selected: false,
            enabled: true,
            back_label,
        }));
        Self { height, children }
    }
}
impl Widget for Sidebar {
    fn style(&self) -> WidgetStyle {
        WidgetStyle {
            flex_shrink: 0.0,
            size: UiSize {
                width: ui_length(C.sidebar_width as f32),
                height: ui_length(self.height as f32),
            },
            ..Default::default()
        }
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, _: WidgetState) {
        let p = theme.palette;
        fill(
            canvas,
            area,
            niwoe_tokens::Color {
                a: C.sidebar_alpha,
                ..p.background
            },
            Radius::DEFAULT.none,
        );
        for (label, y) in [
            ("CONTROL CENTER · F8", C.outer_pad + S.lg),
            ("ARBEITSRÄUME", C.outer_pad + S.xxl + S.lg),
            (
                "DESKTOP & SYSTEM",
                C.outer_pad
                    + S.xxl * 2
                    + row_height(self.height) * DESKTOP_START as i32
                    + C.sidebar_section_gap,
            ),
        ] {
            paint_text(
                canvas,
                label,
                area.x + C.outer_pad,
                area.y + y,
                Typography::DEFAULT.caption_size as f32,
                p.text_dim,
            );
        }
    }
    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
}

pub(crate) fn draw_sidebar(
    canvas: &mut PixmapMut<'_>,
    height: u32,
    active: Page,
    configuring: bool,
    theme: &Theme,
) {
    let sidebar = Sidebar::new(
        height,
        active,
        if configuring {
            "‹ Zurück zu Räumen"
        } else {
            "‹ Zurück zum Hub"
        },
    );
    if let Ok(layout) = niwoe_ui::compute_layout(
        &sidebar,
        niwoe_ui::PixelSize {
            width: C.sidebar_width as u32,
            height,
        },
    ) {
        let _ = niwoe_ui::render(&sidebar, &layout, canvas, theme, &|_| WidgetState::Idle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sidebar_layout_and_hit_geometry_match_on_both_workareas() {
        for height in [1032, 720] {
            let tree = Sidebar::new(height, Page::Apps, "‹ Zurück zum Hub");
            let layout = niwoe_ui::compute_layout(
                &tree,
                niwoe_ui::PixelSize {
                    width: C.sidebar_width as u32,
                    height,
                },
            )
            .unwrap();
            for (index, node) in layout.root.children.iter().take(PAGES.len()).enumerate() {
                assert_eq!(node.rect, entry_rect(height, index));
                let r = node.rect;
                assert_eq!(
                    hit(height, r.x + r.width / 2, r.y + r.height / 2),
                    Some(PAGES[index])
                );
                assert!(r.y + r.height <= back_rect(height).y);
            }
            assert_eq!(tree.children().len(), PAGES.len() + 1);
            assert!(tree.children().iter().all(|child| child.id().is_some()));
            assert_eq!(tree.children()[1].id(), Some("cc-apps"));
        }
    }
    #[test]
    fn active_page_follows_existing_provider_category() {
        use crate::settings_view::SettingsCategory as Category;
        for (category, page) in [
            (Category::DefaultApps, Page::Apps),
            (Category::Users, Page::Users),
            (Category::SystemOverview, Page::System),
            (Category::Wallpaper, Page::Settings),
            (Category::Updates, Page::Updates),
        ] {
            assert_eq!(Page::for_settings(category), page);
        }
    }
}
