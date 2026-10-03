#!/bin/sh
set -eu
cd /home/eduard/niwoe-desktop
mkdir -p target/r9-primary-gates
python3 target/r9-primary-source-review.py > target/r9-primary-gates/source.log 2>&1
python3 -c 'import json,os; [os.utime(p,None) for p in json.load(open("target/r9-primary-source-hashes.json"))]'
cargo fmt --all -- --check > target/r9-primary-gates/fmt.log 2>&1
cargo check --workspace > target/r9-primary-gates/check.log 2>&1
cargo test --workspace > target/r9-primary-gates/test.log 2>&1
cargo test -p niwoe-tokens --test design_guard > target/r9-primary-gates/design_guard.log 2>&1
cargo clippy --workspace --all-targets -- -D warnings > target/r9-primary-gates/clippy.log 2>&1
cargo build --release -p niwoe -p niwoe-shell > target/r9-primary-gates/release.log 2>&1
sha256sum target/release/niwoe target/release/niwoe-shell > target/r9-primary-gates/release.sha256
printf 'PASS all gates and shell release\n'
