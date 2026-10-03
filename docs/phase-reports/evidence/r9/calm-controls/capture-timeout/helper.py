"""Observe the new shared controls and all seven regular navigation entries."""
import hashlib,json,runpy,subprocess,sys,time
from pathlib import Path
sys.argv=['review','snapshot'];ui=runpy.run_path('target/ui-visual-review-20261002.py')
p,key,capture=ui['p'],ui['key'],ui['capture']
data=Path('target/r9-calm-final-native-review');data.mkdir(exist_ok=False)
initial=p.snapshot();assert not initial['window-snapshot']['windows']
assert initial['window-snapshot']['active_workspace']==0
outputs=initial['output-workspace-snapshot']['outputs']
primary,=[o for o in outputs if o['primary']]
assert primary['output_name']=='drm-0' and primary['scale_millis']==1000
assert primary['width']==1920 and primary['height']==1080
files=[Path.home()/'.config/niwoe'/f for f in ('config.toml','panel.toml','rooms.toml','first-run.toml')]
files += [Path.home()/'.config'/f for f in ('mimeapps.list','kdeglobals','gtk-3.0/settings.ini','gtk-4.0/settings.ini','gtk-3.0/gtk.css','gtk-4.0/gtk.css')]
original={f:f.read_bytes() if f.exists() else None for f in files}
pid=subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()
sha=hashlib.sha256(Path('/proc/'+pid+'/exe').read_bytes()).hexdigest()
assert sha=='e9bed1136f1dc3c014a498f90b03fd56e531fae687163a0279ab7c383a703451'
_,env=p.environment();marker=Path(env['XDG_RUNTIME_DIR'])/'niwoe'/('first-login-hub-shown-'+env['XDG_SESSION_ID'])
stamp=marker.stat().st_mtime_ns;records=[]
def guard():
    assert all((f.read_bytes() if f.exists() else None)==v for f,v in original.items())
    assert p.snapshot()==initial
    assert marker.stat().st_mtime_ns==stamp
    assert subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()==pid
def click(x,y):
    guard();ui['pointer']('click',[x,y+48]);time.sleep(1);guard()
def shot(label):
    guard();capture('r9-calm-final-'+label);guard()
    image=ui['DATA']/('r9-calm-final-'+label+'.png')
    records.append(dict(label=label,shell_pid=pid,shell_sha256=sha,scale_millis=1000,image_sha256=hashlib.sha256(image.read_bytes()).hexdigest()))
    (data/'cases.json').write_text(json.dumps(records,indent=2))
try:
    shot('watchdog-neutral-desktop')
    key(['hub']);shot('hub');key(['tab']);shot('hub-keyboard-focus');key(['escape']*5)
    p.send(type='open-system-settings');time.sleep(3)
    click(80,112);shot('rooms-mouse')
    key(['f8','down','enter']);time.sleep(2);shot('apps-keyboard')
    key(['f8','down','enter']);time.sleep(2);shot('appearance-keyboard')
    key(['f8','tab']);shot('appearance-keyboard-focus')
    key(['tab','tab','enter']);time.sleep(2);shot('display-keyboard')
    click(1100,108);shot('cursor-mouse')
    key(['f7']);time.sleep(2);shot('panel-keyboard')
    click(80,360);shot('system-mouse')
    click(1220,108);shot('sound-mouse')
    click(676,108);shot('network-mouse')
    key(['f8','down','enter']);time.sleep(2);shot('users-keyboard')
    key(['f8','down','enter']);time.sleep(2);shot('updates-keyboard')
    key(['f8','down','enter']);time.sleep(2);shot('rooms-keyboard-wrap')
    click(450,440);shot('room-general')
    click(450,224);shot('room-apps')
    click(550,224);shot('room-files')
finally:
    key(['escape']*5)
    focused,=[o for o in outputs if o['focused']]
    ui['pointer']('move',[focused['x']+focused['width']//2,focused['y']+focused['height']//2])
    time.sleep(1);guard()
    result=dict(protected_files_byte_equal=True,state_equal=True,neutral_foyer=True,login_marker_unchanged=True,process_unchanged=True,shell_pid=pid,shell_sha256=sha)
    (data/'cleanup.json').write_text(json.dumps(result,indent=2));print(json.dumps(result),flush=True)
