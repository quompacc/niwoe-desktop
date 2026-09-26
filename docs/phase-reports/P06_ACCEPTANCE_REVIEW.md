# P06 – Abschlussabgleich, 26.09.2026

Ergebnis: **in-progress, nicht accepted**. Die geprüften Fehlerkorrekturen und
Live-Abläufe bleiben gültig. Der Abgleich findet zusätzliche Modellpflichten,
die noch nicht implementiert sind; der Phasenabschluss wird nicht vorgezogen.
Maßgeblich sind der aktive Plan §4.2 und P06-01 bis P06-06. Dieser Bericht
fasst die chronologischen Zwischenberichte zusammen und ersetzt deren jeweils
damalige offene Punkte als aktuelle Übersicht.

## Arbeitspakete gegen den Code

| Paket | Ergebnis | Beleg / verbleibende Arbeit |
| --- | --- | --- |
| P06-01 Typen, Validierung, Schema nach §4 | Teilweise | `niwoe-config/src/rooms.rs`: ID, Name, Beschreibung, Assignment, Reihenfolge/Zähler vorhanden. Icon, App-Präferenzen und Layout-/Restore-Einstellungen fehlen. |
| P06-02 Migration und atomare Speicherung | Für vorhandenes Schema belegt | Store-Tests, persistente stabile IDs, echte Loginprüfung; keine Wiederverwendung gelöschter IDs. Erweiterungsmigration für fehlende Felder noch nötig. |
| P06-03 Räume und konsistente Runtime-Slots | Vorhandene Abläufe belegt | Create/Edit/Reorder/Delete live, Fensterzustände und Slotverdichtung; zusätzliche IPC-Kapazitätsprüfung 1–64 bestanden. |
| P06-04 Commands, Snapshots, Acks, Revision | Für vorhandene Felder belegt | `niwoe-ipc/src/rooms.rs`, Registry und IPC; Konflikt/Reconnect getestet. Fehlende Modellfelder besitzen auch noch keinen Wire-Vertrag. |
| P06-05 Raum-/Fensterzuordnung | Wesentliche Fälle belegt, separater Nachweis offen | Raumwechsel, Delete-Migration, Dialoge, minimiert/Floating, zwei Outputs/Hotplug geprüft. Manuellen Raum-Move unabhängig vom Löschen nochmals gezielt live belegen. |
| P06-06 Shell als Snapshot-Consumer | Für vorhandenen Umfang belegt | Authentifizierungs-/Reconnect-Korrektur, vollständige aktuelle Snapshots und Revisionskonflikte. Erweiterte Felder müssen in denselben Pfad aufgenommen werden. |

Die fehlenden Modellfelder sind keine neu erfundenen Anforderungen:
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

## Zusätzliche Prüfung dieser Runde

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

## Konkrete verbleibende Reihenfolge

1. Fehlende Raumdefinitionen für vorhandenes Icon, App-Präferenzen und
   Layout-/Restore-Einstellungen typisieren, validieren und bestehende Dateien
   ohne Datenverlust übernehmen. Kein Prozess-/Fensterhandle persistieren.
2. Daten konsistent durch Registry, IPC-Snapshot und Shell-Consumer führen;
   Roundtrip-, Kompatibilitäts- und Fehlerfälle prüfen. Keine zweite Shell-Policy.
3. Manuellen Fensterverschiebepfad separat mit ID-/Inhalts-/Fokusprüfung live
   belegen; anschließend betroffene Integrationsfälle auf dem Endstand wiederholen.
4. Erst dann P06-Diff und Nachweise abschließend bewerten und gegebenenfalls
   `accepted` setzen. P07 hat in dieser Runde nicht begonnen.

P07-Overflow über neun Räume und App-Zuordnungslogik, P09-Layoutwiederherstellung
sowie alte README-/Login-/Settings-Dokumentationsreste bleiben ausdrücklich
außerhalb dieses Abschlussabgleichs. USB-Hub ist als Verkabelungsbefund erledigt.
