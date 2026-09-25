# Allgemeiner NIWOE-Audit – 25.09.2026

## Geltungsbereich und Maßstab

Read-only-Sichtung des aktuellen Arbeitsbaums: Produktplan, Designmanifest,
aktive Rust-Pfade für Räume, Shell/Settings, Login und Compositor, Dokumentation,
CI/Release sowie gezielte Suche nach Altbezeichnungen und auffälligen
Fehlerpfaden. Maßgeblich sind `NIWOE_IMPLEMENTATION_PLAN.md`,
`docs/niwoe_design_manifest.md` und die Nutzerentscheidungen vom 24./25.09.2026.
Dies ist eine Quellcode- und Prozesssichtung, kein vollständiges Security-Audit,
Profiling oder erneuter Fedora-Livetest. Der Arbeitsbaum enthält bereits viele
laufende P04/P05-Änderungen; dieser Audit wertet deren aktuellen Stand aus.

## Befunde nach Dringlichkeit

### Hoch – Reproduzierbarkeit des installierten Stands

`git status --short` zeigte 67 geänderte oder unversionierte Pfade, darunter
Compositor, IPC, Shell, Tokens und Phasenberichte. Der jüngste Commit ist
`bd4f81a` vom 23.09.2026; die inzwischen auf Fedora installierten und geprüften
P04/P05-Stände liegen also im ungesicherten Arbeitsbaum. Ein neuer Checkout kann
den Teststand nicht aus Git reproduzieren. Vor größeren P06-Änderungen sollten
die bestehenden Änderungen nach Verantwortlichkeit geprüft und in kleine,
getestete Commits übernommen werden. Keine Altdatei allein wegen ihres Namens
löschen.

### Hoch – Raumkonfiguration besitzt erst einen schmalen Datenpfad

Die Konfigurationsansicht zeichnet sechs Reiter, aber `ConfigurationAction` und
`hit_configuration` bieten nur Zurück, Name, Früher/Später, Speichern und
Abbrechen (`crates/niwoe-shell/src/room_management_view/configuration.rs:43,138,301`).
Beschreibung, Wiederherstellung, Start-Apps und Automatisierung sind sichtbare
Fähigkeitsgrenzen (`configuration/body.rs:74,139,183`). Das ist im aktuellen
Zwischenstand ehrlich gekennzeichnet, erklärt aber den vom Nutzer beobachteten
geringen Funktionsumfang.

Das persistente Schema kennt bereits Beschreibung und Zuordnungsmodus
(`crates/niwoe-config/src/rooms.rs:31`), der Shell-Snapshot überträgt sie nicht.
`RoomChange` kann nur `Rename`/`Move` (`crates/niwoe-ipc/src/rooms.rs:20`);
`RoomRegistry::apply` setzt nur diese beiden Mutationen um und `open` akzeptiert
vorübergehend exakt neun Räume (`crates/niwoe-compositor/src/room_registry.rs:21,79`).
Weitere Formulare ohne diesen Pfad wären Scheinfunktionen. P06 (Raummodell,
Erzeugen/Löschen, IPC und Revisionen) ist der erste fachliche Ausbau, danach
P07 Zuordnung, P08 echte Raumdaten im Hub, P09 Restore und P10 die vollständige
Konfiguration gemäß aktivem Plan.

### Hoch – Login-Branding widerspricht dem Designmanifest

Der aktive Login-Renderer zeichnet weiterhin eine Kompassmarke und den Text
`M E R I D I A N` (`crates/niwoe-login/src/main/ui.rs:65,69`,
`main/controls.rs:47`). Das Manifest erlaubt in Welcome, Login und About nur die
NIWOE-Wortmarke und schließt den Kompass aus
(`docs/niwoe_design_manifest.md:108`). Die Login-UI ist deshalb ein konkreter
visueller Produktwiderspruch. Den Renderer/Font-Pfad nicht pauschal entfernen;
zuerst die sichtbare Marke gezielt ersetzen und am Login-Bild prüfen.
Der vorhandene Bootsplash und Loginmanager sind ausdrücklich zu erhalten;
dieser Befund fordert weder ihren Ersatz noch Änderungen an PAM oder Sitzung.

### Mittel – altes Theme- und Distributionsverhalten ist noch erreichbar

Die alte Settings-Navigation enthält weiterhin die Kategorie `Theme`, deren
Light/Dark-Auswahl und die Aktion `ApplyThemeByIndex`
(`crates/niwoe-shell/src/settings_view.rs:56`,
`settings_view/content/theme.rs`, `wayland/handlers/widget_dispatch/dispatch.rs:144`).
Die Shell rendert diese Settings-Ansicht weiterhin
(`wayland/render/launcher.rs:151`). Das widerspricht dem festgelegten einen
dunkelgrünen Alpha-Theme; P10-06 plant die Entfernung bereits ausdrücklich.
Bis dahin keine zweite Theme-Abnahme aus altem Code ableiten. Bestehende
Nutzerkonfiguration bei der Migration erhalten.

