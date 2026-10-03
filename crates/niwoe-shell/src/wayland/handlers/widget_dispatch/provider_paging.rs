impl NiwoeShell {
    fn page_providers(&mut self, qh: &QueueHandle<NiwoeShell>, forward: bool) {
        use crate::settings_view::SettingsCategory;
        let category = self.effective_settings_category();
        let count = match category {
            SettingsCategory::Network => self
                .network_profiles
                .len()
                .min(8)
                .max(self.wifi_networks.len().min(10)),
            SettingsCategory::Sound => self
                .audio_snapshot
                .outputs
                .len()
                .max(self.audio_snapshot.inputs.len())
                .min(8),
            SettingsCategory::Bluetooth => self.bluetooth_snapshot.devices.len().min(8),
            _ => return,
        };
        let (_, height) = self.launcher_content_size();
        let content_height = height.saturating_sub(
            (niwoe_tokens::Settings::DEFAULT.header_height
                + niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height) as u32,
        );
        let slots = crate::settings_view::provider_page_size(category, content_height);
        let last = count.div_ceil(slots).max(1) - 1;
        let current = self.provider_page.min(last);
        self.provider_page = if forward {
            current.saturating_add(1).min(last)
        } else {
            current.saturating_sub(1)
        };
        self.control_center_nav.widget_focus = Some(if self.provider_page > 0 {
            "provider-page-previous"
        } else {
            "provider-page-next"
        });
        self.draw_launcher(qh, RepaintReason::Pointer);
    }
}
