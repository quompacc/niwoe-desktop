# P00 — Gegencheck

Datum: 2026-09-21

Urteil: **nicht abgenommen; P00 bleibt blocked**.

Geprüft: uncommitteter Arbeitsbaum auf Basis
`349cfe4a31d293de03c1f181597ad4c256b06ed7`, P00-Phasenbericht,
P00-Anforderungen im NIWOE-Plan, geänderte Regeln/Dokumente, DNF-Erweiterung
und zugehöriger bestehender Start-/Installationspfad. Kein Linux-Lauf durch
dieses Review. Bereits vor P00 vorhandene Assets/-Löschungen sind keine
Implementierungsänderungen dieser Phase.

## F1 — P1: Linux-Baseline fehlt, Tickets sind dennoch vollständig abgehakt

Fundstelle: `docs/phase-reports/P00.md:44–49`.

Der Gesamtstatus `blocked` und die NOT-RUN-Angaben sind korrekt. P00-05 verlangt
jedoch eine eingerichtete und dokumentierte Linux-Testumgebung, P00-06 tatsächliche
Gates auf dem Ausgangsstand. Dokumentation dieser Schritte erledigt die Tickets
nicht. Auch „Keine Baseline-Codefixes nötig“ ist ohne Baselineprüfung nicht belegt.

Korrekturauftrag:

1. P00-05 und P00-06 offen bzw. teilweise erledigt markieren; ausdrücklich nur
   „keine Baseline-Codefixes vorgenommen“ behaupten.
2. Einen verfügbaren Linux-Testhost identifizieren und seine aktuelle Eignung
   prüfen. Der früher dokumentierte Arch-Host ist ein möglicher Prüfpfad, aber
   sein heutiger Zustand und seine Erreichbarkeit sind nicht belegt. Nicht
   automatisch annehmen, dass er noch vorhanden ist oder funktioniert.
3. Solange Fedora nicht als konkrete Hostwahl bestätigt wurde, darf eine fehlende
   Fedora-Neuinstallation nicht allein einen sonst geeigneten Linux-Testhost
   ausschließen. Die Fedora-DNF-Erweiterung benötigt zusätzlich ihren eigenen
   tatsächlichen Fedora-Installationsnachweis.
4. Den exakten zu prüfenden Arbeitsstand übertragen: derzeit sind Änderungen
   uncommittet und wichtige Planungs-/Baseline-Dateien untracked. Ein Checkout
   des Basis-Commits allein testet nicht dieses Reviewpaket. Übertragene Dateien
   und Stand nachvollziehbar festhalten.
5. fmt, Workspace-Check, Clippy, Workspace-Tests und enthaltene Guards auf Linux
   mit Exitcodes/Logs belegen. Nested-Start, authentifizierte Shell-Verbindung
   und realen Wayland-Client nachweisen. Vorhandene Fehler gesondert behandeln.
6. Wenn kein Host verfügbar ist: notwendige Zugangs-/Hostinformation konkret
   benennen, Status blocked erhalten und nur unabhängige Korrekturen abschließen.

Abnahme: Keine offenen Pflicht-Gates; Bericht und Ticketstatus entsprechen den
Belegen. Hardwarefälle gemäß Phasenplan einordnen; Nested-Ergebnisse nicht als
DRM-/GPU-Nachweis deklarieren. P01 noch nicht beginnen.

## F2 — P2: Dokumentationsindex und Hardware-Smoke enthalten aktive Altvorgaben

Fundstellen: `docs/README.md:7–12,18–24`, `docs/HARDWARE_SMOKE.md:3–5`.
Weitere betroffene Einstiegspunkte: `docs/ARCHITECTURE.md:99–104` und
`docs/PROJECT_STATUS.md` mit alter Plattformrichtung/offenen BSD-Aufgaben.

Der Dokumentationsindex gibt dem BSD-Handoff und den alten Plänen noch immer
erste Priorität und nennt die alte UI-Reihenfolge verbindlich. Der Hardware-Smoke
verweist weiterhin auf den „neuen OpenBSD/Acer decision path“. Damit ist die
P00-Abnahme „keine widersprüchliche aktive BSD-/WebKit-Anweisung“ noch nicht
erfüllt; der Hardware-Smoke war außerdem ausdrücklich Teil von P00-02.

Korrekturauftrag:

1. `docs/README.md` auf die NIWOE-Dokumentenpriorität und neue Baseline umstellen.
2. `docs/HARDWARE_SMOKE.md` als aktuellen Linux-Runbook einordnen oder ausdrücklich
   historische Teile abgrenzen und auf den neuen ausführbaren Ablauf verweisen.
3. Architektur-/Statusdokumente auf aktive Zukunftsanweisungen prüfen. Technische
   Fakten und historische Messungen erhalten, alte BSD-/WebKit-Ziele und deren
   „nächste Schritte“ als abgelöst kennzeichnen.
