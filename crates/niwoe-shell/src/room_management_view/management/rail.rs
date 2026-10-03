use super::*;

fn section(
    pm: &mut tiny_skia::PixmapMut<'_>,
    area: Rect,
    label: &str,
    config: &niwoe_config::ThemeConfig,
) {
    let p = crate::ui::tokens::theme_from_config(config).palette;
    fill(pm, area, alpha(p.surface, C.card_alpha), Radius::DEFAULT.sm);
    outline(pm, area, p.border, Controls::BORDER);
    paint_text_left_centered(
        pm,
        label,
        area.x + C.card_pad,
        Rect {
            height: S.xxl + S.md,
            ..area
        },
        Typography::DEFAULT.title_size as f32,
        p.text,
    );
}

pub(super) fn draw(pm: &mut tiny_skia::PixmapMut<'_>, width: u32, context: &Context<'_>) {
    let config = context.card.config;
    let p = crate::ui::tokens::theme_from_config(config).palette;
    let quick = Rect {
        x: right_rail_x(width),
        y: list::grid_top(width),
        width: C.right_rail_width,
        height: C.quick_actions_height,
    };
    section(pm, quick, "Schnellaktionen", config);
    for (index, label) in ["Neuen Raum anlegen"].iter().enumerate() {
        list::control(
            pm,
            list::quick_rect(width, index),
            label,
            list::ControlState {
                enabled: index == 0 && context.all_rooms.len() < niwoe_config::rooms::MAX_ROOMS,
                focused: index == 0 && context.list.focus == Some(5),
                ..Default::default()
            },
            config,
        );
    }
    let stats = Rect {
        y: quick.y + quick.height + C.card_gap,
        height: C.statistics_height,
        ..quick
    };
    section(pm, stats, "Statistik", config);
    let occupied = list::occupancy(context.all_rooms, context.card.window_counts);
    let windows: usize = context
        .all_rooms
        .iter()
        .filter_map(|room| {
            context
                .card
                .window_counts
                .get(room.workspace.saturating_sub(1) as usize)
        })
        .map(|count| usize::from(*count))
        .sum();
    let apps: usize = context
        .all_rooms
        .iter()
        .map(|room| room.preferences.apps.len())
        .sum();
    for (index, text) in [
        format!("{} Räume gesamt", context.all_rooms.len()),
        format!("{occupied} Belegt"),
        format!("{} Leer", context.all_rooms.len() - occupied),
        format!("{windows} Offene Fenster"),
        format!("{apps} App-Zuordnungen"),
    ]
    .iter()
    .enumerate()
    {
        paint_text_left_centered(
            pm,
            text,
            stats.x + C.card_pad,
            Rect {
                y: stats.y + S.xxl + S.md + index as i32 * Controls::MIN_HEIGHT,
                height: Controls::MIN_HEIGHT,
                ..stats
            },
            Typography::DEFAULT.body_size as f32,
            if index == 1 { p.success } else { p.text_dim },
        );
    }
}
