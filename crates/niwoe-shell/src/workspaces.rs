use std::cell::RefCell;

use niwoe_config::ThemeConfig;
use niwoe_tokens::{Interaction, Radius, WorkspaceSwitcher};

use crate::{
    popup_card::{draw_card_body, draw_card_title, BODY_TOP, PAD_BOTTOM, PAD_X},
    ClickAction, ClickZone, Painter, Rect, TextRenderer, WORKSPACE_POPUP_HEIGHT,
    WORKSPACE_POPUP_WIDTH,
};

const LAYOUT: WorkspaceSwitcher = WorkspaceSwitcher::DEFAULT;
const CELL_COUNT: usize = (LAYOUT.columns * LAYOUT.rows) as usize;
#[path = "workspaces/details.rs"]
mod details;

pub(crate) fn page_size() -> usize {
    CELL_COUNT
}

pub struct WorkspacePopupState {
    pub selected: usize,
    pub clicks: Vec<ClickZone>,
    pub(crate) rooms: crate::room_editor::RoomUi,
}

impl WorkspacePopupState {
    pub fn new() -> Self {
        Self {
            selected: 0,
            clicks: Vec::new(),
            rooms: Default::default(),
        }
    }

    pub fn select_active(&mut self, workspace: u8) {
        self.selected = self
            .rooms
            .snapshot
            .rooms
            .iter()
            .position(|r| r.workspace == workspace)
            .unwrap_or(0);
    }

    pub fn select_relative(&mut self, delta: isize) {
        self.selected = self
            .selected
            .saturating_add_signed(delta)
            .min(self.rooms.snapshot.rooms.len().saturating_sub(1));
    }

    pub fn visible_range(&self) -> std::ops::Range<usize> {
        let count = self.rooms.snapshot.rooms.len();
        let selected = self.selected.min(count.saturating_sub(1));
        let start = selected / CELL_COUNT * CELL_COUNT;
        start..(start + CELL_COUNT).min(count)
    }
}

pub struct WorkspacePopupInput {
    pub active_workspace: u32,
    pub occupied: [bool; niwoe_config::rooms::MAX_ROOMS],
    pub hovered_idx: Option<usize>,
}

fn grid_geometry() -> (i32, i32, i32, i32) {
    let width = WORKSPACE_POPUP_WIDTH as i32;
    let height = WORKSPACE_POPUP_HEIGHT as i32;
    let grid_left = PAD_X;
    let grid_top = BODY_TOP;
    let grid_w = width - 2 * PAD_X;
    let grid_bottom = height - PAD_BOTTOM - LAYOUT.footer_height;
    let grid_h = grid_bottom - grid_top;
    let tile_w = (grid_w - (LAYOUT.columns - 1) * LAYOUT.tile_gap) / LAYOUT.columns;
    let tile_h = (grid_h - (LAYOUT.rows - 1) * LAYOUT.tile_gap) / LAYOUT.rows;
    (grid_left, grid_top, tile_w, tile_h)
}

pub fn workspace_popup_hover_idx(x: f64, y: f64) -> Option<usize> {
    let (left, top, tile_w, tile_h) = grid_geometry();
    for i in 0..CELL_COUNT {
        let col = i as i32 % LAYOUT.columns;
        let row = i as i32 / LAYOUT.columns;
        let rect = Rect {
            x: left + col * (tile_w + LAYOUT.tile_gap),
            y: top + row * (tile_h + LAYOUT.tile_gap),
            w: tile_w,
            h: tile_h,
        };
        if rect.contains(x, y) {
            return Some(i);
        }
    }
    None
}

