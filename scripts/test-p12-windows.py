#!/usr/bin/env python3
"""P12 real input and protocol state, with disposable GTK clients.

Run on an empty real NIWOE desktop with temporary uinput permission. Does not
claim an everyday-app workflow or a physical hotplug test. No room definitions
are changed. Every action records client state and the compositor snapshot.
"""
import json
from pathlib import Path
import runpy
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
ui['lib']['KEYS'].update(f5=63, f6=64, f9=67, f10=68, f11=87)
p = ui['p']
DATA = ROOT / 'target/p12-evidence/windows'
DATA.mkdir(parents=True, exist_ok=False)
initial = p.snapshot()
assert not initial['window-snapshot']['windows'], 'empty desktop required'
processes = []
records = []


def client(kind):
    return json.loads((DATA / (kind + '.json')).read_text())


def check(label, predicate):
    deadline = time.monotonic() + 8
    while time.monotonic() < deadline:
        snapshot = p.snapshot()
        states = {kind: client(kind) for kind in ('wayland', 'x11') if (DATA / (kind + '.json')).exists()}
        if predicate(snapshot, states):
            record = dict(label=label, snapshot=snapshot, clients=states)
            records.append(record)
            (DATA / 'results.json').write_text(json.dumps(records, indent=2))
            print('PASS', label, flush=True)
            return record
        time.sleep(.15)
    raise AssertionError((label, snapshot, states))


def combo(*keys):
    with ui['lib']['VirtualKeyboard']() as keyboard:
        for code in keys:
            keyboard.emit(1, code, 1)
            keyboard.emit(0, 0, 0)
        time.sleep(.05)
        for code in reversed(keys):
            keyboard.emit(1, code, 0)
            keyboard.emit(0, 0, 0)
        time.sleep(.15)  # Consume releases before removing the virtual device.
    time.sleep(.4)


def key(code):
    with ui['lib']['VirtualKeyboard']() as keyboard:
        keyboard.tap(code)
    time.sleep(.4)


def focus(kind):
    window = next(w for w in p.windows() if w['title'] == 'P12 ' + kind)
    p.send(type='focus-window', id=window['id'])
    check('backend and toolkit focus ' + kind, lambda _s, c: c[kind]['focused'] and c[kind]['active'])
    return window['id']


try:
    ui['key'](*(['escape'] * 5))
    combo(125, 2)  # Super+1
    _, env = p.environment()
    for kind in ('wayland', 'x11'):
        log = (DATA / (kind + '.log')).open('w')
        process = subprocess.Popen(['python3', str(ROOT / 'scripts/p12-client.py'), kind, str(DATA)],
                                   env=env | {'GDK_BACKEND': kind}, stdout=log, stderr=log)
        processes.append(process)
        check('launch ' + kind, lambda s, c: kind in c and
              any(w['title'] == 'P12 ' + kind for w in s['window-snapshot']['windows']))
    focus('wayland')
    ui['key']('text:p12-wayland-text')
    check('Wayland input', lambda _s, c: c['wayland']['content'] == 'p12-wayland-text')
    combo(29, 30); combo(29, 46)  # Ctrl+A/C
    focus('x11'); combo(29, 47)
    check('clipboard Wayland to XWayland', lambda _s, c: c['x11']['content'] == 'p12-wayland-text')
    combo(29, 30)
    ui['key']('text:p12-x11-text')
    combo(29, 30); combo(29, 46)
    focus('wayland'); combo(29, 30); combo(29, 47)
    check('clipboard XWayland to Wayland', lambda _s, c: c['wayland']['content'] == 'p12-x11-text')
    for kind in ('wayland', 'x11'):
        identity = focus(kind)
        combo(29, 31)
        check('save ' + kind, lambda _s, c: (DATA / (kind + '.txt')).exists()
              and (DATA / (kind + '.txt')).read_text() == c[kind]['content'])
        key(68)  # F10 max
        check('maximize ' + kind, lambda _s, c: bool(c[kind]['state'] & 4))
        key(68)
        check('unmaximize ' + kind, lambda _s, c: not c[kind]['state'] & 4)
        key(87)  # F11 fullscreen
        check('fullscreen ' + kind, lambda _s, c: bool(c[kind]['state'] & 16))
        key(87)
        check('unfullscreen ' + kind, lambda _s, c: not c[kind]['state'] & 16)
        key(67)  # F9 minimize
        check('minimize ' + kind, lambda s, _c: next(w for w in s['window-snapshot']['windows'] if w['id'] == identity)['minimized'])
        focus(kind)
        check('restore ' + kind, lambda s, c: c[kind]['focused'] and not next(w for w in s['window-snapshot']['windows'] if w['id'] == identity)['minimized'])
        for code, name, modal in ((63, 'F5', False), (64, 'F6', True)):
            key(code)
            check('dialog ' + kind + ' ' + name, lambda s, c: any(
                d['title'] == 'P12 ' + kind + ' ' + name and d['focused'] and d['active'] and d['modal'] == modal
                for d in c[kind]['dialogs']) and any(
                    w['title'] == 'P12 ' + kind + ' ' + name for w in s['window-snapshot']['windows']))
            key(1)
            check('dialog closed ' + kind + ' ' + name,
                  lambda s, c: not c[kind]['dialogs'] and c[kind]['active'] and not any(
                      w['title'] == 'P12 ' + kind + ' ' + name for w in s['window-snapshot']['windows']))
            focus(kind)
        combo(125, 42, 3)  # move to room 2
        check('move room ' + kind, lambda s, _c: next(w for w in s['window-snapshot']['windows'] if w['id'] == identity)['workspace'] == 2)
        combo(125, 3)
        focus(kind)
        combo(125, 42, 2); combo(125, 2)
        focus(kind)
        check('return room ' + kind, lambda s, _c: next(w for w in s['window-snapshot']['windows'] if w['id'] == identity)['workspace'] == 1)
    assert p.snapshot()['room-snapshot'] == initial['room-snapshot']
    print('PASS room definitions unchanged', flush=True)
finally:
    for process in processes:
        if process.poll() is None:
            process.terminate()
            try: process.wait(timeout=5)
            except subprocess.TimeoutExpired: process.kill(); process.wait()
    p.send(type='switch-workspace', workspace=initial['window-snapshot']['active_workspace'])
    check('owned clients removed', lambda s, _c: not s['window-snapshot']['windows'])
