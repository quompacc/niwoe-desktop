# P04 Hub- und Control-Center-Zwischenstand

Stand: 25.09.2026

Status: **in Arbeit**. Hub, „Räume verwalten“ und „Raum konfigurieren“ sind auf
dem Fedora-Testrechner installiert und visuell geprüft. Maus- und
Tastaturnavigation sowie der wiederholte Overlay-Lifecycle sind live getestet.
Der physische 1366×768-Bildvergleich und weitere Zustände sind noch offen.

## Fortschritt am 25.09.2026

- Raumkarten in „Räume verwalten“ öffnen jetzt die vollständige native
  Konfigurationsseite des gewählten Raums. Breadcrumb, Sidebar-Zurück,
  Abbrechen und `Esc` führen zur Verwaltung zurück.
- Die Seite enthält Bildkopf, Reiter, Raumdetails, Kontext und Wiederherstellung,
  Start-Apps, Automatisierungsregeln, Vorschau, Erklärung und Speicherleiste
  entsprechend Mockup `(4)`. Die Vorschau zeigt den tatsächlichen Raumnamen
  und bis zu drei reale offene Fenstertitel. Das vorhandene NIWOE-Wallpaper wird
  einmalig dekodiert und nur bei ereignisgetriebenem Neuzeichnen skaliert.
- Name und Präsentationsreihenfolge verwenden die bestehenden
  `RoomChange::Rename`- und `RoomChange::Move`-Befehle mit Revision und
  bestätigter Persistenz. Speichern kehrt erst nach erfolgreicher Bestätigung
  zur Verwaltung zurück; Konflikt, Fehler und Timeout bleiben am Formular
  sichtbar. Während einer ausstehenden Mutation ist Abbrechen gesperrt.
- Beschreibung, Restore, Start-Apps und Regeln sind in der Mockup-Struktur
  sichtbar, aber ehrlich als noch nicht verfügbar gekennzeichnet. Hierfür gibt
  es derzeit keine wirksamen Schalter oder erfundene Daten.
- `target/p04-room-configuration-preview.png` wurde nativ gerastert und gegen
  das Originalbild `(4)` angesehen. Ein anfänglich falsch platzierter
  Bildausschnitt wurde korrigiert. Der installierte Livevergleich steht noch aus.
- Die neue Seite ist in `room_management_view/configuration.rs` und
  `configuration/body.rs` getrennt. Die gewachsene Shell-Initialisierung wurde
  verhaltensgleich in `wayland/init/shell_state.rs` ausgelagert, damit die
  verbindliche Grenze von 600 Zeilen je Rust-Datei eingehalten wird.

Prüfungen auf Fedora: `cargo fmt --all -- --check`, `cargo check --workspace
--locked`, `cargo test --workspace --locked -q` einschließlich Design- und
Source-Size-Guard, `cargo clippy --workspace --all-targets --locked -- -D
warnings` und `cargo build --release -p niwoe-shell --locked` sind grün.
Das Release liegt unter `target/release/niwoe-shell`; Installation über
`target/p04-room-configuration/install.sh` ist vorbereitet und benötigt die
sudo-Passworteingabe im Nutzerterminal. Release-SHA-256:
`b637fcf2c8d902133a59e4612d3b920bd22e48bf1c6f244e833abb13e72d5083`.

Verbindliche Referenzen:

- `assets/ChatGPT Image 21. Sept. 2026, 17_13_54 (2).png`: Hub.
- `assets/ChatGPT Image 21. Sept. 2026, 17_13_54 (3).png`: Räume verwalten.
- `assets/ChatGPT Image 21. Sept. 2026, 17_13_54 (4).png`: Raum konfigurieren,
  nächster sichtbarer Arbeitsschritt.

## Festgelegter Ablauf

1. Bei jeder neuen NIWOE-Login-Sitzung öffnet der Hub genau einmal als
   Willkommensansicht. Eine atomar erzeugte Sitzungsmarke unter
   `$XDG_RUNTIME_DIR/niwoe/` verhindert ein erneutes Öffnen bei einem
   Shell-/Watchdog-Neustart derselben Sitzung.
