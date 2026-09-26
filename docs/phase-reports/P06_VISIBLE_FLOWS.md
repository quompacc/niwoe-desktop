# P06 – sichtbare Raumabläufe (26.09.2026)

Status: **in-progress**, keine vollständige P06-Live-Abnahme.
Basis: `bc92b45` (Reconnect-Fix), davor `190c896`.

Nach dem Neulogin meldete der Nutzer einen verlorenen Erstklick. Der Fehler
wurde auf dem installierten Release reproduziert und im gemeinsamen
Compositor-Eingabepfad korrigiert. Belege und Aktivierungsstatus stehen im
[Erstklick-Bericht](P06_FIRST_CLICK.md).

## Ergebnis

„Neuer Raum“ öffnet einen Entwurf in der vollständigen bestehenden
Konfigurationsseite. Name und Beschreibung werden bei Speichern gemeinsam
validiert und atomar persistiert. Abbrechen erzeugt keinen Raum. Im bestehenden
Raum ist die Beschreibung jetzt bearbeitbar; verborgene Zuordnungsmetadaten
werden durch die Formularänderung nicht überschrieben. Die Zuordnungsregel-UI,
Start-Apps und Restore bleiben den späteren Planphasen vorbehalten.

Löschen befindet sich im rechten Bereich „Raum löschen“ der Konfiguration:
Zielraum ausdrücklich auswählen, „Raum löschen“, anschließend „Löschen
bestätigen“. Auf Nutzerwunsch ersetzt eine aufklappbare Raumliste die frühere
zyklische Zielaktion; die Beschriftung lautet „Fenster verschieben nach …“.
Slotnummer und Name unterscheiden auch gleichnamige Ziele. Der letzte Raum ist
nicht löschbar. Der Compositor verschiebt vorhandene Fenster über den bestehenden
P06-Pfad. Keine Anwendung wird durch die Shell geschlossen.

Aktueller Installations- und Bedienstand: [Raumwahl beim Löschen](P06_DELETE_TARGET_MENU.md).
Die folgenden ursprünglichen Installationshashes und Live-Gates beschreiben
den Stand vor dieser Bedienpräzisierung; der verlinkte Bericht führt sie fort.

Pending sperrt weitere Mutationen und Abbrechen. Konflikte und Speicherfehler
bleiben im Formular sichtbar. Nach unklarer Erzeugungsbestätigung (Timeout oder
Durability) ist erneutes Speichern gesperrt; der Nutzer prüft zuerst die Raumliste.
Das verhindert eine versehentliche doppelte Erzeugung durch Wiederholen.

## Dateien und Call-Flow

- `niwoe-ipc/src/rooms.rs`: additive `CreateDetails`-/`UpdateDetails`-Commands.
  Bestehende Commands bleiben kompatibel. Die neue Shell benötigt für die neuen
  Formularaktionen den gemeinsam ausgelieferten Compositor.
- `niwoe-compositor/src/room_registry.rs`: vollständige Formularänderung in
  einer validierten Revision; Update erhält die bestehende Zuordnungsart.
  `state/ipc/rooms.rs` ergänzt für CreateDetails den echten Space-/WM-Slot.
  `room_registry_tests.rs` prüft atomare Ablehnung und persistiertes Wiederladen.
- `niwoe-shell/src/room_editor.rs`: Entwurf, Beschreibung, explizites Löschziel,
  Bestätigung und Schutz vor doppeltem Erzeugen. `room_editor_tests.rs` prüft
  Pending, Abbruch, atomare Formularcommands, Zielwechsel und Timeout.
- `room_management_view.rs`: aktiver Neuer-Raum-Button und Fokusdarstellung;
  bestehende Tests verhaltensgleich nach `room_management_view/tests.rs` versetzt.
- `room_management_view/configuration.rs`: neue Trefferflächen und Formularstatus;
  Tests nach `configuration/tests.rs` versetzt und um beide Zielgrößen ergänzt.
  `configuration/body.rs` zeichnet das echte Beschreibungsfeld.
  `configuration/actions.rs` kapselt Beschreibung und Löschaktionen.
- `wayland/state/rooms.rs`: Formularnavigation, Textbearbeitung und IPC-Versand.
  `wayland/types.rs` ergänzt Ziel-/Löschaktionen.
  `wayland/handlers/{keyboard/room_navigation,pointer/launcher}.rs`: Maus- und
  Tastaturzugang zu Neuer Raum; Fokus berücksichtigt die Raumobergrenze.
  `wayland/render/launcher.rs`: Entwurf ohne erfundene persistente Raumidentität.
- `scripts/test-room-lifecycle.py`: Integrationstest ausschließlich in einem
  isolierten Smoke-Profil. `scripts/smoke-nested.sh` aktiviert ihn optional mit
  `NIWOE_ROOM_SMOKE=1`.

