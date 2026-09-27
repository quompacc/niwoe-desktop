#!/usr/bin/env python3
"""P09 Fedora acceptance. Own room/desktop IDs are recorded and removed explicitly.

setup -> existing -> [relogin/reboot] -> restart -> failures -> cleanup.
Never records credentials. All assertions use actual IPC/state/client results.
"""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'target/p09-live'
DATA.mkdir(exist_ok=True)
OWNER = DATA / 'owned.json'


def environment():
    pid, = subprocess.check_output(['pgrep', '-x', 'niwoe-shell'], text=True).split()
    raw = Path('/proc/' + pid + '/environ').read_bytes()
    env = dict(value.split(b'=', 1) for value in raw.split(b'\0') if b'=' in value)
    token = env.pop(b'NIWOE_IPC_TOKEN').decode()
    return token, os.environ | {k.decode(): v.decode() for k, v in env.items()}


class Connection:
    def __init__(self):
        token, env = environment()
        self.sock = socket.socket(socket.AF_UNIX)
        self.sock.settimeout(55)
        self.sock.connect(env['XDG_RUNTIME_DIR'] + '/niwoe.sock')
        self.reader = self.sock.makefile()
        self.send(type='authenticate', role='shell', token=token)

    def send(self, **command):
        self.sock.sendall((json.dumps(command) + '\n').encode())

    def until(self, kind, predicate=lambda e: True):
        deadline = time.monotonic() + 55
        while time.monotonic() < deadline:
            line = self.reader.readline()
            assert line, 'IPC disconnected'
            event = json.loads(line)
            if event['type'] == kind and predicate(event):
                return event
        raise AssertionError('deadline: ' + kind)

    def close(self):
        self.reader.close()
        self.sock.close()


def snapshot():
    c = Connection()
    result = {}
    for kind in ('room-snapshot', 'window-snapshot', 'output-workspace-snapshot'):
        result[kind] = c.until(kind)
    c.close()
    return result


def mutate(**change):
    c = Connection()
    revision = c.until('room-snapshot')['snapshot']['revision']
    request = 'p09-' + str(time.time_ns())
    c.send(type='mutate-room', request_id=request, expected_revision=revision, change=change)
    result = c.until('room-mutation-result', lambda e: e['request_id'] == request)
    assert result['error'] is None, result
    c.close()


def send(**command):
    if command.get('type') == 'layout':
        command.setdefault('request_id', 'p09-' + str(time.time_ns()))
    c = Connection()
    c.until('room-snapshot')
    c.send(**command)
    time.sleep(.15)
    c.close()


def action(room, operation, **kwargs):
    if operation in ('save', 'set-file') and 'expected_revision' not in kwargs:
        try:
            kwargs['expected_revision'] = tomllib.loads(layout_path(room).read_text())['revision']
        except (OSError, ValueError, KeyError):
            kwargs['expected_revision'] = 0
    c = Connection()
    c.until('room-snapshot')
    request_id = 'p09-' + str(time.time_ns())
    c.send(type='layout', request_id=request_id, action=dict(action=operation, room_id=room, **kwargs))
    result = c.until('layout', lambda e: e['request_id'] == request_id and e['notice']['notice'] == 'status'
                     and e['notice']['room_id'] == room and not e['notice']['running'])['notice']
    c.close()
    with (DATA / 'results.jsonl').open('a') as log:
        log.write(json.dumps(dict(operation=operation, result=result), ensure_ascii=False) + '\n')
    return result


def owned():
    return json.loads(OWNER.read_text())


def layout_path(room):
    _, env = environment()
    return Path(env.get('XDG_STATE_HOME', str(Path.home() / '.local/state'))) / 'niwoe/layouts' / f'room-{room}.toml'


def windows():
    return snapshot()['window-snapshot']['windows']


def wait_windows(number):
    deadline = time.monotonic() + 12
    while time.monotonic() < deadline:
        result = [w for w in windows() if w['title'].startswith('P09 ')]
        if len(result) == number:
            return result
        time.sleep(.2)
    raise AssertionError(('windows', number, result))


def launch(kind):
    _, env = environment()
    env.pop('NIWOE_IPC_TOKEN', None)
    env['GDK_BACKEND'] = 'x11' if kind == 'x11' else 'wayland'
    p = subprocess.Popen(['/usr/bin/python3', str(ROOT / 'scripts/p09-client.py'), kind],
                         env=env, stdout=(DATA / (kind + '.log')).open('a'), stderr=subprocess.STDOUT)
    return p.pid


def close_clients():
    for entry in launches():
        # Verify ownership again before asking a process to exit.
        proc = Path('/proc') / str(entry['pid']) / 'cmdline'
        if proc.exists() and b'/scripts/p09-client.py' in proc.read_bytes():
            (DATA / f"command-{entry['pid']}").write_text('close')
    wait_windows(0)


def launches():
    path = DATA / 'launches.jsonl'
    return [json.loads(line) for line in path.read_text().splitlines()] if path.exists() else []


