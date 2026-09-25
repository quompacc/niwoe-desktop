# Verbindlicher Mockup- und Workflowplan

Stand: 25.09.2026. Dieser Plan konkretisiert die Phasen P00–P13 in
[`NIWOE_IMPLEMENTATION_PLAN.md`](../NIWOE_IMPLEMENTATION_PLAN.md). Maßgebliche
visuelle Quelle sind **die letzten vier** PNGs
`assets/ChatGPT Image 21. Sept. 2026, 17_13_54 (1)` bis `(4).png`.
Die vier Bilder `17_13_13 (1)` bis `(4).png` wurden angesehen, sind nach
Nutzerkorrektur aber redundant und erzeugen keine eigenen Pflichtseiten.
Bei einer visuellen Entscheidung gilt zuerst das
[`Designmanifest`](niwoe_design_manifest.md), einschließlich der ausdrücklichen
Nutzerkorrekturen, dann das jeweilige Mockup. Dieser Plan legt die vollständigen
Bildschirme und ihre Übergänge fest. Ein früherer Bericht oder ein bestehender
vereinfachter Shell-Dialog ist keine Designfreigabe.

## 1. Die vier verbindlichen Lieferobjekte

| Bild unter `assets/` | Verbindliches Lieferobjekt | Bildtreue und Funktionsgrenze |
|---|---|---|
| `17_13_54 (1).png` | Alltagsdesktop | Obere, durchgehende Raumleiste; links eine stabile Folge benannter Räume mit aktivem Goldrahmen, rechts Status/Suche, kleines Systemdeck darunter. Die spätere Nutzerkorrektur setzt die Uhr exakt in die Bildschirmmitte. Kein Alltagslogo, keine angehefteten App-Symbole, keine untere Launcher-Leiste. |
| `17_13_54 (2).png` | Hub | Großes zentriertes Overlay mit Bildkopf, vier sichtbaren Raumkarten, drei unteren Informationsbereichen und Schließen. Raumkarten sind der Kern; Dateien, Aufgaben und Systemzustand erscheinen nur mit echter Datenquelle und sonst als klar benannter Fähigkeitszustand im gleichen Raster. |
| `17_13_54 (3).png` | Control Center: Räume verwalten | Vollständige Verwaltungsseite mit linker Navigation, Bildkopf, Filtern, Sortierung, Suche, „Neuer Raum“, Raster, rechter Schnellaktions- und Statistikspalte. Keine reduzierte Popup-Liste als Endzustand. |
| `17_13_54 (4).png` | Control Center: Raum konfigurieren | Breadcrumb, Bildkopf, Reiter, links Details, Kontext/Wiederherstellung, Start-Apps und Regeln; rechts große Vorschau und Erklärung; unten Abbrechen/Speichern. Nicht verfügbare Fähigkeiten bleiben sichtbar und verständlich als nicht verfügbar, niemals als wirksame Schalter. |

Die Bilder zeigen alte MERIDIAN-Marken und illustrative Fremd-App-Inhalte. Diese
werden gemäß Manifest durch NIWOE-Text an den erlaubten Stellen beziehungsweise
echte Inhalte ersetzt. Die Oberfläche, Informationshierarchie, Spalten, Karten,
Abstände, Zustandsdarstellung und Navigationswege werden nicht nach Geschmack
weggelassen. Eine technisch nötige Abweichung braucht vor Umsetzung einen Eintrag
mit Bild, Element, Grund und gleichwertigem Ersatz im Phasenbericht. Ein fehlender
Backend-Provider rechtfertigt einen sichtbaren Fähigkeitszustand, nicht das
Zusammenstreichen der gesamten Seite.

Die atmosphärischen Bildköpfe und strukturierte Hintergrundflächen sind Teil des
sichtbaren Entwurfs. Sie werden als geeignete, gecachte Assets mit derselben
Bildkomposition umgesetzt; ein flacher Farbblock zählt nicht als Bildabgleich.
Die Originale sind Referenzbilder, keine Quelle für herausgeschnittene Marken
oder vorgetäuschte App-Screenshots. Die Desktop-Alpha setzt diese Komposition
im verbindlichen dunkelgrünen NIWOE-Theme um.

## 2. Durchgehender Benutzerablauf

