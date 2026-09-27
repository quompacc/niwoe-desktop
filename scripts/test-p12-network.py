#!/usr/bin/env python3
"""Real Ethernet/WLAN route transition with a timed Ethernet recovery.

Run after verifying SSH over both addresses. Never creates a network profile
or records Wi-Fi credentials. The user's already connected WLAN stays enabled.
"""
import getpass
import json
from pathlib import Path
import runpy
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
data = ROOT / 'target/p12-evidence/network'
data.mkdir(parents=True, exist_ok=False)
assert not p.windows()
password = getpass.getpass('Fedora sudo password (not logged): ')
ethernet, wifi = 'enp4s0f1', 'wlp3s0'
unit = 'niwoe-p12-network-recovery'
process = None
armed = False
records = []


def run(*command):
    return subprocess.check_output(command, text=True).strip()


def privileged(*command):
    result = subprocess.run(['sudo', '-S', '-p', '', *command],
        input=password + '\n', text=True, capture_output=True, timeout=60)
    assert result.returncode == 0, (command, result.returncode, result.stderr)
    return result.stdout


def wait(predicate):
    end = time.monotonic() + 90
    while time.monotonic() < end:
        if predicate(): return
        time.sleep(.2)
    raise AssertionError('network/client condition timeout')


def connected(device):
    return run('nmcli', '-g', 'GENERAL.STATE', 'device', 'show', device).startswith('100 ')


def route_device():
    words = run('ip', '-4', 'route', 'get', '1.1.1.1').split()
    return words[words.index('dev') + 1]


def input_check(label):
    identity = next(w['id'] for w in p.windows() if w['title'] == 'P12 wayland')
    p.send(type='focus-window', id=identity)
    with ui['lib']['VirtualKeyboard']() as keys:
        keys.combo(29, 30); keys.type_text(label)
    wait(lambda: json.loads((data / 'wayland.json').read_text())['content'] == label)
    records.append(dict(label=label, route=route_device(),
        ethernet=connected(ethernet), wifi=connected(wifi), snapshot=p.snapshot()))
    (data / 'results.json').write_text(json.dumps(records, indent=2))
    print('PASS', label, flush=True)


try:
    ui['key'](*(['escape'] * 5))
    with ui['lib']['VirtualKeyboard']() as keys: keys.combo(125, 2)
    time.sleep(.5)
    assert connected(ethernet) and connected(wifi)
    assert route_device() == ethernet
    assert run('systemctl', 'show', unit + '.timer', '-p', 'LoadState', '--value') == 'not-found'
    _, env = p.environment()
    process = subprocess.Popen(['python3', str(ROOT / 'scripts/p12-client.py'), 'wayland', str(data)],
        env=env, stdout=(data / 'client.log').open('w'), stderr=subprocess.STDOUT)
    wait(lambda: (data / 'wayland.json').exists() and bool(p.windows()))
    input_check('p12-ethernet-before')
    privileged('systemd-run', '--unit=' + unit, '--on-active=120s',
               '/usr/bin/nmcli', 'device', 'connect', ethernet)
    armed = True
    privileged('nmcli', 'device', 'disconnect', ethernet)
    wait(lambda: route_device() == wifi and not connected(ethernet))
    gateway = run('nmcli', '-g', 'IP4.GATEWAY', 'device', 'show', wifi)
    run('ping', '-I', wifi, '-c', '3', '-W', '3', gateway)
    input_check('p12-wifi-active')
    privileged('nmcli', 'device', 'connect', ethernet)
    wait(lambda: connected(ethernet) and route_device() == ethernet)
    input_check('p12-ethernet-restored')
finally:
    if armed:
        # Keep timed recovery armed if explicit reconnection fails.
        privileged('nmcli', 'device', 'connect', ethernet)
        wait(lambda: connected(ethernet))
        privileged('systemctl', 'stop', unit + '.timer')
    password = ''
    if process and process.poll() is None:
        process.terminate()
        process.wait(timeout=5)
