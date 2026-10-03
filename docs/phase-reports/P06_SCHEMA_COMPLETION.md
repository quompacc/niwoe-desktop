# P06 – vollständiger Datenvertrag und manueller Fenster-Move

Stand: 26.09.2026. **Accepted für den dokumentierten Fedora-Aufbau nach finalem Neulogin.**
Dieser Bericht ergänzt den [Abschlussabgleich](P06_ACCEPTANCE_REVIEW.md).
P07 wurde nicht begonnen. Keine Cargo-/Dependency-Änderungen.

## Änderungen

- `niwoe-config/src/rooms/preferences.rs`, `rooms.rs`: Schema 2 enthält pro Raum
  `preferences` mit optionalem Freedesktop-Iconnamen, geordneten exakten
  App-Referenzen (`native`/`xwayland`), Layout (`tiling`/`floating`) und Restore
  (`disabled`/`layout-only`/`relaunch-apps`). Keine Prozesshandles oder Befehle.
  Standard: kein eigenes Icon, keine App-Regeln, Tiling, Restore ausgeschaltet.
  Iconnamen: höchstens 128 ASCII-Bytes, Buchstaben/Ziffern/Bindestrich/Unterstrich/
  Punkt, kein führender Punkt. App-IDs: nicht leer, höchstens 255 Bytes, keine
  äußeren Leerzeichen/Steuerzeichen; höchstens 64 eindeutige typisierte Referenzen.
  Nicht installierte Apps bleiben gültige Referenzen. Die Reihenfolge bleibt erhalten.
- `niwoe-config/src/rooms/store.rs`: Schema 1 wird validiert und ohne Änderung
  von Revision, IDs, Zähler, Reihenfolge oder bisherigen Metadaten übernommen.
  Vor Ersetzung entsteht die bytegleiche private `rooms.toml.v1.bak` atomar.
  Wiederholung akzeptiert ausschließlich dieselbe reguläre Sicherungsdatei.
  Andere Sicherungen, unbekannte Schemata und defekte Dateien werden erhalten
  und mit Fehler gemeldet. Schreiben bleibt atomar und auf 128 KiB begrenzt.
- `niwoe-ipc/src/room_preferences.rs`, `rooms.rs`, `lib.rs`: unabhängige typisierte
  Wire-Daten, additive Snapshot-Felder und revisionsgesichertes `set-preferences`.
  Alte Snapshots ohne Präferenzen bekommen Defaults. Unbekannte Enumwerte oder
  Präferenzfelder werden abgewiesen.
- `niwoe-compositor/src/room_preferences.rs`, `room_registry.rs`,
  `state/ipc/rooms.rs`: expliziter Wire-/Config-Adapter, zentrale Validierung,
  vollständige Snapshots und atomare Mutation. Normale Metadatenänderungen und
  Umsortieren erhalten Präferenzen.
- `niwoe-shell/src/room_editor.rs`: Snapshot übernimmt alle Felder und verwendet
  die Config-Validierung. Kein zweiter Policy-Pfad; Konvertierung nur beim
  Snapshotempfang, keine Arbeit pro Renderframe. Weitere RoomEntry-Konstruktoren
  in Hub, Verwaltung, Konfiguration und Launcher erhalten identische Defaults.
- `niwoe-compositor/src/state/layout/workspace.rs`: manueller Move übernimmt
  Tiling-/Floating-Mitgliedschaft zwischen den WM-Workspaces und aktualisiert
  beide Layouts. Fokus-Suche erfasst auch die Wayland-Surface eines XWayland-
  Fensters. Der bisherige Pfad hatte nur die Space-Mitgliedschaft verschoben.
- Tests in Config, Registry, IPC und Shell sowie `scripts/test-room-errors.py`
  und `scripts/test-room-persistence.py`: Migration/Sicherung, Validierung,
  vollständiger Roundtrip, ungültige atomare Mutation, Edit/Reorder/Restart,
  Reconnect und unveränderte stabile Identitäten.

Dies sind persistente Einstellungen, noch keine automatische App-Zuordnung und
kein ausgeführter Restore. Die zugehörigen Bedien-/Ausführungspfade folgen gemäß
Plan in P07–P09. Die bestehende Oberfläche wurde visuell nicht umgestaltet.
Keine neuen Effekte, Assets, Animationen oder regelmäßigen Hintergrundarbeiten.

## Verifikation

Fedora: `cargo check --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check` und
`cargo build --release -p niwoe -p niwoe-shell` erfolgreich.
Logs: `target/p06-preferences-{check,tests,clippy,fmt,build}.log`.
Geänderte Rust-Dateien bleiben unter 600 Zeilen. Input-/IPC-Call-Flow geprüft.

