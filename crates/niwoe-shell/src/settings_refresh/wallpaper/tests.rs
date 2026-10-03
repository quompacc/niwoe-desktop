use super::*;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "niwoe-wallpaper-preview-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn entry(&self, name: &str) -> WallpaperEntry {
        let path = self.0.join(name).to_string_lossy().into_owned();
        WallpaperEntry {
            display_name: name.into(),
            apply_path: path.clone(),
            thumbnail_path: path,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn image(path: &str, width: u32, height: u32) {
    image::RgbaImage::from_pixel(width, height, image::Rgba([200, 100, 50, 128]))
        .save(path)
        .unwrap();
}

#[test]
fn bounded_thumbnail_keeps_aspect_and_premultiplies_once() {
    let fixture = Fixture::new();
    let entry = fixture.entry("photo.png");
    image(&entry.apply_path, 160, 90);
    let previews = load_previews(&[entry], vec![], (112, 63));
    let image = previews[0].image.as_ref().unwrap();
    assert_eq!((image.width(), image.height()), (112, 63));
    assert_eq!(&image.data()[..4], &[100, 50, 25, 128]);
}

#[test]
fn unchanged_files_share_pixels_and_changes_size_or_deletion_invalidate() {
    let fixture = Fixture::new();
    let entry = fixture.entry("photo.png");
    image(&entry.apply_path, 160, 90);
    let catalog = [entry.clone()];
    let first = load_previews(&catalog, vec![], (112, 63));
    let same = load_previews(&catalog, first.clone(), (112, 63));
    assert!(Arc::ptr_eq(
        first[0].image.as_ref().unwrap(),
        same[0].image.as_ref().unwrap()
    ));
    let resized = load_previews(&catalog, same, (56, 31));
    assert!(!Arc::ptr_eq(
        first[0].image.as_ref().unwrap(),
        resized[0].image.as_ref().unwrap()
    ));
    image(&entry.apply_path, 32, 32);
    let replaced = load_previews(&catalog, first.clone(), (112, 63));
    assert!(!Arc::ptr_eq(
        first[0].image.as_ref().unwrap(),
        replaced[0].image.as_ref().unwrap()
    ));
    std::fs::remove_file(&entry.apply_path).unwrap();
    let deleted = load_previews(&catalog, replaced, (112, 63));
    assert!(deleted[0].image.is_none());
    image(&entry.apply_path, 32, 32);
    assert!(load_previews(&catalog, deleted, (112, 63))[0]
        .image
        .is_some());
}

#[test]
fn oversized_input_dimension_and_invalid_files_are_unavailable() {
    let fixture = Fixture::new();
    let huge = fixture.entry("huge.png");
    File::create(&huge.apply_path)
        .unwrap()
        .set_len(INPUT_LIMIT + 1)
        .unwrap();
    let wide = fixture.entry("wide.png");
    image(&wide.apply_path, DIMENSION_LIMIT + 1, 1);
    let bad = fixture.entry("bad.png");
    std::fs::write(&bad.apply_path, b"not an image").unwrap();
    let missing = fixture.entry("missing.png");
    assert!(
        load_previews(&[huge, wide, bad, missing], vec![], (112, 63))
            .iter()
            .all(|p| p.image.is_none())
    );
}

#[test]
fn catalog_growth_is_bounded_and_reordering_keeps_path_identity() {
    let fixture = Fixture::new();
    let photo = fixture.entry("photo.png");
    image(&photo.apply_path, 32, 32);
    let missing = fixture.entry("missing.png");
    let first = load_previews(&[photo.clone(), missing.clone()], vec![], (112, 63));
    let reorder = load_previews(&[missing, photo], first.clone(), (112, 63));
    assert!(reorder[0].image.is_none());
    assert!(Arc::ptr_eq(
        first[0].image.as_ref().unwrap(),
        reorder[1].image.as_ref().unwrap()
    ));
    let entries: Vec<_> = (0..64)
        .map(|i| fixture.entry(&format!("{i}.png")))
        .collect();
    assert_eq!(load_previews(&entries, vec![], (112, 63)).len(), 40);
}
