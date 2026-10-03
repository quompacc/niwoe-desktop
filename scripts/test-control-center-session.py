#!/usr/bin/env python3
"""Compare stable persisted intent across an actual login; slots are runtime IDs."""
import importlib.util
import json
from pathlib import Path
import sys

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p
record = ui.DATA / 'session-before.json'


def intent(rooms): return [{k: v for k, v in r.items() if k != 'workspace'} for r in rooms]


if sys.argv[1] == 'prepare':
    p.close_clients()
    assert not p.windows()
    state = p.snapshot()
    record.write_text(json.dumps(dict(state=state, panel=ui.panel_state(), launches=len(p.launches())), indent=2))
    print('PASS recorded actual persisted room/panel intent; all test clients closed', flush=True)
elif sys.argv[1] == 'verify':
    before = json.loads(record.read_text())
    state = p.snapshot()
    assert state['window-snapshot']['active_workspace'] == 0, state
    assert not state['window-snapshot']['windows'], state
    assert len(p.launches()) == before['launches'], 'unexpected login app start'
    assert intent(state['room-snapshot']['snapshot']['rooms']) == intent(before['state']['room-snapshot']['snapshot']['rooms'])
    assert ui.panel_state() == before['panel']
    for room in state['room-snapshot']['snapshot']['rooms']: ui.persisted(room)
    (ui.DATA / 'session-after.json').write_text(json.dumps(state, indent=2))
    print('PASS actual relogin: stable room intent/order and panel persisted; neutral foyer, no app/restore start', flush=True)
else:
    raise SystemExit('prepare or verify')
