# NIWOE: Umsetzungsplan und Phasenabnahme

**Aktueller Arbeitsstand, 02.10.2026: visuelle Abnahme wieder offen.**
Der Nutzer hat die Behauptung einer fertigen Desktop-Alpha ausdrücklich
zurückgewiesen. Der erneute Live-Vergleich mit allen vier verbindlichen Mockups
bestätigt erhebliche Mängel an Hub, Control Center, Formularen und Einstellungen.
Die früheren Funktions-, Sicherheits- und Performancebelege bleiben historische
Nachweise; sie ersetzen keine Abnahme von Komposition, Material, Typografie,
Abständen, Icons und Navigation. Die betroffenen visuellen Gates P02–P12 sind
wieder offen. P13 bleibt unbegonnen; nächster Auftrag ist die native Qualitätsrunde.
Befunde, Screenshots und erster installierter Navigationsfix:
[UI-Prüfbericht vom 02.10.2026](docs/phase-reports/UI_VISUAL_AUDIT_2026-10-02.md).

## Aktiver Reparaturplan für die Benutzeroberfläche

Stand 02.10.2026. Der Prüfbericht ist die **Befund- und Belegbasis**;
dieser Abschnitt ist der **verbindliche Abarbeitungsplan**. Er konkretisiert
die wieder geöffneten visuellen Gates der bestehenden Phasen und ersetzt
deren frühere Fertigmeldungen nicht durch neue. Es entsteht keine zweite
Roadmap. Designmanifest und die vier verbindlichen Mockups einschließlich
späterer Nutzerkorrekturen bestimmen weiterhin das Ziel.

**Ausgangsstatus:** V01–V20 sind offen. Nur der im Prüfbericht separat
beschriebene Layer-/Rückwegfehler ist bereits geprüft und installiert.
Insbesondere V01 ist dadurch nicht erledigt: Apps öffnet weiterhin die alte
Settings-Oberfläche. Auch die bislang nur beobachteten Popup-/Vorschaufehler
und die fehlende Prüfmatrix werden unten ausdrücklich mitgeführt.

**Fortschreibung nach der Reparaturrunde:** Der Ausgangsstatus beschreibt
den Beginn der Prüfung. R1–R3 sind umgesetzt, geprüft und installiert;
der gemeinsame Control-Center-Rahmen R4 ist inzwischen visuell geprüft,
einschließlich aller regulären Settings-Einstiege, Rückwege, beider Outputs
und aktueller 40-Zyklen-/3×300-s-Messreihe. Apps öffnet jetzt die Seite im
gemeinsamen Rahmen. R5 ist implementiert und in den dokumentierten Verwaltungs-
zuständen visuell geprüft: Kopf, sechs native Karten, Toolbar, echte Vorschauen,
Liste und Paging auf beiden Outputs bei vier Skalierungen. Aktueller
installierter Shell-Stand:
`98864dc2a01163b005b0bdc2ba580609fe73d19710a57d69c64a8fa6c7182c62`.
Die aktuelle 40-Zyklen-/3×300-s-Serie liegt im CPU-Budget; der Speicher blieb
während aller Ruheproben konstant. Der zusätzliche Compositor-Speicheraufbau
zwischen Zyklus 20 und 40 bleibt Gegenstand der längeren R9-Prüfung.
Details und Grenzen stehen im Prüfbericht. R6 ist im dokumentierten Umfang umgesetzt: Kopf, echte Vorschau
und Neu-Raum-Entwurf, Symbol-/Kontextcontrols, App-Auswahl sowie getrennte
Datei-/Restore-Seiten sind in den dokumentierten Zuständen geprüft/installiert.
Bestätigtes Raum-/Layout-/Dateiverweisspeichern und Anordnen wurden mit eigenen
Testdaten nachgewiesen; anschließend wurden diese Daten entfernt. Als Nächstes
ist die Leistenpreview im Control Center in lesbarer Originalgeometrie geprüft.
Der Fokus-/Textabstandsbefund am ersten Stand wurde zentral korrigiert; alle
47 finalen Live-/Matrixaufnahmen wurden angesehen. Speichern/Wiederöffnen,
Tastaturbedienung und exakte Bereinigung sind bestätigt. Die aktuelle R6-Serie
mit 40 Bedienzyklen und drei Ruheproben zu je 300 Sekunden ist bestanden:
CPU im Budget, kein RSS-Aufbau zwischen Zyklus 20 und 40, einmalig 128 KiB
Shell-RSS in der ersten Ruheprobe, danach konstant. R7 läuft nach der aktuellen
Settings-Inventur weiter: Cursor und Wallpaper sind in ihrem dokumentierten
Bedien-/Monitor-/Scale-Umfang visuell geprüft; die Anzeige-Reparatur ist geprüft
und installiert; alle 32 Bilder ihrer Monitor-/Scale-/Navigationsmatrix sind
durchgesehen. Der dabei gesondert reproduzierte Drehungsfehler V31 ist korrigiert
und für den vollständigen nativen Drehzyklus visuell geprüft. Auflösung,
Skalierung und Primärrückwahl sind im dokumentierten Speicherumfang geprüft;
die Layerzuordnung beim laufenden Primärwechsel bleibt als V32 in R9 offen.
Die Standard-App-Darstellung ist in fünf regulären und allen 32 tatsächlichen
Monitor-/Scale-/Listenbildern durchgesehen. Der anschließend reproduzierte
optimistische Schreibfehler V33 ist korrigiert/installiert; der Fall einer
entfernten eigenen App sowie erfolgreiche Maus-/Tastaturspeicherung,
Wiederöffnen und Erhaltung vorhandener Auto-Zuordnungen sind in zwölf
Nachherbildern nativ visuell geprüft. Weitere Fehlerzustände bleiben in R9 offen.
Netzwerk-/Bluetooth-Zeilen und Controls sind auf die gemeinsamen Rollen
überführt, geprüft und installiert. Die reguläre FHD-Darstellung ist in vier tatsächlichen
Bildern geprüft, einschließlich korrekter Kabelnetz-/WLAN-Symbole und
Bluetooth-Suchfokus. Die anschließende Provider-Matrix umfasst 82 tatsächlich
angesehene Bilder auf beiden Outputs bei vier Skalierungen, einschließlich
der zweiten Netzwerk-/Bluetooth-/Audioseite bei kleinerem Canvas.
Audio und Drucker verwenden die gemeinsamen Zeilen/Controls; der installierte
Stand ist in vier tatsächlichen FHD-Aufnahmen mit Tastaturfokus durchgesehen.
Alle vier Provider sind zusätzlich in 32 tatsächlichen Leer-/Aus-/Lesefehler-/
Überlaufbildern bei 100/200 Prozent durchgesehen. Fehlerhafte Bluetooth-/Audio-/
Druckerantworten bleiben unbekannt statt leer oder null (V37 korrigiert).
Der dabei gefundene sprachlich vermischte Netzwerk-Listenstatus V38 ist korrigiert,
vollständig automatisch geprüft und installiert. Zeilentexte sind nativ bestätigt;
ein zusätzlich gefundener unbekannter Footerzähler ist ebenfalls korrigiert und
installiert und in sechs tatsächlichen Bildern beider Teilfehlerrichtungen und
ungültiger Antworten bei 100/200 Prozent bestätigt. Die aktuelle R7-Serie mit
40 Bedienzyklen und drei vollen 300-Sekunden-Ruheproben ist abgeschlossen:
CPU im Budget, kein RSS-Zuwachs im Leerlauf. Die Shell nimmt zwischen Zyklus
20 und 40 noch 2,125 MiB auf; die längere Lebensdauerprüfung bleibt in R9 offen.
Diese lesenden Fixtures ersetzen keine echten Geräte-Schreib-/Hardwarebelege.
Übersicht, Konten, Updates und Energie sind im dokumentierten Inhaltsschritt
installiert; alle acht regulären FHD-Bilder sind durchgesehen. Der zusätzliche
Bestätigungsbefund V35 ist korrigiert und in fünf nativen Erstbestätigungs-/
Abbruch-/Ablaufbildern geprüft. Fedora zeigt die fehlende Paketintegration ehrlich;
deren gesonderter Backendumfang bleibt offen. Historische Panel-Pinninganfragen
führen zur aktiven Apps-Seite, gespeicherte Daten bleiben erhalten.
R7 ist im dokumentierten Darstellungsumfang visuell geprüft und installiert;
R8 ist umgesetzt und installiert; alle fünf finalen FHD-Seiten und alle 40
Monitor-/Scale-Bilder der Einführung sind tatsächlich angesehen. Die native
Bedienrunde mit 19 angesehenen Bildern einschließlich eigener bestätigter
Leistenübernahme ist bestanden und exakt bereinigt. Der Lesefehlerfall ist nach
einem korrigierten Prüfeinstieg tatsächlich angesehen: Weiter deaktiviert,
Neuladen und sicherer Ausstieg erhalten. Der dabei bestätigte englische
Parserhinweis V39 ist korrigiert, vollständig automatisch geprüft/installiert
und in beiden aktuellen Fehlerbildern tatsächlich bestätigt.
Die aktuelle R8-Serie mit 40 Fünfseitenzyklen und drei vollen 300-Sekunden-
Ruheproben ist abgeschlossen: CPU im Budget, RSS im Leerlauf konstant.
Shell +2,125 MiB und Compositor +7,03125 MiB zwischen Zyklus 20 und 40 bleiben
in R9 zu prüfen. Die Messung stammt vom Darstellungsrelease vor dem reinen
Fehlertextfix. R8 ist im dokumentierten Umfang geschlossen; R9, zusätzliche Live-/Fehlerfälle
aus R6, lange Datenfälle
und die Skalierungsweichheit V23 bleiben offen. Keine Gesamtfreigabe.

R9/V32 ist inzwischen automatisch geprüft und als Release installiert:
Compositor `8f1e15e3…`, Shell `010d2ac4…`, 1309 Tests bestanden.
Nach der tatsächlichen neuen Anmeldung vom 03.10.2026 laufen die neuen
Identitäten. Beide nativen Primärwechsel sind ohne Prozessneustart visuell
geprüft: Panel und Control Center folgen dem neuen Primärmonitor, der andere
zeigt ausschließlich Wallpaper. Alle neun Nachher-/Sitzungsbilder angesehen;
Loge, einmaliger Login-Hub und Watchdog ohne erneute Begrüßung bestätigt.
Originaldateien, Outputs und Eingabegeräte-ACL exakt wiederhergestellt.
Der interne Tailfehler des ersten Helpers und der erfolgreiche separate
Abschluss bleiben ausdrücklich dokumentiert, nicht als ein fehlerfreier
Gesamtlauf umgedeutet. [Native Belege](docs/phase-reports/evidence/r9/primary-after/README.md).
Physischer Hotplug und weitere R9-Pflichtfälle bleiben offen.

**Neue Designkorrektur, 03.10.2026:** Der Nutzer beanstandet gelbe
Aktivitätsstriche, gequetschte Controls/Flächen und komplexe Menüführung.
Diese Korrektur wird in der laufenden Qualitätsrunde vor der weiteren
R9-Abschlussmatrix umgesetzt: neutrale Auswahlflächen, getrennter Hover und
Tastaturfokus, mehr Höhe/Innenabstand, eine klare globale Navigation mit sieben
verfügbaren Bereichen. Alle Einstellungen bleiben im Control Center;
raumbezogene Dateien/Wiederherstellung in der Raumkonfiguration. Manifest
fortgeschrieben; keine neue Roadmap oder Alpha-Freigabe. Der größere
gemeinsame Dichteschritt öffnet die betroffenen visuellen Abstände-/Layoutgates
erneut; frühere R4–R8-Bilder ersetzen seinen aktuellen Nachhervergleich nicht.

Die erste gemeinsame Korrekturrunde ist automatisch geprüft und auf dem
Testrechner aktiv: Shell `e9bed1136f1dc3c014a498f90b03fd56e531fae687163a0279ab7c383a703451`,
Compositor weiterhin `8f1e15e3…`. 1311 Workspacetests, Format, Workspacecheck,
Design-/Größen-/Zentralitätsguards und striktes Clippy bestanden. 19 aktuelle
FHD-Ansichten und 60 zusätzliche Ansichten beider tatsächlichen Monitore bei
100/150/200 Prozent einzeln angesehen; ursprüngliche Konfiguration und
Sitzungszustand wiederhergestellt. Neutrale Auswahl/Fokus, sieben Sidebarziele,
Settings im Control Center, größere Controls und sechs ausreichend hohe
Appauswahlen nachgewiesen. [Änderungen und genaue Grenzen](docs/phase-reports/evidence/r9/calm-controls/README.md).
Weitere acht angesehene Bilder bestätigen nach korrigierter Helper-Scrollrichtung
den erreichbaren unteren Formularbereich auf FHD bei 150/200 Prozent; alle
Bereinigungsbelege einschließlich ursprünglicher Eingabegeräte-ACL bestätigt.
Die bekannte Skalierungsweichheit V23 und die lange Speicher-/Ruheprüfung des
neuen Binärstandes bleiben offen; dies ist keine R9-Gesamtfreigabe.

### Arbeitsweise und Abschluss eines Mangels

- Vor einer Änderung den Befund am installierten Desktop reproduzieren,
  aktuellen Screenshot und Buildidentität sichern. Prüfen, ob ein gemeinsamer
  Komponentenfehler mehrere Seiten betrifft; dann an dieser Quelle reparieren.
- In kleinen, nach Verantwortung getrennten Rust-Modulen arbeiten. Designwerte
  ausschließlich in `niwoe-tokens`/`niwoe-config`, gemeinsame Controls in
  `niwoe-ui`; keine lokale Farb-, Alpha-, Radius- oder Geometriekorrektur.
  Keine neue Dependency und keine Änderung an Cargo-Manifesten im Reparaturumfang.
- Für Bildköpfe, Icons und Vorschauen jeweils Cache-Schlüssel, Invalidierung,
  Speichergrenze und Lebensdauer festhalten. Keine neue dauernde Capture- oder
  Frame-Schleife; compositorseitiges Glas und Renderreihenfolge erhalten.
- Nach Rust-Änderungen `cargo check --workspace`, für Fixes
  `cargo test --workspace`, `cargo fmt` und den Design Guard ausführen.
  Bei Rendering/Input/IPC den echten Call-Flow zusätzlich prüfen. Relevante
  Regressionstests ergänzen; keine Tests als Ersatz für die Bildprüfung.
- Jeden erfolgreich geprüften Implementierungsstand als Release auf Fedora
  installieren. Release-, installierte und laufende Hashes vergleichen;
  Shelländerungen über den vorhandenen Watchdog aktivieren, bei
  Compositoränderungen einen erforderlichen Neulogin ausdrücklich melden.
- Ein Mangel erhält erst **visuell geprüft**, wenn Vorher-/Nachher-Aufnahmen
  des installierten Stands angesehen wurden, das unten genannte Kriterium
  erfüllt ist und Maus/Tastatur funktionieren. **Geprüft/installiert** allein
  genügt nicht. Eine Nutzerabnahme wird nur bei tatsächlicher Freigabe vermerkt.
  Fehlende Pflichtfälle bleiben `NOT RUN` und offen.

