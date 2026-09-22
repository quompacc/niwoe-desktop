# NIWOE: Übergabe an Terra/Sol und Phasenreview

> **Arbeitsweise abgelöst am 21.09.2026:** Der Nutzer hat die Modellübergaben
> beendet. Der aktuelle Agent implementiert, prüft und dokumentiert die Phasen
> selbstständig. Die nachfolgenden Übergabeprompts sind historische Vorlagen und
> verlangen keinen Reviewstopp mehr. Phasengates und Ergebnisnachweise gelten
> weiter gemäß `../NIWOE_IMPLEMENTATION_PLAN.md`.

Stand: 21.09.2026. Die Umsetzung ist noch nicht gestartet.

## 1. Arbeitsteilung

- Terra/Sol bearbeitet eine komplette beauftragte Phase: Code, Dokumentation,
  sinnvolle Tests, manuelle Verifikation, Diagnose und eigene Korrekturen.
- Astra prüft erst das fertige Phasenpaket. Es übernimmt keine Such-/Ersetzarbeiten,
  Testläufe im Auftrag des Implementierungsmodells oder gewöhnliche Bugfixes.
- Ein Gegencheck ist ein tatsächlicher Diff-/Verhaltensreview, keine pauschale
  Bestätigung aufgrund eines positiven Abschlussberichts.
- Eine Phase endet am Reviewpunkt. Nach Abnahme kann dasselbe Modell die nächste
  Phase übernehmen. Keine Rückfrage pro Ticket, kein automatischer Sprung über Gates.

## 2. Kopierfertiger Startauftrag

Im folgenden Text `P00` bei späteren Phasen durch die gewünschte Phase ersetzen.
Modellwahl: Terra für P00/P02–P05/P08/P10/P11; Sol bevorzugt für P01/P06/P07/P09/P12.

```text
Setze Phase P00 aus NIWOE_IMPLEMENTATION_PLAN.md vollständig um.

Lies AGENTS.md, NIWOE_IMPLEMENTATION_PLAN.md, docs/NIWOE_DESIGN_BRIEF.md und
docs/NIWOE_AGENT_HANDOFF.md. Prüfe den letzten abgenommenen Phasenbericht.
Für P00 ist kein vorheriger Bericht erforderlich. Lies danach gezielt die
betroffenen Dateien und das geltende Designmanifest.

Arbeite alle Tickets der Phase selbstständig ab. Halte dich an die dort
getroffenen Architekturentscheidungen, den bestehenden Rust-Pfad und die
600-Zeilen-Grenze. UI bleibt nativ. Verwende die vorhandenen Komponenten,
Provider und Caches. Bewahre fremde/uncommittete Änderungen.

Du brauchst keinen Astra-Review zwischen Tickets. Löse gewöhnliche Compiler-,
Test-, Format- und Integrationsfehler selbst. Prüfe jeweils den tatsächlichen
Call-Flow. Erweitere den Scope nicht durch neue Features oder Dependencies.
Die beauftragte P01-Umbenennung erlaubt die dafür erforderlichen Cargo-
Manifeständerungen; sie erlaubt keine beliebigen Dependency-Upgrades.

Erstelle kleine überprüfbare Änderungen. Führe die Phasenchecks auf dem
dokumentierten Linux-Testhost aus. Nach Logik-/Teständerungen Workspace-Tests.
Berichte nie einen nicht ausgeführten Test als erfolgreich. Fehlende Runtime-
oder Hardwarebelege bleiben NOT RUN und bei Pflichtfällen ein Blocker.

Erzeuge docs/phase-reports/P00.md mit Tickets, Änderungen pro Datei,
Basis-/Endstand, Befehlen und Ergebnissen, Runtimebelegen, Screenshots,
Performance und offenen Risiken. Erstelle die Übersicht aller Phasen dort,
falls sie noch fehlt. Markiere diese Phase erst bei bestandenen Gates als
implemented. Halte dann für den abschließenden Astra-Gegencheck an.

Keine Veröffentlichung, kein Release, kein Force-Push und kein Umbau fremder
Repositoryhistorie. Bei einem tatsächlichen Blocker: minimale Reproduktion,
bisherige Versuche und konkrete benötigte Entscheidung liefern. Nicht bei
bereits entschiedenen Detailfragen stoppen.
```

## 3. Vorgehen innerhalb einer Phase

1. Basis-Commit, Branch und bestehenden Arbeitsbaum erfassen. Vorhandene
   Nutzeränderungen mit Pfad dokumentieren. Keine fremden Änderungen zurücksetzen.
2. Tickets als kleine Checkliste führen. Vor jedem Ticket relevante vorhandene
   Tests und Aufrufer lesen; nicht zuerst neue Abstraktionen schreiben.
3. Pro Verantwortung ändern. Große Rust-Dateien zuerst verhaltensgleich aufteilen
   und prüfen, erst anschließend neue Logik einbauen.
4. Verifikation passend zum Risiko: Pure Functions mit Tests, IPC mit Serialisierung/
   Fehlerfällen, UI zusätzlich real starten, Hardwareaktionen tatsächlich nachweisen.
