# Native Shell: Panel, Launcher, System-Deck

Dauerhafte [Bildbelege mit Herkunft und Grenzen](../design/evidence/P03/README.md).
Aktueller Einstieg für Folgesitzungen: [Handoff](P02_HANDOFF.md).

Aktueller Stand 23.09.: [Symbolaktionen und Bluetooth](P05_DECK_SYMBOLS.md),
geprüft, installiert und im laufenden Desktop angesehen. Keine Gesamtabnahme.

Fortsetzung 23.09.: [Deck-Komposition, erster Korrekturschritt](P05_DECK_COMPOSITION.md).
Kompakte Audiozeile, ruhige Abschnittstrennung und Settings im Fußbereich;
Dark/Light nativ geprüft und Release installiert. Rechner am Greeter; Live-Prüfung
nach Anmeldung und vollständige Mockup-Abnahme offen.

## Visuelle Korrektur nach Live-Review

Nutzerfeedback: Statussymbole/Uhr passen nicht, Launcher zu flach/generisch;
Programmsymbole gehören nicht ins Panel. Alle acht Originalmockups erneut
angesehen. Sie sind jetzt visuell verbindlich; Apple Spotlight ist zusätzlich
für Launcher/Suche freigegeben. Siehe aktualisierte Projektregeln und Manifest.

- Panel: angeheftete App-Buttons entfernt; Räume, Statusgruppe und Uhr bleiben.
  Dunklere Basisfläche, aktive Räume mit feiner Außenkontur wie im Desktopmockup.
- Statusgruppe: einheitliche native Kontursymbole statt unterschiedlich
  proportionierter Theme-Raster; Akku mit echtem Prozentwert. Symbolcache auf
  32 Einträge begrenzt, Schlüssel aus Symbol/Zustand/Farbe, pro Shell-Lebensdauer.
  Keine neue Animation, kein neuer Polling-Takt oder Dekodierung im Renderpfad.
- Uhr: Datum mit Wochentag/Monat und Uhrzeit auf einer gemeinsamen Grundlinie.
- Suche: großzügige Suchzeile, Suchsymbol, Trefferanzahl, feinere Trennung,
  App-Kategorien bzw. tatsächlicher Programmname statt repetitivem „Anwendung“;
  Tastenkappen und klarer markierte Ergebniswahl. Farben/Abstände bleiben zentral.
- Dateien: `panel_view/{layout,chips,rooms,render,status_symbols}.rs`,
  `panel_view.rs`, `panel_view_tests.rs`, `wayland/time.rs`, `app_view.rs`,
  `app_view_chrome.rs`, `niwoe-tokens/src/chrome.rs` und Designregeln.

Noch nicht umgesetzt (keine technisch unvermeidbaren Designabweichungen): Räume haben noch keine persistenten Namen;
der aktuelle Launcher durchsucht Anwendungen, noch keine Dateien/Systemaktionen.
Das Wallpaper und die übrigen noch unfertigen Oberflächen sind dadurch nicht
automatisch mockupgetreu. Diese Korrektur ist keine Gesamtabnahme.

Prüfung der Korrektur: `cargo fmt --all -- --check`,
`cargo check --workspace --locked`, `cargo test --workspace --locked`
(1.093 bestanden, 0 fehlgeschlagen, 1 ignoriert, einschließlich Design-Guard)
und `cargo clippy --workspace --all-targets --locked -- -D warnings` bestanden.
Drei überholte Tests für entfernte Pinned-Widgets/Batterie-Themeicons entfallen;
der bestehende Panel-Integrationstest prüft jetzt ausdrücklich, dass selbst bei
konfigurierten Pins keine App-Startflächen mehr entstehen. Hit-Zonen bleiben
an den Widgetbaum gekoppelt; Launcher-Treffergeometrie folgt dem Header-Token.

