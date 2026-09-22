fn draw_circle(canvas: &mut PixmapMut<'_>, cx: f32, cy: f32, radius: f32, color: Color) {
    use tiny_skia::{FillRule, Paint, PathBuilder, Transform};
    let mut pb = PathBuilder::new();
    pb.push_circle(cx, cy, radius);
    if let Some(path) = pb.finish() {
        let mut paint = Paint {
            anti_alias: true,
            ..Paint::default()
        };
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        canvas.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

// ── build_panel_widget_tree ─────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
/// Recolour a premultiplied-RGBA pixmap to `color`, keeping its alpha (shape).
/// Used to tint the symbolic battery icon to the active power-profile colour.
fn tint_pixmap_premul(pm: &mut Pixmap, color: Color) {
    for px in pm.data_mut().as_chunks_mut::<4>().0.iter_mut() {
        let a = px[3] as u16;
        px[0] = ((color.r as u16 * a) / 255) as u8;
        px[1] = ((color.g as u16 * a) / 255) as u8;
        px[2] = ((color.b as u16 * a) / 255) as u8;
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_panel_widget_tree(
    width: u32,
    _pinned_apps: &[PinnedApp],
    _window_entries: &[PanelWindowEntry],
    network_state: &NetworkState,
    audio_snapshot: &AudioSnapshot,
    status_notifier_items: &[StatusNotifierItem],
    network_popup_open: bool,
    audio_popup_open: bool,
    battery: &crate::battery::BatterySnapshot,
    _power_profile: Option<crate::power_profile::PowerProfile>,
    active_workspace: u8,
    total_workspaces: u8,
    clock: &str,
    icon_cache: &IconCache,
    screenshot_icon: Option<Pixmap>,
    theme: &Theme,
) -> Box<dyn Widget> {
    use status_symbols::{icon, Symbol};
    let connected = matches!(network_state, NetworkState::Connected { .. });
    let network_symbol = if matches!(
        network_state,
        NetworkState::Connected {
            kind: crate::network::ConnectionKind::Wifi { .. },
            ..
        }
    ) {
        Symbol::Wifi(connected)
    } else {
        Symbol::Wired(connected)
    };
    let network_icon = icon(network_symbol, theme.palette.text);
    let muted = audio_snapshot
        .default_output
        .as_ref()
        .is_none_or(|d| d.muted || d.volume_percent == Some(0));
    let audio_icon = icon(Symbol::Audio(muted), theme.palette.text);
    let battery_icon = icon(
        Symbol::Battery(battery.capacity.min(100) / 25, battery.on_ac),
        theme.palette.text,
    );

    // Left cluster
    let mut left_children: Vec<Box<dyn Widget>> = Vec::new();
    let launcher_icon = build_launcher_icon(theme);
    left_children.push(Box::new(PanelChip::new(
        "panel-launcher",
        "Apps".into(),
        launcher_icon,
        LAUNCHER_W,
        false,
    )));
    left_children.push(Box::new(PanelWorkspaceChip {
        active: active_workspace,
        total: total_workspaces,
    }));
    // Budget rooms before optional app shortcuts. The active room is never
    // hidden; the room menu retains access to every slot on narrow outputs.
    let tray_count = status_notifier_items.len().min(SNI_PANEL_IDS.len());
    let status_width =
        TRAY_W * if battery.present { 3 } else { 2 } + if battery.present { LAUNCHER_W } else { 0 };
    let reserved = LEFT_PADDING
        + RIGHT_PADDING
        + LAUNCHER_W
        + WS_W
        + CLOCK_W
        + status_width
        + SCREENSHOT_W
        + DIVIDER_W * 3
        + (tray_count as i32 * (SNI_W + GAP))
        + GAP * 8;
    let room_space = (width as i32 - reserved).max(0);
    let room_stride = PanelTokens::DEFAULT.room_width as i32 + GAP;
    let capacity = (room_space / room_stride).max(1) as usize;
    let rooms = visible_rooms(active_workspace, total_workspaces, capacity);
    for workspace in rooms {
        left_children.push(Box::new(RoomTab {
            workspace,
            active: workspace == active_workspace,
        }));
    }
    let left_cluster = Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            align_items: Some(AlignItems::Center),
            gap: UiSize {
                width: ui_length(GAP as f32),
                height: ui_length(0.0_f32),
            },
            ..Default::default()
        },
        left_children,
    );

    // Center cluster — empty spacer; window indicators are now shown as badges on pinned icons
    let center_children: Vec<Box<dyn Widget>> = Vec::new();
    let center_cluster = Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            flex_grow: 1.0,
            align_items: Some(AlignItems::Center),
            gap: UiSize {
                width: ui_length(GAP as f32),
                height: ui_length(0.0_f32),
            },
            overflow: TaffyPoint {
                x: Overflow::Hidden,
                y: Overflow::Hidden,
            },
            ..Default::default()
        },
        center_children,
    );

    // Right cluster
    let mut right_children: Vec<Box<dyn Widget>> = Vec::new();
    for (idx, item) in status_notifier_items
        .iter()
        .take(SNI_PANEL_IDS.len())
        .enumerate()
    {
        let icon = item
            .icon_name
            .as_deref()
            .and_then(|name| icon_cache.lookup(name, STATUS_ICON_SIZE))
            .and_then(icon_image_to_pixmap)
            .map(|mut icon| {
                tint_pixmap_premul(&mut icon, theme.palette.text);
                icon
            });
        right_children.push(Box::new(PanelChip::new(
            SNI_PANEL_IDS[idx],
            status_notifier_label(item).into_boxed_str(),
            icon,
            SNI_W,
            false,
        )));
    }
    if !right_children.is_empty() {
        right_children.push(Box::new(PanelDivider));
    }
    let screenshot_icon = screenshot_icon.map(|mut icon| {
        tint_pixmap_premul(&mut icon, theme.palette.text);
        icon
    });
    right_children.push(Box::new(PanelChip::new(
        "panel-screenshot",
        "Foto".into(),
        screenshot_icon,
        SCREENSHOT_W,
        false,
    )));
    right_children.push(Box::new(PanelDivider));

    let mut status_children: Vec<Box<dyn Widget>> = vec![
        Box::new(PanelStatusIcon {
            show_label: false,
            label: "NET".into(),
            icon: network_icon,
        }),
        Box::new(PanelStatusIcon {
            show_label: false,
            label: audio_snapshot.panel_label().into_boxed_str(),
            icon: audio_icon,
        }),
    ];
    if battery.present {
        status_children.push(Box::new(PanelStatusIcon {
            show_label: true,
            label: battery.label().into_boxed_str(),
            icon: battery_icon,
        }));
    }
    right_children.push(Box::new(PanelStatusGroup {
        active: network_popup_open || audio_popup_open,
        children: status_children,
    }));
    right_children.extend([
        Box::new(PanelDivider) as Box<dyn Widget>,
        Box::new(PanelClockChip {
            value: clock.into(),
        }),
    ]);
    let right_cluster = Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            align_items: Some(AlignItems::Center),
            gap: UiSize {
                width: ui_length(GAP as f32),
                height: ui_length(0.0_f32),
            },
            ..Default::default()
        },
        right_children,
    );

    // The bar holds the three clusters and is inset to the island content box.
    let inner_w = (width as i32 - 2 * SIDE_MARGIN).max(0);
    let bar = Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            justify_content: Some(JustifyContent::SpaceBetween),
            align_items: Some(AlignItems::Center),
            size: UiSize {
                width: ui_length(inner_w as f32),
                height: ui_length(PANEL_H as f32),
            },
            padding: TaffyRect {
                left: ui_length(LEFT_PADDING as f32),
                right: ui_length(RIGHT_PADDING as f32),
                top: ui_length(0.0_f32),
                bottom: ui_length(0.0_f32),
            },
            ..Default::default()
        },
        vec![
            Box::new(left_cluster) as Box<dyn Widget>,
            Box::new(center_cluster) as Box<dyn Widget>,
            Box::new(right_cluster) as Box<dyn Widget>,
        ],
    );

    // Outer surface wrapper: full width, inset on the sides + a bottom gap so
    // the island floats. The padding offsets the bar so render() and the click
    // zones inherit the inset automatically.
    Box::new(Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            align_items: Some(AlignItems::FlexStart),
            size: UiSize {
                width: ui_length(width as f32),
                height: ui_length(SURFACE_H as f32),
            },
            padding: TaffyRect {
                left: ui_length(SIDE_MARGIN as f32),
                right: ui_length(SIDE_MARGIN as f32),
                top: ui_length(ISLAND_TOP as f32),
                bottom: ui_length(BOTTOM_GAP as f32),
            },
            ..Default::default()
        },
        vec![Box::new(bar) as Box<dyn Widget>],
    ))
}

// ── collect_click_zones ─────────────────────────────────────────────────────
