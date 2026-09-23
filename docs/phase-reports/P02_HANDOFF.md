# NIWOE – aktueller Handoff, 23.09.2026

**Nutzerfreigabe:** „Sieht gut aus. Lassen wir so.“ bestätigt auch die gedämpfte
Light-Neufassung. Auch die [Deck-Mutationen](P05_DECK_MUTATIONS.md) sind nach
Installation mit „passt, schaut gut aus“ bestätigt. Beide Themes bleiben bestehen.
Aktueller nächster Schritt: [P06 Raumgrundlage](P06.md).

Fortschritt: [sichtbarer Raumeditor](P06_ROOM_EDITOR.md) umgesetzt und geprüft;
Installation und DRM-Bedienabnahme verfolgen. Dark bleibt ausgewählt.

Die folgenden Abschnitte dokumentieren frühere Zwischenstände; ausstehende
Light-/Deck-Freigaben darin sind durch die obige Nutzerabnahme erledigt.

**Light-Neufassung:** Die zentrale Light-Palette verwendet jetzt gedämpftes
Stein-/Salbeigrau, dunkle Schrift und Messing. Auf Fedora als Benutzertheme unter
`~/.config/niwoe/themes/light/theme.toml` installiert und per nativer IPC aktiviert.
Die Datei ist bytegleich mit `themes/light/theme.toml`; bei künftigen systemweiten
Theme-Updates muss diese vorrangige Benutzerkopie ebenfalls aktualisiert werden.
Dark, Geometrie und Materialparameter unverändert. Keine zusätzlichen Renderpässe
oder laufenden Berechnungen: bestehender Theme-Reload invalidiert die Farb-Caches.
`cargo fmt --all`, Linux `cargo check --workspace --locked` und
`cargo test --workspace --locked` bestanden, einschließlich Design-Guard und
Kontrastprüfungen. Aktuelle Kontrastminima in `docs/design/P02_COMPONENTS.md`.
Visuelle Nutzerabnahme der neuen Palette steht aus; das bisherige Light-Bild zeigt
weiterhin den verworfenen Stand. Danach bleiben Deckzustände/Fehlerrückmeldung offen.

**Aktueller Nutzerstand:** Dark-Material und Panel-/Launcher-Anordnung nach
Installation/Neustart positiv bestätigt. Textzentrierung und größere Uhr ebenfalls
bestätigt. Light wurde bei der Abnahme als zu grell abgelehnt; zuerst die helle
Palette als gedämpften Gegenpol überarbeiten. Danach Deckzustände/Fehlerrückmeldung.
Viewportmatrix (1024/1366/1920, beide Paletten, Räume 1/5/9) mit zentrierter Uhr
und getrennten Klickzielen bestanden; `target/panel-acceptance/` check/test grün.
Das ist Layoutprüfung, kein physischer DRM-Moduswechsel. Aktiver DRM-Ausgang
meldet nur 1920×1080/60 Hz. Light-Livebild unter P05 `accepted-light-deck.png`
ist ein Prüfbeleg, ausdrücklich keine Nutzerfreigabe des Light-Designs.

**Neueste Nutzerpriorität:** [Panel nach Mockup](PANEL_MOCKUP_CORRECTION.md).
Andere UI-Arbeiten sind bis zur visuellen Panelabnahme zurückgestellt.

**Vorrangiger offener Auftrag:** [Dialog-Klickfehler und Materialbefund](P05_INPUT_MATERIAL_FINDINGS.md).
Nutzer meldet nicht bedienbare Screenshot-Zustimmung über Settings (Escape geht)
und lehnt die bisherige Mockup-Nähe ab. Input-Fix auf Fedora geprüft und
installiert: 1.098 Tests grün, fmt/check/clippy/Release bestanden. Alle sechs
Programme bytegleich, KDE-/GTK-Konfiguration unverändert. SSH-Schlüsselzugang
funktioniert. Nach Neuanmeldung neue Buildidentität bestätigt; Nutzer bestätigt
erfolgreichen Mausklick auf Erlauben über Settings. Zusätzlich Capturefehler
gefunden: Aufnahmen erfolgten vor Glasauflösung. Reihenfolge korrigiert,
Folgeschritt-Gates/Installation im verlinkten Bericht verfolgen.
Die 1.095 grünen Tests unten gelten ausschließlich für den früheren Symbolstand.

