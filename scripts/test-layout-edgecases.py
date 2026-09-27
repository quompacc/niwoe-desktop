#!/usr/bin/env python3
"""Additional real-window P09 cases; run after test-layout-restore.py restart."""
import importlib.util
import json
import os
from pathlib import Path
import signal
import subprocess
import time

spec = importlib.util.spec_from_file_location('p09', Path(__file__).with_name('test-layout-restore.py'))
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)


def fixture(room, entries):
    lines = ['schema_version = 1', 'revision = 1', f'room_id = {room}', 'mode = "floating"', 'tree = []']
    for key, kind, title, file in entries:
        app_kind = 'xwayland' if kind == 'x11' else 'native'
        app_id = 'P09X11' if kind == 'x11' else 'org.niwoe.P09' + kind.title()
        lines.extend(['[[entries]]', f'key = {key}', f'title = {json.dumps(title)}', 'floating = true',
                      f'app = {{ kind = "{app_kind}", id = {json.dumps(app_id)} }}',
                      'geometry = { x = 2147483647, y = -2147483648, width = 2147483647, height = -20 }',
                      'output = { name = "P09-removed-monitor", scale_millis = 2000, workarea = { x = 4000, y = 0, width = 1920, height = 1080 } }'])
        if file is not None:
            lines.append('file = ' + json.dumps(str(file)))
    p.layout_path(room).write_text('\n'.join(lines) + '\n')


def terminate_owned():
    for entry in p.launches():
        proc = Path('/proc') / str(entry['pid']) / 'cmdline'
        if proc.exists() and any(name in proc.read_bytes() for name in (b'/scripts/p09-client.py', b'/scripts/p09-single-client.py')):
            os.kill(entry['pid'], signal.SIGTERM)
    p.wait_windows(0)


def main():
    room, _ = p.owned()['rooms']
    original = p.layout_path(room).read_bytes()
    terminate_owned()
    try:
        before = len(p.launches())
        fixture(room, [(10, 'missing', 'P09 missing', None), (11, 'native', 'P09 native', p.DATA / 'missing-file.txt')])
        result = p.action(room, 'restore', relaunch=True)
        assert len(p.launches()) == before, result
        assert any('Datei fehlt' in r['message'] for r in result['results']), result
        assert any('Katalog' in r['message'] for r in result['results']), result
        print('PASS missing app and missing explicit file; no app start', flush=True)

        document = p.DATA / 'literal $(echo no-shell) document.txt'
        document.write_text('P09 explicitly selected file')
        fixture(room, [(20, 'native', 'P09 native', document)])
        result = p.action(room, 'restore', relaunch=True)
        assert result['results'][0]['message'] == 'Fenster angeordnet', result
        client = p.launches()[-1]
        assert client['args'] == [str(document)], client
        time.sleep(1)
        size = json.loads((p.DATA / f"client-{client['pid']}.json").read_text())
        outputs = p.snapshot()['output-workspace-snapshot']['outputs']
        assert 32 <= size['height'] <= max(o['height'] for o in outputs), size
        assert 32 <= size['width'] <= max(o['width'] for o in outputs), size
        p.action(room, 'save')
        stored = p.tomllib.loads(p.layout_path(room).read_text())
        geometry = stored['entries'][0]['geometry']
        assert any(o['x'] <= geometry['x'] < o['x'] + o['width'] and o['y'] <= geometry['y'] < o['y'] + o['height'] for o in outputs), geometry
        p.action(room, 'restore', relaunch=False)  # disarm autosave before fixture replacement
        print('PASS literal file argv, invalid geometry, absent-monitor fallback and useful client dimensions', flush=True)

        # Two real windows with the same app ID and title must remain ambiguous.
        p.launch('native')
        p.wait_windows(2)
        fixture(room, [(30, 'native', 'P09 native', None), (31, 'native', 'P09 native', None)])
        before = len(p.launches())
        result = p.action(room, 'restore', relaunch=True)
        assert all('Mehrdeutig' in r['message'] for r in result['results']), result
        assert len(p.launches()) == before
        print('PASS ambiguous same-app windows reported without start-order assignment', flush=True)
        terminate_owned()

        fixture(room, [(40, 'native', 'P09 native', None), (41, 'slow', 'P09 slow', None), (42, 'x11', 'P09 x11', None)])
        before = len(p.launches())
        p.send(type='layout', action=dict(action='restore', room_id=room, relaunch=True))
        deadline = time.monotonic() + 5
        while len(p.launches()) < before + 2 and time.monotonic() < deadline:
            time.sleep(.01)
        result = p.action(room, 'cancel')
        time.sleep(1)
        assert 'abgebrochen' in result['message'], result
        assert len(p.launches()) == before + 2, p.launches()[before:]
        assert all(r['kind'] != 'x11' for r in p.launches()[before:])
        p.action(room, 'restore', relaunch=False)
        assert len(p.launches()) == before + 2
        print('PASS cancellation after two starts; queued third app not started', flush=True)
        terminate_owned()

        # A true single-instance app with no remaining window redirects the new invocation.
        record = p.owned()
        desktop = Path.home() / '.local/share/applications/org.niwoe.P09Single.desktop'
        with desktop.open('x') as file:
            file.write('[Desktop Entry]\nType=Application\nName=P09 Single\nExec=/usr/bin/python3 ' + str(p.ROOT / 'scripts/p09-single-client.py') + '\n')
        record['desktop_files'].append(str(desktop))
        p.OWNER.write_text(json.dumps(record))
        p.send(type='toggle-launcher')
        time.sleep(2)
        p.send(type='toggle-launcher')
        _, env = p.environment()
        subprocess.Popen(['/usr/bin/python3', str(p.ROOT / 'scripts/p09-single-client.py')], env=env,
                         stdout=(p.DATA / 'single.log').open('a'), stderr=subprocess.STDOUT)
        p.wait_windows(1)
        (p.DATA / 'single-command').write_text('hide')
        p.wait_windows(0)
        fixture(room, [(50, 'single', 'P09 single', None)])
        before = len(p.launches())
        start = time.monotonic()
        result = p.action(room, 'restore', relaunch=True)
        elapsed = time.monotonic() - start
        assert 'Weiterleitung' in result['results'][0]['message'], result
        assert 14 <= elapsed < 25, elapsed
        assert (p.DATA / 'forwarded.log').exists(), 'no real D-Bus forwarding observed'
        p.action(room, 'restore', relaunch=True)
        assert len(p.launches()) == before + 1
        print('PASS real single-instance forwarding, finite deadline and no repeated invocation', flush=True)
    finally:
        terminate_owned()
        p.layout_path(room).write_bytes(original)


if __name__ == '__main__':
    main()
