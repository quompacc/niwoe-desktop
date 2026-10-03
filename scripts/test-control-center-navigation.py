#!/usr/bin/env python3
"""Remaining room list/filter/sort and keyboard enum/order actions, real clients."""
import importlib.util
from pathlib import Path
import subprocess

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p


def room(): return ui.wait_room('p10-mouse')


def main():
    baseline = room()
    p.close_clients()
    for kind in ('native', 'x11'): p.launch(kind)
    assert all(w['workspace'] == baseline['workspace'] for w in p.wait_windows(2))
    ui.manage()
    ui.search('p10')
    ui.click(464, 276)  # occupied
    ui.click(430, 370)
    ui.click(950, 339)
    ui.key('text:p10-filter-proof')
    ui.click(1810, 1051)
    assert room()['description'] == 'p10-filter-proof'
    ui.click(581, 276)  # empty: p10-keyboard only
    subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-empty-filter'], check=True)
    ui.click(715, 276)  # alphabetic order
    ui.click(430, 370)
    ui.click(950, 339)
    ui.key('text:p10-empty-proof')
    ui.click(1810, 1051)
    assert ui.wait_room('p10-keyboard')['description'] == 'p10-empty-proof'
    ui.click(334, 276)  # all
    # Toggle each filter/sort by reverse keyboard navigation, then open sole occupied room.
    ui.manage(False)
    ui.key('backtab', 'enter', 'backtab', 'enter', 'backtab', 'enter', 'tab', 'tab', 'tab', 'enter')
    ui.click(950, 339)
    ui.key('text:' + baseline['description'])
    ui.click(1810, 1051)
    assert room()['description'] == baseline['description']
    ui.click(334, 276)
    # Dedicated and Free really persist, then restore Preferred. Restore preference
    # also cycles through explicit RelaunchApps and back without launching apps.
    before_launches = len(p.launches())
    for expected, presses in [('dedicated', 1), ('free', 1), ('preferred', 1)]:
        ui.open_room('p10-mouse')
        ui.key(*(['tab'] * 4), *(['enter'] * presses), *(['tab'] * 7), 'enter')
        assert room()['assignment'] == expected
    ui.open_room('p10-mouse')
    ui.key(*(['tab'] * 5), 'enter', 'enter', 'enter', *(['tab'] * 6), 'enter')
    assert room()['preferences']['restore'] == baseline['preferences']['restore']
    assert len(p.launches()) == before_launches
    # Keyboard reorder/save and inverse reorder/save preserve slot and stable ID.
    order = [r['id'] for r in ui.rooms()]
    ui.open_room('p10-mouse')
    ui.key('tab', 'tab', 'enter', *(['tab'] * 9), 'enter')
    changed = [r['id'] for r in ui.rooms()]
    assert changed.index(baseline['id']) == order.index(baseline['id']) - 1
    ui.open_room('p10-mouse')
    ui.key('tab', 'tab', 'tab', 'enter', *(['tab'] * 8), 'enter')
    assert [r['id'] for r in ui.rooms()] == order
    assert room() == baseline
    p.close_clients()
    print('PASS actual occupancy/sort/filter mouse+keyboard, all assignment modes, restore cycle, keyboard reorder/save', flush=True)


if __name__ == '__main__':
    main()
