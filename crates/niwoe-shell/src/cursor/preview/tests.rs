use super::*;

fn cursor(frames: &[(u32, u32, u32, u8)]) -> Vec<u8> {
    let mut bytes = b"Xcur".to_vec();
    for n in [16u32, 1, frames.len() as u32] {
        bytes.extend(n.to_le_bytes());
    }
    let mut offset = 16 + frames.len() * 12;
    for (size, width, height, _) in frames {
        for n in [0xfffd_0002, *size, offset as u32] {
            bytes.extend(n.to_le_bytes());
        }
        offset += 36 + (width * height * 4) as usize;
    }
    for (size, width, height, pixel) in frames {
        for n in [36u32, 0xfffd_0002, *size, 1, *width, *height, 0, 0, 10] {
            bytes.extend(n.to_le_bytes());
        }
        for _ in 0..width * height {
            bytes.extend([*pixel, *pixel, *pixel, u8::MAX]);
        }
    }
    bytes
}

#[test]
fn nearest_preview_uses_first_frame_and_preserves_compositor_bytes() {
    let bytes = cursor(&[(16, 16, 16, 10), (32, 32, 32, 20), (32, 32, 32, 30)]);
    let image = xcursor::first_nearest_frame(&bytes, 24).unwrap();
    assert_eq!(image.width(), 16);
    assert_eq!(&image.data()[..4], &[10, 10, 10, u8::MAX]);
    let image = xcursor::first_nearest_frame(&bytes, 30).unwrap();
    assert_eq!(image.width(), 32);
    assert_eq!(image.data()[0], 20);
}

#[test]
fn invalid_offsets_dimensions_hotspots_and_truncated_files_are_rejected() {
    let bytes = cursor(&[(24, 24, 24, 10)]);
    for length in [0, 4, 15, 27, 63, bytes.len() - 1] {
        assert!(xcursor::first_nearest_frame(&bytes[..length], 24).is_none());
    }
    for (offset, value) in [
        (4, u32::MAX),
        (12, 4097),
        (24, u32::MAX),
        (28 + 16, 0),
        (28 + 20, u32::MAX),
        (28 + 24, 25),
    ] {
        let mut invalid = bytes.clone();
        invalid[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert!(xcursor::first_nearest_frame(&invalid, 24).is_none());
    }
    assert!(xcursor::first_nearest_frame(&cursor(&[(24, 129, 129, 10)]), 24).is_none());
    // Large unselected frames do not prevent a valid bounded selected preview.
    assert!(
        xcursor::first_nearest_frame(&cursor(&[(24, 24, 24, 10), (128, 129, 129, 20)]), 24)
            .is_some()
    );
}

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "niwoe-cursor-preview-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn theme(&self, name: &str, metadata: &str, bytes: Option<&[u8]>) {
        let root = self.0.join(name);
        std::fs::create_dir_all(root.join("cursors")).unwrap();
        std::fs::write(root.join("index.theme"), metadata).unwrap();
        if let Some(bytes) = bytes {
            std::fs::write(root.join("cursors/left_ptr"), bytes).unwrap();
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn inherited_preview_has_localized_name_reuses_arc_and_invalidates_size_or_file() {
    let root = Fixture::new();
    root.theme(
        "Child",
        "[Icon Theme]\nName=Child\nName[de]=Lesbarer Zeiger\nInherits=Parent\n",
        None,
    );
    root.theme(
        "Parent",
        "[Icon Theme]\nName=Parent\n",
        Some(&cursor(&[(16, 16, 16, 10), (32, 32, 32, 20)])),
    );
    let themes = vec!["Child".to_string()];
    let load = |size, previous| {
        CursorPreviews::load_in_roots(
            themes.clone(),
            size,
            previous,
            std::slice::from_ref(&root.0),
        )
    };
    let first = load(16, CursorPreviews::default());
    assert_eq!(first.entries[0].label, "Lesbarer Zeiger");
    let image = first.entries[0].image.as_ref().unwrap().clone();
    let again = load(16, first);
    assert!(Arc::ptr_eq(
        &image,
        again.entries[0].image.as_ref().unwrap()
    ));
    assert!(!again.matches(&themes, 32));
    assert!(!again.matches(&["Other".to_string()], 16));
    let larger = load(32, again);
    assert_eq!(larger.entries[0].image.as_ref().unwrap().width(), 32);
    assert!(!Arc::ptr_eq(
        &image,
        larger.entries[0].image.as_ref().unwrap()
    ));
    root.theme(
        "Parent",
        "[Icon Theme]\n",
        Some(&cursor(&[(32, 31, 31, 30)])),
    );
    let changed = load(32, larger);
    assert_eq!(changed.entries[0].image.as_ref().unwrap().width(), 31);
    std::fs::remove_file(root.0.join("Parent/cursors/left_ptr")).unwrap();
    assert!(load(32, changed).entries[0].image.is_none());
}

#[test]
fn lookup_respects_roots_stops_cycles_and_rejects_oversized_input() {
    let first = Fixture::new();
    let second = Fixture::new();
    first.theme(
        "Theme",
        "[Icon Theme]\n",
        Some(&cursor(&[(24, 24, 24, 10)])),
    );
    second.theme(
        "Theme",
        "[Icon Theme]\n",
        Some(&cursor(&[(24, 24, 24, 20)])),
    );
    let previews = CursorPreviews::load_in_roots(
        vec!["Theme".to_string()],
        24,
        CursorPreviews::default(),
        &[first.0.clone(), second.0.clone()],
    );
    assert_eq!(previews.entries[0].image.as_ref().unwrap().data()[0], 10);
    first.theme("A", "Inherits=B\n", None);
    first.theme("B", "Inherits=A\n", None);
    assert!(find_icon(
        std::slice::from_ref(&first.0),
        "A",
        "left_ptr",
        &mut HashSet::new()
    )
    .is_none());
    let path = first.0.join("oversized");
    std::fs::write(&path, vec![0u8; FILE_LIMIT + 1]).unwrap();
    assert!(read_bounded(&path, FILE_LIMIT).is_none());
}
