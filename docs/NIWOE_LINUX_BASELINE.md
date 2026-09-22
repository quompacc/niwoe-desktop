# NIWOE Linux-Baseline und Hardware-Smoke

Stand: 2026-09-22. Status: vollständige Linux-Gates und isolierter Nested-Smoke
am 21.09. bestanden. Echter Displaymanager-/DRM-Login, Portal-Theme-Abfrage und
Polkit-Dialog am 22.09. geprüft. Sitzungsende, KDE-Rundlauf und Polkit-Neustart
einschließlich Negativfällen inzwischen bestanden, siehe P00-Bericht. Windows bleibt die
Bearbeitungsumgebung; Linux-Belege stammen vom Acer.

## Zielhost erfassen

Der Nutzer hat Fedora KDE installiert und den Entwicklungsrechner per SSH
freigegeben. Verifizierter Host: `eduard@192.168.1.203`, Hostname `fedora-dev`,
Fedora 44 KDE Plasma Desktop, Kernel nach Update am 22.09.
`7.2.6-200.fc44.x86_64`, SELinux `Enforcing`.
Grafik: Intel HD Graphics 620 (`i915`) und NVIDIA 940MX (`nouveau`). Auf der
Btrfs-Systempartition sind rund 468 GiB frei. Arbeitskopie: `~/niwoe-desktop`.
Passwörter werden nicht im Repository gespeichert; sudo bleibt passwortgeschützt.
Vor einem Gate die aktuellen Versionen im Phasenbericht erfassen:

```bash
cat /etc/os-release
uname -a
rustc -Vv
cargo -V
rpm -qa | sort > niwoe-rpm-manifest.txt
inxi -Gxx || lspci -nnk | grep -A3 -E 'VGA|3D|Display'
loginctl session-status
getenforce
```

Fedora bleibt als normale Rettungs-/Arbeits-Sitzung installiert. NIWOE wird als
zusätzliche Sitzung eingerichtet; ein Login-/Boot-Ersatz ist kein P00-Schritt.

## Abhängigkeiten

Die Paketnamen in `scripts/install-deps.sh --manager dnf` wurden am 2026-09-21
gegen den offiziellen Fedora-44-Paketindex geprüft. Build, Runtime und
Hardwaretest bleiben bewusst getrennt:

```bash
scripts/install-deps.sh --manager dnf build
scripts/install-deps.sh --manager dnf runtime
scripts/install-deps.sh --manager dnf hardware-test
```

