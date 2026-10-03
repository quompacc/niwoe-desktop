#!/usr/bin/env python3
"""Fresh-login Loge, once-only Hub, watchdog and direct real-app launch.

Run immediately after a real login. Screenshots use public portal consent;
their visible content must be inspected separately from protocol assertions.
"""
import json
import os
from pathlib import Path
import runpy
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
data = ROOT / 'target/p12-evidence/session'
data.mkdir(parents=True, exist_ok=False)
initial = p.snapshot()
assert initial['window-snapshot']['active_workspace'] == 0 and not p.windows()
_, env = p.environment()
marker = Path(env['XDG_RUNTIME_DIR']) / 'niwoe' / ('first-login-hub-shown-' + env['XDG_SESSION_ID'])
stamp = marker.stat().st_mtime_ns
(data / 'login.json').write_text(json.dumps(initial, indent=2))
subprocess.run(['python3', str(ROOT / 'scripts/test-p12-portal.py'), 'session_welcome'], check=True)
ui['key'](*(['escape'] * 5))
old = int(subprocess.check_output(['pgrep', '-x', 'niwoe-shell']))
os.kill(old, signal.SIGKILL)
end = time.monotonic() + 20
while time.monotonic() < end:
    value = subprocess.run(['pgrep', '-x', 'niwoe-shell'], capture_output=True, text=True).stdout.strip()
    if value and value != str(old): break
    time.sleep(.2)
else: raise AssertionError('watchdog timeout')
time.sleep(3)
assert marker.stat().st_mtime_ns == stamp
assert p.snapshot()['window-snapshot']['active_workspace'] == 0 and not p.windows()
subprocess.run(['python3', str(ROOT / 'scripts/test-p12-portal.py'), 'session_watchdog'], check=True)
assert marker.stat().st_mtime_ns == stamp
print('PASS fresh neutral Loge and unchanged once marker after watchdog', flush=True)
file = data / 'p12-loge.txt'
file.write_text('p12-loge-direct-launch\n')
config = data / 'config'
config.mkdir()
p.send(type='launch-app', program='env', args=['QT_QPA_PLATFORM=wayland',
    'XDG_CONFIG_HOME=' + str(config), 'kwrite', str(file)], room_id=None)
end = time.monotonic() + 25
while time.monotonic() < end:
    windows = [w for w in p.windows() if file.name in w['title']]
    if windows: break
    time.sleep(.2)
else: raise AssertionError('direct KWrite launch timeout')
assert len(windows) == 1 and windows[0]['workspace'] == 1
assert p.snapshot()['window-snapshot']['active_workspace'] == 1
assert p.snapshot()['room-snapshot'] == initial['room-snapshot']
(data / 'direct-launch.json').write_text(json.dumps(p.snapshot(), indent=2))
p.send(type='focus-window', id=windows[0]['id'])
with ui['lib']['VirtualKeyboard']() as keys: keys.combo(29, 16)
end = time.monotonic() + 10
while p.windows() and time.monotonic() < end: time.sleep(.2)
assert not p.windows() and file.read_text() == 'p12-loge-direct-launch\n'
print('PASS direct KWrite launch from Loge activates room 1; no extra room; normal close', flush=True)
