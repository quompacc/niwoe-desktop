# P07 – Abschlussabgleich

26.09.2026. **`accepted` für den Fedora-Testrechner und den unten beschriebenen
Prüfumfang.** P07-01 bis P07-05 abgeschlossen. P08 noch nicht begonnen.

## Implementierung und Teilcommits

Die früheren Navigations-/Zuordnungsblöcke sind in
[P07_NAVIGATION.md](P07_NAVIGATION.md) und
[P07_APP_ASSIGNMENT.md](P07_APP_ASSIGNMENT.md) beschrieben.
Die abschließenden Teilblöcke wurden getrennt dokumentiert und committed:

- `b2312aa`: vollständiger Fensterzugang, Maus-Verschieben, zentraler
  `WindowPicker`-Token, typisierter Move-IPC und Integrationstest.
- `223da1c`: Nachweis der installierten Fensterfunktionen auf DRM.
- `d90435a`: einmalige explizite Startkorrelation, native/X11-Metadaten,
  minimierte Zuordnung und reale Protokolltests.

Dateien und Änderungen je Modul stehen in
[P07_WINDOW_ACCESS.md](P07_WINDOW_ACCESS.md) und
[P07_LAUNCH_CORRELATION.md](P07_LAUNCH_CORRELATION.md).
Keine Cargo.toml-/Dependency-Änderungen; keine zusätzlichen P08/P09-Funktionen.

## Abnahmematrix

| Paket/Fall | Ergebnis und Nachweis |
| --- | --- |
| P07-01: echte IDs, Reihenfolge, Aktivität, Belegung | Native Snapshots, Panel und Raumauswahl; aktive Auswahl sichtbar, neutrales Belegt-Signal. Vorheriger Navigationsbericht plus aktuelle DRM-Screenshots. |
| P07-02: 1/9/64 Räume, Overflow, lange Namen | Alle drei Anzahlen live auf DRM geprüft. Mit 64 Räumen aktiviert Ende/Enter Raum 64; Panel zeigt 61–64 und +60, Popup Seite 8/8 samt vollständigem langen Namen. `target/p06-click-p07-room64-active.png`, `target/p07-room64-active.json`. |
| P07-03: Tastatur und Maus | Echter Super+Shift+4 erhält Fenster/Inhalt; Maus öffnet Fensterliste, wählt stabiles Raumziel und aktiviert konkretes Fenster. Gespeicherte Reihenfolge Raum 2 vor Entwicklung berücksichtigt. |
| P07-04: Preferred/Dedicated | Native und X11, Hintergrunddialoge, exakte/fehlende/späte Metadaten, mehrere Treffer, Elternpriorität, manuelle Moves; isolierte Protokolltests und DRM. Preferred stiehlt keinen Fokus, Dedicated akzeptiert fremde Apps. |
| P07-04: Startkorrelation | Gleichartige native Fenster erhalten verschiedene explizite Räume; Token vor Toplevel, verspätet/minimiert, Restore, Replay auf unabhängige Oberfläche, manueller Move, Parent und X11-Startup-ID geprüft. `nested.yeroux` und `target/p07-launch-drm-fixed.log`. |
| P07-05: vollständiger Fensterzugang | Neun Fenster derselben App, zweite Seite, minimiertes neuntes Fenster per Ende/Enter wiederhergestellt. Native DRM-Aufnahme `target/p06-click-p07-windows-page-live.png`, Test `target/p07-window-drm-pages-fixed.log`. Native/X11-Moves erhalten IDs und Inhalt. |
| Zwei Monitore | Intern 1920×1080@60, HDMI 3840×2160@30, logische Fläche 5760×2160. Unabhängig Raum 2/3 gewählt; App ohne Regel bei HDMI-Fokus landet in Raum 3, interner Raum bleibt 2. `target/p07-output1-room2.json`, `target/p07-output-launch/result.json`. |
| Tray/Benachrichtigungen | Test-SNI sichtbar und tatsächlicher Mausaufruf als `Activate` im Dienst empfangen. Native Benachrichtigung bleibt nach Raumwechsel sichtbar. `target/p07-chrome/activated`, `target/p06-click-p07-chrome.png`. |
| Raumwechsel ohne Programmstart | Leere Snapshots bei den Navigationsprüfungen und am Abschluss; Launch ausschließlich durch Testaufrufe. Keine Restore-/Autostartfunktion an Raumwechsel gekoppelt. |

