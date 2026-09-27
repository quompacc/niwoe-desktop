#!/usr/bin/env python3
"""Remove only recorded P09/P10 fixtures; finish keyboard deletion acceptance."""
import importlib.util
import json
from pathlib import Path
import subprocess

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p
record = ui.owned()
p.close_clients()
assert not p.windows()
if p.OWNER.exists():
    p09 = p.owned()
    # P09 was set up before later edits to the P10-owned fixtures. Reset those
    # test records only, so its own cleanup can assert its exact starting intent.
    for index, room in enumerate(p09['original']):
        if room['id'] not in record['rooms']: continue
        fields = {k: room[k] for k in ('name', 'description', 'assignment', 'preferences')}
        p.mutate(operation='configure', id=room['id'], position=index, **fields)
    subprocess.run(['python3', str(ui.ROOT / 'scripts/test-layout-restore.py'), 'cleanup'], check=True)
ui.manage(False)
ui.open_room('p10-keyboard')
keyboard_room = ui.wait_room('p10-keyboard')
assert keyboard_room['id'] in record['rooms']
ui.key(*(['tab'] * 9), 'enter', 'enter', 'tab', 'enter')
assert any(r['id'] == keyboard_room['id'] for r in ui.rooms()), 'first keyboard Delete must only confirm'
ui.key('enter')
assert all(r['id'] != keyboard_room['id'] for r in ui.rooms())
print('PASS keyboard destination selection and two-step room deletion', flush=True)
panel = Path.home() / '.config/niwoe/panel.toml'
if 'panel_before' in record:
    if record['panel_before'] is None: panel.unlink(missing_ok=True)
    else: panel.write_text(record['panel_before'])
ui.cleanup()
ui.key(*(['escape'] * 5))
print('PASS panel document restored to pre-test state; only original user rooms remain', flush=True)