Pro Mangel im Prüfbericht fortschreiben: Status (`offen`, `in Arbeit`,
`geprüft/installiert`, `visuell geprüft`), Ursache, geänderte Dateien,
Buildhash, Befehle/Exitcodes, Bildvergleich, getestete Zustände und offene Fälle.
Historische Bilder und Fehlversuche erhalten; sie niemals nachträglich als
bestandene neue Prüfung umdeuten. Neue Befunde ab V21 fortlaufend ergänzen.

### Reihenfolge der Arbeitspakete

Nutzerpräzisierung vom 02.10.2026: Die Einstellungen gehören vollständig
in das gemeinsame Control Center (R4). Die separate Settings-Karte ist kein
Zielzustand. Die Reparaturen werden weiterhin sorgfältig in der festgelegten
Reihenfolge ausgeführt; R4 wird nicht vor Hub und Quick Settings vorgezogen.

Die Pakete sind Reparaturschritte innerhalb P02–P12, keine neuen Produktphasen.
Aktuell ist kein Paket als fertig erklärt. Mit R1 beginnen; die vorhandene
Panelgestaltung bleibt erhalten, sofern kein konkret belegter Fehler eine
Korrektur verlangt. Danach Hub und Quick Settings prüfen, bevor weitere
Verwaltungsflächen umgebaut werden.

| Reihenfolge | Reparaturauftrag | Lieferobjekt und Abhängigkeit |
| --- | --- | --- |
| R1 | Gemeinsame Text-, Icon-, Fokus- und Formularrollen; Panel gegen `(1)` prüfen. | Sichtbar passende Überschriftenhierarchie, getrennte Haupt-/Unterzeilen, native Icons statt fehlender Glyphen, einheitlicher Fokus und klare Controlzustände. Panelhöhe, mittige Uhr, Raumfolge und freigegebenes Material bewahren. Bestandene Panelprüfung ist Voraussetzung für R2. |
| R2 | Hub nach `(2)` einschließlich Suche und Fensterliste. | Eigener gecachter atmosphärischer Bildkopf, vier vollständig komponierte Raumkarten, drei untere Informationsbereiche; reale Vorschauen in der vorgesehenen Bildfläche. Lesbarkeit über Wallpaper und heller Anwendung; echte Fähigkeitszustände statt erfundener Inhalte. Gemeinsame Rollen aus R1 verwenden. |
| R3 | Quick Settings und zugehörige Popups nach `(1)`. | Freigegebenes Deck-/OSD-Material erhalten; Icons, Text und Fokus vereinheitlichen. Kalender und Raumoverflow am auslösenden Control ausrichten, auf Outputgrenzen begrenzen. Popup-Kollision und Kontextmenü-Flyout reproduzieren und beheben, falls bestätigt. Nach R1/R2. |
| R4 | Ein gemeinsamer Control-Center-Rahmen und konsistente Navigation. | Sidebar mit Icons und tatsächlichem Aktivzustand für Räume, Apps, Benutzer, System, Einstellungen und Leiste. Vorhandene Settings-Inhalte und Provider in diesen Rahmen übernehmen; alte parallele Settings-Karte aus allen regulären Einstiegen entfernen. Kein weiterer Settings-Neubau daneben. Nach der Qualitätsrunde R1–R3. |
| R5 | Räume verwalten nach `(3)`. | Eigener proportionaler Bildkopf, vollständige Toolbar, bildliche Raumkarten, Schnellaktionen und Statistik im vorgesehenen Raster. Vorhandene Sortierung/Filter wirklich anbinden; fehlende Kategorie-/Import-/Vorlagenfähigkeiten in dieser Struktur ehrlich kennzeichnen. R4 und vorhandene Vorschauquelle nutzen. |
| R6 | Raum konfigurieren nach `(4)` und Leistenkonfiguration. | Proportionaler Bildkopf, Symbolvorschau, klare Detail-/Kontextcontrols, echte räumliche Vorschau; getrennte Apps-, Dateien- und Restore-Seiten. Leistenpreview in lesbarer Originalgeometrie oder ausdrücklich bezeichnetem Ausschnitt. Entwürfe, Speichern/Abbrechen und Fehler erhalten. Nach R4/R5. |
| R7 | Übrige Settings-Inhalte und Providerdarstellung bereinigen. | Bild-/Cursorvorschauen, lesbare Katalognamen, konsistente Sprache, verständliche Geräte-/Statusdarstellung, Standard-App-Auswahl. Veraltete Panel-Pinning-Behauptung entfernen. Fedora-Updates gesondert behandeln wie unten beschrieben. R4-Rahmen und R1-Controls nutzen. |
| R8 | Alle fünf First-Run-Seiten mit denselben Komponenten korrigieren. | Inhalt, Gruppen, Vorschauen und primäre/sekundäre Aktionen passend zur verfügbaren Fläche. Keine fünf gleich gewichteten Footeraktionen und keine langen Pseudo-Checkboxbuttons. Entwurf/Wiederaufnahme und Übernahmegrenzen erhalten. Nach R1/R6/R7. |
| R9 | Vollständige visuelle und funktionale Abschlussrunde. | Vier Mockup-Bildschirme und sämtliche Nebenflächen in der unten genannten Matrix; offene Beobachtungen nachprüfen, neue Mängel aufnehmen. Aktuelle Performancebelege und vollständiges Cleanup. Erst danach Gesamtfreigabe/P13 erneut bewerten. |

R1 korrigiert gemeinsame Rollen, ersetzt aber nicht die gesonderte Abnahme
jeder betroffenen Seite. Beispielsweise gilt V15 erst nach Prüfung der
Standard-App-Zeilen **und** der Netzwerksymbole als visuell geprüft.

### Zuordnung und überprüfbare Kriterien für V01 bis V20

Alle Status bleiben zunächst **offen**. Die Kriterien unten beschreiben das
Ergebnis am installierten Desktop, nicht allein eine Quellcodeänderung.

| Befund | Paket | Erforderlicher sichtbarer Abschlussnachweis |
| --- | --- | --- |
| V01 Alte Settings-Familie | R4 | Apps, Benutzer, System und Einstellungen bleiben im selben Control-Center-Rahmen. Zurück/Escape erreichen den richtigen Ursprung; kein Einstieg öffnet die alte separate Settings-Karte. Hublage und Glas bleiben nach jedem Rückweg korrekt. |
| V02 Hubkopf und Material | R2 | Hub neben `(2)` vergleichen: atmosphärischer Kopf, passende Größe/Position/Gewichtung der Texte und vollständige Kartenkomposition. Dahinterliegende Apptexte konkurrieren nicht mit dem Hubinhalt. Beide Hintergründe mit demselben zentralen Materialpfad prüfen. |
| V03 Verwaltungskopf | R5 | Eigene Landschaftskomposition wie `(3)`, ohne sichtbares fremdes Appmenü. Kopf bleibt bei Wallpaper und heller Anwendung stabil, lesbar und proportional. |
| V04 Gestauchter Konfigurationskopf | R6 | Landschaft wird proportional skaliert und passend beschnitten; Berge/Strukturen bleiben unverzerrt. Ausschnitt in beiden Zielgrößen neben `(4)` prüfen. |
| V05 Fehlende Raumvorschauen | R5/R6 | Belegter Raum zeigt echte Fenster-/Layoutvorschau in Verwaltung und Konfiguration; Fensterende/Minimieren/Raumwechsel aktualisieren oder invalidieren sie. Leerer Raum erhält einen gestalteten ehrlichen Leerzustand. Keine erfundenen Beispiel-Appbilder. |
| V06 Unvollständige Toolbar | R5 | Filter mit echten Zählern, klare Sortierung, Kategoriencontrol, Suche, Neuer Raum und Raster-/Listencontrols an den vorgesehenen Stellen. Verfügbare Controls wirken tatsächlich; ein Control ohne Backend ist sichtbar als nicht verfügbar gekennzeichnet. Fehlende Kategoriequelle nicht durch erfundene Kategorien ersetzen. |
| V07 Sidebaricons und Aktivzustand | R4 | Alle vorgesehenen Einträge nutzen native Icons; die Markierung folgt der tatsächlich geöffneten Seite. Leiste markiert nicht Räume. Maus und Tastatur erreichen dieselben Ziele. |
| V08 Reduzierte Raumdetails | R6 | Sichtbare Symbolvorschau und geordnete Beschriftung/Felder; Kontextoptionen als verständliche Controlzeilen mit Fähigkeitszustand. Keine namenlosen Großbuttons; Entwurfsänderung und Abbrechen nachvollziehbar. |
| V09 Raum-App-Liste | R6 | Appicon, lesbarer Name, sekundäre Beschreibung und eindeutige Auswahl pro Zeile; Identitätsdetails nur wo entscheidungsrelevant. Seitenwechsel, lange Namen und native/XWayland-Dubletten verständlich bedienbar. |
| V10 Dateien und Restore vermischt | R6 | Dateien zeigt explizite Dateireferenzen, Restore seine Layout-/Wiederöffnenaktionen und Ergebnisse. Leerzustand erklärt fehlendes Layout ohne rohen OS-Fehler. Eingabe, Abbruch, Fehler und Bestätigung jeweils am passenden Inhalt. |
| V11 Winzige Panelvorschau | R6 | Preview zeigt dieselbe Panelgeometrie und Textmetriken wie das Produkt; Vorschau und Einstellungsgruppen bilden eine lesbare Seite. Keine Verzerrung oder unbeschriftete Miniatur. Übernahme/Abbrechen an beiden Zielgrößen prüfen. |
| V12 Wallpaperliste | R7 | Tatsächliche gecachte Bildthumbnails; eindeutige Katalognamen ohne ungeprüfte Screenshotduplikate. Deutsche, verständliche Moduscontrols; fehlende Datei hat erkennbaren Fehlerzustand. Auswahl wirkt nur über den vorhandenen bestätigten Pfad. |
| V13 Cursorliste | R7 | Tatsächliche Cursorvorschau und benutzerlesbare Theme-/Größenbezeichnungen. Vorschau entspricht Auswahl; Laden/Fehler/Skalierung geprüft. |
| V14 Veraltetes Panel-Pinning | R4/R7 | Kein aktiver Einstellungsweg verspricht angeheftete Programme im Panel. Bestehende Daten nicht löschen; einen verbleibenden echten Hub-/Schnellstartzweck ausdrücklich korrekt beschreiben. Falls kein Produktzweck besteht, veralteten UI-Eintrag entfernen. |
| V15 Textabstände und fehlende Glyphen | R1/R3/R7 | Standard-App-Haupt-/Unterzeilen berühren oder überdecken sich nicht. Dropdown-/Sicherheitsicons sind erkennbar statt Rechteckglyphen. Normal, Fokus, Auswahl, lange Namen und Scale-Wechsel auf beiden Seiten prüfen. |
| V16 Technische und falsche Statusdarstellung | R7 | Geräte-/System-/Audio-/Druckerseiten benutzerlesbar und sprachlich konsistent. Fedora-Updates verwendet einen vorhandenen Fedora-Provider oder benennt eine tatsächlich fehlende Integration korrekt. Kein „apt nicht verfügbar“ als Fedora-Produktstatus und keine erfundene Updatezahl. Backendumfang separat nachweisen. |
| V17 Popupanker und Sprache | R3 | Kalender an der mittigen Uhr, Raumoverflow an seinem Auslöser; beide auf dem richtigen Output und innerhalb dessen Grenzen. Deutsche Wochentage und einheitlicher Fokus bei langen Raumlisten/Scale. |
| V18 First Run | R8 | Alle fünf Seiten mit klarer Hierarchie, lesbaren Controls und eindeutiger Hauptaktion. Katalog-/Panelvorschauen nutzbar; Entwurf fortsetzen, Abbrechen und bestätigte Übernahme visuell und funktional prüfen. |
| V19 Suche und Fensterlistenfokus | R1/R2 | Suchzeilen mit Appicon, sauber getrennten Textrollen und einheitlicher Auswahl; Fensterliste verwendet die gemeinsame Fokusrolle. Pfeile/Enter/Escape und langer Ergebnistext ohne Layoutbruch. |
| V20 Technischer Neu-Raum-Platzhalter | R6 | Ungespeicherter Raum erscheint als Entwurf ohne „Raum 0“. Benennen aktualisiert die Vorschau; Abbrechen erzeugt keinen Raum. Loge bleibt neutral und wird weder angelegt noch als Raum mitgezählt. |

V16 wird bei Bedarf in dokumentierte Teilaufträge für Darstellung und
Fedora-Provider aufgeteilt. Der jetzige Provider ruft ausschließlich `apt`
auf (`crates/niwoe-shell/src/updates.rs`); ein Fedora-Provider ist daher nicht
durch bloße Textkorrektur nachgewiesen. Eine neue Paketverwaltungsfunktion
oder privilegierte Updateaktion gehört nicht stillschweigend zu dieser
UI-Reparatur. Bis ein fehlender Backendumfang entschieden und geprüft ist,
bleibt diese Teilanforderung offen; eine ehrliche Fähigkeitsanzeige beseitigt
nur den irreführenden UI-Zustand.

### Noch nicht bestätigte Fehler und fehlende Bildschirmabdeckung

Diese Punkte sind eigene offene Prüfaufträge in R3/R9 und dürfen nicht hinter
den zwanzig bestätigten Befunden verschwinden:

- Raumoverflow plus Desktopkontextmenü: genaue Klickfolge reproduzieren;
  feststellen, welche Popups gleichzeitig zulässig sind und ob das fremde
  Popup korrekt geschlossen wird. Erst nach Reproduktion einen Fix zuordnen.
- Settings-Flyout: Hover sowie Klick/Pfeiltasten prüfen; tatsächliche
  Interaktionsvorgabe von fehlendem Rendern oder falschem Hitbereich trennen.
- Zeitweise fehlende Fenstervorschau: Fenster-/Raumidentität, Generation und
  Invalidierung beim Öffnen der Fensterliste prüfen; kein dauernder Capture
  als Ersatz für die Cachekorrektur.
- Noch nicht besichtigte Oberflächen inventarisieren und wirklich öffnen:
  Audio-OSD, Screenshot-Regionsauswahl, alle Picker, Bestätigungen und
  Fehlerdialoge sowie Login, Lock, PAM und Polkit. Nicht nur Happy Paths zählen.
  Schutzflächen nicht für Screenshots aufheben; wenn die Aufnahme gesperrt ist,
  lokal visuell prüfen und Aufnahmegrenze dokumentieren. Ein nicht ausgeführter
  Pflichtfall bleibt offen.

### Gemeinsame visuelle Prüfmatrix und Abschlussgate

