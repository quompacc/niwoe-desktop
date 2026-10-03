# Ruhigere Controls und einfacheres Control Center

Nutzerauftrag 03.10.2026: gelbe Aktivitätsstriche entfernen, gequetschte
Bedienelemente entzerren und die komplexe Menüführung vereinfachen. Dies ist
eine ausdrückliche Fortschreibung des Designmanifests; das zentrale dunkelgrüne
Theme und die native Rust-Architektur bleiben maßgeblich.

## Änderung und betroffene Dateien

| Datei beziehungsweise Verantwortung | Änderung |
| --- | --- |
| `niwoe-tokens`: `color.rs`, `spacing.rs`, `control_center.rs`, `settings.rs` | Neutraler kontrastreicher Fokus, Controls mindestens 40 px und Formfelder 44 px, größere Abstände und Innenränder. |
| `niwoe-ui`: `widget/component.rs`, `selection_row.rs`, `tests/components.rs` | Neutrale Auswahlflächen, keine gelben unteren Auswahlstriche; Tabs/Chips/Karten kennzeichnen Auswahl zusätzlich mit Häkchen. Regressionen für Auswahl und Fokus bei 100/150/200 Prozent. |
| Shell: `control_center.rs`, `wayland/state/control_center.rs`, `wayland/handlers/keyboard/control_center_navigation.rs` | Sieben tatsächlich erreichbare Sidebarziele: Räume, Apps, Darstellung, Leiste, System & Geräte, Benutzer, Updates. Keine separate doppelte Übersicht und keine globalen Datei-/unverfügbaren Backup-/Logeinträge. Tastaturreihenfolge folgt der sichtbaren Sidebar. |
| Shell: Settings-Navigation, Contentbuilder und Default-App-Paging | Eine einzige Unterseite benötigt keine zusätzliche Tab-Leiste; Zeichnung und Eingabepaging verwenden dieselbe tatsächliche Navigationshöhe. Alle vorhandenen Settings-Provider bleiben erreichbar. |
| Shell: Raumverwaltung, Konfigurationsformular, App-Auswahl, Leistenkonfiguration | Mehr Platz, neutrale aktive Flächen, sechs App-Auswahlen als zwei Spalten mit drei Zeilen. Dateien/Wiederherstellung bleiben beim jeweiligen Raum. Entfernte Schnellaktionen waren unverfügbare Platzhalter. |
| Shell: Hub, Workspaces und Window-Picker-Test | Neutrale Auswahl-/Fokusdarstellung; entfallener Goldstreifenhelper `ui/primitives.rs` entfernt. Der BGRA-Fokustest prüft weiterhin die tatsächliche Tokenfarbe. |
| Shell: Einführungslayout | Unnötigen Zwischenraum vor der Leistenpreview reduziert, damit die größeren Controls und der Footer auch im bisherigen Mindestcanvas passen. |
| Manifest, Agent-Regeln und aktiver Plan | Nutzerkorrektur, Reihenfolge und wieder betroffene visuelle Gates ausdrücklich dokumentiert. |

Konkrete Dateiliste dieser Designrunde, jeweils relativ zum Repository:

