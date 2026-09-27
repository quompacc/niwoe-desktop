#!/usr/bin/env python3
"""P11 isolated compositor/storage checks; never touches personal config.

Run under dbus-run-session with the real parent Wayland socket. Journal fixtures
model crash boundaries on disk; they are not claimed as injected real crashes.
"""
import copy
import json
import os
from pathlib import Path
import shutil
import signal
import socket
import subprocess
import sys
import tempfile
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'target/p11-live'
DATA.mkdir(exist_ok=True)
PARENT = Path(sys.argv[1]).resolve()
assert PARENT.is_socket()


def toml_document(value):
    """Small fixture writer for scalar/table/table-array TOML, no dependency."""
    lines = []
    def scalar(v):
        if isinstance(v, bool): return 'true' if v else 'false'
        if isinstance(v, str): return json.dumps(v, ensure_ascii=False)
        if isinstance(v, list): return '[' + ', '.join(scalar(x) for x in v) + ']'
        return str(v)
    def table(v, path):
        for k, x in v.items():
            if x is not None and not isinstance(x, dict) and not (isinstance(x, list) and x and isinstance(x[0], dict)):
                lines.append(k + ' = ' + scalar(x))
        for k, x in v.items():
            target = path + [k]
            if isinstance(x, dict):
                lines.append('[' + '.'.join(target) + ']')
                table(x, target)
            elif isinstance(x, list) and x and isinstance(x[0], dict):
                for item in x:
                    lines.append('[[' + '.'.join(target) + ']]')
                    table(item, target)
    table(value, [])
    text = '\n'.join(lines) + '\n'
    assert tomllib.loads(text) == value
    return text


class Profile:
    def __init__(self, name, existing=False, legacy=False):
        self.path = Path(tempfile.mkdtemp(prefix='niwoe-p11-'))
        self.name = name
        self.process = None
        self.env = os.environ.copy()
        for key in ('DISPLAY', 'NIWOE_IPC_TOKEN', 'SESSION_MANAGER'):
            self.env.pop(key, None)
        for key, part in dict(HOME='home', XDG_CONFIG_HOME='config', XDG_DATA_HOME='data',
                             XDG_CACHE_HOME='cache', XDG_RUNTIME_DIR='runtime', XDG_CONFIG_DIRS='system').items():
            target = self.path / part
            target.mkdir(mode=0o700)
            self.env[key] = str(target)
        self.env.update(WAYLAND_DISPLAY=str(PARENT), GSETTINGS_BACKEND='memory', RUST_LOG='info')
        self.directory = self.path / 'config/niwoe'
        self.directory.mkdir()
        self.config = b'[cursor]\nsize = 32\n'
        if existing or legacy:
            directory = self.path / ('config/meridian' if legacy else 'config/niwoe')
            directory.mkdir(exist_ok=True)
            (directory / 'config.toml').write_bytes(self.config)

    def start(self):
        log = (DATA / ('protocol-' + self.name + '.log')).open('a')
        self.process = subprocess.Popen([str(ROOT / 'target/release/niwoe')], env=self.env,
                                        stdout=log, stderr=log, start_new_session=True)
        log.close()
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            assert self.process.poll() is None, self.name + ' exited'
            pids = subprocess.run(['pgrep', '-P', str(self.process.pid), '-x', 'niwoe-shell'], capture_output=True, text=True).stdout.split()
            if len(pids) == 1 and (self.path / 'runtime/niwoe.sock').exists():
                env = dict(p.split(b'=', 1) for p in Path(f'/proc/{pids[0]}/environ').read_bytes().split(b'\0') if b'=' in p)
                self.token = env[b'NIWOE_IPC_TOKEN'].decode()
                self.shell = int(pids[0])
                return
            time.sleep(.1)
        raise AssertionError('startup deadline')

    def stop(self):
        if self.process and self.process.poll() is None:
            os.killpg(self.process.pid, signal.SIGTERM)
            try: self.process.wait(timeout=8)
            except subprocess.TimeoutExpired:
                os.killpg(self.process.pid, signal.SIGKILL)
                self.process.wait(timeout=5)
        self.process = None

    def close(self):
        self.stop()
        assert self.path.parent == Path('/tmp') and self.path.name.startswith('niwoe-p11-')
        shutil.rmtree(self.path)

    def request(self, operation, **args):
        request = 'p11-' + str(time.time_ns())
        with socket.socket(socket.AF_UNIX) as sock:
            sock.settimeout(15)
            sock.connect(str(self.path / 'runtime/niwoe.sock'))
            sock.sendall((json.dumps(dict(type='authenticate', role='shell', token=self.token)) + '\n').encode())
            sock.sendall((json.dumps(dict(type='first-run', request_id=request,
                                         action=dict(operation=operation, **args))) + '\n').encode())
            with sock.makefile() as reader:
                for line in reader:
                    event = json.loads(line)
                    if event['type'] == 'first-run' and event['request_id'] == request: return event
        raise AssertionError('disconnected')

    def read(self, name): return tomllib.loads((self.directory / name).read_text())


