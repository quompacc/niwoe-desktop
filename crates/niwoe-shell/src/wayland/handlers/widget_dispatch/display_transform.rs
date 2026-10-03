/// Accept both the optimistic config value and the compositor's Debug snapshot.
/// A mirrored or unknown transform returns to normal on the existing four-step control.
fn next_output_transform(current: Option<&str>) -> Option<&'static str> {
    match current {
        None | Some("" | "Normal") => Some("90"),
        Some("90" | "_90") => Some("180"),
        Some("180" | "_180") => Some("270"),
        _ => None,
    }
}

#[cfg(test)]
mod display_transform_tests {
    use super::next_output_transform;

    #[test]
    fn rotation_cycle_survives_compositor_snapshot_after_every_reload() {
        let snapshots = ["Normal", "_90", "_180", "_270", "Normal"];
        let config = [None, Some("90"), Some("180"), Some("270"), None];
        for index in 0..4 {
            assert_eq!(
                next_output_transform(Some(snapshots[index])),
                config[index + 1]
            );
            assert_eq!(next_output_transform(config[index]), config[index + 1]);
        }
        assert_eq!(next_output_transform(Some("")), Some("90"));
    }

    #[test]
    fn mirrored_and_unknown_snapshots_reset_to_normal() {
        for value in [
            "Flipped",
            "Flipped90",
            "Flipped180",
            "Flipped270",
            "unexpected",
        ] {
            assert_eq!(next_output_transform(Some(value)), None);
        }
    }
}
