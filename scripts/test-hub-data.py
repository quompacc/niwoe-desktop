#!/usr/bin/env python3
"""P08 live Fedora checks. Run as the session user; input needs /dev/uinput access.
Only the explicitly created P08 windows/room IDs are removed by cleanup.
"""
import json
import os
from pathlib import Path
import runpy
import shutil
import socket
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'target/p08-live'
DATA.mkdir(exist_ok=True)

def client(kind):
    import gi
    gi.require_version('Gtk', '3.0')
    from gi.repository import Gtk, GLib
    GLib.set_prgname('niwoe-p08-' + kind)
    window = Gtk.Window(title='P08 ' + kind)
    window.set_default_size(420, 260)
    area = Gtk.DrawingArea()
    def draw(_widget, cr):
        cr.set_source_rgb(*( (0.8, 0.12, 0.08) if kind == 'native' else (0.08, 0.2, 0.8)))
        cr.paint()
    area.connect('draw', draw)
    window.add(area)
    window.show_all()
    def tick():
        path = DATA / (kind + '-command')
        value = path.read_text() if path.exists() else ''
        if value == 'minimize': window.iconify(); path.unlink()
        if value == 'close': Gtk.main_quit(); return False
        return True
    GLib.timeout_add(100, tick)
    Gtk.main()

if len(sys.argv) > 1 and sys.argv[1] == '--client':
    client(sys.argv[2])
    raise SystemExit

shell, = subprocess.check_output(['pgrep', '-x', 'niwoe-shell'], text=True).split()
env = dict(v.split(b'=', 1) for v in Path('/proc/' + shell + '/environ').read_bytes().split(b'\0') if b'=' in v)
runtime = Path(env[b'XDG_RUNTIME_DIR'].decode())
assert str(runtime) == '/run/user/1000', 'explicit Fedora test user required'
token = env.pop(b'NIWOE_IPC_TOKEN').decode()
client_env = os.environ | {k.decode(): v.decode() for k, v in env.items()}
client_env.pop('NIWOE_IPC_TOKEN', None)

def connection():
    sock = socket.socket(socket.AF_UNIX)
    sock.settimeout(8)
    sock.connect(str(runtime / 'niwoe.sock'))
    sock.sendall((json.dumps(dict(type='authenticate', role='shell', token=token)) + '\n').encode())
    return sock

def send(**command):
    with connection() as sock:
        sock.sendall((json.dumps(command) + '\n').encode())
        time.sleep(.12)

def snapshot():
    with connection() as sock, sock.makefile() as reader:
        data = {}
        while not all(k in data for k in ('room-snapshot', 'window-snapshot', 'output-workspace-snapshot')):
            event = json.loads(reader.readline())
            data[event['type']] = event
        return data

def mutate(**change):
    revision = snapshot()['room-snapshot']['snapshot']['revision']
    request = 'p08-' + str(time.time_ns())
    with connection() as sock, sock.makefile() as reader:
        sock.sendall((json.dumps(dict(type='mutate-room', request_id=request, expected_revision=revision, change=change)) + '\n').encode())
        for line in reader:
            event = json.loads(line)
            if event['type'] == 'room-mutation-result' and event['request_id'] == request:
                assert event['error'] is None, event
                return

def windows():
    return snapshot()['window-snapshot']['windows']

def keyboard():
    lib = runpy.run_path(str(ROOT / 'scripts/test-login-uinput.py'), run_name='input_library')
    lib['KEYS'].update(escape=1, home=102, end=107, left=105, right=106, up=103, down=108, pageup=104, pagedown=109, ctrl=29, shift=42, f6=64, backspace=14)
    return lib

def capture_once(label):
    request_id = 'p08-shot-' + str(time.time_ns())
    output = next(o['output_name'] for o in snapshot()['output-workspace-snapshot']['outputs'] if o['primary'])
    with socket.socket(socket.AF_UNIX) as sock, connection() as control:
        sock.connect(str(runtime / 'niwoe.sock'))
        sock.settimeout(15)
        request = dict(type='screenshot-request', request=dict(request_id=request_id,kind='full-output',output=output,include_cursor=False,region=None,metadata=dict(origin='portal-dbus',requester='NIWOE P08 verification',identity_trusted=False,interactive=False)))
        sock.sendall((json.dumps(request) + '\n').encode())
        with control.makefile() as reader:
            for line in reader:
                event = json.loads(line)
                if event['type'] == 'screenshot-consent-request':
                    # Exercise the real consent UI with Enter.
                    time.sleep(.8)
                    lib=keyboard()
                    with lib['VirtualKeyboard']() as keys: keys.tap(28)
                    break
        with sock.makefile() as reader:
            for line in reader:
                response=json.loads(line)
                if response['type'] != 'screenshot-response': continue
                assert response['result']['status']=='success', response
                source=Path(response['result']['response']['file_descriptor_token'])
                shutil.copyfile(source,DATA/(label+'.png'))
                source.unlink(missing_ok=True)
                print('CAPTURE',label,flush=True)
                return

def capture(label):
    # The bridge can select either output. Prefer the primary monitor's width.
    # Equal-sized outputs after hotplug are ambiguous: visually inspect the
    # retained image for the panel/Hub before using it as primary-output evidence.
    for attempt in range(8):
        capture_once(label)
        header = (DATA / (label + '.png')).read_bytes()[:24]
        if int.from_bytes(header[16:20], 'big') == 1920:
            return
    raise AssertionError('primary output screenshot unavailable')

mode=sys.argv[1]
if mode=='library':
    pass
