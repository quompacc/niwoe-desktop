impl NiwoeShell {
    fn page_display(&mut self, qh: &QueueHandle<NiwoeShell>, modes: bool, forward: bool) {
        let count = self.output_workspaces.len().min(16);
        if count == 0 {
            return;
        }
        self.display_pages.output = self.display_pages.output.min(count - 1);
        let (current, pages) = if modes {
            let (_, height) = self.launcher_content_size();
            let content_height = height.saturating_sub(
                (niwoe_tokens::Settings::DEFAULT.header_height
                    + niwoe_tokens::ControlCenter::DEFAULT.config_tabs_height)
                    as u32,
            );
            let indices = crate::settings_view::display_mode_indices(
                &self.output_workspaces[self.display_pages.output],
            );
            let slots = crate::settings_view::display_mode_page_size(content_height);
            (
                self.display_pages.modes,
                indices.len().div_ceil(slots).max(1),
            )
        } else {
            (self.display_pages.output, count)
        };
        let next = if forward {
            current.saturating_add(1).min(pages - 1)
        } else {
            current.saturating_sub(1)
        };
        if modes {
            self.display_pages.modes = next;
        } else {
            self.display_pages.output = next;
            self.display_pages.modes = 0;
            self.display_mode_dropdown_open = None;
        }
        self.control_center_nav.widget_focus = Some(match (modes, next > 0) {
            (true, true) => "display-modes-previous",
            (true, false) => "display-modes-next",
            (false, true) => "display-outputs-previous",
            (false, false) => "display-outputs-next",
        });
        self.draw_launcher(qh, RepaintReason::Pointer);
    }
}
