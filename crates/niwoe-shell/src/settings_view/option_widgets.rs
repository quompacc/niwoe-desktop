struct SettingsControl {
    id: Option<&'static str>,
    kind: niwoe_ui::widget::ComponentKind,
    disabled: bool,
    label: Box<str>,
    selected: bool,
    width: i32,
}
impl Widget for SettingsControl {
    fn id(&self) -> Option<&'static str> {
        if self.disabled {
            None
        } else {
            self.id
        }
    }
    fn style(&self) -> WidgetStyle {
        niwoe_ui::widget::Component::new(self.kind, &self.label, self.width).style()
    }
    fn paint(&self, area: Rect, canvas: &mut PixmapMut<'_>, theme: &Theme, state: WidgetState) {
        let mut chip = niwoe_ui::widget::Component::new(self.kind, &self.label, self.width);
        chip.state.selected = self.selected;
        chip.state.disabled = self.disabled;
        chip.paint(area, canvas, theme, state);
    }
}
