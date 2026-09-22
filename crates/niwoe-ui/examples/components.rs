//! Offline native component sheet. Same widgets and renderer as the shell.
//! Run: cargo run --release -p niwoe-ui --example components -- OUTPUT_DIR
use niwoe_tokens::{Controls, Spacing, Typography};
use niwoe_ui::{
    paint_text,
    widget::{Component, ComponentKind, ComponentState},
    Rect, Theme, WidgetState,
};
use tiny_skia::Pixmap;

fn main() {
    let out = std::env::args().nth(1).expect("output directory");
    std::fs::create_dir_all(&out).unwrap();
    let kinds = [
        ComponentKind::Surface,
        ComponentKind::Text,
        ComponentKind::Button,
        ComponentKind::Tab,
        ComponentKind::Chip,
        ComponentKind::Card,
        ComponentKind::Input,
        ComponentKind::Slider,
    ];
    let states = [
        "Normal",
        "Hover",
        "Fokus",
        "Gedrückt",
        "Deaktiviert",
        "Fehler",
    ];
    for (name, theme) in [("dark", Theme::DARK), ("light", Theme::LIGHT)] {
        for (width, height) in [(1366, 768), (1920, 1080)] {
            for scale in [1.0_f32, 1.5, 2.0] {
                for (wall, bg) in [
                    ("dark", Theme::DARK.palette.background),
                    ("light", Theme::LIGHT.palette.background),
                ] {
                    // A full sheet has a fixed logical viewport; physical output
                    // increases with scale, so this is not a small-output fit test.
                    let px = |v: i32| (v as f32 * scale).round() as i32;
                    let mut sheet = Pixmap::new(px(width) as u32, px(height) as u32).unwrap();
                    sheet.fill(tiny_skia::Color::from_rgba8(bg.r, bg.g, bg.b, bg.a));
                    let gap = Spacing::DEFAULT.lg;
                    let pad = Spacing::DEFAULT.xxl;
                    let mut canvas = sheet.as_mut();
                    let surface = Rect {
                        x: pad,
                        y: pad,
                        width: width - pad * 2,
                        height: height - pad * 2,
                    };
                    let path = niwoe_ui::rounded_rect_path(
                        Rect {
                            x: px(surface.x),
                            y: px(surface.y),
                            width: px(surface.width),
                            height: px(surface.height),
                        },
                        px(theme.radius.lg),
                    )
                    .unwrap();
                    // Match default glass solidity; keep this example coupled to
                    // the central token below (the config default uses it too).
                    let c = theme.palette.surface_overlay();
                    niwoe_ui::paint_fill(
                        &mut canvas,
                        &path,
                        niwoe_tokens::Color::rgba(
                            c.r,
                            c.g,
                            c.b,
                            niwoe_tokens::chrome::SURFACE_ALPHA,
                        ),
                    );
                    scaled_text(
                        &mut canvas,
                        scale,
                        "Komponenten · gleiche Widgets, gleiche Geometrie",
                        pad * 2,
                        pad * 2,
                        Typography::DEFAULT.display_size as f32,
                        theme.palette.text,
                    );
                    let label_w = pad * 4;
                    let col = (surface.width - pad * 2 - label_w) / states.len() as i32;
                    for (i, state) in states.iter().enumerate() {
                        scaled_text(
                            &mut canvas,
                            scale,
                            state,
                            pad * 2 + label_w + i as i32 * col,
                            pad * 3,
                            Typography::DEFAULT.caption_size as f32,
                            theme.palette.text_dim,
                        );
                    }
                    let mut y = pad * 3 + gap;
                    for kind in kinds {
                        scaled_text(
                            &mut canvas,
                            scale,
                            &format!("{kind:?}"),
                            pad * 2,
                            y + Controls::MIN_HEIGHT / 2,
                            Typography::DEFAULT.body_size as f32,
                            theme.palette.text_dim,
                        );
                        for (i, _) in states.iter().enumerate() {
                            let label = if i == 4 {
                                "Nicht verfügbar"
                            } else if i == 5 {
                                "! Eingabe prüfen"
                            } else {
                                "Änderungen öffnen"
                            };
                            let mut widget = Component::new(kind, label, col - gap);
                            widget.state = ComponentState {
                                focused: i == 2,
                                disabled: i == 4,
                                error: i == 5,
                                selected: i == 2,
                            };
                            let pointer = match i {
                                1 => WidgetState::Hovered,
                                3 => WidgetState::Pressed,
                                _ => WidgetState::Idle,
                            };
                            widget.paint_scaled(
                                Rect {
                                    x: pad * 2 + label_w + i as i32 * col,
                                    y,
                                    width: col - gap,
                                    height: widget.height(),
                                },
                                &mut canvas,
                                &theme,
                                pointer,
                                scale,
                            );
                        }
                        y += if kind == ComponentKind::Card {
                            Controls::CARD_HEIGHT + gap
                        } else {
                            Controls::FORM_HEIGHT + gap
                        };
                    }
                    scaled_text(&mut canvas,scale,"Sans-Fallback: ÄÖÜ äöü ß € – … ← → ✓ | Lange deutsche Bezeichnungen werden gekürzt.",pad*2,y+gap,Typography::DEFAULT.caption_size as f32,theme.palette.text_dim);
                    let file = format!(
                        "{out}/{name}-{width}x{height}-{}-{wall}.png",
                        (scale * 100.0) as u32
                    );
                    sheet.save_png(&file).unwrap();
                    println!("{file}");
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn scaled_text(
    canvas: &mut tiny_skia::PixmapMut<'_>,
    scale: f32,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: niwoe_tokens::Color,
) {
    paint_text(
        canvas,
        text,
        (x as f32 * scale).round() as i32,
        (y as f32 * scale).round() as i32,
        size * scale,
        color,
    );
}
