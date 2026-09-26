# P06 – verlorener Erstklick (26.09.2026)

Basis: `ae387d4`. Nutzer meldet wirkungslosen ersten Klick.

## Reproduktion auf Fedora

Nach dem Neulogin laufen Compositor PID 20918 und Shell PID 20935 seit
08:39:19 Uhr mit den im P06-Formularbericht dokumentierten Releasehashes.
Die laufenden Binärdateien wurden über `/proc/<pid>/exe` geprüft.

Virtuelle Hardwareeingabe über den bestehenden `/dev/uinput`-Testpfad:

1. Mit Esc Konfiguration, Verwaltung und Hub verlassen.
2. Zeiger auf die spätere Position „Räume verwalten“ setzen (1420,170 bei
   1920×1080).
3. Hub per Super+Space öffnen, ohne den Zeiger zu bewegen.
4. Einmal an derselben Stelle klicken: Hub bleibt geöffnet.
5. Zeiger um einen Pixel nach rechts bewegen und einmal klicken:
   Raumverwaltung öffnet sofort.

Belege: `target/p06-click-stationary-first.png` und
`target/p06-click-moved-first.png`. Neuer Raum, Abbrechen und Konfigurieren
reagierten bei vorheriger Zeigerbewegung bereits beim ersten Klick
(`target/p06-click-new-first.png`, `target/p06-click-config-first.png`).
Es wurden keine Räume oder Nutzerdaten verändert.

## Ursache und Korrektur

`crates/niwoe-compositor/src/input/pointer/button.rs`: Der Klickpfad ermittelte
das aktuelle `surface_under`, setzte aber vor `pointer.button` nicht den
Pointer-Fokus neu. Nach Mapping/Unmapping unter einem ruhenden Zeiger blieb
das frühere Ziel im PointerHandle. Die Tastaturfokussierung allein aktualisiert
dieses Zeigerziel nicht. Die erste Klickfolge ging deshalb an die alte Fläche.

Vor dem ersten Press ohne bestehenden Grab aktualisiert jetzt `pointer.motion`
das Ziel mit der unveränderten Position und der vorhandenen aktuellen
Trefferermittlung. Anschließend folgen unverändert Fokus-/Dekorationslogik,
Button und Frame. Bestehende Drag-/Popup-/implizite Grabs betreten den Zweig
nicht; Release-Routing bleibt unverändert. Der bestehende vorgeschaltete
Lock-Filter in `input/mod.rs` bleibt erhalten.

Keine Renderänderung, keine neue Abfrage im Idle, kein Timer. Die vorhandene
Trefferermittlung wird wiederverwendet; zusätzlicher Aufwand entsteht nur bei
einem ungebundenen Press. Datei bleibt mit 595 physischen Zeilen im Limit.

## Prüfung und verbleibendes Gate

Fedora: `cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace -q`, `cargo clippy --workspace --all-targets -- -D warnings`
und `cargo build --release -p niwoe --locked` bestanden.
Logs unter `target/p06-first-click-*.log`.
Die erste Compilerprüfung erkannte einen Borrow-Konflikt mit der nachfolgenden
Outputauswahl; die Fokusaktualisierung wurde vor diese unveränderte Auswahl
verlegt. Der anschließende Check-/Test-/Lintlauf ist grün.

Der Compositor wurde atomar installiert und per `cmp` gegen den Release sowie
SHA-256 geprüft:
`eb48aa3d4675d3eda0985fd95da17e72b8cbd6e533b47f420bdbb8377b6bf283`.
Nur `/usr/local/bin/niwoe` wurde ersetzt; Shell, Login, PAM und Konfigurationen
blieben unverändert. Prozess 20918 verwendet bis zum nächsten Login weiterhin
den vorherigen Release `77ebffb8…`.

Der oben dokumentierte Fehler wurde auf dem alten Release real reproduziert.
Die korrigierte Erstklickfolge benötigt noch einen Live-Rundlauf mit dem neuen
Compositor. Ein alleiniger Shell-Neustart aktiviert den Fix nicht; die laufende
Sitzung wird deshalb nicht automatisch beendet. P06 bleibt bis zur weiteren
Abnahme in Arbeit.
