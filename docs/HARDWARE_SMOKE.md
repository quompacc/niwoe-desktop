# Controlled Hardware Smoke

> **Scope note (2026-09-21):** NIWOE targets Linux. Start with
> [the current baseline](NIWOE_LINUX_BASELINE.md), keeping the existing desktop
> and display manager available. The boot-login replacement procedure below is
> historical and optional; it is not a P00 requirement. BSD evidence is historical.

This runbook separates preflight work from checks that can only be proven on
real DRM/input hardware.

## Goal

Get from boot/login to a usable NIWOE desktop once, with recovery available.
Do not combine the first boot with broad hotplug, suspend, or gaming tests.

## Preflight before enabling tty1 login

1. Build and install from the repo:

   ```bash
   scripts/install-deps.sh all
   scripts/install-local.sh --build
   ```

2. Verify recovery:

   ```bash
   ssh <host> true
   systemctl status getty@tty2.service --no-pager
   ```

3. Verify installed files exist:

   ```bash
   command -v niwoe niwoe-shell niwoe-login niwoe-lock niwoe-portal niwoe-polkit-agent
   test -f /etc/pam.d/niwoe-login
   test -f /etc/pam.d/niwoe-login-password
   test -d /usr/local/share/niwoe/themes
   test -f /usr/local/share/xdg-desktop-portal/portals/niwoe.portal
   test -f /usr/local/share/dbus-1/services/org.freedesktop.impl.portal.desktop.niwoe.service
   ```

4. Only then enable boot login:

   ```bash
   scripts/install-local.sh --enable-boot
   ```

## First boot pass criteria

After reboot, verify from the machine and over SSH:

- bootsplash/login appears, or login appears directly if bootsplash is absent
- password fallback works when no registered YubiKey is ready
- the desktop reaches panel + launcher
- keyboard and pointer input work
- `Super+Space` opens the launcher
- a terminal or simple app launches
- `Super+L` starts `niwoe-lock` and unlock succeeds
- logout returns to `niwoe-login`

Collect:

```bash
systemctl --failed --no-pager
systemctl status --no-pager niwoe-login.service
sudo journalctl -b -u niwoe-login.service --no-pager
journalctl --user -b --no-pager | grep -E 'niwoe|portal|polkit|autostart' || true
```

## Portal smoke

Inside the NIWOE session:

```bash
busctl --user --list | grep -E 'xdg|portal|niwoe' || true
busctl --user introspect org.freedesktop.impl.portal.desktop.niwoe /org/freedesktop/portal/desktop --no-pager
```

FileChooser requires a real xdg-desktop-portal client. Screenshot requires the
NIWOE consent modal and should produce a file URI when allowed.

## Hotplug and mode-change follow-up

Only after the first boot is stable, run the hardware-only checks:

1. Change mode/refresh if available and verify pointer corners, window drag,
   panel hit targets, and launcher positioning.
2. Disconnect a non-primary monitor and verify no crash, no lost windows, layer
   shell recovery, workspace snapshot logs.
3. Reconnect it and verify output add, layer recovery, pointer routing, and
   workspace state.
4. Repeat with the primary monitor if recovery access is solid.

Expected log patterns are documented in `docs/DEBUGGING.md` and
`docs/MULTI_MONITOR.md`.

## Stop conditions

Stop and recover through SSH/tty2 if any of these happen:

- login loop on tty1
- no input in both login and desktop
- black screen with no SSH access
- repeated failed units in `systemctl --failed`
- hotplug causes a compositor crash or unrecoverable blank output
