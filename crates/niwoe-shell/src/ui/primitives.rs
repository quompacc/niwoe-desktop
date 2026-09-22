use niwoe_config::ThemeConfig;

use crate::{Painter, Rect};

pub enum ActiveIndicatorEdge {
    Top,
}

pub fn draw_active_indicator(
    painter: &mut Painter<'_>,
    rect: Rect,
    edge: ActiveIndicatorEdge,
    theme: &ThemeConfig,
) {
    const THICKNESS: i32 = niwoe_tokens::Controls::FOCUS_WIDTH;
    let bar = match edge {
        ActiveIndicatorEdge::Top => Rect {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: THICKNESS.min(rect.h),
        },
    };
    painter.rect(bar, theme.colors.accent);
}
