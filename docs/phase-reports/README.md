# NIWOE-Phasenstatus

Aktueller Stand 02.10.2026: **Die visuelle Desktop-Abnahme ist wieder offen.**
Der erneute Vergleich mit den vier verbindlichen Mockups bestätigt die
Nutzerkritik. Technische Abschlüsse sind keine Freigabe der sichtbaren UI.
[UI-Prüfbericht mit Screenshots und installiertem Navigationsfix](UI_VISUAL_AUDIT_2026-10-02.md).
Die Abarbeitung folgt dem [aktiven Reparaturplan mit V01–V20 und Abnahmekriterien](../../NIWOE_IMPLEMENTATION_PLAN.md#aktiver-reparaturplan-für-die-benutzeroberfläche).

| Phase | Status | Bericht |
|---|---|---|
| P00 | historisch accepted | [P00.md](P00.md) |
| P01 | historisch accepted | [P01.md](P01.md) |
| P02 | visuelles Gate wieder offen | [P02.md](P02.md) |
| P03 | visuelles Gate wieder offen | [P03.md](P03.md) |
| P04–P05 | visuelle Gates wieder offen | [Hub](P04_HUB_FOUNDATION.md), [Deck](P05_DECK_COMPOSITION.md) |
| P06 | technische Belege vorhanden, visuelle Abläufe wieder offen | [P06.md](P06.md) |
| P07 | technische Belege vorhanden, visuelle Abläufe wieder offen | [Abnahmebericht](P07_ACCEPTANCE_REVIEW.md) |
| P08 | technische Belege vorhanden, visuelles Gate wieder offen | [P08.md](P08.md) |
| P09 | technische Belege vorhanden, sichtbarer Restore-Ablauf wieder offen | [P09.md](P09.md) |
| P10 | technische Belege vorhanden, visuelles Gate wieder offen | [P10.md](P10.md) |
| P11 | technische Belege vorhanden, visuelles Gate wieder offen | [P11.md](P11.md) |
| P12 | Gesamtfreigabe wieder offen | [P12.md](P12.md) |
| P13 | not-started | — |

## Historische Chronik

**Eintrag 22.09.:** P02-Designgrundlagen implementiert und geprüft.
Auf Nutzerauftrag keine weitere Abnahmerunde der alten Shell. Panel, Such-Launcher
und System-Deck sind nativ neu aufgebaut: [Umsetzungsstand](P03_P05_NATIVE_REBUILD.md).
Offene Scale-/HiDPI-/Performance-Nachweise an den neuen
Oberflächen erbringen; P02 bleibt bis dahin ausdrücklich unvollständig abgenommen.
Maßgeblich: [aktueller Handoff](P02_HANDOFF.md) und die Scope-Korrektur im Plan.

P01, Abschluss 22.09.2026: Namensmigration und nicht überschreibende
XDG-/Installationsmigration implementiert und lokal committet. Fedora-Gates
grün: 1.085 Tests bestanden, 2 bewusst ignoriert, Release und Guards bestanden.
Nested, Staging-Installation, Live-IPC, Portal-Backendneustart, fehlender Picker
und Polkit-Neuregistrierung einschließlich Passwort-/Abbruchfällen geprüft.
Beim Shellneustart gefundene doppelte Polkit-Instanz korrigiert und nachgeprüft.
Sperrfokusfehler nach Portalstart korrigiert: alter Release fällt im isolierten
Protokolltest durch, korrigierter Release besteht einschließlich Popup-Grab,
Unlock und fail-closed bei Clientverlust. Korrektur installiert. Echte Lock-
Passwortfälle, Dateiportal SaveFile/SaveFiles/Abbruch und normaler KDE-Rundlauf
bestanden; gemeinsame Einstellungen unverändert, NIWOE-Dienste beendet und
KDE-Portale aktiv. Code-Endstand `364edcd`; Diff, Restfunde und Belege geprüft.
**P01 accepted. P02 nicht begonnen.** Details: [P01.md](P01.md).

## Historische P00-Checkpoints

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
