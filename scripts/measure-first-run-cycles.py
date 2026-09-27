#!/usr/bin/env python3
"""P11 stress: three series of 20 real Settings/wizard open+cancel cycles.

Each transition waits for its actual correlated protocol response. Timings are
input/IPC round trips including deliberate settling, never optical latency.
"""
import importlib.util
import json
from pathlib import Path
import runpy
import subprocess
import time

u = runpy.run_path(str(Path(__file__).with_name('test-first-run-ui.py')))
ui = u['ui']
spec = importlib.util.spec_from_file_location('usage', ui.ROOT / 'scripts/measure-hub-performance.py')
usage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(usage)
ui.key(*(['escape'] * 5))
connection = ui.p.Connection()
connection.until('room-snapshot')
pids = {name: subprocess.check_output(['pgrep', '-x', name], text=True).strip() for name in ('niwoe', 'niwoe-shell')}
documents = {name: (u['DIRECTORY'] / name).read_bytes() for name in ('rooms.toml', 'panel.toml')}
results = []
with ui.lib['VirtualKeyboard']() as keys:
    def cycle():
        keys.key_down(125); keys.key_down(56); keys.tap(57); keys.key_up(56); keys.key_up(125)
        time.sleep(.3)
        keys.tap(28)
        opened = connection.until('first-run', lambda e: e['request_id'].startswith('first-run-open-'))
        assert opened['error'] is None
        keys.tap(1)
        closed = connection.until('first-run', lambda e: e['request_id'].startswith('first-run-save-'))
        assert closed['error'] is None
        time.sleep(.2)
    for _ in range(5): cycle()
    for run in range(3):
        before = {name: usage.process_usage(pid) for name, pid in pids.items()}
        gpu_before = usage.gpu_time(pids['niwoe'])
        start = time.monotonic()
        samples = []
        for _ in range(20):
            cycle()
            samples.append({name: usage.process_usage(pid)[1] for name, pid in pids.items()})
        seconds = time.monotonic() - start
        after = {name: usage.process_usage(pid) for name, pid in pids.items()}
        gpu_after = usage.gpu_time(pids['niwoe'])
        results.append(dict(run=run, seconds=seconds, cycles=20, before=before, after=after, rss_samples=samples,
                            gpu_engine_percent=None if gpu_before is None or gpu_after is None else 100*(gpu_after-gpu_before)/1e9/seconds))
        print('PASS wizard actual input+response cycles', run, len(samples), flush=True)
connection.close()
for name, value in documents.items(): assert (u['DIRECTORY'] / name).read_bytes() == value
(u['DATA'] / 'wizard-cycles.json').write_text(json.dumps(results, indent=2))
