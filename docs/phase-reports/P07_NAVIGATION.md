# P07 – Raumleiste und vollständige Raumauswahl

**Aktueller Abschluss:** P07 einschließlich visueller DRM-Prüfung mit 1/9/64
Räumen und zwei Outputs ist abgeschlossen. Siehe
[P07-Abschlussabgleich](P07_ACCEPTANCE_REVIEW.md). Der folgende Bericht bleibt
historischer Nachweis des ersten Teilblocks.

Fortsetzung: [App-Zuordnung, zweiter Block](P07_APP_ASSIGNMENT.md). Die hier
noch als offen beschriebene App-Policy ist inzwischen implementiert; die
Gesamtphase bleibt offen. Nachstehend der historische erste Navigationsblock.

26.09.2026. **P07: in-progress, nicht accepted.** Erster geprüfter und
installierter Baustein: Raumdarstellung, Overflow und Tastaturnavigation.
P06 bleibt abgeschlossen. App-Regeln und der vollständige Fensterzugang werden
durch diesen Bericht nicht als umgesetzt oder abgenommen dargestellt.

## Änderungen nach Datei

- `niwoe-shell/src/panel_view/rooms.rs`: feste Seiten in gespeicherter Reihenfolge
  halten den aktiven Raum sichtbar. Wechsel innerhalb einer Seite verschiebt
  keine Reiter; kein MRU-Sortieren. Neutraler Belegungspunkt im Raumsymbol.
- `panel_view/layout.rs`, `panel_view/render.rs`: echte Raumanzahl und Belegung
  werden bis zum Reiter durchgereicht; Overflow zählt tatsächlich verborgene Räume.
- `wayland/render/core.rs`, `wayland/handlers/pointer/panel_and_popups.rs`:
  fest verdrahtete Neun-Räume-Grenzen aus Render- und Trefferprüfung entfernt.
  Beide Wege verwenden dieselben vollständigen Snapshotdaten.
- `workspaces.rs`: seitenweise Auswahl aller 1–64 Räume, sichere Bereichsgrenzen
  bei Snapshotänderungen, neutrales Belegt-Signal, Gold nur für aktiven Raum.
  Zusätzlicher neutraler Tastatur-Fokusrahmen aus Controls-Tokens.
- `workspaces/details.rs`: vollständiger Name bei Hover/Tastaturauswahl sowie
  Zurück/Weiter und Seitenstand; alle neuen Geometriewerte aus Tokens.
- `wayland/handlers/keyboard/workspace_navigation.rs`, `keyboard.rs`: Pfeile,
  Tab/Shift-Tab, Bild auf/ab, Pos1/Ende, Enter, Escape und F2 für den ausgewählten
  Raum. Navigation allein aktiviert keinen Raum und startet keine Programme.
- `wayland/state/popups.rs`: Öffnen beginnt auf der Seite des aktiven Raums.
- `wayland/state/panel_actions.rs`, `wayland/types.rs`: Seitenaktionen bleiben
  innerhalb des Popups; Enter/Klick übergeben den tatsächlichen Workspace-Slot.
- `wayland/render/calendar_workspace.rs`: obsolete konstante Raumanzahl entfernt.
- `niwoe-tokens/src/chrome.rs`: zentraler Zusatzbereich für vollständigen Namen
  und Seitennavigation; Raster bleibt geometrisch unverändert. Popuphöhe 392,
  Footer 144, Zeilenhöhe 18, Navigationshöhe 32 logische Pixel. Die Vergrößerung
  schafft Platz für zulässige lange Namen und 64-Räume-Navigation; das Panel,
  die zentrierte Uhr und die bestehenden großen Hub-/Verwaltungsseiten behalten
  ihre Gestaltung.
- `panel_view_tests.rs`, Tests in `rooms.rs` und `workspaces.rs`: feste Seiten,
  aktiver Raum sichtbar, 1/9/64 Räume, Reihenfolge unabhängig von Slotnummer,
  Verkleinerung eines Snapshots sowie Raum-64-Klickziel ohne Klemmen auf 9.

