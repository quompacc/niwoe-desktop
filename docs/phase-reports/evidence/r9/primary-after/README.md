# R9/V32: tatsächliche Nachherprüfung am 03.10.2026

Neue echte Login-Sitzung 1431: Compositor PID 647751,
SHA-256 `8f1e15e3678860e9160cc00ee17e61c0650e9c15ae1ecb086fef53e48c3290e3`;
Shell zunächst PID 647769, SHA-256
`010d2ac4b1afd1fe1b4f27270ee1ef1b07b019b4c97a0df9a21580031837da17`.
Neutraler Raumindex 0, keine Fenster, zwei tatsächliche Outputs, 100 Prozent.

Native Primärwahl nach drm-1: Panel und Control Center wechseln auf UHD,
auf FHD bleibt nur Wallpaper. Native Rückwahl nach drm-0: beide Oberflächen
kehren auf FHD zurück, auf UHD bleibt nur Wallpaper. Compositor- und Shell-PID
bleiben während beider Wechsel unverändert. Sieben Aufnahmen tatsächlich
angesehen. Anschließend Originaldateien und rohe Outputs/Fokus exakt
wiederhergestellt; alle Felder von `primary-cleanup.json` sind true.

Der historische Helper beendet sich danach mit Exit 1: seine interne
`owned`-Variable wurde nach der bereits erfolgreichen Wiederherstellung
nicht aktualisiert. Die Meldung `concurrent config edit; stop` ist an dieser
Stelle ein Fixturefehler, kein nachgewiesener fremder Eingriff und kein
fehlgeschlagener nativer Primärwechsel. Der tatsächlich ausgeführte Helper
bleibt unverändert erhalten. Die zwei fehlenden Sitzungsaufnahmen wurden
separat mit `r9-session-tail-review.py` ergänzt, Exit 0.

Nach dem zur Bereinigung verwendeten Watchdog läuft Shell PID 648754 mit
demselben SHA-256. Erste Tailaufnahme: neutraler Desktop ohne wieder geöffneten
Hub. Zweite: regulärer Hub nach Super+Space, weiterhin Loge und neun Räume.
Once-pro-Login-Datei und deren Zeitstempel unverändert; keine Markierung
entfernt oder Sitzung simuliert. Beide Tailbilder tatsächlich angesehen.
Alle neun Bildhashes stehen in `viewed.json`; PNGs und private Rohdaten bleiben
im autorisierten Git-ignorierten Prüfverzeichnis. UHD-Ansichten belegen die
Zuordnung, keine native Pixelschärfe des verkleinerten Bildbetrachters.

Die temporäre Eingabegerätefreigabe wurde nach Prüfung gegen fremde
ACL-Änderungen entfernt; Original-ACL exakt bestätigt. Direkter Appstart aus
der Loge, physischer Hotplug, weitere Layerrollen und andere R9-Pflichtfälle
bleiben offen. Keine Gesamtfreigabe.
