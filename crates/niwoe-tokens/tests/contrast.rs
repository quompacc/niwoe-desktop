//! Measurements use sRGB relative luminance after alpha compositing.
use niwoe_tokens::{Color, Interaction, Palette};

fn luminance(c: Color) -> f64 {
    let linear = |v: u8| {
        let s = v as f64 / 255.0;
        if s <= 0.04045 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(c.r) + 0.7152 * linear(c.g) + 0.0722 * linear(c.b)
}
fn ratio(a: Color, b: Color) -> f64 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}
fn composite(fg: Color, bg: Color) -> Color {
    composite_alpha(fg, bg, niwoe_tokens::chrome::SURFACE_ALPHA)
}
fn composite_alpha(fg: Color, bg: Color, alpha: u8) -> Color {
    let a = alpha as f64 / 255.0;
    let blend = |x: u8, y: u8| (x as f64 * a + y as f64 * (1.0 - a)).round() as u8;
    Color::rgb(blend(fg.r, bg.r), blend(fg.g, bg.g), blend(fg.b, bg.b))
}

#[test]
fn panel_glass_text_and_active_border_survive_extreme_backdrops() {
    let panel = niwoe_tokens::Panel::DEFAULT;
    for p in [Palette::DARK, Palette::LIGHT] {
        for wallpaper in [Color::rgb(0, 0, 0), Color::rgb(255, 255, 255)] {
            // Popup shader tint followed by the pane's alpha blend.
            let base = composite(composite(p.surface_alt, wallpaper), wallpaper);
            for overlay in [
                Color::rgba(0, 0, 0, 0),
                Color::rgba(
                    p.surface_alt.r,
                    p.surface_alt.g,
                    p.surface_alt.b,
                    panel.hover_alpha,
                ),
                Color::rgba(
                    p.surface_alt.r,
                    p.surface_alt.g,
                    p.surface_alt.b,
                    panel.pressed_alpha,
                ),
            ] {
                let control = composite_alpha(overlay, base, overlay.a);
                let active = composite_alpha(p.accent, control, panel.active_alpha);
                assert!(ratio(p.text, control) >= 4.5);
                assert!(ratio(p.text, active) >= 4.5);
                assert!(
                    ratio(p.accent, active) >= 3.0,
                    "active border contrast: {}",
                    ratio(p.accent, active)
                );
            }
        }
    }
}

#[test]
fn composed_text_status_controls_and_focus_meet_contrast_targets() {
    let mut failures = Vec::new();
    for (name, p) in [("dark", Palette::DARK), ("light", Palette::LIGHT)] {
        for (role, fg, target) in [
            ("primary", p.text, 4.5),
            ("secondary", p.text_dim, 4.5),
            ("disabled", p.text_disabled(), 4.5),
            ("success", p.success, 4.5),
            ("warning", p.warning, 4.5),
            ("error", p.error, 4.5),
            ("control", p.border_control(), 3.0),
            ("focus", p.border_focus(), 3.0),
        ] {
            let mut minimum = f64::MAX;
            for surface in [p.background, p.surface, p.surface_alt] {
                for bg in [
                    surface,
                    Interaction::DEFAULT.hover(surface),
                    Interaction::DEFAULT.pressed(surface),
                ] {
                    for wallpaper in [Color::rgb(0, 0, 0), Color::rgb(255, 255, 255)] {
                        minimum = minimum.min(ratio(fg, composite(bg, wallpaper)));
                    }
                }
            }
            println!("{name},{role},{minimum:.3},{target}");
            if minimum < target {
                failures.push(format!("{name}/{role}: {minimum:.3} < {target}"));
            }
        }
        for accent in [
            p.accent,
            Interaction::DEFAULT.hover(p.accent),
            Interaction::DEFAULT.pressed(p.accent),
        ] {
            let value = ratio(niwoe_tokens::contrast_text(accent), accent);
            println!("{name},on_accent,{value:.3},4.5");
            assert!(value >= 4.5);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
