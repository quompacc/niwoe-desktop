# P06 – zwei Outputs und Fehlerfälle, 26.09.2026

Status: **in-progress**. USB-Hub auf Nutzerwunsch beendet; die Verkabelung zum
anderen PC erklärt die fehlende USB-Anmeldung, kein nachgewiesener NIWOE-Fehler.

## Tatsächlicher Zweimonitor-Durchlauf

Compositor PID 79150, Release `8c1d874c…`: internes Display 1920×1080/60 Hz,
HDMI 3840×2160/30 Hz rechts daneben, beide mit Skalierung 100 %. Keine Fenster
zu Beginn, neun Räume, Revision 49. Native uinput-Zeigerbewegung über die
Gesamtfläche 5760×2160 bestimmt den fokussierten Output; Raumwahl erfolgt über
den bestehenden SwitchWorkspace-Pfad.

- Links Raum 2 aktivieren; rechts Raum 3 aktivieren: linke Auswahl bleibt 2.
- Eigene native GTK-Anwendung rechts starten: Hauptfenster, weiteres normales
  Fenster, Dialog und minimiertes Fenster liegen alle in Raum 3. Der Titel
  „Floating“ des Testfensters bedeutet hier keine Floating-Abnahme; in dieser
  Runde wurde kein Super+T ausgeführt.
- Links nach Raum 4 wechseln: rechts bleibt Raum 3. Fenster-IDs, Raumzuordnung
  und Minimierungszustand bleiben exakt gleich.
- Testanwendung beenden: drei verwaiste Einträge bleiben zunächst mit Titel
  „Window“ in Raum 3 zurück. Deshalb war die erste Bereinigungsprüfung rot.
- Gezielter Wechsel des rechten Outputs von Raum 3 nach 4 und zurück entfernt
  die Einträge im bestehenden Release. Endstand: keine Fenster, links Raum 2,
  rechts Raum 3; persistente Raumdaten einschließlich Revision unverändert.

Belege auf Fedora: `target/p06-live-ui/{continue-p06,dual-left,
dual-right-client,dual-left-change,dual-cleanup,dual-cleanup-final}.json`.
Die Sitzung begann in der Loge; die Tests aktivierten Räume. Kein Neulogin
zur Wiederherstellung des Logenzustands erzwungen.

## Gefundener Fehler und Korrektur

`state/handlers/xdg/lifecycle.rs` entfernte Runtime-Metadaten und meldete
WindowClosed, entfernte das native Window jedoch nicht aus Space/WM-Struktur.
Der Renderpfad aktualisiert nur den fokussierten Space. Ein Clientende im
anderen Raum konnte dadurch in späteren Snapshots erneut als leeres Fenster
erscheinen. Der Destroy-Handler entfernt das konkrete Fenster nun unmittelbar
aus dem zugehörigen Space und WM-Modell, aktualisiert das Layout und sendet
einen konsistenten Snapshot. Raumwahl und Renderreihenfolge bleiben unverändert.
Zusatzarbeit ausschließlich beim Destroy-Ereignis, begrenzt durch maximal
64 Räume; keine neuen Idle-Timer oder Effekte.

`scripts/test-room-transients.py` prüft zusätzlich Clientende im Hintergrundraum.
Die Gegenprobe auf dem alten Release scheitert genau am verwaisten Fenster:
`target/p06-close-baseline.log`, `target/p01-evidence/nested.CiaLU8`.

## Isolierte Startup-/IPC-Fehlerfälle

Neues `scripts/test-room-errors.py`, ausgeführt unter eigener D-Bus-Sitzung
und eigenen HOME-/XDG-Verzeichnissen. Keine Nutzerdaten verändert:

- Defekte TOML-Datei: Compositorstart scheitert mit Parse-Diagnose, Datei bytegleich.
- Schema 999: Start scheitert mit Schema-Diagnose, Datei bytegleich.
- Ein-Raum-Profil: Löschen über IPC abgewiesen, Datei unverändert.
- Zwei authentifizierte Verbindungen senden unterschiedliche Umbenennungen
  mit derselben Revision: genau ein Erfolg, ein Konflikt; keine verlorene Änderung.
- Neue Verbindung erhält den tatsächlich persistierten Gewinner und Revision 1.

Befehl: `dbus-run-session -- python3 scripts/test-room-errors.py /run/user/1000/wayland-1`.
Alle Fälle erfolgreich; `target/p06-errors.log`, Profilbelege unter
`/tmp/niwoe-p06-errors.iw3hhhos`. Nur der eigene isolierte Compositor wird beendet.
Die Startfehler sind technisch diagnostiziert; eine grafische Fehlerhilfe im
Loginmanager ist damit nicht abgenommen.

## Verifikation und Installation

Fedora, alle finalen Befehle Exitcode 0: `cargo check --workspace`,
`cargo test --workspace -q`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check`, `cargo build --release -p niwoe --locked`.
Logs: `target/p06-close-{check,test,clippy,build}.log`.

`NIWOE_ROOM_TRANSIENT_SMOKE=1 bash scripts/smoke-nested.sh /run/user/1000/wayland-1`
besteht mit dem Fix einschließlich Hintergrund-Clientende; Belege:
`target/p06-close-fixed.log`, `target/p01-evidence/nested.xdjKpm`.
Eltern-KDE-/GTK-Konfigurationen im Smoke unverändert. Isolierte AT-SPI-Warnung
begründet keine Barrierefreiheitsabnahme. Fehlerfallskript auf dem finalen Release
erneut erfolgreich: `target/p06-errors-fixed.log`, `/tmp/niwoe-p06-errors.kksud8qi`.

Compositor atomar installiert, `cmp` und SHA-256 bestätigen
`e1e2167078f550399c3b87cafea3197a4d0f25541981f6672721b08763e12484`.
Shell bleibt `5636d59c…`. PID 79150 läuft bis zum regulären Nutzer-Neulogin
weiter mit `8c1d874c…`; laufende Sitzung nicht beendet.

## Verbleibende Live-Gates

Korrigierte Fensterbereinigung nach Aktivierung des neuen Compositors auf DRM
nachprüfen. Monitorentfernung mit belegten Fenstern und die vollständige
Zweimonitor-/Skalierungs-Matrix bleiben offen. P07-Navigation über neun Räume
wird nicht vorgezogen. Kein Cargo.toml-/Dependency-, Theme- oder Login-Umbau.
