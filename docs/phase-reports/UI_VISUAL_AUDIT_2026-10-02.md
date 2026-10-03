# NIWOE Liveprüfung der Benutzeroberfläche am 2 Oktober 2026

Die installierte Oberfläche erfüllt die vier verbindlichen Mockups nicht.
Die Nutzerkritik ist durch echte Bildschirmaufnahmen und Bedienläufe bestätigt.
Grüne Tests und frühere Funktionsberichte begründen keine visuelle Fertigmeldung.
Die betroffenen visuellen Gates P02–P12 sind wieder offen; P13 bleibt unbegonnen.

Dieser Durchlauf erfasst die regulären Desktop-, Hub-, Verwaltungs-, Settings-
und Einführungspfade. Er enthält einen geprüften und auf Fedora installierten
Fix für die Rücknavigation aus den Einstellungen. Die übrigen unten genannten
Gestaltungs- und Funktionsmängel sind ausdrücklich noch offen.

**Verbindliche Abarbeitung:** Der [aktive Reparaturplan im Umsetzungsplan](../../NIWOE_IMPLEMENTATION_PLAN.md#aktiver-reparaturplan-für-die-benutzeroberfläche)
ordnet V01–V20 Arbeitspaketen und sichtbaren Abschlusskriterien zu und ergänzt
die noch fehlende Prüfmatrix. Dieser Bericht bleibt die Befund-/Belegbasis;
seine grobe Reparaturfolge unten ist keine zusätzliche aktive Roadmap.

## Vergleichsgrundlage und tatsächliche Prüfbedingungen

Maßgeblich sind [Designmanifest](../niwoe_design_manifest.md),
[Mockup- und Workflowplan](../MOCKUP_WORKFLOW_PLAN.md) und ausschließlich
die vier Bilder `assets/ChatGPT Image 21. Sept. 2026, 17_13_54 (1)` bis `(4).png`.
Alle vier Originale wurden angesehen und mit nativen Ausgaben verglichen.
Die späteren Nutzerkorrekturen gelten weiter: kein Alltagsbranding, keine
angehefteten App-Symbole im Panel, mittige Uhr, kein separater Spotlight-Launcher,
neutraler Login in der Loge und vorerst kein eigener Automatisierungsbereich.
Diese gewollten Abweichungen werden nicht als fehlende Mockup-Elemente bewertet.

Prüfgerät: vorhandener Fedora-Testrechner, bestehende NIWOE-Sitzung.
Primary `drm-0`: 1920×1080, 60 Hz, 100 %. Secondary `drm-1`:
3840×2160, 30 Hz, 100 %. Die hier untersuchten UI-Aufnahmen stammen vom Primary.
Anfangszustand: neun vorhandene Räume, neutrale Loge, keine App-Fenster,
bereits defekter transparenter Hub. Später wurde KWrite aus dem Hub gestartet,
um belegte Karten und Glas über einer realen Anwendung zu prüfen. Es wurde
kein Dokument geöffnet oder bearbeitet.

Die Eingaben erfolgten über echte virtuelle Linux-Eingabegeräte. Screenshots
wurden über den vorhandenen Screenshotpfad mit realer Zustimmung aufgenommen
und als Bilder angesehen. IPC-Snapshots dienten zusätzlich zum Abgleich der
Raumdefinitionen und Fenster. Keine reine Render-Testausgabe ersetzt diese Bilder.
Bei den ersten Escape-Versuchen fehlte die Geräteankündigung dieser Taste im
Prüfhelfer. Deshalb zählen Aufnahmen 49/50 nicht als Escape-Nachweis; nach
Korrektur des Prüfhelfers belegen erst 51/52 den tatsächlichen Tastaturweg.

Ausgewählte Belege liegen im versionierbaren
[Belegverzeichnis](UI_VISUAL_AUDIT_2026-10-02-evidence/).
Die vollständige lokale Aufnahmefolge und Prüfhilfen liegen unter
`target/ui-visual-review-2026-10-02/` beziehungsweise `target/` und sind nicht
versioniert. Insbesondere Netzwerkaufnahmen mit vorhandenen SSIDs bleiben dort.

## Die vier verbindlichen Bildschirme

| Vorgabe | Tatsächlicher Zustand | Offene Korrektur |
| --- | --- | --- |
| Desktop und Deck `(1)` | Panelgrundstruktur, mittige Uhr und kleines Deck sind vorhanden. Einige Nebenansichten zeigen andere Ausrichtung und Sprache. | Gemeinsame Icon-, Fokus- und Textrollen sowie Popup-Anker prüfen. Freigegebenes Deckmaterial bewahren. |
| Hub `(2)` | Vier Raumkarten und drei untere Bereiche sind vorhanden. Der große atmosphärische Bildkopf fehlt; die Überschrift und Hierarchie sind erheblich kleiner. Die verfügbare Fenstervorschau füllt nur einen kleinen Teil der Karte. Über KWrite sind dessen Texte deutlich durch die Hubfläche lesbar. | Bildkomposition, Überschriftenrollen, Kartengeometrie, Vorschauen und lesbare Materialwirkung gegen beide Hintergründe herstellen. |
| Räume verwalten `(3)` | Sidebar, Raster und rechte Spalte existieren. Der Bildkopf ist lediglich eine transparente Tönung; mit KWrite dahinter wird er grau und zeigt dessen Menü. Raumkarten enthalten auch mit vorhandenem Fenster nur Titeltext. Zahlreiche Icons und Verwaltungscontrols fehlen. | Vollständiger Bildkopf, Karteninhalt und Toolbar im vorgesehenen Raster; gemeinsame Navigation ohne Wechsel in die alte Settings-Familie. |
| Raum konfigurieren `(4)` | Grundspalten und Reiter existieren. Der Bildkopf wird stark gestaucht. Rechts steht eine große leere Raumkarte mit Fenstertitel statt einer räumlichen Vorschau. Details und Kontextoptionen wurden in schlichte Textbuttons reduziert. | Seitenverhältnis, Typografie, Details-/Controlhierarchie und reale Vorschau korrigieren; nicht verfügbare Fähigkeiten innerhalb dieser Struktur ehrlich kennzeichnen. |

Das Fehlen echter Daten rechtfertigt keine andere Komposition. Beispiel:
[leere Verwaltung](UI_VISUAL_AUDIT_2026-10-02-evidence/04-room-management.png)
und [Verwaltung mit KWrite](UI_VISUAL_AUDIT_2026-10-02-evidence/41-management-with-window.png)
zeigen beide keine vorgesehene bildliche Kartenvorschau. Eine belegte Karte
enthält lediglich „Willkommen — KWrite“.

## Reproduzierter Navigationsfehler und installierte Korrektur

Vorher: Hub öffnen → Räume verwalten → Sidebar Apps → alte Standard-Apps-Seite
→ Zurück. Die Rückkehr führt sofort zum Hub; er verschiebt sich um 24 Pixel
nach unten und verliert sein compositorseitiges Glas. Auf 1920×1080 liegt
der reguläre Hub bei `(260, 124)`, der defekte bei `(260, 148)`.

Belege: [regulärer Hub](UI_VISUAL_AUDIT_2026-10-02-evidence/03-hub-panel-click.png),
[alte Apps-Seite](UI_VISUAL_AUDIT_2026-10-02-evidence/05-sidebar-apps.png),
[defekter Rückweg](UI_VISUAL_AUDIT_2026-10-02-evidence/06-hub-after-apps-back.png).

Ursache: Die Verwaltung setzt die Launcher-Layer-Exclusive-Zone auf `0` und
belegt die Arbeitsfläche unterhalb des Panels. Die Settings-Übergänge löschten
nur Ansichtsflags, ohne die volle Outputfläche mit Zone `-1` wiederherzustellen.
Der DRM-Renderpfad in `scene_composition.rs` komponiert Hubglas nur für eine
Launcher-Layer mit Ursprung am oberen Outputrand. Die verbliebene Layerlage
erklärt sowohl die Verschiebung als auch das fehlende Glas.

Die Settings-Einstiege aus Widgets, Panel, IPC und Audio-/Netzwerkpfaden nutzen
jetzt denselben Vorbereitungspfad. Beim Übergang aus der Verwaltung wird die
Outputfläche wiederhergestellt und ein neuer Configure abgewartet. Der
Rückweg merkt sich die Verwaltung als Herkunft. Zurück und Escape verwenden
denselben Schließpfad; aus der Verwaltung aufgerufene Einstellungen kehren zur
Verwaltung zurück, Hub-Einstiege zum Hub. Schließen beziehungsweise erneutes
Öffnen des Launchers löscht diese Herkunft.

Nachher mit Maus:
[Apps in korrekter Layerlage](UI_VISUAL_AUDIT_2026-10-02-evidence/46-apps-fixed-layer.png)
→ [Verwaltung](UI_VISUAL_AUDIT_2026-10-02-evidence/47-apps-back-management.png)
→ [Hub](UI_VISUAL_AUDIT_2026-10-02-evidence/48-apps-back-hub.png).
Nachher mit Escape:
[Verwaltung](UI_VISUAL_AUDIT_2026-10-02-evidence/51-apps-escape-management.png)
→ [Hub](UI_VISUAL_AUDIT_2026-10-02-evidence/52-apps-escape-hub.png).
Auch nach Schließen der eigenen Testanwendung wurde derselbe Rückweg erneut
geprüft: [finaler Hub vor dem Wallpaper](UI_VISUAL_AUDIT_2026-10-02-evidence/60-final-hub.png).

**Grenze des Fixes:** Apps öffnet weiterhin die alte Standard-Apps-Seite.
Die separate Settings-Architektur und die schlechte Materialwirkung über hellen
Anwendungen sind damit nicht behoben. Der Fix beseitigt den falschen
Flächenzustand und Rückweg; er ist keine visuelle Hub- oder Control-Center-Abnahme.

Performance-Modell: ein zusätzlicher Herkunftsstatus und Layerkonfiguration
nur beim Ansichtswechsel. Keine neuen Timer, Animationen, Effekte, Assets oder
Renderdurchläufe im Leerlauf; Renderreihenfolge und vorhandene Caches bleiben.

## Weitere konkrete Befunde

| Nr. | Befund und Beleg | Nächster betroffener Pfad |
| --- | --- | --- |
| V01 | Die alte Settings-Familie hat eine andere Sidebar und eine zentrierte Karte statt des vollständigen Control Centers. Apps, Benutzer, System und weitere Verwaltungsziele verlassen den gemeinsamen Rahmen. [Apps](UI_VISUAL_AUDIT_2026-10-02-evidence/46-apps-fixed-layer.png). | `state/panel_form.rs`, `settings_view` und Control-Center-Navigation. |
| V02 | Hubkopf ohne eigene Landschaftskomposition; Überschrift und Untertitel haben nicht die Gewichtung aus `(2)`. Über KWrite ist der Inhalt darunter klar erkennbar. [Hub mit App](UI_VISUAL_AUDIT_2026-10-02-evidence/39-hub-with-window.png). | Gemeinsame Tokens für Überschriften/Abstände, Hubkopf und vorhandener Materialpfad. |
| V03 | Verwaltungskopf lässt Anwendungsmenü und weiße Fläche durchscheinen. Es gibt dort kein eigenes Landschaftsasset. [Verwaltung mit App](UI_VISUAL_AUDIT_2026-10-02-evidence/41-management-with-window.png). | `room_management_view.rs::draw_header` malt nur die transparente Tönung. |
| V04 | Der Konfigurationskopf staucht das Landschaftsbild unabhängig in X und Y. [Konfiguration](UI_VISUAL_AUDIT_2026-10-02-evidence/42-config-with-window.png). | `room_management_view/configuration.rs::draw_landscape`; proportionaler Ausschnitt mit passender Cachestrategie. |
| V05 | Verwaltungskarten und große Konfigurationsvorschau besitzen mit echtem Fenster kein Bild. Generische Diamanten ersetzen differenzierte Raumdarstellung. [Vorschau](UI_VISUAL_AUDIT_2026-10-02-evidence/42-config-with-window.png). | Bestehende Vorschauquelle in Verwaltungs-/Konfigurationskarten einbinden; ehrliche leere Zustände erhalten. |
| V06 | Toolbar ohne Zähler in Filterchips, ohne Kategorienauswahl und ohne Raster-/Listencontrols. „Raumreihenfolge“ ersetzt das vorgesehene Sortierungscontrol. [Verwaltung](UI_VISUAL_AUDIT_2026-10-02-evidence/04-room-management.png). | Toolbarstruktur gemäß `(3)`, tatsächlich verfügbare Fähigkeiten kennzeichnen. |
| V07 | Sidebar ohne Icons; aktive Markierung bleibt auch in der Leistenkonfiguration auf Räume stehen. [Leiste](UI_VISUAL_AUDIT_2026-10-02-evidence/11-panel-configuration.png). | Gemeinsame Sidebar aus aktueller Ansichtsidentität ableiten. |
| V08 | Raumdetails zeigen einen Textbutton „Icon“ statt einer Symbolvorschau. Kontextoptionen sind riesige Textflächen statt klarer Controlzeilen. [Allgemein](UI_VISUAL_AUDIT_2026-10-02-evidence/07-room-general.png). | Gemeinsame native Formularcontrols mit zentralen Tokens. |
| V09 | Raum-Apps erscheinen als schlichte `[ ]`-Textzeilen ohne Appicons, mit technischen Native-/XWayland-Identitäten und sehr viel Leerraum. [Apps-Reiter](UI_VISUAL_AUDIT_2026-10-02-evidence/08-room-apps.png). | Kataloglisten, Zeilenhierarchie, Auswahl und Seitenwechsel. |
| V10 | Dateien und Wiederherstellung zeigen nahezu denselben Restore-Block. „Kein lesbares Layout: No such file or directory (os error 2)“ erscheint als roher Fehler. Das Dateifeld sitzt allein am unteren Rand. [Dateien](UI_VISUAL_AUDIT_2026-10-02-evidence/09-room-files.png), [Restore](UI_VISUAL_AUDIT_2026-10-02-evidence/10-room-restore.png). | Getrennte fachliche Seiten und verständliche Leer-/Fehlerzustände. |
| V11 | Die Leistenkonfiguration enthält eine winzige herunterskalierte Panelvorschau mitten in einer fast leeren Vollbildfläche. [Leiste](UI_VISUAL_AUDIT_2026-10-02-evidence/11-panel-configuration.png). | Previewlayout, Maßstab und Formulargruppen. |
| V12 | Wallpaperkacheln zeigen nur einfarbige Felder; sichtbare Einträge enthalten Duplikate und Screenshotbezeichnungen statt verlässlicher Bildvorschauen. Englische Fill/Fit/Center/Tile-Controls mischen sich mit Deutsch. [Wallpaper](UI_VISUAL_AUDIT_2026-10-02-evidence/13-settings-wallpaper.png). | Assetkatalog, gecachte Thumbnails, verständliche Namen und Lokalisierung. |
| V13 | Cursorseite ohne Cursorvorschauen, mit Rohbezeichnern wie `Breeze_Light` und `breeze_cursors`. [Cursor](UI_VISUAL_AUDIT_2026-10-02-evidence/14-settings-cursor.png). | Vorschau und benutzerlesbare Katalognamen. |
| V14 | „Angeheftet“ behauptet Auswahl von Anwendungen im Panel, obwohl diese Produktfunktion ausdrücklich entfernt wurde. [Angeheftet](UI_VISUAL_AUDIT_2026-10-02-evidence/15-settings-pinned.png). | Aktive Navigation und Beschreibung des tatsächlich verbleibenden Datenzwecks. |
| V15 | Standard-App-Zeilen haben kaum Abstand zwischen Haupttext und Unterzeile. Dropdownzeichen erscheinen als fehlende Glyphen. Auch Netzwerksicherheit zeigt Rechtecke statt Icons. [Standard-Apps](UI_VISUAL_AUDIT_2026-10-02-evidence/46-apps-fixed-layer.png); Netzwerkaufnahme 32 im lokalen Belegbestand. | Gemeinsame Textmetriken und vorhandenes natives Iconsystem, keine Unicode-Symbolannahmen. |
| V16 | Anzeige-, Audio-, Drucker- und Systemseiten zeigen technische IDs sowie gemischte englische/deutsche Texte. Updates meldet auf Fedora „apt nicht verfügbar“. [Updates](UI_VISUAL_AUDIT_2026-10-02-evidence/23-settings-updates.png). | Providergrenzen verständlich darstellen und Fedora-Paketpfad gesondert prüfen; kein Paketvorgang wurde ausgelöst. |
| V17 | Kalender öffnet rechts oben, obwohl er von der mittigen Uhr aufgerufen wird; englische Wochentagskürzel in sonst deutscher UI. Raumoverflow öffnet weit links unterhalb seiner sichtbaren Schaltfläche. | Popupankerpunkte, Lokalisierung und Fokusdarstellung am realen Panel prüfen. |
| V18 | First Run verwendet über fünf Seiten kleine Inhalte im oberen Bereich und eine fast leere Vollbildfläche, fünf gleich große Footerbuttons und teils lange Pseudo-Checkboxzeilen. Panelvorschau bleibt winzig. [Welcome](UI_VISUAL_AUDIT_2026-10-02-evidence/24-first-run.png), [Leiste im Wizard](UI_VISUAL_AUDIT_2026-10-02-evidence/27-first-run-panel.png). | Native Seitenhierarchie, lesbare Gruppen und klare primäre/sekundäre Aktionen; keine neue illustrative Funktion nötig. |
| V19 | Suche zeigt Text ohne Appicon; Ergebnis-Unterzeile liegt sehr dicht am Haupttext. Fensterliste verwendet auffälligen cyanfarbenen Fokus statt der sonstigen gemeinsamen Goldrolle. [Suche](UI_VISUAL_AUDIT_2026-10-02-evidence/30-hub-search.png), [Fensterliste](UI_VISUAL_AUDIT_2026-10-02-evidence/40-window-list.png). | Gemeinsame Ergebniszeile, Textrollen, Icons und Fokusrollen. |
| V20 | Die Vorschau eines noch ungespeicherten neuen Raums zeigt die technische Platzhalternummer „Raum 0“. [Neu-Raum-Entwurf](UI_VISUAL_AUDIT_2026-10-02-evidence/58-new-room-draft.png). | Verständlicher Entwurfszustand ohne temporäre Workspace-Nummer; die Loge bleibt ausdrücklich kein zusätzlicher Raum. |

Weitere Beobachtungen benötigen eine eigenständige Reproduktion: Beim Prüfen
des Desktopkontextmenüs waren zugleich Raumoverflow und Kontextmenü sichtbar;
das erwartete Settings-Untermenü wurde durch den ersten Hoverversuch nicht
sichtbar. In der Fensterliste erschien zeitweise „Vorschau nicht verfügbar“.
Diese Beobachtungen sind keine abschließend erklärte Ursache und kein PASS.

## Besichtigte Ansichten und verbleibende Abdeckung

Besichtigt wurden Desktop/Panel, leerer und belegter Hub, Suche, Fensterliste,
Räume verwalten leer und belegt, Raum konfigurieren Allgemein/Apps/Dateien/
Wiederherstellung, Leistenkonfiguration, sämtliche sichtbaren Kategorien der
alten Settings-Navigation, alle fünf First-Run-Seiten, Systemdeck,
Netzwerkunterseite, Kalender, Raumoverflow und Desktopkontextmenü.
Zusätzlich wurden der Neu-Raum-Entwurf und die
[Browser-Standard-App-Auswahl](UI_VISUAL_AUDIT_2026-10-02-evidence/59-default-app-picker.png)
geöffnet und ohne Übernahme wieder verlassen.

Die Settings-Kategorien umfassen Hintergrund, Mauszeiger, Angeheftet,
Standard-Apps, Anzeige, Netzwerk, Bluetooth, Audio, Drucker, Energie,
Benutzer, Updates und Systemübersicht. Aufnahmen 12–23 dokumentieren diese
Seiten; Standard-Apps ist separat in 05/46 dokumentiert. First Run wurde mit
„Abbrechen, Entwurf behalten“ verlassen. Die Einführungseinstellungen wurden
nicht übernommen; die Navigationsposition eines vorhandenen Entwurfs kann
durch diese Besichtigung fortgeschrieben worden sein.

Nicht neu visuell abgenommen: 1366×768, andere Skalierungen, UI auf dem
zweiten Output, Login/Lock/PAM/Polkit, sämtliche Picker-/Bestätigungsdialoge,
alle Fehler- und Langtextzustände sowie Audio-OSD und Screenshot-Regionsauswahl.
Deshalb ist dies keine Behauptung, jedes mögliche Fenster und jeden Zustand
abschließend geprüft zu haben. Diese Abdeckung gehört zur anschließenden
Qualitätsrunde und bleibt offen.

## Reparaturfolge und Abnahmeregel

1. Den bestätigten Rückweg stabilisieren: in diesem Stand installiert. Danach
   Settings-Ziele in den gemeinsamen Control-Center-Rahmen überführen und
   Seitenauswahl aus der tatsächlichen Ansicht ableiten.
2. Gemeinsame native Text-, Icon-, Fokus- und Formularrollen korrigieren;
   Darstellung der freigegebenen Materialquelle über Wallpaper und hellen
   Anwendungen vergleichen. Alle neuen Werte ausschließlich Tokens/Config.
3. Hubkopf und Raumkarten gemäß `(2)`; anschließend Verwaltungsbildkopf,
   Toolbar und Karten gemäß `(3)`; Konfiguration mit proportionalem Bildkopf,
   Formularhierarchie und realer Vorschau gemäß `(4)`.
4. Weitere Settings-Seiten, Popupanker, Picker und Einführung mit denselben
   Komponenten bereinigen. Technische Grenzen bleiben ehrlich sichtbar.
5. Jeden geprüften Stand als Release auf Fedora installieren und dort mit
   echten Eingaben besichtigen: leer/belegt, kurze/lange Texte, Fehler/Fokus,
   beide Auflösungen und verfügbare Skalierungen. Keine neue Phasenfreigabe
   allein aus Testzahlen oder einem leeren Desktop ableiten.

## Geänderte Dateien

| Datei | Änderung |
| --- | --- |
| `crates/niwoe-shell/src/wayland/state/popups.rs` | Gemeinsamer Settings-Einstieg, korrekte Layerzone beim Verlassen der Verwaltung, gemeinsamer Rückweg und Herkunftsreset. |
| `crates/niwoe-shell/src/wayland/shell.rs`, `init/shell_state.rs` | Herkunftsstatus für den Rückweg ergänzt und initialisiert. |
| `crates/niwoe-shell/src/wayland/handlers/widget_dispatch/dispatch.rs` | Settings-Widget nutzt gemeinsame Öffnen-/Schließenpfade. |
| `crates/niwoe-shell/src/wayland/handlers/keyboard.rs` | Escape nutzt denselben Schließpfad wie Zurück. |
| `crates/niwoe-shell/src/wayland/state/panel_actions.rs`, `shell_actions.rs` | Panel-Einstiege vereinheitlicht; Herkunft beim Appstart/Launcherschließen zurückgesetzt. |
| `NIWOE_IMPLEMENTATION_PLAN.md` | Aktuelle visuelle Freigabe wieder offen; historische Funktionsbelege getrennt. |
| `docs/MOCKUP_WORKFLOW_PLAN.md` | Historische visuelle Checkpoints als nicht aktuelle Freigabe gekennzeichnet. |
| `docs/phase-reports/P12.md`, `README.md` | Aktueller Status korrigiert, veralteter Phasenindex aktualisiert. |
| Dieser Bericht und Belegverzeichnis | Tatsächliche Aufnahmefolge, Abweichungen, Fixnachweis und offene Abdeckung dokumentiert. |

## Verifikation und installierter Stand

Auf Fedora nach den Rust-Änderungen:

| Befehl | Ergebnis und Rohbeleg |
| --- | --- |
| `cargo fmt --all -- --check` | PASS; zuvor lokal `cargo fmt --all`. [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/gates/fmt.log). |
| `cargo check --workspace` | PASS. [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/gates/check.log). |
| `cargo test --workspace` | PASS: 1.221 bestanden, 0 fehlgeschlagen, 1 ignorierter isolierter D-Bus-Test. [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/gates/tests.log). |
| `cargo test -p niwoe-tokens --test design_guard` | PASS. [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/gates/design-guard.log). |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS. [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/gates/clippy.log). |
| `cargo build --release -p niwoe-shell --locked` | PASS. [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/gates/release.log). |
| Manueller Call-Flow | Verwaltung → Settings: Zone `0` → `-1`, Configure vor neuer Bufferdarstellung; Zurück/Escape → Verwaltung: Zone `0`; Verwaltung → Hub: Zone `-1`. Live-Maus- und Tastaturbilder oben. |
| Dateigrößen | Alle sieben geänderten Rust-Dateien bleiben unter 600 physischen Zeilen. |
| Quellabgleich | SHA-256 aller sieben geänderten lokalen und gebauten Fedora-Dateien identisch. [Hashes](UI_VISUAL_AUDIT_2026-10-02-evidence/changed-source-hashes.txt). |

Release-Shell in `/usr/local/bin/niwoe-shell` installiert und über den vorhandenen
Watchdog erneuert: PID 1385 → 11493. Kein Neulogin erforderlich.
Release-, installierter und laufender Shellhash stimmen überein:
`10124fbace72666a152a94ce905cd1bbe596a1870d97380ac22c6ca10cec0e4e`.
Der laufende Compositor bleibt beim zuvor verifizierten Hash
`c09985e39b77bea259fb81aaae8ab92f4933bf50a0a8971756bde146457d6f65`.

Keine neuen Dependencies oder Cargo-Manifeste, keine Renderreihenfolge geändert.
Die temporäre Eingabegeräteberechtigung wurde auf die zuvor gesicherte ACL
zurückgesetzt und auf exakte Gleichheit geprüft. Der finale Raum-Snapshot
ist unverändert; kein Testfenster bleibt offen. Der direkte Appstart hatte
Raum 1 aktiviert, der aktive Kontext bleibt dort. [Abschlussprüfung](UI_VISUAL_AUDIT_2026-10-02-evidence/final-verification.json).
SSH-Zugangsdaten werden nicht in Projektdateien
gespeichert. Neue Idle-/Latenzwerte wurden nicht gemessen; frühere Zahlen sind
keine neue Performanceabnahme dieses UI-Durchlaufs.

## R1 begonnen: Quellprüfung und aktuelle Basisgates, 02.10.2026

Status: **in Arbeit**, noch keine UI-Reparatur oder neue visuelle Abnahme.
Der aktive Reparaturplan beginnt mit gemeinsamen Text-, Icon-, Fokus- und
Formularrollen und dem Panelvergleich. Die vorhandenen Navigationsänderungen
bleiben erhalten. R2–R9 und V01–V20 bleiben offen.

Aktueller Fedora-Zugang über den im vorhandenen Prüfskript benannten
Projektschlüssel bestätigt. Installierte und laufende Shell (PID 11493)
haben weiterhin denselben SHA-256:
`10124fbace72666a152a94ce905cd1bbe596a1870d97380ac22c6ca10cec0e4e`.
IPC-Snapshot: keine Fenster; Primary 1920×1080/100 %, Secondary
3840×2160/100 %, neun Räume. Kein Raum oder Einstellungsentwurf verändert.
Die drei untersuchten Quelldateien `niwoe-ui/src/effect/text.rs`,
`niwoe-shell/src/settings_view/audio_system_widgets.rs` und
`niwoe-shell/src/hub_view/search.rs` stimmen lokal und auf Fedora per
SHA-256 überein.

Quellbefunde, mit den bereits vorhandenen Bildern abgeglichen:

- V15: `DefaultAppCategoryRow` in `settings_view/audio_system_widgets.rs`
  setzt Haupt-/Unterzeilen auf Baselines 24/33 bei Schriftgrößen 13,5/10,5.
  Die neun Pixel Baselineabstand erklären die kollidierenden Textrollen.
  Der Expander verwendet `▴`/`▾` als Schriftzeichen.
- V15: `network_popup.rs` hängt ein Schloss-Emoji an den WLAN-Namen.
  Die fehlende Glyphendarstellung benötigt ein natives Symbol aus der
  gemeinsamen UI-Quelle. Die Settings-Netzwerkzeile verwendet bereits
  verständlichen Sicherheitstext; beide Ansichten müssen getrennt geprüft werden.
- V19: `hub_view/search.rs` zeichnet Textzeilen mit lokal zusammengesetzten
  Baselines und ohne Appicon. Die Fensterliste und das Systemdeck zeichnen
  Fokusrahmen separat; der gemeinsame Fokus-Innenabstand wird dort nicht verwendet.
  Der cyanfarbene Livebefund bleibt vor einer Ursache-/Palettekorrektur erneut
  zu reproduzieren; der Quellpfad verwendet bereits die Config-Akzentfarbe.

| Prüfung auf unverändertem Fedora-Ausgangsstand | Ergebnis |
| --- | --- |
| `cargo check --workspace` | Exit 0; [Basislog](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-baseline-check.log). |
| `cargo test --workspace` | Exit 0; 1.221 bestanden, 0 fehlgeschlagen, 1 ignoriert (isolierter D-Bus-Test); [Basislog](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-baseline-tests.log). Design-, Source-Size- und Centralization-Guard in diesem Lauf enthalten. |
| `git diff --check` lokal | Exit 0. |
| Neue Live-Reproduktion, Screenshot und Eingaben | `NOT RUN`: `/dev/uinput` ist für `eduard` nicht beschreibbar; `sudo -n` meldet notwendiges Passwort. |
| Rust-Reparatur, Releasebau und neue Installation | `NOT RUN`: vor einer Änderung verlangt der Plan aktuelle Live-Reproduktion und Screenshot. |

Für die Live-Prüfung wurde die temporäre Eingabegerätefreigabe
`sudo setfacl -m u:eduard:rw /dev/uinput` angefragt. Sie ist bislang nicht
erteilt oder verändert. Die vorhandene ACL enthält ausschließlich
`user::rw-`, `group::---`, `other::---`; nach einer tatsächlichen Freigabe
ist genau dieser Ausgangszustand wiederherzustellen. Die Releaseinstallation
nach bestandenen Gates benötigt ebenfalls einen verfügbaren sudo-Zugang.
Keine Zugangsdaten, zusätzlichen Berechtigungen, Dependencies oder
Cargo-Manifeständerungen angelegt. Aktuelle Bilder, Performancebelege und
Panelprüfung stehen aus; die Basisgates sind kein Reparaturabschluss.

## R1: gemeinsame Rollen umgesetzt, erneute Live-Prüfung läuft

Der vorstehende Zugangsblock ist historisch. Der Nutzer hat am 02.10.2026
die begrenzte Wiederverwendung des bisherigen sudo-Zugangs ausdrücklich
freigegeben. Die ursprüngliche `/dev/uinput`-ACL wurde gesichert und die
temporäre Freigabe eingerichtet; Rücksetzung gehört weiterhin zum Cleanup.
Zugangsdaten werden weder ausgegeben noch in Projektdateien gespeichert.
Die Befunde wurden vor der Reparatur auf der unveränderten installierten
Shell erneut aufgenommen. Screenshots und gescheiterte Versuche bleiben
getrennt von neuen Abnahmebelegen erhalten.

| Quelle | Änderung und Ursache |
| --- | --- |
| `niwoe-tokens/src/spacing.rs` | Gemeinsame Textzeilen- und Symbolgeometrie sowie begrenzter Symbolcache. |
| `niwoe-ui/src/effect/text_roles.rs`, `effect/mod.rs`, `tests/text_roles.rs` | Getrennte Haupt-/Unterzeilen anhand tatsächlicher Fontmetriken; pixelbreitenabhängiges Kürzen; gemeinsamer Fokus mit Token-Innenabstand. Ein erster Test deckte ein Rundungsproblem auf; Ascent und Descent werden jetzt einzeln nach außen gerundet. |
| `niwoe-ui/src/effect/symbol.rs` | Native Chevron-, Schloss-, Auswahl-, App-, Fenster- und Raumsymbole statt fehlender Schriftglyphen; Tests für Rasterung, Cacheidentität und Begrenzung. |
| `niwoe-shell/src/settings_view/audio_system_widgets.rs` | Standard-App-Zeilen nutzen gemeinsame Rollen und native Chevrons. Keine kollidierenden Baselines oder lokale Radiuskorrektur. |
| `niwoe-shell/src/hub_view/search.rs`, `interaction_tests.rs`, `wayland/render/launcher.rs` | Suchzeilen zeigen vorhandene gecachte Appicons, getrennte Texte und gemeinsamen Fokus; native Ersatzsymbole bei fehlendem Icon. Ein Regressionstest prüft den tatsächlichen Cachepfad. |
| `niwoe-shell/src/ui/tokens.rs`, `window_picker.rs`, `draw/painter.rs` | Ursache des cyanfarbenen Fokus: RGBA-Farben wurden direkt in einen BGRA-Puffer gezeichnet. Zentraler Formatadapter erhält die Config-Palette; Test prüft die tatsächlichen goldenen Pufferbytes. Kein zusätzlicher Ganzbild-Konvertierungspass. |
| `niwoe-shell/src/network_popup.rs` | Sicherheits- und Auswahlsymbole erhalten eigene Bildflächen; lange SSIDs werden innerhalb ihrer Textspalte gekürzt. |

Performance-Modell: Text und Symbole werden nur bei bestehenden
eventgetriebenen Neuzeichnungen verwendet. Symbolschlüssel sind Identität,
Config-Farbe und Rastergröße; maximal 32 Einträge à 96×96×4 Bytes
(1,125 MiB Rasterdaten je Renderthread), FIFO-Verdrängung, threadgebundene
Lebensdauer. Ein Treffer gibt einen `Arc` zurück und kopiert keine Pixel.
Keine neue Capture-, Timer- oder Frameschleife; Renderreihenfolge unverändert.
Vor der Installation läuft eine neue Basisserie aus drei 300-Sekunden-Proben
mit 30 Sekunden Warm-up und unverändertem leeren Desktop.

Status bleibt **in Arbeit**, bis Installation, Hashabgleich und aktuelle
Bildprüfung belegt sind. R2–R9 und die vollständige V15-/V19-Matrix bleiben
offen; bestandene Quellgates allein ersetzen diese Prüfungen nicht.

### R1 installiert und erste Bildprüfung bestanden

Release am 02.10.2026 installiert; Watchdogwechsel PID 11493 → 41731,
kein Neulogin nötig. Release-, Installations- und laufender Hash:
`95be48c09da342672c359e6e9f610ed95366f7b013c140da2b2b5a9bd87b62ad`.
Alle 13 betroffenen Quelldateien stimmen lokal und auf Fedora überein:
[Quellhashes](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/source-hashes.txt).

| Neues Gate auf R1 | Ergebnis |
| --- | --- |
| `cargo fmt --all -- --check` | Exit 0; [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/fmt.log). Include-Fragmente zuvor zusätzlich mit `rustfmt --edition 2021` formatiert. |
| `cargo check --workspace` | Exit 0; [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/check.log). |
| `cargo test --workspace` | Exit 0; 1.227 bestanden, 0 fehlgeschlagen, 1 bekannter isolierter D-Bus-Test ignoriert. Design-, Source-Size- und Centralization-Guard enthalten; [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/tests.log). |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0; [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/clippy.log). |
| `cargo build --release -p niwoe-shell --locked` | Exit 0; [Log](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/release.log). |
| Erster Textmetriken-Testlauf | Rundungsfehler entdeckt, korrigiert, anschließend vollständige Gates grün. [Erhaltener Fehlversuch](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/tests-first-attempt.log). |

Bildprüfung auf drm-0, physisch 1920×1080, 100 %, grünes Theme:

| Vergleich | Gesehener Befund |
| --- | --- |
| [Panel vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-before-panel.png) / [Panel danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-after-watchdog-retry.png) | 48-Pixel-Leiste, exakt mittige Uhr, Raumfolge und freigegebenes Material erhalten. Dieser Panelvergleich erfüllt die Voraussetzung für R2; zusätzliche Matrixfälle stehen weiter aus. |
| [Suche vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-before-search.png) / [Suche danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-after-search.png) | Reales Firefox-Cacheicon sichtbar, Haupt-/Unterzeilen getrennt, goldener Fokus mit Innenabstand. |
| [Standard-Apps vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-before-apps-confirmed.png) / [danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-after-apps.png) | Alle neun Rollen lesbar, Haupt-/Unterzeilen ohne Berührung, native Chevrons. |
| [Dropdown danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-after-default-picker.png) | Aufklappen per Maus, natives Chevron nach oben, aktuelle Auswahl und goldener Fokus sichtbar; keine Zuordnung geändert. |
| [Fensterliste vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-before-picker.png) / [danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-after-window-picker.png) | Gold statt Cyan, Texte sauber getrennt; F6, Pfeile und Escape verwendet. Eigenes KWrite-Testfenster hinter dem Hub. |
| [Deck danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/r1-after-deck.png) | Freigegebenes Deckmaterial unverändert. Netzwerkaufnahme unmittelbar nach dem Öffnen zeigt noch leeren Scancache; sie ist kein Beleg für die neuen Schlossicons. V15 bleibt bis zum Daten-/Scalevergleich offen. |

Der erste Screenshot unmittelbar nach dem Watchdogwechsel lief in einen
Timeout. Nach Escape und erneutem regulärem Consent gelang die Aufnahme;
der Fehlversuch ist keine bestandene Lebenszyklusprüfung. Die vollständige
Once-per-Login-/Watchdogmatrix bleibt R9 zugeordnet.

Neue Basisperformance: [drei gültige 300-s-Proben](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/before-idle.json)
nach 30 Sekunden Warm-up, gleicher leerer Desktop und beide ursprünglichen
DRM-Modi. Shell 0,18–0,19 % eines Kerns, Compositor 1,06–1,07 %, gemessene
DRM-Engineauslastung rund 0,01 %, Shell-RSS konstant 98.623.488 Bytes.
[40 Öffnungszyklen](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/before-cycles.json)
mit Hub, Deck, Kalender und Raumoverflow sind ebenfalls gültig und erhalten
den IPC-Snapshot. Diese Daten gehören zum unveränderten Ausgangsrelease;
die Nachher-Serie auf dem reparierten Gesamtstand steht noch aus.

R1 ist **geprüft/installiert**, seine genannten 100-%-Bildfälle sind geprüft.
V15/V19, die vollständige Formular-/Scale-Matrix und Gesamtfreigabe bleiben
offen. R2 ist begonnen; übrige Pakete sind noch nicht abgeschlossen.

### R2: Bildkopf, Kartenkomposition und große Überschrift installiert

Status **geprüft/installiert**, bisherige Bildfälle auf drm-0 bei 1920×1080,
100 % angesehen; vollständige Daten-/Scale-/Outputmatrix noch offen.
Der letzte Hubstand hat den laufenden Hash
`ff9b9356b1a09b51657039a8f8857db0964b165495e7879681035795cfb2548c`
(Watchdog PID 64352 → 74788). Keine Änderung am Compositor oder Neulogin.

Geänderte Dateien und Verantwortung:

- `assets/ui/niwoe-context-landscape-v1.png` und `README.md`: eigenes statisches
  Landschaftsasset; Entstehung, Prompt und SHA dokumentiert. Kein Fremd-Appinhalt.
- Tokens `artwork.rs`, `hub.rs`, `typography.rs`, `font.rs`, `lib.rs`:
  Bildcachegrenzen, proportionale Bildfläche, größere Karten-/Previewgeometrie
  und zentraler Serif-Displaywert 42. Die Sansleiter 12/14/18/28 bleibt erhalten.
- UI `effect/image_cover.rs`, `heading.rs`, `mod.rs`: proportionaler Cover-
  Ausschnitt mit Fokuspunkt; große Serifrolle mit begrenztem Glyphencache und
  derselben Textfarbmischung wie die übrige native UI.
- Shell `landscape.rs`, `main.rs`, `hub_view.rs`, `hub_view/rooms.rs`,
  `previews.rs`, `interaction_tests.rs`: gecachter eigener Kopf, native
  Raumsymbole, reale proportionale Fensterbilder, getrennte Kartenabschnitte;
  Aufgabenbereich mit ehrlichem Zustand der fehlenden Integration.

Noto Serif Regular 2.015 ist zentral eingebettet; die Datei selbst nennt
Copyright 2022 Noto Project Authors und SIL OFL 1.1. Die unveränderte Schrift
hat SHA `19e72cd8d595fae5bd74a5206f5d938512e1183d4fed7abb1ec1be1d7efa5f88`;
offizieller Lizenztext liegt neben dem Font als `NotoSerif-OFL.txt`.
[Lizenzquelle](https://github.com/notofonts/latin-greek-cyrillic/blob/main/OFL.txt).

Cachemodell: Quell-PNG einmal pro Prozess dekodiert (rund 6 MiB); angepasste
Raster nach Assetidentität und Zielgröße, FIFO maximal vier Einträge und
8 MiB Rasterdaten. Serif-Font einmal geparst; Glyphen nach Zeichen bei fester
zentraler Rollengröße, maximal 128 Einträge/512 KiB, Arc-Treffer ohne Pixelkopie.
Farbe wird beim Malen aus dem Theme genommen. Keine Capture-/Frame-/Timerserie
im Produkt ergänzt; bestehendes Compositorglas und Stapelreihenfolge erhalten.

[Vorher über KWrite](UI_VISUAL_AUDIT_2026-10-02-evidence/r2/r2-before-hub-light-app.png),
[Karten danach über KWrite](UI_VISUAL_AUDIT_2026-10-02-evidence/r2/r2-after-cards-light-app.png),
[danach über Wallpaper](UI_VISUAL_AUDIT_2026-10-02-evidence/r2/r2-after-hub-wallpaper.png),
[große Serifüberschrift danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r2-heading/r2-after-serif-heading.png).
Die Alpha-224-Zwischenaufnahme ist erhalten; nach sichtbarer Konkurrenz durch
fremde Apptexte wurde der zentrale Kartenwert auf 255 korrigiert und erneut
vollständig geprüft/installiert. Das Glas zwischen den Inhaltsflächen bleibt.

Alle fünf Gates (Fmt, Check, Workspace-Tests einschließlich aller Guards,
Clippy mit `-D warnings`, Releasebuild) jeweils Exit 0 auf dem letzten Stand.
R2 zunächst 1.230, mit Serifrolle und Popupankern 1.233 bestandene Tests, 0 fehlgeschlagen,
1 bekannter isolierter D-Bus-Test ignoriert. Rohlogs:
[R2](UI_VISUAL_AUDIT_2026-10-02-evidence/r2/tests.log),
[Serifrolle](UI_VISUAL_AUDIT_2026-10-02-evidence/r2-heading/tests.log).
Erhaltene erste Check-/Clippy-Fehlversuche sind keine bestandenen Gates.
V02/V19 bleiben bis zur vollständigen Matrix offen.

### R3: Anker und bestätigte Popup-/Flyoutbefunde

Status **in Arbeit**, mehrere geprüfte Zwischenreleases installiert.
Kalender mittig unter der Uhr (Layer-Anchor TOP), deutsche Wochentage;
Raumoverflow berechnet die Position aus seinem tatsächlichen Panel-Hitbereich,
begrenzt die sichtbare Karte auf die logische Outputbreite. Gemeinsame
Text-/Fokusrollen für Raumtiles. WLAN zeigt bei leerem Cache während des
vorhandenen asynchronen Scans einen Ladezustand. Kein zusätzlicher Worker
oder Pollingtimer. Quellen: `wayland/popup_placement.rs`, `mod.rs`, `init.rs`,
`handlers/layer.rs`, `state/popups.rs`, `calendar.rs`, `workspaces.rs`,
`draw/painter.rs`, `network_popup.rs`, `render/network_audio.rs`.

Ankerrelease `78c29099ebe2769012d0e2ee848fb0f5bc8efd1ee0d0c8b32fb6721250972299`
lief als PID 64352; Release-, Installation- und Prozesshash identisch.
Fmt/Check/Tests/Clippy/Release jeweils Exit 0. Clippy-Importfehlversuche erhalten.
Aktuelle 100-%-Bilder:
[Kalender](UI_VISUAL_AUDIT_2026-10-02-evidence/r3/r3-after-calendar.png),
[Overflow](UI_VISUAL_AUDIT_2026-10-02-evidence/r3/r3-after-overflow.png).

**V21 bestätigt:** Overflow öffnen → Desktoprechtsklick ließ beide Popups
sichtbar. Gegenseitiges Schließen in `state/popups.rs`/`state/timers.rs`
implementiert. [Vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r3/r3-before-overflow-context.png),
[danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r3/r3-after-overflow-context.png):
in dieser Richtung nur Desktopmenü sichtbar. Gegenrichtung und Scales folgen.

**V22 bestätigt:** Hover zeigt den Settings-Flyout; Elternklick öffnete die
Mauszeigerseite. Die Pointerauswahl unterscheidet nun Eltern-/Untermenüaktionen
und berücksichtigt die sichtbare Untermenüfläche; Enter und Motion behandeln
Hover gleich. Menümaße zentral in Tokens, Beschriftungen „Hub“ und „Audio“.
Quellen `context_menu.rs`, `context_menu/selection.rs`,
`handlers/pointer/overlays_and_desktop.rs`, Tokens `spacing.rs`.
Release `9409c4f8c3bc555cf8759d317c08cac3f3f895a4b6802f9ff8d017ea0aaba645`
installiert (PID 81421), alle Gates Exit 0 und 1.235 Tests bestanden.
Dieser Stand ist **noch kein visueller Abschluss**: Der erneute Klick schließt
das Menü weiterhin. Die Call-Flow-Prüfung zeigt `WindowFocused` auch für die
Menülayer; die Shell schließt bisher bei jedem solchen Fokusereignis das Menü.
Der weitere Fix begrenzt diese Regel auf tatsächliche Appfenster.
Ein Regressionstest prüft native/XWayland-Fenster gegenüber einem Layer-ID.
Die Launcher-Worker werden dabei verhaltensgleich aus `state/ipc_events.rs`
nach `state/launcher_workers.rs` ausgelagert, um das 600-Zeilen-Limit einzuhalten.
Nachprüfung und Installation dieses Folgefixes stehen in diesem Bericht noch aus.

**Folgeprüfung durchgeführt:** Release
`0f974ef8fac12a73f800af5a54e3ca0d92bfeaafea8af730d127ca752af581a6`
ist installiert; Release-, Installations- und Prozesshash stimmen überein.
Fmt, `cargo check --workspace`, `cargo test --workspace`, Clippy mit
`-D warnings` und Releasebuild jeweils Exit 0: 1.236 Tests bestanden,
0 fehlgeschlagen, 1 bekannter isolierter D-Bus-Test ignoriert.
Der erste Source-Size-Guard-Fehler (609 Zeilen) bleibt im Fehlversuchslog;
die ausgelagerten Worker bringen die Quelldatei unter 600 Zeilen.
47 geänderte Rust-Quellen wurden zwischen lokalem Stand und Fedora-Build
mit normalisierten Zeilenenden SHA-256-geprüft: keine Abweichung.
[Quellvergleich](UI_VISUAL_AUDIT_2026-10-02-evidence/r3-menu-focus/r13-source-verified.json).

Das installierte Desktopmenü wurde mit dauerhaft vorhandenem Pointergerät
nachgeprüft: Hover, Elternklick und Überqueren zum Untermenü halten den
Flyout sichtbar; die Tastatur zeigt dieselben Ziele.
[Elternklick](UI_VISUAL_AUDIT_2026-10-02-evidence/r3-menu-focus/r3-after-menu-focus-fix-persistent-click.png),
[Untermenü](UI_VISUAL_AUDIT_2026-10-02-evidence/r3-menu-focus/r3-after-menu-focus-fix-persistent-traverse.png),
[Tastatur](UI_VISUAL_AUDIT_2026-10-02-evidence/r3-menu-focus/r3-after-keyboard-flyout.png).
Auch die Gegenrichtung von V21 ist geprüft: Desktopmenü → Raumoverflow
zeigt ausschließlich den Overflow.
[Gegenrichtung](UI_VISUAL_AUDIT_2026-10-02-evidence/r3-menu-focus/r3-after-context-overflow.png).
Der vorhandene WLAN-Scan lieferte vier sichtbare Netze mit getrennten nativen
Schloss-/Verbindungssymbolen. Die Aufnahme mit SSIDs bleibt im privaten
Testverzeichnis und wird nicht als öffentliches Projektartefakt abgelegt.

### R1–R3: installierte Skalierungs- und Outputrunde

Auf demselben Folgefix wurden 48 Aufnahmen erstellt und einzeln angesehen:
Panel, Hub mit Fokus, Suche mit Fokus, Kalender, Raumoverflow und Deck;
jeweils auf `drm-0` und `drm-1` bei 100/150/200 % und zusätzlicher Skalierung
1920/1366 (logisch 1366×768). Die physischen Testmodi waren jeweils
1920×1080; ein physischer 1366×768-DRM-Modus wird damit nicht behauptet.
Beim Wechsel des primären Testoutputs wurden die unveränderlich gebundenen
Shell-Layer über den bestehenden Watchdog erneuert. Raumdaten wurden nicht
geändert und keine App gestartet.
[Fälle/Buildidentität](UI_VISUAL_AUDIT_2026-10-02-evidence/r13-matrix/cases.json).

Alle sechs Flächen bleiben in den geprüften Fällen sichtbar und bedienbar;
Kalender liegt unter der mittigen Uhr, Overflow unter seinem echten Auslöser.
Hub zeigt vier Karten und drei untere Bereiche auch bei logischer 960×540-Fläche.
Suchicon, Haupt-/Unterzeile und Goldfokus sind erkennbar.
Die Vergrößerung des vorhandenen logischen Shell-Rasters erzeugt jedoch
sichtbar weichere Text-/Iconkanten bei höherer Skalierung (**V23**, offen,
gemeinsamer Rasterpfad in R9); diese Runde ist keine vollständige
HiDPI-Qualitätsfreigabe. Belegte Räume, Vorschau-Lifecycle, lange Kataloge,
WLAN-/Standard-App-Zeilen bei allen Scales und Performanceabschluss bleiben
gesonderte Pflichtfälle.

Der erste Cleanup-Assert schlug fehl: Entfernen temporärer Outputabschnitte
setzt den aktuellen DRM-Modus nicht zurück. Die exakten Vorher-Modi wurden
daraufhin explizit geladen, anschließend die ursprünglichen Konfigurationsbytes
wiederhergestellt. Der vollständige Output-Snapshot einschließlich 4K/30 Hz
auf `drm-1`, Scale, Primärmonitor und Fokus sowie die Raumdaten stimmen
wieder mit dem Vorher-Snapshot überein. Rohfehler bleiben erhalten;
der spätere erfolgreiche Cleanup ist separat dokumentiert.

Die erste Flyoutprüfung mit einem pro Einzelbewegung neu angelegten Testgerät
war kein verlässlicher Hovernachweis. Mit einem durchgehend vorhandenen Gerät
ist das Untermenü sichtbar; ursprüngliche Aufnahmen bleiben erhalten.
R4 wurde noch nicht implementiert. Die Nutzerpräzisierung zu vollständigen
Settings im Control Center und zur sorgfältigen Reihenfolge ist im Plan ergänzt.

### R1–R3: aktuelle Performance-Nachprüfung

Der installierte Stand `0f974ef8` wurde nach 30 Sekunden Warm-up auf derselben
Hardware und den wiederhergestellten Modi mit drei Proben zu jeweils 300 Sekunden
gemessen. Alle drei liefen ohne Eingabe, Build oder Snapshotänderung durch.
Shell-CPU: 0,197/0,190/0,187 %, Compositor: 1,087/1,077/1,077 % eines Kerns;
die Medianabweichung zur Vorher-Serie liegt deutlich unter dem Budget von
0,5 Prozentpunkten. Der verfügbare GPU-Engine-Zähler lag bei 0,0086–0,0087 %.
Compositor-RSS blieb bei 127.270.912 Bytes, Shell-RSS bei 31.232.000 Bytes
mit einmaligem Anstieg auf 31.363.072 Bytes in der dritten Probe. Die nach
Watchdog-Neustart zunächst kalte Shell ist kein Beleg für eine gegenüber der
bereits aufgewärmten Vorher-Shell gesunkene Cachegröße.
[Nachher-Rohdaten](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/after-r13-idle.json),
[Messlog](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/after-r13-idle.log).
Repaint- und Eingabe-bis-sichtbar-Latenz sind durch dieses Werkzeug nicht gemessen.

Die erste Öffnungsrunde brach nach dem ersten 20er-Block am vollständigen
Snapshotvergleich ab. Die Nachprüfung zeigte ausschließlich eine neue Laufzeit-ID
für `drm-1` (2 → 3); Modi, Geometrie, Scale, Fokus, Raumdaten und Fensterbestand
blieben gleich. Die Ursache des Monitor-Reconnects ist damit nicht bewiesen.
Der Versuch bleibt fehlgeschlagen und wird nicht als bestandene Cacheprüfung
gezählt. Eine neue Runde erhält vor und nach jedem Block beide vollständigen
Snapshots, damit eine weitere Abweichung samt Messwerten erhalten bleibt.

Die Wiederholung mit dem aktuellen Monitorsnapshot bestand beide 20er-Blöcke
(40 Öffnungszyklen insgesamt), ohne Snapshotänderung oder Prozessneustart.
Shell-RSS: 95.633.408 → 97.607.680 → 97.570.816 Bytes; Compositor-RSS blieb
bei 163.151.872 Bytes. Nach dem ersten Block ist kein stetiges Wachstum erkennbar.
Die vorangegangene abgebrochene Runde hatte die Assets bereits aufgewärmt;
der neue Nullpunkt ist deshalb ausdrücklich kein kalter Prozesszustand.
[Vollständige Zyklen/Snapshots](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/after-r13-cycles-recheck.json),
[Prüflog](UI_VISUAL_AUDIT_2026-10-02-evidence/r1/after-r13-cycles-recheck.log).

### R4: gemeinsame Control-Center-Navigation in Arbeit

Vor dem Quellumbau wurden sechs installierte Ansichten reproduziert und angesehen.
Release, installierte und laufende Shell hatten identisch den vollständigen SHA-256
`0f974ef8fac12a73f800af5a54e3ca0d92bfeaafea8af730d127ca752af581a6`, PID 89505.
Apps, Benutzer, System und Einstellungen öffneten die alte zentrierte Karte;
die Leistenkonfiguration zeigte fälschlich die Aktivmarkierung „Räume“.
[Apps vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-before-apps.png),
[Einstellungen vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-before-settings.png),
[Leiste vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-before-panel.png).

Der Umbau verwendet eine gemeinsame Sidebar als natives Widget mit zentraler
Geometrie, nativen Symbolen und Aktion-IDs. Die Raumseiten rendern dieselbe
Komponente; Settings verwenden sie direkt in ihrem vorhandenen Widgetbaum.
Vorhandene Provider und Inhalte bleiben bestehen. Ihre zwölf aktiven Kategorien
liegen nun in der jeweiligen Inhaltsnavigation; Theme-Auswahl und überholtes
Panel-Pinning sind dort nicht erreichbar. Die zugrunde liegenden Daten bleiben.
Files ist nur in einer Raumkonfiguration verfügbar; fehlende Backups/Protokolle
bleiben deaktiviert. „Übersicht“ führt wie bisher zum Hub.

Zeichnen und Maus verwenden denselben Content-/Fit-Maßstab. Tastaturziele stammen
aus den tatsächlichen sichtbaren Widget-IDs; `F8` erreicht die gemeinsame Sidebar,
`F7` die Leiste. Es entstehen keine Timer, Captures oder dauernden Frame-Schleifen.
Symbolspeicher bleibt im bestehenden, nach Artwork/Farbe/Größe begrenzten Cache;
Widgetbaum und Layout leben jeweils nur bis zum Ende der vorhandenen Invalidierung.
Besuch einer Settings-Seite bewahrt Raum-/Leistenentwurf und Rücksprung.
Hub-Rückwege wechseln weiterhin ausdrücklich auf seinen ursprünglichen Layerpfad.

Die erste Linux-Checkrunde fand fehlende Taffy-Rechteckdefaults und einen
Symbolgrößen-Typmix; der zweite Check bestand. Der erste Clippy-Lauf forderte
explizite `f32`-Literale im neuen Geometrietest. Diese Diagnosen wurden korrigiert;
ihre Rohlogs bleiben separat erhalten. Vollständige Gates, Releaseinstallation
und Nachher-Bildprüfung stehen für diesen Quellstand noch aus.

### R4: erste Release und anschließende Escape-Korrektur

Der erste gemeinsame Rahmen wurde mit identischem Release-/Installations-/
Laufzeithash `7cec92e7791d41b25fcbfde11738686249830c5c7c0a1b7192efb8b340884e98`
über den Watchdog aktiviert. Alle fünf Gates bestanden: Formatcheck, Workspace-
Check, Clippy mit `-D warnings`, Workspace-Tests und Shell-Releasebuild.
1241 Tests bestanden, keiner schlug fehl; ein bereits bekanntes isoliertes
D-Bus-Integrationstestziel blieb ignoriert. Der Workspace-Lauf enthält Design-,
Quellgrößen- und Zentralitätsguard. 66 geänderte/neue Rust-Quelldateien wurden
LF-normalisiert zwischen Windows und Fedora abgeglichen, ohne Abweichung.
Die sieben ersten Nachher-Aufnahmen wurden tatsächlich angesehen: Watchdog ohne
erneuten Willkommens-Hub, Räume, Apps, Benutzer, System, Einstellungen und Leiste.
Die gemeinsamen Icons und Aktivmarkierungen folgen der jeweils geöffneten Seite.
[Apps nachher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-after-apps.png),
[Einstellungen nachher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-after-settings.png).

Die ersten Navigationshelfer waren keine bestandenen Prüfungen: fünf Tab-Schritte
im Hub wählten Raum 5 statt „Räume verwalten“, weil der tatsächliche Navigator
alle neun Räume enthält. Raum 1 wurde wiederhergestellt; Desktopaufnahmen dieser
Versuche bleiben Fehlversuche. Außerdem akzeptiert der virtuelle Textgenerator
nur Kleinbuchstaben/Ziffern/Bindestrich. Ein späterer Einzelhelfer hatte F8 nur
in der Aktionsliste, nicht in den erlaubten Gerätecoden ergänzt; seine beiden
mit `checked` beschrifteten Fokusaufnahmen belegen keinen F8-Übergang.
Der korrigierte Helfer schaltet Code 66 vor dem Anlegen des Geräts frei.
[Tatsächlicher Browserzeilenfokus](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-keyboard-browser-focus-valid.png).

Mit diesem Helfer wurde ein echter Fehler reproduziert: Escape bei offener
Standard-App-Auswahl schloss die gesamte Settings-Seite und kehrte zur Leiste
zurück. Beim erneuten Apps-Besuch blieb außerdem die alte Auswahl geöffnet.
[Offene Auswahl vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-picker-open-valid.png),
[Falscher Escape-Rückweg vorher](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-picker-escape-valid.png).
Der neue Escape-Vorrang schließt zuerst Appauswahl bzw. Modusdropdown;
der folgende Escape verwendet weiterhin Suche und echten Seitenursprung.
Beim regulären Seitenwechsel werden diese flüchtigen Auswahlen zurückgesetzt.
Kein Zuordnungs-, Speicher- oder privilegierter Providerpfad wurde geändert.
Ein Regressionstest prüft die Dismiss-Reihenfolge einschließlich gleichzeitig
gesetzter Zustände. Keine neuen Timer, Bilder oder Caches.

Dieser Fix bestand erneut alle fünf Gates mit 1242 bestandenen Tests, 0 Fehlern
und demselben einen ignorierten Integrationstest. Release, installierte und
laufende Shell haben identisch SHA-256
`76425b4af7bbf65dcc7dd99f91f8086909c5631ca8ec56274300f8d58237775b`;
Watchdogwechsel PID 103461 → 111334, kein Neulogin erforderlich.
Der Compositor wurde nicht verändert. Die finale Live-Nachprüfung sowie
R4-Abdeckung beider Outputs/Skalierungen und weiterer Einstiege laufen noch;
R4 und die Gesamtfreigabe sind damit noch nicht abgeschlossen.

Die separaten finalen Aufnahmen bestätigen den Escape-Fix am laufenden Stand:
[offen](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-final-picker-open.png),
[nach Escape weiterhin Apps](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-final-picker-escape-apps.png).
Die mit `r4-valid-…` beschriftete erste Gesamtrunde ist trotz erfolgreichem
Cleanup kein bestandener Navigationstest: der aufgenommene Fokus lag auf
„Übersicht“, danach folgten Hub-/Desktopbilder statt Picker/Entwurf.
Auch ihr erster Skriptstart brach am falschen Raum-Snapshot-Schlüssel ab.
Die Snapshot-Erhaltung behauptet ausschließlich unveränderte gespeicherte Daten.
Diese Bilder und Rohlogs bleiben erhalten.

Die anschließend mit getrenntem Apps-Übergang und 0,8 Sekunden Abstand zwischen
Eingaben durchgeführte Runde enthält elf tatsächlich angesehene Bilder mit Suffix
`paced`. Der Fokus liegt auf der Browserzeile; Enter öffnet die Auswahl, erster
Escape kehrt zu Apps zurück, zweiter zur Raumverwaltung. Raumentwurf `r4draft`
und deaktivierter Tray im Leistenentwurf bleiben nach Apps-Besuch und Rückkehr
sichtbar erhalten. Anschließender Abbruch speichert beides nicht.
Konfigurationsbytes, vollständiger gespeicherter Raum-Snapshot und vollständiger
Output-Snapshot sind vor/nach identisch; Fensterbestand blieb leer.
[Raumentwurf zurück](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-valid-room-draft-return-paced.png),
[Leistenentwurf zurück](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-valid-panel-draft-return-paced.png),
[Rohlog](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/r4-navigation-paced.log).
Die bewusste Prüftaktung ist kein Latenznachweis; schnelle Eingabe über
Layer-/Fokuswechsel ist damit noch nicht geprüft.

#### R4-Dateien und geprüfter Call-Flow

| Dateien | Änderung und Zweck |
| --- | --- |
| `niwoe-shell/src/control_center.rs`, `room_management_view/sidebar.rs` | Gemeinsame Sidebar mit einer Geometrie, IDs und nativen Symbolen für alle Control-Center-Seiten. |
| `settings_view.rs`, `settings_view/navigation.rs`, `settings_view/content_builders.rs`, `settings_view/basic_widgets.rs`, `settings_view/draw.rs`, `settings_view/content/system_overview.rs` | Bestehende zwölf aktive Settings-Provider in den gemeinsamen Rahmen und die passende Inhaltsnavigation übernehmen; echte sichtbare Tastaturfokusrolle. |
| `widget_action.rs`, `widget_traversal.rs`, `wayland/handlers/widget_dispatch/dispatch.rs` | Gemeinsame Seitenaktionen sowie Keyboardziele aus demselben Layout und denselben IDs wie die Maus. |
| `wayland/state/control_center.rs`, `wayland/state/popups.rs`, `wayland/state/panel_form.rs` | Gemeinsame Seitenwechsel, echter Ursprung und erhaltene Raum-/Leistenentwürfe; flüchtige Auswahllisten beim Eintritt zurücksetzen. |
| `wayland/handlers/keyboard/control_center_navigation.rs`, `wayland/handlers/keyboard.rs`, `wayland/handlers/pointer/launcher.rs` | F7/F8, Sidebar-/Widgetnavigation, sichtbare aktivierbare Ziele, Escape-Vorrang für Auswahlen und identische Pointerkoordinaten. |
| `wayland/render/launcher.rs`, `wayland/shell.rs`, `wayland/init/shell_state.rs`, `wayland/state.rs` | Gemeinsamer Vollflächen-/Fit-/Zustandspfad, Sidebarfokus an bestehender Renderposition, neue Navigation initialisieren und Modul anbinden. |
| `niwoe-tokens/src/control_center.rs`, `niwoe-ui/src/lib.rs`, `niwoe-ui/src/effect/symbol.rs` | Zentrale gemeinsame Mindestcanvasgröße, additive native Layoutposition-API und gecachte funktionale Sidebaricons. |

Quellprüfung: Sidebar-Maus-ID oder F8/Enter → dieselbe `navigate_control_center_page`
→ vorhandener Settings-Provider bzw. Raum-/Leistenentwurf → vorhandene bestätigte
Snapshots und Invalidierung → gemeinsamer Render-/Fitpfad. Desktop-Flyout und
Deck rufen `open_settings_category` auf; Tray und System-IPC rufen dieselbe
`prepare_settings_category` auf. Alle regulären Einstiege verwenden dadurch die
volle Control-Center-Geometrie. Dieser Call-Flow ersetzt die noch laufende
Liveprüfung der zusätzlichen Einstiege nicht. Speicher-/Provider-IPC und
Compositor-Renderreihenfolge bleiben unverändert; native Shell bleibt unprivilegiert.

### R4: Monitorrunde und Standard-App-Layoutkorrektur

Die 48 Aufnahmen `r4-frame-drm-{0,1}-s{1000,1406,1500,2000}-…` wurden
tatsächlich angesehen: Räume, Apps, Benutzer, System, Einstellungen und Leiste
auf beiden Outputs. Installierter Stand war `76425b4…37775b`. Beide Outputs
verwendeten für diese Runde physisch 1920×1080 bei 60 Hz; die logischen Größen
waren 1920×1080, 1366×768, 1280×720 und 960×540. 1406 bezeichnet gerundet
die Skalierung `1920/1366`, keinen physischen 1366-Modus. Der gemeinsame
Inhaltscanvas bleibt mindestens 1366×720; die Fit-Abbildung verkleinert ihn
bei weniger Platz. Die weicheren Kanten und die leicht unterschiedliche
X-/Y-Abbildung bei 150/200 % bleiben Teil des offenen Befunds V23 in R9.
Die Seitenmarkierungen und Sidebaricons stimmen in allen 48 Aufnahmen.
Die alten Raumkompositionen, Wallpaperinhalte und Leistenpreview bleiben
ausdrücklich die nachgeordneten Reparaturaufträge R5–R7.
[Matrix und Buildnachweise](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/cases.json),
[exakte Wiederherstellung](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/cleanup.json).
Konfigurationsbytes, vollständiger Raum-/Output-Snapshot einschließlich
Fokus/Primärmonitor/Modi/Skalierung sind identisch; der Fensterbestand blieb leer.

**V24 bestätigt und korrigiert:** Escape schloss die Appauswahl zusammen mit
der Seite; erneuter Eintritt behielt die alte Auswahl. Reproduktion und
Nachherbilder stehen im vorherigen R4-Abschnitt. Die Korrektur schließt zuerst
die flüchtige Auswahl und setzt sie bei regulärem Seiteneintritt zurück.

**V25 bestätigt:** Im Mindestcanvas war die letzte Standard-App-Zeile
„Archive“ abgeschnitten und dadurch auch nicht per Tastatur erreichbar.
[Vorher bei 1366×768 logisch](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/r4-frame-drm-0-s1406-apps.png).
Ursache ist die nach Einbau der gemeinsamen Tabzeile überflüssige zweite
Abschnittsüberschrift im Gruppeninhalt. `settings_view/content/default_apps.rs`
entfernt sie in Übersicht und Auswahl; alle neun Kategorien, ihre Zeilenhöhe,
Textrollen und Aktionen bleiben erhalten. Keine neuen Assets, Timer oder
Caches; weniger Widgets werden nur bei bestehender Invalidierung gezeichnet.
`settings_view/layout_tests.rs` prüft am vollständigen echten Settings-Baum
bei 1920×1032 und 1366×720 alle neun vollständig sichtbaren Tastaturziele und
die dazu identischen Pointerhits. `settings_view.rs` bindet das Testmodul ein.

Alle Prüfungen bestanden: `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1243 bestanden, 0 Fehler,
1 bekannter isolierter D-Bus-Test ignoriert),
`cargo test -p niwoe-tokens --test design_guard`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo build --release -p niwoe-shell`. 68 Rust-Dateien stimmen nach
LF-Normalisierung zwischen Windows und Fedora überein.
[Unveränderte Rohlogs](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/apps-fit-gates/test.log).
Release, Installation und laufende Shell haben identisch SHA-256
`52f965f514a7fa8b9dd4c23ddccff17fa0eedb01993feeda8cd3f72c5bfa808b`;
Watchdogwechsel 125247 → 128499. Kein Neulogin erforderlich.
Die 32 gesonderten Nachheraufnahmen der Archivzeile wurden tatsächlich
angesehen: vollständige Appübersicht, sichtbarer Archivfokus, Ark-Auswahl und
Escape zurück zur Übersicht auf beiden Outputs bei allen vier Skalierungen.
Alle neun Zeilen bleiben erreichbar; keine Zuordnung wurde gespeichert.
[Nachhermatrix](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/apps-fit/cases.json),
[exakte Wiederherstellung](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/apps-fit/cleanup.json).
V25 ist damit visuell geprüft. R4 bleibt wegen der zusätzlichen Einstiege
und der Performanceprüfung in Arbeit; der Gesamtplan ist offen.

### R4: Zusätzliche Einstiege und Layerkonfiguration

**V26 bestätigt:** Beim direkten Einstieg aus Deck, System-IPC und
Desktopmenü lag die Settings-Kopfzeile unter dem Panel. Die vorhandene
Layer-Configure-Behandlung setzte die korrekte Arbeitsflächenreservierung
erneut auf Vollbild zurück, weil sie allein `room_management_open` berücksichtigte.
Ein Besuch aus der Raumkonfiguration behielt dagegen die richtige Lage.
[Deck davor](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-original/r4-entry-deck-system.png),
[Raumkonfiguration mit korrektem Rahmen davor](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-original/r4-entry-archive-picker-mouse.png).
`wayland/handlers/layer.rs` verwendet an beiden Stellen denselben
`control_center_visible()`-Zustand wie Rendern und Pointer. Keine neue Geometrie,
Compositoränderung, Animation oder zusätzliche Arbeit im Leerlauf.

Die zwölf Originalbilder wurden tatsächlich angesehen. Deck, IPC, Suche nach
Cursor sowie Raumdateien → Apps/Archivauswahl → zurück zum unveränderten
Raumdateienentwurf sind belegt. Der Desktop-Prüfhelfer traf mehrere Unterseiten
nicht; diese Bilder sind **NOT PASS**. Die Aufnahme mit dem Namen `power`
zeigte tatsächlich Netzwerk und bleibt wegen privater Profilnamen nur unter
`target/`. Der KWrite-Schritt schlug mit leerem Fensterbestand fehl, weil der
Prüfhelfer den bereits offenen Hub nochmals umschaltete. Das ist kein
bestandener heller Hintergrundtest. Konfiguration, Räume, Outputs und leerer
Fensterbestand wurden trotzdem exakt wiederhergestellt.
[Unveränderter Fehlversuch](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-original/r4-entry-review.log).
Die korrigierte Prüfung verwendet einen eindeutigen Desktop-Mausweg sowie
einen gezielt gestarteten KWrite mit isoliertem Testprofil und eigener Datei;
bestehende KDE-/GTK-Konfiguration wird nicht geändert.

Die elf korrigierten Aufnahmen wurden tatsächlich angesehen. Deck, System-IPC,
Cursor-Suche und alle sechs Desktop-Unterseiten zeigen nun den gemeinsamen
Rahmen korrekt unter dem Panel; kein Parallel-Settingsfenster erschien.
Netzwerk- und Audioaufnahmen mit privaten Profil-/Gerätenamen bleiben unter
`target/r4-corrected-images`. Der eigene KWrite wurde regulär geschlossen,
Konfigurationsbytes und vollständige Raum-/Output-Snapshots sind exakt identisch.
[Deck danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-corrected/r4-corrected-deck-system.png),
[Desktop-Hintergrundseite danach](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-corrected/r4-corrected-desktop-wallpaper.png),
[Cleanup](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-corrected/cleanup.json).
Alle sechs Gates und Release bestanden erneut, 1243 Workspace-Tests bestanden,
0 Fehler, 1 bekannter isolierter D-Bus-Test ignoriert. Release/Installation/
laufende Shell: `87ffe99df2617924faa560019f92e08df32ba6680183f29180346ed7065a803a`;
Watchdog 136617 → 141074.
[Prüflogs](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/layer-gates/test.log).

**V27 bestätigt:** Im hellen Hintergrundtest schienen KWrite-Menüs und Text
durch die freie Settings-Inhaltsfläche. Die Texte der beiden Oberflächen
dürfen nicht konkurrieren.
[Vorher mit echter Anwendung](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-corrected/r4-corrected-over-bright-application.png).
Die zentrale Control-Center-Inhaltsrolle in `niwoe-tokens/src/control_center.rs`
wird deckend; Panel-, Hub-, Deck- und Lautstärkematerial bleiben in ihren
freigegebenen compositorseitigen Rollen. Kein weiterer Effekt und keine
zusätzliche Raster-/Capturearbeit. Prüfung und Nachheraufnahme ausstehend.

### R4: Direkteinstieg auf beiden Outputs

Die zusätzliche direkte Matrix wurde vollständig ausgeführt und alle 40
Aufnahmen tatsächlich angesehen: System-Direkteinstieg, Apps über Sidebar,
Tastaturfokus auf Archive, Enter zur Ark-Auswahl und Escape zurück zu Apps.
Beide Outputs wurden jeweils als Primäroutput bei 100 %, 1920/1366,
150 % und 200 % geprüft. Während der Matrix stehen beide physisch bei
1920×1080/60 Hz;
1366×768 ist die logische Testgröße, kein behaupteter DRM-Modus.
Der gemeinsame Rahmen liegt in allen Fällen unter dem Panel, alle neun
Appzuordnungen bleiben vollständig sichtbar und bedienbar. V26 ist damit
visuell geprüft. Die bekannte Rasterweichheit V23 bei hoher Skalierung bleibt
offen und wird nicht durch das bestandene Geometriegate erledigt.
[Matrix und Buildidentität](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/direct-fit/cases.json),
[exaktes Cleanup](UI_VISUAL_AUDIT_2026-10-02-evidence/r4-matrix/direct-fit/cleanup.json).
Konfiguration, vollständige Raum-/Output-Snapshots und leerer Fensterbestand
sind nach expliziter Wiederherstellung identisch; Stand ist der Layer-Release
`87ffe99df2617924faa560019f92e08df32ba6680183f29180346ed7065a803a`.
Das bedeutet anschließend wieder drm-0 mit 1920×1080/60 Hz und drm-1 mit
3840×2160/30 Hz, jeweils 100 %, wie im ursprünglichen Snapshot.

Die anschließende zentrale Kontrastkorrektur besteht dieselben sechs Gates
einschließlich Releasebau: 1243 Tests bestanden, 0 Fehler und 1 bekannter
isolierter D-Bus-Test ignoriert; 68 Rust-Quellen stimmen LF-normalisiert überein.
[Prüflogs](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/contrast-gates/test.log).
Release/Installation/laufende Shell haben identisch SHA-256
`ed28bb6c99ff25031946fed377d8c2db7d66fc87f13ec0d7befd139aeb0b151a`;
Watchdogwechsel 155484 → 156233, kein Neulogin erforderlich.
[Aktivierungsnachweis](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/contrast-gates/activation.json).
Die tatsächliche Nachherprüfung über heller Anwendung läuft separat;
V27 erhält bis zu deren Bildprüfung keinen visuellen Abschluss.

Die elf Nachherbilder des Kontrast-Releases wurden anschließend tatsächlich
angesehen: Deck, direkter Systemaufruf, Cursor-Suche, alle sechs Desktop-
Unterseiten und die reale helle KWrite-Anwendung vor/hinter dem Control Center.
Fremde Menüs und Dokumenttext scheinen nicht mehr durch die Inhaltsfläche;
Rahmen, Sidebar-Aktivzustand und Seiteninhalte bleiben korrekt. V27 ist damit
visuell geprüft. Die Inhaltsmängel an Wallpaper, Cursor und Gerätetexten bleiben
ausdrücklich R7 und sind durch diesen Materialnachweis nicht abgeschlossen.
[Nachher über heller Anwendung](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-contrast/r4-contrast-over-bright-application.png),
[Wiederherstellung](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/entry-contrast/cleanup.json).
Der eigene KWrite ist regulär geschlossen; Konfigurationsbytes, Räume,
Outputs und leerer Fensterbestand sind identisch. Audio-/Netzwerkbilder mit
privaten Namen bleiben ausschließlich unter `target/r4-contrast-images`.

Die vier zusätzlichen Popup-Aufnahmen wurden tatsächlich angesehen. Der
Netzwerk-Footer führt direkt zur Netzwerkseite im gemeinsamen Rahmen;
Escape stellt den Desktop wieder her. Die erste Audioaufnahme zeigte dagegen
das Systemdeck: Der sichtbare Lautstärkeeintrag führt wie die übrige aktive
Statusinsel zum Deck. Der alte separate `audio_popup` hat derzeit keinen
aktiven Panel-Einstieg (`ToggleAudioPopup` wird von keinem Panelhit erzeugt).
Die mit `sound-settings` benannte Aufnahme zeigte folglich nur den Desktop
und ist **NOT PASS** für einen Audio-Footer. Kein neuer Produktpfad wird
dafür erfunden. Der reguläre Audio-Einstieg über das Desktopmenü ist in der
vorherigen Runde belegt. Alle unveränderten Rohbilder bleiben unter
`target/r4-footer-images`; [Cleanup und Build](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/footer/cleanup.json)
belegen identische Konfiguration, Räume, Outputs und leeren Fensterbestand.

### R4: Öffnungszyklen und aktuelles Leerlaufgate

Auf `ed28bb6c99ff25031946fed377d8c2db7d66fc87f13ec0d7befd139aeb0b151a`
sind 40 Öffnen-/Navigieren-/Schließen-Zyklen ausgeführt: System → Apps →
Benutzer → Einstellungen → System → Leiste → zurück zum Desktop. Kein
Speichern. Die Messpunkte bei 0/20/40 Zyklen enthalten vollständige Snapshots;
Konfigurationsbytes, Räume, Outputs und leerer Fensterbestand sind identisch.
Shell-RSS: 161058816 → 177704960 → 177836032 Byte. Compositor-RSS:
169701376 → 177598464 → 177598464 Byte. Im zweiten 20er-Block steigt die Shell
nur um 131072 Byte; beim Compositor gibt es kein weiteres Wachstum.
[Unveränderte Zyklusdaten](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/performance/cycles.json).

Danach 30 Sekunden Warm-up und drei vollständige 300-Sekunden-Proben, ohne
GUI-/Buildaktivität während Idle. drm-0: 1920×1080/60 Hz; drm-1:
3840×2160/30 Hz, jeweils 100 %. Das sind dieselben Modi und derselbe
Raum-/Fensterbestand wie vor R1. Shell-CPU: 0,17667 / 0,18667 / 0,18000 %;
Compositor-CPU: 1,07333 / 1,07667 / 1,08000 %, jeweils ein CPU-Kern.
CPU-Mediane gegenüber der Ausgangsmessung: −0,00333 Prozentpunkte Shell,
+0,01336 Prozentpunkte Compositor, deutlich unter dem 0,5-Punkte-Budget.
GPU-Engine-Zeit: 0,00783 / 0,00828 / 0,00825 %; das ist die vorhandene
Engine-Metrik, keine behauptete Gesamtauslastung der GPU. In allen drei
Proben bleiben Shell-RSS 177836032 und Compositor-RSS 177598464 Byte exakt
stabil; PIDs, Konfiguration und vollständige Snapshots sind unverändert.
Die älteren RSS-Werte hatten einen anderen aufgewärmten UI-Bestand;
darauf wird keine Speicherersparnisbehauptung gestützt. Repaint- und
Eingabe-bis-sichtbar-Latenz wurden nicht gemessen und bleiben unbekannt.
[Rohdaten](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/performance/idle.json),
[Phasenprotokoll](UI_VISUAL_AUDIT_2026-10-02-evidence/r4/performance/r4-performance-final.log).

R4 ist damit für den gemeinsamen Rahmen und die geprüften Zugangs-/Rückwege
visuell geprüft: V01 und V07 sind erledigt, V24–V27 ebenfalls. Der veraltete
Settings-Pinning-Eintrag ist entfernt; weitere Pinning-Texte gehören zu R7.
V23 und die Inhalte von R5–R8 bleiben offen. Dies ist keine Nutzerabnahme
oder Gesamtfreigabe; P13 bleibt gesperrt. Nächster Schritt ist R5, zunächst
der eigene Verwaltungskopf nach gesicherter Live-Reproduktion.
## R5, erster kleiner Schritt: Verwaltungskopf und Sidebar

Status: **in Arbeit**. Vor der Änderung wurde der laufende R4-Stand
`ed28bb6c99ff25031946fed377d8c2db7d66fc87f13ec0d7befd139aeb0b151a`
erneut über Wallpaper und einer eigens geöffneten hellen KWrite-Anwendung
aufgenommen. Alle drei Aufnahmen wurden angesehen; Belege liegen in
`UI_VISUAL_AUDIT_2026-10-02-evidence/r5/before/`. Der Verwaltungskopf zeigte
fremde Menüs und Dokumenttext. Zusätzlich war der gesamte Sidebarinhalt über
dem hellen Fenster durchscheinend. Dieser neu reproduzierte Sidebarfehler ist
**V28**; der zuvor geprüfte Settings-Einstieg hatte einen anderen, deckenden
Untergrund und deckte diesen Fall nicht ab.

Geändert: `room_management_view.rs` delegiert den Kopf an das kleine Modul
`room_management_view/header.rs`. Dieses verwendet die vorhandene Landschaft
mit proportionalem Cover-Ausschnitt und die gemeinsame Serif-Überschriftenrolle.
`niwoe-tokens/src/control_center.rs` setzt die zentrale Sidebar-Deckkraft auf
255. Panel, Hub und Deck behalten ihren freigegebenen Materialpfad.

Performance: dieselbe einmalig dekodierte Landschaft (6-MiB-Quellgrenze) und
derselbe nach Zielmaßen begrenzte Cache mit höchstens vier Einträgen/8 MiB;
keine zusätzliche Capture-, Timer- oder Animationsschleife. Die Sidebar bleibt
eine statische eventgetriebene Fläche. Das eigens geöffnete Testfenster wurde
regulär geschlossen; vollständige Konfiguration und Raum-/Outputzustand sind
vorher/nachher verglichen. Raster, Toolbar und bildliche Raumkarten folgen erst
nach Prüfung und Installation dieses Schritts. R5 ist dadurch nicht abgeschlossen.

Nachherstand vom 03.10.2026: **V03/V28 am Primary bei 1920×1080/100 %
visuell geprüft**, weitere Zielgrößen folgen in der R5-Matrix. Alle drei echten
Nachherbilder unter `r5/header-after/` wurden angesehen. Landschaft und Sidebar
bleiben über Wallpaper und KWrite deckend und lesbar; keine fremden Menüs oder
Dokumentzeilen sind sichtbar. Die ursprünglichen neun Textkarten bleiben als
noch offener Befund sichtbar. Installierter und laufender Shell-Hash:
`ddc45aa02a004d986bfdea19455f6e7ac934315119314ba971a79d206f387715`.
Watchdog-PID 156233 → 196585; kein Neulogin erforderlich. Alle sechs Prüfungen
und Releasebau haben Exit 0: Formatierung, Workspace-Check, Workspace-Tests
(1243 bestanden, 0 Fehler, 1 bekannter isolierter D-Bus-Test ignoriert),
Design Guard und Clippy mit `-D warnings`. Alle 69 übertragenen Rust-Quellen
stimmen nach LF-Normalisierung überein. Vollständige Logs und drei identische
Buildhashes: `r5/header-gates/`. Nachher-Cleanup bestätigt unveränderte
Konfigurationsbytes, vollständige Raum-/Output-Snapshots und keine Testfenster.

### R5, zweiter Schritt: sechs Karten und echte Seitennavigation

Status: **in Arbeit**. Die gerade geprüften Header-Nachherbilder dienen als
unveränderte Vorherbasis dieses Schritts: neun gleichartige Textkarten ohne
sichtbare Seitennavigation. Die zentrale Geometrie in
`niwoe-tokens/src/control_center.rs` definiert jetzt drei Spalten/zwei Reihen
und einen begrenzten Seitenfooter. `room_management_view.rs` benutzt diese
Geometrie für Zeichnen und Treffer; `room_management_view/pages.rs` enthält
sichtbare, an den Seitengrenzen deaktivierte Zurück-/Weitercontrols.

`wayland/state/room_form.rs` bindet die Aktionen an die tatsächliche gefilterte
Raumliste. `handlers/keyboard/room_navigation.rs` überspringt nicht verfügbare
Seitenaktionen und den neuen Raum bei voller Kapazität; der Wechsel auf
Toolbarfokus springt nicht mehr ungewollt zur letzten Raumseite.
`handlers/pointer/launcher.rs` verwendet dieselbe getestete Wayland-
Scrollrichtung wie der Hub und verwirft veralteten Kartenfokus beim Blättern.
`state/popups.rs` erhält die Seite bei einem vorübergehenden Settings-Besuch.
Die gezielten Tests prüfen 0/1/6/9/10/64 Räume, beide Zielcanvases, stabile
Eintragsindices, vollständig erreichbare Treffer und verfügbare Fokusziele.

Performance: feste statische Geometrie, höchstens sechs gezeichnete Karten;
Seitenwechsel bleiben eventgetrieben. Keine Assets, Captures oder Timer neu.
Die weiterhin fehlende Toolbarkomposition und bildlichen Karten bleiben offen.

Dieser Raster-/Paging-Schritt ist am installierten Stand
`39ca37599e18ee055570bd1de602cfebb108a30aba30a683d739ae5f5acffea4`
**visuell geprüft**. Alle zwölf echten Aufnahmen in `r5/pages-after/` wurden
angesehen: erste/zweite Seite, Settings-Rückweg auf Seite 2, Raum 9 per Maus,
Tastaturfokus auf Raum 9 und Öffnen per Enter, jeweils bei logischen 1920×1080
und 1366×768 am Primary. Sechs Karten und beide Seitenschaltflächen sind
vollständig erreichbar; die echten Konfigurationsansichten zeigen jeweils
Raum 9/Position 9 von 9. Der bislang gestauchte Konfigurationskopf bleibt ein
sichtbarer R6-Befund. Die Skalierungsweichheit V23 bleibt ebenfalls offen.

Alle sechs Gates und der Releasebau haben Exit 0, 1245 Workspace-Tests
bestanden, 0 Fehler/1 bekannter isolierter D-Bus-Test ignoriert. Alle 73
übertragenen Rust-Quellen stimmen überein; Release/installiert/laufend haben
denselben Hash. Logs: `r5/pages-gates/`. Das abschließende Cleanup bestätigt
exakte ursprüngliche Konfigurationsbytes, vollständige Raum-/Output-Snapshots,
Display-Modi/Skalen/Primary/Fokus und keine Fenster. Es wurden ausschließlich
temporäre Outputtabellen verwendet; die ursprünglichen Modi sind wiederhergestellt.

### R5, dritter Schritt: Toolbar, Kartenbilder und konsistente Statistik

Status: **in Arbeit**. Die drei aktuellen Vorherbilder unter
`r5/preview-before/` wurden angesehen. Build `39ca375…` zeigt trotz echter
KWrite-Anwendung nur deren Fenstertitel, keine bildliche Vorschau. Der
Vorherlauf hat die unveränderten Konfigurationsbytes und vollständigen
Raum-/Output-Snapshots sowie das reguläre Schließen seines Testfensters belegt.

Geänderte Verantwortungen: `room_management_view/list.rs` enthält gemeinsame
Toolbar-/Treffergeometrie, echte Filterzähler, Sortierung, native Such-, Raster-
und Listensymbole sowie klar fehlende Kategorien. Bei geringer Breite steht die
Suche in einer zweiten Zeile; diese notwendige Anpassung erhält lesbare Controls
im 1366×768-Canvas. `room_management_view/management.rs` und `management/rail.rs`
zeichnen die bildlichen Karten, echte Listenansicht, gestaltete Leerzustände,
Schnellaktionen und Statistik mit einheitlichem Bezug auf **alle** Räume.
App-/Fensterzahlen stammen ausschließlich aus Katalog-/Raum-/Fensterdaten.
Für Datei- und Automatisierungszahlen fehlt eine Quelle; solche Zahlen werden
nicht erfunden. Vorlagen/Import/Export bleiben ausdrücklich nicht verfügbar.

`room_management_view.rs`, `cards.rs` und `configuration/body.rs` trennen die
neue Verwaltungsdarstellung von der bisherigen, erst in R6 zu reparierenden
Konfigurationsvorschau. `room_editor_list.rs`, `state/room_form.rs`,
`keyboard/room_navigation.rs`, `pointer/launcher.rs` und `render/launcher.rs`
binden die tatsächliche Ansicht an Maus/Tastatur und an dieselben stabilen IDs.
Neue Geometrie/Bounds stammen ausschließlich aus `niwoe-tokens/control_center.rs`;
`niwoe-ui/effect/symbol.rs` liefert zwei funktionale Vektoricons im bestehenden
Cache. `effect/text_roles.rs` ergänzt die gemeinsame Titel-/Unterzeilenrolle
mit unverändertem fontmetrischem Mindestabstand, exportiert über `effect/mod.rs`.

`hub_state.rs` und `wayland/hub.rs` nutzen dieselbe generationengebundene
Vorschauquelle für Hub und Verwaltung. Verwaltungsgrenze: höchstens sechs
512×192-RGBA-Bilder, also maximal 2.359.296 Rasterbytes plus begrenzte Metadaten;
Hubgrenzen bleiben vier 320×112-Bilder. Cache-Schlüssel sind sichtbare stabile
Raum-/Fensteridentitäten, Seite, Größenlimit, Output/Canvas-Kontext. Ein
unverändertes Prepare erzeugt keine Requests. Seiten-/Filter-/Fenster-/Minimier-
und Outputwechsel invalidieren; Schließen/Lock leert Bilder und Pending-Requests.
Verspätete oder fremde Antworten werden abgewiesen. Keine neue Capture- oder
Animationsschleife. Bilder werden proportional innerhalb ihres Slots gezeigt;
die kompakte Zielgröße verwendet entsprechend kleinere Thumbnails.

Der erste Prüflauf bestand Check, Workspace-Tests und Design Guard, scheiterte
aber an Clippy `-D warnings`: der alte, unbenutzte `chip`-Helfer war übrig.
Dieser Helfer wurde entfernt; der vollständige zweite Lauf folgt separat.
Der Fehlversuch ist unverändert unter `r5/cards-attempt-1/` erhalten und zählt
nicht als Releasefreigabe. R5 bleibt bis zur tatsächlichen Nachherprüfung offen.

Der zweite Lauf hat alle sechs Gates und Release mit Exit 0 bestanden:
1248 Workspace-Tests, 0 Fehler/1 bekannter isolierter D-Bus-Test ignoriert,
Formatierung, Check, Design Guard, Source-Size-/Zentralitätsguard und Clippy
`-D warnings` grün. Alle 82 übertragenen Rust-Quellen stimmen nach
LF-Normalisierung überein. Release/installiert/laufend haben den Hash
`f262beb0dde9944d9bed34e15cc29ed56d91b8d230e1eb0a09762b60804e4489`;
Watchdog-PID 206404 → 216583, ohne Neulogin. Belege: `r5/cards-gates/`.

Alle drei echten Nachherbilder unter `r5/cards-after/` wurden angesehen:
Verwaltung über Wallpaper, echte helle KWrite-Anwendung und Verwaltung darüber.
Die Karte zeigt deren tatsächliches proportioniertes Fensterbild; übrige Räume
haben einen ehrlichen Leerzustand. Toolbar, native Icons und Statistik bleiben
lesbar und hintergrundunabhängig. Cleanup bestätigt unveränderte Bytes und
vollständige Snapshots sowie regulär geschlossenes eigenes KWrite-Fenster.

Zusätzlich wurden alle neun tatsächlichen Aufnahmen in `r5/lifecycle/`
angesehen. Eigene reale GTK-Wayland-/X11-Fenster zeigen Bilder, Minimierstatus,
erneuerte Bilder nach Restore und Leerzustand nach Fensterende. Ein eigener
Wayland-Fenstermove nach Raum 2 entfernt das Bild aus Raum 1 und zeigt es nur
in Raum 2; der aktive Raum bleibt unverändert. Die Bilder sind echte Captures
dieser Anwendungen, keine eingebauten Beispielbilder. Nur diese selbst
gestarteten Prozesse wurden geschlossen; exakte Konfiguration, Raum-/Output-
Snapshots und leere Fensterliste sind bestätigt. V05 ist damit für die
**Verwaltung am Primary/1920×1080/100 %** visuell geprüft; Konfiguration folgt
in R6. Toolbarbedienung, vollständige Output-/Scale-Matrix und aktuelle
Performanceprüfung sind noch nicht als bestanden erklärt.

Alle zehn Bedienaufnahmen in `r5/controls/` wurden anschließend angesehen:
Belegt-Filter ohne Ergebnis, Leer-Filter, Name-A–Z-Sortierung, echte Listenansicht,
Suche ohne Treffer, Rückkehr zum Raster, neuer ungespeicherter Entwurf und
Abbrechen sowie Tastaturfokus/Enter am Weitercontrol. Zähler und Statistik
behalten den richtigen Datenbezug; nicht verfügbare Kategorien und
Schnellaktionen sind klar bezeichnet. Die vorläufige Konfigurationsansicht
zeigt weiterhin den bereits zugeordneten R6-Mangel „Raum 0“ und den gestauchten
Kopf; diese Aufnahme ist kein bestandener R6-Nachweis. Abbrechen hat keinen
Raum angelegt. Exakte ursprüngliche Konfigurationsbytes, vollständige Raum-
und Output-Snapshots und leere Fensterliste sind bestätigt.

Die Monitor-/Scale-Matrix ist vollständig abgeschlossen. Alle 56 echten
Aufnahmen unter `r5/matrix/` wurden angesehen: beide Outputs jeweils bei
100 %, 140,6 % (logisch 1366×768), 150 % und 200 %. Jeder Fall umfasst erste
Rasterseite, Liste, zweite Seite, Rückkehr aus Einstellungen zur zweiten
Seite sowie Raum 9 per Maus, Tastaturfokus und Enter. Raster und Listen bleiben
innerhalb der gemeinsamen Fläche; Paging und Konfigurationsziel stimmen.
Die in den Konfigurationsbildern sichtbaren R6-Mängel bleiben ausdrücklich
offen. Die schon dokumentierte Rasterweichheit V23 bleibt ebenfalls offen.
Cleanup bestätigt bytegleiche Konfiguration, vollständige Raum-/Output-
Snapshots und leere Fensterliste einschließlich Wiederherstellung der
ursprünglichen 4K/30-Hz-Konfiguration auf dem zweiten Output. Laufender Hash
ist unverändert `f262beb0dde9944d9bed34e15cc29ed56d91b8d230e1eb0a09762b60804e4489`.
Die aktuelle 40-Zyklen-/3×300-s-Performanceprüfung ist danach vollständig
abgeschlossen (Exit 0, Rohdaten und Log unter `r5/performance/`). Dieselbe
Hardware und ursprüngliche Outputkonfiguration wurden verwendet. Nach 40
Raster-/Listen-/Paging-Öffnungszyklen und 30 s Warm-up gab es während der drei
Ruheproben weder GUI- noch Buildaktivität.

| Probe (je 300 s) | Shell-CPU, ein Kern | Compositor-CPU, ein Kern | GPU-Engine-Zeit |
| --- | --- | --- | --- |
| 1 | 0,189999820 % | 1,053332334 % | 0,007792641 % |
| 2 | 0,183333152 % | 1,069998941 % | 0,008010203 % |
| 3 | 0,176666487 % | 1,073332243 % | 0,006769722 % |

CPU-Median gegenüber R4: Shell +0,003333344, Compositor −0,006666613
Prozentpunkte; gegenüber R1: Shell +0,000004158, Compositor +0,006690775.
Auch die Einzelwerte bleiben unter dem Budget von +0,5 Prozentpunkten.
GPU-Engine-Zeit ist keine Messung der vollständigen GPU-Auslastung; daraus
wird kein Einsparungsversprechen abgeleitet.

RSS nach 0/20/40 Zyklen: Shell 31.416.320 / 130.428.928 / 130.560.000 Byte;
Compositor 128.049.152 / 168.607.744 / 175.710.208 Byte. Die zweiten 20 Zyklen
erhöhen Shell-RSS um 131.072 und Compositor-RSS um 7.102.464 Byte. Während aller
drei Ruheproben sind beide Werte vor/nach jeder Probe exakt konstant:
130.560.000 / 175.710.208 Byte. Das belegt Ruhekonstanz, keinen unbegrenzten
Lebensdauernachweis; längere Compositor-/Cache-Zyklen bleiben in R9 offen.
Konfigurationsbytes, vollständiger Raum-/Output-/Fensterzustand und PIDs sind
über alle Zyklen und Proben unverändert. V03/V06 sind in den dokumentierten
R5-Zuständen visuell geprüft; V05 für die Verwaltung ist mit echten Wayland-
und X11-Lebenszyklen belegt. Raumkonfiguration, lange Live-Datenfälle und V23
bleiben offen. R6 beginnt erst nach diesem R5-Abschluss; keine Gesamtfreigabe.

### R6: Vorhervergleich und erster gemeinsamer Kopf-/Vorschauschritt

Vor der Änderung wurden alle acht Aufnahmen in `r6/before/` angesehen.
Sieben sind echte Seitenbelege des installierten R5-Hashes: Allgemein leer,
Apps, Dateien, Wiederherstellung, neuer Entwurf, Allgemein über einem eigenen
realen GTK-Wayland-Fenster und Leiste im Control Center. Der erste F7-Aufruf
bei geschlossenem Control Center zeigt nur das eigene Testfenster; diese
historische Aufnahme wird ausdrücklich nicht als Leistenprüfung gewertet.
Der korrigierte sichtbare Sidebaraufruf und sein vollständiger Cleanup sind
separat erhalten. Beide Prüfungen bestätigen unveränderte Konfigurationsbytes
und vollständigen Raum-/Outputzustand; nur der eigene GTK-Prozess wurde beendet.
Vorherbelege der kleineren Konfiguration liegen zusätzlich in `r5/matrix/`.

V04/V05/V08–V11/V20 sind reproduziert: gestreckter Bildkopf, Fenstername ohne
Bild, kleiner „Icon“-Textbutton, Appzeilen ohne Icons/getrennte Textrollen,
identische Dateien-/Restore-Flächen mit rohem OS-Leerfehler, skalierte
Leistenminiatur und „Raum 0“ im neuen Entwurf. Zuerst werden Bildkopf,
gemeinsame native Raumkarte und begrenzte Vorschauquelle korrigiert. Erst
danach folgen die Detailcontrols, Apps, getrennte Dateien/Restore und Leiste.
Ein bestandener erster Schritt schließt R6 insgesamt noch nicht ab.

Der erste R6-Schritt ist geprüft und installiert, noch nicht visuell freigegeben.
Dateien: `room_management_view/header.rs` nutzt für beide Köpfe den gleichen
proportionalen Landschaftscache und die zentrale Serif-Überschriftenrolle;
`configuration/chrome.rs` trennt den festen Kopf, Tabs und Footer vom
scrollbaren Inhalt. `configuration.rs` begrenzt die Vorschau auf die verfügbare
Höhe, `configuration/body.rs` verwendet die native Karte aus `management.rs`
mit Entwurfsname und echten Bildern. `room_management_view.rs` nimmt die alte
Karte aus dem aktiven Renderpfad. Name/Beschreibung und Aktionen verwenden
gemeinsame Fokus-/Controlrollen. `configuration/form.rs` passt die Appgeometrie
an den größeren Kopf an; die vollständige Appdarstellung folgt separat.
`wayland/hub.rs`, `hub_state.rs` und `render/launcher.rs` binden die Vorschau
ausschließlich an den stabilen bearbeiteten Raum, ohne Raumaktivierung.
`control_center.rs` enthält die zentrale Kopf- und Capturegeometrie;
`configuration/tests.rs` prüft jetzt die tatsächliche kleine Control-Center-
Fläche, `hub_state/tests.rs` die Auswahl und Cachegenerationen.

Performance-Modell: Der bereits begrenzte Landschaftscache wird geteilt,
ohne zweiten Decoder. Die allgemeine Konfiguration hält maximal ein echtes
512×288-RGBA-Bild (589.824 Byte) plus begrenzte Requestmetadaten; sechs Karten
der Verwaltung bleiben bei ihrer bisherigen 512×192-Grenze. Wiederholte
Renderaufrufe derselben Signatur starten keine Captures. Raum-/Fensteridentität,
Minimieren, Output/Canvas, Tabs, Fensterende und Schließen invalidieren den
Cache. Neue ungespeicherte Räume starten keine Fenstercaptures. Vorhandenes
typisiertes IPC und dessen serverseitige Capturegrenzen bleiben erhalten.

Alle 85 übertragenen Rust-Quellen stimmen LF-normalisiert überein.
`cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace` (1.249 bestanden, 0 fehlgeschlagen, 1 bekannter
DBus-Test ignoriert), Design Guard und Clippy mit `-D warnings` sowie der
Releasebuild sind grün. Belege: `r6/preview-gates/`. Der erste Installations-
Hilfsaufruf scheiterte an `ProcessStartInfo.ArgumentList` unter Windows
PowerShell; dessen Kompatibilitätsfallback wurde im ignorierten lokalen Helper
korrigiert. Ein zu früh gestarteter Testlauf auf dem alten Hash wurde
unterbrochen und vollständig bereinigt; er ist unter `r6/preview-attempt-1/`
als Fehlversuch erhalten. Er zählt nicht als R6-Nachherprüfung.
Erst danach war die Installation erfolgreich. Release/installiert/laufend
haben jetzt `5ef80cb0758c3e63cd92e83721866d48cb2be19ab05a57d4b8607a79d81f9ed9`;
Watchdog-PID 237680 → 238464, ohne Neulogin. Die neue Nachherprüfung verlangt
vor jeder Testfenstererzeugung genau diesen dreifachen Hash.

Alle 14 echten Nachheraufnahmen unter `r6/preview-after/` wurden angesehen.
Eigene Wayland- und X11-Fenster zeigen ihr wirkliches Bild, einen klaren
Minimierzustand, eine erneuerte Vorschau nach Restore und den Leerzustand nach
Fensterende. Ein eigener Wayland-Fenstermove entfernt das Bild aus Raum 1;
beim Bearbeiten von Raum 2 erscheint es dort und verschwindet nach Rückmove.
Der aktive Raum bleibt in allen Fällen Raum 1. Der neue Entwurf zeigt einen
Namen/Entwurfsstatus statt „Raum 0“, übernimmt den eingegebenen Namen sichtbar
in Kopf und Vorschau und erzeugt nach Abbrechen keinen neuen Raum.
Der Kopf ist gegenüber den Vorherbildern proportional, mit gleicher zentraler
Typografie wie die Verwaltung. Beide Bildpfade bleiben über dem echten hellen
GTK-Testfenster lesbar. Vollständige Raum-/Output-Snapshots, Konfigurationsbytes
und leere Fensterliste nach Beendigung ausschließlich eigener Prozesse sind
bestätigt. Diese Nachweise gelten für Primary/1920×1080/100 %; die kleineren
Flächen und der zweite Output werden separat geprüft.

Konkrete technische Grenze gegenüber dem illustrativen Mehrfensterbild `(4)`:
das vorhandene Thumbnail-IPC liefert einzelne echte Fenster; `WindowInfo`
enthält keine räumlichen Fensterkoordinaten. Die Vorschau zeigt deshalb ein
wirkliches repräsentatives Fenster des bearbeiteten Raums und dessen reale
Zähler, ohne eine erfundene Mehrfensteranordnung. Eine andersartige Capture-
oder Geometrie-API wurde in dieser UI-Reparatur nicht eingeführt.


Die separate Größenprüfung ist abgeschlossen: alle 20 Nachherbilder unter
`r6/preview-matrix/` wurden angesehen (beide Outputs, logische Control-Center-
Flächen 1920×1032 und 1366×720, jeweils Allgemein, untere Controls,
neuer Entwurf, Live-Benennung und Abbrechen). Die physischen Tests liefen
jeweils bei 1920×1080; Scale 1920/1366 stellt die kleinere logische Fläche her
und ist kein Nachweis eines physischen 1366×768-DRM-Modus. Kopf und Vorschau
bleiben proportional; Name, Entwurfsstatus und Abbrechen stimmen auf beiden
Outputs. Die untere Konfiguration ist per Tastatur vollständig erreichbar,
während Tabs und Footer fest bleiben. Diese Scrollgrenze ist bei der kleinen
Fläche erforderlich: Kopf, Tabs und die vollständigen Details-/Kontextgruppen
passen gemeinsam nicht in die verfügbare Höhe. Die Vorschau passt ihre Höhe
an diese Fläche an. Die hier noch vorhandenen alten Symbol-/Kontextbuttons
werden im nächsten R6-Schritt ersetzt; ihre Darstellung gilt nicht als V08-
Abschluss. Die Skalierungsweichheit V23 bleibt offen.

Die Matrixbereinigung bestätigt identische Konfigurationsbytes, vollständige
Raum- und Output-Snapshots einschließlich der ursprünglichen Modi/Skalen sowie
keine eigenen Restfenster. Laufender Hash bleibt `5ef80cb…`, Shell-PID 241110.
R6 ist damit teilweise geprüft/installiert: Kopf, echte Vorschau und Neu-Raum-
Entwurf. Symbol-/Kontextcontrols, Appzeilen, getrennte Datei-/Restore-Inhalte
und die native Leistenpreview bleiben in der festgelegten Reihenfolge offen.

### R6 – zweiter Teilschritt: Symbol, Kontext und App-Auswahl

**Status:** in den folgenden Zuständen visuell geprüft und installiert; keine
Gesamtfreigabe von R6 oder V08/V09. Der Vorher-Stand `5ef80cb…` zeigte die
alten Textbuttons, unstrukturierte Appzeilen und unpassende Fokusziele.
Vorher-Aufnahmen liegen in `evidence/r6/forms-before/`.

Geänderte Dateien und Zweck:
- `configuration/form.rs`, neue `form/general.rs` und `form/apps.rs`:
  gemeinsame Geometrie, sichtbare gecachte Symbolvorschau, beschriftete
  Kontextzeilen und native Appzeilen mit Icon, Haupt-/Unterzeile und Checkbox.
  Native und XWayland-Referenzen bleiben unabhängig auswählbar; fehlende
  Katalogeinträge bleiben sichtbar und entfernbar.
- `configuration.rs`, `configuration/chrome.rs`, `configuration/restore.rs`:
  passende Feldabstände, native Tabicons, klare Entwurfs-/Footertexte sowie
  ein gemeinsamer Tab-Hitbereich. Noch keine Trennung der Restore-Inhalte.
- `room_editor_form.rs`, `wayland/state/room_form.rs`, `configuration/tests.rs`:
  Fokusreihenfolge aus tatsächlichen Fähigkeiten und sichtbaren Zeilen;
  deaktivierte Tabs/Seitenwechsel und nicht vorhandene Appzeilen werden
  übersprungen. Regressionen prüfen Neu-Raum-Entwurf, fehlende Referenzen,
  getrennte Native/XWayland-Auswahl und Tabgeometrie.

Performance: vorhandene Icon- und Desktopkatalog-Caches werden verwendet.
Nur sechs sichtbare Appzeilen werden komponiert; Namensgleichheit wird im
vorhandenen begrenzten Katalog geprüft. Keine neue Capture-, Lade-, Timer-
oder Frame-Schleife, kein zusätzlicher Bildcache. Änderungen bleiben lokal
im Raum-Entwurf bis zum bestehenden Speichern; Abbrechen verwirft sie.

Verifikation auf Fedora, alle Exitcodes 0: `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1251 bestanden, 0
fehlgeschlagen, ein bekannter DBus-Test ignoriert),
`cargo test -p niwoe-tokens --test design_guard`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo build --release -p niwoe-shell`. Source-Size-/Zentralitätsguards sind
Teil der bestandenen Workspace-Tests. Alle 89 übertragenen Rust-Dateien
wurden nach LF-Normalisierung mit dem lokalen Stand verglichen. Logs und
Aktivierungsbeleg: `evidence/r6/forms-gates/`.

Release, installierte und laufende Shell haben denselben SHA256:
`8a61bbcbb3714d3a169d16cf2485f835c0c3d3a771a5105df563312f3575430b`.
Watchdog-Aktivierung PID 241110 → 245173; kein Neulogin erforderlich.

**40 neue Live-Aufnahmen tatsächlich angesehen:** beide primären Outputs
mit 100 % und dem aus FHD abgeleiteten 1366er Mindestcanvas (Skala 1,406),
jeweils Allgemein, Ordnersymbol, bevorzugte App-Zuordnung, Layout-Only-
Entwurf, Apps, zwei getrennt ausgewählte Ark-Referenzen, Tastaturfokus auf
Speichern, Such-Leerzustand, Neu-Raum mit nicht verfügbaren Layoutaktionen
und verworfene Entwürfe. Symbol-/Textabstände, Auswahl, native Icons und
Fokus passen in diesen Zuständen; der Mindestcanvas bleibt bedienbar.
Die aufgenommenen Quellen sind 1920×1032 bzw. 1366×720 bei physischem FHD,
kein behaupteter physischer 1366er DRM-Modus. Bilder und Rohdaten:
`evidence/r6/forms-matrix/`. Die bekannte Rasterweichheit V23 bleibt offen.

Cleanup bestätigt identische Konfigurationsbytes, vollständige Raum- und
Output-Snapshots und keine Testfenster; laufender Hash bleibt `8a61bbc…`,
Shell-PID nach Wiederherstellung 247454. Diese Runde hat keine Raumänderung
persistiert. Lange Live-Appnamen, fehlende installierte Katalogeinträge,
aktiver Seitenwechsel und bestätigtes Speichern bleiben Teil der weiteren
Prüfung; diese 40 Bilder ersetzen dafür keine Abschlussbelege. Als Nächstes
folgen getrennte Datei-/Restore-Seiten, danach die Leistenpreview.

### R6 – dritter Teilschritt: Dateien und Wiederherstellung

**Status:** in den dokumentierten Zuständen visuell geprüft und installiert.
V10 ist für diese Fälle belegt; lange Inhalte, laufende Wiederherstellung,
Abbruch, externe Revisionskonflikte und explizites Wiederöffnen fehlender Apps
bleiben für R9 offen. R6 insgesamt bleibt bis zur Leistenprüfung offen.

Ursache: Beide Tabs verwendeten dieselben sechs Layoutaktionen und denselben
Dateipfadeditor. Fehlendes Layout erschien als roher OS-Fehler, leere Zeilen
und unsichtbare Controls blieben per Tastatur erreichbar. Vorherbilder des
installierten `8a61bbc…` liegen in `evidence/r6/layout-before/`.

Geänderte Dateien und Zweck:

- `configuration/restore.rs`, neue `restore/draw.rs` und `restore/tests.rs`:
  getrennte Seiten, native Controls, lesbarer Leer-/Fehlerzustand und echte
  Ergebniszeilen. Gemeinsame Geometrie begrenzt Seiten auf sechs tatsächliche
  Zeilen; Mindestcanvas verwendet drei. Fokus und Hitprüfung folgen derselben
  sichtbaren und verfügbaren Controlmenge.
- `room_editor.rs`, `wayland/layout_restore.rs` und `layout_restore/input.rs`:
  Ladezustand, eindeutiger Seiteneinstieg, separate Bestätigung über bestehende
  typisierte/revisionierte IPC. Relative Dateipfade werden vor dem Senden
  abgelehnt; Auswahl, Ergebnisliste und korrigierbarer Entwurf bleiben erhalten.
- `wayland/state/room_form.rs`, `configuration.rs`, `configuration/chrome.rs`,
  `room_management_view/header.rs` und `configuration/tests.rs`: passender
  Tabfokus, keine zweite Fokusmarkierung, klare Bestätigungs-/Entwurfsgrenzen.
- `niwoe-tokens/src/control_center.rs`: zentrale Höchstzahl sichtbarer Zeilen.

Performance: maximal sechs sichtbare Zeilen aus den bereits begrenzten
Layoutresultaten; keine neue Datei-/Icon-Ladeschleife, kein Bildcache, Timer
oder regelmäßiger Capture. Layoutspeichern, Dateiverweise und Anordnen nutzen
die vorhandenen Services. Anordnen öffnet keine zusätzlichen Anwendungen.

Alle finalen Fedora-Gates haben Exitcode 0: `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1254 bestanden, 0
fehlgeschlagen, ein bekannter DBus-Test ignoriert), Design Guard, Clippy mit
`--workspace --all-targets -- -D warnings` und Releasebuild der Shell.
Alle 94 übertragenen Rust-Dateien stimmen nach LF-Normalisierung überein.
Logs unter `evidence/r6/layout-gates/` erhalten auch den ersten fehlgeschlagenen
Test: dessen Erwartung verbot einen an dieser Stelle legitimen Restore-
Ergebniszeilenhit. Die Erwartung wurde auf den tatsächlich verbotenen
Dateieditorhit korrigiert und anschließend vollständig neu geprüft.

Erster geprüfter Zwischenstand `c75ef249…` wurde installiert; seine beiden
angesehenen Leerzustandsbilder liegen ausdrücklich als Zwischenstand unter
`layout-empty-initial/`. Die zusätzliche lokale Pfadprüfung wurde danach
erneut vollständig geprüft. Finaler SHA256 für Release, Installation und
laufende Shell:
`f3881dcfc6b2c392f5bf8457cca83c4c788a77036cc89f91782a5a68a4cb1981`.
Watchdog-Aktivierung PID 257251 → 260340; kein Neulogin erforderlich.

**56 finale Aufnahmen tatsächlich angesehen:** 16 Livezustände plus 40 Bilder
auf beiden primären Outputs mit 100 % und Skala 1,406, logische Flächen
1920×1032 bzw. 1366×720 bei physischem FHD. Nachgewiesen wurden eigener
gespeicherter Raum mit Ordnersymbol und getrennten Native-/XWayland-Apprefs,
fehlendes Layout, Tastatureinstieg, Layoutspeichern mit sieben echten GTK-
Testfenstern (vier Wayland, drei X11), Paging, Dateiverweisauswahl, ungültiger
Pfad mit erhaltenem Entwurf, bestätigtes Speichern und Entfernen eines eigenen
absoluten Dateiverweises sowie Anordnen aller sieben vorhandenen Fenster ohne
Appstart. Der versteckte Dateieditor ist auf Restore nicht bedienbar.
Bilder, Skripte und Rohdaten: `layout-live/` und `layout-matrix/`.
Die skalierte Rasterweichheit V23 bleibt offen.

Die Größenmatrix hat Konfigurationsbytes sowie vollständige Raum- und
Output-Snapshots exakt wiederhergestellt, Shell-PID danach 263692. Anschließend
wurden ausschließlich eigener Testraum, eigenes Layout und eigene Prozesse
entfernt. Alle neun ursprünglichen Raumeinträge bleiben semantisch identisch;
Raumrevision 815 → 817 ist die erwartete Folge von Anlegen und Löschen, keine
behauptete Bytegleichheit. Konfiguration und Outputs sind identisch, Fenster
leer, eigener Raum entfernt. Als Nächstes folgt die Leistenseite.

### R6 – Leistenprüfung, erster installierter Stand

**In Arbeit, keine visuelle Freigabe.** Vorherbilder unter `evidence/r6/panel-before/`
zeigen verkleinerte Preview und Pseudo-Checkboxbuttons. Die drei korrigiert
benannten Vorheraufnahmen wurden angesehen; Konfigurationsbytes, vollständige
Raum-/Output-Snapshots und Fensterzustand sind nach Abbruch identisch.
Der erste Vorherlauf stoppte vor Seiteneingriff wegen eines bereits vorhandenen
Evidenzverzeichnisses; der zweite verwendete einen eigenen Namen.

Erster geprüfter und installierter SHA256:
`4d43eeb26929d447266425096563d1ff9eed69d62292e5eddf952f5f3e83d1d8`.
Alle 99 Rust-Dateien stimmen überein. Finale Gates dieses Zwischenstands:
Fmt, Workspace-Check/-Tests (1260 bestanden, 0 Fehler, 1 bekannter DBus-Test
ignoriert), Design Guard, Clippy und Release jeweils Exitcode 0. Watchdog
263692 → 287032; Release-, Installations- und laufender Hash identisch.

Sieben tatsächliche Aufnahmen dieses Stands wurden angesehen und erhalten
unter `panel-live-initial/`. Native Entwürfe speichern nicht automatisch;
Speichern bestätigt die eigene Moduländerung (Revision 1), Wiederöffnen zeigt
sie. Rücksetzen der ursprünglichen Reihenfolge und Speichern bestätigt Revision
2. Die ursprünglich fehlende `panel.toml` wurde ausschließlich als eigene
Testdatei nach Byte-/Eigentumsprüfung entfernt; Watchdog-Neustart lädt wieder
die ursprünglichen Defaultmodule bei Revision 0. Cleanup bestätigt identische
Configbytes, volle Raum-/Output-Snapshots, keine Fenster und ursprüngliche
Dateiabwesenheit, PID 287813.

**Neuer Befund V29:** In den 38er Modulcontrols berührt der gemeinsame
Fokusrahmen die zweite Textzeile. Der erste Stand erhält deshalb keine
visuelle Freigabe. Eine zentrale höhere Modulzeile mit Fontmetrik-/Fokus-
Abstandsregression wird vor der Größenmatrix erneut geprüft. Andere kompakte
zweizeilige Fokusrollen werden in R9 ausdrücklich erneut verglichen.

Prüfhistorie unter `panel-gates-initial/` erhält Compilerbefunde an Textrollen-
Signatur und Borrow-Grenze. Beim Quellenabgleich wurde außerdem erkannt, dass
übertragene ältere Dateizeiten Cargo einen vorhandenen Release weiterverwenden
lassen können. Vor den finalen Gates wurden daher die Zeiten aller übertragenen
Quellen aktualisiert; Check/Tests/Clippy/Release liefen erneut mit tatsächlicher
Neukompilierung. Der Zwischenstand vor dieser Korrektur wurde nicht installiert.

### R6 – Leistenkonfiguration, korrigierter installierter Stand

**Visuell geprüft in den folgenden Zuständen; aktuelle R6-Performance bestanden.**
Die zentrale Modulhöhe beträgt jetzt 50 statt 38 logische Pixel. Der Regressionstest
prüft die tatsächliche Höhe der gemeinsamen Textrollen einschließlich Abstand
zum inneren Fokusrahmen. Alle sieben finalen Livebilder und alle 40 Matrixbilder
wurden tatsächlich angesehen. Auf beiden Outputs mit 100 % und Skala 1,406 passen
Modulzeilen, Vorschau und Footer ohne Überschneidung; die bestehende Rasterweichheit
V23 bleibt ausdrücklich offen. Mindestcanvas ist 1366×720 bei physischem FHD,
kein behaupteter physischer 1366er DRM-Modus.

Geänderte Dateien und Zweck:

- `niwoe-tokens/src/control_center.rs`: zentrale Modulhöhe und maximale
  Previewbreite. `room_management_view/panel.rs` und neue `panel/tests.rs`:
  gemeinsame native Checkbox-/Text-/Fokusrollen, verständliche Modulnamen,
  echte Positionscontrols, ein gemeinsamer sichtbarer Hit-/Fokusumfang und
  lesbare Vorschau in Originalgeometrie. Save ist die eindeutige Hauptaktion;
  nicht verfügbare Positionsaktionen und ausstehende Saves sind nicht bedienbar.
- `room_editor_panel.rs`: gemeinsame deutsche Namen für Statussymbole und
  Systemstatus; vorhandenes Modulmodell und Speicherprotokoll bleiben erhalten.
- `icons/cache.rs`: Generation ändert sich bei neuer/geladener Iconquelle,
  nicht beim Lookup oder erneutem Warmen eines vorhandenen Schlüssels.
- Neue `wayland/panel_preview.rs`: einzelner Cacheeintrag mit typisiertem
  Schlüssel für Breite, Raum-/Belegungsdaten, Uhr, Netzwerk, Audio, Tray,
  Akku, Module, Theme und Icongeneration. Tests prüfen Arc-Identität bei Hits,
  Invalidierung, Ersetzen, Freigeben und Breitenbegrenzung vor Allokation.
- `wayland/mod.rs`, `shell.rs` und `init/shell_state.rs`: internes Cachemodul
  und dessen Lebensdauer im vorhandenen Shellzustand.
- `wayland/state/panel_form.rs`: vorhandenen Panelrenderer auf Cachemiss
  verwenden; Save/Cancel und Tastaturfolge respektieren dieselbe Verfügbarkeit.
  `state/room_form.rs` übergibt den tatsächlichen Entwurfszustand an Hitprüfung.
- `wayland/render/core.rs`: Screenshoticon über dieselbe vorhandene Quelle
  wie die reale Leiste beziehen; enge Iconcache-Borrow-Grenze erhalten.
- `wayland/render/launcher.rs` und `handlers/layer.rs`: Arc-Preview verwenden;
  Cache bei Abbrechen, erfolgreichem Verlassen, Unmapping und geschlossenem
  Layer freigeben. Wizard behält bis R8 seinen bestehenden Previewpfad.

Performance-Modell: genau ein dauerhaft gehaltener Previeweintrag, maximal
4096×48×4 = 786432 Bytes (0,75 MiB). Bei FHD sind es 1632×48, am Mindestcanvas
1098×48. Der native Renderer und die BGRA-Konvertierung laufen nur bei geänderten
gemalten Eingaben; Cachehits klonen nur den Arc. Alte und neue Raster können bei
Ersetzen vorübergehend zugleich existieren; die Cachegrenze ist keine Behauptung
über den gesamten transienten Prozessspeicher. Keine neue Icondecodierung,
Dateisuche, Frame-/Capture-Schleife oder Timer. Größere Flächen werden vor
Allokation begrenzt und ausdrücklich als Originalgrößen-Ausschnitt bezeichnet.

Verifikation: LF-Abgleich aller 99 Rust-Quellen; Quelldateizeiten vor Cargo
aktualisiert und tatsächliche Neukompilierung in den Logs belegt. `cargo fmt
--all -- --check`, `cargo check --workspace`, `cargo test --workspace` (1260
bestanden, 0 Fehler, 1 bekannter DBus-Test ignoriert), Design Guard, Clippy
`--workspace --all-targets -- -D warnings` und Releasebuild jeweils Exitcode 0.
Manuell geprüfter Call-Flow: Sidebar → Entwurf → Maus/Tab/Backtab → korrelierter
Save über bestehende IPC → bestätigter Zustand → Wiederöffnen bzw. Cancel →
gespeicherter Zustand; Cachefreigabe auch beim tatsächlichen Unmapping.
Logs und Quellenabgleich unter `evidence/r6/panel-gates/`.

Release-, Installations- und laufender SHA256:
`f52fd60635b2fc60fb9c2e254b6a61655e22847d89318f0a57d6159bb73c06a0`.
Watchdog-Aktivierung 287813 → 294734, kein Neulogin.

Native Liveprüfung unter `panel-live/`: Screenshotmodul ausschalten, ohne
automatisches Speichern; explizites Speichern bestätigt Revision 1 und
Wiederöffnen zeigt den gespeicherten Zustand. Wieder einschalten und zweimal
früher verschieben stellt die ursprüngliche Reihenfolge her; explizites
Speichern bestätigt Revision 2, erneutes Öffnen bestätigt alle vier Module.
Die ursprünglich fehlende `panel.toml` wurde nur nach exakter Prüfung der eigenen
zuletzt gespeicherten Bytes entfernt. Watchdog-Neustart lädt Originaldefaults
mit Revision 0. Alle sechs Cleanup-Prüfungen grün, PID 295382.

Die vier Matrixgruppen unter `panel-matrix/` prüfen jeweils Default,
unverfügbare erste Positionsaktion, einzeln ausgeschaltetes Screenshot-/Suchmodul,
alle optionalen Module aus, Tastaturfokus Save, Systemstatus per Tastatur an,
Statussymbole im Entwurf umordnen, Abbrechen und Originalmodule erneut öffnen.
Alle 40 Bilder angesehen. Keine dieser Matrixaktionen speichert Daten. Danach
Config-/Panelbytes, komplette Raum-/Output-Snapshots und leerer Fensterzustand
exakt gleich, alle fünf Cleanup-Prüfungen grün, Shell-PID 297317.

Offen für R9 bleiben ausstehende/fehlerhafte bzw. konkurrierende Live-Saves,
lange Namen, Ultrawide-Ausschnitt und die kompakte Fokusrolle anderer Seiten.
Die Prüfungen ersetzen keine Nutzerabnahme oder Gesamtfreigabe. Vor R7 folgt
die laufende aktuelle R6-Messreihe mit 40 Bedienzyklen und 3×300 s Leerlauf.

### R6 – aktuelle Performance und Übergang zu R7

**40 Bedienzyklen und alle drei Ruheproben bestanden, Exitcode 0.** Installierter
und laufender Stand bleiben `f52fd606…`, Shell-PID 297317. Gleiche ursprüngliche
Outputs: drm-0 FHD/60 Hz, drm-1 UHD/30 Hz, beide Scale 1; keine Anwendungsfenster.
Jeder Zyklus öffnet Verwaltung/Konfiguration, besucht Apps/Dateien/Restore,
ändert zwei Panelmodule nur im Entwurf und bricht vollständig ab. Nach Zyklus
0/20/40 bleiben Config- und Panelbytes sowie vollständige Snapshots identisch.
Nach 30 Sekunden Warm-up folgen drei 300-Sekunden-Idle-Proben ohne GUI-/Build-
Aktivität. Die Prozessidentitäten bleiben unverändert; auch danach alle
Zustands-/Konfigurationsprüfungen grün. Rohdaten und Helfer: `evidence/r6/performance/`.

| Probe | Shell-CPU, % eines Kerns | Compositor-CPU, % eines Kerns | GPU-Engine-Zeit, % | Shell-RSS vorher → nachher, Bytes | Compositor-RSS, Bytes |
| --- | --- | --- | --- | --- | --- |
| 1 | 0,193333 | 1,086666 | 0,007781 | 130584576 → 130715648 | 175439872 → 175439872 |
| 2 | 0,183333 | 1,096666 | 0,007999 | 130715648 → 130715648 | 175439872 → 175439872 |
| 3 | 0,180000 | 1,099999 | 0,007288 | 130715648 → 130715648 | 175439872 → 175439872 |

CPU aus `/proc/<pid>/stat` mit `SC_CLK_TCK` und realer monotoner Messdauer;
RSS aus dem vorhandenen Messwerkzeug. GPU ist ausschließlich der zugängliche
Compositor-Engine-Zähler, keine vollständige GPU-Auslastung. CPU-Mediane
0,183333/1,096666 liegen gegenüber R1 (0,183329/1,063308) um weniger als
0,000005 bzw. 0,033358 Prozentpunkte höher, deutlich innerhalb des Budgets von
0,5. Gegenüber R5: praktisch gleiche Shell-CPU, Compositor +0,026667 Punkte.
Repaint-/End-to-End-Latenzmetriken sind in dieser Serie nicht gemessen.

RSS bei 0/20/40 Zyklen: Shell 31031296/130584576/130584576 Bytes,
Compositor 127762432/175439872/175439872 Bytes. Nach dem Aufwärmen kein Aufbau
zwischen 20 und 40; die erste Ruheprobe bringt einmalig 128 KiB Shell-RSS,
danach konstant. Der in R5 beobachtete zusätzliche Compositor-Aufbau wurde in
dieser Serie nicht wiederholt; die längere Lebensdauerprüfung bleibt in R9.
V29 ist für die Panelmodule visuell behoben, die kompakte Fokusrolle anderer
Seiten bleibt in R9 zu prüfen. R6 ist für den dokumentierten Umfang umgesetzt,
geprüft und installiert. Die genannten zusätzlichen Live-/Fehlerfälle bleiben
Pflichten von R9. Als Nächstes R7, danach R8 und R9; keine Gesamt-/Nutzerfreigabe.

### R7 – Inventur des installierten Stands vor der Reparatur

**In Arbeit.** Alle 13 Vorherbilder des installierten `f52fd606…` wurden
angesehen: zwölf reguläre Settings-Inhalte über tatsächliche Sidebar-/Tab-
Navigation, zusätzlich Standard-Apps über die Suche. Die Konfigurations-,
Panel- und MIME-Zuordnungsdateien bleiben bytegenau gleich; vollständige Räume
und Outputs identisch, keine Fenster. Native Screenshotzustimmung verwendet,
keine Schutzfläche aufgehoben. Bilder, Helfer und Cleanup unter `evidence/r7/before/`.

V12/V13/V16 sind erneut sichtbar bestätigt: Wallpaper zeigt Pack-Screenshots
als zusätzliche Hintergründe und bei Altai eine Portraitthumbnail; Modus-/
Dateiauswahl auf Englisch. Cursor listet Adwaita/Breeze nur als technische Namen
und ohne Bildvorschau. Audio/Drucker haben englische, technische Statuszeilen;
Updates nennt auf tatsächlichem Fedora 44 „apt nicht verfügbar“. Die Anzeige-
Seite zeigt rohe Positions-/Scale-/Raum-/ID-Daten und einen Textpfeil. Bei
Standard-Apps bleibt eine nicht aufgelöste gespeicherte Desktop-ID sichtbar.
Energie gewichtet alle Sitzungsaktionen gleichzeitig als Akzentaktion.

**Neuer Befund V30:** Tippen von „standard“ aus System zeigt bereits Apps samt
korrekter Sidebar, bleibt aber bei „Lade installierte Anwendungen …“. Derselbe
Provider lädt beim anschließenden direkten Sidebar-Einstieg korrekt. Ursache:
Suche wechselt nur die effektive Renderkategorie; sie fordert den zugehörigen
Worker nicht an. Workerabschlüsse vergleichen außerdem die gespeicherte statt
der effektiven Kategorie. Der gemeinsame Daten-/Renderablauf wird in R7
korrigiert, bevor diese Suche als funktionsfähig bewertet wird.

Als erster kleiner Schritt werden die unveränderten Settings-Refreshmethoden
und Settings-Tokens aus fast vollen Dateien in eigene Verantwortungsmodule
verschoben. Kein neuer Provider, Daten-/Designwert oder Navigationsweg; Gates
und tatsächlicher Release folgen vor den fachlichen Reparaturen.

### R7 – kleiner geprüfter Modulsplit

**Geprüft/installiert, ohne UI-/Providerverhaltensänderung.**
`wayland/state/timers.rs` gibt die zwei bestehenden Settings-Refreshmethoden
unverändert an das neue `state/settings_refresh.rs` ab; `state.rs` bindet das
Modul ein. `niwoe-tokens/src/chrome.rs` gibt die unveränderte Settings-Struktur
an `src/settings.rs` ab und exportiert sie weiter am bisherigen Pfad; `lib.rs`
bindet das neue Tokenmodul ein. Bestehende öffentliche Pfade bleiben gültig;
alle Dateien unter 600 Zeilen. Keine neuen Designwerte oder Timer.

102 LF-Quellen abgeglichen, Zeiten vor Cargo aktualisiert. Fmt, Workspace-Check,
Workspace-Tests (1260 bestanden/0 Fehler/1 bekannter DBus-Ignore), Design Guard,
striktes Clippy und tatsächlicher Shell-Release jeweils Exitcode 0.
Belege: `evidence/r7/state-gates/`. Release, Installation und laufende Shell
SHA256 `0d02f2003a3e2ddbfa86119951a49a93b45940294e9b5cd1e87fa50d96952a56`;
Watchdog 297317 → 309542, kein Neulogin. Kein visueller Mangel dadurch geschlossen.
Als nächster Fachschritt wird V30 im gemeinsamen Such-/Workerablauf korrigiert.

### R7 – V30, erster geprüfter Suchfix und fehlgeschlagene Live-Versuche

**Geprüft/installiert, visuelle Nachprüfung noch offen.**
`state/control_center.rs` bestimmt die effektive Kategorie an einer Stelle und
fordert nur beim tatsächlichen Such-Kategoriewechsel den vorhandenen Worker an.
`state/settings_refresh.rs` invalidiert die effektiv sichtbare Seite bei dessen
Abschluss. `handlers/keyboard.rs` führt Text, Backspace und Search-Escape über
diesen gemeinsamen Ablauf; `keyboard/control_center_navigation.rs` lässt Space
bei fehlendem Controlfokus an die Suche weiter. Keine neuen Timer oder Provider;
vorhandene Requests pro Kategorie weiter durch Inflight-Set zusammengefasst.

102 LF-Quellen identisch, Zeiten vor Cargo aktualisiert. Fmt, Workspace-Check,
Workspace-Tests (1260/0/1 bekannter DBus-Ignore), Design Guard, striktes Clippy,
Shell-Release jeweils Exitcode 0. Release/Installation/laufender SHA256
`9d56d98fe94a076579bb604f9c826fc7177103074989bca885a9cff215f3c5e7`;
Watchdog 309542 → 316242. Belege unter `evidence/r7/search-gates/`.

Erster Livehelfer scheitert bei Texteingabe an seinem fehlenden Leerzeichen-
Keymapping, Exitcode 1. Sechs vorherige Bilder wurden angesehen und zeigen
nur Wallpaper, keine Settings. Korrigierter Helfer mit gleicher unmittelbarer
Watchdogfolge endet zwar mit exaktem Cleanup, aber auch seine sieben angesehenen
Bilder zeigen nur Wallpaper: **kein bestandener UI-Funktionsnachweis**. Daten,
Bilder und Skripte unter `search-initial/` und `search-second/` bleiben erhalten.
Zweiter Watchdog 316242 → 317112, derselbe Hash. Ursprüngliche Config-/Panel-/
MIME-Bytes und vollständige Räume/Outputs in beiden Läufen gleich; keine Fenster.

Die Diagnose mit der inzwischen laufenden Shell bestätigt einen sichtbaren
System-Einstieg; das tatsächlich angesehene Bild liegt in `search-diagnostic/`.
Ein Diagnoseversuch verwendete zuvor einen nicht unterstützten CLI-Modus des
Helfers und stoppte vor dem Einstieg. Die Skripte müssen nach Watchdogaktivierung
auch die Bereitschaft der Shell beachten, nicht allein den PID-/Hashwechsel.
V30 bleibt bis zur erneuten echten Bild-/Bedienprüfung offen; die unmittelbare
Neustartfolge wird in R9 zusätzlich geprüft. Kein behaupteter erfolgreicher
Provider-/Pickerzustand aus diesen Fehlversuchen.

### R7 – tatsächliche Live-Nachprüfung des Suchfixes

**V30 im dokumentierten Einstieg visuell geprüft.** Alle sieben finalen Bilder
unter `evidence/r7/search-live/` wurden angesehen. Bei laufender, initialisierter
Shell lädt die erste Suche „standard“ aus System die echten neun Standard-App-
Kategorien; zuvor blieb dieser Einstieg ohne Daten. Maus öffnet den echten
Firefox-Picker, Escape schließt ihn und ein weiteres Escape leert die Suche
und zeigt System. „wlan“ lädt den echten Netzwerkstatus samt Profilen.
Leerzeichen bleibt bei fehlendem Controlfokus in „standard app“ erhalten.
Diese Mehrwortsuche findet derzeit keine Kategorie und zeigt System; das Bild
ist kein Nachweis einer passenden Mehrwortsuche.

Die drei Tab-Schritte im Picker erreichen tatsächlich die Apps-Navigation,
nicht den Kandidaten. Kandidaten-Tastaturbedienung ist damit **NOT RUN**.
Das zusätzliche goldene Auswahlrechteck an der geöffneten Kategorie wird in
R7 zusammen mit den Standard-App-Zeilen geprüft; es ist kein bestandener
Nachweis einer eindeutigen Fokusrolle. Beide ursprünglichen MIME-/Config-/
Panel-Dateizustände bleiben bytegenau erhalten, vollständige Räume und Outputs
gleich, keine Fenster. Helfer Exitcode 0, laufender SHA256 weiterhin
`9d56d98fe94a076579bb604f9c826fc7177103074989bca885a9cff215f3c5e7`.
Keine Nutzer-/Gesamtfreigabe. Weitere Skalierungs-, Picker- und Neustartfälle
bleiben offen; als Nächstes die native Assetdarstellung in R7.
### R7 – erster installierter Cursorstand, Bildprüfung noch nicht bestanden

**Geprüft/installiert, V13 weiter offen.** `cursor/preview.rs` mit getrenntem
Xcursor-Reader und Tests liest echte statische Erstbilder nach dem bestehenden
Compositor-Lookup (left_ptr/default/arrow, Theme-Inheritance, erste nächste
Nominalgröße). Lokalisierter Katalogname stammt aus `index.theme`, Deutsch vor
Name. Die Shell gibt Cache/Anfrage über Settings-Worker und den gemeinsamen
Control-Center-Baum weiter; Cursorgrößenänderung fordert einen neuen Worker an,
alte Ergebnisse werden am Themen-/Größenschlüssel verworfen. Zwei native
Katalogspalten und Größencontrols beziehen Geometrie ausschließlich aus
`niwoe-tokens::Settings`; Haupt-/Unterzeile nutzt die gemeinsame Textrolle.

Cache: höchstens zwölf Arc-Pixmaps, je 128×128, maximal 768 KiB; höchstens alter
und neuer Satz während einer Anfrage, Eingabedatei höchstens 4 MiB, Metadaten
16 KiB, TOC 4096, Inheritance acht besuchte Themen. Dateiidentität aus Pfad,
Länge und Änderungszeit sowie angeforderter Größe; unveränderte Bilder teilen
Arc. Metadaten-/Bild-I/O ausschließlich bei Einstieg oder Größenwechsel im
vorhandenen einmaligen Worker, keine Animation und kein neuer Timer. Cache
lebt mit der Shell. Unterstützte Theme-Wurzeln entsprechen dem Compositor.

108 LF-Quellen identisch, Zeiten vor Cargo aktualisiert. Fmt/Workspace-Check/
Tests (1264/0/1 bekannter DBus-Ignore)/Design Guard/striktes Clippy/Release
Exitcode 0. Release/installiert/laufend SHA256
`f3ab9ce86514fdb78bf8b582e0cba6fba67de356cddc15f66ae5325b5c7af2b1`,
Watchdog 317112 → 333574. Kein Neulogin. Vorheriger Compiler-Typbefund und die
spätere unbenutzte Altkonstante sind korrigiert, Fehlprotokolle erhalten unter
`evidence/r7/cursor-gates/`. Ein Transferversuch erstellte wegen falschem
Arbeitsverzeichnis kein neues Paket; die Wiederholung wurde ausdrücklich im
Projekt ausgeführt und die tatsächliche LF-Identität erneut geprüft.

Sieben erste sowie zehn zweite Livebilder wurden tatsächlich angesehen;
`cursor-initial/` bzw. `cursor-second/`. Erster Tastaturhelfer erwartete nach
Mausauswahl fälschlich Fokusfortsetzung und scheiterte: Tab startet tatsächlich
an der Sidebar. Zweiter Helfer nutzt F8 und den vorhandenen Tabpfad; Theme-
Auswahl von Adwaita über Breeze Hell zu Breeze Dunkel ist bestätigt, alle vier
Größen gespeichert. Config/MIME/Panel bytegenau wiederhergestellt, vollständige
Räume/Outputs gleich, keine Fenster und nur eigene Themefixture entfernt.

**Bildprüfung nicht bestanden:** „Sehr groß · 48 px“ ist gekürzt. Bei 48 px
stehen verkleinerte Breeze-Bilder außerhalb ihrer Vorschaufläche, weil die
Pixmap-Translation vor der Skalierung angewendet wird. Die getestete leere
Fixture erbt zudem legitim das Default-Bild und ist kein Fehlerzustandsnachweis.
Als Nächstes zentrale Knopfbreite, gemeinsame native Bildpositionierung samt
Bildgrenzen-Regressionsfall und eine explizit ungültige eigene Xcursor-Datei.
Kein Cursor-, R7- oder Gesamtabschluss aus diesen beiden Läufen.
### R7 – korrigierter Cursorstand, zehn finale Livebilder angesehen

**Im dokumentierten Liveumfang visuell geprüft; Matrix noch offen.**
`niwoe-ui/src/effect/image_contain.rs` mit Export in `effect/mod.rs` positioniert
gecachte Bilder proportional innerhalb ihrer tatsächlichen Zielgeometrie;
Translation nach der Skalierung. Ohne Decoder/Allokation, Animation oder neuen
Cache beim Zeichnen. Ein Pixelgrenzen-Regressionsfall prüft verkleinerte und
native Bilder an einem von Null entfernten Ziel. Cursor verwendet diesen
Helfer, die zentrale Knopfbreite in `niwoe-tokens/src/settings.rs` beträgt 160.
Keine Änderung am Compositor-/Glaspfad oder der Renderreihenfolge.

109 LF-Quellen identisch, Zeiten vor Cargo aktualisiert; Fmt, Workspace-Check,
Workspace-Tests (1265/0/1 bekannter DBus-Ignore), Design Guard, striktes Clippy,
Shell-Release Exitcode 0. Belege `evidence/r7/cursor-final-gates/`.
Release/installiert/laufend SHA256
`f4f0b67555e5c7450a54c703d4e3523bc16feeb2cbaba3fe27bd8af4c0cd1e9b`,
Watchdog 335449 → 342292; kein Neulogin.

Alle zehn Bilder unter `evidence/r7/cursor-live/` tatsächlich angesehen:
vier Größen samt vollständigem „Sehr groß · 48 px“, echte Adwaita-/Breeze-
Bilder bleiben in ihren Katalogflächen, lesbare deutsche Namen. Mausauswahl
Adwaita, F8-/Tab-/Enter-Auswahl Breeze Hell und Breeze Dunkel, ein gemeinsamer
Goldfokus und davon getrennte Auswahlfläche. Eigene ungültige Xcursor-Datei
zeigt „Vorschau nicht verfügbar“ und wird nur fokussiert, nicht ausgewählt.
Helfer Exitcode 0. Config/MIME/Panel bytegenau gleich, vollständige Räume und
Outputs identisch, keine Fenster; nur die eigene geprüfte Fixture entfernt.
End-PID 343033. Zwölf Themen, lange Namen, Wiederöffnen und alle vier Scale-
Werte auf beiden Outputs werden anschließend in der nativen Matrix geprüft;
diese ist noch nicht bestanden. Kein R7-/Gesamtabschluss.
## R7 – Cursor-Matrix auf beiden Outputs vollständig angesehen

Der installierte Cursorstand `f4f0b67555e5c7450a54c703d4e3523bc16feeb2cbaba3fe27bd8af4c0cd1e9b`
wurde mit zwölf Einträgen (drei reale Themes und neun eigene, anschließend entfernte
Testeinträge) auf `drm-0` und `drm-1` bei 100 %, etwa 140,6 %, 150 % und 200 % geprüft.
Alle **32 Aufnahmen wurden einzeln angesehen**: Katalog mit langen Namen und defektem
Zeigerbild, Tastaturfokus auf dem letzten Eintrag, tatsächliche 48-px-Vorschauen und
Wiederöffnen nach gespeicherter Größenwahl. Alle zwölf Einträge bleiben erreichbar,
Vorschauen bleiben in ihrer Zeile, Fehlertext und Größenbeschriftungen sind sichtbar.
Die allgemeine Rasterweichheit V23 bei hoher Skalierung bleibt ausdrücklich offen.
Dies bestätigt den dokumentierten Cursorumfang von V13; keine Gesamtfreigabe R7/R9.

Bilder, Helper, Protokoll und acht Fallbeschreibungen:
[evidence/r7/cursor-matrix](evidence/r7/cursor-matrix/).
Der Helper endete mit Exit 0; `cleanup.json` bestätigt bytegleiche Config/MIME/Panel,
identische Räume und Outputs, keine Fenster und Entfernung ausschließlich eigener
Fixtures. Laufende Shell nach Bereinigung: PID `353739`, Hash wie oben. Originale
Monitorauflösungen, Primärausgabe und Skalierung sind wiederhergestellt. Keine neue
Rust-Änderung oder erneute Performance-Abnahme innerhalb dieser Bildmatrix.
## R7 – Wallpaper-Katalog getrennt korrigiert und installiert

Status: Katalogquelle geprüft/installiert und in vier aktuellen Vorher-/Nachherbildern
angesehen; **V12 bleibt in Arbeit**. `niwoe-config/src/config.rs` dokumentiert die
Paket-/Einzelbildidentität; `config/mutation.rs` verwendet den begrenzten Scanner;
`config/wallpapers.rs` verliert die alte ordnerweise Dateigrößenauswahl.
Neu: `config/wallpaper_catalog.rs` und `config/wallpaper_catalog/tests.rs`.
Verifizierte KDE-Pakete liefern ein Bild (größtes benanntes Querformat nach Pixelzahl),
keine zusätzlichen Präsentations-Screenshots. Andere Fotos bleiben getrennte Einträge.
Explizite benutzerdefinierte oder fehlende aktuelle Pfade bleiben im begrenzten Katalog
sichtbar. Scanbudget: 4096 Einträge, fünf Ebenen, maximal 40 Auswahlziele.
Keine neue Dependency oder Änderung eines Cargo-Manifests.

114 LF-Quellen stimmen mit Fedora überein. `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1270 bestanden, 0 fehlgeschlagen,
1 bestehender D-Bus-Test ignoriert), `cargo test -p niwoe-tokens --test design_guard`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo build --release -p niwoe-shell`: jeweils Exit 0. Der anfänglich falsche Pfad
des Testmoduls wurde vor diesen erfolgreichen Prüfungen korrigiert.
Release/installiert/laufend stimmen überein:
`3c75eee69da663edbeb11970d5ea9e138452cc6e98d00702a6ee8638572294da`,
Watchdog PID `353739` → `355130`; kein Neulogin erforderlich.

[evidence/r7/wallpaper-catalog](evidence/r7/wallpaper-catalog/) enthält Gates,
Aktivierung, Helfer und alle vier angesehenen Bilder. Altai zeigt nach dem Laden
jetzt das passende Querformat; Paket-Screenshotduplikate und Auflösungsnamen sind
verschwunden. Die erste Nachheraufnahme dokumentiert den noch unbeschrifteten
Ladezustand der alten UI, die zweite die fertig geladenen echten Vorschauen.
Vorher und nachher bestätigen bytegleiche Config/MIME/Panel, identische Räume/Outputs,
keine Fenster. Keine Auswahl oder Änderung eines Nutzer-Wallpapers vorgenommen.
Offen bleiben Arc-Cache/Invalidierung, deutsche Controls, Lade-/Fehlerrollen,
Erreichbarkeit des gesamten Katalogs und die vollständige Ausgabematrix.
## R7 – native Wallpaper-Oberfläche mit Arc-Cache geprüft und installiert

Status: technisch geprüft/installiert; die ersten zwei Bilder angesehen. Die laufende
Funktions-/Fehlerrunde und vollständige Monitor-/Skalierungsmatrix sind noch offen.
Keine Gesamtfreigabe V12/R7. Installierter/laufender Release:
`f04f2dfc223a94f7e258f880d147b3a2057ab2a4c254060c5ab7d90c083224c6`,
Watchdog `355130` → `368817`. Kein Neulogin erforderlich.

Geänderte Dateien dieses abgegrenzten Schritts:

| Datei | Änderung |
| --- | --- |
| `niwoe-shell/src/settings_refresh/wallpaper.rs`, `wallpaper/tests.rs` | Neuer begrenzter Decoder-/Metadatencache im bestehenden Einmal-Worker; Arc-Wiederverwendung und Invalidierung geprüft. |
| `settings_refresh.rs`, `wayland/state/settings_refresh.rs` | Katalog beim Seiteneinstieg aktualisieren; Cache übernehmen, veraltete Ergebnisse verwerfen und betroffene Auswahlziele zurücksetzen. |
| `settings_view/content/wallpaper.rs`, `wallpaper_widgets.rs` | Native Vorschaukarten mit gemeinsamen Textrollen, ehrlichen Lade-/Fehlertexten, deutsche Modi/Dateiauswahl und Katalogseiten. |
| `settings_view/option_widgets.rs`, `content/cursor.rs` | Vorhandene Cursor-Chips in einen gemeinsamen Adapter für bestehende native Component-Chips und sekundäre Aktionen ausgelagert; Cursorverhalten unverändert. |
| `settings_view/appearance_widgets.rs`, `basic_widgets.rs`, `settings_view.rs` | Alte Wallpaper-Zeilen und lokale Geometriekonstanten entfernt; neue Module angebunden. |
| `niwoe-tokens/src/settings.rs` | Vorschaugröße 112×63, Zeilen-/Raster-/Controlgeometrie zentral festgelegt. |
| `settings_view/content_builders.rs`, `draw.rs`, `wayland/state/control_center.rs`, `wayland/render/launcher.rs` | Interne Cache-/Pagingparameter gemeinsam weitergereicht; Suche setzt die Katalogseite zurück. |
| `wayland/shell.rs`, `wayland/init/shell_state.rs`, `state/popups.rs` | Cachezustand und temporäre Katalogseite im Shellzustand; neue Einstiege beginnen auf Seite 1. |
| `widget_action.rs`, `wayland/handlers/widget_dispatch/dispatch.rs` | Seitenwechsel und erreichbare Fokusziele an bestehende Eingabewege angebunden. |
| `wayland/state/shell_actions.rs`, `state.rs`, `wayland/mod.rs` | Vorschauen nach bestätigter Bildwahl aktualisieren; alten unbeschränkten Vec-Decoder entfernt. |
| `settings_view/layout_tests.rs` | Alle 40 Wallpaperziele über sämtliche Seiten an 1920×1032 und 1366×720 auf Tastatur und Pointer geprüft; vorhandene App-/Cursorprüfung erhalten. |

120 LF-Quellen auf Fedora hashgleich. `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (**1275 bestanden, 0 Fehler,
1 bestehender D-Bus-Test ignoriert**), `cargo test -p niwoe-tokens --test design_guard`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo build --release -p niwoe-shell`: alle Exit 0. Erstlauf: Check/Tests/Guard grün,
Clippy beanstandete drei Iteratorstellen; diese wurden korrigiert und der gesamte
Lauf wiederholt. Der Erstlauf bleibt erhalten.
[evidence/r7/wallpaper-gates](evidence/r7/wallpaper-gates/).

Performance-Modell: einmaliger vorhandener Hintergrundworker bei Seiteneinstieg,
keine neue Frame-/Animations-/Polling-Schleife. Cache-Schlüssel: Pfad, Dateigröße,
Änderungszeit und Zielgröße; Lebensdauer Shell. Maximal 40×112×63×4 = 1.08 MiB
Rasterdaten, bei gleichzeitigem altem/neuem Cache höchstens 2.16 MiB. Widgetbäume
teilen Arcs statt Bildpixel zu kopieren. Eingabedatei maximal 32 MiB; Bilddimension
maximal 8192 je Achse; 128 MiB Decoderbudget ist laut Decoder-API best effort,
kein garantierter Gesamt-RSS-/Scratchdeckel. Vier neue Cachetests belegen
Seitenverhältnis/Premultiplikation, unveränderte Arc-Identität, Größen-/Dateiwechsel,
Löschen/Neuanlegen, fehlende/defekte/zu große Dateien und Katalogbegrenzung.

Die erste aktuelle Aufnahme zeigt lesbar „Vorschau wird geladen …“, die zweite
die echten geladenen Bilder und die deutsche Dateiauswahl mit nativem Ordnersymbol.
39 ursprüngliche Bilder verteilen sich auf drei Seiten. Rahmen, Textpaare, native
Bildgeometrie und deaktiviertes „Zurück“ sind in diesen zwei Bildern geprüft.
Tatsächliche Seitenwechsel, letzte Auswahlziele, Dateiauswahl, Fehlerzustände und
Speichern werden erst nach abgeschlossener Live-Runde und Bildsichtung bewertet.

## R7 – Wallpaper-Bedienrunde, Dateiauswahl und ergänzter Release

Status: die dokumentierte Bedienrunde ist visuell geprüft; die vollständige
Monitor-/Skalierungsmatrix des ergänzten Releases läuft noch. R7 bleibt offen.

Alle 17 Aufnahmen des zweiten Laufs und seiner gezielten Dialogfortsetzung wurden
angesehen. Drei Katalogseiten mit 40 Einträgen, erste/letzte Bildkarte per Tastatur,
eigener defekter Eintrag mit „Vorschau nicht verfügbar“, langer eigener Bildname,
Mausauswahl, Füllen/Einpassen/Zentrieren/Kacheln, gespeicherte Auswahl nach erneutem
Öffnen und ersetzte Bilddatei mit erneuerter Vorschau sind belegt. Nach Entfernen
des eigenen Fehlereintrags zeigt die Oberfläche 39 Bilder. Ein bestätigter eigener
Pfad außerhalb des Katalogs wird aufgenommen; Abbrechen bewahrt die Auswahl.
Nach einer Auswahl kann der aktualisierte Katalog eine andere Reihenfolge haben:
ein vorher nur wegen seiner Auswahl enthaltener externer Pfad entfällt. Dann wird
die Seite bewusst zurückgesetzt. Die ersten Auswahl-/Modusbilder zeigen deshalb
Seite 1; erst die Wiederöffnungsbilder zeigen die ausgewählte eigene Karte.

Die beiden Skriptfehler sind keine bestandenen Gesamtläufe. Der erste Lauf nahm
fälschlich an, eigene Ordnerbilder stünden zuerst, und wählte dadurch Autumn.
Die gesicherte Originalconfig aus R6 wurde vor dem Rücksetzen hashgeprüft und
der aktuelle Unterschied ausschließlich auf diesen einen Pfad begrenzt.
Der zweite Lauf erreichte die Dateiauswahl; nach der Screenshot-Freigabe erreichte
Escape den Dialog nicht. Die eindeutig eigene GTK-Oberfläche wurde in einem
separaten Helfer gezielt per Pointer bedient. Diese Fortsetzung endete mit Exit 0.
Konfiguration bytegleich, eigene Bilddateien entfernt, Räume und Outputs identisch,
keine Fenster. Der leere Wallpaper-Elternordner bleibt erhalten, weil der erste
Helfer seinen ursprünglichen Existenzzustand nicht dauerhaft gesichert hatte.
Der Fokuswechsel nach Screenshot-Freigabe ist für R9 erneut zu prüfen.
Belege einschließlich Fehlversuchen und Wiederherstellung:
[evidence/r7/wallpaper-live](evidence/r7/wallpaper-live/).

Ergänzte Dateien:

| Datei | Änderung |
| --- | --- |
| `crates/niwoe-shell/src/wayland/state.rs` | Deutscher Titel, Bestätigen-/Abbrechen-Text und deutscher Bildfilter für vorhandenen GTK-Helper bzw. Zenity-Fallback. |
| `scripts/niwoe-file-picker` | Optionaler Abbrechen-Text; bisheriges Verhalten anderer Aufrufer bleibt erhalten. Keine globale GTK-/KDE-Änderung. |
| `crates/niwoe-shell/src/wayland/state/ipc_events.rs` | Auch bestätigte Bildpfade über den bestehenden Appearance-IPC-Einstieg aktualisieren den vorhandenen Katalogworker. |

120 LF-Quellen auf Fedora hashgleich. `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1275 bestanden, 0 Fehler,
1 bestehender D-Bus-Test ignoriert), Design Guard, striktes Workspace-Clippy und
Shell-Release: alle Exit 0. Dateidialog-Helper zusätzlich mit `py_compile` geprüft;
sein vorheriger Installationshash entsprach exakt der unveränderten Ausgangsdatei.
Release/installiert/laufend:
`349c770d9343ccdb19f2039146e268e3bce6614f332168ba1871a0e887a03271`,
Watchdog `372720` → `379813`; Helper
`2a5c6d6f91a2530a4c2b950b0935d1809633ed455b6a8400aacae770720678dc`.
Ein fehlerhafter Installationsaufruf endete vor jeder Dateiänderung; die getrennten
korrekten Installationen und die anschließende Identitätsprüfung endeten mit Exit 0.
Kein Neulogin erforderlich. [evidence/r7/wallpaper2-gates](evidence/r7/wallpaper2-gates/).
Cache-/Speichermodell wie oben; zusätzlich ein Einmal-Refresh bei expliziter IPC-Bildwahl,
keine neue dauernde Schleife. Noch offen: Bilder des aktuellen deutschen Dateidialogs,
Monitor-/Skalierungsmatrix, übrige Providerdarstellung und übergreifende R9-Fälle.

