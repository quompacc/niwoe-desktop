use super::*;
use crate::widget_action::{action_for_id, WidgetAction};

#[test]
fn audio_device_targets_preserve_snapshot_indices_and_defaults_are_inert() {
    for width in [320, 1024] {
        for is_default in [false, true] {
            let device = AudioDevice {
                id: 987,
                name: "Langer Mikrofon- und Lautsprechername ".repeat(8),
                volume_percent: None,
                muted: true,
                is_default,
            };
            for (ids, symbol, action) in [
                (
                    AUDIO_OUTPUT_IDS,
                    niwoe_ui::effect::Symbol::Speaker,
                    WidgetAction::SetDefaultAudioOutput(3),
                ),
                (
                    AUDIO_INPUT_IDS,
                    niwoe_ui::effect::Symbol::Microphone,
                    WidgetAction::SetDefaultAudioInput(3),
                ),
            ] {
                let root = audio_device_row(
                    &device,
                    ids.get(3).copied(),
                    symbol,
                    width,
                    &niwoe_ui::style::Palette::DARK,
                );
                let viewport = niwoe_ui::PixelSize {
                    width: width as u32,
                    height: niwoe_tokens::Controls::TEXT_PAIR_HEIGHT as u32,
                };
                let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
                let targets =
                    crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
                assert_eq!(targets.len(), usize::from(!is_default));
                if !is_default {
                    assert_eq!(action_for_id(targets[0].0), Some(action));
                }
            }
        }
    }
}

#[test]
fn network_rows_preserve_provider_identity_and_connected_rows_are_inert() {
    let palette = niwoe_ui::style::Palette::DARK;
    for width in [320, 1024] {
        for connected in [false, true] {
            let name = "Langer Geräte- und Netzwerkname mit Umlauten ".repeat(8);
            let profile = ConnectionProfile {
                name: name.clone(),
                type_label: "Ethernet".into(),
                active: connected,
            };
            let wifi = WifiNetwork {
                ssid: name.clone(),
                signal: 87,
                secured: true,
                in_use: connected,
            };
            let bluetooth = crate::bluetooth::BluetoothDevice {
                name,
                address: "AA:BB:CC:DD:EE:FF".into(),
                paired: true,
                connected,
            };
            for (row, action) in [
                (
                    network_profile_row(3, &profile, width, &palette),
                    WidgetAction::ActivateConnection(3),
                ),
                (
                    wifi_row(3, &wifi, width, &palette),
                    WidgetAction::WifiConnect(3),
                ),
                (
                    bluetooth_device_row(3, &bluetooth, width, &palette),
                    WidgetAction::BluetoothDevice(3),
                ),
            ] {
                let viewport = niwoe_ui::PixelSize {
                    width: width as u32,
                    height: niwoe_tokens::Controls::TEXT_PAIR_HEIGHT as u32,
                };
                let layout = niwoe_ui::compute_layout(row.as_ref(), viewport).unwrap();
                let targets =
                    crate::widget_traversal::focus_targets(row.as_ref(), &layout, viewport);
                assert_eq!(targets.len(), usize::from(!connected));
                if !connected {
                    let (id, area) = targets[0];
                    assert_eq!(action_for_id(id), Some(action));
                    let path = niwoe_ui::hit_test(
                        &layout,
                        niwoe_ui::PointerPosition {
                            x: area.x + area.width / 2,
                            y: area.y + area.height / 2,
                        },
                    )
                    .unwrap();
                    assert_eq!(
                        crate::widget_traversal::find_widget_at_path(row.as_ref(), &path)
                            .and_then(|w| w.id()),
                        Some(id)
                    );
                }
            }
        }
    }
}

#[test]
fn bluetooth_discovery_is_only_startable_when_powered_and_not_scanning() {
    for powered in [false, true] {
        for scanning in [false, true] {
            let root = bluetooth_adapter_controls(&BluetoothSnapshot {
                adapter_present: true,
                powered,
                scanning,
                device_status_available: true,
                devices: Vec::new(),
            });
            let viewport = niwoe_ui::PixelSize {
                width: 400,
                height: niwoe_tokens::Controls::MIN_HEIGHT as u32,
            };
            let layout = niwoe_ui::compute_layout(root.as_ref(), viewport).unwrap();
            let targets = crate::widget_traversal::focus_targets(root.as_ref(), &layout, viewport);
            assert!(targets.iter().any(|(id, _)| *id == "bt-power-toggle"));
            assert_eq!(
                targets.iter().any(|(id, _)| *id == "bt-scan-toggle"),
                powered && !scanning
            );
        }
    }
}
