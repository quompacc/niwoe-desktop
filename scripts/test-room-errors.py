#!/usr/bin/env python3
"""Isolated startup/IPC failures. Run under dbus-run-session with parent socket."""
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import tempfile
import time
import tomllib

repo = Path(__file__).resolve().parent.parent
binary = repo / 'target/release/niwoe'
parent = Path(sys.argv[1]).resolve()
assert parent.is_socket(), 'Parent Wayland socket required'
profile = Path(tempfile.mkdtemp(prefix='niwoe-p06-errors.'))
env = os.environ.copy()
for key in ('DISPLAY', 'NIWOE_IPC_TOKEN', 'SESSION_MANAGER'):
    env.pop(key, None)
for key, relative in dict(HOME='home', XDG_CONFIG_HOME='config',
                          XDG_DATA_HOME='data', XDG_CACHE_HOME='cache',
                          XDG_RUNTIME_DIR='runtime', XDG_CONFIG_DIRS='system').items():
    path = profile / relative
    path.mkdir(mode=0o700)
    env[key] = str(path)
env.update(WAYLAND_DISPLAY=str(parent), GSETTINGS_BACKEND='memory', RUST_LOG='info')
directory = profile / 'config/niwoe'
directory.mkdir()
rooms_file = directory / 'rooms.toml'
print('Evidence:', profile, flush=True)

for name, contents, diagnostic in (
    ('broken', 'broken!', 'TOML'),
    ('future', 'schema_version = 999\nnew_field = true\n', 'Raumschema: 999'),
    ('duplicate-id', 'schema_version = 2\nrevision = 0\nnext_id = 2\n'
     '[[rooms]]\nid = 1\nname = "A"\ndescription = ""\nassignment = "free"\n'
     '[[rooms]]\nid = 1\nname = "B"\ndescription = ""\nassignment = "free"\n', 'Raum-ID'),
    ('zero-id', 'schema_version = 2\nrevision = 0\nnext_id = 2\n'
     '[[rooms]]\nid = 0\nname = "A"\ndescription = ""\nassignment = "free"\n', 'Raum-ID'),
):
    rooms_file.write_text(contents)
    result = subprocess.run([str(binary)], env=env, capture_output=True, timeout=10)
    output = result.stdout + result.stderr
    (profile / (name + '.log')).write_bytes(output)
    assert result.returncode != 0, name
    assert diagnostic in output.decode(errors='replace'), output.decode(errors='replace')
    assert rooms_file.read_text() == contents, 'Invalid file overwritten'
    print('PASS startup rejects', name, 'and preserves file', flush=True)

rooms_file.write_text('''schema_version = 1
revision = 0
next_id = 2
[[rooms]]
id = 1
name = "Only Room"
description = ""
assignment = "free"
''')
legacy_bytes = rooms_file.read_bytes()


class Connection:
    def __init__(self, token):
        self.socket = socket.socket(socket.AF_UNIX)
        self.socket.settimeout(8)
        self.socket.connect(str(profile / 'runtime/niwoe.sock'))
        self.reader = self.socket.makefile()
        self.send(type='authenticate', role='shell', token=token)
        self.initial = self.until('room-snapshot')['snapshot']

    def send(self, **event):
        self.socket.sendall((json.dumps(event) + '\n').encode())

    def until(self, kind, request=None):
        for line in self.reader:
            event = json.loads(line)
            if event['type'] == kind and (request is None or event['request_id'] == request):
                return event
        raise AssertionError('Unexpected IPC disconnect')

    def mutate(self, request, revision, change):
        self.send(type='mutate-room', request_id=request,
                  expected_revision=revision, change=change)

    def close(self):
        self.reader.close()
        self.socket.close()


connections = []
with (profile / 'compositor.log').open('w') as log:
    process = subprocess.Popen([str(binary)], env=env, stdout=log, stderr=log,
                               start_new_session=True)
