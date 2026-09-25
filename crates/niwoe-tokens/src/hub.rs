//! Geometry and opacity tokens for the room-first Hub.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hub {
    pub width: i32,
    pub height: i32,
    pub outer_pad: i32,
    pub header_height: i32,
    pub close_width: i32,
    pub manage_width: i32,
    pub room_columns: i32,
    pub room_height: i32,
    pub lower_height: i32,
    pub lower_columns: i32,
    pub section_gap: i32,
    pub card_gap: i32,
    pub card_pad: i32,
    pub room_icon_size: i32,
    pub status_dot_size: i32,
    pub search_height: i32,
    pub card_alpha: u8,
    pub quiet_alpha: u8,
}

impl Hub {
    pub const DEFAULT: Hub = Hub {
        width: 1400,
        height: 832,
        outer_pad: 32,
        header_height: 236,
        close_width: 112,
        manage_width: 152,
        room_columns: 4,
        room_height: 232,
        lower_height: 220,
        lower_columns: 3,
        section_gap: 24,
        card_gap: 16,
        card_pad: 16,
        room_icon_size: 44,
        status_dot_size: 8,
        search_height: 52,
        card_alpha: 132,
        quiet_alpha: 92,
    };
}

impl Default for Hub {
    fn default() -> Self {
        Self::DEFAULT
    }
}
