#!/usr/bin/env bash
# Tests the installed launch contract without a display, root or system changes.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
mkdir -p target
test_root="$(mktemp -d "$(pwd)/target/niwoe-session-test.XXXXXX")"
cleanup() {
  rm -f -- "${test_root}/bin/meridian" "${test_root}/bin/systemctl" \
    "${test_root}/stopped" "${test_root}/session" "${test_root}/session.desktop"
  rmdir -- "${test_root}/bin" "${test_root}"
}
trap cleanup EXIT
mkdir "${test_root}/bin"
sed "s#@PREFIX@#${test_root}#g" packaging/meridian-session > "${test_root}/session"
sed "s#@PREFIX@#${test_root}#g" packaging/wayland-sessions/meridian.desktop > "${test_root}/session.desktop"
cat > "${test_root}/bin/meridian" <<'EOF'
#!/bin/sh
set -eu
test "${DISPLAY+set}" != set
test "${WAYLAND_DISPLAY+set}" != set
test "$XDG_SESSION_TYPE" = wayland
test "$XDG_CURRENT_DESKTOP" = Meridian
test "$XDG_SESSION_DESKTOP" = meridian
test "$DESKTOP_SESSION" = meridian
test "$XDG_RUNTIME_DIR" = /session-test/runtime
test "$DBUS_SESSION_BUS_ADDRESS" = unix:path=/session-test/bus
test "$XDG_CONFIG_DIRS" = "$SESSION_TEST_ROOT/share/meridian/session-config:$SESSION_TEST_CONFIG_DIRS"
exit 37
EOF
chmod +x "${test_root}/bin/meridian"
cat > "${test_root}/bin/systemctl" <<'EOF'
#!/bin/sh
case "$*" in
  '--user show-environment')
    printf 'XDG_CURRENT_DESKTOP=%s\n' "$SESSION_TEST_DESKTOP" ;;
  '--user stop meridian-session.target' | '--user stop xdg-desktop-portal.service meridian-session.target')
    printf '%s\n' "$*" > "$SESSION_TEST_ROOT/stopped" ;;
  *) exit 1 ;;
esac
EOF
chmod +x "${test_root}/bin/systemctl"
for desktop in Meridian KDE; do
for config_dirs in '' '/session-test/config:/etc/xdg'; do
set +e
DISPLAY=:parent WAYLAND_DISPLAY=parent-wayland \
XDG_CURRENT_DESKTOP=KDE XDG_SESSION_TYPE=x11 \
XDG_RUNTIME_DIR=/session-test/runtime \
DBUS_SESSION_BUS_ADDRESS=unix:path=/session-test/bus \
PATH="${test_root}/bin:$PATH" SESSION_TEST_ROOT="$test_root" \
SESSION_TEST_DESKTOP="$desktop" \
XDG_CONFIG_DIRS="$config_dirs" SESSION_TEST_CONFIG_DIRS="${config_dirs:-/etc/xdg}" \
sh "${test_root}/session"
result=$?
set -e
test "$result" -eq 37
test -f "${test_root}/stopped"
if [[ "$desktop" == Meridian ]]; then
  test "$(cat "${test_root}/stopped")" = '--user stop xdg-desktop-portal.service meridian-session.target'
else
  test "$(cat "${test_root}/stopped")" = '--user stop meridian-session.target'
fi
rm -f "${test_root}/stopped"
done
done
grep -Fx "Exec=${test_root}/bin/meridian-session" "${test_root}/session.desktop"
grep -Fx "TryExec=${test_root}/bin/meridian-session" "${test_root}/session.desktop"
grep -Fx 'DesktopNames=Meridian;' "${test_root}/session.desktop"
grep -Fx 'Hidden=true' packaging/session-config/autostart/org.kde.xwaylandvideobridge.desktop
echo 'PASS: DRM environment, session-local XDG overlay, inherited runtime/bus, exit status, target cleanup and session entry'