2. Danach öffnen `Super+Space` und die linke Panel-Schaltfläche denselben Hub.
   Eine zusätzliche Launcher-Oberfläche existiert nicht.
3. Tippen im Hub wechselt innerhalb derselben Fläche in die Suche. Das erste
   `Esc` kehrt zum Hub zurück, das zweite schließt ihn.
4. Der Hub führt sichtbar zu „Räume verwalten“. Die Verwaltungsseite bleibt
   unter dem persistenten Panel und besitzt die vollständige linke Sidebar.
5. Als Nächstes öffnet eine Raumkarte „Raum konfigurieren“. Speichern oder
   Abbrechen führt ohne Verlust des gewählten Raums zurück.

## Lieferumfang

- `Super+Space` und die linke Panel-Schaltfläche öffnen die große,
  raumzentrierte Hub-Übersicht nach dem verbindlichen Mockup `(2)`.
- Die vier ersten Räume erscheinen als Hauptkarten. Namen, aktiver Raum und
  Fensterzahlen stammen aus dem laufenden Shell-Zustand.
- Die unteren Bereiche zeigen echte offene Fenster und Systemdaten. Der mittlere
  Bereich ist als „Schnellhilfe“ ausgeführt und erklärt die tatsächlich
  verfügbaren Tasten: Tippen, Pfeile, `Enter` und `Esc`.
- Die Panel-Lupe öffnet die Suche direkt. Erstes `Esc` verlässt die Suche und
  kehrt zum Hub zurück; das nächste `Esc` schließt den Hub. Raumkarten wechseln
  den Raum über den vorhandenen IPC-Pfad.
- Text in Hub, Schaltflächen, Raumkarten und Fußbereichen wird anhand realer
  Fontmetriken zentriert. Der Materialpfad verwendet das bereits freigegebene
  Glas des Systemdecks; Hub und alter Launcher werden nie gleichzeitig gezeigt.
- „Räume verwalten“ besitzt Header, Suche, Filter, Sortierung, ein
  dreispaltiges Raster der neun aktuellen Räume sowie Schnellaktionen und
  Statistiken. Namen, aktiver Zustand, Fensterzahlen und Fenstertitel stammen
  aus dem Shell-Zustand.
- Der anfangs sichtbare doppelte Panelabstand wurde korrigiert. Der Hub belegt
  bewusst die volle Ausgabe; das Control Center respektiert die reservierte
  Panelhöhe ohne zusätzlichen Rand.

## Visuelle Abnahme am 24.09.2026

- Das Hub-Material wurde nach einer Transparenzregression korrigiert und vom
  Nutzer bestätigt.
- Die Verwaltungsseite wurde nach Live-Screenshot bei Abstand und
  Textzentrierung korrigiert; der Nutzer bestätigte den verbesserten Stand.
- Die Hub-Typografie wurde anschließend ebenfalls anhand der Fontmetriken
  korrigiert. Die Schnellhilfe und das Tippen-zur-Suche-Verhalten wurden
  ausdrücklich positiv bestätigt.
- Der automatische Hub-Start pro Login-Sitzung wurde installiert und vom Nutzer
  als gewünschter Abschluss bestätigt.

Die Sichtprüfung dieses Zwischenstands gilt für Dark auf dem vorhandenen
Fedora-Testrechner. Sie ist keine vollständige P04-Abnahme.

## Implementierungspfade

- `crates/niwoe-shell/src/hub_view.rs` und `hub_view/{alignment,hit}.rs`:
  Hub-Komposition, Fontmetrik, Schnellhilfe und Trefferflächen.
- `crates/niwoe-shell/src/room_management_view.rs` und
  `room_management_view/cards.rs`: vollständige Verwaltungsgrundfläche,
  Seitenleiste, Raumkarten und Zustandsdarstellung.
- `crates/niwoe-shell/src/first_login.rs` und `main.rs`: einmalige
  Hub-Aktivierung pro Login-Sitzung über die atomare Laufzeitmarke.
