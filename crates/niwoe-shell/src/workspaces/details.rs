use super::*;

pub(super) fn draw(
    painter: &mut Painter<'_>,
    font: &RefCell<Option<TextRenderer>>,
    theme: &ThemeConfig,
    state: &mut WorkspacePopupState,
    hovered: Option<usize>,
) {
    let range = state.visible_range();
    let position = hovered
        .map(|i| range.start + i)
        .filter(|i| range.contains(i))
        .unwrap_or(state.selected);
    let width = WORKSPACE_POPUP_WIDTH as i32 - 2 * PAD_X;
    let top = WORKSPACE_POPUP_HEIGHT as i32 - PAD_BOTTOM - LAYOUT.footer_height;
    if let Some(room) = state.rooms.snapshot.rooms.get(position) {
        let mut line = String::new();
        let mut baseline = top + LAYOUT.tile_pad + LAYOUT.line_height;
        for character in room.name.chars() {
            let candidate = format!("{line}{character}");
            let measured = font
                .borrow_mut()
                .as_mut()
                .map(|r| r.measure_text(&candidate))
                .unwrap_or(
                    candidate.chars().count() as i32
                        * niwoe_tokens::Typography::DEFAULT.caption_size as i32,
                );
            if measured > width && !line.is_empty() {
                painter.text_clipped(font, &line, PAD_X, baseline, width, theme.colors.text);
                baseline += LAYOUT.line_height;
                line.clear();
            }
            line.push(character);
        }
        painter.text_clipped(font, &line, PAD_X, baseline, width, theme.colors.text);
    }
    let y = WORKSPACE_POPUP_HEIGHT as i32 - PAD_BOTTOM - LAYOUT.navigation_height;
    let button_width = width / 3;
    for (step, label, x, enabled) in [
        (-1, "Zurück", PAD_X, range.start > 0),
        (
            1,
            "Weiter",
            PAD_X + width - button_width,
            range.end < state.rooms.snapshot.rooms.len(),
        ),
    ] {
        let rect = Rect {
            x,
            y,
            w: button_width,
            h: LAYOUT.navigation_height,
        };
        painter.roundish_rect_with_radius(rect, theme.colors.surface_alt, Radius::DEFAULT.sm);
        painter.text_centered(
            font,
            label,
            rect,
            if enabled {
                theme.colors.text
            } else {
                theme.colors.text_dim
            },
        );
        if enabled {
            state.clicks.push(ClickZone {
                id: Some(format!("workspace-page-{step}")),
                rect,
                action: ClickAction::WorkspacePage(step),
            });
        }
    }
    let pages = state.rooms.snapshot.rooms.len().div_ceil(CELL_COUNT).max(1);
    painter.text_centered(
        font,
        &format!("{} / {pages}", range.start / CELL_COUNT + 1),
        Rect {
            x: PAD_X + button_width,
            y,
            w: width - 2 * button_width,
            h: LAYOUT.navigation_height,
        },
        theme.colors.text_dim,
    );
}
