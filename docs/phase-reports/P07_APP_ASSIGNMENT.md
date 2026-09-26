# P07 – App-Zuordnung, zweiter Block

26.09.2026. Implementiert, geprüft und auf Fedora installiert. Nach Nutzer-
Neulogin im regulären DRM-Compositor aktiv und auf dem internen Display live
geprüft. Zweimonitor-Nachtest noch offen. **P07 bleibt in-progress, nicht accepted.**

## Geänderte Dateien und Verhalten

- `crates/niwoe-compositor/src/room_assignment.rs`: reine getestete Policy.
  Elternraum vor explizitem Ziel, dann erste passende Regel in gespeicherter
  Raumreihenfolge, danach bisheriger Fensterraum. Stabile IDs; Free überspringt
  Regeln, Preferred/Dedicated bevorzugen passende Apps ohne Zugangssperre.
- `state/assignment.rs`: gemeinsamer Native-/X11-Lifecycle einschließlich
  minimierter Eltern/Kinder. Exakte getrennte Identitäten ohne Titelheuristik,
  Kleinschreibung oder erratene Desktop-Endung. Leere/ungültige Metadaten bleiben
  unaufgelöst; die erste gültige Identität entscheidet einmal. Anschließend
  bleiben Fenster liegen; spätere Elternbezüge haben weiterhin Vorrang.
- `state/handlers/xdg/{mod,lifecycle}.rs`, `state/handlers/core/compositor.rs`:
  App-ID/Elternereignisse verwenden diese Policy. Initialer nativer Fokus erst
  beim ersten Puffer und nur im noch aktuellen Raum. `xdg/transient.rs` entfällt
  zugunsten der gemeinsamen Elternlogik. `lib.rs`/`state/mod.rs` binden Module ein.
- `protocols/xwayland/{window_lifecycle,input_selection}.rs`: Zuordnung bei Map
  und späteren Class-/TransientFor-Ereignissen, auch minimiert.
  Override-redirect-Fenster werden nicht als eigenständige Apps umverteilt.
- `state/layout/workspace.rs`, `state/ipc/rooms.rs`: manuelle Moves und
  Löschmigration schützen auch Fenster ohne bisherige App-ID. Laufzeitmerker
  gehören zum tatsächlichen Window, nicht zu allen Fenstern derselben App.
- `crates/niwoe-shell/src/room_management_view/configuration/{body,tests}.rs`:
  vorhandene Start-Apps-Karte erklärt den Modus, für Dedicated ausdrücklich
  „andere Apps bleiben erlaubt“. Zentrale Geometrie/Material unverändert.
- `crates/niwoe-shell/examples/room_assignment_probe.rs`,
  `scripts/test-room-assignment.py`, `scripts/smoke-nested.sh`: native
  Protokollclients und GTK-X11-Clients mit überprüfbaren Laufzeitassertionen.
- `crates/niwoe-lock/examples/lock_focus_probe.rs`: bestehender Fokustest
  verwendet konfigurierte SHM-Puffer statt unsichtbarer Toplevels.

Policy ereignisgesteuert, ohne Timer oder Produkt-Polling; kein erneuter Fokus
bei folgenden Commits. Keine neuen Assets/Effekte/Animationen. Raumwechsel
starten keine Programme. Keine Cargo-/Dependency-Änderungen; Rust-Dateien <600 Zeilen.

## Verifikation

