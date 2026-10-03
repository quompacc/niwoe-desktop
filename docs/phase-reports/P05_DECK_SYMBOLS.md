# P05 — Symbolaktionen und Bluetooth-Zugang

**Nachfolgender Blocker:** [Dialog-Eingabe und Material](P05_INPUT_MATERIAL_FINDINGS.md).
Die Capture-Prüfung der Bluetooth-Seite wurde per Escape abgelehnt, weil die
Zustimmungsbuttons über Settings nicht klickbar waren. Kein vollständiger
Live-Bediennachweis. Nutzer beanstandet weiterhin die Gestaltung.

23.09.2026, Status **in-progress**. Fortsetzung des
[ersten Kompositionsschritts](P05_DECK_COMPOSITION.md), weiterhin keine
vollständige P05-Abnahme. Basis: `fd965ed` plus dort dokumentierter Arbeitsstand.

## Ergebnis und Dateien

| Datei unter `crates/` | Änderung |
|---|---|
| `niwoe-shell/src/quick_settings_popup.rs` | Lautstärke mit Kontursymbol; vier Symbolaktionen, bestehende Darstellung/Akku-/Sitzungsaktionen erhalten. |
| `niwoe-shell/src/deck_controls.rs` | Symbolzeile Netzwerk, Bluetooth, Anzeige und Leistung; zentrale Geometrie, Zustandsbeschriftungen und gemeinsame Trefferflächen. |
| `niwoe-shell/src/panel_view.rs`, `panel_view/status_symbols.rs` | Bestehende gecachte Kontursymbole für das Deck zugänglich; Bluetooth-, Anzeige- und Leistungssymbol ergänzt. |
| `niwoe-tokens/src/chrome.rs` | Symbolzeile 88, Statuszeile 48 logische Pixel; Deck weiterhin 360×400. |
| `niwoe-shell/src/network_popup.rs`, `wayland/render/network_audio.rs` | Bluetooth-Snapshot und laufende Abfrage an die Darstellung durchgereicht. |
| `niwoe-shell/src/wayland/state/popups.rs` | Beim Öffnen einmalige Bluetooth-Abfrage über vorhandenen Settings-Worker angefordert. |
| `niwoe-shell/src/wayland/state/timers.rs` | Bestätigte Worker-Antwort aktualisiert ein sichtbares Deck; kein Repaint bei geschlossenem Deck. |
| `niwoe-shell/src/wayland/state/deck_actions.rs` | Bluetooth öffnet vorhandene Geräteverwaltung; Anzeige öffnet vorhandene Anzeigeeinstellungen. |
| `niwoe-shell/src/deck_keyboard.rs` | Fokus an Aktionsidentität gebunden, damit asynchron hinzukommende/entfallende Ziele keine andere Aktion auswählen. |
| `niwoe-shell/src/deck_tests.rs` | Beide Themes, alle zehn Ziele, Adapter fehlt/ein/aus/lädt, Mausziel und stabiler Fokus bei wechselnder Verfügbarkeit. |

Dokumentation: dieser Bericht, aktueller Handoff/Phasenindex und dauerhafte
Bildbelege unter `docs/design/evidence/P05/`.

## Call-Flow und Kostenmodell

Deck öffnen → `request_settings_refresh(Bluetooth)` → vorhandener Worker →
`BluetoothSnapshot::poll()` → Ergebniskanal → Snapshot ersetzen → sichtbares
Deck neu zeichnen. Während der Abfrage steht „Lädt …“; fehlender Adapter steht
als „Fehlt“ ohne Maus-/Tab-Ziel. Powered-Status kommt aus der Providerantwort.
Klick/Enter auf verfügbares Bluetooth öffnet die vorhandenen Einstellungen und
schließt das Deck. Der Deck-Button ist kein unbestätigter Ein/Aus-Toggle.

Maximal eine laufende Anfrage pro Kategorie über den vorhandenen Inflight-Satz.
Keine neue periodische Abfrage, kein Timer, keine Animation und kein neuer
Blurpfad. Ein geschlossenes Deck löst keine Bluetooth-Abfragen aus.
Der vorhandene Symbolcache bleibt auf 32 Rasterbilder beschränkt, Schlüssel aus
Symbol/Zustand/Farbe, Lebensdauer Shellprozess. Die feste logische Rastergröße
kommt aus dem Panel-Token; die bestehende Surface-Skalierung bleibt zuständig.
Für den BGRA-Deckpuffer wird die Symbolfarbe vor dem gecachten Rasterisieren
kanalgetauscht; kein vollständiger Frame-Konvertierungspass.

## Verifikation

Fedora: `cargo fmt --all -- --check`, `cargo check --workspace --locked`,
`cargo test --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings`: **Exit 0**.
Die inkludierten Test-/Tastaturdateien zusätzlich mit `rustfmt --edition 2021`
formatiert und geprüft. Workspace-Lauf enthält Design-, Source-Size- und
Centralization-Guard. Lokaler `git diff --check`: Exit 0.
**1.095 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert.**

Dark-/Light-Rasteransichten geprüft: gleiche Geometrie, Symbole und Texte innerhalb
der Karte. Alle Rust-Dateien unter 600 Zeilen. Logs unter `target/p05-symbols/`.

Release: `cargo build --release --workspace --locked`, Exit 0, 1m47s.
Installation über `scripts/install-local.sh --desktop-user eduard`, Exit 0;
alle sechs installierten Binaries bytegleich zum Release. Gemeinsame KDE-/GTK-
Dateien bytegleich vor/nach Installation. User-Bus-Hinweis im sudo-Kontext durch
anschließenden erfolgreichen User-Daemon-Reload mit Sitzungsumgebung behandelt.

Shell PID 8032 gezielt beendet, Watchdog startet PID 17228; laufende Shell
bytegleich zum installierten Release. Compositor PID 8017 und Sitzung erhalten.
Live-Sichtprüfung: `drm-0`, 1920×1080, 100 %, Dark. Bluetooth „Ein“ stimmt mit
`bluetoothctl show` überein. Leistungsprofil fehlt und hat kein aktives Ziel.
Erster Capture zeigte das geschlossene Deck; erneutes Öffnen und Capture
erfolgreich. [Dauerhafte Bildbelege](../design/evidence/P05/README.md).

## Offene Grenzen

- Raumabschnitt folgt dem persistenten Raummodell; kein erfundener Raumname.
- Anstelle des noch nicht durchgesetzten Ruhemodus bleibt das vorhandene
  Leistungsprofil erreichbar. Vollständige Mockup-Abnahme weiterhin offen.
- Bluetooth ist Status plus Zugang zur Geräteverwaltung. Die bestehende
  Settings-Ein/Aus-Aktion setzt derzeit noch optimistisch lokalen Zustand;
  eine bestätigte Mutation mit sichtbarer Fehlerbehandlung bleibt P05-Arbeit.
- HiDPI, Hotplug, Idle-Messreihen und vollständige Backendfehlerfälle bleiben
  **NOT RUN**. Rasterbelege ersetzen diese Nachweise nicht.
