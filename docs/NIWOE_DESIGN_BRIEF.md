# NIWOE: Design-Brief aus den aktuellen Mockups

**Verbindliche Korrektur, 22.09.2026:** Nutzer erklärt die acht Mockups zur
visuellen Vorgabe. Nachfolgende frühere redaktionelle Reduktionen gelten nur,
soweit sie ausdrücklich vom Nutzer bestätigt wurden oder technisch notwendig
sind. Nicht als Erlaubnis für eine vereinfachte Ersatzgestaltung verwenden.
Siehe die aktuelle Entscheidung im Designmanifest.

**Präzisierung 24.09.2026:** Hub, Räume verwalten, Raum konfigurieren und ihre
Navigation sind als vollständiger Workflow bildtreu umzusetzen. Die frühere
Reduktionssprache in §3 und §5 beschreibt keine Erlaubnis, Karten, Spalten,
Filter, Reiter, Vorschau oder Informationsbereiche aus den Mockups zu entfernen.
Verbindlich sind allein die letzten vier Bilder `17_13_54 (1)` bis `(4)`;
die ersten vier `17_13_13 (1)` bis `(4)` sind redundanter Kontext.
Verbindliche Zuordnung und Phasengates:
[Mockup- und Workflowplan](MOCKUP_WORKFLOW_PLAN.md).

**Theme-Entscheidung 25.09.2026:** Für die Desktop-Alpha ist nur das
dunkelgrüne NIWOE-Theme verbindlich. Die folgenden Light-Werte dokumentieren
einen früheren Entwurfsstand und erzeugen kein Alpha-Liefergate. Eine helle
Variante erfordert eine spätere Produktentscheidung.

Stand: 21.09.2026. Alle acht PNGs unter `assets/` wurden visuell angesehen;
die Nutzerkorrektur vom 24.09.2026 begrenzt die Bildbindung auf die letzten vier.
Dieser Brief präzisiert den vorgeschlagenen Produktentwurf. P00 übernimmt ihn
in das verbindliche Designmanifest; bis dahin ersetzt er es nicht stillschweigend.

## 1. Referenzen und ihre Rollen

Die Dateinamen beginnen jeweils mit `ChatGPT Image 21. Sept. 2026, `.

| Rest des Dateinamens | Inhalt | Verwendung |
|---|---|---|
| `17_13_13 (1).png` | Willkommen/Übersicht mit vier Raumkarten und Kennzahlen | Redundanter Kontext; kein eigenes Bildgate |
| `17_13_13 (2).png` | Räume, sechs Karten, Filter und erklärende rechte Spalte | Redundanter Kontext; `17_13_54 (3)` ist maßgeblich |
| `17_13_13 (3).png` | Raum Entwicklung mit eingebetteten Appansichten und Aufgaben | Redundanter Kontext; kein Raumdetail-Bildgate |
| `17_13_13 (4).png` | Horizontaler Raumwechsel mit großer ausgewählter Karte | Redundanter Kontext; kein eigener Raumwechsel-Bildschirm vorgeschrieben |
| `17_13_54 (1).png` | Desktop, obere Raumleiste, kleines Deck rechts | Wichtigste Referenz für normalen Desktop und Deck |
| `17_13_54 (2).png` | Hub-Overlay mit Räumen, letzten Dateien, Aufgaben, Systemzustand | Raumkarten/Overlay; zusätzliche Bereiche erst mit realer Datenquelle |
| `17_13_54 (3).png` | Control Center, Räume verwalten | Informationsdichte, Navigation und Filter |
| `17_13_54 (4).png` | Raum konfigurieren, Tabs, Formular, Restore, Vorschau | Formularhierarchie und Einstellungen; nur umgesetzte Fähigkeiten aktiv |

