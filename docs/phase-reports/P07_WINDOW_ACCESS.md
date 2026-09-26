# P07: Fensterzugang und Maus-Verschieben

**Abschlussnachtrag:** P07 ist für die dokumentierte Fedora-Matrix abgeschlossen.
Die nachstehenden offenen Punkte sind historische Checkpoints; aktueller Stand:
[P07-Abschlussabgleich](P07_ACCEPTANCE_REVIEW.md).

Stand 26.09.2026: Teilblock implementiert, geprüft und installiert. P07 insgesamt
bleibt in Arbeit; die DRM-Nachprüfung dieses Builds steht noch aus.

## Änderungen

- `niwoe-shell/src/window_picker.rs`, `wayland/window_picker.rs`: vollständige
  Fensterliste aus echten Snapshots, sechs Zeilen pro Seite, minimierte Fenster
  und mehrere Fenster derselben App. Raumzielauswahl umfasst bis zu 64 Räume.
  Maus/Rechtsklick sowie F6, F2, Pfeile, Home/End und Seitenwechsel sind angebunden.
- Hub-, Render-, Pointer-, Keyboard- und Popup-State-Module: Zugang über die
  Fensterkarte, Fokus auf konkrete Fenster-ID, sauberes Zurücksetzen beim Schließen.
  Aktionen erfolgen beim Loslassen, damit der implizite Pointer-Grab beendet ist
  und kein Release auf eine darunterliegende Raumkarte durchgreift.
- `niwoe-ipc/src/lib.rs`, Compositor `state/ipc/window_move.rs`: typisierter
  Move-Befehl mit stabiler Raum-ID; erhält Fensteridentität, minimierten Zustand
  und Floating-/Tiling-Mitgliedschaft. Kein Raumwechsel als Nebenwirkung.
  Unbekannte Fenster/Ziele bleiben wirkungslos; manuelle Zuordnung wird geschützt.
- `state/ipc/window_focus.rs`: vorhandene Fokuslogik verhaltensgleich aus
  `commands.rs` extrahiert, damit die 600-Zeilen-Grenze eingehalten bleibt.
- `niwoe-tokens/src/window_picker.rs`: zentrale Geometrie. Farben, Typografie und
  Material kommen aus den bestehenden Tokens. Zeichnen nur bei Invalidierung;
  keine neuen Animationen, Captures oder dauerhaften Hintergrundabfragen.
- `scripts/test-window-move.py`, `scripts/smoke-nested.sh`: echte native und
  X11-Fenster, ungültige Ziele, minimierte Moves, Restore und optionale UI-Tests.

## Verifikation

Fedora: `cargo check --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check` und Release-Build erfolgreich, einschließlich
Design- und Quelldateigrößen-Guard. Input-/IPC-Call-Flow geprüft.

Isolierte echte Protokolltests: native Moves (`nested.U3Isgy`), X11
(`nested.pLFodB`), neun gleichartige Fenster einschließlich minimiertem letzten
Eintrag per Tastatur (`nested.VD1WIz`). Abschließender reiner Mausablauf mit
Release-Korrektur erfolgreich (`nested.dBnADD`): Hub-Karte, Verschieben,
Zielraum, Fensteraktivierung. Native Rastervorschauen für Fensterliste und
Räume 61–64 visuell geprüft. Winit liefert keinen DRM-Bildnachweis.

Installiert und per `cmp`/SHA-256 gegen Release geprüft:

- niwoe: `7f96afb2f4b2499eea63edce79681f5d751f713281d358b0a1189ed25a3b64a3`
- niwoe-shell: `970b9323c406241ac964626c51f7cbe5b54304bcd0c97567e11a36253075b786`

KDE-/GTK-Dateihashes vor/nach Installation unverändert. Laufende Prozesse zum
Installationszeitpunkt noch vorheriger Build; Neulogin und DRM-Nachtest folgen.
Der Nutzer hat automatische Ab-/Anmeldungen für den P07-Abschluss autorisiert.

## Offen

DRM-Fensterzugang, abschließende Zweimonitor-/1/9/64-Matrix und Startkorrelation.
Kein abgeschlossener Gesamt-P07-Status aus Unit-Tests abgeleitet.

## DRM-Nachprüfung desselben Builds

Automatische Ab-/Anmeldung über den vorhandenen Plasma-Loginmanager erfolgreich;
laufende `/proc/164784/exe` und `/proc/164798/exe` entsprechen den oben genannten
Hashes. Keine Änderung an Loginmanager/PAM. Beide Outputs aktiv: 1920×1080 und
3840×2160, gemeinsam 5760×2160.

Preferred, native/X11-Transients, Dedicated und echter Super+Shift+4 erneut
bestanden (`target/p07-drm-1790443051825500269`). Vollständige Raumdefinitionen
nach temporären Testregeln wiederhergestellt, keine Testfenster zurückgelassen.
DRM-Mausablauf mit gespeicherter Reihenfolge Raum 2 vor Entwicklung bestanden
(`target/p07-window-drm-fixed.log`). Neun Fenster derselben App, zweite Seite und
Wiederherstellen des minimierten neunten Fensters bestanden
(`target/p07-window-drm-pages-fixed.log`); nativen Screenshot visuell geprüft
(`target/p06-click-p07-windows-page-live.png`). Ein anfängliches 25-Sekunden-Limit
des isolierten Tests war für Fernsteuerung plus Screenshot zu kurz; DRM-Test mit
90-Sekunden-Eingabefenster wiederholt, kein Produktfix daraus abgeleitet.

Diese Prüfung erfolgt auf dem Zweimonitorsystem; unterschiedliche Raumwahl je
Output und die abschließende 1/9/64-Matrix werden separat abgeschlossen.
