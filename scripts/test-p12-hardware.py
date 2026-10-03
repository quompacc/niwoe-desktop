#!/usr/bin/env python3
"""Staged real HDMI and optical lock checks, using only owned GTK clients."""
import getpass
import json
import os
from pathlib import Path
import runpy
import signal
import subprocess
import sys
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
ui['lib']['KEYS']['f8'] = 66
p = ui['p']
data = ROOT / 'target/p12-evidence/hardware'
stage, = sys.argv[1:]


def wait(predicate):
    end = time.monotonic() + 25
    while time.monotonic() < end:
        if predicate(): return
        time.sleep(.2)
    raise AssertionError('hardware/client condition timeout')


def combo(*codes):
    with ui['lib']['VirtualKeyboard']() as keys:
        for code in codes: keys.emit(1, code, 1); keys.emit(0, 0, 0)
        time.sleep(.1)
        for code in reversed(codes): keys.emit(1, code, 0); keys.emit(0, 0, 0)
        time.sleep(.2)
    time.sleep(.5)


def client(kind):
    return json.loads((data / (kind + '.json')).read_text())


def focus(kind):
    identity = next(w['id'] for w in p.windows() if w['title'] == 'P12 ' + kind)
    p.send(type='focus-window', id=identity)
    wait(lambda: client(kind)['active'])


def layout_entries():
    p.action(owner['room'], 'save')
    layout = tomllib.loads(p.layout_path(owner['room']).read_text())
    return {entry['title'].split()[-1]: entry for entry in layout['entries']}


def move_center(kind, end):
    focus(kind)
    geo = layout_entries()[kind]['geometry']
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    start = (geo['x'] + geo['width']//2, geo['y'] + geo['height']//2)
    with ui['lib']['VirtualPointer'](max(o['x']+o['width'] for o in outputs), max(o['y']+o['height'] for o in outputs)) as mouse, ui['lib']['VirtualKeyboard']() as keys:
        mouse.move_to(*start)
        keys.emit(1, 125, 1); keys.emit(0, 0, 0); time.sleep(.1)
        mouse.emit(1, 272, 1); mouse.emit(0, 0, 0); time.sleep(.1)
        for step in range(1, 11): mouse.move_to(*(round(a+(b-a)*step/10) for a,b in zip(start,end)))
        mouse.emit(1, 272, 0); mouse.emit(0, 0, 0)
        keys.emit(1, 125, 0); keys.emit(0, 0, 0); time.sleep(.2)
    time.sleep(.5)


def check_clients():
    assert len(p.windows()) == 2
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    for kind in ('wayland', 'x11'):
        focus(kind)
        entry = layout_entries()[kind]['geometry']
        assert any(entry['x'] >= o['x'] and entry['y'] >= o['y'] + 48
            and entry['x'] + entry['width'] <= o['x'] + o['width']
            and entry['y'] + entry['height'] <= o['y'] + o['height'] for o in outputs), entry
        combo(29, 30); ui['key']('text:p12-protected-' + kind)
        wait(lambda: client(kind)['content'] == 'p12-protected-' + kind)


if stage == 'prepare':
    assert not p.windows()
    data.mkdir(parents=True, exist_ok=False)
    room = p.snapshot()['room-snapshot']['snapshot']['rooms'][0]['id']
    layout = p.layout_path(room)
    owner = dict(room=room, pids=[], old_layout=layout.exists())
    if layout.exists(): (data / 'layout-before.toml').write_bytes(layout.read_bytes())
    ui['key'](*(['escape'] * 5)); combo(125, 2)
    _, env = p.environment()
    for kind in ('wayland', 'x11'):
        process = subprocess.Popen(['python3', str(ROOT / 'scripts/p12-client.py'), kind, str(data)],
            env=env | {'GDK_BACKEND': 'wayland' if kind == 'wayland' else 'x11'},
            stdout=(data / (kind + '.log')).open('w'), stderr=subprocess.STDOUT)
        owner['pids'].append(process.pid)
        (data / 'owner.json').write_text(json.dumps(owner))
        wait(lambda: (data / (kind + '.json')).exists() and any(w['title'] == 'P12 ' + kind for w in p.windows()))
        focus(kind)
        if not layout_entries()[kind]['floating']: combo(125, 20)
        ui['key']('f8'); wait(lambda: client(kind)['size'][0] == 620)
    for kind in ('wayland', 'x11'):
        focus(kind)
        wait(lambda: client(kind)['size'][0] == 620)
    move_center('wayland', (1200, 500))
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    assert len(outputs) == 2
    second = next(o for o in outputs if o['output_name'] == 'drm-1')
    end = (second['x'] + second['width']//2, second['y'] + second['height']//2)
    move_center('x11', end)
    assert layout_entries()['x11']['output']['name'] == 'drm-1'
    check_clients()
else:
    owner = json.loads((data / 'owner.json').read_text())
    if stage in ('unplug', 'replug'):
        wait(lambda: len(p.snapshot()['output-workspace-snapshot']['outputs']) == (1 if stage == 'unplug' else 2))
        check_clients()
    elif stage == 'lock':
        second = next(o for o in p.snapshot()['output-workspace-snapshot']['outputs'] if o['output_name'] == 'drm-1')
        move_center('x11', (second['x'] + second['width']//2, second['y'] + second['height']//2))
        check_clients()
        p.send(type='lock-session')
        wait(lambda: subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode == 0)
    elif stage == 'locked-restart':
        assert subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode == 0
        old = int(subprocess.check_output(['pgrep', '-x', 'niwoe-shell']))
        os.kill(old, signal.SIGKILL)
        wait(lambda: subprocess.run(['pgrep', '-x', 'niwoe-shell'], capture_output=True, text=True).stdout.strip() not in ('', str(old)))
        time.sleep(3)
        assert subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode == 0
    elif stage == 'unlock':
        password = getpass.getpass('Fedora unlock password (not logged): ')
        with ui['lib']['VirtualKeyboard']() as keys: keys.type_text(password); keys.tap(28)
        password = ''
        wait(lambda: subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode != 0)
        check_clients()
    elif stage == 'cleanup':
        for pid in owner['pids']:
            command = Path('/proc', str(pid), 'cmdline')
            if command.exists() and str(data).encode() in command.read_bytes(): os.kill(pid, signal.SIGTERM)
        wait(lambda: not p.windows())
        layout = p.layout_path(owner['room'])
        if owner['old_layout']: layout.write_bytes((data / 'layout-before.toml').read_bytes())
        else: layout.unlink(missing_ok=True)
    else:
        raise AssertionError('unknown stage')
(data / (stage + '.json')).write_text(json.dumps(p.snapshot(), indent=2))
print('PASS hardware stage', stage, '(optical observations recorded separately)', flush=True)
