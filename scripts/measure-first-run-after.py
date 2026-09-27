#!/usr/bin/env python3
"""Compare final P11 to recorded final P10 on the restored personal session."""
import json
from pathlib import Path
import runpy
import subprocess
import time

ui = runpy.run_path(str(Path(__file__).with_name('test-control-center.py')))
p = ui['p']
data = ui['ROOT'] / 'target/p11-live'
before = json.loads((data / 'before-state.json').read_text())
after = p.snapshot()
fields = ('output_name', 'x', 'y', 'width', 'height', 'scale_millis', 'transform', 'refresh_millihz', 'primary')
def conditions(s): return [{k: o[k] for k in fields} for o in s['output-workspace-snapshot']['outputs']]
assert conditions(before) == conditions(after)
assert before['room-snapshot']['snapshot'] == after['room-snapshot']['snapshot']
assert before['window-snapshot'] == after['window-snapshot']
pids = {name: subprocess.check_output(['pgrep', '-x', name], text=True).strip() for name in ('niwoe', 'niwoe-shell')}
hashes = subprocess.check_output(['sha256sum', '/usr/local/bin/niwoe', '/usr/local/bin/niwoe-shell'] + [f'/proc/{pid}/exe' for pid in pids.values()], text=True)
(data / 'after-state.json').write_text(json.dumps(after, indent=2))
(data / 'after-identity.json').write_text(json.dumps(dict(pids=pids, hashes=hashes, outputs=conditions(after), started=time.time()), indent=2))
ui['manage']()
ui['click'](430, 370)
subprocess.run(['python3', str(ui['ROOT'] / 'scripts/measure-layout-cycles.py')], check=True)
(data / 'after-cycles.json').write_bytes((ui['ROOT'] / 'target/p09-layout-cycles.json').read_bytes())
ui['key'](*(['escape'] * 5))
time.sleep(30)
print('Starting three 300 s idle samples after warm-up; no competing GUI/build work', flush=True)
subprocess.run(['python3', str(ui['ROOT'] / 'scripts/measure-hub-performance.py'), str(data / 'after-idle.json')], check=True)
assert conditions(p.snapshot()) == conditions(after)
assert p.snapshot()['room-snapshot']['snapshot'] == before['room-snapshot']['snapshot']
print('PASS final P11 same conditions, 3x20 real cycles, 3x300 s idle, original rooms preserved', flush=True)
