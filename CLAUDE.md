# NIWOE – Regeln für Claude Code

## Unix-Code nicht auf Windows als Produktnachweis validieren

Die lokale Entwicklungsmaschine ist Windows. Die Wayland-Crates kompilieren
dort nicht vollständig, daher laufen Build, Lints, Tests und Runtime-Gates auf
einem dokumentierten Linux-Host. Ein Windows- oder VM-Build ist kein Nachweis
für DRM/KMS, Input, Suspend oder Hardware-Performance.

Bis ein Linux-Testhost eingerichtet und im Phasenbericht erfasst ist, ist der
historische Arch-Host kein aktueller Nachweis. Die empfohlene Werkbank ist
Fedora KDE nach `docs/NIWOE_DEVELOPMENT_DISTRO.md` und
`docs/NIWOE_LINUX_BASELINE.md`.

## Aktive Produktstrategie (2026-09-21)

- NIWOE bleibt ein eigener nativer Rust-Wayland-Compositor. Eigene Crates,
  Programme und Integrationskennungen verwenden `niwoe-*` (P01).
- Linux ist Produktziel. Zuerst entsteht ein nutzbarer NIWOE-Desktop auf einer
  bestehenden Distribution, danach wird ein eigenes Linux-basiertes OS geplant.
- Der archivierte WebKit-Prototyp ist ausschließlich eine visuelle Referenz.
- Die verbindliche Reihenfolge ist `NIWOE_IMPLEMENTATION_PLAN.md`; alte
  NIWOE-/BSD-Pläne sind historische Evidenz.
- Die Shell bleibt unprivilegiert. Privilegierte Aktionen bleiben in kleinen
  Rust-Services/Helpern hinter eng typisierten Grenzen.

## Paketquellen

Nur offizielle Distributions-Repositories verwenden. AUR-Pakete, fremde
Mesa-Builds und ungeprüfte Paketquellen sind nicht zulässig.

## Design – verbindlich

- `docs/niwoe_design_manifest.md` ist das verbindliche Manifest und
  präzisiert `docs/NIWOE_DESIGN_BRIEF.md`.
- Eine zentrale Designquelle: `niwoe-tokens` + `niwoe-config`; keine
  lokalen Farb-, Alpha-, Radius- oder Mix-Hardcodes im Renderer.
- Genau zwei Themes, die sich ausschließlich in den zentralen Farbtabellen
  unterscheiden.
- NIWOE-Wortmarke nur in Welcome, Login und About; kein NIWOE-Kompass und
  keine Markenfläche in Alltags-UI oder Launcher-Button.
- `cargo test -p niwoe-tokens --test design_guard` bleibt grün.

## Vor dem Abschluss einer Rust-Änderung

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- einschlägige Guards

Keine neue Desktopfunktion vor der Reihenfolge Panel → Launcher → Deck. Keine
externen Kernabhängigkeiten ohne Auftrag, keine Heap-Allokation oder Theme-Clones
im Render-Loop.
