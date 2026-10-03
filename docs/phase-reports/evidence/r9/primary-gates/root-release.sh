#!/bin/sh
set -eu
cd /home/eduard/niwoe-desktop
python3 target/r9-primary-source-review.py > target/r9-primary-gates/source.log 2>&1
cargo build --release -p niwoe -p niwoe-shell > target/r9-primary-gates/release-root.log 2>&1
sha256sum target/release/niwoe target/release/niwoe-shell > target/r9-primary-gates/release-root.sha256
python3 -c 'import hashlib; from pathlib import Path; value=hashlib.sha256(Path("target/release/niwoe").read_bytes()).hexdigest(); assert value != "c09985e39b77bea259fb81aaae8ab92f4933bf50a0a8971756bde146457d6f65"; print("PASS rebuilt root compositor",value)'
