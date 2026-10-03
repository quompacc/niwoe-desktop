# Native NIWOE-Screenshots

Diese fünf Aufnahmen zeigen den tatsächlich laufenden Entwicklungsstand vom
**03.10.2026** auf dem Fedora-Testrechner: 1920 × 1080 Pixel, 100 Prozent
Skalierung, dunkelgrünes NIWOE-Theme. Sie sind unveränderte PNG-Aufnahmen
über den bestehenden Screenshotpfad mit sichtbarer Zustimmung.

| Datei | Ansicht |
| --- | --- |
| [hub.png](hub.png) | Hub und Panel über dem Desktop-Wallpaper |
| [rooms.png](rooms.png) | Raumverwaltung mit einer echten Fenstervorschau |
| [room-configuration.png](room-configuration.png) | Raumdetails mit einer echten Fenstervorschau |
| [appearance.png](appearance.png) | Hintergrundeinstellungen im gemeinsamen Control Center |
| [panel-configuration.png](panel-configuration.png) | Leistenmodule und Vorschau in Originalgröße |

Für die beiden Raumansichten war ein echtes KWrite-Fenster mit einem eigenen
Beispieldokument geöffnet. Die Vorschauen entstehen durch den normalen
NIWOE-Capturepfad. Danach wurde das Fenster geschlossen. Bestehende Raum-,
Config-, Panel-, First-Run- und MIME-Dateien blieben bytegleich; Outputzustand
und aktiver Raum wurden erhalten. Die temporäre Eingabegerätefreigabe wurde
nach der Aufnahmerunde entfernt; die ursprüngliche ACL ist exakt bestätigt.

Alle ausgewählten Bilder wurden vor der Aufnahme ins Repository angesehen.
Die Hashes und tatsächlich laufenden Buildidentitäten stehen in
[captures.json](captures.json). Die Aufnahmen stammen von Shell `98864dc2…`
und Compositor `c09985e3…`; der zusätzlich installierte R9-Compositor war
zu diesem Zeitpunkt noch nicht durch eine neue Anmeldung aktiviert.

Die Galerie dokumentiert eine Auswahl des aktuellen Designs. Sie ist keine
Gesamtfreigabe oder Behauptung einer fertigen Desktop-Alpha. Die weitere
Entwicklung richtet sich nach dem aktiven
[Umsetzungsplan](../../NIWOE_IMPLEMENTATION_PLAN.md).
