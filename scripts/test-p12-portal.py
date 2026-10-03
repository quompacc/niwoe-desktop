#!/usr/bin/env python3
"""Public desktop portal + real native consent, never a forged IPC answer."""
import json
import os
from pathlib import Path
import runpy
import sys
import time
from urllib.parse import unquote, urlparse

from gi.repository import Gio, GLib

ROOT = Path(__file__).resolve().parents[1]
ui = runpy.run_path(str(ROOT / 'scripts/test-control-center.py'))
_, env = ui['p'].environment()
os.environ['DBUS_SESSION_BUS_ADDRESS'] = env['DBUS_SESSION_BUS_ADDRESS']
bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
label = sys.argv[1] if len(sys.argv) > 1 else 'portal'
assert label.isidentifier()
data = ROOT / 'target/p12-evidence' / label
data.mkdir(parents=True, exist_ok=True)
records = []

for decision in ('allow', 'deny'):
    response = []
    token = 'p12_' + str(time.time_ns())
    expected = '/org/freedesktop/portal/desktop/request/' + bus.get_unique_name()[1:].replace('.', '_') + '/' + token

    def received(_bus, _sender, path, _iface, _signal, parameters):
        response.append((path, parameters.unpack()))

    subscription = bus.signal_subscribe('org.freedesktop.portal.Desktop',
        'org.freedesktop.portal.Request', 'Response', expected, None,
        Gio.DBusSignalFlags.NONE, received)
    connection = ui['p'].Connection()
    connection.until('room-snapshot')
    try:
        result = bus.call_sync('org.freedesktop.portal.Desktop',
            '/org/freedesktop/portal/desktop', 'org.freedesktop.portal.Screenshot',
            'Screenshot', GLib.Variant('(sa{sv})', ('', {
                'handle_token': GLib.Variant('s', token),
                'interactive': GLib.Variant('b', False)})),
            None, Gio.DBusCallFlags.NONE, 5000, None).unpack()
        assert result[0] == expected, result
        consent = connection.until('screenshot-consent-request')
        time.sleep(.5)
        ui['key']('enter' if decision == 'allow' else 'escape')
        deadline = time.monotonic() + 20
        context = GLib.MainContext.default()
        while not response and time.monotonic() < deadline:
            while context.pending(): context.iteration(False)
            time.sleep(.05)
        assert response, 'no public portal Response'
        code, values = response[0][1]
        assert code == (0 if decision == 'allow' else 1), response
        if decision == 'allow':
            parsed = urlparse(values['uri'])
            assert parsed.scheme == 'file' and parsed.netloc in ('', 'localhost')
            source = Path(unquote(parsed.path))
            image = source.read_bytes()
            assert image.startswith(b'\x89PNG\r\n\x1a\n')
            (data / 'allowed.png').write_bytes(image)
            # Only the exact artifact returned by this test request is removed.
            source.unlink()
        else:
            assert 'uri' not in values
        records.append(dict(decision=decision, response_code=code,
                            request_path=expected, consent=consent))
        (data / 'results.json').write_text(json.dumps(records, indent=2))
        print('PASS public portal Screenshot', decision, code, flush=True)
    finally:
        connection.close()
        bus.signal_unsubscribe(subscription)
