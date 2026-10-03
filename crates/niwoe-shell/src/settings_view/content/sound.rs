fn build_sound_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let half_w = (row_w - c.option_gap) / 2;
    let snapshot = ctx.audio_snapshot;
    let slots = provider_page_size(SettingsCategory::Sound, ctx.content_h);
    let count = snapshot
        .outputs
        .len()
        .max(snapshot.inputs.len())
        .min(SOUND_MAX);
    let range = provider_page_range(ctx.provider_page, slots, count);
    let running = snapshot.service == AudioServiceState::Running;
    let mut rows: Vec<Box<dyn Widget>> = vec![settings_text_row(
        if running {
            "Audio-Dienst aktiv"
        } else {
            "Audio-Status nicht verfügbar"
        },
        if running {
            format!(
                "{} Ausgabegeräte · {} Eingabegeräte",
                snapshot.outputs.len(),
                snapshot.inputs.len()
            )
        } else {
            "Der Status der Audio-Geräte konnte nicht gelesen werden.".into()
        },
        row_w,
        None,
        false,
        None,
        None,
        ctx.pal,
    )];
    if running {
        if let Some(default) = snapshot.default_output.as_ref() {
            rows.push(settings_text_row(
                "Standard-Ausgabe",
                default.name.as_str(),
                row_w,
                None,
                false,
                Some(niwoe_ui::effect::Symbol::Speaker),
                None,
                ctx.pal,
            ));
            rows.push(audio_volume_controls(default));
        } else {
            rows.push(settings_text_row(
                "Keine Standard-Ausgabe",
                "Es ist kein Ziel für Lautstärke und Stummschaltung bekannt.",
                row_w,
                None,
                false,
                None,
                None,
                ctx.pal,
            ));
        }
        let mut columns = Vec::new();
        for (label, devices, ids, symbol) in [
            (
                "AUSGABE",
                &snapshot.outputs,
                AUDIO_OUTPUT_IDS,
                niwoe_ui::effect::Symbol::Speaker,
            ),
            (
                "EINGABE",
                &snapshot.inputs,
                AUDIO_INPUT_IDS,
                niwoe_ui::effect::Symbol::Microphone,
            ),
        ] {
            let mut device_rows: Vec<Box<dyn Widget>> = vec![Box::new(SidebarSectionLabel {
                text: label,
                width: half_w,
                pad_top: c.group_gap,
            })];
            if devices.is_empty() {
                device_rows.push(settings_text_row(
                    "Keine Geräte gemeldet",
                    "Für diesen Bereich ist kein Gerät bekannt.",
                    half_w,
                    None,
                    false,
                    Some(symbol),
                    None,
                    ctx.pal,
                ));
            }
            for (index, device) in devices
                .iter()
                .take(SOUND_MAX)
                .enumerate()
                .skip(range.start)
                .take(range.len())
            {
                device_rows.push(audio_device_row(
                    device,
                    ids.get(index).copied(),
                    symbol,
                    half_w,
                    ctx.pal,
                ));
            }
            columns.push(Box::new(Container::column(c.option_gap, device_rows)) as Box<dyn Widget>);
        }
        rows.push(Box::new(Container::row(c.option_gap, columns)));
        if let Some(footer) = provider_navigation(
            row_w,
            ctx.provider_page,
            slots,
            count,
            format!(
                "{} · {}",
                provider_count_label(
                    snapshot.outputs.len().min(SOUND_MAX),
                    snapshot.outputs.len(),
                    "Ausgaben"
                ),
                provider_count_label(
                    snapshot.inputs.len().min(SOUND_MAX),
                    snapshot.inputs.len(),
                    "Eingaben"
                )
            ),
        ) {
            rows.push(footer);
        }
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Audio-Geräte",
        "Ausgabe, Eingabe und Lautstärke für diese Sitzung.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}
