//! Bounded introduction composition, shared by painting and input geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstRun {
    pub content_max_width: i32,
    pub column_gap: i32,
    pub exit_width: i32,
    pub primary_width: i32,
    pub information_height: i32,
}

impl FirstRun {
    pub const DEFAULT: Self = Self {
        content_max_width: 1280,
        column_gap: 32,
        exit_width: 220,
        primary_width: 200,
        information_height: 80,
    };
}

impl Default for FirstRun {
    fn default() -> Self {
        Self::DEFAULT
    }
}
