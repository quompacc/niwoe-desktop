# P02: gemeinsame native Komponenten

Die editierbare Designquelle bleibt `niwoe-tokens` plus `niwoe-config`.
Die `Palette` behält ihre Config-Feldnamen; semantische Aliase heißen
`surface_base`, `surface_raised`, `surface_overlay`, `border_subtle`,
`border_control`, `border_focus`, `on_accent`, `text_disabled`.
Notwendige Controlgrenzen nutzen den neutralen Sekundärtext statt die absichtlich
dezente dekorative Kontur. Gold bleibt Auswahl, Fokus und Hauptaktion vorbehalten.

## Endgültige Farbrollen

| Feld/Rolle | Dark | Light |
|---|---|---|
| background / surface.base | #101710 | #D0D5CA |
| surface / surface.raised | #19221A | #DCE0D5 |
| surface_alt / surface.overlay | #202B22 | #C0CBBB |
| text / primary | #F1EEE3 | #202B23 |
| text_dim / secondary, disabled, control border | #BAC3B7 | #2D3A31 |
| border / subtle | #435044 | #869583 |
| accent / focus | #D6B35B | #5D4618 |
| accent_alt | #C4A149 | #503B14 |
| on_accent | #151B13 | #FFFFFF |
| success | #A0CFA9 | #15401F |
| warning | #EFB47D | #552C14 |
| error | #FFB7AD | #6A1F18 |

Der Light-Sekundärtext weicht bewusst vom Brief-Startwert ab. Die Messung über
Hover/Pressed und transparente Flächen erforderte dunkleren Text. Statusfarben
sind aus demselben Grund angepasst. `contrast.rs` misst sRGB-Luminanz nach
Alpha-Komposition, einschließlich schwarzem und weißem Hintergrund.

| Gemessenes Minimum | Dark | Light | Ziel |
|---|---:|---:|---:|
| Primärtext | 7,312 | 5,764 | 4,5 |
| Sekundär-/Disabled-Text | 4,683 | 4,684 | 4,5 |
| Erfolg | 4,851 | 4,621 | 4,5 |
| Warnung | 4,649 | 4,696 | 4,5 |
| Fehler | 5,108 | 4,527 | 4,5 |
| wesentliche Grenzen | 4,683 | 4,684 | 3,0 |
| Fokus | 4,230 | 3,503 | 3,0 |
| Text auf Akzent, inklusive Hover/Pressed | 6,798 | 6,730 | 4,5 |

Dekorative subtile Konturen haben ausdrücklich kein 3:1-Versprechen. Sie dürfen
keine notwendige Controlgrenze oder alleinige Fokuskennzeichnung ersetzen.
Messwerte gelten für die getesteten Standardtokens, nicht beliebige Benutzerfarben.

## Maße, Zustände und Schrift

Abstände: 4/8/12/16/24/32 logische Pixel; Controls mindestens 32, Eingabe 36,
Karte 72, Innenabstand 16. Radien 4/8/12; `xl` bleibt kompatibler Alias für 12.
Fokus: zusätzliche innere 2-Pixel-Kontur mit 4-Pixel-Abstand. Bei gefülltem
Akzentbutton ist sie in der kontrastierenden Vordergrundfarbe sichtbar.
Panelhöhe 48, bestehende Position und Struktur unverändert bis P03.

Sans-Hierarchie: Caption 12, Body 14, Abschnitt 18, Display 28. Die bestehenden
Fontconfig-/Embedded-Pfade bleiben bestehen; die aufgelöste Schrift wird beim
Laden auf den zentralen UI-Zeichensatz geprüft. Fehlende Pfeile oder andere
erforderliche Zeichen führen zum gemeinsamen Embedded-Sans-Fallback für beide
Textpfade. Das ist keine vollständige Unicode-Fallback-Kette. Serif ist optional für große
Überschriften; P02 führt keinen neuen Serif-Font ein und nutzt Sans auch dort.

`niwoe-ui::widget::Component` rendert Surface, Text, Button, Tab, Chip, Card,
Input und Slider. Fokus, Disabled, Error und Selected sind unabhängige Flags;
Hover/Pressed kommen aus dem bestehenden `WidgetState`. Disabled ignoriert
Pointerzustände und `accepts_input()` liefert false. Anwendungsaktionen bleiben
beim jeweiligen Dispatcher; der Baustein erfindet keine Backendbestätigung.
Statische Surface/Text nehmen keine Eingabe an. Fehler brauchen zusätzlich Text
oder Icon; die Galerie kennzeichnet Fehler mit `! Eingabe prüfen` bzw. Spaltentitel.

## Reproduktion und Grenzen

```sh
python3 scripts/gen-niwoe-theme.py --check
cargo test -p niwoe-tokens --test contrast -- --nocapture
cargo run --release -p niwoe-ui --example components -- target/p02-evidence/components
```

Die 24 PNGs bilden zwei logische Viewports (1366×768, 1920×1080), zwei Themes,
drei Rastermaßstäbe und zwei Hintergrundhelligkeiten ab. Bei 150/200 % wächst
die physische Bildgröße. Glyphen werden in der physischen Größe neu gerastert;
es wird kein kleines PNG hochskaliert. Das belegt Raster- und Themeparität,
ersetzt aber keinen Test von Platzmangel auf einem festen physischen Output.
Echte Sitzungs-/Outputfälle und Hardwaregrenzen stehen im Phasenbericht.

Visuell geprüfte native Offscreen-Ausgaben (keine Desktop-Captures):

- [Dark, 1366×768, 100 %, heller Hintergrund](evidence/P02/dark-1366x768-100-light.png)
- [Light, 1366×768, 100 %, dunkler Hintergrund](evidence/P02/light-1366x768-100-dark.png)
- [Dark, 1920×1080 logisch, 150 %](evidence/P02/dark-1920x1080-150-light.png)
- [Light, 1920×1080 logisch, 200 %](evidence/P02/light-1920x1080-200-dark.png)

## Performance und Invalidierung

Keine neuen Threads, Polling- oder Animationstimer. Die Settings-Suche wird wie
bisher bei Query-/Pointer-/Theme-/Scale-Änderung neu gezeichnet. Pfade und gekürzte
Beschriftungen sind auf ein Control begrenzt und leben nur für den Paint-Aufruf.
Wie bei den bestehenden Tile-/Button-Pfaden sind dies temporäre Allokationen;
der allgemeine Anspruch eines vollständig allokationsfreien `Widget::paint`
ist damit weiterhin nicht erreicht. Es entsteht kein neuer persistenter Cache.
Die Galerie ist ein einmaliger Offline-Prozess. Sie hält jeweils ein Ausgabebild
und nutzt denselben bestehenden Textpfad; es gibt keinen dauerhaften Galeriedienst.
Glyphencache-Schlüssel bleiben Zeichen und physische Schriftgröße innerhalb des
aktiven Fonts. Theme-/Fontwechsel verwenden die bestehenden Invalidierungspfade.
Diese Beschreibung ist kein Ersatz für die gemessenen Werte im Phasenbericht.
