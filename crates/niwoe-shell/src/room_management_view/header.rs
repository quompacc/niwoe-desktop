//! Shared cached landscape and display role for the room-management heading.
use super::*;

pub(super) fn draw_header(
    pm: &mut tiny_skia::PixmapMut<'_>,
    width: u32,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let area = Rect {
        x: C.sidebar_width,
        y: 0,
        width: width as i32 - C.sidebar_width,
        height: C.header_height,
    };
    // Static opaque artwork keeps application menus out of the heading.
    // The source and fitted raster use the existing bounded landscape cache.
    fill(pm, area, p.background, Radius::DEFAULT.none);
    crate::landscape::paint(pm, area);
    let x = area.x + C.outer_pad * 2;
    paint_text(
        pm,
        "RÄUME",
        x,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    niwoe_ui::effect::paint_display_heading(
        pm,
        "Räume verwalten",
        x,
        C.outer_pad + S.xxl * 2,
        area,
        p.text,
    );
    for (index, text) in [
        "Organisiere deine Arbeitsumgebungen. Räume bündeln Kontext, Apps, Dateien",
        "und Einstellungen – für fokussiertes Arbeiten in jedem Bereich.",
    ]
    .into_iter()
    .enumerate()
    {
        paint_text(
            pm,
            &truncate_to_fit(
                text,
                area.width - C.outer_pad * 4,
                Typography::DEFAULT.body_size as f32,
            ),
            x,
            C.outer_pad + S.xxl * 3 + S.lg + index as i32 * S.xl,
            Typography::DEFAULT.body_size as f32,
            p.text_dim,
        );
    }
}

pub(super) fn draw_configuration(
    pm: &mut tiny_skia::PixmapMut<'_>,
    room: &RoomEntry,
    edit: &crate::room_editor::Edit,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let area = Rect {
        x: C.sidebar_width,
        y: 0,
        width: pm.width() as i32 - C.sidebar_width,
        height: C.config_header_height,
    };
    fill(pm, area, p.background, Radius::DEFAULT.none);
    crate::landscape::paint(pm, area);
    let x = area.x + C.outer_pad * 2;
    let name = if edit.name.trim().is_empty() {
        room.name.as_str()
    } else {
        edit.name.as_str()
    };
    paint_text(
        pm,
        &truncate_to_fit(
            &format!("RÄUME  ›  {name}  ›  KONFIGURATION"),
            area.width - C.outer_pad * 4,
            Typography::DEFAULT.caption_size as f32,
        ),
        x,
        C.outer_pad + S.lg,
        Typography::DEFAULT.caption_size as f32,
        p.accent,
    );
    niwoe_ui::effect::paint_display_heading(
        pm,
        if edit.id == 0 {
            "Neuer Raum"
        } else {
            "Raum konfigurieren"
        },
        x,
        C.outer_pad + S.xxl * 2,
        area,
        p.text,
    );
    for (index, text) in [
        "Gestalte deinen Arbeitskontext mit Apps, Dateien und Einstellungen.",
        if edit.restore.open {
            "Dateiverweise und Layoutaktionen werden separat bestätigt; Raumdetails bleiben im Entwurf."
        } else {
            "Änderungen bleiben im Entwurf, bis du sie speicherst."
        },
    ]
    .into_iter()
    .enumerate()
    {
        paint_text(
            pm,
            &truncate_to_fit(
                text,
                area.width - C.outer_pad * 4,
                Typography::DEFAULT.body_size as f32,
            ),
            x,
            C.outer_pad + S.xxl * 3 + S.lg + index as i32 * S.xl,
            Typography::DEFAULT.body_size as f32,
            p.text_dim,
        );
    }
}
