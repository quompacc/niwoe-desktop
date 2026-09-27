#!/usr/bin/env python3
"""P10 live input with state assertions. Never uses IPC to navigate the UI.

Run as the Fedora session user with temporary /dev/uinput access. Only rooms
recorded by prepare/mouse/keyboard are removed by cleanup. Existing room contents
and configuration are compared with the recorded baseline. No credentials logged.
"""
import importlib.util
import json
from pathlib import Path
import runpy
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'target/p10-live'
DATA.mkdir(exist_ok=True)
OWNER = DATA / 'owned.json'
spec = importlib.util.spec_from_file_location('p09', ROOT / 'scripts/test-layout-restore.py')
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)
lib = runpy.run_path(str(ROOT / 'scripts/test-login-uinput.py'), run_name='input_library')
lib['KEYS'].update(escape=1, backspace=14, tab=15, enter=28, space=57, f7=65,
                   home=102, end=107, left=105, right=106, up=103, down=108,
                   shift=42, ctrl=29, pageup=104, pagedown=109)
lib['KEYS'][' '] = 57
lib['KEYS']['/'] = 53
lib['KEYS']['.'] = 52


def key(*actions):
    with lib['VirtualKeyboard']() as keys:
        for action in actions:
            if action == 'hub': keys.combo(125, 57)
            elif action == 'backtab': keys.combo(42, 15)
            elif action.startswith('text:'): keys.type_text(action[5:])
            else: keys.tap(lib['KEYS'][action])
            time.sleep(.12)
    time.sleep(.25)


def click(x, y):
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    width = max(o['x'] + o['width'] for o in outputs)
    height = max(o['y'] + o['height'] for o in outputs)
    with lib['VirtualPointer'](width, height) as pointer:
        pointer.click(round(x), round(y))
    time.sleep(.35)


def rooms(): return p.snapshot()['room-snapshot']['snapshot']['rooms']
def owned(): return json.loads(OWNER.read_text())


def wait_room(name):
    deadline = time.monotonic() + 8
    while time.monotonic() < deadline:
        result = next((r for r in rooms() if r['name'] == name), None)
        if result: return result
        time.sleep(.2)
    raise AssertionError('visible Save did not persist ' + name)


def remember(room):
    record = owned()
    assert room['id'] not in {r['id'] for r in record['original']}
    if room['id'] not in record['rooms']: record['rooms'].append(room['id'])
    OWNER.write_text(json.dumps(record, indent=2))


