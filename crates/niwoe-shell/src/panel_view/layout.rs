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
    room_entries: &[niwoe_ipc::RoomEntry],
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
    // Budget rooms before optional app shortcuts. The active room is never
    // hidden; the room menu retains access to every slot on narrow outputs.
    // Equal side tracks pin the clock to the output midpoint, not the midpoint
    // of whatever free space happens to remain between the two clusters.
    let side_width = ((width as i32 - 2 * SIDE_MARGIN - LEFT_PADDING - RIGHT_PADDING - CLOCK_W)
        .max(0) as f32)
        / 2.0;
    let status_width =
        TRAY_W * if battery.present { 3 } else { 2 } + if battery.present { LAUNCHER_W } else { 0 };
    let tray_capacity = ((side_width as i32
        - status_width
        - SCREENSHOT_W * 2
        - DIVIDER_W
        - GAP * 4)
        .max(0)
        / (SNI_W + GAP)) as usize;
    let tray_count = status_notifier_items
        .len()
        .min(SNI_PANEL_IDS.len())
        .min(tray_capacity);
    let room_stride = PanelTokens::DEFAULT.room_width as i32 + GAP;
    let room_space_without_overflow = (side_width as i32 - LAUNCHER_W - GAP).max(0);
    let capacity_without_overflow = (room_space_without_overflow / room_stride).max(1) as usize;
    let total_rooms = total_workspaces.clamp(1, niwoe_config::rooms::MAX_ROOMS as u8) as usize;
    let capacity = if total_rooms <= capacity_without_overflow {
        total_rooms
    } else {
        let room_space = (side_width as i32 - LAUNCHER_W - WS_W - GAP * 2).max(0);
        (room_space / room_stride).max(1) as usize
    }
    .min(PanelTokens::DEFAULT.visible_rooms)
    .min(total_rooms);
    let active_position = room_entries
        .iter()
        .position(|r| r.workspace == active_workspace)
        .map(|position| position as u8 + 1)
        .unwrap_or(active_workspace);
    let rooms = visible_rooms(total_workspaces, capacity);
    for position in rooms {
        let room = room_entries.get(position as usize-1);
        let workspace = room.map(|r| r.workspace).unwrap_or(position);
        left_children.push(Box::new(RoomTab {
            workspace,
            label: room.map(|r| r.name.clone()).unwrap_or_else(|| format!("Raum {workspace}")),
            active: workspace == active_workspace,
        }));
    }
    let hidden_count = total_rooms.saturating_sub(capacity) as u8;
    if hidden_count > 0 {
        left_children.push(Box::new(PanelWorkspaceChip {
            hidden_count,
            active_hidden: active_position as usize > capacity,
        }));
    }
    let left_cluster = Container::new(
        WidgetStyle {
            size: UiSize {
                width: ui_length(side_width),
                height: ui_length(CHIP_H as f32),
            },
            flex_shrink: 0.0,
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

    let center_children: Vec<Box<dyn Widget>> = vec![Box::new(PanelClockChip {
        value: clock.into(),
    })];
    let center_cluster = Container::new(
        WidgetStyle {
            flex_direction: FlexDirection::Row,
            size: UiSize {
                width: ui_length(CLOCK_W as f32),
                height: ui_length(CHIP_H as f32),
            },
            flex_shrink: 0.0,
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
    for (idx, item) in status_notifier_items.iter().take(tray_count).enumerate() {
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
    right_children.push(Box::new(PanelChip::new(
        "panel-search",
        "Suche".into(),
        icon(Symbol::Search, theme.palette.text),
        SCREENSHOT_W,
        false,
    )));

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
    let right_cluster = Container::new(
        WidgetStyle {
            size: UiSize {
                width: ui_length(side_width),
                height: ui_length(CHIP_H as f32),
            },
            flex_shrink: 0.0,
            justify_content: Some(JustifyContent::FlexEnd),
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
