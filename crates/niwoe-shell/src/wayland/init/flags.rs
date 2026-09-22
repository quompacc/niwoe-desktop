pub(super) fn enabled(name: &str) -> bool {
    niwoe_config::environment::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_flag_is_disabled() {
        assert!(!enabled("NIWOE_TEST_FLAG_THAT_MUST_NOT_EXIST"));
    }
}
