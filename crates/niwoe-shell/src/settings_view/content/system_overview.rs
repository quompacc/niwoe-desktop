fn build_system_overview_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let row_w = settings_group_inner_width(ctx.content_w);
    let mut rows: Vec<Box<dyn Widget>> = ctx
        .system_info
        .rows()
        .iter()
        .map(|(label, value)| {
            Box::new(SystemInfoRow {
                label: (*label).into(),
                value: (*value).into(),
                row_width: row_w,
            }) as Box<dyn Widget>
        })
        .collect();
    rows.insert(0, Box::new(IntroductionButton(row_w)));
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Geräteinformationen",
        "Grundlegende Daten dieser NIWOE-Installation.",
        Box::new(Container::column(4, rows)),
    )
}

/// The overview has one primary action; Enter activates the visibly focused
/// shared component. The same id dispatches pointer activation.
struct IntroductionButton(i32);
impl IntroductionButton {
    fn component(&self) -> niwoe_ui::widget::Component<'static> {
        use niwoe_ui::widget::{Component, ComponentKind};
        let mut component = Component::new(
            ComponentKind::Button,
            "Einführung öffnen / fortsetzen · Enter",
            self.0,
        );
        component.state.focused = true;
        component
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