Eingabe → lokaler Entwurf → revisionsgesicherter Command → Registry validiert
und speichert → Compositor synchronisiert Slots/Fenster → RoomSnapshot → Ack →
Shell kehrt zur Verwaltung zurück. Bei Fehler bleibt der Entwurf bestehen.
Der bestehende Render-/Layerpfad und die Renderreihenfolge bleiben erhalten.

## Design und Performance

Die vollständige Sidebar, Karten, Kopf-, Vorschau- und Fußstruktur aus Mockup
`17_13_54 (4)` bleibt erhalten. Die ausdrücklich beauftragte Löschfunktion
ergänzt die vorhandene rechte Informationskarte; es gibt keinen neuen Popupeditor.
Beschreibung nutzt den vorgesehenen Platz. Alle Maße/Farben/Radien stammen
weiter aus bestehenden Tokens; keine neue Render-Hardcode-Ausnahme.

Native Renderbelege: `target/p06-visible-1920.png`,
`target/p06-visible-1366.png`, `target/p06-create-preview.png`.
Dies sind native Render-PNGs, keine Screenshots eines installierten neuen
DRM-Bedienlaufs. Die neue Bedienung mit Maus/Tastatur ist daher noch live offen.

Keine neuen Timer, Effekte oder Assetdekodierung. Zeichnen nur bei Eingabe,
Öffnen oder Snapshotänderung. Zielwahl durchsucht höchstens 64 Räume;
Verwaltung zeichnet weiterhin höchstens neun Karten pro Seite. Bestehender
Wallpaper-Cache bleibt erhalten. Eine neue Idle-/GPU-Messreihe wurde nicht
durchgeführt; aus dem unveränderten Timerpfad folgt keine gemessene Null-Last.

## Verifikation und Grenzen

Fedora-Gates, alle bestanden (Exitcode 0): `cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace -q`,
`cargo clippy --workspace --all-targets -- -D warnings` sowie
`cargo build --release -p niwoe -p niwoe-shell --locked`.
Logs: `target/p06-visible-{fmt,check,test,clippy,build,nested}.log` auf Fedora.
Die 474 erfassten Rust-/Cargo-Quelldateien stimmen über SHA-256 mit dem lokalen
Arbeitsstand überein (`target/p06-source.sha256`, Prüfung ohne Abweichungen).
Workspace-Tests enthalten Design-, Source-Size- und Centralization-Guards.
Die gezielten nativen Renderläufe prüfen beide Inhaltsgrößen (1920×1032 und
1366×720, zuzüglich 48-Pixel-Panel). Ein erster Export mit relativem Zielpfad
schlug wegen des Crate-Arbeitsverzeichnisses fehl; absolute Ausgabepfade
beheben den Testaufruf.

Der isolierte Release-Smoke prüft verzögert eintreffende Authentifizierung,
CreateDetails, UpdateDetails, Metadaten, Reihenfolge, Revisionskonflikt,
Raumwechsel, natives Testfenster, Löschen mit Fenstermigration, Slotverdichtung,
Reconnect und die tatsächlich geschriebene TOML-Datei. Elternkonfigurationen
werden vor/nach dem Test verglichen. Der bestehende DRM-Grundlagenlauf ist im
[Grundlagenbericht](P06_ROOM_MODEL_FOUNDATION.md) dokumentiert.
Der abschließende isolierte Lauf ist erfolgreich (Exitcode 0), Detailbelege:
`target/p01-evidence/nested.4EE9Ku`. Zenity meldete dabei Vulkan-Surface-Warnungen;
der konfigurierte Client und sein Fenster blieben bis zur Testbereinigung
vorhanden. Dies ist kein allgemeiner Vulkan-/Rendering-Stabilitätsnachweis.

## Installation

Beide Releases wurden über `target/p06-install-visible.sh` atomar pro Datei
installiert und mit `cmp` sowie SHA-256 gegen die Buildartefakte geprüft:

- `niwoe`: `77ebffb88e14d7a4344a836b6cddaf1650503fb38f41f5742273fb27ec9fb87d`
- `niwoe-shell`: `122f591243d0e08ef11c22680523a1ee4c2121f520586999b533ae3a7c9ab669`

Die erfassten KDE-/GTK-Dateien sind vor und nach der Installation bytegleich
(`target/p06-visible-shared-{before,after}.txt`). Bootsplash, Loginmanager,
PAM und Sitzungsstart wurden nicht verändert. Die laufenden alten Prozesse
wurden nicht beendet. Ein NIWOE-Neulogin muss **beide** neuen Prozesse aktivieren;
ein alleiniger Shell-/Watchdog-Neustart genügt für die neuen IPC-Commands nicht.

Offen bleiben tatsächliche UI-Eingaben im neuen DRM-Release, minimierte Fenster,
Floating/Tiling/Dialoge/XWayland, zwei Outputs/Hotplug, Fehler-/Grenzfälle und
ein vollständiger Login-Rundlauf nach Raumänderungen. Erfolgreiche Unit- und
Nested-Tests ersetzen diese Abnahme nicht. Ältere README-/Login-/Settings-
Dokumentationsreste aus dem allgemeinen Audit sind weiterhin offen.
