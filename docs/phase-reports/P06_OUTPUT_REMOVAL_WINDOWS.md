# P06 – Monitorentfernung mit Fenstern, 26.09.2026

Status: **dieser Live-Abnahmefall bestanden** nach Korrektur und physischer
Wiederholung. Keine pauschale vollständige P06-Abnahme.

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
Shell unverändert (`5636d59c…`). Zum Installationszeitpunkt lief PID 87272
weiter mit `e1e21670…`; der erforderliche Nutzer-Neulogin ist inzwischen erfolgt.

## Erfolgreiche Live-Wiederholung

Neuer Compositor PID 94758, gestartet am 26.09.2026 um 16:47:50, tatsächlicher
Hash entspricht `e1953fc4…` oben. Zunächst ohne Kabelziehen geprüft: links
Raum 1, rechts Raum 3/global Raum 3; anschließend links ebenfalls Raum 3
auswählen und minimiertes Fenster wiederherstellen. Beide Output-Zuordnungen
melden Raum 3, das Fenster ist nicht mehr minimiert. Belege:
`target/p06-live-ui/selection-{before,after}.json`.

Anschließend frische Testfenster, links Raum 1/rechts Raum 3, mit dem Nutzer
den physischen HDMI-Ab-/Ansteckzyklus wiederholt. Automatischer Helfer
`target/p06-unplug-retry.py` meldet sämtliche Prüfungen erfolgreich:

- Nach Entfernen: genau ein Output; alle vier Fenster-IDs, Raumzuordnungen,
  Minimierungszustände und drei Eingabefeldinhalte unverändert.
- Raum 3 auf dem verbleibenden Output erfolgreich ausgewählt; minimiertes
  Fenster wiederhergestellt, während weiterhin nur ein Output vorhanden ist.
- Nach Wiederanschließen: zwei Outputs; dieselben Fenster-IDs, Raumzuordnungen
  und Eingabefeldinhalte. Raum-Snapshot und Konfigurationsdatei unverändert.
- Eigene Testanwendung beendet; keine Testfenster übrig, Konfiguration bytegleich.

Belege: `target/p06-unplug-retry.log`,
`target/p06-unplug-retry-evidence/{before,removed,restored-single-output,
reconnected,client-before,client-after,cleanup}.json` und abschließender
Snapshot `target/p06-live-ui/unplug-retry-final.json`. Der begrenzte Helfer
ist beendet. In dieser Nachprüfung nur Dokumentation geändert; die bisherigen
Cargo-Prüfungen gelten weiterhin für den identischen installierten Build.

Dieser Test deckt den vorhandenen eDP-/HDMI-Aufbau ab. Er ist keine Abnahme
aller Skalierungen, anderer GPUs/Anschlusskombinationen oder der gesamten
P06-Matrix. Der dokumentierte Wiederherstellungsfehler ist live nachgeprüft.
