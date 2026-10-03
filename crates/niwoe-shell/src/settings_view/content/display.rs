fn build_display_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    use niwoe_ui::effect::Symbol;
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    let outputs = ctx.output_workspaces.len().min(DISPLAY_OUTPUT_MAX);
    if outputs == 0 {
        rows.push(settings_text_row(
            "Keine Bildschirme verfügbar",
            "Die Anzeigeinformationen wurden noch nicht empfangen.",
            row_w,
            None,
            false,
            Some(Symbol::Monitor),
            None,
            ctx.pal,
        ));
    } else {
        let idx = ctx.display_pages.output.min(outputs - 1);
        let output = &ctx.output_workspaces[idx];
        rows.push(display_navigation(
            row_w, idx, outputs, false, outputs, outputs,
        ));
        let title = format!(
            "Bildschirm {}{}",
            idx + 1,
            if output.primary {
                " · Primäranzeige"
            } else {
                ""
            }
        );
        let subtitle = selected_display_mode(output)
            .map(display_mode_label)
            .unwrap_or_else(|| "Aktueller Modus nicht verfügbar".into());
        rows.push(settings_text_row(
            title,
            subtitle,
            row_w,
            None,
            false,
            Some(Symbol::Monitor),
            None,
            ctx.pal,
        ));
        let indices = display_mode_indices(output);
        let expanded = ctx.display_mode_dropdown_open == Some(idx);
        let combo_label = selected_display_mode(output)
            .map(display_mode_label)
            .unwrap_or_else(|| "Keine Modi verfügbar".into());
        rows.push(settings_text_row(
            "Auflösung und Bildwiederholrate",
            combo_label,
            row_w,
            (!indices.is_empty()).then(|| DISPLAY_MODE_TOGGLE_IDS[idx]),
            false,
            None,
            Some(if expanded {
                Symbol::ChevronUp
            } else {
                Symbol::ChevronDown
            }),
            ctx.pal,
        ));
        let width = (row_w - c.display_control_gap * 2) / 3;
        let scale = format!("{:.1} %", output.scale_millis as f64 / 10.0).replace('.', ",");
        let rotation = match output.transform.as_deref() {
            Some("90" | "_90") => "90°",
            Some("180" | "_180") => "180°",
            Some("270" | "_270") => "270°",
            Some("Flipped") => "Gespiegelt",
            Some("Flipped90") => "Gespiegelt · 90°",
            Some("Flipped180") => "Gespiegelt · 180°",
            Some("Flipped270") => "Gespiegelt · 270°",
            Some("Normal" | "") => "0°",
            _ => "Ausrichtung unbekannt",
        };
        rows.push(Box::new(Container::row(
            c.display_control_gap,
            vec![
                settings_text_row(
                    "Primäranzeige",
                    if output.primary {
                        "Ausgewählt"
                    } else {
                        "Als primär setzen"
                    },
                    width,
                    (!output.primary).then(|| DISPLAY_PRIMARY_IDS[idx]),
                    output.primary,
                    None,
                    output.primary.then_some(Symbol::Check),
                    ctx.pal,
                ),
                settings_text_row(
                    "Skalierung",
                    scale,
                    width,
                    Some(DISPLAY_SCALE_IDS[idx]),
                    false,
                    None,
                    None,
                    ctx.pal,
                ),
                settings_text_row(
                    "Drehung",
                    rotation,
                    width,
                    Some(DISPLAY_ROTATE_IDS[idx]),
                    false,
                    None,
                    None,
                    ctx.pal,
                ),
            ],
        )));
        if expanded {
            let slots = display_mode_page_size(ctx.content_h);
            let pages = indices.len().div_ceil(slots).max(1);
            let page = ctx.display_pages.modes.min(pages - 1);
            for mode_idx in indices.iter().skip(page * slots).take(slots) {
                let mode = &output.modes[*mode_idx];
                let subtitle = match (mode.current, mode.preferred) {
                    (true, true) => "Aktuell · Bevorzugter Modus",
                    (true, false) => "Aktuell",
                    (false, true) => "Bevorzugter Modus",
                    (false, false) => "Verfügbarer Modus",
                };
                rows.push(settings_text_row(
                    display_mode_label(mode),
                    subtitle,
                    row_w,
                    display_mode_option_id(idx, *mode_idx),
                    mode.current,
                    None,
                    mode.current.then_some(Symbol::Check),
                    ctx.pal,
                ));
            }
            let total = output
                .modes
                .iter()
                .filter(|mode| mode.width > 0 && mode.height > 0)
                .count();
            rows.push(display_navigation(
                row_w,
                page,
                pages,
                true,
                indices.len(),
                total,
            ));
        }
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Bildschirme",
        "Bildschirm wählen und Auflösung, Skalierung oder Ausrichtung anpassen.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}