Für jede geänderte Seite festhalten: Zielgröße, Scale, Output, Datenzustand,
Eingabemethode, installierter Build und Screenshot. Den neuen Screenshot
direkt neben das passende Originalmockup stellen. Bei Nebenflächen ohne
eigenes Mockup dieselben geprüften Komponenten und Rollen als Maßstab verwenden.

| Dimension | Pflichtfälle |
| --- | --- |
| Zielgrößen | 1920×1080 und 1366×768; physische Modi und logische Testgröße unterscheiden. Fehlenden DRM-Modus nicht als physisch getestet ausgeben. |
| Skalierung und Outputs | 100/150/200 % wo verfügbar; beide vorhandenen Outputs. Eingabe-/Popupkoordinaten, Monitorwechsel und Grenzen prüfen. |
| Hintergründe | Wallpaper und helle reale Anwendung; besonders Hub/Control Center auf lesbaren Kontrast und unerwünschtes Durchscheinen prüfen. |
| Inhalte | Leere und belegte Räume, mehrere echte Fenster, lange Namen/Beschreibungen, große gültige Raum-/Appkataloge, fehlende Daten/Assets/Provider. |
| Controlzustände | Normal, Hover, Fokus, gedrückt, ausgewählt, deaktiviert, Laden und Fehler; Textposition, Größe, Abstand, Icon und Hitbereich prüfen. |
| Bedienwege | Maus und Tastatur; Hub → Verwaltung → Konfiguration → zurück, sämtliche Sidebarziele, Suche, Picker, Speichern/Abbrechen/Konflikt. |
| Lebenszyklus | Wiederöffnen, Shell-Watchdog, neue Login-Sitzung: Loge, genau einmaliger Willkommens-Hub, direkter Appstart nach Raum 1; keine alten Layer-/Fokuszustände. |

Aktuelle Performancebelege für Änderungen an Material, Assets, Vorschauen
und Invalidierung gemäß §7: gleiche Hardware/Modi/Last, Warm-up,
drei Fünf-Minuten-Idle-Proben vor/nach der relevanten Änderung und die dort
genannten Öffnungszyklen. Cache-/RSS-Lebensdauer beim wiederholten Öffnen prüfen.
Fehlende GPU-, Repaint- oder Latenzmetriken bleiben ausdrücklich unbekannt;
historische P12-Werte sind keine neue Messung. Nach jeder Prüfrunde eigene
Testfenster/Entwürfe bereinigen und temporäre Zugänge/Berechtigungen entfernen;
vorhandene Räume und KDE-/GTK-Konfiguration erhalten.

**Gesamtabschluss erst**, wenn V01–V20 einschließlich aller Teilanforderungen
belegt erledigt oder durch eine ausdrückliche Produktentscheidung ersetzt
sind, zusätzliche bestätigte Mängel abgearbeitet sind und keine erforderliche
Bildschirm-/Zustandsprüfung als `NOT RUN` verbleibt. Die vier Kompositionen
und durchgehenden Wege müssen am installierten Desktop stimmen. Erst dann
den Gesamtstatus erneut bewerten; P13 bleibt bis dahin gesperrt.

## Historische Belege für Implementierung und Abnahme

**Historischer P12-Funktionsabschluss, 27.09.2026: `accepted`.** Ausgangsstand `f5d7885`,
Produktstand `8970b17` auf Fedora installiert und laufende Hashes geprüft.
Pflichtmatrix, Dateiliste und Messgrenzen: [P12-Bericht](docs/phase-reports/P12.md).
Reale KWrite-Arbeit einschließlich RTC-Suspend/Resume, physischer HDMI-Hotplug,
optischer Lockschutz während Shellabsturz sowie Ethernet/WLAN-Rundlauf belegt.
Fokus-, Stapel- und Hotplug-Fehler korrigiert und als Release installiert.
1.221 Workspace-Tests, separate D-Bus-/Integrationsgates und vollständiges Cleanup
belegt. Finale 3×300-s-Serie: CPU-Medianänderung +0,01000/+0,000002 Prozentpunkte
Compositor/Shell, auch alle Einzelwerte innerhalb des Budgets. RSS über 60 finale
UI-Zyklen konstant. Optische Latenz und Cache-Zähler bleiben unbekannt;
eine getrennte Repaint-Stichprobe zeigt nur den erwarteten Minutenwechsel.
P13 bleibt unbegonnen. Frühere Phasenbelege ersetzen keine P12-Integrationstests.

**P11-Abschluss, 27.09.2026: `accepted` für den dokumentierten Fedora-Funktionsumfang.**
P11-01 bis P11-05 umgesetzt: fünf native Seiten, konservative Frischerkennung,
Maus-/Tastaturbedienung, reale Katalog-/Raum-/Leistenpfade, drei Übungen und
versionierter Entwurf mit dauerhaftem Mehrdokumentjournal und Wiederaufnahme.
1.215 Workspace-Tests plus isolierter D-Bus-Test, Check, Clippy und alle Guards grün.
Release `4b271d1` auf Fedora installiert, laufende Binärhashes geprüft; echte
Fehler-/Crash-/Sitzungs- und Skalierungsabnahme bestanden, Testzugänge bereinigt.
Drei finale 300-s-Idle-Proben: CPU-Mediandifferenz −0,00333/+0,00333 Prozentpunkte
Compositor/Shell; 60 Vergleichszyklen ohne RSS-Wachstum. Optische Latenz und
Repaint-Zähler bleiben ungemessen. Dateimatrix, Rohdaten und Grenzen:
[P11-Bericht](docs/phase-reports/P11.md). **Aktueller P12-Status siehe oben.**

**P10-Abschluss, 27.09.2026: `accepted` für den dokumentierten Fedora-Funktionsumfang.**
P10-01 bis P10-07 umgesetzt: vollständige Raumformulare mit atomarem Save/Cancel,
stabilen IDs, Suche/Belegung/Reihenfolge, echtem Appkatalog, Zuordnung und manuellen
P09-Restore-/Dateiverweisen; persistente Leistenmodule und gemeinsame Produktvorschau.
Alter Farbtheme-Wähler und Automatisierungsplatzhalter aus aktiver Navigation entfernt.
1.209 Workspace-Tests plus der separat isolierte D-Bus-Test bestanden; alle
Pflichtguards/Clippy grün. Endrelease installiert und laufende Hashes geprüft.
Reguläre Hub-Wege, Maus/Tastatur, Fehler/Konflikte, native/XWayland-Fenster,
Shell-Watchdog und Neulogin praktisch geprüft. Logische Größen 1920×1080 und
1366×768 sowie 150/200 % geprüft; kein physischer 1366×768-DRM-Modus verfügbar.
Je drei kontrollierte Vorher-/Nachher-Idle-Proben und 60 Öffnungszyklen:
CPU-Mediandifferenz +0,010/−0,003 Prozentpunkte Compositor/Shell, P10-RSS stabil.
Die letzte reine Listen-Renderbereinigung wurde erneut geprüft/installiert und
mit weiteren 60 Zyklen getestet; die Idle-Aussage wird begründet auf den
unveränderten geschlossenen Pfad übertragen, nicht als neue Messung ausgegeben.
Optische Latenz-/Repaint-Nachweise bleiben ausdrücklich nicht gemessen.
Dateimatrix, Livebelege, Bereinigung und Grenzen: [P10-Bericht](docs/phase-reports/P10.md).
Zum P10-Abschluss war P11 noch nicht begonnen. Backups, Logs und Benachrichtigungsautomatik bleiben
als nicht verfügbar gekennzeichnet; P10 ist kein Abschluss des gesamten Desktops.

**P09-Abschluss, 27.09.2026: `accepted` für den dokumentierten Fedora-Funktionsumfang.**
P09-01 bis P09-06 umgesetzt: versionierte private Layout-Snapshots, ausdrückliche
und entprellte Speicherung, eindeutiges LayoutOnly, begrenztes manuelles
RelaunchApps mit Deadline/Abbruch sowie native Bedienung und Einzelergebnisse.
1199 unterschiedliche Tests bestanden, Release installiert und laufende
Buildidentitäten geprüft. Echte native/XWayland-Fenster, Reboot, Mehrdeutigkeit,
Single-Instance-Weiterleitung, Fehlerfälle, Monitor-/Scale-Wechsel und 60
IPC-bestätigte Öffnungszyklen geprüft. Neutrale Loge/einmaliger Hub erhalten;
temporäre Zugänge und Testrechte entfernt. Drei neue Vorher-/Nachher-Idle-Proben
mit gleichen Display-Modi; Messbuild, spätere reine Textänderungen und fehlende
optische Latenz-/Repaint-Nachweise ausdrücklich dokumentiert.
Details: [P09-Bericht](docs/phase-reports/P09.md).
Der anschließende P10-Stand ist oben und im P10-Bericht dokumentiert.

**P08-Abschluss, 27.09.2026: `accepted` für den dokumentierten Fedora-Funktionsumfang.**
P08-01 bis P08-05 umgesetzt: persistente Raum-/Fensterdaten, Navigation und
Suche, isolierte statische native/X11-Vorschauen sowie begrenzte,
generationsgebundene Cache-Lebensdauer bis einschließlich Lock.
1185 Tests bestanden; Release installiert und per Neulogin aktiviert.
64 Räume, große gültige Unicode-Snapshots, zwei Outputs, 150/200 %,
Fensterende, minimiertes Aktivieren und 60 Öffnungszyklen geprüft.
Drei fünfminütige Idle-Proben dokumentiert; geänderte Monitor-Modi begrenzen
den strengen Vorher-/Nachher-Vergleich, eine optische Latenzbaseline fehlt.
Diese Messgrenzen sind keine behaupteten bestandenen quantitativen Gates.
Details und Dateimatrix: [P08-Bericht](docs/phase-reports/P08.md).
P09 ist inzwischen im oben dokumentierten Umfang abgeschlossen.


**P07-Abschluss, 26.09.2026: `accepted` für den dokumentierten Fedora-Aufbau.**
P07-01 bis P07-05 sind abgeschlossen: echte Raumleiste, vollständige Navigation
für 1–64 Räume, gleichwertiges Maus-/Tastaturverschieben, Preferred/Dedicated-
Policy mit Startkorrelation und vollständiger Fensterzugang einschließlich
minimierter Fenster. Live-Prüfungen mit 1/9/64 Räumen und zwei physischen Outputs,
nativen/X11-Fenstern sowie Tray-/Benachrichtigungszugang dokumentiert.
Geprüfte Releases sind installiert und nach automatischem Neulogin aktiv.
Keine ausstehende Aktivierung oder P07-Live-Abnahme. Genaue Matrix und Grenzen:
[P07-Abschlussabgleich](docs/phase-reports/P07_ACCEPTANCE_REVIEW.md).
Zum damaligen P07-Abschluss war P08 noch nicht begonnen. Die folgenden Checkpoints
bleiben historische Evidenz und sind gegen diese Abschlussmatrix zu lesen.

**P07 vierter Block, 26.09.2026: `in-progress`.** Einmalige Startkorrelation
über native Aktivierungskennungen und X11-Startup-ID implementiert, vollständig
geprüft und installiert. Eltern-/Move-Priorität, verspätete Metadaten,
minimiertes Restore und Replay getestet. DRM-Aktivierung und Gesamtabschluss
folgen. Details: [P07-Startkorrelation](docs/phase-reports/P07_LAUNCH_CORRELATION.md).

**P07 dritter Block, 26.09.2026: `in-progress`.** Vollständige native Fensterliste
im Hub einschließlich minimierter Fenster, seitenweiser Navigation und
Maus-Verschieben nach stabiler Raum-ID implementiert, geprüft und installiert.
Native-/X11-Move und Wiederherstellung sowie echter Mausablauf bestanden;
Klickdurchgriff beim Aktivieren korrigiert. Aktivierung durch Neulogin und
DRM-Nachprüfung folgen. Beide physischen Outputs sind wieder erkannt.
Startkorrelation und abschließende P07-Matrix bleiben offen. Details:
[P07-Fensterzugang](docs/phase-reports/P07_WINDOW_ACCESS.md).

**P07 zweiter Block, 26.09.2026: `in-progress`.** App-Zuordnung für
Preferred/Dedicated implementiert und installiert: reine Policy,
Native-/XWayland-Lifecycle, Elternpriorität, spätes App-ID/Class und Schutz
manueller Moves. Echte isolierte Protokoll-/Eingabetests, Lock- und
Migrations-/Persistenzregression grün. Dedicated-Hinweis nativ geprüft.
Nach Nutzer-Neulogin laufende Buildidentitäten bestätigt und regulärer DRM-
Nachtest bestanden: Preferred/Native/X11, Dialoge, Dedicated ohne Sperre und
echter Tastatur-Move. Aktuell nur interner Output aktiv; Zweimonitor-Nachtest,
explizite Startkorrelation, Maus-Verschieben und vollständiger Fensterzugang
bleiben offen. Details: [P07-App-Zuordnung](docs/phase-reports/P07_APP_ASSIGNMENT.md).

**P07 begonnen, 26.09.2026: `in-progress`.** Der erste Navigationsbaustein ist
geprüft, installiert und ohne Neulogin aktiv: echte Raumanzahl bis 64 in Panel
und Trefferprüfung, feste Seiten mit sichtbarem aktivem Raum, neutrale Belegung,
vollständiger Name und Maus-/Tastaturblättern in der Raumauswahl. Native
Neun-Räume-Darstellung sowie isolierte 64-Räume-Eingabe geprüft; der isolierte
Winit-Renderpfad liefert noch keinen visuellen 64-Räume-Abnahmenachweis.
App-Regeln, gleichwertige Mausaktionen fürs Verschieben und vollständiger
Fensterzugang bleiben offen. Details und nächste Pakete:
[P07-Raumnavigation](docs/phase-reports/P07_NAVIGATION.md).

**P06-Abschluss, 26.09.2026: `accepted` für den dokumentierten Fedora-Aufbau.**
Der vollständige Datenvertrag nach §4.2 einschließlich Icon, App-Referenzen und
Layout-/Restore-Präferenzen ist implementiert. Schema-2-Migration mit privater
Sicherung, IPC, Reconnect und echte Prozess-Neustart-Persistenz sind geprüft.
Nach Nutzer-Neulogin laufen die geprüften Releases; die tatsächlichen neun Räume
behalten IDs, Reihenfolge, Revision 49 und Zähler 19. Neutrale Loge bestätigt.
Manuelle native Floating-/Tiling-Moves und XWayland-Move sind live belegt;
die dabei gefundene WM-Mitgliedschaftslücke ist korrigiert und nachgeprüft.
Details: [P06-Endstand](docs/phase-reports/P06_SCHEMA_COMPLETION.md).
Die verbindliche aktuelle Matrix steht im
[P06-Abschlussabgleich](docs/phase-reports/P06_ACCEPTANCE_REVIEW.md).
P07 ist die aktuelle Folgephase; siehe den jüngsten Checkpoint oben. Nachfolgende Checkpoints
bleiben historische Nachweise; ihre damaligen offenen Punkte sind gegen diese
aktuelle Matrix zu lesen.

