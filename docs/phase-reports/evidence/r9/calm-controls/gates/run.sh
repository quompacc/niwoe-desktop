#!/bin/sh
set -eu
cd /home/eduard/niwoe-desktop
mkdir -p target/r9-calm-gates
python3 -c 'import hashlib,json; from pathlib import Path; expected=json.loads(Path("target/r9-calm-source-hashes.json").read_text()); actual={p:hashlib.sha256(Path(p).read_bytes().replace(b"\r\n",b"\n")).hexdigest() for p in expected}; different=[p for p in expected if expected[p]!=actual[p]]; print("Source files",len(actual),"mismatches",different); assert not different; [Path(p).touch() for p in expected]' > target/r9-calm-gates/source.log 2>&1
cargo fmt --all -- --check > target/r9-calm-gates/fmt.log 2>&1
cargo check --workspace > target/r9-calm-gates/check.log 2>&1
cargo test --workspace > target/r9-calm-gates/test.log 2>&1
cargo test -p niwoe-tokens --test design_guard > target/r9-calm-gates/design_guard.log 2>&1
cargo clippy --workspace --all-targets -- -D warnings > target/r9-calm-gates/clippy.log 2>&1
cargo build --release -p niwoe-shell > target/r9-calm-gates/release.log 2>&1
sha256sum target/release/niwoe-shell > target/r9-calm-gates/release.sha256
printf 'PASS calm controls gates and shell release\n'
