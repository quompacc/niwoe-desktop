# NIWOE Designmanifest

> Dieses Manifest ist die verbindliche Design-Spezifikation. Der
> [NIWOE Design-Brief](NIWOE_DESIGN_BRIEF.md) ist seine Mockup-Präzisierung;
> dessen Rollen und Einschränkungen sind verbindlich, seine P02-Startwerte sind
> noch keine Renderer-Hardcodes.

## Produkt und Quelle

NIWOE ist eine native Rust-Desktopoberfläche. Es gibt genau eine Designquelle:
`niwoe-tokens` (`Palette`, `Interaction`, `Elevation`, `Radius`) und
`niwoe-config` (`Decorations`). Jede Farbe,
Alpha, Geometrie, Radius- und Effektentscheidung kommt ausschließlich daraus.
Lokale Render-Hardcodes sind verboten, außer bei Assets/Tests mit begründeter
`guard:allow`-Ausnahme.

Es gibt genau zwei Themes, Dark und Light. Sie unterscheiden sich ausschließlich
in den zentralen Farbtabellen; Layout, Abstände, Radien, Glas, Blur, Schatten
und Interaktionsgeometrie sind identisch. Der Design Guard und der
Zentralitätsplan bleiben verbindlich.

## Visuelle Richtung

Die Oberfläche ist ruhig, präzise und materialorientiert: tiefe entsättigte
Grünflächen, warmes Off-White und zurückhaltendes Messing/Gold. Gold markiert
nur aktuelle Auswahl, Fokus und die wesentliche Aktion; inaktive Icons und
Konturen sind neutral. Statusinformationen erhalten eigene semantische Farben
und Text/Icon, nie allein Farbe.

Feine Konturen, klare Flächen, ruhige Überschriften und kompakte sachliche
Bedienung bilden eine Familie. Die zentrale Abstandsleiter startet mit
4/8/12/16/24/32 logischen Pixeln; Panel (48), Controls (mindestens 32),
Formularfelder (36) sowie Radien (4/8/12) werden erst in P02 als Tokens
festgeschrieben. Textkontrast beträgt mindestens 4,5:1 für normalen Text und
3:1 für große Schrift/wesentliche Grenzen auf der zusammengesetzten Fläche.

## Oberflächen und Branding

Der Arbeitsdesktop bleibt frei von Dashboardkarten. Das Panel liegt oben:
Launcher, Räume, flexible Lücke, vorhandene Module und Uhr. Der Launcher ist
ein kompaktes zentriertes Such-Popup, das Deck eine kleine Karte rechts oben;
beide bleiben tastaturfest und datenwahr. Hub, Settings und Wizard folgen den
im Brief definierten Informationsgrenzen und erfinden keine Datenquellen.

NIWOE-Wortmarke ist nur in Welcome, Login und About erlaubt. Der alte
NIWOE-Kompass wird nicht weiterverwendet. Taskbar/Panel und Launcher-Button
verwenden neutrale funktionale Symbole. Wallpaper und illustrative Mockups sind
keine pro Frame berechneten Effekte und müssen nicht vorhanden sein, damit die
UI lesbar bleibt.

## Performance und Abnahme

Statische Flächen sind eventgetrieben. Icons, Fonts, Schatten und Assets werden
nach Identität, Theme und Scale gecacht; jede neue visuelle Funktion beschreibt
Schlüssel, Invalidierung, Obergrenze und Lebensdauer. Blur bleibt beim
Compositor. Neue Animationen oder Effekte benötigen eine spätere Spezifikation
und Hardwarebelege.

Native Ausgaben werden in beiden Themes für leere, volle, lange, fehlerhafte und
fokussierte Zustände bei 1366×768 und 1920×1080 sowie verfügbaren Skalierungen
geprüft. Die Mockups liefern Hierarchie und Tonalität, keinen Pixelgleichheitstest.
