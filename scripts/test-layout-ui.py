#!/usr/bin/env python3
"""Run while the owned P09 Layout restore tab is visible at 1920x1080."""
import importlib.util
from pathlib import Path
import subprocess
import time

spec = importlib.util.spec_from_file_location('edges', Path(__file__).with_name('test-layout-edgecases.py'))
edges = importlib.util.module_from_spec(spec)
spec.loader.exec_module(edges)
p = edges.p


def ui(*args):
    subprocess.run(['python3', str(p.ROOT / 'scripts/test-hub-data.py'), *args], check=True)


def main():
    room, _ = p.owned()['rooms']
    original = p.layout_path(room).read_bytes()
    try:
        for kind in ('native', 'x11'):
            p.launch(kind)
        p.wait_windows(2)
        ui('click', '1080', '275')
        time.sleep(1)
        result = p.action(room, 'status')
        assert all(r['message'] == 'Fenster angeordnet' for r in result['results']), result
        ui('capture', 'p09-ui-success')
        before = p.tomllib.loads(p.layout_path(room).read_text())['revision']
        ui('click', '530', '275')
        time.sleep(1)
        assert p.tomllib.loads(p.layout_path(room).read_text())['revision'] == before + 1
        p.action(room, 'restore', relaunch=False)
        edges.terminate_owned()
        key = time.time_ns() % 1000000000
        edges.fixture(room, [(key, 'native', 'P09 native', None), (key + 1, 'slow', 'P09 slow', None), (key + 2, 'x11', 'P09 x11', None)])
        before = len(p.launches())
        ui('click', '1640', '275')
        deadline = time.monotonic() + 5
        while len(p.launches()) < before + 2 and time.monotonic() < deadline:
            time.sleep(.1)
        assert len(p.launches()) >= before + 2
        ui('click', '530', '325')
        result = p.action(room, 'status')
        assert result['message'] == 'Wiederherstellung abgebrochen', result
        after_cancel = len(p.launches())
        ui('capture', 'p09-ui-cancelled')
        assert len(p.launches()) == after_cancel
        print('PASS actual pointer save, successful layout, partial-start cancellation and visible results', flush=True)
    finally:
        p.layout_path(room).write_bytes(original)
        edges.terminate_owned()


if __name__ == '__main__':
    main()