- `crates/niwoe-shell/src/wayland/{handlers,init,render,state}`: gemeinsamer
  Overlay-Lifecycle, Tastatur und Zeiger, Layer-Geometrie, Umschaltung zwischen
  Hub, Suche und Verwaltung sowie Weitergabe des Shell-Zustands.
- `crates/niwoe-compositor/src/backend/drm/render/scene_composition.rs`:
  vorhandenes Glas nur für den vollflächigen Hub; das Control Center beginnt
  unter dem Panel und erhält keine irrtümliche zweite Hub-Behandlung.
- `crates/niwoe-tokens/src/{hub,control_center}.rs` und `lib.rs`: zentrale
  Geometrie-, Abstands- und Typografiewerte für beide Flächen.
- `AGENTS.md`, `NIWOE_IMPLEMENTATION_PLAN.md`,
  `docs/{niwoe_design_manifest,MOCKUP_WORKFLOW_PLAN,NIWOE_INTERACTION_MODEL}.md`:
  verbindliche Bildquelle, Ablauf, Sitzungsstart, Status und nächster Schritt.

Die Renderreihenfolge bleibt Teil der Korrektheit: Das Panel liegt als eigene
Top-Layer-Fläche über dem Control Center. Der Hub überspannt die Ausgabe mit
`exclusive_zone(-1)`; die Verwaltung verwendet `exclusive_zone(0)` und keinen
zusätzlichen oberen Rand. Dadurch entsteht genau eine Panelhöhe Abstand.

## Performance-Modell

Hub und Control Center werden ausschließlich bei Öffnen, Eingabe,
Zeigerzustandswechsel oder Zustandsinvalidierung neu gezeichnet. Es gibt keine
Animation und keine Dekodierung pro Frame. Glas und Blur bleiben im bestehenden
Compositorpfad; der Shell-Renderer zeichnet nur tokenisierte Flächen, Text und
einfache Vektorsymbole. Die Prüfung des Login-Starts besteht aus genau einem
atomaren Dateierzeugungsversuch beim Shell-Start und verursacht keine Idle-Arbeit.

## Verifikation

- `cargo fmt --all -- --check`: grün auf Fedora.
- `cargo check --workspace --locked`: grün auf Fedora.
- `cargo test --workspace --locked`: grün; einschließlich 355 Shell-Tests,
  Hub-/Control-Center-Render- und Hit-Tests, Design-Guard und Source-Size-Guard.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: grün.
- `cargo build --release -p niwoe-shell --locked`: grün.
- Zieltest `cargo test -p niwoe-shell
  first_login::tests::marker_is_claimed_exactly_once --locked`: grün.
- Letzter Shell-Release: SHA-256
  `bfd73711a3c51278b92069f705707b4817f94f32a4cc632080ed6859e4711665`.
- Installationspaket des letzten Schritts:
  `target/p04-login-hub/install.sh`; Aktualisierung der laufenden Shell über den
  vorhandenen Watchdog. Gemeinsame KDE-/GTK-Konfiguration blieb erhalten.

## Bildbelege

- `target/p04-room-configuration-live.png`: Screenshot des installierten
  Konfigurationsstands am 25.09.2026 bei 1920×1080. Die Reiter und Karten
  begannen etwa 70 Pixel zu tief; unter den unteren Karten blieb ein großer
  ungenutzter Bereich bis zur Fußleiste.
- `target/p04-hub-material/live-hub.png`: installierter Hub-Materialstand.
- `target/p04-room-management-live-gap.png`: Live-Befund des doppelten Abstands.
- `target/p04-room-management-centered.png`: korrigierte Verwaltungsansicht.
- `target/p04-hub-live-text.png`: Live-Befund vor der Typografiekorrektur.
- `target/p04-hub-centered.png`: korrigierte Hub-Typografie und Schnellhilfe.

## Live-Nachweis 25.09.2026

