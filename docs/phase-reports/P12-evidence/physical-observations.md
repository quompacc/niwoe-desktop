# Physische Beobachtungen, Fedora, 27.09.2026

Benutzerantworten während der aktiven P12-Tests, keine simulierten Ergebnisse:

- WLAN wurde vom Nutzer in der bestehenden KDE-Sitzung eingerichtet.
- Erster HDMI-Abzug auf `e9f9355`: „HDMI abgezogen, ein Fenster fehlt oder
  Anzeige fehlerhaft“, präzisiert als „Nur ein Testfenster sichtbar“.
  Der damalige Skript-PASS prüfte nur Schnittmenge/Input und war für vollständige
  Sichtbarkeit unzureichend. Sichtdiagnose: übergroßes Wayland-Fenster,
  beide gemappten Ursprünge `(0,0)` unter dem Panel. Kein Hotplug-PASS daraus.
- Nach `8970b17`, erneutem Abziehen und verschärfter vollständiger
  Arbeitsflächenprüfung: „Ja, beide Fenster sind vollständig sichtbar“.
- Anschließendes Einstecken: „HDMI wieder eingesteckt, beide Bildschirme zeigen
  ein Bild“. Beide Clients blieben per echter Texteingabe bedienbar.

- Optische Sperrprüfung mit einem Testfenster je Output und gesperrtem
  Shell-SIGKILL/Watchdog: „Nein, auf beiden Bildschirmen blieb alles geschützt“.
  Die Software bestätigte den fortbestehenden Lock und einen neuen Shell-PID.
