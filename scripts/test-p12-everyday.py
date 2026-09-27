#!/usr/bin/env python3
"""Two real KWrite backends and owned files across lock and real suspend.

Arms RTC wake before real systemd suspend; requires retained SSH recovery.
Uses private application settings; never writes personal editor preferences.
"""
import getpass
import ctypes
import hashlib
import json
from pathlib import Path
import runpy
import subprocess
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
data = ROOT / 'target/p12-evidence/everyday'
data.mkdir(parents=True, exist_ok=False)
initial = p.snapshot()
assert not initial['window-snapshot']['windows']
password = getpass.getpass('Fedora unlock password (not logged): ')
_, env = p.environment()
private_config = data / 'config'
private_config.mkdir()
env |= {'XDG_CONFIG_HOME': str(private_config)}
room = initial['room-snapshot']['snapshot']['rooms'][0]['id']
layout = p.layout_path(room)
old_layout = layout.read_bytes() if layout.exists() else None
if old_layout is not None: (data / 'layout-before.toml').write_bytes(old_layout)
processes = []
records = []
alarm_armed = False
suspend_requested = False
files = {kind: data / ('p12-' + kind + '.txt') for kind in ('wayland', 'x11')}


def wait(predicate, seconds=15):
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        if predicate(): return
        time.sleep(.2)
    raise AssertionError('condition timeout')


def combo(*codes):
    with ui['lib']['VirtualKeyboard']() as keyboard:
        for code in codes:
            keyboard.emit(1, code, 1); keyboard.emit(0, 0, 0)
        time.sleep(.08)
        for code in reversed(codes):
            keyboard.emit(1, code, 0); keyboard.emit(0, 0, 0)
        time.sleep(.15)
    time.sleep(.4)


def window(kind):
    return next(w for w in p.windows() if files[kind].name in w['title'])


def focus(kind):
    p.send(type='focus-window', id=window(kind)['id'])
    time.sleep(.5)


def record(label):
    value = dict(label=label, snapshot=p.snapshot(),
        files={kind: dict(content=file.read_text(), sha256=hashlib.sha256(file.read_bytes()).hexdigest())
               for kind, file in files.items()}, pids=[process.pid for process in processes])
    records.append(value)
    (data / 'results.json').write_text(json.dumps(records, indent=2))
    print('PASS', label, flush=True)


def unlock():
    if subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode == 0:
        time.sleep(2)
        with ui['lib']['VirtualKeyboard']() as keyboard:
            keyboard.type_text(password); keyboard.tap(28)
        wait(lambda: subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode != 0)


def privileged(*command):
    # Credential remains in memory/stdin, never argv, a file or a log.
    result = subprocess.run(['sudo', '-S', '-p', '', *command], input=password + '\n',
                            text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=90)
    assert result.returncode == 0, (command, result.returncode, result.stdout)
    return result.stdout


