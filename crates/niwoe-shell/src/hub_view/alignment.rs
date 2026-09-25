fn centered_baseline(rect: Rect, size: f32) -> i32 {
    let (ascent, descent) = niwoe_ui::effect::ui_line_metrics(size);
    (rect.y as f32 + (rect.height as f32 + ascent + descent) / 2.0).round() as i32
}

fn paint_text_left_centered(
    pm: &mut tiny_skia::PixmapMut<'_>,
    text: &str,
    x: i32,
    rect: Rect,
    size: f32,
    color: Color,
) {
    paint_text(pm, text, x, centered_baseline(rect, size), size, color);
}

fn paint_text_centered(
    pm: &mut tiny_skia::PixmapMut<'_>,
    text: &str,
    rect: Rect,
    size: f32,
    color: Color,
) {
    let width = niwoe_ui::effect::measure_text(text, size).0;
    paint_text_left_centered(pm, text, rect.x + (rect.width - width) / 2, rect, size, color);
}

fn paint_text_right_centered(
    pm: &mut tiny_skia::PixmapMut<'_>,
    text: &str,
    right: i32,
    rect: Rect,
    size: f32,
    color: Color,
) {
    let width = niwoe_ui::effect::measure_text(text, size).0;
    paint_text_left_centered(pm, text, right - width, rect, size, color);
}
