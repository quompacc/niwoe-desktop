fn build_system_overview_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let row_w = settings_group_inner_width(ctx.content_w);
    let values = ctx.system_info.rows().into_iter().map(|(label, value)| {
        (
            label.to_string(),
            if value.is_empty() || value == "—" {
                "Nicht verfügbar".into()
            } else {
                value.to_string()
            },
        )
    });
    let rows = vec![
        readonly_settings_rows(values, row_w, None, ctx.pal),
        Box::new(IntroductionButton(row_w)) as Box<dyn Widget>,
    ];
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Geräteinformationen",
        "Grundlegende Daten dieser NIWOE-Installation.",
        Box::new(Container::column(SETTINGS_CHROME.option_gap, rows)),
    )
}

/// Uses the same focus traversal and action as every other settings control.
struct IntroductionButton(i32);
impl IntroductionButton {
    fn component(&self) -> niwoe_ui::widget::Component<'static> {
        use niwoe_ui::widget::{Component, ComponentKind};
        Component::new(
            ComponentKind::Button,
            "Einführung öffnen / fortsetzen",
            self.0,
        )
    }
}
impl Widget for IntroductionButton {
    fn id(&self) -> Option<&'static str> {
        Some("first-run-open")
    }
    fn style(&self) -> WidgetStyle {
        self.component().style()
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        self.component().paint(area, canvas, theme, state);
    }
}
