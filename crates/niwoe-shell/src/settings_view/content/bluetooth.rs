fn build_bluetooth_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let snapshot = ctx.bluetooth_snapshot;
    let slots = provider_page_size(SettingsCategory::Bluetooth, ctx.content_h);
    let count = snapshot.devices.len().min(BT_DEVICE_IDS.len());
    let range = provider_page_range(ctx.provider_page, slots, count);
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    if !snapshot.adapter_present {
        rows.push(settings_text_row(
            "Bluetooth nicht verfügbar",
            "Kein Adapter erkannt oder Bluetooth-Status nicht lesbar.",
            row_w,
            None,
            false,
            Some(niwoe_ui::effect::Symbol::Bluetooth),
            None,
            ctx.pal,
        ));
    } else {
        rows.push(bluetooth_adapter_controls(snapshot));
        rows.push(Box::new(SidebarSectionLabel {
            text: "GERÄTE",
            width: row_w,
            pad_top: c.group_gap,
        }));
        if !snapshot.powered {
            rows.push(settings_text_row(
                "Bluetooth ist ausgeschaltet",
                "Zum Suchen und Verbinden zuerst Bluetooth einschalten.",
                row_w,
                None,
                false,
                None,
                None,
                ctx.pal,
            ));
        } else if snapshot.devices.is_empty() {
            rows.push(settings_text_row(
                if snapshot.device_status_available {
                    "Keine Geräte verfügbar"
                } else {
                    "Gerätestatus nicht verfügbar"
                },
                if snapshot.device_status_available {
                    "Mit „Geräte suchen“ nach Geräten in der Nähe suchen."
                } else {
                    "Geräteliste oder Kopplungsstatus konnten nicht gelesen werden."
                },
                row_w,
                None,
                false,
                None,
                None,
                ctx.pal,
            ));
        } else {
            for (index, device) in snapshot
                .devices
                .iter()
                .take(BT_DEVICE_IDS.len())
                .enumerate()
                .skip(range.start)
                .take(range.len())
            {
                rows.push(bluetooth_device_row(index, device, row_w, ctx.pal));
            }
            if let Some(footer) = provider_navigation(
                row_w,
                ctx.provider_page,
                slots,
                count,
                provider_count_label(count, snapshot.devices.len(), "Geräte"),
            ) {
                rows.push(footer);
            }
        }
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Bluetooth-Geräte",
        "Adapterstatus, Suche und gekoppelte Geräte.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}
