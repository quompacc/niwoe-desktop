# NIWOE: Bedienkonzept nach Omarchy

Verbindliche Nutzerentscheidung vom 22.09.2026. Bedienreferenz ist das
[offizielle Omarchy-Handbuch](https://learn.omacom.io/2/the-omarchy-manual/53/hotkeys),
abgerufen am 22.09.2026. NIWOE behält seine native Architektur und Gestaltung.
Diese Spezifikation ist ein Zielbild, kein Implementierungsnachweis.

## Grundprinzip

Tiling ist der Normalfall. Dialoge und ausdrücklich freigestellte Fenster dürfen
floaten. Keine zusätzliche SSD-Titelleiste, nur eine dünne Fokus-/Außenkontur.
App-eigene CSD bleibt erhalten. Maus und Tastatur lösen dieselben Aktionen aus.
Jede Shell-Oberfläche unterstützt Pfeile/Tab, Enter und Escape; Öffnen setzt den
Fokus sinnvoll, Schließen gibt ihn ans vorherige App-Fenster zurück.

## Verbindliche Kernbelegung

| Kürzel | NIWOE-Aktion |
|---|---|
| Super+Space | Hub öffnen; Tippen darin startet die Suche |
| Super+Alt+Space | NIWOE-Steuerung / Einstellungen |
| Super+Escape | System-Deck mit Sitzungsaktionen |
| Super+K | Durchsuchbare Übersicht der tatsächlich verfügbaren Kürzel |
| Super+Return | Standardterminal |
| Super+Shift+Return | Standardbrowser |
| Super+Shift+F | Standarddateimanager |
| Super+W | Fenster schließen |
| Super+T | Einzelnes Fenster zwischen Tiling und Floating wechseln |
| Super+F | Vollbild umschalten |
| Super+Pfeil | Fokus in die entsprechende Richtung |
| Super+Shift+Pfeil | Fenster in die entsprechende Richtung tauschen |
| Super+1…9 | Raum wechseln |
| Super+Shift+1…9 | Fenster in den Raum verschieben und folgen |
| Super+Shift+Alt+1…9 | Fenster verschieben, im aktuellen Raum bleiben |
| Super+Tab / Super+Shift+Tab | Nächster / vorheriger Raum |
| Alt+Tab / Alt+Shift+Tab | Nächstes / vorheriges Fenster im Raum |
| Super+linke Maustaste ziehen | Fenster verschieben |
| Super+rechte Maustaste ziehen | Fenstergröße ändern |
| Super+Mausrad | Räume durchlaufen |
| Super+Ctrl+L | Sitzung sperren |

Omarchy-Zusatzfunktionen wie Scratchpad, Gruppen, alternative Layouts und
universelle Zwischenablage bleiben Teil der Bedienreferenz. Ihre Kürzel dürfen
nicht anders belegt werden; ohne implementierte Funktion sind sie nicht aktiv.
Keine Übernahme von Omarchys vorinstallierten Apps, Webdiensten oder Branding.
Die bestehende Neuner-Raumgrenze bleibt bis zur echten Raummigration sichtbar.

## Konflikte mit dem bisherigen NIWOE-Stand

- Super+Space öffnet den Hub und wird nicht wie im früheren Konzept zum Deck.
  Eine separate Launcher-Oberfläche ist durch Nutzerkorrektur vom 24.09.2026
  entfallen; Tippen im Hub startet die Suche.
- Super+W ersetzt Super+Q für Schließen.
- Super+Ctrl+L ersetzt Super+L für Sperren.
- Super+T muss künftig ein einzelnes Fenster freistellen statt den ganzen Raum.
- Super+Pfeile müssen Fokus bewegen statt Splitgrößen ändern.
- Alte belegte Nutzer-Konfigurationen brauchen eine explizite Migration; nicht
  still behaupten, dass neue Defaults bereits in jeder laufenden Sitzung gelten.

## Mauswege

Panel: Hub-Schaltfläche öffnet den Hub, die Lupe öffnet dessen Suche, Raumwahl
wechselt Räume, Statusgruppe öffnet Deck, Uhr öffnet Kalender. Das Deck verlinkt
Einstellungen.
Hub und Steuerung sind durchsuchbar; keine Kategorienwand. Der Hub öffnet einmal
pro neuer NIWOE-Login-Sitzung automatisch als Willkommensansicht; Shell- oder
Watchdog-Neustarts innerhalb derselben Sitzung öffnen ihn nicht erneut. Die
Sitzung beginnt in der neutralen Loge ohne aktiven Raum. Raumwahl beendet
diese Loge; ein App-Schnellstart aktiviert Raum 1 als Fensterkontext. Im Deck
stehen Audio und echte verfügbare Systemfunktionen im Vordergrund. Keine funktionslosen
Bedienelemente oder erfundenen Zustände. Eine gemeinsame Aktionsquelle liefert
Kürzelhilfe und sichtbare Hinweise, damit Beschriftung und Wirkung übereinstimmen.
