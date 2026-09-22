# NIWOE Agent Rules

## Projekt (Kurz)
NIWOE ist ein Rust-Wayland-Compositor mit separatem Shell-Prozess. Produktziel
ist zuerst ein kohärenter nativer Linux-Desktop auf einer bestehenden Linux-
Distribution; ein eigenes Linux-basiertes OS folgt erst nach der Desktop-Alpha.
NIWOE-eigene UI bleibt vollständig nativ in Rust und baut auf
`niwoe-ui`, `niwoe-tokens` und `niwoe-config` auf. Der archivierte
WebKit-Prototyp ist nur visuelle Referenz und kein Produktpfad. Die aktive
Reihenfolge ist ausschließlich `NIWOE_IMPLEMENTATION_PLAN.md`; frühere
NIWOE-/BSD-Roadmaps sind historische Evidenz und keine offenen Pflichten.

## Design – VERBINDLICHE Vorgabe (gilt für jede UI-/Render-Änderung)
Die Datei **`docs/niwoe_design_manifest.md` ist die
maßgebliche Design-Spezifikation.** Sie präzisiert `docs/NIWOE_DESIGN_BRIEF.md`.
Jede Änderung an Aussehen, Farben, Geometrie oder Effekten MUSS ihr entsprechen.
Bei Konflikt schlägt das Manifest jede andere Quelle (Audits, Altcode).

Daraus abgeleitete, nicht verhandelbare Invarianten:
- **Eine** zentrale Design-Quelle: `niwoe-tokens` (`Palette`, `Interaction`,
  `Elevation`, `Radius`) + `niwoe-config` (`Decorations`). Jedes UI-Element
  zieht Farbe/Alpha/Geometrie/Radius/Effekt **ausschließlich** daraus.
- **Kein hartverdrahteter Farb-/Alpha-/Radius-/Mix-Wert im Render-Code** außerhalb
  dieser Quelle. Ausnahmen nur für Marken-Assets/Icons und Tests, und nur explizit
  via `// guard:allow: <grund>` bzw. `guard:allow-file` begründet.
- Genau **2 Themes (hell/dunkel)**, identisch bis auf Farben (Layout, Geometrie,
  Radien, Glas/Blur/Schatten gleich). Theme-Wechsel = nur Farbtabelle tauschen.
- **Branding nur als NIWOE-Wortmarke in Welcome, Login und About**; keine
  NIWOE-Kompassgrafik und keine Markenfläche in der Alltags-UI, Taskbar oder
  dem Launcher-Button.
- **Guard-Test muss grün bleiben:** `cargo test -p niwoe-tokens --test design_guard`
  schlägt bei neuen Hardcodes fehl. Roten Guard nie ignorieren — entweder
  zentralisieren oder bewusst mit `// guard:allow: <grund>` freigeben.
- Definition of Done für Zentralität: `docs/GUI_CENTRALIZATION_PLAN.md` §9.
- Die acht Mockups unter `assets/` sind gemäß Nutzerentscheidung vom 22.09.2026
  visuell verbindlich; Abweichungen nur bei konkret dokumentierter technischer
  Notwendigkeit oder ausdrücklicher Nutzerkorrektur. Launcher/Suche orientieren
  sich auf Nutzerwunsch an Apple Spotlight. Native Produkt-UI bezieht weiterhin
  alle Designwerte direkt aus `niwoe-tokens` + `niwoe-config`.

## Harte Regeln für Codex
1. Keine Feature-Änderung ohne expliziten Auftrag.
2. Bestehende Architekturpfade respektieren (`main -> backend -> state -> handlers/render`).
3. Keine stillen API-Brüche zwischen Crates.
4. Refactors nur modular, verhaltensgleich, klein und testbar.
5. Große Dateien in Module splitten statt Logik neu schreiben.
6. Render order is part of correctness. Do not reorder render elements unless the task explicitly requires it and the visual stacking rules are preserved.
7. Visual quality matters, but not at the cost of idle CPU/GPU usage.
8. Prefer cached visual assets over per-frame recomputation.
9. Do not add animations, blur, shadows, or icon decoding without cache/invalidation strategy.
10. Every visual feature must explain its performance model.
10a. Die native Shell bleibt unprivilegiert; privilegierte Aktionen bleiben in
    kleinen Rust-Services/Helpern mit engen typisierten IPC-Grenzen.
10b. Keine große neue Desktop-Funktion vor der nativen Qualitätsrunde
    Panel -> Launcher -> Quick Settings.
11. After every Rust code change, run at least `cargo check --workspace`.
12. If tests were added or changed, run `cargo test --workspace`.
13. For formatting-sensitive Rust changes, run `cargo fmt`.
14. A task with Rust changes is not complete until the check/test results are reported.
15. Rust source files must stay at or below 600 physical lines. Split by
    responsibility before adding code to an oversized file; refactors must
    remain behavior-preserving and be committed in small, tested steps.
15. Plattformannahmen explizit kapseln; OpenBSD- und FreeBSD-Sicherheitsmodelle
    nicht künstlich gleichsetzen.

## Erlaubt
- Rust-Code in betroffenen Modulen ändern.
- Tests ergänzen/aktualisieren.
- Dokumentation und Projektregeln pflegen.
- Logging ergänzen, wenn zur Diagnose nötig.

## Verboten
- `Cargo.toml`-Änderungen ohne Auftrag.
- Neue Dependencies ohne Auftrag.
- Destruktive Git-Operationen.
- Unbegründete Performance-Regressionen.
- “Fixes” ohne `cargo test --workspace` (außer explizit untersagt).

## Pflichtchecks
1. Nach Rust-Codeänderung mindestens `cargo check --workspace`.
2. Bei Test-/Logikänderungen zusätzlich `cargo test --workspace`.
3. Bei formatierungssensitiven Änderungen `cargo fmt`.
4. Bei Rendering/Input/IPC: betroffene Pfade manuell gegen Call-Flow prüfen.

## Berichtformat
1. Geänderte Dateien.
2. Was geändert wurde (pro Datei, 1-3 Zeilen).
3. Verifikation (Befehle + Ergebnis).
4. Offene Risiken/Annahmen.
