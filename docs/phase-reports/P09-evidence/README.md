# P09-Nachweise

- `p09-*.log`: finale Workspace-Prüfungen, expliziter Designguard, separater
  D-Bus-Test, Release-Build und SHA-256-Abgleich von Build/Installation/Prozess.
  Die leere `p09-fmt.log` entspricht einem erfolgreichen stillen fmt-Check.
  Zusätzliche leere Endzeilen der Testlogs wurden für `git diff --check` entfernt;
  die Testausgaben selbst sind unverändert.
- `source-manifest.json`: SHA-256 der 37 Rust-Dateien des Abschlussstands;
  auf dem Fedora-Buildbaum einzeln abgeglichen. `idle-source-manifest.json`
  und `idle-build-identity.log` halten den Messstand `85b785c` vor der letzten
  reinen Ergebnisbeschriftungsänderung fest.
- `results.jsonl`: korrelierte Statusantworten der isolierten Live-Testläufe,
  einschließlich absichtlich ausgelöster Fehler und Wiederholungen. Frühere
  Prüfläufe und der abschließende Lauf sind enthalten; kein Erfolgsfilter.
- `debounce.json`: Zeitstempel vor/bei zwölf relevanten Änderungen sowie nach
  Ablauf der Ruhefrist. Während der Änderungserie kein Schreibvorgang.
- `displays.json`: aktuelle Outputs und danach gespeicherte Geometrie nach
  echten Config-Reloads bei 200 % sowie 150 % mit abgeschaltetem Zweitmonitor.
- `p09-before-context.json`, `p09-after-context.json`: gleiche neun persönlichen
  Räume, keine Fenster, gleicher aktiver Workspace und identische Display-Modi
  für die getrennten neuen P09-Vergleichsmessungen.
- `p09-idle-before.json` und abschließend `p09-idle-after.json`: je drei
  300-Sekunden-Samples mit `scripts/measure-hub-performance.py`.
- `p09-layout-cycles.json`: drei Serien von je 20 echten Tastatureingaben zum
  Öffnen/Schließen des Wiederherstellungstabs nach fünf Warm-up-Zyklen. Jede
  Öffnung bestätigt ihre IPC-Statusantwort. Prozess-Ticks: CLK_TCK=100.
- `p09-ui-*.png`: tatsächliche finale native Oberfläche nach Erfolg, fehlenden
  Fenstern und Abbruch. Keine nachgestellten Mockups.
- `p09-final-clean-welcome.png`, `p09-final-watchdog.png`, `p09-final-session.json`,
  `p09-session-check.log`: neutraler Sitzungszustand und Watchdog-Neustart ohne
  erneuten Willkommens-Hub. Dieser wurde anschließend manuell geöffnet.
- `p09-ui-1366.png`, `p09-final-general.png`: native Test-Renderer-Ausgaben für
  kleine Restore-Fläche bzw. abschließende konsistente Hinweise unter Allgemein.
  Diese beiden Dateien sind Renderfixtures, keine Live-Screenshots.

CPU-Prozent beziehen sich auf einen Kern; GPU-Wert ist deduplizierte summierte
DRM-Engine-Zeit, keine optische Latenz oder elektrische Leistung. P08-Messgrenzen
werden durch diese neue Messung nicht rückwirkend aufgehoben.
