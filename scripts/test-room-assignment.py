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
    import ctypes as c
    import gi
    gi.require_version('Gtk', '3.0')
    gi.require_version('GdkX11', '3.0')
    from gi.repository import Gtk, GLib, GdkX11
    window = Gtk.Window(title=title)
    window.set_wmclass('probe', app_class)
    window.set_default_size(320, 180)
    window.add(Gtk.Label(label='P07 X11 assignment'))
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
directory = runtime.parent / 'assignment-evidence'
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


def launch(name, identity):
    target = directory / name.replace(' ', '-')
    target.mkdir()
    log = (target / 'client.log').open('w')
    process = subprocess.Popen(['target/release/examples/room_assignment_probe', str(target), name, identity], env=client_env, stdout=log, stderr=log)
    processes.append((process, log))
    deadline = time.monotonic() + 5
    while not (target / 'state.json').exists():
        assert process.poll() is None, (target / 'client.log').read_text()
        assert time.monotonic() < deadline
        time.sleep(.05)
    return target


def launch_x11(name, identity):
    target = directory / name.replace(' ', '-')
    target.mkdir()
    log = (target / 'client.log').open('w')
    process = subprocess.Popen([sys.executable, __file__, '--x11', str(target), name, identity], env=client_env | {'GDK_BACKEND': 'x11'}, stdout=log, stderr=log)
    processes.append((process, log))
    deadline = time.monotonic() + 5
    while not (target / 'ready').exists():
        assert process.poll() is None, (target / 'client.log').read_text()
        assert time.monotonic() < deadline
        time.sleep(.05)
    return target


if len(sys.argv) > 2 and sys.argv[2] == '--focus-setup':
    send(type='toggle-launcher')
    send(type='switch-workspace', workspace=1)
    time.sleep(.3)
    raise SystemExit

try:
    # Fresh isolated login opens the welcome Hub. Close it before testing
    # application focus; its exclusive layer would correctly reclaim focus.
    send(type='toggle-launcher')
    time.sleep(.3)
    # Two identical rules: stable saved order wins, even for Dedicated.
    for room, mode in ((2, 'preferred'), (3, 'dedicated')):
        mutate(operation='set-assignment', id=room, assignment=mode)
        mutate(operation='set-preferences', id=room, preferences=dict(apps=[dict(kind='native', id='org.niwoe.Assignment')]))
    send(type='switch-workspace', workspace=1)
    foreground = launch('P07 Foreground', 'org.niwoe.Unmatched')
    await_room('P07 Foreground', 1)
    assert json.loads((foreground / 'state.json').read_text())['focused']
    preferred = launch('P07 Preferred', 'org.niwoe.Assignment')
    await_room('P07 Preferred', 2)
    time.sleep(.2)
    assert json.loads((preferred / 'state.json').read_text())['enters'] == 0, 'background focus stolen'
    assert json.loads((foreground / 'state.json').read_text())['focused'], 'foreground focus lost'
    assert snapshot()['window-snapshot']['active_workspace'] == 1
    print('PASS preferred initial placement without even temporary keyboard focus theft', flush=True)
    (preferred / 'command').write_text('dialog org.niwoe.Unmatched')
    await_room('P07 Preferred Dialog', 2)
    assert snapshot()['window-snapshot']['active_workspace'] == 1
    assert json.loads((preferred / 'state.json').read_text())['enters'] == 0
    print('PASS transient inherits background parent instead of fallback', flush=True)
    late = launch('P07 Late', '-')
    await_room('P07 Late', 1)
    (late / 'command').write_text('app org.niwoe.Assignment')
    await_room('P07 Late', 2)
    print('PASS absent app-id fallback and late native app-id assignment', flush=True)
    if os.environ.get('NIWOE_ASSIGNMENT_MANUAL') == '1':
        manual = launch('P07 Manual', '-')
        await_room('P07 Manual', 1)
        marker.write_text(json.dumps(dict(compositor=int(sys.argv[1]), profile=str(runtime.parent))))
        print('READY manual: z invokes MoveToWorkspace(4) in private test keymap', flush=True)
        deadline = time.monotonic() + 25
        while windows()['P07 Manual']['workspace'] != 4:
            assert time.monotonic() < deadline, 'manual input not received'
            time.sleep(.1)
        (manual / 'command').write_text('app org.niwoe.Assignment')
        time.sleep(.3)
        assert windows()['P07 Manual']['workspace'] == 4, 'late metadata undid manual move'
        marker.unlink()
        print('PASS real keyboard manual move survives late native app-id', flush=True)
    mutate(operation='move', id=3, position=0)
    dedicated = launch('P07 Dedicated', 'org.niwoe.Assignment')
    await_room('P07 Dedicated', 3)
    assert json.loads((dedicated / 'state.json').read_text())['enters'] == 0
    send(type='switch-workspace', workspace=3)
    launch('P07 Dedicated Guest', 'org.niwoe.Unmatched')
    await_room('P07 Dedicated Guest', 3)
    print('PASS saved priority order and Dedicated accepts unrelated applications', flush=True)
    mutate(operation='set-preferences', id=2, preferences=dict(apps=[dict(kind='native', id='org.niwoe.Assignment'), dict(kind='xwayland', id='P07Class')]))
    send(type='switch-workspace', workspace=1)
    x11 = launch_x11('P07 X11', 'P07Class')
    await_room('P07 X11', 2)
    (x11 / 'command').write_text('dialog')
    await_room('P07 X11 Dialog', 2)
    assert snapshot()['window-snapshot']['active_workspace'] == 1
    launch_x11('P07 Distinct', 'org.niwoe.Assignment')
    await_room('P07 Distinct', 1)
    xlate = launch_x11('P07 X11 Late', '')
    await_room('P07 X11 Late', 1)
    (xlate / 'command').write_text('class P07Class')
    await_room('P07 X11 Late', 2)
    print('PASS XWayland class, transient, late class, native/class namespace separation', flush=True)
    (directory / 'result.json').write_text(json.dumps(snapshot(), indent=2))
finally:
    marker.unlink(missing_ok=True)
    for process, log in processes:
        process.terminate()
        process.wait(timeout=5)
        log.close()