- Der Konfigurationskopf verwendet nun eine eigene Höhe aus `niwoe-tokens`.
  Die unteren Karten nutzen die verfügbare Höhe. Release-Build und
  Workspace-Tests sind grün; Installation und zweiter Live-Screenshot sind
  geprüft.
- Nutzerauftrag vom 25.09.2026 umgesetzt: Der Compositor beginnt in der
  neutralen Loge, zeichnet und trifft dort keine Raumfenster und meldet per IPC
  keinen aktiven Raum (0). Hub, Panel und Raumverwaltung zeigen keine aktive
  Raumkarte. Raumwahl beendet die Loge; ein App-Schnellstart nutzt Raum 1 als
  Fensterziel. Die Loge ist kein zusätzlicher Raum. Gemeinsamer Release für
  `niwoe` und `niwoe-shell` ist gebaut und installiert. Der neue Login wurde
  geprüft. `target/p04-lobby/install.sh` installiert beide Binärdateien.
  Finaler Fedora-Stand: `cargo check --workspace --locked`,
  `cargo test --workspace --locked -q`,
  `cargo clippy --workspace --all-targets --locked -- -D warnings` und
  `cargo build --release -p niwoe -p niwoe-shell --locked` sind grün.
  SHA-256: `niwoe` `83e59b6809844acf1c0a0ed63f5951627c8de91209b3e2d4059ba0fb85fec4f6`,
  `niwoe-shell` `2aa405a18d7f0f4061645af5bbf2a8a72d586f1053abcacf32c508005ccd35b2`.
  Im Logen-Zustand werden Raumfenster nicht gerendert, per Zeiger getroffen
  oder durch die normale Tastaturnavigation fokussiert. Renderpfad: nur ein
  boolescher Szenenfilter; keine zusätzliche Idle-Arbeit, Bilddekodierung oder
  Animation.
  Am installierten Fedora-Desktop sind `/proc/1299/exe` und `/proc/1313/exe`
  bytegleich mit den beiden Releases. Der Screenshot
  `target/p04-lobby-live.png` zeigt nach neuer Anmeldung den Hub als Loge und
  keinen markierten Raum in der Leiste oder in den Hub-Karten. Nach einem
  Klick auf Raum 2 zeigt `target/p04-lobby-room2-live.png` ausschließlich
  Raum 2 als aktiv und keinen Hub mehr.
  `target/p04-room-configuration-corrected-live.png` zeigt die installierte
  Raumkonfiguration nach der Kopfkorrektur. Reiter und Karten beginnen wieder
  direkt unter dem Bildkopf. Die unteren Karten füllen die verfügbare Höhe,
  enthalten wegen der fehlenden Provider jedoch noch viel Leerraum; das ist
  keine vollständige Bildabnahme von `(4)`.
## Noch offen in P04

Am 25.09. wurde der installierte Hub-Raumkarten-Klick nach der
Zeiger-Grab-Korrektur selbst mit virtueller Hardwareeingabe geprüft:
`Super+Space` öffnete den Hub, ein Klick auf Raum 1 aktivierte Raum 1 und
schloss den Hub. Der native Beleg ist `target/p04-hub-room1-after-click.png`.
Die Panel-Raumwahl wurde ebenfalls mit Einzelklicks auf Raum 1 und Raum 2
bestätigt; siehe `PANEL_MOCKUP_CORRECTION.md`.

