# P06 – verständliche Raumwahl beim Löschen (26.09.2026)

Status: **in-progress** für P06 insgesamt. Nutzerauftrag: „Ziel: Bitte wählen“
erklärt den Ablauf nicht; eigener Löschbereich und echte Auswahlliste wurden
als Verbesserung bestätigt.

## Verhalten und geänderte Dateien

Unter `crates/niwoe-shell/src/`:

- `room_editor.rs`: getrennte Markierung und bestätigte Zielauswahl über stabile
  IDs. Öffnen/Navigieren verändert noch kein Ziel; verschwundene Ziele werden
  verworfen. Die Löschbestätigung nennt den gewählten Raum.
- `room_editor_tests.rs`: Auswahl ohne Mutation, Abbrechen, Grenzen,
  Umsortierung, verschwundene Ziele und explizite Löschbestätigung geprüft.
- `room_management_view/configuration/body.rs`: eigener Bereich „Raum löschen“,
  Erklärung zum Erhalt offener Anwendungen, Hinweis beim letzten Raum.
- `room_management_view/configuration/actions.rs`: „Fenster verschieben nach …“,
  „Raum auswählen“ sowie nativer, fontunabhängiger Aufklapppfeil.
- `room_management_view/configuration/target_menu.rs`: begrenzte Raumliste,
  übereinstimmende Zeichen-/Treffergeometrie, Positionsanzeige und Tests für
  alle 63 möglichen Ziele bei beiden Abnahmegrößen.
- `room_management_view/configuration/tests.rs`: Renderbeleg mit geöffnetem Menü.
- `room_management_view/configuration.rs`: Liste über dem Inhalt und unter dem
  bestehenden Kopf-/Fußbereich; neue Raumdaten als Renderparameter.
- `room_management_view.rs`: exportiert die Trefferermittlung für die Raumliste.
- `wayland/render/launcher.rs`: reicht den vollständigen Snapshot zum Zeichnen durch.
- `wayland/state/rooms.rs`: Mauswahl, Außenklick, Escape, Tab, Pfeiltasten,
  Home/End und Enter; Außenklick schließt ohne darunterliegende Aktion.
- `wayland/handlers/pointer/launcher.rs`: Klicks/Mausrad bei offener Liste gehen
  zuerst an das Menü; die Seite scrollt dabei nicht mit.

`crates/niwoe-tokens/src/control_center.rs`: zentrale Obergrenze von fünf
sichtbaren Zielzeilen. Alle übrigen Designwerte kommen aus bestehenden Tokens.
Aktiver Plan sowie die Berichte zu sichtbaren Abläufen und Erstklick führen
den aktuellen Stand nach.

Der Ablauf ist: Raum konfigurieren → unter „Raum löschen“ einen anderen Raum
wählen → „Raum löschen“ → den angezeigten Raum prüfen → „Löschen bestätigen“.
Escape schließt zunächst nur die offene Auswahlliste; eine bereits gewählte
Zielauswahl bleibt erhalten. Kein IPC- oder Persistenzschema wurde geändert.

## Verifikation

Fedora: `cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace -q`, `cargo clippy --workspace --all-targets -- -D warnings`
und `cargo build --release -p niwoe-shell --locked`.
Alle Befehle auf dem endgültigen Stand mit Exitcode 0 abgeschlossen; enthalten
sind Design-, Centralization- und Source-Size-Guards.
Logs: `target/p06-target-{check,test,clippy,build}.log`.

Native Renderbelege: `target/p06-target-1920.png` und
`target/p06-target-1366.png` (1920×1032 bzw. 1366×720 Inhaltsfläche).
Der erste Sichtlauf zeigte ein nicht unterstütztes Dreieckzeichen sowie eine
überdeckte Beschriftung im offenen Menü. Ein kleiner nativer Pfad ersetzt das
Zeichen; der aus Tokens abgeleitete Abstand hält die Beschriftung frei.

Zeichnen erfolgt ausschließlich bei Eingabe/Öffnen/Snapshotänderung. Die
Raumdaten enthalten höchstens 64 Einträge; höchstens fünf Zielzeilen und eine
Statuszeile werden gezeichnet. Keine Timer, Animationen, Effekte oder neuen
Assets. Der vorhandene Wallpaper-Cache bleibt erhalten. Keine neue
Idle-/GPU-Messreihe; keine behauptete gemessene Null-Last.

## Installation, Live-Nachweise und offene Gates

Die Shell wurde atomar installiert und über den vorhandenen Watchdog erneuert.
Buildartefakt, installierte Datei und `/proc/38357/exe` sind bytegleich:
`5636d59c57dd4c9508ce57fdb75614532c8b52ff27f18485c2d70bb065c7cf18`.
Der Compositor blieb PID 25333 mit dem nach Nutzer-Neulogin aktiven Erstklick-Fix
`eb48aa3d4675d3eda0985fd95da17e72b8cbd6e533b47f420bdbb8377b6bf283`.
Kein weiterer Neulogin erforderlich. Erfasste KDE-/GTK-Konfigurationen vor/nach
Installation bytegleich (`target/p06-target-shared-{before,after}.txt`).

Auf dem realen DRM-Desktop mit 1920×1080 einschließlich Panel nachgeprüft:

- Maus öffnet die Liste mit sichtbaren Einträgen und Positionsanzeige.
- End/Enter wählt Raum 9; erneutes Öffnen, Home, Escape erhält Raum 9 und
  schließt nur die Liste.
- Mauswahl übernimmt Raum 3. Ein Klick auf „Raum löschen“ zeigt die zusätzliche
  Bestätigung und benennt Raum 3 im Hinweis.
- Erneutes Öffnen verwirft die Bestätigung. Ein Außenklick auf den darunter
  liegenden Löschknopf schließt nur die Liste und löst keine Bestätigung aus.
- Danach Testauswahl durch Zurück-/Neuöffnen der Konfiguration verworfen.
  Kein vorhandener Raum wurde während dieses gezielten Bedienlaufs gelöscht.

Belege: `target/p06-click-target-final-{open,escape,confirm,outside}.png`.
Der Renderbeleg bei 1366×720 ist keine zweite physische Displayprüfung.
Mausrad/Tab wurden im Call-Flow geprüft; der Live-Lauf verwendete Mauswahl,
Home/End, Enter und Escape. Die bestehende tatsächliche Fenstermigration wurde
hier nicht erneut ausgelöst; deren vorherige Nachweise stehen im Grundlagen-
und Visible-Flows-Bericht. Die Erstklickregression ist separat in
[P06_FIRST_CLICK.md](P06_FIRST_CLICK.md) mit neuem DRM-Beleg nachgeführt.

Die allgemeinen P06-Gates für minimierte Fenster, Floating/Tiling/Dialoge,
XWayland, zwei Outputs/Hotplug und einen vollständigen Login-Rundlauf nach
Raumänderungen bleiben offen. Auch die im allgemeinen Audit genannten alten
README-/Login-/Settings-Dokumentationsreste sind nicht erledigt.
