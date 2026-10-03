fn draw_sidebar(
    pm: &mut tiny_skia::PixmapMut<'_>,
    height: u32,
    active: crate::control_center::Page,
    configuring: bool,
    config: &niwoe_config::ThemeConfig,
) {
    crate::control_center::draw_sidebar(
        pm,
        height,
        active,
        configuring,
        &crate::ui::tokens::theme_from_config(config),
    );
}