elif mode=='snapshot':
    print(json.dumps(snapshot(),indent=2))
elif mode=='setup':
    before=snapshot()
    assert not (DATA/'owned.json').exists(), 'cleanup previous run first'
    state=dict(original=before['room-snapshot']['snapshot']['rooms'],rooms=[],pids=[])
    (DATA/'owned.json').write_text(json.dumps(state))
    for kind in ('native','x11'):
        (DATA/(kind+'-command')).unlink(missing_ok=True)
        app_env=client_env | {'GDK_BACKEND':'wayland' if kind=='native' else 'x11'}
        if kind=='x11':
            compositor=subprocess.check_output(['pgrep','-x','niwoe'],text=True).strip()
            xpid=subprocess.check_output(['pgrep','-P',compositor,'-x','Xwayland'],text=True).strip()
            app_env['DISPLAY']=next(v.decode() for v in Path('/proc/'+xpid+'/cmdline').read_bytes().split(b'\0') if v.startswith(b':'))
        process=subprocess.Popen(['python3',__file__,'--client',kind],env=app_env,stdout=open(DATA/(kind+'.log'),'w'),stderr=subprocess.STDOUT)
        state['pids'].append(process.pid)
        (DATA/'owned.json').write_text(json.dumps(state))
        deadline=time.monotonic()+10
        while time.monotonic()<deadline:
            if any(w['title']=='P08 '+kind for w in windows()): break
            time.sleep(.2)
        else: raise AssertionError('window missing: '+kind)
    print('PASS native and X11 windows ready')
elif mode=='rooms64':
    state=json.loads((DATA/'owned.json').read_text())
    while len(snapshot()['room-snapshot']['snapshot']['rooms'])<64:
        name='P08 Raum '+str(len(snapshot()['room-snapshot']['snapshot']['rooms'])+1)
        mutate(operation='create',name=name)
        room=next(r for r in snapshot()['room-snapshot']['snapshot']['rooms'] if r['name']==name)
        state['rooms'].append(room['id'])
        (DATA/'owned.json').write_text(json.dumps(state))
    print('PASS 64 rooms created; original IDs recorded')
elif mode=='metadata64':
    state=json.loads((DATA/'owned.json').read_text())
    assert len(snapshot()['room-snapshot']['snapshot']['rooms'])==64
    for room in state['rooms']:
        mutate(operation='update-details',id=room,name='P08 '+'🗂'*60,description='🗂'*200)
    event=snapshot()['room-snapshot']
    size=len(json.dumps(event,ensure_ascii=False,separators=(',',':')).encode())
    assert size>64*1024, size
    print('PASS valid Unicode room snapshot',size,'bytes',flush=True)
elif mode=='input':
    lib=keyboard()
    with lib['VirtualKeyboard']() as keys:
        for action in sys.argv[2:]:
            if action=='hub': keys.combo(125,57)
            elif action=='enter': keys.tap(28)
            elif action=='tab': keys.tap(15)
            elif action.startswith('text:'): keys.type_text(action[5:])
            elif action.startswith('wait:'): time.sleep(float(action[5:]))
            else: keys.tap(lib['KEYS'][action])
            time.sleep(.15)
    print('PASS keyboard',sys.argv[2:])
elif mode=='click':
    lib=keyboard()
    outputs=snapshot()['output-workspace-snapshot']['outputs']
    width=max(o['x']+o['width'] for o in outputs)
    height=max(o['y']+o['height'] for o in outputs)
    with lib['VirtualPointer'](width,height) as pointer:
        pointer.click(int(sys.argv[2]),int(sys.argv[3]))
    print('PASS pointer')
elif mode=='capture': capture(sys.argv[2])
elif mode=='send':
    send(**json.loads(sys.argv[2]))
elif mode=='client':
    (DATA/(sys.argv[2]+'-command')).write_text(sys.argv[3])
elif mode=='thumbnails':
    for window in [w for w in windows() if w['title'].startswith('P08 ')]:
        request_id='probe-'+str(time.time_ns())
        with connection() as sock, sock.makefile() as reader:
            sock.sendall((json.dumps(dict(type='capture-window-thumbnail',request_id=request_id,id=window['id'],max_width=288,max_height=80))+'\n').encode())
            for line in reader:
                event=json.loads(line)
                if event['type']=='window-thumbnail' and event.get('request_id')==request_id:
                    pixels=Path(event['path']).read_bytes()
                    assert 0<event['width']<=288 and 0<event['height']<=80
                    assert len(pixels)==event['width']*event['height']*4
                    red=sum(pixels[i+2]>150 and pixels[i]<80 for i in range(0,len(pixels),4))
                    blue=sum(pixels[i]>150 and pixels[i+2]<80 for i in range(0,len(pixels),4))
                    expected=red if window['title']=='P08 native' else blue
                    assert expected>event['width']*event['height']*.4, (event,red,blue)
                    Path(event['path']).unlink()
                    print('PASS isolated thumbnail',window['title'],event['width'],event['height'],flush=True)
                    break
elif mode=='cleanup':
    state=json.loads((DATA/'owned.json').read_text())
    for kind in ('native','x11'): (DATA/(kind+'-command')).write_text('close')
    time.sleep(1)
    for room in reversed(state['rooms']): mutate(operation='delete',id=room,target_id=state['original'][0]['id'])
    assert snapshot()['room-snapshot']['snapshot']['rooms']==state['original']
    (DATA/'owned.json').rename(DATA/('completed-'+str(time.time_ns())+'.json'))
    print('PASS original rooms restored; test windows closed')
else: raise SystemExit('unknown mode '+mode)
