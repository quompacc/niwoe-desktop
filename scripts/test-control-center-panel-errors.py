#!/usr/bin/env python3
"""Live panel draft revision and actual storage error checks."""
import importlib.util
from pathlib import Path
import stat
import subprocess
import time

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)


def external_save(modules):
    c = ui.p.Connection()
    c.until('room-snapshot')
    request = 'p10-panel-fixture-' + str(time.time_ns())
    c.send(type='panel-preferences', request_id=request, action=dict(operation='get'))
    state = c.until('panel-preferences', lambda e: e['request_id'] == request)
    c.send(type='panel-preferences', request_id=request + '-save', action=dict(operation='save', expected_revision=state['revision'], modules=modules))
    result = c.until('panel-preferences', lambda e: e['request_id'] == request + '-save')
    assert result['error'] is None, result
    c.close()


def main():
    baseline = ui.panel_state()
    assert baseline == ['tray', 'screenshot', 'search', 'status']
    ui.manage()
    ui.key('f7')
    ui.click(700, 447)  # local Search off
    external_save(['status'])
    ui.click(1810, 1051)
    assert ui.panel_state() == ['status'], 'stale draft overwrote concurrent update'
    subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-panel-conflict'], check=True)
    ui.click(1810, 1051)
    assert ui.panel_state() == ['tray', 'screenshot', 'status']
    external_save(baseline)
    ui.key('f7', 'enter')  # local Tray off
    directory = Path.home() / '.config/niwoe'
    mode = stat.S_IMODE(directory.stat().st_mode)
    try:
        directory.chmod(mode & ~0o222)
        ui.click(1810, 1051)
        assert ui.panel_state() == baseline
        subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-panel-storage-error'], check=True)
    finally:
        directory.chmod(mode)
    ui.click(1810, 1051)
    assert ui.panel_state() == baseline[1:]
    external_save(baseline)
    print('PASS panel concurrent writer cannot silently rebase draft; visible storage failure and explicit retries', flush=True)


if __name__ == '__main__':
    main()
