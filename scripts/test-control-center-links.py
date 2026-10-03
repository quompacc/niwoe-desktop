#!/usr/bin/env python3
"""Final visible General links and catalog paging; never saves user room data."""
import importlib.util
from pathlib import Path
import subprocess

from PIL import Image

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
original = ui.rooms()


def open_first():
    ui.manage()
    ui.click(430, 370)


def rows(label):
    subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', label], check=True)
    return Image.open(ui.ROOT / 'target/p08-live' / (label + '.png')).convert('RGB').crop((280, 420, 1850, 710)).tobytes()


try:
    open_first()
    ui.click(500, 775)  # General -> actual Apps tab, no F-key navigation.
    first = rows('p10-catalog-first')
    ui.click(1600, 389)
    second = rows('p10-catalog-second')
    assert second != first, 'Next must display different real catalog rows'
    ui.click(500, 389)
    assert rows('p10-catalog-mouse-back') == first
    ui.key('tab', 'enter')  # Previous focus -> Next.
    assert rows('p10-catalog-keyboard-next') == second
    ui.key('backtab', 'enter')
    assert rows('p10-catalog-keyboard-back') == first
    # Keyboard General -> Apps gives the same real catalog page.
    open_first()
    ui.key(*(['tab'] * 7), 'enter')
    assert rows('p10-catalog-keyboard-link') == first
    for label, tabs, point in [('restore', 6, (650, 653)), ('files', 8, (950, 775))]:
        for keyboard in (False, True):
            open_first()
            connection = ui.p.Connection()
            connection.until('room-snapshot')
            if keyboard: ui.key(*(['tab'] * tabs), 'enter')
            else: ui.click(*point)
            notice = connection.until('layout', lambda e: e['request_id'].startswith('status-'))
            connection.close()
            assert notice['notice']['room_id'] == original[0]['id'], notice
            print('PASS General link', label, 'keyboard' if keyboard else 'mouse', flush=True)
    assert ui.rooms() == original
    print('PASS real catalog next/previous by mouse+keyboard; all General links; no persistent room changes', flush=True)
finally:
    ui.key(*(['escape'] * 5))
    assert ui.rooms() == original
