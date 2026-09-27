#!/usr/bin/env python3
"""P12 idle samples with explicit before/after runtime state validation.

Usage: measure-p12-idle.py OUTPUT [COUNT]. Keep interrupted results as evidence;
never silently accept an output change or process replacement as stable idle.
"""
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
usage = runpy.run_path(str(ROOT / 'scripts/measure-hub-performance.py'))
live = runpy.run_path(str(ROOT / 'scripts/test-layout-restore.py'))
destination = Path(sys.argv[1])
count = int(sys.argv[2]) if len(sys.argv) > 2 else 3
assert 1 <= count <= 3 and not destination.exists()
pids = {name: subprocess.check_output(['pgrep', '-x', name], text=True).strip()
        for name in ('niwoe', 'niwoe-shell')}
state = live['snapshot']()
assert not state['window-snapshot']['windows'], 'empty desktop required'
print('Warm-up 30 s; no GUI input, hotplug or builds until completion', flush=True)
time.sleep(30)
results = []
for index in range(count):
    before_state = live['snapshot']()
    before = {name: usage['process_usage'](pid) for name, pid in pids.items()}
    gpu_before = usage['gpu_time'](pids['niwoe'])
    start = time.monotonic()
    time.sleep(300)
    after = {name: usage['process_usage'](pid) for name, pid in pids.items()}
    gpu_after = usage['gpu_time'](pids['niwoe'])
    seconds = time.monotonic() - start
    after_state = live['snapshot']()
    valid = before_state == after_state == state
    sample = dict(run=index, seconds=seconds, state_unchanged=valid,
        before_state=before_state, after_state=after_state,
        gpu_engine_percent=None if gpu_before is None or gpu_after is None else
            100 * (gpu_after - gpu_before) / 1e9 / seconds,
        processes={name: dict(cpu_percent=100 * (after[name][0] - before[name][0]) /
            os.sysconf('SC_CLK_TCK') / seconds, rss_before=before[name][1],
            rss_after=after[name][1]) for name in pids})
    results.append(sample)
    destination.write_text(json.dumps(results, indent=2))
    assert valid, 'runtime state changed during idle; sample retained but invalid'
    print('PASS idle sample', index, seconds, flush=True)
