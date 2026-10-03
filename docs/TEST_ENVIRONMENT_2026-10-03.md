# Testumgebung beim Parken

Lesend erfasst am **03.10.2026, 21:35 Europe/Berlin**, ohne Paketänderung,
UI-Eingabe oder erzeugten Grafik-Kontext. Maschinenlesbarer Originalnachweis:
[environment.json](phase-reports/evidence/parked-2026-10-03/environment.json).
Sammler: [capture-test-environment.py](../scripts/capture-test-environment.py).
Diese Inventur beschreibt den Parkzustand; frühere Läufe behalten ihre eigenen
Versionen und Binäridentitäten.

| Teil | Tatsächlich erfasster Stand |
| --- | --- |
| Gerät | Acer Aspire F5-573G, DMI-Produktversion V1.15 |
| CPU / RAM | Intel Core i7-7500U, 2,70 GHz; MemTotal 16227136 kB |
| Distribution / Architektur | Fedora Linux 44 KDE Plasma Desktop, x86_64 |
| Aktiver Kernel | `7.2.6-200.fc44.x86_64` (weitere installierte Kernel sind nicht der aktive Kernel) |
| SELinux | Enforcing |
| Intel-GPU | HD Graphics 620, PCI `8086:5916`, `0000:00:02.0`; Treiber `i915` |
| NVIDIA-GPU | GeForce 940MX / GM107, PCI `10de:179c`, `0000:01:00.0`; Treiber `nouveau` |
| Tatsächlicher Compositor-DRM-Pfad | `/proc/647751/fdinfo`: `drm-driver=i915`, `drm-pdev=0000:00:02.0` |
| Kernel-Grafiktreiberversion | `i915` und `nouveau` liefern keine separate Modulversion; vermagic bindet beide an Kernel `7.2.6-200.fc44.x86_64` |
| Mesa DRI / EGL / GL / Vulkan | `26.2.2-6.fc44`, x86_64 |
| libdrm / libinput | `2.4.134-1.fc44` / `1.31.3-1.fc44` |
| XWayland | `24.1.13-1.fc44` |
| Firmware-Pakete | linux-firmware und nvidia-gpu-firmware `20260916-1.fc44` |
| Vulkan-Loader | `1.4.341.0-1.fc44` |
| Rust / Cargo | Fedora `1.98.1-1.fc44`; rustc Commit `48a229cea`, LLVM 22.1.8 |
| GCC / glibc | `16.2.1-2.fc44` / `2.43-8.fc44` |

**Kein proprietärer NVIDIA-Treiber im Nachweis:** `nvidia` und `nvidia_drm`
nicht geladen, `/proc/driver/nvidia/version` fehlt. Das installierte
`nvidia-gpu-firmware`-Paket ist keine proprietäre Treiberfreigabe. Die Intel-DRM-
Messungen belegen weder NVIDIA-Rendering/Offload noch Gaming auf der 940MX.
Ein späterer Wechsel zu einem NVIDIA-Treiber erhält eine eigene Versions- und
Fähigkeitsmatrix; Ergebnisse nicht mit dieser Intel-/nouveau-Basis gleichsetzen.

## Installierte und laufende Produktidentität

Codebasis: `dc5b65dfe04cc58726f53da88f00746a9f4f8e9b`, Releaseprofil.
Sammler vergleicht installierte Dateien und laufende `/proc/PID/exe`-Inhalte;
beide stimmen je Binary überein. PIDs sind Momentwerte, keine dauerhaften IDs.

| Binary | SHA-256 | PID bei Inventur |
| --- | --- | --- |
| Compositor `niwoe` | `8f1e15e3678860e9160cc00ee17e61c0650e9c15ae1ecb086fef53e48c3290e3` | 647751 |
| Shell `niwoe-shell` | `e9bed1136f1dc3c014a498f90b03fd56e531fae687163a0279ab7c383a703451` | 678998 |

Letzte reale Output-/Scale-Prüfung vom selben Tag: `drm-0` FHD 1920×1080/60 Hz
und `drm-1` UHD 3840×2160/30 Hz, beide je als Primary bei 100/150/200 Prozent.
Ursprünglich FHD Primary links, UHD rechts, beide 100 Prozent; Zustand nach
Prüfung wiederhergestellt. Fallweise Modi/Position/Scale und Grenzen:
[Matrixmetadaten](phase-reports/evidence/r9/calm-controls/matrix/cases.json).
Keine zusätzliche Live-Output- oder EDID-Inventur beim Parken behauptet.

## Vergleich beim Wiederaufnehmen

Auf dem Testgerät im Checkout, vor Updates oder einem neuen Testlauf:

```bash
mkdir -p target
python3 scripts/capture-test-environment.py > target/test-environment-new.json
```

Gerät/PCI-IDs, aktiven Kernel und benutzten DRM-Treiber, proprietäre Version
falls vorhanden, Mesa/Firmware/Toolchain und Binärhashes vergleichen. Abweichungen
im neuen Prüfbericht festhalten. Aktuelle Outputmodi/Scale separat erfassen;
die Inventur startet keine GUI-Benchmarks. Fehlende Messwerte bleiben unbekannt.
Zugangsdaten gehören außerhalb des Repositorys.
