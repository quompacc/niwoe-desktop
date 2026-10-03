#!/usr/bin/env python3
"""Reject invalid external IDs/preferences without mutating the actual session."""
import importlib.util
import json
from pathlib import Path
import subprocess
import time

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p
before = p.snapshot()['room-snapshot']['snapshot']
room = next(r for r in before['rooms'] if r['name'] == 'p10-mouse')
assert room['id'] in ui.owned()['rooms']
change = {k: room[k] for k in ('name', 'description', 'assignment', 'preferences')}
change.update(operation='configure', id=room['id'], position=next(i for i, r in enumerate(before['rooms']) if r['id'] == room['id']))
c = p.Connection()
c.until('room-snapshot')
results = []
for mutation, revision, expected in [
    (change | dict(id=0), before['revision'], 'invalid'),
    (change | dict(id=18446744073709551615), before['revision'], 'invalid'),
    (change | dict(preferences=room['preferences'] | dict(apps=room['preferences']['apps'] * 2)), before['revision'], 'invalid'),
    (change, before['revision'] - 1, 'conflict'),
]:
    request = 'p10-invalid-' + str(time.time_ns())
    c.send(type='mutate-room', request_id=request, expected_revision=revision, change=mutation)
    result = c.until('room-mutation-result', lambda e: e['request_id'] == request)
    assert result['error'] == expected, result
    assert p.snapshot()['room-snapshot']['snapshot'] == before
    results.append(result)
c.close()
(ui.DATA / 'protocol.json').write_text(json.dumps(results, indent=2))
print('PASS real IPC invalid IDs, duplicate app identities and stale revision leave session unchanged', flush=True)
_, env = p.environment()
parent = Path(env['WAYLAND_DISPLAY'])
if not parent.is_absolute(): parent = Path(env['XDG_RUNTIME_DIR']) / parent
result = subprocess.run(['dbus-run-session', '--', 'python3', str(ui.ROOT / 'scripts/test-room-errors.py'), str(parent)], capture_output=True, text=True)
(ui.DATA / 'protocol-isolated.log').write_text(result.stdout + result.stderr)
print(result.stdout, flush=True)
assert result.returncode == 0, result.stderr