Bedienreferenz: [Apple Spotlight](https://support.apple.com/guide/mac-help/find-what-you-need-with-spotlight-mchlp1008/mac).
Die Freigabe umfasst die Suchoberfläche, nicht automatisch Apples Datenquellen.

Release-Build (1m51s) und Installation bestanden; sechs Binärdateien bytegleich
zum Release, gemeinsame KDE-/GTK-Konfiguration unverändert. Shell über bestehenden
Watchdog erneuert (PID 173594), laufender Compositor/Sitzung erhalten. Erster
Screenshotversuch kam vor der IPC-Bereitschaft; erneuter Versuch erfolgreich.
Echte Screenshots `target/p03-evidence/panel-live-after.png` und
`launcher-live-after.png` angesehen: App-Pins weg, Statussymbole/100%-Akku und
Datum/Uhrzeit auf gemeinsamer Linie, Suchfeld und echte App-Ergebnisse sichtbar.
Das ist eine überprüfte Zwischenkorrektur, keine Behauptung vollständiger
Mockup- oder Spotlight-Funktionsgleichheit. Light-/HiDPI-Live-Abnahme noch offen.

## Vorheriger Implementierungsstand

Stand: 22.09.2026. Auf Nutzerauftrag gemeinsam neu aufgebaut, vorhandene
Rust-Backends erhalten. Omarchy ist die Bedienreferenz. Noch keine visuelle Abnahme.

## Änderungen

- `niwoe-shell/src/panel_view/{layout,rooms}.rs`: obere Leiste mit neutralem
  Launcher, direkter Raumwahl, Überlaufzugang und vorhandenen Statusmodulen.
  Sichtbare Raumzahl richtet sich nach Platz und aktivem Raum.
- `niwoe-shell/src/app_view.rs`, Tests, `launcher.rs`, Wayland-Launcher-Render
  und Input: zentrierte 640×480-Suche statt Kategorienraster. Mehrwortsuche,
  stabile Sortierung, Terminal-Anwendungen, Pfeilauswahl, Enter, Escape und
  größenabhängige Scroll-/Treffergeometrie. Settings bleiben separat 880×620.
- `niwoe-shell/src/quick_settings_popup.rs`, `deck_keyboard.rs`,
  `wayland/state/deck_actions.rs`: 360×400-Systemkarte mit Audio, Netzwerk,
  Leistungsprofil, Darstellung, Energie und Sitzungsaktionen. Nicht verfügbare
  Dienste haben keine aktiven Treffer-/Tab-Ziele. Maus und Tastatur verwenden
  dieselben Aktionen; Lautstärke mit Links/Rechts.
- `niwoe-config/src/keybind/*`, `niwoe-ipc/src/lib.rs`, Shell-IPC:
  Super+Space Suche, Super+Alt+Space Einstellungen, Super+Escape Deck,
  Super+Return Terminal, Super+Shift+Return Browser, Super+Shift+F Dateien,
  Super+W Schließen und Super+Ctrl+L Sperren.
- Compositor `input/{keyboard,navigation}.rs`, Modifier-Drag, Decoration-Modell
  und `niwoe-wm`: Tiling als Standard, Super+T einzelnes Fenster freistellen,
  Super+F Vollbild, Richtungsfokus, Kacheltausch, Raum-/Fensterwechsel.
  Keine SSD-Titelleiste, zentrale 1–2-Pixel-Kontur. App-CSD bleibt erhalten.
- `niwoe-tokens/src/{chrome,launcher_geometry}.rs`: zentrale Geometrien.
  Tiling berücksichtigt den Arbeitsbereich des zugeordneten Ausgabegeräts;
  X11-Fenster erhalten die berechnete Kachelgeometrie.

## Verifikation

Fedora-Testrechner, Rust 1.98.1:

- `cargo fmt --all -- --check`: bestanden.
- `cargo check --workspace --locked`: bestanden.
- `cargo test --workspace --locked`: **1.096 bestanden, 0 fehlgeschlagen,
  1 ignoriert**, einschließlich Design-Guard.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: bestanden.
- `git diff --check`: bestanden.
- `cargo build --release --workspace --locked`: bestanden (1m52s).
- Release auf Acer installiert; alle sechs installierten Binärdateien stimmen
  bytegenau mit dem Build überein. Gemeinsame KDE-/GTK-Dateien unverändert.
  Installer meldete fehlenden User-Bus im sudo-Kontext; anschließendes
  `systemctl --user daemon-reload` mit expliziter Sitzungsumgebung erfolgreich.
- Alte Testsitzung (Compositor PID 116545) beendet, Greeter aktiv. Benutzer um
  erneute Anmeldung gebeten; Live-Screenshot und Bedienprüfung stehen aus.
- Native Rasteransichten geprüft: `target/p03-evidence/panel-native.png`,
  `launcher-native.png`, `deck-native.png`. Das Deck-Testbild komponiert die
  transparente Vordergrundebene auf die Theme-Tönung; kein Live-Screenshot.
- Input-Callflow geprüft: Compositor-Keybind → IPC → Shell-Popup/App-Start;
  Deck-Maus-/Tastaturaktionen → gemeinsamer Dispatcher → vorhandene Dienste.

## Leistung

Kein neuer Animations-/Polling-Loop. Zeichnen bei vorhandenen Invalidierungen;
Schrift-/Icon-Caches bleiben erhalten, Blur beim Compositor. Raumlayout ist
auf neun Räume begrenzt. Launcher filtert/sortiert bei Suche/Neuzeichnung die
App-Liste; keine Icon-Dekodierung pro Frame.

## Offen

- Nach erneuter Anmeldung echte laufende Sitzung prüfen und fotografisch belegen.
- Visuelle Abnahme beider Themes, Zielauflösungen, Skalierungen und realer Apps.
- Omarchy: Super+K-Kürzelhilfe, Super+Mausrad, Trennung von Verschieben mit
  Folgen / stillem Verschieben. Super+Shift+1…9 verschiebt derzeit ohne Folgen.
- Durchgängige Tastaturnavigation in Settings und WLAN-Detailansicht.
- Suchanbieter für Settings/Systemaktionen, sichtbare Startfehler und stabile
  Auswahlidentität beim asynchronen Aktualisieren der App-Liste.
- HiDPI-/Idle-Messungen aus P02. P03–P05 sind in Arbeit, nicht accepted.