Fedora: `cargo check --workspace`, `cargo test --workspace` einschließlich
Design Guard, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check`, Release-Build für niwoe/niwoe-shell und beide
Protokollprobes erfolgreich. Logs: `target/p07-assignment-{check,tests,clippy,fmt,build}.log`.

Echte isolierte Compositor-Tests unter `target/p01-evidence/`:

- `nested.JownBo`, `nested.Zc1TUd`: Preferred im Hintergrund ohne einziges
  Keyboard-Enter, Vordergrundfokus erhalten; Dialog erbt Elternraum; fehlende
  und spätere native ID; umgeordnete konkurrierende Regeln; Dedicated akzeptiert
  fremde Apps; XWayland-Klasse, verspätete Klasse, Transients und getrennte
  Native-/Class-Namespaces bestanden.
- `nested.Zc1TUd`, Profil `/tmp/niwoe-p01.nYDoNs`: echter uinput-Move bleibt
  nach späterer App-ID erhalten. Private Testbelegung `z` führt dieselbe
  MoveToWorkspace-Aktion aus, da der äußere Compositor Super+Shift+4 abfängt.
  Keine Änderung der Benutzertastenbelegung.
- Finaler Release, `nested.J87QFs`: Lock-Fokus bei Dialog/Popup, Freigabe und
  verlorenem Lock-Client bestanden.
- Finaler Release, `nested.mX5seg`: Hintergrund-/minimierter Elternraum,
  Dialog-Configure/Draw, Löschmigration, minimiertes Wiederherstellen, stabile
  IDs/Inhalte, Client-Cleanup und echter Neustart mit identischen Präferenzen
  und neutraler Loge bestanden.

Alle Testprozessgruppen beendet. Dedicated-Hinweis mit nativem Renderer erzeugt
und visuell geprüft: `target/p07-dedicated.png`; kein DRM-Live-Abnahmenachweis.
Anfangsfehler des Testaufbaus wurden korrigiert: schreibgeschützter SHM-FD,
noch geöffneter exklusiver Willkommens-Hub, frühes geerbtes `DISPLAY=:0` statt
des tatsächlichen Nested-XWayland-Kinddisplays sowie abgefangener Shortcut.
Fehlgeschlagene Anläufe zählen nicht als erfolgreiche Abnahme.

## Installation und offene Punkte

Release und installierte Dateien bytegleich bestätigt:

- niwoe: `2c299fb2bc680581f6be0569350d8773d687e35e2db23e0bdec7641aec87e11a`
- niwoe-shell: `e5bcacd0e0125a71dd98edb0c86836260b3058a2f6a5c3545da9fdcca68366f3`

KDE-/GTK-Dateien unverändert. Bei Installation liefen noch Compositor PID
109259 und Shell PID 124711 mit den alten Dateien. Neulogin angefragt;
laufende Sitzung nicht automatisch beendet.

### DRM-Nachtest nach Nutzer-Neulogin

Compositor PID 143457 und Shell PID 143474 verwenden nachweislich exakt die
oben angegebenen installierten SHA-256. Vor dem Test: neutrale Loge, keine
Fenster, neun unveränderte Räume, Revision 49, next_id 19.

Fedora meldete nur `drm-0`, 1920×1080 bei 60 Hz. Der externe Monitor war nicht
aktiv; deshalb ist dieser Nachweis ausdrücklich **kein Zweimonitortest**.

Eigene native GTK- und X11-Testfenster auf dem echten DRM-Backend bestanden:
Preferred-Zuordnung nach Raum 3 ohne Raumwechsel/Fokusdiebstahl, Hintergrund-
Dialoge im Elternraum, Dedicated akzeptiert eine nicht zugeordnete native App,
echter `Super+Shift+4`-Move mit erhaltenem Fensterinhalt. Der erste Anlauf wurde
wegen des noch exklusiven Hubs abgebrochen und vollständig bereinigt; nach
Schließen über Escape bestand der gesamte Ablauf.

Evidenz auf Fedora: `target/p07-drm.log` und
`target/p07-drm-1790440976626235615/{before,tested,after}.json`; Testhelfer
`target/p07-drm.py`, `target/p07-drm-client.py`. Die temporären Regeln wurden
über reguläres IPC gesetzt und im Finally-Pfad zurückgesetzt. Alle ursprünglichen
Raumdefinitionen einschließlich IDs, Reihenfolge und Präferenzen exakt bestätigt;
Revision durch Testmutationen auf 58 erhöht, next_id unverändert 19. Keine
Testfenster verbleiben, abschließend Raum 1 aktiv. Keine Rust-Änderung in diesem
Nachtest; erneuter Build oder erneute Installation nicht erforderlich.

LaunchApp-IPC hat bisher kein explizites Raumziel. Programm-/Argumentpfad
unverändert; normale App-Regeln gelten für entstehende Fenster. Eindeutige
Korrelation eines zukünftigen raumbezogenen Startvorhabens über Aktivierungs-/
Launch-Metadaten ist **noch nicht implementiert**; App-ID wird dafür nicht
als individuelle Fensteridentität ausgegeben. Der App-Auswahleditor gehört
zur späteren Einstellungsphase und bleibt sichtbar als nicht verfügbar markiert.

Weiter offen: Zweimonitor-Endabnahme, explizite Startkorrelation,
Mausaktionen zum Verschieben, vollständiger Fensterzugang und visuelle
64-Räume-Abnahme. Ältere README-/Login-/Settings-Dokumentationsreste bleiben offen.
