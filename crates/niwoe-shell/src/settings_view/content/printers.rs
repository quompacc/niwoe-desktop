fn build_printers_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let snapshot = ctx.printer_snapshot;
    let mut rows: Vec<Box<dyn Widget>> = vec![settings_text_row(
        match snapshot.service {
            PrinterServiceState::Running => "Druckdienst aktiv",
            PrinterServiceState::Stopped => "Druckdienst nicht aktiv",
            PrinterServiceState::Unavailable => "Druckstatus nicht verfügbar",
        },
        if snapshot.service == PrinterServiceState::Running {
            format!(
                "{} · {}",
                if snapshot.list_available {
                    provider_count_label(
                        snapshot.printers.len().min(PRINTER_MAX),
                        snapshot.printers.len(),
                        "Drucker",
                    )
                } else {
                    "Druckerliste nicht verfügbar".into()
                },
                printer_job_count_label(snapshot.job_count)
            )
        } else {
            printer_service_message(snapshot.service).into()
        },
        row_w,
        None,
        false,
        Some(niwoe_ui::effect::Symbol::Printer),
        None,
        ctx.pal,
    )];
    if snapshot.service == PrinterServiceState::Running {
        rows.push(settings_text_row(
            "Standard-Drucker",
            if snapshot.default_available {
                snapshot
                    .default_printer
                    .as_deref()
                    .unwrap_or("Kein Standard-Drucker gemeldet")
            } else {
                "Standard-Drucker nicht verfügbar"
            },
            row_w,
            None,
            false,
            None,
            None,
            ctx.pal,
        ));
        if snapshot.printers.is_empty() {
            rows.push(settings_text_row(
                if snapshot.list_available {
                    "Keine Drucker eingerichtet"
                } else {
                    "Druckerliste nicht verfügbar"
                },
                if snapshot.list_available {
                    "Der Druckdienst hat keine eingerichteten Drucker gemeldet."
                } else {
                    "Die Liste der eingerichteten Drucker konnte nicht gelesen werden."
                },
                row_w,
                None,
                false,
                None,
                None,
                ctx.pal,
            ));
        } else {
            let half_w = (row_w - c.option_gap) / 2;
            let mut pending = snapshot.printers.iter().take(PRINTER_MAX);
            loop {
                let pair: Vec<_> = pending
                    .by_ref()
                    .take(2)
                    .map(|printer| printer_device_row(printer, half_w, ctx.pal))
                    .collect();
                if pair.is_empty() {
                    break;
                }
                rows.push(Box::new(Container::row(c.option_gap, pair)));
            }
        }
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Drucker",
        "Druckdienste, Warteschlangen und konfigurierte Geräte.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}