def draft(profile, add=True):
    rooms = profile.read('rooms.toml')
    return dict(step=4, profile='preserve', rooms_revision=rooms['revision'], panel_revision=0,
                panel=['status', 'search'], practiced=[False]*3,
                rooms=[dict(operation='configure', id=None, name='P11 isolated', description='explicit draft',
                            assignment='free', position=len(rooms['rooms']),
                            preferences=dict(icon=None, apps=[], layout='tiling', restore='disabled'))] if add else [])


results = []
for mode in ('fresh', 'existing', 'legacy'):
    p = Profile(mode, existing=mode == 'existing', legacy=mode == 'legacy')
    try:
        p.start()
        first = p.request('get')
        assert first['error'] is None and first['snapshot']['fresh'] == (mode == 'fresh'), first
        if mode != 'fresh': assert (p.directory / 'config.toml').read_bytes() == p.config
        original = (p.directory / 'rooms.toml').read_bytes()
        d = draft(p)
        saved = p.request('save-draft', expected_revision=0, draft=d)
        assert saved['error'] is None, saved
        assert (p.directory / 'rooms.toml').read_bytes() == original
        assert not (p.directory / 'panel.toml').exists()
        old_shell = p.shell
        os.kill(old_shell, signal.SIGKILL)
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            children = subprocess.run(['pgrep', '-P', str(p.process.pid), '-x', 'niwoe-shell'], capture_output=True, text=True).stdout.split()
            if len(children) == 1 and int(children[0]) != old_shell:
                p.shell = int(children[0])
                break
            time.sleep(.2)
        assert p.shell != old_shell, 'watchdog must replace crashed shell'
        assert p.request('get')['snapshot']['draft'] == d
        # Real process termination/restart preserves the separate draft.
        p.stop(); p.start()
        resumed = p.request('get')
        assert resumed['snapshot']['draft'] == d and not resumed['snapshot']['completed'], resumed
        stale = p.request('save-draft', expected_revision=0, draft=d)
        assert stale['error'] and (p.directory / 'rooms.toml').read_bytes() == original
        complete = p.request('complete', expected_revision=saved['snapshot']['revision'])
        assert complete['error'] is None and complete['snapshot']['completed'], complete
        rooms = p.read('rooms.toml')
        assert len(rooms['rooms']) == len(tomllib.loads(original.decode())['rooms']) + 1
        assert p.read('panel.toml')['modules'] == ['status', 'search']
        assert p.read('first-run.toml')['completed'] and 'journal' not in p.read('first-run.toml')
        repeated = p.request('complete', expected_revision=saved['snapshot']['revision'])
        assert repeated['error'] and p.read('rooms.toml') == rooms
        p.stop(); p.start()
        assert p.request('get')['snapshot']['completed'] and p.read('rooms.toml') == rooms
        if mode != 'fresh': assert (p.directory / 'config.toml').read_bytes() == p.config
        results.append(dict(case=mode, first=first, complete=complete, repeated=repeated))
        print('PASS', mode, 'origin, separate draft, real restart/reconnect, persisted completion, repeat without duplicates', flush=True)
    finally: p.close()

