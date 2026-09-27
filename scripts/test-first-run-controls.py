#!/usr/bin/env python3
"""Sweep optional module controls and keyboard exercise paths in owned profile."""
from pathlib import Path
import runpy
import time

u = runpy.run_path(str(Path(__file__).with_name('test-first-run-ui.py')))
ui, directory = u['ui'], u['DIRECTORY']
before = {name: (directory / name).read_bytes() for name in ('rooms.toml', 'panel.toml')}
u['keyboard_open'](); u['footer'](4); time.sleep(.4)
u['footer'](1); u['wait_step'](1)
for choice, profile in enumerate(('preserve', 'mouse', 'keyboard')):
    u['control'](choice, 1)
    assert u['draft']()['profile'] == profile
u['footer'](2); u['wait_step'](2)
u['footer'](2); u['wait_step'](3)
modules = ['tray', 'screenshot', 'search', 'status']
u['control'](0, 3); u['control'](0, 3)  # Persist the initialized preview without a net change.
for index, module in enumerate(modules):
    if module not in u['draft']()['panel']: u['control'](index * 3, 3)
for index, module in enumerate(modules):
    for direction in (1, 2):
        old = u['draft']()['panel']
        position = old.index(module)
        if (direction == 1 and position == 0) or (direction == 2 and position == 3):
            # Move from the endpoint first so both enabled arrows are exercised.
            u['control'](index * 3 + (3 - direction), 3)
            old = u['draft']()['panel']
            position = old.index(module)
        expected = old.copy()
        other = position + (-1 if direction == 1 else 1)
        expected[position], expected[other] = expected[other], expected[position]
        u['control'](index * 3 + direction, 3)
        assert u['draft']()['panel'] == expected
    u['control'](index * 3, 3)
    assert module not in u['draft']()['panel']
    u['control'](index * 3, 3)
    assert module in u['draft']()['panel']
u['footer'](2); u['wait_step'](4)
assert u['draft']()['panel'] is None
for exercise in range(3):
    ui.key('home', *(['tab'] * exercise), 'enter')
    if exercise == 1:
        assert ui.p.snapshot()['window-snapshot']['active_workspace'] == 1
    u['keyboard_open']()
    u['wait_step'](4)
assert u['draft']()['practiced'] == [True] * 3
ui.key('escape')
for name, value in before.items(): assert (directory / name).read_bytes() == value
# Real global shortcuts outside the wizard, with screenshot and workspace proof.
ui.key(*(['escape'] * 5), 'hub')
u['capture']('p11-shortcut-hub')
ui.key('escape')
with ui.lib['VirtualKeyboard']() as keys: keys.combo(125, 2)
assert ui.p.snapshot()['window-snapshot']['active_workspace'] == 1
with ui.lib['VirtualKeyboard']() as keys: keys.combo(125, 1)
time.sleep(.4)
u['capture']('p11-shortcut-deck')
ui.key('escape')
assert not ui.p.windows()
print('PASS every profile/module/toggle/arrow, skip rollback, three keyboard exercises and real global shortcuts', flush=True)
