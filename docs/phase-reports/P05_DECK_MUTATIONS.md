# Systemdeck: bestätigte Änderungen, 23.09.2026

Dark und Light sind vom Nutzer visuell freigegeben. Diese Qualitätsrunde
ändert keine Farben, Materialwerte oder Geometrie.

**Nutzerabnahme:** Nach Installation und bestätigter laufender Shell meldet der
Nutzer „passt, schaut gut aus.“ Die Deck-Qualitätsrunde ist damit abgenommen.
Dies ersetzt keine weiter offenen Mehrmonitor-/HiDPI-Hardwarebelege.

## Verhalten

- Lautstärke/Stummschaltung und Energieprofil laufen außerhalb des UI-Threads.
  Die Anzeige übernimmt den zurückgelesenen Zustand, nicht den angeforderten Wert.
- Die vorhandene Audio-Unterzeile zeigt währenddessen „Wird geändert …“ und
  bei Abweichung „Nicht übernommen – erneut versuchen“. Das Energieprofil zeigt
  „Ändert …“ beziehungsweise „Fehler“ in seiner bisherigen Statuszeile.
- Während eines Auftrags sind doppelte Mute-/Profilaktionen gesperrt. Weitere
  Lautstärkewerte ersetzen einen einzigen vorgemerkten Wert. Tastaturwiederholung
  rechnet mit dem letzten gewünschten Wert, nicht einem veralteten Istwert.
- Mausziehen zeigt eine getrennte Vorschau mit „Loslassen zum Übernehmen“;
  die tatsächliche Audioaufnahme und das Panel bleiben dabei unverändert.
  Erst Loslassen startet die Mutation. Schließen verwirft die Vorschau.
- Audio prüft vor Ausführung und nach Rücklesen die Identität des Standardausgangs.
  Fehlender/gewechselter Ausgang wird nicht als erfolgreiche Änderung angezeigt.
  Ein Wechsel zwischen Prüfung und dem plattformspezifischen Default-Sink-Befehl
  bleibt eine bestehende Backend-Race; es gibt keine atomare Gerätebindung.

## Dateien und Call-Flow

`deck_mutation.rs` und seine Tests kapseln begrenzte Aufträge, Rückleseprüfung,
Fehler und Zusammenfassung schneller Reglerwerte. `power_profile.rs` verwendet
den bestehenden Prozesshelper mit zwei Sekunden Frist pro Befehl.

`state/deck_actions.rs`, `state/audio_and_network_popups.rs`, Pointer-/Keyboard-
Handler und die beiden Deck-IPC-Aktionen reichen Anforderungen dorthin weiter.
Der bestehende Tick übernimmt Ergebnisse und zeichnet nur bei Abschluss neu.
`quick_settings_popup.rs`, `deck_controls.rs`, `network_popup.rs` und der
Netzwerkrenderer reichen die Statuswerte in vorhandene Text-/Klickbereiche.
Shell/Init/Main registrieren den Zustand; bestehende UI-Tests erhalten Idle-Werte.

## Performance und Grenzen

Maximal ein Audio- und ein Profilauftrag gleichzeitig; maximal ein vorgemerkter
Lautstärkewert. Keine dauerhaft wartenden Worker, keine neuen Abhängigkeiten,
keine Animation und keine zusätzlichen Renderpässe. Der vorhandene Tick läuft
nur während Pending mit 250 ms statt einer Sekunde. Im Idle werden lediglich
leere optionale Receiver geprüft; es entstehen keine Backendprozesse.
Die Audioadapter behalten ihre Plattformgrenzen und bestehenden Prozessfristen.

Die Änderung betrifft die Systemdeck-Mutationen. Andere Audioeinstellungs- und
Hardwaretastenpfade sind weiterhin separat. Die Profilfunktion bleibt bei einem
fehlenden Backend deaktiviert. Live-Abnahme erst nach Installation.

## Verifikation

**Installation bestätigt:** Der Nutzer hat den Installer ausgeführt; alle sechs
installierten Programme entsprechen dem Release, KDE-/GTK-Dateien unverändert.
Der Watchdog hat Shell 1642 erfolgreich durch 21194 ersetzt. Das erste Prüfskript
traf währenddessen auf den bereits beendeten Prozess und brach bei `/proc/1642/exe`
ab. Der Prozesswechsel wird jetzt bei allen relevanten `/proc`-Lesezugriffen
abgefangen; eine bereits aktuelle Shell wird nicht erneut gestartet.
SSH-Nachprüfung: Shell 21194 ist bytegleich mit dem installierten Release.
Compositor 1626 blieb erhalten. Die unten beschriebene sudo-Blockade ist erledigt;
die interaktive Funktionsabnahme des Decks steht noch aus.

Fedora: `cargo check --workspace --locked`, `cargo test --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` und
`cargo build --release --workspace --locked` erfolgreich; Formatierung geprüft.
Automatische Installation scheitert aktuell ausschließlich an `sudo: Ein Passwort
ist notwendig`. Der fertige Installer liegt in `target/deck-mutations/install.sh`.
Er prüft alle sechs installierten Binärdateien und unveränderte KDE-/GTK-Dateien;
anschließend erneuert er die eindeutig zugeordnete Shell über den Watchdog und
vergleicht die laufende Datei. Bis zur Ausführung ist der neue Shell-Stand nicht live.

Linux-Logs: `target/deck-mutations/{check,test,clippy,build}.log`.
Tests prüfen Rückleseabweichung, Gerätewechsel/fehlendes Gerät, abgebrochenen
Worker, erfolgreiches Ergebnis, Zusammenfassung schneller Reglerwerte sowie
Klick-/Tastaturziele während Pending und nach Fehlern. Bestehende Design-,
Kontrast-, Zentralitäts- und Quellgrößengrenzen bleiben Teil des Workspace-Laufs.
Windows-Workspace-Check scheitert an fehlendem Wayland/pkg-config; maßgeblich
ist die vollständige Linux-Prüfung auf Fedora.
