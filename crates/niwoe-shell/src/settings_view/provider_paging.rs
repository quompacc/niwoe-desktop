/// Shared, transient paging for bounded provider lists. Device indices remain
/// indices into the original snapshot, including on later pages.
pub(crate) fn provider_page_size(category: SettingsCategory, content_height: u32) -> usize {
    let c = SETTINGS_CHROME;
    let text = niwoe_tokens::Controls::TEXT_PAIR_HEIGHT;
    let control = niwoe_tokens::Controls::MIN_HEIGHT;
    let section = c.group_gap + c.sidebar_section_height;
    // Reserve the largest summary for the category and the navigation footer.
    let reserved = match category {
        SettingsCategory::Network => text * 2 + section + control + c.option_gap * 4,
        SettingsCategory::Sound => text * 2 + control * 2 + section + c.option_gap * 5,
        SettingsCategory::Bluetooth => control * 2 + section + c.option_gap * 3,
        _ => return 1,
    };
    let room = settings_group_body_height(content_height).saturating_sub(reserved as u32);
    ((room + c.option_gap as u32) / (text + c.option_gap) as u32).max(1) as usize
}

pub(crate) fn provider_page_range(
    page: usize,
    slots: usize,
    count: usize,
) -> std::ops::Range<usize> {
    let slots = slots.max(1);
    let page = page.min(count.div_ceil(slots).max(1) - 1);
    let start = (page * slots).min(count);
    start..(start + slots).min(count)
}

fn provider_navigation(
    width: i32,
    page: usize,
    slots: usize,
    count: usize,
    label: String,
) -> Option<Box<dyn Widget>> {
    let pages = count.div_ceil(slots.max(1)).max(1);
    if pages == 1 {
        return (count > 0)
            .then(|| Box::new(SettingsPageLabel { label, width }) as Box<dyn Widget>);
    }
    let page = page.min(pages - 1);
    let c = SETTINGS_CHROME;
    let mut children: Vec<Box<dyn Widget>> = Vec::new();
    for (id, text, disabled) in [
        ("provider-page-previous", "Zurück", page == 0),
        ("provider-page-next", "Weiter", page + 1 >= pages),
    ] {
        children.push(Box::new(SettingsControl {
            id: Some(id),
            kind: niwoe_ui::widget::ComponentKind::Button,
            disabled,
            label: text.into(),
            selected: false,
            width: c.wallpaper_control_width,
        }));
    }
    children.push(Box::new(SettingsPageLabel {
        label: format!("Seite {} von {} · {label}", page + 1, pages),
        width: (width - c.wallpaper_control_width * 2 - c.option_gap * 2).max(0),
    }));
    Some(Box::new(Container::row(c.option_gap, children)))
}

fn provider_count_label(count: usize, total: usize, label: &str) -> String {
    if count < total {
        format!("{count} von {total} {label}")
    } else {
        format!("{count} {label}")
    }
}

fn network_count_label(count: usize, total: Option<usize>, wifi: bool) -> String {
    match total {
        None if wifi => "WLAN-Anzahl unbekannt".into(),
        None => "Profilanzahl unbekannt".into(),
        Some(1) if wifi => provider_count_label(count, 1, "WLAN"),
        Some(1) => provider_count_label(count, 1, "Profil"),
        Some(total) => provider_count_label(count, total, if wifi { "WLANs" } else { "Profile" }),
    }
}

fn printer_job_count_label(count: Option<usize>) -> String {
    match count {
        Some(1) => "1 Druckauftrag".into(),
        Some(count) => format!("{count} Druckaufträge"),
        None => "Druckaufträge nicht verfügbar".into(),
    }
}

#[cfg(test)]
mod network_count_tests {
    use super::network_count_label;

    #[test]
    fn unknown_network_counts_are_not_zero_and_known_counts_keep_their_limits() {
        for (wifi, unknown, empty, single, capped) in [
            (
                false,
                "Profilanzahl unbekannt",
                "0 Profile",
                "1 Profil",
                "8 von 12 Profile",
            ),
            (
                true,
                "WLAN-Anzahl unbekannt",
                "0 WLANs",
                "1 WLAN",
                "8 von 12 WLANs",
            ),
        ] {
            assert_eq!(network_count_label(0, None, wifi), unknown);
            assert_eq!(network_count_label(0, Some(0), wifi), empty);
            assert_eq!(network_count_label(1, Some(1), wifi), single);
            assert_eq!(network_count_label(8, Some(12), wifi), capped);
        }
    }
}