Eine neue Login-Sitzung beginnt in der neutralen **Loge**, ohne aktiven Raum.
Der Hub öffnet dort als Willkommensansicht. Schließen des Hubs lässt die Loge
mit Hintergrund und Raumleiste sichtbar; `Super+Space` öffnet den Hub erneut.
Erst die Wahl eines Raums setzt den aktiven Kontext. Ein direkter App-Schnellstart
aus der Loge aktiviert Raum 1 als Ziel für das neue Fenster. Die Loge zählt
nicht zu den neun Räumen und hat keine eigenen Raumfenster.

1. Der Desktop zeigt die gespeicherten Räume in **ihrer gespeicherten Reihenfolge**.
   Klick auf einen Raumnamen wechselt auf dem betreffenden Output. Der aktive Raum
   ist eindeutig hervorgehoben. Die neutrale Schaltfläche links im Panel öffnet
   direkt den Hub; eine separate Spotlight-/App-Launcher-Oberfläche existiert nicht.
2. Der Hub öffnet über dem Desktop und zeigt dieselben Raum-IDs, Namen, Reihenfolge
   und aktiven Zustand wie die Leiste. Karte oder Tastaturwahl öffnet den Raum;
   eine eindeutig beschriftete Aktion führt zu „Räume verwalten“. Escape schließt
   und gibt den Fokus zurück. Der Hub hat entsprechend Bild `(2)` keine Sidebar.
3. „Räume verwalten“ zeigt Suche, Filter, Sortierung und Kartenraster wie im Bild.
   Hier beginnt die vollständige linke Control-Center-Sidebar aus Bild `(3)`.
   Karten öffnen „Raum konfigurieren“. „Neuer Raum“ legt einen
   Raum über den echten persistierten Pfad an; Löschen verlangt bei belegtem Raum
   einen Zielraum. Listen-, Karten- und Panelzustand bleiben synchron.
4. „Raum konfigurieren“ öffnet den richtigen Raum anhand stabiler ID. Die Reiter
   bleiben räumlich und semantisch wie im Mockup; die linke Sidebar aus Bild `(4)`
   bleibt erhalten. Allgemein bearbeitet Name,
   Beschreibung und Symbol; Apps/Dateien/Wiederherstellung/Automatisierung/
   Benachrichtigungen zeigen ihre echten Fähigkeiten und Grenzen. Änderungen
   erscheinen in der Vorschau. Abbrechen verwirft den Entwurf; Speichern validiert
   und wartet auf bestätigte Persistenz. Fehler und Revisionskonflikte bleiben am
   Formular sichtbar, ohne den Entwurf zu verlieren.
5. Nach Speichern führen Breadcrumb oder Zurück zur Verwaltungsseite. Raumkarte,
   Hub und Panel zeigen sofort denselben bestätigten Namen, dasselbe Symbol und
   dieselbe Reihenfolge. Nach Shell-Neustart und erneutem Login bleibt der Stand.
   Ein Raumwechsel startet und beendet keine Anwendung.

Dieser Ablauf wird als ein Ende-zu-Ende-Fall auf dem Fedora-Testrechner geprüft:
Raum umbenennen, Beschreibung ändern, Reihenfolge ändern, neuen Raum anlegen,
zwischen Räumen wechseln, einen belegten Raum mit Zielraum löschen, Abbrechen,
Speichern, Neustart und Rücknavigation. Ein isolierter Editor-Popup erfüllt ihn
nicht.

## 3. Panelregel gegen abgeschnittene Räume

- Die Raumgruppe ist eine **stabile Projektion der gespeicherten Reihenfolge**.
  Aktivierung darf die sichtbare Folge nicht automatisch um den aktiven Raum
  verschieben. Das heutige gleitende Fenster über höchstens vier Positionen ist
  kein abgenommener Endzustand.
- Bei ausreichender Breite werden die im Desktop-Mockup sichtbaren vier benannten
  Räume in dieser Folge dargestellt. Weitere Räume bleiben über eine klar
  beschriftete Raumaktion vollständig erreichbar. Breite wird aus tatsächlicher
  verfügbarer Fläche und Textmessung abgeleitet; vier ist **kein globales Limit**.
- Bei Platzmangel schrumpfen zuerst optionale Statusmodule in einen erreichbaren
  Überlauf. Raumlabels werden erst danach gekürzt; voller Name, Zustand und
  Auswahl bleiben im Raumüberlauf zugänglich. Uhrmitte, Hub-Zugang und Klickflächen
  dürfen nicht kollidieren. Der aktive Raum muss dort eindeutig markiert sein,
  auch wenn er gerade im Überlauf liegt.
