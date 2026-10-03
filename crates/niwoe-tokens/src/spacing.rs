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
    /// Existing icon footer geometry; retained while the panel/launcher layout
    /// is migrated in later phases.
    pub const ICON_BUTTON_SIZE: i32 = 48;
    pub const ICON_LABEL_PADDING: i32 = Spacing::DEFAULT.sm;
    pub const ICON_LABEL_BASELINE: i32 = Spacing::DEFAULT.sm;
    pub const ARMED_LABEL_BASELINE_OFFSET: i32 = 2;
    pub const TILE_BASE_SIZE: i32 = 96;
    pub const TILE_LABEL_BASELINE: i32 = Spacing::DEFAULT.md;
    pub const TILE_ICON_CENTER_FRACTION: f32 = 0.35;
    pub const PROGRESS_RING_INSET: i32 = 3;
    pub const PROGRESS_RING_WIDTH: f32 = 3.0;
    pub const PROGRESS_RING_SEGMENTS: usize = 64;
}
