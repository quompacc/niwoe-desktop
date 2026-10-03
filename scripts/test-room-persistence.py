#!/usr/bin/env python3
"""Prepare/verify room data around an isolated compositor process restart."""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time
import tomllib

pid, = subprocess.check_output(['pgrep', '-P', sys.argv[1], '-x', 'niwoe-shell'], text=True).split()
env = dict(part.split(b'=', 1) for part in Path(f'/proc/{pid}/environ').read_bytes().split(b'\0') if b'=' in part)
runtime = Path(env[b'XDG_RUNTIME_DIR'].decode())
assert str(runtime).startswith('/tmp/niwoe-p01.'), 'isolated profile required'
token = env.pop(b'NIWOE_IPC_TOKEN').decode()
config = Path(env[b'XDG_CONFIG_HOME'].decode()) / 'niwoe/rooms.toml'
expected = runtime.parent / 'persistence-expected.json'
sock = socket.socket(socket.AF_UNIX)
sock.settimeout(5)
sock.connect(str(runtime / 'niwoe.sock'))
def send(**event):
    sock.sendall((json.dumps(event) + '\n').encode())
send(type='authenticate', role='shell', token=token)
reader = sock.makefile()
def until(kind, predicate=lambda e: True):
    while True:
        line = reader.readline()
        assert line, 'IPC disconnected'
        event = json.loads(line)
        if event['type'] == kind and predicate(event):
            return event
initial = until('window-snapshot')
def snapshot():
    send(type='request-room-snapshot')
    return until('room-snapshot')['snapshot']
def mutate(change):
    request = str(time.time_ns())
    send(type='mutate-room', request_id=request, expected_revision=snapshot()['revision'], change=change)
    result = until('room-mutation-result', lambda e: e['request_id'] == request)
    assert result['error'] is None, result
if sys.argv[2] == 'prepare':
    mutate(dict(operation='create-details', name='P06 Restart', description='persisted-description', assignment='preferred'))
    new = next(r for r in snapshot()['rooms'] if r['name'] == 'P06 Restart')
    mutate(dict(operation='set-preferences', id=new['id'], preferences=dict(
        icon='applications-development', apps=[dict(kind='native', id='org.example.Editor'),
        dict(kind='xwayland', id='Editor')], layout='floating', restore='relaunch-apps')))
    mutate(dict(operation='move', id=new['id'], position=0))
    expected.write_text(json.dumps(tomllib.loads(config.read_text())))
    print('PASS prepared persisted metadata, stable ID and changed room order')
else:
    saved = json.loads(expected.read_text())
    actual = snapshot()
    assert tomllib.loads(config.read_text()) == saved
    assert actual['revision'] == saved['revision']
    # TOML omits an absent Option; the JSON snapshot explicitly sends null.
    for room in saved['rooms']:
        room['preferences'].setdefault('icon', None)
    assert [{k:v for k,v in r.items() if k != 'workspace'} for r in actual['rooms']] == saved['rooms']
    assert initial['active_workspace'] == 0, initial
    assert not initial['windows'], initial
    print('PASS new compositor reloads IDs/order/all preferences/revision and starts in neutral foyer')
