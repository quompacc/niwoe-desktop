#!/usr/bin/env python3
"""P09 real output reload and scale acceptance; restores the exact config bytes."""
import importlib.util
import json
from pathlib import Path
import time

spec = importlib.util.spec_from_file_location('edges', Path(__file__).with_name('test-layout-edgecases.py'))
edges = importlib.util.module_from_spec(spec)
spec.loader.exec_module(edges)
p = edges.p


def main():
    room, _ = p.owned()['rooms']
    config = Path.home() / '.config/niwoe/config.toml'
    original = config.read_bytes()
    layout = p.layout_path(room).read_bytes()
    baseline = p.snapshot()['output-workspace-snapshot']['outputs']
    assert len(baseline) == 2 and all(o['scale_millis'] == 1000 for o in baseline)
    (p.DATA / 'display-config-before.toml').write_bytes(original)
    records = []
    try:
        for kind in ('native', 'x11'):
            p.launch(kind)
        p.wait_windows(2)
        for scale, secondary in ((2.0, True), (1.5, False)):
            extra = f'\n[outputs."drm-0"]\nprimary = true\nscale = {scale}\n[outputs."drm-1"]\nenabled = {str(secondary).lower()}\n'
            config.write_bytes(original + extra.encode())
            p.send(type='reload-config')
            time.sleep(3)
            outputs = p.snapshot()['output-workspace-snapshot']['outputs']
            assert next(o for o in outputs if o['output_name'] == 'drm-0')['scale_millis'] == int(scale * 1000), outputs
            assert len(outputs) == (2 if secondary else 1), outputs
            edges.fixture(room, [(301, 'native', 'P09 native', None), (302, 'x11', 'P09 x11', None)])
            result = p.action(room, 'restore', relaunch=False)
            assert all(r['message'] == 'Fenster angeordnet' for r in result['results']), result
            time.sleep(1)
            p.action(room, 'save')
            saved = p.tomllib.loads(p.layout_path(room).read_text())
            for entry in saved['entries']:
                g, area = entry['geometry'], entry['output']['workarea']
                assert area['x'] <= g['x'] <= area['x'] + area['width'] - g['width'], entry
                assert area['y'] <= g['y'] <= area['y'] + area['height'] - g['height'], entry
                assert 100 <= g['width'] <= area['width'] and 50 <= g['height'] <= area['height'], entry
            p.action(room, 'restore', relaunch=False)
            records.append(dict(scale=scale, secondary=secondary, outputs=outputs, entries=saved['entries']))
            print('PASS native/X11 reachable after actual display reload:', scale, secondary, flush=True)
        (p.DATA / 'displays.json').write_text(json.dumps(records, indent=2))
    finally:
        config.write_bytes(original)
        p.send(type='reload-config')
        time.sleep(3)
        assert config.read_bytes() == original
        restored = p.snapshot()['output-workspace-snapshot']['outputs']
        assert [(o['output_name'], o['width'], o['height'], o['scale_millis']) for o in restored] == [(o['output_name'], o['width'], o['height'], o['scale_millis']) for o in baseline]
        p.layout_path(room).write_bytes(layout)
        edges.terminate_owned()
        print('PASS exact personal config and both output modes restored', flush=True)


if __name__ == '__main__':
    main()
