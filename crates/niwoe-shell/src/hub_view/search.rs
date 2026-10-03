pub(crate) fn search_rows(height: u32) -> usize {
    ((height as i32 - H.header_height - H.outer_pad) / H.search_row_height).max(1) as usize
}
pub(crate) fn hit_search(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    selected: usize,
    count: usize,
) -> Option<usize> {
    if x < H.outer_pad || x >= width as i32 - H.outer_pad || y < H.header_height {
        return None;
    }
    let rows = search_rows(height);
    let row = ((y - H.header_height) / H.search_row_height) as usize;
    let index = selected / rows * rows + row;
    (row < rows && index < count).then_some(index)
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_search(
    canvas: &mut [u8],
    width: u32,
    height: u32,
    query: &str,
    rows: &[crate::hub_state::ResultRow],
    selected: usize,
    apps: &[crate::launcher::DesktopApp],
    windows: &[crate::wayland::WindowInfo],
    icons: &crate::icons::IconCache,
    config: &niwoe_config::ThemeConfig,
) {
    let Some(mut image) = Pixmap::new(width, height) else {
        return;
    };
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let tint = config.glass_tint_color();
    image.fill(tiny_skia::Color::from_rgba8(
        tint.r,
        tint.g,
        tint.b,
        config
            .decorations
            .shell_surface_fill_alpha(niwoe_config::ThemeSurface::Launcher),
    ));
    let mut pm = image.as_mut();
    draw_header(&mut pm, width, 0, false, false, config);
    let caption = Typography::DEFAULT.caption_size as f32;
    let query_rect = Rect {
        x: H.outer_pad,
        y: H.header_height - Controls::MIN_HEIGHT - S.sm,
        width: inner_width(width),
        height: Controls::MIN_HEIGHT,
    };
    fill(&mut pm, query_rect, p.surface, Radius::DEFAULT.sm);
    paint_text_left_centered(
        &mut pm,
        &truncate_to_fit(
            &format!("Suche: {query} · {} Treffer", rows.len()),
            query_rect.width - S.md,
            caption,
        ),
        query_rect.x + S.sm,
        query_rect,
        caption,
        p.text,
    );
    let page_size = search_rows(height);
    let start = selected / page_size * page_size;
    for (slot, row) in rows.iter().skip(start).take(page_size).enumerate() {
        let r = Rect {
            x: H.outer_pad,
            y: H.header_height + slot as i32 * H.search_row_height,
            width: inner_width(width),
            height: H.search_row_height - S.xs,
        };
        fill(
            &mut pm,
            r,
            if start + slot == selected {
                niwoe_tokens::Interaction::DEFAULT.neutral_hover
            } else {
                alpha(p.surface, H.card_alpha)
            },
            Radius::DEFAULT.sm,
        );
        if start + slot == selected {
            niwoe_ui::effect::paint_focus(&mut pm, r, p.border_focus(), Radius::DEFAULT.sm);
        }
        draw_search_icon(&mut pm, row, r, apps, windows, icons, p.text_dim);
        let text_x = r.x + S.lg + H.app_icon_size + S.md;
        niwoe_ui::effect::paint_text_pair(
            &mut pm,
            Rect {
                x: text_x,
                width: r.x + r.width - S.lg - text_x,
                ..r
            },
            &row.title,
            &row.detail,
            p.text,
            p.text_dim,
        );
    }
    if rows.is_empty() {
        paint_text(
            &mut pm,
            "Keine passenden Räume, Fenster oder Anwendungen",
            H.outer_pad,
            H.header_height + S.xl,
            caption,
            p.text_dim,
        );
    }
    for (rgba, bgra) in image
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .zip(canvas.as_chunks_mut::<4>().0)
    {
        bgra.copy_from_slice(&[rgba[2], rgba[1], rgba[0], rgba[3]]);
    }
}

fn draw_search_icon(
    pm: &mut tiny_skia::PixmapMut<'_>,
    row: &crate::hub_state::ResultRow,
    rect: Rect,
    apps: &[crate::launcher::DesktopApp],
    windows: &[crate::wayland::WindowInfo],
    icons: &crate::icons::IconCache,
    color: Color,
) {
    use crate::hub_state::Target;
    use niwoe_ui::effect::{symbol_icon, Symbol};
    let app_id = match &row.target {
        Target::App(id) => Some(id.as_str()),
        Target::Window(id) => windows
            .iter()
            .find(|w| &w.id == id)
            .and_then(|w| w.app_id.as_deref()),
        Target::Room(_) => None,
    };
    let name = app_id
        .and_then(|id| {
            apps.iter()
                .find(|a| a.desktop_id == id || a.startup_wm_class.as_deref() == Some(id))
        })
        .and_then(|a| a.icon_name.as_deref());
    let x = rect.x + S.lg;
    let y = rect.y + (rect.height - H.app_icon_size) / 2;
    if let Some(image) = name
        .and_then(|name| icons.lookup(name, H.app_icon_size as u32))
        .and_then(crate::icons::icon_image_to_pixmap)
    {
        pm.draw_pixmap(
            x,
            y,
            image.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
    } else {
        let symbol = match row.target {
            Target::Room(_) => Symbol::Room,
            Target::Window(_) => Symbol::Window,
            Target::App(_) => Symbol::App,
        };
        if let Some(image) = symbol_icon(symbol, color, H.app_icon_size as u32) {
            pm.draw_pixmap(
                x,
                y,
                image.as_ref().as_ref(),
                &tiny_skia::PixmapPaint::default(),
                tiny_skia::Transform::identity(),
                None,
            );
        }
    }
}
