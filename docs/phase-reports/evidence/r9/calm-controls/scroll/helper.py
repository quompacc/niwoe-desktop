"""Native geometry checks on both actual outputs; preserve owned temporary config."""
import hashlib,json,runpy,subprocess,sys,time,tomllib
from pathlib import Path
sys.argv=['review','snapshot'];ui=runpy.run_path('target/ui-visual-review-20261002.py')
p,key,capture=ui['p'],ui['key'],ui['capture']
data=Path('target/r9-calm-scroll-review');data.mkdir(exist_ok=False)
config=Path.home()/'.config/niwoe/config.toml';original=config.read_bytes();owned=original
assert not tomllib.loads(original.decode()).get('outputs')
files=[Path.home()/'.config/niwoe'/f for f in ('panel.toml','rooms.toml','first-run.toml')]
files += [Path.home()/'.config'/f for f in ('mimeapps.list','kdeglobals','gtk-3.0/settings.ini','gtk-4.0/settings.ini')]
others={f:f.read_bytes() if f.exists() else None for f in files}
initial=p.snapshot();assert not initial['window-snapshot']['windows']
assert initial['window-snapshot']['active_workspace']==0
before_outputs=initial['output-workspace-snapshot']['outputs'];assert len(before_outputs)==2
assert {o['output_name'] for o in before_outputs}=={'drm-0','drm-1'}
pid=subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()
expected=Path('target/r9-calm-gates/release.sha256').read_text().split()[0]
assert hashlib.sha256(Path('/proc/'+pid+'/exe').read_bytes()).hexdigest()==expected
compositor=subprocess.check_output(['pgrep','-x','niwoe'],text=True).strip()
_,env=p.environment();marker=Path(env['XDG_RUNTIME_DIR'])/'niwoe'/('first-login-hub-shown-'+env['XDG_SESSION_ID'])
stamp=marker.stat().st_mtime_ns;records=[]
(data/'config-original.toml').write_bytes(original)
(data/'before.json').write_text(json.dumps(initial,indent=2))
def guard():
    assert config.read_bytes()==owned,'concurrent config edit; stop'
    assert all((f.read_bytes() if f.exists() else None)==v for f,v in others.items())
    snapshot=p.snapshot()
    assert snapshot['room-snapshot']==initial['room-snapshot']
    assert not snapshot['window-snapshot']['windows'] and snapshot['window-snapshot']['active_workspace']==0
    assert marker.stat().st_mtime_ns==stamp
    assert subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()==pid
    assert subprocess.check_output(['pgrep','-x','niwoe'],text=True).strip()==compositor
    return snapshot
def output_config(primary=None,scale=None):
    value=original.decode()
    for output in before_outputs:
        value+='\n[outputs.'+json.dumps(output['output_name'])+']\nenabled = true\nprimary = '+str(output['primary'] if primary is None else output['output_name']==primary).lower()
        value+='\nscale = '+str(output['scale_millis']/1000 if scale is None else scale)+'\nposition = "auto"\n'
        value+='mode = { width = %d, height = %d, refresh_millihz = %d }\n'%(output['width'],output['height'],output['refresh_millihz'])
    return value.encode()
def write(value):
    global owned
    guard();config.write_bytes(value);owned=value
    (data/'config-latest.toml').write_bytes(owned)
    p.send(type='reload-config');time.sleep(3);guard()
def geometry():
    outputs=guard()['output-workspace-snapshot']['outputs']
    output,=[o for o in outputs if o['primary']]
    scale=output['scale_millis']/1000
    width,height=round(output['width']/scale),round(output['height']/scale)
    return output,width,height,max(width,1366),max(height-48,720)
def point(x,y):
    output,width,height,sw,sh=geometry()
    return [round(output['x']+x*width/sw),round(output['y']+48+y*(height-48)/sh)]
def click(x,y):
    guard();ui['pointer']('click',point(x,y));time.sleep(1);guard()
def shot(label):
    output,width,height,sw,sh=geometry()
    name='r9-calm-scroll-'+output['output_name']+'-s'+str(output['scale_millis'])+'-'+label
    capture(name);guard()
    records.append(dict(label=name,output=output,source_canvas=[sw,sh],shell_pid=pid,shell_sha256=expected,image_sha256=hashlib.sha256((ui['DATA']/(name+'.png')).read_bytes()).hexdigest()))
    (data/'cases.json').write_text(json.dumps(records,indent=2))
try:
    for primary in ('drm-0',):
        for scale in (1.5,2.0):
            key(['escape']*5);write(output_config(primary,scale))
            actual=geometry()[0];assert actual['output_name']==primary and actual['scale_millis']==round(scale*1000)
            p.send(type='open-system-settings');time.sleep(3)
            for index,label in enumerate(('rooms',)):
                click(80,112+48*index+(56 if index>=2 else 0))
                time.sleep(2);shot(label)
            click(80,112);click(450,440);shot('room-general')
            ui['pointer']('scroll',point(650,450)+[16]);time.sleep(1);shot('room-general-lower')
            click(450,224);shot('room-apps')
finally:
    key(['escape']*5)
    write(output_config())
    write(original)
    focused,=[o for o in before_outputs if o['focused']]
    ui['pointer']('move',[focused['x']+focused['width']//2,focused['y']+focused['height']//2])
    time.sleep(1);after=guard()
    result=dict(config_bytes_equal=config.read_bytes()==original,other_files_equal=all((f.read_bytes() if f.exists() else None)==v for f,v in others.items()),state_equal=after==initial,neutral_foyer=True,login_marker_unchanged=True,processes_unchanged=True,shell_sha256=expected,shell_pid=pid)
    (data/'cleanup.json').write_text(json.dumps(result,indent=2));assert result['state_equal'];print(json.dumps(result),flush=True)