| Datei | Änderung |
| --- | --- |
| `crates/niwoe-tokens/src/color.rs` | Zentrale Fokusfarbe neutral und kontrastreich. |
| `crates/niwoe-tokens/src/spacing.rs` | Gemeinsame Mindesthöhen und Höhe für zweizeilige Controls erhöht. |
| `crates/niwoe-tokens/src/control_center.rs` | Abstände, Formular-/Tab-/Footerhöhe und Verwaltungsrail zentral angepasst. |
| `crates/niwoe-tokens/src/settings.rs` | Größere Innenränder/Abstände; Cursorwahl in drei Spalten. |
| `crates/niwoe-ui/src/widget/component.rs` | Neutrale Auswahl, zurückhaltende Tab-/Chip-/Kartenkontur, Häkchen statt Goldstreifen. |
| `crates/niwoe-ui/src/widget/selection_row.rs` | Neutrale ausgewählte Zeilenfläche. |
| `crates/niwoe-ui/tests/components.rs` | Auswahl-/Fokusregressionen für drei Skalen und Pointerzustände. |
| `crates/niwoe-shell/src/control_center.rs` | Sieben Sidebarziele und neutraler aktiver Eintrag; Layout-/Navigationstests angepasst. |
| `crates/niwoe-shell/src/wayland/state/control_center.rs` | Vorhandene Settings-Einstiege auf die vereinfachte Navigation abgebildet. |
| `crates/niwoe-shell/src/wayland/handlers/keyboard/control_center_navigation.rs` | Vorwärts-/Rückwärts-Wrap durch die sieben sichtbaren Ziele samt Tests. |
| `crates/niwoe-shell/src/settings_view/navigation.rs` | Redundante einzelne Untertabs entfallen, gemeinsame tatsächliche Navigationshöhe. |
| `crates/niwoe-shell/src/settings_view/content_builders.rs` | Inhaltshöhe berücksichtigt entfallene einzelne Untertabs. |
| `crates/niwoe-shell/src/wayland/handlers/widget_dispatch/default_apps_paging.rs` | Default-App-Eingabepaging verwendet dieselbe Höhe wie die Zeichnung. |
| `crates/niwoe-shell/src/room_management_view/sidebar.rs` | Gemeinsamer vereinfachter Sidebaraufruf. |
| `crates/niwoe-shell/src/room_management_view/cards.rs` | Neutrale aktive Raumkarten. |
| `crates/niwoe-shell/src/room_management_view/list.rs` | Neutrale aktive Filterflächen. |
| `crates/niwoe-shell/src/room_management_view/management.rs` | Neutrale Auswahl in der Verwaltung. |
| `crates/niwoe-shell/src/room_management_view/management/rail.rs` | Nur die vorhandene echte Schnellaktion; größere Statistikgeometrie. |
| `crates/niwoe-shell/src/room_management_view/configuration/chrome.rs` | Raumdateien bleiben im Bereich Räume; neutrale aktive Tabs. |
| `crates/niwoe-shell/src/room_management_view/configuration/form.rs` | Größere Formfelder und sechs Appoptionen als 2×3; Hit-/Zeichengeometrie gemeinsam. |
| `crates/niwoe-shell/src/room_management_view/configuration/form/apps.rs` | Appauswahl verwendet neutrale Flächen. |
| `crates/niwoe-shell/src/room_management_view/configuration/actions.rs` | Raumaktionen verwenden die gemeinsame Fokusfarbe. |
| `crates/niwoe-shell/src/room_management_view/configuration/restore/draw.rs` | Neutrale Auswahl in der Wiederherstellungsansicht. |
| `crates/niwoe-shell/src/room_management_view/first_run/layout.rs` | Previewabstand passt zu größeren Controls im Mindestcanvas. |
| `crates/niwoe-shell/src/room_management_view/panel.rs` | Neutrale aktivierte Modulflächen und Häkchen. |
| `crates/niwoe-shell/src/hub_view/rooms.rs` | Neutrale Raumwahl im Hub. |
| `crates/niwoe-shell/src/workspaces.rs` | Aktive Fläche neutral, oberer Goldstreifen entfernt. |
| `crates/niwoe-shell/src/ui/primitives.rs` | Obsoleter Goldstreifenhelper entfernt. |
| `crates/niwoe-shell/src/ui/mod.rs` | Entfernten Helper nicht mehr exportieren. |
| `crates/niwoe-shell/src/window_picker.rs` | Fokusregression prüft neutralen Tokenwert und weiterhin BGRA-Reihenfolge. |
| `docs/niwoe_design_manifest.md` | Explizite Nutzerkorrektur als verbindliche Designvorgabe. |
| `AGENTS.md` | Nutzerkorrektur in den Projektinvarianten verankert. |
| `NIWOE_IMPLEMENTATION_PLAN.md` | Aktueller installierter Stand, Prüfungen und verbleibende R9-Grenzen. |
| `docs/phase-reports/UI_VISUAL_AUDIT_2026-10-02.md` | Neue native Primärwechsel- und Designbelege mit tatsächlichen Grenzen. |
| `docs/phase-reports/evidence/r9/PREPARATION.md` | Veralteten Wartezustand durch aktuelle Nachweise ergänzt. |
| `docs/phase-reports/evidence/r9/primary-after/` und dieses Verzeichnis | Getrennte historische/aktuelle Logs, Helper, Hashes und Bereinigungsbelege. |