Verifizierte Kernpakete: [libseat-devel](https://packages.fedoraproject.org/pkgs/seatd/libseat-devel/),
[libinput-devel](https://packages.fedoraproject.org/pkgs/libinput/libinput-devel/),
[wayland-devel](https://packages.fedoraproject.org/pkgs/wayland/wayland-devel/),
[libdrm-devel](https://packages.fedoraproject.org/pkgs/libdrm/libdrm-devel/fedora-44.html),
[drm_info](https://packages.fedoraproject.org/pkgs/drm_info/drm_info/) und
[Xwayland](https://packages.fedoraproject.org/pkgs/xorg-x11-server-Xwayland/xorg-x11-server-Xwayland/fedora-44.html).
Vor Installation den tatsächlichen Fedora-Release mit `dnf info <paket>`
gegenprüfen; keine Drittquellen, AUR oder fremden Mesa-Builds verwenden.

## Sitzung, SELinux, Portal und Polkit

**Desktop-Isolation:** NIWOE darf gemeinsame `kdeglobals`, GTK-Einstellungen,
GTK-CSS oder persistente GSettings anderer Desktops nicht überschreiben.
Der alte Theme-Export verletzte das und setzte auf dem Acer ein nicht
installiertes Icon-Theme. Die Originaldateien wurden aus vorhandenen Backups
wiederhergestellt; der Nutzer bestätigt, dass KDE-Symbole wieder sichtbar sind.
Toolkit-Dateien werden künftig unter `$XDG_CONFIG_HOME/niwoe/toolkit`
(Standard: `~/.config/niwoe/toolkit`) erzeugt. Eine Anwendung dieser Dateien
auf Fremdanwendungen benötigt eine spätere, ausdrücklich sitzungsspezifische
Integration. Die öffentliche Hell-/Dunkel-Einstellung kommt weiterhin aus dem
Portal. Globale Dateiumschreibung ist kein zulässiger Ersatz dafür.

Seit P01 verwenden auch die ausführbaren Dateien NIWOE-Namen. Der Installer fügt
`/usr/share/wayland-sessions/niwoe.desktop` mit dem Anzeigenamen
`NIWOE (development)` sowie `/usr/local/bin/niwoe-session` hinzu:

```bash
bash scripts/install-local.sh --build
test -x /usr/local/bin/niwoe-session
test -f /usr/share/wayland-sessions/niwoe.desktop
```

Für den ersten Smoke wurde mit `--debug` aus den geprüften `target/debug`-
Binaries installiert. Seit der Performance-Diagnose am 22.09. ist der
vollständig gebaute Release-Stand installiert. Der Standard bleibt Release; `--build`
baut das mit `--debug` ausgewählte Profil. Der Sessionwrapper stoppt beim
Beenden `niwoe-session.target`, dessen Portal gehört per `PartOf` dazu.
Solange die systemd-Aktivierungsumgebung noch `XDG_CURRENT_DESKTOP=NIWOE`
enthält, stoppt er gleichzeitig `xdg-desktop-portal.service`, damit die nächste
Sitzung ihren Backendnamen neu bestimmt. Bei bereits fremder Desktopkennung
wird ausschließlich das eigene Target beendet; der gemeinsame grafische
Session-Target wird nicht direkt gestoppt.

Als normaler Benutzer mit sudo ausführen; KDE abmelden und im bestehenden
Displaymanager `NIWOE (development)` auswählen. **Kein `--enable-boot`** verwenden.
Der Wrapper entfernt geerbte Displayvariablen für die DRM-Backendwahl, setzt
die Desktopkennung und erhält Runtime-Verzeichnis und Sessionbus aus dem
Displaymanager. Der Compositor startet die Shell aus demselben Binärverzeichnis;
die Shell aktiviert den vorhandenen Session-/Portalpfad. Rückkehr: NIWOE abmelden
bzw. beenden und im Displaymanager wieder KDE auswählen. Start, Panel und
Launcher sind seit dem Hardwaretest am 22.09. bestätigt; das Sitzungsende
ist noch nicht abgenommen.

KDE und NIWOE bis zur Lifecycle-Prüfung nicht gleichzeitig unter demselben
Benutzer betreiben: systemd-Usermanager und dessen Aktivierungsumgebung sind
benutzerweit. Für erste Versuche ein separates Testprofil verwenden. Der
Autostartparser berücksichtigt jetzt `OnlyShowIn`/`NotShowIn` und unterscheidet
`Hidden` von reiner Menüunsichtbarkeit (`NoDisplay`). Die tatsächliche
Agenten-/Portalaktivierung und das Sessionende bleiben auf Hardware zu prüfen.

SELinux bleibt `Enforcing`. Bei einem AVC werden Kontext und Ursache mit
`ausearch -m AVC -ts recent` festgehalten und die Installation/Labels oder eine
eng begrenzte, überprüfbare Policy korrigiert. SELinux wird nicht global
deaktiviert. Innerhalb der gewählten Sitzung prüfen:

```bash
systemctl --user status xdg-desktop-portal
systemctl --user status niwoe-portal
busctl --user list | grep -E 'portal|polkit'
```

Die Befehle verwenden die technischen Namen ab P01; P00-Nachweise bewahren
ihre damaligen Namen im Phasenbericht. Portal und Polkit dürfen nur einmal pro Sitzung
aktiv sein; keine konkurrierende KDE- und NIWOE-Agenteninstanz.

## Gates und Smoke

Nach dem Checkout auf dem Linux-Host laufen die P00-Gates auf demselben Stand:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p niwoe-tokens --test design_guard
cargo test -p niwoe-tokens --test source_size_guard
cargo test -p niwoe-shell --test centralization_guard
git diff --check
```

### Nested-Start und richtiger Testclient

Für den reproduzierbaren, isolierten P00-Test bevorzugen:

```bash
cargo build --workspace --locked
bash scripts/smoke-nested.sh /run/user/$(id -u)/wayland-0
```

Den vorhandenen KDE-Socket vorher prüfen; der Parameter bezeichnet den
**Eltern-Compositor**, nicht den Socket des gestarteten NIWOE-Compositors.
Das Skript verwendet temporäre HOME-/XDG-Verzeichnisse, einen eigenen Sessionbus
und einen GSettings-Speicherbackend. Es prüft Shell-Authentifizierung,
Wayland-Globals und Configure/Buffer-Attach des Testclients, beendet die
Testprozessgruppe und vergleicht KDE-Konfigurationshashes und Sitzungsvariablen.
Profile bleiben zur Diagnose unter `/tmp/niwoe-p00.*`; Logs liegen unter
`target/p00-evidence/nested.*`. Der Lauf `nested.Bqbwez` bestand nach dem
Winit-Fix. Kein Nachweis für echte Portal-/Polkit-/Accessibility-Integration.

Die folgenden manuellen Schritte sind nur für ein separates Testprofil gedacht:

In der grafischen Sitzung des Testbenutzers (Build im selben Checkout):

```bash
cargo build --workspace
mkdir -p target/p00-evidence
RUST_LOG=info target/debug/niwoe > target/p00-evidence/nested.log 2>&1 &
niwoe_pid=$!
```

Die vorhandene Backendwahl erkennt das geerbte `WAYLAND_DISPLAY`/`DISPLAY` und
startet Winit. Im Log müssen `Detected parent display`, die Zeile
`NIWOE running on socket:` sowie der erfolgreiche Shellstart erscheinen.
Aus dieser Socketzeile den tatsächlichen Namen übernehmen, nicht `wayland-0`
raten. In einem zweiten Terminal des gleichen Testbenutzers:

```bash
read -r -p 'Socket aus nested.log: ' niwoe_socket
test -S "$XDG_RUNTIME_DIR/$niwoe_socket"
WAYLAND_DISPLAY="$niwoe_socket" wayland-info
WAYLAND_DISPLAY="$niwoe_socket" GDK_BACKEND=wayland zenity --info --text='NIWOE P00 client'
```

Die Diagnoseclients `wayland-info` und `zenity` vorher über die offiziellen
Repos bereitstellen (auf Fedora Paket `wayland-utils` und `zenity`, Verfügbarkeit
bei Einrichtung prüfen). Der Dialog muss **innerhalb des NIWOE-Fensters** liegen.
Shell-IPC separat am bestätigten Verbindungs-/Snapshotpfad prüfen; ein gestarteter
Prozess allein beweist keine authentifizierte Verbindung. Zum Beenden zuerst
den Dialog und das Winit-Fenster schließen; Exitstatus und Logs erfassen.

Auf echter Hardware folgt die zusätzliche DRM-Sitzung mit Wayland-/XWayland-Client.
Fehler mit Log, Exitcode, Display, GPU, Scale und Sessionart dokumentieren.

## Performance-Ausgangswerte

Für Bedienungs- und Performanceabnahmen Release verwenden. Der erste
Hardware-Smoke verwendete Debug-Binaries; der Nutzer meldete dabei einen
teilweise unbrauchbar langsamen Launcher bei gut bedienbarem Terminal.
Dieser Zustand ist keine bestandene Performanceabnahme.

Reproduzierbarer CPU-Vergleich des bestehenden Launcher-Zeichenpfads:

```bash
cargo test -p niwoe-shell launcher_paint_baseline -- --ignored --nocapture
cargo test --release -p niwoe-shell launcher_paint_baseline -- --ignored --nocapture
```

Beide Testbinaries vorher kompilieren; während der eigentlichen Messung keine
anderen Builds laufen lassen. Je Fall drei Warm-up-Durchläufe und 20 Messungen,
200 synthetische Apps, 880×620-Inhalt, leere Suche und Suchtext, wechselnde
Auswahl/Hover. Gemessen wird die CPU-Zeichenzeit ohne Icon-I/O, äußeren Schatten,
Wayland-Übertragung oder GPU-Präsentation. Median/P95 sind Diagnosewerte,
kein Ersatz für Hardware-Interaktion oder eine stabile CI-Zeitgrenze.

Nach Warm-up auf derselben Hardware drei Messungen für fünf Minuten Idle und
je zwanzig Öffnen/Schließen-Zyklen durchführen. Erfassen: CPU von Compositor
und Shell, verfügbare GPU-Werte, Repaint-Zähler, RSS/Cachegröße sowie
Eingabe-bis-sichtbar-Latenz; Werkzeug, Einheit, Last und Display-Hz angeben.
Ohne Host/HW sind alle Werte **NOT RUN**, nicht null.
