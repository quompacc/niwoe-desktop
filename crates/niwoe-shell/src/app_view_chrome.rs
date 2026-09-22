use super::*;
use niwoe_ui::effect::measure_text;

pub(super) fn keycap(
    pm: &mut PixmapMut<'_>,
    label: &str,
    x: i32,
    y: i32,
    config: &niwoe_config::ThemeConfig,
) -> i32 {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let size = Typography::DEFAULT.caption_size as f32;
    let width = measure_text(label, size).0 + S.md * 2;
    let rect = Rect {
        x,
        y,
        width,
        height: S.xl,
    };
    fill(pm, rect, p.surface_alt, Radius::DEFAULT.sm);
    if let Some(path) = rounded_rect_path(rect, Radius::DEFAULT.sm) {
        paint_border(pm, &path, p.border, Controls::BORDER as f32);
    }
    paint_text(pm, label, x + S.md, y + S.lg, size, p.text_dim);
    width
}

pub(super) fn header(
    pm: &mut PixmapMut<'_>,
    width: u32,
    query: &str,
    count: usize,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let body = Typography::DEFAULT.title_size as f32;
    let caption = Typography::DEFAULT.caption_size as f32;
    let search = Rect {
        x: S.lg,
        y: S.md,
        width: width as i32 - S.lg * 2,
        height: L.search_height + S.lg,
    };
    keycap(
        pm,
        "Esc",
        width as i32 - S.xl - S.xxl - S.md,
        search.y + S.lg,
        config,
    );
    // Magnifier built from the shared spacing geometry, not an external icon theme.
    let cx = (search.x + S.xl) as f32;
    let cy = (search.y + search.height / 2 - S.xs) as f32;
    let mut path = tiny_skia::PathBuilder::new();
    path.push_circle(cx, cy, S.sm as f32);
    path.move_to(cx + S.xs as f32, cy + S.xs as f32);
    path.line_to(cx + S.md as f32, cy + S.md as f32);
    if let Some(path) = path.finish() {
        let mut paint = tiny_skia::Paint {
            anti_alias: true,
            ..Default::default()
        };
        paint.set_color_rgba8(p.text_dim.r, p.text_dim.g, p.text_dim.b, p.text_dim.a);
        pm.stroke_path(
            &path,
            &paint,
            &tiny_skia::Stroke {
                width: Controls::BORDER as f32,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );
    }
    let text = if query.is_empty() {
        "Anwendungen suchen …"
    } else {
        query
    };
    let text = truncate_to_fit(text, search.width - S.xxl * 4, body);
    paint_text(
        pm,
        &text,
        search.x + S.xxl + S.lg,
        search.y + (search.height + body as i32) / 2 - S.xs,
        body,
        if query.is_empty() { p.text_dim } else { p.text },
    );
    fill(
        pm,
        Rect {
            x: S.lg,
            y: search.y + search.height,
            width: search.width,
            height: Controls::BORDER,
        },
        p.border,
        0,
    );
    let summary = if query.is_empty() {
        "Installierte Anwendungen".to_string()
    } else {
        format!("{count} Treffer")
    };
    paint_text(
        pm,
        &summary,
        S.xl,
        L.header_height - S.md,
        caption,
        p.text_dim,
    );
    if query.is_empty() {
        let label = count.to_string();
        let w = measure_text(&label, caption).0;
        paint_text(
            pm,
            &label,
            width as i32 - S.xl - w,
            L.header_height - S.md,
            caption,
            p.text_dim,
        );
    }
}

pub(super) fn detail(app: &DesktopApp) -> &str {
    if app.terminal {
        return "Terminal";
    }
    for (category, label) in [
        ("Development", "Entwicklung"),
        ("Office", "Büro"),
        ("Network", "Internet"),
        ("Graphics", "Grafik"),
        ("AudioVideo", "Medien"),
        ("System", "System"),
        ("Utility", "Werkzeug"),
        ("Game", "Spiel"),
    ] {
        if app.categories.iter().any(|c| c == category) {
            return label;
        }
    }
    std::path::Path::new(&app.program)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&app.program)
}

pub(super) fn footer(
    pm: &mut PixmapMut<'_>,
    width: u32,
    y: i32,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(
        pm,
        Rect {
            x: 0,
            y,
            width: width as i32,
            height: L.footer_height,
        },
        p.background,
        0,
    );
    fill(
        pm,
        Rect {
            x: S.lg,
            y,
            width: width as i32 - S.lg * 2,
            height: Controls::BORDER,
        },
        p.border,
        0,
    );
    let mut x = S.xl;
    for (key, label) in [("↑ ↓", "Auswählen"), ("Enter", "Öffnen")] {
        x += keycap(pm, key, x, y + S.md, config) + S.md;
        paint_text(
            pm,
            label,
            x,
            y + S.xl,
            Typography::DEFAULT.caption_size as f32,
            p.text_dim,
        );
        x += measure_text(label, Typography::DEFAULT.caption_size as f32).0 + S.xl;
    }
}
