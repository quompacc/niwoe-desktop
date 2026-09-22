# NIWOE - Installation

This guide documents the **currently implemented Linux installation path**. It
does not decide NIWOE's long-term operating system. OpenBSD is the next
real-hardware evaluation target; FreeBSD remains the existing BSD install path.

The WebKit UI runtime described in the active plan is not installed by these
steps yet. They deploy the current native Rust shell.

> **FreeBSD?** FreeBSD has no systemd/logind and uses its own turnkey installer —
> see [docs/FREEBSD.md](docs/FREEBSD.md). The steps below do not apply there.

> **OpenBSD?** Support is not yet claimed and there is no installer. Follow the
> evidence-first checklist in [docs/OPENBSD.md](docs/OPENBSD.md); do not adapt
> Linux commands blindly.

For the full boot experience keep the sibling checkouts next to each other:

```text
~/bootsplash
~/niwoe-desktop
```

`bootsplash` owns the early DRM splash and hands over to `niwoe-login`.
`niwoe-desktop` owns the login manager, compositor, shell, lock screen,
portal backend, polkit agent, PAM files, themes, and install metadata.

## 1. Install dependencies on Arch

From the NIWOE checkout:

```bash
cd ~/niwoe-desktop
scripts/install-deps.sh --manager pacman all
rustup default stable
```

The helper also auto-detects `pacman`, so this is equivalent on a normal Arch
host:

```bash
scripts/install-deps.sh all
```

Dependency modes:

```bash
scripts/install-deps.sh build
scripts/install-deps.sh runtime
scripts/install-deps.sh hardware-test
```

What the dependency sets cover:

- `build`: `base-devel`, `pkgconf`, Rust via `rustup`, PAM, libseat/logind,
  Wayland, libinput, EGL/GLES/GBM/DRM, font and pixman development libraries.
- `runtime`: D-Bus, NetworkManager, Breeze cursor theme, fonts/xkb data,
  Python GTK3 for `niwoe-file-picker`, xdg-desktop-portal, polkit, pam_u2f,
  CUPS client tools, PipeWire/WirePlumber, and XWayland.
- `hardware-test`: libinput diagnostics, DRM/PCI/USB inspection tools,
  `mesa-utils`, `notify-send`, and `jq`.

On Debian/apt systems use the same script with `--manager apt` or auto-detect:

```bash
scripts/install-deps.sh --manager apt all
```

## 2. Build and install NIWOE

The normal local install path is:

```bash
cd ~/niwoe-desktop
scripts/install-local.sh --build
```

To also install the sibling bootsplash checkout:

```bash
scripts/install-local.sh --build --bootsplash ../bootsplash
```

This installs:

- binaries: `niwoe`, `niwoe-shell`, `niwoe-login`, `niwoe-lock`,
  `niwoe-portal`, `niwoe-polkit-agent`, `niwoe-file-picker`
- PAM: `niwoe-login`, `niwoe-login-password`
- themes: `${prefix}/share/niwoe/themes`
- portal metadata: D-Bus service, systemd user unit, `.portal`,
  `niwoe-portals.conf`
- polkit autostart: `/etc/xdg/autostart/niwoe-polkit-agent.desktop`
- login service: `/etc/systemd/system/niwoe-login.service`
- appearance state dir: `/var/lib/niwoe`, owned by the desktop user

Default prefix is `/usr/local`. Override only if the corresponding XDG and D-Bus
search paths on the target system include that prefix:

```bash
scripts/install-local.sh --build --prefix /usr/local
```

The script does not enable the boot login service by default. First verify SSH
or another recovery route.

## 3. Prepare recovery before taking over tty1

Enable a regular getty on tty2 and verify SSH:

```bash
sudo systemctl enable --now getty@tty2.service
systemctl status getty@tty2.service --no-pager
ssh <host> true
```

Verify installed files:

```bash
command -v niwoe niwoe-shell niwoe-login niwoe-lock niwoe-portal niwoe-polkit-agent
command -v niwoe-file-picker
if command -v bootsplash >/dev/null; then bootsplash --help >/dev/null || true; fi
test -f /etc/pam.d/niwoe-login
test -f /etc/pam.d/niwoe-login-password
test -d /usr/local/share/niwoe/themes
test -f /usr/local/share/xdg-desktop-portal/portals/niwoe.portal
test -f /usr/local/share/dbus-1/services/org.freedesktop.impl.portal.desktop.niwoe.service
```

## 4. Enable boot login

Without bootsplash:

```bash
cd ~/niwoe-desktop
scripts/install-local.sh --enable-boot
sudo reboot
```

With bootsplash:

```bash
cd ~/niwoe-desktop
scripts/install-local.sh --enable-boot --bootsplash ../bootsplash
sudo reboot
```

The boot chain is:

- `bootsplash.service` starts early, opens the DRM card, and listens on
  `/run/bootsplash.sock`.
- `niwoe-login.service` replaces `getty@tty1`, authenticates through PAM,
  opens a logind session, and starts `niwoe` as the authenticated user.
- `niwoe` starts `niwoe-shell`; the shell autostarts the polkit agent and
  apps from XDG autostart directories.
- `niwoe-portal` is activated by xdg-desktop-portal through the installed
  portal metadata.

Recovery if login fails: `Ctrl+Alt+F2` should bring up a regular getty on tty2.

## 5. Post-boot checks

After the first reboot:

```bash
systemctl --failed --no-pager
systemctl status --no-pager bootsplash.service niwoe-login.service
sudo journalctl -b -u bootsplash.service -u niwoe-login.service --no-pager
pgrep -a niwoe
```

Inside the logged-in NIWOE session, or over SSH with the user bus exported:

```bash
busctl --user list | grep -E "xdg|portal|niwoe" || true
busctl --user introspect org.freedesktop.impl.portal.desktop.niwoe /org/freedesktop/portal/desktop --no-pager
```

For the first controlled hardware pass, follow `docs/HARDWARE_SMOKE.md`.

## 6. NetworkManager

The panel network tray queries `nmcli`, so NetworkManager must manage the active
interface.

On Arch, enable NetworkManager if it is not already active:

```bash
sudo systemctl enable --now NetworkManager.service
nmcli general status
```

If another network stack owns the interface, migrate deliberately and keep SSH
recovery available. For a simple wired interface:

```bash
ip link
sudo nmcli connection add type ethernet ifname enp1s0 con-name Wired autoconnect yes
sudo nmcli connection up Wired
```

Adjust `enp1s0` to the real interface name.

## 7. Cursor theme

NIWOE defaults to `Breeze_Light` at size `24`. The Arch runtime dependency
installs Breeze. To make it explicit:

```toml
# ~/.config/niwoe/config.toml
[cursor]
theme = "Breeze_Light"
size = 24
```

## Build-only systems

If you only need to compile and run tests:

```bash
scripts/install-deps.sh build
cargo build --workspace
cargo test --workspace
```