pub fn draw_workspace_popup(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    input: WorkspacePopupInput,
    state: &mut WorkspacePopupState,
) {
    state.clicks.clear();
    let colors = &theme.colors;
    state.select_relative(0);

    draw_card_body(painter, theme);
    draw_card_title(
        painter,
        font,
        theme,
        if state.rooms.edit.is_some() {
            "Raum bearbeiten"
        } else {
            "Räume · Pfeile wählen, Enter öffnet"
        },
    );
    if state.rooms.edit.is_some() {
        crate::room_editor::draw(painter, font, theme, &state.rooms, &mut state.clicks);
        return;
    }

    let (left, top, tile_w, tile_h) = grid_geometry();

    for (i, position) in state.visible_range().enumerate() {
        let room = &state.rooms.snapshot.rooms[position];
        let ws_id = room.workspace as u32;
        let col = i as i32 % LAYOUT.columns;
        let row = i as i32 / LAYOUT.columns;
        let rect = Rect {
            x: left + col * (tile_w + LAYOUT.tile_gap),
            y: top + row * (tile_h + LAYOUT.tile_gap),
            w: tile_w,
            h: tile_h,
        };

        let is_active = ws_id == input.active_workspace;
        let is_occupied = input.occupied[room.workspace.saturating_sub(1) as usize];
        let is_hovered = input.hovered_idx == Some(i) || state.selected == position;

        let resting_bg = if is_active {
            Interaction::DEFAULT.selected_tint(colors.surface_alt)
        } else {
            colors.surface_alt
        };
        let bg = if is_hovered {
            Interaction::DEFAULT.hover(resting_bg)
        } else {
            resting_bg
        };
        painter.roundish_rect_with_radius(rect, bg, Radius::DEFAULT.sm);
        if state.selected == position {
            painter.focus(
                rect,
                crate::ui::tokens::theme_from_config(theme)
                    .palette
                    .border_focus(),
                Radius::DEFAULT.sm,
            );
        }

        let text_color = if is_active || is_occupied || is_hovered {
            colors.text
        } else {
            colors.text_dim
        };
        let state_label = if is_active {
            "AKTIV"
        } else if is_occupied {
            "BELEGT"
        } else {
            "FREI"
        };
        painter.text_pair(
            Rect {
                x: rect.x + LAYOUT.tile_pad,
                y: rect.y,
                w: rect.w - 2 * LAYOUT.tile_pad,
                h: rect.h,
            },
            &room.name,
            state_label,
            text_color,
            if is_active {
                colors.accent
            } else {
                colors.text_dim
            },
        );
        if is_occupied && !is_active {
            let dot_size = LAYOUT.occupied_dot_size;
            painter.roundish_rect_with_radius(
                Rect {
                    x: rect.x + rect.w - LAYOUT.tile_pad - dot_size,
                    y: rect.y + rect.h - LAYOUT.tile_pad - dot_size,
                    w: dot_size,
                    h: dot_size,
                },
                colors.text_dim,
                dot_size,
            );
        }
        state.clicks.push(ClickZone {
            id: Some(format!("workspace-popup-{ws_id}")),
            rect,
            action: ClickAction::SwitchWorkspace(ws_id as u8),
        });
    }
    details::draw(painter, font, theme, state, input.hovered_idx);
}

#[cfg(test)]
mod tests {
    use crate::ClickAction;

    use super::{
        draw_workspace_popup, workspace_popup_hover_idx, WorkspacePopupInput, WorkspacePopupState,
    };

    #[test]
    fn all_rooms_remain_reachable_after_reorder_and_shrink() {
        for count in [1, 9, 64] {
            let mut state = WorkspacePopupState::new();
            let template = state.rooms.snapshot.rooms[0].clone();
            state.rooms.snapshot.rooms = (1..=count)
                .rev()
                .map(|slot| {
                    let mut room = template.clone();
                    room.id = slot as u64 + 100;
                    room.workspace = slot;
                    room
                })
                .collect();
            for slot in 1..=count {
                state.select_active(slot);
                assert!(state.visible_range().contains(&state.selected));
                assert_eq!(state.rooms.snapshot.rooms[state.selected].workspace, slot);
            }
            let mut reachable = std::collections::BTreeSet::new();
            state.selected = 0;
            loop {
                let range = state.visible_range();
                for position in range.clone() {
                    reachable.insert(state.rooms.snapshot.rooms[position].workspace);
                }
                if range.end == count as usize {
                    break;
                }
                state.select_relative(super::page_size() as isize);
            }
            assert_eq!(reachable.len(), count as usize);
            state.rooms.snapshot.rooms.truncate(1);
            state.select_relative(0);
            assert_eq!(state.visible_range(), 0..1);
        }
    }

