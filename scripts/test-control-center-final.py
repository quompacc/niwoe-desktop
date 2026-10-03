#!/usr/bin/env python3
"""Final-release legacy theme reload and keyboard icon persistence regression."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import time
import tomllib

from PIL import Image

spec = importlib.util.spec_from_file_location('ui', Path(__file__).with_name('test-control-center.py'))
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)
config = Path.home() / '.config/niwoe/config.toml'
original = config.read_bytes()
foreign = [Path.home() / '.config' / path for path in
           ('kdeglobals', 'gtk-3.0/settings.ini', 'gtk-4.0/settings.ini')]
foreign_before = {str(path): path.read_bytes() if path.exists() else None for path in foreign}


def capture(label):
    subprocess.run(['python3', str(ui.ROOT / 'scripts/test-hub-data.py'), 'capture', label], check=True)
    return Image.open(ui.ROOT / 'target/p08-live' / (label + '.png')).convert('RGB')


ui.prepare()
try:
    ui.manage()
    ui.click(1760, 276)
    ui.key('text:p10-final', 'backtab', 'enter')  # Name -> Icon, real keyboard action.
    ui.click(1810, 1051)
    room = ui.wait_room('p10-final')
    ui.remember(room)
    assert room['preferences']['icon'] == 'folder', room
    ui.persisted(room)
    ui.open_room('p10-final')
    before = capture('p10-final-general')
    text = original.decode()
    section = re.search(r'(?ms)^\[general\][^\[]*', text)
    assert section, 'explicit existing general section required'
    replacement, count = re.subn(r'(?m)^theme\s*=.*$', 'theme = "light"', section.group())
    if not count:
        replacement += '\ntheme = "light"\n'
    changed = text[:section.start()] + replacement + text[section.end():]
    config.write_text(changed)
    ui.p.send(type='reload-config')
    time.sleep(3)
    assert tomllib.loads(config.read_text())['general']['theme'] == 'light'
    after = capture('p10-legacy-light-reload')
    # Stable opaque form/background patches exclude clock/cursor/notifications.
    points = [(300, 260), (1200, 700), (800, 850)]
    samples = [(before.getpixel(point), after.getpixel(point)) for point in points]
    assert all(max(abs(a - b) for a, b in zip(left, right)) <= 3 for left, right in samples), samples
    assert all(max(right) < 100 for _, right in samples), samples
    ui.key('backtab', 'enter', 'escape')  # Icon preview edit then full draft rollback.
    assert ui.wait_room('p10-final') == room
    for path in foreign:
        assert (path.read_bytes() if path.exists() else None) == foreign_before[str(path)]
    (ui.DATA / 'final-regression.json').write_text(json.dumps(dict(
        theme_samples=samples, config_sha256=hashlib.sha256(original).hexdigest(),
        legacy_choice_preserved=True, foreign_preferences_unchanged=True,
        keyboard_icon_persisted=True, icon_cancel_unchanged=True), indent=2))
    print('PASS final release: keyboard icon persisted, cancel unchanged; legacy light reload stays dark and preserves independent preferences', flush=True)
finally:
    config.write_bytes(original)
    ui.p.send(type='reload-config')
    time.sleep(3)
    assert config.read_bytes() == original
    ui.cleanup()
    ui.key(*(['escape'] * 5))
