/// Transient page positions, never written to the user's configuration.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DisplayPages {
    pub output: usize,
    pub modes: usize,
}

pub(crate) fn display_mode_page_size(content_height: u32) -> usize {
    let c = SETTINGS_CHROME;
    // Identity, mode chooser, settings row, two navigation rows and their gaps.
    let reserved = niwoe_tokens::Controls::TEXT_PAIR_HEIGHT * 3
        + niwoe_tokens::Controls::MIN_HEIGHT * 2
        + c.option_gap * 5;
    let room = settings_group_body_height(content_height).saturating_sub(reserved as u32);
    ((room + c.option_gap as u32)
        / (niwoe_tokens::Controls::TEXT_PAIR_HEIGHT + c.option_gap) as u32)
        .max(1) as usize
}

pub(crate) fn display_mode_indices(output: &OutputWorkspaceState) -> Vec<usize> {
    output
        .modes
        .iter()
        .enumerate()
        .take(DISPLAY_MODE_OPTION_MAX)
        .filter(|(_, mode)| mode.width > 0 && mode.height > 0)
        .map(|(index, _)| index)
        .collect()
}

fn display_navigation(
    width: i32,
    page: usize,
    pages: usize,
    modes: bool,
    count: usize,
    total: usize,
) -> Box<dyn Widget> {
    let c = SETTINGS_CHROME;

    let mut children: Vec<Box<dyn Widget>> = Vec::new();
    for (id, label, disabled) in [
        (
            if modes {
                "display-modes-previous"
            } else {
                "display-outputs-previous"
            },
            "Zurück",
            page == 0,
        ),
        (
            if modes {
                "display-modes-next"
            } else {
                "display-outputs-next"
            },
            "Weiter",
            page + 1 >= pages,
        ),
    ] {
        children.push(Box::new(SettingsControl {
            id: Some(id),
            kind: niwoe_ui::widget::ComponentKind::Button,
            disabled,
            label: label.into(),
            selected: false,
            width: c.wallpaper_control_width,
        }));
    }

    children.push(Box::new(SettingsPageLabel {
        label: if modes {
            format!(
                "Seite {} von {} · {} {}",
                page + 1,
                pages,
                count,
                display_mode_count_label(count, total)
            )
        } else {
            format!("Bildschirm {} von {}", page + 1, pages)
        },
        width: (width - c.wallpaper_control_width * 2 - c.option_gap * 2).max(0),
    }));
    Box::new(Container::row(c.option_gap, children))
}

fn display_mode_count_label(count: usize, total: usize) -> String {
    if count < total {
        format!("von {total} Moduseinträgen")
    } else {
        if count == 1 { "Modus" } else { "Modi" }.into()
    }
}

#[cfg(test)]
mod display_limit_tests {
    use super::*;

    #[test]
    fn mode_limit_preserves_indices_and_exposes_omitted_entries() {
        let output = OutputWorkspaceState {
            modes: (0..300)
                .map(|index| OutputModeState {
                    width: if index == 3 { 0 } else { 1920 },
                    height: 1080,
                    refresh_millihz: Some(60000),
                    current: index == 0,
                    preferred: index == 0,
                })
                .collect(),
            ..Default::default()
        };
        let indices = display_mode_indices(&output);
        assert_eq!(indices.len(), 255);
        assert_eq!(indices.last(), Some(&255));
        assert!(!indices.contains(&3));
        assert_eq!(
            display_mode_count_label(indices.len(), 299),
            "von 299 Moduseinträgen"
        );
        assert_eq!(display_mode_count_label(1, 1), "Modus");
        assert_eq!(display_mode_count_label(36, 36), "Modi");
    }
}
