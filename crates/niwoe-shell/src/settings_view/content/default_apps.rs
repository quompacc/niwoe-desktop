fn build_default_apps_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    if let Some(category) = ctx.default_apps_picker_open {
        rows.push(default_apps_button(
            "default-apps-back",
            "Zurück zur Übersicht",
            row_w,
            false,
        ));
        rows.push(default_app_category_row(ctx, category, row_w, None));
        if let Some(index) = ctx.default_apps_index {
            let category_index = crate::default_apps::DefaultAppCategory::ALL
                .iter()
                .position(|cat| *cat == category)
                .unwrap_or(0);
            let candidates = index.apps_for_mime(category.representative_mime());
            let count = candidates.len().min(DEFAULT_APP_MAX_APPS_PER_CAT);
            if count == 0 {
                rows.push(settings_text_row(
                    "Keine passende App installiert",
                    "Für diese Aufgabe wurde keine auswählbare Anwendung gefunden.",
                    row_w,
                    None,
                    false,
                    None,
                    None,
                    ctx.pal,
                ));
            } else {
                let slots = default_apps_page_size(ctx.content_h);
                let pages = count.div_ceil(slots);
                let page = ctx.default_apps_page.min(pages - 1);
                for (app_index, app) in candidates
                    .iter()
                    .enumerate()
                    .take(count)
                    .skip(page * slots)
                    .take(slots)
                {
                    let current = ctx
                        .default_apps_current
                        .get(&category)
                        .is_some_and(|id| id == &app.desktop_id);
                    let mut row = niwoe_ui::widget::TextRow::new(
                        app.name.clone(),
                        if current {
                            "Aktuelle Zuordnung"
                        } else {
                            "Als Standard verwenden"
                        },
                        row_w,
                    );
                    row.id = (!ctx.default_apps_status.busy)
                        .then(|| default_apps_set_id(category_index, app_index))
                        .flatten();
                    row.selected = current;
                    row.leading = default_app_icon(ctx, Some(app));
                    row.trailing = current
                        .then(|| {
                            niwoe_ui::effect::symbol_icon(
                                niwoe_ui::effect::Symbol::Check,
                                ctx.pal.accent,
                                niwoe_tokens::Controls::SYMBOL_SIZE,
                            )
                        })
                        .flatten();
                    rows.push(Box::new(row));
                }
                rows.push(default_apps_navigation(
                    row_w,
                    page,
                    pages,
                    count,
                    candidates.len(),
                ));
            }
        } else {
            rows.push(settings_text_row(
                "Apps werden geladen",
                "Installierte Anwendungen und gespeicherte Zuordnungen werden geprüft.",
                row_w,
                None,
                false,
                None,
                None,
                ctx.pal,
            ));
        }
    } else {
        rows.push(default_apps_button(
            "default-apps-auto",
            "Leere Zuordnungen automatisch ergänzen",
            row_w,
            ctx.default_apps_index.is_none() || ctx.default_apps_status.busy,
        ));
        let width = (row_w - c.option_gap) / 2;
        for categories in crate::default_apps::DefaultAppCategory::ALL.chunks(2) {
            rows.push(Box::new(Container::row(
                c.option_gap,
                categories
                    .iter()
                    .map(|category| {
                        let category_index = crate::default_apps::DefaultAppCategory::ALL
                            .iter()
                            .position(|cat| cat == category)
                            .unwrap_or(0);
                        default_app_category_row(
                            ctx,
                            *category,
                            width,
                            ctx.default_apps_index
                                .and_then(|_| default_apps_pick_id(category_index)),
                        )
                    })
                    .collect(),
            )));
        }
    }
    let description = if ctx.default_apps_status.message.is_empty() {
        std::borrow::Cow::Borrowed("Anwendungen für häufige Dateitypen und Aufgaben auswählen.")
    } else {
        std::borrow::Cow::Owned(ctx.default_apps_status.message.clone())
    };
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Standardzuordnungen",
        description,
        Box::new(Container::column(c.option_gap, rows)),
    )
}
