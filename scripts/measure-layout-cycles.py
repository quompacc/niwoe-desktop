#!/usr/bin/env python3
"""Three real-input series of 20 restore-tab open/close cycles after warm-up.

Precondition: owned P09 Layout room configuration visible, General tab selected.
No saving or restoring is triggered. Input timings are not optical latency.
"""
import importlib.util
import json
from pathlib import Path
import runpy
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('usage', ROOT / 'scripts/measure-hub-performance.py')
usage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(usage)
lib = runpy.run_path(str(ROOT / 'scripts/test-login-uinput.py'), run_name='input_library')
lib['KEYS'].update(escape=1, f6=64)
spec = importlib.util.spec_from_file_location('live', ROOT / 'scripts/test-layout-restore.py')
live = importlib.util.module_from_spec(spec)
spec.loader.exec_module(live)
connection = live.Connection()
connection.until('room-snapshot')
pids = {name: subprocess.check_output(['pgrep', '-x', name], text=True).strip()
        for name in ('niwoe', 'niwoe-shell')}
results = []
with lib['VirtualKeyboard']() as keys:
    def cycle():
        keys.tap(64)  # F6 opens restore tab; status read only.
        # Every opening must produce its actual status request, not merely
        # inject a key that a misconfigured virtual device might discard.
        connection.until('layout', lambda e: e['request_id'].startswith('status-'))
        time.sleep(.2)
        keys.tap(1)   # Escape returns to General.
        time.sleep(.2)

    for _ in range(5):
        cycle()
    for run in range(3):
        samples = []
        before = {name: usage.process_usage(pid) for name, pid in pids.items()}
        gpu_before = usage.gpu_time(pids['niwoe'])
        start = time.monotonic()
        for index in range(20):
            cycle()
            samples.append({name: usage.process_usage(pid)[1] for name, pid in pids.items()})
        elapsed = time.monotonic() - start
        after = {name: usage.process_usage(pid) for name, pid in pids.items()}
        gpu_after = usage.gpu_time(pids['niwoe'])
        results.append(dict(run=run, seconds=elapsed, cycles=20, rss_samples=samples,
                            before=before, after=after,
                            gpu_engine_percent=None if gpu_before is None or gpu_after is None
                            else 100 * (gpu_after - gpu_before) / 1e9 / elapsed))
        print('PASS real restore-tab cycles', run, len(samples), flush=True)
Path(ROOT / 'target/p09-layout-cycles.json').write_text(json.dumps(results, indent=2))
connection.close()
