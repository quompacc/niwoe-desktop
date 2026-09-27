#!/usr/bin/env python3
"""Actual keyboard restore and mouse/keyboard explicit-file configuration."""
import importlib.util
from pathlib import Path
import subprocess
import time

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p


def main():
    room, _ = p.owned()['rooms']
    original = p.layout_path(room).read_bytes()
    document = Path('/tmp/niwoe-p10-explicit-file')
    with document.open('x') as file: file.write('P10 explicit test document')
    try:
        p.close_clients()
        ui.manage(False)
        ui.open_room('p09 layout')
        ui.key('backtab', 'backtab', 'enter')  # General name -> Icon -> Restore tab.
        # Disabled restore preference initially focuses Save. Select explicit relaunch.
        before = len(p.launches())
        ui.key('tab', 'tab', 'enter')
        p.wait_windows(2)
        assert len(p.launches()) == before + 2
        ui.key('backtab', 'enter')  # LayoutOnly, no new launches.
        assert len(p.launches()) == before + 2
        revision = p.tomllib.loads(p.layout_path(room).read_text())['revision']
        ui.key('backtab', 'enter')  # Save via keyboard.
        assert p.tomllib.loads(p.layout_path(room).read_text())['revision'] == revision + 1
        ui.click(484, 208)  # Visible Files tab, same persisted P09 capability.
        ui.click(500, 505)  # Select actual first layout entry.
        ui.click(500, 981)
        ui.key('text:' + str(document))
        ui.click(1820, 981)
        stored = p.tomllib.loads(p.layout_path(room).read_text())
        assert stored['entries'][0]['file'] == str(document), stored
        subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-explicit-file'], check=True)
        # Remove the same reference using only keyboard focus and typing.
        ui.key('backtab', *(['backspace'] * len(str(document))), 'tab', 'enter')
        assert 'file' not in p.tomllib.loads(p.layout_path(room).read_text())['entries'][0]
        ui.click(606, 208)
        ui.click(1080, 275)  # Stop debounced tracking through an explicit restore.
        print('PASS keyboard relaunch/LayoutOnly/save; explicit file set by mouse and removed by keyboard', flush=True)
    finally:
        p.close_clients()
        p.layout_path(room).write_bytes(original)
        document.unlink()


if __name__ == '__main__':
    main()
