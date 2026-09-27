#!/usr/bin/env bash
# Remove only files that match a staged installation of this exact release.
# User data, directories, changed files and optional boot integration are retained.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
PREFIX=/usr/local
DESTDIR=
PROFILE=()
while (($#)); do
  case "$1" in
    --prefix) PREFIX="${2:?missing prefix}"; shift 2 ;;
    --destdir) DESTDIR="${2:?missing destdir}"; shift 2 ;;
    --debug) PROFILE=(--debug); shift ;;
    -h|--help)
      echo 'Usage: scripts/uninstall-local.sh [--prefix PATH] [--destdir PATH] [--debug]'
      echo 'Requires the matching built release. Log out into KDE before live removal.'
      echo 'Refuses changed files, symlinks and enabled boot integration; preserves user data.'
      exit 0 ;;
    *) echo "Unknown option: $1" >&2; exit 2 ;;
  esac
done
[[ "$PREFIX" =~ ^/[a-zA-Z0-9_/-]+$ && "$PREFIX" != / &&
   "$(realpath -m -- "$PREFIX")" == "$PREFIX" ]] || {
  echo 'prefix must be a normalized absolute path without symlinks' >&2; exit 2;
}
if [[ -n "$DESTDIR" ]]; then
  [[ "$DESTDIR" == /* && "$DESTDIR" != / &&
     "$(realpath -m -- "$DESTDIR")" == "$DESTDIR" ]] || {
    echo 'destdir must be a normalized absolute staging path without symlinks' >&2; exit 2;
  }
else
  [[ $EUID -eq 0 ]] || { echo 'Use sudo for live removal from a different desktop.' >&2; exit 2; }
  for name in niwoe niwoe-shell niwoe-login niwoe-lock niwoe-portal; do
    if pgrep -x "$name" >/dev/null; then
      echo "Refusing removal while $name is running; log out into KDE first." >&2
      exit 1
    fi
  done
  if pgrep -f '^(/[^ ]*/)?niwoe-polkit-agent([[:space:]]|$)' >/dev/null; then
    echo 'NIWOE Polkit agent still running; finish the NIWOE session first.' >&2; exit 1
  fi
  if systemctl is-enabled --quiet niwoe-login.service; then
    echo 'Boot login enabled: restore the existing display manager/getty before removal.' >&2
    exit 1
  fi
fi

# The existing installer defines the payload; no second list can drift from it.
# This reference is private, never an activation root or the removal target.
reference="$(mktemp -d "$(pwd)/target/niwoe-uninstall-reference.XXXXXX")"
trap 'rm -rf -- "$reference"' EXIT
bash scripts/install-local.sh --destdir "$reference" --prefix "$PREFIX" "${PROFILE[@]}" >/dev/null
declare -a remove=()
blocked=0
while IFS= read -r -d '' source; do
  relative="${source#"$reference"}"
  installed="$DESTDIR$relative"
  # Reject symlinked parents as well as symlinked files; never escape staging.
  if [[ "$(realpath -m -- "$installed")" != "$installed" || -L "$installed" ]]; then
    echo "Refusing symlink/redirected path: $installed" >&2
    blocked=1
  elif [[ -e "$installed" ]]; then
    if [[ ! -f "$installed" ]] || ! cmp -s -- "$source" "$installed"; then
      echo "Retained differing file: $installed" >&2
      blocked=1
    else
      remove+=("$installed")
    fi
  fi
done < <(find "$reference" -type f -print0)
((blocked == 0)) || { echo 'No files removed. Resolve differences or use the matching release.' >&2; exit 1; }
for installed in "${remove[@]}"; do
  rm -- "$installed"
  printf 'Removed %s\n' "$installed"
done
if [[ -z "$DESTDIR" ]]; then
  systemctl daemon-reload
  echo 'Run systemctl --user daemon-reload in your desktop session.'
fi
echo 'NIWOE payload removed; personal data, legacy backups and unowned files retained.'
