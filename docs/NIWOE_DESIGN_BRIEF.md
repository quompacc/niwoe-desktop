# NIWOE: Design-Brief aus den aktuellen Mockups

Stand: 21.09.2026. Alle acht PNGs unter `assets/` wurden visuell angesehen.
Dieser Brief präzisiert den vorgeschlagenen Produktentwurf. P00 übernimmt ihn
in das verbindliche Designmanifest; bis dahin ersetzt er es nicht stillschweigend.

## 1. Referenzen und ihre Rollen

Die Dateinamen beginnen jeweils mit `ChatGPT Image 21. Sept. 2026, `.

| Rest des Dateinamens | Inhalt | Verwendung |
|---|---|---|
| `17_13_13 (1).png` | Willkommen/Übersicht mit vier Raumkarten und Kennzahlen | Onboarding-Tonalität und Kartenfamilie; kein dauerhafter Desktop |
| `17_13_13 (2).png` | Räume, sechs Karten, Filter und erklärende rechte Spalte | Verwaltungsraster im Control Center |
| `17_13_13 (3).png` | Raum Entwicklung mit eingebetteten Appansichten und Aufgaben | Kontextidee; keine IDE-/Task-App in der Shell nachbauen |
| `17_13_13 (4).png` | Horizontaler Raumwechsel mit großer ausgewählter Karte | Auswahlzustand und Tastaturnavigation, kleinere Alltagsfassung |
| `17_13_54 (1).png` | Desktop, obere Raumleiste, kleines Deck rechts | Wichtigste Referenz für normalen Desktop und Deck |
| `17_13_54 (2).png` | Hub-Overlay mit Räumen, letzten Dateien, Aufgaben, Systemzustand | Raumkarten/Overlay; zusätzliche Bereiche erst mit realer Datenquelle |
| `17_13_54 (3).png` | Control Center, Räume verwalten | Informationsdichte, Navigation und Filter |
| `17_13_54 (4).png` | Raum konfigurieren, Tabs, Formular, Restore, Vorschau | Formularhierarchie und Einstellungen; nur umgesetzte Fähigkeiten aktiv |

Alle gelieferten Ansichten sind dunkel und tragen noch NIWOE-Schriftzüge und
Kompassmarken. Sie sind unveränderte Referenzbilder, keine fertigen NIWOE-Assets.
Es gibt keine helle Ansicht, keine ausgearbeitete Spotlight-Suche und keine
vollständige Wizard-Sequenz. Animation kann aus statischen Bildern nicht abgeleitet
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

## 3. Was bewusst präzisiert oder reduziert wird

Die Mockups sind als Gestaltung stark, aber als tägliches Werkzeug teilweise zu
dekorativ: hohe Landschaftsbanner, wiederholte Leitsprüche und große Markenflächen
beanspruchen viel Platz. Diese Motive passen am besten zu Welcome/Login. Im Hub
sind Räume die erste Information, in Settings das bearbeitete Formular.

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
Kompass als Standardicon. Launcher bekommt ein neutrales funktionales Symbol.
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
| spacing / typography / radius / elevation | gemeinsame Geometrie beider Themes |

### Startwerte für P02

Die folgende Tabelle bewahrt die Entwurfsbasis. Verbindliche implementierte
P02-Werte samt Kontrastbegründung: [P02-Komponenten](design/P02_COMPONENTS.md).
Insbesondere Light-Sekundärtext und Statusfarben wurden nach Messung verfeinert.

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
- Dark/Light: identische Abmessungen, Schatten, Blur, Alpha und Interaktion.
- Alpha startet ohne neue Animation. Vorhandene Effekte nur mit belegter
  Cache-/Invalidierungsstrategie; neue Bewegung benötigt später ein Motion-Spec.

## 5. Spezifikation der Oberflächen

### Desktop und Panel

Desktop ist die Arbeitsfläche der Apps. Keine Dashboardkarten als ständige Ebene.
Panel oben: Launcher, Räume, flexible Lücke, verfügbare Module, Uhr. Aktiver Raum
Gold, nicht aktive Räume neutral. Bei Platzmangel Overflow statt kollidierender
Texte. Optionen ändern Module, nicht die grundlegende Designsprache.

Tiled Apps ohne zusätzliche dekorative NIWOE-Titlebar. Native CSD bleibt erhalten.
Apps, die SSD benötigen, behalten einen funktionalen Fallback mit erreichbarem
Schließen/Verschieben/Resize; keine pauschale Entfernung von Controls ohne
Bedienalternative. Floating hat eine feine Kontur und optional bestehenden
gecachten Schatten. Alpha führt keinen neuen Effekt allein für die Mockupnähe ein.

### Launcher

Zentriertes Popup auf dem fokussierten Output, Entwurfsbreite 640 logische Pixel,
an verfügbare Fläche angepasst. Suchfeld oben, Ergebnisse darunter, keine
Startmenü-Kategorienwand. Name und kurzer Typ/Untertitel genügen. Aktives Ergebnis
mit Form/Kontur und zurückhaltendem Akzent. Keine Landschaft und kein Leitspruch.

### Deck

Rechts oben unter dem Panel, Entwurfsbreite 360 logische Pixel. Audio zuerst,
darunter wenige verfügbare Systemschalter, Raumhinweis nach P06, Link zu Settings.
Nicht verfügbare Funktionen verständlich kennzeichnen oder ausblenden; keine
grünen Beispieldaten. Keine automatisch wandernden Controls in der Alpha.

### Hub

Overlay mit kompakter Überschrift, Raumfilter und Raster. Karten 260–340 logische
Pixel breit, je nach Output 1–4 Spalten, vertikal scrollbar. Größerer Inhalt nutzt
keinen Zwang zum horizontalen Karussell. Ausgewählte Karte klar markiert.
App-Icons, Fensteranzahl und optionale statische Vorschau gehören in die Karte.
Ein kleiner Link führt zur Verwaltung; Formularbearbeitung bleibt in Settings.

Kein Aufgabenmanager, keine erfundene Aktivitätschronik, keine Systemtelemetrie
als Hub-Pflicht. Dateien erscheinen später nur, wenn sie explizit einem Raum
zugeordnet wurden und der Zugriff definiert ist.

### Settings und Wizard

Settings darf dichter sein. Navigation links, klarer Seitentitel, bearbeitbare
Felder, echte Vorschau rechts nur bei ausreichender Breite. Auf kleinen Flächen
wandert die Vorschau unter das Formular oder entfällt; Bedienelemente bleiben
erreichbar. Speichern/Abbrechen und Feldfehler sind eindeutig.

Wizard erklärt eine Entscheidung pro Schritt, zeigt dieselben echten Widgets
und besitzt gute Defaults. Große Bild-/Serifakzente sind hier vertretbar.

## 6. Visuelle Abnahme

P02 erstellt eine kleine Vergleichsgalerie aus realer nativer Ausgabe. Jede weitere
UI-Phase ergänzt ihre Oberfläche in beiden Themes. Originalmockup und Screenshot
werden nach Farbrollen, Hierarchie, Abständen, Dichte, Fokus und Bedienbarkeit
verglichen; ein Pixelgleichheitstest gegen generierte App-Inhalte wäre ungeeignet.

Pflichtfälle: leer, viele Einträge, lange deutsche Namen, Fehler, Disabled, Keyboard-
Fokus, 1366×768, 1920×1080, unterschiedliche Skalierung und Wallpaperhelligkeit.
Screenshots dürfen nicht allein die funktionale oder Performance-Abnahme ersetzen.