- Die Mockupnamen sind Beispiele, keine Seed-Daten. Gemessen wird mit echten
  gespeicherten Namen, einschließlich langen deutschen Namen, 1/4/9/64 Räumen,
  1366×768 und 1920×1080, im verbindlichen grünen Theme und bei mehreren Scales.
- Material für Panel und Hub folgt der bereits freigegebenen Deck-/Lautstärke-
  Behandlung. Keine neue Alpha-, Blur- oder Shadow-Sonderlösung. Die Uhr bleibt
  aufgrund der ausdrücklichen Nutzerkorrektur unabhängig von Gruppengrößen exakt
  in der Outputmitte.

## 4. Verbindliche Entwicklungsfolge

Die Phasennummern und technischen Arbeitspakete des Implementierungsplans bleiben
erhalten. Für die visuelle Abnahme gelten zusätzlich diese Liefergates. Bereits
geschriebener Code ist Bestand, kein Nachweis der Abnahme.
Seit der Nutzerentscheidung vom 25.09.2026 gelten die Gates für das grüne
Alpha-Theme. Eine helle Variante ist kein Teil dieser Phasenabnahme. Der
Theme-Wähler der älteren Settings-Seite ist noch vorhandener Code; seine
Entfernung aus dem aktiven Alpha-Ablauf ist in P10 eingeplant.

| Phase | Sichtbares Ergebnis und Gate |
|---|---|
| P00 | Produkt- und Designregeln, vier verbindliche Bildreferenzen, Linux-Testumgebung und reproduzierbare Baseline. Abnahme: ein aktiver Plan und dokumentierte Abweichungen. |
| P01 | Vollständige NIWOE-Namensmigration. Abnahme: keine MERIDIAN-Marke in der Produkt-UI; historische Bilder bleiben unverändert. |
| P02 | Zentrale Tokens und native Komponenten für das verbindliche grüne Theme. Die vorhandene Light-Palette ist historischer Bestand. Abnahme: Design-Guard, Kontrast und Komponentenblatt. |
| P03 | **Panel zuerst fertigstellen.** Desktopbild `17_13_54 (1)` mit freigegebenem Material, stabiler Raumfolge, mittiger Uhr, Status und Overflow gegen echten Desktop vergleichen. P03 bleibt bis zur visuellen Abnahme offen; keine weitere UI-Phase wird deshalb als abgeschlossen erklärt. |
| P04 | Zusammengehörige UI-Grundflächen nach `(2)`–`(4)`: Hub als großes zentriertes Overlay; „Räume verwalten“ und „Raum konfigurieren“ als vollständige Control-Center-Seiten mit Sidebar. Navigation Hub → Verwaltung → Konfiguration → zurück ist Teil desselben Lieferstands. Keine separate Spotlight-Oberfläche. Abnahme: vollständige Kompositionen, Tastatur, Maus und Fokus im grünen Theme am installierten Desktop. |
| P05 | Kleines Deck entsprechend `17_13_54 (1)` mit echten Audio-/Netzwerk-/Bluetooth-Zuständen und einem sichtbar beschrifteten Abmelden-Button über den vorhandenen Logout-Pfad. Abnahme: sichtbare und funktionale Parität einschließlich fehlender Geräte und Abmelden. |
| P06 | Persistente Raum-IDs, Namen, Reihenfolge, Hinzufügen/Löschen und revisionsgesicherte IPC. Abnahme: Raumzustand über Reconnect/Neustart und zwei Outputs; der abgelehnte Mini-Editor gilt nicht als UI-Abnahme. |
| P07 | Panel und Raumwahl verwenden dieselben Raumdaten und echte Fensterzuordnung. Abnahme: stabile Reihenfolge, alle Räume erreichbar, aktive Auswahl, keine verlorenen Fenster. Das frühe Raumwechselbild erzeugt keine eigene Bildschirm-Pflicht. |
| P08 | Hub-Datenintegration: Die in P04 gebaute Oberfläche verwendet persistente Raumdaten, echte Fenster/App-Zustände und definierte Provider/Fähigkeitszustände. Abnahme: Hub → Raumwechsel → Hub → Räume verwalten mit Tastatur und Maus. |
| P09 | Begrenzte, explizite Restore-Funktionen als Backend für Hub und Settings. Abnahme: UI behauptet nur erfüllbare Layout-/App-/Dateifunktionen; Raumwechsel löst keinen Restore aus. |
| P10 | Die bereits in P04 gebauten Control-Center-Seiten vollständig an Erzeugen, Löschen, App-/Dateizuordnung, Restore, Regeln und bestätigte Persistenz anbinden. Abnahme: vollständiger Ablauf aus §2, echte Vorschau, Speichern/Abbrechen/Fehler und Neustartpersistenz. |
| P11 | First Run gemäß bestehendem Implementierungsplan mit denselben nativen Komponenten. Die ersten vier Bilder legen dafür keine zusätzliche Oberfläche fest. Abnahme: neues und migriertes Profil, keine erfundenen Kennzahlen. |
| P12 | Gesamt-Desktop-Alpha auf Fedora: die vier verbindlichen Bildschirme und Übergänge als native Screenshots/Bedienläufe, 1366×768 und 1920×1080, Scale/Hotplug, Leistung und täglicher Arbeitsablauf. |
| P13 | OS-Architektur erst aus der abgenommenen Desktop-Alpha ableiten; keine UI-Ersatzlösung für offene P03–P12-Gates. |

