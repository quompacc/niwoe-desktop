fn build_power_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let idle_chips = IDLE_TIMEOUT_OPTIONS
        .iter()
        .map(|(secs, id, label)| {
            Box::new(SettingsControl {
                id: Some(id),
                kind: niwoe_ui::widget::ComponentKind::Chip,
                disabled: false,
                label: (*label).into(),
                selected: *secs == ctx.idle_timeout_secs,
                width: c.volume_preset_width,
            }) as Box<dyn Widget>
        })
        .collect();
    let mut rows: Vec<Box<dyn Widget>> = vec![
        Box::new(SidebarSectionLabel {
            text: "BILDSCHIRM-LEERLAUF",
            width: row_w,
            pad_top: 0,
        }),
        Box::new(Container::row(c.option_gap, idle_chips)),
        Box::new(SidebarSectionLabel {
            text: "SITZUNG",
            width: row_w,
            pad_top: c.group_gap,
        }),
    ];
    let armed = ctx
        .armed_power
        .filter(|(_, elapsed)| {
            elapsed.is_finite()
                && *elapsed >= 0.0
                && *elapsed * 1000.0 < crate::POWER_ARM_TIMEOUT_MS as f32
        })
        .map(|(id, _)| id);
    for (id, label) in [
        ("power-sleep", "Bereitschaft"),
        ("power-lock", "Sperren"),
        ("power-logout", "Abmelden"),
        ("power-restart", "Neu starten"),
        ("power-off", "Ausschalten"),
    ] {
        rows.push(power_action_row(id, label, armed, row_w, ctx.pal));
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Leerlauf und Sitzung",
        "Sitzungsaktionen mit erneuter Bestätigung. Eine andere Auswahl bricht die Bestätigung ab.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}

fn power_action_row(
    id: &'static str,
    label: &'static str,
    armed: Option<&str>,
    width: i32,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    let selected = armed == Some(id);
    settings_text_row(
        label,
        if selected {
            "Erneut auswählen, um zu bestätigen"
        } else {
            "Zweimal auswählen, um die Aktion auszuführen"
        },
        width,
        Some(id),
        selected,
        Some(if id == "power-lock" {
            niwoe_ui::effect::Symbol::Lock
        } else {
            niwoe_ui::effect::Symbol::System
        }),
        selected.then_some(niwoe_ui::effect::Symbol::Check),
        pal,
    )
}
