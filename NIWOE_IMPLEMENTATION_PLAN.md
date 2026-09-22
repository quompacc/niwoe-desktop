# NIWOE: Umsetzungsplan und Phasenabnahme

Stand: 21.09.2026. Grundlage: `NIWOE_Konzeptzusammenfassung.md`, alle acht
aktuellen PNG-Mockups unter `assets/` und eine gezielte Sichtung der bestehenden
Rust-Architektur. Dies ist ein Arbeitsplan, kein Nachweis abgeschlossener Features.

**Arbeitsweise aktualisiert am 21.09.2026:** Auf ausdrücklichen Nutzerauftrag
übernimmt der aktuelle Agent Implementierung, Tests und Phasenprüfung selbst.
Es gibt keine Übergabe zwischen Modellen und keinen manuellen Reviewstopp am
Phasenende. Die fachlichen Phasen und ihre Prüfgates bleiben bestehen. Ein kurzer
fortgeschriebener Phasenbericht dokumentiert Ergebnis, Prüfungen und offene Risiken.

**Scope-Korrektur auf Nutzerauftrag, 22.09.2026:** Keine weitere Ausbau- oder
Abnahmerunde für die abzulösende untere Taskleiste und den alten kombinierten
Launcher/Settings-Container. P02 liefert die wiederverwendbaren Designgrundlagen;
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

**Neubauentscheidung, 22.09.2026:** Panel, Such-Launcher und System-Deck werden
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
ausreichendes Produktmerkmal. Launcher, Hub, Deck und Settings bekommen klar
getrennte Aufgaben. NIWOE baut keine eigene IDE, Notiz-App oder Aufgabenverwaltung.

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
| P04 | Tastaturfester Spotlight-Launcher | Terra; Sol bei Inputproblemen | P03 |
| P05 | Funktionierendes System Deck | Terra | P04 |
| P06 | Persistente Räume und zuverlässige Fensterzuordnung | Sol | P05 |
| P07 | Raumleiste und App-Zuordnungsregeln | Sol Kern, Terra Darstellung | P06 |
| P08 | Nutzbarer Hub | Terra; Sol bei Lifecycleproblemen | P07 |
| P09 | Begrenztes, ehrliches Restore | Sol | P08 |
| P10 | Raum- und Leistenkonfiguration | Terra | P09 |
| P11 | First-Run-Assistent | Terra | P10 |
| P12 | Desktop-Alpha auf Linux-Hardware | Sol | P11 |
| P13 | Entscheidungsreife OS-Architektur | Sol recherchiert, Astra prüft | P12 |

Dieser neue Plan empfiehlt Panel → Launcher → Deck vor großen neuen
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
   Fehlende Light-/Launcher-/Wizard-Referenzen ausdrücklich kennzeichnen.
4. P00-04: Im alten Manifest die neue Designrichtung explizit festhalten und den
   Brief als Präzisierung referenzieren. Zentralität, zwei Themes und Effizienz
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

1. P02-01: Semantische Rollen und konkrete Dark-/Light-Paletten aus dem Brief in
   vorhandene Tokenstrukturen überführen. Keine parallele Palette einführen.
2. P02-02: Abstände, Komponentenhöhen, Fokusrahmen und Schrifthierarchie zentral
   definieren. Neue Tokenmodule dürfen große vorhandene Dateien entlasten.
3. P02-03: Komponentenblatt mit Surface, Text, Button, Tab, Chip, Karte, Eingabe,
   Slider und Zuständen normal/hover/focus/pressed/disabled/error erzeugen.
   Vorhandene Render-/Testwerkzeuge bevorzugen; kein neuer UI-Testframeworkbau.
4. P02-04: Beide Themes aus denselben Widgets rendern. Schriftfallback und fehlende
   Glyphen prüfen; Serif nur für größere Überschriften, Sans für Bedienelemente.
5. P02-05: Kontrastpaare, 100/150/200%-Skalierung und helles/dunkles Wallpaper
   prüfen. Messwerte und Screenshots ablegen. Aus einem PNG keine Animation ableiten.

**Abgrenzung seit 22.09.:** P02-05 betrifft die gemeinsamen Komponenten. Die
vollständige Screenshotmatrix und Bedienprüfung des alten Desktops entfällt.
Output-Skalierung und Rasterqualität werden mit den neuen P03–P05-Oberflächen
integriert geprüft; dafür keine zweite Testreihe am ersetzten Layout starten.

**Abnahme:** Nur Farbtabellen unterscheiden sich zwischen Themes; keine neue lokale
Designkonstante außerhalb zentraler Tokens; Dark-/Light-Komponentenblatt lesbar.
Kontrastziele des Briefs erreicht. Keine neue dauerhafte Render-/Timerlast.

