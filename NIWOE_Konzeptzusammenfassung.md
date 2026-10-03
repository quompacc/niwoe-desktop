# NIWOE – Konzeptzusammenfassung

## 1. Projektname

Der neue Arbeits- und Produktname lautet:

# **NIWOE**

Der Name ist ein Kunstwort und wird im Deutschen ungefähr wie **„Niveau“** gelesen.

Die Assoziation ist dabei angenehm indirekt:

- Ebene
- Niveau
- Qualität
- Anspruch
- Ordnung
- Struktur

Niwoe ist bewusst **nicht** aus einer bekannten Mythologie, Kultur oder Fantasy-Welt abgeleitet. Dadurch wirkt der Name weniger kitschig, weniger verbraucht und kann seine Bedeutung vollständig aus dem Produkt selbst entwickeln.

Technisch ist der Name unkompliziert nutzbar:

```text
NIWOE
niwoe
niwoe-core
niwoe-shell
niwoe-os
```

Die bisherige Recherche ergab keinen offensichtlichen Konflikt mit einem bestehenden Betriebssystem, Desktop oder größeren Softwareprodukt unter exakt diesem Namen. Vor einer kommerziellen Markenanmeldung sollte dennoch eine formale Ähnlichkeitsprüfung bei EUIPO / TMview / DPMA für insbesondere die Klassen 9 und 42 erfolgen.

---

# 2. Grundidee

Niwoe soll **nicht einfach eine weitere Linux-Distribution** werden.

Die Vision ist:

> Ein kohärentes Desktop-Betriebssystem auf Linux-Basis, das organisatorisch und strukturell eher wie ein klassisches BSD gedacht ist.

Linux wird dabei als technische Plattform genutzt:

```text
Linux Kernel
    ↓
Treiber / Mesa / Vulkan / Hardware
    ↓
Niwoe Base System
    ↓
Niwoe Desktop
    ↓
Anwendungen
```

Linux liefert also Kernel, Treiber, Hardwareunterstützung und das große Software-Ökosystem.

Niwoe definiert darüber:

- die Systemstruktur
- die Benutzererfahrung
- die Update-Strategie
- die Paketlogik
- die Desktop-Shell
- die Design-Sprache
- das Workspace-/Raum-Modell
- die klare Trennung zwischen Betriebssystem und Anwendungen

---

# 3. Warum Linux und nicht FreeBSD?

FreeBSD ist konzeptionell sehr attraktiv, weil es Kernel und Basissystem stärker als Einheit betrachtet.

Das passt gut zu Niwoes Wunsch nach:

- klaren Zuständigkeiten
- einem definierten Basissystem
- sauberen Grenzen zwischen OS und Drittsoftware
- nachvollziehbarer Systempflege

Für einen modernen Desktop entstehen unter FreeBSD jedoch zusätzliche Risiken:

- GPU-Treiber
- Mesa / Vulkan
- Steam
- Proton
- Wine
- Gamepads
- Laptop-Hardware
- Wi-Fi
- Bluetooth
- moderne Audio-/Video-Hardware
- aktuelle Desktop-Unterstützung

Deshalb ist der realistischere Weg:

> **BSD-artige Systemphilosophie auf Linux-Basis.**

---

# 4. Das Niwoe Base System

Das System soll eine bewusst definierte Basis besitzen.

Mögliche Komponenten:

```text
Linux Kernel
glibc
Mesa
Wayland
XWayland
libinput
udev
PipeWire
BlueZ
Netzwerkstack
Session-/Seat-Management
Basiswerkzeuge
Niwoe Compositor
Niwoe Systemdienste
Niwoe Shell
```

Diese Komponenten sollen nicht als zufällige Sammlung einzelner Pakete behandelt werden, sondern als zusammenhängendes Release.

Beispiel:

```text
Niwoe OS 1.0
Niwoe OS 1.1
Niwoe OS 2.0
```

Der Benutzer soll ein Betriebssystem wahrnehmen, keine Ansammlung unabhängiger Linux-Komponenten.

---

# 5. Klare Trennung zwischen Base System und Anwendungen

Ein zentraler Punkt der Architektur ist die Trennung zwischen Betriebssystem und Drittsoftware.

Beispiel:

```text
Niwoe Base System
├── Kernel
├── libc
├── Mesa
├── Wayland
├── PipeWire
├── Systemdienste
└── Niwoe Shell

Niwoe Applications
├── Firefox
├── Steam
├── Blender
├── LibreOffice
├── IDEs
└── weitere Software
```

Das Basissystem wird kontrolliert und versioniert.

Anwendungen werden getrennt davon verwaltet.