def persisted(room):
    import tomllib
    _, env = p.environment()
    path = Path(env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe/rooms.toml'
    stored = tomllib.loads(path.read_text())
    found = next(r for r in stored['rooms'] if r['id'] == room['id'])
    for field in ('name', 'description', 'assignment', 'preferences'):
        # TOML omits optional None values.
        left = found[field]
        right = room[field]
        if field == 'preferences':
            left = {k: v for k, v in left.items() if v is not None}
            right = {k: v for k, v in right.items() if v is not None}
        assert left == right, (field, left, right)


def manage(mouse=True):
    key('escape', 'escape', 'escape', 'escape', 'escape')
    if mouse:
        click(30, 24)  # Actual panel Hub button.
        click(1424, 172)  # Actual Hub "Räume verwalten" at primary 1920x1080.
    else:
        key('hub', 'backtab', 'enter')


def search(name):
    click(1760, 88)
    key(*(['backspace'] * 32), 'text:' + name)


def open_room(name):
    search(name)
    click(430, 370)  # Filtered first card; stable ID resolved by product hit-test.


def prepare():
    assert not OWNER.exists(), 'clean up previous P10 run first'
    initial = p.snapshot()
    assert not initial['window-snapshot']['windows'], 'require empty test session'
    OWNER.write_text(json.dumps(dict(original=rooms(), rooms=[]), indent=2))
    print('PASS recorded original rooms; no test mutation', flush=True)


def mouse():
    manage()
    click(1760, 276)  # New room.
    before = rooms()
    click(1810, 1051)  # Empty name is invalid.
    assert rooms() == before, 'invalid form mutated persistent state'
    click(500, 339)
    key('text:p10-mouse')
    click(950, 339)
    key('text:p10-description')
    click(306, 339)  # Icon.
    click(650, 510)  # Preferred assignment.
    click(650, 582)  # LayoutOnly manual preference.
    click(1810, 1051)
    room = wait_room('p10-mouse')
    remember(room)
    assert room['description'] == 'p10-description', room
    assert room['assignment'] == 'preferred', room
    assert room['preferences']['icon'] == 'folder', room
    assert room['preferences']['restore'] == 'layout-only', room
    persisted(room)
    original = rooms()
    open_room('p10-mouse')
    click(500, 339)
    key('text:p10-cancelled')
    click(1000, 403)  # Reorder draft.
    click(306, 339)
    click(1640, 1051)  # Cancel.
    assert rooms() == original, 'cancel leaked draft fields or order'
    open_room('p10-mouse')
    click(1000, 403)
    click(1810, 1051)
    after = rooms()
    assert [r['id'] for r in after].index(room['id']) == len(after) - 2
    assert next(r for r in after if r['id'] == room['id']) == room
    print('PASS mouse Hub/management/create/field-validation/icon/assignment/restore/order/save/cancel with persistent state', flush=True)


def keyboard():
    manage(False)
    # Search state survives reopening. Focus New after the sole filtered result.
    key('tab', 'tab', 'enter')
    key('text:p10-keyboard', 'tab', 'text:p10-keydescription')
    key('tab', 'tab', 'tab', 'enter')  # Preferred.
    key('tab', 'enter')  # LayoutOnly.
    key(*(['tab'] * 6), 'enter')  # Save.
    room = wait_room('p10-keyboard')
    remember(room)
    assert room['description'] == 'p10-keydescription', room
    assert room['assignment'] == 'preferred', room
    assert room['preferences']['restore'] == 'layout-only', room
    persisted(room)
    print('PASS keyboard regular Hub/management/new/form/save and durable values', flush=True)


def panel_state():
    c = p.Connection()
    c.until('room-snapshot')
    request = 'p10-panel-read-' + str(time.time_ns())
    c.send(type='panel-preferences', request_id=request, action=dict(operation='get'))
    result = c.until('panel-preferences', lambda e: e['request_id'] == request)
    c.close()
    assert result['error'] is None, result
    return result['modules']


def panel():
    baseline = panel_state()
    assert baseline == ['tray', 'screenshot', 'search', 'status'], baseline
    _, env = p.environment()
    path = Path(env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe/panel.toml'
    record = owned()
    if 'panel_before' not in record:
        record['panel_before'] = path.read_text() if path.exists() else None
    OWNER.write_text(json.dumps(record, indent=2))
    manage()
    click(100, 444)  # Visible Leiste sidebar item.
    for y in (323, 385, 447, 509): click(700, y)
    assert panel_state() == baseline, 'live preview persisted before Save'
    subprocess.run(['python3', str(ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-panel-hidden-preview'], check=True)
    click(1640, 1051)
    assert panel_state() == baseline, 'cancel changed saved panel'
    subprocess.run(['python3', str(ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-panel-cancelled-preview'], check=True)
    click(100, 444)
    click(700, 447)  # Hide Search.
    for _ in range(3): click(1650, 509)  # Status moves before the remaining modules.
    click(1810, 1051)
    assert panel_state() == ['status', 'tray', 'screenshot']
    assert path.exists()
    key('f7', *(['tab'] * 6), 'enter')  # Add Search at the end.
    key(*(['tab'] * 5), 'enter', 'enter', 'enter', 'tab', 'enter')
    assert panel_state() == baseline, 'keyboard reorder/save did not restore module order'
    key('f7', 'enter', 'escape')  # Keyboard toggle and cancel.
    assert panel_state() == baseline
    print('PASS panel mouse/keyboard visibility/order/save/cancel; durable state checked', flush=True)


def cleanup():
    record = owned()
    for room in list(reversed(record['rooms'])):
        if any(r['id'] == room for r in rooms()):
            assert not any(w['workspace'] == next(r['workspace'] for r in rooms() if r['id'] == room) for w in p.windows()), 'close owned test clients first'
            p.mutate(operation='delete', id=room, target_id=record['original'][0]['id'])
    assert rooms() == record['original'], 'original room data must be unchanged'
    OWNER.rename(DATA / ('completed-' + str(time.time_ns()) + '.json'))
    print('PASS isolated P10 rooms removed; original definitions preserved', flush=True)


if __name__ == '__main__':
    mode = sys.argv[1]
    if mode == 'key': key(*sys.argv[2:])
    elif mode == 'click': click(float(sys.argv[2]), float(sys.argv[3]))
    elif mode == 'open': open_room(sys.argv[2])
    elif mode == 'snapshot': print(json.dumps(p.snapshot(), indent=2))
    elif mode in ('prepare', 'mouse', 'keyboard', 'panel', 'cleanup'): globals()[mode]()
    else: raise SystemExit('unknown mode')
