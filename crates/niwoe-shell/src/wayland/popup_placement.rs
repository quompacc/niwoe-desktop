//! Trigger-centered popup placement in the panel's logical output coordinates.
use super::NiwoeShell;

fn margin(output_width: i32, center: i32, card_width: i32, shadow: i32) -> i32 {
    (center - card_width / 2).clamp(0, (output_width - card_width).max(0)) - shadow
}

impl NiwoeShell {
    pub(crate) fn workspace_popup_left_margin(&self) -> i32 {
        let trigger = self
            .panel_state
            .clicks
            .iter()
            .find(|zone| matches!(zone.action, super::ClickAction::ToggleWorkspacePopup));
        trigger.map_or(crate::WORKSPACE_POPUP_LEFT_MARGIN, |zone| {
            margin(
                self.width as i32,
                zone.rect.x + zone.rect.w / 2,
                crate::WORKSPACE_POPUP_WIDTH as i32,
                crate::POPUP_SHADOW_PAD,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::margin;
    #[test]
    fn popup_tracks_trigger_and_clamps_card_not_its_transparent_shadow() {
        let width = crate::WORKSPACE_POPUP_WIDTH as i32;
        let pad = crate::POPUP_SHADOW_PAD;
        for output in [1920, 1366, 1280, 960] {
            assert_eq!(margin(output, 568, width, pad) + pad + width / 2, 568);
            assert_eq!(margin(output, 4, width, pad) + pad, 0);
            assert_eq!(margin(output, output - 4, width, pad) + pad + width, output);
        }
    }
}