---

# 6. Ein Installations- und Verwaltungsmodell

Niwoe soll das typische Linux-Durcheinander möglichst vermeiden:

```text
Distribution-Pakete
Flatpak
Snap
AppImage
Fremdrepositories
Tarballs
curl | sh
manuelle Binaries
```

Der Benutzer soll möglichst **einen einzigen sichtbaren Mechanismus** haben.

Beispiel:

```bash
niwoe install firefox
niwoe install steam
niwoe update
niwoe remove firefox
```

Intern darf die Herkunft unterschiedlich sein:

```text
Firefox
→ offizieller Mozilla-Build

Steam
→ Valve-Binary

Mesa
→ eigener Build

OpenSSH
→ Source Build
```

Die zentrale Regel lautet:

> **Die Herkunft darf unterschiedlich sein. Die Verwaltung muss einheitlich sein.**

---

# 7. Kein Programm verändert das Basissystem eigenmächtig

Ein weiterer Grundsatz:

> Programme installieren sich nicht unkontrolliert in das Basissystem.

Also möglichst kein:

```bash
curl https://example.com/install.sh | sh
```

und kein unkontrolliertes Schreiben nach:

```text
/usr/bin
/usr/lib
/etc
```

Niwoe soll jederzeit nachvollziehen können:

- Woher stammt eine Datei?
- Zu welchem Paket gehört sie?
- Wer aktualisiert sie?
- Wie wird sie entfernt?
- Gehört sie zum System oder zu einer Anwendung?

---

# 8. Bestehender Desktop-Code bleibt das Fundament

Der bisherige Meridian-Code soll **nicht neu geschrieben** werden.

Er enthält bereits wertvolle technische Infrastruktur:

```text
Wayland Compositor
Smithay
XWayland
Input
Multi-Monitor
IPC
Login
Lockscreen
Portals
Polkit
Fensterverwaltung
Workspaces
```

Der technische Kern bleibt bestehen.

Neu gedacht werden vor allem:

- Name
- visuelle Identität
- Shell
- Informationsmodell
- Raum-/Kontextmodell
- Launcher
- Hub
- System Deck
- Leiste
- First-Run Experience

---

# 9. Fensterphilosophie

Niwoe soll nicht versuchen, GTK-, Qt-, Electron-, Firefox- oder andere Drittanbieter-Anwendungen künstlich in ein gemeinsames Fensterdesign zu zwingen.

Das ist technisch problematisch, weil viele Anwendungen ihre Headerbars oder Navigation selbst zeichnen.

Daher:

> **Niwoe organisiert Anwendungen, statt sie visuell umzubauen.**

Der Compositor kontrolliert:

```text
Position
Größe
Tiling
Floating
Gaps
Focus
Border
Shadow
Animation
Raum
```

Aber nicht zwanghaft den inneren App-Inhalt.

---

# 10. Fensterklassen

## Tiled Windows

- keine zusätzliche Niwoe-Titlebar
- sehr dünner Border
- kaum oder kein Shadow
- klare Fokusdarstellung

## Floating Windows

- dünner Border
- leichter Shadow
- optional kleine Rundung
- weiterhin möglichst wenig zusätzliche Chrome

## Niwoe System UI

- vollständige eigene Designsprache
- Panel
- Hub
- Launcher
- Settings
- Deck
- Login
- Lock Screen
- First-Run Wizard

---

# 11. Design-Sprache

Die visuelle Basis wird aus dem Völund-Stil abgeleitet.

Kernfarben:

```text
Dunkles Grün
Gold / Gelb
Off-White
```

Bedeutung:

```text
Dunkelgrün
→ Raum / Hintergrund / Fläche

Off-White
→ Information / Text

Gold
→ Fokus / aktiver Zustand / wichtige Aktion
```

Gold soll sparsam eingesetzt werden.

Nicht:

```text
alles Anklickbare = Gold
```

sondern:

```text
Gold = aktuell relevant
```

---

# 12. Dark Theme

Das Dark Theme soll wirken wie:

```text
Tannengrün
Graphit
Messing
Papier
```

Zielwirkung:

- ruhig
- präzise
- hochwertig
- industriell
- erwachsen

Nicht:

- Neon
- Cyberpunk
- RGB-Gaming
- Sci-Fi-HUD

---

# 13. Light Theme

Das alte helle Meridian-Design war zu generisch:

```text
Beige
Hellgrau
Blau
klassische Window Controls
```

Die neue Light-Version soll dieselbe Identität tragen wie das Dark Theme:

```text
Elfenbein
Schwarzgrün
Gold
```

