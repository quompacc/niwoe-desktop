use niwoe_tokens::Launcher;

#[test]
fn settings_card_is_centered_with_independent_size() {
    assert_eq!(
        Launcher::DEFAULT.fitted_rect(1920, 1080),
        (520, 230, 880, 620)
    );
}

#[test]
fn card_fits_logical_outputs_at_supported_scales() {
    for (physical_w, physical_h) in [(1920u32, 1080u32), (1366, 768)] {
        for scale in [1.0_f64, 1.5, 2.0] {
            let w = (f64::from(physical_w) / scale).round() as u32;
            let h = (f64::from(physical_h) / scale).round() as u32;
            let (x, y, card_w, card_h) = Launcher::DEFAULT.fitted_rect(w, h);
            assert!(x >= 0 && y >= 0);
            assert!(x as u32 + card_w <= w);
            assert!(y as u32 + card_h <= h);
            assert!(card_w > 0 && card_h > 0);
        }
    }
    assert_eq!(Launcher::DEFAULT.fitted_rect(960, 540), (40, 0, 880, 540));
}

#[test]
fn transient_empty_configure_cannot_underflow_card_geometry() {
    assert_eq!(Launcher::DEFAULT.fitted_rect(0, 0), (0, 0, 1, 1));
    assert_eq!(Launcher::DEFAULT.fitted_rect(1, 1), (0, 0, 1, 1));
}

#[test]
fn hub_controls_stay_below_panel_on_short_hidpi_outputs() {
    for (w, h) in [(1920, 1080), (1366, 768), (1280, 720), (960, 540)] {
        let (_, y, _, card_h) = Launcher::HUB.fitted_rect(w, h);
        assert!(y >= niwoe_tokens::Panel::DEFAULT.height as i32);
        assert!(y as u32 + card_h <= h);
    }
    assert_eq!(Launcher::HUB.fitted_rect(1920, 1080), (260, 124, 1400, 832));
    assert_eq!(Launcher::HUB.fitted_rect(0, 0), (0, 0, 1, 1));
}
