pub(crate) fn hit_room(x: i32, y: i32, width: u32, room_count: usize) -> Option<usize> {
    (0..room_count.min(H.room_columns as usize)).find(|&index| {
        let rect = room_rect(index, width);
        x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
    })
}

pub(crate) fn hit_close(x: i32, y: i32, width: u32) -> bool {
    let rect = Rect {
        x: width as i32 - H.outer_pad - H.close_width,
        y: H.outer_pad,
        width: H.close_width,
        height: Controls::MIN_HEIGHT,
    };
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}

pub(crate) fn hit_manage_rooms(x: i32, y: i32, width: u32) -> bool {
    let rect = Rect {
        x: width as i32 - H.outer_pad - H.close_width - H.card_gap - H.manage_width,
        y: H.outer_pad,
        width: H.manage_width,
        height: Controls::MIN_HEIGHT,
    };
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}
pub(crate) fn hit_recent(x: i32, y: i32, width: u32, height: u32) -> Option<usize> {
    let rect = lower_rect(0, width, height);
    let top = rect.y + S.xxl + S.lg;
    if x < rect.x || x >= rect.x + rect.width || y < top || y >= rect.y + rect.height {
        return None;
    }
    let index = ((y - top) / S.xxl) as usize;
    (index < 4).then_some(index)
}
pub(crate) fn hit_windows(x: i32, y: i32, width: u32, height: u32) -> bool {
    let rect = lower_rect(0, width, height);
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}