**P06-Monitorentfernung live bestanden, 26.09.2026:** Nach Neulogin läuft
`e1953fc4…`. Gezielte Raumauswahl bei gleichem globalem Index und der gesamte
physische HDMI-Ab-/Anstecktest mit vier Testfenstern erfolgreich: IDs/Inhalte
erhalten, minimiertes Fenster am verbleibenden Display wiederhergestellt,
zwei Outputs nach Reconnect, vollständige Bereinigung, Raumdaten unverändert.
Dieser Abnahmefall ist erledigt; die vollständige P06-Abnahme bleibt separat.

**P06-Monitorentfernung vom 26.09.2026:** Physischer Test mit normalem,
schwebendem, minimiertem Fenster und Dialog erhält Fenster und Inhalte, deckt
aber eine veraltete Output-Raumauswahl auf. Wiederherstellung des minimierten
Fensters dadurch blockiert. Raumauswahl auch bei gleichem globalem Index
korrigiert; kompletter Live-Nachtest inzwischen wie oben bestanden. Details:
[Monitorentfernung mit Fenstern](docs/phase-reports/P06_OUTPUT_REMOVAL_WINDOWS.md).

**P06-DRM-Nachtest vom 26.09.2026:** Nach Nutzer-Neulogin läuft `e1e21670…`.
Native Fensterbereinigung im Hintergrundraum mit Dialog und minimiertem
Elternfenster auf dem Zweimonitor-DRM-System bestanden; keine verwaisten
Einträge, Raum-/Outputauswahl und Konfiguration unverändert.
Monitorentfernung mit belegten Fenstern inzwischen wie oben bestanden; P06 weiterhin in Arbeit.

**P06-Zweimonitor-/Fehlerrunde vom 26.09.2026:** Unabhängige Raumwahl und
Fensterzuordnung auf eDP/HDMI live geprüft. Defekte/unbekannte Raumdateien,
letzter Raum, konkurrierende IPC-Mutation und Reconnect isoliert bestanden.
Dabei gefundene verwaiste native Fenster im Hintergrundraum werden nun direkt
beim Destroy entfernt; DRM-Nachprüfung inzwischen wie oben bestätigt. USB-Hub auf
Nutzerwunsch abgeschlossen (Verkabelung erklärt den Befund). Details:
[Zweimonitor- und Fehlerfälle](docs/phase-reports/P06_DUAL_OUTPUT_ERRORS.md).

**P06-HDMI-Live-Nachweis vom 26.09.2026:** Mit `8c1d874c…` bestätigt der Nutzer
funktionierendes Hotplug; das Sitzungslog belegt Entfernen/Wiederhinzufügen des
HDMI-Connectors. Monitor-USB-Hub separat offen: bislang keine Kernel-/USB-Erkennung,
Upstream-Verbindung zu klären. Vollständige Zweimonitor-Matrix bleibt offen.

**P06-HDMI-Fortsetzung vom 26.09.2026:** Beide Monitore beim Start vom Nutzer
bestätigt; Hotplug schlägt weiterhin fehl. Logs belegen funktionierende
udev-Erkennung und einen Fehler bei erneuter DRM-Geräteinitialisierung.
Add/Reaktivierung verwenden nun das bestehende Gerät; Hardware-Nachtest offen.
Details: [HDMI-Hotplug](docs/phase-reports/P06_HDMI_HOTPLUG.md).

**P06-HDMI-Befund vom 26.09.2026:** HDMI ist inzwischen angeschlossen und wird
von Linux erkannt, vom laufenden NIWOE aber noch nicht als Ausgang übernommen.
Direkter Linux-Hotplug-Ereignispfad und nutzbare Sitzungsdiagnosen ergänzt;
Zweimonitor-/Hotplug-Abnahme bleibt offen. Details und Aktivierungsstand:
[HDMI-Hotplug](docs/phase-reports/P06_HDMI_HOTPLUG.md).

**P06-XWayland-Prüfung vom 26.09.2026:** Schließen eines X11-Fensters im
inaktiven Raum auf dem DRM-Compositor bestanden, einschließlich Raumrückkehr,
Erhalt des anderen Fensters und unveränderter Raumkonfiguration. Wiederholbarer
Test: `scripts/test-room-xwayland.py`. Zum Prüfzeitpunkt war nur eDP angeschlossen;
zwei physische Outputs und Hotplug blieben offen. P06 ist weiterhin in Arbeit.

**P06-Live-Runde vom 26.09.2026:** Nativer Create/Edit/Reorder/Delete-Ablauf,
Migration normaler/schwebender/minimierter Fenster, Wiederherstellen und
Bereinigung erfolgreich. Ein live nachgewiesener Fehler der Dialog-Raumvererbung
ist korrigiert, isoliert mit Gegenprobe getestet und installiert. Nach dem
Nutzer-Neulogin sind Buildidentität, neutrale Loge, persistente Raumdaten und
Dialogzuordnung auch auf DRM bestätigt; P06 bleibt wegen weiterer Prüfgates in Arbeit.
Die Neun-Räume-Grenze der alten Überlaufliste ist als P07-Lücke bestätigt.
Details: [Live-Lebenszyklus](docs/phase-reports/P06_LIVE_LIFECYCLE.md).

**P06-Bedienpräzisierung vom 26.09.2026:** Auf Nutzerwunsch ersetzt ein eigener
Bereich „Raum löschen“ mit „Fenster verschieben nach …“ und einer aufklappbaren
Raumliste die unklare zyklische Zielwahl. Der Erstklick-Fix läuft inzwischen im
neuen Compositor und wurde mit stationärem Zeiger live nachgeprüft.
Prüfungen, installierte Buildidentität und verbleibende Gates:
[Raumwahl beim Löschen](docs/phase-reports/P06_DELETE_TARGET_MENU.md).

**P06-Fortsetzung vom 26.09.2026:** Die installierte Grundlage läuft inzwischen
nach Neulogin mit bestätigten Buildidentitäten. Create/Delete, Metadaten,
Raumwechsel, native Fenstermigration und Persistenz wurden auf dem DRM-Host
geprüft. Ein dabei gefundener IPC-Reconnect-Race ist behoben. Die vollständige
Konfigurationsseite erhält nun Neuer-Raum-Entwurf, Beschreibung und explizite
Zielwahl mit Löschbestätigung. P06 bleibt in Arbeit; die neue UI benötigt ihren
eigenen Live-Bediennachweis. Details und offene Gates stehen im
[Bericht der sichtbaren P06-Abläufe](docs/phase-reports/P06_VISIBLE_FLOWS.md).

**Verbindliche Präzisierung vom 24.09.2026:** Der
[Mockup- und Workflowplan](docs/MOCKUP_WORKFLOW_PLAN.md) konkretisiert die vier
verbindlichen letzten Bildvorlagen und die End-to-End-Wege Desktop → Hub → Räume verwalten → Raum
konfigurieren → Speichern → Desktop. Seine visuellen Liefergates gelten für
P00–P13 zusätzlich zu den technischen Paketen unten. Bei Widerspruch zwischen
einem früheren Reduktionsvorschlag dieses Plans und der Bildvorlage gilt das
Designmanifest samt ausdrücklicher Nutzerkorrektur, dann der neue Workflowplan.
Insbesondere sind Hub und beide Control-Center-Seiten vollständige Oberflächen;
ein kleiner Raumeditor und eine reine Raumliste sind kein Phasenabschluss.
Das Panel wurde nach der Nutzerklarstellung vom 24.09.2026 visuell als in
Ordnung bestätigt; weitere Panelgestaltung ist kein eigenes Liefergate.
Die abgelehnte P06-Editoroberfläche ist nicht abgenommen.

**Verbindliche Ablaufkorrektur vom 24.09.2026:** Die Panel-Schaltfläche öffnet
direkt den vollständigen Hub aus `17_13_54 (2)`. Alle früheren Anforderungen an
einen separaten Spotlight-/Such-Launcher sind aufgehoben. P04 baut als einen
zusammenhängenden sichtbaren Lieferstand den Hub sowie „Räume verwalten“ aus `(3)`
und „Raum konfigurieren“ aus `(4)`. Die vollständige linke Sidebar gehört zu
beiden Control-Center-Seiten. Sie wird weder durch einen Popup-Editor noch durch
eine vereinfachte Liste ersetzt. P08/P10 binden nach dem Raumdatenmodell weitere
echte Daten und Mutationen an diese bereits vollständigen Oberflächen an.

**Umsetzungscheckpoint vom 24.09.2026:** Der Hub aus `(2)` und die Grundfläche
„Räume verwalten“ aus `(3)` sind installiert und im Dark Theme visuell bestätigt.
Der Hub öffnet genau einmal pro neuer NIWOE-Login-Sitzung als Willkommensansicht;
danach ist `Super+Space` der reguläre Zugang und Tippen startet die Suche. P04
bleibt offen. Als nächster sichtbarer Schritt folgt „Raum konfigurieren“ aus `(4)`
mit dem vollständigen Hin- und Rückweg. Details, Prüfungen, Bildbelege und offene
Grenzen stehen im
[P04 Hub- und Control-Center-Zwischenstand](docs/phase-reports/P04_HUB_FOUNDATION.md).

**P04-Fortschritt vom 25.09.2026:** Die vollständige native Grundfläche
„Raum konfigurieren“ aus `(4)` samt Übergang aus „Räume verwalten“ ist gebaut.
Name und Reihenfolge sind an die vorhandene revisionsgesicherte Raum-IPC
angebunden; weitere Mockup-Bereiche zeigen ihre Fähigkeitsgrenze. Fedora-Tests
und Release-Build sind grün. Der Stand ist installiert und als nativer
Screenshot geprüft; P04 bleibt bis zum vollständigen Bildvergleich und zur
Bedienabnahme offen. Nachweise stehen im verlinkten P04-Bericht.

**Logen-Auftrag vom 25.09.2026:** Der Sitzungsstart erhält vor der Raumwahl
einen neutralen Zustand ohne aktiven Raum. Der einmalige Willkommens-Hub bleibt
sein Einstieg. Compositor und Shell verwenden für diesen Zustand die
IPC-Kennung 0; ein Raumklick aktiviert den gewählten Raum, ein App-Schnellstart
aus der Loge aktiviert Raum 1 als Fensterziel. Der Code und beide Releases sind
gebaut, getestet und installiert. Login-Loge und Raumwahl sind mit nativen
Screenshots bestätigt; die vollständige Bedienabnahme bleibt offen.

**Theme-Entscheidung vom 25.09.2026:** Die Desktop-Alpha hat ein verbindliches
dunkelgrünes NIWOE-Theme. Die bisherige Pflicht zu Dark- und Light-Abnahmen
entfällt in allen nachfolgenden Phasen. Vorhandener Light-Code darf ungenutzt
erhalten bleiben; im Alpha-Endzustand wird keine Farbtheme-Auswahl angeboten.
Kontrast, Lesbarkeit, Zustände, Auflösungen und Skalierung werden weiterhin im
grünen Theme geprüft. Eine helle Variante ist eine spätere Produktentscheidung.
Der Theme-Wähler in der älteren Settings-Oberfläche ist vorhandener Code und
noch kein Nachweis dieser Produktumstellung. Bei der Control-Center-Migration
wird er aus dem Alpha-Ablauf entfernt; bestehende Nutzerkonfigurationen und
externe GTK-/KDE-Einstellungen dürfen dabei nicht still überschrieben werden.

**P06-Zwischenstand vom 25.09.2026:** Dynamische Raum-Mutationen, vollständige
Basis-Metadaten im Snapshot, Laufzeit-Slotpflege und eine seitenweise Darstellung
in „Räume verwalten“ sind implementiert. Die rechte Leiste ist geometrisch
korrigiert. Workspace-Check, Clippy, Tests und Release-Build auf Fedora sind
grün. Compositor und Shell sind installiert; die Shell läuft bereits neu, der
Compositor benötigt noch einen NIWOE-Neulogin. P06 bleibt bis zu den
Live-Integrationsfällen und den sichtbaren Erzeugungs-/Löschabläufen offen.
Details: [P06-Raummodell-Zwischenstand](docs/phase-reports/P06_ROOM_MODEL_FOUNDATION.md).

Stand: 21.09.2026. Grundlage: `NIWOE_Konzeptzusammenfassung.md`, die acht
damals gesichteten PNG-Mockups unter `assets/` und eine gezielte Sichtung der
bestehenden Rust-Architektur. Seit 24.09.2026 sind nur die letzten vier Bilder
visuell verbindlich; die ersten vier bleiben Kontext. Dies ist ein Arbeitsplan,
kein Nachweis abgeschlossener Features.

**Arbeitsweise aktualisiert am 21.09.2026:** Auf ausdrücklichen Nutzerauftrag
übernimmt der aktuelle Agent Implementierung, Tests und Phasenprüfung selbst.
Es gibt keine Übergabe zwischen Modellen und keinen manuellen Reviewstopp am
Phasenende. Die fachlichen Phasen und ihre Prüfgates bleiben bestehen. Ein kurzer
fortgeschriebener Phasenbericht dokumentiert Ergebnis, Prüfungen und offene Risiken.

**Scope-Korrektur auf Nutzerauftrag, 22.09.2026:** Keine weitere Ausbau- oder
Abnahmerunde für die abzulösende untere Taskleiste und den alten kombinierten
Hub/Settings-Container. P02 liefert die wiederverwendbaren Designgrundlagen;
die vollständige visuelle, Scale-/Input- und native Performance-Abnahme erfolgt
an den neuen Oberflächen in P03–P05. Die dokumentierten offenen DRM-/HiDPI-Befunde
bleiben sichtbar, sind aber keine Voraussetzung für den Beginn von P03.
Nächste Implementierung: neue obere Leiste nach Manifest und Desktop-Mockup.
Vorhandene Backends weiterverwenden; keine zusätzliche Alt-UI-Politur.
Pflichtchecks nach tatsächlichen Codeänderungen bleiben bestehen. Bestandene
Prüfungen ohne relevante Änderung oder neuen Befund nicht wiederholen.

**Explizite Nutzerentscheidung vom 21.09.2026:** Der gesamte alte Meridian-Plan
ist obsolet. Zuerst entsteht niwoe-desktop auf einer bestehenden Linux-Distribution,
danach ein eigenes Linux-basiertes OS. Eine erneute BSD-vs.-Linux-Evaluation ist
kein Arbeitspaket. Bestehender Code bleibt nutzbar, alte Roadmap-Verpflichtungen nicht.
Empfohlener Entwicklungshost: Fedora KDE Plasma Desktop, siehe
[Distributionsentscheidung](docs/NIWOE_DEVELOPMENT_DISTRO.md). Der Nutzer hat
Fedora 44 KDE auf dem Acer installiert; der SSH-Testhost ist jetzt verfügbar.

