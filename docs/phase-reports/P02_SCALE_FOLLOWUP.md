# P02 — Scale-Korrektur, 22.09.2026

Status: **in-progress**. Implementiert, auf Fedora geprüft und installiert;
echte DRM-Ausgaben und native Performance nach dieser Änderung noch offen.
Historischer P02-Checkpoint; inzwischen P03–P05 in Arbeit. Aktueller
[Handoff](P02_HANDOFF.md) und [Shell-Bericht](P03_P05_NATIVE_REBUILD.md) gehen vor.

## Scope-Entscheidung nach Konzeptabgleich

Auf ausdrücklichen Nutzerauftrag vom 22.09. wird die Alt-UI-Testreihe beendet.
Geprüfte Quellen: `NIWOE_Konzeptzusammenfassung.md` (insbesondere §8, §16–19,
§29 und §40), Designmanifest, Designbrief §5, Plan P02–P05 sowie erneut visuell
Desktop-/Deck-Mockup `17_13_54 (1).png` und Control Center `17_13_54 (3).png`.

Das Konzept erhält den technischen Kern und ersetzt Shell, Leiste, Launcher,
Deck und Informationsmodell. Der aktive Plan konkretisiert die Reihenfolge
Panel → Launcher → Deck; die beispielhafte Raum-zuerst-Reihenfolge in Konzept
§40 ersetzt diesen Plan nicht. Das Problem war die zusätzliche Alt-UI-Abnahme,
nicht ein fehlendes neues Designziel im Plan.

| Arbeit | Entscheidung |
|---|---|
| Paletten, zentrale Maße, native Komponenten, Font-Fallback, Kontrast-/Design-Guards | Weiterverwenden; das ist die Grundlage der neuen UI. |
| DRM-Scale, Capture-/Glas-Koordinaten, Wallpaper-Quellrechteck, Cursor-Hotspots, Layer-Reconfigure | Technische Grundlage behalten; verbleibende Integration mit neuer UI prüfen. |
| Begrenzung des alten 880×620-Launchers, bisherige Bottom-left-Position und zugehörige Hit-Tests (`Launcher::fitted_rect`, Shell-Layer/Render/Pointer) | Übergangslösung, kein neues Designziel. In P04 durch die zentrierte Suchpopup-Komposition ersetzen; keine weitere Verfeinerung oder Abnahmeserie. |
| Alte untere Leiste und deren Glasgeometrie | Keine weitere Gestaltung; P03 ersetzt Position, Aufbau und Reservierung gemeinsam. |
| Dark/Light × 100/150/200 %-Screenshots und native Performance der alten Shell | Zusätzliche Reihe entfällt. Diese Prüfungen an den tatsächlich neuen Oberflächen P03–P05 durchführen. |
| Wiederholte unveränderte Workspace-Gates, Nested-Läufe und Installationen | Entfallen ohne relevante Änderung oder neuen Fehler. Pflichtchecks nach Codeänderungen bleiben. |

Nächste Umsetzung: P03-Leiste oben mit Launcher, Workspace-/später Raumwahl,
flexiblem Abstand, echten Statusmodulen und Uhr. Danach zentriertes Suchpopup
(P04) und kleines Deck rechts oben (P05). Keine neuen Backends oder Raumfunktionen
vorziehen. Bestehende geprüfte Änderungen bleiben vorerst erhalten; ein bloßer
Rollback der Übergangslösung würde einen weiteren Code-/Testzyklus verursachen.

P02 wird nicht fälschlich vollständig accepted gesetzt. Seine Grundlagen sind
für P03 nutzbar; offene Hardware-/HiDPI-Nachweise werden mit den neuen Oberflächen
erbracht und blockieren deren Beginn nicht.

Hardwarezwischenstand: Nutzer hat sich regulär in NIWOE angemeldet. Laufender
Compositor und Shell entsprechen dem geprüften Release. `drm-0` meldete
1920×1080/60 Hz/Scale 1000. Saubere Dark-100-%-Aufnahme mit vollständigem Panel
und Settings nach regulärem Consent-Klick vorhanden (`target/p02-evidence/scale/
dark-100-clean.png`). Kein Wechsel auf 150 %, keine neue Performance-Messung.
Der Testclient lief vorher in sein Timeout; die Aufnahme wurde dennoch nach dem
Klick vom Compositor erzeugt und anschließend aus dem Runtime-Verzeichnis geholt.

Die folgenden Implementierungs- und Prüfnachweise bleiben historische Evidenz.
Ihre offenen Alt-UI-Testaufträge sind durch diese Scope-Entscheidung ersetzt.

## Geänderte Dateien

Pfade relativ zu `crates/`:

