macro_rules! handle_launcher_pointer {
    ($shell:ident, $qh:ident, $event:ident) => {{
        if $shell.pointer_surface == SurfaceKind::Launcher {
            let (content_width, content_height) = $shell.launcher_content_size();
            // Translate to content-buffer coordinates first.
            // In fullscreen mode the launcher surface covers the full screen but
            // draw_overlay / hit tests operate on the fitted content
            // buffer, so we need coords relative to that buffer, not the surface.
            // A click outside the visual area is discarded early.
            let local_pos = if $shell.launcher_is_fullscreen {
                let (px, py) = $event.position;
                let vx = $shell.launcher_visual_x as f64;
                let vy = $shell.launcher_visual_y as f64;
                let (fitted_width, fitted_height) = $shell.launcher_fitted_size();
                let (lw, lh) = (fitted_width as f64, fitted_height as f64);
                if px < vx || px >= vx + lw || py < vy || py >= vy + lh {
                    if let PointerEventKind::Press { button: 0x110, .. } = $event.kind {
                        $shell.close_launcher_after_launch($qh, RepaintReason::Pointer);
                    }
                    continue;
                }
                (
                    (px - vx) * content_width as f64 / lw,
                    (py - vy) * content_height as f64 / lh,
                )
            } else {
                $event.position
            };

            if $shell.workspace_state.rooms.wizard.open {
                if workspace_click_activation(&$event.kind) {
                    if let Some(index) = crate::room_management_view::first_run::hit(local_pos.0 as i32,
                        local_pos.1 as i32, content_width, content_height, &$shell.workspace_state.rooms.wizard) {
                        $shell.first_run_action($qh, index);
                    }
                }
                continue;
            }
            if $shell.window_picker_pointer(
                $qh,
                &$event.kind,
                local_pos,
                content_width,
                content_height,
            ) {
                continue;
            }
            if $shell.hub_pointer($qh, &$event.kind, local_pos, content_width, content_height) {
                continue;
            }
            // ── Step 0: Context-menu left-click — before the widget tree so clicking
            //    a menu item does not also fire the underlying tile.
            if let PointerEventKind::Press { button: 0x110, .. } = $event.kind {
                if let Some(cm) = $shell.context_menu.take() {
                    let items = context_menu::item_list(
                        cm.is_terminal,
                        cm.is_pinned,
                        cm.running_window_id.is_some(),
                    );
                    let n = items.len();
                    if let Some(idx) = context_menu::hit_item(&cm, n, local_pos.0, local_pos.1) {
                        let action = items[idx].1;
                        $shell.handle_context_menu_action($qh, action, &cm);
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                        continue;
                    }
                    // Click outside menu: dismiss and fall through to widget tree.
                    $shell.draw_launcher($qh, RepaintReason::Pointer);
                }
            }

            // ── Step 2: Context-menu hover tracking.
            if let PointerEventKind::Motion { .. } = $event.kind {
                if let Some(ref mut cm) = $shell.context_menu {
                    let items = context_menu::item_list(
                        cm.is_terminal,
                        cm.is_pinned,
                        cm.running_window_id.is_some(),
                    );
                    let n = items.len();
                    let new_hover = context_menu::hit_item(cm, n, local_pos.0, local_pos.1);
                    if new_hover != cm.hover_idx {
                        cm.hover_idx = new_hover;
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                    }
                }
            }

            // Hub room-card hover tracking.
            if let PointerEventKind::Motion { .. } = $event.kind {
                if $shell.room_management_open && $shell.room_configuration_id.is_none() {
                    let next = crate::room_management_view::hit_room(
                        local_pos.0 as i32,
                        local_pos.1 as i32,
                        content_width,
                        content_height,
                        $shell.visible_rooms().len(),
                        $shell.room_management_page,
                    );
                    if next != $shell.hovered_bento_idx {
                        $shell.hovered_bento_idx = next;
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                    }
                } else if !$shell.launcher_settings_open && !$shell.hub_search_active {
                    let next = crate::hub_view::hit_room(
                        local_pos.0 as i32,
                        local_pos.1 as i32,
                        content_width,
                        $shell.workspace_state.rooms.snapshot.rooms.len(),
                    );
                    if next != $shell.hovered_bento_idx {
                        $shell.hovered_bento_idx = next;
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                    }
                }
            }

            // ── Step 2b: Hub search-result hover tracking.
            if let PointerEventKind::Motion { .. } = $event.kind {
                if !$shell.launcher_settings_open && $shell.hub_search_active {
                    let new_app = {
                        let hit = crate::app_view::hit_app_row(
                            local_pos.0 as i32,
                            local_pos.1 as i32,
                            $shell.app_view_scroll_y,
                            content_width,
                            content_height,
                        );
                        let filtered = crate::app_view::collect_palette_apps(
                            &$shell.launcher_state.apps,
                            &$shell.search_query,
                            &$shell.hidden_execs,
                            $shell.launcher_state.category,
                            &$shell.pinned_apps,
                        );
                        hit.filter(|&i| i < filtered.len())
                    };
                    if new_app != $shell.hovered_app_card_idx {
                        $shell.hovered_app_card_idx = new_app;
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                    }
                }
            }

            // ── Step 3: Right-click — open context menu for app under cursor.
            if let PointerEventKind::Press { button: 0x111, .. } = $event.kind {
                $shell.context_menu = None;
                if !$shell.launcher_settings_open && $shell.hub_search_active {
                    let filtered = crate::app_view::collect_palette_apps(
                        &$shell.launcher_state.apps,
                        &$shell.search_query,
                        &$shell.hidden_execs,
                        $shell.launcher_state.category,
                        &$shell.pinned_apps,
                    );
                    let hit = crate::app_view::hit_app_row(
                        local_pos.0 as i32,
                        local_pos.1 as i32,
                        $shell.app_view_scroll_y,
                        content_width,
                        content_height,
                    );
                    if let Some(idx) = hit.filter(|&i| i < filtered.len()) {
                        let app = filtered[idx];
                        let exec_str: Box<str> = app.program.clone().into();
                        let app_name: Box<str> = app.name.clone().into();
                        let is_terminal = app.terminal;
                        let is_pinned = $shell
                            .pinned_apps
                            .iter()
                            .any(|p| p.program == exec_str.as_ref());
                        let running_window_id = $shell
                            .windows
                            .iter()
                            .find(|w| {
                                w.app_id
                                    .as_deref()
                                    .map(|a| {
                                        a.eq_ignore_ascii_case(&exec_str)
                                            || a.to_lowercase().contains(&exec_str.to_lowercase())
                                    })
                                    .unwrap_or(false)
                            })
                            .map(|w| w.id.clone());
                        let is_running = running_window_id.is_some();
                        let items = context_menu::item_list(is_terminal, is_pinned, is_running);
                        let (mx, my) = context_menu::clamp_position(
                            local_pos.0 as i32,
                            local_pos.1 as i32,
                            items.len(),
                            content_width as i32,
                            content_height as i32,
                        );
                        $shell.context_menu = Some(context_menu::ContextMenuState {
                            x: mx,
                            y: my,
                            app_name,
                            exec: exec_str,
                            is_terminal,
                            is_pinned,
                            running_window_id,
                            hover_idx: None,
                        });
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                        continue;
                    }
                }
                // Fall through to widget tree for settings right-click
                let tree = if $shell.launcher_settings_open {
                    crate::settings_view::build_settings_widget_tree(
                        content_width,
                        content_height,
                        $shell.settings_category,
                        &$shell.settings_search,
                        &$shell.available_themes,
                        &$shell.theme_name,
                        &$shell.available_wallpapers,
                        &$shell.wallpaper_thumbnails,
                        $shell.wallpaper_path.as_deref(),
                        $shell.wallpaper_mode,
                        $shell.cursor_size,
                        $shell.available_cursor_themes.as_slice(),
                        $shell.cursor_theme.as_str(),
                        $shell.idle_timeout_secs,
                        &$shell.pinned_apps,
                        &$shell.output_workspaces,
                        $shell.display_mode_dropdown_open,
                        &$shell.printer_snapshot,
                        &$shell.audio_snapshot,
                        &$shell.system_info,
                        $shell.network_controller.state(),
                        $shell.network_profiles.as_slice(),
                        &$shell.bluetooth_snapshot,
                        $shell.wifi_networks.as_slice(),
                        $shell.settings_pinned_adding,
                        &$shell.launcher_state.apps,
                        &$shell.icon_cache,
                        None,
                        $shell.default_apps_index.as_ref(),
                        &$shell.default_apps_current,
                        $shell.default_apps_picker_open,
                        &crate::ui::tokens::theme_from_config(&$shell.theme),
                    )
                } else {
                    return;
                };
                let pixel_size = niwoe_ui::PixelSize {
                    width: content_width,
                    height: content_height,
                };
                if let Ok(layout) = niwoe_ui::compute_layout(&*tree, pixel_size) {
                    let pos = niwoe_ui::PointerPosition {
                        x: local_pos.0 as i32,
                        y: local_pos.1 as i32,
                    };
                    if let Some(path) = niwoe_ui::hit_test(&layout, pos) {
                        if let Some(widget) =
                            crate::widget_traversal::find_widget_at_path(&*tree, &path)
                        {
                            if let Some(exec) = widget.launch_exec() {
                                let app = $shell
                                    .launcher_state
                                    .apps
                                    .iter()
                                    .find(|a| a.program == exec);
                                let app_name: Box<str> =
                                    app.map(|a| a.name.as_str()).unwrap_or(exec).into();
                                let is_terminal = app.map(|a| a.terminal).unwrap_or(false);
                                let exec_str: Box<str> = exec.into();
                                let is_pinned = $shell
                                    .pinned_apps
                                    .iter()
                                    .any(|p| p.program == exec_str.as_ref());
                                let running_window_id = $shell
                                    .windows
                                    .iter()
                                    .find(|w| {
                                        w.app_id
                                            .as_deref()
                                            .map(|a| {
                                                a.eq_ignore_ascii_case(&exec_str)
                                                    || a.to_lowercase()
                                                        .contains(&exec_str.to_lowercase())
                                            })
                                            .unwrap_or(false)
                                    })
                                    .map(|w| w.id.clone());
                                let is_running = running_window_id.is_some();
                                let items =
                                    context_menu::item_list(is_terminal, is_pinned, is_running);
                                let (mx, my) = context_menu::clamp_position(
                                    local_pos.0 as i32,
                                    local_pos.1 as i32,
                                    items.len(),
                                    content_width as i32,
                                    content_height as i32,
                                );
                                $shell.context_menu = Some(context_menu::ContextMenuState {
                                    x: mx,
                                    y: my,
                                    app_name,
                                    exec: exec_str,
                                    is_terminal,
                                    is_pinned,
                                    running_window_id,
                                    hover_idx: None,
                                });
                                $shell.draw_launcher($qh, RepaintReason::Pointer);
                            }
                        }
                    }
                }
                continue;
            }

            // ── Step 4: Scroll in the launcher.
            if let PointerEventKind::Axis { vertical, .. } = $event.kind {
                if $shell.room_configuration_id.is_some() {
                    let delta = if vertical.discrete != 0 {
                        -vertical.discrete * niwoe_tokens::Spacing::DEFAULT.xxl
                    } else {
                        (-vertical.absolute * 4.0) as i32
                    };
                    if $shell.room_target_menu_scroll($qh, delta) {
                        continue;
                    }
                    let max = crate::room_management_view::max_configuration_scroll(content_height);
                    let next = ($shell.room_configuration_scroll_y + delta).clamp(0, max);
                    if next != $shell.room_configuration_scroll_y {
                        $shell.room_configuration_scroll_y = next;
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                    }
                    continue;
                } else if $shell.room_management_open {
                    let count = $shell.visible_rooms().len();
                    let max = crate::room_management_view::max_room_page(count);
                    let next = if vertical.discrete < 0 || vertical.absolute < -1.0 {
                        $shell.room_management_page.saturating_add(1).min(max)
                    } else if vertical.discrete > 0 || vertical.absolute > 1.0 {
                        $shell.room_management_page.saturating_sub(1)
                    } else {
                        $shell.room_management_page
                    };
                    if next != $shell.room_management_page {
                        $shell.room_management_page = next;
                        $shell.hovered_bento_idx = None;
                        $shell.draw_launcher($qh, RepaintReason::Pointer);
                    }
                    continue;
                } else if !$shell.launcher_settings_open && $shell.hub_search_active {
                    let step_px: i32 = 60;
                    let delta_px = if vertical.discrete != 0 {
                        $shell.launcher_scroll_remainder = 0.0;
                        -vertical.discrete * step_px
                    } else {
                        $shell.launcher_scroll_remainder += -vertical.absolute * 4.0;
                        if $shell.launcher_scroll_remainder.abs() < 6.0 {
                            0
                        } else {
                            let delta = $shell.launcher_scroll_remainder.trunc() as i32;
                            $shell.launcher_scroll_remainder -= f64::from(delta);
                            delta
                        }
                    };
                    let max_scroll = crate::app_view::max_scroll_for_palette(
                        &$shell.launcher_state.apps,
                        &$shell.search_query,
                        &$shell.hidden_execs,
                        $shell.launcher_state.category,
                        &$shell.pinned_apps,
                        content_height,
                    );
                    if delta_px != 0 {
                        let new_scroll = ($shell.app_view_scroll_y + delta_px).clamp(0, max_scroll);
                        if new_scroll != $shell.app_view_scroll_y {
                            $shell.app_view_scroll_y = new_scroll;
                            $shell.draw_launcher($qh, RepaintReason::Pointer);
                        }
                    }
                }
            }

            // ── Step 5: Hub overview and inline-search click handling.
            if $shell.room_management_open {
                if let PointerEventKind::Press { button: 0x110, .. } = $event.kind {
                    $shell.control_center_click(
                        $qh,
                        local_pos.0 as i32,
                        local_pos.1 as i32,
                        content_width,
                        content_height,
                    );
                    continue;
                }
            } else if !$shell.launcher_settings_open && !$shell.hub_search_active {
                if let PointerEventKind::Press { button: 0x110, .. } = $event.kind {
                    let cx = local_pos.0 as i32;
                    let cy = local_pos.1 as i32;
                    if crate::hub_view::hit_close(cx, cy, content_width) {
                        $shell.close_launcher_after_launch($qh, RepaintReason::Pointer);
                        continue;
                    }
                    if crate::hub_view::hit_manage_rooms(cx, cy, content_width) {
                        $shell.open_room_management($qh);
                        continue;
                    }
                }
                if workspace_click_activation(&$event.kind) {
                    let cx = local_pos.0 as i32;
                    let cy = local_pos.1 as i32;
                    let room_count = $shell.workspace_state.rooms.snapshot.rooms.len();
                    if let Some(index) =
                        crate::hub_view::hit_room(cx, cy, content_width, room_count)
                    {
                        if let Some(workspace) = $shell
                            .workspace_state
                            .rooms
                            .snapshot
                            .rooms
                            .get(index)
                            .map(|room| room.workspace)
                        {
                            $shell.handle_panel_click(
                                $qh,
                                crate::wayland::ClickAction::SwitchWorkspace(workspace),
                            );
                            $shell.close_launcher_after_launch($qh, RepaintReason::Pointer);
                            continue;
                        }
                    }
                }
            }
            if !$shell.launcher_settings_open && $shell.hub_search_active {
                if let PointerEventKind::Press { button: 0x110, .. } = $event.kind {
                    let cx = local_pos.0 as i32;
                    let cy = local_pos.1 as i32;

                    // App grid / search results
                    if let Some(idx) = crate::app_view::hit_app_row(
                        cx,
                        cy,
                        $shell.app_view_scroll_y,
                        content_width,
                        content_height,
                    ) {
                        let filtered = crate::app_view::collect_palette_apps(
                            &$shell.launcher_state.apps,
                            &$shell.search_query,
                            &$shell.hidden_execs,
                            $shell.launcher_state.category,
                            &$shell.pinned_apps,
                        );
                        if let Some(app) = filtered.get(idx) {
                            crate::launcher::LauncherState::launch_desktop_app(
                                (*app).clone(),
                                &mut $shell.ipc,
                            );
                            $shell.close_launcher_after_launch($qh, RepaintReason::Pointer);
                            continue;
                        }
                    }
                }
            }

            if $shell.launcher_settings_open {
                if let Some(ev) = translate_pointer_event(&$event.kind, local_pos) {
                    let tree = {
                        crate::settings_view::build_settings_widget_tree(
                            content_width,
                            content_height,
                            $shell.settings_category,
                            &$shell.settings_search,
                            &$shell.available_themes,
                            &$shell.theme_name,
                            &$shell.available_wallpapers,
                            &$shell.wallpaper_thumbnails,
                            $shell.wallpaper_path.as_deref(),
                            $shell.wallpaper_mode,
                            $shell.cursor_size,
                            $shell.available_cursor_themes.as_slice(),
                            $shell.cursor_theme.as_str(),
                            $shell.idle_timeout_secs,
                            &$shell.pinned_apps,
                            &$shell.output_workspaces,
                            $shell.display_mode_dropdown_open,
                            &$shell.printer_snapshot,
                            &$shell.audio_snapshot,
                            &$shell.system_info,
                            $shell.network_controller.state(),
                            $shell.network_profiles.as_slice(),
                            &$shell.bluetooth_snapshot,
                            $shell.wifi_networks.as_slice(),
                            $shell.settings_pinned_adding,
                            &$shell.launcher_state.apps,
                            &$shell.icon_cache,
                            None,
                            $shell.default_apps_index.as_ref(),
                            &$shell.default_apps_current,
                            $shell.default_apps_picker_open,
                            &crate::ui::tokens::theme_from_config(&$shell.theme),
                        )
                    };
                    let pixel_size = niwoe_ui::PixelSize {
                        width: content_width,
                        height: content_height,
                    };
                    let layout = niwoe_ui::compute_layout(&*tree, pixel_size);
                    match layout {
                        Ok(layout) => {
                            let pos = niwoe_ui::PointerPosition {
                                x: local_pos.0 as i32,
                                y: local_pos.1 as i32,
                            };
                            let path = niwoe_ui::hit_test(&layout, pos);
                            let clicked_path = super::pointer_state::detect_click(
                                $shell.ui_preview_widget_state.as_ref(),
                                &ev,
                                path.as_ref(),
                            );
                            let new_state = super::pointer_state::apply_pointer_event(
                                $shell.ui_preview_widget_state.clone(),
                                &ev,
                                path,
                            );
                            if new_state != $shell.ui_preview_widget_state {
                                $shell.ui_preview_widget_state = new_state;
                                $shell.draw_launcher($qh, RepaintReason::Pointer);
                            }
                            if let Some(clicked_path) = clicked_path {
                                if let Some(widget) = crate::widget_traversal::find_widget_at_path(
                                    &*tree,
                                    &clicked_path,
                                ) {
                                    if let Some(action) =
                                        widget.id().and_then(crate::widget_action::action_for_id)
                                    {
                                        $shell.dispatch_widget_action($qh, action);
                                    } else if let Some(exec) = widget.launch_exec() {
                                        $shell.dispatch_widget_action(
                                            $qh,
                                            crate::widget_action::WidgetAction::LaunchExec(
                                                exec.to_string(),
                                            ),
                                        );
                                    } else if let Some((program, args)) = widget.launch_info() {
                                        $shell.dispatch_widget_action(
                                            $qh,
                                            crate::widget_action::WidgetAction::LaunchApp {
                                                program: program.to_string(),
                                                args: args.to_vec(),
                                            },
                                        );
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!("launcher layout failed: {:?}", e);
                        }
                    }
                }
            } // end launcher_settings_open
            continue;
        }
    }};
}
