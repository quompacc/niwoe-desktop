# P05 — Deck-Komposition, erster Korrekturschritt

Status: **in-progress**, 23.09.2026. Basis: `fd965ed`, Branch
`codex/niwoe-p00`. Dieser Bericht dokumentiert einen begrenzten Schritt innerhalb
des [nativen Neubaus](P03_P05_NATIVE_REBUILD.md), keine Phasenabnahme.

## Ergebnis und Dateien

| Datei | Änderung |
|---|---|
| `crates/niwoe-shell/src/quick_settings_popup.rs` | Kompakte Lautstärkezeile, Gerätename darunter mit breitenbegrenzter Ausgabe. Gefüllte Abschnittskacheln durch feine Trennlinien ersetzt. Systemeinstellungen im Fußbereich. Fehlender Audioausgang wird nicht mehr als aktive Stummschaltung hervorgehoben. |
| `crates/niwoe-tokens/src/chrome.rs` | Audioabschnitt zentral von 112 auf 64 logische Pixel reduziert; Außenmaß weiterhin 360×400. |
| `crates/niwoe-shell/src/deck_keyboard.rs` | Tab-Reihenfolge folgt der neuen visuellen Reihenfolge; Systemeinstellungen zuletzt. |
| `crates/niwoe-shell/src/deck_tests.rs` | Bestehende Tests unverändert in eigene Datei verschoben; Vorschauausgabe als Helfer. Neuer Test für verfügbare Geräte, lange Namen, kollisionsfreie Trefferflächen und identische Geometrie in beiden Themes. |
| `docs/design/evidence/P05/` | Drei native Rasterbelege einschließlich Herkunft und Einschränkungen. |
| `docs/phase-reports/{P02_HANDOFF,P03_P05_NATIVE_REBUILD,README}.md` | Aktuellen Einstieg und diesen Korrekturschritt verlinkt. |
| `AGENTS.md` | Nutzerauftrag ergänzt: geprüfte Implementierungsstände immer als Release auf dem Fedora-Testrechner installieren. |

Der Renderer bleibt mit 514 physischen Zeilen unter der Projektgrenze;
Tests 191, Tastaturdatei 51 und Chrome-Tokens 570 Zeilen.

## Verifikation

Fedora-Testhost `fedora-dev`, Arbeitskopie `/home/eduard/niwoe-desktop`.
Die vier geänderten Rust-Dateien wurden nach lokalem Formatieren übertragen.
Kein Cargo-Manifest und keine Dependency geändert.

| Befehl | Ergebnis |
|---|---|
| `cargo fmt --all` lokal; `cargo fmt --all -- --check` Fedora | Exit 0 |
| `cargo check --workspace --locked` | Exit 0 |
| `cargo test --workspace --locked` | Exit 0; **1.094 bestanden, 0 fehlgeschlagen, 1 ignoriert** |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 |
| `git diff --check` lokal | Exit 0 |
| `cargo build --release --workspace --locked` | Exit 0; 1m48s |
| `sudo bash scripts/install-local.sh --desktop-user eduard` | Exit 0; sechs installierte Binaries bytegleich zum Release |

Workspace-Lauf enthält `design_guard`, `source_size_guard` und
`centralization_guard` sowie die beiden Deck-Tests für verfügbare und fehlende
Geräte. Logs lokal und auf Fedora unter `target/p05-deck/`.
Ein erster Test-Build fand eine nicht vorhandene TOML-Testabhängigkeit;
der Test verwendet nun direkt `Palette::DARK/LIGHT`. Abschließender Lauf grün.

Installation auf ausdrücklichen Folgeauftrag „immer installieren“ durchgeführt.
Gemeinsame KDE-/GTK-Dateien vor/nach Installation bytegleich. Der Installer
meldete den fehlenden User-Bus im sudo-Kontext; anschließend erfolgreiches
`systemctl --user daemon-reload` mit expliziter Sitzungsumgebung. Rechner am
Greeter, kein NIWOE-Prozess aktiv: neuer Stand beim nächsten NIWOE-Login sichtbar,
Deck über Super+Escape. Installationslog, Release-Log und SHA-256-Belege unter
`target/p05-deck/`. Kein laufender Desktop beendet.

Call-Flow geprüft: Popup-Render → gemeinsame Trefferflächen → Maus oder
Tab/Enter → `dispatch_deck_action` → bestehende Provider bzw. Settings-Aufruf.
Lautstärkedrag liest weiterhin dieselbe Sliderfläche. Escape, Öffnen/Schließen,
Backendbestätigung und Power-Bestätigung wurden nicht verändert.

## Sichtprüfung und Performance

[Native Dark-/Light-/Unavailable-Belege](../design/evidence/P05/README.md)
angesehen. Gerätebezeichnungen werden innerhalb der Karte abgeschnitten;
Bedienelemente bleiben innerhalb der Karte und überlappen sich nicht.
Der Test prüft gleiche Geometrie beider Themes.

Kein neuer Timer, keine Animation, kein neuer Effekt oder Cache und keine
zusätzliche Providerabfrage. Bestehende Repaint-Auslöser bleiben bestehen;
die flächigen Abschnittsfüllungen entfallen. CPU-/GPU-Verbesserungen wurden
nicht gemessen und werden nicht behauptet.

## Offene Grenzen

- Rasterbelege sind Testausgaben mit kontrollierten Daten, keine Live-Screenshots.
  Release installiert; bei abschließender SSH-Prüfung lief kein NIWOE-Prozess.
  Live-Sichtprüfung nach Anmeldung bleibt offen.
- Vollständige Mockup-Komposition bleibt offen: Symbolaktionen, Bluetooth-Anbindung,
  Raumabschnitt nach P06 und abschließende Material-/Abstandsprüfung. Der heutige
  Zwischenstand ist keine akzeptierte Designabweichung.
- Live-Bedienung, HiDPI, Hotplug, echte Backendfehler und Idle-Messungen: **NOT RUN**.
- P02–P05 bleiben **in-progress**. Kein Vorgriff auf persistente Räume/P06.

Review des tatsächlichen Diffs: begrenzter Kompositionsschritt, zentrale Tokens,
gemeinsame Eingabegeometrie und bestehende Provider erhalten. Keine vollständige
visuelle oder funktionale P05-Abnahme.