**Neubauentscheidung, korrigiert 24.09.2026:** Panel, Hub und System-Deck werden
als neue Oberflächen mit eigenen Layouts und Eingabemodellen gebaut und an die
vorhandenen Backends angeschlossen. Die alte Shell-Komposition ist keine
Ausbaubasis. P03–P05 bilden gemeinsam diesen Lieferumfang; die Reihenfolge bleibt
Panel, Launcher, Deck, die Integration darf phasenübergreifend erfolgen.
Compositor, IPC, App-Katalog, Systemdienste und zentrale UI-Grundbausteine bleiben.
Zusätzlich ersetzt ein dünner Außenrahmen die compositor-eigene SSD-Titelleiste
auch bei freien Fenstern. Verschieben, Resize und Schließen müssen ohne sie
erreichbar sein. Native CSD wird nicht aus Anwendungen entfernt.

**Bedienentscheidung:** Omarchy ist die konkrete Referenz; verbindliche
Kernbelegung und Migrationskonflikte in
[NIWOE_INTERACTION_MODEL.md](docs/NIWOE_INTERACTION_MODEL.md). Diese Entscheidung
ersetzt widersprechende frühere Shortcutvorschläge, insbesondere Super+Space
für das Deck. Tiling und vollständige Tastaturbedienung sind das Produktziel.

## 1. Ziel und Einordnung

NIWOE wird zuerst ein kohärenter nativer Linux-Desktop auf dem vorhandenen
Compositor. Das eigene Basissystem folgt nach einer brauchbaren Desktop-Alpha.
Der bestehende Rust-/Smithay-Kern bleibt erhalten. Kein Rewrite und kein neuer
WebKit-, Electron- oder Browser-UI-Pfad.

Das überzeugende Produktmerkmal sind benannte, verlässliche Arbeitskontexte.
Grün/Gold unterstützt diese Identität; allein eine neue Farbwelt wäre kein
ausreichendes Produktmerkmal. Hub, Deck und Settings bekommen klar getrennte
Aufgaben. NIWOE baut keine eigene IDE, Notiz-App oder Aufgabenverwaltung.

Die wichtigsten Einschränkungen für die erste Version:

- Ein Raumwechsel zeigt vorhandene Fenster. Er beendet oder startet keine Apps.
- Wiederöffnen von Apps nach einem Neustart ist eine separate, freiwillige Aktion.
- Beliebige Dateizustände, Browser-Tabs und laufende Terminal-Prozesse lassen sich
  nicht aus dem Fensterlayout rekonstruieren. Keine entsprechende UI-Zusage.
- Die einheitliche Softwareverwaltung ist zunächst eine spätere Produktschnittstelle,
  keine Aufforderung, jetzt Paketmanager, Distribution und Updater neu zu bauen.
- Bestehender BSD-Code wird nicht pauschal gelöscht; daraus folgt keine aktive
  BSD-Supportzusage und kein verpflichtendes BSD-Releasegate. Linux ist Produktziel.

## 2. Historische Ausgangssichtung vor P00/P01

Die folgenden technischen Altnamen dokumentieren den Planungsstand. Aktuelle
Pfade und Migrationsnachweise stehen in `docs/phase-reports/P01.md`.

| Bereich | Gesehener Stand | Konsequenz |
|---|---|---|
| Compositor | `crates/meridian-compositor/src/workspace.rs`: neun Spaces, aktiver Index | Vorhandenen Lifecycle erweitern, keinen zweiten Fenstermanager bauen |
| Multi-Monitor | `state/workspace_output_state.rs`: aktiver Workspace pro Output | Raumwahl muss Output-Semantik erhalten |
| WM | `crates/meridian-wm/src/workspace.rs`: Floating/Tiling und Floating-Ausnahmen | Raumzuordnung und Layoutmodus sind verschiedene Eigenschaften |
| IPC | `WindowSnapshotEntry.workspace: u8`, Output-Snapshots, `ShellCommand`/`ShellEvent` | Stabile Raum-ID ergänzen; Indexkonvertierungen bewusst migrieren |
| Shell | Native Panel-, Launcher-, Quick-Settings-, Settings- und Thumbnail-Pfade | Komposition umbauen, bewährte Backends wiederverwenden |
| Design | `meridian-tokens`, `meridian-config`, Design-/Zentralitäts-/600-Zeilen-Guards | Bestehende zentrale Pipeline weiterverwenden |
| Shortcuts | `Super+Space` startet derzeit den Launcher | Konflikt mit Deck-Ziel ausdrücklich migrieren |
| Produktregeln | AGENTS: BSD zuerst, blaues Manifest, native Qualitätsrunde | P00 muss die neue Richtung verbindlich und widerspruchsfrei eintragen |
| README | Beschreibt noch WebKit als aktive Richtung | Inhaltlich korrigieren, bloßes Umbenennen reicht nicht |
| CI | Ubuntu, fmt, clippy, Workspace-Tests | Als Linux-Buildbasis nutzen; expliziten Workspace-Check ergänzen |
| Git | Branch bei Sichtung: `codex/openbsd-native` | Für Umsetzung geeigneten `codex/niwoe-…`-Branch verwenden; keine Historie umschreiben |
| Remotes | `github` zeigt noch auf `quompacc/meridian-desktop`; `origin` auf Codeberg | GitHub-URL aktualisieren; Codeberg-Ziel nicht erfinden |
| Arbeitsbaum | Neue Konzepte/Mockups untracked, elf alte Assets bereits gelöscht | Vorhandene Nutzeränderungen bewahren, gezielt zuordnen |

Die Konzepte im Root und unter `assets/` waren bei Sichtung SHA-256-identisch.
Das Root-Dokument wird die kanonische Quelle; P00 ersetzt das Duplikat durch
einen Verweis oder entfernt es als ausdrücklich dokumentierte Duplikatbereinigung.

Diese Sichtung ist kein vollständiger Code-Audit und keine neue Build-Verifikation.
Vorhandene Statusberichte über bestandene BSD-Tests ersetzen die Linux-Baseline nicht.

## 3. Verbindliche Arbeitsstruktur nach Start der Umsetzung

Lesereihenfolge:

1. `AGENTS.md` und gegebenenfalls zusätzliche Regeln im betroffenen Verzeichnis.
2. Dieser Plan und [Design-Brief](docs/NIWOE_DESIGN_BRIEF.md).
3. [Agentenübergabe](docs/NIWOE_AGENT_HANDOFF.md).
4. Das in P00 aktualisierte Designmanifest und die betroffenen Architekturdateien.

P00 überführt den bereits entschiedenen Richtungswechsel in die aktiven Dateien.
Alte strategische Dokumente dürfen den neuen Nutzerauftrag nicht blockieren.
Technische Schutzregeln wie zentrale Tokens, native UI, Tests und enge
Privilegiengrenzen werden bewusst weitergeführt. Die konkreten Phasenschnitte und
Designwerte dieses Plans sind Umsetzungsvorschläge; die Linux-/NIWOE-Richtung
bedarf keiner erneuten Grundsatzentscheidung.

Jede Phase hat den Status `not-started`, `in-progress`, `implemented`, `accepted`
oder `blocked`. `implemented` bedeutet: ihre Prüfungen sind bestanden und Belege
liegen vor. `accepted` setzt eine eigene abschließende Prüfung des tatsächlichen
Diffs und der Belege voraus. Abhängige Phasen starten erst nach `accepted`; der
Agent führt diese Prüfung selbst durch und arbeitet danach ohne Benutzerfreigabe weiter.
Reine Dokumentation für eine spätere Phase darf unabhängig vorbereitet werden.

## 4. Architekturentscheidungen für die Desktop-Alpha

Diese Entscheidungen sind Umsetzungsdefaults. Wer einen technischen Widerspruch
findet, dokumentiert ihn mit Codebeleg und einem konkreten Lösungsvorschlag.
Keine spontanen Alternativarchitekturen während eines Tickets.

### 4.1 Zuständigkeiten

```text
niwoe-config: gespeicherte Konfiguration, Schema, Validierung, Migration
niwoe-tokens: Farben, Maße, Typografie, Interaktion, Effekte
niwoe-ui: wiederverwendbare native Widgets und Layout
niwoe-compositor: verbindliche Raum-/Fensterzuordnung, Fokus, Output, Stacking
niwoe-wm: bestehende Tiling-/Floating-Mechanik
niwoe-ipc: serialisierbare Commands, Snapshots, Events, Fehler
niwoe-shell: UI, lokale Eingabezustände, Darstellung der Snapshots
kleine vorhandene Services/Helper: privilegierte Systemoperationen
```

Kein generisches Plugin-System und zunächst keine zusätzlichen Crates. Gemeinsame
Raumkonfiguration gehört in das bestehende Config-Crate; Wire-Typen ins IPC-Crate.
Abhängigkeiten müssen azyklisch bleiben. Die Shell führt kein zweites verbindliches
Raummodell mit eigener Fenstereinteilung.

### 4.2 Räume und Outputs

- `RoomId`: persistente opaque ID, unabhängig von Name, Position, Workspace-Index
  und PID. Erzeugung zentral, vorhandene ID-/Zufallswerkzeuge bevorzugen. Ohne
  solche Werkzeuge genügt ein atomar persistierter monotoner ID-Zähler; kein neuer
  UUID-Crate allein für diese Funktion. Gelöschte IDs werden nicht wiederverwendet.
- Gespeicherte Definition: ID, Name, Beschreibung optional, vorhandenes Icon,
  Reihenfolge, Zuordnungsmodus, App-Präferenzen, Layout-/Restore-Einstellungen.
- Flüchtiger Zustand: Fensterzuordnung, Sichtbarkeit pro Output, Fokusverlauf,
  Fensteranzahl. Keine Wayland-Objekte, Prozesshandles oder PIDs persistieren.
- Migration: vorhandene neun Workspace-Slots erhalten einmalig stabile IDs und
  Namen `Raum 1` bis `Raum 9`. Bestehende Belegung und Shortcuts bleiben erhalten.
  Personalisierte neue Räume werden später im Wizard vorgeschlagen.
- Intern bleibt zunächst die bestehende Space-/Indexmechanik. Eine explizite
  ID-zu-Index-Abbildung schützt sie vor Umbenennen und UI-Sortieren.
- Raumwahl gilt für den adressierten, andernfalls fokussierten Output. Sie darf
  nicht beiläufig alle Monitore umschalten. Bestehendes Verhalten, wenn derselbe
  Workspace auf mehreren Outputs sichtbar ist, wird vor Änderung charakterisiert
  und erhalten. Ein Raum bekommt deshalb keinen einzelnen `owner_output`.
- Dynamisches Hinzufügen/Entfernen erweitert vorhandene Spaces und WM-Zustände
  konsistent; keine reine UI-Liste über einem unveränderlichen Neuner-Array.
- Mindestens ein Raum bleibt bestehen. Löschen eines belegten Raums benötigt einen
  expliziten Zielraum; Fenster werden verschoben, niemals geschlossen.
- Raumlisten-Reihenfolge und Speicherslot sind getrennt. Nummernshortcuts folgen
  der angezeigten Reihenfolge; Raumidentität bleibt dabei unverändert.

### 4.3 Zuordnung, Start und Wiederherstellung

`Free` akzeptiert alle Fenster. `Preferred` beeinflusst automatische Erstzuordnung.
`Dedicated` ist in der Alpha eine bevorzugte Aufgabenbindung mit sichtbarem Hinweis,
keine Sicherheitsgrenze und kein Verbot, manuell andere Fenster hineinzuschieben.

Zuordnungspriorität:

1. Dialog/Transient übernimmt den Raum seines Elternfensters.
2. Explizit zugeordnetes Startvorhaben oder manuelles Verschieben.
3. Exakte konfigurierte App-ID-Regel; bei mehreren Treffern die erste Regel in
   gespeicherter Prioritätsreihenfolge.
4. Raum des fokussierten Outputs.

Native `app_id` und XWayland-Klassen müssen getrennt normalisiert werden. Fenster-
titel sind keine belastbare App-Identität. Spät eintreffende Metadaten dürfen ein
bereits manuell verschobenes Fenster nicht wieder zurückziehen.

Starts laufen über den bestehenden App-Katalog/Launch-Pfad. Keine aus Namen oder
Dateipfaden zusammengebauten Shell-Kommandos. Startkorrelation möglichst über die
vorhandenen Aktivierungs-/Launch-Metadaten; nie die falsche Sicherheit anbieten,
dass `app_id` allein mehrere identische Fenster eindeutig unterscheiden könne.

Wiederherstellung kennt getrennte Fähigkeiten:

- `LayoutOnly`: vorhandene, eindeutig zuordenbare Fenster anordnen.
- `RelaunchApps`: ausdrücklich konfigurierte Apps neu starten, best effort.
- Dateien: nur explizit hinterlegte Referenzen an Apps übergeben.
- App-interne Sitzung/Terminals: erst über später spezifizierte Integrationen.

Raumwechsel löst keinen Restore aus. Restore ist zunächst manuell, idempotent,
abbrechbar und zeitlich begrenzt. Mehrdeutige Zuordnung wird angezeigt oder
übersprungen, nicht still auf irgendein Fenster angewendet.

#### Produktentscheidung 27.09.2026: Automatisierung und vorhandene Einstellungen

Der Automatisierungsplatzhalter im Mockup entstand ohne konkret vereinbarten
Anwendungsfall. Ein eigener Automatisierungsbereich entfällt vorerst; sein
Navigationseintrag wurde in P10 entfernt. Die möglichen Anwendungsfälle werden
wie folgt eingeordnet:

| Anwendungsfall | Zuständigkeit und Umfang |
| --- | --- |
| Bestimmte Apps beim Anmelden starten | Gehört fachlich zu Start-Apps. Ein Login-Autostart ist eine mögliche spätere, ausdrücklich zu aktivierende Erweiterung; P09 führt ihn nicht automatisch ein. Die neutrale Loge und der Willkommens-Hub bleiben der Sitzungsbeginn. |
| Arbeitskontext mit Apps, Dateien und Fensteranordnung öffnen | Gehört zur Wiederherstellung in P09 und ihrer Konfiguration in P10. Zunächst ausdrücklich ausgelöst; Dateien nur über hinterlegte Referenzen und unterstützte App-Aufrufe. |
| Beim Raumwechsel Einstellungen ändern, etwa Benachrichtigungen stummschalten | Gehört fachlich zu den jeweiligen Raumeinstellungen. Raumspezifische Benachrichtigungsregeln benötigen vor Umsetzung eine eigene Spezifikation; daraus entsteht kein pauschaler P10-Lieferumfang. |
| Programme oder Skripte zeitgesteuert ausführen | Möglicher späterer Anwendungsfall, aktuell kein vereinbarter Bedarf und kein Alpha-Lieferumfang. |

Ein eigener Bereich wird erst bei konkretem Bedarf für Regeln nach dem Muster
„Wenn X passiert, führe Y aus“ erneut geprüft. Start-Apps, Wiederherstellung und
Benachrichtigungen erhalten ihre Einstellungen im jeweiligen Funktionsbereich;
die Einordnung verspricht keine noch nicht implementierten Integrationen.
Ein normaler Raumwechsel zeigt vorhandene Fenster und startet weder Apps noch
eine Wiederherstellung. Diese Entscheidung dokumentiert Produktumfang und
Planung; sie ist kein Nachweis einer bereits geänderten Oberfläche.

