#!/usr/bin/env python3
"""After a real display-manager relogin: neutral Hub, watchdog and draft resume."""
import json
from pathlib import Path
import runpy
import signal
import os
import subprocess
import time
import sys

u = runpy.run_path(str(Path(__file__).with_name('test-first-run-ui.py')))
ui = u['ui']
before = u['state']()
if len(sys.argv) > 1 and sys.argv[1] == 'prepare':
    (u['DATA'] / 'pre-relogin-state.json').write_text(json.dumps(before))
    print('Saved isolated draft before real relogin', flush=True)
    raise SystemExit(0)
expected = json.loads((u['DATA'] / 'pre-relogin-state.json').read_text())
assert before == expected
snapshot = ui.p.snapshot()
assert snapshot['window-snapshot']['active_workspace'] == 0 and not ui.p.windows()
u['capture']('p11-relogin-hub')
ui.click(1560, 172)  # Visible close, then no further workflow input before crash capture.
pid = int(subprocess.check_output(['pgrep', '-x', 'niwoe-shell']))
_, env = ui.p.environment()
marker = Path(env['XDG_RUNTIME_DIR']) / 'niwoe' / ('first-login-hub-shown-' + env['XDG_SESSION_ID'])
stamp = marker.stat().st_mtime_ns
os.kill(pid, signal.SIGKILL)
deadline = time.monotonic() + 20
new = pid
while time.monotonic() < deadline:
    values = subprocess.run(['pgrep', '-x', 'niwoe-shell'], capture_output=True, text=True).stdout.split()
    if len(values) == 1 and int(values[0]) != pid:
        new = int(values[0]); break
    time.sleep(.2)
assert new != pid
time.sleep(5)
assert marker.stat().st_mtime_ns == stamp and u['state']() == before
assert ui.p.snapshot()['window-snapshot']['active_workspace'] == 0 and not ui.p.windows()
try:
    u['capture']('p11-watchdog-no-welcome')
except subprocess.CalledProcessError:
    # The existing screenshot bridge can time out during shell replacement.
    # Record the failed first attempt; one bounded retry is optical evidence only.
    print('Screenshot bridge timeout after watchdog; retry once, no workflow input', flush=True)
    time.sleep(2)
    u['capture']('p11-watchdog-no-welcome')
u['keyboard_open']()
assert u['state']() == before
u['capture']('p11-watchdog-resume')
record = dict(before=pid, after=new, snapshot=snapshot, marker=marker.name, unchanged_marker=True, state=before)
(u['DATA'] / 'session.json').write_text(json.dumps(record, indent=2))
print('PASS real relogin neutral Hub, SIGKILL/watchdog, IPC resubscription, unchanged once marker and explicit draft resume', flush=True)
