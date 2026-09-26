# P06 – Monitorentfernung mit Fenstern, 26.09.2026

Status: **in-progress**, physischer Test hat einen Fehler gefunden.

## Durchführung und tatsächliches Ergebnis

Auf dem DRM-Compositor `e1e21670…` eigene GTK-Anwendung in Raum 3 auf HDMI
gestartet: Hauptfenster, per Super+T schwebendes Fenster, minimiertes Fenster
und Dialog. Floating-Zustand zusätzlich am fehlenden GDK-Tiled-Bit geprüft.
Der Nutzer hat HDMI nach bestätigter Bereitschaft abgezogen und wieder eingesteckt.

Automatische Prüfung `target/p06-unplug-live.py`:

- Entfernen des HDMI-Outputs erkannt. Alle vier Fenster-IDs, Raumzuordnungen,
  Minimierungszustände und Eingabefeldinhalte bleiben erhalten.
- Raum 3 auf dem verbleibenden Output anfordern und minimiertes Fenster
  wiederherstellen: **fehlgeschlagen**. WindowSnapshot meldet global Raum 3,
  der fokussierte verbleibende Output jedoch weiterhin Raum 1. FocusWindow
  weist das minimierte Fenster deshalb als Fenster eines anderen Raums ab.
- Fehlerpfad beendet ausschließlich die Testanwendung. Alle Testfenster
  entfernt; Raumkonfiguration bytegleich. Kein Sitzungsabbruch.
- Das spätere Wiederanschließen ist im Compositor-Log als Output-Add sichtbar,
  der vollständige Fenstervergleich nach Reconnect wurde wegen des vorherigen
  Abbruchs ausdrücklich nicht mehr durchgeführt.

Belege auf Fedora: `target/p06-unplug-live.log`,
`target/p06-unplug-live-evidence/{before,removed,client-before,cleanup}.json`.

## Korrektur

`state/layout/workspace.rs`: `switch_workspace` darf eine Auswahl nicht allein
wegen Übereinstimmung mit dem globalen Kompatibilitätsindex ignorieren. Nach
Monitorentfernung kann der verbleibende Output einen anderen Raum anzeigen.
Bei abweichender Output-Zuordnung wird diese nun aktualisiert, neu gezeichnet
und der konsistente Snapshot gesendet. Der Tastaturpfad
`switch_workspace_for_focused_output` erhält für dieselbe Situation ebenfalls
Neuzeichnen und Snapshot, nachdem er die Zuordnung aktualisiert hat.

Pointer-Grab-/Indexprüfungen bleiben erhalten; keine automatische Migration
in einen anderen Raum, kein zusätzlicher Timer, keine Änderung an Renderreihenfolge,
Konfiguration, Theme, Cargo.toml oder Dependencies. Zusatzarbeit nur bei einer
tatsächlichen Änderung der Raumauswahl. Call-Flow gegen FocusWindow und
WorkspaceOutputState geprüft. Plan und dieser Bericht dokumentieren den Befund.

## Verifikation und Installation

Fedora, jeweils Exitcode 0: `cargo check --workspace`, `cargo test --workspace -q`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all -- --check`, `cargo build --release -p niwoe --locked`.
Logs: `target/p06-output-selection-{check,test,clippy,build}.log`.

Compositor atomar installiert und mit Release per `cmp`/SHA-256 verglichen:
`e1953fc4f7fa45c1ec6ad61b3f77e4a659ab8315deb33980e823ef9b3874a893`.
Shell unverändert (`5636d59c…`). PID 87272 läuft weiter mit `e1e21670…`;
ein regulärer Nutzer-Neulogin ist für die Korrektur erforderlich.

## Verbleibende Live-Abnahme

Korrigierten Compositor aktivieren und denselben Raum auf einem anderen
fokussierten Output auswählen; danach minimiertes Fenster wiederherstellen.
Der gesamte physische Ab-/Anstecktest mit belegten Fenstern bleibt bis zur
erfolgreichen Wiederholung offen. Keine Erfolgsbehauptung allein aus Cargo-Tests.
