#!/usr/bin/env python3
"""Real mouse/keyboard P11 workflow, asserting draft and productive documents.

Requires the isolated DRM session in target/p11-profile. No IPC navigation.
"""
import importlib.util
import json
import math
from pathlib import Path
import subprocess
import sys
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'target/p11-live'
spec = importlib.util.spec_from_file_location('ui', ROOT / 'scripts/test-control-center.py')
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
_, env = ui.p.environment()
DIRECTORY = Path(env['XDG_CONFIG_HOME']) / 'niwoe'
assert DIRECTORY == ROOT / 'target/p11-profile/config/niwoe', 'isolated DRM profile required'
ui.lib['KEYS']['alt'] = 56


def state(): return tomllib.loads((DIRECTORY / 'first-run.toml').read_text())
def draft(): return json.loads(state()['draft'])
def stored_rooms(): return tomllib.loads((DIRECTORY / 'rooms.toml').read_text())


def wait_step(step):
    deadline = time.monotonic() + 6
    while time.monotonic() < deadline:
        if 'draft' in state() and draft()['step'] == step: return draft()
        time.sleep(.1)
    raise AssertionError(('expected persisted UI step', step, state()))


def geometry():
    output = next(o for o in ui.p.snapshot()['output-workspace-snapshot']['outputs'] if o['primary'])
    scale = output['scale_millis'] / 1000
    width, height = round(output['width'] / scale), round(output['height'] / scale) - 48
    ratio = min(width / 1366, height / 768, 1)
    return width, height, math.ceil(width / ratio), math.ceil(height / ratio)


def canvas(x, y):
    width, height, cw, ch = geometry()
    ui.click(x * width / cw, 48 + y * height / ch)


