fn build_wallpaper_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    let modes: Vec<Box<dyn Widget>> = [
        ("wallpaper-mode-fill", "Füllen", WallpaperMode::Fill),
        ("wallpaper-mode-fit", "Einpassen", WallpaperMode::Fit),
        ("wallpaper-mode-center", "Zentrieren", WallpaperMode::Center),
        ("wallpaper-mode-tile", "Kacheln", WallpaperMode::Tile),
    ]
    .into_iter()
    .map(|(id, label, mode)| {
        Box::new(SettingsControl {
            id: Some(id),
            kind: niwoe_ui::widget::ComponentKind::Chip,
            disabled: false,
            label: label.into(),
            selected: mode == ctx.wallpaper_mode,
            width: c.wallpaper_control_width,
        }) as Box<dyn Widget>
    })
    .collect();
    rows.push(Box::new(Container::row(c.option_gap, modes)));
    rows.push(Box::new(WallpaperBrowseRow { row_width: row_w }));
    let entries = wallpaper_visible_indices(ctx.available_wallpapers, ctx.query);
    let slots = wallpaper_page_size(ctx.content_h);
    let pages = entries.len().div_ceil(slots).max(1);
    let page = ctx.wallpaper_page.min(pages - 1);
    let width = (row_w - c.option_gap * (c.wallpaper_grid_columns - 1) as i32)
        / c.wallpaper_grid_columns as i32;
    let cards: Vec<Box<dyn Widget>> = entries
        .iter()
        .skip(page * slots)
        .take(slots)
        .map(|index| {
            let entry = &ctx.available_wallpapers[*index];
            let preview = ctx
                .wallpaper_thumbnails
                .get(*index)
                .filter(|p| p.path == entry.thumbnail_path);
            let selected = ctx.current_wallpaper == Some(entry.apply_path.as_str());
            Box::new(WallpaperRow {
                index: *index,
                display_name: entry.display_name.as_str().into(),
                thumbnail: preview.and_then(|p| p.image.clone()),
                is_selected: selected,
                row_width: width,
                status: if preview.is_none() {
                    "Vorschau wird geladen …"
                } else if preview.is_some_and(|p| p.image.is_none()) {
                    "Vorschau nicht verfügbar"
                } else if selected {
                    "Ausgewählt"
                } else {
                    "Hintergrundbild"
                },
            }) as Box<dyn Widget>
        })
        .collect();
    let mut pending = cards.into_iter();
    loop {
        let group: Vec<_> = pending.by_ref().take(c.wallpaper_grid_columns).collect();
        if group.is_empty() {
            break;
        }
        rows.push(Box::new(Container::row(c.option_gap, group)));
    }
    if entries.is_empty() {
        rows.push(Box::new(SettingsPlaceholder {
            width: row_w,
            text: "Keine Hintergrundbilder gefunden. Eigenes Bild auswählen.",
        }));
    }
    let mut footer: Vec<Box<dyn Widget>> = Vec::new();
    for (id, label, disabled) in [
        ("wallpaper-page-previous", "Zurück", page == 0),
        ("wallpaper-page-next", "Weiter", page + 1 == pages),
    ] {
        footer.push(Box::new(SettingsControl {
            id: Some(id),
            kind: niwoe_ui::widget::ComponentKind::Button,
            disabled,
            label: label.into(),
            selected: false,
            width: c.wallpaper_control_width,
        }));
    }
    footer.push(Box::new(SettingsPageLabel {
        label: format!(
            "Seite {} von {} · {} Bilder",
            page + 1,
            pages,
            entries.len()
        ),
        width: (row_w - c.wallpaper_control_width * 2 - c.option_gap * 2).max(0),
    }));
    rows.push(Box::new(Container::row(c.option_gap, footer)));
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Hintergrund",
        "Bild und Darstellung wählen. Die Vorschau zeigt das ausgewählte Motiv.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}
