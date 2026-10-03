#!/usr/bin/env python3
"""Disposable native/X11 clients for P09, never an installed product component."""
import json
import os
from pathlib import Path
import sys
import time

DATA = Path(__file__).resolve().parents[1] / 'target/p09-live'
DATA.mkdir(exist_ok=True)
kind = sys.argv[1]
pid = os.getpid()
with (DATA / 'launches.jsonl').open('a') as log:
    log.write(json.dumps(dict(kind=kind, pid=pid, args=sys.argv[2:], time=time.time())) + '\n')
if kind == 'slow':
    time.sleep(20)
import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, GLib
GLib.set_prgname('org.niwoe.P09' + kind.title())
window = Gtk.Window(title='P09 ' + kind)
if kind == 'x11':
    window.set_wmclass('p09-x11', 'P09X11')
window.set_default_size(420, 260)
window.add(Gtk.Label(label='P09 isolated ' + kind + '\n' + '\n'.join(sys.argv[2:])))
window.connect('destroy', Gtk.main_quit)
window.show_all()


def tick():
    command = DATA / f'command-{pid}'
    if command.exists():
        value = command.read_text()
        command.unlink()
        if value == 'close':
            Gtk.main_quit()
            return False
        if value == 'minimize':
            window.iconify()
        if value.startswith('title:'):
            window.set_title(value[6:])
    width, height = window.get_size()
    (DATA / f'client-{pid}.json').write_text(json.dumps(dict(kind=kind, width=width, height=height)))
    return True


GLib.timeout_add(200, tick)
Gtk.main()