Alle gelieferten Ansichten sind dunkel und tragen noch NIWOE-Schriftzüge und
Kompassmarken. Sie sind unveränderte Referenzbilder, keine fertigen NIWOE-Assets.
Es gibt keine helle Ansicht und keine vollständige Wizard-Sequenz. Animation kann
aus statischen Bildern nicht abgeleitet
werden. Fehlende Referenzen werden über gemeinsame Regeln ergänzt, nicht erfunden
und anschließend als Nutzerfreigabe ausgegeben.

## 2. Was übernommen wird

- Tiefe schwarzgrüne Flächen, warmes Off-White, zurückhaltendes Messing/Gold.
- Feine Konturen und klar getrennte Flächen statt dicke Fensterrahmen.
- Große ruhige Überschriften in Verwaltungs-/Willkommensansichten, kompakte
  sachliche Schrift für Bedienung, Status und Listen.
- Raumkarten mit Name, kurzer Beschreibung, klarer Auswahl und kleinen App-Icons.
- Top-Panel mit Raumorientierung; kleines Deck statt bildschirmfüllendem Dashboard.
- Gleiche visuelle Familie für Desktop, Hub, Settings und Onboarding.

## 3. Präzisierungen gegenüber illustrativen Inhalten

Die Bildköpfe, Hintergrundstruktur, Karten, Spalten und Informationsbereiche
gehören zur verbindlichen Komposition der gezeigten Seiten. Wiederholte alte
MERIDIAN-Marken und Kompassflächen entfallen gemäß Manifest; die übrige
Seitenarchitektur bleibt erhalten. Im Hub sind Räume die erste Information, in
Settings das bearbeitete Formular. Fehlende Datenquellen erhalten einen
verständlichen Fähigkeitszustand an der vorgesehenen Stelle.

Auch Gold wird in den Bildern häufig für nahezu jedes Icon und jede Kontur benutzt.
Für das Produkt gilt das Konzept: Gold kennzeichnet aktuelle Auswahl/Fokus und
die wesentliche Aktion. Inaktive Symbole und Borders bleiben neutral. Erfolg,
Warnung und Fehler bekommen eigene semantische Farben plus Text/Icon.

Die Landschaft ist Hintergrund/Asset, kein pro Frame zu berechnender Effekt.
UI muss auch ohne dieses Wallpaper und vor einem hellen Bild lesbar sein.
Die sichtbaren App-Innenflächen sind illustrative Beispiele; NIWOE zeichnet sie
nicht nach und injiziert kein erzwungenes GTK-/Qt-/Electron-Theme.

Verbindliche Branding-Regel seit P00: einfache NIWOE-Wortmarke nur in Welcome,
Login und About. Kein weiterverwendeter NIWOE-
Kompass als Standardicon. Der Hub-Zugang bekommt ein neutrales funktionales Symbol.
Eine endgültige eigenständige NIWOE-Bildmarke ist ein späteres Asset, kein Blocker.

## 4. Semantische Designquelle

`niwoe-tokens` + `niwoe-config`. Bestehende `Palette`, `Interaction`, `Elevation`, `Radius`,
`Decorations` und Geometrietokens erweitern. Keine zweite JSON-/CSS-Palette,
keine gemessenen PNG-Farben direkt im Renderer.

Benötigte Rollen, auf bestehende Felder abbilden oder zentral ergänzen:

| Rolle | Zweck |
|---|---|
| surface.base / raised / overlay | Desktopnahe Fläche, Karte, Popup |
| text.primary / secondary / disabled | Informationshierarchie |
| border.subtle / focus | neutrale Trennung bzw. klarer Fokus |
| accent / on_accent | Gold und darauf lesbarer Text |
| status.success / warning / error | echte Zustandsinformation |
| spacing / typography / radius / elevation | gemeinsame Geometrie der nativen Komponenten |

### Startwerte für P02

Die folgende Tabelle bewahrt die historische Entwurfsbasis. Für die Desktop-Alpha
ist nur die Dark-Spalte produktverbindlich. Implementierte P02-Werte und
Kontrastbegründung: [P02-Komponenten](design/P02_COMPONENTS.md). Die Light-Spalte
dokumentiert frühere Messungen ohne aktuelle Produktpflicht.

