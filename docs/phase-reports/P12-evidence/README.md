# P12-Nachweise reproduzieren

Die Auswertung und Grenzen stehen in [P12.md](../P12.md). Logs mit `final`
beziehen sich auf die im Bericht genannten installierten Binärhashes.
Fehlgeschlagene Vorläufe sind keine bestandenen Fälle. Rohdaten mit persönlichen
Snapshots und Originalbackups bleiben privat unter `target/p12-evidence/`
auf Fedora; die hier abgelegte numerische Auswertung enthält deren SHA-256.

## Voraussetzungen

Aus dem Fedora-Checkout in einer echten NIWOE-Displaymanager-Sitzung arbeiten.
Zwei Outputs, freie Arbeitsfläche ohne persönliche Anwendungen, erreichbares
SSH/TTY und gesicherte persönliche NIWOE-/KDE-/GTK-Konfiguration voraussetzen.
Die Fixtures verwenden die im Bericht genannten Outputnamen und vorhandenen
Modi; ihre Assertions prüfen diese Voraussetzungen. Einige Tests benötigen
temporären Zugriff auf `/dev/uinput`: vorher `getfacl -p /dev/uinput` sichern,
nachher mit `sudo setfacl --restore=<gesicherte ACL>` exakt wiederherstellen.
Passwörter ausschließlich am nicht protokollierenden `getpass`-Prompt eingeben.

Viele Fixtures verweigern ein bereits vorhandenes Ergebnisverzeichnis. Einen
Vorläufer unter einem eindeutigen Namen erhalten, bevor derselbe Test wiederholt
wird. Testskripte verändern keine Raumdefinitionen; temporäre Layouts und
Output-Konfigurationen werden aus den vorher gesicherten Bytes wiederhergestellt.

## Funktionale Prüfungen

```sh
bash scripts/check-p12.sh
python3 scripts/test-p12-windows.py
python3 scripts/test-p12-desktop.py
python3 scripts/test-p12-screenshot-output.py after
python3 scripts/test-p12-portal.py portal_final
python3 scripts/test-p12-lock.py
python3 scripts/test-p12-polkit.py initial
sudo systemctl restart polkit.service
python3 scripts/test-p12-polkit.py restarted
python3 scripts/test-p12-audio-hdmi.py
```

Den Netzwerk-Test erst nach eigenständigem Einrichten von WLAN und geprüftem
SSH-Zugang über beide Interfaces ausführen: `python3 scripts/test-p12-network.py`.
Er prüft Ethernet→WLAN→Ethernet und hält einen unabhängigen zeitgesteuerten
Ethernet-Wiederzugang bereit. `python3 scripts/test-p12-everyday.py` benötigt
funktionierenden RTC-Wakeup und führt mit privaten KWrite-Dateien den vollständigen
Arbeitsablauf einschließlich echter Suspend-/Resume-Zeitmessung aus.

Nach frischem Login prüft `python3 scripts/test-p12-session.py` die neutrale Loge,
den einmaligen Hub, Shell-Watchdog und direkten Appstart. Die erzeugten Aufnahmen
visuell kontrollieren. Regulärer NIWOE-/KDE-Logout und Rückkehr sind eigene
Sitzungsschritte; ein Displaymanager-Neustart ersetzt diese Prüfung nicht.

Der physische HDMI-Test verwendet `python3 scripts/test-p12-hardware.py STUFE`:
`prepare`, Kabel abziehen, `unplug`, beide Fenster visuell prüfen, Kabel einstecken,
`replug`, `lock`, beide Displays beobachten, `locked-restart`, `unlock`, `cleanup`.
Die Nutzerbeobachtung muss getrennt von den Software-Assertions dokumentiert sein.

## Vergleichbare Messungen

Vor Produktänderungen Ausgangssnapshot und Identität sichern, drei gültige
300-s-Idle-Proben und drei Serien mit je 20 echten UI-Zyklen aufnehmen.
Unterbrochene Proben ausdrücklich kennzeichnen und vollständig ersetzen.
Die Messwerkzeuge sind `scripts/measure-p12-idle.py` und
`scripts/measure-p12-cycles.py`; Skriptzeiten sind keine optische Latenz.

Für den abschließenden installierten Release:

```sh
python3 scripts/prepare-p12-performance.py apply
python3 scripts/measure-p12-cycles.py after
python3 scripts/measure-p12-idle.py target/p12-evidence/after-idle.json 3
python3 scripts/summarize-p12-performance.py
python3 scripts/prepare-p12-performance.py restore
```

Während Idle weder GUI-Eingaben, Hotplug noch Builds. Der Summarizer erhält
Rohdaten, exportiert numerische Werte ohne persönliche Raum-Snapshots und prüft
auch den höchsten einzelnen finalen CPU-Wert gegen Baseline-Median +0,5
Prozentpunkte. CPU bezieht sich auf einen Kern, RSS auf Bytes, GPU auf summierte
DRM-Engine-Zeit. Unterschiedliche Prozesshistorien begrenzen absolute RSS-Vergleiche.

Repaint-/Render-/Commit-Zähler erfordern eine getrennte Sitzung mit
`NIWOE_SHELL_REPAINT_STATS=1`, `NIWOE_SHELL_RENDER_STATS=1`,
`NIWOE_SHELL_COMMIT_STATS=1` und erfasstem Shell-stdout. Diese instrumentierte
Stichprobe nicht als normalen Vorher-/Nachher-CPU-Lauf ausgeben. Optische Latenz
und fehlende Cache-Zähler bleiben ausdrücklich unbekannt.