    #[test]
    fn last_page_renders_room_64_without_slot_clamping() {
        let mut state = WorkspacePopupState::new();
        let template = state.rooms.snapshot.rooms[0].clone();
        state.rooms.snapshot.rooms = (1..=64)
            .map(|slot| {
                let mut room = template.clone();
                room.id = slot as u64;
                room.workspace = slot;
                room.name = if slot == 64 {
                    "Sehr langer Raumname für Entwicklung Recherche und Dokumentation".into()
                } else {
                    format!("Raum {slot}")
                };
                room
            })
            .collect();
        state.select_active(64);
        let width = crate::WORKSPACE_POPUP_WIDTH;
        let height = crate::WORKSPACE_POPUP_HEIGHT;
        let mut buffer = vec![0; (width * height * 4) as usize];
        let font = std::cell::RefCell::new(crate::TextRenderer::new(
            "sans-serif",
            niwoe_tokens::Typography::DEFAULT.body_size as u32,
        ));
        let theme = niwoe_config::ThemeConfig::default();
        draw_workspace_popup(
            &mut crate::Painter::new(&mut buffer, width as i32, height as i32),
            &font,
            &theme,
            WorkspacePopupInput {
                active_workspace: 64,
                occupied: [true; 64],
                hovered_idx: None,
            },
            &mut state,
        );
        assert!(matches!(
            state.clicks[0].action,
            ClickAction::SwitchWorkspace(64)
        ));
        assert!(state
            .clicks
            .iter()
            .any(|z| matches!(z.action, ClickAction::WorkspacePage(-1))));
        assert!(!state
            .clicks
            .iter()
            .any(|z| matches!(z.action, ClickAction::WorkspacePage(1))));
        for a in &state.clicks {
            assert!(
                a.rect.x >= 0
                    && a.rect.y >= 0
                    && a.rect.x + a.rect.w <= width as i32
                    && a.rect.y + a.rect.h <= height as i32
            );
        }
        if let Ok(path) = std::env::var("NIWOE_WORKSPACE_PREVIEW_PNG") {
            for pixel in buffer.as_chunks_mut::<4>().0 {
                let inverse = 255 - pixel[3] as u16;
                let background = theme.colors.surface;
                for (value, channel) in
                    pixel[..3]
                        .iter_mut()
                        .zip([background.b, background.g, background.r])
                {
                    *value = (*value as u16 + channel as u16 * inverse / 255).min(255) as u8;
                }
                pixel.swap(0, 2);
                pixel[3] = 255;
            }
            image::save_buffer(path, &buffer, width, height, image::ColorType::Rgba8).unwrap();
        }
    }

    #[test]
    fn workspace_popup_generates_nine_switch_click_zones() {
        let mut surface =
            vec![0_u8; (crate::WORKSPACE_POPUP_WIDTH * crate::WORKSPACE_POPUP_HEIGHT * 4) as usize];
        let mut painter = crate::Painter::new(
            &mut surface,
            crate::WORKSPACE_POPUP_WIDTH as i32,
            crate::WORKSPACE_POPUP_HEIGHT as i32,
        );
        let mut state = WorkspacePopupState::new();
        let theme = niwoe_config::ThemeConfig::default();
        let font = std::cell::RefCell::new(None);

        draw_workspace_popup(
            &mut painter,
            &font,
            &theme,
            WorkspacePopupInput {
                active_workspace: 3,
                occupied: [false; niwoe_config::rooms::MAX_ROOMS],
                hovered_idx: None,
            },
            &mut state,
        );

        assert_eq!(state.clicks.len(), 9);
        assert!(matches!(
            state.clicks[0].action,
            ClickAction::SwitchWorkspace(1)
        ));
        assert!(matches!(
            state.clicks[8].action,
            ClickAction::SwitchWorkspace(9)
        ));
    }

    #[test]
    fn workspace_popup_hover_idx_finds_first_tile_inside_grid() {
        let probe_x = crate::popup_card::PAD_X as f64 + 4.0;
        let probe_y = crate::popup_card::BODY_TOP as f64 + 4.0;
        assert_eq!(workspace_popup_hover_idx(probe_x, probe_y), Some(0));
        assert_eq!(workspace_popup_hover_idx(0.0, 0.0), None);
    }

    #[test]
    fn workspace_grid_uses_central_geometry() {
        let (left, top, tile_w, tile_h) = super::grid_geometry();
        let layout = niwoe_tokens::WorkspaceSwitcher::DEFAULT;
        assert_eq!(left, crate::popup_card::PAD_X);
        assert_eq!(top, crate::popup_card::BODY_TOP);
        assert!(tile_w > 0 && tile_h > 0);
        assert_eq!(layout.columns * layout.rows, 9);
    }
}
