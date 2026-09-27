# NIWOE auf Fedora: Installation, Update und Entfernung

Die Alpha bleibt eine zusätzliche Sitzung neben dem vorhandenen KDE-Desktop.
Der P12-Abnahmestand steht im [P12-Bericht](phase-reports/P12.md); diese Anleitung
ist keine Behauptung, dass alle Hardware- und Integrationstests bestanden sind.

## Voraussetzungen und Sicherung

Im passenden NIWOE-Checkout unter Fedora arbeiten. Paketgruppen werden über
`bash scripts/install-deps.sh --manager dnf build` und entsprechend `runtime`
installiert. `hardware-test` enthält gesonderte Diagnosewerkzeuge. Das Skript
und die tatsächlich verfügbaren Pakete vor Ausführung prüfen. Rust/Cargo und
die Versionen von Kernel/Mesa/Wayland im Prüfbericht erfassen.

Vor Änderungen persönliche `~/.config/niwoe` und `~/.local/state/niwoe` sichern.
KDE-/GTK-Dateien nicht löschen oder überschreiben. Einen funktionierenden SSH-
oder lokalen TTY-Zugang und die vorhandene KDE-Sitzung erhalten. Kein
`--enable-boot` für die Desktop-Alpha: Der vorhandene Displaymanager bleibt aktiv.

## Release installieren oder aktualisieren

Nach den Gates aus [Implementierungsplan §7](../NIWOE_IMPLEMENTATION_PLAN.md#7-gemeinsame-definition-of-done):

```sh
cargo build --release --workspace --locked
bash scripts/install-local.sh
sha256sum target/release/niwoe /usr/local/bin/niwoe
sha256sum target/release/niwoe-shell /usr/local/bin/niwoe-shell
```

Der Installer fragt bei Bedarf sudo an. Er installiert sechs Binärdateien,
Sessionwrapper, Dateiauswahlhelper, Themes, PAM-Konfiguration, Portalmetadaten,
Userdienste und den Sessioneintrag unter
`/usr/share/wayland-sessions/niwoe.desktop`. Die gemeinsame Displaymanager-
Konfiguration und persönliche KDE-/GTK-Einstellungen gehören nicht zum Payload.
`/var/lib/niwoe` bleibt der NIWOE-eigene Datenbereich. Historische Meridian-
Aktivierungsdateien werden vom vorhandenen Migrationspfad gesichert, nicht
pauschal gelöscht; laufende alte Sitzungen verhindern die Migration.

Im Displaymanager **NIWOE (development)** auswählen. Nach einem Compositor-
Update ist ein echter Logout/Neulogin erforderlich. Bei einer reinen Shell-
Aktualisierung kann der vorhandene Watchdog die Shell ersetzen; dabei PID-
Wechsel und `/proc/<pid>/exe` gegen den installierten Hash prüfen. Eine bloß
ersetzte Datei beweist noch keine Aktivierung. Für Updates anderer laufender
Komponenten deren Prozessidentität ebenfalls prüfen oder neu anmelden.

Zur Rückkehr NIWOE regulär abmelden und im Displaymanager KDE auswählen.
Der Sessionwrapper beendet NIWOEs User-Target und das Portalfrontend nur,
solange die Aktivierungsumgebung noch der NIWOE-Sitzung gehört.

## Isolierter Lebenszyklustest

```sh
bash scripts/test-install-migration.sh
bash scripts/test-install-lifecycle.sh
```

Diese Tests verwenden das wirklich gebaute Release, erzeugen ein Staging unter
`target/` und aktivieren keine Dienste. Der Lebenszyklustest prüft Installation,
Update, verweigerte Entfernung bei veränderten Dateien und Symlink-Eltern,
Entfernung, Wiederholung und Neuinstallation. Persönliche Beispielkonfiguration
und eine fremde Datei müssen erhalten bleiben. Das ist ein Dateisystemtest;
Login, PAM, SELinux und Dienstaktivierung sind separate reale Sitzungstests.

## Entfernung mit passendem Release

Zuerst regulär aus NIWOE abmelden und KDE anmelden. Den Checkout und die
gebauten Binärdateien des **installierten Releases** behalten:

```sh
sudo bash scripts/uninstall-local.sh
systemctl --user daemon-reload
```

Die Entfernung erzeugt mit dem Installer eine private Referenzinstallation.
Vor dem ersten Löschen vergleicht sie jede vorhandene Payload-Datei byteweise.
Andere Inhalte, Dateitypen oder symlinkumgeleitete Pfade brechen den gesamten
Vorgang ab. Fehlende Dateien sind zulässig. Nur die übereinstimmenden einzelnen
Payload-Dateien werden entfernt; keine rekursive Entfernung installierter
Verzeichnisse. Fremde Dateien, persönliche Profile, `/var/lib/niwoe`-Daten und
Legacy-Sicherungen bleiben erhalten. Bei einer Abweichung zuerst Ursache und
passendes Release klären; kein pauschales `rm -rf` als Ausweichweg.

Laufende NIWOE-Prozesse oder ein aktiviertes eigenes Bootlogin verhindern die
Liveentfernung. Eine zuvor separat eingerichtete Bootsplash-/Bootintegration
gehört nicht zu diesem Verfahren und muss mit ihrem belegten Rückweg behandelt
werden. Ein benutzerdefiniertes Installationspräfix benötigt dasselbe `--prefix`
bei Installation und Entfernung. `--destdir` erlaubt die isolierte Prüfung.

Anschließend mit demselben geprüften Release `bash scripts/install-local.sh`
ausführen, Hashes prüfen und NIWOE erneut im Displaymanager wählen. Aufbewahrte
Benutzerdaten ermöglichen die Wiederaufnahme; das ersetzt keine Datensicherung.