### 4.4 IPC und Persistenz

- Neue typisierte Raum-Commands erhalten Request-ID und Success-/Error-Antwort.
  Für Änderungen gilt: validieren, speichern, Zustand veröffentlichen; bei Fehler
  bleibt der letzte gültige Zustand erhalten.
- Vollständiger Raum-Snapshot mit Revision beim Verbindungsaufbau/Reconnect.
  Events enthalten Revision; bei Lücke oder unbekannter Revision Snapshot neu laden.
- Mutationen tragen erwartete Revision, damit zwei UI-Ansichten keine Änderungen
  still überschreiben. Konflikt als typisierten Fehler melden.
- Bestehende Workspace-Wire-Felder nur über einen ausdrücklich dokumentierten
  Adapter ablösen. Legacy-Nummern bleiben während der Übergangsphase lesbar.
- Private, versionierte Zustandsdateien unter XDG-Config/State; ein verantwortlicher
  Schreiber. Atomare Ersetzung, begrenzte Größe, Validierung und Backup vor Migration.
- Unbekannte neuere Schemaversion: verständlicher Fehler, kein Zurückschreiben
  einer älteren Struktur. Beschädigte Datei aufbewahren, keine stille Datenvernichtung.
- Bestehende IPC-Authentifizierung bleibt erhalten. Eine unprivilegierte Shell
  bekommt durch Raumfunktionen keine neuen Root-Fähigkeiten.

## 5. Phasenübersicht

| Phase | Ergebnis | Umsetzung | Abhängigkeit |
|---|---|---|---|
| P00 | Eindeutige Regeln und Linux-Baseline | Terra; Sol bei Buildblocker | keine |
| P01 | Vollständige NIWOE-Namensmigration | Sol empfohlen | P00 |
| P02 | Zentrales NIWOE-Designsystem | Terra | P01 |
| P03 | Native obere Leiste in neuer Gestaltung | Terra | P02 |
| P04 | Hub und Control-Center-Grundflächen nach drei Mockups | Terra; Sol bei Inputproblemen | P03 |
| P05 | Funktionierendes System Deck | Terra | P04 |
| P06 | Persistente Räume und zuverlässige Fensterzuordnung | Sol | P05 |
| P07 | Raumleiste und App-Zuordnungsregeln | Sol Kern, Terra Darstellung | P06 |
| P08 | Hub mit echten Raum-/Fensterdaten | Terra; Sol bei Lifecycleproblemen | P07 |
| P09 | Begrenztes, ehrliches Restore | Sol | P08 |
| P10 | Raum- und Leistenkonfiguration | Terra | P09 |
| P11 | First-Run-Assistent | Terra | P10 |
| P12 | Desktop-Alpha auf Linux-Hardware | Sol | P11 |
| P13 | Entscheidungsreife OS-Architektur | Sol recherchiert, Astra prüft | P12 |

Dieser neue Plan empfiehlt Panel → Hub → Deck vor großen neuen
Desktopfunktionen. Das ist keine Verpflichtung aus dem abgelösten Meridian-Plan.
Es weicht bewusst von der groben Reihenfolge in Konzept §40 ab: Die Grundbedienung
soll tragfähig sein, bevor Räume und Restore sie erweitern.
P03 verwendet vorhandene Workspace-Daten; P07 ersetzt deren Darstellung durch
vollständige Räume. Keine Fake-Raumdaten in produktiver UI.

## 6. Arbeitspakete und Abnahme

Alle Phasen unterliegen zusätzlich den gemeinsamen Gates in Abschnitt 7.
Ein Ticket ist eine kleine zusammenhängende Änderung; kein einzelner großer
Commit für eine komplette mehrteilige Phase.

### P00 — Richtungswechsel und reproduzierbarer Ausgangspunkt

**Ergebnis:** Das nächste Modell findet genau einen aktiven Produktpfad.

1. P00-01: Arbeitsbaum und Basis-Commit erfassen. Nutzer-Assets/-Löschungen in
   Phasebericht festhalten; weder reset noch ungefragtes Stash noch pauschales `git add .`.
2. P00-02: `AGENTS.md`, `CLAUDE.md`, `README.md`, `ROADMAP.md`, `PLAN.md`,
   `NIWOE_OS_PLAN.md`, `docs/NATIVE_UI_PLAN.md`, `docs/UI_PLATFORM.md` prüfen.
   Linux als Produktziel, natives Rust als UI-Pfad, dieses Phasenmodell als aktive
   Reihenfolge eintragen. Alle alten Meridian-Roadmaps als abgelöste Historie
   markieren; keine unerledigten Altphasen als NIWOE-Pflicht übernehmen.
   Auch Plattformmatrix in `docs/TESTING.md` und Hardware-Smoke-Dokumentation anpassen.
3. P00-03: Mockup-Katalog aus Design-Brief übernehmen; Konzeptduplikat bereinigen.
   Fehlende Launcher-/Wizard-Referenzen ausdrücklich kennzeichnen. Für Light
   liegt keine verbindliche Mockupvorlage und seit 25.09.2026 kein Alpha-Gate vor.
4. P00-04: Im alten Manifest die neue Designrichtung explizit festhalten und den
   Brief als Präzisierung referenzieren. Zentralität, Theme-Entscheidung und Effizienz
   beibehalten; alte Blau-Palette und Kompassbindung nicht weiter normativ lassen.
5. P00-05: Linux-Testumgebung nach bestehendem CI-/Installpfad reproduzieren.
   Distribution, Toolchain, Systempakete, Hardware und verfügbare Runtime erfassen.
   Windows dient der Bearbeitung; ein Windows-Build gilt nicht als Linux-Nachweis.
   Bei Fedora-Wahl `scripts/install-deps.sh` um einen getesteten dnf-Pfad ergänzen:
   Build-/Runtime-/Hardwarepakete getrennt, Paketnamen aus aktuellen Fedora-Repos
   prüfen, keine geratenen Installlisten. NIWOE-Sitzung zusätzlich zur vorhandenen
   Desktop-Sitzung einrichten. SELinux und sessionabhängige Portal-/Polkit-Aktivierung
   korrekt integrieren, nicht global deaktivieren. Bestehende Ubuntu-CI beibehalten.
6. P00-06: Unveränderte Codebasis mit den Gates prüfen, vorhandene Fehler benennen.
   Kleine notwendige Baseline-Fixes separat halten und vollständig testen.

**Abnahme:** Keine widersprüchliche aktive WebKit-/BSD-first-Anweisung; Linux-Check,
Tests und Lints bestanden. Nested-Session startet, Shell verbindet sich, ein Client
öffnet. Hardwaremessung vorbereiten. Ohne Linux-Testhost ist P00 blockiert, nicht grün.

**Reviewfokus:** Sind die neue Richtung und der vorhandene technische Schutz klar
getrennt? Wurden Testfehler statt Regeln repariert? Noch keine neuen Desktopfeatures.

### P01 — Meridian vollständig nach NIWOE migrieren

**Ergebnis:** Produkt, Build und installierte Sitzung benutzen konsistent NIWOE.

**Betroffen:** Root/alle eigenen Crate-Manifeste, `src/`, `crates/`, `packaging/`,
`scripts/`, `.github/`, `.githooks/`, aktive Dokumentation, Config-/IPC-Namen.

1. P01-01: Case-insensitive Inventar aller `meridian`-Namen, Pfade und externen
   Kennungen erstellen. Produktcode, Migration, historische Quellen und fremde
   Provenienz getrennt ausweisen. Verzeichnis-/Dateinamen zusätzlich durchsuchen.
2. P01-02: `meridian` → `niwoe`, `meridian-*` → `niwoe-*`,
   Rust-Imports `meridian_*` → `niwoe_*`, eigene Typpräfixe `Meridian` → `Niwoe`.
   Workspace-Members, Pfadabhängigkeiten, Binärnamen und Cargo.lock gemeinsam
   aktualisieren. Lockfile mit Cargo erzeugen, keine Versionsupdates dabei.
   Die beauftragte Umbenennung umfasst dafür notwendige Cargo.toml-Änderungen;
   neue Dependencies sind damit nicht pauschal erlaubt.
3. P01-03: Session-/Desktop-Dateien, Exec-/TryExec-Pfade, systemd/rc-Skripte,
   PAM-Dienstnamen, Polkit-, D-Bus-/Portal-Backendnamen, Helperpfade, Installskripte,
   Umgebungsvariablen, Socketname und Themegenerator gemeinsam migrieren.
   Standardisierte `org.freedesktop.*`-Interfaces und fremde Copyrights erhalten.
4. P01-04: XDG-Datenmigration: NIWOE-Datei gewinnt. Existiert nur Meridian, sichern,
   validieren und einmalig kopieren. Niemals beide Konfigurationen still vereinigen.
   Alte Dateien zunächst erhalten; Wiederholung darf nichts überschreiben.
   Für Environment-Overrides explizite Priorität NIWOE vor Legacy dokumentieren.
5. P01-05: Live-IPC nur über `niwoe.sock`; Altprozesse per sauberem Session-Neustart
   ablösen. Keine ungeschützte Doppel-Socket-Lösung. Installer erkennt alte Dienste
   und erklärt/koordiniert deren Ablösung, statt zwei Login-/Portalinstanzen zu starten.
6. P01-06: `docs/meridian_design_manifest.md` → `docs/niwoe_design_manifest.md`,
   `MERIDIAN_OS_PLAN.md` → `NIWOE_OS_PLAN.md`; alle aktiven Links/Testpfade anpassen.
   Release-Anzeigename ebenfalls NIWOE. Historische Dateien eindeutig archivieren.
7. P01-07: Lokales GitHub-Remote `github` auf
   `git@github.com:quompacc/niwoe-desktop.git` setzen und mit `git ls-remote github`
   prüfen. Das GitHub-Repository ist laut Nutzer bereits umbenannt. `origin` ist
   Codeberg: alten gültigen Mirror vorerst behalten und als externen Restpunkt
   dokumentieren; weder dort Umbenennung noch Remote-Löschung unterstellen.
8. P01-08: Restfundliste abarbeiten. Nur Migrationsfixtures, ausdrücklich historische
   Dokumente, unveränderte Originalmockups und erforderliche Provenienz dürfen
   Meridian enthalten. Jede Ausnahme mit Pfad und Grund, keine globale Ausnahme.

**Abnahme:** Metadaten und Build referenzieren nur neue eigene Crate-Pfade; frische
Installation und Migration einer synthetischen Altinstallation funktionieren;
Login, Lock, Shell-IPC, Portal und Polkit finden ihre neuen Gegenstellen. Jeweils
Neustart/Reconnect und fehlender Helper getestet. Auth-Fehler bleiben fail-closed.

**Reviewfokus:** Deployment-/Konfigurationsbruch, verlorene Daten, ungeprüfte
Altnamen in Tests/Guards, Sicherheitsabschwächung durch Kompatibilitätscode.
Ein grüner Compiler allein nimmt die Namensmigration nicht ab.

### P02 — Designsystem statt lokaler Farbkorrekturen

**Betroffen nach P01:** `niwoe-tokens/src/{color,typography,font,radius,elevation,interaction,chrome}.rs`,
`niwoe-config/src/theme/`, `niwoe-ui`, Manifest und `docs/design/`.

1. P02-01: Semantische Rollen und die konkrete dunkelgrüne Palette aus dem Brief
   in vorhandene Tokenstrukturen überführen. Keine parallele Palette einführen.
   Die bereits implementierte Light-Palette ist historischer Bestand.
2. P02-02: Abstände, Komponentenhöhen, Fokusrahmen und Schrifthierarchie zentral
   definieren. Neue Tokenmodule dürfen große vorhandene Dateien entlasten.
3. P02-03: Komponentenblatt mit Surface, Text, Button, Tab, Chip, Karte, Eingabe,
   Slider und Zuständen normal/hover/focus/pressed/disabled/error erzeugen.
   Vorhandene Render-/Testwerkzeuge bevorzugen; kein neuer UI-Testframeworkbau.
4. P02-04: Das grüne Theme aus denselben nativen Widgets rendern. Schriftfallback
   und fehlende Glyphen prüfen; Serif nur für größere Überschriften, Sans für
   Bedienelemente.
5. P02-05: Kontrastpaare, 100/150/200%-Skalierung und helles/dunkles Wallpaper
   prüfen. Messwerte und Screenshots ablegen. Aus einem PNG keine Animation ableiten.

**Abgrenzung seit 22.09.:** P02-05 betrifft die gemeinsamen Komponenten. Die
vollständige Screenshotmatrix und Bedienprüfung des alten Desktops entfällt.
Output-Skalierung und Rasterqualität werden mit den neuen P03–P05-Oberflächen
integriert geprüft; dafür keine zweite Testreihe am ersetzten Layout starten.

**Abnahme:** Keine neue lokale Designkonstante außerhalb zentraler Tokens;
Komponentenblatt des grünen Themes lesbar.
Kontrastziele des Briefs erreicht. Keine neue dauerhafte Render-/Timerlast.

**Reviewfokus:** Designidentität und Light-Parität, zentrale Werte, Lesbarkeit,
keine versteckten Ausnahmen zur Umgehung der Guards.

### P03 — Obere Leiste als stabile native Grundlage

**Betroffen:** `niwoe-shell/src/panel*.rs`, `panel_view/`, `wayland/init/`,
`wayland/handlers/output.rs`, `wayland/render/`, zentrale Panel-Tokens.

1. P03-01: Panel oben über nutzbare Outputbreite verankern. Exclusive Zone, Popover-
   Anker, Hitboxes und verfügbaren Fensterbereich gemeinsam korrigieren.
2. P03-02: Layout Hub-Zugang und stabile Raumfolge links, Uhr exakt in der
   Outputmitte, Suche/Statusmodule rechts. Alle gespeicherten Räume bleiben
   über eine beschriftete Raumaktion erreichbar; Aktivierung verschiebt die
   sichtbare Folge nicht automatisch. Bestehende Fenster-/Tray-Zugänge solange
   erhalten, bis P07/P08 einen vollständigen Ersatz für minimierte Fenster und
   Fensterwahl bieten.
3. P03-03: Netzwerk, Audio, Akku und Uhr an vorhandene Daten anschließen.
   Kein Akku auf Geräten ohne Akku; keine erfundenen CPU-/GPU-Werte.
4. P03-04: Schmale Displays, lange Texte und Overflow bedienen. Zuerst
   optionale Statusmodule verdichten; danach Raumlabels kürzen und vollständige
   Namen im Raumüberlauf zugänglich halten. Uhr/Hub-Zugang bleiben erreichbar.
5. P03-05: Popup-Positionen nach Monitorwechsel, Scale-Wechsel und Hotplug testen.
   Uhr ohne Sekunden aktualisiert nur zum Minutenwechsel, nicht pro Frame.

