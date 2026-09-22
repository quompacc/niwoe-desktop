use super::*;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "niwoe-migration-test-{}-{}",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn write(&self, path: &str, value: &str) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, value).unwrap();
    }
    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.0.join(path)).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn fresh_install_does_not_create_legacy_or_configuration() {
    let fixture = Fixture::new();
    migrate_legacy_at(&fixture.0).unwrap();
    assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 0);
}

#[test]
fn legacy_is_backed_up_copied_and_never_overwrites_on_repeat() {
    let fixture = Fixture::new();
    let original = "[general]\ntheme = \"light\"\n";
    fixture.write("meridian/config.toml", original);
    fixture.write("meridian/hidden_apps.txt", "org.example.Hidden.desktop\n");
    fixture.write("meridian/themes/dark/theme.toml", "");
    migrate_legacy_at(&fixture.0).unwrap();
    assert_eq!(fixture.read("niwoe/config.toml"), original);
    assert_eq!(fixture.read("meridian/config.toml"), original);
    assert_eq!(
        fixture.read("meridian/niwoe-migration-backup/config.toml"),
        original
    );
    assert_eq!(
        fixture.read("niwoe/hidden_apps.txt"),
        "org.example.Hidden.desktop\n"
    );
    fixture.write("meridian/config.toml", "[general]\ntheme = \"dark\"\n");
    migrate_legacy_at(&fixture.0).unwrap();
    assert_eq!(fixture.read("niwoe/config.toml"), original);
    assert_eq!(
        fixture.read("meridian/niwoe-migration-backup/config.toml"),
        original
    );
}

#[test]
fn existing_new_config_wins_even_if_legacy_is_invalid() {
    let fixture = Fixture::new();
    fixture.write("niwoe/config.toml", "[general]\ntheme = \"dark\"\n");
    fixture.write("meridian/config.toml", "not valid toml");
    migrate_legacy_at(&fixture.0).unwrap();
    assert!(fixture.read("niwoe/config.toml").contains("dark"));
    assert!(!fixture
        .0
        .join("meridian/niwoe-migration-backup/config.toml")
        .exists());
}

#[test]
fn invalid_legacy_is_preserved_and_backed_up_but_not_published() {
    let fixture = Fixture::new();
    fixture.write("meridian/config.toml", "not valid toml");
    assert!(migrate_legacy_at(&fixture.0).is_err());
    assert!(!fixture.0.join("niwoe/config.toml").exists());
    assert_eq!(
        fixture.read("meridian/niwoe-migration-backup/config.toml"),
        "not valid toml"
    );
    assert_eq!(fixture.read("meridian/config.toml"), "not valid toml");
}

#[test]
fn never_migrate_global_toolkit_settings_or_unknown_files() {
    let fixture = Fixture::new();
    fixture.write("meridian/toolkit/kdeglobals", "old output");
    fixture.write("meridian/unknown", "unknown data");
    fixture.write("kdeglobals", "KDE owned");
    migrate_legacy_at(&fixture.0).unwrap();
    assert!(!fixture.0.join("niwoe").exists());
    assert_eq!(fixture.read("kdeglobals"), "KDE owned");
}

#[cfg(unix)]
#[test]
fn symlinks_cannot_redirect_migration() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    let outside = Fixture::new();
    fixture.write("meridian/config.toml", "");
    symlink(&outside.0, fixture.0.join("niwoe")).unwrap();
    assert!(migrate_legacy_at(&fixture.0).is_err());
    assert!(!outside.0.join("config.toml").exists());
}
