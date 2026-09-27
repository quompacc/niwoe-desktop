#!/usr/bin/env python3
"""Real modifier drags, scroll, tiling and mixed-scale input on two outputs.

Uses the existing layout save path to observe logical window geometry. Saves
and restores the exact personal layout/config bytes, including on failure.
"""
import json
from pathlib import Path
import runpy
import subprocess
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
ui['lib']['KEYS']['f8'] = 66
p = ui['p']
data = ROOT / 'target/p12-evidence/desktop'
data.mkdir(parents=True, exist_ok=False)
initial = p.snapshot()
assert not initial['window-snapshot']['windows']
_, env = p.environment()
config = Path(env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe/config.toml'
original = config.read_bytes()
assert b'[outputs' not in original
room = initial['room-snapshot']['snapshot']['rooms'][0]['id']
layout = p.layout_path(room)
old_layout = layout.read_bytes() if layout.exists() else None
(data / 'config-before.toml').write_bytes(original)
if old_layout is not None: (data / 'layout-before.toml').write_bytes(old_layout)
processes = []
records = []


def combo(*codes):
    with ui['lib']['VirtualKeyboard']() as keyboard:
        for code in codes:
            keyboard.emit(1, code, 1); keyboard.emit(0, 0, 0)
        time.sleep(.08)
        for code in reversed(codes):
            keyboard.emit(1, code, 0); keyboard.emit(0, 0, 0)
        time.sleep(.15)  # Let libinput consume releases before removing the device.
    time.sleep(.5)


def state(kind):
    return json.loads((data / (kind + '.json')).read_text())


def wait(predicate):
    deadline = time.monotonic() + 12
    while time.monotonic() < deadline:
        if predicate(): return
        time.sleep(.2)
    raise AssertionError('condition timeout')


def observe(label):
    result = p.action(room, 'save')
    assert not result['running']
    saved = tomllib.loads(layout.read_text())
    value = dict(label=label, layout=saved, snapshot=p.snapshot(),
                 clients={kind: state(kind) for kind in ('wayland', 'x11')})
    records.append(value)
    (data / 'results.json').write_text(json.dumps(records, indent=2))
    return {entry['title'].split()[-1]: entry for entry in saved['entries']}


def focus(kind):
    identity = next(w['id'] for w in p.windows() if w['title'] == 'P12 ' + kind)
    p.send(type='focus-window', id=identity)
    wait(lambda: state(kind)['active'])


def pointer():
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    return ui['lib']['VirtualPointer'](max(o['x'] + o['width'] for o in outputs),
                                       max(o['y'] + o['height'] for o in outputs))


def drag(start, end, button):
    with pointer() as mouse, ui['lib']['VirtualKeyboard']() as keyboard:
        mouse.move_to(*start)
        keyboard.emit(1, 125, 1); keyboard.emit(0, 0, 0)
        time.sleep(.1)
        mouse.emit(1, button, 1); mouse.emit(0, 0, 0)
        time.sleep(.1)
        for step in range(1, 11):
            mouse.move_to(*(round(a + (b-a)*step/10) for a, b in zip(start, end)))
        mouse.emit(1, button, 0); mouse.emit(0, 0, 0)
        keyboard.emit(1, 125, 0); keyboard.emit(0, 0, 0)
        time.sleep(.15)
    time.sleep(.5)


try:
    ui['key'](*(['escape'] * 5)); combo(125, 2)
    for kind in ('wayland', 'x11'):
        log = (data / (kind + '.log')).open('w')
        processes.append(subprocess.Popen(['python3', str(ROOT / 'scripts/p12-client.py'), kind, str(data)],
            env=env | {'GDK_BACKEND': kind}, stdout=log, stderr=log))
        wait(lambda: (data / (kind + '.json')).exists())
    for kind in ('wayland', 'x11'):
        focus(kind)
        before = observe('initial ' + kind)[kind]
        # Explicitly enter both modes; an initially unmanaged X11 window can
        # have floating=false without yet belonging to the tiling tree.
        if not before['floating']: combo(125, 20)
        combo(125, 20)
        tiled = observe('tiled ' + kind)[kind]
        assert not tiled['floating'], tiled
        assert any(node.get('key') == tiled['key'] for node in records[-1]['layout']['tree'])
        combo(125, 20)
        floating = observe('floating ' + kind)[kind]
        assert floating['floating'], floating
        ui['key']('f8')
        wait(lambda: state(kind)['size'][0] == 620)
        geo = observe('floating client size ' + kind)[kind]['geometry']
        start = (geo['x'] + geo['width']//2, geo['y'] + geo['height']//2)
        drag(start, (start[0]+100, start[1]+60), 272)
        moved = observe('modifier move ' + kind)[kind]['geometry']
        assert abs(moved['x']-geo['x']-100) <= 2 and abs(moved['y']-geo['y']-60) <= 2, (geo, moved)
        start = (moved['x']+moved['width']-40, moved['y']+moved['height']-40)
        drag(start, (start[0]-100, start[1]-70), 273)
        resized = observe('modifier resize ' + kind)[kind]['geometry']
        assert abs(resized['width']-moved['width']+100) <= 2 and abs(resized['height']-moved['height']+70) <= 2, (moved, resized)
        print('PASS tiling, floating, modifier move/resize', kind, flush=True)
        # Populate through actual keyboard events, then scroll through pointer.
        with ui['lib']['VirtualKeyboard']() as keyboard:
            for _ in range(80): keyboard.type_text('x'); keyboard.tap(28)
        combo(29, 102)
        wait(lambda: state(kind)['scroll'] == 0)
        with pointer() as mouse:
            mouse.click(resized['x']+60, resized['y']+80)
            mouse.scroll(-5)
            mouse.scroll(-5)
        wait(lambda: state(kind)['scroll'] > 0)
        observe('pointer scroll ' + kind)
        print('PASS pointer scroll', kind, flush=True)
        combo(29, 30); ui['key']('text:p12-preserved')
    for scale in (1.0, 1.5, 2.0):
        config.write_bytes(original + f'\n[outputs."drm-0"]\nprimary = true\n[outputs."drm-1"]\nscale = {scale}\n'.encode())
        p.send(type='reload-config'); time.sleep(3)
        output = next(o for o in p.snapshot()['output-workspace-snapshot']['outputs'] if o['output_name'] == 'drm-1')
        assert output['scale_millis'] == round(scale*1000), output
        for kind in ('wayland', 'x11'):
            focus(kind)
            geo = observe(f'before output move {scale} {kind}')[kind]['geometry']
            start = (geo['x']+geo['width']//2, geo['y']+geo['height']//2)
            end = (output['x']+output['width']//2, output['y']+output['height']//2)
            drag(start, end, 272)
            entry = observe(f'output 2 scale {scale} {kind}')[kind]
            assert entry['output']['name'] == 'drm-1', entry
            combo(29, 30); ui['key']('text:p12-preserved')
            wait(lambda: state(kind)['content'] == 'p12-preserved' and state(kind)['active'])
            print('PASS second output input', kind, scale, flush=True)
    assert p.snapshot()['room-snapshot'] == initial['room-snapshot']
finally:
    for process in processes:
        if process.poll() is None:
            process.terminate()
            try: process.wait(timeout=5)
            except subprocess.TimeoutExpired: process.kill(); process.wait()
    config.write_bytes(original); p.send(type='reload-config'); time.sleep(3)
    assert config.read_bytes() == original
    if old_layout is None: layout.unlink(missing_ok=True)
    else: layout.write_bytes(old_layout)
    assert (layout.read_bytes() if layout.exists() else None) == old_layout
    p.send(type='switch-workspace', workspace=initial['window-snapshot']['active_workspace'])
    assert not p.windows()
    print('PASS owned clients removed; config/layout bytes restored', flush=True)