## R7 – abgeschlossene Wallpaper-Matrix und Anzeige-Vorherbefund

Die Wallpaper-Matrix des ergänzten Releases `349c770d9343ccdb19f2039146e268e3bce6614f332168ba1871a0e887a03271`
ist abgeschlossen: beide Outputs bei 1000/1406/1500/2000 Scale-Millis, jeweils
erste Seite, letzte Seite mit Tastaturfokus am Fehlerbild bzw. langen eigenen
Bildnamen sowie gespeichertes eigenes Bild im Modus Einpassen nach Wiederöffnen.
Alle 32 Aufnahmen wurden tatsächlich angesehen. Karten, Fehlertext, Seitenwechsel,
letzte Tastaturziele, Speichern und Wiederöffnen sind im dokumentierten Umfang
bestanden. Originalconfig bytegleich, MIME-/Paneldateien bytegleich, eigene
Bilddateien entfernt, Räume/Outputs identisch und keine Fenster. Watchdog-PID
nach Bereinigung: 385300, laufender Hash unverändert.
[evidence/r7/wallpaper-matrix](evidence/r7/wallpaper-matrix/).
V23 bleibt sichtbar: die logische Rasterfläche wird bei höheren Skalierungen
weich. Dies ist keine Gesamtfreigabe von R7 oder R9.

