#!/bin/sh
set -eu
cd /home/eduard/niwoe-desktop
mkdir -p target/r9-calm-install-backup
test -f target/r9-calm-gates/release.sha256
sha256sum -c target/r9-calm-gates/release.sha256
if [ ! -e target/r9-calm-install-backup/niwoe-shell ]; then
    cp -a /usr/local/bin/niwoe-shell target/r9-calm-install-backup/niwoe-shell
fi
install -m 0755 target/release/niwoe-shell /usr/local/bin/.niwoe-shell-r9-calm-new
mv /usr/local/bin/.niwoe-shell-r9-calm-new /usr/local/bin/niwoe-shell
cmp target/release/niwoe-shell /usr/local/bin/niwoe-shell
sha256sum /usr/local/bin/niwoe-shell /usr/local/bin/niwoe
