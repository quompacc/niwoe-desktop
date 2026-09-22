# NIWOE: empfohlene Linux-Entwicklungsbasis

Stand: 21.09.2026. Status: Fedora 44 KDE vom Nutzer installiert und als
Entwicklungsrechner per SSH freigegeben; siehe `NIWOE_LINUX_BASELINE.md`.
Beschlossen ist bereits: bestehende Linux-Distribution für niwoe-desktop,
später eigenes Linux-basiertes NIWOE OS. Der Meridian-Plan ist vollständig abgelöst.

## Empfehlung

Fedora KDE Plasma Desktop in der regulären stabilen Ausgabe. Die offizielle
Downloadseite bietet zum Prüfzeitpunkt Fedora 44 an:
[Fedora KDE herunterladen](https://www.fedoraproject.org/kde/download/).

Für diese Aufgabe ist die Abwägung: aktueller Desktop-Unterbau, feste Releases,
vollständige benutzbare Ausweichsitzung und direkte Installation von Entwicklungs-
und Integrationspaketen. Fedora beschreibt upstreamnahe aktuelle Technik und
ungefähr 13 Monate Unterstützung je Ausgabe:
[Fedora KDE](https://fedoraproject.org/kde/).

Das ist eine technische Empfehlung für das Projekt, kein bereits ausgeführter
Kompatibilitätsnachweis. Konkrete Mesa-/Kernel-/Wayland-Versionen werden bei
Einrichtung aus den installierten Paketen aufgenommen.

## Vergleich

| Distribution | Stärke für NIWOE | Nachteil für diese Entwicklungsphase | Rolle |
|---|---|---|---|
| Fedora | Zeitgemäßer Desktop mit festen Releasegrenzen | Regelmäßige Release-Upgrades | Eingerichteter primärer Entwicklungs-/Hardwarehost; dnf-Pfad getestet |
| Arch | Direkter Zugriff auf aktuelle Pakete; pacman-Pfad existiert | Rollende Paketbasis erschwert konstante Vergleichsstände | Gute Alternative, insbesondere auf bereits eingerichtetem Arch |
| Debian Stable | Langfristig ruhige und gut vergleichbare Basis; apt-Pfad existiert | Neue Grafikfunktionen/Fixes können zusätzliche Backports benötigen | Konservatives zusätzliches Build-/Kompatibilitätsziel |

Arch verlangt zusammenhängende Systemupdates; Teilupgrades sind nicht unterstützt.
Für reproduzierbare Fehlerberichte Paketstände erfassen, nicht einzelne zentrale
Bibliotheken beliebig einfrieren:
[Arch-Systempflege](https://wiki.archlinux.org/title/System_maintenance).

Debian Stable ist aktuell Debian 13 und hat einen fünfjährigen regulären/LTS-
Lebenszyklus. Es ist für Compositorentwicklung grundsätzlich geeignet; die
Wertung als Zweitziel ist eine Abwägung zugunsten einer aktuelleren primären
Desktopbasis, keine behauptete Inkompatibilität:
[Debian-Releases](https://www.debian.org/releases/).

## Entwicklungsaufbau

1. Fedora KDE als normale Arbeits-/Rettungssitzung erhalten. Fedora Workstation
   mit GNOME ist gleichfalls möglich, wenn bevorzugt; NIWOE hängt nicht von KDE ab.
2. Rust-Toolchain und Cargo.lock reproduzierbar dokumentieren. Systempakete über
   die Distribution verwalten. Keine fremden Mesa-Builds als Defaultvoraussetzung.
3. Schnelle UI-/Inputentwicklung über den vorhandenen Winit-/Nested-Pfad.
4. NIWOE als eigene zusätzliche Login-Sitzung für echte DRM/KMS-, libinput-,
   Multi-Monitor-, Lock- und Leistungstests. Der bestehende Displaymanager darf
   zunächst bleiben; eigener Login/Bootpfad ist kein Startblocker für Shellarbeit.
5. Portal-, Polkit- und Sessiondienste pro Sitzung aktivieren; keine parallelen
   konkurrierenden NIWOE-/KDE-Agenten. SELinux eingeschaltet lassen und konkrete
   Zugriffsfehler über korrekte Installation/Labels bzw. eng begrenzte Policy lösen.
6. Bestehende Ubuntu-CI weiterverwenden. Sobald die Fedora-Baseline läuft, Fedora-
   Buildjob ergänzen; Debian Stable als weiteres Kompatibilitätsziel nachziehen.
   Containerjobs beweisen Buildbarkeit, keine DRM-/GPU-/Suspend-Funktion.

Für häufige Änderungen an Sessiondateien, PAM, Helpern und Systemintegration
empfehle ich zunächst die reguläre veränderbare Fedora-Ausgabe. Atomic-/Image-
Ansätze sind interessante Kandidaten für die spätere OS-Phase, würden jetzt aber
zusätzliche Installations-/Deploymentarbeit erzeugen. Das ist eine Workflow-
Abwägung, keine Aussage, dass Compositorentwicklung darauf unmöglich wäre.

## Konkreter Aufwand im vorhandenen Repository

`scripts/install-deps.sh` akzeptiert derzeit nur `auto|pacman|apt`. Fedora wird
noch nicht automatisch eingerichtet. P00 ergänzt bei Fedora-Wahl `dnf`, prüft
reale Paketnamen für Build/Runtime/Hardwaretest und dokumentiert die Installation.
CI läuft bereits auf Ubuntu. Eine neue Distribution benötigt somit überschaubare
Einrichtungsarbeit, aber sie funktioniert nicht nachweislich ohne Anpassung.

Die Entscheidung für Fedora als Werkbank legt weder RPM noch Fedora als Basis
des späteren NIWOE OS fest. Dessen Image-, Paket-, App- und Updatearchitektur wird
nach einer funktionierenden Desktop-Alpha anhand realer Anforderungen entschieden.