Der anschließende rein lesende Lauf hat Exit 0; alle drei Bilder wurden angesehen.
Der installierte GTK-Helper zeigt „Hintergrundbild auswählen“, „Auswählen“ und
„Abbrechen“. Escape schließt ihn vor einer Screenshot-Freigabe korrekt.
Das frühere Verhalten nach der Freigabe bleibt als Fokusfall für R9 erhalten.
Die Anzeige-Vorherbilder belegen noch technische englische Kurztexte, lokale
Monitorzeichnung, kleine Textrollen, Glyphenpfeile/-stern und eine Auswahl von
nur acht aus 36 tatsächlich gemeldeten Modi des zweiten Outputs. Config,
MIME, Panel, Räume und Outputs blieben identisch; Fenster leer. Dieser Befund
wird jetzt innerhalb des bestehenden Settings-Tabs im Control Center repariert.
[evidence/r7/appearance-before](evidence/r7/appearance-before/).
## R7 – Anzeige-Reparatur, gemeinsames Textzeilen-Control und Release

Status: implementiert, geprüft und installiert; aktuelle Native-Bildprüfung läuft.
Keine visuelle Gesamtfreigabe. Vorherbefund siehe unmittelbar oben.

| Datei | Änderung |
| --- | --- |
| `crates/niwoe-tokens/src/spacing.rs` | Gemeinsame 50-Pixel-Zeile für 14/12-Textrollen einschließlich Fokusabstand. |
| `crates/niwoe-ui/src/widget/text_row.rs`, `widget/mod.rs` | Wiederverwendbare zweizeilige Row mit gemessener Textkürzung, neutralen Interaktionsflächen und vorbereiteten Arc-Symbolen. Ein globaler Tastaturfokusrahmen. |
| `crates/niwoe-ui/src/effect/symbol.rs` | Native Monitorzeichnung im vorhandenen gecachten Symbolsystem. |
| `crates/niwoe-shell/src/settings_view/content/display.rs` | Deutsche Bildschirmidentität, gemeinsame Auflösungs-/Primär-/Scale-/Drehungscontrols und echte aktuelle/bevorzugte Moduszustände. |
| `settings_view/display_widgets.rs`, `display_controls.rs` | Technischen Monitor-/Detaildump und Glyphencontrols entfernt; andere bestehende Gerätehelfer und AddAppRow erhalten. |
| `settings_view/display_paging.rs`, `ids.rs`, `settings_view.rs` | Höhenabhängige Modusseiten mit stabilen Originalindizes; einzelner Bildschirm pro Seite. Bis 16 Outputs und 256 Modi pro Output statt acht auswählbaren Modi. |
| `settings_view/content_builders.rs`, `draw.rs`, `wayland/render/launcher.rs`, `wayland/state/control_center.rs` | Private Settings-Grenzen vollständig koordiniert um transienten Seitenzustand ergänzt. |
| `settings_view/wallpaper_widgets.rs`, `content/wallpaper.rs` | Seitenlabel für die gemeinsame Nutzung umbenannt, Wallpaper-Verhalten erhalten. |
| `wayland/shell.rs`, `init/shell_state.rs`, `state/popups.rs`, `state/ipc_events.rs` | Transienter Zustand; Reset beim Seiten-/Katalogwechsel. Keine neue persistierte Einstellung. |
| `widget_action.rs`, `wayland/handlers/widget_dispatch.rs`, `widget_dispatch/dispatch.rs`, `display_paging.rs` | Explizite Seitenaktionen, deaktivierte Navigation ohne Fokusziel. |
| `widget_dispatch/settings_and_context.rs` | Modusauswahl benutzt den unveränderten Snapshotindex; ungültige Geometrien verschieben keine späteren Ziele mehr. |
| `settings_view/layout_tests.rs` | 16 Outputs mit je 64 Modi inklusive ungültigem Zwischeneintrag: vollständige Maus-/Tastaturerreichbarkeit über alle Seiten in FHD/Minimalfläche. Leere Snapshots und übergroße gespeicherte Seitenpositionen. |

