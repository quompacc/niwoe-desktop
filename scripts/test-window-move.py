#!/usr/bin/env python3
"""Native GTK transient regression, only in an isolated nested smoke profile."""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time
import tomllib


def client(directory):
    import gi
    gi.require_version('Gtk', '3.0')
    from gi.repository import Gtk, GLib
    GLib.set_prgname('niwoe-transient-regression')
    parent = Gtk.Window(title='P07 Parent')
    parent.set_default_size(360, 180)
    entry = Gtk.Entry()
    entry.set_text('p07-preserved-content')
    parent.add(entry)
    windows = {'Parent': parent}
    drawn = set()
    def record_draw(_window, _cr, name):
        drawn.add(name)
        return False
    parent.connect('draw', record_draw, 'Parent')
    parent.show_all()
    handled = set()
    def tick():
        command_file = directory / 'command'
        command = command_file.read_text() if command_file.exists() else ''
        if command and command not in handled:
            handled.add(command)
            if command.startswith('minimize'):
                parent.iconify()
            elif command.startswith('Window'):
                window = Gtk.Window(title='P07 ' + command)
                window.set_default_size(320, 180)
                window.add(Gtk.Label(label='same-app-separate-window'))
                window.show_all()
                windows[command] = window
            elif command.startswith('Dialog'):
                dialog = Gtk.Dialog(title='P07 ' + command, transient_for=parent, modal=False)
                dialog.get_content_area().add(Gtk.Label(label='dialog-content'))
                dialog.connect('draw', record_draw, command)
                dialog.show_all()
                windows[command] = dialog
        value = {'drawn': sorted(drawn), 'content': entry.get_text()}
        temporary = directory / 'client-state.tmp'
        temporary.write_text(json.dumps(value))
        temporary.replace(directory / 'client-state.json')
        return True
    GLib.timeout_add(50, tick)
    Gtk.main()


if sys.argv[1] == '--client':
    client(Path(sys.argv[2]))
    raise SystemExit

shell_pid, = subprocess.check_output(['pgrep', '-P', sys.argv[1], '-x', 'niwoe-shell'], text=True).split()
env = dict(part.split(b'=', 1) for part in Path(f'/proc/{shell_pid}/environ').read_bytes().split(b'\0') if b'=' in part)
runtime = Path(env[b'XDG_RUNTIME_DIR'].decode())
assert str(runtime).startswith('/tmp/niwoe-p01.'), 'isolated profile required'
token = env.pop(b'NIWOE_IPC_TOKEN').decode()
client_env = os.environ | {k.decode(): v.decode() for k, v in env.items()}
client_env.pop('NIWOE_IPC_TOKEN', None)
client_env['GDK_BACKEND'] = os.environ.get('NIWOE_WINDOW_TEST_BACKEND', 'wayland')
if client_env['GDK_BACKEND'] == 'x11':
    child, = subprocess.check_output(['pgrep', '-P', sys.argv[1], '-x', 'Xwayland'], text=True).split()
    args = Path(f'/proc/{child}/cmdline').read_bytes().split(b'\0')
    client_env['DISPLAY'] = next(arg.decode() for arg in args if arg.startswith(b':'))
directory = runtime.parent / 'window-move-evidence'
directory.mkdir()


def connection():
    sock = socket.socket(socket.AF_UNIX)
    sock.settimeout(5)
    sock.connect(str(runtime / 'niwoe.sock'))
    sock.sendall((json.dumps({'type': 'authenticate', 'role': 'shell', 'token': token}) + '\n').encode())
    return sock


def snapshot():
    with connection() as sock:
        data = {}
        with sock.makefile() as reader:
            while not all(kind in data for kind in ('window-snapshot', 'room-snapshot')):
                event = json.loads(reader.readline())
                data[event['type']] = event
        return data


def send(**command):
    with connection() as sock:
        sock.sendall((json.dumps(command) + '\n').encode())
        time.sleep(.1)


def wait_for(predicate):
    deadline = time.monotonic() + 5
    while time.monotonic() < deadline:
        value = snapshot()
        if predicate(value):
            return value
        time.sleep(.1)
    raise AssertionError(json.dumps(value))


def windows(data):
    return {w['title']: w for w in data['window-snapshot']['windows'] if w['title'].startswith('P07 ')}


def mutate(change):
    revision = snapshot()['room-snapshot']['snapshot']['revision']
    request = 'transient-' + str(time.time_ns())
    with connection() as sock:
        sock.sendall((json.dumps(dict(type='mutate-room', request_id=request, expected_revision=revision, change=change)) + '\n').encode())
        with sock.makefile() as reader:
            for line in reader:
                event = json.loads(line)
                if event['type'] == 'room-mutation-result' and event['request_id'] == request:
                    assert event['error'] is None, event
                    return