Die aktive Seite „Updates“ startet ausschließlich `apt list --upgradable` und
meldet auf Fedora „apt nicht verfügbar“
(`crates/niwoe-shell/src/updates.rs:1,55`). Der Aufruf läuft korrekt in einem
Hintergrund-Thread und blockiert die Shell nicht; die Plattformfunktion ist
auf dem vorgesehenen Fedora-Testsystem jedoch unzutreffend. Bei Weiterbetrieb
der Seite braucht sie eine gekapselte Fedora-Implementierung oder einen klaren
Status „derzeit nicht unterstützt“.

### Mittel – aktive Dokumentations-Einstiege sind veraltet

`README.md:32` bezeichnet noch zwei Themes als verbindlich und enthält alte
P01-Namenshinweise (`README.md:19,55`). `docs/phase-reports/README.md:3`
priorisiert einen inzwischen behobenen P05-Dialogfehler und nennt alte
Dark/Light-Gates. Zugleich führt `docs/README.md:12,28` diese Seite als aktuellen
Status. `docs/NIWOE_LOGIN.md:4` und `docs/DESKTOP_SETTINGS_CONTRACT.md:3,52`
beschreiben noch WebKit als mögliche/aktive UI-Grenze; `docs/README.md:47-48`
verzeichnet sie als aktuelle Subsystemreferenzen. Diese Einstiege können Arbeit
auf überholte Pfade lenken. Inhalt anhand der implementierten nativen Grenzen
aktualisieren oder klar historisch markieren. Historische Evidenz behalten.

### Niedrig – automatische Lieferung und verbleibende Prüfgrenzen

`.github/workflows/ci.yml` deckt Formatierung, Clippy, Workspace-Check und
Workspace-Tests auf Ubuntu ab. `.github/workflows/release.yml` veröffentlicht
Release-Notizen, aber keine Binärartefakte. Die Fedora-Installation ist aktuell
ein dokumentierter manueller Testrechnerprozess. Für eine teilbare Alpha fehlt
somit noch ein reproduzierbarer Artefaktpfad; dies ist kein Hindernis für den
unmittelbaren P06-Beginn, sollte aber vor externer Auslieferung geklärt werden.

Gezielte Suche nach `todo!`, `unimplemented!` und Produktions-`panic!` ergab
keinen offensichtlichen harten Laufzeitabbruch. Die untersuchten `unwrap()` in
`crates/niwoe-portal/src/file_chooser.rs:233,240` liegen in Tests. Login-
Rechtewechsel, Raum-Persistenz und Portal-Freigabe wurden nur statisch gesichtet;
daraus folgt keine Sicherheitsfreigabe. Idle-CPU/GPU, Eingabelatenz und DRM/
HiDPI benötigen für die Phasenabnahme Messungen auf Fedora.

## Bewusst erhaltene Altpfade

- `crates/niwoe-config/src/migration.rs` und `environment.rs` enthalten
  `meridian`/`MERIDIAN_` als Daten- und Umgebungs-Migration. Die Namen sind hier
  Kompatibilität, keine sichtbare Produktmarke.
- `docs/design-reference/webkit-prototype/`, ältere Audits und BSD-Pläne sind
  historische Referenz und im Dokuindex bereits entsprechend eingeordnet.
- Der ungenutzte Light-Code darf laut Produktentscheidung bestehen bleiben;
  entscheidend ist, ob die Alpha ihn im aktiven Ablauf anbietet.

## Empfohlene nächste Reihenfolge

Nutzerentscheidung nach diesem Audit: Das Raummodell wird vor der isolierten
Branding-Korrektur umgesetzt. Siehe `phase-reports/P06_ROOM_MODEL_FOUNDATION.md`.

1. Aktuellen, getesteten P04/P05-Stand nachvollziehbar versionieren und die
   veralteten Einstiegshinweise korrigieren.
2. Den Login-Widerspruch als kleinen, isolierten Designfix beheben und den
   installierten Login-Build prüfen.
3. P06-Raummodell entlang des bestehenden Plans umsetzen: zunächst stabile IDs,
   variable Anzahl, Validierung und persistente Create/Delete/Update-Operationen
   samt Snapshots/Revisionen; anschließend Shell und Bedienpfade anbinden.
4. Danach die sichtbaren Konfigurationsbereiche in der Reihenfolge ihrer echten
   Fähigkeiten freischalten; Theme-Altansicht und Fedora-Updates gesondert im
   geplanten Settings-Umbau bereinigen.

## Verifikation dieses Audits

Genutzt wurden `git status --short`, `git log -1`, `rg` und gezielte Lektüre der
oben genannten Quellen sowie der CI-Workflows. Keine Rust-Dateien, Binärdateien
oder Fedora-Installation wurden in diesem Audit verändert. Cargo-Gates wurden
deshalb nicht erneut ausgeführt; die vorherigen Fedora-/Workspace-Gates sind
im jeweiligen P04/P05-Phasenbericht dokumentiert.
