impl NiwoeShell {
    fn page_default_apps(&mut self, qh: &QueueHandle<Self>, forward: bool) {
        let (Some(category), Some(index)) = (
            self.default_apps_picker_open,
            self.default_apps_index.as_ref(),
        ) else {
            return;
        };
        let count = index
            .apps_for_mime(category.representative_mime())
            .len()
            .min(crate::default_apps::MAX_APPS_PER_CATEGORY);
        let (_, height) = self.launcher_content_size();
        let content_height = height.saturating_sub(
            (niwoe_tokens::Settings::DEFAULT.header_height
                + crate::settings_view::category_navigation_height(crate::settings_view::SettingsCategory::DefaultApps)) as u32,
        );
        let pages = count
            .div_ceil(crate::settings_view::default_apps_page_size(content_height))
            .max(1);
        self.default_apps_page = if forward {
            self.default_apps_page.saturating_add(1).min(pages - 1)
        } else {
            self.default_apps_page.saturating_sub(1)
        };
        self.control_center_nav.widget_focus = Some(if self.default_apps_page > 0 {
            "default-apps-previous"
        } else {
            "default-apps-next"
        });
        self.draw_launcher(qh, RepaintReason::Pointer);
    }
}
