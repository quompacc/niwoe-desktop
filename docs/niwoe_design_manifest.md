# NIWOE Designmanifest

> Dieses Manifest ist die verbindliche Design-Spezifikation. Der
> [NIWOE Design-Brief](NIWOE_DESIGN_BRIEF.md) ist seine Mockup-Präzisierung;
> dessen Rollen und Einschränkungen sind verbindlich. Die in P02 geprüften
> Werte stehen in [Komponenten und Messungen](design/P02_COMPONENTS.md);
> editierbare Quelle bleiben ausschließlich Rust-Tokens und Config.

## Produkt und Quelle

**Nutzerkorrektur 24.09.2026:** Die letzten vier Mockup-PNGs unter `assets/`
(`17_13_54 (1)` bis `(4)`) sind verbindliche visuelle Vorgaben. Die ersten vier
(`17_13_13 (1)` bis `(4)`) sind redundante Kontextbilder ohne eigenes
Liefergate. Damit ist die frühere pauschale Bindung aller acht Bilder präzisiert.
Die vier verbindlichen Bilder sind keine frei interpretierbare Inspiration.
Abweichungen sind nur bei konkreter technischer Notwendigkeit zulässig und
müssen mit Bild, betroffener Stelle und Begründung dokumentiert werden.
Explizite Nutzerkorrekturen bleiben vorrangig: NIWOE statt MERIDIAN, kein
Alltagsbranding, keine SSD-Titelleiste und keine angehefteten Programmsymbole im
Panel. Die native Implementierung verwendet weiterhin zentrale Tokens/Config.
**Nutzerkorrektur 24.09.2026:** Die Panel-Schaltfläche öffnet den vollständigen
Hub aus `17_13_54 (2)`. Eine separate Spotlight-Launcher-Oberfläche ist kein
Produktpfad. Vom Hub führt der sichtbare Weg in das Control Center
„Räume verwalten“ aus `(3)` und von dort in „Raum konfigurieren“ aus `(4)`.
Die Sidebar gehört vollständig zu beiden Control-Center-Seiten; der Hub selbst
folgt seiner großen zentrierten Overlay-Komposition aus `(2)`.

**Nutzerpräzisierung 24.09.2026:** Der Hub ist zugleich die Willkommensansicht
einer NIWOE-Sitzung. Er öffnet genau einmal bei jeder neuen Login-Sitzung. Ein
Shell- oder Watchdog-Neustart in derselben Sitzung darf ihn nicht erneut öffnen.
Nach dem Schließen öffnet `Super+Space` den Hub; direktes Tippen darin wechselt
in die Suche. Das erste `Esc` kehrt von der Suche zum Hub zurück, das nächste
schließt ihn.

**Nutzerkorrektur 25.09.2026:** Die neue Login-Sitzung beginnt in einer
neutralen Loge ohne aktiven Raum. Der Willkommens-Hub zeigt die Raumwahl; erst
eine Auswahl aktiviert einen Raum. Die Loge ist kein zusätzlicher Raum und
erhält keine eigenen Fenster. Schließen des Hubs lässt den neutralen Desktop
sichtbar; `Super+Space` öffnet ihn erneut. Ein direkter App-Schnellstart aus der
Loge aktiviert Raum 1 als Kontext für das neue Fenster.

**Nutzerpräzisierung 24.09.2026:** Die Bildtreue betrifft den gesamten Ablauf,
besonders Hub, „Räume verwalten“ und „Raum konfigurieren“. Diese Seiten werden
mit ihrer sichtbaren Informationsarchitektur, Karten-/Spaltenstruktur,
Navigation und Formularhierarchie umgesetzt. Frühere Vorschläge für reduzierte
Hub- oder Settings-Fassungen sind keine Freigabe zum Weglassen. Der
[Mockup- und Workflowplan](MOCKUP_WORKFLOW_PLAN.md) ordnet jedem verbindlichen
Bild ein Lieferobjekt und jedem Phasenschritt ein visuelles Gate zu. Fehlende Backend-
Fähigkeiten werden in der vorgesehenen Struktur ehrlich als nicht verfügbar
gekennzeichnet; sie rechtfertigen keine andere Seitenarchitektur.

NIWOE ist eine native Rust-Desktopoberfläche. Es gibt genau eine Designquelle:
`niwoe-tokens` (`Palette`, `Interaction`, `Elevation`, `Radius`) und
`niwoe-config` (`Decorations`). Jede Farbe,
Alpha, Geometrie, Radius- und Effektentscheidung kommt ausschließlich daraus.
Lokale Render-Hardcodes sind verboten, außer bei Assets/Tests mit begründeter
`guard:allow`-Ausnahme.

