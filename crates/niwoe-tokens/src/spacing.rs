//! Shared logical spacing and control geometry, identical in both themes.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spacing {
    pub xs: i32,
    pub sm: i32,
    pub md: i32,
    pub lg: i32,
    pub xl: i32,
    pub xxl: i32,
}

impl Spacing {
    pub const DEFAULT: Self = Self {
        xs: 4,
        sm: 8,
        md: 12,
        lg: 16,
        xl: 24,
        xxl: 32,
    };
}

/// Shared component sizes. Width is supplied by the containing layout.
pub struct Controls;
impl Controls {
    pub const MIN_HEIGHT: i32 = 32;
    pub const FORM_HEIGHT: i32 = 36;
    pub const BORDER: i32 = 1;
    pub const FOCUS_WIDTH: i32 = 2;
    pub const FOCUS_INSET: i32 = 4;
    pub const TRACK_HEIGHT: i32 = 4;
    pub const THUMB_SIZE: i32 = 16;
    pub const CARD_HEIGHT: i32 = 72;
}