# Recovery from exact on-disk crash boundaries, using validated journal fixtures.
for boundary in ('before-writes', 'after-rooms', 'after-panel', 'conflict'):
    p = Profile(boundary)
    try:
        p.start()
        d = draft(p)
        saved = p.request('save-draft', expected_revision=0, draft=d)
        assert saved['error'] is None
        p.stop()
        before = p.read('rooms.toml')
        after = copy.deepcopy(before)
        after['revision'] += 1
        after['rooms'].append(dict(id=after['next_id'], name='P11 isolated', description='explicit draft', assignment='free',
                                   preferences=dict(apps=[], layout='tiling', restore='disabled')))
        after['next_id'] += 1
        panel_before = dict(schema_version=1, revision=0, modules=['tray', 'screenshot', 'search', 'status'])
        panel_after = dict(schema_version=1, revision=1, modules=['status', 'search'])
        state = p.read('first-run.toml')
        state['revision'] += 1
        state['journal'] = dict(rooms_before=before, rooms_after=after, panel_before=panel_before, panel_after=panel_after)
        (p.directory / 'first-run.toml').write_text(toml_document(state))
        if boundary != 'before-writes': (p.directory / 'rooms.toml').write_text(toml_document(after))
        if boundary == 'after-panel': (p.directory / 'panel.toml').write_text(toml_document(panel_after))
        if boundary == 'conflict':
            conflict = copy.deepcopy(after)
            conflict['revision'] += 1
            conflict['rooms'][0]['name'] = 'Concurrent user change'
            (p.directory / 'rooms.toml').write_text(toml_document(conflict))
        p.start()
        loaded = p.request('get')
        assert loaded['snapshot']['applying'] and not loaded['snapshot']['completed']
        observed = p.read('rooms.toml')
        result = p.request('complete', expected_revision=state['revision'])
        if boundary == 'conflict':
            assert result['error'] and p.read('rooms.toml') == observed
            assert p.read('first-run.toml') == state
        else:
            assert result['error'] is None and result['snapshot']['completed'], result
            assert p.read('rooms.toml') == after and p.read('panel.toml') == panel_after
        results.append(dict(case=boundary, result=result))
        print('PASS persisted journal fixture', boundary, 'checked against actual documents', flush=True)
    finally: p.close()

for case in ('write-denied', 'panel-conflict', 'room-conflict'):
    p = Profile(case)
    try:
        p.start()
        d = draft(p)
        saved = p.request('save-draft', expected_revision=0, draft=d)
        assert saved['error'] is None
        original_state = (p.directory / 'first-run.toml').read_bytes()
        original_rooms = (p.directory / 'rooms.toml').read_bytes()
        if case == 'write-denied':
            p.directory.chmod(0o500)
        elif case == 'panel-conflict':
            (p.directory / 'panel.toml').write_text(toml_document(dict(schema_version=1, revision=1, modules=['status'])))
        else:
            p.stop()
            changed = p.read('rooms.toml')
            changed['revision'] += 1
            changed['rooms'][0]['name'] = 'Concurrent room edit'
            (p.directory / 'rooms.toml').write_text(toml_document(changed))
            original_rooms = (p.directory / 'rooms.toml').read_bytes()
            p.start()
        result = p.request('complete', expected_revision=saved['snapshot']['revision'])
        assert result['error'] and not result['snapshot']['completed'], result
        assert (p.directory / 'rooms.toml').read_bytes() == original_rooms
        assert (p.directory / 'first-run.toml').read_bytes() == original_state
        if case == 'write-denied':
            p.directory.chmod(0o700)
            retry = p.request('complete', expected_revision=saved['snapshot']['revision'])
            assert retry['error'] is None and retry['snapshot']['completed'], retry
        results.append(dict(case=case, result=result))
        print('PASS actual storage rejection/conflict preserves productive data and draft:', case, flush=True)
    finally:
        p.directory.chmod(0o700)
        p.close()

for content in ('broken = [', 'schema_version = 99\nrevision = 0\nfresh = true\ncompleted = false\n'):
    p = Profile('invalid-state')
    try:
        (p.directory / 'first-run.toml').write_text(content)
        p.start()
        result = p.request('get')
        assert result['error'] and result['snapshot'] is None, result
        assert (p.directory / 'first-run.toml').read_text() == content
        results.append(dict(case='invalid-state', result=result))
        print('PASS damaged/unknown state: visible protocol error, original retained', flush=True)
    finally: p.close()

(DATA / 'protocol.json').write_text(json.dumps(results, indent=2))