def setup():
    assert not OWNER.exists(), 'cleanup the recorded previous P09 run first'
    initial = snapshot()
    record = dict(original=initial['room-snapshot']['snapshot']['rooms'], rooms=[], desktop_files=[])
    OWNER.write_text(json.dumps(record))
    for name in ('P09 Layout', 'P09 Target'):
        mutate(operation='create', name=name)
        room = next(r for r in snapshot()['room-snapshot']['snapshot']['rooms'] if r['name'] == name)
        record['rooms'].append(room['id'])
        OWNER.write_text(json.dumps(record))
    apps = Path.home() / '.local/share/applications'
    apps.mkdir(parents=True, exist_ok=True)
    for kind in ('native', 'x11', 'slow'):
        path = apps / ('org.niwoe.P09' + kind.title() + '.desktop')
        with path.open('x') as file:
            file.write('[Desktop Entry]\nType=Application\nName=P09 ' + kind + '\n'
                       + 'Exec=/usr/bin/env GDK_BACKEND=' + ('x11' if kind == 'x11' else 'wayland')
                       + ' /usr/bin/python3 ' + str(ROOT / 'scripts/p09-client.py') + ' ' + kind + ' %f\n'
                       + ('StartupWMClass=P09X11\n' if kind == 'x11' else ''))
        record['desktop_files'].append(str(path))
        OWNER.write_text(json.dumps(record))
    # Refresh through normal Hub opening on the next explicit UI operation.
    send(type='toggle-launcher')
    time.sleep(2)
    send(type='toggle-launcher')
    for kind in ('native', 'x11'):
        launch(kind)
    clients = wait_windows(2)
    for w in clients:
        send(type='move-window-to-room', id=w['id'], room_id=record['rooms'][0])
    result = action(record['rooms'][0], 'save')
    assert 'gespeichert' in result['message'], result
    original = layout_path(record['rooms'][0]).read_bytes()
    (DATA / 'original-layout.toml').write_bytes(original)
    assert len(tomllib.loads(original.decode())['entries']) == 2
    print('PASS setup native/X11 windows, isolated rooms, bounded layout persisted', flush=True)


def existing():
    room, target = owned()['rooms']
    # Freeze the explicit saved intent before moving its windows elsewhere.
    action(room, 'restore', relaunch=False)
    for w in wait_windows(2):
        send(type='move-window-to-room', id=w['id'], room_id=target)
    before = len(launches())
    result = action(room, 'restore', relaunch=False)
    assert all(r['message'] == 'Fenster angeordnet' for r in result['results']), result
    room_slot = next(r['workspace'] for r in snapshot()['room-snapshot']['snapshot']['rooms'] if r['id'] == room)
    assert all(w['workspace'] == room_slot for w in wait_windows(2))
    action(room, 'restore', relaunch=True)
    assert len(launches()) == before
    print('PASS LayoutOnly native/X11 relocation and repeat without launches', flush=True)
    close_clients()
    print('READY relogin/reboot; layout retained and test clients closed', flush=True)


def debounce():
    room, _ = owned()['rooms']
    action(room, 'save')
    path = layout_path(room)
    stamp = path.stat().st_mtime_ns
    client = next(r for r in reversed(launches()) if r['kind'] == 'native')
    samples = []
    for index in range(12):
        (DATA / f"command-{client['pid']}").write_text('title:P09 native ' + str(index))
        time.sleep(.2)
        samples.append(path.stat().st_mtime_ns)
    assert all(value == stamp for value in samples), 'write during ongoing changes'
    time.sleep(3)
    after = path.stat().st_mtime_ns
    assert after != stamp, 'no debounced persistence'
    time.sleep(3)
    assert path.stat().st_mtime_ns == after, 'write during unchanged idle'
    (DATA / 'debounce.json').write_text(json.dumps(dict(before=stamp, during=samples, after=after)))
    (DATA / f"command-{client['pid']}").write_text('title:P09 native')
    time.sleep(.5)
    action(room, 'save')
    action(room, 'restore', relaunch=False)
    print('PASS 12 relevant changes coalesced, stable idle produces no further writes', flush=True)


def restart():
    room, _ = owned()['rooms']
    state = snapshot()
    assert state['window-snapshot']['active_workspace'] == 0, 'login must start in foyer'
    assert not [w for w in windows() if w['title'].startswith('P09 ')]
    before = len(launches())
    for r in state['room-snapshot']['snapshot']['rooms'][:3]:
        send(type='switch-workspace', workspace=r['workspace'])
    assert len(launches()) == before, 'room switches launched apps'
    result = action(room, 'restore', relaunch=True)
    assert all(r['message'] == 'Fenster angeordnet' for r in result['results']), result
    wait_windows(2)
    assert len(launches()) == before + 2
    action(room, 'restore', relaunch=True)
    assert len(launches()) == before + 2
    print('PASS persisted restore after session restart, native/X11 relaunch, idempotence, no room-switch launch', flush=True)


def failures():
    room, _ = owned()['rooms']
    path = layout_path(room)
    original = path.read_bytes()
    try:
        for data in (b'invalid = [', b'schema_version = 99\n'):
            path.write_bytes(data)
            result = action(room, 'restore', relaunch=True)
            assert 'fehlgeschlagen' in result['message'], result
            action(room, 'save')
            assert path.read_bytes() == data, 'invalid state overwritten'
        path.unlink()
        path.mkdir()
        result = action(room, 'save')
        assert 'fehlgeschlagen' in result['message'], result
        path.rmdir()
    finally:
        path.write_bytes(original)
    print('PASS corrupt/newer schema retained and storage failure visible', flush=True)


def cleanup():
    record = owned()
    close_clients()
    for room in reversed(record['rooms']):
        mutate(operation='delete', id=room, target_id=record['original'][0]['id'])
        layout_path(room).unlink(missing_ok=True)
    for path in record['desktop_files']:
        Path(path).unlink()
    assert snapshot()['room-snapshot']['snapshot']['rooms'] == record['original']
    OWNER.rename(DATA / ('completed-' + str(time.time_ns()) + '.json'))
    print('PASS only owned test data removed; original room definitions unchanged', flush=True)


if __name__ == '__main__':
    modes = dict(setup=setup, debounce=debounce, existing=existing, restart=restart, failures=failures, cleanup=cleanup)
    modes[sys.argv[1]]()
