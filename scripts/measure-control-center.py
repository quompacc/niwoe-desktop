#!/usr/bin/env python3
"""Same-host P09/P10: real restore-tab cycles, then three five-minute idle runs.

Install the chosen release and start a new empty NIWOE session beforehand.
No build or other GUI driving may run concurrently. Timings are not optical latency.
"""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
label = sys.argv[1]
assert label in ('controlled-before', 'controlled-after')
state = ui.p.snapshot()
assert not state['window-snapshot']['windows']
assert state['window-snapshot']['active_workspace'] == 0
outputs = state['output-workspace-snapshot']['outputs']
assert outputs[0]['width'] == 1920 and outputs[0]['height'] == 1080
assert all(o['scale_millis'] == 1000 for o in outputs)
conditions = [{k: o[k] for k in ('output_name', 'x', 'y', 'width', 'height', 'scale_millis', 'refresh_millihz')} for o in outputs]
if label == 'controlled-after':
    baseline = json.loads((ui.DATA / 'controlled-before.json').read_text())
    assert conditions == baseline['outputs'], (conditions, baseline['outputs'])
pids = {name: subprocess.check_output(['pgrep', '-x', name], text=True).strip() for name in ('niwoe', 'niwoe-shell')}
hashes = subprocess.check_output(['sha256sum', '/usr/local/bin/niwoe', '/usr/local/bin/niwoe-shell'] + [f'/proc/{pid}/exe' for pid in pids.values()], text=True)
metadata = dict(outputs=conditions, pids=pids, hashes=hashes, started=time.time(), latency='NOT MEASURED: no optical baseline', repaint_counters='NOT MEASURED: no comparable exported counters')
(ui.DATA / (label + '.json')).write_text(json.dumps(metadata, indent=2))
ui.manage()
ui.click(430, 370)  # First real room's full configuration, no test fixture needed.
subprocess.run(['python3', str(ui.ROOT / 'scripts/measure-layout-cycles.py')], check=True)
(ui.DATA / (label + '-cycles.json')).write_bytes((ui.ROOT / 'target/p09-layout-cycles.json').read_bytes())
ui.key(*(['escape'] * 5))
time.sleep(30)
subprocess.run(['python3', str(ui.ROOT / 'scripts/measure-hub-performance.py'), str(ui.DATA / (label + '-idle.json'))], check=True)
assert ui.p.snapshot()['output-workspace-snapshot']['outputs'] == outputs
print('PASS comparable cycles and three 300 s idle samples:', label, flush=True)
