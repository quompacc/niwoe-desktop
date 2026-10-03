//! Catalog-backed draft rows; icon decoding remains in the existing worker.
use super::*;
use crate::launcher::DesktopApp;
use niwoe_ipc::AppReference;
use niwoe_ui::effect::{paint_focus, paint_text_pair, Symbol};

fn application<'a>(reference: &AppReference, catalog: &'a [DesktopApp]) -> Option<&'a DesktopApp> {
    catalog.iter().find(|app| match reference {
        AppReference::Native(id) => app.desktop_id.strip_suffix(".desktop") == Some(id.as_str()),
        AppReference::Xwayland(id) => app.startup_wm_class.as_deref() == Some(id.as_str()),
    })
}

pub(crate) fn draw_apps(
    pm: &mut tiny_skia::PixmapMut<'_>,
    edit: &Edit,
    icons: &crate::icons::IconCache,
    catalog: &[DesktopApp],
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let x = C.sidebar_width + C.outer_pad;
    paint_text(
        pm,
        "Apps diesem Raum zuordnen",
        x,
        body_top() + S.xl,
        Typography::DEFAULT.title_size as f32,
        p.text,
    );
    paint_text(
        pm,
        "Auswahl bleibt im Entwurf. Apps starten dadurch nicht automatisch.",
        x,
        body_top() + S.xxl + S.sm,
        Typography::DEFAULT.caption_size as f32,
        p.text_dim,
    );
    let rows = edit.app_rows();
    let search = app_rect(pm.width(), 14);
    general::native_control(pm, search, "", edit.focus == 14, true, config);
    let search_icon = Rect {
        x: search.x + S.sm,
        width: S.xl,
        ..search
    };
    super::super::super::list::symbol(pm, search_icon, Symbol::Search, p.text_dim);
    paint_text_left_centered(
        pm,
        &truncate_to_fit(
            if edit.form.query.is_empty() {
                "Apps filtern …"
            } else {
                &edit.form.query
            },
            search.width - S.xl - S.sm * 3,
            Typography::DEFAULT.body_size as f32,
        ),
        search_icon.x + search_icon.width + S.sm,
        search,
        Typography::DEFAULT.body_size as f32,
        p.text,
    );
    for (index, label, enabled) in [
        (15, "Vorige Apps".to_owned(), edit.form.page > 0),
        (
            16,
            format!(
                "Weitere Apps · Seite {} / {}",
                edit.form.page + 1,
                rows.len().saturating_sub(1) / 6 + 1
            ),
            (edit.form.page + 1) * 6 < rows.len(),
        ),
    ] {
        general::native_control(
            pm,
            app_rect(pm.width(), index),
            &label,
            edit.focus == index,
            enabled,
            config,
        );
    }
    for (row, index) in rows.iter().skip(edit.form.page * 6).take(6).enumerate() {
        let reference = &edit.form.apps[*index].1;
        let selected = edit.preferences.apps.contains(reference);
        let app = application(reference, catalog);
        let area = app_rect(pm.width(), 20 + row);
        fill(
            pm,
            area,
            if selected {
                Interaction::DEFAULT.selected_tint(p.surface)
            } else {
                p.surface
            },
            Radius::DEFAULT.sm,
        );
        outline(pm, area, p.border_control(), Controls::BORDER);
        let checkbox = Rect {
            x: area.x + S.sm,
            y: area.y + (area.height - S.lg) / 2,
            width: S.lg,
            height: S.lg,
        };
        fill(
            pm,
            checkbox,
            if selected { p.accent } else { p.surface_alt },
            Radius::DEFAULT.sm,
        );
        outline(pm, checkbox, p.border_control(), Controls::BORDER);
        if selected {
            super::super::super::list::symbol(pm, checkbox, Symbol::Check, p.on_accent());
        }
        let icon = Rect {
            x: checkbox.x + checkbox.width + S.sm,
            width: S.xxl,
            ..area
        };
        if let Some(image) = app
            .and_then(|app| app.icon_name.as_deref())
            .and_then(|name| icons.lookup(name, S.xxl as u32))
            .and_then(crate::icons::icon_image_to_pixmap)
        {
            pm.draw_pixmap(
                icon.x,
                icon.y + (icon.height - image.height() as i32) / 2,
                image.as_ref(),
                &tiny_skia::PixmapPaint::default(),
                tiny_skia::Transform::identity(),
                None,
            );
        } else {
            super::super::super::list::symbol(pm, icon, Symbol::App, p.text_dim);
        }
        let (AppReference::Native(id) | AppReference::Xwayland(id)) = reference;
        let detail = if let Some(app) = app {
            let kind = match reference {
                AppReference::Native(_) => "Native App",
                AppReference::Xwayland(_) => "XWayland-Fenster",
            };
            if catalog
                .iter()
                .filter(|other| other.name == app.name)
                .take(2)
                .count()
                > 1
            {
                format!("{kind} · {id}")
            } else {
                kind.to_owned()
            }
        } else {
            format!("Nicht installiert · {id} · Auswahl kann entfernt werden")
        };
        let text_x = icon.x + icon.width + S.sm;
        paint_text_pair(
            pm,
            Rect {
                x: text_x,
                width: area.x + area.width - text_x - S.sm,
                ..area
            },
            app.map_or("Unbekannte App", |app| app.name.as_str()),
            &detail,
            p.text,
            p.text_dim,
        );
        if edit.focus == 20 + row {
            paint_focus(pm, area, p.border_focus(), Radius::DEFAULT.sm);
        }
    }
    if rows.is_empty() {
        let area = app_rect(pm.width(), 20);
        paint_text_pair(
            pm,
            area,
            "Keine passenden Apps",
            "Passe den Suchtext an. Fehlende ausgewählte Apps bleiben entfernbar.",
            p.text,
            p.text_dim,
        );
    }
}
