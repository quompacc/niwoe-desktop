#!/usr/bin/env python3
"""Real analog/HDMI profile roundtrip, silence stream and continued input."""
import json
import os
from pathlib import Path
import runpy
import subprocess
import time
import wave

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
p = ui['p']
_, env = p.environment()
os.environ.update({k: env[k] for k in ('XDG_RUNTIME_DIR', 'DBUS_SESSION_BUS_ADDRESS')})
os.environ['LC_ALL'] = 'C.UTF-8'
data = ROOT / 'target/p12-evidence/audio-hdmi'
data.mkdir(parents=True, exist_ok=False)
assert not p.windows()


def run(*args):
    return subprocess.check_output(args, text=True).strip()


def listing(kind):
    return json.loads(run('pactl', '-f', 'json', 'list', kind))


def wait(predicate):
    end = time.monotonic() + 20
    while time.monotonic() < end:
        if predicate(): return
        time.sleep(.2)
    raise AssertionError('audio/client timeout')


original = run('pactl', 'get-default-sink')
before = next(s for s in listing('sinks') if s['name'] == original)
card = next(c for c in listing('cards') if c['name'] == before['properties']['device.name'])
profile = card['active_profile']
hdmi = 'output:hdmi-stereo+input:analog-stereo'
assert card['profiles'][hdmi]['available'] is True
assert 'analog-stereo' in profile
(data / 'before.json').write_text(json.dumps(dict(sink=before, card=card), indent=2))
sample = data / 'silence.wav'
with wave.open(str(sample), 'wb') as output:
    output.setnchannels(2); output.setsampwidth(2); output.setframerate(44100)
    output.writeframes(bytes(44100 * 4 * 4))
client = None
records = []
try:
    client = subprocess.Popen(['python3', str(ROOT / 'scripts/p12-client.py'), 'wayland', str(data)],
        env=env, stdout=(data / 'client.log').open('w'), stderr=subprocess.STDOUT)
    wait(lambda: bool(p.windows()) and (data / 'wayland.json').exists())
    for target_profile in (hdmi, profile):
        run('pactl', 'set-card-profile', card['name'], target_profile)
        suffix = 'hdmi-stereo' if target_profile == hdmi else 'analog-stereo'
        wait(lambda: any(s['properties'].get('device.name') == card['name'] and s['name'].endswith(suffix) for s in listing('sinks')))
        target = next(s for s in listing('sinks') if s['properties'].get('device.name') == card['name'] and s['name'].endswith(suffix))
        run('pactl', 'set-default-sink', target['name'])
        assert run('pactl', 'get-default-sink') == target['name']
        stream = subprocess.Popen(['paplay', '--client-name=NIWOE-P12-Audio', str(sample)],
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        try:
            time.sleep(1)
            active = next(s for s in listing('sinks') if s['name'] == target['name'])
            routed = next(s for s in listing('sink-inputs') if s['properties'].get('application.name') == 'NIWOE-P12-Audio')
            assert active['state'] == 'RUNNING' and routed['sink'] == active['index']
            result, _ = stream.communicate(timeout=15)
            assert stream.returncode == 0, result
        finally:
            if stream.poll() is None: stream.terminate(); stream.wait(timeout=5)
        identity = next(w['id'] for w in p.windows() if w['title'] == 'P12 wayland')
        p.send(type='focus-window', id=identity)
        with ui['lib']['VirtualKeyboard']() as keys:
            keys.combo(29, 30); keys.type_text('p12-audio-' + suffix)
        wait(lambda: json.loads((data / 'wayland.json').read_text())['content'] == 'p12-audio-' + suffix)
        records.append(dict(profile=target_profile, sink=active['name'], state=active['state'], stream_sink=routed['sink'], input=True))
        (data / 'results.json').write_text(json.dumps(records, indent=2))
        print('PASS real audio profile, running stream and input', target_profile, flush=True)
finally:
    run('pactl', 'set-card-profile', card['name'], profile)
    wait(lambda: any(s['name'] == original for s in listing('sinks')))
    run('pactl', 'set-default-sink', original)
    run('pactl', 'set-sink-volume', original, *(str(v['value']) for v in before['volume'].values()))
    run('pactl', 'set-sink-mute', original, str(int(before['mute'])))
    restored = next(s for s in listing('sinks') if s['name'] == original)
    assert restored['volume'] == before['volume'] and restored['mute'] == before['mute']
    assert run('pactl', 'get-default-sink') == original
    sample.unlink(missing_ok=True)
    if client and client.poll() is None: client.terminate(); client.wait(timeout=5)
    print('PASS original card profile, default, volume and mute restored', flush=True)
