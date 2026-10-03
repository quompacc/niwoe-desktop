#!/usr/bin/env bash
# Sourced by install-local.sh. Never terminate a running desktop implicitly.
legacy_preflight() {
  [[ -z "$DESTDIR" ]] || return 0
  local name
  for name in meridian meridian-shell meridian-login meridian-lock meridian-portal; do
    if pgrep -x "$name" >/dev/null; then
      echo "install-local: legacy process $name is running. Log out normally, select KDE, then rerun installation." >&2
      return 1
    fi
  done
  if pgrep -f '^(/[^ ]*/)?meridian-polkit-agent([[:space:]]|$)' >/dev/null; then
    echo 'install-local: legacy Polkit agent is running; finish the old session first.' >&2
    return 1
  fi
  if systemctl --user is-active --quiet meridian-session.target; then
    echo 'install-local: legacy session target is active; finish the old session first.' >&2
    return 1
  fi
}

legacy_retire() {
  local path backup
  local paths=(
    /usr/share/wayland-sessions/meridian.desktop
    /etc/xdg/autostart/meridian-polkit-agent.desktop
    /etc/systemd/system/meridian-login.service
    /etc/pam.d/meridian-login /etc/pam.d/meridian-login-password /etc/pam.d/meridian-lock
    "$PREFIX/share/dbus-1/services/org.freedesktop.impl.portal.desktop.meridian.service"
    "$PREFIX/lib/systemd/user/meridian-portal.service"
    "$PREFIX/lib/systemd/user/meridian-session.target"
    "$PREFIX/share/xdg-desktop-portal/portals/meridian.portal"
    "$PREFIX/share/xdg-desktop-portal/meridian-portals.conf"
  )
  # The optional boot/login appearance marker is data, never an activation file.
  local old_appearance="$DESTDIR/var/lib/meridian/appearance"
  local new_appearance="$DESTDIR/var/lib/niwoe/appearance"
  if [[ ! -e "$new_appearance" && ! -L "$new_appearance" && -f "$old_appearance" && ! -L "$old_appearance" ]]; then
    case "$(cat "$old_appearance")" in
      dark|light)
        backup="$DESTDIR/var/lib/niwoe/legacy-install-backup/var/lib/meridian/appearance"
        "${SUDO[@]}" install -d "$(dirname "$backup")"
        [[ -e "$backup" ]] || "${SUDO[@]}" cp -n -- "$old_appearance" "$backup"
        "${SUDO[@]}" cp -n -- "$old_appearance" "$new_appearance"
        ;;
      *) echo 'install-local: invalid legacy appearance retained without migration' >&2 ;;
    esac
  fi
  local binary
  for binary in meridian meridian-session meridian-shell meridian-login meridian-lock meridian-portal meridian-polkit-agent meridian-file-picker; do
    paths+=("$PREFIX/bin/$binary")
  done
  if [[ -z "$DESTDIR" && -f /etc/systemd/system/meridian-login.service ]]; then
    "${SUDO[@]}" systemctl disable meridian-login.service
  fi
  for path in "${paths[@]}"; do
    [[ -e "$DESTDIR$path" || -L "$DESTDIR$path" ]] || continue
    backup="$DESTDIR/var/lib/niwoe/legacy-install-backup$path"
    if [[ -e "$backup" || -L "$backup" ]]; then
      echo "install-local: backup already exists for $path; refusing to replace either file" >&2
      return 1
    fi
    "${SUDO[@]}" install -d "$(dirname "$backup")"
    "${SUDO[@]}" mv -- "$DESTDIR$path" "$backup"
    echo "install-local: preserved retired legacy file $path in $backup"
  done
}
