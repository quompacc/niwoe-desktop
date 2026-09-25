# Panelkorrektur nach verbindlichem Mockup

## Eingabebefund vom 25.09.2026

Die visuelle Panel-Freigabe vom 24.09. bleibt bestehen. Beim aktuellen
Fedora-Bedienlauf brauchte die Raum-2-Schaltfläche laut Nutzer mehrere
Klickversuche. Ein danach aufgenommener nativer Screenshot
(`target/p03-room2-click-live.png`) zeigt Raum 2 mit aktivem Goldrahmen;
damit ist die Raumwahl schließlich angekommen. Die Codeprüfung zeigte die
Ursache: Panel und Hub sendeten `SwitchWorkspace` beim Drücken der linken
Maustaste. Der Compositor verwirft einen Raumwechsel während eines aktiven
Zeiger-Grabs. Beide Raumwahlen lösen jetzt beim Loslassen aus, wie bereits
die Auswahl im Raum-Popup. Andere Panel- und Hub-Aktionen bleiben beim
Drücken. Die vorübergehende Klickspur wurde vor dem Release entfernt.
Fedora `cargo fmt --all -- --check`, `cargo check --workspace --locked`,
`cargo test --workspace --locked -q`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` und
`cargo build --release -p niwoe-shell --locked` sind grün. Release-SHA-256:
`4ffba769df2af79902fd019776326a6c63ae010177d06796b3f8a3ab69593b1d`.
`/usr/local/bin/niwoe-shell` und `/proc/47939/exe` sind bytegleich mit dem
Release. Die KDE-/GTK-Konfiguration blieb unverändert. Ein virtueller
Hardware-Mausklick auf Raum 1 und danach genau einer auf Raum 2 wurde durch
native Screenshots bestätigt: `target/p03-room1-after-single-click.png` und
`target/p03-room2-after-single-click.png`. Jeder Klick wechselte sofort den
markierten aktiven Raum. Ein separater Hub-Kartentest öffnete den Hub mit
`Super+Space`, wählte Raum 1 mit einem Klick und schloss den Hub; der native
Beleg liegt unter `target/p04-hub-room1-after-click.png`. Danach wurde Raum 2
wiederhergestellt. Für diese Klickprüfung entsteht keine Laufzeit-Arbeit im
Produkt; die virtuelle Eingabe und Screenshots sind externe Testwerkzeuge.

**Nutzerklarstellung 24.09.2026:** Das Panel war bereits in Ordnung. Keine weitere
Panelgestaltung und kein erneuter visueller Blocker. Der installierte Stand mit
stabiler Raumfolge bleibt erhalten; die nächste UI-Arbeit ist der vollständige
Hub aus `17_13_54 (2)`. Die frühere Formulierung einer noch offenen ausdrücklichen
Panelabnahme ist damit überholt.

## 24.09.2026: stabile Raumfolge statt aktivitätsabhängigem Ausschnitt

Nach erneuter Prüfung der verbindlichen letzten vier Mockups ist die Raumgruppe
im Panel jetzt stabil. Ein Raumwechsel verschiebt die sichtbaren Tabs nicht mehr
um den aktiven Raum. Die Leiste zeigt stets den Anfang der gespeicherten
Raumreihenfolge, soweit er in die linke Spur passt. Weitere Räume bleiben über
den Überlauf erreichbar. Dieser zeigt die Anzahl der ausgeblendeten Räume und
erhält Akzentfläche und Kontur, wenn der aktive Raum darin liegt. Wenn alle Räume
direkt passen, entfällt der bisher ständig sichtbare Überlauf vollständig.

Geändert wurden `panel_view/rooms.rs`, `panel_view/chips.rs`,
`panel_view/layout.rs` und `panel_view_tests.rs`. Material, mittige Uhr,
Statusgruppe, Panelhöhe, Renderreihenfolge und IPC bleiben unverändert. Die
zusätzlichen Regressionen prüfen eine identische sichtbare Folge bei Wechsel von
Raum 1 zu Raum 9, den vollständigen Vier-Raum-Fall ohne Überlauf sowie weiterhin
kollisionsfreie Ziele in beiden Themes und den vorhandenen Viewportbreiten.

Performance: keine neuen Abfragen, Timer, Assets oder Renderpasses. Die Auswahl
der sichtbaren Präfixlänge läuft nur beim bestehenden Panel-Neuaufbau. Der
Überlauf zeichnet höchstens dieselbe tokenbasierte Akzentfläche und Kontur wie
ein aktiver Raumtab.

Fedora-Prüfungen: `cargo fmt --all -- --check`,
`cargo check --workspace --locked`, `cargo test --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` und
`cargo build --release -p niwoe-shell --locked` bestanden. Lokaler Design-Guard
und `git diff --check` bestanden. Die native 1024-Pixel-Rasterung unter
`target/p03-room-rail/panel-1024.png` wurde angesehen; sie zeigt die stabile
Folge `Raum 1`, `Raum 2`, danach den Überlauf `+7`, die mittige Uhr und getrennte
Statusziele. Die Rasterung belegt keinen compositorseitigen Blur.

Releaseinstallation wurde versucht, aber `sudo -n` verlangt das Nutzerpasswort;
es wurde nichts installiert. Der geprüfte Ein-Schritt-Installer liegt auf Fedora
unter `/home/eduard/niwoe-desktop/target/p03-room-rail/install.sh`. Er installiert
nur die geänderte Shell, prüft Release-/Installationsidentität und unveränderte
KDE-/GTK-Konfiguration und beendet danach die laufende Shell, damit der
Compositor-Watchdog den neuen Stand startet. Visuelle DRM-Abnahme bleibt bis zur
Ausführung dieses Skripts offen.

**Installation anschließend durch den Nutzer ausgeführt und unabhängig geprüft.**
Release, `/usr/local/bin/niwoe-shell` und `/proc/9344/exe` sind bytegleich mit
SHA-256 `4e39f6a83997aef2c874b4c1601c1c42df90abb9e9530e3b1eb113cbde688dc9`.
Der Watchdog ersetzte die vorherige Shell PID 1655 durch PID 9344; der laufende
Compositor PID 1641 blieb erhalten. Die vor/nach der Installation erfassten
KDE-/GTK-Konfigurationsdateien sind unverändert.

Reale DRM-Aufnahme des installierten 1920×1080-Stands:
`target/p03-room-rail/panel-live.png` auf Fedora und lokal
`target/p03-room-rail-panel-live.png`. Angesehen: vier stabile direkte Tabs
`Raum 1` bis `Raum 4`, danach `+5`, Uhr exakt in der Outputmitte, Such- und
Statusgruppe rechts, freigegebenes Panelmaterial über strukturiertem Wallpaper.
Die technische Installation und Sichtprüfung sind damit belegt. Die ausdrückliche
Nutzerabnahme des sichtbaren Panels bleibt offen; P03 wird bis dahin nicht als
`accepted` markiert und P04 nicht begonnen.

## Aktuell: Schriftzentrierung und größere Uhrzeit

Nutzer hat den installierten Materialabgleich visuell als sehr gut bestätigt.
Genehmigte aktuelle Aufnahme: `target/material-match/room-alignment-before.png`.
Offener Befund betrifft nur Text: Raumlabels zu hoch und Symbol/Text getrennt
ausgerichtet; Uhrzeit zu klein.

- `panel_view/rooms.rs`: Symbol und Beschriftung als gemeinsame Gruppe zentriert;
  Grundlinie aus gemessener Schrifthöhe der festen Labels ohne Unterlängen.
  Rasterregression misst die vollständige antialiaste Textkontur aller neun
  Raumlabels, nicht nur vollständig deckende Glyphenpixel.
- `panel_view/chips.rs`: Uhrzeit verwendet zentralen Body-Token (14 statt 12),
  Datum bleibt Caption; Uhrgruppe weiter in Bildschirmmitte. Zeit-Grundlinie
  anhand gemessener Ziffernhöhe.
- Material, Panelkontur, Klickflächen und Layoutbreiten unverändert.

Isolierte Rasterprüfung und gemeinsame Linux-Gates bestanden:
fmt/check/test/clippy, Release (34s), `git diff --check`. 1.101 Tests bestanden,
0 fehlgeschlagen, 1 ignoriert. Logs/native Rasterausgabe unter
`target/room-alignment/`; Rasterung angesehen. Installation über vorbereitetes
`target/room-alignment/install.sh` noch offen (sudo im Nutzerterminal).
Danach genügt eine Shell-Erneuerung; Liveabnahme der Schriftkorrektur offen.

## Aktuell: freigegebenes Deckmaterial für Panel und Launcher

Nutzer bestätigt anschließend Systemdeck/Lautstärke als perfekte Materialreferenz
und beauftragt die Übernahme auf Panel **und Launcher**. Diese Freigabe ersetzt
die versuchsweise individuelle 80-Prozent-Tönung des Panels unten.

- `niwoe-config/src/theme/types/config.rs`: gemeinsame Materialbehandlung für
  Panel/Launcher/Popup; keine Panel-Sondertönung oder ungetönter Panel-Backdrop.
  `shell_surface_fill_alpha` verhindert zusätzliche Shell-Füllung bei aktivem
  Compositorglas; ohne Glas/Blur bleibt eine definierte getönte Füllung erhalten.
  Regression prüft Gleichheit bei Default-/angepasster Opazität und beide Fallbacks.
- `panel_view/render.rs`, `app_view.rs`: verwenden die gemeinsame Shell-Füllregel
  und dieselbe Tintfarbe wie die Popups. Kontur, Geometrie, Uhr und Inhalt bleiben.
- `niwoe-tokens/src/chrome.rs`: experimentellen Panel-Alpha entfernt.
  `tests/contrast.rs` misst das freigegebene zweistufig komponierte Popupmaterial;
  `panel_view_tests.rs` schützt gegen zusätzliche Shellfüllung.
- Systemdeck und Lautstärke: weder Renderer noch Materialparameter geändert.

Kein weiterer Shader/Blurpass, keine zusätzliche Cache-/Timerarbeit.
Aktiver Compositor und Shell verwenden vor diesem Schritt noch ältere Prozesse
als die bereits installierten Binaries. Der gemeinsame Panelmaterialpfad erfordert
nach Installation dieses Folgestands eine Neuanmeldung. Linux-Gates unter
`target/material-match/`: fmt/check/test/clippy bestanden, 1.100 Tests bestanden,
0 fehlgeschlagen, 1 ignoriert; Release bestanden (1m49s). Lokale Kontrast- und
Design-/Dateigrößenguards sowie `git diff --check` ebenfalls grün.
`target/material-match/install.sh` auf Fedora bereit; sudo-Eingabe weiterhin
im Nutzerterminal erforderlich. Installation und visuelle Vergleichsabnahme offen.

## Folgeschritt: sichtbares Material und strukturierter Hintergrund

Aktive Shell und Compositor vor Änderung gegen installierten Release geprüft:
beide aktuell. Nutzer findet die Anordnung besser, Material aber zu schwach.
Panelveil deshalb als zentrale Panelrolle von 235 auf 204 Alpha (80 Prozent)
reduziert; die übrigen Oberflächen behalten ihre Tönung. Primärtext statt
Sekundärtext für Panelbeschriftung, themenfarbene Hover-/Pressed-Flächen und
dezenterer aktiver Tabgrund erhalten Lesbarkeit. Untere Ein-Pixel-Kontur nutzt
Sekundärtextfarbe mit bestehender Framealpha. Keine Geometrieänderung.

Neuer Kontrasttest komponiert Panel, Hover/Pressed und aktiven Tab über Schwarz
und Weiß für beide Themes: Text mindestens 4,5:1 und aktive Kontur mindestens
3:1. Lokale Kontrast-/Design-/Dateigrößenguards bestanden. Linux-Gates unter
`target/panel-glass/`: fmt/check/test/clippy und Release bestanden (1m50s),
1.100 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert. Ein Test-Scopefehler wurde
korrigiert und die Gates erneut ausgeführt. Installation vorbereitet, aber
weiterhin durch nicht startendes Werkzeug-PTY blockiert; sudo benötigt das
Nutzerterminal. Nach Installation nur Shell-Neustart und Livevergleich offen.

Nutzer fordert zusätzlich strukturiertes Wallpaper und sichtbarere Konturen.
`assets/wallpapers/niwoe-alpine-dawn.png` mit eingebautem image_gen erzeugt,
auf Fedora übertragen und über AppearanceWallpaperSet aktiviert. Vorherige
Wallpaper-Einstellung auf Fedora unter `target/panel-glass/wallpaper-before.json`
gesichert. Genehmigter Screenshot bestätigt neuen Hintergrund; die Aufnahme
`wallpaper-before-material.png` zeigt noch das vorherige 92-Prozent-Panelmaterial.
Prompt/Herkunft in `assets/wallpapers/README.md`.

Materialänderung nutzt vorhandene Blurtexturen und Shader; keine weiteren
Passes/Timer/Cacheeinträge. Wallpaper wird beim Wechsel decodiert, nicht pro
Frame. Laufender Compositor kann seinen bisherigen ungetönten Panelblur
behalten; nach Installation genügt für diesen Schritt ein Shell-Neustart.

23.09.2026. Nutzerpriorität: ausschließlich das Panel korrigieren; keine
Fortsetzung anderer UI-Phasen vor dessen visueller Abnahme.

Direkt angesehene Referenzen: `assets/ChatGPT Image 21. Sept. 2026,
17_13_54 (1).png` (Desktop) und `(2).png` (Hub mit derselben oberen Leiste).
Die bisherigen Nicht-Branding-Vorgaben bleiben erhalten.

Explizite anschließende Nutzerkorrektur: Uhr exakt in die Bildschirmmitte.
Das Layout verwendet gleich breite linke/rechte Spuren mit einer festen Uhrspur
dazwischen; Raumüberlauf und begrenzte Traybelegung schützen den mittigen Bereich.
Der Geometrietest prüft die Mitte und kollisionsfreie Klickflächen. Die frühere
rechte Uhrposition ist verworfen. Erneute Linux-Gates fmt/check/test/clippy
und Release bestanden (Shell-Release 35s). Native Rasterung angesehen:
Uhrmitte bei x=512 auf 1024 Pixeln, Raumüberlauf davor, Statusgruppe rechts.
Das vorbereitete Installationsskript verwendet den aktualisierten Release.

## Befund und Änderungen

- `niwoe-config/src/theme/types/config.rs`: neuer zentraler
  `compositor_surface_treatment` trennt beim Panel den ungetönten, deckenden
  Blur-Hintergrund von der vorhandenen transparenten Shell-Tönung. Bisher
  legten beide Pfade fast deckende Tönungen übereinander; der Hintergrundanteil
  wurde dadurch auf ungefähr ein Prozent reduziert. Popup-Material unverändert.
  Regression prüft Panel-Blur, einfache Tönung und Nicht-Glas-Fallback.
- `niwoe-compositor/src/backend/drm/render/scene_helpers.rs`: verwendet diese
  zentrale Materialaufteilung. Bestehende Blurtexturen/Cache/Invalidierung bleiben;
  keine weiteren Blurpasses, Shader, Timer oder dauerhaft gespeicherten Texturen.
- `niwoe-tokens/src/chrome.rs`: vier sichtbare Raum-Tabs, 120 Pixel breit,
  kompakter 32-Pixel-Überlauf und 164-Pixel-Uhr. Aktiver Raum bleibt auch am
  Ende der neun vorhandenen Arbeitsflächen erreichbar/sichtbar.
- `niwoe-shell/src/panel_view.rs`, `panel_view/{layout,chips,rooms,render}.rs`:
  16-Pixel-Seitenabstand, Caption-Schrift, kleine Raumsymbole, dezenter Goldgrund
  plus Kontur am aktiven Tab. Überlauf folgt den Tabs. Suchlupe rechts öffnet
  den vorhandenen Launcher; Screenshot bleibt über die vorhandenen anderen
  Zugänge erreichbar. Harte vertikale Trenner neben Status/Uhr entfallen.
  Untere Panelkontur verwendet die zentrale Material-Alpha.
- `panel_view/status_symbols.rs`: Suchlupe im bestehenden begrenzten,
  farb-/symbolabhängigen Rastercache. Keine neue Dependency.
- `panel_view_tests.rs`: überprüft transparente Shell-Fläche, kurze Raumleiste,
  Suchaktion und weiterhin gültige Klickflächen ohne angeheftete Apps.

Die Namen aus dem Mockup werden nicht als falsche Konfigurationsdaten eingebaut.
Bis benannte Räume existieren, bleiben echte Workspace-Nummern sichtbar.
Dark und Light verwenden identische Geometrie und Materialparameter.

## Verifikation und offene Abnahme

Linux-Gates unter `target/panel-mockup/` bestanden:
`cargo fmt --all -- --check`, `cargo check --workspace --locked`,
`cargo test --workspace --locked` (1.099 bestanden, 0 fehlgeschlagen, 1 ignoriert),
`cargo clippy --workspace --all-targets --locked -- -D warnings` und
`cargo build --release --workspace --locked` (1m49s).
Eine zunächst gemeldete ungenutzte Zeichenfunktion wurde entfernt; danach
alle Gates erneut erfolgreich. `git diff --check` ebenfalls bestanden.
Native 1024-Pixel-Panelrasterung angesehen: vier Tabs, Überlauf, Suchlupe,
keine überlappenden Ziele. Kein Ersatz für die noch offene Materialabnahme.
Installationsskript mit Binärvergleich und KDE-/GTK-Erhalt auf Fedora liegt
unter `target/panel-mockup/install.sh`. Installation dieses Stands noch offen;
sudo-Passworteingabe benötigt das Nutzerterminal (Werkzeug-PTY startet nicht).
Der vorherige Capture-Reihenfolgefix ist enthalten, damit der Bildvergleich
auch die compositorseitigen Materialien erfasst.

Eine isolierte Panelrasterung belegt Layout und Eingabegeometrie, keinen Blur.
Erforderlich bleibt der Vergleich am laufenden DRM-Desktop mit Struktur hinter
der Leiste (ein glatter Verlauf kann die Blurwirkung nicht belegen), Dark/Light,
Hover/aktiver Raum und schmalerer Ausgabe. Keine visuelle Abnahme behauptet.
