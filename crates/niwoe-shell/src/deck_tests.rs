#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn populated_deck_targets_do_not_overlap_and_follow_visual_order_in_both_themes() {
        use crate::audio::{AudioDevice, AudioServiceState};
        let q = QuickSettings::DEFAULT;
        let font = RefCell::new(TextRenderer::new(
            "sans",
            niwoe_tokens::Typography::DEFAULT.body_size.into(),
        ));
        let mut audio = AudioSnapshot::unavailable();
        audio.service = AudioServiceState::Running;
        audio.default_output = Some(AudioDevice {
            id: 1,
            name: "Ein sehr langer Name für einen angeschlossenen Audioausgang".into(),
            volume_percent: Some(70),
            muted: false,
            is_default: true,
        });
        let mut previous_targets = None;
        for (name, p) in [
            ("dark", niwoe_tokens::Palette::DARK),
            ("light", niwoe_tokens::Palette::LIGHT),
        ] {
            let theme = ThemeConfig {
                colors: niwoe_config::ThemeColors {
                    background: p.background,
                    surface: p.surface,
                    surface_alt: p.surface_alt,
                    accent: p.accent,
                    accent_alt: p.accent_alt,
                    text: p.text,
                    text_dim: p.text_dim,
                    border: p.border,
                    error: p.error,
                    warning: p.warning,
                    success: p.success,
                },
                ..ThemeConfig::default()
            };
            let mut canvas = vec![0; (q.width * q.height * 4) as usize];
            reset_keyboard_focus();
            draw(
                &mut Painter::new(&mut canvas, q.width, q.height),
                &font,
                &theme,
                QuickSettingsState {
                    network: &NetworkState::Connected {
                        kind: ConnectionKind::Wifi { signal: Some(80) },
                        connection_name: "Ein sehr langer drahtloser Netzwerkname".into(),
                    },
                    audio: &audio,
                    bluetooth: &crate::bluetooth::BluetoothSnapshot {
                        adapter_present: true,
                        powered: true,
                        ..Default::default()
                    },
                    bluetooth_pending: false,
                    volume_preview: None,
                    audio_status: crate::deck_mutation::Status::Idle,
                    power_status: crate::deck_mutation::Status::Idle,
                    power_profile: Some(PowerProfile::Standard),
                    room_name: "Raum 2",
                    power_armed: false,
                    logout_armed: false,
                },
            );
            let targets = focus_targets();
            assert_eq!(targets.len(), 11);
            assert_eq!(targets.last().unwrap().1, QuickSettingsHit::Settings);
            for (i, (rect, _)) in targets.iter().enumerate() {
                assert!(rect.x >= 0 && rect.y >= 0);
                assert!(rect.x + rect.w <= q.width && rect.y + rect.h <= q.height);
                for (other, _) in &targets[i + 1..] {
                    assert!(
                        rect.x + rect.w <= other.x
                            || other.x + other.w <= rect.x
                            || rect.y + rect.h <= other.y
                            || other.y + other.h <= rect.y
                    );
                    assert!((rect.y, rect.x) < (other.y, other.x));
                }
            }
            let geometry: Vec<_> = targets
                .iter()
                .map(|(r, action)| (r.x, r.y, r.w, r.h, *action))
                .collect();
            if let Some(previous) = previous_targets.as_ref() {
                assert_eq!(previous, &geometry);
            }
            previous_targets = Some(geometry);
            if let Ok(prefix) = std::env::var("NIWOE_DECK_POPULATED_PREVIEW") {
                save_preview(canvas, &theme, &format!("{prefix}-{name}.png"));
            }
        }
        reset_keyboard_focus();
    }

    #[test]
    fn unavailable_controls_are_not_clickable_or_keyboard_targets() {
        let q = QuickSettings::DEFAULT;
        let mut canvas = vec![0; (q.width * q.height * 4) as usize];
        let font = RefCell::new(TextRenderer::new(
            "sans",
            niwoe_tokens::Typography::DEFAULT.body_size.into(),
        ));
        let theme = ThemeConfig::default();
        reset_keyboard_focus();
        draw(
            &mut Painter::new(&mut canvas, q.width, q.height),
            &font,
            &theme,
            QuickSettingsState {
                network: &NetworkState::Offline,
                audio: &AudioSnapshot::unavailable(),
                bluetooth: &crate::bluetooth::BluetoothSnapshot::default(),
                bluetooth_pending: false,
                volume_preview: None,
                audio_status: crate::deck_mutation::Status::Idle,
                power_status: crate::deck_mutation::Status::Idle,
                power_profile: None,
                room_name: "Loge",
                power_armed: false,
                logout_armed: false,
            },
        );
        let targets = focus_targets();
        assert!(!targets.iter().any(|(_, action)| matches!(
            action,
            QuickSettingsHit::AudioMute
                | QuickSettingsHit::Volume(_)
                | QuickSettingsHit::PowerProfile
                | QuickSettingsHit::Network
                | QuickSettingsHit::Bluetooth
        )));
        for (rect, action) in &targets {
            assert!(
                rect.x >= 0
                    && rect.y >= 0
                    && rect.x + rect.w <= q.width
                    && rect.y + rect.h <= q.height
            );
            assert_eq!(
                hit_test(
                    q.width as u32,
                    q.height as u32,
                    f64::from(rect.x + rect.w / 2),
                    f64::from(rect.y + rect.h / 2)
                ),
                Some(*action)
            );
        }
        for (_, action) in &targets {
            focus_next(false);
            assert_eq!(focused_action(), Some(*action));
        }
        focus_next(false);
        assert_eq!(focused_action(), targets.first().map(|(_, action)| *action));
        if let Ok(path) = std::env::var("NIWOE_DECK_PREVIEW") {
            save_preview(canvas, &theme, &path);
        }
        reset_keyboard_focus();
    }

    fn save_preview(mut canvas: Vec<u8>, theme: &ThemeConfig, path: &str) {
        let q = QuickSettings::DEFAULT;
        // The live compositor supplies the glass surface below this transparent layer.
        // Show the foreground against its theme tint in the standalone preview.
        let background = theme.glass_tint_color();
        for pixel in canvas.as_chunks_mut::<4>().0 {
            let inverse = 255 - u16::from(pixel[3]);
            for (channel, base) in
                pixel[..3]
                    .iter_mut()
                    .zip([background.b, background.g, background.r])
            {
                *channel = (u16::from(*channel) + u16::from(base) * inverse / 255) as u8;
            }
            pixel[3] = 255;
            pixel.swap(0, 2);
        }
        tiny_skia::Pixmap::from_vec(
            canvas,
            tiny_skia::IntSize::from_wh(q.width as u32, q.height as u32).unwrap(),
        )
        .unwrap()
        .save_png(path)
        .unwrap();
    }

    #[test]
    fn outside_is_not_a_hit() {
        assert_eq!(hit_test(384, 468, -1.0, 10.0), None);
    }

    #[test]
    fn bluetooth_target_requires_completed_refresh_and_adapter() {
        reset_keyboard_focus();
        let q = QuickSettings::DEFAULT;
        let font = RefCell::new(None);
        let theme = ThemeConfig::default();
        let mut canvas = vec![0; (q.width * q.height * 4) as usize];
        for (adapter_present, powered, pending, enabled) in [
            (true, true, false, true),
            (true, true, true, false),
            (false, false, false, false),
            (true, false, false, true),
        ] {
            draw(
                &mut Painter::new(&mut canvas, q.width, q.height),
                &font,
                &theme,
                QuickSettingsState {
                    network: &NetworkState::Offline,
                    audio: &AudioSnapshot::unavailable(),
                    bluetooth: &crate::bluetooth::BluetoothSnapshot {
                        adapter_present,
                        powered,
                        ..Default::default()
                    },
                    bluetooth_pending: pending,
                    volume_preview: None,
                    audio_status: crate::deck_mutation::Status::Idle,
                    power_status: crate::deck_mutation::Status::Idle,
                    power_profile: None,
                    room_name: "Raum 1",
                    power_armed: false,
                    logout_armed: false,
                },
            );
            assert_eq!(
                focus_targets()
                    .iter()
                    .any(|(_, action)| *action == QuickSettingsHit::Bluetooth),
                enabled
            );
            let x = q.outer_pad + (q.width - q.outer_pad * 2) / 3;
            let y = q.header_height + q.audio_height + q.section_gap * 2;
            assert_eq!(
                hit_test(q.width as u32, q.height as u32, x.into(), y.into()),
                Some(if enabled {
                    QuickSettingsHit::Bluetooth
                } else {
                    QuickSettingsHit::Card
                })
            );
            // A refresh must preserve action identity when earlier targets change.
            if KEYBOARD_FOCUS.with(Cell::get).is_some() {
                assert_eq!(focused_action(), Some(QuickSettingsHit::Display));
            }
            for _ in 0..focus_targets().len() {
                if focused_action() == Some(QuickSettingsHit::Display) {
                    break;
                }
                focus_next(false);
            }
            assert_eq!(focused_action(), Some(QuickSettingsHit::Display));
        }
        reset_keyboard_focus();
    }

    #[test]
    fn pending_controls_block_duplicate_actions_but_allow_latest_volume_and_retry() {
        use crate::deck_mutation::Status;
        let q = QuickSettings::DEFAULT;
        let font = RefCell::new(TextRenderer::new(
            "sans",
            niwoe_tokens::Typography::DEFAULT.body_size.into(),
        ));
        let mut audio = AudioSnapshot::unavailable();
        audio.default_output = Some(crate::audio::AudioDevice {
            id: 1,
            name: "Test".into(),
            volume_percent: Some(30),
            muted: false,
            is_default: true,
        });
        for status in [Status::Pending, Status::Failed, Status::Idle] {
            let mut canvas = vec![0; (q.width * q.height * 4) as usize];
            draw(
                &mut Painter::new(&mut canvas, q.width, q.height),
                &font,
                &ThemeConfig::default(),
                QuickSettingsState {
                    network: &NetworkState::Offline,
                    audio: &audio,
                    bluetooth: &Default::default(),
                    bluetooth_pending: false,
                    volume_preview: Some(80),
                    audio_status: status,
                    power_status: status,
                    power_profile: Some(PowerProfile::Standard),
                    room_name: "Raum 1",
                    power_armed: false,
                    logout_armed: false,
                },
            );
            let targets = focus_targets();
            for action in [QuickSettingsHit::AudioMute, QuickSettingsHit::PowerProfile] {
                assert_eq!(
                    targets.iter().any(|(_, hit)| *hit == action),
                    status != Status::Pending
                );
            }
            assert!(targets
                .iter()
                .any(|(_, hit)| matches!(hit, QuickSettingsHit::Volume(_))));
            assert_eq!(
                audio.default_output.as_ref().unwrap().volume_percent,
                Some(30)
            );
        }
        reset_keyboard_focus();
    }

    #[test]
    fn slider_x_maps_and_clamps_to_percent() {
        let slider = Rect {
            x: 100,
            y: 0,
            w: 200,
            h: 28,
        };
        assert_eq!(volume_from_slider_x(slider, 100.0), 0);
        assert_eq!(volume_from_slider_x(slider, 200.0), 50);
        assert_eq!(volume_from_slider_x(slider, 300.0), 100);
        assert_eq!(volume_from_slider_x(slider, 40.0), 0);
        assert_eq!(volume_from_slider_x(slider, 500.0), 100);
    }
}
