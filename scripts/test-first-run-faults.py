#!/usr/bin/env python3
"""Real UI error/retry checks in the explicitly owned P11 DRM profile only.

For partial-fail the operator temporarily makes the profile directory root-owned
1777 and panel.toml root-owned. Restore original owners/modes before partial-retry.
This makes only the second productive rename fail after the room write succeeded.
"""
import json
from pathlib import Path
import runpy
import sys
import time

u = runpy.run_path(str(Path(__file__).with_name('test-first-run-ui.py')))
ui, directory, data = u['ui'], u['DIRECTORY'], u['DATA']
mode = sys.argv[1]

if mode == 'prepare':
    u['keyboard_open']()
    u['footer'](4)
    time.sleep(.4)
    before = {name: (directory / name).read_text() for name in ('rooms.toml', 'panel.toml', 'first-run.toml')}
    assert not any(r['name'] == 'Arbeit' for r in u['stored_rooms']()['rooms'])
    u['footer'](1); u['wait_step'](1)
    u['footer'](2); u['wait_step'](2)
    u['control'](0, 2)
    u['footer'](1); u['wait_step'](3)
    u['control'](0, 3)
    u['footer'](1); u['wait_step'](4)
    (data / 'fault-before.json').write_text(json.dumps(before))
    print('PASS explicit UI draft prepared; productive documents unchanged', flush=True)
elif mode == 'partial-fail':
    before = json.loads((data / 'fault-before.json').read_text())
    u['footer'](1)
    time.sleep(.5)
    state = u['state']()
    assert 'journal' in state and 'draft' in state, state
    assert (directory / 'panel.toml').read_text() == before['panel.toml']
    assert len(u['stored_rooms']()['rooms']) == len(__import__('tomllib').loads(before['rooms.toml'])['rooms']) + 1
    assert state['completed'] == __import__('tomllib').loads(before['first-run.toml'])['completed']
    u['capture']('p11-real-partial-error')
    (data / 'partial-error.json').write_text(json.dumps(state, indent=2))
    print('PASS real second-document rename denied after rooms committed; active journal, no false new completion', flush=True)
elif mode == 'partial-discard':
    before = u['state']()
    assert 'journal' in before
    documents = {name: (directory / name).read_bytes() for name in ('rooms.toml', 'panel.toml')}
    u['footer'](4)
    assert u['state']() == before, 'first click must only ask for confirmation'
    u['capture']('p11-partial-discard-confirm')
    u['footer'](4)
    time.sleep(.4)
    after = u['state']()
    assert 'journal' not in after and 'draft' not in after
    assert after['completed'] == before['completed']
    for name, value in documents.items(): assert (directory / name).read_bytes() == value
    print('PASS explicit two-click partial abandonment retains all already published documents and previous completion', flush=True)
elif mode == 'partial-retry':
    rooms = u['stored_rooms']()
    ui.key('escape')
    u['keyboard_open']()
    assert 'journal' in u['state']()
    u['footer'](1)
    time.sleep(.5)
    state = u['state']()
    assert state['completed'] and 'journal' not in state and 'draft' not in state
    assert u['stored_rooms']() == rooms
    expected = json.loads((data / 'partial-error.json').read_text())['journal']['panel_after']
    assert __import__('tomllib').loads((directory / 'panel.toml').read_text()) == expected
    print('PASS return/retry completes only remaining write, exact panel and no duplicate rooms', flush=True)
elif mode == 'write-error':
    u['keyboard_open']()
    u['footer'](4); time.sleep(.4)
    u['footer'](1); u['wait_step'](1)
    before = (directory / 'first-run.toml').read_bytes()
    try:
        directory.chmod(0o500)
        u['control'](2, 1)
        assert (directory / 'first-run.toml').read_bytes() == before
        u['capture']('p11-real-write-error')
        ui.key('escape')
        u['keyboard_open']()
        assert (directory / 'first-run.toml').read_bytes() == before
        u['capture']('p11-unsaved-resume')
    finally:
        directory.chmod(0o700)
    u['footer'](1)
    assert u['wait_step'](2)['profile'] == 'keyboard'
    u['footer'](4)
    print('PASS actual draft write denied; original retained, local choice survives cancel/reopen and saves after repair', flush=True)
elif mode == 'invalid':
    ui.key(*(['escape'] * 5))
    path = directory / 'first-run.toml'
    original = path.read_bytes()
    try:
        for label, text in [('broken', b'broken = ['), ('future', b'schema_version = 99\nrevision = 0\nfresh = false\ncompleted = false\n')]:
            path.write_bytes(text)
            u['keyboard_open']()
            u['capture']('p11-error-' + label)
            assert path.read_bytes() == text
            ui.key('escape')
            assert path.read_bytes() == text
    finally:
        path.write_bytes(original)
    u['keyboard_open']()
    print('PASS corrupt/future state visible in real UI; original error documents retained, recovery after repair', flush=True)
else:
    raise ValueError(mode)