Die Einzelbildschirmseite ist eine konkrete Layoutnotwendigkeit für vollständige
Bedienziele im kleinen Arbeitsbereich: das alte Stapeln aller Outputs und der
acht angehängten Modi schnitt Ziele ab. Die gesamte Anzeige bleibt im bestehenden
Control Center; kein separates Settings-Fenster. Keine neuen Abhängigkeiten,
keine Cargo-Manifeständerung, keine Änderung von Renderreihenfolge oder Glas.

Performance: nur ereignisgesteuerter Treeaufbau, keine neue Abfrage-/Frame-Schleife.
Symbole werden beim Treeaufbau vorbereitet und verwenden den vorhandenen FIFO-
Cache nach Artwork/Farbe/Rastergröße, maximal 32 × 96 × 96 × 4 Byte. Textrollen
kommen aus der gemeinsamen Quelle. Die einmalig initialisierten 4096 stabilen
Modus-IDs sind auf 16 × 256 begrenzt; Strings maximal 26 Byte, Referenzen 32 KiB
(ohne Allocatoroverhead), Lebensdauer Shellprozess. Begrenzung auf 256 ist kein
Nachweis für Displays mit mehr angebotenen Modi. Die reale Hardware meldet 1/36.

130 LF-Quellen hashgleich auf Fedora. `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1277 bestanden, 0 Fehler,
1 bestehender D-Bus-Test ignoriert), Design Guard, striktes Workspace-Clippy und
Shell-Release: alle Exit 0. Alle Rustdateien bleiben höchstens 600 physische Zeilen.
Release/installiert/laufend:
`841ae8090ad3f00cd8b7d4196cec0caf5cf09959f7f7c630ff803e7214a34e82`,
Watchdog `385300` → `393896`. Kein Neulogin erforderlich.
[evidence/r7/display-gates](evidence/r7/display-gates/).
Manueller Call-Flow: Widget-ID → typisierte Page-/Mode-Aktion → überprüfter
Output-/Original-Modusindex → vorhandenes Configsave/Reload-IPC; Pointer und
Keyboard bauen dieselbe private Settings-Struktur. Noch offen: tatsächliche
Nachherbilder, native Auswahl-/Speicherfälle, Output-/Scale-Matrix und übriges R7.
### Anzeige: erster nativer Lauf und ergänzte Textkorrekturen

Alle sechs initialen Nachherbilder wurden tatsächlich angesehen. Sie belegen die
neuen gemeinsamen Zeilen, neutrale native Symbole und den bevorzugten/aktuellen
Einzelmodus auf Bildschirm 1. Die benannten Bilder für Bildschirm 2 bzw. dessen
Modusseiten sind **keine Nachweise dieser Fälle**: der zweite Monitor verschwand
zwischenzeitlich und wurde mit Output-ID 5 statt 4 wieder angemeldet. Ein Bild
zeigt tatsächlich „Bildschirm 1 von 1“; die folgenden Bilder zeigen wieder zwei
Outputs und den zurückgesetzten ersten Bildschirm. Der unveränderte Snapshot
am Ende unterscheidet sich ausschließlich durch diese neue ID. Config-/MIME-/
Paneldateien und Räume blieben gleich, Fenster leer. Der Helfer endete korrekt
mit Exit 1 statt einer Freigabe. Keine Datenrestauration erforderlich oder ausgeführt.
[evidence/r7/display-readonly-initial](evidence/r7/display-readonly-initial/).

Aus den echten Bildern wurden zwei kleine Textfälle korrigiert: „1 Modus“ statt
„1 Modi“ und eine unbekannte Ausrichtung wird ausdrücklich unbekannt dargestellt,
auch wenn kein Transformwert verfügbar ist. Kein erfundener 0°-Wert. Der nächste
Lauf prüft die Outputidentität vor/nach jeder Aufnahme, damit ein weiterer
Hardwarewechsel sofort als unterbrochene Prüfrunde erkennbar ist.
Der ergänzte Anzeige-Release wurde erneut vollständig geprüft: 130 LF-Quellen
hashgleich; fmt, Workspacecheck/-tests (1277/0/1), Design Guard, striktes Clippy
und Release alle Exit 0. Release/installiert/laufend jetzt
`9abc457c96f51347e362e9755c3dbde2184b8e283dd0e843b6fc301b009b5521`,
Watchdog `393896` → `401163`. Kein Neulogin erforderlich.
[evidence/r7/display2-gates](evidence/r7/display2-gates/).
### Anzeige: tatsächlicher zweiter Livevergleich bestanden

Der zweite rein lesende Lauf endete mit Exit 0. Alle sechs Aufnahmen wurden
angesehen: beide echten Bildschirmseiten, erste/zweite/vierte von vier Modusseiten,
alle neun Ziele der letzten Seite bis zum letzten tatsächlichen Modus
720 × 400 · 70,08 Hz und Tastaturfokus auf diesem Ziel. Die stabile Outputprüfung
lief vor/nach jeder Aufnahme; kein Hardwarewechsel. Deutsche Textpaare, native
Monitor-/Chevron-/Check-Symbole und ein einzelner Fokusrahmen sind bestätigt.
Outputnavigation und Modusnavigation zeigen ihre jeweilige echte Seitenzahl;
„Weiter“ ist auf der letzten Seite deaktiviert. Config/MIME/Panel bytegleich,
Räume und Outputs identisch, keine Fenster. Build `9abc457c96f51347e362e9755c3dbde2184b8e283dd0e843b6fc301b009b5521`.
[evidence/r7/display-readonly2](evidence/r7/display-readonly2/).
Dies belegt Darstellung und reine Navigation am ersten FHD-Output; wirkliche
Modus-/Scale-/Primaryänderungen und die folgende Zweimonitor-/Skalierungsmatrix
sind noch offen. Die 28 zuvor nicht auswählbaren Modi sind jetzt sichtbar
und über weitere Seiten erreichbar; dieser Satz behauptet keine Aktivierung
jedes Hardwaremodus.
### Anzeige-Matrix: erster Sichtungsstand

Die 16 Aufnahmen des ersten Outputs und acht Aufnahmen des zweiten Outputs bei
1000/1406 wurden angesehen. Die Moduswahl wechselt in der Minimalfläche auf
vier Ziele pro Seite und neun Seiten; letzte Auswahl und Footer bleiben im
sichtbaren Bereich, Tastaturfokus erreicht den letzten Eintrag. Bei 1000 bleiben
es neun Ziele und vier Seiten. Primärstatus und Prozentanzeige folgen dem echten
Outputzustand. Die bekannte Rasterweichheit V23 bleibt sichtbar, besonders auf
dem ersten Output. Diese Zwischenprüfung ersetzt noch nicht den Gesamtlauf und
seinen exakten abschließenden Cleanupnachweis. Noch acht Aufnahmen ausstehend.

### Anzeige-Matrix: vollständige Sichtung und Bereinigung

Alle 32 Aufnahmen wurden tatsächlich angesehen, einschließlich des zweiten
Outputs bei 1500 und 2000. Beide Primärausgaben und die Skalierungen
1000/1406/1500/2000 wurden geprüft. FHD bietet neun Modusziele auf vier Seiten,
die Minimalfläche vier Ziele auf neun Seiten; jeweils sind letzte Seite und
letzter Tastatureintrag sichtbar. Die 36 tatsächlich gelieferten Moduseinträge
bleiben erhalten, einschließlich gleichlautender Hardwareeinträge. Das ist kein
Nachweis von 36 verschiedenen oder tatsächlich aktivierten Modi.
Der Lauf endete mit Exit 0: Config/MIME/Panel bytegleich, Räume und vollständiger
Outputsnapshot identisch, Fenster leer. Laufende Shell `9abc457c96f51347e362e9755c3dbde2184b8e283dd0e843b6fc301b009b5521`, PID 406493.
[Bilder, acht Fälle und Cleanup](evidence/r7/display-matrix/).
Diese Freigabe gilt für Geometrie und Navigation; V23 bleibt sichtbar und offen.

### Neuer Befund V31: Drehung speichert wiederholt den Normalzustand

Vor der Änderung am installierten Stand `9abc457c96f51347e362e9755c3dbde2184b8e283dd0e843b6fc301b009b5521`
wurden drei echte Bilder angesehen. Beide Klicks auf die Drehung des zweiten
Monitors lassen ihn tatsächlich bei `Normal`/0°. Die vorbestehende Routine
vergleicht Configwerte (`90`) mit dem Compositor-Debugsnapshot (`Normal`, `_90`).
Deshalb wählt sie wieder den ersten Zykluswert. `rotation_first_click_pass=false`
bleibt als Fehlbeleg erhalten; der Reproduktionshelfer selbst endet mit Exit 0
nach bestätigtem Befund und exakter Bereinigung: Config/MIME/Panel bytegleich,
Räume und Outputs identisch, keine Fenster.
[Vorherbilder und Snapshots](evidence/r7/display-rotation-before/).
V31 ist in Arbeit; eine Korrektur allein ist noch keine visuelle Freigabe.

### Anzeige: geprüfter und installierter Speicherfix

| Datei | Änderung |
| --- | --- |
| `wayland/handlers/widget_dispatch/display_transform.rs` (neu) | Normalisiert Config-/Snapshotnamen für den vollständigen Drehzyklus; Spiegelung/unbekannt kehrt auf 0° zurück. Zwei Regressionstests prüfen den Wechsel nach jedem Snapshot und die Rückfallfälle. |
| `wayland/handlers/widget_dispatch.rs`, `widget_dispatch/dispatch.rs` | Bindet die kleine Zuordnung ein; Kategorienwechsel setzt auch Anzeige-Paging zurück. Vorhandenes Configsave/Reload bleibt erhalten. |
| `widget_dispatch/display_paging.rs` | Nutzt dieselbe minimale Control-Center-Fläche wie der Widgetbaum, statt der rohen Layerhöhe. |
| `settings_view/ids.rs` | Entfernt den nicht mehr benötigten abweichenden Drehzyklus. |
| `settings_view/display_paging.rs`, `content/display.rs` | Benennt bei mehr als 256 Modepositionen die tatsächlich angebotene und gesamte gültige Anzahl; Regression sichert Originalindex und Grenze auch bei ungültigem Eintrag. |

131 LF-Quellen hashgleich auf Fedora. `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1280 bestanden, 0 Fehler,
1 bestehender D-Bus-Test ignoriert), Design Guard, striktes Workspace-Clippy und
Shell-Release alle Exit 0. Lokales fmt plus rustfmt der Include-Dateien Exit 0;
alle Rustdateien höchstens 600 physische Zeilen. Release/installiert/laufend:
`ccbbd65664e46a4da5a68f5b6157e7b47b66784223880ad69eee4b21229b1e8f`,
Watchdog `406493` → `414069`, kein Neulogin.
[Prüflogs und Buildidentität](evidence/r7/display3-gates/).
Kein zusätzlicher I/O-, Decode- oder Framepfad: reine begrenzte Zuordnung,
bestehende eventgetriebene Controls. Native Speicherprüfung folgt; V31 ist
damit erst geprüft/installiert.

