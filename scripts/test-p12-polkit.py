#!/usr/bin/env python3
"""Real system Polkit authorization for the compositor's logind session.

Checks wrong-password/cancel then success. Never executes a privileged payload.
Run again after a separately logged polkit.service restart to prove recovery.
"""
import getpass
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
label = sys.argv[1] if len(sys.argv) > 1 else 'initial'
assert label in ('initial', 'restarted')
data = ROOT / ('target/p12-evidence/polkit-' + label)
data.mkdir(parents=True, exist_ok=False)
password = getpass.getpass('Fedora Polkit password (not logged): ')
assert all(char in ui['lib']['KEYS'] for char in password)
pid, = subprocess.check_output(['pgrep', '-x', 'niwoe'], text=True).split()
start = Path('/proc', pid, 'stat').read_text().split()[21]
subject = f'{pid},{start},{os.getuid()}'
records = []
for mode in ('wrong', 'cancel', 'allow'):
    process = subprocess.Popen(['pkcheck', '--action-id', 'org.freedesktop.policykit.exec',
        '--process', subject, '--allow-user-interaction'],
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    try:
        time.sleep(2)
        assert process.poll() is None, 'expected actual authentication UI, no cached authorization'
        if mode == 'wrong':
            ui['key']('text:p12-wrong-password', 'enter')
            time.sleep(4)
            assert process.poll() is None, 'wrong password should remain denied with retry UI'
            ui['key']('escape')
        elif mode == 'cancel':
            ui['key']('escape')
        else:
            with ui['lib']['VirtualKeyboard']() as keyboard:
                keyboard.type_text(password)
                keyboard.tap(28)
        output, _ = process.communicate(timeout=15)
        (data / (mode + '.log')).write_text(output)
        assert process.returncode == (0 if mode == 'allow' else 1), (mode, process.returncode, output)
        records.append(dict(mode=mode, exit_code=process.returncode, subject=subject))
        (data / 'results.json').write_text(json.dumps(records, indent=2))
        print('PASS real Polkit', mode, process.returncode, flush=True)
    finally:
        if process.poll() is None:
            process.terminate()
            try: process.wait(timeout=5)
            except subprocess.TimeoutExpired: process.kill(); process.wait()
        ui['key']('escape')
password = ''
