#!/usr/bin/env python3
"""P12 comparable real-input cycles; intentionally not an optical latency test."""
import hashlib
import json
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
label, = sys.argv[1:]
assert label in ('before', 'after')
data = ROOT / 'target/p12-evidence'
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
state = ui['p'].snapshot()
assert not state['window-snapshot']['windows'], 'empty desktop required'
baseline = json.loads((data / 'before-state.json').read_text())
def conditions(snapshot):
    # IDs are session-local and change on a real hotplug. Compare the actual
    # named output, focus, geometry, scale, refresh and modes instead.
    value = json.loads(json.dumps(snapshot))
    outputs = value['output-workspace-snapshot']
    outputs.pop('focused_output_id')
    for output in outputs['outputs']:
        output.pop('output_id')
    return value


assert conditions(state) == conditions(baseline), 'restore baseline rooms, outputs and workspace first'
(data / (label + '-cycles-state.json')).write_text(json.dumps(state, indent=2))
identity = {}
for name in ('niwoe', 'niwoe-shell'):
    pid, = subprocess.check_output(['pgrep', '-x', name], text=True).split()
    identity[name] = dict(pid=pid,
        installed=hashlib.sha256(Path('/usr/local/bin', name).read_bytes()).hexdigest(),
        running=hashlib.sha256(Path('/proc', pid, 'exe').read_bytes()).hexdigest())
    assert identity[name]['installed'] == identity[name]['running']
(data / (label + '-identity.json')).write_text(json.dumps(identity, indent=2))
try:
    ui['manage']()
    ui['click'](430, 370)
    subprocess.run(['python3', str(ROOT / 'scripts/measure-layout-cycles.py')], check=True)
    (data / (label + '-cycles.json')).write_bytes((ROOT / 'target/p09-layout-cycles.json').read_bytes())
finally:
    ui['key'](*(['escape'] * 5))
    assert conditions(ui['p'].snapshot()) == conditions(baseline)
print('PASS P12', label, '3x20 real restore-tab cycles; exact baseline state preserved')
