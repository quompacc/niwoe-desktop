#!/usr/bin/env python3
"""Sample three five-minute idle periods after external warm-up.

Run as the NIWOE session user. Do not drive the GUI or run builds during
measurement. CPU is percent of one core; GPU is summed DRM engine time,
deduplicated by DRM client ID. Unavailable GPU data is JSON null.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time


def process_usage(pid):
    fields = Path(f"/proc/{pid}/stat").read_text().split()
    ticks = int(fields[13]) + int(fields[14])
    rss = int(fields[23]) * os.sysconf("SC_PAGE_SIZE")
    return ticks, rss


def gpu_time(pid):
    clients = {}
    for path in Path(f"/proc/{pid}/fdinfo").iterdir():
        try:
            fields = dict(
                line.split(":", 1)
                for line in path.read_text().splitlines()
                if ":" in line
            )
        except (OSError, ValueError):
            continue
        if "drm-client-id" in fields:
            clients[fields["drm-client-id"].strip()] = fields
    engines = [
        int(value.split()[0])
        for fields in clients.values()
        for key, value in fields.items()
        if key.startswith("drm-engine-") and value.strip().endswith("ns")
    ]
    return sum(engines) if engines else None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    pids = {}
    for name in ("niwoe", "niwoe-shell"):
        pid, = subprocess.check_output(["pgrep", "-x", name], text=True).split()
        pids[name] = pid
    results = []
    for run in range(3):
        start = time.monotonic()
        before = {name: process_usage(pid) for name, pid in pids.items()}
        gpu_before = gpu_time(pids["niwoe"])
        time.sleep(300)
        gpu_after = gpu_time(pids["niwoe"])
        seconds = time.monotonic() - start
        after = {name: process_usage(pid) for name, pid in pids.items()}
        gpu_percent = None
        if gpu_before is not None and gpu_after is not None:
            gpu_percent = 100 * (gpu_after - gpu_before) / 1e9 / seconds
        results.append({
            "run": run,
            "seconds": seconds,
            "gpu_engine_percent": gpu_percent,
            "processes": {
                name: {
                    "cpu_percent": 100 * (after[name][0] - before[name][0])
                        / os.sysconf("SC_CLK_TCK") / seconds,
                    "rss_before": before[name][1],
                    "rss_after": after[name][1],
                }
                for name in pids
            },
        })
        args.output.write_text(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()