**Nutzerentscheidung 25.09.2026:** Die Desktop-Alpha verwendet genau ein
verbindliches dunkelgrünes NIWOE-Theme. Seine grüne Farbwelt prägt Desktop,
Panel, Hub, Control Center, Deck und Onboarding. Eine Theme-Auswahl gehört nicht
zum Alpha-Workflow. Die vorhandene Light-Palette darf als ungenutzter technischer
Bestand erhalten bleiben; sie ist kein zweites aktives Produkt-Theme und kein
Abnahmeziel. Eine spätere helle Variante erfordert eine neue Produktentscheidung
und eigene visuelle Abnahme. Zentrale Tokens/Config, Design Guard,
Kontrastgrenzen und der Zentralitätsplan bleiben verbindlich.

## Visuelle Richtung

**Historischer Light-Stand 23.09.2026:** Gedämpftes Stein- und Salbeigrau
ersetzten damals die nahezu weißen Flächen. Diese Palette bleibt als bestehender
Code dokumentiert und ist seit der Nutzerentscheidung vom 25.09.2026 kein
Alpha-Lieferziel.

Die Oberfläche ist ruhig, präzise und materialorientiert: tiefe entsättigte
Grünflächen, warmes Off-White und zurückhaltendes Messing/Gold. Gold markiert
nur aktuelle Auswahl, Fokus und die wesentliche Aktion; inaktive Icons und
Konturen sind neutral. Statusinformationen erhalten eigene semantische Farben
und Text/Icon, nie allein Farbe.

Feine Konturen, klare Flächen, ruhige Überschriften und kompakte sachliche
Bedienung bilden eine Familie. Die zentrale Abstandsleiter startet mit
4/8/12/16/24/32 logischen Pixeln; Panel (48), Controls (mindestens 32),
Formularfelder (36) sowie Radien (4/8/12) sind in P02 als Tokens definiert.
Sans-Schriftgrößen sind 12/14/18/28. Serif bleibt großen Überschriften
vorbehalten und ist keine Voraussetzung für ein vollständig lesbares UI.
Textkontrast beträgt mindestens 4,5:1 für normalen Text und
3:1 für große Schrift/wesentliche Grenzen auf der zusammengesetzten Fläche.
Dekorative Konturen und notwendige Controlgrenzen sind getrennte Rollen.
Fokus hat einen zusätzlichen 2-Pixel-Rahmen mit 4-Pixel-Innenabstand; auf
gefülltem Akzent verwendet er die kontrastierende Vordergrundfarbe.

## Oberflächen und Branding

**Materialfreigabe 23.09.2026:** Der Nutzer bestätigt Farbe, Transparenz und
Glaswirkung von Systemdeck und Lautstärke ausdrücklich als Referenz.
Panel und Hub übernehmen genau diesen compositorseitigen Materialpfad,
ohne zusätzliche Shell-Tönung oder eigene Opazitätsfaktoren. Die freigegebenen
beiden Popups werden dafür nicht verändert. Panelgeometrie und mittige Uhr bleiben.

Der Arbeitsdesktop bleibt frei von Dashboardkarten. Das Panel liegt oben:
Hub-Zugang und Räume links, Statusmodule rechts. Nach Nutzerkorrektur vom
23.09.2026 steht die Uhr exakt in der Bildschirmmitte, unabhängig von Raum-
und Statusgruppenbreite. Der Hub ist das große zentrierte Overlay aus dem
verbindlichen Bild; das Deck bleibt eine kleine Karte rechts oben. Hub, Settings
und Wizard bleiben tastaturfest und datenwahr und erfinden keine Datenquellen.

NIWOE-Wortmarke ist nur in Welcome, Login und About erlaubt. Der alte
NIWOE-Kompass wird nicht weiterverwendet. Taskbar/Panel und Hub-Button
verwenden neutrale funktionale Symbole. Wallpaper und illustrative Mockups sind
keine pro Frame berechneten Effekte und müssen nicht vorhanden sein, damit die
UI lesbar bleibt.

Fenster erhalten keine compositor-eigene Titelleiste. Die SSD-Geometrie liefert
nur eine 1–2 logische Pixel dünne Außenkontur; Farben und Breite kommen aus
Tokens/Decorations. App-eigene CSD bleibt erhalten. Schließen, Verschieben und
Resize bleiben über Tastatur und Modifier-Mausbedienung erreichbar.

## Performance und Abnahme

Statische Flächen sind eventgetrieben. Icons, Fonts, Schatten und Assets werden
nach Identität, Theme und Scale gecacht; jede neue visuelle Funktion beschreibt
Schlüssel, Invalidierung, Obergrenze und Lebensdauer. Blur bleibt beim
Compositor. Neue Animationen oder Effekte benötigen eine spätere Spezifikation
und Hardwarebelege.

Native Ausgaben werden im verbindlichen grünen Theme für leere, volle, lange,
fehlerhafte und fokussierte Zustände bei 1366×768 und 1920×1080 sowie
verfügbaren Skalierungen geprüft. Die Mockups bestimmen Hierarchie, Tonalität,
Material, Konturen, Ikonografie und Abstände. Native Screenshots werden direkt
daneben geprüft;
freie Vereinfachung ist kein technischer Grund für eine Abweichung.
