#!/usr/bin/env python3
"""Run room IPC integration only inside an isolated NIWOE smoke profile."""
import json, os, socket, subprocess, sys, time, tomllib
from pathlib import Path

pids = subprocess.check_output(['pgrep', '-P', sys.argv[1], '-x', 'niwoe-shell'], text=True).split()
assert len(pids) == 1
env = dict(x.split(b'=', 1) for x in Path(f'/proc/{pids[0]}/environ').read_bytes().split(b'\0') if b'=' in x)
runtime = Path(env[b'XDG_RUNTIME_DIR'].decode())
assert str(runtime).startswith('/tmp/niwoe-p01.'), 'isolated smoke profile required'
token = env.pop(b'NIWOE_IPC_TOKEN').decode()
client_env = os.environ | {k.decode(): v.decode() for k, v in env.items()}

class Connection:
    def __init__(self):
        self.s = socket.socket(socket.AF_UNIX)
        self.s.settimeout(8)
        self.s.connect(str(runtime / 'niwoe.sock'))
        self.r = self.s.makefile()
        time.sleep(0.3)  # Authentication deliberately arrives after accept/poll.
        self.send(type='authenticate', role='shell', token=token)
    def send(self, **event):
        self.s.sendall((json.dumps(event) + '\n').encode())
    def until(self, kind, predicate=lambda e: True):
        while True:
            event = json.loads(self.r.readline())
            if event['type'] == kind and predicate(event): return event
    def snapshot(self):
        self.send(type='request-room-snapshot')
        return self.until('room-snapshot')['snapshot']
    def close(self):
        self.r.close(); self.s.close()

c = Connection()
initial_windows = c.until('window-snapshot')
initial = c.snapshot()
created = []
client = None

def mutate(change, revision=None, error=None):
    snap = c.snapshot()
    request = 'p06-live-' + str(time.time_ns())
    c.send(type='mutate-room', request_id=request,
           expected_revision=snap['revision'] if revision is None else revision, change=change)
    result = c.until('room-mutation-result', lambda e: e['request_id'] == request)
    assert result['error'] == error, result
    return c.snapshot()

try:
    for name in ['P06 Testquelle', 'P06 Testziel']:
        snap = mutate(dict(operation='create-details', name=name, description='Created atomically', assignment='free'))
        room = next(r for r in snap['rooms'] if r['id'] not in {r['id'] for r in initial['rooms']} | set(created))
        created.append(room['id'])
    source, target = created
    snap = mutate(dict(operation='update-details', id=source, name='P06 Livefenster', description='Updated atomically'))
    assert next(r for r in snap['rooms'] if r['id'] == source)['description'] == 'Updated atomically'
    snap = mutate(dict(operation='set-description', id=source, description='Temporäre Liveprüfung'))
    snap = mutate(dict(operation='set-assignment', id=source, assignment='preferred'))
    snap = mutate(dict(operation='move', id=source, position=0))
    assert snap['rooms'][0]['id'] == source
    mutate(dict(operation='rename', id=source, name='Veraltet'), revision=initial['revision'], error='conflict')
    source_room = next(r for r in snap['rooms'] if r['id'] == source)
    c.send(type='switch-workspace', workspace=source_room['workspace'])
    c.until('window-snapshot', lambda e: e['active_workspace'] == source_room['workspace'])
    client = subprocess.Popen(['zenity', '--info', '--title=P06 Livefenster', '--text=Temporary migration test'], env=client_env)
    time.sleep(2)
    c.close(); c = Connection()
    event = c.until('window-snapshot')
    window = next(w for w in event['windows'] if w['title'] == 'P06 Livefenster')
    assert window['workspace'] == source_room['workspace'], window
    snap = mutate(dict(operation='delete', id=source, target_id=target))
    created.remove(source)
    c.close(); c = Connection()
    event = c.until('window-snapshot')
    snap = c.snapshot()
    moved = next(w for w in event['windows'] if w['id'] == window['id'])
    target_room = next(r for r in snap['rooms'] if r['id'] == target)
    assert moved['workspace'] == target_room['workspace'], moved
    assert event['active_workspace'] == target_room['workspace'], event
    stored = tomllib.loads((Path(os.environ['XDG_CONFIG_HOME']) / 'niwoe/rooms.toml').read_text())
    assert stored['revision'] == snap['revision']
    assert [r['id'] for r in stored['rooms']] == [r['id'] for r in snap['rooms']]
    print('PASS create, metadata, reorder, stale-revision conflict, room switch, native window migration, active slot compaction, reconnect, persisted IDs/revision', flush=True)
finally:
    if client:
        client.terminate()
        client.wait(timeout=5)
    c.close(); c = Connection()
    for room_id in created:
        mutate(dict(operation='delete', id=room_id, target_id=initial['rooms'][0]['id']))
    if initial_windows['active_workspace']:
        c.send(type='switch-workspace', workspace=initial_windows['active_workspace'])
    final = c.snapshot()
    assert final['rooms'] == initial['rooms'], 'Original room definitions changed'
    print('PASS cleanup: original room definitions preserved; monotonic IDs/revision advanced', flush=True)
    c.close()
