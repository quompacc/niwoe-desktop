#!/usr/bin/env python3
"""Instrumented real GTK3 Wayland/X11 client; test-only, never installed.

F5/nonmodal, F6/modal, F9/minimize, F10/maximize, F11/fullscreen,
Ctrl+S/save. Input, client geometry and focus are independently observable.
"""
import json
import os
from pathlib import Path
import sys

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, Gdk, GLib

kind, directory = sys.argv[1], Path(sys.argv[2]).resolve()
assert directory.is_dir()
GLib.set_prgname('org.niwoe.P12' + kind)
window = Gtk.Window(title='P12 ' + kind)
window.set_default_size(620, 400)
if kind == 'wayland':
    header = Gtk.HeaderBar(title='P12 wayland', subtitle='Wayland CSD test')
    header.set_show_close_button(True)
    window.set_titlebar(header)
else:
    window.set_wmclass('niwoe-p12', 'NiwoeP12X11')
assert ('Wayland' if kind == 'wayland' else 'X11') in type(Gdk.Display.get_default()).__name__
text = Gtk.TextView()
scroll = Gtk.ScrolledWindow()
scroll.add(text)
window.add(scroll)
events = []
dialogs = []


def contents():
    buffer = text.get_buffer()
    return buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter(), True)


def key(_widget, event):
    name = Gdk.keyval_name(event.keyval)
    events.append(name)
    if name == 's' and event.state & Gdk.ModifierType.CONTROL_MASK:
        (directory / (kind + '.txt')).write_text(contents())
        return True
    if name in ('F5', 'F6'):
        dialog = Gtk.Dialog(title='P12 ' + kind + ' ' + name,
                            transient_for=window, modal=name == 'F6')
        dialog.add_button('Close', Gtk.ResponseType.CLOSE)
        dialog.get_content_area().add(Gtk.Entry())
        dialog.connect('response', lambda d, _response: d.destroy())
        dialog.show_all()
        dialogs.append(dialog)
        return True
    state = window.get_window().get_state()
    if name == 'F9': window.iconify(); return True
    if name == 'F10':
        (window.unmaximize if state & Gdk.WindowState.MAXIMIZED else window.maximize)()
        return True
    if name == 'F11':
        (window.unfullscreen if state & Gdk.WindowState.FULLSCREEN else window.fullscreen)()
        return True
    return False


window.connect('key-press-event', key)
window.connect('destroy', Gtk.main_quit)
window.show_all()
text.grab_focus()


def record():
    gdk = window.get_window()
    value = dict(pid=os.getpid(), backend=type(Gdk.Display.get_default()).__name__,
                 content=contents(), active=window.is_active(),
                 focused=bool(gdk.get_state() & Gdk.WindowState.FOCUSED),
                 size=list(window.get_size()), position=list(window.get_position()),
                 state=int(gdk.get_state()), events=events[-200:],
                 scroll=scroll.get_vadjustment().get_value(),
                 dialogs=[dict(title=d.get_title(), active=d.is_active(),
                               focused=bool(d.get_window().get_state() & Gdk.WindowState.FOCUSED), modal=d.get_modal())
                          for d in dialogs if d.get_visible()])
    temporary = directory / (kind + '.json.tmp')
    temporary.write_text(json.dumps(value))
    temporary.replace(directory / (kind + '.json'))
    return True


GLib.timeout_add(100, record)
Gtk.main()
