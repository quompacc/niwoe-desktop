pub(crate) fn sidebar_row_height(height: u32) -> i32 {
    ((height as i32 - C.config_footer_height - C.outer_pad - S.xxl * 3 - C.sidebar_section_gap)
        / 11)
        .clamp(C.config_field_height, C.sidebar_item_height)
}

fn sidebar_item(
    pm: &mut tiny_skia::PixmapMut<'_>,
    y: i32,
    label: &str,
    active: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let rect = Rect {
        x: C.outer_pad / 2,
        y,
        width: C.sidebar_width - C.outer_pad,
        height: sidebar_row_height(pm.height()),
    };
    if active {
        fill(
            pm,
            rect,
            Interaction::DEFAULT.selection(p.surface, p.accent, Interaction::SELECTION_ACTIVE),
            Radius::DEFAULT.sm,
        );
        fill(
            pm,
            Rect {
                x: rect.x,
                y: rect.y,
                width: Controls::FOCUS_WIDTH,
                height: rect.height,
            },
            p.accent,
            Radius::DEFAULT.none,
        );
    }
    paint_text_left_centered(
        pm,
        label,
        rect.x + S.xl,
        rect,
        Typography::DEFAULT.body_size as f32,
        if active { p.text } else { p.text_dim },
    );
}

fn draw_sidebar(
    pm: &mut tiny_skia::PixmapMut<'_>,
    height: u32,
    configuring: bool,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(
        pm,
        Rect {
            x: 0,
            y: 0,
            width: C.sidebar_width,
            height: height as i32,
        },
        alpha(p.background, C.sidebar_alpha),
        Radius::DEFAULT.none,
    );
    paint_text(
        pm,
        "CONTROL CENTER",
        C.outer_pad,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let mut y = C.outer_pad + S.xxl * 2;
    for (label, active) in [
        ("Übersicht", false),
        ("Räume", true),
        ("Apps", false),
        ("Dateien · im Raum", false),
        ("Benutzer", false),
        ("System", false),
        ("Leiste · F7", false),
    ] {
        sidebar_item(pm, y, label, active, config);
        y += sidebar_row_height(height);
    }
    y += C.sidebar_section_gap;
    paint_text(
        pm,
        "WARTUNG",
        C.outer_pad,
        y,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    y += S.xxl;
    for label in [
        "Updates",
        "Backups · nicht verfügbar",
        "Protokolle · nicht verfügbar",
        "Einstellungen",
    ] {
        sidebar_item(pm, y, label, false, config);
        y += sidebar_row_height(height);
    }
    let back = back_rect(height);
    outline(pm, back, p.border, Controls::BORDER);
    paint_text_centered(
        pm,
        if configuring {
            "‹  Zurück zu Räumen"
        } else {
            "‹  Zurück zur Übersicht"
        },
        back,
        Typography::DEFAULT.caption_size as f32,
        p.text,
    );
}