Der Mausablauf Hub → Raumverwaltung → Raum-2-Konfiguration → Abbrechen und
erneut Konfiguration → Speichern ohne Datenänderung → Raumverwaltung wurde
auf dem Fedora-Desktop durchgeführt. Native Belege:
`target/p04-room-management-flow-live.png`,
`target/p04-room-config-flow-live.png`,
`target/p04-room-cancel-flow-live.png` und
`target/p04-room-save-fixed-flow-live.png`.
Nach Nutzerfeedback zur gequetschten Konfiguration wurden Raumdetails und
Kontextzeilen erhöht; die drei Kontextzeilen überlappen sich jetzt nicht mehr.
Zurück-, Abbrechen- und Speichern-Button haben gleiche Höhe und Unterkante;
Speichern ist für seine Beschriftung breiter. Alle Maße stammen aus
`niwoe-tokens::ControlCenter`. Die Änderung erzeugt nur beim Öffnen oder
Neuzeichnen der Seite Rasterarbeit, keine Idle-Arbeit. Fedora Formatprüfung,
`cargo check --workspace --locked`, `cargo test --workspace --locked -q`,
Design-Guard, Clippy mit `-D warnings` und Release-Build sind grün.
`target/p04-room-config-layout-fixed-live.png` zeigt den installierten Stand.
`/usr/local/bin/niwoe-shell` und der laufende Prozess sind bytegleich mit
Release-SHA-256 `ed3f387bad1565bff836200f47726233e1b8466f936d06df91d217ceefca9841`.

Nach weiterer Nutzerkorrektur wurden die beiden Textzeilen der drei
Kontextkacheln als Block vertikal zentriert. Auch die Platzhaltertexte der
unteren drei Karten sitzen nun mittig im Bereich unter ihren Überschriften.
Die Vorschau ist zehn Pixel höher; dadurch beginnen und enden die drei
unteren Karten auf gleicher Höhe. Die Geometrie kommt aus
`niwoe-tokens::ControlCenter`, und ein Regressionstest prüft die bündigen
Kanten auch für kürzere und höhere Viewports. Das Rendering erfolgt nur bei
Seitenänderung, ohne zusätzlichen Idle-Pfad. Fedora Formatprüfung,
Workspace-Check und -Tests, Design-Guard, Clippy mit `-D warnings` und
Release-Build sind grün. Der installierte und laufende Build hat SHA-256
`65db799cfaf8adb57556ab163f07f37db91a04dfee719cd30af751ddb0ee882b`.
`target/p04-room-config-centered-live.png` ist der native Live-Beleg; die
korrigierte Konfigurationsseite blieb für die Sichtprüfung geöffnet.

Die Tastaturnavigation ist auf dem installierten Fedora-Release ebenfalls
geprüft: `Super+Space` öffnete den Hub, Tab markierte Raum 2 und danach
„Räume verwalten“ sichtbar; Enter öffnete die Verwaltung. Dort fokussierte
Tab Raum 1, und Enter öffnete dessen Konfiguration. Native Belege sind
`target/p04-keyboard-focus.png`, `target/p04-keyboard-manage3.png`,
`target/p04-keyboard-manage4.png` und `target/p04-keyboard-config.png`.
Die Auswahl verwendet nur vorhandene Fokus-, Farb- und Geometrietokens;
sie wird nur bei Eingabe neu gezeichnet und erzeugt keine Idle-Arbeit.
`cargo fmt --all -- --check`, Workspace-Check und -Tests, Design-Guard,
Clippy mit `-D warnings` und Shell-Release-Build sind auf Fedora grün.
Installierte Datei und laufender Watchdog-Prozess sind bytegleich mit dem
Release-SHA-256 `af1dbca0f0fdc2050a08066477d6670fddc79866b59d908c4df3fb4e5ee05360`.

Der vollständige Tastaturablauf in „Raum konfigurieren“ ist ebenfalls live
geprüft. Tab überspringt bei Raum 1 die deaktivierte Aktion „Früher“ und
markiert „Später“ mit einem sichtbaren Fokusrahmen. Ein temporärer Namensentwurf
`test` wurde über „Abbrechen“ verworfen; danach blieb „Raum 1“ erhalten.
Anschließend wurde `test` über das Namensfeld und „Änderungen speichern“
gespeichert und erschien in Raumverwaltung und Panel. Der ursprüngliche Name
„Raum 1“ wurde auf demselben Tastaturweg wiederhergestellt. Native Belege:
`target/p04-config-focus-later.png`, `target/p04-config-keyboard-draft.png`,
`target/p04-config-keyboard-cancel.png`, `target/p04-config-keyboard-saved.png`
und `target/p04-config-keyboard-restored.png`. Fedora Formatprüfung,
Workspace-Check und -Tests, Design-Guard, Clippy mit `-D warnings` und
Release-Build sind grün. Installierte Datei und laufender Watchdog-Prozess
sind bytegleich mit Release-SHA-256
`b5eb0a0f9ab6d7e3d32a404d40d28bfff3d0e29b617c9dddc6624c1ffc281fef`.
Die Fokuszeichnung erfolgt nur bei Eingabe und nutzt vorhandene Tokens.