try:
    ui['key'](*(['escape'] * 5)); combo(125, 2)
    ui['click'](400, 250)  # Establish the primary output before launching editors.
    for kind in files:
        files[kind].write_text('p12-original-' + kind + '\n')
        log = (data / (kind + '.log')).open('w')
        processes.append(subprocess.Popen(['kwrite', str(files[kind])],
            env=env | {'QT_QPA_PLATFORM': 'wayland' if kind == 'wayland' else 'xcb'},
            stdout=log, stderr=log))
        wait(lambda: any(files[kind].name in w['title'] for w in p.windows()))
        assert window(kind)['id'].startswith('x11:') == (kind == 'x11'), window(kind)
    record('real KWrite Wayland and XWayland opened')
    focus('wayland'); combo(29, 30); ui['key']('text:p12-everyday-content'); combo(29, 31)
    wait(lambda: files['wayland'].read_text().strip() == 'p12-everyday-content')
    combo(29, 30); combo(29, 46)
    focus('x11'); combo(29, 30); combo(29, 47); combo(29, 31)
    wait(lambda: files['x11'].read_bytes() == files['wayland'].read_bytes())
    expected = files['wayland'].read_bytes()
    record('edited, saved and clipboard transferred; files byte-identical')
    combo(125, 42, 3); combo(125, 3)
    assert window('x11')['workspace'] == 2
    focus('x11'); combo(125, 42, 2); combo(125, 2)
    focus('x11')
    identity = window('x11')['id']
    # Real ICCCM request, as issued by an X11 application's minimize action.
    xlib = ctypes.CDLL('libX11.so.6')
    xlib.XOpenDisplay.argtypes = [ctypes.c_char_p]
    xlib.XOpenDisplay.restype = ctypes.c_void_p
    xlib.XIconifyWindow.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int]
    xlib.XFlush.argtypes = xlib.XCloseDisplay.argtypes = [ctypes.c_void_p]
    display = xlib.XOpenDisplay(env['DISPLAY'].encode())
    assert display
    try:
        assert xlib.XIconifyWindow(display, int(identity.split(':')[1]), 0)
        xlib.XFlush(display)
    finally:
        xlib.XCloseDisplay(display)
    wait(lambda: window('x11')['minimized'])
    focus('x11'); wait(lambda: not window('x11')['minimized'])
    record('room round trip and minimize/restore')
    p.action(room, 'save')
    entry = next(e for e in tomllib.loads(layout.read_text())['entries'] if files['x11'].name in e['title'])
    if not entry['floating']: combo(125, 20)
    p.action(room, 'save')
    geo = next(e['geometry'] for e in tomllib.loads(layout.read_text())['entries'] if files['x11'].name in e['title'])
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    second = next(o for o in outputs if o['output_name'] == 'drm-1')
    start = (geo['x']+geo['width']//2, geo['y']+geo['height']//2)
    end = (second['x']+second['width']//2, second['y']+second['height']//2)
    with ui['lib']['VirtualPointer'](max(o['x']+o['width'] for o in outputs), max(o['y']+o['height'] for o in outputs)) as mouse, ui['lib']['VirtualKeyboard']() as keyboard:
        mouse.move_to(*start)
        keyboard.emit(1, 125, 1); keyboard.emit(0, 0, 0); time.sleep(.1)
        mouse.emit(1, 272, 1); mouse.emit(0, 0, 0); time.sleep(.1)
        for step in range(1, 11): mouse.move_to(*(round(a+(b-a)*step/10) for a, b in zip(start, end)))
        mouse.emit(1, 272, 0); mouse.emit(0, 0, 0)
        keyboard.emit(1, 125, 0); keyboard.emit(0, 0, 0); time.sleep(.15)
    time.sleep(.5); p.action(room, 'save')
    entry = next(e for e in tomllib.loads(layout.read_text())['entries'] if files['x11'].name in e['title'])
    moved = entry['geometry']
    (data / 'layout-after-drag.toml').write_bytes(layout.read_bytes())
    (data / 'snapshot-after-drag.json').write_text(json.dumps(p.snapshot(), indent=2))
    move_matches = abs(moved['x']-geo['x']-(end[0]-start[0])) <= 2 and abs(moved['y']-geo['y']-(end[1]-start[1])) <= 2
    (data / 'output-move.json').write_text(json.dumps(dict(start=start, end=end, before=geo, outputs=outputs, entry=entry)))
    if entry['output']['name'] != 'drm-1' or not move_matches:
        subprocess.run(['python3', str(ROOT / 'scripts/test-p12-portal.py'), 'everyday_move'], check=True)
    assert entry['output']['name'] == 'drm-1', entry
    assert move_matches, (geo, moved, start, end)
    record('KWrite moved to second physical output')
    p.send(type='lock-session')
    wait(lambda: subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode == 0)
    unlock()
    record('real lock and PAM unlock, files unchanged')
    rtc = Path('/sys/class/rtc/rtc0')
    assert (rtc / 'device/power/wakeup').read_text().strip() == 'enabled'
    assert not (rtc / 'wakealarm').read_text().strip(), 'preserve any pre-existing RTC alarm'
    alarm_log = privileged('rtcwake', '-m', 'no', '-s', '35')
    alarm_armed = True
    alarm = (rtc / 'wakealarm').read_text().strip()
    assert alarm and int(alarm) > time.time()
    (data / 'rtc-alarm.json').write_text(json.dumps(dict(alarm=alarm, result=alarm_log)))
    print('READY: RTC alarm armed; entering real systemd suspend', flush=True)
    before = time.clock_gettime(time.CLOCK_BOOTTIME) - time.monotonic()
    suspend_requested = True
    suspend_log = privileged('systemctl', 'suspend')
    # systemctl returns after queuing suspend, not after waking. Never cancel
    # the alarm merely because the machine has not entered sleep yet.
    wait(lambda: time.clock_gettime(time.CLOCK_BOOTTIME) - time.monotonic() - before > 10, 90)
    slept = time.clock_gettime(time.CLOCK_BOOTTIME) - time.monotonic() - before
    (data / 'suspend.json').write_text(json.dumps(dict(suspend_seconds=slept, result=suspend_log)))
    assert slept > 10, 'kernel clocks must confirm actual sleep'
    privileged('rtcwake', '-m', 'disable')
    alarm_armed = False
    unlock()
    assert all(process.poll() is None for process in processes)
    assert all(file.read_bytes() == expected for file in files.values())
    for kind in files:
        focus(kind); combo(29, 107); ui['key']('text:-continued'); combo(29, 31)
        wait(lambda: b'-continued' in files[kind].read_bytes())
    assert files['wayland'].read_bytes() == files['x11'].read_bytes()
    record('after real suspend/resume: both apps usable, continued files byte-identical')
    for kind in files:
        focus(kind); combo(29, 16)  # saved documents, normal Ctrl+Q
    wait(lambda: not p.windows())
    record('applications closed normally with saved data')
finally:
    # On an ambiguous/failed suspend keep the recovery alarm armed. It expires
    # naturally; removing it here could strand the machine just entering sleep.
    if alarm_armed and not suspend_requested: privileged('rtcwake', '-m', 'disable')
    password = ''
    for process in processes:
        if process.poll() is None:
            process.terminate()
            try: process.wait(timeout=5)
            except subprocess.TimeoutExpired: process.kill(); process.wait()
    if old_layout is None: layout.unlink(missing_ok=True)
    else: layout.write_bytes(old_layout)
    assert (layout.read_bytes() if layout.exists() else None) == old_layout
