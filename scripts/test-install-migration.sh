#!/usr/bin/env bash
# Staged deployment contract; uses the actual built binaries but no root/services.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
test_root="$(mktemp -d "$(pwd)/target/niwoe-install-test.XXXXXX")"
stage="$test_root/root"
mkdir -p "$stage"
bash scripts/install-local.sh --destdir "$stage" > "$test_root/fresh.log"
test -x "$stage/usr/local/bin/niwoe"
test -x "$stage/usr/local/bin/niwoe-file-picker"
test -f "$stage/etc/pam.d/niwoe-login-password"
test -f "$stage/etc/pam.d/niwoe-lock"
grep -Fx 'DesktopNames=NIWOE;' "$stage/usr/share/wayland-sessions/niwoe.desktop"
grep -Fx 'Hidden=true' "$stage/usr/local/share/niwoe/session-config/autostart/org.kde.xwaylandvideobridge.desktop"
grep -Fx 'DBusName=org.freedesktop.impl.portal.desktop.niwoe' "$stage/usr/local/share/xdg-desktop-portal/portals/niwoe.portal"
test ! -e "$stage/etc/systemd/system/display-manager.service"

mkdir -p "$stage/etc/xdg/autostart"
printf 'legacy session\n' > "$stage/usr/share/wayland-sessions/meridian.desktop"
printf 'legacy agent\n' > "$stage/etc/xdg/autostart/meridian-polkit-agent.desktop"
printf 'legacy binary\n' > "$stage/usr/local/bin/meridian"
bash scripts/install-local.sh --destdir "$stage" > "$test_root/migrate.log"
test ! -e "$stage/usr/share/wayland-sessions/meridian.desktop"
test ! -e "$stage/etc/xdg/autostart/meridian-polkit-agent.desktop"
grep -Fx 'legacy session' "$stage/var/lib/niwoe/legacy-install-backup/usr/share/wayland-sessions/meridian.desktop"
grep -Fx 'legacy binary' "$stage/var/lib/niwoe/legacy-install-backup/usr/local/bin/meridian"
bash scripts/install-local.sh --destdir "$stage" > "$test_root/repeat.log"
grep -Fx 'legacy session' "$stage/var/lib/niwoe/legacy-install-backup/usr/share/wayland-sessions/meridian.desktop"

# Running legacy sessions are refused before any installation mutation.
(
  source scripts/migrate-legacy-install.sh
  DESTDIR=''
  pgrep() { return 0; }
  if legacy_preflight; then exit 1; fi
)
echo "PASS: fresh/staged migration, repeat, legacy-session refusal; evidence: $test_root"
