#!/usr/bin/env python3
"""The exercise must match actual Super+1 after P10 presentation reordering."""
import json
from pathlib import Path
import runpy

u = runpy.run_path(str(Path(__file__).with_name('test-first-run-ui.py')))
ui = u['ui']
original = ui.p.snapshot()['room-snapshot']['snapshot']['rooms']
target = next(room for room in original if room['workspace'] == 1)
position = original.index(target)
assert position == 0 and len(original) > 1
try:
    ui.p.mutate(operation='move', id=target['id'], position=len(original)-1)
    reordered = ui.p.snapshot()['room-snapshot']['snapshot']['rooms']
    assert reordered[0]['workspace'] != 1
    with ui.lib['VirtualKeyboard']() as keys: keys.combo(125, 2)
    shortcut = ui.p.snapshot()['window-snapshot']['active_workspace']
    assert shortcut == reordered[0]['workspace']
    u['keyboard_open'](); u['footer'](4)
    u['footer'](1); u['wait_step'](1)
    for step in (2, 3, 4): u['footer'](2); u['wait_step'](step)
    u['control'](1, 4)
    assert ui.p.snapshot()['window-snapshot']['active_workspace'] == shortcut
    with ui.lib['VirtualKeyboard']() as keys: keys.combo(125, 3)
    assert ui.p.snapshot()['window-snapshot']['active_workspace'] == reordered[1]['workspace']
    u['keyboard_open']()
    ui.key('home', 'tab', 'enter')
    assert ui.p.snapshot()['window-snapshot']['active_workspace'] == shortcut
    assert not ui.p.windows()
    (u['DATA'] / 'room-order.json').write_text(json.dumps(dict(original=original, reordered=reordered, shortcut_workspace=shortcut, exercised_workspace=shortcut), indent=2))
finally:
    ui.p.mutate(operation='move', id=target['id'], position=position)
    assert ui.p.snapshot()['room-snapshot']['snapshot']['rooms'] == original
print('PASS reordered display list: actual Super+1, mouse and keyboard exercise agree; order restored', flush=True)
