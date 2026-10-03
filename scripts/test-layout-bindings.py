#!/usr/bin/env python3
"""Regression: restore keeps proven session bindings and explicit file metadata."""
import importlib.util
from pathlib import Path
import time

spec = importlib.util.spec_from_file_location('p', Path(__file__).with_name('test-layout-restore.py'))
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)


def main():
    room, _ = p.owned()['rooms']
    saved = p.tomllib.loads(p.layout_path(room).read_text())
    entry = next(e for e in saved['entries'] if e['app']['kind'] == 'native')
    document = p.DATA / 'binding-document.txt'
    document.write_text('Explicit file metadata retained across restore and Save')
    p.action(room, 'set-file', key=entry['key'], path=str(document))
    for _ in range(2):
        result = p.action(room, 'restore', relaunch=False)
        assert all(r['message'] == 'Fenster angeordnet' for r in result['results']), result
    stamp = p.layout_path(room).stat().st_mtime_ns
    time.sleep(3)
    assert p.layout_path(room).stat().st_mtime_ns == stamp, 'restore rearmed autosave'
    p.action(room, 'save')
    saved = p.tomllib.loads(p.layout_path(room).read_text())
    assert next(e for e in saved['entries'] if e['key'] == entry['key'])['file'] == str(document)
    p.action(room, 'restore', relaunch=False)
    p.close_clients()
    for _ in range(2):
        p.launch('native')
    for window in p.wait_windows(2):
        p.send(type='move-window-to-room', id=window['id'], room_id=room)
    p.action(room, 'save')
    for _ in range(3):
        result = p.action(room, 'restore', relaunch=False)
        assert all(r['message'] == 'Fenster angeordnet' for r in result['results']), result
    print('PASS explicit files survive restore/Save; same-title session bindings survive repeated restore; autosave stays disarmed', flush=True)


if __name__ == '__main__':
    main()
