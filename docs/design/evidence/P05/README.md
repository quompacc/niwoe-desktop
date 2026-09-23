# Deck-Rasterbelege, 23.09.2026

## Fortsetzung: Symbolaktionen

| Beleg | Herkunft |
|---|---|
| [Symbole Dark](symbols-dark.png) | Native Rasterausgabe mit kontrollierten Daten. |
| [Symbole Light](symbols-light.png) | Gleiche Daten/Geometrie, helle zentrale Palette. |
| [Symbole live](symbols-live.png) | Echter DRM-Screenshot, Fedora/Acer, 1920×1080, 100 %, Dark. Audio 40 %, Bluetooth ein, Leistungsprofil-Dienst fehlt. |

Details und Grenzen: [Symbolaktionen](../../../phase-reports/P05_DECK_SYMBOLS.md).
Das blaue Hintergrundbild und das sichtbare Glasmaterial entsprechen noch nicht
vollständig dem Desktopmockup; der Screenshot ist kein Gesamt-Abnahmebeleg.

## Einschränkung der bisherigen DRM-Aufnahmen

Der Capturepfad lief vor der Glasauflösung. Deshalb fehlen compositorseitige
Glasflächen in `symbols-live.png` und den folgenden Aufnahmen; diese Bilder
belegen keine vollständige Materialdarstellung am Bildschirm:

- [Einstellungen nach Input-Fix](settings-after-input-fix.png): genehmigte
  Aufnahme nach erfolgreichem Mausklick auf Erlauben; Nutzer bestätigt den Klick.
- [Deck vor Capture-Fix](deck-before-capture-fix.png): neuer Input-Build,
  weiterhin unvollständiger Capturepfad.

Ursache/Korrektur: [Input- und Materialbefund](../../../phase-reports/P05_INPUT_MATERIAL_FINDINGS.md).

## Vorheriger Kompositionsschritt

Aus dem echten nativen Deck-Renderer auf Fedora, 360×400 logische Pixel,
Skalierung 100 %. Transparente Vordergrundebene für diese Vorschauen auf die
jeweilige Theme-Tönung komponiert. Kontrollierte Testdaten, **keine Live-Screenshots**.

| Beleg | Inhalt |
|---|---|
| [Dark](deck-dark.png) | Audio 70 %, langer Geräte-/WLAN-Name, Leistungsprofil Standard. |
| [Light](deck-light.png) | Identische Daten und Geometrie mit heller zentraler Palette. |
| [Nicht verfügbar](deck-unavailable.png) | Audio, Netzwerk und Leistungsprofil fehlen; keine aktiven Eingabeziele dafür. |

Erzeugt durch die Tests in `crates/niwoe-shell/src/deck_tests.rs` mit
`NIWOE_DECK_POPULATED_PREVIEW` bzw. `NIWOE_DECK_PREVIEW`.
Herkunft und offene Mockup-Unterschiede:
[Korrekturbericht](../../../phase-reports/P05_DECK_COMPOSITION.md).
