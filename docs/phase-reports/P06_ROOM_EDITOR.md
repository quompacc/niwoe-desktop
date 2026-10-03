# Räume benennen und sortieren

## Nutzerentscheidung vom 23.09.2026

**Gestalterisch abgelehnt, nicht abgenommen.** Der Nutzer erklärt, dass diese
Umsetzung nicht der verbindlichen Vorgabe entspricht, und beendet die Arbeit
für heute. Der Implementierungsstand `7c8e8b4` bleibt zur Nachvollziehbarkeit
erhalten; er ist keine akzeptierte Designreferenz. Anschließend sind nur
Dokumentation, Commit und Push beauftragt, keine weitere Umsetzung oder Installation.

Die unten genannten bestandenen technischen Tests und der Offscreen-Renderbeleg
belegen keine Übereinstimmung mit den verbindlichen Mockups. Der visuelle Abgleich
war nicht ausreichend. Konkrete Abweichungen sind noch nicht einzeln dokumentiert;
bei Wiederaufnahme müssen Manifest und Mockups gegen die tatsächliche Darstellung
geprüft werden, bevor daraus weitere UI-Arbeit abgeleitet wird.

Der Installer wurde bereitgestellt, die Installation dieses Editor-Builds wurde
im Gespräch nicht bestätigt. Direkte DRM-Bedienabnahme bleibt ebenfalls offen.
Die zuvor bestätigten Material-/Theme-Stände werden durch diese Ablehnung nicht
nachträglich als abgelehnt gewertet.

## Implementierter, nicht abgenommener Stand

Der erste sichtbare Raum-Schritt: Rechtsklick auf einen Raum im Panel oder in
der Raumübersicht öffnet den nativen Editor. Ein neuer Name wird per Enter oder
„Speichern“ übernommen. „Nach links“/„Nach rechts“ ändern die Anzeigenreihenfolge.
„Zurück“ bzw. Escape führen zur Übersicht; ein weiteres Escape schließt sie.
In der geöffneten Übersicht öffnet F2 den Editor des aktiven Raums. Tab/Shift+Tab
und Enter bedienen die Controls; nicht mögliche Richtungen sind deaktiviert.
Beim ersten Tippen wird der bisherige Name ersetzt. Das dunkle Theme bleibt aktiv.

## Verhalten und Grenzen

- Der Compositor ist der einzige Schreiber. Die Shell zeigt bestätigte Snapshots;
  während Speichern läuft, werden weitere Mutationen gesperrt. Fehler erhalten
  den Entwurf und werden im vorhandenen Popup angezeigt.
- Editoröffnung merkt sich die Revision. Ein später eintreffender Snapshot setzt
  einen offenen Entwurf nicht still auf eine neuere Revision. Konflikt und Retry
  sind explizit; nach zehn Sekunden ohne Antwort wird der Stand neu angefordert,
  keine Mutation automatisch wiederholt.
- IDs, Space-Slots und Fensterzuordnung ändern sich nicht beim Umsortieren.
  Panel und Übersicht verwenden echte gespeicherte Namen. Lange Panelnamen werden
  mit Auslassungszeichen gekürzt; der Editor bietet den Namen zum Bearbeiten.
- Nummernshortcuts einschließlich Fallback, Verschieben mit Nummernshortcuts und
  Super+Tab folgen der neuen Anzeigenreihenfolge. Die bestehenden fokussierten
  Output-Pfade bleiben erhalten. Legacy-IPC-Workspace-Nummern bleiben Space-Slots.
- Es bleiben neun Räume. Hinzufügen/Löschen und weitere Raumpräferenzen sind
  weiterhin offen; P06 insgesamt bleibt in-progress.

## Dateien

