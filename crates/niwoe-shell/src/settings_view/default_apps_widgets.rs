// ─── DefaultApps page widgets ────────────────────────────────────────────────

const DEFAULT_APP_MAX_CATS: usize = 12;
const DEFAULT_APP_MAX_APPS_PER_CAT: usize = crate::default_apps::MAX_APPS_PER_CATEGORY;

/// Stable widget ids for the category rows (the row click toggles the
/// candidate-list expander). Lazily leaked once so the `'static` lifetime
/// the Widget trait wants is satisfied without const-string gymnastics.
fn default_apps_pick_id(idx: usize) -> Option<&'static str> {
    use std::sync::OnceLock;
    static IDS: OnceLock<Vec<&'static str>> = OnceLock::new();
    let ids = IDS.get_or_init(|| {
        (0..DEFAULT_APP_MAX_CATS)
            .map(|i| Box::leak(format!("default-apps-pick-{}", i).into_boxed_str()) as &'static str)
            .collect()
    });
    ids.get(idx).copied()
}

/// Stable widget ids for the candidate rows within each category's picker.
fn default_apps_set_id(cat_idx: usize, app_idx: usize) -> Option<&'static str> {
    use std::sync::OnceLock;
    static IDS: OnceLock<Vec<Vec<&'static str>>> = OnceLock::new();
    let ids = IDS.get_or_init(|| {
        (0..DEFAULT_APP_MAX_CATS)
            .map(|c| {
                (0..DEFAULT_APP_MAX_APPS_PER_CAT)
                    .map(|a| {
                        Box::leak(format!("default-apps-set-{}-{}", c, a).into_boxed_str())
                            as &'static str
                    })
                    .collect()
            })
            .collect()
    });
    ids.get(cat_idx).and_then(|row| row.get(app_idx)).copied()
}

fn default_app_icon(
    ctx: &SettingsContentContext<'_>,
    app: Option<&crate::default_apps::MimeAppCandidate>,
) -> Option<std::sync::Arc<Pixmap>> {
    app.and_then(|app| ctx.default_apps_index?.app_icon(app))
        .or_else(|| {
            niwoe_ui::effect::symbol_icon(
                niwoe_ui::effect::Symbol::App,
                ctx.pal.text_dim,
                niwoe_tokens::Controls::SYMBOL_SIZE,
            )
        })
}

fn default_app_category_row(
    ctx: &SettingsContentContext<'_>,
    category: crate::default_apps::DefaultAppCategory,
    width: i32,
    id: Option<&'static str>,
) -> Box<dyn Widget> {
    let current = ctx.default_apps_current.get(&category);
    let app = current.and_then(|id| ctx.default_apps_index?.lookup(id));
    let subtitle = if ctx.default_apps_status.failed_reads.contains(&category) {
        "Zuordnung konnte nicht geladen werden"
    } else if ctx.default_apps_index.is_none() {
        "Zuordnung wird geladen"
    } else if let Some(app) = app {
        app.name.as_str()
    } else if current.is_some() {
        "Gespeicherte App nicht verfügbar"
    } else {
        "Noch nicht festgelegt"
    };
    let mut row = niwoe_ui::widget::TextRow::new(category.label(), subtitle, width);
    row.id = id;
    row.leading = default_app_icon(ctx, app);
    row.trailing = id.and_then(|_| {
        niwoe_ui::effect::symbol_icon(
            niwoe_ui::effect::Symbol::ChevronDown,
            ctx.pal.text_dim,
            niwoe_tokens::Controls::SYMBOL_SIZE,
        )
    });
    Box::new(row)
}

fn default_apps_button(
    id: &'static str,
    label: &str,
    width: i32,
    disabled: bool,
) -> Box<dyn Widget> {
    Box::new(SettingsControl {
        id: Some(id),
        label: label.into(),
        width,
        disabled,
        selected: false,
        kind: niwoe_ui::widget::ComponentKind::Button,
    })
}

pub(crate) fn default_apps_page_size(content_height: u32) -> usize {
    let c = SETTINGS_CHROME;
    let reserved = niwoe_tokens::Controls::MIN_HEIGHT * 2
        + niwoe_tokens::Controls::TEXT_PAIR_HEIGHT
        + c.option_gap * 3;
    let room = settings_group_body_height(content_height).saturating_sub(reserved as u32);
    ((room + c.option_gap as u32)
        / (niwoe_tokens::Controls::TEXT_PAIR_HEIGHT + c.option_gap) as u32)
        .max(1) as usize
}

fn default_apps_navigation(
    width: i32,
    page: usize,
    pages: usize,
    count: usize,
    total: usize,
) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let count_label = if count < total {
        format!("{count} von {total} Apps")
    } else {
        format!("{count} {}", if count == 1 { "App" } else { "Apps" })
    };
    Box::new(Container::row(
        c.option_gap,
        vec![
            default_apps_button(
                "default-apps-previous",
                "Zurück",
                c.wallpaper_control_width,
                page == 0,
            ),
            default_apps_button(
                "default-apps-next",
                "Weiter",
                c.wallpaper_control_width,
                page + 1 >= pages,
            ),
            Box::new(SettingsPageLabel {
                label: format!("Seite {} von {} · {count_label}", page + 1, pages),
                width: (width - c.wallpaper_control_width * 2 - c.option_gap * 2).max(0),
            }),
        ],
    ))
}
