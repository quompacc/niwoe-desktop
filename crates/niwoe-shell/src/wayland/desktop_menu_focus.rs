//! Focus broadcasts also name layer surfaces. Only actual app windows dismiss
//! the desktop menu; a click on the menu's own layer must keep it mapped.
pub(super) fn is_app_focus<'a>(id: &str, mut window_ids: impl Iterator<Item = &'a str>) -> bool {
    window_ids.any(|window_id| window_id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menu_layer_focus_does_not_count_as_app_focus() {
        let actual_windows = ["wl_surface@27[18]", "x11:4194305"];
        assert!(is_app_focus(
            actual_windows[0],
            actual_windows.iter().copied()
        ));
        assert!(is_app_focus(
            actual_windows[1],
            actual_windows.iter().copied()
        ));
        assert!(!is_app_focus(
            "wl_surface@29[4]",
            actual_windows.iter().copied()
        ));
        assert!(!is_app_focus("wl_surface@29[4]", std::iter::empty()));
    }
}