| Dateien | Änderung |
|---|---|
| `niwoe-ipc/src/rooms.rs`, `lib.rs` | Additive RoomSnapshot-/RoomChange-/Result-Typen mit Revision und Request-ID; bestehende Wire-Felder bleiben |
| `niwoe-compositor/src/room_registry.rs`, `room_registry_tests.rs` | Revision prüfen, Kandidat validieren und speichern, erst dann veröffentlichen; Slot-Abbildung unverändert |
| `niwoe-compositor/src/state/ipc/{rooms,commands,mod}.rs` | Authentifizierte Raum-Commands und vollständige Snapshots, auch beim Reconnect |
| `niwoe-compositor/src/workspace.rs`, `input/keyboard.rs` | Zugriff auf alleinige Registry und Übersetzung von Anzeigenposition zu Slot für Tastaturaktionen |
| `niwoe-shell/src/room_editor{,_draw,_tests}.rs` | Lokaler Entwurf, Bestätigung/Fehler/Timeout, native tokenbasierte Controls, Fokus und Renderbeleg |
| `niwoe-shell/src/workspaces.rs`, `panel_view/{layout,render,rooms}.rs`, `panel_view_tests.rs` | Namen/Reihenfolge aus Snapshots; stabile Klickziele und gekürzte Paneltexte |
| `niwoe-shell/src/wayland/state/{rooms,ipc_events,panel_actions,popups,timers}.rs`, `state.rs` | Editoraktionen, bestätigte Updates, einmaliges Timeout und gezielte Neuzeichnung |
| `niwoe-shell/src/wayland/handlers/keyboard.rs`, `handlers/pointer/panel_and_popups.rs` | Rechtsklick und Tastaturbedienung |
| `niwoe-shell/src/wayland/{ipc,mod,types}.rs`, `render/core.rs`, `main.rs` | Snapshot-Anforderung nach Authentifizierung, typisierte UI-Aktionen und Panel-Datenweitergabe |

## Verifikation

Fedora: `cargo check --workspace --locked`, `cargo test --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` und
`cargo build --release --workspace --locked` erfolgreich. Formatierung geprüft;
Design-/Zentralitäts-/Quellgrößen-Guards Bestandteil des Workspace-Laufs.
Logs: `target/p06-room-ui/`.

Neue Tests: Wire-Roundtrip, Rename/Reorder/Persistenz/Neuladen, stale Revision,
unbekannte IDs, ungültige Namen/Zielpositionen, gescheiterte Speicherung ohne
optimistische Übernahme, offener Entwurf gegen konkurrierenden Snapshot,
zugeordnete Acks, Timeout, ungültige Snapshot-Slots und Editor-Klickgeometrie.

Isolierter Winit-Lauf mit echtem Testfenster:
`target/p01-evidence/nested.xmJw8u`, Exit 0. Rename und Reorder über authentifizierte
IPC erfolgreich, Datei geprüft, stale Mutation abgewiesen, Reconnect-Snapshot
korrekt, Fenster-ID und Space-Slot unverändert. Elternkonfiguration unverändert.
Portal-/AT-SPI-Warnungen des isolierten Profils sind kein Audio-/Portaltest.

Die [native Editor-Ausgabe](../design/evidence/P06/room-editor-native.png) ist ein
visuell geprüfter Offscreen-Renderbeleg, kein Screenshot einer bedienten Sitzung.
Direkte Maus-/Tastaturabnahme auf dem DRM-Desktop steht nach Installation noch aus.
Compositor und Shell wurden geändert: nach Installation neu bei NIWOE anmelden.

Installer auf Fedora: `target/p06-room-ui/install.sh` (Shell-Syntax geprüft).
Letzter Installationsversuch scheiterte vor dem Installieren: `sudo -n true`
verlangt ein Passwort. SSH-Schlüsselzugang funktioniert. Nach der Nutzerablehnung
wird keine weitere Installation angestoßen; der bereitgestellte Installer ist
lediglich ein technisches Artefakt, keine Installationsempfehlung.

## Performance

Keine Animation, kein neuer dauerhafter Worker, kein zusätzlicher Renderpass.
Der vorhandene Shell-Tick kontrolliert lediglich einen optionalen Pending-Zeitpunkt.
Snapshots lösen Repaints aus; Panelnamen werden innerhalb der bestehenden
Widget-/Schriftpfade dargestellt. Änderungen schreiben eine begrenzte Datei mit
fsync nur nach expliziter Aktion, niemals beim Tippen, Ziehen oder Rendern.
Die bisherigen synchronen Config-Speicheroperationen bleiben synchron; unter
langsamer/gestörter Speicherung kann die Commit-Aktion entsprechend dauern.