mutate(dict(operation='create', name='P07 Move Source'))
rooms = snapshot()['room-snapshot']['snapshot']['rooms']
source = next(r for r in rooms if r['name'] == 'P07 Move Source')
target = rooms[0]
send(type='switch-workspace', workspace=source['workspace'])
log = (directory / 'client.log').open('w')
process = subprocess.Popen([sys.executable, __file__, '--client', str(directory)], env=client_env, stdout=log, stderr=log)
marker = Path('target/p07-window-ui-ready.json')
try:
    initial = wait_for(lambda d: 'P07 Parent' in windows(d))
    key = windows(initial)['P07 Parent']['id']
    send(type='move-window-to-room', id=key, room_id=target['id'])
    moved = wait_for(lambda d: windows(d)['P07 Parent']['workspace'] == target['workspace'])
    assert moved['window-snapshot']['active_workspace'] == source['workspace']
    assert windows(moved)['P07 Parent']['id'] == key
    send(type='move-window-to-room', id=key, room_id=999999)
    send(type='move-window-to-room', id='closed-window', room_id=source['id'])
    assert windows(snapshot())['P07 Parent']['workspace'] == target['workspace']
    send(type='switch-workspace', workspace=target['workspace'])
    send(type='focus-window', id=key)
    (directory/'command').write_text('minimize')
    wait_for(lambda d: windows(d)['P07 Parent']['minimized'])
    send(type='move-window-to-room', id=key, room_id=source['id'])
    minimized = wait_for(lambda d: windows(d)['P07 Parent']['workspace'] == source['workspace'])
    assert windows(minimized)['P07 Parent']['minimized']
    send(type='switch-workspace', workspace=source['workspace'])
    send(type='focus-window', id=key)
    restored = wait_for(lambda d: not windows(d)['P07 Parent']['minimized'])
    assert windows(restored)['P07 Parent']['id'] == key
    assert json.loads((directory/'client-state.json').read_text())['content'] == 'p07-preserved-content'
    print('PASS explicit stable-ID move, invalid/dead targets, minimized move/restore, content, no room switch',flush=True)
    if os.environ.get('NIWOE_WINDOW_UI_SMOKE') == '1':
        # Welcome Hub is still open in this fresh isolated session.
        send(type='toggle-launcher')
        send(type='toggle-launcher')
        marker.write_text(json.dumps(dict(compositor=int(sys.argv[1]), profile=str(runtime.parent))))
        print('READY UI: F6, move first window to room 2, then open it',flush=True)
        deadline=time.monotonic()+30
        while time.monotonic()<deadline:
            current=snapshot()
            if windows(current)['P07 Parent']['workspace']==2 and current['window-snapshot']['active_workspace']==2:
                break
            time.sleep(.1)
        assert windows(current)['P07 Parent']['workspace']==2, current
        assert current['window-snapshot']['active_workspace']==2, current
        print('PASS real UI room-target click and window activation',flush=True)
    if os.environ.get('NIWOE_WINDOW_PAGES_SMOKE') == '1':
        for index in range(8):
            (directory/'command').write_text('Window' + str(index))
            wait_for(lambda d: 'P07 Window' + str(index) in windows(d))
        (directory/'command').write_text('minimize-again')
        wait_for(lambda d: windows(d)['P07 Parent']['minimized'])
        # Toggle the initial welcome Hub closed/open to reacquire keyboard.
        send(type='toggle-launcher')
        send(type='toggle-launcher')
        marker.write_text(json.dumps(dict(compositor=int(sys.argv[1]), profile=str(runtime.parent))))
        print('READY pages: F6 End Enter restores last, minimized window',flush=True)
        deadline=time.monotonic()+25
        while time.monotonic()<deadline:
            current=snapshot()
            if not windows(current)['P07 Parent']['minimized']: break
            time.sleep(.1)
        assert not windows(current)['P07 Parent']['minimized']
        assert len(windows(current))==9
        assert len({w['id'] for w in windows(current).values()})==9
        print('PASS UI pagination reaches minimized ninth window; same-app windows remain distinct',flush=True)
    (directory/'result.json').write_text(json.dumps(snapshot(),indent=2))
finally:
    marker.unlink(missing_ok=True)
    process.terminate(); process.wait(timeout=5); log.close()
    mutate(dict(operation='delete', id=source['id'], target_id=target['id']))
