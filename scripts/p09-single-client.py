#!/usr/bin/env python3
"""Real D-Bus single-instance forwarding fixture; keeps running without a window."""
import json
import os
from pathlib import Path
import sys
import time
import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, Gio, GLib

DATA = Path(__file__).resolve().parents[1] / 'target/p09-live'
with (DATA / 'launches.jsonl').open('a') as log:
    log.write(json.dumps(dict(kind='single', pid=os.getpid(), args=[], time=time.time())) + '\n')
app = Gtk.Application(application_id='org.niwoe.P09Single', flags=Gio.ApplicationFlags.FLAGS_NONE)
state = dict(activated=False, window=None)


def activate(application):
    if state['activated']:
        with (DATA / 'forwarded.log').open('a') as file:
            file.write('forwarded\n')
        return
    state['activated'] = True
    application.hold()
    window = Gtk.ApplicationWindow(application=application, title='P09 single')
    window.set_default_size(420, 260)
    window.add(Gtk.Label(label='P09 real single-instance forwarding'))
    window.show_all()
    state['window'] = window


def tick():
    path = DATA / 'single-command'
    if path.exists():
        command = path.read_text()
        path.unlink()
        if command == 'hide' and state['window'] is not None:
            state['window'].destroy()
            state['window'] = None
        if command == 'close':
            app.quit()
            return False
    return True


app.connect('activate', activate)
GLib.timeout_add(100, tick)
sys.exit(app.run(sys.argv))
