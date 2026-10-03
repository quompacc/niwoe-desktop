# P05 — Systemdeck: Raumbezug und Abmelden

Stand: 25.09.2026. Bindende Bildquelle für das kleine Deck ist
`assets/ChatGPT Image 21. Sept. 2026, 17_13_54 (1).png`.

## Geänderte Dateien

- `crates/niwoe-shell/src/quick_settings_popup.rs`: Die bisherige
  Darstellung/Energie-Zeile zeigt den aktuellen Raum und öffnet den Hub.
  Die Aktionszeile enthält Sperren, Abmelden und Ausschalten; Abmelden zeigt
  nach dem ersten Klick „Bestätigen“.
- `crates/niwoe-shell/src/network_popup.rs` und
  `wayland/render/network_audio.rs`: Der Room-Snapshot liefert den angezeigten
  Namen; in der Loge steht „Loge“. Die bestehende zweistufige Power-Bestätigung
  liefert den sichtbaren Abmeldestatus.
- `crates/niwoe-shell/src/deck_keyboard.rs` und
  `wayland/state/deck_actions.rs`: Raum- und Abmelden-Aktionen sind mit Maus und
  Tastatur erreichbar. Abmelden verwendet den vorhandenen Compositor-IPC-Pfad.
- `crates/niwoe-shell/src/wayland/state/{shell_actions,timers}.rs`: Der
  Bestätigungszustand zeichnet das Deck beim Ablauf einmal neu; der Launcher
  zeichnet seinen Countdown nur, wenn er sichtbar ist.
- `crates/niwoe-shell/src/deck_tests.rs`: Geometrie- und Fokusziele für die
  neue Aktionsreihe aktualisiert.

## Bildabgleich und Grenze

`target/p05-deck-before.png` zeigt den installierten Stand vor dem Umbau.
`target/p05-deck-preview-dark.png` zeigt den neuen nativen Renderstand.
`target/p05-deck/live-deck.png` zeigt den installierten Stand in der aktiven
NIWOE-Sitzung bei 1920×1080. Das Deck sitzt rechts direkt unter dem Panel;
Audio, Netzwerk, Bluetooth, Anzeige, Raumzeile und die drei Energieaktionen
sind sichtbar. Die Raumzeile zeigt in der neutralen Loge korrekt „Loge“.
Die Grundgeometrie und die grüne Glasfläche entsprechen dem verbindlichen
Desktop-Mockup `(1)`; der ausdrücklich beauftragte Abmelden-Button ergänzt
dessen Fußbereich. Die echte Sitzung hat keine geöffneten App-Fenster und
belegt daher noch keinen Vergleich der Fensterkomposition.
Material, Position, Audio, Netzwerk, Bluetooth und Anzeige bleiben im
vorhandenen Deck. Der Raumbezug entspricht der Informationshierarchie des
Mockups. Eine Ruhe-/DND-Schaltfläche wird erst mit tatsächlich durchgesetztem
Backend angeboten; derzeit zeigt das Deck die vorhandene Leistungsfähigkeit
mit ehrlichem „Fehlt“-Zustand. Die Akkuanzeige bleibt im Panel.

## Performance und Verifikation

Der Deck-Renderer zeichnet nur beim Öffnen, bei Eingabe oder bestätigter
Zustandsänderung. Der Abmelde-Button hat keine eigene Abfrage und keine
Animation. Für seine Bestätigungsfrist läuft der vorhandene Timer; bei Ablauf
erfolgt ein Deck-Repaint, ohne einen unsichtbaren Launcher pro Tick zu zeichnen.

- `cargo fmt --all -- --check`: grün.
- `cargo check --workspace --locked`: grün auf Fedora.
- `cargo test --workspace --locked -q`: grün auf Fedora, einschließlich
  Deck-Geometrie/Fokus und Design-Guard.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: grün.
- `cargo build --release -p niwoe-shell --locked`: grün.
- Release-SHA-256: `49de4e2609513c10cf8af152fed0cb49f76ce61673f988fe93ff83a34efdacfb`.
- Installationsskript: `target/p05-deck/install.sh`. Der aktuelle Shell-Release
  wurde am 25.09.2026 auf Fedora installiert. `/usr/local/bin/niwoe-shell`,
  `target/release/niwoe-shell` und der laufende Prozess `/proc/18428/exe`
  sind bytegleich (SHA-256
  `49de4e2609513c10cf8af152fed0cb49f76ce61673f988fe93ff83a34efdacfb`).
  Die gesicherten KDE-/GTK-Dateien vor und nach der Installation sind
  bytegleich (`target/p05-deck/shared-before.txt` und `shared-after.txt`).
  Die aktive logind-Sitzung 2 meldet `Desktop=NIWOE`, `Type=wayland`.
  `spectacle -b -n` endete in dieser Sitzung mit Exitcode 1. Das User-Journal nennt die Ursache:
  Spectacle benötigt unter Wayland KWin, während die Sitzung auf NIWOE läuft.
  Der vorhandene NIWOE-Screenshot-Portalpfad lieferte anschließend den
  Live-Bildbeleg erfolgreich (`Screenshot`-Antwortcode 0).
