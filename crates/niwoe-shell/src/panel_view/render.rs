fn collect_click_zones(
    widget: &dyn Widget,
    node: &LayoutNode,
    parent_x: i32,
    parent_y: i32,
    out: &mut Vec<ClickZone>,
) {
    let abs_x = parent_x + node.rect.x;
    let abs_y = parent_y + node.rect.y;

    let action = widget
        .id()
        .and_then(action_for_id_as_click)
        .or_else(|| widget.pinned_app_idx().map(ClickAction::LaunchPinnedApp))
        .or_else(|| {
            widget
                .focus_window_id()
                .map(|id| ClickAction::FocusWindow(id.to_string()))
        });

    if let Some(action) = action {
        out.push(ClickZone {
            id: widget.id().map(str::to_string),
            rect: ShellRect {
                x: abs_x,
                y: abs_y,
                w: node.rect.width,
                h: node.rect.height,
            },
            action,
        });
    }

    for (child, child_node) in widget.children().iter().zip(node.children.iter()) {
        collect_click_zones(child.as_ref(), child_node, abs_x, abs_y, out);
    }
}

// ── draw_panel_ui ───────────────────────────────────────────────────────────

fn blit_rgba_to_argb(src: &[u8], dst: &mut [u8]) {
    if src.len() != dst.len() || !src.len().is_multiple_of(4) {
        return;
    }
    for (rgba, argb) in src
        .as_chunks::<4>()
        .0
        .iter()
        .zip(dst.as_chunks_mut::<4>().0.iter_mut())
    {
        argb[0] = rgba[2];
        argb[1] = rgba[1];
        argb[2] = rgba[0];
        argb[3] = rgba[3];
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_panel_ui(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    pinned_apps: &[PinnedApp],
    window_entries: &[PanelWindowEntry],
    network_state: &NetworkState,
    audio_snapshot: &AudioSnapshot,
    status_notifier_items: &[StatusNotifierItem],
    network_popup_open: bool,
    audio_popup_open: bool,
    battery: &crate::battery::BatterySnapshot,
    power_profile: Option<crate::power_profile::PowerProfile>,
    active_workspace: u8,
    total_workspaces: u8,
    clock: &str,
    icon_cache: &IconCache,
    screenshot_icon: Option<Pixmap>,
    theme_config: &niwoe_config::ThemeConfig,
    state_fn: &dyn Fn(&[usize]) -> WidgetState,
    clicks_out: &mut Vec<ClickZone>,
) {
    let expected_len = (width as usize)
        .saturating_mul(height as usize)
        .saturating_mul(4);
    if canvas.len() != expected_len {
        tracing::warn!(
            "draw_panel_ui: canvas size mismatch, expected {} got {}",
            expected_len,
            canvas.len()
        );
        return;
    }

    let theme = glass_theme_from_config(theme_config);
    let treatment = theme_config
        .decorations
        .surface_treatment(ThemeSurface::Panel);

    let root = build_panel_widget_tree(
        width,
        pinned_apps,
        window_entries,
        network_state,
        audio_snapshot,
        status_notifier_items,
        network_popup_open,
        audio_popup_open,
        battery,
        power_profile,
        active_workspace,
        total_workspaces,
        clock,
        icon_cache,
        screenshot_icon,
        &theme,
    );

    let Ok(layout) = compute_layout(&*root, PixelSize { width, height }) else {
        return;
    };

    let Some(mut pixmap) = Pixmap::new(width, height) else {
        return;
    };
    // Edge-attached bar: one token-driven surface and a quiet bottom separator.
    let base = theme.palette.background;
    let body = Rect {
        x: SIDE_MARGIN,
        y: ISLAND_TOP,
        width: (width as i32 - 2 * SIDE_MARGIN).max(0),
        height: PANEL_H,
    };
    {
        let mut pc = pixmap.as_mut();
        if let Some(path) = rounded_rect_path(body, PanelTokens::DEFAULT.edge_radius) {
            paint_fill(
                &mut pc,
                &path,
                Color::rgba(base.r, base.g, base.b, treatment.fill_alpha),
            );
        }
        let border = niwoe_tokens::Controls::BORDER;
        let separator = Rect {
            x: body.x,
            y: body.y + body.height - border,
            width: body.width,
            height: border,
        };
        if let Some(path) = rounded_rect_path(separator, PanelTokens::DEFAULT.edge_radius) {
            paint_fill(&mut pc, &path, theme.palette.border);
        }
        let _ = render(&*root, &layout, &mut pc, &theme, state_fn);
    }
    blit_rgba_to_argb(pixmap.data(), canvas);

    clicks_out.clear();
    collect_click_zones(&*root, &layout.root, 0, 0, clicks_out);
}
