use super::*;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "niwoe-wallpaper-catalog-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn file(&self, path: &str, length: usize) -> PathBuf {
        let path = self.0.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, vec![0; length]).unwrap();
        path
    }
    fn scan(&self, current: Option<&str>) -> Vec<WallpaperEntry> {
        scan(std::slice::from_ref(&self.0), current)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn verified_pack_uses_landscape_resolution_and_never_its_presentation_screenshot() {
    let root = Fixture::new();
    root.file("Altai/metadata.json", 1);
    root.file("Altai/contents/screenshot.png", 1000);
    root.file("Altai/contents/images/1080x1920.png", 900);
    root.file("Altai/contents/images/1920x1080.png", 500);
    let best = root.file("Altai/contents/images/5120x2880.png", 1);
    let entries = root.scan(None);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].display_name, "Altai");
    assert_eq!(entries[0].apply_path, best.to_str().unwrap());
    assert_eq!(entries[0].thumbnail_path, entries[0].apply_path);
}

#[test]
fn unrelated_images_are_individual_choices_even_with_resolution_like_names() {
    let root = Fixture::new();
    let photo = root.file("Photos/Sommer_Foto.jpg", 10);
    let other = root.file("Photos/1920x1080.png", 20);
    let entries = root.scan(None);
    assert_eq!(entries.len(), 2);
    assert!(entries
        .iter()
        .any(|entry| entry.apply_path == photo.to_str().unwrap()));
    assert!(entries
        .iter()
        .any(|entry| entry.apply_path == other.to_str().unwrap()));
    assert!(entries
        .iter()
        .any(|entry| entry.display_name.contains("Sommer Foto")));
    assert!(entries
        .iter()
        .all(|entry| entry.thumbnail_path == entry.apply_path));
}

#[test]
fn selected_missing_custom_image_stays_reachable_beyond_the_bounded_catalog() {
    let root = Fixture::new();
    for index in 0..64 {
        root.file(&format!("Photo-{index:02}.png"), 1);
    }
    let missing = root.0.join("ZZ-explicit-missing.jpg");
    let entries = root.scan(missing.to_str());
    assert_eq!(entries.len(), CATALOG_LIMIT);
    assert!(entries
        .iter()
        .any(|entry| entry.apply_path == missing.to_str().unwrap()));
    assert!(entries
        .iter()
        .all(|entry| entry.thumbnail_path == entry.apply_path));
}

#[test]
fn repeated_roots_and_selected_catalog_path_do_not_duplicate_a_choice() {
    let root = Fixture::new();
    let photo = root.file("Photo.png", 1);
    let entries = scan(&[root.0.clone(), root.0.clone()], photo.to_str());
    assert_eq!(entries.len(), 1);
}

#[test]
fn catalog_traversal_respects_depth_and_entry_budget() {
    let root = Fixture::new();
    root.file("a/b/c/d/e/f/TooDeep.png", 1);
    root.file("Visible.png", 1);
    let entries = root.scan(None);
    assert_eq!(entries.len(), 1);
    assert!(entries[0].display_name.ends_with("Visible"));
    let mut remaining = 1;
    let mut entries = Vec::new();
    visit(&root.0, DEPTH_LIMIT, &mut remaining, &mut entries);
    assert_eq!(remaining, 0);
    assert!(entries.len() <= 1);
}
