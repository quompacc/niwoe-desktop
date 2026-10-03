# Codekarte für den Wiedereinstieg

Gültig für Produktcode `dc5b65d` vom 03.10.2026. Große Teile wurden mit
Modellunterstützung entwickelt. Ein Modellwechsel ändert weder Produktregeln
noch Abnahmegrenzen. Code, Call-Flow und konkrete Belege beurteilen; eine
Fertigmeldung oder ein grüner Test ersetzt keine reale optische Prüfung.

## Verantwortung und Call-Flow

| Einstieg / Bereich | Verantwortung und wichtige Grenze |
| --- | --- |
| [src/main.rs](../src/main.rs), [Compositor-Backend](../crates/niwoe-compositor/src/backend/mod.rs), [State](../crates/niwoe-compositor/src/state/mod.rs) | `main → backend → state → handlers/render`: DRM/KMS, Wayland/XWayland, Input, Fokus, Stacking und Policy. Renderreihenfolge ist Teil der Korrektheit. |
| [Shell main](../crates/niwoe-shell/src/main.rs), [Wayland-State](../crates/niwoe-shell/src/wayland/state.rs), [Renderkern](../crates/niwoe-shell/src/wayland/render/core.rs) | Separater unprivilegierter Prozess; Events/Dirtyzustand treiben die Darstellung. Keine versteckte dauernde Redrawschleife ergänzen. |
| [Control Center](../crates/niwoe-shell/src/control_center.rs), [State/Routes](../crates/niwoe-shell/src/wayland/state/control_center.rs), [Tastaturpfad](../crates/niwoe-shell/src/wayland/handlers/keyboard/control_center_navigation.rs) | Sieben sichtbare globale Ziele. Darstellung, Bounds/Hit-Tests, IDs und Tastaturreihenfolge zusammen prüfen. |
| [Settings-Navigation](../crates/niwoe-shell/src/settings_view/navigation.rs), [Contentbuilder](../crates/niwoe-shell/src/settings_view/content_builders.rs), [Refresh](../crates/niwoe-shell/src/settings_refresh.rs) | Provider lesen Daten; Seiten komponieren UI. Zeichnung und Eingabepaging teilen die tatsächliche Navigationshöhe; Einzelunterseiten haben keine redundanten Tabs. |
| [Raumverwaltung](../crates/niwoe-shell/src/room_management_view.rs), [Formular](../crates/niwoe-shell/src/room_management_view/configuration/form.rs), [Hub](../crates/niwoe-shell/src/hub_view.rs) | Räume/App-Zuordnungen/Dateiverweise; Dateien und Wiederherstellung lokal zum Raum. Loge ist kein zusätzlicher Raum. |
| [niwoe-tokens](../crates/niwoe-tokens/src/lib.rs), [Config](../crates/niwoe-config/src/config.rs), [Component](../crates/niwoe-ui/src/widget/component.rs) | Zentrale Farbe, Geometrie, Material und Interaktion. Designmanifest einschließlich Nutzerkorrektur 03.10. ist verbindlich. |
| [Typisierte IPC](../crates/niwoe-ipc/src/lib.rs) | Prozess-/Privileggrenzen und Validierung erhalten; keine stillen API-Brüche oder privilegierte Shellaktionen. |
| [Layer-Zuordnung](../crates/niwoe-compositor/src/state/handlers/core/layer_shell/assignment.rs) | Implizite Primary-Wahl bleibt erhalten; vorhandene Layer beim Outputwechsel umordnen, ohne ihre Rollen/Fokusidentitäten neu zu erzeugen. V32-Hin-/Rückwechsel belegt, Hotplug noch offen. |

## Rasterung und Lebensdauer: bekannte Einstiegspunkte

- V23: [Launcher-Renderpfad](../crates/niwoe-shell/src/wayland/render/launcher.rs)
  und [Scale-Callback](../crates/niwoe-shell/src/wayland/handlers/compositor.rs).
  Logischer Canvas, physischer Buffer, Text-/Iconraster und Pointerabbildung
  gemeinsam verfolgen. Die Layoutmatrix belegt keine native HiDPI-Schärfe.
- [Symbolcache](../crates/niwoe-ui/src/effect/symbol.rs): Schlüssel aus Symbol,
  Farbe, physischer Größe; FIFO auf 32 Einträge bis 96×96 begrenzt, Arc-Hits.
  [Iconcache](../crates/niwoe-shell/src/icons/cache.rs) und Glyphen-/Headingcaches
  separat verfolgen. Das sind Prüfstellen, keine bewiesene Ursache des RSS-Zuwachses.
- Neue visuelle Arbeit muss ihr Cache-/Invalidierungs-/Performance-Modell erklären.
  Neue Timer, Animationen, Blurpässe oder Decoding nicht aus optischen Gründen
  ungeprüft in den Event-/Renderpfad aufnehmen.

