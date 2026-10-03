fn build_network_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;
    let row_w = settings_group_inner_width(ctx.content_w);
    let summary_w = (row_w - c.option_gap) / 2;
    let summary: Vec<Box<dyn Widget>> = ctx
        .network_state
        .settings_rows()
        .into_iter()
        .map(|(label, value)| {
            settings_text_row(label, value, summary_w, None, false, None, None, ctx.pal)
        })
        .collect();
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    let mut pending = summary.into_iter();
    loop {
        let pair: Vec<_> = pending.by_ref().take(2).collect();
        if pair.is_empty() {
            break;
        }
        rows.push(Box::new(Container::row(c.option_gap, pair)));
    }
    let slots = provider_page_size(SettingsCategory::Network, ctx.content_h);
    let profiles = ctx.network_profiles.len().min(NETWORK_PROFILE_IDS.len());
    let networks = ctx.wifi_networks.len().min(WIFI_NETWORK_IDS.len());
    let count = profiles.max(networks);
    let range = provider_page_range(ctx.provider_page, slots, count);
    let mut profile_rows: Vec<Box<dyn Widget>> = Vec::new();
    let mut wifi_rows: Vec<Box<dyn Widget>> = Vec::new();
    profile_rows.push(Box::new(SidebarSectionLabel {
        text: "GESPEICHERTE VERBINDUNGEN",
        width: summary_w,
        pad_top: c.group_gap,
    }));
    if ctx.network_profiles.is_empty() {
        profile_rows.push(settings_text_row(
            if ctx.network_list_status.profiles_available {
                "Keine Verbindungen gespeichert"
            } else {
                "Profilliste nicht verfügbar"
            },
            if ctx.network_list_status.profiles_available {
                "Es wurden keine gespeicherten Verbindungen gemeldet."
            } else {
                "Die gespeicherten Verbindungen konnten nicht gelesen werden."
            },
            summary_w,
            None,
            false,
            None,
            None,
            ctx.pal,
        ));
    } else {
        for (index, profile) in ctx
            .network_profiles
            .iter()
            .take(NETWORK_PROFILE_IDS.len())
            .enumerate()
            .skip(range.start)
            .take(range.len())
        {
            profile_rows.push(network_profile_row(index, profile, summary_w, ctx.pal));
        }
    }
    wifi_rows.push(Box::new(SidebarSectionLabel {
        text: "WLAN-NETZWERKE",
        width: summary_w,
        pad_top: c.group_gap,
    }));
    if ctx.wifi_networks.is_empty() {
        wifi_rows.push(settings_text_row(
            if ctx.network_list_status.wifi_available {
                "Keine WLAN-Netzwerke gemeldet"
            } else {
                "WLAN-Liste nicht verfügbar"
            },
            if ctx.network_list_status.wifi_available {
                "Die letzte Abfrage hat keine sichtbaren WLANs gemeldet."
            } else {
                "Die verfügbaren WLANs konnten nicht gelesen werden."
            },
            summary_w,
            None,
            false,
            Some(niwoe_ui::effect::Symbol::Network),
            None,
            ctx.pal,
        ));
    } else {
        for (index, network) in ctx
            .wifi_networks
            .iter()
            .take(WIFI_NETWORK_IDS.len())
            .enumerate()
            .skip(range.start)
            .take(range.len())
        {
            wifi_rows.push(wifi_row(index, network, summary_w, ctx.pal));
        }
    }
    rows.push(Box::new(Container::row(
        c.option_gap,
        vec![
            Box::new(Container::column(c.option_gap, profile_rows)),
            Box::new(Container::column(c.option_gap, wifi_rows)),
        ],
    )));
    if let Some(footer) = provider_navigation(
        row_w,
        ctx.provider_page,
        slots,
        count,
        format!(
            "{} · {}",
            network_count_label(
                profiles,
                ctx.network_list_status
                    .profiles_available
                    .then_some(ctx.network_profiles.len()),
                false,
            ),
            network_count_label(
                networks,
                ctx.network_list_status
                    .wifi_available
                    .then_some(ctx.wifi_networks.len()),
                true,
            )
        ),
    ) {
        rows.push(footer);
    }
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Verbindungen",
        "Aktiver Netzwerkstatus, gespeicherte Profile und verfügbare WLANs.",
        Box::new(Container::column(c.option_gap, rows)),
    )
}