P06 enthält bereits technische Teilimplementierung; der P06-Editor wurde visuell
nicht abgenommen. Der Nutzer hat am 24.09.2026 klargestellt, dass das Panel bereits
in Ordnung war; weitere Panelgestaltung ist damit beendet.

### Umsetzungscheckpoint 24.09.2026

- Der vollständige Hub aus `(2)` und die Grundfläche „Räume verwalten“ aus `(3)`
  sind installiert und im Dark Theme visuell bestätigt. Material, Abstand und
  Textzentrierung wurden anhand nativer Screenshots korrigiert.
- Der Hub ersetzt den separaten Launcher. Tippen startet die Suche; die unteren
  Karten enthalten echte Zustände und eine Schnellhilfe für die Bedienung.
- Der Hub öffnet einmal pro neuer NIWOE-Login-Sitzung als Willkommensansicht.
  Innerhalb derselben Sitzung öffnet ihn danach `Super+Space`; Watchdog-Neustarts
  lösen die Willkommensansicht nicht erneut aus.
- P04 bleibt **in Arbeit**. Der nächste sichtbare Schritt ist die vollständige
  Seite „Raum konfigurieren“ aus `(4)` samt Navigation aus der Verwaltung und
  zurück. Danach folgen die Zustandsabnahmen des gesamten Wegs im grünen Theme.
- Technischer und visueller Nachweis:
  [P04 Hub- und Control-Center-Zwischenstand](phase-reports/P04_HUB_FOUNDATION.md).

### Umsetzungscheckpoint 25.09.2026

Die native Seite „Raum konfigurieren“ aus `(4)` und der Weg von den Raumkarten
dorthin sind implementiert. Name und Reihenfolge nutzen bestätigte Mutationen;
noch fehlende Fähigkeiten sind sichtbar deaktiviert. Native Vorschau und alle
Rust-Pflichtprüfungen auf Fedora sind grün. Release und Installationsskript sind
installiert. Der frühere Versatz im Konfigurationskopf ist im Live-Screenshot
behoben; die Bildtreue der unteren Karten und die vollständige Bedienabnahme
stehen noch aus.

## 5. Prüfpaket für jeden sichtbaren Schritt

Vor Code: jeweiliges Originalbild neben den letzten **installierten**
NIWOE-Screenshot legen; Abweichungen zeilenweise nach Aufbau, Abständen,
Material, Typografie, Karten, Inhalt, Zuständen und Navigation erfassen. Nach Code:
Pflichtchecks aus `AGENTS.md`, Release auf dem Fedora-Testrechner installieren,
Buildidentität prüfen und, falls nötig, Neulogin ausdrücklich melden. Offscreen-
Rasterungen ergänzen den Vergleich, ersetzen ihn nicht. Performance-Modell benennt
Cache-Schlüssel, Invalidierung, Speichergrenze und Idle-Kosten.

Jeder Phasenbericht enthält Originalbild und nativen Screenshot des grünen Themes,
Zustände für leer/voll/lang/Fehler/Fokus, getesteten Workflow, Buildidentität,
Prüfbefehle und eine Tabelle verbleibender Bildabweichungen. Eine Phase erhält
`accepted` erst nach echter visueller und funktionaler Prüfung am installierten
Stand. Ein technischer Testlauf allein reicht nicht. Keine neue Oberfläche wird
durch einen kleineren Popup oder eine textuelle Liste als „mockupgetreu“ verbucht.