Keine Cargo.toml-/Dependency-Änderungen. Native Rust-UI, verbindliches grünes Theme.
Keine neuen Effekte, Asset-Decodes, Timer oder Hintergrundabfragen. Panel nutzt
die vorhandene Render-Signatur einschließlich Belegung; Raum-Snapshots
invalidieren sie bereits. Popup arbeitet mit höchstens neun Karten und einem
vollständigen Namen (maximal 64 Zeichen) je bestehendem Zeichenvorgang. Keine
Arbeit für geschlossene Popups; Material/Blur bleibt beim Compositor.

## Verifikation

Auf Fedora bestanden:

- `cargo check --workspace`
- `NIWOE_WORKSPACE_PREVIEW_PNG=/home/eduard/niwoe-desktop/target/p07-room64.png cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo build --release -p niwoe-shell`

Logs: `target/p07-navigation-{check,tests,clippy,fmt,build}.log`.
Design Guard ist Teil der grünen Workspace-Tests. Alle geänderten Rust-Dateien
bleiben unter 600 Zeilen. Render-/Pointer-/Keyboard-/IPC-Call-Flow abgeglichen.

DRM-Live mit den vorhandenen neun Räumen: Popup per Panelklick geöffnet,
`Ende, Enter` aktiviert tatsächlich Raum 9; Panel enthält danach `panel-room-9`
und meldet acht verborgene Räume. Abschließender Stand nach Watchdog-Erneuerung
mit Pfeilnavigation und getrenntem aktiven Raum/Fokusrahmen aufgenommen und
visuell geprüft: `target/p07-final.png` (lokal),
`target/p06-click-p07-final.png` (Fedora). Vorheriger Raum 1 wieder ausgewählt.

Isolierter echter Compositor mit 64 Räumen, Profil
`/tmp/niwoe-p07-ui.xxci94am`: native uinput-Klicks/Tasten erreichen über
`Ende, Enter` Raum 64; Panel-Klickzonen zeigen Räume 61–64. Mausklick auf
„Zurück“, dann Enter aktiviert Raum 55. `Pos1, Tab, Enter` aktiviert Raum 2.
IPC-Nachweise: `end64.json`, `previous-page.json`, `home-tab.json` im Profil.
Testprozessgruppe anschließend beendet; Benutzerraumdatei nicht verändert.

**Grenze dieses Nachweises:** Der isolierte Winit-Compositor zeigte die
Shell-Flächen im Screenshot nicht korrekt, obwohl Layer/Input und IPC arbeiteten.
Das ist ein Eingabe-/Zustandsnachweis, keine visuelle 64-Räume-Live-Abnahme.
Die letzte Seite samt langem Namen wurde zusätzlich mit dem tatsächlichen nativen
Renderer als `target/p07-room64.png` erzeugt und geprüft. Vollständige visuelle
64-Räume-/Zweimonitor-Abnahme bleibt Teil der P07-Abschlussrunde.

## Installation

Finale Shell gebaut, installiert und laufend bytegleich bestätigt:
`24a5b88cb572d08fa5e63d5dd7656f807ef6c5992f4f5fe193ea059327add08d`.
Shell PID 124711, über vorhandenen Watchdog erneuert. Kein Neulogin erforderlich.
Compositor PID 109259 unverändert:
`51ba3d9b7b10eeee4ff3bd51ff38fec68aa7e9bd3a1b9b5c54e806d3e19cfc28`.
KDE-/GTK-Konfigurationsdateien vor/nach Installation identisch.

## Verbleibende P07-Pakete

1. P07-03: bestehende nummerierte Shortcuts behalten ihre P06-Zuordnung;
   gleichwertige Mausaktionen zum Fensterverschieben fehlen noch.
2. P07-04: reine Zuordnungsentscheidung für Preferred/Dedicated, getrennte
   Native-/XWayland-Identitäten, Startkorrelation und Lifecycle-Anbindung;
   manuelle Moves und Transients dürfen nicht überschrieben werden.
   Semantischer Dedicated-Hinweis gehört zu dieser Anbindung.
3. P07-05: nachgewiesener Zugriff auf alle Fenster einschließlich minimierter
   und mehrerer Fenster derselben App; Tray/Benachrichtigungen weiter erreichbar.
4. Gesamtabnahme: Preferred ohne Fokusdiebstahl, Dedicated ohne Aussperren,
   spätes App-ID, zwei Outputs, 1/9/64 Räume, kein App-Start durch Raumwechsel.

Die P07-Phase wird erst nach diesen Paketen und echten Endtests geschlossen.
