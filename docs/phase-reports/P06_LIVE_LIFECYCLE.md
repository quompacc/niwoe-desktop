# P06 – nativer Live-Lebenszyklus und Dialogkorrektur (26.09.2026)

Status: **in-progress**, keine vollständige P06-Abnahme.
Auftrag: Erstellen/Bearbeiten, Raumwechsel, Fensterzustände und Löschen möglichst
selbstständig prüfen; laufende Nutzersitzung nicht unerwartet beenden.

## Tatsächlicher DRM-Durchlauf

Ausgangsstand: Compositor PID 25333 (`eb48aa3d…`), Shell PID 38357
(`5636d59c…`), ein Output mit 1920×1080. Neun vorhandene Räume, keine Fenster,
neutrale Loge. Die Helfer speichern keine Zugangsdaten oder IPC-Token.

Über native Maus-/Tastatureingaben geprüft:

1. „Neuer Raum“ erzeugt `p06-live-source`, ID 18, zunächst Workspace 10.
   Name und Beschreibung werden gemeinsam gespeichert (Revision 36 → 37).
2. Auf der zweiten Verwaltungsseite öffnet sich der neue Raum. Umbenennen in
   `p06-live-renamed` und Ändern der Beschreibung funktionieren (Revision 38).
3. Neun „Früher“-Aktionen verschieben ausschließlich den Testraum an den Anfang;
   die relative Reihenfolge der ursprünglichen Räume bleibt erhalten (Revision 47).
   Auswahl seiner Hub-Karte aktiviert tatsächlich Workspace 10.
4. Eine temporäre GTK3-Anwendung erzeugt normales, minimiertes und später per
   Super+T schwebend geschaltetes Fenster. Der Wechsel über die Raumleiste nach
   Raum 2 lässt die drei Fenster in Workspace 10. Umbenennen bei geöffneten
   Fenstern in `p06-live-with-windows` erhält deren IDs und Inhalte (Revision 48).
5. Im Löschbereich Raum 2 ausgewählt, Bestätigung visuell geprüft und den
   Testraum gelöscht. Alle drei Fenster wechseln mit unveränderten IDs nach
   Workspace 2. Das minimierte Fenster bleibt minimiert; Wiederherstellen über
   den bestehenden FocusWindow-Pfad gelingt. Tiling-/Floating-Zustand und die
   drei Eingabefeldinhalte bleiben erhalten (Revision 49).
6. Nur die Testanwendung beendet. Ursprüngliche Raumdefinitionen einschließlich
   IDs, Namen, Beschreibung, Zuordnung und Reihenfolge sind wieder identisch.
   Revision und nächster ID-Zähler bleiben korrekt monoton erhöht. Keine
   Testfenster übrig. Der aktive Raum ist danach Raum 2; eine Rückkehr in die
   Login-Loge wurde nicht durch eine Sitzungsbeendigung erzwungen.

Belege auf Fedora: `target/p06-live-ui/*.json`, automatischer Belegvergleich
`target/p06-verify-live.py` erfolgreich. Screenshots:
`target/p06-click-live-{real-draft,source-config,reordered,hub-source,
floating-correct,delete-confirm,after-delete}.png`.
Ein erster Eingabeversuch traf den bereits geschlossenen Desktop; die
anschließende Snapshotprüfung bestätigte keine Mutation. Der dokumentierte
Durchlauf beginnt mit dem anschließend erfolgreich geöffneten Hub.

## Gefundene Fehler und Abgrenzung

**Behobener P06-Fehler:** Ein GTK-Dialog mit `transient_for` auf das Fenster in
Workspace 10 öffnete während aktivem Workspace 2 fälschlich dort. Der Parent
blieb in Workspace 10. Die vorherige Handlerimplementierung ignorierte
`XdgShellHandler::parent_changed`. Außerdem suchte der Commit-Pfad Toplevels
für initiales Configure nur im aktiven Space.

**Noch offene P07-Lücke:** Die bisherige Raumleiste/Überlaufliste begrenzt sich
auf neun Räume; bei zehn vorhandenen Räumen zeigte sie weiterhin `+5` und neun
Einträge. Ein hinterer Raum ist über diese Liste nicht erreichbar. Die
Verwaltung paginiert korrekt; Umsortieren macht den Raum im Hub erreichbar.
Dies ist ausdrücklich keine vollständige 1–64-Räume-Navigationsabnahme und
wurde nicht durch einen vorgezogenen Umbau der Raumleiste kaschiert.

## Geänderte Dateien und Call-Flow

- `crates/niwoe-compositor/src/state/handlers/xdg/mod.rs`: verarbeitet die
  Elternänderung über ein eigenes Modul.
- `…/xdg/transient.rs`: ermittelt den Raum des tatsächlichen Elternfensters,
  einschließlich minimierter Eltern; führt Space- und WM-Zuordnung zusammen.
  Kein automatischer Raumwechsel; ein Hintergrunddialog behält keinen Fokus
  im aktuellen Raum. Entfernen einer Elternbeziehung erzwingt keinen Move.
- `…/core/compositor.rs`: Commit/Configure findet den Toplevel auch im
  Hintergrundraum. Bestehende Popup-/Layer-/Resize-Reihenfolge bleibt erhalten.
- `scripts/test-room-transients.py`: isolierter nativer GTK3-Regressionstest für
  Hintergrundeltern, minimierte Eltern, Configure/Draw, Migration, Wiederherstellen,
  stabile IDs, Fensterinhalt und gespeicherte Löschung.
- `scripts/test-room-persistence.py`: vorbereitetes CreateDetails/Umsortieren
  und Vergleich nach Neustart des isolierten Compositor-Prozesses.