## Automatische Verifikation und Installation

Endgültiger Shell-Release:
`e9bed1136f1dc3c014a498f90b03fd56e531fae687163a0279ab7c383a703451`.
Installiert, über den vorhandenen Watchdog aktiviert und über `/proc/PID/exe`
bestätigt; Shell PID 678998. Compositor PID 647751 bleibt
`8f1e15e3678860e9160cc00ee17e61c0650e9c15ae1ecb086fef53e48c3290e3`.
Kein neuer Login für diese Shelländerung erforderlich. Geschützte NIWOE-,
MIME-, KDE- und GTK-Dateien beim Aktivieren bytegleich.

Fedora-Gates in `gates/`: 180 normalisierte Quelldateien, keine Hashabweichung;
`cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace` mit 1311 bestandenen Tests, 0 Fehlern und einem
bestehenden isolierten D-Bus-Ignore;
`cargo test -p niwoe-tokens --test design_guard`, Größen-/Zentralitätsguards,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo build --release -p niwoe-shell` bestanden.
Der lokale Windows-Workspacecheck scheitert an der fehlenden Linux-
Wayland/pkg-config-Umgebung; maßgeblich ist der vollständige Fedora-Lauf.
Keine neuen Dependencies oder Cargo-Manifeständerungen.

## Tatsächlich angesehene native Ansichten

`cases.json`, `cleanup.json` und `viewed.json`: 19 aktuelle Bilder auf FHD bei
100 Prozent tatsächlich einzeln angesehen und gegen die Aufnahmehashes geprüft.
Desktop nach Watchdog ohne erneute Begrüßung, regulärer Hub und Tastaturfokus,
alle sieben Sidebarziele, Hintergrund mit geladenen Vorschauen, Anzeige,
Mauszeiger, Audio, Netzwerk sowie Raumkonfiguration mit Allgemein/Apps/Dateien.
Neutrale Flächen und Fokusrahmen sichtbar; alle sechs Appauswahlen passen.
Maus-/Tastaturwege und Sidebar-Wrap nachgewiesen. Globale Einstellungen liegen
im Control Center, die Dateiseite bleibt dem gewählten Raum zugeordnet.

Die ersten Hintergrundbilder zeigen den legitimen asynchronen Ladezustand;
die nachfolgende Fokusaufnahme zeigt die tatsächlich geladenen Vorschauen.
WLAN-Liste und Fedora/RPM-Paketintegration melden ihre realen Grenzen weiterhin
offen. Diese Darstellung ist kein Nachweis neuer Backend-Funktionalität.
Sämtliche Cleanupfelder true: Originaldateien und roher Snapshot identisch,
leere neutrale Loge, Loginmarkierung und Prozess unverändert. Keine Speichervorgänge,
Appstarts, Geräteaktionen oder Änderung bestehender KDE-/GTK-Einstellungen.
Private PNGs verbleiben ausschließlich im Git-ignorierten, autorisierten
lokalen Prüfverzeichnis; hier stehen nur Metadaten und Bildhashes.

### Beide Monitore und kleinere Ansichten

`matrix/`: 60 weitere Aufnahmen, alle einzeln angesehen und gegen ihre
Aufnahmehashes geprüft. Zehn Fälle je tatsächlichem FHD-/UHD-Primärmonitor
bei 100/150/200 Prozent: sieben Sidebarseiten sowie Raum-Allgemein,
zusätzlicher Scrollversuch und Raum-Apps. Native Modi FHD 60 Hz/UHD 30 Hz
erhalten; primäre Layer wechseln im laufenden Prozess auf den jeweiligen
Monitor. Alle sieben Seiten, Footer und sechs Appauswahlen bleiben sichtbar.
Configbytes, andere geschützte Dateien, roher ursprünglicher Sitzungs-/
Outputsnapshot und beide Prozessidentitäten nach dem Lauf exakt erhalten.
Alle Cleanupfelder true, Exit 0.

Der erste Matrixhelper scrollte mit dem falschen Vorzeichen zum bereits
erreichten oberen Rand. Seine `room-general-lower`-Bilder werden ausdrücklich
nicht als Nachweis des unteren Formularbereichs gezählt. `scroll/` enthält
den getrennten Ergänzungslauf mit korrekter Richtung: acht Bilder bei FHD
150/200 Prozent tatsächlich angesehen, darunter beide unteren Formularbereiche
mit erreichbaren App-/Dateiverweis-/Raumlöschcontrols und unverändert sichtbarem
Footer. Tabwechsel zu Apps anschließend weiterhin funktionsfähig. Auch dieser
Lauf endete mit Exit 0 und vollständig identischem Originalzustand.

Insgesamt 87 tatsächlich angesehene Bilder auf dem endgültigen Release;
17 Bilder des früheren Zwischenrelease separat. Bei kleinen logischen Größen
bleiben einige lange Erläuterungen in den unteren Formularkarten gekürzt;
die vollständige Text-/Dichteabnahme ist damit nicht behauptet. V23 bleibt
sichtbar: die logische Shellrasterung wird bei höherer Skalierung weich, unter
dem Mindestcanvas zusätzlich eingepasst. Die Layoutprüfung ist kein Nachweis
nativer hochauflösender Schrift oder maßstabsgetreuer Mindestzielgrößen.

Nach beiden Läufen eigene `/dev/uinput`-Freigabe erst auf ausschließlich eigene
ACL-Abweichung geprüft, danach ursprünglichen Snapshot wiederhergestellt und
exakte Gleichheit bestätigt (`input-cleanup.json`). Keine Prüfhelper laufen weiter.

## Historische Zwischenstände und Fehlversuche

`initial-layout-failures/`: erster größerer Abstandsstand mit fünf echten
Mindestlayout-Testfehlern. Die Geometrie wurde korrigiert, nicht die Assertions
entfernt. `gates-first-pass/`, `cases-first-pass.json`,
`cleanup-first-pass.json`, `viewed-first-pass.json` und die zugehörigen Helper:
bestandener Zwischenrelease `f5de3bdc…`, 17 wirklich angesehene Bilder.
Er besaß noch goldene Fokusrahmen und Leistenmodulflächen und wurde deshalb
durch den oben dokumentierten neutralen Release ersetzt.

`neutral-focus-test-failure/`: der alte Window-Picker-Test erwartete nach der
bewussten zentralen Farbänderung weiterhin Gold. Er prüft jetzt die neue
Tokenfarbe und die unveränderte BGRA-Reihenfolge; endgültige Gates bestanden.
`capture-timeout/`: erster finaler Bildlauf direkt nach Aktivierung endete
beim Aufnahme-Zustimmungsdialog mit Timeout, bevor ein Bild entstand.
Bereinigung vollständig bestätigt. Der getrennte Wiederholungslauf auf der
stabil laufenden Shell endete erfolgreich; sein tatsächlicher Helper steht
in `native-helper.py`. Die Timeoutursache ist nicht als Produktfix behauptet.

## Performance-Modell und Grenzen

Die Controls zeichnen weiterhin ausschließlich über vorhandene Invalidierung;
kein Timer, neue Animation, Blurpass, I/O oder laufendes Decoding. Der entfernte
Streifen spart einen Zeichenpfad. Auswahlhäkchen nutzen den bestehenden begrenzten
Symbolcache mit Größen-/Farbschlüsseln; kein neuer unbeschränkter Cache.
Raster-/Canvasstrategie und Renderstack-Reihenfolge unverändert. Die größere
Geometrie wird durch vorhandenes Paging und Formularscrollen aufgenommen.

Die neue lange RSS-/Ruheprobe ist noch nicht durchgeführt. Frühere R7/R8-
Messungen sind keine Messungen dieses Binärstandes. Native hochauflösende
Glyphenrasterung V23, heller Apphintergrund V02, weitere Fokus-/Nebenflächen,
Schutz-/Hardwarefälle und R9-Gesamtabnahme bleiben offen. P13 bleibt unbegonnen.
