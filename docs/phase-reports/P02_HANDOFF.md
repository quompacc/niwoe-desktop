# P02 – Übergabe an Codex in VS Code

Repository: `D:\300_Projekte\310_Aktiv\niwoe-desktop`, Branch `codex/niwoe-p00`.
P02 **in-progress**, P03 nicht begonnen. Vollbericht: [P02.md](P02.md).

Produktcommits:

- `3546c05`: zentrale Paletten, Maße, native Komponenten, Kontrast/Galerie.
- `afd7756`: verbleibende gemeinsame Widgetmaße aus Tokens.
- `02d029b`: Font-Fallback bei fehlenden UI-Glyphen; auf Hardware gefundener Fix.

Letzter Linux-Stand: Format, Check, 1.089 Tests (2 ignoriert), Clippy und Release
bestanden. Alle sechs Binaries auf dem Acer entsprechen dem letzten Build.
Acer: `eduard@192.168.1.203`, `/home/eduard/niwoe-desktop`, kein Git-Checkout.
Zuletzt wieder **KDE**, fünf gemeinsame KDE-/GTK-Dateien unverändert.
Keine Zugangsdaten in Dateien sichern; vorhandene Anmeldung/Agent verwenden.

## Nächste notwendige Arbeit

1. `AGENTS.md`, Plan P02 und Bericht lesen; aktuellen Git-Status prüfen.
2. Echten Scale-Fehler modular beheben: Outputname in NIWOE **drm-0**, nicht
   KDEs eDP-1. Bei 150/200 % halbiertes Panel / abgeschnittene Settings. Relevante
   Pfade: `niwoe-compositor/src/backend/drm/render.rs` (Scale 1.0), Output-Layout,
   Shell-Buffer-/Layer-Konfiguration. Keine P03-Neupositionierung vorziehen.
3. Änderung mit vollständigen Linux-Gates, finalem isoliertem Nested-Smoke und
   echten Output-Screenshots prüfen. Die Offscreen-Galerie ist bereits korrekt,
   ersetzt diesen Integrationsnachweis jedoch nicht.
4. Relevante native Performance nachweisen. Die vorhandenen drei Nested-Reihen
   zeigen ~99,72 % Compositor-CPU auch beim alten Release; Shell 0,04–0,07 %.
   Keine erfundenen GPU-/Input-to-present-Werte. Renderbenchmark ist unverändert.
5. Tatsächlichen Gesamtdiff und Nachweise prüfen, erst dann P02 accepted setzen.

## Testbedienung

Der Nutzer möchte **genau einen manuellen Schritt pro Frage** und knappe Updates.
Screenshot-Erlaubnis wurde erteilt. Der direkte Testpeer kann Aufnahmen erzeugen,
lässt aber die Shell-Consent-Fläche sichtbar; solche Bilder nicht als saubere
Referenzen ausgeben. Regulärer Shell-Buttonpfad oder vorhandenes Screencopy
verwenden. Keine Sperrtests oder Sitzungswechsel ohne Koordination.

Belege: `target/p02-evidence/` lokal und Acer; repräsentative PNGs und Mess-JSON
sind in `docs/design/evidence/P02/` gesichert. Die ersten Dateien mit Namen
`light-1920x1080-150/200.png` sind **ungültige Scale-Nachweise** (falscher Outputname).
`light-real-150/200.png` zeigen den tatsächlichen Fehler, allerdings mit sichtbarer
Consent-Fläche. Vor einem neuen Lauf tatsächliche Output-Snapshots lesen.
