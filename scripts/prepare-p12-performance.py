#!/usr/bin/env python3
"""Match baseline output workload temporarily, then restore exact settings."""
import json
from pathlib import Path
import runpy
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
data = ROOT / 'target/p12-evidence/performance-setup'
_, env = p.environment()
config = Path(env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe/config.toml'
stage, = sys.argv[1:]
assert not p.windows()


def configure(original, outputs):
    overrides = ''
    for output in outputs:
        assert output['output_name'] in ('drm-0', 'drm-1')
        overrides += '\n[outputs."' + output['output_name'] + '"]\n'
        overrides += 'primary = ' + str(output['primary']).lower() + '\n'
        overrides += 'scale = ' + str(output['scale_millis'] / 1000) + '\n'
        overrides += f'mode = {{ width = {output["width"]}, height = {output["height"]}, refresh_millihz = {output["refresh_millihz"]} }}\n'
    config.write_bytes(original + overrides.encode())
    p.send(type='reload-config')
    time.sleep(4)
    actual = p.snapshot()['output-workspace-snapshot']['outputs']
    fields = ('output_name', 'width', 'height', 'scale_millis', 'refresh_millihz')
    assert [[o[k] for k in fields] for o in actual] == [[o[k] for k in fields] for o in outputs]


if stage == 'apply':
    data.mkdir(parents=True, exist_ok=False)
    original = config.read_bytes()
    assert b'[outputs' not in original
    (data / 'config-before.toml').write_bytes(original)
    (data / 'outputs-before.json').write_text(json.dumps(p.snapshot()['output-workspace-snapshot']['outputs'], indent=2))
    baseline = json.loads((data.parent / 'before-state.json').read_text())
    configure(original, baseline['output-workspace-snapshot']['outputs'])
    ui['key'](*(['escape'] * 5))
    ui['click'](400, 250)
    with ui['lib']['VirtualKeyboard']() as keys: keys.combo(125, 2)
    time.sleep(.5)
    (data / 'state-applied.json').write_text(json.dumps(p.snapshot(), indent=2))
elif stage == 'restore':
    original = (data / 'config-before.toml').read_bytes()
    configure(original, json.loads((data / 'outputs-before.json').read_text()))
    config.write_bytes(original)
    p.send(type='reload-config')
    time.sleep(3)
    assert config.read_bytes() == original
    (data / 'state-restored.json').write_text(json.dumps(p.snapshot(), indent=2))
else:
    raise AssertionError('unknown stage')
print('PASS performance setup', stage, flush=True)