try:
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        assert process.poll() is None, 'Nested compositor exited'
        pids = subprocess.run(['pgrep', '-P', str(process.pid), '-x', 'niwoe-shell'],
                              capture_output=True, text=True).stdout.split()
        if len(pids) == 1 and (profile / 'runtime/niwoe.sock').exists():
            parts = Path(f'/proc/{pids[0]}/environ').read_bytes().split(b'\0')
            shell_env = dict(p.split(b'=', 1) for p in parts if b'=' in p)
            token = shell_env[b'NIWOE_IPC_TOKEN'].decode()
            break
        time.sleep(.1)
    else:
        raise AssertionError('Shell startup timeout')
    first = Connection(token)
    connections.append(first)
    assert rooms_file.with_suffix('.toml.v1.bak').read_bytes() == legacy_bytes
    assert tomllib.loads(rooms_file.read_text())['schema_version'] == 2
    assert first.initial['rooms'][0]['preferences'] == dict(
        icon=None, apps=[], layout='tiling', restore='disabled')
    print('PASS schema 1 migrated with exact backup and inert defaults', flush=True)
    before = rooms_file.read_bytes()
    first.mutate('last', 0, dict(operation='delete', id=1, target_id=1))
    rejected = first.until('room-mutation-result', 'last')
    assert rejected['error'] == 'invalid', rejected
    assert rooms_file.read_bytes() == before
    print('PASS IPC rejects deletion of only room without persistence change', flush=True)
    second = Connection(token)
    connections.append(second)
    assert first.initial == second.initial
    for connection, name in ((first, 'Writer A'), (second, 'Writer B')):
        connection.mutate(name, 0, dict(operation='rename', id=1, name=name))
    results = [first.until('room-mutation-result', 'Writer A'),
               second.until('room-mutation-result', 'Writer B')]
    assert sum(r['error'] is None for r in results) == 1, results
    assert sum(r['error'] == 'conflict' for r in results) == 1, results
    winner = next(r['request_id'] for r in results if r['error'] is None)
    stored = tomllib.loads(rooms_file.read_text())
    assert stored['revision'] == 1 and stored['rooms'][0]['name'] == winner, stored
    reconnect = Connection(token)
    connections.append(reconnect)
    assert reconnect.initial['revision'] == 1
    assert reconnect.initial['rooms'][0]['name'] == winner
    (profile / 'result.json').write_text(json.dumps(dict(
        last=rejected, writers=results, reconnect=reconnect.initial), indent=2))
    print('PASS two writers with same revision: one commit, one conflict; reconnect/persistence agree', flush=True)
    revision = reconnect.initial['revision']
    for number in range(2, 65):
        request = 'capacity-' + str(number)
        reconnect.mutate(request, revision, dict(operation='create', name='Room ' + str(number)))
        result = reconnect.until('room-mutation-result', request)
        assert result['error'] is None, result
        revision += 1
    maximum = rooms_file.read_bytes()
    reconnect.mutate('overflow', revision, dict(operation='create', name='Room 65'))
    overflow = reconnect.until('room-mutation-result', 'overflow')
    assert overflow['error'] == 'invalid', overflow
    assert rooms_file.read_bytes() == maximum
    capacity = Connection(token)
    connections.append(capacity)
    entries = capacity.initial['rooms']
    assert len(entries) == 64
    assert len({r['id'] for r in entries}) == 64
    assert len({r['workspace'] for r in entries}) == 64
    assert capacity.initial['revision'] == revision
    (profile / 'capacity.json').write_text(json.dumps(dict(
        snapshot=capacity.initial, rejected=overflow), indent=2))
    print('PASS IPC capacity: 64 distinct rooms/slots, room 65 rejected without file mutation', flush=True)
    preferences = dict(icon='applications-development', apps=[
        dict(kind='native', id='org.example.Editor'),
        dict(kind='xwayland', id='Editor')], layout='floating', restore='layout-only')
    capacity.mutate('preferences', revision, dict(
        operation='set-preferences', id=1, preferences=preferences))
    assert capacity.until('room-mutation-result', 'preferences')['error'] is None
    revision += 1
    valid_bytes = rooms_file.read_bytes()
    invalid = preferences | dict(apps=preferences['apps'] * 2)
    capacity.mutate('invalid-preferences', revision, dict(
        operation='set-preferences', id=1, preferences=invalid))
    assert capacity.until('room-mutation-result', 'invalid-preferences')['error'] == 'invalid'
    assert rooms_file.read_bytes() == valid_bytes
    capacity.mutate('preserve-preferences', revision, dict(
        operation='update-details', id=1, name='Metadata preserved', description='P06'))
    revision += 1
    assert capacity.until('room-mutation-result', 'preserve-preferences')['error'] is None
    final = Connection(token)
    connections.append(final)
    assert final.initial['revision'] == revision
    assert final.initial['rooms'][0]['preferences'] == preferences
    assert tomllib.loads(rooms_file.read_text())['rooms'][0]['preferences'] == preferences
    assert rooms_file.with_suffix('.toml.v1.bak').read_bytes() == legacy_bytes
    (profile / 'preferences.json').write_text(json.dumps(final.initial, indent=2))
    print('PASS preferences through IPC/storage/reconnect; invalid update atomic; name edit preserves metadata', flush=True)
finally:
    for connection in connections:
        connection.close()
    # Only the isolated compositor and its own process group are terminated.
    if process.poll() is None:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait(timeout=5)
