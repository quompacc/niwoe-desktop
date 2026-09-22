use niwoe_config::{ThemeConfig, ThemeSurface};
use niwoe_tokens::Palette;

#[test]
fn shipped_theme_files_match_canonical_palettes_and_shared_geometry() {
    let dark: ThemeConfig =
        toml::from_str(include_str!("../../../themes/dark/theme.toml")).unwrap();
    let light: ThemeConfig =
        toml::from_str(include_str!("../../../themes/light/theme.toml")).unwrap();
    for (config, p) in [(&dark, Palette::DARK), (&light, Palette::LIGHT)] {
        let c = &config.colors;
        assert_eq!(
            [
                c.background,
                c.surface,
                c.surface_alt,
                c.accent,
                c.accent_alt,
                c.text,
                c.text_dim,
                c.border,
                c.error,
                c.warning,
                c.success
            ],
            [
                p.background,
                p.surface,
                p.surface_alt,
                p.accent,
                p.accent_alt,
                p.text,
                p.text_dim,
                p.border,
                p.error,
                p.warning,
                p.success
            ]
        );
    }
    for surface in [
        ThemeSurface::Panel,
        ThemeSurface::Launcher,
        ThemeSurface::Popup,
        ThemeSurface::Modal,
        ThemeSurface::Control,
    ] {
        assert_eq!(
            dark.decorations.surface_treatment(surface),
            light.decorations.surface_treatment(surface)
        );
    }
    assert_eq!(dark.fonts.ui, light.fonts.ui);
    assert_eq!(dark.icons.theme, light.icons.theme);
}
