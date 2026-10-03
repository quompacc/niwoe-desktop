# P06 – Abschlussabgleich, 26.09.2026

Ergebnis nach Umsetzung und finalem Nutzer-Neulogin: **accepted am 26.09.2026
für den dokumentierten Fedora-Aufbau**. Die zunächst gefundenen Modelllücken
sind geschlossen; Migration und korrigierter manueller Move sind live bestätigt.
Der [Endstandsbericht](P06_SCHEMA_COMPLETION.md) enthält Dateien, Tests,
installierte/laufende Hashes und Grenzen der Abnahme.
Maßgeblich sind der aktive Plan §4.2 und P06-01 bis P06-06. Dieser Bericht
fasst die chronologischen Zwischenberichte zusammen und ersetzt deren jeweils
damalige offene Punkte als aktuelle Übersicht.

## Arbeitspakete gegen den Code

| Paket | Ergebnis | Beleg / verbleibende Arbeit |
| --- | --- | --- |
| P06-01 Typen, Validierung, Schema nach §4 | Belegt | Vollständige Definition einschließlich Icon, Native-/XWayland-Referenzen, Layout und Restore; Defaults und zentrale Validierung geprüft. |
| P06-02 Migration und atomare Speicherung | Belegt | Schema 1 → 2 mit privater Sicherung isoliert und beim echten Neulogin geprüft; IDs, Revision, Zähler und Reihenfolge erhalten. |
| P06-03 Räume und konsistente Runtime-Slots | Vorhandene Abläufe belegt | Create/Edit/Reorder/Delete live, Fensterzustände und Slotverdichtung; zusätzliche IPC-Kapazitätsprüfung 1–64 bestanden. |
| P06-04 Commands, Snapshots, Acks, Revision | Belegt | Vollständiger Wire-Vertrag, SetPreferences, atomare Fehlerbehandlung, Konflikt/Reconnect/Persistenz geprüft. |
| P06-05 Raum-/Fensterzuordnung | Belegt | Bisherige Fälle plus manuelle native Floating-/Tiling-Moves mit Ziel-WM-/Eingabeprüfung und XWayland-Move auf dem neuen DRM-Build. |
| P06-06 Shell als Snapshot-Consumer | Belegt | Vollständige Snapshots einschließlich zentral validierter Präferenzen; Defaults bei alten Snapshots und Reconnect geprüft. |

Die inzwischen ergänzten Modellfelder sind keine neu erfundenen Anforderungen:
§4.2 nennt sie ausdrücklich, P06-01 verweist auf §4. Bereits der historische
[P06-Bericht](P06.md) nennt sie als noch fehlende Voraussetzung. P07 implementiert
die App-Zuordnungspolitik, P09 das tatsächliche Layout/Restore-Verhalten; diese
späteren Ausführungsfunktionen werden durch den notwendigen Datenvertrag nicht
vorweggenommen. Der aktive Plan wird für den Abschluss nicht nachträglich gekürzt.

## Explizite P06-Abnahmefälle