- `scripts/smoke-nested.sh`: optionale Flags `NIWOE_ROOM_TRANSIENT_SMOKE=1` und
  `NIWOE_ROOM_RESTART_SMOKE=1`; startet nur seinen eigenen Test-Compositor neu.
- Aktiver Plan und dieser Bericht: Abnahmestand und verbleibende Gates.

Keine Cargo-/Dependency-, Theme-, Login-, PAM- oder Sitzungsmanageränderung.
Zusätzliche Sucharbeit nur bei Elternänderung und Oberflächen-Commits, über
höchstens 64 Räume. Keine neuen Idle-Timer oder Render-/Effektpfade; keine
separate Idle-/GPU-Messreihe durchgeführt. Testhelfer benötigen vorhandenes
Python/PyGObject/GTK3 ausschließlich für den optionalen Integrationstest.

## Verifikation und Installation

Fedora, endgültiger Stand jeweils Exitcode 0:

- `cargo check --workspace`
- `cargo test --workspace -q` (einschließlich bestehender Guards)
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo build --release -p niwoe --locked`
- `NIWOE_ROOM_TRANSIENT_SMOKE=1 NIWOE_ROOM_RESTART_SMOKE=1 bash scripts/smoke-nested.sh /run/user/1000/wayland-1`

Logs: `target/p06-transient-{check,test,clippy,build,nested}.log`.
Der Regressionstest scheitert auf dem alten Release genau an der Raumvererbung
(`target/p06-transient-baseline.log`, `nested.iPujg2`) und besteht mit dem Fix
(`target/p01-evidence/nested.6bRp0c`). Der isolierte Neustart lädt IDs,
Reihenfolge, Beschreibung, Zuordnung und Revision unverändert und startet in
der neutralen Loge. Das ist ein neuer echter Compositor-Prozess, aber kein
vollständiger Loginmanager-/PAM-/DRM-Neulogin.
Isolierte Portal-/AT-SPI-/PipeWire-Warnungen begründen keine Audio- oder
Barrierefreiheitsabnahme. Erfasste Eltern-KDE-/GTK-Konfigurationen blieben im
Nested-Smoke bytegleich.

Compositor atomar installiert, per `cmp` gegen den Release und SHA-256 geprüft:
`47a77cd257b3817e0a73123e0addc89412fad6d13d32ffa00f503ff3e642b62e`.
Shell unverändert: `5636d59c57dd4c9508ce57fdb75614532c8b52ff27f18485c2d70bb065c7cf18`.
Die damalige Sitzung verwendete bis zum Neulogin den alten Compositor
`eb48aa3d4675d3eda0985fd95da17e72b8cbd6e533b47f420bdbb8377b6bf283`.

## Nachprüfung nach Nutzer-Neulogin

Am 26.09.2026 um 11:54:42 gestartete DRM-Sitzung: Compositor PID 56919,
Shell PID 56937. Die SHA-256 der tatsächlich laufenden Binärdateien stimmen
mit den oben angegebenen installierten Releases überein; der Dialog-Fix ist aktiv.
Der erste Snapshot zeigt die neutrale Loge (aktiver Workspace 0), neun Räume
und keine Fenster. IDs, Reihenfolge, Metadaten und Revision 49 entsprechen
exakt dem bereinigten Zustand vor dem Login. Auch die gespeicherte
Raumkonfiguration wurde verglichen.

`python3 target/p06-postlogin.py` erfolgreich (Exitcode 0), auf dem echten
DRM-Compositor mit einer eigenen temporären GTK3-Anwendung:

- Dialog zu einem Elternfenster im inaktiven Raum bleibt in dessen Raum;
  der aktive Raum wechselt nicht.
- Ein weiterer Dialog zu demselben, inzwischen minimierten Elternfenster
  bleibt ebenfalls im Elternraum.
- Wiederherstellen des Elternfensters gelingt; beide Dialoge werden gezeichnet,
  der Eingabefeldinhalt bleibt erhalten.
- Nur die Testanwendung beendet; keine Fenster übrig. Raum-Snapshot und
  Konfigurationsdatei bleiben unverändert. Zum Abschluss ist Raum 2 aktiv.

Belege: `target/p06-live-ui/post-login.json` und
`target/p06-postlogin-evidence/{inactive-parent,minimized-parent,client-state,final}.json`.
Der neue Sitzungsmarker `first-login-hub-shown-178` wurde um 11:54:45 angelegt.
Die erste eigene Aufnahme zeigt bereits „Räume verwalten“; sie beweist daher
nicht die ursprüngliche Willkommensansicht. In dieser Runde wurde kein
zusätzlicher Shell-Neustart zur Prüfung des einmaligen Hubs ausgelöst.
Der isolierte Neustarttest oben deckt zusätzlich eigens erzeugte Metadaten ab;
die echte Loginprüfung vergleicht die neun beibehaltenen Nutzerräume.

In dieser Nachprüfung nur Plan und Bericht geändert, kein Rust-Code.
Die vorherigen Build-/Testnachweise gelten unverändert; die neuen Nachweise
sind Live-Prüfungen und keine vollständige P06-Abnahme.

## Offene Gates

- Zwei physische Outputs/Hotplug, XWayland-Schließen im inaktiven Raum sowie
  weitere Fehler-/Grenzfälle der gesamten P06-Matrix.
- P07-Navigation für mehr als neun Räume; kein Vorziehen weiterer P07-Features.
- Alte README-/Login-/Settings-Dokumentationsreste des allgemeinen Audits.
