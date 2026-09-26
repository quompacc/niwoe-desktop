#!/usr/bin/env python3
"""P07 protocol integration, only inside smoke-nested.sh's private profile."""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time


def x11_client(directory, title, app_class):
    startup_id = os.environ.get('DESKTOP_STARTUP_ID')
    import ctypes as c
    import gi
    gi.require_version('Gtk', '3.0')
    gi.require_version('GdkX11', '3.0')
    from gi.repository import Gtk, GLib, GdkX11
    window = Gtk.Window(title=title)
    if startup_id:
        window.set_startup_id(startup_id)
    window.set_wmclass('probe', app_class)
    window.set_default_size(320, 180)
    window.add(Gtk.Label(label='P07 X11 assignment'))
    (directory / 'pid').write_text(str(os.getpid()))
    window.show_all()
    xlib = c.CDLL('libX11.so.6')
    xlib.XOpenDisplay.restype = c.c_void_p
    display = xlib.XOpenDisplay(None)
    class Hint(c.Structure):
        _fields_ = [('name', c.c_char_p), ('class_', c.c_char_p)]
    xlib.XSetClassHint.argtypes = [c.c_void_p, c.c_ulong, c.POINTER(Hint)]
    xlib.XFlush.argtypes = [c.c_void_p]
    xid = GdkX11.X11Window.get_xid(window.get_window())
    previous = ''
    dialogs = []
    def tick():
        nonlocal previous
        command = (directory / 'command').read_text() if (directory / 'command').exists() else ''
        if command != previous:
            if command == 'quit': Gtk.main_quit()
            if command.startswith('class '):
                hint = Hint(b'probe', command[6:].encode())
                xlib.XSetClassHint(display, xid, c.byref(hint))
                xlib.XFlush(display)
            if command == 'dialog':
                dialog = Gtk.Dialog(title=title + ' Dialog', transient_for=window)
                dialog.show_all()
                dialogs.append(dialog)
            previous = command
        (directory / 'ready').touch()
        return True
    GLib.timeout_add(50, tick)
    Gtk.main()


if sys.argv[1] == '--x11':
    x11_client(Path(sys.argv[2]), sys.argv[3], sys.argv[4])
    raise SystemExit

shell, = subprocess.check_output(['pgrep', '-P', sys.argv[1], '-x', 'niwoe-shell'], text=True).split()
env = dict(part.split(b'=', 1) for part in Path(f'/proc/{shell}/environ').read_bytes().split(b'\0') if b'=' in part)
runtime = Path(env[b'XDG_RUNTIME_DIR'].decode())
assert str(runtime).startswith('/tmp/niwoe-p01.'), 'isolated profile required'
token = env.pop(b'NIWOE_IPC_TOKEN').decode()
client_env = os.environ | {k.decode(): v.decode() for k, v in env.items()}
client_env.pop('NIWOE_IPC_TOKEN', None)
client_env['WAYLAND_DISPLAY'] = str(runtime / env[b'WAYLAND_DISPLAY'].decode())
# The watchdog starts before XWayland is ready. Its inherited DISPLAY can
# still be :0; select the actual child server instead of touching the parent.
xwayland, = subprocess.check_output(['pgrep', '-P', sys.argv[1], '-x', 'Xwayland'], text=True).split()
xargs = Path(f'/proc/{xwayland}/cmdline').read_bytes().split(b'\0')
client_env['DISPLAY'] = next(arg.decode() for arg in xargs if arg.startswith(b':'))
directory = runtime.parent / 'launch-evidence'
directory.mkdir()
processes = []
marker = Path('target/p07-assignment-manual.json')
marker.unlink(missing_ok=True)


def connection():
    sock = socket.socket(socket.AF_UNIX)
    sock.settimeout(5)
    sock.connect(str(runtime / 'niwoe.sock'))
    sock.sendall((json.dumps(dict(type='authenticate', role='shell', token=token)) + '\n').encode())
    return sock


def snapshot():
    with connection() as sock, sock.makefile() as reader:
        data = {}
        while not all(k in data for k in ('window-snapshot', 'room-snapshot')):
            event = json.loads(reader.readline())
            data[event['type']] = event
        return data


def send(**command):
    with connection() as sock:
        sock.sendall((json.dumps(command) + '\n').encode())
        time.sleep(.1)


def mutate(**change):
    revision = snapshot()['room-snapshot']['snapshot']['revision']
    request = str(time.time_ns())
    with connection() as sock, sock.makefile() as reader:
        sock.sendall((json.dumps(dict(type='mutate-room', request_id=request, expected_revision=revision, change=change)) + '\n').encode())
        for line in reader:
            event = json.loads(line)
            if event['type'] == 'room-mutation-result' and event['request_id'] == request:
                assert event['error'] is None, event
                return