Dies sind begründete Entwurfswerte, keine pixelgenaue Extraktion aus den Bildern.
P02 darf sie nach Kontrast- und Rastervergleich innerhalb dieser Rollen verfeinern
und dokumentiert die endgültigen zentralen Werte.

| Rolle | Dark | Light |
|---|---|---|
| Basis | `#101710` | `#F4F1E7` |
| erhöhte Fläche | `#19221A` | `#FFFCF3` |
| Overlay | `#202B22` | `#E9E9DC` |
| Primärtext | `#F1EEE3` | `#18271D` |
| Sekundärtext | `#BAC3B7` | `#4F5E50` |
| dezente Kontur | `#435044` | `#BDC5B5` |
| Akzent | `#D6B35B` | `#735715` |
| Text auf Akzent | `#151B13` | `#FFFFFF` |

Gold im Light-Theme ist für Text/Fokus dunkler: dasselbe helle Gelb auf Elfenbein
wäre kein verlässlicher Kontrast. Entscheidend ist die gemeinsame Farbfamilie.
Warn-/Fehlerfarben bleiben eigenständige geprüfte semantische Rollen.

Projektziel: Textkontrast mindestens 4,5:1 für normalen Text, 3:1 für große Schrift
und wesentliche Bedienelementgrenzen. Fokus wird nicht nur durch Farbe vermittelt.
Die Werte müssen auf den tatsächlich zusammengesetzten Flächen geprüft werden;
Transparenz kann eine isolierte Farbpaarprüfung ungültig machen.

### Geometrie und Schrift

- Abstandsleiter als Entwurfsdefault: 4/8/12/16/24/32 logische Pixel.
- Top-Panel: 48 logische Pixel; Controls mindestens 32 hoch; keine großen Logos.
- Controls für Formulare: 36 logische Pixel; Karten innen 16, größere Gruppen 24.
- Radien als Start: klein 4, mittel 8, groß 12; keine Pillen für jede Fläche.
- Sans-Text 14, sekundäre Labels 12, Abschnitt 18; große Überschriften 28–32.
- Vorhandene Fontauflösung nutzen; Sans-Fallback muss vollständig funktionieren.
  Serif nur auf ausdrücklich ausgewählten Überschriften, nie auf dichtem Statustext.
- Alle Werte zentral als Rollen/Tokens. Renderer nimmt keine lokale Kopie auf.
- Das grüne Alpha-Theme verwendet gemeinsame Abmessungen, Schatten, Blur,
  Alpha und Interaktionsregeln aus Tokens/Config.
- Alpha startet ohne neue Animation. Vorhandene Effekte nur mit belegter
  Cache-/Invalidierungsstrategie; neue Bewegung benötigt später ein Motion-Spec.

## 5. Spezifikation der Oberflächen

### Desktop und Panel

Desktop ist die Arbeitsfläche der Apps. Keine Dashboardkarten als ständige Ebene.
Panel oben: Hub-Zugang, Räume, flexible Lücke, verfügbare Module, Uhr. Aktiver Raum
Gold, nicht aktive Räume neutral. Bei Platzmangel Overflow statt kollidierender
Texte. Optionen ändern Module, nicht die grundlegende Designsprache.

Nutzerentscheidung vom 22.09.2026: keine zusätzliche NIWOE-Titelleiste, auch
nicht bei freien Fenstern. Compositor-eigene Dekoration beschränkt sich auf
eine feine Außenkontur von 1–2 logischen Pixeln. Schließen ist per Super+W
erreichbar; Super+Ziehen verschiebt,
Super+Rechtsziehen vergrößert/verkleinert. Das verbindliche vollständige
Bedienziel steht in [NIWOE_INTERACTION_MODEL.md](NIWOE_INTERACTION_MODEL.md).
Native CSD bleibt erhalten. Floating darf den bestehenden gecachten Schatten
nutzen. Alpha führt keinen neuen Effekt allein für die Mockupnähe ein.