### Anzeige: Speicherfälle und neuer Befund V32

Alle 15 Aufnahmen des ersten Speicherlaufs wurden angesehen. Auflösung
1920 × 1080/60 Hz per Maus und 3840 × 2160/30 Hz per Tastatur werden tatsächlich
gespeichert, vom Compositor übernommen und nach Wiederöffnen angezeigt.
Skalierungen 1250/1500/2000/1000 und der Drehzyklus 90/180/270/Normal stimmen
jeweils in Config, Snapshot und angezeigtem Text überein. V31 ist für diesen
nativen vollständigen Drehzyklus visuell geprüft. Das ist kein optischer
Nachweis des gedrehten zweiten Desktops: die Bilder zeigen das Control Center
auf dem ersten Monitor, die tatsächliche Ausrichtung belegt der Snapshot.

**V32, offen:** Eine neue Primäranzeige wird gespeichert und im Outputsnapshot
aktiv, aber vorhandene Shell-Layer bleiben am ursprünglichen Output. Beide
Aufnahmen des neuen Primärmonitors zeigen tatsächlich nur Wallpaper. Die
folgenden automatisierten Klicks waren deshalb auf der falschen Oberfläche;
der Gesamtversuch endet mit Exit 1 (`KeyError: drm-0`), keine Gesamtfreigabe.
Bereinigung ist trotzdem vollständig bestanden: Config/MIME/Panel bytegleich,
Räume und vollständige Outputs identisch, Fenster leer. Watchdog-PID 415292,
weiter Build `ccbbd65664e46a4da5a68f5b6157e7b47b66784223880ad69eee4b21229b1e8f`.
[15 tatsächliche Bilder, Fälle und Fehlerlog](evidence/r7/display-save-initial/).
Die Rückwahl wird getrennt an der tatsächlich noch vorhandenen Oberfläche
geprüft; Layerzuordnung beim Primärwechsel gehört in die gemeinsame
Output-/Layer-Lifecycleprüfung von R9. V32 wird dadurch nicht geschlossen.

