#!/usr/bin/env python3
"""Select actual desktop catalog identities, then observe real native/X11 policy."""
import importlib.util
from pathlib import Path
import time

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p


def main():
    room = next(r for r in ui.rooms() if r['name'] == 'p10-mouse')
    assert room['id'] in ui.owned()['rooms']
    p.close_clients()
    ui.manage()
    ui.open_room('p10-mouse')
    ui.click(500, 775)  # General -> Apps.
    ui.click(500, 339)
    ui.key('text:p09')
    ui.click(500, 439)  # P09 native.
    ui.key('tab', 'tab', 'tab', 'space', 'tab', 'tab', 'tab', 'enter')
    saved = next(r for r in ui.rooms() if r['id'] == room['id'])
    assert saved['preferences']['apps'] == [dict(kind='native', id='org.niwoe.P09Native'), dict(kind='xwayland', id='P09X11')], saved
    ui.persisted(saved)
    active = p.snapshot()['window-snapshot']['active_workspace']
    for kind in ('native', 'x11'): p.launch(kind)
    clients = p.wait_windows(2)
    assert all(w['workspace'] == room['workspace'] for w in clients), clients
    assert p.snapshot()['window-snapshot']['active_workspace'] == active
    print('PASS actual native/XWayland catalog selection by mouse/keyboard and Preferred placement without focus theft', flush=True)
    # A populated room is deleted through its visible two-step confirmation.
    ui.manage()
    ui.click(1760, 276)
    ui.key('text:p10-delete')
    ui.click(1810, 1051)
    doomed = ui.wait_room('p10-delete')
    ui.remember(doomed)
    for window in clients: p.send(type='move-window-to-room', id=window['id'], room_id=doomed['id'])
    ui.open_room('p10-delete')
    ui.click(1500, 967)
    ui.key('enter')  # Explicitly choose first, original destination room.
    ui.click(1820, 967)
    assert any(r['id'] == doomed['id'] for r in ui.rooms()), 'first click must only confirm'
    ui.click(1820, 967)
    assert all(r['id'] != doomed['id'] for r in ui.rooms())
    assert {w['id'] for w in p.wait_windows(2)} == {w['id'] for w in clients}
    assert all(w['workspace'] == ui.owned()['original'][0]['workspace'] for w in p.wait_windows(2))
    print('PASS visible safe deletion: confirmation, stable IDs, both actual clients retained/migrated', flush=True)
    p.close_clients()


if __name__ == '__main__':
    main()