**Reviewfokus:** Designidentität und Light-Parität, zentrale Werte, Lesbarkeit,
keine versteckten Ausnahmen zur Umgehung der Guards.

### P03 — Obere Leiste als stabile native Grundlage

**Betroffen:** `niwoe-shell/src/panel*.rs`, `panel_view/`, `wayland/init/`,
`wayland/handlers/output.rs`, `wayland/render/`, zentrale Panel-Tokens.

1. P03-01: Panel oben über nutzbare Outputbreite verankern. Exclusive Zone, Popover-
   Anker, Hitboxes und verfügbaren Fensterbereich gemeinsam korrigieren.
2. P03-02: Layout Launcher | vorhandene Workspace-Auswahl | flexible Lücke |
   Statusmodule | Uhr. Bestehende Fenster-/Tray-Zugänge solange erhalten, bis
   P07/P08 einen vollständigen Ersatz für minimierte Fenster und Fensterwahl bieten.
3. P03-03: Netzwerk, Audio, Akku und Uhr an vorhandene Daten anschließen.
   Kein Akku auf Geräten ohne Akku; keine erfundenen CPU-/GPU-Werte.
4. P03-04: Schmale Displays, lange Texte und Overflow bedienen. Uhr/Launcher
   bleiben erreichbar; optionale Module dürfen in Overflow wechseln.
5. P03-05: Popup-Positionen nach Monitorwechsel, Scale-Wechsel und Hotplug testen.
   Uhr ohne Sekunden aktualisiert nur zum Minutenwechsel, nicht pro Frame.

**Abnahme:** Kein überdeckter App-Inhalt, keine veraltete untere Reserved Zone,
korrekte Eingabe auf zwei Outputs, Tab-/Fokusdarstellung und beide Themes geprüft.

**Reviewfokus:** Layer-Shell-Geometrie, Renderreihenfolge, CPU im Idle; keine neue
Raumlogik in UI-Dateien vor P06.

### P04 — Launcher und eindeutige Overlay-Eingabe

**Betroffen:** `launcher.rs`, `app_view/`, `niwoe-app-catalog`,
`wayland/handlers/keyboard.rs`, `niwoe-config/src/keybind/`, Compositor-Inputpfad.

1. P04-01: Zentriertes Suchpopup auf fokussiertem Output, Suchfeld sofort aktiv.
   Alpha-Suchumfang: installierte Apps und vorhandene Settings/Systemaktionen.
2. P04-02: Tastatur: Tippen, Pfeile, Enter, Escape; Mausklick gleichwertig.
   Stabile Auswahl bei Ergebnisupdates, Leerzustand und Startfehler darstellen.
3. P04-03: Omarchy-Belegung: Super+Space öffnet den Launcher. Super allein
   löst keine Aktion aus; kombinierte Kürzel dürfen keinen zweiten Trigger auslösen.
4. P04-04: Super+Alt+Space öffnet die Steuerung/Einstellungen, Super+Escape das
   Deck. Super+Tab wechselt zum nächsten Raum. Benutzerdefinierte Bindings
   erhalten; Konflikt erkennen und verständlich anzeigen.
5. P04-05: Gemeinsamen Overlay-Lifecycle für Launcher/Deck und später Hub festlegen:
   höchstens ein Hauptoverlay, Escape schließt, alter Fensterfokus wird korrekt
   zurückgegeben, Lock übersteuert und schließt Overlays.

**Abnahme:** Zwanzig Open/Type/Launch/Escape-Zyklen ohne Fokusverlust; Kombinationen
ohne Fehltrigger; kein Vollscan des Dateisystems je Tastendruck. Reconnect/Shell-
Neustart hinterlässt kein unsichtbares keyboard-exclusive Overlay.

**Reviewfokus:** Press-/Release-Semantik, Fokus und Launch-Pfad; keine generische
Shell-Befehlssuche als ungeplante Erweiterung.

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
5. P05-05: Teure Statusabfragen nur bei Sichtbarkeit/Änderung; keine neue Polling-
   Schleife für geschlossene Overlays. Kontextabhängige automatische Sortierung später.

**Abnahme:** Tatsächliche Lautstärke-/Mute-Änderung; Netzwerkzustand/Fehler;
Bluetooth ohne Adapter; Escape und Fokus; beide Themes. Für noch fehlende Provider
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

### P08 — Hub als schnelle Übersicht

**Betroffen:** neue kleine `niwoe-shell/src/hub/`-Module, vorhandene
Thumbnail-/IPC-/Overlaypfade, Launcher-Provider für Räume/Fenster.

1. P08-01: Per Maus und über eine später konfliktfrei festgelegte Belegung ein
   Overlay mit Raumkarten öffnen. Super+Tab bleibt Omarchys Raumwechsel.
   Karten: Name, aktive/belegte Zustände, App-Icons, Fensteranzahl und Auswahl.
