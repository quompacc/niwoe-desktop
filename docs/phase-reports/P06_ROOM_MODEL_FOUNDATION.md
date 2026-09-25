# P06 – Raummodell: dynamische Grundlage (25.09.2026)

## Stand

P06 ist begonnen, aber noch nicht vollständig abgenommen. Bootsplash,
Loginmanager, PAM und Sitzungsstart wurden nicht geändert. Der bestehende
persistente Raumkatalog mit stabilen IDs, Revision, Validierung und atomarer
Speicherung wurde auf dynamische Mutationen erweitert.

## Implementiert

- `RoomChange` trägt nun Create, Delete mit explizitem Zielraum,
  SetDescription und SetAssignment zusätzlich zu Rename/Move. Der vollständige
  Room-Snapshot überträgt Beschreibung und Zuordnungsmodus; fehlende neue Felder
  werden für ältere Gegenstellen als leer/Free gelesen.
- `RoomRegistry` vergibt monotone IDs, erhält die vom Anzeigenamen unabhängigen
  Slots, erlaubt 1–64 Räume, verhindert das Löschen des letzten Raums und
  veröffentlicht nur gespeicherte Änderungen. Bei einem bereits publizierten,
  aber nicht vollständig synchronisierten Dokument meldet sie Durability und
  übernimmt den publizierten Zustand.
- Der Compositor ergänzt/entfernt echte `Space`- und WM-Einträge. Beim Löschen
  wandern sichtbare Fenster mit Position und Floating-Eigenschaft in den
  ausdrücklich gewählten Zielraum. Minimierte Fenster, aktiver Index und
  Output-Zuordnungen werden auf die verdichteten Slots umgesetzt.
- Shell-Zähler, IPC-Index und Snapshot-Prüfung akzeptieren die fachliche Grenze
  von 64 Räumen. Die Verwaltung zeigt höchstens neun Karten pro Seite;
  Mausrad und Tastaturfokus erreichen weitere Seiten. „Neuer Raum“ bleibt in
  dieser P06-Grundlage sichtbar deaktiviert, bis der vollständige
  Erzeugungs-/Bearbeitungsablauf angebunden ist.
- Suchfeld, „Neuer Raum“, Schnellaktionen und Statistik verwenden dieselbe
  rechte X-Kante. Die native Vorschau wurde auf Fedora gerendert und geprüft.

Performance: Raum-Mutationen laufen nur auf ausdrücklichen IPC-Befehl; die
zusätzliche Index- und Fenstermigration ist auf höchstens 64 Räume begrenzt.
Die Verwaltungsseite zeichnet pro Bild höchstens neun Raumkarten. Keine neue
Polling-Schleife oder pro-Frame-Dekodierung.

## Verifikation und Release

- Fedora: `cargo check --workspace` grün.
- Fedora: `cargo clippy --workspace --all-targets -- -D warnings` grün.
- Fedora: `cargo test --workspace -q` grün, einschließlich neuer Tests für
  Create/Delete/Metadaten, Slotverdichtung, Fenstermigration, Output-Remapping,
  dynamische Snapshots, Seitenzuordnung und rechte Geometrie.
- `cargo fmt --all` ausgeführt; `cargo build --release -p niwoe -p niwoe-shell --locked` grün.
- Native Verwaltungsvorschau auf Fedora erzeugt und visuell geprüft:
  `target/p06-rooms.png`.
- Installierter Compositor ist bytegleich zum Release
  `7627a192a60cb689b5519bc1ebb854affffb70f9735af5db63ab2f398d89c4e2`.
  Der derzeit laufende Compositor-Prozess 1333 hat noch die alte Buildidentität
  `83e59b6809844acf1c0a0ed63f5951627c8de91209b3e2d4059ba0fb85fec4f6`.
- Installierte und per Watchdog neu gestartete Shell ist bytegleich zum Release
  `8fe72a6ddb3302ae3071f6be0a0f446bd1e82637193af372cc6e76bae772fe96`
  (Prozess 122427 zum Prüfzeitpunkt).

## Offen für P06-Abnahme

- NIWOE-Neulogin, damit der installierte Compositor-Release tatsächlich läuft.
- Live-Prüfung von Create/Delete mit Fenstern, minimierten Fenstern, Floating,
  Tiling, Dialogen, XWayland und zwei Outputs. Die Unit-Tests beweisen noch
  keine vollständige Integration aller Laufzeitpfade.
- UI-Bedienweg für Erzeugen/Löschen und Metadatenbearbeitung samt Fehler- und
  Konfliktanzeige. Der derzeit deaktivierte „Neuer Raum“-Button ist keine
  vorgetäuschte Funktion.
- Zuordnungsregeln, Start-Apps und Restore bleiben die ausdrücklich späteren
  P07–P10-Arbeiten des aktiven Plans.
