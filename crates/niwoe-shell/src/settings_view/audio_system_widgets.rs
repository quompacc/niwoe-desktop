fn audio_volume_controls(default: &AudioDevice) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let mut controls: Vec<Box<dyn Widget>> = VOLUME_PRESET_OPTIONS
        .iter()
        .map(|(percent, id, label)| {
            Box::new(SettingsControl {
                id: Some(id),
                kind: niwoe_ui::widget::ComponentKind::Chip,
                disabled: false,
                label: (*label).into(),
                selected: default.volume_percent == Some(*percent),
                width: c.volume_preset_width,
            }) as Box<dyn Widget>
        })
        .collect();
    controls.push(Box::new(SettingsControl {
        id: Some("mute-toggle"),
        kind: niwoe_ui::widget::ComponentKind::Chip,
        disabled: false,
        label: if default.muted {
            "Stumm: an"
        } else {
            "Stumm: aus"
        }
        .into(),
        selected: default.muted,
        width: c.device_control_width,
    }));
    Box::new(Container::row(c.option_gap, controls))
}

fn audio_device_row(
    device: &AudioDevice,
    id: Option<&'static str>,
    symbol: niwoe_ui::effect::Symbol,
    width: i32,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    let volume = device
        .volume_percent
        .map(|percent| format!("Lautstärke {percent} %"))
        .unwrap_or_else(|| "Lautstärke unbekannt".into());
    let mute = if device.muted { " · Stumm" } else { "" };
    let target = if device.is_default {
        "Standardgerät"
    } else {
        "Als Standard verwenden"
    };
    settings_text_row(
        device.name.as_str(),
        format!("{volume}{mute} · {target}"),
        width,
        if device.is_default { None } else { id },
        device.is_default,
        Some(symbol),
        device.is_default.then_some(niwoe_ui::effect::Symbol::Check),
        pal,
    )
}

fn printer_device_row(
    printer: &PrinterInfo,
    width: i32,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    // Translate only known CUPS states; retain an unknown diagnostic from the provider.
    let status = if printer.status == "is idle" {
        "Warteschlange im Leerlauf"
    } else if printer.status.starts_with("is printing") {
        "Druckt"
    } else if !printer.enabled {
        "Warteschlange deaktiviert"
    } else if printer.status.is_empty() {
        "Status unbekannt"
    } else {
        &printer.status
    };
    let accepting = match printer.accepting {
        Some(true) => "Nimmt Aufträge an",
        Some(false) => "Nimmt keine Aufträge an",
        None => "Auftragsannahme unbekannt",
    };
    settings_text_row(
        printer.name.as_str(),
        format!(
            "{status} · {accepting} · {}",
            printer_job_count_label(printer.job_count)
        ),
        width,
        None,
        printer.is_default,
        Some(niwoe_ui::effect::Symbol::Printer),
        printer
            .is_default
            .then_some(niwoe_ui::effect::Symbol::Check),
        pal,
    )
}

fn readonly_settings_rows(
    values: impl IntoIterator<Item = (String, String)>,
    width: i32,
    symbol: Option<niwoe_ui::effect::Symbol>,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let half_w = (width - c.option_gap) / 2;
    let mut values = values.into_iter();
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    loop {
        let pair: Vec<_> = values
            .by_ref()
            .take(2)
            .map(|(label, value)| {
                settings_text_row(label, value, half_w, None, false, symbol, None, pal)
            })
            .collect();
        if pair.is_empty() {
            break;
        }
        rows.push(Box::new(Container::row(c.option_gap, pair)));
    }
    Box::new(Container::column(c.option_gap, rows))
}