def windows():
    return {w['title']: w for w in snapshot()['window-snapshot']['windows']}


def await_room(name, room):
    deadline = time.monotonic() + 5
    while time.monotonic() < deadline:
        value = windows()
        if name in value and value[name]['workspace'] == room:
            return value[name]
        time.sleep(.05)
    raise AssertionError(value)


owned=[]
def launch(name, room, mode='activate', backend='native'):
    target=directory/name.replace(' ', '-')
    target.mkdir(); owned.append(target)
    if backend=='native':
        args=[str(Path('target/release/examples/room_assignment_probe').resolve()),str(target),name,'org.niwoe.LaunchTest',mode]
    else:
        args=['/usr/bin/env','GDK_BACKEND=x11','DISPLAY='+client_env['DISPLAY'],sys.executable,str(Path(__file__).resolve()),'--x11',str(target),name,'P07LaunchClass']
    send(type='launch-app',program=args[0],args=args[1:],room_id=room)
    return target

rooms=snapshot()['room-snapshot']['snapshot']['rooms']
rule=rooms[2]
try:
    mutate(operation='set-preferences',id=rule['id'],preferences=rule['preferences'] | {'apps':[dict(kind='native',id='org.niwoe.LaunchTest'),dict(kind='xwayland',id='P07LaunchClass')]})
    mutate(operation='set-assignment',id=rule['id'],assignment='preferred')
    send(type='switch-workspace',workspace=1)
    a=launch('P07 Launch A',rooms[1]['id'])
    b=launch('P07 Launch B',rooms[3]['id'])
    aid=await_room('P07 Launch A',2)['id']
    bid=await_room('P07 Launch B',4)['id']
    assert aid!=bid
    assert snapshot()['window-snapshot']['active_workspace']==1
    print('PASS two equal native app IDs correlated to separate explicit rooms, no room switch',flush=True)
    normal=launch('P07 Launch Normal',None,mode='plain')
    await_room('P07 Launch Normal',3)
    deferred=launch('P07 Launch Deferred',rooms[1]['id'],mode='plain')
    deferred_id=await_room('P07 Launch Deferred',3)['id']
    send(type='switch-workspace',workspace=3)
    send(type='focus-window',id=deferred_id)
    (deferred/'command').write_text('minimize')
    time.sleep(.2)
    assert windows()['P07 Launch Deferred']['minimized']
    send(type='switch-workspace',workspace=1)
    (deferred/'command').write_text('activate')
    assert await_room('P07 Launch Deferred',2)['minimized']
    send(type='switch-workspace',workspace=2)
    send(type='focus-window',id=deferred_id)
    assert not windows()['P07 Launch Deferred']['minimized']
    send(type='switch-workspace',workspace=1)
    (a/'command').write_text('replay org.niwoe.LaunchTest')
    await_room('P07 Launch A Replay',3)
    print('PASS late minimized activation/restore and single-use token on unrelated surface',flush=True)
    late=launch('P07 Launch Late',rooms[1]['id'],mode='plain')
    late_id=await_room('P07 Launch Late',3)['id']
    send(type='move-window-to-room',id=late_id,room_id=rooms[4]['id'])
    (late/'command').write_text('activate')
    time.sleep(.2); await_room('P07 Launch Late',5)
    (a/'command').write_text('dialog org.niwoe.LaunchTest')
    await_room('P07 Launch A Dialog',2)
    send(type='move-window-to-room',id=aid,room_id=rooms[5]['id'])
    (a/'command').write_text('activate')
    time.sleep(.2); await_room('P07 Launch A',6)
    print('PASS normal Preferred, late activation after manual move, replay and parent priority',flush=True)
    x=launch('P07 Launch X11',rooms[1]['id'],backend='x11')
    await_room('P07 Launch X11',2)
    print('PASS X11 startup ID explicit destination',flush=True)
    invalid=launch('P07 Invalid',999999)
    time.sleep(.3)
    assert not (invalid/'pid').exists()
    assert 'P07 Invalid' not in windows()
    (directory/'result.json').write_text(json.dumps(snapshot(),indent=2))
    print('PASS missing destination does not launch',flush=True)
finally:
    for target in owned:
        if (target/'pid').exists():
            (target/'command').write_text('quit')
    time.sleep(.3)
    mutate(operation='set-preferences',id=rule['id'],preferences=rule['preferences'])
    mutate(operation='set-assignment',id=rule['id'],assignment=rule['assignment'])
