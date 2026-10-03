#!/usr/bin/env python3
"""Real lock/PAM, shell SIGKILL/watchdog and hostile-focus regression.

Password is read from the terminal, never passed in argv or written by this
test. Run with empty desktop and uinput permission, with SSH recovery available.
Optical leak freedom still requires visual review of the locked displays.
"""
import getpass
import json
import os
from pathlib import Path
import runpy
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
directory = ROOT / 'target/p12-evidence/lock'
directory.mkdir(parents=True, exist_ok=False)
password = getpass.getpass('Fedora unlock password (not logged): ')
assert all(char in ui['lib']['KEYS'] for char in password), 'unsupported keyboard character'
assert not p.windows(), 'empty desktop required'
_, env = p.environment()
marker = Path(env['XDG_RUNTIME_DIR']) / 'niwoe' / ('first-login-hub-shown-' + env['XDG_SESSION_ID'])
stamp = marker.stat().st_mtime_ns
process = subprocess.Popen(['python3', str(ROOT / 'scripts/p12-client.py'), 'wayland', str(directory)],
    env=env | {'GDK_BACKEND': 'wayland'}, stdout=(directory / 'client.log').open('w'), stderr=subprocess.STDOUT)
records = []


def wait(predicate):
    deadline = time.monotonic() + 20
    while time.monotonic() < deadline:
        if predicate(): return
        time.sleep(.15)
    raise AssertionError('condition timeout')


def state():
    return json.loads((directory / 'wayland.json').read_text())


def record(label, **data):
    records.append(dict(label=label, **data))
    (directory / 'results.json').write_text(json.dumps(records, indent=2))
    print('PASS', label, flush=True)


def crash_shell():
    old, = subprocess.check_output(['pgrep', '-x', 'niwoe-shell'], text=True).split()
    os.kill(int(old), signal.SIGKILL)
    def replaced():
        values = subprocess.run(['pgrep', '-x', 'niwoe-shell'], text=True, capture_output=True).stdout.split()
        return len(values) == 1 and values[0] != old
    wait(replaced)
    time.sleep(3)
    new, = subprocess.check_output(['pgrep', '-x', 'niwoe-shell'], text=True).split()
    assert marker.stat().st_mtime_ns == stamp
    return dict(old=old, new=new)


try:
    wait(lambda: any(w['title'] == 'P12 wayland' for w in p.windows()))
    identity = next(w['id'] for w in p.windows() if w['title'] == 'P12 wayland')
    p.send(type='focus-window', id=identity)
    wait(lambda: state()['active'])
    ui['key']('text:p12-protected-content')
    wait(lambda: state()['content'] == 'p12-protected-content')
    record('unlocked watchdog', **crash_shell())
    assert state()['content'] == 'p12-protected-content'
    connection = p.Connection()
    connection.until('room-snapshot')
    connection.send(type='lock-session')
    connection.until('session-locked')
    connection.close()
    wait(lambda: not state()['active'])
    time.sleep(2)
    ui['key']('text:p12-wrong-password', 'enter')
    time.sleep(4)
    assert subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode == 0
    assert state()['content'] == 'p12-protected-content'
    ui['key']('escape')
    record('wrong password and Escape remain locked')
    record('locked watchdog', **crash_shell())
    # Reconnect explicitly queries lock state; no inference from process alone.
    connection = p.Connection()
    connection.until('room-snapshot')
    connection.send(type='request-room-snapshot')
    connection.until('session-locked')
    connection.send(type='focus-window', id=identity)
    connection.close()
    ui['key']('text:p12-blocked-input', 'escape')
    time.sleep(.5)
    assert state()['content'] == 'p12-protected-content'
    assert not state()['active']
    record('locked IPC reconnect and input isolation after focus attempt')
    with ui['lib']['VirtualKeyboard']() as keyboard:
        keyboard.type_text(password)
        keyboard.tap(28)
    password = ''
    wait(lambda: subprocess.run(['pgrep', '-x', 'niwoe-lock'], capture_output=True).returncode != 0)
    p.send(type='focus-window', id=identity)
    wait(lambda: state()['active'])
    assert state()['content'] == 'p12-protected-content'
    ui['key']('text:-continued')
    wait(lambda: state()['content'] == 'p12-protected-content-continued')
    record('real PAM unlock and continued input; protected content preserved')
finally:
    password = ''
    if process.poll() is None:
        process.terminate()
        try: process.wait(timeout=5)
        except subprocess.TimeoutExpired: process.kill(); process.wait()
    # Never forcibly unlock in cleanup. A failed test leaves the lock intact.