Die beiden Themes sollen klar zur selben Designfamilie gehören.

---

# 14. Designreferenz

Die aktuellen Mockups sollen als **Design-Baseline** dienen.

Nicht pixelgenau, aber verbindlich in:

- Farbwelt
- Hierarchie
- Spacing
- Typografie
- Komponenten
- Flächenlogik
- Fokusdarstellung
- Informationsdichte
- Animation
- Tonalität

Empfohlene Dokumentstruktur:

```text
docs/design/
├── principles.md
├── colors.md
├── typography.md
├── spacing.md
├── components.md
├── motion.md
├── desktop.md
├── hub.md
├── system-deck.md
├── rooms.md
└── references/
```

---

# 15. Design Tokens

Die Oberfläche sollte über definierte Tokens aufgebaut werden.

Beispiel:

```text
surface.base
surface.raised
surface.overlay

text.primary
text.secondary

border.subtle
border.focus

accent.gold

radius.small
radius.medium
radius.large

spacing.1
spacing.2
spacing.3
```

Dadurch bleibt das System visuell konsistent, auch wenn es wächst.

---

# 16. Die Leiste

Niwoe soll keine klassische Taskleiste im Stil von Windows oder KDE benötigen.

Stattdessen:

```text
[ Launcher ] [ Räume ]          [ optionale Module ] [ Uhr ]
```

Optionale Module:

- Media
- Netzwerk
- Akku
- CPU
- GPU
- Temperatur
- Benachrichtigungen
- Kalender
- Systemstatus

Der Benutzer darf die Leiste in den Settings anpassen.

Ziel:

> **geordnet, aber nicht bevormundend.**

---

# 17. Informationshierarchie in der Leiste

Zustände sollen unterschiedlich gewichtet werden:

```text
normal
→ ruhig

aktiv / relevant
→ Gold

kritisch
→ semantische Warnfarbe
```

So bleibt auch eine informationsreiche Leiste ruhig.

---

# 18. Launcher

Der Launcher soll eher an Apple Spotlight erinnern als an ein klassisches Startmenü oder dmenu.

Aufruf beispielsweise:

```text
Super
```

Dann:

```text
zentriertes Popup
↓
Cursor sofort aktiv
↓
Tippen
↓
Ergebnisse
```

Langfristig kann der Launcher durchsuchen:

- Anwendungen
- Dateien
- Settings
- Fenster
- Räume
- Systemaktionen
- Berechnungen
- Befehle

Seine zentrale Frage:

> **Was möchte ich tun?**

---

# 19. System Deck

Ein zweites zentrales Overlay ist das **System Deck**.

Beispiel-Shortcut:

```text
Super + Space
```

Mögliche Inhalte:

- Audio
- WLAN
- Bluetooth
- Displays
- Helligkeit
- Power
- Media
- Do Not Disturb
- aktueller Raum

Das Deck soll nicht zu einem riesigen Dashboard werden.

Es ist für schnelle Systemsteuerung gedacht.

---

# 20. Kontextabhängiges System Deck

Später kann das Deck je nach Nutzung relevante Module priorisieren.

Laptop:

```text
Akku
Power Profile
Helligkeit
Wi-Fi
Bluetooth
```

Gaming:

```text
GPU
Temperatur
Controller
Audio
Recording
```

Videocall:

```text
Mikrofon
Kamera
Audio
Screen Sharing
Do Not Disturb
```

Damit wird das Deck zu einem:

> **kontextuellen Systeminstrument**

---

# 21. Räume als zentrales Bedienmodell

Der wichtigste konzeptionelle Unterschied zu klassischen Desktops:

> Workspaces werden zu **Räumen**.

Ein Raum ist nicht nur ein virtueller Desktop.

Ein Raum ist:

```text
Arbeitskontext
├── Apps
├── Fenster
├── Dateien
├── Layout
├── Prozesse
├── Regeln
└── Zustand
```

---

# 22. Räumliches Gedächtnis

Der Benutzer soll nicht denken:

> Wo ist Firefox?

sondern:

> Firefox ist in Recherche.

Beispiel:

```text
Entwicklung
Recherche
Konstruktion
Kommunikation
```

Das ist leichter zu merken als:

```text
Workspace 1
Workspace 2
Workspace 3
```

---

# 23. Raum-Modi

Mögliche Modi:

```text
Free
→ alles darf hinein

Preferred
→ bestimmte Apps landen bevorzugt dort

Dedicated
→ Raum gehört einer Aufgabe oder Anwendung
```

Beispiel:

```text
Recherche

Preferred:
Firefox
Zotero
PDF Viewer
Obsidian
```

---

# 24. Raumzustand speichern