**Abnahme:** Kein überdeckter App-Inhalt, keine veraltete untere Reserved Zone,
korrekte Eingabe auf zwei Outputs, Tab-/Fokusdarstellung im grünen Theme geprüft.

**Reviewfokus:** Layer-Shell-Geometrie, Renderreihenfolge, CPU im Idle; keine neue
Raumlogik in UI-Dateien vor P06.

### P04 — Hub und Control Center als zusammenhängender sichtbarer Workflow

**Betroffen:** neue kleine `niwoe-shell/src/hub/`-Module, bestehender
Launcher-/Layer-Surface-Pfad als Migrationsbasis, `wayland/handlers/keyboard.rs`,
`niwoe-config/src/keybind/` und Compositor-Inputpfad.

1. P04-01: Die linke neutrale Panel-Schaltfläche öffnet auf dem fokussierten
   Output das große zentrierte Hub-Overlay aus `assets/…17_13_54 (2).png`.
   Keine separate Spotlight-, App-Raster- oder Startmenü-Oberfläche behalten.
2. P04-02: Vollständige Hub-Komposition bauen: atmosphärischer Bildkopf,
   Titel/Untertitel, Schließen, vier Raumkarten sowie die drei unteren Bereiche
   „Zuletzt aktiv“, „Schnellhilfe“ und „Systemzustand“. Die Schnellhilfe zeigt
   Tippen, Pfeile, Enter und Escape als tatsächlich verfügbare Hub-Bedienung.
   Vor P08 verwenden Raumkarten vorhandene echte Workspace-/Snapshotdaten;
   fehlende Provider zeigen einen klaren Fähigkeitszustand in ihrem vorgesehenen
   Bereich.
3. P04-03: „Räume verwalten“ als vollständige Seite aus Bild `(3)` bauen:
   linke Sidebar, Bildkopf, Filter/Sortierung/Suche, Neuer Raum, Kartenraster,
   rechte Schnellaktionen und Statistik. Vorhandene echte Daten verwenden;
   noch fehlende Mutationen als klaren Fähigkeitszustand darstellen.
4. P04-04: „Raum konfigurieren“ als vollständige Seite aus Bild `(4)` bauen:
   Sidebar, Breadcrumb, Reiter, Details, Kontext/Wiederherstellung, Start-Apps,
   Regeln, Vorschau, Erklärung sowie Abbrechen/Speichern. Vorhandenes Rename/
   Reorder-Backend anbinden; noch fehlende Fähigkeiten nicht vortäuschen.
5. P04-05: Hubnavigation mit Tab/Pfeilen/Enter/Escape und Maus. Raumkarte wechselt
   den Raum; die Verwaltungsaktion öffnet `(3)`, eine Karte dort öffnet `(4)`,
   Breadcrumb/Zurück führen ohne Zustandsverlust zurück. Escape schließt den Hub
   beziehungsweise führt aus dem Control Center eine Ebene zurück.
6. P04-06: Bestehende Öffnungsbelegung auf den Hub umstellen. Super allein löst
   keine Aktion aus; Super+Escape öffnet das Deck, Super+Tab wechselt den Raum.
   Benutzerdefinierte Bindings erhalten und Konflikte verständlich anzeigen.
7. P04-07: Gemeinsamen Overlay-Lifecycle für Hub, Control Center und Deck festlegen: höchstens
   ein Hauptoverlay, Lock schließt es, Reconnect hinterlässt keine unsichtbare
   keyboard-exclusive Fläche. Bilder, Icons und statische Vorschauen werden nach
   Theme/Scale/Identität gecacht und nur bei Änderung invalidiert.

**Abnahme:** Native Ausgabe entspricht Aufbau und Hierarchie der Bilder `(2)`,
`(3)` und `(4)` im grünen Theme. Zwanzig Öffnen/Navigieren/Raumwechsel/
Verwalten/Konfigurieren/Zurück/Schließen-Zyklen ohne Fokusverlust. Keine fehlenden
Bildbereiche werden durch ein kleineres Suchpopup oder einen Mini-Editor ersetzt.

**Reviewfokus:** Bildtreue, Overlay-/Fokus-Lifecycle, gecachte Assets und echte
Daten/Fähigkeitszustände.

### P05 — System Deck auf vorhandenen Systembackends

**Betroffen:** `quick_settings_popup.rs`, `audio/`, `network/`, `bluetooth.rs`,
`battery.rs`, `power_profile.rs`, `system_power.rs`, Popup-Lifecycle.

1. P05-01: Kleine rechts oben verankerte Karte nach Desktop-Mockup umsetzen.
2. P05-02: Audio/Mute, Netzwerk, Bluetooth, verfügbare Helligkeit/Power-Funktionen
   und Link zu Settings über bestehende Fähigkeiten einbinden.
3. P05-03: Pending-/Fehler-/Unavailable-Zustände anzeigen. Ein Toggle darf Erfolg
   erst nach Backendbestätigung darstellen; Berechtigungen bleiben beim Helper.
4. P05-04: Power-Aktionen behalten vorhandene Bestätigung. Fehlende Hardware oder
   Provider erzeugen keine toten Attrappen. DND nur anbieten, wenn durchgesetzt.
   Nutzerwunsch vom 25.09.2026: Das Systemdeck erhält einen sichtbar
   beschrifteten **Abmelden**-Button. Er nutzt den vorhandenen Logout-Pfad
   über die Compositor-IPC und wird im Deck-Bedienlauf geprüft.
   Zwischenstand 25.09.2026: Button und Raumzeile sind nativ gebaut und
   geprüft. Der Shell-Release ist auf Fedora installiert und läuft bytegleich;
   die Live-Bild- und Bedienabnahme steht noch aus, siehe
   [P05-Systemdeck-Bericht](docs/phase-reports/P05_SYSTEM_DECK_LOGOUT.md).
5. P05-05: Teure Statusabfragen nur bei Sichtbarkeit/Änderung; keine neue Polling-
   Schleife für geschlossene Overlays. Kontextabhängige automatische Sortierung später.

**Abnahme:** Tatsächliche Lautstärke-/Mute-Änderung; Netzwerkzustand/Fehler;
Bluetooth ohne Adapter; Escape und Fokus im grünen Theme. Für noch fehlende Provider
klarer Capability-Zustand statt vorgetäuschter Funktion.

**Reviewfokus:** Verbindungen UI → IPC/Provider → bestätigter Zustand;
keine zusätzlichen Privilegien. Native Qualitätsrunde jetzt abgeschlossen.

### P06 — Räume als konsistenter Compositor-Zustand

**Betroffen:** `niwoe-config` neue `rooms`-Module, `niwoe-ipc` neue Raumtypen,
Compositor `workspace.rs`, `state/workspace_output_state.rs`, `state/layout/`,
`state/ipc/`, vorhandene Workspace-Tests; WM-Zustände.

1. P06-01: Typen/Validierung/Schema nach §4 implementieren. Name nicht leer,
   maximal 64 Zeichen; Beschreibung maximal 200; eindeutige IDs; gültige Verweise.
   Schutzlimit Alpha: 64 Räume, zentral als fachliche Grenze definiert.
2. P06-02: Migration der neun bestehenden Slots und atomare Persistenz implementieren.
   Eine Wiederholung erzeugt keine neuen IDs und ändert keine Raumreihenfolge.
3. P06-03: ID-/Space-Abbildung, Erzeugen, Umbenennen, Umsortieren und Löschen
   konsistent durchziehen. Bei Entfernen Indizes, Outputzuordnung und WM-Zustände
   gemeinsam korrigieren. Laufende Fenster behalten Identität und Inhalt.
4. P06-04: Typisierte Commands/Snapshots/Acks und Revisionen implementieren.
   IPC-Dateien vor Überschreiten von 600 Zeilen nach Verantwortung aufteilen.
5. P06-05: Raumwechsel und explizites Fensterverschieben über bestehende Pfade
   anbinden. Fokusverlauf, minimierte Fenster, Floating, Tiling und Dialoge beachten.
6. P06-06: Shell als Snapshot-Consumer anbinden; Reconnect stellt vollständige
   Daten wieder her. Zunächst einfache vorhandene Raum-/Workspace-Ansicht verwenden.

**Abnahmefälle:** Umbenennen bei geöffneten Fenstern; Reihenfolge ändern; Raum mit
Fenstern in Zielraum löschen; letzten Raum löschen abweisen; zwei Outputs; Output
entfernen; Dialog erbt Raum; XWayland-Fenster im inaktiven Raum schließen; defekte
Datei; neues unbekanntes Schema; Reconnect und konkurrierende Mutation.

**Reviewfokus:** Eine einzige Wahrheit, keine stale Indizes, keine verlorenen Fenster,
keine ungeprüfte IPC-Eingabe oder still überschriebenen Konfigurationen.

### P07 — Raumleiste, Navigation und Preferred/Dedicated

**Status: `accepted`, 26.09.2026.** Nachweise und ausdrücklich begrenzter
Abnahmeumfang: [P07-Abschlussabgleich](docs/phase-reports/P07_ACCEPTANCE_REVIEW.md).

**Betroffen:** Shell `panel_view/`, `workspaces.rs`, Config-Raumregeln,
Compositor-Fenster-Lifecycle und `state/ipc/launch.rs`, App-Katalog.

1. P07-01: IDs/Namen und aktive Auswahl aus echten Snapshots rendern. Gold nur
   für aktiven Zustand, neutraler Belegungsindikator, semantisch getrennte Warnung.
2. P07-02: Overflow für mehr Räume, lange Namen kürzen mit erreichbarem Vollnamen;
   aktiven Raum sichtbar halten. Kein unvorhersehbares Umsortieren durch MRU.
3. P07-03: Nummernshortcuts auf Raumreihenfolge und `Super+Shift+Zahl` auf
   Verschieben abbilden; Kontextmenü bietet gleichwertige Mausaktionen.
4. P07-04: App-Regeln nach §4.3 als reine testbare Entscheidungsfunktion einführen;
   Native/XWayland, mehrere Treffer, fehlende App-ID und spätes App-ID testen.
5. P07-05: Fensterzugang im Raum sicherstellen, einschließlich minimierter Fenster
   und mehrerer Fenster einer App. Erst dann alte Fenster-Taskliste als Default
   entfernen. Tray/Benachrichtigungen bleiben erreichbar.

**Abnahme:** Start in Preferred-Raum ohne Fokusdiebstahl; manueller Move bleibt
erhalten; Dedicated sperrt niemanden aus; 1/9/64 Räume; zwei Monitore; alle
Fenster weiterhin auffindbar. Raumwechsel startet keine Programme.

**Reviewfokus:** Startkorrelation und Transients, Alltag ohne klassische Taskliste,
keine zweite Policy in der Shell.

### P08 — Hub vollständig an Raum- und Fensterdaten anbinden

**Betroffen:** die P04-Hubmodule, vorhandene Thumbnail-/IPC-/Overlaypfade und
Raum-/Fenstersnapshots. **Bildgate:** Die in P04 vollständig gebaute Oberfläche
bleibt geometrisch unverändert und erhält echte persistente Daten. Eine kompakte
Raumkartenliste oder ein Suchpopup ist kein Zwischen- oder Endersatz.

1. P08-01: Karten an persistente IDs, Namen, Beschreibungen, Reihenfolge,
   aktive/belegte Zustände, App-Icons, Fensteranzahl und Auswahl anbinden.
2. P08-02: Pfeile/Tab/Enter/Escape, Scrollen und Filter nach Raumname; Klick auf
   Raum wechselt, Klick auf Fenster aktiviert das konkrete Fenster.
3. P08-03: Bestehende Thumbnail-Capture-Fähigkeit wiederverwenden; Alpha statische
   Vorschau bei Öffnung, maximal eine pro sichtbarer Karte, begrenzter Cache.
   Während geschlossen keine Captures. Ohne Bild steht ein sinnvoller Platzhalter.
4. P08-04: Captureantworten nur für aktuelle Hub-Generation akzeptieren;
   Theme-/Scale-/Fensterende invalidieren; Lock schließt und verwirft Vorschauen.
5. P08-05: Suche innerhalb der vorgesehenen Hub-/Control-Center-Struktur an
   Raum- und Fensterdaten anbinden. Kein zweiter Suchindex und keine globalen
   Dateiscans.

**Abnahme:** Hub mit leeren/vollen Räumen und 64 Einträgen bedienbar;
Fenster während offener Vorschau schließen; Output wechseln; wiederholt öffnen
ohne wachsenden Cache; Lock zeigt keine alten Vorschaubilder. Keine erfundenen
Tasks-, Datei-, Synchronisations- oder Prozesszahlen aus Mockups.

**Reviewfokus:** Orientierung, Tastatur, Grenzen der Bildaufnahme und Cache-Lebensdauer.

### P09 — Layout speichern und best effort wieder öffnen

**Status: accepted, 27.09.2026.** P09-01 bis P09-06 implementiert, getestet,
auf Fedora installiert und praktisch geprüft; Nachweise und Messgrenzen im
[P09-Bericht](docs/phase-reports/P09.md). Kein automatischer Login-/Raumwechsel-Restore.

**Betroffen:** Config-/State-Persistenz, Compositor-Layout, vorhandener Launchpfad,
Raum-IPC und kleine Restore-Module; keine allgemeine Prozess-Checkpoint-Engine.

1. P09-01: Versionierten Snapshot definieren: Raum-ID, Layoutmodus, Layoutbaum,
   logische Floating-Geometrie, Outputhinweis, App-Referenzen, optionale explizite
   Dateiverweise. Live-Fenster-IDs nur sitzungsgebunden, nicht als Reboot-Identität.
2. P09-02: Speichern ausdrücklich/manuell sowie entprellt nach relevanter Änderung;
   keine Platte je Mausbewegung/Frame. Privater State, keine Screenshotpersistenz.
3. P09-03: `LayoutOnly` für vorhandene eindeutig zuordenbare Fenster umsetzen;
   Output entfernt/Scale geändert → erreichbare Arbeitsfläche und Mindestgrößen.
4. P09-04: Opt-in `RelaunchApps` mit Startliste, Begrenzung paralleler Starts,
   Deadline, Abbruch und sichtbarem Teilergebnis. Bereits erfüllte Einträge nicht
   doppelt starten. Fehlende/deinstallierte App überspringen und melden.
5. P09-05: Ambige Mehrfenster-Apps und Single-Instance-Weiterleitung explizit behandeln.
   Keine Zuordnung allein nach Startreihenfolge und keine dauerhaften Retry-Loops.
6. P09-06: UI erläutert die Fähigkeiten: Layout, Apps, explizite Dateien. Terminal-
   Prozessfortsetzung und Browser-Sitzungen ohne Integration nicht versprechen.

**Abnahme:** Reboot-Neustarttest, zweite Restore-Ausführung ohne Duplikate,
fehlende App/Datei, zwei Fenster gleicher App, ungültige Geometrie, Monitorwechsel,
Abbruch nach Teilstart, Schreibfehler. Kein Start als Nebeneffekt eines Raumwechsels.

