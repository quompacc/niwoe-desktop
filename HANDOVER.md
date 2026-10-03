# NIWOE: geparkter Stand / Übergabe

Stand **03.10.2026**. Auf Nutzerwunsch geparkt. Produktcode-Basis:
[`dc5b65d`](https://github.com/quompacc/niwoe-desktop/commit/dc5b65dfe04cc58726f53da88f00746a9f4f8e9b),
Branch `codex/niwoe-p00`, GitHub `quompacc/niwoe-desktop`.
Der konfigurierte Remote `origin` ist ein historischer Codeberg-Remote;
für diesen Stand wurde `github` verwendet.

## Abgenommen und nachgewiesen

- Der Nutzer hat die **erste ruhigere Designrunde** positiv abgenommen:
  neutrale Auswahl/Fokus statt gelber Aktivitätsstriche, größere Controls,
  mehr Abstand und vereinfachte Navigation. Alle globalen Einstellungen im
  Control Center; Dateien/Wiederherstellung beim jeweiligen Raum.
- Dieser Stand ist als Release auf Fedora installiert und aktiv. Workspacecheck,
  Format, striktes Clippy und Guards grün; **1311 Tests bestanden**, kein Fehler,
  ein bestehender isolierter D-Bus-Test ignoriert. 180 geprüfte Quellhashes identisch.
- **87 finale native Prüfbilder** tatsächlich angesehen: 19 Einstiegs-/Fokusfälle,
  60 FHD-/UHD-Matrixfälle bei 100/150/200 Prozent und acht ergänzende Scrollbilder.
  Das belegt den beschriebenen Layout-/Navigationsumfang, keine komplette Alpha.
- Primärmonitorwechsel hin/zurück mit vorhandenen Layern funktioniert im belegten
  V32-Lauf. Echte neue Sitzung, neutrale Loge und Watchdog ohne erneute Begrüßung
  nachgewiesen. Das ist keine physische Hotplug-Abnahme.
- Geschützte NIWOE-/KDE-/GTK-/MIME-Dateien wurden wiederhergestellt;
  temporäre Eingabegerätefreigabe entfernt. Original-ACL beim Parken erneut geprüft.

Details: [aktueller Design-Prüfbericht](docs/phase-reports/evidence/r9/calm-controls/README.md)
und [V32-/Sitzungsbelege](docs/phase-reports/evidence/r9/primary-after/README.md).
Frühere P12-Abnahme ist historische Evidenz; die visuelle Gesamtfreigabe wurde
am 02.10. zurückgenommen. **R9 ist offen, P13 unbegonnen, keine fertige Desktop-Alpha.**

## Bekannt fehlerhaft oder begrenzt

| Befund | Stand / nächster Nachweis |
| --- | --- |
| **V23: weiche Schrift/Icons bei hoher Skalierung** | Auf aktuellem Release sichtbar. Shell zeichnet logisch und skaliert/einpasst Raster; native physische Glyphenauflösung und Eingabegeometrie gemeinsam prüfen. |
| Lange Erläuterungen bei kleinen logischen Ansichten | Im unteren Raumformular teilweise gekürzt. Vollständige Text-/Dichteabnahme offen; Erreichbarkeit per Scrollen belegt. |
| **V02: heller Appinhalt scheint störend durch den Hub** | In älteren realen Aufnahmen reproduziert; aktuelle Kontrast-/Materialnachprüfung offen. Nicht als behoben führen. |
| Fedora-Paketupdates | Backendintegration fehlt; UI benennt die Einschränkung. Anzeige ist kein funktionierender RPM-Updatepfad. |
| Provider-/Fokusfälle | WLAN-Lesefehler ehrlich sichtbar. Screenshotzustimmung über GTK-Dateipicker und Rückwärtstab aus unbekanntem Settings-Fokus erneut reproduzieren; kein aktueller Fixnachweis. |
| Speicher über Bedienzyklen | Frühere R7/R8-Serien: Shell +2,125 MiB zwischen Zyklus 20 und 40; R8-Compositor +7,03125 MiB. Ursache/unbegrenztes Wachstum nicht bewiesen; lange Prüfung des aktuellen Releases fehlt. |

## Ausdrücklich offen

- **Gaming und Vollbild unter Spielelast:** native Wayland-/XWayland-Spiele,
  Fokus/Rückkehr zum Desktop, Monitor-/Moduswechsel, Grafik- und Eingabelatenz
  im aktuellen Stand nicht freigegeben. Historische Maximize/Fullscreen-Smokes
  ersetzen keine Spieleprüfung. NVIDIA-Offload/proprietärer Treiber nicht geprüft.
- **Dauerbetrieb:** mehrstündige Sitzungen, lange Warm-up-/Öffnungsserien,
  Cache-/RSS-Verlauf und drei aktuelle volle Ruheproben fehlen. Alte Messungen
  gelten für ihre damaligen Binärstände, nicht automatisch für `e9bed113…`.
- Übrige R9-Matrix: Nebenflächen/OSD/Picker/Benachrichtigungen, lange Daten und
  fehlende Assets/Provider, echte Geräteaktionen/Hotplug, Schutzflächen mit
  erhaltener Aufnahmesperre und direkter echter Appstart aus der Loge nach Raum 1.

## Wiedereinstieg

1. [Testumgebung](docs/TEST_ENVIRONMENT_2026-10-03.md) mit neuer lesender Inventur
   vergleichen: Gerät, tatsächlich benutzte GPU, Kernel/Treiber/Mesa und Toolchain.
2. [Codekarte und Guards](docs/CODE_GUIDE.md) lesen. Guards beim Modell-/Agentwechsel
   beibehalten; Assertions, Scanbereiche und 600-Zeilen-Grenze nicht lockern.
3. Den [aktiven Plan](NIWOE_IMPLEMENTATION_PLAN.md) bei R9 und dessen
   [Vorbereitung/offener Matrix](docs/phase-reports/evidence/r9/PREPARATION.md)
   fortsetzen: zuerst V23 und Lebensdauer/Fokus reproduzieren, kleine geprüfte Schritte.
4. Aktuelle Linux-Gates, identische Release-/Installations-/Prozesshashes und reale
   Nachherbelege erfassen; erst danach Gesamtfreigabe/P13 erneut beurteilen.

Die README-Galerie zeigt einen **früheren Stand vom selben Tag**; Buildherkunft
steht bei den Bildern. Öffentliche aktuelle Evidenz enthält Logs, Helper,
Fallmetadaten und Bildhashes. Private PNGs/Rohdaten bleiben lokal; ein frischer
Clone enthält diese Archive nicht. Orte und Grenzen stehen in der Codekarte.
