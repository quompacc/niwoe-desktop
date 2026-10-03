#!/usr/bin/env python3
"""Real DRM regression: a named screenshot must come from that output.

Uses two distinguishable real modes and actual consent input, restores config
in finally. No fabricated shell consent response and no unowned file removal.
"""
import json
from pathlib import Path
import runpy
import socket
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
label, = sys.argv[1:]
assert label in ('before', 'after')
data = ROOT / ('target/p12-evidence/screenshot-' + label)
data.mkdir(parents=True, exist_ok=False)
_, env = p.environment()
config = Path(env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe/config.toml'
original = config.read_bytes()
assert b'[outputs' not in original, 'this fixture requires no personal output overrides'
baseline = p.snapshot()['output-workspace-snapshot']['outputs']
assert {o['output_name'] for o in baseline} == {'drm-0', 'drm-1'}
assert any(m['width'] == 1600 and m['height'] == 900 for o in baseline if o['output_name'] == 'drm-1' for m in o['modes'])
(data / 'config-before.toml').write_bytes(original)
records = []
try:
    config.write_bytes(original + b'\n[outputs."drm-0"]\nprimary = true\n[outputs."drm-1".mode]\nwidth = 1600\nheight = 900\n')
    p.send(type='reload-config')
    time.sleep(3)
    outputs = p.snapshot()['output-workspace-snapshot']['outputs']
    assert next(o for o in outputs if o['output_name'] == 'drm-1')['width'] == 1600
    for output in ('drm-0', 'drm-1', None):
        target = next(o for o in outputs if o['output_name'] == (output or 'drm-0'))
        request_id = 'p12-output-' + str(time.time_ns())
        connection = p.Connection()
        connection.until('room-snapshot')
        with socket.socket(socket.AF_UNIX) as bridge:
            bridge.settimeout(20)
            bridge.connect(env['XDG_RUNTIME_DIR'] + '/niwoe.sock')
            request = dict(request_id=request_id, kind='full-output', output=output,
                           include_cursor=False, region=None,
                           metadata=dict(origin='portal-dbus', requester='P12 output regression',
                                         identity_trusted=False, interactive=False))
            bridge.sendall((json.dumps(dict(type='screenshot-request', request=request)) + '\n').encode())
            connection.until('screenshot-consent-request', lambda e: e['request_id'] == request_id)
            time.sleep(.5)
            ui['key']('enter')
            with bridge.makefile() as reader:
                for line in reader:
                    result = json.loads(line)
                    if result['type'] == 'screenshot-response': break
                else: raise AssertionError('bridge closed')
            assert result['result']['status'] == 'success', result
            source = Path(result['result']['response']['file_descriptor_token'])
            image = source.read_bytes()
            source.unlink()
            actual = [int.from_bytes(image[16:20], 'big'), int.from_bytes(image[20:24], 'big')]
            expected = [target['width'], target['height']]
            (data / ((output or 'default') + '.png')).write_bytes(image)
            records.append(dict(output=output, expected=expected, actual=actual, passed=actual == expected))
            (data / 'results.json').write_text(json.dumps(records, indent=2))
            print('PASS' if actual == expected else 'FAIL', output, expected, actual, flush=True)
        connection.close()
    assert all(r['passed'] for r in records), records
finally:
    # Removing an override keeps the current hardware mode. Explicitly restore
    # the observed modes before restoring the exact personal file bytes.
    restore_modes = ''.join('\n[outputs."' + o['output_name'] + '".mode]\n'
        + f'width = {o["width"]}\nheight = {o["height"]}\n'
        + f'refresh_millihz = {o["refresh_millihz"]}\n' for o in baseline)
    config.write_bytes(original + restore_modes.encode())
    p.send(type='reload-config')
    time.sleep(3)
    config.write_bytes(original)
    p.send(type='reload-config')
    time.sleep(3)
    assert config.read_bytes() == original
    restored = p.snapshot()['output-workspace-snapshot']['outputs']
    fields = ('output_name', 'width', 'height', 'scale_millis', 'refresh_millihz')
    assert [[o[k] for k in fields] for o in restored] == [[o[k] for k in fields] for o in baseline]
    print('PASS personal config and real output modes restored', flush=True)
