# P07: Explizite Startkorrelation

26.09.2026, implementiert, geprüft und installiert. DRM-Aktivierung/Nachprüfung
folgen; P07-Abschlussmatrix separat.

## Änderungen nach Datei

- `niwoe-ipc/src/lib.rs`, `lib_tests.rs`: optionales `room_id` für `LaunchApp`,
  mit rückwärtskompatiblem Default. Alte Befehle und normale Shell-Starts behalten
  die App-Policy; die Shell berechnet keine zweite Zuordnungsregel.
- Compositor `state/launch_intent.rs`: serverseitige, einmalige Kennung über
  bestehendes xdg-activation; 60 Sekunden Gültigkeit, maximal 128 offene Tokens,
  Bereinigung bei neuen Requests. Kein Polling, App-ID-/Titel-/PID-Raten.
- `state/ipc/commands.rs`: explizite stabile Ziel-ID validieren, Kennung über
  `XDG_ACTIVATION_TOKEN`/`DESKTOP_STARTUP_ID` an den argv-Launch weitergeben.
  Ungültiges Ziel startet kein Programm; geerbte fremde Kennungen entfernen.
- `state/handlers/wayland_extra.rs`, `state/assignment.rs`: native Aktivierung
  vor/nach Toplevel-Erzeugung sowie X11-Startup-ID verwenden. Elternpriorität und
  manuelle Moves bleiben erhalten; keine Aktivierung eines Hintergrundraums.
  Verspätete Zuordnung minimierter Fenster erhält auch die WM-Mitgliedschaft.
- `protocols/xwayland/input_selection.rs`: spätes Startup-ID als Lifecycle-Ereignis.
- Shell-Launch-Aufrufstellen: optionales Feld ausdrücklich ohne Ziel befüllen.
- `examples/room_assignment_probe.rs`, `scripts/test-launch-room.py`,
  `scripts/smoke-nested.sh`: echte Token-Protokolltests, gleiche App-ID in zwei
  unterschiedlichen Räumen, normale Preferred-Regel, verspätete Aktivierung,
  minimiertes Fenster/Restore, manuell verschobenes Fenster, Transients,
  Replay auf unabhängige Oberfläche, X11 und fehlendes Ziel.

## Verifikation und Installation

Fedora: `cargo check --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check`, Release-Builds und Probe erfolgreich.
Echte Integration: `NIWOE_LAUNCH_ROOM_SMOKE=1 bash scripts/smoke-nested.sh
/run/user/1000/wayland-1`, Evidenz `nested.yeroux`, alle Fälle bestanden.
Design-/600-Zeilen-Guard grün; keine Cargo.toml-/Dependency-Änderung.

Installierte Release-SHA-256:

- niwoe: `16690e7543a068cdf7c7821f217ee832e6697adb55019fba88c319bfca760234`
- niwoe-shell: `9f14eb6f44e6394f34e0a1aa2e6f5464af01a7273b9e1428a0ddebc48f5eb9d9`

Release/installierte Dateien per `cmp` identisch; KDE-/GTK-Konfiguration erhalten.
Die erneute Sitzung aktiviert diesen Build. Laufende Identität noch nachzuweisen.

## Grenzen

Das Ziel wird nur bei tatsächlich zurückgelieferter xdg-Aktivierung bzw.
X11-Startup-ID korreliert. Apps ohne diese Metadaten folgen der normalen Policy;
keine angeblich eindeutige Zuordnung anhand gleicher App-IDs. Single-Instance-
Weiterleitung benötigt ebenfalls Tokenweitergabe durch die App. Ein generisches
Restore-/App-Integrationssystem bleibt P09. Die aktive Shell-Oberfläche bietet
weiter normale Starts; das explizite Ziel ist ein optionaler Launch-Vertrag.
