#!/usr/bin/env python3
"""Live P10 conflict/storage/missing-app checks on a recorded owned room only."""
import importlib.util
from pathlib import Path
import stat
import subprocess

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
p = ui.p


def capture(label):
    subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', label], check=True)


def current(room):
    return next(r for r in ui.rooms() if r['id'] == room)


def configure(room, **changes):
    value = current(room)
    fields = {k: value[k] for k in ('name', 'description', 'assignment', 'preferences')}
    fields.update(changes)
    p.mutate(operation='configure', id=room, position=next(i for i, r in enumerate(ui.rooms()) if r['id'] == room), **fields)


def main():
    room = next(r['id'] for r in ui.rooms() if r['name'] == 'p10-mouse')
    assert room in ui.owned()['rooms']
    original = current(room)
    ui.manage()
    ui.open_room('p10-mouse')
    ui.click(950, 339)
    ui.key('text:p10-local-draft')
    configure(room, description='p10-concurrent')
    ui.click(1810, 1051)
    assert current(room)['description'] == 'p10-concurrent'
    capture('p10-conflict')
    ui.click(1810, 1051)
    assert current(room)['description'] == 'p10-local-draft'
    ui.persisted(current(room))
    ui.open_room('p10-mouse')
    ui.click(950, 339)
    ui.key('text:p10-storage-retry')
    _, env = p.environment()
    directory = Path(env.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'niwoe'
    mode = stat.S_IMODE(directory.stat().st_mode)
    try:
        directory.chmod(mode & ~0o222)
        ui.click(1810, 1051)
        assert current(room)['description'] == 'p10-local-draft'
        capture('p10-storage-error')
    finally:
        directory.chmod(mode)
    ui.click(1810, 1051)
    assert current(room)['description'] == 'p10-storage-retry'
    ui.persisted(current(room))
    # A disappeared catalog reference is kept in preferences and must be removable.
    prefs = current(room)['preferences']
    missing = dict(kind='native', id='org.niwoe.p10missing')
    prefs['apps'].append(missing)
    configure(room, preferences=prefs)
    ui.open_room('p10-mouse')
    ui.click(402, 208)
    ui.click(500, 339)
    ui.key('text:p10missing')
    capture('p10-missing-app')
    ui.click(500, 439)
    ui.click(1810, 1051)
    assert missing not in current(room)['preferences']['apps']
    configure(room, description=original['description'])
    print('PASS visible conflict, retained draft/retry, storage failure/retry, missing-app removal', flush=True)


if __name__ == '__main__':
    main()
