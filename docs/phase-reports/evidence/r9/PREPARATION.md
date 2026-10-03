# R9: vorbereitete Nachprüfung

Diese Datei ergänzt ausschließlich das R9-Abschlussgate des aktiven
Umsetzungsplans. Sie ist keine neue Roadmap und kein bestandener Lauf.
Stand: 03.10.2026. Die folgenden vorbereiteten Schritte sind inzwischen
ausgeführt; ihre ursprüngliche Beschreibung bleibt als Ablauf erhalten.

**Aktueller Nachweis:** In der echten neuen Sitzung 1431 wurden beide nativen
Primärwechsel ohne Prozessneustart nachgewiesen. Alle neun Bilder aus Haupt-
und Ergänzungslauf tatsächlich angesehen; siehe
[primary-after](primary-after/README.md). Der Haupthelper endete nach
erfolgreicher Wiederherstellung mit einem eigenen Guard-Fehler (veraltete
Variable des eigenen Configstands); diese historische Exit-1-Evidenz bleibt
erhalten. Der ergänzende Sitzungshelper endete mit Exit 0. Beide Cleanupbelege
bestätigen den ursprünglichen Zustand einschließlich Loge und unveränderter
Once-pro-Login-Markierung. Die ursprüngliche Eingabegeräte-ACL wurde bestätigt.

**Anschließende Designkorrektur:** Die Nutzerkorrektur vom 03.10.2026
priorisiert neutrale Auswahlzustände, größere gemeinsame Controls und eine
einfachere Navigation mit allen globalen Einstellungen im Control Center.
Geprüft/installiert ist nun Shell
`e9bed1136f1dc3c014a498f90b03fd56e531fae687163a0279ab7c383a703451`;
der aktuelle Compositor bleibt `8f1e15e3…`. 1311 Workspacetests sowie Format,
Workspacecheck, Design-/Größenguards und striktes Clippy sind bestanden.
Historische Zwischenstände und neue native Bildläufe stehen getrennt unter
`calm-controls/`. Für deren Durchführung wurde die autorisierte temporäre
Eingabefreigabe erneut eingerichtet; die abschließende Bereinigung wird
separat belegt: `calm-controls/input-cleanup.json` bestätigt die Original-ACL.
19 native Einstiegs-/Tastaturbilder, 60 Monitor-/Scalebilder und acht
ergänzende Scrollbilder tatsächlich angesehen; alle finalen Cleanupfelder true.
Keine Prüfhelper laufen weiter. Die Monitor-/Scale-Geometrieprüfung ersetzt weder V23 noch
die lange Speicher-/Ruheprüfung dieses neuen Binärstandes.

## Unmittelbarer Lauf nach dem neuen Login

Der damalige vorbereitete Helper `target/r9-primary-after-review.py` prüft vor jeder Eingabe die neuen
laufenden Binärhashes, zwei vorhandene Outputs, einen leeren Desktop und
die neutrale Loge. Die erste Aufnahme erfolgt vor dem Schließen des Hubs.
Danach folgen native Primärwahl auf Monitor 2, Aufnahme beider Outputs,
Rückwahl auf Monitor 1 und erneut beide Outputs. Kein Prozessneustart darf
zwischen den beiden Primärwechseln liegen. Originaldateien werden mit
Vergleich gegen den jeweils eigenen Stand wiederhergestellt; der abschließende
Watchdog gehört nur zur Bereinigung. Alle Bilder müssen separat angesehen
werden. Ein Login oder Map-Wechsel ist durch die Policytests nicht belegt.
Die bestehende Once-pro-Login-Markierung wird ausschließlich gelesen; ihre
Zeitmarke muss den Watchdog unverändert überstehen. Danach werden neutraler
Desktop und regulärer `Super+Space`-Hub erneut aufgenommen. Es wird weder eine
Markierung entfernt noch eine neue Sitzung durch Umgebungswerte vorgetäuscht.
Die eigene temporäre `/dev/uinput`-Freigabe wurde während des ausstehenden
Logins nach einem Vergleich auf fremde ACL-Änderungen entfernt; ursprüngliche
ACL exakt bestätigt. Vor dem nächsten tatsächlichen Eingabelauf ist sie im
bereits autorisierten engen Umfang erneut temporär einzurichten und danach
wieder zu entfernen. Der damalige Wartezustand ist durch den oben beschriebenen Lauf abgelöst.