**Reviewfokus:** Datenmodell, sichere Launchargumente, fehlertolerante Zuordnung,
ehrliche UI und keine unbemerkte Wiederholung externer Aktionen.

### P10 — Control-Center-Funktionen und Persistenz vervollständigen

**Status 27.09.2026:** P10-01 bis P10-07 `accepted` für den im
[P10-Bericht](docs/phase-reports/P10.md) belegten Fedora-Umfang. Vollständige
Maus-/Tastaturabnahme und Persistenz einschließlich Konflikt-/Fehlerfällen;
Endrelease aktiv. Mess-/Hardwaregrenzen stehen ausdrücklich im Bericht.

**Betroffen:** `niwoe-shell/src/settings_view/`, bestehende Widgets,
Config-/Raum-Commands, Manifest-Komponenten.
**Bildgate:** Die vollständigen, bereits in P04 gebauten Seiten aus `(3)` und `(4)`
bleiben geometrisch erhalten. P10 vervollständigt ihre echten Datenquellen,
Mutationen und Persistenz. Ein einzelner Popup-Editor erfüllt P10 nicht.

1. P10-01: Raumliste mit Suche, Erzeugen, Name/Beschreibung/Icon, Reihenfolge,
   belegtem Zustand und sicherem Löschen erstellen.
2. P10-02: Formular mit lokalem Entwurf und Speichern/Abbrechen; Validierungsfehler
   direkt am Feld. Erfolgsanzeige erst nach bestätigter Persistenz.
3. P10-03: Free/Preferred/Dedicated, App-Präferenzen und Restorefähigkeiten
   konfigurieren. Nicht implementierte Integrationen nicht als aktive Toggles zeigen.
4. P10-04: Leistenmodule ein-/ausblenden und anordnen. Hub-Zugang/Raumzugang/Uhr
   bleiben erreichbar; CPU/GPU/Sensoren nur bei realem Provider.
5. P10-05: Vorschau aus denselben Komponenten/Tokens rendern; kein zweites
   Designsystem. Vorschauänderung wird bei Abbruch zurückgenommen.
6. P10-06: Den alten Farbtheme-Wähler aus der aktiven Settings-Navigation
   entfernen. Cursor-, Wallpaper- und andere eigenständige Einstellungen
   erhalten. Vorhandene Nutzerkonfigurationen und fremde GTK-/KDE-Preferences
   beim Umbau nicht überschreiben.
7. P10-07: Den Navigationseintrag „Automatisierung“ gemäß Produktentscheidung
   vom 27.09.2026 entfernen. Start-Apps und Wiederherstellung in ihren jeweiligen
   Einstellungen belassen; keine allgemeine Regel- oder Skriptoberfläche ergänzen.

**Abnahme:** Ungültiger Name, doppelte ID aus Eingabe, verschwundene App,
konkurrierende Änderung, Speichern schlägt fehl, Abbruch und Neustart im grünen Theme.

**Reviewfokus:** Model/View-Grenze, echte Persistenz, kein unbeauftragtes
Automatisierungs-, Backup-, Aufgaben- oder Dateimanagerprojekt.

### P11 — First Run als kurze Einführung

Abnahme: **accepted**, 27.09.2026; Umsetzung und belegte Grenzen im
[P11-Bericht](docs/phase-reports/P11.md). P12-Abnahme siehe Folgeabschnitt.

**Betroffen:** kleine neue Shell-Wizard-Module, vorhandene Settings-/Room-API,
benutzerbezogener versionierter First-Run-State.

1. P11-01: Willkommen → Bedienprofil → Räume → Leiste → Fertig.
   Defaults brauchbar, jeder optionale Schritt überspringbar.
2. P11-02: Bedienprofile setzen ausschließlich Defaults; keine verschiedene
   Architektur für Maus/Tastatur. Bestehende Nutzeranpassungen erhalten.
3. P11-03: Raumvorschläge und Apps nur aus tatsächlichem Katalog; keine fehlenden
   Apps automatisch installieren. Änderungen bis Abschluss als Entwurf sammeln.
4. P11-04: Drei Shortcuts und Mausäquivalente in einfacher interaktiver Erklärung.
5. P11-05: Abschlussmarker erst nach erfolgreichem Speichern; Crash/Abbruch kann
   fortgesetzt werden, ohne Räume zu duplizieren. Erneuter Aufruf in Settings.

**Abnahme:** Frisches Profil, bestehendes migriertes Profil, Überspringen,
Abbruch/Neustart, Nur-Tastatur und kleiner Bildschirm im grünen Theme.
Keine fingierten Datenschutz-/Updateeinstellungen vor einem realen OS-Unterbau.

**Reviewfokus:** Onboarding erklärt vorhandene Funktionen und verändert kein
etabliertes Nutzerprofil ungefragt.

### P12 — Linux-Desktop-Alpha stabilisieren

**Status: `accepted`, 27.09.2026.** P12-01 bis P12-05 auf realer Fedora-/Intel-
Hardware belegt. Maßgeblicher Teststand, vollständige Pflichtmatrix, rohe
Messnachweise und ausdrücklich ungemessene Metriken: [P12](docs/phase-reports/P12.md).

1. P12-01: Reproduzierbare Installation/Deinstallation auf der gewählten
   Entwicklungsdistribution dokumentieren; NIWOE als eigene Session testen.
2. P12-02: Wayland- und XWayland-Apps, Tiling/Floating, CSD/SSD, Vollbild,
   Dialoge, minimierte Fenster, Clipboard und Input durchtesten.
3. P12-03: Login/Lock/Unlock, Shellabsturz, IPC-Reconnect, Portalzustimmung und
   Polkit testen. Fehler dürfen keine gesperrte Sitzung sichtbar/bedienbar machen.
4. P12-04: Reale Intel-Hardware, zwei Outputs/Hotplug, 100/150/200%, Suspend/
   Resume und Audio-/Netzwerkwechsel dokumentieren. VM-Ergebnisse separat halten.
5. P12-05: Wiederholbare Performance-Messung und Vorher/Nachher-Vergleich aus §7.
   Blocker beheben, Dokumentation/Versionshinweise auf tatsächlichen Stand bringen.

**Abnahme:** Desktop täglich für einen zusammenhängenden Arbeitsablauf nutzbar;
keine offenen Datenverlust-, Lock-, Fokus- oder unerreichbare-Fenster-Blocker.
Bekannte Hardwaregrenzen konkret mit Testgerät und Reproduktionsschritten.

**Reviewfokus:** Gesamtsystem statt einzelne Screenshots. Erst hier ist die
Desktopgrundlage ausreichend, um Anforderungen an ein eigenes OS abzuleiten.

### P13 — OS-Architektur entscheiden, noch keinen Paketmanager bauen

**Ergebnis:** Entscheidungsdokument `docs/os/BASE_SYSTEM_ADR.md` und gesonderter
ausführbarer OS-Plan. Diese Phase ist Recherche/Prototypbewertung, kein pauschaler
Auftrag für Installer, Partitionierung oder Systemupdates auf dem Arbeitsgerät.

1. Anforderungen aus P12 zusammentragen: Kernel/Mesa/glibc, Seat/Session, Audio,
   Netzwerk, Bluetooth, Auth/Polkit/Portal, Spiele und Hardwarematrix.
2. Höchstens drei tatsächlich verfügbare Linux-Basis-/Image-/Updateansätze anhand
   aktueller Primärdokumentation vergleichen. Ein bestehender Mechanismus ist
   bevorzugt, solange er die Anforderungen erfüllt.
3. Getrennte Entscheidungen: versioniertes Base-Image, beschreibbare Konfiguration/
   Nutzerdaten, App-Backend und dessen Runtime, lokale Entwicklungswerkzeuge.
4. Eine sichtbare `niwoe install/update/remove`-Schnittstelle auf einen definierten
   Katalog planen. Quellenvielfalt bedeutet nicht automatisch beliebige Installer
   ausführen. App-IDs, Herkunft, Signaturen, Abhängigkeiten und Ownership festlegen.
5. Transaktion/Recovery planen: unterbrochener Download, Signaturfehler, volle
   Platte, Stromverlust, fehlgeschlagener Boot, Rückkehr zur letzten Version,
   Daten-/Schemasicherheit bei Rollback. Steam-eigene Spieldaten getrennt erklären.
6. Einen kleinen VM-Proof für Update/Rollback spezifizieren; Hardware und App-
   Kompatibilität später als eigene Abnahme. Kein eigener Resolver ohne belegte
   Unzulänglichkeit bestehender Lösungen.

**Abnahme:** begründete Basisentscheidung, benannte Vertrauensgrenzen, messbarer
Proof-Plan und begrenzte folgende Phasen. Keine Behauptung eines fertigen NIWOE OS.

## 7. Gemeinsame Definition of Done

### Automatisierte Gates

Auf dem dokumentierten Linux-Host, nach jeder zusammenhängenden Rust-Änderung
mindestens Check; nach Logik-/Teständerungen zusätzlich Workspace-Tests.
Am Phasenende laufen alle zutreffenden Gates auf demselben Stand:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p niwoe-tokens --test design_guard
cargo test -p niwoe-tokens --test source_size_guard
cargo test -p niwoe-shell --test centralization_guard
git diff --check
```

Vor P01 heißen die betreffenden Packages noch `meridian-*`. Bei Formatänderungen
zuerst `cargo fmt --all`, danach prüfen. Keine unbekannten Tests als bestanden
deklarieren. Kein `--no-verify`, keine pauschalen Guard-Ausnahmen und kein
Hochsetzen des 600-Zeilen-Limits. Nur Markdown geändert: Diff-/Linkprüfung genügt;
Rust-Gates nicht als ausgeführt behaupten.

Die gezielten Guard-Kommandos dienen der klaren Diagnose; wenn der vollständige
Workspace-Lauf dieselben Tests nachweislich bereits ausgeführt hat, muss man sie
nicht ohne Anlass nochmals laufen lassen. Im Bericht konkrete Testnamen belegen.

### Manuelle Prüfung und Belege

- Call-Flow für Rendering/Input/IPC: Eintritt → Zustandsänderung → IPC/Backend →
  bestätigter Snapshot → Invalidierung/Rendern → Fokus-/Fehlerbehandlung.
- UI-Phasen: Screenshots des grünen Themes mit identischen Inhalten, zusätzlich kleinster
  Zielauflösung 1366×768 und 1920×1080; 100/150/200% Skalierung wo verfügbar.
- Tastatur und Maus, leere/lange/fehlerhafte Daten, Wiederöffnen und Reconnect.
- Netzwerk-/Auth-/Hardwarefälle benötigen echte Backendbelege; gemockte Unit-Tests
  decken das Verhalten zusätzlich ab, ersetzen aber nicht den Integrationsnachweis.
- Fehlende Hardware wird als `NOT RUN` mit konkretem benötigtem Test benannt.
  Pflichtfälle der Phase bleiben bis zu ihrem Nachweis offen.

### Performance-Modell und Budget

P00 nimmt Ausgangswerte auf gleicher Linux-Hardware auf. Pro relevante Phase drei
Messungen nach Warm-up; fünf Minuten Idle sowie zwanzig Öffnen/Schließen-Zyklen.
CPU von Compositor/Shell, verfügbare GPU-Daten, Repaint-Zähler, RSS/Cachegröße und
Eingabe-bis-sichtbar-Latenz festhalten. Werkzeug/Einheit/Last/Display-Hz angeben.

- Statische geschlossene Oberflächen erzeugen keine neuen Frame-Timer oder Captures.
- Caches haben Schlüssel, Invalidierung, Obergrenze und Lebensende. Mehrfaches
  Öffnen darf nach Warm-up kein stetiges Speicherwachstum verursachen.
- Bei Idle-CPU-Erhöhung um mehr als 0,5 Prozentpunkte eines CPU-Kerns oder
  reproduzierbarer Latenzverschlechterung über 10% gegenüber Baseline muss die
  Ursache vor Abnahme geklärt und behoben oder ausdrücklich begründet akzeptiert
  werden. Das sind Projektbudgets, keine behaupteten heutigen Messwerte.
- Fehlende GPU-Metriken sind unbekannt, nicht automatisch null. Blur bleibt beim
  Compositor; keine zweite Szenenabtastung in der Shell.

## 8. Review-Paket und Aufgabenverteilung

Pro Phase legt das Modell `docs/phase-reports/Pxx.md` an, gemäß Vorlage in der
Agentenübergabe. Enthalten: Basis-/Endstand, Tickets, Änderungen pro Datei,
Befehle/Exitcodes, konkrete Runtimefälle, Screenshots, Performance, offene Risiken.

Der implementierende Agent prüft am Phasenende den tatsächlichen Diff. Ergebnis:
`accepted` oder priorisierte Findings mit Datei/Zeile, Wirkung und erwartetem Test.
Findings werden in derselben Phase selbst behoben. Danach gezielte Nachprüfung.
Ein Review kann fehlende Belege nicht weginterpretieren.

Bei echtem Blocker innerhalb einer Phase: gezielt diagnostizieren und unabhängige
Arbeit abschließen; notwendige Host-/Zugangsinformation beim Benutzer erfragen.
Es gibt keine Modellübergabe mehr. Keine Architekturfrage
erfinden, wenn die Entscheidung bereits in diesem Plan steht. Keine endlose
Versuchsschleife und kein Verstecken eines Blockers hinter einem grünen Teiltest.

## 9. Modellwahl

> Historische Empfehlung aus der ursprünglichen Arbeitsteilung. Seit dem
> Folgeauftrag übernimmt der aktuelle Agent alle Phasen selbst; diese Empfehlungen
> lösen keine Übergaben oder Zwischenfreigaben aus.

Terra reicht voraussichtlich für viele klar begrenzte Pakete: Dokumentation,
Widgets, Farben/Tokens, Formulare und bestehende Provider anbinden. Die riskanteren
Pakete P01, P06, P07, P09 und P12 würde ich Sol geben, wenn der Anspruch lautet,
innerhalb einer Phase ohne laufende stärkere Betreuung voranzukommen.

Das ist eine Einschätzung aus dieser Architektur, keine gemessene Erfolgsquote.
Die offizielle Einordnung nennt Terra ein Modell für das Verhältnis von Intelligenz
und Kosten und Sol ein Flagship für komplexe professionelle Arbeit:
[Terra](https://developers.openai.com/api/docs/models/gpt-5.6-terra),
[Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol).

Praktischer Start: Terra mit hoher Reasoning-Stufe für P00/P02–P05; Sol mit hoher
Reasoning-Stufe für die genannten Kernphasen. Nach der ersten abgenommenen Phase
an Zahl/Schwere der Findings und tatsächlich aufgewendeter Nacharbeit bewerten.
Falls nur ein Implementierungsmodell gewünscht ist: Sol. Falls Kosten besonders
wichtig sind: Terra als Standard mit klarer Sol-Eskalation, nicht Terra für alles.
