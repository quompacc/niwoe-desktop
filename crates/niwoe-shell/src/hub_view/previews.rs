fn preview_rect(rect: Rect) -> Rect {
    Rect {
        x: rect.x + H.card_pad,
        y: rect.y + H.card_pad + H.room_icon_size + S.xl,
        width: rect.width - H.card_pad * 2,
        height: H.preview_height as i32,
    }
}
pub(crate) fn hit_preview(x: i32, y: i32, width: u32, slot: usize) -> bool {
    let r = preview_rect(room_rect(slot, width));
    x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
}
#[allow(clippy::too_many_arguments)]
fn draw_room_preview(
    pm: &mut tiny_skia::PixmapMut<'_>,
    rect: Rect,
    room: &RoomEntry,
    windows: &[WindowInfo],
    hub: &crate::hub_state::HubState,
    apps: &[crate::launcher::DesktopApp],
    icons: &crate::icons::IconCache,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let caption = Typography::DEFAULT.caption_size as f32;
    let description = truncate_to_fit(&room.description, rect.width - H.card_pad * 2, caption);
    paint_text(
        pm,
        &description,
        rect.x + H.card_pad,
        rect.y + H.card_pad + H.room_icon_size + S.md,
        caption,
        p.text_dim,
    );
    let slot = preview_rect(rect);
    fill(
        pm,
        slot,
        alpha(p.surface_alt, H.quiet_alpha),
        Radius::DEFAULT.sm,
    );
    let window = crate::hub_state::preview_window(room, windows);
    if let Some(image) = window.and_then(|w| hub.images.get(&w.id)) {
        let factor = (slot.width as f32 / image.width() as f32)
            .min(slot.height as f32 / image.height() as f32);
        let x = slot.x as f32 + (slot.width as f32 - image.width() as f32 * factor) / 2.0;
        let y = slot.y as f32 + (slot.height as f32 - image.height() as f32 * factor) / 2.0;
        pm.draw_pixmap(
            0,
            0,
            image.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::from_scale(factor, factor).post_translate(x, y),
            None,
        );
    } else {
        let label = if windows.iter().any(|w| w.workspace == room.workspace) {
            "Vorschau nicht verfügbar"
        } else {
            "Raum ist leer"
        };
        paint_text_centered(pm, label, slot, caption, p.text_dim);
    }
    let mut identities: Vec<&str> = windows
        .iter()
        .filter(|w| w.workspace == room.workspace)
        .filter_map(|w| w.app_id.as_deref())
        .collect();
    for app in &room.preferences.apps {
        let (niwoe_ipc::AppReference::Native(id) | niwoe_ipc::AppReference::Xwayland(id)) = app;
        identities.push(id);
    }
    let mut seen = std::collections::HashSet::new();
    identities.retain(|id| seen.insert(*id));
    for (index, id) in identities.iter().take(H.room_columns as usize).enumerate() {
        let icon_name = apps
            .iter()
            .find(|a| {
                a.desktop_id
                    .trim_end_matches(".desktop")
                    .eq_ignore_ascii_case(id)
                    || a.program
                        .rsplit('/')
                        .next()
                        .is_some_and(|p| p.eq_ignore_ascii_case(id))
            })
            .and_then(|a| a.icon_name.as_deref())
            .unwrap_or(id);
        let x = slot.x + index as i32 * (H.app_icon_size + S.sm);
        let y = slot.y + slot.height + S.sm;
        if let Some(image) = icons
            .lookup(icon_name, H.app_icon_size as u32)
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
            if let Some(image) = niwoe_ui::effect::symbol_icon(
                niwoe_ui::effect::Symbol::App,
                p.text_dim,
                H.app_icon_size as u32,
            ) {
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
}
fn page_rect(width: u32, back: bool) -> Rect {
    Rect {
        x: width as i32
            - H.outer_pad
            - if back {
                H.close_width * 2 + S.sm
            } else {
                H.close_width
            },
        y: H.header_height - Controls::MIN_HEIGHT - S.sm,
        width: H.close_width,
        height: Controls::MIN_HEIGHT,
    }
}
pub(crate) fn hit_page(x: i32, y: i32, width: u32) -> Option<bool> {
    [true, false].into_iter().find(|&back| {
        let r = page_rect(width, back);
        x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
    })
}
fn draw_pages(
    pm: &mut tiny_skia::PixmapMut<'_>,
    width: u32,
    page: usize,
    count: usize,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    for (back, label) in [(true, "‹ Zurück"), (false, "Weiter ›")] {
        let r = page_rect(width, back);
        fill(
            pm,
            r,
            alpha(p.surface_alt, H.quiet_alpha),
            Radius::DEFAULT.sm,
        );
        paint_text_centered(
            pm,
            label,
            r,
            Typography::DEFAULT.caption_size as f32,
            p.text,
        );
    }
    paint_text_right_centered(
        pm,
        &format!(
            "{} / {}",
            page + 1,
            count.div_ceil(H.room_columns as usize).max(1)
        ),
        page_rect(width, true).x - S.md,
        page_rect(width, true),
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
}