**Aktuellster Schritt:** [Deck-Symbolaktionen und Bluetooth](P05_DECK_SYMBOLS.md).
1.095 Tests grün, fmt/check/clippy/Release bestanden, installiert. Shell per
Watchdog erneuert (17228), Compositor 8017 erhalten. Echte Deck-Aufnahme bei
1920×1080/100 %/Dark geprüft. Bluetoothstatus plus Geräteverwaltung, Anzeigezugang,
stabiler Tastaturfokus bei asynchronem Status. Details/Restarbeiten im Bericht.
Nächster Schwerpunkt: bestätigte Backendmutationen mit Pending-/Fehlerzustand;
danach übrige Mockup-/HiDPI-/Performance-Nachweise. Immer installieren.

**Vorheriger Schritt:** [Deck-Komposition](P05_DECK_COMPOSITION.md), auf Fedora
geprüft: fmt/check/test/clippy grün, 1.094 Tests bestanden, 1 ignoriert.
Dark-/Light-Rasterbelege vorhanden. Release am 23.09. installiert, alle sechs
Binaries bytegleich, KDE-/GTK-Dateien unverändert. Rechner am Greeter; neuer Stand
beim nächsten NIWOE-Login, Deck über Super+Escape. Nutzerauftrag: geprüfte Stände
immer installieren (auch in AGENTS.md). Die folgenden älteren Installationsangaben
sind vom 22.09.
Weiterarbeit: Deck-Symbolaktionen und übrige dokumentierte Mockup-Lücken;
P02–P05 bleiben in-progress.

Repository: `D:/300_Projekte/310_Aktiv/niwoe-desktop`, Branch `codex/niwoe-p00`.
P02–P05 **in-progress**, nicht visuell abgenommen. Aktueller Bericht:
[P03–P05 Native Shell](P03_P05_NATIVE_REBUILD.md). Die älteren Abschnitte unten
sind historische Evidenz und keine Aufforderung, abgeschlossene Tests zu wiederholen.

## Verbindlicher Auftrag und Design

- Bestehenden nativen Rust-Unterbau weiterverwenden; Panel, Launcher und Deck neu bauen.
- Alle acht Originalmockups unter `assets/` sind visuell verbindlich. Abweichungen
  nur bei konkreter technischer Notwendigkeit oder expliziter Nutzerkorrektur.
- Launcher/Suche dürfen sich ausdrücklich an Apple Spotlight orientieren.
- Kein Alltagsbranding, keine angehefteten Programmsymbole im Panel.
- Tiling und Omarchy-Bedienung, keine compositor-eigene Titelleiste, nur 1–2px Kontur.
- Designwerte zentral in Tokens/Config; keine neuen Dependencies.

## Aktueller installierter Stand

Acer: Fedora, `drm-0`, 1920×1080, 100 %, Dark. Nutzer ist angemeldet.
Letzte Änderung installiert und die Shell über den Watchdog neu geladen;
Compositor-Sitzung blieb bestehen. Kein weiteres Login angefordert.
Alle sechs installierten Programme bytegleich zum letzten Release. Der bereits
laufende Compositor stammt aus dem vorherigen Release; die aktuelle Korrektur
betrifft Shell-Darstellung, nicht das IPC-Protokoll. Vollständiger Release ab
nächstem regulärem Sitzungsstart. Gemeinsame KDE-/GTK-Konfiguration unverändert.

Panel ohne App-Pins, mit Raumkontur, konsistenten Statussymbolen, Akku-Prozentwert
und einzeiliger Datums-/Uhranzeige. Launcher: Suchzeile nach Spotlight-Vorbild,
App-Ergebnisse, Auswahl, Tastaturhinweise. Deck mit echten Backendaktionen und
Tab-/Pfeilbedienung. Quelländerungen und offene Einschränkungen stehen im Bericht.

## Verifikation und Belege

Linux: fmt, workspace check, workspace test (**1.093 bestanden, 0 fehlgeschlagen,
1 ignoriert**), Design-Guard, Clippy `-D warnings`, Release erfolgreich.
Live-Screenshots der letzten Panel-/Launcher-Korrektur angesehen und dauerhaft
unter [docs/design/evidence/P03](../design/evidence/P03/README.md) abgelegt.
Der Deck-Beleg ist eine native Rastervorschau, kein aktueller Live-Screenshot.

## Nächster Arbeitsschritt

Am bestehenden Mockup-Abgleich weiterarbeiten; keine freie Ersatzgestaltung und
keine weitere Alt-UI-Testschleife. Noch offen: vollständige visuelle Übereinstimmung
(u. a. Deck/Wallpaper), benannte Räume, vollständige Omarchy-Belegung/Kürzelhilfe,
Tastaturnavigation in Settings/WLAN, weitere Suchanbieter und Startfehleranzeige,
Light-/HiDPI-/Idle-Nachweise. Technisch noch nicht implementiert bedeutet nicht
technisch unmöglich. Keine dieser Lücken als akzeptierte Designabweichung ausgeben.

