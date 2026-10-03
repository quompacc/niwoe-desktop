# P05 — Blockierender Dialog und unzureichendes Material

23.09.2026. **Input-Fix auf Fedora geprüft, installiert und im ursprünglichen Fehlerfall bestätigt.**
Nach Neuanmeldung entspricht der laufende Compositor dem installierten Build.
Nutzer bestätigt: Mausklick auf Erlauben über den Einstellungen funktioniert.
Materialkorrektur bleibt offen. Vorheriger Stand: [Deck-Symbolaktionen](P05_DECK_SYMBOLS.md).

## Nutzerbefund und Reproduktion

Bei geöffneten Einstellungen liegt „Bildschirmfoto erlauben?“ sichtbar davor,
seine Buttons reagieren jedoch nicht auf Klicks. Nutzer bestätigt: Escape
schließt den Dialog. Die laufende Capture-Prüfung nach dem Bluetooth-Aufruf
wartete entsprechend auf Zustimmung und endete nach Escape mit
`permission-denied`. Kein erfolgreicher Bluetooth-Seiten-Screenshot vorhanden.

## Ursache und vorbereiteter Input-Fix

`third_party/smithay/src/desktop/wayland/layer.rs`: `layer_under()` durchsucht
`layers_on(layer).rev()`. NIWOEs DRM-Renderer benutzt dagegen `layers()` und
eine stabile Overlay-vor-Top-Sortierung; erstes Render-Element liegt oben.
Dadurch kann das sichtbare Vordergrundfenster die Maus an die verdeckte
Einstellungsfläche verlieren. Der bisherige gesonderte Launcher-Fallback
berücksichtigt ebenfalls keine gemeinsame visuelle Reihenfolge.

Lokaler Diff:

| Datei unter `crates/niwoe-compositor/src/` | Änderung |
|---|---|
| `layer_order.rs` | Gemeinsame obere Layer-Einordnung und Trefferreihenfolge; keine Allokation/Sortierung pro Mausbewegung. Drei Regressionstests: Dialog über Settings, transparente/verborgene Eingabefläche, Overlay/Top und temporär veraltete Shell-Rolle. |
| `lib.rs` | Internes Modul registriert. |
| `backend/drm/render/layers.rs` | Vorhandene visuelle Reihenfolge über gemeinsamen Schlüssel ausgedrückt; DRM-Reihenfolge verhaltensgleich. |
| `backend/winit/layers.rs` | Gleiche Overlay-vor-Top-Reihenfolge wie DRM. Relative Reihenfolge innerhalb derselben Ebene erhalten. |
| `state/layout/surface.rs` | Trefferprüfung folgt dem sichtbaren oberen Stapel; ohne tatsächlichen Surface-/Input-Hit zum nächsten Kandidaten weitergehen. Separater Launcher-Vorrang entfällt. |

Erforderliche Linux-Prüfungen: Workspace-Check/-Tests/Clippy, Guards, isolierte
Inputreproduktion und echter Dialog über Settings mit Erlauben/Ablehnen per
Maus sowie Escape. Außerdem nach Dialogschluss zugängliche Settings und
Wiederöffnen prüfen. Lock- und Screenshot-Autorisierung unverändert lassen.
Der Fix betrifft den Compositor; Installation allein aktualisiert den bereits
laufenden Compositor nicht. Sitzungsneustart nach erfolgreicher Prüfung nötig.

## Verifikation und Werkzeugblocker

- `cargo fmt --all`, `git diff --check`: lokal bestanden.
- `cargo test -p niwoe-tokens --test design_guard --test source_size_guard --locked`:
  lokal bestanden, beide Guards grün. Dies ersetzt keine Linux-Workspace-Gates.
- `cargo check --workspace --locked`: unter Windows fehlgeschlagen, fehlendes
  `pkg-config`/Linux-Wayland-Buildumfeld in `wayland-sys`.
- `cargo test --workspace --locked`: gleicher Plattformblocker;
  `target/p05-symbols/windows-test-attempt.log`. Keine Tests als bestanden zählen.
- Fedora: `cargo fmt --all -- --check`, `cargo check --workspace --locked`,
  `cargo test --workspace --locked`,
  `cargo clippy --workspace --all-targets --locked -- -D warnings` und
  `cargo build --release --workspace --locked` bestanden.
  **1.098 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert**, einschließlich
  der drei neuen Layer-Regressionsprüfungen. Logs: `target/p05-input/`.