Auf Nutzerwunsch wurden die „Konfigurieren“-Aktionen in den neun Raumkarten
als gleich große, neutral gefüllte Schaltflächen im Kartenfuß gestaltet.
Maus- und Tastaturfokus betonen ihre Kontur. Breite, Höhe, Radius und Farben
stammen aus `niwoe-tokens` und der aktiven Palette. Das zusätzliche Zeichnen
erfolgt nur bei Karten-Neuzeichnung; der Idle-Pfad bleibt unverändert.
`target/p04-config-button-live.png` zeigt das installierte Raster,
`target/p04-config-button-click.png` die über den Button geöffnete
Raum-1-Konfiguration. Fedora Formatprüfung, Workspace-Check und -Tests,
Design-Guard, Clippy mit `-D warnings` und Release-Build sind grün.
Installierte Datei und laufender Watchdog-Prozess sind bytegleich mit
Release-SHA-256
`85b08aca1812ec1964373e6d0523d5f8e7271cf5e7a641f304131cc354b9dccd`.
- Beschreibung und weitere Formularteile erst mit den vorgesehenen persistenten
  Backends wirksam machen; bis dahin bleiben sie sichtbar deaktiviert.
- Die sichtbaren Fähigkeiten „Neuer Raum“, Import, Export und Vorlagen erst mit
  den vorgesehenen Backends wirksam machen. Bis dahin bleiben sie ehrlich als
  nicht verfügbar gekennzeichnet; das Backend führt aktuell neun feste Räume.
- Suche um laufende Fenster, Räume und Einstellungsziele ergänzen.
- Reale Raumvorschauen und weitere Datenprovider anbinden.
Die drei P04-Ansichten wurden zusätzlich mit dem nativen Rust-Renderer bei
1366×720 logischen Inhaltspixeln geprüft (1366×768 inklusive 48-Pixel-Panel).
Der Fedora-Ausgang bietet derzeit nur 1920×1080 als physischen Modus an;
deshalb sind dies native Render-PNGs und keine Aufnahme eines umgeschalteten
Monitors. `target/p04-hub-1366-final.png`,
`target/p04-manage-1366-final.png` und
`target/p04-config-1366-final.png` belegen Hub, Verwaltung und den nach unten
gescrollten Konfigurationsbereich. Die Hub-Fußzeile reserviert jetzt genug
Platz; die kompakten Raumkarten zeigen Status und Fensterzahl oberhalb der
Schaltfläche ohne Überlappung. Die kurzen Platzhaltertexte bleiben in den
unteren Konfigurationskarten vollständig lesbar. Das Rendering geschieht nur
bei Ansichtänderung; die Höhenberechnung und Textwahl erzeugen keine Idle-Arbeit.
Fedora Formatprüfung, Workspace-Check und -Tests, Design-Guard, Clippy mit
`-D warnings` und Release-Build sind grün. Installierte und laufende Shell
sind bytegleich mit Release-SHA-256
`77fecdfbd2e2bced77cbd9caf1aa6db04f0bdbeea2a571e52cc6e28a3e3423fb`.

