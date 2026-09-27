#!/usr/bin/env bash
# Run after idle measurements finish. Keep exit codes, including failures.
set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
evidence=target/p12-evidence/gates
mkdir -p "$evidence"
: > "$evidence/status.tsv"
failed=0
gate() {
  local name="$1" status
  shift
  "$@" > "$evidence/$name.log" 2>&1
  status=$?
  printf '%s\t%s\n' "$name" "$status" | tee -a "$evidence/status.tsv"
  ((status == 0)) || failed=1
}
gate fmt cargo fmt --all -- --check
gate check cargo check --workspace
gate clippy cargo clippy --workspace --all-targets -- -D warnings
gate tests cargo test --workspace
gate dbus dbus-run-session -- cargo test -p niwoe-polkit authority_restart_reregisters_and_cancels_pending_auth -- --ignored
gate build cargo build --release --workspace --locked
gate session bash scripts/test-session-launcher.sh
gate migration bash scripts/test-install-migration.sh
gate lifecycle bash scripts/test-install-lifecycle.sh
gate portal-helper dbus-run-session -- python3 scripts/test-portal-missing-helper.py
gate python env PYTHONPYCACHEPREFIX=target/p12-pycache python3 -m py_compile scripts/p12-client.py scripts/test-p12-*.py scripts/measure-p12-*.py scripts/prepare-p12-performance.py scripts/summarize-p12-performance.py
exit "$failed"