## Weitere aktuelle Pflichtfälle

| Bereich | Zu prüfender aktueller Zustand | Status |
| --- | --- | --- |
| Sitzung | Loge, einmaliger Willkommens-Hub, Watchdog ohne Wiederbegrüßung, direkter echter Appstart nach Raum 1 | neue Sitzung/Loge/Watchdog/regulärer Hub nachgewiesen; direkter Appstart weiterhin NOT RUN |
| V23 | Native Text-/Icon-Schärfe, logische Geometrie und Eingabe bei 100/150/200 Prozent auf beiden Outputs | offen; Quellpfad gelesen |
| Fokus | GTK-Dateipicker, Screenshotzustimmung darüber, anschließend Escape und normaler Rückweg | offen; aktueller Vorher-/Nachherlauf fehlt |
| Navigation | Rückwärtstab aus unbekanntem Settings-Fokus; keine unbeabsichtigte Wallpaperänderung | offen; aktueller reproduzierbarer Lauf fehlt |
| Nebenflächen | Audio-OSD, Regionsauswahl mit Ziehen/Bestätigen/Abbrechen, Fenster-/App-/Dateipicker, Benachrichtigung, WLAN-Dialog | vollständige aktuelle Matrix NOT RUN |
| Daten | Leere/belegte Räume, echte native/XWayland-Fenster, lange Namen, große gültige Raum-/Appkataloge, fehlende Assets/Provider | ergänzende R9-Fälle NOT RUN |
| Schutzflächen | Login, Lock/PAM, Polkit, Fehler/Abbruch; optische Prüfung bei erhaltener Aufnahmesperre | aktueller R9-Lauf NOT RUN |
| Hardware | Echte Geräte-Schreibpfade und physischer Outputwechsel/Hotplug, reale Fähigkeitsgrenzen | ergänzende R9-Belege NOT RUN |
| Lebensdauer | Lange Öffnungsserie nach Warm-up, Cache-/RSS-Verlauf; drei volle Ruheproben mit aktuellen Identitäten | NOT RUN; R7/R8-Zykluszuwachs bleibt offen |

## Lesende Vorbereitung für V23 und Lebensdauer

`wayland/handlers/compositor.rs` invalidiert beim Skalenwechsel bislang nur
den Hub; ein neuer Buffermaßstab wird dort nicht gesetzt. Der Launcher zeichnet
den Inhalt in logischer Größe und passt anschließend das fertige Raster an.
Ein bloßer Filterwechsel wäre deshalb kein Nachweis nativer Textschärfe.
Die folgenden Verantwortungen müssen beim späteren Fix gemeinsam berücksichtigt
werden: physischer Buffermaßstab, logische Layout-/Hitgeometrie, tatsächliche
Glyphenrastergröße, Clipkoordinaten, Preview-/Artworkschlüssel sowie Invalidierung.
Für Fractional Scale muss die bestehende Wayland-/SCTK-Grenze respektiert werden.

Die vorhandene Serifüberschrift begrenzt ihren Glyphencache auf 128 Einträge
und 512 KiB, derzeit bei einer festen Rollengröße. FreeTypes `glyph_cache` in
`niwoe-freetype` und der Shell-`IconCache` verwenden dagegen HashMaps ohne
Eviction. Das ist ein Quellbefund für die spätere Speicherprüfung, **keine**
nachgewiesene Ursache des beobachteten RSS-Zuwachses. Zusätzliche Rastergrößen
dürfen deren Lebensdauer nicht unkontrolliert erweitern. Ein notwendiger Fix
braucht einen begrenzten Cache, nachvollziehbare Schlüssel und einen echten
Warm-up-/Langzeitvergleich; keine zusätzliche Frame- oder Capture-Schleife.
Der Fontwechsel wird im vorhandenen Config-IPC-Pfad nur bei geändertem
Fontmuster ausgelöst, nicht bei jedem Öffnen der Einführung. Die vorhandenen
Font-Leaks bei seltenen tatsächlichen Wechseln erklären daher nicht allein
die gemessene Fünfseitenserie ohne Fontänderung.

Keine zusätzliche Ruständerung wurde allein aufgrund der lesenden V23-/Cache-Vorbereitung vorgenommen.
Die ausstehenden Pflichtfälle bleiben offen; P13 und Gesamtfreigabe bleiben
gesperrt.
