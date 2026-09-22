#!/usr/bin/env python3
"""Measure an isolated nested release session; never operates on the real session.

Usage: python3 scripts/measure-design.py /run/user/1000/wayland-0 OUTPUT_DIR
Three 300-second idle samples and 20 launcher cycles per sample, after warm-up.
GPU and input-to-present latency are explicitly unknown; IPC-to-commit is measured.
No dependencies beyond Python, dbus-run-session and the built release workspace.
"""
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]


def proc_sample(pid):
    fields = Path(f"/proc/{pid}/stat").read_text().rsplit(")", 1)[1].split()
    return {"ticks": int(fields[11]) + int(fields[12]),
            "rss_kib": int(fields[21]) * os.sysconf("SC_PAGE_SIZE") // 1024}


def inside(evidence):
    runtime = Path(os.environ["XDG_RUNTIME_DIR"])
    if not str(runtime).startswith("/tmp/niwoe-p02-"):
        raise SystemExit("Refusing a non-isolated runtime")
    log_path = evidence / "runtime.log"
    with log_path.open("w") as log:
        process = subprocess.Popen([str(ROOT / "target/release/niwoe")], cwd=ROOT,
                                   stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        try:
            shell_pid = None
            for _ in range(150):
                if process.poll() is not None:
                    raise RuntimeError("nested compositor exited")
                children = Path(f"/proc/{process.pid}/task/{process.pid}/children").read_text().split()
                for child in children:
                    if Path(f"/proc/{child}/comm").read_text().strip() == "niwoe-shell":
                        shell_pid = int(child)
                if shell_pid and "IPC client authenticated as shell" in log_path.read_text():
                    break
                time.sleep(0.2)
            if not shell_pid:
                raise RuntimeError("no shell process")
            env = dict(item.split(b"=", 1) for item in Path(f"/proc/{shell_pid}/environ").read_bytes().split(b"\0") if b"=" in item)
            with socket.socket(socket.AF_UNIX) as ipc:
                ipc.connect(str(runtime / "niwoe.sock"))
                def send(command):
                    ipc.sendall((json.dumps(command) + "\n").encode())
                send({"type": "authenticate", "role": "shell", "token": env[b"NIWOE_IPC_TOKEN"].decode()})
                del env
                # Read-only event snapshots are drained so the measurement peer
                # cannot impose socket backpressure on the compositor.
                ipc.setblocking(False)
                def drain():
                    while True:
                        try:
                            if not ipc.recv(65536):
                                raise RuntimeError("IPC peer closed")
                        except BlockingIOError:
                            return
                def toggle(opening):
                    offset = log_path.stat().st_size
                    start = time.monotonic()
                    send({"type": "toggle-launcher"})
                    marker = "draw_launcher committed:" if opening else "unmap_launcher:"
                    while time.monotonic() - start < 10:
                        drain()
                        with log_path.open() as reader:
                            reader.seek(offset)
                            if marker in reader.read():
                                return (time.monotonic() - start) * 1000
                        time.sleep(0.005)
                    raise RuntimeError(f"No launcher commit: {opening}")
                time.sleep(10)
                for _ in range(3):
                    toggle(True)
                    toggle(False)
                results = []
                for run in range(1, 4):
                    before = {"compositor": proc_sample(process.pid), "shell": proc_sample(shell_pid)}
                    log_start = log_path.stat().st_size
                    started = time.monotonic()
                    while time.monotonic() - started < 300:
                        time.sleep(1)
                        drain()
                    duration = time.monotonic() - started
                    after = {"compositor": proc_sample(process.pid), "shell": proc_sample(shell_pid)}
                    hz = os.sysconf("SC_CLK_TCK")
                    record = {"run": run, "idle_seconds": duration, "gpu": None,
                              "input_to_present_ms": None, "cache_bytes": None,
                              "cpu_percent_one_core": {key: (after[key]["ticks"]-before[key]["ticks"])/hz/duration*100 for key in before},
                              "before": before, "after_idle": after,
                              "idle_log_byte_range": [log_start, log_path.stat().st_size]}
                    record["ipc_to_commit_ms"] = []
                    for _ in range(20):
                        record["ipc_to_commit_ms"].append(toggle(True))
                        toggle(False)
                    record["after_cycles"] = {"compositor": proc_sample(process.pid), "shell": proc_sample(shell_pid)}
                    results.append(record)
                    (evidence / "measurements.json").write_text(json.dumps(results, indent=2))
                    print(f"Completed sample {run}/3", flush=True)
        finally:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()


def main():
    if sys.argv[1] == "--inside":
        inside(Path(sys.argv[2]))
        return
    parent = Path(sys.argv[1]).resolve()
    if not parent.is_socket():
        raise SystemExit("Parent Wayland socket missing")
    evidence = Path(sys.argv[2]).resolve()
    evidence.mkdir(parents=True, exist_ok=True)
    profile = Path(tempfile.mkdtemp(prefix="niwoe-p02-"))
    env = os.environ.copy()
    for key in ("DISPLAY", "SESSION_MANAGER", "DBUS_SESSION_BUS_ADDRESS", "NIWOE_IPC_TOKEN"):
        env.pop(key, None)
    for key, suffix in [("HOME", "home"), ("XDG_RUNTIME_DIR", "runtime"),
                        ("XDG_CONFIG_HOME", "config"), ("XDG_CONFIG_DIRS", "system-config"),
                        ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache")]:
        path = profile / suffix
        path.mkdir(mode=0o700)
        env[key] = str(path)
    env.update(WAYLAND_DISPLAY=str(parent), GSETTINGS_BACKEND="memory",
               NIWOE_THEME_DIR=str(ROOT / "themes"), NIWOE_SHELL_REPAINT_STATS="1",
               RUST_LOG="info,niwoe_shell::wayland=debug")
    (evidence / "profile.txt").write_text(str(profile))
    result = subprocess.run(["dbus-run-session", "--", sys.executable, __file__, "--inside", str(evidence)], env=env)
    raise SystemExit(result.returncode)


if __name__ == "__main__":
    main()
