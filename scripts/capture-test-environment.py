#!/usr/bin/env python3
"""Print a read-only, shareable test inventory without host/account identifiers."""
import datetime
import hashlib
import json
import platform
import re
import shutil
import subprocess
from pathlib import Path


def run(args):
    try:
        result = subprocess.run(args, capture_output=True, text=True, timeout=20)
        return {"exit_code": result.returncode, "stdout": result.stdout.strip()}
    except (OSError, subprocess.TimeoutExpired) as error:
        return {"exit_code": None, "error": type(error).__name__}


def read(path):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


os_info = {}
for line in (read("/etc/os-release") or "").splitlines():
    if "=" in line:
        key, value = line.split("=", 1)
        if key in ("ID", "VERSION_ID", "PRETTY_NAME", "VARIANT", "VARIANT_ID"):
            os_info[key] = value.strip('"')

rpm = run(["rpm", "-qa", "--qf", "%{NAME}\t%{VERSION}-%{RELEASE}\t%{ARCH}\n"])
packages = []
prefixes = ("mesa-", "kernel-", "nvidia", "akmod-nvidia", "kmod-nvidia",
            "xorg-x11-drv-nvidia", "xorg-x11-drv-nouveau", "linux-firmware",
            "plasma-workspace", "xdg-desktop-portal")
names = {"libdrm", "libinput", "wayland", "xorg-x11-server-Xwayland",
         "glibc", "gcc", "rust", "cargo", "pipewire", "vulkan-loader"}
for line in rpm.get("stdout", "").splitlines():
    fields = line.split("\t")
    if len(fields) == 3 and (fields[0].startswith(prefixes) or fields[0] in names):
        packages.append(dict(name=fields[0], version_release=fields[1], arch=fields[2]))

lspci = run(["lspci", "-Dnnk"])
gpu_blocks = [block for block in re.split(r"\n(?=[0-9a-f]{4}:[0-9a-f]{2}:)",
              lspci.get("stdout", "")) if re.search(
              r"VGA compatible controller|3D controller|Display controller", block)]
drivers = {}
for name in ("i915", "nouveau", "nvidia", "nvidia_drm"):
    base = Path("/sys/module") / name
    drivers[name] = dict(loaded=base.exists(), loaded_version=read(base / "version"),
                        loaded_srcversion=read(base / "srcversion"),
                        module_version=run(["modinfo", "-k", platform.release(), "-F", "version", name]),
                        module_vermagic=run(["modinfo", "-k", platform.release(), "-F", "vermagic", name]))

processes, binaries = {}, {}
for name in ("niwoe", "niwoe-shell"):
    identities = []
    for pid in run(["pgrep", "-x", name]).get("stdout", "").splitlines():
        if not pid.isdigit():
            continue
        item = dict(pid=int(pid), executable_sha256=None, drm_devices=[])
        try:
            item["executable_sha256"] = hashlib.sha256(Path("/proc", pid, "exe").read_bytes()).hexdigest()
            devices = set()
            for file in Path("/proc", pid, "fdinfo").iterdir():
                fields = dict(line.split(":", 1) for line in (read(file) or "").splitlines()
                              if line.startswith(("drm-driver:", "drm-pdev:")))
                if fields:
                    devices.add(tuple(sorted((key.strip(), value.strip()) for key, value in fields.items())))
            item["drm_devices"] = [dict(device) for device in sorted(devices)]
        except OSError:
            pass
        identities.append(item)
    processes[name] = identities
    binary = Path("/usr/local/bin") / name
    binaries[name] = hashlib.sha256(binary.read_bytes()).hexdigest() if binary.exists() else None

toolchain = {}
for name, flag in (("rustc", "-Vv"), ("cargo", "-V")):
    toolchain[name] = run([shutil.which(name) or name, flag])
cpu = next((line.split(":", 1)[1].strip() for line in (read("/proc/cpuinfo") or "").splitlines()
            if line.startswith("model name")), None)
memory = next((line.split(":", 1)[1].strip() for line in (read("/proc/meminfo") or "").splitlines()
               if line.startswith("MemTotal:")), None)
result = dict(captured_at=datetime.datetime.now().astimezone().isoformat(),
              collection="read-only; no package changes, graphics contexts or UI input",
              os=os_info, kernel_release=platform.release(),
              device=dict(vendor=read("/sys/class/dmi/id/sys_vendor"),
                          product_name=read("/sys/class/dmi/id/product_name"),
                          product_version=read("/sys/class/dmi/id/product_version"),
                          cpu=cpu, memory_total=memory),
              selinux=run(["getenforce"]), gpu_pci=gpu_blocks,
              kernel_drivers=drivers, nvidia_proprietary_version_file=read("/proc/driver/nvidia/version"),
              nvidia_drm_modeset=read("/sys/module/nvidia_drm/parameters/modeset"),
              graphics_runtime_packages=sorted(packages, key=lambda package: (package["name"], package["arch"])),
              rpm_query_exit_code=rpm.get("exit_code"), toolchain=toolchain,
              running_processes=processes, installed_binary_sha256=binaries)
if shutil.which("nvidia-smi"):
    result["nvidia_smi"] = run(["nvidia-smi", "--query-gpu=name,driver_version,pci.bus_id", "--format=csv,noheader"])
print(json.dumps(result, indent=2))