Der P04-Lifecycle wurde anschließend mit virtueller Hardwaretastatur auf dem
installierten Fedora-Release 20-mal durchlaufen: abwechselnd Raum 2 und Raum 1
per `Super+2`/`Super+1`, Hub per `Super+Space`, Verwaltung und Konfiguration per
Tab/Enter, dreimal `Esc` zurück zum Desktop. Der erste vollständige Probelauf
ist durch native Bilder für Hub, Verwaltung und Konfiguration dokumentiert:
`target/p04-cycle-1-hub.png`, `target/p04-cycle-1-management.png` und
`target/p04-cycle-1-configuration.png`. Im 20er-Lauf waren alle 20
Eingabefolgen ausführbar; Anfangs- und Endbild stimmen im relevanten
Desktopbereich bis auf einen mittleren RGB-Wert von 0,00 überein. Das Endbild
`target/p04-cycle-20-desktop.png` zeigt wieder Raum 1 ohne offenes Overlay.
Der Shell-Prozess blieb PID 89726; installierte Datei und Prozess verwenden
weiter den obigen Release-Build. Für die Zwischenzyklen liegen
Eingabeprotokolle, aber keine Einzelbilder vor, da wiederholte
Screenshot-Freigaben das Portal sporadisch blockierten.

Die P04-Öffnungsbelegung wurde auf demselben Release mit virtueller Tastatur
geprüft. Ein Druck auf `Super` allein ließ den Desktop unverändert (mittlere
RGB-Abweichung 0,00). `Super+Tab` aktivierte sichtbar Raum 2; `Super+Escape`
öffnete das Systemdeck und `Super+Space` nach dessen Schließen den Hub über
Raum 2. Nach `Esc` stimmte der Desktopbereich wieder mit dem Ausgangsbild
überein (0,00). Die Live-Belege sind `target/p04-cycle-1-room-switch.png`,
`target/p04-cycle-1-deck.png` und `target/p04-cycle-1-hub.png`.
Fünf Screenshot-Anfragen auf dem ruhenden Desktop liefen nacheinander durch.
Bei Aufnahmen unmittelbar nach Overlay-Wechseln kam es zunächst vereinzelt zum
Timeout der Freigabe. Die Diagnose zeigte: Die Shell empfing die Anfrage und
zeichnete den Dialog nach dem erneuten Mapping, aber `Enter` erreichte sie
nicht. Beim Öffnen setzte sie bis dahin nur die Größe erneut; die exklusive
Tastaturinteraktivität stammte noch aus der initialen Layer-Konfiguration.
`open_consent_modal` setzt sie jetzt bei jedem Öffnen zusammen mit der Größe
erneut. Der Aufwand entsteht ausschließlich pro Screenshot-Anfrage, nicht im
Idle-Pfad. Der endgültige Release ohne Diagnoseinstrumentierung bestand zehn
schnelle Folgen `Hub öffnen → Esc → Screenshot anfordern → Enter`: jede
Freigabe reagierte auf den ersten Tastendruck und antwortete nach etwa 1,5 s.
Fedora `cargo fmt --all -- --check`, `cargo check --workspace --locked`,
`cargo test --workspace --locked -q`, Design-Guard, Clippy mit `-D warnings`
und Shell-Release-Build sind grün. Installierte Datei und laufender Prozess
sind bytegleich mit SHA-256
`be5962ba7ed494fe7e364be49a505b616e08b8d18f1c34c20579cc146647314a`.

- Das grüne Theme bei 1366×768 nach Verfügbarkeit eines physischen Modus
  als Live-Screenshot vergleichen; lange, leere, volle, fokussierte und
  Fehlerzustände weiter prüfen.
- Die Login-Sitzungskennung ist auf Fedora durch `XDG_SESSION_ID` bestätigt.
  Plattformen ohne diese Variable verwenden gekapselt `WAYLAND_DISPLAY` als
  Fallback; deren Sitzungsgrenze braucht vor einer weiteren Plattformfreigabe
  einen eigenen Laufzeittest.
- Die bestehende ältere Settings-Seite besitzt noch einen Farbtheme-Wähler.
  Die Entscheidung vom 25.09.2026 macht nur das dunkelgrüne Theme für die Alpha
  verbindlich; der Wähler wird bei der Control-Center-Migration aus dem aktiven
  Ablauf entfernt, ohne Nutzer- oder fremde Desktopkonfiguration zu überschreiben.
