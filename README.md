# NIWOE Desktop

NIWOE ist ein experimenteller nativer Rust-Wayland-Desktop und Compositor.
Zuerst entsteht ein kohärenter Desktop auf einer bestehenden Linux-
Distribution; ein eigenes Linux-basiertes NIWOE OS wird erst nach einer
brauchbaren Desktop-Alpha entschieden und geplant.

> **Aktiver Produktpfad (2026-09-21):** Native Rust-Oberflächen für
> Compositor, Shell, Panel, Launcher, Deck und Settings. Der archivierte
> WebKit-Prototyp ist ausschließlich eine visuelle Referenz. Frühere
> NIWOE-/BSD-Strategien sind historische Evidenz, keine offenen Aufgaben.

## Architektur

- Der Smithay-basierte Compositor verantwortet Wayland/XWayland, DRM/KMS,
  Input, Fensterverwaltung, Fokus, Stacking und Policy.
- Die separate native Shell verantwortet Produktkomposition, lokale Eingabe und
  Oberflächenlebenszyklen.
- `niwoe-tokens`, `niwoe-config` und `niwoe-ui` sind bis zur P01-
  Namensmigration die zentrale Design- und Komponentenbasis.
- Privilegierte Aktionen liegen weiterhin hinter kleinen, typisierten
  Services/Helpern; die Shell bleibt unprivilegiert.

Externe Anwendungen bleiben normale Wayland- oder XWayland-Clients. NIWOE
rendert oder ersetzt ihre Toolkit-Oberflächen nicht.

## Aktiver Plan und Design

Die verbindliche Reihenfolge und Phasenabnahme steht in
[NIWOE_IMPLEMENTATION_PLAN.md](NIWOE_IMPLEMENTATION_PLAN.md). Der
[NIWOE Design-Brief](docs/NIWOE_DESIGN_BRIEF.md) präzisiert das
[Designmanifest](docs/niwoe_design_manifest.md). Zwei Themes,
zentrale Tokens und effiziente, eventgetriebene Oberflächen bleiben verbindlich.

Die empfohlene Linux-Werkbank ist Fedora KDE. Der konkrete,
reproduzierbare Einrichtungs- und Messablauf steht in
[docs/NIWOE_LINUX_BASELINE.md](docs/NIWOE_LINUX_BASELINE.md). Ubuntu-CI bleibt
als Buildbasis bestehen. Windows dient der Bearbeitung, nicht als Linux-Nachweis.

## Build und Test

Auf einem dokumentierten Linux-Host:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p niwoe-tokens --test design_guard
cargo test -p niwoe-tokens --test source_size_guard
cargo test -p niwoe-shell --test centralization_guard
git diff --check
```

Vor P01 tragen die Pakete noch ihre technischen `niwoe-*`-Namen.

## Historische Dokumentation

`NIWOE_OS_PLAN.md`, `ROADMAP.md`, `PLAN.md`, `docs/UI_PLATFORM.md` sowie
OpenBSD-/FreeBSD-spezifische Berichte bleiben für Herkunft und technische
Belege erhalten, sind aber keine aktive Produktstrategie. Sie dürfen den
NIWOE-Plan nicht überstimmen.
