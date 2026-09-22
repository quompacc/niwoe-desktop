# NIWOE Documentation Index

> Updated 2026-09-21. Linux desktop first, native Rust UI, Linux-based OS later.

## Precedence

When documents disagree, use this order:

1. `../NIWOE_IMPLEMENTATION_PLAN.md` and the current user decisions
2. `meridian_design_manifest.md` for every visual decision
3. `phase-reports/README.md` for current phase status and validation
4. `ARCHITECTURE.md` for current system boundaries
5. focused technical documents
6. dated audits and superseded plans as historical evidence

`AGENTS.md` and `CLAUDE.md` define patch discipline and validation rules.

## Active strategy and architecture

- `../NIWOE_IMPLEMENTATION_PLAN.md` — active phases and acceptance gates
- `NIWOE_LINUX_BASELINE.md` — Linux setup and reproducible validation
- `NIWOE_DESIGN_BRIEF.md` — mockup interpretation and design roles
- `ARCHITECTURE.md` — current native architecture
- `phase-reports/README.md` — current implementation status
- `PROJECT_STATUS.md` — dated pre-NIWOE implementation evidence
- `GUI_CENTRALIZATION_PLAN.md` — binding native token pipeline and DoD
- `meridian_design_manifest.md` — binding visual specification
- `technical-design-guidelines.md` — engineering decision rules

## Platforms and compatibility

- `OPENBSD.md` — historical Acer evaluation, no active roadmap obligation
- `FREEBSD.md` — retained platform implementation reference
- `APP_STACK.md` — external Wayland/XWayland application matrix
- `FRAME_STRATEGY.md` — decoration findings; deferred compatibility context
- `NVIDIA_PASSTHROUGH.md` — Linux/NVIDIA-specific evidence
- `HARDWARE_SMOKE.md` — current Linux controlled-hardware smoke

## Current subsystem references

- `CODE_INDEX.md` — generated source map of the native implementation
- `CONFIGURATION.md` — config format and reload
- `DEBUGGING.md` — current diagnostics and manual test procedures
- `TESTING.md` — native and platform validation
- `PERFORMANCE_RULES.md` / `VISUAL_PERFORMANCE.md` — native UI budgets
- `DESKTOP_SETTINGS_CONTRACT.md` — settings ownership and toolkit export
- `MERIDIAN_LOGIN.md` — current login architecture
- `MULTI_MONITOR.md` / `WORKSPACES.md` — compositor output/workspace policy
- `XDG_PORTALS.md` — portal architecture

## Historical or deferred documents

- `../MERIDIAN_OS_PLAN.md`, `../ROADMAP.md`, `../PLAN.md`, root BSD handoff
- `NATIVE_UI_PLAN.md` — superseded phase sequence
- `AUDIT_2026-05-25.md`
- `AUDIT_2026-06-20.md`
- `SSD_FRAME_PLAN.md`
- `REFACTORING_PLAN.md` (native-shell maintenance only)
- `UI_PLATFORM.md` (retired WebKit prototype architecture)
- `design-reference/webkit-prototype/` (nonnormative visual reference)

Historical documents are kept because their measurements and failed approaches
are useful. Their priority lists do not override the active roadmap.
