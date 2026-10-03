fn display_mode_label(mode: &OutputModeState) -> String {
    let refresh = mode
        .refresh_millihz
        .map(|millihz| format!("{:.2} Hz", millihz as f64 / 1000.0).replace('.', ","))
        .unwrap_or_else(|| "Bildwiederholrate unbekannt".to_string());
    format!("{} × {} · {}", mode.width, mode.height, refresh)
}

fn selected_display_mode(output: &OutputWorkspaceState) -> Option<&OutputModeState> {
    output
        .modes
        .iter()
        .find(|mode| mode.current)
        .or_else(|| {
            output.modes.iter().find(|mode| {
                mode.width == output.width
                    && mode.height == output.height
                    && mode.refresh_millihz == output.refresh_millihz
            })
        })
        .or_else(|| {
            output
                .modes
                .iter()
                .find(|mode| mode.width > 0 && mode.height > 0)
        })
}

#[allow(clippy::too_many_arguments)] // Private adapter for the common row and its cached icons.
fn settings_text_row(
    title: impl Into<Box<str>>,
    subtitle: impl Into<Box<str>>,
    width: i32,
    id: Option<&'static str>,
    selected: bool,
    leading: Option<niwoe_ui::effect::Symbol>,
    trailing: Option<niwoe_ui::effect::Symbol>,
    pal: &niwoe_ui::style::Palette,
) -> Box<dyn Widget> {
    let mut row = niwoe_ui::widget::TextRow::new(title, subtitle, width);
    row.id = id;
    row.selected = selected;
    let side = niwoe_tokens::Spacing::DEFAULT.xl as u32;
    row.leading =
        leading.and_then(|symbol| niwoe_ui::effect::symbol_icon(symbol, pal.text_dim, side));
    row.trailing = trailing.and_then(|symbol| {
        niwoe_ui::effect::symbol_icon(
            symbol,
            if selected { pal.accent } else { pal.text_dim },
            side,
        )
    });
    Box::new(row)
}
