#!/usr/bin/env python3
"""P11 real layer sizes, all pages, focus, input and rollback in owned profile."""
import json
import runpy
import time

u = runpy.run_path(str(__import__('pathlib').Path(__file__).with_name('test-first-run-ui.py')))
ui, directory = u['ui'], u['DIRECTORY']
config = directory / 'config.toml'
original = config.read_bytes()
rooms = (directory / 'rooms.toml').read_bytes()
panel = (directory / 'panel.toml').read_bytes()
records = []
try:
    for scale in (1.0, 1920 / 1366, 1.5, 2.0):
        ui.key(*(['escape'] * 5))
        config.write_text(f'[outputs."drm-0"]\nprimary = true\nscale = {scale}\n[outputs."drm-1"]\nenabled = false\n')
        ui.p.send(type='reload-config')
        time.sleep(3)
        u['keyboard_open']()
        ui.key('end', 'enter')  # Discard previous optional draft.
        time.sleep(.4)
        for step in range(5):
            if step:
                u['wait_step'](step)
            ui.key('home', 'tab', 'backtab', 'end')
            u['capture'](f'p11-scale-{round(scale*100)}-step-{step}')
            if step == 1:
                u['control'](2, step)
                assert u['draft']()['profile'] == 'keyboard'
            elif step == 2:
                u['control'](5, step)  # Actual catalog pagination, then back.
                u['control'](4, step)
            elif step == 3:
                before = u['draft']()['panel']
                u['control'](0, step)
                assert u['draft']()['panel'] != before
                u['control'](0, step)
            with ui.lib['VirtualPointer'](1920, 1080) as pointer:
                pointer.move_to(600, 500)
                pointer.scroll(-2)
            if step < 4:
                u['footer'](1)
        u['footer'](3)
        assert (directory / 'rooms.toml').read_bytes() == rooms
        assert (directory / 'panel.toml').read_bytes() == panel
        output, = ui.p.snapshot()['output-workspace-snapshot']['outputs']
        assert abs(output['scale_millis'] - scale * 1000) < 1
        logical = [round(output['width'] / scale), round(output['height'] / scale)]
        if 1.4 < scale < 1.41: assert logical == [1366, 768]
        records.append(dict(scale=scale, logical=logical, output=output))
        print('PASS', logical, scale, 'all pages, keyboard focus, mouse choices, wheel, cancel rollback', flush=True)
finally:
    config.write_bytes(original)
    ui.p.send(type='reload-config')
    time.sleep(3)
    assert config.read_bytes() == original
    (u['DATA'] / 'scales.json').write_text(json.dumps(records, indent=2))
