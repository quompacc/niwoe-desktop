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
            outline(&mut pm, r, p.accent, Controls::FOCUS_WIDTH);
        }
        paint_text(
            &mut pm,
            &truncate_to_fit(
                &row.title,
                r.width - S.lg,
                Typography::DEFAULT.body_size as f32,
            ),
            r.x + S.sm,
            r.y + S.lg,
            Typography::DEFAULT.body_size as f32,
            p.text,
        );
        paint_text(
            &mut pm,
            &truncate_to_fit(&row.detail, r.width - S.lg, caption),
            r.x + S.sm,
            r.y + S.lg + S.md,
            caption,
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