- SSH funktioniert mit dem vom Nutzer angelegten Schlüssel `niwoe_fedora`.
  Das interaktive Werkzeugterminal bleibt durch den Windows-Prozessstartfehler
  blockiert. Nutzer führte den vorbereiteten sudo-Installationsschritt aus.
- Installation unabhängig per SSH bestätigt: alle sechs Programme bytegleich
  zum Release, KDE-/GTK-Konfiguration unverändert. User-`daemon-reload` mit
  explizitem Runtime-/Buspfad erfolgreich; vorherige Buswarnung betraf den
  Installer-Kontext. Boot-Login-Service nicht zusätzlich aktiviert.
- Nach freiwilliger Neuanmeldung aktive Buildidentität per `/proc/.../exe`
  bestätigt. Screenshot-Anfrage über geöffneten Einstellungen erfolgreich;
  Nutzer bestätigt ausdrücklich Mausklick auf Erlauben. Einstellungen nach
  Dialogschluss sichtbar, anschließend Deck geöffnet und weitere Aufnahme
  erfolgreich. Ablehnen und Escape nach dem Fix noch nicht separat geprüft.

## Weiterer Befund: Capture vor Glasauflösung

`backend/drm/render.rs` bediente Screencopy, Thumbnails und Screenshots bisher
vor dem Ersetzen von `GlassElement::Pending` durch `Ready`. Pending zeichnet
absichtlich nichts. Deshalb enthalten bisherige DRM-Aufnahmen keine
compositoreigenen Glasflächen und sind als Nachweis des Bildschirmmaterials
unvollständig. Das erklärt den fehlenden Materialkörper in den Aufnahmen,
nicht automatisch alle vom Nutzer am Bildschirm beanstandeten Abweichungen.

Korrektur: Die drei Capture-Aufrufe folgen der vorhandenen Glasauflösung,
bleiben aber vor `render_frame`/KMS-Planezuweisung. Keine Änderung der visuellen
Elementreihenfolge oder Screenshot-Autorisierung. Kein zusätzlicher Blurpass,
keine neue Allokation und kein neuer Timer; bestehende Glastexturen und Cache
werden auch für die Aufnahme verwendet. Linux-Gates für diesen Folgeschritt
separat unter `target/p05-capture/`: fmt/check/test/clippy/Release bestanden,
1.098 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert. Installation/Liveprüfung
noch offen; `target/p05-capture/install.sh` auf Fedora vorbereitet.

## Nutzerkorrektur zur Gestaltung

Der Nutzer beanstandet erneut, dass die native Ausgabe die verbindlichen Mockups
nicht trifft. Die bisherigen kleinteiligen Korrekturen und eingefärbten
Rastervorschauen sind kein ausreichender Fortschritts-/Abnahmenachweis.
Die sichtbare Deck-Fläche übernimmt im Livebild viel Blau vom Hintergrund;
Vorlage: dunkle, klar abgegrenzte Glasfläche. Proportionen, Typografie und
Bedienelemente bleiben ebenfalls unzureichend angenähert.

Konkreter Pipelinebefund: `niwoe-shell/src/popup_card.rs::draw_card_body` zeichnet
nur transparente Fläche und Kontur. Die DRM-Komposition in
`backend/drm/render/scene_composition.rs` erzeugt die zugehörigen Glasflächen
nur unter `decorations.glass && decorations.glass_blur`. Ohne diesen Pfad
fehlt ein garantierter Flächen-Fallback. Ob genau diese Bedingung die aktuelle
Live-Abweichung verursacht, muss anhand der wirksamen Hostkonfiguration und
Renderdiagnose belegt werden; noch keine solche Ursachenbehauptung.

Nächste Reihenfolge: Capture-Fix installieren und live prüfen; danach vollständige
Materialpipeline im Compositor untersuchen (Tönung/Deckkraft, Blur, Schatten,
Kontur und Geometrie). Fehlende Grundbausteine zentral ergänzen. Native Liveausgabe
direkt gegen das Desktopmockup prüfen, mit identischem Kontext in Dark/Light;
isolierte Rasteransichten bleiben nur Hilfsmittel. Keine weitere freie
Ersatzgestaltung und keine Behauptung, Rust begrenze die erreichbare Qualität.