def footer(index):
    _, _, width, height = geometry()
    cell = (width - 40 - 48) // 5
    canvas(20 + index * (cell + 12) + cell // 2, height - 60 + 12 + 19)


def control(index, step):
    _, _, width, _ = geometry()
    columns = 3 if step == 3 else 2 if step == 2 else 1
    row, col = divmod(index, columns)
    cell = (width - 40 - 12 * (columns - 1)) // columns
    canvas(20 + col * (cell + 12) + cell // 2, 192 + 96 + row * 50 + 19)


def keyboard_open():
    ui.key(*(['escape'] * 5))
    with ui.lib['VirtualKeyboard']() as keys:
        keys.key_down(125); keys.key_down(56); keys.tap(57); keys.key_up(56); keys.key_up(125)
    time.sleep(.7)
    ui.key('enter')
    time.sleep(.5)


def mouse_open(hub_open=False):
    assert geometry()[:2] == (1920, 1032), 'mouse reference path at 100 percent'
    if not hub_open: ui.click(30, 24)
    ui.click(1424, 172)  # Visible Hub -> room management.
    ui.click(120, 396)  # Actual System sidebar row, including panel offset.
    ui.click(1000, 312)  # Overview inside the panel-excluding layer from room management.
    time.sleep(.4)


def capture(label):
    subprocess.run(['python3', str(ROOT / 'scripts/test-hub-data.py'), 'capture', label], check=True)


def mouse():
    assert state()['fresh'] and not state()['completed']
    before = stored_rooms()
    if 'draft' not in state(): footer(1)
    wait_step(1)
    control(2, 1)  # Keyboard defaults; product remains the same.
    footer(1)
    d = wait_step(2)
    assert d['profile'] == 'keyboard' and 'search' not in d['panel']
    assert stored_rooms() == before and not (DIRECTORY / 'panel.toml').exists()
    control(0, 2); control(1, 2); control(2, 2)
    control(2, 2); control(2, 2)  # Same suggestion toggled back: no duplicate.
    control(6, 2)  # Actual first catalog app, Preferred on the selected suggestion.
    footer(1)
    d = wait_step(3)
    assert len(d['rooms']) == 3 and len({r['name'] for r in d['rooms']}) == 3
    assert d['rooms'][-1]['preferences']['apps'] and d['rooms'][-1]['assignment'] == 'preferred'
    assert stored_rooms() == before and not (DIRECTORY / 'panel.toml').exists()
    control(0, 3); control(3, 3)  # Hide tray and screenshot.
    control(6, 3)  # Enable search, appended after status.
    control(7, 3)  # Move search before status.
    capture('p11-panel-draft')
    footer(3)  # Cancel, preserving only the resume draft.
    assert stored_rooms() == before and not (DIRECTORY / 'panel.toml').exists()
    d = wait_step(3)
    assert d['panel'] == ['search', 'status'], d
    keyboard_open()
    capture('p11-resumed')
    footer(0); wait_step(2)
    footer(1); wait_step(3)
    footer(1); wait_step(4)
    capture('p11-ready')
    footer(1)
    deadline = time.monotonic() + 8
    while not state()['completed'] and time.monotonic() < deadline: time.sleep(.1)
    assert state()['completed'] and 'draft' not in state() and 'journal' not in state(), state()
    rooms = stored_rooms()
    assert len(rooms['rooms']) == len(before['rooms']) + 3
    assert rooms['rooms'][:len(before['rooms'])] == before['rooms']
    assert tomllib.loads((DIRECTORY / 'panel.toml').read_text())['modules'] == ['search', 'status']
    live = ui.p.snapshot()['room-snapshot']['snapshot']
    assert len(live['rooms']) == len(rooms['rooms']) and len({r['id'] for r in live['rooms']}) == len(rooms['rooms'])
    assert not ui.p.windows(), 'completion must not launch apps'
    (DATA / 'mouse.json').write_text(json.dumps(dict(before=before, draft=d, rooms=rooms, completion=state(), snapshot=live), indent=2))
    print('PASS real mouse choices, catalog, toggle/back/cancel/resume, panel order, durable completion and no app starts', flush=True)


def keyboard_skip():
    before = stored_rooms()
    panel = (DIRECTORY / 'panel.toml').read_bytes()
    keyboard_open()
    ui.key('enter')  # Welcome's focused Next.
    wait_step(1)
    # At each optional page, focus begins at first control. Reverse traversal
    # reaches discard -> cancel -> skip; disabled controls are not focus stops.
    for step in (1, 2, 3):
        ui.key('backtab', 'backtab', 'backtab', 'enter')
        wait_step(step + 1)
    ui.key('backtab', 'backtab', 'backtab', 'enter')  # Finish: Skip disabled, Next reached.
    time.sleep(.5)
    assert state()['completed'] and 'draft' not in state(), state()
    assert stored_rooms() == before and (DIRECTORY / 'panel.toml').read_bytes() == panel
    assert not ui.p.windows()
    (DATA / 'keyboard-skip.json').write_text(json.dumps(dict(state=state(), rooms=before), indent=2))
    print('PASS complete keyboard-only entry/skip/finish preserves established rooms and panel', flush=True)


def mouse_existing():
    # Start with the visible Hub; every workflow input below is a click.
    before = stored_rooms()
    panel = (DIRECTORY / 'panel.toml').read_bytes()
    mouse_open(hub_open=True)
    footer(4)  # Explicitly discard any previous optional draft.
    footer(1); wait_step(1)
    control(1, 1)  # Established values must survive a mouse-default choice.
    footer(1); wait_step(2)
    footer(1); wait_step(3)
    control(0, 3); control(0, 3)  # Real preview change and reversal.
    footer(1); wait_step(4)
    for exercise in range(3):
        control(exercise, 4)
        if exercise == 1:
            assert ui.p.snapshot()['window-snapshot']['active_workspace'] == 1
        mouse_open(hub_open=exercise == 0)
        wait_step(4)
    assert draft()['practiced'] == [True, True, True], draft()
    capture('p11-exercises')
    footer(1)
    assert state()['completed'] and 'draft' not in state(), state()
    assert stored_rooms() == before and (DIRECTORY / 'panel.toml').read_bytes() == panel
    assert not ui.p.windows()
    (DATA / 'mouse-only.json').write_text(json.dumps(dict(state=state(), rooms=before), indent=2))
    print('PASS complete mouse-only Settings entry, five pages, three actual exercises with mouse return, completion', flush=True)


def keyboard_choices():
    before = stored_rooms()
    panel = (DIRECTORY / 'panel.toml').read_bytes()
    assert all(r['name'] not in ('Arbeit', 'Privat', 'Entwicklung') for r in before['rooms'])
    keyboard_open()
    ui.key('end', 'enter')
    time.sleep(.4)
    ui.key('home', 'enter'); wait_step(1)
    ui.key('home', 'down', 'space')
    assert draft()['profile'] == 'mouse'
    def next_page(): ui.key('end', 'backtab', 'backtab', 'backtab', 'enter')
    next_page(); wait_step(2)
    ui.key('home', 'space', 'tab', 'enter', 'tab', 'space')
    assert len(draft()['rooms']) == 3
    # Page forward then back with keyboard; app controls use real catalog IDs.
    ui.key('tab', 'tab', 'enter', 'home', 'tab', 'tab', 'tab', 'tab', 'enter')
    ui.key('home', *(['tab'] * 5), 'enter')
    assert draft()['rooms'][-1]['preferences']['apps']
    next_page(); wait_step(3)
    ui.key('home', 'space')
    assert draft()['panel'] != __import__('tomllib').loads(panel.decode())['modules']
    # Cancel using Esc and resume purely through the normal keyboard entry.
    chosen = draft()
    ui.key('escape')
    assert stored_rooms() == before and (DIRECTORY / 'panel.toml').read_bytes() == panel
    keyboard_open()
    assert draft() == chosen
    next_page(); wait_step(4)
    ui.key('end', 'backtab', 'backtab', 'enter')
    time.sleep(.4)
    assert state()['completed'] and 'draft' not in state(), state()
    after = stored_rooms()
    assert len(after['rooms']) == len(before['rooms']) + 3
    assert after['rooms'][:len(before['rooms'])] == before['rooms']
    assert tomllib.loads((DIRECTORY / 'panel.toml').read_text())['modules'] == chosen['panel']
    assert not ui.p.windows()
    (DATA / 'keyboard-choices.json').write_text(json.dumps(dict(before=before, draft=chosen, rooms=after, state=state()), indent=2))
    print('PASS full keyboard-only choices, real app, paging, cancel/resume and durable completion', flush=True)


if __name__ == '__main__':
    {'mouse': mouse, 'keyboard-skip': keyboard_skip, 'mouse-existing': mouse_existing,
     'keyboard-choices': keyboard_choices}[sys.argv[1]]()