| Geforderter Fall | Tatsächlicher Nachweis |
| --- | --- |
| Umbenennen bei geöffneten Fenstern | DRM, IDs und Inhalte erhalten: [Lebenszyklus](P06_LIVE_LIFECYCLE.md) |
| Reihenfolge ändern | Native Verwaltung, stabile IDs/Slots und erhaltene relative Reihenfolge: gleicher Bericht |
| Raum mit Fenstern in Zielraum löschen | DRM mit normalem, schwebendem und minimiertem Fenster; Dialogmigration zusätzlich isoliert: gleicher Bericht |
| Letzten Raum löschen abweisen | Echter Compositor/IPC im isolierten Ein-Raum-Profil: [Fehlerfälle](P06_DUAL_OUTPUT_ERRORS.md) |
| Zwei Outputs | Unabhängige Raumwahl, Fensterzuordnung und gleiche Raumwahl auf beiden Outputs: [Zweimonitor](P06_DUAL_OUTPUT_ERRORS.md), [Nachprüfung](P06_OUTPUT_REMOVAL_WINDOWS.md) |
| Output entfernen | Physisches HDMI-Ab-/Anstecken, Fenster-/Inhaltserhalt und Wiederherstellen am verbleibenden Output: [Monitorentfernung](P06_OUTPUT_REMOVAL_WINDOWS.md) |
| Dialog erbt Raum | Inaktives und minimiertes Elternfenster, echter DRM-Nachtest: [Lebenszyklus](P06_LIVE_LIFECYCLE.md) |
| XWayland-Fenster im inaktiven Raum schließen | GTK-X11-Client auf DRM, IDs/Inhalte des anderen Fensters erhalten: gleicher Bericht |
| Defekte Datei | Startfehler mit Diagnose, Datei bytegleich: [Fehlerfälle](P06_DUAL_OUTPUT_ERRORS.md) |
| Neues unbekanntes Schema | Schema 999 abgewiesen, kein Überschreiben: gleicher Bericht |
| Reconnect | Vollständiger Snapshot auch bei verzögerter Authentifizierung; Gewinnerrevision nach Konflikt: [Grundlage](P06_ROOM_MODEL_FOUNDATION.md), [Fehlerfälle](P06_DUAL_OUTPUT_ERRORS.md) |
| Konkurrierende Mutation | Zwei echte IPC-Verbindungen mit gleicher Revision: ein Commit, ein Konflikt; Persistenz stimmt: [Fehlerfälle](P06_DUAL_OUTPUT_ERRORS.md) |

Die Fälle sind auf dem dokumentierten eDP-/HDMI-Aufbau beziehungsweise im
jeweils ausdrücklich bezeichneten isolierten Profil belegt. Daraus folgt keine
allgemeine Abnahme anderer GPUs, beliebiger Skalierungen oder aller Restore-Fälle.

## Historischer Kapazitätsnachweis vor der Schemaergänzung

`scripts/test-room-errors.py` erweitert: aus dem Ein-Raum-Profil 63 weitere
Räume über authentifizierte IPC anlegen, 65. Raum zurückweisen, gespeicherte
Datei bytegleich prüfen und per neuer Verbindung 64 eindeutige IDs und Slots
sowie korrekte Revision nachweisen. Ausschließlich isoliertes Profil.

Fedora erfolgreich:

- `dbus-run-session -- python3 scripts/test-room-errors.py /run/user/1000/wayland-1`:
  sämtliche bisherigen Fehlerfälle plus Kapazitätsprüfung bestanden.
  `target/p06-audit-errors.log`, `/tmp/niwoe-p06-errors.d1c80p4s/capacity.json`.
- `cargo test --workspace -q`: Exitcode 0, `target/p06-audit-tests.log`.
- Dokumentations-/Codeabgleich und `git diff --check` ohne Fehler.

Kein Rust-/Produktcode geändert, keine neue Binärinstallation erforderlich.
Das neue Testskript liegt auch auf Fedora. Installierter Produktstand bleibt
Compositor `e1953fc4…`, Shell `5636d59c…`; keine Sitzung beendet.

## Abschlussbewertung

Die damalige Restreihenfolge ist abgearbeitet: vollständige Definitionen und
kompatible Übernahme, Registry/IPC/Shell-Anbindung, isolierte Wiederholungsprüfungen,
Installation und echte Login-/Move-Nachprüfung. Der anfängliche manuelle Move
erhielt zwar Space-Zuordnung und Inhalt, führte aber die WM-Mitgliedschaft nicht
mit. Die Korrektur wurde auf dem neuen DRM-Build mit Floating → Tiling → Floating
im Zielraum, separatem Tiling-Move sowie XWayland-Move nachgeprüft.
Alle Testfenster sind entfernt; die echte Raumdatei bleibt nach Migration
bei Revision 49 und Zähler 19. Einzelheiten im [Endstandsbericht](P06_SCHEMA_COMPLETION.md).
P07 hat in dieser Runde nicht begonnen.

P07-Overflow über neun Räume und App-Zuordnungslogik, P09-Layoutwiederherstellung
sowie alte README-/Login-/Settings-Dokumentationsreste bleiben ausdrücklich
außerhalb dieses Abschlussabgleichs. USB-Hub ist als Verkabelungsbefund erledigt.
