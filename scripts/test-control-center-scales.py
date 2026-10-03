#!/usr/bin/env python3
"""Actual output scale reloads; exact personal configuration restored in finally."""
import importlib.util
import json
import math
from pathlib import Path
import subprocess
import time

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p


def open_keyboard():
    ui.manage(False)
    ui.key('text:x', *(['backspace'] * 32), 'text:p10-mouse', 'enter', 'tab', 'enter')


def main():
    config = Path.home() / '.config/niwoe/config.toml'
    original = config.read_bytes()
    assert '[outputs.' not in original.decode(), 'need preserve existing output sections'
    (ui.DATA / 'scale-config-before.toml').write_bytes(original)
    room = next(r for r in ui.rooms() if r['name'] == 'p10-mouse')
    assert room['id'] in ui.owned()['rooms']
    records = []
    try:
        for scale in (1.0, 1920 / 1366, 1.5, 2.0):
            ui.key(*(['escape'] * 5))
            config.write_bytes(original + f'\n[outputs."drm-0"]\nprimary = true\nscale = {scale}\n[outputs."drm-1"]\nenabled = false\n'.encode())
            p.send(type='reload-config')
            time.sleep(3)
            output, = p.snapshot()['output-workspace-snapshot']['outputs']
            assert abs(output['scale_millis'] - scale * 1000) < 1, output
            # IPC registry reports physical mode bounds; layer-shell configure
            # uses the output's logical dimensions.
            width, height = round(output['width'] / scale), round(output['height'] / scale)
            if 1.4 < scale < 1.41:
                assert (width, height) == (1366, 768)
            fitted = height - 48
            ratio = min(width / 1366, fitted / 768, 1)
            cw, ch = math.ceil(width / ratio), math.ceil(fitted / ratio)
            def canvas(x, y): ui.click(x * width / cw, 48 + y * fitted / ch)
            open_keyboard()
            left_width = (cw - 300) * 3 // 5
            description_x = 346 + (left_width - 106) // 2 + 12
            canvas(description_x + 30, 291)
            text = 'p10-scale-' + str(int(scale * 100))
            ui.key('text:' + text)
            canvas(cw - 110, ch - 29)
            saved = ui.wait_room('p10-mouse')
            assert saved['description'] == text, saved
            ui.persisted(saved)
            open_keyboard()
            ui.key(*(['tab'] * 9))
            subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', 'p10-scale-' + str(int(scale * 100))], check=True)
            with ui.lib['VirtualPointer'](output['width'], output['height']) as pointer:
                pointer.move_to(width // 2, height // 2)
                pointer.scroll(20)
            time.sleep(.3)
            canvas(306, 291)
            canvas(cw - 283, ch - 29)
            assert ui.wait_room('p10-mouse') == saved
            open_keyboard()
            canvas(402, 160)
            canvas(500, 291)
            ui.key('text:p09')
            canvas(500, 391)
            canvas(cw - 283, ch - 29)
            assert ui.wait_room('p10-mouse') == saved
            ui.key('f7', 'enter', 'escape')
            assert ui.panel_state() == ['tray', 'screenshot', 'search', 'status']
            records.append(dict(output=output, logical=[width, height], canvas=[cw, ch], stored=saved))
            print('PASS real scale', scale, 'form hit-tests, durable save, cancel, tab/search, focus+wheel scroll, panel rollback', flush=True)
    finally:
        config.write_bytes(original)
        p.send(type='reload-config')
        time.sleep(3)
        assert config.read_bytes() == original
        (ui.DATA / 'scales.json').write_text(json.dumps(records, indent=2))
    # Restore only the owned room description, through its visible form.
    open_keyboard()
    ui.click(950, 339)
    ui.key('text:' + room['description'])
    ui.click(1810, 1051)
    assert ui.wait_room('p10-mouse') == room
    print('PASS exact personal config and original owned room values restored', flush=True)


if __name__ == '__main__':
    main()
