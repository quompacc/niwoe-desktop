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

## Fortsetzung 26.09.2026: Live-Grundlage und Reconnect

Ausgangsstand: `190c896`, Branch `codex/niwoe-p00`, sauberer lokaler
Arbeitsbaum. Fedora enthält eine synchronisierte Quellkopie ohne `.git`.
Die Sitzung wurde inzwischen neu gestartet: Compositor PID 1357 und Shell
PID 1371 laufen seit 08:03:42 Uhr. Die SHA-256-Werte von `/proc/1357/exe`
und `/proc/1371/exe` entsprechen den oben dokumentierten installierten
P06-Releases. Der zuvor offene Neulogin für die Grundlage ist damit erledigt.

Live auf diesem DRM-Compositor geprüft: zwei temporäre Räume erzeugen,
Name/Beschreibung/Zuordnung ändern, Reihenfolge ändern, veraltete Revision
abweisen, Raum wechseln, ein natives Zenity-Fenster im Quellraum öffnen,
Quellraum mit explizitem Ziel löschen. Fenster-ID und Inhalt bleiben erhalten;
der Snapshot meldet das Fenster und den aktiven Raum im verdichteten Zielslot.
Ein neuer IPC-Client erhält den Raumzustand; IDs, Reihenfolge und Revision
stimmen mit `rooms.toml` überein. Eigene Testfenster und Testräume wurden
anschließend entfernt. Die ursprünglichen neun Raumdefinitionen einschließlich
„Entwicklung“ sind unverändert. Revision und monotoner ID-Zähler wurden durch
die Tests erwartungsgemäß erhöht. Kein Sitzungsende und keine Änderung an
KDE-/GTK-Konfigurationen. Testskript: lokales `target/p06-live-probe.py`.

Dabei festgestellt: `IpcServer::poll` meldete neue Socketverbindungen als
Auslöser der Anfangssnapshots. Trifft die Authentifizierung erst im nächsten
Poll ein, werden die Snapshots zuvor nur an bereits authentifizierte Clients
gesendet; dem neuen Client fehlen Fenster-/Outputdaten. Der Test benötigte
auf dem alten Release deshalb einen zusätzlichen Verbindungsimpuls.

Korrektur in `state/ipc/server.rs` und `commands.rs`: Anfangssnapshots werden
beim erstmaligen erfolgreichen Übergang zur authentifizierten Shell gesendet.
Öffentliche Verbindungen lösen sie nicht mehr aus; wiederholte Authentifizierung
erzeugt keine zusätzlichen Snapshots. `server_tests.rs` prüft ausdrücklich
Verbindungsannahme und Authentifizierung in getrennten Polls.
Keine Wire-Änderung, kein neuer Timer und keine zusätzlichen Idle-Abfragen.

Fedora: `cargo check --workspace`, `cargo test --workspace -q`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check` und
`cargo build --release -p niwoe --locked` bestanden. Workspace-Tests enthalten
den neuen Reconnect-Test sowie Design-, Source-Size- und Centralization-Guards.
Logs: `target/p06-reconnect-{check,test,clippy,build}.log` auf Fedora.
Der neue Release bestand außerdem den isolierten Nested-Smoke mit nativer
Shell und Zenity sowie Rename/Reorder, Konflikt, Persistenz und Reconnect:
`target/p06-reconnect-nested.log`, Detailbeleg
`target/p01-evidence/nested.ykBowb`. Die geprüften Elternkonfigurationen blieben
bytegleich. Die isolierte Umgebung meldete fehlende AT-SPI-/PipeWire-Anbindung;
Audio und Barrierefreiheit sind durch diesen Smoke nicht abgenommen.

Neuer Compositor-Release SHA-256:
`dd250a551741878b0914e6b90fb6d6436a4a118a2cc09e6ba7cccfd59890d52a`.
Die erste Installationsprüfung verlangte nach dem Neulogin erneut sudo-
Authentifizierung. Nach anschließender Nutzer-Authentifizierung wurde
der Fix installiert und bytegleich geprüft; die laufende Sitzung blieb erhalten.
Dieser Zwischenrelease wird durch den folgenden gemeinsamen P06-Stand ersetzt.

P06 bleibt **in-progress**. Nicht geprüft: minimierte/Floating-/Tiling-Fälle,
Dialogvererbung, XWayland, zwei Outputs/Hotplug, vollständiger Neustartnachweis
der mutierten Persistenz und vollständige Fehler-/Grenzfallmatrix im Livebetrieb.
Die sichtbaren Create/Delete-/Metadatenabläufe sind weiterhin offen.

Die anschließende Implementierung dieser sichtbaren Abläufe wird separat in
[P06_VISIBLE_FLOWS.md](P06_VISIBLE_FLOWS.md) dokumentiert. Der vorstehende
Grundlagenstand ist ein historischer Checkpoint, keine vollständige Abnahme.
