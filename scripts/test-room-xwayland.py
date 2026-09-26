#!/usr/bin/env python3
"""Close an owned X11 window in an inactive room on an empty test desktop.

Usage: python3 scripts/test-room-xwayland.py COMPOSITOR_PID EVIDENCE_DIRECTORY
Requires the existing GTK3/PyGObject X11 backend; changes no room definitions.
"""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time


def client(directory):
    import gi
    gi.require_version('Gtk', '3.0')
    from gi.repository import Gtk, Gdk, GLib
    assert 'X11' in type(Gdk.Display.get_default()).__name__, 'X11 required'
    windows = {}
    handled = set()

    def tick():
        command_file = directory / 'command'
        command = command_file.read_text() if command_file.exists() else ''
        if command and command not in handled:
            handled.add(command)
            if command == 'close-source':
                windows.pop('Source').destroy()
            else:
                window = Gtk.Window(title='P06 X11 ' + command)
                window.set_default_size(320, 180)
                entry = Gtk.Entry()
                entry.set_text('preserved-' + command)
                window.add(entry)
                window.show_all()
                windows[command] = window
        state = {name: window.get_child().get_text() for name, window in windows.items()}
        temporary = directory / 'client-state.tmp'
        temporary.write_text(json.dumps(state))
        temporary.replace(directory / 'client-state.json')
        return True

    GLib.timeout_add(50, tick)
    Gtk.main()


if sys.argv[1] == '--client':
    client(Path(sys.argv[2]))
    raise SystemExit

shell_pid, = subprocess.check_output(
    ['pgrep', '-P', sys.argv[1], '-x', 'niwoe-shell'], text=True).split()
env = dict(part.split(b'=', 1) for part in
           Path(f'/proc/{shell_pid}/environ').read_bytes().split(b'\0') if b'=' in part)
runtime = Path(env[b'XDG_RUNTIME_DIR'].decode())
token = env.pop(b'NIWOE_IPC_TOKEN').decode()
client_env = os.environ | {k.decode(): v.decode() for k, v in env.items()}
client_env.pop('NIWOE_IPC_TOKEN', None)
client_env['GDK_BACKEND'] = 'x11'
assert client_env.get('DISPLAY'), 'XWayland DISPLAY required'
directory = Path(sys.argv[2]).resolve()
directory.mkdir(exist_ok=False)
config = Path(client_env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe/rooms.toml'
before_file = config.read_bytes()


def connection():
    sock = socket.socket(socket.AF_UNIX)
    sock.settimeout(5)
    sock.connect(str(runtime / 'niwoe.sock'))
    sock.sendall((json.dumps(dict(type='authenticate', role='shell', token=token)) + '\n').encode())
    return sock


def snapshot():
    with connection() as sock, sock.makefile() as reader:
        events = {}
        while not all(k in events for k in ('room-snapshot', 'window-snapshot')):
            event = json.loads(reader.readline())
            events[event['type']] = event
        return events


def send(**command):
    with connection() as sock, sock.makefile() as reader:
        sock.sendall((json.dumps(command) + '\n').encode())
        for line in reader:
            event = json.loads(line)
            if (event['type'] == 'window-snapshot'
                    and event['active_workspace'] == command['workspace']):
                return
        raise AssertionError('IPC disconnected before workspace snapshot')


def windows(data):
    return {w['title']: w for w in data['window-snapshot']['windows']}


def wait(predicate):
    deadline = time.monotonic() + 8
    while time.monotonic() < deadline:
        value = snapshot()
        if predicate(value):
            return value
        time.sleep(.1)
    raise AssertionError(value)


initial = snapshot()
assert not windows(initial), 'Empty test desktop required'
rooms = initial['room-snapshot']['snapshot']['rooms']
assert len(rooms) >= 2
source, target = (r['workspace'] for r in rooms[:2])
original = initial['window-snapshot']['active_workspace']
assert original, 'Select a room first; do not consume the login foyer'
process = None
try:
    send(type='switch-workspace', workspace=source)
    wait(lambda d: d['window-snapshot']['active_workspace'] == source)
    (directory / 'command').write_text('Source')
    with (directory / 'client.log').open('w') as log:
        process = subprocess.Popen([sys.executable, __file__, '--client', str(directory)],
                                   env=client_env, stdout=log, stderr=log)
    first = wait(lambda d: 'P06 X11 Source' in windows(d))
    assert windows(first)['P06 X11 Source']['workspace'] == source
    send(type='switch-workspace', workspace=target)
    wait(lambda d: d['window-snapshot']['active_workspace'] == target)
    (directory / 'command').write_text('Survivor')
    before = wait(lambda d: 'P06 X11 Survivor' in windows(d))
    survivor = windows(before)['P06 X11 Survivor']
    assert survivor['workspace'] == target
    (directory / 'command').write_text('close-source')
    after = wait(lambda d: 'P06 X11 Source' not in windows(d))
    assert after['window-snapshot']['active_workspace'] == target
    assert windows(after)['P06 X11 Survivor']['id'] == survivor['id']
    assert windows(after)['P06 X11 Survivor']['workspace'] == target
    send(type='switch-workspace', workspace=source)
    wait(lambda d: d['window-snapshot']['active_workspace'] == source)
    send(type='switch-workspace', workspace=target)
    revisited = wait(lambda d: d['window-snapshot']['active_workspace'] == target)
    assert set(windows(revisited)) == {'P06 X11 Survivor'}
    state = json.loads((directory / 'client-state.json').read_text())
    assert state == {'Survivor': 'preserved-Survivor'}, state
    (directory / 'result.json').write_text(json.dumps(
        dict(before=before, after=after, revisited=revisited, client=state), indent=2))
    print('PASS X11 background close, no room switch, survivor ID/content, room revisit', flush=True)
finally:
    if process:
        process.terminate()
        process.wait(timeout=5)
    send(type='switch-workspace', workspace=original)
    final = wait(lambda d: not windows(d) and d['window-snapshot']['active_workspace'] == original)
    assert final['room-snapshot'] == initial['room-snapshot']
    assert config.read_bytes() == before_file
    (directory / 'final.json').write_text(json.dumps(final, indent=2))
    print('PASS cleanup: no test windows, original active room, unchanged room/config data')
