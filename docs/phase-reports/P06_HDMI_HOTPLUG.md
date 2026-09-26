# P06 – HDMI-Hotplug, 26.09.2026

Status: **in-progress**. Kein Nachweis einer funktionierenden Zweimonitor-Sitzung.

## Live-Befund

Der Nutzer hat HDMI im laufenden Betrieb angeschlossen. Linux meldet
`card2-eDP-1` und `card2-HDMI-A-1` als verbunden; HDMI liefert unter anderem
3840×2160, 3440×1440 und 1920×1080 als Modi. Der unverändert laufende
Compositor PID 56919 (`47a77cd2…`) meldet weiterhin nur `drm-0`, 1920×1080.
Auch Raumwechsel und ein eigener nativer Testclient führten nicht zur Aufnahme
des HDMI-Ausgangs. Der Client wurde beendet, der ursprüngliche Raum 2 wieder
aktiviert. Raumdefinitionen wurden nicht geändert.

Belege auf Fedora: `target/p06-live-ui/hdmi-{connected,after-repaint,
second-repaint,client}.json`. Das ist eine reproduzierte Erkennungslücke, keine
erfolgreiche Hotplug-Abnahme. Die Hypothese „nur fehlendes Repaint“ reicht nach
der Clientprobe nicht als vollständige Erklärung aus.

## Änderung und Call-Flow

- `backend/drm/init/hotplug_events.rs`: Linux-udev-Monitor für den ausgewählten
  DRM-Knoten. `Changed` löst unmittelbar den bestehenden Connector-Abgleich aus.
  Ereignisse anderer GPUs werden ignoriert; kein Ausbau zur Multi-GPU-Verwaltung.
- `backend/drm/init.rs`: Registrierung vor der initialen Connector-Erfassung,
  damit Änderungen während der Initialisierung bereits gepuffert werden.
- `backend/drm/init/hotplug.rs`: explizite Ereignisse umgehen die bisherige
  750-ms-Drossel, damit rasches Ab-/Anstecken nicht vom letzten Scan verdeckt wird.
- `backend/drm/init/event_sources.rs`: VBlank-/Fehlerpfad behält seine bisherige
  Drossel als ergänzenden Abgleich.
- `src/main.rs`: Diagnosen nach stderr, da der laufende Login stdout nach
  `/dev/null` leitet, stderr jedoch im Sitzungsprotokoll aufbewahrt.
- Aktiver Plan und dieser Bericht: neuer Befund und verbleibende Live-Gates.

Zuvor existierte kein eigener Hotplug-Ereignispfad: Connector-Scans wurden nur
bei VBlank oder DRM-Fehlern angestoßen. Der neue Pfad beseitigt diese belegte
Architekturlücke; ob weitere Fehler bei KMS-/Modus-/Surface-Erstellung vorliegen,
muss mit dem neuen Release und nutzbaren Diagnosen auf DRM geklärt werden.
Kein zusätzlicher Timer oder kontinuierliches Polling; zusätzlicher Scan nur
bei expliziter Änderung des ausgewählten Geräts. Keine neue Dependency,
Cargo.toml-, Theme-, Login-/PAM- oder Nutzerkonfigurationsänderung.
Der udev-Pfad ist auf Linux begrenzt; andere Plattformen behalten den Bestand.

## Verifikation

Fedora: `cargo check --workspace`, `cargo test --workspace -q`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo fmt --all -- --check` erfolgreich. Logs:
`target/p06-hotplug-{check,test,clippy}.log`.
Call-Flow gegen Smithays vorhandenen `UdevBackend` und die Connector-Pipeline
geprüft. Diese Prüfungen simulieren keinen echten Monitorwechsel.

`cargo build --release -p niwoe --locked` und
`bash scripts/smoke-nested.sh /run/user/1000/wayland-1`: Exitcode 0.
Nested-Beleg `target/p01-evidence/nested.ENe8NK`; Client-Configure/Buffer und
authentifizierte Shell bestanden, erfasste KDE-/GTK-Konfiguration unverändert.
Die isolierte AT-SPI-Warnung begründet keine Barrierefreiheitsabnahme.
Logs: `target/p06-hotplug-{build,nested}.log`.

Release atomar installiert, per `cmp` und SHA-256 geprüft:
`7cb65e3f176aaa4043ac826dd533b5f9f7abc62d7f7c3dc684462fbf6e367b74`.
Shell unverändert: `5636d59c57dd4c9508ce57fdb75614532c8b52ff27f18485c2d70bb065c7cf18`.
Der laufende Compositor PID 56919 behält bis zum Neulogin den alten Hash
`47a77cd257b3817e0a73123e0addc89412fad6d13d32ffa00f503ff3e642b62e`.

## Offene Live-Gates

Neuen Compositor per regulärem Nutzer-Neulogin aktivieren; danach tatsächliche
Ausgänge/Modi und Logs prüfen. Erst bei zwei aktiven Ausgängen Raumwahl/Fokus,
Fensterzuordnung und anschließend Entfernen/Wiederanschließen prüfen.
Die laufende Nutzersitzung wird nicht automatisch beendet. P06 bleibt offen.
