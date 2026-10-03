use niwoe_ui::{
    widget::{Component, ComponentKind, ComponentState},
    Rect, Theme, Widget, WidgetState,
};
use tiny_skia::Pixmap;

fn render(theme: Theme, state: ComponentState, pointer: WidgetState) -> Vec<u8> {
    let mut image = Pixmap::new(180, 40).unwrap();
    let mut widget = Component::new(ComponentKind::Input, "Änderungen", 180);
    widget.state = state;
    widget.paint(
        Rect {
            x: 0,
            y: 0,
            width: 180,
            height: 40,
        },
        &mut image.as_mut(),
        &theme,
        pointer,
    );
    image.data().to_vec()
}

#[test]
fn focus_is_additional_geometry_and_disabled_ignores_pointer() {
    for theme in [Theme::DARK, Theme::LIGHT] {
        let idle = render(theme, ComponentState::default(), WidgetState::Idle);
        let focus = render(
            theme,
            ComponentState {
                focused: true,
                ..Default::default()
            },
            WidgetState::Idle,
        );
        assert_ne!(idle, focus);
        let disabled = ComponentState {
            disabled: true,
            ..Default::default()
        };
        assert_eq!(
            render(theme, disabled, WidgetState::Idle),
            render(theme, disabled, WidgetState::Pressed)
        );
        assert_eq!(
            render(theme, disabled, WidgetState::Idle),
            render(theme, disabled, WidgetState::Hovered)
        );
        let mut widget = Component::new(ComponentKind::Button, "Disabled", 180);
        widget.state = disabled;
        assert!(!widget.accepts_input());
    }
}

#[test]
fn themes_share_geometry_and_alpha_coverage() {
    assert_eq!(Theme::DARK.spacing, Theme::LIGHT.spacing);
    assert_eq!(Theme::DARK.radius, Theme::LIGHT.radius);
    let dark = render(Theme::DARK, ComponentState::default(), WidgetState::Idle);
    let light = render(Theme::LIGHT, ComponentState::default(), WidgetState::Idle);
    assert_eq!(
        dark.as_chunks::<4>()
            .0
            .iter()
            .map(|p| p[3])
            .collect::<Vec<_>>(),
        light
            .as_chunks::<4>()
            .0
            .iter()
            .map(|p| p[3])
            .collect::<Vec<_>>()
    );
}

#[test]
fn embedded_sans_covers_interface_glyphs_and_identifies_missing_glyphs() {
    let font = niwoe_ui::ui_font();
    for ch in "ÄÖÜ äöü ß € – … ← → ✓"
        .chars()
        .filter(|c| !c.is_whitespace())
    {
        assert_ne!(font.lookup_glyph_index(ch), 0, "missing {ch}");
    }
    assert_eq!(font.lookup_glyph_index('\u{10ffff}'), 0);
}

#[test]
fn font_loading_rejects_missing_glyphs_and_invalid_bytes() {
    let bytes = niwoe_tokens::font::ADWAITA_SANS_REGULAR;
    assert!(niwoe_ui::font_supports_text(bytes, "ÄÖÜ äöü ß € – … ← → ✓"));
    assert!(!niwoe_ui::font_supports_text(bytes, "\u{10ffff}"));
    assert!(!niwoe_ui::font_supports_text(b"not a font", "Text"));
}

#[test]
fn selection_is_neutral_and_keyboard_focus_remains_distinct_at_each_scale() {
    for scale in [1.0_f32, 1.5, 2.0] {
        for kind in [
            ComponentKind::Button,
            ComponentKind::Tab,
            ComponentKind::Chip,
            ComponentKind::Card,
        ] {
            let mut component = Component::new(kind, "Auswahl", 180);
            component.state.selected = true;
            let mut image = Pixmap::new((180.0 * scale) as u32, (48.0 * scale) as u32).unwrap();
            let area = Rect {
                x: 0,
                y: 0,
                width: 180,
                height: 48,
            };
            for pointer in [
                WidgetState::Idle,
                WidgetState::Hovered,
                WidgetState::Pressed,
            ] {
                image.fill(tiny_skia::Color::TRANSPARENT);
                component.paint_scaled(area, &mut image.as_mut(), &Theme::DARK, pointer, scale);
                let accent = Theme::DARK.palette.accent;
                assert!(
                    !image.pixels().iter().any(|pixel| {
                        (pixel.red(), pixel.green(), pixel.blue(), pixel.alpha())
                            == (accent.r, accent.g, accent.b, accent.a)
                    }),
                    "{kind:?} has an accent stripe/fill during {pointer:?} at {scale}"
                );
            }
            let selected = image.data().to_vec();
            component.state.focused = true;
            component.paint_scaled(
                area,
                &mut image.as_mut(),
                &Theme::DARK,
                WidgetState::Pressed,
                scale,
            );
            assert_ne!(selected, image.data(), "keyboard focus must remain visible");
        }
    }
}
