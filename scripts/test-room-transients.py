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
    parent = Gtk.Window(title='P06 Parent')
    parent.set_default_size(360, 180)
    entry = Gtk.Entry()
    entry.set_text('p06-preserved-content')
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
            if command == 'minimize':
                parent.iconify()
            elif command.startswith('Dialog'):
                dialog = Gtk.Dialog(title='P06 ' + command, transient_for=parent, modal=False)
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
client_env['GDK_BACKEND'] = 'wayland'
directory = runtime.parent / 'transient-evidence'
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
    return {w['title']: w for w in data['window-snapshot']['windows'] if w['title'].startswith('P06 ')}


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


mutate(dict(operation='create', name='P06 Transient Source'))
rooms = snapshot()['room-snapshot']['snapshot']['rooms']
source = next(r for r in rooms if r['name'] == 'P06 Transient Source')
target = rooms[0]
send(type='switch-workspace', workspace=source['workspace'])
log = (directory / 'client.log').open('w')
process = subprocess.Popen([sys.executable, __file__, '--client', str(directory)], env=client_env, stdout=log, stderr=log)
try:
    wait_for(lambda d: 'P06 Parent' in windows(d))
    send(type='switch-workspace', workspace=target['workspace'])
    (directory / 'command').write_text('Dialog1')
    first = wait_for(lambda d: 'P06 Dialog1' in windows(d))
    assert windows(first)['P06 Dialog1']['workspace'] == source['workspace'], first
    assert first['window-snapshot']['active_workspace'] == target['workspace']
    send(type='switch-workspace', workspace=source['workspace'])
    (directory / 'command').write_text('minimize')
    wait_for(lambda d: windows(d)['P06 Parent']['minimized'])
    send(type='switch-workspace', workspace=target['workspace'])
    (directory / 'command').write_text('Dialog2')
    second = wait_for(lambda d: 'P06 Dialog2' in windows(d))
    assert windows(second)['P06 Dialog2']['workspace'] == source['workspace'], second
    assert second['window-snapshot']['active_workspace'] == target['workspace']
    mutate(dict(operation='delete', id=source['id'], target_id=target['id']))
    migrated = wait_for(lambda d: all(w['workspace'] == target['workspace'] for w in windows(d).values()))
    assert {t:w['id'] for t,w in windows(second).items()} == {t:w['id'] for t,w in windows(migrated).items()}
    assert windows(migrated)['P06 Parent']['minimized']
    send(type='focus-window', id=windows(migrated)['P06 Parent']['id'])
    wait_for(lambda d: not windows(d)['P06 Parent']['minimized'])
    deadline = time.monotonic() + 5
    while time.monotonic() < deadline:
        state = json.loads((directory / 'client-state.json').read_text())
        if set(state['drawn']) == {'Parent', 'Dialog1', 'Dialog2'}:
            break
        time.sleep(.1)
    assert set(state['drawn']) == {'Parent', 'Dialog1', 'Dialog2'}, state
    assert state['content'] == 'p06-preserved-content'
    stored = tomllib.loads((Path(client_env['XDG_CONFIG_HOME']) / 'niwoe/rooms.toml').read_text())
    assert not any(r['id'] == source['id'] for r in stored['rooms'])
    (directory / 'result.json').write_text(json.dumps({'before': second, 'after': migrated, 'client': state}, indent=2))
    print('PASS inactive parent, minimized parent, no room switch, dialog configure/draw, delete migration, minimized restore, IDs/content/persisted deletion')
finally:
    process.terminate()
    process.wait(timeout=5)
    log.close()
