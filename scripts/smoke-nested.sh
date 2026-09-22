#!/usr/bin/env bash
# Isolated Linux nested smoke. Requires built workspace, KDE/Wayland, zenity,
# wayland-info and dbus-run-session. Never installs or activates a desktop session.
set -euo pipefail

if [[ "${1:-}" == --inside ]]; then
  repo="$2"
  evidence="$3"
  cd "$repo"
  RUST_LOG=info "target/${NIWOE_SMOKE_PROFILE:-release}/niwoe" > "$evidence/compositor.log" 2>&1 &
  compositor_pid=$!
  cleanup_inner() {
    kill "$compositor_pid" 2>/dev/null || true
    wait "$compositor_pid" 2>/dev/null || true
  }
  trap cleanup_inner EXIT
  socket=""
  for ((attempt=0; attempt<100; attempt++)); do
    kill -0 "$compositor_pid" || { echo 'FAIL: compositor exited'; exit 1; }
    socket="$(find "$XDG_RUNTIME_DIR" -maxdepth 1 -type s -name 'wayland-*' -print -quit)"
    if [[ -n "$socket" ]] && grep -q 'IPC client authenticated as shell' "$evidence/compositor.log"; then
      break
    fi
    sleep 0.2
  done
  [[ -n "$socket" ]]
  grep -q 'Detected parent display' "$evidence/compositor.log"
  grep -q 'IPC client authenticated as shell' "$evidence/compositor.log"
  export WAYLAND_DISPLAY="$socket"
  timeout 10s wayland-info > "$evidence/wayland-info.log" 2>&1
  set +e
  GDK_BACKEND=wayland GTK_USE_PORTAL=0 GIO_USE_VFS=local WAYLAND_DEBUG=client \
    timeout 8s zenity --info --title='NIWOE P01 smoke' --text='NIWOE nested test client' \
    > "$evidence/client.log" 2>&1
  client_exit=$?
  set -e
  [[ "$client_exit" == 0 || "$client_exit" == 124 ]]
  grep -Eq 'xdg_surface.*configure\(' "$evidence/client.log"
  grep -Eq 'wl_surface.*attach\(wl_buffer' "$evidence/client.log"
  kill -0 "$compositor_pid"
  echo 'PASS: nested compositor, authenticated shell, Wayland globals, configured client buffer'
  exit 0
fi

[[ "$(uname -s)" == Linux && "$EUID" != 0 ]] || { echo 'Run as a normal Linux user'; exit 2; }
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
for command in dbus-run-session wayland-info zenity timeout setsid; do
  command -v "$command" >/dev/null || { echo "Missing: $command"; exit 2; }
done
profile_name="${NIWOE_SMOKE_PROFILE:-release}"
[[ "$profile_name" == release || "$profile_name" == debug ]]
[[ -x "$repo/target/$profile_name/niwoe" && -x "$repo/target/$profile_name/niwoe-shell" ]]
parent_socket="${1:-${WAYLAND_DISPLAY:-}}"
[[ -n "$parent_socket" ]] || { echo 'Pass the parent Wayland socket'; exit 2; }
[[ "$parent_socket" == /* ]] || parent_socket="${XDG_RUNTIME_DIR:?}/$parent_socket"
[[ -S "$parent_socket" ]] || { echo 'Parent Wayland socket is not available'; exit 2; }
mkdir -p "$repo/target/p01-evidence"
evidence="$(mktemp -d "$repo/target/p01-evidence/nested.XXXXXX")"
profile="$(mktemp -d /tmp/niwoe-p01.XXXXXX)"
mkdir -m 700 -p "$profile/home/.config" "$profile/home/.local/share" \
  "$profile/runtime" "$profile/system-config" "$profile/cache"
printf '%s\n' "$profile" > "$evidence/profile-path.txt"
record_parent() {
  systemctl --user show-environment | grep -E '^(WAYLAND_DISPLAY|XDG_CURRENT_DESKTOP|XDG_SESSION_TYPE)=' | sort
  for config in "$HOME/.config/kdeglobals" "$HOME/.config/gtk-3.0/settings.ini" "$HOME/.config/gtk-4.0/settings.ini" "$HOME/.config/gtk-3.0/gtk.css" "$HOME/.config/gtk-4.0/gtk.css"; do
    if [[ -f "$config" ]]; then sha256sum "$config"; else printf 'absent: %s\n' "$config"; fi
  done
}
record_parent > "$evidence/parent-before.txt"
session_pid=""
cleanup() {
  if [[ -n "$session_pid" ]]; then
    kill -TERM -- "-$session_pid" 2>/dev/null || true
    sleep 1
    kill -KILL -- "-$session_pid" 2>/dev/null || true
    wait "$session_pid" 2>/dev/null || true
  fi
  # Keep isolated files beside the log reference for inspection; no recursive delete.
}
trap cleanup EXIT
setsid timeout --kill-after=5s 45s env -u DISPLAY -u SESSION_MANAGER \
  HOME="$profile/home" XDG_CONFIG_HOME="$profile/home/.config" \
  XDG_CONFIG_DIRS="$profile/system-config" XDG_DATA_HOME="$profile/home/.local/share" \
  XDG_CACHE_HOME="$profile/cache" XDG_RUNTIME_DIR="$profile/runtime" \
  WAYLAND_DISPLAY="$parent_socket" GSETTINGS_BACKEND=memory \
  dbus-run-session -- bash "$repo/scripts/smoke-nested.sh" --inside "$repo" "$evidence" \
  > "$evidence/result.log" 2>&1 &
session_pid=$!
result=0
wait "$session_pid" || result=$?
cleanup
session_pid=""
record_parent > "$evidence/parent-after.txt"
if ! cmp -s "$evidence/parent-before.txt" "$evidence/parent-after.txt"; then
  echo 'FAIL: parent environment/configuration changed' >> "$evidence/result.log"
  result=1
fi
printf '%s\n' "$result" > "$evidence/result.exit"
cat "$evidence/result.log"
echo "Evidence: $evidence"
exit "$result"
