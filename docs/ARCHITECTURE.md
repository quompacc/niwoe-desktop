# NIWOE Architecture

> Updated 2026-09-21. Product UI is native Rust; Linux is the product target.
> Existing `niwoe-*` identifiers remain until the P01 naming migration.
> The retired WebKit prototype is retained only as a visual reference.

## Stable system boundary

NIWOE is a Wayland compositor and desktop, not a themed layer over another
desktop. Rust remains responsible for protocol correctness, hardware access,
policy, IPC and privileged integration on Linux and BSD.

External applications remain Wayland/XWayland clients in both the current and
target architecture.

## Current implementation

The current executable path is:

```text
boot/login → niwoe compositor → native niwoe-shell
                                  ├─ panel / launcher / popups / settings
                                  └─ IPC to compositor and system services
```

Important workspace responsibilities:

- `src/main.rs`: backend selection, XWayland, IPC timer and shell watchdog
- `niwoe-compositor`: Wayland server, DRM/Winit backends, input, rendering,
  output/workspace policy and compositor IPC
- `niwoe-shell`: native Rust layer-shell client and current desktop UI
- `niwoe-config`: TOML configuration, themes, outputs and keybindings
- `niwoe-tokens`: authoritative design values and guard tests
- `niwoe-ipc`: shell/compositor contracts
- `niwoe-wm`: workspace, tiling and floating logic
- `niwoe-login`, `niwoe-lock`: authentication/session surfaces
- `niwoe-portal`: portal policy and D-Bus integration
- `niwoe-polkit`: authorization agent
- `niwoe-ui`: reusable native UI primitives

The exact source inventory is generated in `CODE_INDEX.md`.

## Product UI architecture

```text
NIWOE compositor
├─ Wayland / XWayland
├─ DRM/KMS, input and output management
├─ window/workspace policy and effects
└─ typed IPC and supervision
          │
          ▼
native niwoe-shell (unprivileged)
├─ Wayland surface lifecycle and input
├─ niwoe-ui primitives and shared icons
├─ direct niwoe-tokens/config consumption
└─ typed IPC to compositor and small system helpers
          ├─ panel
          ├─ launcher
          ├─ Quick Settings
          └─ later NIWOE system tools
```

The shell is a separate unprivileged Wayland client, not a compositor plugin.
Detailed sequencing and acceptance gates are in `../NIWOE_IMPLEMENTATION_PLAN.md`.

## UI boundary

Panel, launcher and Quick Settings share native primitives and the central Rust
tokens. IPC may grow additively but must not silently break existing decoding.
Login, lock and bootsplash also remain native and keep their narrower security
boundaries.

## Compositor / shell / IPC contract

The compositor owns surfaces, focus, workspaces, outputs, final composition and
policy. UI clients request actions; they do not bypass compositor decisions.

Existing IPC includes window/workspace snapshots, focus, launch, config reload,
quit, thumbnails and screenshot mediation. New bridge/state schemas must reuse
or version these semantic contracts rather than duplicate policy in JavaScript.

## Render order

Visual stacking is correctness and remains:

1. background/wallpaper
2. bottom layer surfaces
3. normal application windows
4. top layer surfaces/panel
5. overlay surfaces/launcher/popups
6. cursor

Changing native surface composition does not authorize reordering.

## Backends and platforms

- DRM/KMS is the authoritative real-session path.
- Winit/nested execution supports development and regression tests.
- Linux is the active development and product platform.
- Existing FreeBSD/OpenBSD adapters remain technical references, not active
  evaluation targets or mandatory release gates.

OS integrations live behind explicit platform adapters. OpenBSD `pledge` and
`unveil` and FreeBSD Capsicum/jails/MAC are not treated as interchangeable APIs.

## Wayland and application boundary

The compositor supports or is developing the expected XDG Shell, Layer Shell,
XDG Decoration, SHM, output, data-device, XWayland, dmabuf/sync, session lock,
idle and output-power paths. Protocol correctness takes priority over
application-specific fixes.

GTK, Qt, browsers, Electron and wxWidgets remain external. See `APP_STACK.md`.

## Design-source flow

```text
niwoe-tokens + niwoe-config
              └─ native niwoe-ui and render consumers
```

There is no parallel product CSS palette or planned WebKit migration.
Archived web assets are historical visual references only.

## Security boundaries

- The native shell runs without root or raw DRM/input handles.
- Privileged operations remain in small Rust services/helpers.
- IPC calls are typed, validated and subject to existing authentication/policy.
- A shell failure must not bypass compositor-owned session locking.

## Performance-sensitive paths

- compositor DRM/Winit render and damage paths
- decorations, wallpaper and captures
- native UI surface commits
- icon/font decode and launcher population
- bridge event fan-out and state serialization

Static UI must be event-driven. Reusable assets are cached with explicit
theme/scale/content invalidation. The documented Linux host provides the
measurement baseline; old BSD results do not substitute for it.

## Related documents

- `../NIWOE_IMPLEMENTATION_PLAN.md` — strategy and execution phases
- `NIWOE_LINUX_BASELINE.md` — setup and validation
- `phase-reports/README.md` — verified implementation status
- `PROJECT_STATUS.md` — historical implementation snapshot
- `OPENBSD.md` / `FREEBSD.md` — platform evidence
- `niwoe_design_manifest.md` — binding visual specification
