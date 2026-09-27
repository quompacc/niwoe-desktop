#!/usr/bin/env bash
# Real payload, isolated root, no service activation and no personal writes.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
root="$(mktemp -d "$(pwd)/target/niwoe-lifecycle.XXXXXX")"
stage="$root/stage"
mkdir -p "$stage/home/test/.config/gtk-3.0" "$stage/usr/local/bin"
printf 'personal config\n' > "$stage/home/test/.config/gtk-3.0/settings.ini"
printf 'foreign binary\n' > "$stage/usr/local/bin/foreign-app"
bash scripts/install-local.sh --destdir "$stage" > "$root/install.log"
bash scripts/install-local.sh --destdir "$stage" > "$root/update.log"
cp "$stage/usr/local/bin/niwoe-shell" "$root/shell.before"
printf 'modified\n' >> "$stage/usr/local/bin/niwoe-shell"
if bash scripts/uninstall-local.sh --destdir "$stage" > "$root/modified.log" 2>&1; then
  echo 'FAIL: modified payload was accepted' >&2; exit 1
fi
test -x "$stage/usr/local/bin/niwoe"
cp "$root/shell.before" "$stage/usr/local/bin/niwoe-shell"
mv "$stage/usr/local/bin" "$root/redirected-bin"
ln -s "$root/redirected-bin" "$stage/usr/local/bin"
if bash scripts/uninstall-local.sh --destdir "$stage" > "$root/symlink.log" 2>&1; then
  echo 'FAIL: symlinked parent was accepted' >&2; exit 1
fi
test -x "$root/redirected-bin/niwoe"
rm "$stage/usr/local/bin"
mv "$root/redirected-bin" "$stage/usr/local/bin"
bash scripts/uninstall-local.sh --destdir "$stage" > "$root/uninstall.log"
test ! -e "$stage/usr/local/bin/niwoe"
test ! -e "$stage/usr/share/wayland-sessions/niwoe.desktop"
test ! -e "$stage/etc/pam.d/niwoe-lock"
test "$(cat "$stage/usr/local/bin/foreign-app")" = 'foreign binary'
test "$(cat "$stage/home/test/.config/gtk-3.0/settings.ini")" = 'personal config'
bash scripts/uninstall-local.sh --destdir "$stage" > "$root/repeat.log"
bash scripts/install-local.sh --destdir "$stage" > "$root/reinstall.log"
cmp target/release/niwoe "$stage/usr/local/bin/niwoe"
cmp target/release/niwoe-shell "$stage/usr/local/bin/niwoe-shell"
test "$(cat "$stage/usr/local/bin/foreign-app")" = 'foreign binary'
echo "PASS: install/update/refuse modified/refuse symlink/remove/repeat/reinstall; evidence $root"
