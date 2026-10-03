use niwoe_tokens::{Controls, Palette, Radius, Spacing, Typography};
use niwoe_ui::{
    effect::{paint_text_pair, text_pair_layout, ui_line_metrics},
    Rect,
};
use tiny_skia::Pixmap;

#[test]
fn two_line_roles_separate_complete_font_boxes_and_stay_inside_rows() {
    for height in [Controls::TEXT_ROW_HEIGHT, 52, 64] {
        let area = Rect {
            x: 16,
            y: 8,
            width: 240,
            height,
        };
        let layout = text_pair_layout(area);
        let (pa, pd) = ui_line_metrics(Typography::DEFAULT.body_size as f32);
        let (sa, sd) = ui_line_metrics(Typography::DEFAULT.caption_size as f32);
        assert!(layout.primary_baseline as f32 - pa >= area.y as f32);
        let primary_bottom = layout.primary_baseline as f32 - pd;
        let secondary_top = layout.secondary_baseline as f32 - sa;
        assert!(secondary_top - primary_bottom >= Spacing::DEFAULT.xs as f32);
        assert!(layout.secondary_baseline as f32 - sd <= (area.y + area.height) as f32);
    }
}

#[test]
fn accented_long_labels_are_truncated_to_their_own_text_column() {
    let mut image = Pixmap::new(320, 64).unwrap();
    let area = Rect {
        x: 16,
        y: 8,
        width: 180,
        height: Controls::TEXT_ROW_HEIGHT,
    };
    paint_text_pair(
        &mut image.as_mut(),
        area,
        "ÄÖÜ Web-Browser mit einem sehr langen Namen",
        "Größere Anwendung mit Unterlängen gypq und sehr langem Namen",
        Palette::DARK.text,
        Palette::DARK.text_dim,
    );
    assert!(image.data().as_chunks::<4>().0.iter().any(|p| p[3] != 0));
    for y in 0..image.height() {
        for x in (area.x + area.width + Spacing::DEFAULT.xs) as u32..image.width() {
            assert_eq!(
                image.pixel(x, y).unwrap().alpha(),
                0,
                "text must leave symbol column free"
            );
        }
    }
    niwoe_ui::effect::paint_focus(
        &mut image.as_mut(),
        area,
        Palette::DARK.border_focus(),
        Radius::DEFAULT.sm,
    );
}
