#!/usr/bin/env python3
"""Switch two actually available PipeWire sinks and verify a live stream route.

Silence avoids unexpected sound. This proves backend routing, not acoustic
playback or a native output selector. Original default is restored in finally.
"""
import json
import os
from pathlib import Path
import runpy
import subprocess
import time
import wave

ROOT = Path(__file__).resolve().parents[1]
_, env = runpy.run_path(str(ROOT / 'scripts/test-layout-restore.py'))['environment']()
os.environ.update({key: env[key] for key in ('XDG_RUNTIME_DIR', 'DBUS_SESSION_BUS_ADDRESS')})
data = ROOT / 'target/p12-evidence/audio'
data.mkdir(parents=True, exist_ok=False)


def run(*args):
    return subprocess.check_output(args, text=True).strip()


original = run('pactl', 'get-default-sink')
sinks = json.loads(run('pactl', '-f', 'json', 'list', 'sinks'))
(data / 'before.json').write_text(json.dumps(sinks, indent=2))
other = next(sink['name'] for sink in sinks if sink['name'] != original)
sample = data / 'silence.wav'
with wave.open(str(sample), 'wb') as output:
    output.setnchannels(2); output.setsampwidth(2); output.setframerate(44100)
    output.writeframes(bytes(44100*4*4))
records = []
try:
    for name in (other, original):
        run('pactl', 'set-default-sink', name)
        assert run('pactl', 'get-default-sink') == name
        process = subprocess.Popen(['paplay', '--client-name=NIWOE-P12-Audio', str(sample)],
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        try:
            time.sleep(1)
            current = json.loads(run('pactl', '-f', 'json', 'list', 'sinks'))
            inputs = json.loads(run('pactl', '-f', 'json', 'list', 'sink-inputs'))
            target = next(sink for sink in current if sink['name'] == name)
            stream = next(stream for stream in inputs if stream['properties'].get('application.name') == 'NIWOE-P12-Audio')
            assert stream['sink'] == target['index'], (stream, target)
            assert target['state'] == 'RUNNING', target
            output, _ = process.communicate(timeout=15)
            assert process.returncode == 0, output
            records.append(dict(default=name, target=target, stream=stream))
            (data / 'results.json').write_text(json.dumps(records, indent=2))
            print('PASS backend default and live stream', name, flush=True)
        finally:
            if process.poll() is None:
                process.terminate(); process.wait(timeout=5)
finally:
    run('pactl', 'set-default-sink', original)
    assert run('pactl', 'get-default-sink') == original
    sample.unlink()
    print('PASS original audio default restored', flush=True)