## Guards und Prüfvertrag bleiben unverändert

| Guard | Was er absichert / Grenze |
| --- | --- |
| [design_guard](../crates/niwoe-tokens/tests/design_guard.rs) | Prüft unerlaubte Designhardcodes in seinen definierten Pfaden. Tokens zentralisieren; Ausnahmen nur begründet gemäß Manifest. |
| [centralization_guard](../crates/niwoe-shell/tests/centralization_guard.rs) | Scan gegen eigene Hover-/Pressed-Farbmischung statt gemeinsamer Interaction. Kein vollständiger formaler Nachweis aller Renderzustände. |
| [source_size_guard](../crates/niwoe-tokens/tests/source_size_guard.rs) | Rust-Dateien unter `crates/` höchstens 600 physische Zeilen. Vor Erweiterung nach Verantwortung modular teilen. |

Beim Wechsel von Modell oder Agent **keinen Guard deaktivieren, Scan verkleinern,
Grenzwert erhöhen oder Regressionassertion entfernen, um einen Lauf grün zu bekommen**.
Kleine Refactors verhaltensgleich halten. Bei Rendering/Input/IPC den betroffenen
Call-Flow manuell prüfen; Linux ist der Nachweishost, Windows die Bearbeitungsumgebung.

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo test -p niwoe-tokens --test design_guard
cargo test -p niwoe-tokens --test source_size_guard
cargo test -p niwoe-shell --test centralization_guard
cargo clippy --workspace --all-targets -- -D warnings
```

Bei Produktänderung passenden Release bauen/installieren und laufende Identität
prüfen; neue realistische Nachherbilder und Cleanup belegen. Historische Logs
unverändert erhalten, fehlgeschlagene Vorläufe als solche kennzeichnen.
CI löst aktuell bei PRs und Push nach `master` aus; ein Push auf
`codex/niwoe-p00` ist allein kein neuer CI-Nachweis.

## Evidenz finden und ihre Grenzen verstehen

- Öffentlich: [aktueller Prüfbericht](phase-reports/evidence/r9/calm-controls/README.md),
  `gates/` mit Tests/Quellhashes/Releaseidentität, `cases.json` und `viewed.json`
  mit tatsächlichen Fall-/Bildhashes, `cleanup.json` und `input-cleanup.json`.
  `gates-first-pass/` und Fehlversuche sind ältere getrennte Stände.
- [Aktueller UI-Audit](phase-reports/UI_VISUAL_AUDIT_2026-10-02.md) ist die
  historische Befundkette; spätere datierte Einträge präzisieren frühere Aussagen.
  Der [aktive Plan](../NIWOE_IMPLEMENTATION_PLAN.md) bestimmt die Reihenfolge.
- Lokal vorhanden: `target/r7-provider-matrix-evidence/` mit privaten Bildern;
  zusätzlich `docs/phase-reports/UI_VISUAL_AUDIT_2026-10-02-evidence/` und
  `docs/phase-reports/evidence/r5/` bis `r8/` mit älteren Bildern/Rohdaten/Helpern.
  Beim Parken lokal in `.git/info/exclude` geschützt, erhalten und nicht gepusht.
  Ein frischer Clone enthält diese Archive nicht. Berichtlinks dorthin sind
  historische lokale Verweise; für erneutes Testen aktuelle Evidenz neu erfassen.
  [Archivübersicht](phase-reports/evidence/parked-2026-10-03/private-archive-summary.json):
  2331 erhaltene Dateien, mit Größen und lokaler SHA-256-Inventur einschließlich
  der neueren privaten Aufnahmen. Vollständiges Manifest unter
  `target/park-private-archive-manifest.json`; dessen Hash steht in der Übersicht.
- Historische UI-Helper sind Ablaufbelege, keine sofort lauffähigen Tests eines
  frischen Clones: mehrere benötigen `target/ui-visual-review-20261002.py`,
  eigene Originalsnapshots und passende damalige Binärhashes. Diese lokalen
  Review-Dateien beim Aufräumen erhalten. Für neue Läufe Fixtures und Identitäten
  passend zum neuen Stand vorbereiten; alte Aktivierungs-/Restorehelper nicht
  ungeprüft erneut ausführen.
- Öffentliche README-Bilder haben eigene [Buildprovenienz](../assets/screenshots/README.md).
  Sie stammen vor der ruhigen Designrunde und beweisen deren neuen Stand nicht.

Screenshots können lokale Benutzer-/Gerätedaten enthalten. Veröffentlichung
benötigt die konkrete Freigabe; neue private Prüfaufnahmen unter `target/` halten.
Metadaten belegen nur die tatsächlich erfassten Fälle. Vor einer vollständigen
Freigabe die offenen Punkte aus [HANDOVER](../HANDOVER.md) bearbeiten.