| Datei | Änderung |
|---|---|
| `niwoe-compositor/src/backend/drm/render.rs` | Szenenaufbau und Blur-Vorbereitung erhalten den tatsächlichen Output-Scale. |
| `niwoe-compositor/src/backend/drm/render/scene_helpers.rs` | Blur-Szene fragt Elementgeometrie mit demselben Scale wie KMS ab, erst danach Downsampling. |
| `niwoe-compositor/src/backend/drm/render/scene_composition.rs` | Skalierte Cursor-Hotspots; Panelglas aus Panel-Tokens; Launcher-Glas aus gemeinsamer begrenzter Geometrie; skaliertes Wallpaper-Ziel. Renderreihenfolge unverändert. |
| `niwoe-compositor/src/backend/drm/glass.rs` | Texturausschnitt, UVs und Shadermaße in physischen Koordinaten; Radien entsprechend skaliert. |
| `niwoe-compositor/src/backend/drm/render/capture.rs` | Screencopy, Screenshot und Thumbnail verwenden Output-Scale; Thumbnail-Ausschnitt in physischen, outputlokalen Koordinaten. |
| `niwoe-compositor/src/wallpaper/gpu.rs` | Vollständige physische Texturquelle mit logischer Zielgröße; bestehender Aufruf ohne Scale bleibt kompatibel. |
| `niwoe-compositor/src/state/setup/layout.rs` | Layer nach Live-Scale-Wechsel sofort neu anordnen und konfigurieren. |
| `niwoe-tokens/src/chrome.rs` | `Launcher::fitted_rect` begrenzt die bestehende untere linke Karte auf die logische Layer-Fläche. Keine P03-Neupositionierung. |
| `niwoe-tokens/tests/launcher_geometry.rs` | Drei Regressionstests: bisherige Desktopgeometrie, zwei Auflösungen bei 100/150/200 %, leere/kleinste Configure-Größe. |
| `niwoe-shell/src/wayland/handlers/layer.rs` | Visual-Position aus derselben Token-Geometrie, kein negativer Y-Offset bei kleinen logischen Outputs. |
| `niwoe-shell/src/wayland/render/launcher.rs` | Settings, Launcher, Kontextmenü und Schatten verwenden die tatsächlich passende Kartengröße. |
| `niwoe-shell/src/wayland/handlers/pointer/launcher.rs` | Hit-Tests und Scrollberechnung verwenden dieselben Inhaltsmaße wie die Zeichnung. |

## Verifikation

Fedora/Acer, `/home/eduard/niwoe-desktop`, Rust-Workspace unverändert in seinen
Dependencies und Cargo-Manifests. Alle zwölf geänderten Quelldateien sind nach
Zeilenenden-Normalisierung identisch zum lokalen Arbeitsbaum.

| Befehl | Abschließendes Ergebnis |
|---|---|
| `cargo fmt --all` | lokal Exit 0 |
| `cargo fmt --all -- --check` | Fedora Exit 0 |
| `cargo check --workspace --locked` | Exit 0 |
| `cargo test --workspace --locked` | Exit 0: **1.092 bestanden, 0 fehlgeschlagen, 2 ignoriert** |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 |
| `cargo build --release --workspace --locked` | Exit 0 |
| `bash scripts/smoke-nested.sh /run/user/1000/wayland-0` | Exit 0; isolierte Umgebung, authentifizierte Shell, Configure/Buffer-Attach des Testclients |
| `git diff --check` | lokal Exit 0 |

Der Workspace-Test enthält Design- und Dateigrößen-Guards. Keine neue
Guard-Ausnahme. Zwischenläufe mit einem Typinferenzfehler im Thumbnail-Ausschnitt
sind nicht als bestandene Gates gewertet; maßgeblich ist `scale-final/`.

Belege auf Acer: `target/p02-evidence/scale-final/`; Nested-Ordner:
`target/p01-evidence/nested.1dw53N/`. Lokales Roharchiv:
`target/p02-evidence/scale-final.tar.gz`. Der Nested-Smoke meldet weiterhin
fehlende AT-SPI-/PipeWire-Dienste in seiner isolierten Umgebung; er belegt keine
Accessibility-/Audio-Integration und keine echte DRM-Skalierung.

Installation mit `scripts/install-local.sh --desktop-user eduard` bestanden;
alle sechs installierten Binaries bytegleich mit dem geprüften Release.
Benutzer-Systemd anschließend im korrekten Benutzerbus erfolgreich neu geladen.
Die fünf gemeinsamen KDE-/GTK-Dateien vor/nach Installation unverändert.
KDE-Session 41 blieb aktiv; kein automatischer Sitzungswechsel oder Sperrtest.

## Call-Flow und Performance-Modell

Config-Reload → Smithay-Output-Scale → Layer-Arrange/Configure → Shell-Kartengröße
→ Buffer-Commit → DRM-Szene/KMS. Die Aufnahmepfade verwenden denselben Scale;
Glas rechnet vor dem bestehenden Downsampling in physische Koordinaten um.
Pointerkoordinaten bleiben logisch und treffen dieselbe Karte wie die Zeichnung.

Keine neuen Timer, Animationen oder Renderdurchläufe. Wallpaper-Textur bleibt
nach Quelle und physischer Outputgröße gecacht; Scale ändert nur das Zielrechteck.
Glas verwendet unverändert den bestehenden Cache für physische Downsamplegrößen.
Launcher zeichnet weiterhin auf bestehende Dirty-/Configure-Ereignisse; sein
vorhandener temporärer Inhaltsbuffer wird bei kleinen Outputs kleiner.

## Noch offen

- Echte saubere Dark-/Light-Aufnahmen von `drm-0` bei 100/150/200 %, nach bestätigten
  Output-Snapshots; Settings-Footer, Hit-Tests und Scale-Rückwechsel prüfen.
- Native DRM-Idle-Messung. Keine neuen GPU-/Input-to-present-Werte behauptet.
- Die Shell rastert weiterhin mit dem vorhandenen Buffer-Scale 1; der Compositor
  skaliert diese Flächen. Diese Korrektur belegt noch keine native HiDPI-Glyphenschärfe.
- Verfügbarkeit kleinerer realer Outputmodi erneut prüfen; geometrische Tests
  ersetzen keinen echten 1366×768-Output.
- Gesamtdiff und Hardwarebelege vor einer P02-Abnahme abschließend prüfen.

Die alten `light-real-150/200.png` bleiben Fehlerbelege des vorherigen Builds,
keine Nachweise für diese Korrektur.