Nutzer hat Commit und Push des gesamten Arbeitsstands beauftragt; Ziel ist der
bestehende Branch im GitHub-Remote `github`. Keine Zugangsdaten einchecken.

## Historischer Stand vor der Scope-Korrektur

**Aktualisierung 22.09.:** Scale-Korrektur im noch uncommitteten Arbeitsbaum
implementiert, auf Fedora vollständig geprüft und installiert. Details und
Dateiliste: [P02_SCALE_FOLLOWUP.md](P02_SCALE_FOLLOWUP.md). Abschließende Gates:
Format, Check, **1.092 Tests (2 ignoriert)**, Clippy, Release und isolierter
Nested-Smoke grün. Alle sechs installierten Binaries bytegleich zum Release.
Neue Belege: `target/p02-evidence/scale-final.tar.gz` lokal und `scale-final/`
auf Acer. KDE blieb aktiv; als erster koordinierter Hardware-Testschritt wurde
der Nutzer gebeten, sich regulär von KDE abzumelden. Auf Bestätigung warten.
Die folgenden älteren Befunde beschreiben die Ausgangslage; der Code-Fix aus
Schritt 2 ist umgesetzt, die echten DRM-Nachweise aus Schritt 3/4 stehen aus.

Produktcommits:

- `3546c05`: zentrale Paletten, Maße, native Komponenten, Kontrast/Galerie.
- `afd7756`: verbleibende gemeinsame Widgetmaße aus Tokens.
- `02d029b`: Font-Fallback bei fehlenden UI-Glyphen; auf Hardware gefundener Fix.

Letzter Linux-Stand: Format, Check, 1.089 Tests (2 ignoriert), Clippy und Release
bestanden. Alle sechs Binaries auf dem Acer entsprechen dem letzten Build.
Acer: `eduard@192.168.1.203`, `/home/eduard/niwoe-desktop`, kein Git-Checkout.
Zuletzt wieder **KDE**, fünf gemeinsame KDE-/GTK-Dateien unverändert.
Keine Zugangsdaten in Dateien sichern; vorhandene Anmeldung/Agent verwenden.

## Nächste notwendige Arbeit

1. `AGENTS.md`, Plan P02 und Bericht lesen; aktuellen Git-Status prüfen.
2. Echten Scale-Fehler modular beheben: Outputname in NIWOE **drm-0**, nicht
   KDEs eDP-1. Bei 150/200 % halbiertes Panel / abgeschnittene Settings. Relevante
   Pfade: `niwoe-compositor/src/backend/drm/render.rs` (Scale 1.0), Output-Layout,
   Shell-Buffer-/Layer-Konfiguration. Keine P03-Neupositionierung vorziehen.
3. Änderung mit vollständigen Linux-Gates, finalem isoliertem Nested-Smoke und
   echten Output-Screenshots prüfen. Die Offscreen-Galerie ist bereits korrekt,
   ersetzt diesen Integrationsnachweis jedoch nicht.
4. Relevante native Performance nachweisen. Die vorhandenen drei Nested-Reihen
   zeigen ~99,72 % Compositor-CPU auch beim alten Release; Shell 0,04–0,07 %.
   Keine erfundenen GPU-/Input-to-present-Werte. Renderbenchmark ist unverändert.
5. Tatsächlichen Gesamtdiff und Nachweise prüfen, erst dann P02 accepted setzen.

## Testbedienung

Der Nutzer möchte **genau einen manuellen Schritt pro Frage** und knappe Updates.
Screenshot-Erlaubnis wurde erteilt. Der direkte Testpeer kann Aufnahmen erzeugen,
lässt aber die Shell-Consent-Fläche sichtbar; solche Bilder nicht als saubere
Referenzen ausgeben. Regulärer Shell-Buttonpfad oder vorhandenes Screencopy
verwenden. Keine Sperrtests oder Sitzungswechsel ohne Koordination.

Belege: `target/p02-evidence/` lokal und Acer; repräsentative PNGs und Mess-JSON
sind in `docs/design/evidence/P02/` gesichert. Die ersten Dateien mit Namen
`light-1920x1080-150/200.png` sind **ungültige Scale-Nachweise** (falscher Outputname).
`light-real-150/200.png` zeigen den tatsächlichen Fehler, allerdings mit sichtbarer
Consent-Fläche. Vor einem neuen Lauf tatsächliche Output-Snapshots lesen.
