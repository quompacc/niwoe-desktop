#[derive(Debug, PartialEq, Eq)]
enum ScreenshotOutputRoute {
    Capture,
    Wait,
    Unavailable,
}

fn screenshot_output_route(
    current: &str,
    requested: Option<&str>,
    primary: Option<&str>,
    available: &[&str],
) -> ScreenshotOutputRoute {
    let Some(target) = requested.or(primary) else {
        return ScreenshotOutputRoute::Unavailable;
    };
    if !available.contains(&target) {
        return ScreenshotOutputRoute::Unavailable;
    }
    if target == current {
        ScreenshotOutputRoute::Capture
    } else {
        ScreenshotOutputRoute::Wait
    }
}

#[cfg(test)]
mod screenshot_output_tests {
    use super::{screenshot_output_route as route, ScreenshotOutputRoute::*};

    #[test]
    fn secondary_request_survives_primary_repaint_and_vice_versa() {
        let outputs = ["drm-0", "drm-1"];
        assert_eq!(route("drm-0", Some("drm-1"), Some("drm-0"), &outputs), Wait);
        assert_eq!(
            route("drm-1", Some("drm-1"), Some("drm-0"), &outputs),
            Capture
        );
        assert_eq!(route("drm-1", Some("drm-0"), Some("drm-0"), &outputs), Wait);
        assert_eq!(
            route("drm-0", Some("drm-0"), Some("drm-0"), &outputs),
            Capture
        );
    }

    #[test]
    fn unspecified_output_uses_primary_even_if_secondary_renders_first() {
        let outputs = ["drm-0", "drm-1"];
        assert_eq!(route("drm-1", None, Some("drm-0"), &outputs), Wait);
        assert_eq!(route("drm-0", None, Some("drm-0"), &outputs), Capture);
        assert_eq!(route("drm-0", None, Some("drm-1"), &outputs), Wait);
        assert_eq!(route("drm-1", None, Some("drm-1"), &outputs), Capture);
    }

    #[test]
    fn removed_or_unknown_output_never_falls_back_to_another_screen() {
        assert_eq!(
            route("drm-0", Some("drm-1"), Some("drm-0"), &["drm-0"]),
            Unavailable
        );
        assert_eq!(
            route("drm-0", Some("unknown"), Some("drm-0"), &["drm-0"]),
            Unavailable
        );
        assert_eq!(route("drm-0", None, None, &[]), Unavailable);
    }
}
