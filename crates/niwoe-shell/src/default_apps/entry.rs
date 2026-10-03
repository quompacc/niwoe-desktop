use super::*;

pub(super) fn parse_mime_entry(path: &std::path::Path) -> Option<(MimeAppCandidate, bool)> {
    let raw = fs::read_to_string(path).ok()?;
    let app = crate::launcher::DesktopApp::parse_mime_handler(&raw).ok()?;
    let desktop_id = path.file_name()?.to_str()?.to_string();
    let mut mime_types = Vec::new();
    let mut in_entry = false;
    let mut selectable = true;
    for line in raw.lines().map(str::trim) {
        if line.starts_with('[') && line.ends_with(']') {
            in_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_entry || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "NoDisplay" => selectable = !value.trim().eq_ignore_ascii_case("true"),
            "MimeType" => {
                for mime in value.split(';').map(str::trim).filter(|s| !s.is_empty()) {
                    if !mime_types.iter().any(|existing| existing == mime) {
                        mime_types.push(mime.to_owned());
                    }
                }
            }
            _ => {}
        }
    }
    Some((
        MimeAppCandidate {
            desktop_id,
            name: app.name,
            icon: app.icon_name,
            mime_types,
        },
        selectable,
    ))
}

#[cfg(test)]
pub(super) fn parse_mime_candidate(path: &std::path::Path) -> Option<MimeAppCandidate> {
    let (app, selectable) = parse_mime_entry(path)?;
    selectable.then_some(app)
}