### Anzeige: Primärrückwahl an der tatsächlichen Oberfläche

Der gesonderte Lauf endete mit Exit 0; alle sieben Bilder wurden angesehen.
Primäranzeige 2 wird per Maus gespeichert, nach Wiederöffnen am tatsächlich
noch vorhandenen Control Center korrekt als ausgewählt dargestellt. Primäranzeige
1 ist per F8/Tab erreichbar und wird mit Enter tatsächlich zurückgesetzt;
Config, Outputsnapshot und wieder geöffnete Seite stimmen überein. Ein einzelner
Fokusrahmen sowie das native Check-Symbol sind bestätigt. Aufnahme des neuen
Primäroutputs zeigt weiterhin nur Wallpaper, Aufnahme des ursprünglichen
Outputs zeigt das Control Center: V32 bleibt offen, keine Umdeutung des Fehlers.
Auch hier Config/MIME/Panel bytegleich, Räume und Outputs identisch, Fenster
leer. Build `ccbbd65664e46a4da5a68f5b6157e7b47b66784223880ad69eee4b21229b1e8f`.
[Sieben Bilder und strikter Cleanup](evidence/r7/display-primary/).
Der Screenshothelfer erlaubt für diesen Vergleich einen ausdrücklich gewählten
vorhandenen Output; die reguläre native Screenshotzustimmung bleibt erforderlich.
Kein Capture-Protection- oder Authentifizierungsbypass.

### R7 – Standard-Apps: tatsächlicher Vorherbefund

Am installierten Build `ccbbd65664e46a4da5a68f5b6157e7b47b66784223880ad69eee4b21229b1e8f`
wurden vier neue Aufnahmen angesehen: Übersicht sowie Browser-, Bildbetrachter-
und PDF-Auswahl. Die Auswahlzeilen sind nur 32 Pixel hoch, zeigen kleine
einzeilige Namen und keine App-Symbole; „AKTUELL“ ist eine zusätzliche kleine
Textrolle. Die ausgeklappte Kategorie malt einen eigenen Fokusrahmen. Die
gespeicherte Bildbetrachterzuordnung zeigt `okularApplication_kimgio.desktop`.
Die gelesene echte Desktopdatei hat `Name=Okular`, `Type=Application`,
`Exec=okular %U`, `Icon=okular` und `NoDisplay=true`: **installierter gültiger
MIME-Handler, keine nachgewiesene fehlende oder kaputte Anwendung.** Die alte
Indexroutine verwirft dessen Metadaten gemeinsam mit der Auswahlliste.
Diese Unterscheidung ist Teil der Korrektur. Alle vorhandenen Zuordnungen
bleiben erhalten. Rein lesender Lauf Exit 0, Config/MIME/Panel bytegleich,
Räume und Outputs identisch, keine Fenster.
[Vorherbilder und tatsächliche Desktopmetadaten](evidence/r7/default-apps-before/).

### Standard-Apps: gemeinsame Zeilen, Metadaten und begrenztes Paging

| Datei/Modul | Änderung |
| --- | --- |
| `niwoe-app-catalog/src/lib.rs`, `tests.rs` | Additiver Parser für gültige MIME-Handler mit `NoDisplay=true`; normale App-Auswahl bleibt ausgeschlossen, übrige Gültigkeitsprüfungen erhalten. Regression für ungültige/versteckte Einträge. |
| `niwoe-shell/src/default_apps.rs`, `default_apps/{entry,previews,tests}.rs` | Metadaten, Vorschauen und Tests nach Verantwortung getrennt. Gespeicherte Handler bleiben benannt; sichtbare Kandidaten werden deterministisch sortiert. Vorhandene Auswahlgrenze von 24 je Kategorie erhalten. |
| `settings_view/default_apps_widgets.rs`, `content/default_apps.rs` | Gemeinsame TextRow mit 14/12-Pixel-Textrollen, tatsächlichem App-Symbol, nativen Chevron/Check-Symbolen und einem Fokusrahmen. Neun Kategorien zweispaltig, Auswahl seitenweise; ehrliche Lade-/Leer-/fehlende-App-Zustände. |
| `settings_view/display_widgets.rs`, `content/display.rs` | Privater TextRow-Adapter erhält einen gemeinsamen Namen; Anzeigeverhalten unverändert. |
| `settings_refresh.rs`, `wayland/state/settings_refresh.rs` | Vorhandener Hintergrundworker erstellt einmalig die begrenzten App-Symbole; Rückgabe als typisierter Settings-Snapshot. |
| `widget_action.rs`, `wayland/handlers/widget_dispatch/default_apps_paging.rs` und zugehörige State-/Rendergrenzen | Paging nutzt dieselbe begrenzte Canvasgröße wie Darstellung und Hit-Test. Seitenposition beim Wechsel zurückgesetzt; Maus und Tastatur behalten originale Kandidatenindizes. |
| `settings_view/layout_tests.rs` | 30 echte eigene Desktop-Metadateneinträge: alle 24 angebotenen Ziele je Seite bei 1920×1032 und 1366×720 per Pointer/Focus vollständig erreichbar, korrekte Action-Indizes und deaktivierte Seitengrenzen. |

139 LF-normalisierte Quellen hashgleich auf Fedora. Lokales `cargo fmt` plus
rustfmt der betroffenen Include-Dateien Exit 0. Alle Rustquellen höchstens
600 physische Zeilen. `cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace` (1283 bestanden, 0 Fehler, 1 bestehender D-Bus-Test
ignoriert), Design Guard, striktes Workspace-Clippy und Shell-Release alle Exit 0.
Release/installiert/laufend hashgleich:
`ac2fcd89825c32298f78646dc4020978e70f886791c51aaff8a301334583cd41`.
Watchdog 416284 → 427515, kein Neulogin.
[Quellbeleg, vollständige Gates und Aktivierung](evidence/r7/default-apps-gates/).

Performance: ein begrenzter Icon-Batch auf dem vorhandenen Worker je
Seitenaktualisierung, höchstens 9×(24 Kandidaten + 1 aktueller Handler),
dedupliziert nach Symbolname. Die Rastergröße stammt aus
`Controls::SYMBOL_SIZE` (16); höchstens 225×16×16×4 = 225 KiB fertige Raster
je Snapshot, alter/neuer Snapshot zusammen höchstens 450 KiB. Gemeinsame
Arc-Pixmaps, kein Decode, Metadatenlesen oder xdg-mime-Aufruf beim Paint.
Neue Snapshots ersetzen den bisherigen Cache und berücksichtigen damit
Katalog-/Theme-/Iconänderungen. Diese Rastergrenze ist keine Grenze für
gesamten Decoder-/Metadatenspeicher oder RSS. Keine Animation oder neue
dauernde Frame-/Capture-Schleife; Material und Renderreihenfolge erhalten.

Alle fünf tatsächlichen Nachheraufnahmen wurden einzeln angesehen:
Übersicht, Browser mit F8/Tab-Fokus, Browserauswahl, Bildbetrachter und PDF.
Neun Kategorien passen in die Übersicht; Haupt-/Unterzeilen überdecken sich
nicht. App-Symbole, Chevron und Auswahlhaken sind sichtbar. Der Browser hat
einen einzelnen Tastaturfokusrahmen. Die gültige gespeicherte NoDisplay-
Bildzuordnung zeigt „Okular“; dessen Handler wird weiterhin nicht als
zusätzliche sichtbare App angeboten. Rein lesender Lauf Exit 0;
Config/MIME/Panel bytegleich, Räume und vollständige Outputs identisch,
Fenster leer. [Fünf angesehene Bilder und Cleanup](evidence/r7/default-apps-readonly/).
Das schließt noch keine Schreib-/Fehlerprüfung oder gesamte V15-Abnahme ein.
Die bisherige optimistische Schreibdarstellung und sofortige synchrone
Nachabfrage sind noch vorhanden und werden getrennt repariert.

### Standard-Apps: 32 tatsächliche Monitor-/Scale-Bilder

Der erste Matrixhelfer verwendete beim Apps-Sidebareintrag fälschlich auch den
zusätzlichen Abstand der unteren Wartungsgruppe. Die ersten vier angesehenen
Bilder zeigen dadurch tatsächlich System statt Apps; **kein Standard-App-
Nachweis**. Der eigene Lauf wurde per SIGINT in seine normale Bereinigung
geführt (Exit 1/KeyboardInterrupt); alle sechs Cleanupbedingungen bestanden.
Die übrigen Aufnahmen dieses Fehlversuchs erhalten keine Sichtfreigabe.
[Unveränderter Fehlversuch und Cleanup](evidence/r7/default-apps-matrix-initial/).

Der separat protokollierte korrigierte Lauf endet mit Exit 0. Alle 32 Bilder
wurden einzeln angesehen: beide primären Outputs, Scale 1000/1406/1500/2000,
jeweils Übersicht, erste und letzte Texteditorseite sowie F8/Tab-Fokus des
letzten angebotenen Kandidaten. 30 eigene inert ausführbare Desktopeinträge
mit langen Namen plus zwei echte Kandidaten; kein Appstart. Bei FHD elf Zeilen
je voller Seite und drei Seiten, bei der minimalen Quellcanvas sechs und vier.
Der Footer zeigt korrekt „24 von 32 Apps“; die vorhandene Auswahlgrenze wurde
nicht erweitert. Namen/Unterzeilen kollidieren nicht, native Symbole bleiben
sichtbar, Seitengrenzen sind deaktiviert und die letzte Auswahl hat einen
einzelnen Fokusrahmen. Kein Überlauf; die bekannte Rasterweichheit V23 bei
Skalierung bleibt ausdrücklich offen. Physisch 1920×1080; Scale 1406 ergibt
logisch 1366×768, minimale Quellcanvas 1366×720 auch bei 150/200 %. Kein
Nachweis eines nicht vorhandenen physischen DRM-Modus 1366×768.

Build durchgehend `ac2fcd89825c32298f78646dc4020978e70f886791c51aaff8a301334583cd41`.
Jeder Fall startet die Shell über den vorhandenen Watchdog nach der eigenen
Outputkonfiguration neu; das schließt V32 nicht. Cleanup: Config/MIME/Panel
bytegleich, vollständige Räume/Outputs identisch, Fenster leer, alle 30 eigenen
Desktopdateien entfernt. Abschließende PID 443894.
[32 angesehene Bilder, Fallidentitäten und Cleanup](evidence/r7/default-apps-matrix/).

### V33 – optimistische Standard-App-Erfolgsmeldung, reproduziert

Am noch installierten Build `ac2f…` wurde eine einzelne eigene inerte
Texteditor-Desktopdatei geladen, nach dem Öffnen der Auswahl exakt entfernt
und anschließend ihr weiterhin gecachter Eintrag gewählt. Beide tatsächlichen
Aufnahmen wurden angesehen. Die Oberfläche schließt die Auswahl und zeigt
„AAA NIWOE eigene entfernte Test-App“ als aktuelle Zuordnung; die unabhängigen
`xdg-mime query`-Ergebnisse bleiben dagegen LibreOffice Writer für text/plain,
Okular für Markdown sowie leer für Readme/Log. Der Nutzer sieht damit eine
nicht übernommene Auswahl als aktuellen Stand. Ursache: lokales optimistisches
Einfügen vor dem ungeprüften Hintergrundschreiben. Auch der Auto-Pfad liest
sofort synchron, statt dessen Abschluss abzuwarten. **V33 in Arbeit**, kein
Schreiberfolg durch diese Reproduktion behauptet.

Reproduktionslauf Exit 0, sämtliche beobachteten Zuordnungs-/Index-/Config-/
Paneldateien bytegleich wiederhergestellt, Räume und vollständige Outputs
identisch, Fenster leer und eigene Desktopdatei entfernt.
[Zwei angesehene Vorherbilder, echte MIME-Abfragen und Cleanup](evidence/r7/default-apps-stale-before/).

### V33 – geprüfter Schreibabschluss statt optimistischer Anzeige

`default_apps/{command,refresh}.rs` trennt typisierte Schreibaufträge,
Backend-Ergebnisse und den anschließend gelesenen Stand. Pick prüft den
Kandidaten erneut im frischen Katalog und verifiziert jede MIME-Zuordnung;
Auto erhält vorhandene Zuordnungen und zählt nur geprüfte Ergänzungen.
Leseversagen ist ein eigener Zustand und wird nicht als leere Zuordnung
behandelt. `default_apps.rs` erhält die bestehende `query_default`-API des
Deck-Schnellstarts unverändert; dessen synchroner Aufruf gehört nicht zu
unserem neuen Settings-Worker.

`settings_refresh.rs`, `wayland/state/settings_refresh.rs` und
`widget_dispatch/dispatch.rs` führen Lesen/Schreiben über denselben einmaligen
Worker und dieselbe laufende Kategorie aus. Kein paralleler Doppelauftrag,
kein optimistischer Eintrag, keine sofortige synchrone Nachabfrage im
Settings-Dispatch. Abschlussdaten gelangen über den vorhandenen Kanal/Ping
zur Shell und erst dann zur Darstellung. Shell-/Init-/Content-/Draw-Kontexte
reichen den privaten Status weiter. Während des Auftrags sind Auto und
Kandidaten nicht aktivierbar; Rückweg und Seitenwechsel bleiben bedienbar.
`settings_view/group_widgets.rs` akzeptiert für die vorhandene Beschreibung
auch einen kurzen eigenen Statustext (`Cow`); statische Gruppen bleiben
geborgt und ihre Geometrie unverändert. Default-App-Content/Widgets zeigen
Erfolg, Leseversagen und Teilfehler in derselben bestehenden Gruppe.

Performance: Prozessaufrufe laufen unprivilegiert ausschließlich im
Settings-Worker. Kinder werden alle 20 ms dort überprüft, nach höchstens
5 s Wartezeit beziehungsweise dem gemeinsamen 30-s-Prozessbudget beendet.
Diese Grenze ist keine harte Grenze für gesamte Katalog-/Iconarbeit oder
für das Lesen einer von Nachfahren offen gehaltenen stdout-Pipe. Die
1024-Byte-Prüfung erfolgt nach dem Einlesen; sie begrenzt gültige Antworten,
nicht den gesamten Ausgabespeicher. Kein neuer Idle-Timer und keine neue
Render-/Capture-Schleife. Vorhandener begrenzter Icon-Snapshot/Invalidierung
und Renderreihenfolge bleiben erhalten.

Fünf Backendregressionen prüfen vollständiges Mehr-MIME-Speichern,
Teil-/abgewiesene Schreibvorgänge, entfernte Kandidaten, Auto-Erhaltung und
Leseversagen; die zusätzliche Layoutregression prüft den laufenden Zustand
in beiden Canvasgrößen einschließlich des erreichbaren Rückwegs.
144 LF-Quelldateien übertragen und per Hash bestätigt, alle Rust-Dateien
höchstens 600 Zeilen. `cargo fmt --all -- --check`, manuelle Formatprüfung
der eingebundenen Dateien, `cargo check --workspace`,
`cargo test --workspace` (1289 bestanden, 0 fehlgeschlagen, 1 bestehender
D-Bus-Test ignoriert), Design Guard, striktes Clippy und Releasebuild: alle
final Exit 0. Der erste Check mit zwei anschließend behobenen API-/Lifetime-
Fehlern bleibt im Beleg erhalten. Release, Installation und laufender Prozess
haben exakt `dc5c9b1d300878ad325193fb6ed200c5c625ecd2264f271316ac09926aa6df01`;
Watchdogwechsel 443894 → 453038, kein Neulogin für diese Shelländerung.
[Quellidentität, vollständige Prüflogs und Aktivierung](evidence/r7/default-apps-write-gates/).

Die identische Entfernung einer eigenen Test-App wurde am neuen Build erneut
nativ geprüft; beide Bilder angesehen. Die entfernte App wird nicht gewählt,
die echte Zuordnung bleibt sichtbar, die Liste wird erneuert und der
Fehlschlag verständlich gemeldet. Alle beobachteten Dateien und MIME-Werte
waren schon vor der Bereinigung unverändert. Exit 0 und exaktes Cleanup.
[Zwei angesehene Fehlerbilder und echte Zuordnungen](evidence/r7/default-apps-stale-after/).

Der erfolgreiche native Lauf wurde ebenfalls vollständig angesehen: zehn
Bilder, Mauswahl einer eigenen App A, Wiederöffnen, F8/Tab/Enter-Wahl von
App B, Wiederöffnen, Auto und Fokus auf die aktuelle Zuordnung sowie der
wiederhergestellte ursprüngliche Stand. Unabhängige `xdg-mime query` bestätigt
jeweils A beziehungsweise B für alle vier Text-MIME-Typen. Auto ergänzt 0 und
erhält alle neun bereits gesetzten Kategorien. Ein einzelner Fokusrahmen,
lesbare Haupt-/Unterzeilen und sichtbarer Auswahlhaken; keine Kollision.
Exit 0. Beobachtete Config/MIME/Index/Panel-Dateien bytegleich
wiederhergestellt, ursprüngliche MIME-Werte sowie vollständige Räume/Outputs
identisch, Fenster leer, beide eigenen Desktopdateien entfernt.
[Zehn angesehene Speicherbilder, unabhängige Abfragen und Cleanup](evidence/r7/default-apps-save/).

**V33 visuell geprüft im dokumentierten Erfolg-/Entfernungsumfang.**
Teilversagen und Leseversagen sind durch Backendregressionen belegt, nicht
als zusätzlich nativ gesehene Fehlerfälle behauptet. R7-Providerseiten,
aktuelle Gesamtperformance und R8/R9 bleiben offen; keine Gesamtfreigabe.

### R7 – Netzwerk-/Bluetooth-Zeilen und korrekte Controlfähigkeiten

Vier aktuelle Vorherbilder am laufenden `dc5c…` und alle vier Bilder des
ersten Nachherstands `138051…` wurden einzeln angesehen. Letzterer zeigt
saubere Textrollen und Tastaturfokus, aber irrtümlich ein WLAN-Symbol auch
am Ethernetprofil. **V34: falsch zugeordnetes Kabelnetz-Symbol**, durch den
Bildvergleich erkannt, noch keine finale Sichtfreigabe dieses Zwischenstands.
Außerdem zeigte die zusätzliche Call-Flow-Prüfung: Bluetooth besitzt eine
zehn Sekunden begrenzte Startsuche, keinen Stop-Befehl. Der neue Zwischen-
text „Suche beenden“ wäre daher irreführend. Der nur geprüfte Zwischenbuild
`cc86b7…` wurde durch die abschließende Korrektur ersetzt und nicht installiert.
Die historischen Gates bleiben in `evidence/r7/network-gates/` und
`evidence/r7/network2-gates/` erhalten.

`settings_view/network_device_widgets.rs` bereitet Netzwerk-/WLAN-/Bluetooth-
Zeilen über den gemeinsamen TextRow vor. Haupt-/Unterzeilen, Kontrast,
Fokus, Begrenzung langer Texte und Auswahlhaken stammen aus derselben Quelle.
Verbunden bedeutet weiterhin den vorhandenen Snapshotwert; bereits verbundene
Zeilen bleiben inert. Ursprüngliche Providerreihenfolge und ID-Indizes bleiben
unverändert (8 Profile, 10 WLANs, 8 Bluetoothgeräte). `content/network.rs`
zeigt die Zusammenfassung in zwei Spalten und verständliche Leeranzeigen;
wegen der bestehenden Best-Effort-API unterscheidet sie keine nicht belegbaren
Fehlerursachen. `content/bluetooth.rs` verwendet gemeinsame Chips/Buttons und
getrennte Statuszeilen. Eine laufende Suche heißt „Suche läuft“; ohne Strom
oder während einer Suche ist der weitere Start nicht aktivierbar. Kein
zusätzlicher Stop-/Pairingpfad. `control_center.rs` korrigiert „MANAGEMENT“
zu „VERWALTUNG“. `audio_system_widgets.rs` verliert nur die dadurch obsolete
Netzwerkhöhenkonstante; Audio-/Druckerdarstellung hier unverändert.

`niwoe-ui/src/effect/symbol.rs` ergänzt funktionale native WLAN-, Kabelnetz-
und Bluetooth-Artwork. Profiltyp WLAN wählt WLAN, VPN das vorhandene Schloss,
sonstige Typen Kabelnetz; gesichertes WLAN zeigt das native Schloss. Artwork
wird nur beim Baumaufbau vorbereitet, nicht in Paint dekodiert/geladen.
Schlüssel Symbol/Farbe/Rastergröße und bestehende FIFO-Grenze 32 × 96² × 4
(maximal 1,125 MiB Rasterdaten) bleiben; konkrete Settingsicons hier 24² × 4,
Arc-Referenzen statt Pixelkopien. Lebensdauer UI-Thread, andere Farbe/Größe
fordert eigenes Raster, Verdrängung bei voller Grenze. Keine Animation, neue
Idle-/Frame-/Capture-Schleife oder Änderung an Glas/Renderreihenfolge.

Manueller Call-Flow: Zeilen-ID → gemeinsame Maus-/Tastaturauflösung →
bisherige Snapshotposition → bestehender Provider. Die Optimistik vorhandener
Netzwerk-/Bluetoothmutationen und deren Abschlussbehandlung wurden in diesem
Darstellungsschritt nicht geändert und bleiben für die weiteren funktionalen
Fehlerfälle zu prüfen. Bluetoothsuche ist weiterhin der vorhandene zeitlich
begrenzte Auftrag. Zwei Regressionen prüfen per echtem Layout/Hit-Test die
Original-ID trotz langer Namen, die fehlende Aktivierbarkeit verbundener
Zeilen und die deaktivierte Suche bei aus-/laufendem Zustand; das bestehende
native Artwork-/Cachetesting enthält die zusätzlichen Symbole.

148 Quellen nach LF-Normalisierung per Hash bestätigt; Rust-Dateien unter
600 Zeilen, lokale Formatierung einschließlich include!-Dateien bestanden.
Workspace-Check/-Tests (1291 bestanden, 0 fehlgeschlagen, 1 bestehender
D-Bus-Ignore), Formatcheck, Design Guard, striktes Clippy und Releasebuild
auf Fedora alle Exit 0. Der lokale Windows-Check konnte mangels
pkg-config/Wayland-Systembibliothek nicht laufen; die Pflichtchecks sind
vollständig am tatsächlichen Linuxziel durchgeführt.
Release/Installation/laufender Prozess:
`35944d84c92910cea38879462bf12fc5026e9429cb061ebe781aed7ed75ff70d`.
Watchdog 472262 → 486294; kein Neulogin erforderlich.
[Finale Gate- und Quellbelege](evidence/r7/network3-gates/).
Finale native Sichtprüfung, zusätzliche lange/volle/gesicherte Datenfälle,
Monitor-/Scale-Matrix und Providerabschlussfehler noch offen.

Die erste finale Nachprüfung (network3) endete mit exaktem Cleanup/Exit 0,
aber ihre ersten zwei Bilder zeigen tatsächlich Systemübersicht statt Netzwerk.
Der Helfer wechselte Sidebar und Tab zu schnell nach dem Watchdogneustart.
Alle vier Bilder angesehen; kein Netzwerknachweis daraus. Der neue Helfer
trennt diese Schritte. Alle vier finalen Bilder (network4) tatsächlich
angesehen: WLAN und Ethernet besitzen nun die passenden nativen Icons;
lesbare Haupt-/Unterzeilen, aktuelle Auswahlhaken, kein Textkontakt oder
Überlauf. Bestehende verbundene Profile bleiben inert. Netzwerk-Tab und
Bluetooth-Suchcontrol sind per F8/Tab mit einem Fokusrahmen erreichbar.
Der leere echte Scan und die leere echte Bluetooth-Geräteliste sind klar
beschriftet. Keine Verbindung, Kopplung oder Suche ausgeführt.