Ein Raum kann optional speichern:

- Fensterlayout
- geöffnete Dateien
- laufende Apps
- Terminal-Sitzungen
- Positionen
- Startregeln
- Restore Policy

Damit kann ein Arbeitskontext wiederhergestellt werden.

---

# 25. Räume ersetzen keine Anwendungen

Wichtiger Scope-Grundsatz:

> Niwoe organisiert Kontext, ersetzt aber nicht die Werkzeuge.

Beispiel:

```text
Niwoe weiß:
Diese PDF gehört zur Recherche.

Niwoe wird aber:
kein PDF-Editor.
```

Oder:

```text
Niwoe weiß:
IDE + Terminal + Browser gehören zu Entwicklung.

Niwoe wird aber:
keine IDE.
```

---

# 26. Raumleiste

Die Räume werden direkt in der Leiste sichtbar.

Beispiel:

```text
Entwicklung   Recherche   Konstruktion   Kommunikation
```

Der aktive Raum bekommt Gold.

Die Raumleiste übernimmt:

- Navigation
- Orientierung
- Zustand

Dadurch wird eine klassische Fenster-Taskleiste weitgehend überflüssig.

---

# 27. Viele Räume

Vier oder fünf Räume funktionieren sehr gut.

Bei mehr Räumen:

```text
Entwicklung
Recherche
Konstruktion
Kommunikation
›
```

Nur wichtige oder zuletzt verwendete Räume bleiben direkt sichtbar.

Alle Räume sind über den Hub erreichbar.

---

# 28. Niwoe Hub

Der Hub ist die visuelle Übersicht über Arbeitskontexte.

Er zeigt beispielsweise:

```text
Entwicklung
VS Code
Terminal
Git
Docker

Recherche
Firefox
Zotero
PDF
Obsidian
```

Der Hub beantwortet:

> **Was ist gerade aktiv und in welchem Kontext?**

---

# 29. Klare UI-Rollen

Die aktuelle Architektur trennt die wichtigsten Oberflächen klar:

```text
Desktop
→ Arbeiten

Räume
→ Kontext

Hub
→ Orientieren

System Deck
→ System schnell steuern

Settings / Control Center
→ Konfigurieren
```

Diese Trennung soll strikt beibehalten werden.

---

# 30. Settings / Control Center

Die Verwaltungsoberfläche darf deutlich informationsreicher sein.

Sie verwaltet:

- Räume
- System
- Leiste
- Launcher
- Hardware
- Updates
- Automatisierung
- Design
- Shortcuts

Hier ist höhere Informationsdichte ausdrücklich erlaubt.

---

# 31. Raum konfigurieren

Zu den stärksten Konzepten gehört die Raumkonfiguration.

Mögliche Optionen:

```text
Fensterlayout merken
Dateien wiederherstellen
Terminal-Sitzungen fortsetzen
Start-Apps
Automatisierungsregeln
Restore Policy
```

Damit unterscheidet sich Niwoe wirklich von klassischen Desktop-Umgebungen.

---

# 32. First-Run / Welcome Wizard

Beim ersten Start erscheint ein visueller Assistent.

Nicht als lästige Installationsroutine, sondern als:

```text
Konfiguration
+
Tutorial
+
Personalisierung
```

Möglicher Ablauf:

```text
Willkommen
↓
Darstellung
↓
Bedienung
↓
Räume
↓
Leiste
↓
Updates / Datenschutz
↓
Fertig
```

---

# 33. Räume im Wizard

Der Benutzer kann direkt Räume einrichten:

```text
Home
Recherche
Dateien
Entwicklung
Media
```

Mögliche Einstellungen:

- Name
- Symbol
- Preferred Apps
- Modus
- Position

Dadurch lernt der Benutzer das Raumkonzept direkt beim ersten Start.

---

# 34. Leiste im Wizard

Die Leiste soll live konfigurierbar sein.

Beispiel:

```text
[ Niwoe ] [ Räume ]          [ Media ] [ Wi-Fi ] [ Uhr ]
```

Mögliche Schalter:

```text
Media anzeigen
Netzwerk anzeigen
CPU anzeigen
Akku anzeigen
Sekunden anzeigen
```

---

# 35. Bedienprofile

Statt „Anfänger / Profi“:

```text
Ausgewogen
Maus + Tastatur

Tastaturorientiert

Mausorientiert
```

Alle Profile verwenden dasselbe System.

Nur die Defaults unterscheiden sich.

---

# 36. Zentrale Shortcuts

Im Wizard könnten drei Kerninteraktionen direkt gezeigt werden:

```text
Super
→ Launcher

Super + Tab
→ Hub / Raumübersicht

Super + Space
→ System Deck
```

Damit versteht der Nutzer bereits einen großen Teil des Bedienmodells.

---

# 37. UX-Kernmodell

Niwoe beantwortet vier zentrale Fragen:

```text
Launcher
→ Was möchte ich tun?

Räume
→ Wo arbeite ich?

Hub
→ Was ist gerade aktiv?

System Deck
→ Was macht mein System?
```

Die Leiste beantwortet:

```text
Wo bin ich gerade?
```

---

# 38. Produktphilosophie

Niwoe soll weder so restriktiv wie ein geschlossenes System noch so chaotisch wie typische frei zusammengesetzte Linux-Setups wirken.

Der Mittelweg:

> **Klare Standards, aber echte Anpassbarkeit innerhalb eines konsistenten Systems.**

Nicht:

```text
Jeder macht alles irgendwie.
```

Aber auch nicht:

```text
Der Hersteller entscheidet alles.
```

Sondern:

```text
Es gibt einen vorgesehenen Weg.

Du darfst ihn anpassen.

Das System bleibt trotzdem kohärent.
```

---

# 39. Designphilosophie

Eine mögliche Kurzform:

> **Ordnung statt Fragmentierung.**  
> **Kontexte statt Fensterstapel.**  
> **Information statt Icon-Sammlung.**  
> **Kontrolle ohne Restriktion.**  
> **Identität ohne Toolkit-Zwang.**

---

# 40. Entwicklungsreihenfolge

Niwoe OS sollte nicht sofort als vollständige eigene Distribution gebaut werden.

Zuerst sollte das neue Desktopmodell im bestehenden technischen Fundament funktionieren.

Sinnvolle Reihenfolge:

```text
1. Raum-Datenmodell
2. Fenster Räumen zuordnen
3. Raumwechsel
4. Raumleiste
5. Preferred / Dedicated Apps
6. Layout Restore
7. Hub
8. System Deck
9. Launcher
10. Settings / Control Center
11. Welcome Wizard
```

Erst danach wird die eigene Linux-Basis sinnvoll.

---

# 41. Danach das eigene Betriebssystem

Wenn der Desktop stabil ist, können die tatsächlichen Systemanforderungen sauber definiert werden:

```text
Kernel-Version
libc
Mesa
Systemdienste
Netzwerk
Audio
Hardware-Support
Libraries
```

Dann schrittweise:

```text
bestehende Distribution als Entwicklungsbasis
↓
definiertes Niwoe Base System
↓
eigene Builds
↓
eigene Paketverwaltung
↓
eigene Releases
↓
Installer / Recovery / Updates
```

---

# 42. Technische Positionierung

Niwoe soll langfristig nicht wahrgenommen werden als:

> „Eine Linux-Distro mit eigenem Theme.“

Sondern eher als:

> **Eigenständiges Desktop-Betriebssystem mit Linux-Kernel.**

Linux ist die technische Basis.

Niwoe ist das System, das der Benutzer wahrnimmt.

---

# 43. Der Kern von Niwoe

Niwoe lässt sich aktuell auf fünf Grundideen reduzieren:

## 1. Kohärenz

Ein definiertes Basissystem, klare Zuständigkeiten und ein einheitlicher Verwaltungsweg.

## 2. Räume

Arbeitskontexte statt anonymer virtueller Desktops.

## 3. Zurückhaltende Fensterverwaltung

Drittanbieter-Apps bleiben Drittanbieter-Apps. Niwoe organisiert sie, ohne sie künstlich umzubauen.

## 4. Eigene visuelle Identität

Dunkelgrün, Gold, Off-White beziehungsweise Elfenbein.

## 5. Information on demand

Launcher, Hub und System Deck erscheinen dann, wenn sie gebraucht werden.

---

# 44. Kurzfassung

```text
NIWOE

Linux unter der Haube.
BSD-artige Ordnung im System.
Räume statt Fensterchaos.
Ein klarer Installationsweg.
Eine ruhige, hochwertige Oberfläche.
Anpassbar, aber nicht chaotisch.
```

---

# 45. Aktueller Status des Namens

**NIWOE** ist der aktuelle festgehaltene Arbeitsname.

Der Name bleibt bestehen, solange keine formale Markenprüfung einen echten Konflikt zeigt.

Vor einer öffentlichen kommerziellen Einführung sollte noch erfolgen:

```text
EUIPO / TMview
DPMAregister
Klasse 9
Klasse 42
Ähnlichkeitssuche
Domain-/Handle-Prüfung
```

Bis dahin:

# **NIWOE**