Der Ein-Raum-Test verwendete zwischen zwei regulären Ab-/Anmeldungen eine
temporäre Raumdatei. Private Sicherung und bytegenaue Wiederherstellung der
vorherigen Datei bestätigt (SHA-256
`dac67322c3929c96c791e9522b456f30ea102ad45b910e2359a15ef6ed42d2d7`).
Keine laufenden Fenster während dieses Profiltests. Aufnahme:
`target/p06-click-p07-one-room.png`, Snapshot `target/p07-one-room.json`.
Die 55 zusätzlichen Räume für die 64er-Prüfung wurden über reguläre IPC-Mutationen
erzeugt und anhand separat erfasster IDs wieder gelöscht. Bestehende IDs, Namen,
Reihenfolge und Präferenzen erhalten. Revisions-/ID-Zähler dürfen dabei steigen.

## Verifikation und aktiver Release

Auf Fedora erfolgreich:

- `cargo check --workspace`
- `cargo test --workspace`, einschließlich Design-/Quelldateigrößen-Guard
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo build --release -p niwoe -p niwoe-shell`
- Release-Probe und echte native/X11-Protokoll-/Eingabeprüfungen wie oben.

Logs des finalen Codeblocks: `target/p07-launch-{check,tests,clippy,fmt,build}.log`.
Input-/IPC-/Lifecycle-Call-Flows gegen bestehende Architektur geprüft.
Keine zusätzliche Arbeit pro Frame durch die Zuordnung; Tokenmenge begrenzt,
Fensterliste nur bei Invalidierung gerendert, keine neuen Dauertimer/Effekte.

Installierte und tatsächlich laufende Dateien stimmen bytegenau mit Release
überein. Nach automatischer Anmeldung Compositor PID 180099; Shell zuletzt über
den bestehenden Watchdog zur Testbereinigung erneuert, PID 183465:

- niwoe: `16690e7543a068cdf7c7821f217ee832e6697adb55019fba88c319bfca760234`
- niwoe-shell: `9f14eb6f44e6394f34e0a1aa2e6f5464af01a7273b9e1428a0ddebc48f5eb9d9`

Kein weiterer Neulogin erforderlich. Automatische Ab-/Anmeldung war ausdrücklich
autorisiert; vorhandener Plasma-Loginmanager/PAM wurden nicht verändert.
KDE-/GTK-Dateihashes vor/nach Installation unverändert.

Abschlusssnapshot `target/p07-completion.json`: neun ursprüngliche Räume,
Revision 189, nächster ID-Zähler 78, keine offenen Testfenster. Intern Raum 1,
HDMI Raum 3. Testprozesse beendet, Test-Tray und Notifications bereinigt.

## Grenzen und Folgearbeit

- Die Startkorrelation erfordert Rückgabe der Aktivierungs-/Startup-Kennung.
  Apps ohne diese Metadaten folgen der normalen App-Policy; kein App-ID-basierter
  Ersatz mit falscher Eindeutigkeit. Der optionale Zielvertrag ist für explizite
  Launch-Aufträge verfügbar; aktuelle normale Shell-Starts bleiben unverändert.
- Benachrichtigungsaktionen/-historie gehören nicht zum bisher implementierten
  Notifications-Umfang. P07 weist die erhaltene Anzeige nach. SNI-Aufruf wurde
  mit registriertem Busnamen geprüft; Objektpfad-Registrierung und Bereinigung
  verschwundener Tray-Dienste sind bestehende Kompatibilitätslücken, im allgemeinen
  Audit nachgeführt. P07 ersetzt kein vollständiges Tray-/Notifications-Projekt.
- Die frühere Winit-Bildgrenze wird nicht als gelöst behauptet. Die hier genannten
  visuellen Abnahmen stammen aus der regulären DRM-Sitzung.
- P08 übernimmt die vollständige Daten-/Thumbnail-Anbindung des Hubs; P09 Restore,
  P10 weitere Editoren. Ältere README-Theme-/WebKit-Aussagen bleiben Auditrest.

Keine offene P07-Live-Abnahme für diese Hardwarematrix. Weitere Hardware,
App-Integrationen und die genannten Folgephasen sind durch diese Abnahme nicht
pauschal freigegeben.