Auf dem bisherigen DRM-Build `e1953fc4…` wurde der tatsächliche
`Super+Shift+3`-Pfad separat ausgelöst: genau das fokussierte native Testfenster
von Raum 1 nach Raum 3, gleiche Fenster-IDs und Inhalte, andere Testfenster
unverändert, keine Raumdatenmutation. Zielraumwahl/Fokussieren und vollständige
Testclient-Bereinigung geprüft. Evidenz: `target/p06-live-ui/move-*.json`.
Ein erster Testversuch hatte die Shift-Taste am virtuellen Gerät nicht registriert;
nach Korrektur des Testgeräts bestand der Move. Das ist kein Produktfehler.
Die spätere WM-Korrektur wurde anschließend separat auf dem neuen Compositor
nachgeprüft; der frühere Move-Nachweis allein wäre dafür nicht ausreichend.

## Installation und endgültige Abnahme

Endstand installiert und bytegleich gegen Release geprüft:

- `niwoe`: `51ba3d9b7b10eeee4ff3bd51ff38fec68aa7e9bd3a1b9b5c54e806d3e19cfc28`
- `niwoe-shell`: `288e71b235026d81951325881000a300de8226eae76f45a93d72aaf3ce478c80`

KDE-/GTK-Dateien vor/nach Installation identisch. Beide Prozesse wurden gemeinsam
beim vom Nutzer ausgeführten Neulogin aktiviert; kein automatischer Sitzungsabbruch
und keine vorzeitige Migration der Datei des noch laufenden alten Compositors.

Endgültige isolierte Integrationsläufe bestanden:

- `dbus-run-session -- python3 scripts/test-room-errors.py /run/user/1000/wayland-1`:
  `target/p06-preferences-errors.log`, `/tmp/niwoe-p06-errors.82xqw8qg`.
- `NIWOE_ROOM_SMOKE=1 NIWOE_ROOM_TRANSIENT_SMOKE=1 NIWOE_ROOM_RESTART_SMOKE=1 bash scripts/smoke-nested.sh /run/user/1000/wayland-1`:
  `target/p06-preferences-smoke.log`, `target/p01-evidence/nested.lgpNm7`.
  Lifecycle, Transients, Hintergrundbereinigung und echte Prozess-Neustart-
  Persistenz einschließlich aller Präferenzen sowie neutraler Loge bestätigt.
  Die isolierte Umgebung protokolliert weiterhin Accessibility-/GTK-Warnungen;
  alle fachlichen Assertions und Exitcodes waren erfolgreich.

Nach dem Nutzer-Neulogin sind beide laufenden `/proc/<pid>/exe`-Hashes identisch
mit den oben installierten Releases: Compositor PID 109259, Shell PID 109277.
Die Loginaufnahme `target/p06-schema2-login.json` zeigt neutrale Loge (Workspace 0),
keine Fenster und den vollständigen Snapshot. Die echte Raumdatei hat Schema 2,
weiterhin Revision 49/Zähler 19 und identische IDs, Namen, Reihenfolge,
Beschreibungen und Zuordnungen. Die private Sicherung (0600) enthält exakt den
vorher erfassten Schema-1-Datenstand. Neue Präferenzen haben die inaktiven Defaults.
Der bytegenaue Sicherungsvorgang ist zusätzlich in Unit- und Integrationstests belegt.

Auf diesem DRM-Endstand anschließend erfolgreich:

1. Native Testfenster: Floating per `Super+T`, tatsächlicher Move mit
   `Super+Shift+3`. Nur das fokussierte Fenster wechselt Raum 1 → 3;
   IDs/Inhalte bleiben gleich, minimiertes Fenster bleibt in Raum 1.
2. Im Zielraum bewirkt der erste `Super+T`-Druck tatsächlich Tiling, der zweite
   wieder Floating. Das prüft die mitgeführte WM-Mitgliedschaft. Texteingabe
   erreicht anschließend nur das verschobene fokussierte Fenster.
3. Separater normaler Tiling-Move in denselben Zielraum: Inhalt/IDs erhalten,
   normales Fenster tiled, anderes Fenster weiterhin floating, minimiertes
   Quellfenster unverändert. Evidenz: `target/p06-live-final/move-*.json`.
4. XWayland-Fenster über denselben Shortcut verschoben; IDs, Inhalte und andere
   Fenster bleiben erhalten. Eingabe nach direktem Klick in das Ziel-Textfeld
   bestätigt. Der vorherige Versuch nur mit IPC-Fokus führte beim GTK-X11-
   Testclient zu keiner Texteingabe; daraus wird keine Abnahme automatischer
   Widget-Fokussierung abgeleitet. Evidenz: `target/p06-live-xmove/move-*.json`.
5. Sämtliche eigenen Testclients beendet, Window-Snapshot leer, persistente
   Raumdefinitionen weiterhin unverändert. Aktiver Testraum zum Schluss: Raum 3.

Damit sind die P06-Gates der aktuellen Matrix erfüllt. Keine allgemeine Freigabe
beliebiger GPUs/Skalierungen. P07-App-Zuordnung/Overflow über neun Räume,
P09-Restore-Ausführung und ältere README-/Login-/Settings-Dokumentationsreste
bleiben eigenständige Folgearbeiten. Keine neue Funktion außerhalb von P06 aktiviert.