- Der Nutzer betätigte „Abmelden“ in der laufenden Sitzung genau einmal:
  Der Button zeigte „Bestätigen“, nach etwa vier Sekunden wieder „Abmelden“.
  Die Shell lief danach weiterhin als PID 18428. Der zweite Klick wurde
  bewusst nicht ausgeführt, da er die Sitzung beendet.
- Audio-Livepfad: Vor dem Klick meldete PipeWire `Volume: 0.64`; der Nutzer
  klickte im Deck auf das Lautsprechersymbol, sah den Stumm-Zustand und das
  Deck blieb offen. `wpctl get-volume @DEFAULT_AUDIO_SINK@` bestätigte danach
  `Volume: 0.64 [MUTED]`. Der zweite Deck-Klick stellte die Wiedergabe wieder
  her; `wpctl` meldete erneut `Volume: 0.64` ohne `MUTED`. Netzwerk meldete
  `connected`, der Bluetooth-Adapter `Powered: yes`; diese Zustände entsprechen
  dem Live-Bild.

## Noch offen

- Deck bei 1366×768 und mit geöffneten App-Fenstern live vergleichen.
- Zweiten Abmelden-Klick und tatsächliches Sitzungsende erst in einem
  geplanten Login-Rundlauf prüfen.
- Netzwerk und Bluetooth in fehlenden/pending/Fehlerzuständen weiter live
  abnehmen.

## Eingabeverzögerung vom 25.09.2026

Der Nutzer beobachtete beim ersten Klick im bereits geöffneten Deck nach
einem Shell-Neustart eine ausbleibende Reaktion; spätere Netzwerk- und
Lautstärkeaktionen liefen zügig. Fünf direkte Fedora-Messungen der vier
verwendeten `nmcli`-Abfragen dauerten jeweils rund 10 ms. Der Befund war nach
dem nächsten Shell-Neustart nicht erneut auslösbar. Der Nutzer hat den
Erstklick im zuletzt installierten Release ausdrücklich erneut getestet:
Er funktioniert. Die Ursache des einmaligen früheren Aussetzers ist damit
nicht nachgewiesen.

Der WLAN-Reiter enthielt dennoch zwei synchrone `nmcli`-Abfragen auf dem
Wayland-Ereignisthread. Er zeigt jetzt sofort die letzte Netzliste und
verwendet den vorhandenen Hintergrund-Refresh; dessen Ergebnis zeichnet den
offenen WLAN-Reiter erneut. Während genau dieses Refreshs läuft, prüft der
Timer viermal pro Sekunde statt einmal; im geschlossenen Zustand entsteht
keine zusätzliche Arbeit. Langsame Deck-Zeichnungen und der Netzwerk-Poll
melden ab 50 ms eine Diagnosewarnung.

Der erste Fixstand bestand auf Fedora `cargo fmt --all -- --check`, `cargo check
--workspace --locked`, `cargo test --workspace --locked -q`, `cargo clippy
--workspace --all-targets --locked -- -D warnings` und `cargo build --release
-p niwoe-shell --locked`. Release-SHA-256:
`d9b8a03bc6ed7b57c7daccdf2024b89cc27436ca97aeed807d67262c6ae038b0`.
Er wurde installiert; der Nutzer meldete bei einer Wiederholung direkt danach
zügige Netzwerk- und Lautstärkereaktionen.

Beim Test fiel eine zweite Bedienregression auf: Der WLAN-Reiter blieb nach
Schließen des Decks ausgewählt, sodass beim erneuten Öffnen nur die Netzliste
statt des Systemdecks erschien. Der sichtbare „Status“-Reiter führt sofort
zurück. Der Öffnungspfad setzt nun den Reiter wieder auf `Status`. Der
korrigierte Stand bestand erneut auf Fedora `cargo fmt --all -- --check`,
`cargo check --workspace --locked`, `cargo test --workspace --locked -q`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` und
`cargo build --release -p niwoe-shell --locked`. Release-SHA-256:
`600140d9caf60008cc4a0938f947451f8603630187567e0f4e66d430be175833`.
`target/release/niwoe-shell`, `/usr/local/bin/niwoe-shell` und der laufende
Prozess `/proc/41142/exe` sind bytegleich. Der Nutzer bestätigte live den
Ablauf Deck öffnen → Netzwerk → Deck schließen → Deck erneut öffnen: Die
Systemdeck-Startansicht erscheint wieder.