5. Auftretende Regressionen im selben Ticket beheben. Tests nur ändern, wenn die
   neue Sollsemantik ausdrücklich im Auftrag steht; niemals rote Tests entfernen,
   nur weil die neue Implementierung sie nicht erfüllt.
6. Nach allen Tickets komplette Phasengates und Bericht. Endstand eindeutig angeben;
   falls uncommittet, Diff und neue Dateien vollständig aufzählen.

Ein Commit pro abgeschlossenem logischen Teil ist sinnvoll, sofern Git-Schreiben
autorisiert/verfügbar ist. Kein erzwungener Commit schmutziger fremder Arbeit;
Review ist auch mit eindeutigem Diff und inventarisierten neuen Dateien möglich.

## 4. Was ohne Rückfrage entschieden werden darf

- Modulschnitt und interne Funktionsnamen innerhalb der bestehenden Zuständigkeit.
- Kleine verhaltensgleiche Aufteilung für die 600-Zeilen-Regel.
- Fehlertexte, Testszenarien und Layoutdetails innerhalb zentraler Designrollen.
- Wiederverwendung eines vorhandenen Widgets statt ähnlichen Neucodes.
- Behebung eigener Fehler und eindeutig belegter Regressionen der Phase.
- Dokumentation der tatsächlich ausgeführten Kommandos und Testumgebung.

Nicht still entscheiden: globale Änderung der Output-/Fokuspolitik, neuer Root-
Service, neue Runtime oder Dependencies, andere Restoregarantien, Paketmanager,
Entfernen vorhandener Sicherheitsgrenzen oder Löschung von Nutzerdaten.

Terra eskaliert einen eingegrenzten technischen Blocker mit Belegen an Sol.
Architekturabweichungen kommen mit Lösungsvorschlag zum Phasenreview; wenn die
Weiterarbeit darauf zwingend angewiesen ist, werden sie unmittelbar als Blocker
gemeldet. Der Reviewrhythmus ist kein Grund, unsichere Entscheidungen zu verstecken.

## 5. Berichtvorlage: docs/phase-reports/Pxx.md

```markdown
# Pxx — Titel

Status: in-progress | implemented | accepted | blocked
Implementierungsmodell:
Datum:
Basis-Commit:
End-Commit oder vollständiges Diff-Inventar:
Branch:
Testhost / Distribution / Rust / Display / GPU:
Vorherige abgenommene Phase:

## Ergebnis
Konkretes nun funktionierendes Verhalten in 2–5 Sätzen.

## Tickets
- [ ] Pxx-01: Ergebnis und zugehörige Änderung

## Geänderte Dateien
| Datei | Änderung (1–3 Zeilen) | Grund |
|---|---|---|

## Verifikation
| Befehl/Testfall | Umgebung | Ergebnis/Exitcode | Beleg |
|---|---|---|---|

## Call-Flow
Eingabe → Verantwortlicher → Command → Zustandsänderung → Event → Render/Fokus.
Fehler, Reconnect und Lock-Verhalten ausdrücklich angeben.

## Visuelle Belege
Dark/Light, Auflösung, Skalierung, Screenshotpfade; Vergleich zur Referenz.

## Performance
Baseline, Messmethode, drei Messwerte, Repaint/CPU/GPU/RSS soweit verfügbar.
Cache-Schlüssel, Invalidierung, Grenze und Lebensdauer.

## Offene Risiken und Abweichungen
Konkrete Einschränkung, Wirkung, nächste Maßnahme. NOT RUN ausdrücklich nennen.

## Review
Durch Implementierungsmodell nicht als accepted markieren.
Reviewer trägt Urteil/Findings und geprüften Stand ein.
```

## 6. Kopierfertiger Auftrag für den Gegencheck

```text
Prüfe die umgesetzte Phase Pxx anhand von NIWOE_IMPLEMENTATION_PLAN.md,
docs/NIWOE_DESIGN_BRIEF.md und docs/phase-reports/Pxx.md.

Lies den tatsächlichen Diff einschließlich neuer Dateien und die relevanten
Aufrufer/Tests. Prüfe Anforderungen, Fehlerfälle, Sicherheitsgrenzen,
Renderreihenfolge, Fokus/Outputs, zentrale Tokens und Performance-Modell.
Vergleiche die UI-Belege mit den Mockups und der vereinbarten Präzisierung.
Fehlende Pflichtbelege sind ein konkretes Finding, kein angenommener Erfolg.

Führe bei unklarer Evidenz gezielte eigene Verifikation durch. Übernimm keine
Routineimplementierung. Gib priorisierte Findings mit Datei/Zeile, Auswirkung
und prüfbarem Korrekturziel zurück. Terra/Sol soll sie selbst beheben.
Wenn keine blockierenden Findings bleiben, dokumentiere accepted für den
geprüften Stand und benenne die nächste Phase. Behaupte keine Fehlerfreiheit
außerhalb des tatsächlich geprüften Umfangs.
```

## 7. Startstatus

P00–P13: `not-started`. Planung und Mockupsichtung abgeschlossen; keine
Implementierungsphase ausgeführt oder abgenommen. Namensmigration, Remote-
Änderung und neue Produktregeln sind beauftragte Umsetzungsarbeit für Terra/Sol.