**V34 visuell geprüft für die tatsächlich vorhandenen Profiltypen.**
Darstellung im dokumentierten regulären FHD-Umfang geprüft; V15/V16 insgesamt
bleiben offen. Gesichertes Scan-WLAN, vollständige Listen, andere Scale-/
Outputzustände und Providerabschlussfehler sind damit nicht nativ belegt.
Finaler Lauf Exit 0, Config/Panel/MIME bytegleich, vollständige Räume/Outputs
identisch, Fenster leer; Build/PID unverändert `35944…`/486294.
Geräteaufnahmen mit privaten Profilnamen bleiben ausschließlich unter
`target/r7-network-before-evidence`, `target/r7-network-after-evidence`,
`target/r7-network3-after-evidence` und `target/r7-network4-after-evidence`.
[Helper, Bildhashes, Bluetoothbilder ohne Gerätenamen und Cleanup](evidence/r7/network-comparison/).

### R7: Audio-/Druckerdarstellung im gemeinsamen Rahmen, 03.10.2026

Vorher: vier native Bilder des installierten 35944-Standes tatsächlich angesehen:
englische Dienst-/Gerätetexte, eigene kleine Audio-/Druckerzeilen. Reale Sitzung:
eine Audio-Ausgabe (84 Prozent), eine Eingabe (100 Prozent), CUPS aktiv, keine
Drucker/Aufträge gemeldet. Kein Gerätewechsel und keine Lautstärkeänderung.

Geänderte Quellen:
- `settings_view/content/sound.rs`, `content/printers.rs`: gemeinsame Textzeilen,
  deutsche Status-/Leertexte, zweispaltige Geräteflächen statt eigener Karten.
- `audio_system_widgets.rs`, `network_device_widgets.rs`, `settings_view.rs`,
  `draw.rs`: alte Audio-/Druckerwidgets entfernt, gemeinsame Preset-/Mutecontrols
  und native Gerätezeilen; ursprüngliche Audio-Snapshotindizes bleiben unverändert,
  Standardgeräte haben keine erneute Auswahlaktion. SystemInfoRow bleibt für den
  nächsten getrennten Inhaltsschritt erhalten.
- `niwoe-tokens/src/settings.rs`: Controlbreiten zentral (Gerät 160, Preset 80).
  Bluetooth verwendet dieselbe Gerätebreite, unverändert 160.
- `niwoe-ui/src/effect/symbol.rs`: funktionale native Speaker-/Microphone-/Printer-
  Vektorassets in derselben begrenzten Artworkcachefamilie; bestehender Assettest
  erweitert. Kein I/O beim Paint, keine Animation, keine neuen Timer/Dependencies.
- `settings_view/network_tests.rs`: Geräteidentität und inerte Standardgeräte für
  Ein-/Ausgabe mit langen Namen und zwei Zeilenbreiten geprüft.

Pflichtprüfungen auf Fedora: 150 normalisierte LF-Quellen ohne Hashabweichung;
`cargo fmt --all -- --check`, `cargo check --workspace`, `cargo test --workspace`,
Design-Guard, `cargo clippy --workspace --all-targets -- -D warnings` und Shell-
Releasebuild erfolgreich. 1292 bestanden, 0 fehlgeschlagen, 1 bestehender D-Bus-
Ignore. Alle Rustquellen höchstens 600 physische Zeilen.
Belege: `evidence/r7/audio-gates/`.

Release/Installation/laufende Shell sind identisch:
`4431c7789a0eee46132b37f1e53c3d02042fc5cfe627c2cbe53df1431fc7513b`.
Watchdog erneuert PID 486294→495111 in derselben Sitzung. Vier finale native FHD-
Bilder tatsächlich einzeln angesehen: deutsche lesbare Audiozeilen, native
Speaker/Microphone/Check, gemeinsame Controls und einzelner 0-Prozent-Fokus;
Druckerdienst/Leerzustand und Druckertabfokus im gleichen Rahmen. Cleanup bestätigt
exakt gleiche Config-/Panel-/MIMEbytes, vollständige Räume/Outputs und keine Fenster.
Bildhashes, Cleanup und Helfer: `evidence/r7/audio-printer-comparison/`; tatsächliche
Audio-Gerätenamen bleiben mit ihren PNGs im ignorierten Testverzeichnis.

Grenzen: Diese Runde prüft die reguläre native FHD-Darstellung. Voll belegte/
fehlerhafte Provider, live Gerätewechsel, erfolgreiche/fehlgeschlagene Lautstärke-
aktionen, zwei Outputs/vier Scales und lange Daten bleiben R7/R9-Prüfaufträge.
Best-effort-Provider können noch Teilfehler und leere Listen zusammenführen;
es ist kein Backendfehlerabschluss und keine Gesamtfreigabe. Die vorhandenen
Audioaktionen und ihre anschließende Abfrage bleiben unverändert; der Renderpfad
führt die neuen Symbole/Textzeilen ohne zusätzliche Provideraufrufe aus.

### R7: Übersicht, Konten, Updates und Energie; V35 bestätigt/korrigiert, 03.10.2026

Zehn tatsächliche Vorheraufnahmen des installierten 4431-Standes angesehen:
acht normale/Fokusbilder der vier Inhalte und zwei gesonderte Energiebilder.
V35: Der erste Bereitschaftsklick setzt den bestehenden Armed-Zustand, zeigt in
Settings aber keinen Bestätigungstext. Ursache: vorhandenes Renderargument wird
in `draw.rs` verworfen; der Settings-Baum hat keine Armed-Information. Der
native Vorherbeleg führt ausschließlich erstes Arming und Kategorieabbruch aus.

Geänderte Dateien und Verhalten:
- `users.rs`, `settings_refresh.rs`, `wayland/state/settings_refresh.rs`: Konten
  einmal pro angefordertem Seitenrefresh auf vorhandenem Hintergrundworker lesen;
  erfolgreich leere Liste und Lesefehler getrennt als Ready/Unavailable behandeln.
  Im Render-/Baumaufbau kein `/etc/passwd`-I/O mehr. Gemeinsame UI zeigt angemeldetes
  bzw. lokales Konto ohne UID-Diagnose; bestehender UID-Filter bleibt erhalten.
- `wayland/shell.rs`, `wayland/init/shell_state.rs`, `settings_view/content_builders.rs`,
  `settings_view/draw.rs`, `wayland/state/control_center.rs`, `wayland/render/launcher.rs`:
  typisierter gecachter Kontostand durch bestehenden Shell-/Settings-Datenfluss;
  vorhandenen Armed-Zustand bis zur Darstellung und Tastaturbaum weiterreichen.
- `settings_view/content/{users,system_overview,updates,power}.rs`,
  `audio_system_widgets.rs`, `display_widgets.rs`, `settings_view.rs`: alte kleine
  SystemInfoRow samt nun ungenutztem Zeichenkürzer/Radius entfernen. Gemeinsame
  zweispaltige Textrollen für Statusdaten; Einführung als gemeinsame sekundäre
  Aktion darunter. Energie nutzt gemeinsame neutrale Textzeilen und Idle-Chips;
  nur das aktuell armierte Ziel zeigt Bestätigungstext/Check/Selektion.
  Bestehende Zweitbestätigung, andere-Auswahl-Abbruch und 4-s-Ablauf unverändert.
- `updates.rs`: vorhandenen APT-Provider nur für Debian/Ubuntu-Familie verwenden,
  aus ID/ID_LIKE statt Anzeigenamen entscheiden und LC_ALL=C beim APT-Aufruf.
  Fedora/RPM und unbekannte Plattform zeigen fehlende Integration und unbekannten
  Updatezustand ohne erfundene Anzahl. Leere lokale APT-Liste verspricht keine
  aktuelle Gesamtsystemfrische. Kein neuer Fedora-Backend-/Installationspfad.
- `settings_view/navigation.rs`, `content/pinned_apps.rs`: historische Kategorie-
  anfragen landen auf der aktiven Apps-Seite. Aktuelles Panel ignoriert Pinning-
  Daten (`panel_view/layout.rs`); aktive Kategorien/Suche enthalten diesen alten
  Weg schon seit R4 nicht. Keine Daten löschen, keine falsche Panelzusage behalten.
- Tests in `users.rs`, `updates.rs`, `settings_view/layout_tests.rs` und
  `navigation.rs`: Lesefehler/erfolgreich leer unterscheiden, Fedora nie APT,
  Ableitungen/Unbekanntstatus, alle Energie-/Idle-Aktionen auf FHD/Mindestcanvas
  per Fokus und Hit-Test erreichbar, historische Pinningroute normalisiert.

Prüfungen: 156 normalisierte LF-Quellen ohne Hashabweichung; Workspace-Check,
Workspace-Test, fmt-Check, Design-Guard, strenges All-Target-Clippy und Shell-
Releasebuild alle erfolgreich. 1296 bestanden, 0 fehlgeschlagen, 1 vorhandener
D-Bus-Ignore; alle Rustdateien höchstens 600 physische Zeilen. Eine direkte lokale
rustfmt-Inventurrunde traf auf das bestehende inkludierte Struct-Ausdrucksfragment
`init/shell_state.rs`; dieses ist keine eigenständige Rustdatei für rustfmt.
Anschließend wurden die betroffenen vollständigen Module/Includes passend
formatiert und die oben genannten Fedora-Gates komplett bestanden.
Belege: `evidence/r7/system-gates/`.

Release/installierte/laufende Shell identisch:
`053fc909a0a18934c146286e3fb9f214258833e2d3705bc0aac7328cb2995d46`.
Watchdog PID 495111→503787. Acht finale normale/Fokusaufnahmen tatsächlich einzeln
angesehen: Systemdaten und echte lokale Konten lesbar, Fedora-Fähigkeitsanzeige
korrekt, Energiecontrols neutral, ein Idle-Fokus. Fünf zusätzliche Energiebilder
alle tatsächlich angesehen: Maus-Erstbestätigung sichtbar, Kategorieabbruch,
Tastatur F8/Tab/Space-Erstbestätigung mit einzelnem Fokus, Bestätigung läuft aus.
Keine zweite Ausführung und keine Sitzungs-/Hardwareaktion. Beide Cleanupberichte
bestätigen exakt gleiche Config-/Panel-/MIMEbytes, volle Räume/Outputs und keine
Fenster. Hashes/Helper/Cleanup/Public-Energiebilder in `evidence/r7/system-comparison/`;
Kontonamen/Systemdatenbilder bleiben im ignorierten Testverzeichnis.

Call-Flow: Seitenanforderung → bestehendes Inflight-Set → Settings-Worker →
SettingsData::Users → gecachter Shellstand → gemeinsame TextRow. Arming/Verbrauch
bleibt ausschließlich im vorhandenen Powerdispatcher; Paint fragt keinen Dienst
ab. Neue Zeilen verwenden begrenzte bestehende Symbol-/Fontcaches und nur
bestehende Invalidierung; keine neuen Animationen, Timer oder Paint-I/O.
Noch offen: volle/lange Providerdaten und Fehler-/Scale-/Outputmatrix, spätere
Fedora-Paketintegration gemäß getrenntem Backendumfang, aktuelle R7-Performance
und R8/R9. V35 ist im dokumentierten ersten Arming-/Abbruch-/Ablaufumfang nativ
visuell geprüft; geschützte/irreversible Sitzungsaktionen sind damit nicht geprüft.


### V36 – volle Providerlisten werden abgeschnitten (R7, offen)

Build `053fc909a0a18934c146286e3fb9f214258833e2d3705bc0aac7328cb2995d46`: acht tatsächliche native Bilder bei 100 und 200 Prozent vollständig angesehen. Eigene ausschließlich lesende Providerfixtures: acht Profile, zehn WLANs, acht Bluetoothgeräte, je acht Audioein-/ausgänge sowie acht Drucker. Netzwerk verliert bereits bei FHD die unteren WLANs; auf dem Mindestcanvas sind WLANs vollständig und mehrere Profile abgeschnitten. Bluetooth verliert das letzte Gerät, Audio mehrere untere Geräte. Drucker passen in beiden Fällen. Lange Namen werden begrenzt; V23 (weich skalierter Rastercanvas) bleibt sichtbar. Keine Geräteaktion ausgeführt. Konfiguration, Räume, Outputs und reale Standardlautstärke exakt wiederhergestellt; eigene Fixtures entfernt. Bildhashes, angesehen-Status, Helper und Bereinigung: `evidence/r7/provider-comparison/before-viewed.json`, `before-review.py`, `before-cleanup.json`.

#### V36: erste Nachher-Bedienserie verworfen und bereinigt

Release `02318d1225ae69000f25e82ff4581636716726d4deb3b1c1a66ca2f6f907b232`, 159 normalisierte Quellhashes, Check/Workspace-Tests (1298 bestanden, 0 Fehler, 1 bestehender D-Bus-Ignore), Format, Design Guard, striktes Clippy und Release grün. Die vollständigen Listenseiten und ursprünglichen Geräteidentitäten werden auf FHD und Mindestcanvas geprüft. Sämtliche Rust-Dateien bleiben unter 601 Zeilen.

Die erste native Nachher-Bedienserie ist teilweise ungültig: FHD-Netzwerk und Netzwerk-Seite 1 bei 200 Prozent sind tatsächlich korrekt durchgesehen; die Rückwärts-Tab-Sequenz landete dagegen in der Sidebar „Einstellungen“. Nachfolgende Space-/Mausaktionen betrafen Wallpaper statt der beschrifteten Providerseite. Diese Bilder sind ausdrücklich keine Paging-/Tastaturbelege. Der Helper brach bei seiner Konfigurationsprüfung ab und entfernte die eigenen Providerwrappers/Marker; die vorsichtige Konfigurationsprüfung verhinderte ein ungeprüftes Überschreiben.

Der gesonderte Cleanup verifizierte anschließend exakt die eigenen Abweichungen (Wallpaper Coast, nur die zuvor eigenen Output-Scales, sämtliche anderen Konfigurationswerte gleich) und stellte ursprüngliche Datei, Outputs und Räume wieder her. Alle Wiederherstellungsprüfungen bestanden, keine Testfenster/Providerwrapper blieben erhalten. Helper, fehlerhafter Log und `rejected-cleanup.json` bleiben in `evidence/r7/provider-comparison/` erhalten. Die Bedienprüfung wird mit Maus und zunächst ausschließlich Vorwärts-Tab-Fokus wiederholt; keine Space-Aktion auf ungeprüftem Fokus.
#### V36: gemeinsame Listenreparatur installiert und im Bedienumfang geprüft

`settings_view/provider_paging.rs` berechnet die sichtbaren Zeilen ausschließlich aus vorhandenen zentralen Gruppen-, Textzeilen-, Gap- und Controltokens. Die Footer verwenden dieselben Controls/Labels wie bestehende Settings-Picker. Auf dem Mindestcanvas sind fünf Profile/WLANs, sechs Bluetoothgeräte und je vier Audioein-/ausgänge pro Seite erreichbar; auf FHD passen sämtliche bestehenden begrenzten Listen. Netzwerk verwendet zwei parallele Spalten. Vorhandene Grenzen von 8 Profilen/10 WLANs/8 Bluetooth-/je 8 Audiogeräten bleiben bestehen und werden mit den jeweiligen Zahlen sichtbar angegeben. Ursprüngliche Snapshotindizes werden durch das Blättern nicht verändert; verbundene/Standardgeräte bleiben inert.

Die Contentmodule `network.rs`, `bluetooth.rs` und `sound.rs` verwenden diese gemeinsame Berechnung. `content_builders.rs`/`draw.rs` und die vorhandenen Wayland-Aufrufer reichen nur einen flüchtigen Seitenstand durch; Kategorie-, Such- und Öffnungswege setzen ihn zurück. `widget_action.rs` sowie der kleine `widget_dispatch/provider_paging.rs` behandeln Vor/Zurück, klemmen den Stand bei Datenwechseln und setzen den Fokus auf einen tatsächlich bedienbaren Footer. Kein neues IPC, keine privilegierte Shellaktion, keine neue Dependency oder persistente Einstellung.

Die neue `provider_tests.rs` prüft sämtliche begrenzten Originaltargets auf jeder Seite mit langem Text und maximaler Netzwerksummary auf FHD und Mindestcanvas, einschließlich Hit-Test, vollständigem Hitbereich, Fokus, deaktivierten Standards, Page-Clamping und sichtbaren Grenzen. Alle Pflichtprüfungen, Build-/Quellidentität und Zeilengrenzen sind unter `evidence/r7/provider-gates/` belegt.

Alle 17 Bilder der korrigierten Maus-/Fokusserie und alle 9 Bilder der anschließenden Tastaturserie sind tatsächlich angesehen. Weiter und Zurück zeigen die richtigen letzten/ersten Einträge in allen drei Listen. Lange Namen werden begrenzt, Fokus und Checksymbole bleiben vorhanden. Die Druckerliste passt bereits in beiden Größen; ihre separate Singularform „1 Aufträge“ ist als weiterer Sprachbefund für den folgenden Providerstatusschritt festgehalten. Nachweishashes/Helper: `after2-viewed.json`, `keyboard-viewed.json` und zugehörige Dateien unter `evidence/r7/provider-comparison/`.

Die Mausserie ist vollständig exakt bereinigt. Die Tastaturserie bewahrte ebenfalls Konfigurationsbytes, andere Dateien, Räume, tatsächliche Standardlautstärke und alle Monitorparameter/Fokuszuordnungen; keine Provideraktion wurde angefordert, alle eigenen Wrappers/Marker wurden entfernt. Die rohe Outputgleichheit ist dabei ausdrücklich **false**, da die flüchtige ID von `drm-1` von 7 auf 8 wechselte. Der gesonderte `output-identity-audit.json` bestätigt alle übrigen Outputfelder unverändert. Die Ursache dieses ID-Wechsels ist nicht bestimmt und wird nicht als behobener Lebenszyklusfehler ausgegeben.

Performance: pro Redraw werden nur die sichtbaren bounded Zeilen erzeugt; keine neue I/O-, Animations-, Decode- oder Capture-Schleife und kein zusätzlicher Assetcache. Die bestehenden einmaligen Providerworker und Symbolcaches werden genutzt. Neue quantitative R7-Messreihe sowie vollständige Monitor-/Scale-/Fehlerabdeckung stehen noch aus; V23 und V32 bleiben offen. Keine Gesamtfreigabe.

### V37 – fehlerhafte Providerantworten erscheinen als leer oder null (R7, offen)

Am installierten Stand 02318d1225ae69000f25e82ff4581636716726d4deb3b1c1a66ca2f6f907b232 in sechs tatsächlichen nativen Bildern bei 100/200 Prozent reproduziert und vollständig angesehen: fehlgeschlagenes bluetoothctl devices erscheint als leere Geräteliste, strukturell ungültiges erfolgreiches wpctl status als aktiver Audio-Dienst mit null Geräten und fehlgeschlagenes lpstat -o als null Druckaufträge. Die eigenen Fixtures betreffen ausschließlich lesende Antworten. Sämtliche Dateien, Räume, Outputs und reale Standardlautstärke exakt erhalten; eigene Fixtures entfernt. Hashes, Helper und Cleanup in evidence/r7/provider-errors-comparison/. Die bereits sichtbare Singularform 1 Aufträge wird im selben Sprach-/Statusschritt korrigiert.

#### V37: unbekannte Statuswerte getrennt, installiert und visuell geprüft

`audio/wpctl.rs` verlangt die vorhandenen Sinks-/Sources-Abschnitte, bevor eine Antwort als aktiver Audiodienst gilt. Ein gültiger leerer Abschnitt bleibt leer; fehlende/ungültige Struktur bleibt unbekannt. `bluetooth.rs` unterscheidet erfolgreiche Geräte-/Kopplungs-/Verbindungsabfragen von Lesefehlern und liest Adapterflags ohne erfundene Aus-Werte. `content/bluetooth.rs` zeigt für fehlende Gerätedaten den ausdrücklichen unbekannten Status. `printers.rs` erhält erfolgreiche leere Abfragen getrennt von unbekannter Drucker-/Standard-/Auftragsliste; Auftragszahlen sind optional. Der Druckdienst wird nur aus bestätigten Statusantworten abgeleitet.

`content/printers.rs`, `audio_system_widgets.rs` und der gemeinsame `provider_paging.rs` verwenden diese Statuswerte und die korrekte Singularform „1 Druckauftrag“. Die vorhandenen Layout-/Provider-/Netzwerktestfixtures wurden an die internen typisierten Zustände angepasst; neue Parsertests prüfen unbekannt gegen erfolgreich leer sowie teilweise fehlgeschlagene Bluetooth-/Druckerabfragen. Kein neues IPC, keine Dependencies und keine Geräte-Schreibaktion. Paint nutzt nur den vorhandenen gecachten Snapshot und vorhandene Zeilen-/Symbolcaches; keine neue I/O-/Frame-Schleife.

162 normalisierte LF-Quellen ohne Hashabweichung. `cargo fmt --all -- --check`, `cargo check --workspace`, `cargo test --workspace`, Design Guard, striktes All-Target-Clippy und Releasebuild erfolgreich; 1301 Tests bestanden, 0 Fehler, 1 bestehender D-Bus-Ignore. Rustdateien höchstens 600 physische Zeilen. Logs, Quellhashes und Aktivierung unter `evidence/r7/provider-errors-gates/`.

Release/installierte/laufende Shell identisch `267a1ca509cb199bf5b4893af935909cc8129eb92d75a4b612eb7e598ff4ab44`, Watchdog PID 531259→539034. Alle sechs Nachherbilder bei 100/200 Prozent tatsächlich angesehen: Bluetooth- und Audiolesefehler ausdrücklich unbekannt, Druckaufträge insgesamt und je Drucker unbekannt statt null. Alle Cleanupfelder einschließlich roher Outputgleichheit sind true, keine eigenen Fixtures/Marker/Testfenster bleiben. Bildhashes, Helper und Cleanup unter `evidence/r7/provider-errors-comparison/`. Die separate Skalierungsweichheit V23 ist weiterhin sichtbar. Vollständige Monitor-/Scale-/Leerzustandsmatrix und aktuelle R7-Performance bleiben offen; keine Aussage über erfolgreiche echte Geräteänderungen aus diesen ausschließlich lesenden Fixtures.


#### R7: vollständige Provider-/Monitor-/Skalierungsmatrix angesehen

Auf dem installierten V37-Stand `267a1ca509cb199bf5b4893af935909cc8129eb92d75a4b612eb7e598ff4ab44` alle 82 nativen Bilder tatsächlich einzeln angesehen: beide Outputs als primäres FHD-Ziel bei 100, etwa 140,6, 150 und 200 Prozent. Je Fall Systemübersicht, Netzwerk, Bluetooth, Audio, Drucker, Energie, Benutzer und Updates; bei den drei nicht ganzzahligen beziehungsweise höheren Skalierungen zusätzlich jeweils die zweite Netzwerk-/Bluetooth-/Audioseite. Die 140,6-Prozent-Probe erreicht 1366 logische Pixel Breite, ersetzt keinen physischen 1366×768-Modusnachweis.

Acht eigene Profile, zehn eigene WLANs, acht eigene Bluetoothgeräte, je acht Audioein-/ausgänge und acht Drucker werden innerhalb der verfügbaren Fläche dargestellt. Auf dem Mindestcanvas sind alle letzten Einträge über Weiter erreichbar. Lange Namen bleiben begrenzt, Statusunterzeilen und Auswahl-/Fokusrollen sind getrennt; Druckaufträge verwenden die korrekte Singularform. Energieaktionen wurden nicht ausgeführt. Konten/Systemübersicht verwenden reale Daten, Fedora-Updates benennt weiterhin ehrlich die fehlende Integration. Die Skalierungsweichheit V23 bleibt sichtbar und offen.

Helper, alle 82 Bildhashes mit tatsächlich angesehen-Status, Fallmetadaten und Cleanup unter `evidence/r7/provider-matrix/`. Private Konto-/Rechnerbilder verbleiben im Git-ignorierten lokalen Prüfverzeichnis; der Nutzer hat den lokalen Abruf ausdrücklich freigegeben. Der Abschluss bestätigt alle Felder einschließlich roher Outputgleichheit: Konfigurations-/Panel-/MIMEbytes, Räume, Outputparameter/Fokus, tatsächliche Standardlautstärke erhalten, keine Fenster oder eigenen Providerwrappers/Marker und keine Geräte-Schreibaktion. Die zuvor beobachtete vorübergehende SSH-Unterbrechung unterbrach den nativen Durchgang nicht; Ursache unbekannt. V32 wurde weiterhin durch den dokumentierten Watchdog-Neustart umgangen und ist dadurch nicht behoben. Leer-/Aus-/Fehler-/Überlaufzustände sowie aktuelle R7-Performance werden anschließend geprüft; R7 insgesamt bleibt in Arbeit.

#### R7: 32 Leer-/Aus-/Fehler-/Überlaufbilder tatsächlich angesehen

Auf demselben V37-Release alle vier Provider bei 100 und 200 Prozent in vier Zuständen einzeln angesehen. Bluetooth, Audio und Drucker unterscheiden bestätigte leere Listen von Lesefehlern; Bluetooth nennt den ausgeschalteten Adapter, Drucker den angehaltenen Dienst. Audio hat hier keinen eigenen Aus-Zustand; die entsprechende Probe zeigt einen nicht verfügbaren Dienst. Zwölf eigene Einträge je Liste werden ehrlich durch die bestehenden Grenzen begrenzt (acht Profile/Bluetooth-/Audiogeräte/Drucker, zehn WLANs); vorhandene Weiter-Seiten bleiben erreichbar. Keine Aussage über erfolgreiche echte Geräteänderungen aus diesen lesenden Fixtures.

Alle 32 Bildhashes, Fallmetadaten, Helper, Fixture-Preflight und Cleanup unter `evidence/r7/provider-states/`. Alle Cleanupfelder einschließlich roher Outputgleichheit sind true; Konfiguration, Panel, MIME, Räume und tatsächliche Standardlautstärke unverändert, eigene Wrappers entfernt. Die separate Skalierungsweichheit V23 bleibt offen.

### V38 – Netzwerk-Leerzustand und Listenlesefehler sprachlich vermischt (R7, in Arbeit)

Die acht Netzwerkbilder der vorherigen Zustandsrunde belegen denselben unbestimmten Hinweis für erfolgreich leere und fehlgeschlagene Listenabfragen. Es wird kein erfundenes Gerät angezeigt, aber der Benutzer kann die Zustände nicht unterscheiden. `network/lists.rs` erhält jetzt den Erfolg beider vorhandenen nmcli-Abfragen getrennt und verwirft strukturell ungültige Antworten als unbekannt. Erfolgreich leere Antworten bleiben ausdrücklich leer. Zwei Parsertests decken fehlgeschlagene, leere, teilweise verfügbare und fehlerhafte Antworten sowie escaped Namen ab.

Der vorhandene einmalige Settingsworker übergibt den typisierten Status über Shellcache und Contentbuilder bis zur Netzwerkseite. Die alten Quick-Settings-Abfragen bleiben best effort; deren Vektoren begründen keinen bestätigten Listenstatus. Keine zusätzlichen Abfragen, Timer, Render-I/O, Assets, Dependencies oder privilegierten Aktionen. Geometrie, Seitenwechsel und ursprüngliche Aktionsindizes bleiben erhalten.

165 normalisierte LF-Quellen ohne Hashabweichung; Format, Workspacecheck, Workspacetests, Design Guard, striktes All-Target-Clippy und Shellrelease erfolgreich. 1303 Tests bestanden, 0 Fehler, 1 bestehender D-Bus-Ignore. Alle Rustdateien höchstens 600 Zeilen; lokaler Diffcheck erfolgreich. Release/installierte/laufende Shell identisch `71f02242b1ad814f36d23b7996cc37b270b0b2cfd852ac7476da140878e46ef1`, Watchdog PID 549781→557208. Belege unter `evidence/r7/network-status-gates/`. Native gezielte Nachprüfung und Performance noch offen; keine Gesamtfreigabe.

#### V38: Zeilentexte bestätigt, zusätzlicher Zählerwiderspruch gefunden

Alle acht Netzwerk-Nachherbilder der vier Zustände bei 100/200 Prozent tatsächlich angesehen. Erfolgreich leere Listen und Lesefehler haben getrennte Hinweise; zwölf Einträge bleiben ehrlich begrenzt. Zusätzlich alle 14 Netzwerkbilder auf beiden primären FHD-Outputs bei 100/140,6/150/200 Prozent angesehen, einschließlich zweiter Seite der kleineren Canvasfälle. Keine neue Layoutabweichung; bestehende V23-Weichheit sichtbar. Belege in `evidence/r7/network-status-after/` und `network-status-matrix/`; beide Cleanups vollständig true einschließlich roher Outputgleichheit.

Die vier zusätzlich angesehenen Teilfehler-/Strukturfehlerbilder in `evidence/r7/network-status-structure/` zeigen: ungültige erfolgreiche Antworten bleiben unbekannt; nur die fehlgeschlagene WLAN-Abfrage ist unbekannt, das erfolgreich gelesene eigene Profil bleibt angezeigt. Allerdings behauptet der Footer in diesem Teilfehlerfall „0 WLANs“ und verwendet „1 Profile“. V38 bleibt deshalb offen. Die Korrektur in `provider_paging.rs`/`content/network.rs` erhält unbekannte Zähler als optional und verwendet Singularformen; ein neuer Semantiktest prüft unbekannt, bestätigte Null, Eins und sichtbare Gesamtbegrenzung. Automatische Gates und native Nachprüfung des ergänzten Releases folgen. Der Strukturtest selbst ist vollständig bereinigt; seine Cleanupfelder sind alle true.

#### V38: ergänzter Zählerfix vollständig geprüft und installiert

165 normalisierte LF-Quellen ohne Hashabweichung; Format, Workspacecheck, Workspacetests, Design Guard, striktes All-Target-Clippy und Shellrelease erfolgreich. 1304 Tests bestanden, 0 Fehler, 1 bestehender D-Bus-Ignore; alle Rustdateien höchstens 600 Zeilen und Diffcheck grün. Belege unter `evidence/r7/network-count-gates/`. Release/installierte/laufende Shell identisch `0cb602769cb1b62172e3b9acfa1742c94ee6edc74873fbfdfcccc9c72b7a47fd`, Watchdog PID 563685→571069. Ausschließlich sprachlicher Zählerfix, keine zusätzliche Providerabfrage oder neue Render-/Cachearbeit. Native Prüfung beider Teilfehlerrichtungen und ungültiger Antworten bei 100/200 Prozent läuft; V38 und R7 bleiben bis zu diesen Belegen beziehungsweise der Performanceprüfung offen.

#### V38: sechs finale Zählerbilder tatsächlich angesehen

Auf dem ergänzten Release alle sechs Bilder bei 100/200 Prozent einzeln angesehen: ein erfolgreiches Profil bei fehlgeschlagener WLAN-Abfrage zeigt „1 Profil · WLAN-Anzahl unbekannt“; ein erfolgreiches WLAN bei fehlgeschlagener Profilabfrage zeigt „Profilanzahl unbekannt · 1 WLAN“. Ungültige erfolgreiche Antworten beider Abfragen zeigen unbekannte Listen ohne erfundene Null. Keine neue Layoutabweichung; V23 bleibt getrennt offen. Alle Cleanupfelder einschließlich roher Outputgleichheit sind true, Shell anschließend ohne Fixtures PID 572165. Belege, sechs Bildhashes und Helper unter `evidence/r7/network-count-after/`. V38 ist im dokumentierten Status-/Zählerumfang korrigiert, geprüft und installiert. Die quantitative R7-Messserie startet danach; R7 insgesamt bleibt bis zu ihrem Ergebnis in Arbeit.


#### R7: aktuelle 40-Zyklen-/3×300-s-Serie abgeschlossen

Auf Shellrelease `0cb602769cb1b62172e3b9acfa1742c94ee6edc74873fbfdfcccc9c72b7a47fd`, Shell PID 572165 und unverändertem Compositor PID 1371 vierzig native Control-Center-Zyklen durchlaufen. Dabei Übersicht, Cursor, Anzeige, Netzwerk, Bluetooth und Audio sowie Benutzer, Updates und Einstellungen geöffnet; jeweils vollständig geschlossen. Anschließend 30 Sekunden Warm-up und drei volle Ruheproben von jeweils mindestens 300 Sekunden, ohne GUI-/Buildaktivität. Alle Zustands- und Konfigurationsvergleiche sowie Panel-/MIMEbyteasserts und Prozessidentitäten bestanden. Belege und reproduzierbare Helper unter `evidence/r7/performance/`; private Desktop-Snapshots bleiben im ignorierten Testverzeichnis, ihre Quellhashes sind im Export enthalten.

| Probe | Shell CPU, % eines Kerns | Compositor CPU, % eines Kerns | verfügbare DRM-Engine, % | Shell RSS vorher/nachher, Bytes | Compositor RSS, Bytes |
| --- | --- | --- | --- | --- | --- |
| 1 | 0,1867 | 1,1367 | 0,00717 | 181661696 / 181661696 | 181932032 konstant |
| 2 | 0,1833 | 1,1600 | 0,00721 | 181661696 / 181661696 | 181932032 konstant |
| 3 | 0,1833 | 1,1367 | 0,00728 | 181661696 / 181530624 | 181932032 konstant |