### Hub-Zugang

Die neutrale Schaltfläche im Panel öffnet direkt den Hub aus `17_13_54 (2)`.
Es gibt keine getrennte Spotlight-, App-Raster- oder Startmenü-Oberfläche.
Anwendungssuche ist eine Funktion innerhalb der im Mockup vorgesehenen Hub- und
Control-Center-Struktur und ersetzt deren Raumkarten, Bildkopf oder Navigation nicht.

### Deck

Rechts oben unter dem Panel, Entwurfsbreite 360 logische Pixel. Audio zuerst,
darunter wenige verfügbare Systemschalter, Raumhinweis nach P06, Link zu Settings.
Nicht verfügbare Funktionen verständlich kennzeichnen oder ausblenden; keine
grünen Beispieldaten. Keine automatisch wandernden Controls in der Alpha.

### Hub

Großes zentriertes Overlay nach `17_13_54 (2).png`: Bildkopf mit Titel und
Schließen, darunter Raumkarten und die drei Bereiche „Zuletzt aktiv“,
„Aufgaben im Fokus“ und „Systemzustand“. Raumkarten zeigen echten Namen,
Beschreibung, Auswahl, App-Icons und vorhandene Vorschau. Auf schmalen Outputs
brechen Karten und untere Bereiche um und werden vertikal scrollbar; sie werden
nicht durch eine andere Popupform ersetzt. Die Raumverwaltung ist vom Hub aus
sichtbar erreichbar; Formularbearbeitung findet auf der Settings-Seite statt.

Die drei unteren Bereiche behalten ihren Platz. Sie zeigen reale verknüpfte
Dateien, Aufgaben und Systemdaten, sobald deren Quelle vorhanden ist; vorher
zeigen sie einen klaren Fähigkeitszustand. Kein Aufgabenmanager, keine erfundene
Aktivitätschronik und keine vorgetäuschte Systemtelemetrie.

### Settings und Wizard

„Räume verwalten“ folgt `17_13_54 (3).png`: linke Navigation, Bildkopf,
Statusfilter, Sortierung, Kategorie/Suche, Neuer Raum, dichtes Kartenraster,
rechte Schnellaktionen und Statistik. „Raum konfigurieren“ folgt
`17_13_54 (4).png`: Breadcrumb, Reiter, Details, Kontext/Wiederherstellung,
Start-Apps und Regeln links, große Vorschau und Erklärung rechts, unten
Abbrechen und Speichern. Nicht vorhandene Funktionen behalten ihren Platz als
klarer Fähigkeitszustand. Auf kleinen Flächen ordnen sich Spalten vertikal;
alle Bereiche und Aktionen bleiben erreichbar. Speichern/Abbrechen und
Feldfehler sind eindeutig. Der durchgehende Ablauf steht im
[Mockup- und Workflowplan](MOCKUP_WORKFLOW_PLAN.md).

Wizard erklärt eine Entscheidung pro Schritt, zeigt dieselben echten Widgets
und besitzt gute Defaults. Große Bild-/Serifakzente sind hier vertretbar.

## 6. Visuelle Abnahme

P02 erstellt eine kleine Vergleichsgalerie aus realer nativer Ausgabe. Jede weitere
UI-Phase ergänzt ihre Oberfläche im verbindlichen grünen Theme. Originalmockup und Screenshot
werden nach Farbrollen, Hierarchie, Abständen, Dichte, Fokus und Bedienbarkeit
verglichen; ein Pixelgleichheitstest gegen generierte App-Inhalte wäre ungeeignet.

Pflichtfälle: leer, viele Einträge, lange deutsche Namen, Fehler, Disabled, Keyboard-
Fokus, 1366×768, 1920×1080, unterschiedliche Skalierung und Wallpaperhelligkeit.
Screenshots dürfen nicht allein die funktionale oder Performance-Abnahme ersetzen.
