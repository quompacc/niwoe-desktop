fn bluetooth_adapter_controls(snapshot: &BluetoothSnapshot) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let control_w = c.device_control_width;
    Box::new(Container::row(
        c.option_gap,
        vec![
            Box::new(SettingsControl {
                id: Some("bt-power-toggle"),
                kind: niwoe_ui::widget::ComponentKind::Chip,
                disabled: false,
                label: if snapshot.powered {
                    "Bluetooth: an"
                } else {
                    "Bluetooth: aus"
                }
                .into(),
                selected: snapshot.powered,
                width: control_w,
            }) as Box<dyn Widget>,
            Box::new(SettingsControl {
                id: Some("bt-scan-toggle"),
                kind: niwoe_ui::widget::ComponentKind::Button,
                disabled: !snapshot.powered || snapshot.scanning,
                label: if snapshot.scanning {
                    "Suche läuft"
                } else {
                    "Geräte suchen"
                }
                .into(),
                selected: false,
                width: control_w,
            }) as Box<dyn Widget>,
        ],
    ))
}

// Prepare text and cached functional artwork once when building the tree.
// IDs continue to refer to the original provider snapshot, including inert
// connected rows; no sorting/filtering changes the dispatch index.
fn network_profile_row(
    index: usize,
    profile: &ConnectionProfile,
    width: i32,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    use niwoe_ui::effect::Symbol;
    let kind = if profile.type_label == "Bridge" {
        "Netzwerkbrücke"
    } else {
        &profile.type_label
    };
    let status = if profile.active {
        "Verbunden"
    } else {
        "Zum Verbinden auswählen"
    };
    settings_text_row(
        profile.name.as_str(),
        format!("{kind} · {status}"),
        width,
        (!profile.active)
            .then(|| NETWORK_PROFILE_IDS.get(index).copied())
            .flatten(),
        profile.active,
        Some(match profile.type_label.as_str() {
            "WLAN" => Symbol::Network,
            "VPN" => Symbol::Lock,
            _ => Symbol::NetworkWired,
        }),
        profile.active.then_some(Symbol::Check),
        pal,
    )
}

fn wifi_row(
    index: usize,
    network: &WifiNetwork,
    width: i32,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    use niwoe_ui::effect::Symbol;
    let security = if network.secured {
        "Gesichert"
    } else {
        "Offenes WLAN"
    };
    let status = if network.in_use {
        "Verbunden"
    } else {
        "Zum Verbinden auswählen"
    };
    settings_text_row(
        network.ssid.as_str(),
        format!("{security} · Signal {} % · {status}", network.signal),
        width,
        (!network.in_use)
            .then(|| WIFI_NETWORK_IDS.get(index).copied())
            .flatten(),
        network.in_use,
        Some(if network.secured {
            Symbol::Lock
        } else {
            Symbol::Network
        }),
        network.in_use.then_some(Symbol::Check),
        pal,
    )
}

fn bluetooth_device_row(
    index: usize,
    device: &crate::bluetooth::BluetoothDevice,
    width: i32,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    use niwoe_ui::effect::Symbol;
    let title = if device.name.is_empty() {
        device.address.as_str()
    } else {
        device.name.as_str()
    };
    let status = if device.connected {
        "Verbunden"
    } else if device.paired {
        "Gekoppelt · Zum Verbinden auswählen"
    } else {
        "Nicht gekoppelt · Zum Koppeln auswählen"
    };
    settings_text_row(
        title,
        status,
        width,
        (!device.connected)
            .then(|| BT_DEVICE_IDS.get(index).copied())
            .flatten(),
        device.connected,
        Some(Symbol::Bluetooth),
        device.connected.then_some(Symbol::Check),
        pal,
    )
}