4. Strategische Textsuche nicht auf die bereits editierten Dateien begrenzen.
   Verbleibende Altbegriffe nach aktiver Regel vs. technischer Historie beurteilen;
   kein globales Ersetzen sämtlicher OpenBSD-/WebKit-Nennungen.

Abnahme: Auch über den Dokumentationsindex erreicht ein Implementierungsmodell
eindeutig den neuen NIWOE-Plan; keine konkurrierende Dokumentenpriorität.

## F3 — P2: Der zugesagte zusätzliche Sitzungsstart ist nicht reproduzierbar

Fundstelle: `docs/NIWOE_LINUX_BASELINE.md:48–52`; außerdem dort Zeile 84.

Die Anleitung verlangt eine zusätzliche Display-Manager-Auswahl „über den
vorhandenen Installpfad“. `scripts/install-local.sh` installiert jedoch keine
Wayland-Session-Desktopdatei; unter `packaging/` gibt es ebenfalls keine solche
Sessiondatei. Die vorhandene Alternative `--enable-boot` aktiviert den eigenen
Login und deaktiviert getty@tty1, was kein Ersatz für die zugesagte zusätzliche
KDE-kompatible Sitzungswahl ist. Die Anleitung verschiebt die Desktopdatei auf
P01, obwohl der technische Startpfad für die Baseline bereits benötigt wird.
Auch „Nested/Winit-Sitzung starten“ enthält noch keinen konkreten Startbefehl
oder eine Anleitung, den Client mit dem richtigen verschachtelten Socket zu verbinden.

Korrekturauftrag:

1. Den P00-Start mit den aktuellen `meridian`-Binärnamen ausführbar dokumentieren:
   Build/Installation, Shell-Auflösung, Prozessstart und Logpfad.
2. Für die zusätzliche Sitzung eine konkrete Desktopdatei bzw. einen minimalen
   Session-Wrapper samt Installationsort angeben/ergänzen. Displaymanager erhalten;
   keine Aktivierung des eigenen Boot-Login als Ersatz. Umbenennung bleibt P01.
3. Sessionumgebung und Lebensende berücksichtigen: eigener DRM-Start vs.
   verschachtelter Winit-Start, XDG-Desktopkennung, User-Service-/D-Bus-Aktivierung
   und keine konkurrierenden Polkit-Agenten. Den realen Call-Flow prüfen.
4. Nested-Start und einen konkreten Testclient samt korrektem `WAYLAND_DISPLAY`
   beschreiben. Sonst kann versehentlich KDE statt NIWOE als Clientnachweis dienen.
5. Mit neuen Installationsdateien zugleich den frischen Installpfad und die
   Rückkehr zur KDE-Sitzung testen; Ergebnis in P00 nachtragen.

Abnahme: Ein nachfolgendes Modell kann die Anleitung auf dem dokumentierten
Host ohne eigene Architektur-/Installationsentscheidungen ausführen. Logs belegen
Backend, Shell-Verbindung und Client im richtigen Compositor.

## Positiv geprüfte Teile

- AGENTS/CLAUDE und Root-README setzen NIWOE/Linux/native Rust als Produktpfad.
- Das Manifest löst die alte Blau-/Kompassrichtung ab und erhält zentrale Tokens,
  zwei Themes und das Performance-Modell.
- Das Konzeptduplikat ist durch einen funktionierenden Root-Verweis ersetzt.
- Der DNF-Zweig ist klein und erhält die getrennten Paketgruppen sowie bestehende
  pacman-/apt-Pfade. Seine echte Installation/Buildvollständigkeit bleibt offen.
- Der Bericht gibt fehlende Linux-/Hardwaretests transparent als NOT RUN an.

## Eigene Verifikation

| Prüfung | Ergebnis |
|---|---|
| Tatsächlicher Diff, neue Baseline und Bericht gegen Plan gelesen | abgeschlossen |
| Installations-/Backendwahl im vorhandenen Code verfolgt | fehlende Sessiondatei bestätigt |
| `git diff --check` | Exit 0; Zeilenendehinweise, keine Whitespacefehler |
| `"C:\Program Files\Git\bin\bash.exe" -n scripts/install-deps.sh` | Exit 0 mit Git Bash 5.3.15 |
| Linux-Cargo-/DNF-/Runtime-/Hardwareprüfungen | nicht ausgeführt |

Der im Implementierungsbericht genannte WSL-Fehler verhindert die lokale
Bash-Syntaxprüfung nicht: Git Bash ist vorhanden. Diese bestandene Syntaxprüfung
ersetzt ausdrücklich keinen Fedora-/Linux-Integrationslauf.

Es wurde ausschließlich diese Reviewdatei neu angelegt; keine Implementierung
repariert, kein Phasenstatus hochgestuft und keine Rust-Datei geändert.

## Übergabe

F2 und F3 sowie die Statuskorrektur aus F1 können sofort bearbeitet werden.
Danach die fehlenden Hostbelege aus F1 nachreichen. Erst auf diesem korrigierten
und getesteten Stand erneut P00-Review anfordern.
