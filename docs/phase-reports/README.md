# NIWOE-Phasenstatus

| Phase | Status | Bericht |
|---|---|---|
| P00 | accepted | [P00.md](P00.md) |
| P01 | in-progress | [P01.md](P01.md) |
| P02–P13 | not-started | — |

P01, Prüfcheckpoint 22.09.2026: Namensmigration und nicht überschreibende
XDG-/Installationsmigration implementiert und lokal committet. Fedora-Gates
grün: 1.085 Tests bestanden, 2 bewusst ignoriert, Release und Guards bestanden.
Nested, Staging-Installation, Live-IPC, Portal-Backendneustart, fehlender Picker
und Polkit-Neuregistrierung einschließlich Passwort-/Abbruchfällen geprüft.
Beim Shellneustart gefundene doppelte Polkit-Instanz korrigiert und nachgeprüft.
Für die Abnahme fehlen Lock, Dateiportal-Hardwarefälle und der abschließende
KDE-Rundlauf. Details und Nachweise: [P01.md](P01.md). P02 nicht begonnen.

P00: Fedora 44 KDE eingerichtet; zusätzliche Entwicklungssitzung installiert.
Format, Workspace-Check, Clippy mit `-D warnings`, alle 1.076 Workspace-Tests
und Build bestehen ohne zusätzlichen Paket-Ausschluss. Der isolierte Nested-
Test prüft Shell-Authentifizierung und ein tatsächlich zeichnendes Clientfenster.
Session-Vertragstest und systemd-Unit-Prüfung bestehen ebenfalls.
Am 22.09. bestehen außerdem echter Display-Manager-/DRM-Start, Panel/Launcher,
Portal-Theme-Abfrage und Polkit-Passwortdialog auf dem Acer. Folgearbeit bleiben
vollständige Portal-Konformität und die breiteren Hardwaremessreihen. Die im Plan
verlangte P00-Baseline ist abgenommen.
Video-Brücken-Autostart korrigiert und nach erneutem Login am 22.09. bestätigt:
kein Hilfsfenster, kein laufender Brückenprozess.
Details und Nachweise: [P00.md](P00.md).

Launcher-Performance: Debug-Zeichenpfad ca. 166 ms, Release ca. 5,9 ms im
reproduzierbaren CPU-Test. Release ist seit dem 22.09. installiert; der Nutzer
bestätigt deutlich schnellere Bedienung. Beim Logout gefundenes Portal-Frontend-Problem
im Wrapper korrigiert und mit beiden Desktop-Eigentumsfällen getestet;
Stop/Neustart beider Portal-Dienste beim NIWOE-Neulogin und korrekter Wechsel
zurück zu KDE bestätigt.

Der aktuelle Agent setzt um und prüft selbstständig; manuelle Modellübergaben
entfallen. Abhängige Phasen beginnen nach bestandenen fachlichen Gates.

KDE-Isolation: Alter Theme-Export hatte gemeinsame KDE-/GTK-Dateien und
Icon-Einstellungen überschrieben. Originaldateien restauriert; Nutzer bestätigt
vollständige Wiederherstellung der KDE-Symbole. Export auf privaten NIWOE-Pfad
begrenzt und persistente GSettings-Schreibaufrufe entfernt; Regressionstest und
Gates grün, korrigierte Release-Shell installiert. Erneuter Desktop-Rundlauf
bestanden: alle fünf KDE-/GTK-Dateien bytegleich, KDE-Theme/Icon-Einstellungen
erhalten, NIWOE-Dienste unter KDE beendet.

Polkit-Neuregistrierung implementiert und als Release installiert; privater
D-Bus-Integrationstest mit zwei Besitzerwechseln und Request-Abbruch besteht.
Alle Workspace-Gates grün. Echter Dienstneustart am 22.09. ebenfalls bestanden:
gleicher Agentprozess, neue polkitd-PID, anschließende Authentifizierung Exit 0.
Negativfälle auf Hardware ebenfalls bestanden: falsches Passwort wird
abgewiesen, Esc bricht nach Fehler sowie ohne Eingabe ab; jeweils Exit 1 und
keine Autorisierung. Sichtbarer Abbrechen-Button fehlt; Esc ist der Abbruchweg.

Dateiportal: SaveFile/SaveFiles liefern korrigierte URI-Arrays. Öffentlicher
Portalpfad auf Hardware für Speichern, Mehrfachspeichern und Abbruch bestanden.
Prüfstand am 22.09.: 1.076 Tests bestanden, 2 manuelle Tests bewusst ignoriert.
Nächste Phase: P01, vollständige NIWOE-Namensmigration.

## Pause und Wiedereinstieg — 22.09.2026 vormittags

Auf Nutzerwunsch beendet; Fortsetzung am Nachmittag. P00 ist accepted,
P01 noch nicht begonnen. Kein Testdialog und kein Build mehr offen.
Auf dem Acer ist die geprüfte Release-Version installiert; zuletzt lief die
NIWOE-Sitzung und der eigene Portal-Dienst war aktiv. Vor weiteren Hardwaretests
SSH-Erreichbarkeit und aktuelle Desktop-Sitzung neu prüfen.

Code, Sitzungsintegration und Abnahme sind in `e051efb`, `309110c` und `a3e2e81`
gesichert. Auf den ausdrücklichen Abschlussauftrag „alles … commiten“ werden
auch die acht bereitgestellten NIWOE-Mockups und die elf bereits vorhandenen
Löschungen alter Assets unverändert übernommen. Die frühere Entscheidung,
diese Bildänderungen separat im Arbeitsbaum zu belassen, ist damit aufgehoben.

Wiedereinstieg: P01 im `NIWOE_IMPLEMENTATION_PLAN.md` lesen, Umbenennungsinventar
erstellen und Migration einschließlich bestehender Konfigurationspfade planen.
GitHub wurde vom Nutzer bereits zu `niwoe-desktop` umbenannt; lokale Remotes
sind noch zu prüfen/anzupassen. Codeberg nicht ungeprüft auf einen neuen Namen
umstellen. KDE-Isolation und die dokumentierten Runtime-Tests müssen erhalten
bleiben. Kein neuer Featureblock vor dieser Migration.

Linux-Nachweise liegen unter `target/p00-evidence/`; lokale Evidence-Archive
liegen unter `target/` und sind bewusst keine Git-Artefakte. Ausführbare
Prüfverfahren und Ergebnisse sind im Repository dokumentiert. Keine erneuten
Rust-Gates für diesen reinen Dokumentations-/Asset-Abschluss erforderlich;
letzter geprüfter Code unverändert, 1.076 Tests bestanden.