Gegenüber der dokumentierten R6-Serie bleibt die CPU deutlich innerhalb des +0,5-Prozentpunkte-Budgets. GPU-Zahl ist die aus deduplizierten DRM-Client-Enginezeiten abgeleitete verfügbare Metrik, keine Messung der gesamten GPU. Kein RSS-Zuwachs im Leerlauf. Zwischen Bedienzyklus 20 und 40 hält der Compositor dieselben 181932032 Bytes, die Shell wächst von 179433472 auf 181661696 Bytes (+2228224 Bytes = 2,125 MiB). Daraus wird kein unbegrenzter Speicheraufbau abgeleitet und auch keine bereits bestandene lange Lebensdauerprüfung behauptet: dieser Punkt bleibt ausdrücklich für R9 offen. Eingabe-bis-sichtbar-Latenz wurde hier nicht gemessen.

R7 ist im dokumentierten Darstellungsumfang visuell geprüft und installiert. Echte Geräte-Schreib-/Hardwarefälle, V23, V32 und weitere R9-Zustände bleiben offen; Fedora-Paketintegration wird weiterhin nicht als umgesetzt behauptet. Keine Desktop-Alpha-/Gesamtfreigabe. Nächster Reparaturschritt ist R8, zunächst die tatsächlichen fünf Vorher-Aufnahmen auf einem eigenen temporären Entwurf mit bytegenauer Wiederherstellung des vorhandenen Nutzerentwurfs.

### R8 / V18 – fünf Vorher-Seiten reproduziert und tatsächlich angesehen

Am installierten R7-Release alle fünf First-Run-Seiten über Control Center → System → Übersicht → Einführung geöffnet. Eigener temporärer Entwurf ohne Räume/Profiländerung; kein Übernehmen, kein Verwerfen des Nutzerentwurfs. Die Bilder bestätigen fünf gleich gewichtete Footerbuttons, seitenbreite Pseudo-Checkboxbuttons, vermischte Raum-/App-Zeilen und überholte technische Rückwegtexte. Originalgröße der bisherigen Leistenpreview ist am FHD-Canvas zufällig erhalten; der alte Renderpfad skaliert sie sonst auf verfügbare Breite.

Alle fünf Bilder tatsächlich angesehen; Belege, Bildhashes, Helper und Cleanup unter `evidence/r8/before/`. Cleanup vollständig true: originaler First-Run-Nutzerentwurf und alle weiteren Dateien bytegleich, Räume/Outputs gleich, keine Fenster. Laufende R7-Shell danach PID 578734. R8 ist in Arbeit; keine Behauptung bereits bestandener Nachher-Prüfung.

Die Korrektur trennt Kopf/Fortschritt, Inhaltsgruppen, Navigation und sichere Ausstiegsaktionen. Ein geborgter nativer Checkbox-/Radio-Zeilenbaustein in niwoe-ui verwendet zentrale Tokens sowie vorhandene Fokus-, Font- und Symbolquellen. Fünf Aktionsindizes und sämtliche Draft-/IPC-/Übernahmegrenzen bleiben erhalten. Einführung erhält einen begrenzten zentralen Inhaltsbereich; Leistenvorschau wird passend zu dessen Breite aus dem vorhandenen gecachten Produktpfad erzeugt und in Originalgeometrie gezeichnet. Keine neue Dependency, Assets, Animation, Timer, Providerabfrage oder Render-I/O. Automatische Gates und echte Nachher-/Bedien-/Matrix-/Performanceprüfung folgen.

#### R8: erster Nachherrelease und fünf tatsächlich angesehene Seiten

171 LF-normalisierte Quellen ohne Hashabweichung; Format, Workspacecheck, 1306 bestandene Tests, 0 Fehler, 1 bestehender isolierter D-Bus-Ignore, Design Guard, Source Size/Centralization Guards, striktes All-Target-Clippy und Shellrelease bestanden. Alle Rustdateien ≤600 Zeilen; lokaler Diffcheck ebenfalls bestanden (separat im normalen Workspace ausgeführt). Release/installierte/laufende Shell identisch `0aaf2a655e12cf66e264fe0a0ff9264e1bc47dbede2e2ddb22ad12ce1eb2faf7`, Watchdog 578734→586443. Belege unter `evidence/r8/gates/`.

Alle fünf Nachherbilder tatsächlich angesehen; `evidence/r8/after/` enthält Metadaten, Helper, Bildhashes und vollständig wahres Cleanup. Danach Shell PID 587366. Überschriften/Fortschritt, echte Radios/Checkboxen, separate Raum-/Appgruppen, klare Hauptaktion, sichere Entwurfausstiege und native Leistenpreview erkennbar. Zwischen Fortschrittsleiste und Gruppenüberschrift bleibt der vertikale Abstand zu knapp. V18 ist deshalb weiterhin in Arbeit. Die gemeinsame Inhaltsgeometrie erhält zusätzliche 16 Tokenpixel und der Gruppentitel den passenden zentralen 24-Pixel-Abstand; nach diesem kleinen Folgefix werden alle Gates und Nachher-/Bedienprüfungen erneut ausgeführt. Die bereits aufgenommenen Bilder bleiben historische Nachweise des ersten Standes.

#### R8: Abstandsfix geprüft, installiert und fünf finale Seiten angesehen

Der kleine Folgefix ist erneut vollständig geprüft: 171 LF-normalisierte Quellen ohne Hashabweichung; Format, Workspacecheck, 1306 bestandene Tests, 0 Fehler, 1 bestehender isolierter D-Bus-Ignore, Design Guard, Source Size/Centralization Guards, striktes All-Target-Clippy und Shellrelease bestanden. Alle Rustdateien höchstens 600 Zeilen. Release/installierte/laufende Shell identisch `39e9bc71e4cb8775aca1c505cf4f5fabc21490bb5e07ae42f6787dde3a4de765`, Watchdog 587366→594655. Gates und Quellidentität unter `evidence/r8/spacing-gates/`.

Alle fünf finalen FHD-Seiten tatsächlich angesehen. Der Gruppenabstand ist korrigiert, Radios/Checkboxen und ihre sekundären Texte sind lesbar, die Hauptaktion bleibt eindeutig; sichere Entwurfausstiege sind getrennt. `evidence/r8/spacing-after/` enthält Bildhashes, Metadaten, Helper und vollständig wahres Cleanup. Originaler Nutzerentwurf und andere Dateien bytegleich, Räume und Outputs unverändert, keine Fenster; Shell danach PID 595583. Monitor-/Scale-Matrix, tatsächliche Auswahl-/Entwurf-/Übernahmewege und aktuelle R8-Performance bleiben zu diesem Zeitpunkt offen.

Performance-Modell: die neuen Zeilen zeichnen ausschließlich bei bestehender Eingabe-/Dateninvalidierung mit den vorhandenen begrenzten Font-/Symbolcaches. Die Einführungsvorschau nutzt denselben Produktcache mit einem Raster von höchstens 1280×48×4 = 245760 Bytes, wird in Originalgeometrie gezeichnet und beim Schließen verworfen. Keine zusätzlichen Timer, Providerabfragen, Decodes, Capture-Schleifen oder Render-I/O; keine Änderung der Compositor-Stapelreihenfolge. Die neue Geometrie wird einschließlich deaktivierter Hitregionen und nicht überlappender Aktionsgruppen auf Mindestcanvas/FHD/UHD automatisch geprüft. Der reale Draft-/Journal-/IPC-Vertrag bleibt erhalten.

#### R8: 40 Monitor-/Scale-Aufnahmen tatsächlich angesehen

Alle fünf Seiten auf beiden Outputs als primärem FHD-Ziel bei 100, etwa 140,6, 150 und 200 Prozent tatsächlich einzeln angesehen. Gruppen, echte Auswahlzeilen, separate Aktionsflächen und native Leistenpreview bleiben innerhalb der verfügbaren Fläche. Die 140,6-Prozent-Probe erreicht 1366 logische Pixel Breite; kein physischer 1366×768-DRM-Modus wird behauptet. Die bekannte Skalierungsweichheit V23 ist weiterhin sichtbar. Im `drm-1`-Fall mit 140,6 Prozent fällt außerdem die kleinere Paneldarstellung gegenüber dem entsprechenden `drm-0`-Bild auf; tatsächliche Surface-/Buffer-/Output-Skalierung ist deshalb Bestandteil der offenen R9-Skalierungsprüfung, keine bereits bestandene Schärfe-/Scale-Freigabe.

Belege, 40 tatsächlich angesehen-Status/Bildhashes, Metadaten und Helper in `evidence/r8/matrix/`. Sämtliche Cleanupfelder einschließlich roher Outputgleichheit sind true: ursprüngliche Konfiguration/First-Run-/Panel-/Raum-/MIMEdateien exakt erhalten, ursprüngliche Monitorparameter/Fokus wiederhergestellt, keine Fenster. Shell danach PID 603463, identischer finaler R8-Release. V32 wurde durch den dokumentierten Watchdog-Neustart umgangen und ist dadurch nicht korrigiert. Native Auswahl-/Entwurf-/Übernahmeprüfung und quantitative R8-Serie folgen; keine Gesamtfreigabe.

#### R8: native Maus-/Tastatur-/Entwurf-/Übernahmerunde geprüft

Am finalen R8-Release alle 19 Bedienbilder tatsächlich angesehen. Einstieg ausschließlich über die sichtbaren Control-Center-Wege; Tastatur über Hub → Räume verwalten → F8 → System → Übersicht → Einführung. Drei Profiloptionen, eigener optionaler Raum und echte native Appauswahl, Apppaging und exakte Entwurfwiederaufnahme nach Schließen und Watchdog nachgewiesen. Tastatur-Zurück erhält die Appwahl; bewusstes Überspringen entfernt nur den eigenen optionalen Raumentwurf. Leistenmodul per Tastatur ausgeblendet, Reihenfolge per Maus geändert und unverändert wiederaufgenommen. Alle drei Übungen wurden tatsächlich geöffnet und ihr bestätigter Übungsstatus gespeichert.

Die bestätigte Übernahme betrifft ausschließlich eine eigene neue Leistenkonfiguration ohne zusätzliche Räume oder Appstart. Der native dauerhafte Übernahme-/Journalpfad bestätigt diese Leiste und den Abschluss. Danach nur die bytegenau eigene neue Leistendatei entfernt, ursprünglichen Nutzerentwurf bytegleich wiederhergestellt und Shell per Watchdog erneuert. Sämtliche Cleanupfelder true: First-Run-Bytes, ursprünglich fehlende Leistendatei, andere Dateien, Räume, Outputs und fehlende Fenster. Shell danach PID 606592, identischer finaler R8-Release. Sanitierte Fallmetadaten, reproduzierbare Helper, Ergebnis, Cleanup und 19 Bildhashes unter `evidence/r8/functional/`; private Rohentwürfe/Bilder bleiben im ignorierten Prüfverzeichnis.

`scripts/test-first-run-ui.py` berücksichtigt jetzt die tatsächliche Control-Center-Canvasgeometrie, getrennte fünf Aktionsflächen, aktuelle Auswahl-/Paging-/Leistenzeilen und den nachgewiesenen Tastatureinstieg. Der Schutz für die isolierte Testumgebung bleibt erhalten. Python-Syntaxprüfung mit eigenem ignoriertem Bytecodecache bestanden; keine Behauptung eines neu ausgeführten vollständigen historischen Isolationsskripts. Keine zusätzlichen Ruständerungen nach dem vollständig geprüften finalen Release.

#### R8: erster Lesefehler-Bildnachweis verworfen

Die zwei Bilder unter `evidence/r8/invalid-state/` tatsächlich angesehen: beide zeigen ausschließlich den Desktop. Die Helperannahme, ein Watchdog-Neustart öffne automatisch die Fehlerseite der Einführung, war falsch; er ist keine neue Login-Sitzung. Die bytegenaue Wiederherstellung und fehlenden produktiven Schreibvorgänge sind belegt, deaktiviertes Weiter und sichtbarer Lesefehler damit ausdrücklich nicht. `viewed.json` markiert beide visuellen Nachweise als fehlgeschlagen. Der Fall wird nach der laufenden Performance-Serie mit dem expliziten sichtbaren Control-Center-Einstieg wiederholt. Kein Produktfix allein aus dieser falschen Prüfannahme.

#### R8: aktuelle 40-Zyklen-/3×300-s-Serie abgeschlossen

Am finalen R8-Release vierzig vollständige native Fünfseitenzyklen über Control Center → System → Einführung durchlaufen, jeweils Entwurf sicher geschlossen; keine bestätigte Übernahme während der Messung. Eigener Entwurf vor der ersten Ruheprobe entfernt und ursprüngliche First-Run-Datei bytegleich wiederhergestellt. Shell PID 608124 und Compositor PID 1371 während aller Zyklen und Proben unverändert. Anschließend 30 Sekunden Warm-up und drei volle Ruheproben ≥300 Sekunden ohne GUI-/Buildaktivität. Alle Konfigurations-/Panel-/Raum-/MIMEbytes, Desktopzustände und Prozessidentitäten bestätigt. Nach der Messung eigener gecachter Entwurf per Watchdog entfernt; Cleanup vollständig true, Shell danach PID 611037.

| Probe | Shell CPU, % eines Kerns | Compositor CPU, % eines Kerns | verfügbare DRM-Engine, % | Shell RSS, Bytes | Compositor RSS, Bytes |
| --- | --- | --- | --- | --- | --- |
| 1 | 0,1933 | 1,1567 | 0,00727 | 131805184 konstant | 182460416 konstant |
| 2 | 0,1833 | 1,1667 | 0,00728 | 131805184 konstant | 182460416 konstant |
| 3 | 0,1900 | 1,1667 | 0,00727 | 131805184 konstant | 182460416 konstant |

Gegenüber R7 bleibt CPU innerhalb des +0,5-Prozentpunkte-Budgets; kein RSS-Aufbau während der Ruheproben. Zwischen Bedienzyklus 20 und 40 wächst die Shell von 129576960 auf 131805184 Bytes (+2228224 = 2,125 MiB), der Compositor von 175087616 auf 182460416 Bytes (+7372800 = 7,03125 MiB). Diese Werte bleiben ausdrücklich Gegenstand der langen R9-Lebensdauerprüfung, keine Behauptung unbegrenzten Wachstums oder bereits bestandener Langzeitprüfung. DRM-Zahl ist die verfügbare deduplizierte Client-Enginezeit, keine gesamte GPU-Messung; Eingabe-bis-sichtbar-Latenz unbekannt. Sanitierte Metriken, Quellhashes der privaten Rohsnapshots, Helper und Cleanup unter `evidence/r8/performance/`.

### V39 – unverständliche englische Parsermeldung in der Einführung (R8, offen)

Die zwei tatsächlich angesehenen Wiederholungsbilder unter `evidence/r8/invalid-state-retry/` zeigen die echte Einführung nach dem sichtbaren Control-Center-Einstieg: Weiter deaktiviert, nur sicherer Ausstieg und Neuladen verfügbar, keine produktiven Schreibvorgänge. Alle Cleanupfelder true, ursprünglicher Nutzerentwurf bytegleich, Shell danach PID 612185. Damit ist die falsche Prüfannahme des ersten Versuchs korrigiert. Der native Hinweis lautet jedoch „expected value at line 1 column 1“ und erklärt keine sinnvolle Benutzeraktion. Dieser neu bestätigte Textbefund wird noch in R8 korrigiert.

`first_run::Wizard::report_error` zeigt für ein nicht validiertes Dokument eine deutsche Ladefehlermeldung mit dem sicheren Neuladen-Weg und dem Hinweis, dass vorhandene Einstellungen erhalten bleiben. Bestätigte Zustände behalten ihre spezifischen Konflikt-/Teilabschlussmeldungen; lokale Entwürfe und Aktionsgrenzen bleiben unverändert. Der bestehende IPC-Empfänger nutzt diese Methode. Ein Semantiktest prüft erhaltenen lokalen Entwurf, ausgeschaltete Übernahmefortsetzung, ausschließlich Ausstieg/Neuladen bei unbekanntem Zustand sowie erhaltene Konfliktmeldung und Übernahmefortsetzung eines bestätigten Teilabschlusses. Keine Designwerte, I/O, Timer, Cache oder Renderreihenfolge verändert; Text entsteht nur bei der vorhandenen IPC-Antwort. Vollständige Rust-Gates, Releaseinstallation und tatsächliche Nachherbilder folgen.

#### V39 korrigiert, installiert und tatsächlich nachgeprüft; R8 im dokumentierten Umfang geschlossen

172 normalisierte LF-Quellen ohne Hashabweichung. Format, Workspacecheck, Workspacetests, Design Guard, Source Size/Centralization Guards, striktes All-Target-Clippy und Shellrelease bestanden: 1307 Tests, 0 Fehler, 1 bestehender isolierter D-Bus-Ignore. Alle Rustdateien ≤600 Zeilen; lokaler Diffcheck grün. Belege unter `evidence/r8/error-gates/`. Release/installierte/laufende Shell identisch `98864dc2a01163b005b0bdc2ba580609fe73d19710a57d69c64a8fa6c7182c62`, Watchdog 612185→619499.

Beide finalen Fehlerbilder tatsächlich angesehen: verständliche deutsche Rückmeldung mit Neuladen-Weg, Weiter deaktiviert, sicherer Ausstieg verfügbar. Neuladen und Klick auf deaktiviertes Weiter verändern die eigene ungültige Datei nicht; ursprünglicher Nutzerentwurf anschließend bytegleich wiederhergestellt. Sämtliche Cleanupfelder true, Shell danach PID 620286. Bildhashes, Metadaten, Helper und Cleanup unter `evidence/r8/error-after/`. V39 im dokumentierten Ladefehlerumfang geschlossen.

R8 ist im dokumentierten Layout-/Bedien-/Wiederaufnahme-/eigenen Übernahme-/Lesefehlerumfang geprüft und installiert. Die 40 Monitor-/Scale-Aufnahmen und quantitative 40-Zyklen-/3×300-s-Serie stammen vom Darstellungsrelease `39e9bc71…`; der abschließende Release verändert ausschließlich die Fehlertextwahl bei vorhandener IPC-Antwort und deren Test, keine Material-, Asset-, Vorschau-, Invalidierungs-, Timer- oder Cachepfade. Diese Messung wird deshalb nicht als neue Messung des abschließenden Binärhashes bezeichnet und nicht ohne sachlichen Anlass wiederholt. V23, V32, der Speicheraufbau während Bedienzyklen, weitere Fehler-/Hardware-/Schutzflächen und sämtliche übrigen R9-Pflichtfälle bleiben offen. Keine Desktop-Alpha-/Gesamtfreigabe. Nächster Schritt R9: vorhandene Layer beim laufenden Primärmonitorwechsel tatsächlich reproduzieren und an der gemeinsamen Zuordnungsquelle reparieren.

### R9 / V32 – aktueller Primärwechsel erneut reproduziert

Auf installierter Shell `98864dc2…` und unverändertem Compositor `c09985e3…`, PID 1371, den zweiten Monitor über die native Anzeigeseite als primär gespeichert. Alle drei aktuellen Bilder tatsächlich angesehen: Anzeigeseite vor dem Wechsel, danach nur Wallpaper am neuen UHD-Primärmonitor, weiterhin vollständiges Panel/Control Center am alten FHD-Monitor. Kein Watchdog zwischen Auswahl und diesen Bildern. Konfiguration und Snapshot bestätigen die Auswahl; V32 ist ein realer laufender Zuordnungsfehler. Danach ursprüngliche Config/Panel/First-Run/Raum/MIMEbytes und rohe Outputs/Fokus exakt wiederhergestellt, keine Fenster. Cleanup vollständig true, Shell danach PID 621223. Belege, Hashes, Helper und ausschließlich Outputmetadaten unter `evidence/r9/primary-before/`.

Call-Flow: implizite Layer werden nur beim Erzeugen dem damaligen primären Output zugeordnet. `reconcile_layer_shell_outputs_after_output_change` ordnet vorhandene Layer nicht um; außerdem endet `reapply_output_layout` nach direkter Registry-/Smithay-Anpassung ohne diesen gemeinsamen Reconcile-Aufruf. Der erste kleine R9-Fix erhält daher die implizite Zuordnungsentscheidung als typisierte Layer-Metadaten und migriert diese Layer beim Primärwechsel mit den vorhandenen Smithay-Unmap/Map-/Arrange-/Configure-Grenzen. Explizite Outputwahlen bleiben erhalten. Kein Renderstack-Umbau, Timer, Capture, Decode oder neue Dependency. Automatische Gates, Installation und tatsächlicher Nachher-/Rückwechsel-/Lebenszyklusnachweis folgen; V32 bleibt bis dahin offen.

#### R9: Primärwechsel-Fix geprüft/installiert; tatsächliche Aktivierung noch ausstehend

175 normalisierte LF-Quellen ohne Hashabweichung. Format, Workspacecheck, Workspacetests, Design Guard, Source Size/Centralization Guards und striktes All-Target-Clippy bestanden: 1309 Tests, 0 Fehler, 1 bestehender isolierter D-Bus-Ignore. Der erste Releaseaufruf baute nur die Compositor-Bibliothek und Shell; der noch alte Root-Programmhash wurde bei der Identitätsprüfung erkannt. Historische Logs/Hashes sind ausdrücklich als `release-initial-lib-only` erhalten, keine Behauptung eines damals neuen Compositors. Der korrigierte Aufruf `cargo build --release -p niwoe -p niwoe-shell` hat das tatsächliche Startprogramm neu gebaut, Quellhashes erneut bestätigt und den alten Root-Hash ausdrücklich ausgeschlossen. Belege unter `evidence/r9/primary-gates/`.

Endgültig geprüft/installiert: Compositor `8f1e15e3678860e9160cc00ee17e61c0650e9c15ae1ecb086fef53e48c3290e3`, Shell `010d2ac4b1afd1fe1b4f27270ee1ef1b07b019b4c97a0df9a21580031837da17`. Die noch laufende Sitzung verwendet Compositor PID 1371/`c09985e3…` und Shell PID 621223/`98864dc2…`. Neuer NIWOE-Login ausdrücklich angefordert; ein Shell-Watchdog ersetzt den Compositor nicht. V32 bleibt bis zur tatsächlichen neuen Prozessidentität und nativen Nachher-/Rückwechselprüfung offen.

Der Nachherhelper ist einschließlich unveränderter Once-pro-Login-Markierung,
neutraler Loge und regulärem Hub nach Watchdog vorbereitet und auf Fedora
syntaxgeprüft, aber nicht ausgeführt. Unabhängige lesende Vorbereitung für
Skalierung, Cache-Lebensdauer und weitere Pflichtfälle unter
`evidence/r9/PREPARATION.md`; keine zusätzliche Ruständerung daraus abgeleitet.
Während der weiterhin ausstehenden Anmeldung eigene temporäre
Eingabegerätefreigabe nach Vergleich gegen fremde Änderungen entfernt;
ursprüngliche `/dev/uinput`-ACL exakt bestätigt. Bereinigungsbeleg unter
`evidence/r9/primary-gates/input-cleanup-awaiting-login.json`. Keine laufenden
Builds oder Prüfhelper, keine behauptete Nachher-/Gesamtfreigabe.

Performance-Modell: kleine typisierte Zuordnungsmetadaten leben genau so lange wie die zugehörige Layeroberfläche. Migration wird ausschließlich bei bestehenden Output-/Konfigurationsänderungen ausgeführt, mit begrenzter Iteration über vorhandene Outputs/Layer und geklonten Handles. Smithay sendet beim tatsächlichen Unmap/Map die bestehenden Leave/Enter- und Arrange-Ereignisse; der gemeinsame Reconcile-Pfad konfiguriert anschließend die endgültigen Geometrien. Keine neue Frame-/Idle-Schleife, Assets, Decodes, Captures, Timer oder Änderung der Layerrollen/Renderstack-Reihenfolge. Automatische Policytests prüfen wechselnde Primärwahl bei gleichzeitig erhaltener expliziter Wahl sowie fehlenden Output/leere Registry; sie ersetzen ausdrücklich keinen echten Map-/Login-/Hotplug-Nachweis.

### Aktuelle Repository-Galerie, Nutzerauftrag vom 03.10.2026

Neun aktuelle native Bilder aufgenommen und tatsächlich angesehen; fünf davon
als unveränderte PNGs in `assets/screenshots/` aufgenommen und in der README
eingebunden. Hub über Wallpaper, Raumverwaltung und Raumkonfiguration mit
wirklicher KWrite-Vorschau eines eigenen Beispieldokuments, native
Hintergrundeinstellungen sowie Leistenkonfiguration. Der erste vermeintliche
Settings-Einstieg zeigte wegen einer falschen Helperkoordinate noch System;
dieses Bild bleibt ausschließlich im ignorierten Prüfverzeichnis. Die echte
Settings-Aufnahme wurde anschließend mit dem sichtbaren Sidebarziel wiederholt
und angesehen. Kein System-/Hostnamebild in der Galerie.

Auf der zusätzlich aufgenommenen Hubansicht über der hellen echten KWrite-App
scheint Anwendungstext weiterhin sichtbar durch; dieses Bild wurde nicht für
die Designgalerie ausgewählt. Der bestehende V02-/R9-Kontrastprüfauftrag bleibt
damit offen, keine neue Behauptung einer bestandenen Material-Gesamtprüfung.
Der laufende Compositor ist weiterhin `c09985e3…`, Shell `98864dc2…`.
Der erforderliche neue Login für den installierten R9-Compositor steht aus.

Beide Aufnahmeläufe bestätigten bytegleiche Config-/Panel-/Raum-/First-Run-/
MIME-Dateien, unveränderte Outputs und aktiven Raum, keine verbleibenden Fenster
und unveränderte Prozessidentitäten. Das eigene KWrite-Fenster ist geschlossen;
die eigene temporäre Eingabegerätefreigabe wurde nach einem Vergleich gegen
fremde ACL-Änderungen entfernt, ursprüngliche ACL exakt bestätigt. Alle fünf
ausgewählten PNGs erfolgreich dekodiert, 1920×1080 Pixel, Hashes bytegleich zur
Aufnahme, 23 lokale Markdownziele vorhanden und README-Diffcheck grün.
Galerie insgesamt 4337967 PNG-Bytes; Herkunft und laufende Hashes in
`assets/screenshots/captures.json`. Keine Rust-/Produktänderung, keine erneute
Rusttestserie erforderlich. README kennzeichnet den unfertigen Entwicklungsstand
ausdrücklich; die Galerie ersetzt keine offene R9-Abnahme.

### R9 / V32 – neue Sitzung und tatsächlicher Nachherlauf

Nach der wirklichen neuen Anmeldung vom 03.10.2026 ist Compositor
`8f1e15e3…`, PID 647751, tatsächlich aktiv. Mit Shell `010d2ac4…`
beide Primärwechsel über die native Anzeigeseite geprüft: Panel und Control
Center wandern zum neuen UHD-Primärmonitor, der ehemalige FHD-Primärmonitor
zeigt Wallpaper; der native Rückwechsel ergibt die umgekehrte Zuordnung.
Kein Prozessneustart zwischen den Wechseln. Alle neun Bilder des Haupt-
und ergänzenden Sitzungsprüflaufs tatsächlich angesehen. Neutraler Desktop
nach Watchdog ohne erneute Begrüßung sowie regulärer `Super+Space`-Hub bestätigt;
Loginmarkierung unverändert. Config-/Panel-/First-Run-/Raum-/MIME-Dateien und
Outputs/Fokus exakt wiederhergestellt, keine Fenster. Eigene temporäre
Eingabegeräte-ACL anschließend entfernt, Original bestätigt.

Der Haupthelper endete nach erfolgreicher Wiederherstellung mit Exit 1:
sein eigener Config-Guard verwendete noch die veraltete Variable des
temporären Stands. Das ist als historischer Fixturefehler erhalten, kein
behaupteter vollständig bestandener Helperlauf. Die fehlenden letzten zwei
Sitzungsbilder wurden im getrennten Helper mit Exit 0 nachgeholt. Hashes,
Metadaten, tatsächliche historische Helper und Cleanup unter
[primary-after](evidence/r9/primary-after/README.md).
V32 ist im dokumentierten laufenden Hin-/Rückwechselumfang geschlossen;
dies ersetzt keine physische Hotplugprüfung oder R9-Gesamtfreigabe.

### R9 – Nutzerkorrektur: ruhigere Controls und einfachere Navigation

Die ausdrückliche Designkorrektur vom 03.10.2026 wird vor dem weiteren
R9-Abschluss bearbeitet. Gemeinsame Controls verwenden neutrale Auswahlflächen
und einen hellen neutralen Fokus, keine goldenen unteren Auswahlstriche.
Mindesthöhe 40 px, Formfelder 44 px; größere Innenränder und Abstände.
Sieben globale Sidebarziele, alle Settings im Control Center, Dateien und
Wiederherstellung beim jeweiligen Raum. Doppelte Übersicht, unverfügbare
globale Backups/Logs, drei unverfügbare Schnellaktionen und unnötige einzelne
Untertabs entfernt. Bestehende Provider und echte Speicher-/Dateipfade erhalten.
Sechs Appauswahlen liegen in zwei Spalten mit drei ausreichend hohen Zeilen.
Leistenmodule und Raum-/Hubauswahl verwenden dieselben neutralen Flächen.

180 normalisierte Quellen ohne Hashabweichung. `cargo fmt --all -- --check`,
`cargo check --workspace`, `cargo test --workspace` (1311 bestanden, 0 Fehler,
ein bestehender isolierter D-Bus-Ignore), Design-/Größen-/Zentralitätsguards,
striktes All-Target-Clippy und Shell-Releasebuild auf Fedora bestanden.
Die echten initialen Mindestlayoutfehler und die alte Gold-Fokus-Testannahme
wurden korrigiert; ihre historischen Logs bleiben getrennt erhalten.
Installierte und über `/proc` bestätigte Shell:
`e9bed1136f1dc3c014a498f90b03fd56e531fae687163a0279ab7c383a703451`,
PID 678998. Compositor unverändert `8f1e15e3…`, PID 647751.
Watchdogaktivierung ohne neuen Login, NIWOE/MIME/KDE/GTK-Dateien bytegleich.

19 aktuelle native FHD-Bilder tatsächlich einzeln angesehen, Aufnahmehashes
bestätigt: Desktop, Hub/Fokus, alle sieben Hauptziele, Hintergrund mit geladenen
Vorschauen, Anzeige, Cursor, Audio, Netzwerk, Raum-Allgemein/Apps/Dateien.
Maus-/Tastaturnavigation und Sidebar-Wrap nachgewiesen, alle Cleanupfelder true.
Erster Bildversuch unmittelbar nach Aktivierung endete vor einer Aufnahme am
Zustimmungsdialog mit Timeout; bereinigt und getrennt dokumentiert. Der
Wiederholungslauf auf stabiler Shell endete erfolgreich. Kein Schutzdialog
umgangen, keine gespeicherten Nutzereinstellungen verändert. Belegübersicht
einschließlich geänderter Verantwortungen und Performance-Modell unter
[calm-controls](evidence/r9/calm-controls/README.md).

Weitere 60 Monitor-/Scalebilder und acht ergänzende Scrollbilder dieses
endgültigen Release tatsächlich einzeln angesehen und gegen Aufnahmehashes
geprüft: FHD/UHD bei 100/150/200 Prozent, alle sieben Sidebarseiten,
Raum-Allgemein/Apps und die unteren Formularbereiche bei kleiner logischer
Größe. Der ursprüngliche Matrixhelper hatte für den Zusatzscroll das falsche
Vorzeichen; diese Bilder bleiben erhalten und werden nicht als Unterseiten-
Nachweis ausgegeben. Der getrennte Ergänzungslauf bestätigt die tatsächliche
Erreichbarkeit. Beide Läufe Exit 0, sämtliche Cleanupfelder true, Original-
dateien und roher Sitzungs-/Outputsnapshot identisch, keine Prozessneustarts.
Eigene temporäre Eingabegerätefreigabe nach Guardvergleich entfernt,
Original-ACL exakt bestätigt. Insgesamt 87 Bilder auf dem aktuellen Release.
Einige lange Erläuterungen werden im kleinen Formular weiterhin gekürzt;
native hochauflösende Schrift und maßstabsgetreue Mindestzielgrößen unterhalb
des bisherigen Mindestcanvas sind durch diese Geometrieprüfung nicht belegt.

Keine neue Animation, Timer-/Frame-/Idle-Schleife oder unbeschränkter Cache.
Auswahlhäkchen verwenden den vorhandenen begrenzten Symbolcache; der frühere
Streifenpfad entfällt. Quantitative lange Speicher-/Ruheprüfung des aktuellen
Binärstandes weiterhin offen. Frühere Bilder und Messungen werden ausdrücklich
nicht als erneute Abnahme der jetzt geänderten visuellen Gates ausgegeben.
V23 (weiche skalierte Shellschrift), V02, weitere Fokus-/Neben-/Schutzflächen,
Hardwarefälle und R9-Gesamtabnahme bleiben offen; P13 unbegonnen.
