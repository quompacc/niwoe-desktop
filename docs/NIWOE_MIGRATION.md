# NIWOE naming migration (P01)

Own binaries, Rust crates/imports, session files, PAM services, private D-Bus
identities and portal backend selection now use NIWOE names. Public interfaces
such as `org.freedesktop.portal.Desktop`, `org.freedesktop.PolicyKit1` and their
object paths remain standard. The backend service is
`org.freedesktop.impl.portal.desktop.niwoe`. The session desktop identity is
`NIWOE`; its session name is `niwoe`.

## User files

`$XDG_CONFIG_HOME/niwoe` defaults to `~/.config/niwoe`. Relative XDG paths are
ignored. At first configuration access per process, existing owned files in
the legacy `meridian` directory are considered: `config.toml`, `hidden_apps.txt`
and user themes. User themes under `$XDG_DATA_HOME/meridian/themes` are also
considered (default `~/.local/share`). Nothing is created for an absent legacy
profile. Shared KDE/GTK settings, old generated toolkit output, unknown files,
caches and sockets are not migrated.

Each existing NIWOE destination wins, including an invalid file or symlink.
Otherwise the legacy file is backed up under its old directory's
`niwoe-migration-backup/`, validated and published without replacement. Original
files remain. TOML uses the existing typed config/theme parser; hidden-app
files must be UTF-8 without NUL. Theme assets must be regular files. Files over
16 MiB and symlinks are refused. A complete temporary file is synced and linked
atomically into place; concurrent migration cannot overwrite a winner.
Repeated starts never merge documents or replace NIWOE files/backups. Failures
are logged and originals retained. Correct an invalid legacy file explicitly
before restarting; do not delete a NIWOE file expecting automatic merging.

## Environment and live IPC

For user overrides, a present `NIWOE_*` value wins over its `MERIDIAN_*`
counterpart, even when empty. The fallback is read-only and logged diagnostics
use the new variable names. This covers theme paths, DRM/input debugging,
login compositor/device selection, file picker and shell diagnostic flags.
Inherited IPC authentication tokens and private DRM file descriptors use only
the new names: they are internal process contracts, not user configuration.
There is exactly one live compositor endpoint: `$XDG_RUNTIME_DIR/niwoe.sock`.
No legacy listener, duplicate portal backend or duplicate Polkit agent is added.

## Installation and session change

1. Build/test the release version. Log out of the old desktop normally and select
   KDE in the existing display manager. Do not terminate another active session.
2. Run `bash scripts/install-local.sh`. Active legacy processes/targets block
   installation before changes. Retired own binaries and activation metadata
   are moved into `/var/lib/niwoe/legacy-install-backup` with their full paths;
   pre-existing backup conflicts abort rather than overwrite.
3. Log out of KDE and select `NIWOE (development)`. The new session owns its
   portal backend and stops the shared portal frontend on exit only while its
   activation environment still belongs to NIWOE. KDE reactivates its own portal.

`--destdir /absolute/staging/path` exercises installation without sudo or any
service activation; `scripts/test-install-migration.sh` checks fresh installation
and a synthetic legacy installation including repetition. Never use
`--enable-boot` for the additional Fedora development session.

The separately optional FIDO login service now uses `pam://niwoe-login` as its
origin/appid. Old origin-bound registrations need deliberate reenrollment;
there is no authentication bypass or fallback to a second old PAM service.
The tested KDE display-manager login does not use that optional FIDO path.

GitHub remote: `git@github.com:quompacc/niwoe-desktop.git` (verified with
`git ls-remote github`). Codeberg `origin` remains
`ssh://git@codeberg.org/QuompaCC/meridian-desktop.git`; its external rename is
not part of this phase.