2. P08-02: Pfeile/Tab/Enter/Escape, Scrollen und Filter nach Raumname; Klick auf
   Raum wechselt, Klick auf Fenster aktiviert das konkrete Fenster.
3. P08-03: Bestehende Thumbnail-Capture-Fähigkeit wiederverwenden; Alpha statische
   Vorschau bei Öffnung, maximal eine pro sichtbarer Karte, begrenzter Cache.
   Während geschlossen keine Captures. Ohne Bild steht ein sinnvoller Platzhalter.
4. P08-04: Captureantworten nur für aktuelle Hub-Generation akzeptieren;
   Theme-/Scale-/Fensterende invalidieren; Lock schließt und verwirft Vorschauen.
5. P08-05: Raum-/Fenstersuche als zusätzliche Launcher-Provider ergänzen. Kein
   zweiter Suchindex und keine globalen Dateiscans.

**Abnahme:** Hub mit leeren/vollen Räumen und 64 Einträgen bedienbar;
Fenster während offener Vorschau schließen; Output wechseln; wiederholt öffnen
ohne wachsenden Cache; Lock zeigt keine alten Vorschaubilder. Keine erfundenen
Tasks-, Datei-, Synchronisations- oder Prozesszahlen aus Mockups.

**Reviewfokus:** Orientierung, Tastatur, Grenzen der Bildaufnahme und Cache-Lebensdauer.

### P09 — Layout speichern und best effort wieder öffnen

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

### P10 — Control Center für Räume und Leiste

**Betroffen:** `niwoe-shell/src/settings_view/`, bestehende Widgets,
Config-/Raum-Commands, Manifest-Komponenten.

1. P10-01: Raumliste mit Suche, Erzeugen, Name/Beschreibung/Icon, Reihenfolge,
   belegtem Zustand und sicherem Löschen erstellen.
2. P10-02: Formular mit lokalem Entwurf und Speichern/Abbrechen; Validierungsfehler
   direkt am Feld. Erfolgsanzeige erst nach bestätigter Persistenz.
3. P10-03: Free/Preferred/Dedicated, App-Präferenzen und Restorefähigkeiten
   konfigurieren. Nicht implementierte Integrationen nicht als aktive Toggles zeigen.
4. P10-04: Leistenmodule ein-/ausblenden und anordnen. Launcher/Raumzugang/Uhr
   bleiben erreichbar; CPU/GPU/Sensoren nur bei realem Provider.
5. P10-05: Vorschau aus denselben Komponenten/Tokens rendern; kein zweites
   Designsystem. Vorschauänderung wird bei Abbruch zurückgenommen.

**Abnahme:** Ungültiger Name, doppelte ID aus Eingabe, verschwundene App,
konkurrierende Änderung, Speichern schlägt fehl, Abbruch, Neustart und Light-Theme.

**Reviewfokus:** Model/View-Grenze, echte Persistenz, kein unbeauftragtes
Automatisierungs-, Backup-, Aufgaben- oder Dateimanagerprojekt.

### P11 — First Run als kurze Einführung

**Betroffen:** kleine neue Shell-Wizard-Module, vorhandene Settings-/Room-API,
benutzerbezogener versionierter First-Run-State.

1. P11-01: Willkommen → Theme → Bedienprofil → Räume → Leiste → Fertig.
   Defaults brauchbar, jeder optionale Schritt überspringbar.
2. P11-02: Bedienprofile setzen ausschließlich Defaults; keine verschiedene
   Architektur für Maus/Tastatur. Bestehende Nutzeranpassungen erhalten.
3. P11-03: Raumvorschläge und Apps nur aus tatsächlichem Katalog; keine fehlenden
   Apps automatisch installieren. Änderungen bis Abschluss als Entwurf sammeln.
4. P11-04: Drei Shortcuts und Mausäquivalente in einfacher interaktiver Erklärung.
5. P11-05: Abschlussmarker erst nach erfolgreichem Speichern; Crash/Abbruch kann
   fortgesetzt werden, ohne Räume zu duplizieren. Erneuter Aufruf in Settings.

**Abnahme:** Frisches Profil, bestehendes migriertes Profil, Überspringen,
Abbruch/Neustart, Nur-Tastatur, kleiner Bildschirm und beide Themes.
Keine fingierten Datenschutz-/Updateeinstellungen vor einem realen OS-Unterbau.

**Reviewfokus:** Onboarding erklärt vorhandene Funktionen und verändert kein
etabliertes Nutzerprofil ungefragt.

### P12 — Linux-Desktop-Alpha stabilisieren

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
- UI-Phasen: Screenshots Dark/Light mit identischen Inhalten, zusätzlich kleinster
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
