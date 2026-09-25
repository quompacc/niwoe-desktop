use std::cell::RefCell;

use niwoe_config::ThemeConfig;
use niwoe_tokens::{Interaction, Radius, WorkspaceSwitcher};

use crate::{
    popup_card::{draw_card_body, draw_card_title, BODY_TOP, PAD_BOTTOM, PAD_X},
    ui::primitives::{draw_active_indicator, ActiveIndicatorEdge},
    ClickAction, ClickZone, Painter, Rect, TextRenderer, WORKSPACE_POPUP_HEIGHT,
    WORKSPACE_POPUP_WIDTH,
};

const LAYOUT: WorkspaceSwitcher = WorkspaceSwitcher::DEFAULT;
const CELL_COUNT: usize = (LAYOUT.columns * LAYOUT.rows) as usize;

pub struct WorkspacePopupState {
    pub clicks: Vec<ClickZone>,
    pub(crate) rooms: crate::room_editor::RoomUi,
}

impl WorkspacePopupState {
    pub fn new() -> Self {
        Self {
            clicks: Vec::new(),
            rooms: Default::default(),
        }
    }
}

pub struct WorkspacePopupInput {
    pub active_workspace: u32,
    pub total_workspaces: u32,
    pub occupied: [bool; niwoe_config::rooms::MAX_ROOMS],
    pub hovered_idx: Option<usize>,
}

fn grid_geometry() -> (i32, i32, i32, i32) {
    let width = WORKSPACE_POPUP_WIDTH as i32;
    let height = WORKSPACE_POPUP_HEIGHT as i32;
    let grid_left = PAD_X;
    let grid_top = BODY_TOP;
    let grid_w = width - 2 * PAD_X;
    let grid_bottom = height - PAD_BOTTOM;
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
    let total_workspaces = input.total_workspaces.max(1);

    draw_card_body(painter, theme);
    draw_card_title(
        painter,
        font,
        theme,
        if state.rooms.edit.is_some() {
            "Raum bearbeiten"
        } else {
            "Räume · Rechtsklick zum Bearbeiten"
        },
    );
    if state.rooms.edit.is_some() {
        crate::room_editor::draw(painter, font, theme, &state.rooms, &mut state.clicks);
        return;
    }

    let (left, top, tile_w, tile_h) = grid_geometry();

    for i in 0..CELL_COUNT.min(state.rooms.snapshot.rooms.len()) {
        let room = &state.rooms.snapshot.rooms[i];
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
        let is_hovered = input.hovered_idx == Some(i);

        let resting_bg = if is_active {
            Interaction::DEFAULT.selection(
                colors.surface_alt,
                colors.accent,
                Interaction::SELECTION_ACTIVE,
            )
        } else if is_occupied {
            Interaction::DEFAULT.selection(
                colors.surface_alt,
                colors.accent,
                Interaction::SELECTION_FOCUSED,
            )
        } else {
            colors.surface_alt
        };
        let bg = if is_hovered {
            Interaction::DEFAULT.hover(resting_bg)
        } else {
            resting_bg
        };
        painter.roundish_rect_with_radius(rect, bg, Radius::DEFAULT.sm);

        if is_active {
            draw_active_indicator(painter, rect, ActiveIndicatorEdge::Top, theme);
        }

        let text_color = if is_active || is_occupied || is_hovered {
            colors.text
        } else {
            colors.text_dim
        };
        painter.text_clipped(
            font,
            &room.name,
            rect.x + LAYOUT.tile_pad,
            rect.y + LAYOUT.tile_pad + 10,
            rect.w - 2 * LAYOUT.tile_pad,
            text_color,
        );
        let state_label = if is_active {
            "AKTIV"
        } else if is_occupied {
            "BELEGT"
        } else {
            "FREI"
        };
        painter.text_clipped(
            font,
            state_label,
            rect.x + LAYOUT.tile_pad,
            rect.y + rect.h - LAYOUT.tile_pad,
            rect.w - 2 * LAYOUT.tile_pad,
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
                colors.accent,
                dot_size,
            );
        }
        state.clicks.push(ClickZone {
            id: Some(format!("workspace-popup-{ws_id}")),
            rect,
            action: ClickAction::SwitchWorkspace(ws_id.clamp(1, total_workspaces) as u8),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::ClickAction;

    use super::{
        draw_workspace_popup, workspace_popup_hover_idx, WorkspacePopupInput, WorkspacePopupState,
    };

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
                total_workspaces: 9,
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
