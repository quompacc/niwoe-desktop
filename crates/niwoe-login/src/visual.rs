use tiny_skia::{Color, Pixmap};

/// Static login background, filled once from the central theme at startup.
/// Redraws only copy the cached pixels before painting the card, text and cursor.
/// No wallpaper file or legacy branding asset is required.
pub(crate) struct LoginBackdrop {
    pixmap: Pixmap,
}

impl LoginBackdrop {
    pub(crate) fn new(width: u32, height: u32) -> Result<Self, &'static str> {
        let mut pixmap = Pixmap::new(width, height).ok_or("backdrop allocation failed")?;
        let bg = crate::login_theme().colors.background;
        pixmap.fill(Color::from_rgba8(bg.r, bg.g, bg.b, bg.a));
        Ok(Self { pixmap })
    }

    pub(crate) fn copy_rgba_to(&self, target: &mut [u8]) -> Result<(), &'static str> {
        if target.len() != self.pixmap.data().len() {
            return Err("backdrop target size mismatch");
        }
        target.copy_from_slice(self.pixmap.data());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backdrop_renders_without_wallpaper_and_restores_frame() {
        let backdrop = LoginBackdrop::new(640, 360).expect("backdrop");
        let mut target = vec![0; 640 * 360 * 4];
        backdrop.copy_rgba_to(&mut target).expect("copy");
        let bg = crate::login_theme().colors.background;
        let expected = Color::from_rgba8(bg.r, bg.g, bg.b, bg.a)
            .premultiply()
            .to_color_u8();
        let expected = [
            expected.red(),
            expected.green(),
            expected.blue(),
            expected.alpha(),
        ];
        assert!(target
            .as_chunks::<4>()
            .0
            .iter()
            .all(|pixel| pixel == &expected));
        target.fill(0);
        backdrop
            .copy_rgba_to(&mut target)
            .expect("restore cached frame");
        assert!(target
            .as_chunks::<4>()
            .0
            .iter()
            .all(|pixel| pixel == &expected));
    }

    #[test]
    fn backdrop_rejects_wrong_target_size() {
        let backdrop = LoginBackdrop::new(64, 64).expect("backdrop");
        assert!(backdrop.copy_rgba_to(&mut [0; 4]).is_err());
    }

    #[test]
    fn backdrop_rejects_empty_output() {
        assert!(LoginBackdrop::new(0, 64).is_err());
        assert!(LoginBackdrop::new(64, 0).is_err());
    }
}
