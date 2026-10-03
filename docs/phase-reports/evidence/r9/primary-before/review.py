"""Exercise only the existing display controls and restore exact owned config."""
import hashlib,json,os,runpy,signal,subprocess,sys,time,tomllib
from pathlib import Path
sys.argv=['review','snapshot']
ui=runpy.run_path('target/ui-visual-review-20261002.py')
p,key,capture=ui['p'],ui['key'],ui['capture']
data=Path('target/r9-primary-before-review')
data.mkdir(exist_ok=False)
config=Path.home()/'.config/niwoe/config.toml'
original=config.read_bytes()
base=tomllib.loads(original.decode())
assert not base.get('outputs')
others={f:f.read_bytes() if f.exists() else None for f in (
    Path.home()/'.config/niwoe/panel.toml',Path.home()/'.config/niwoe/first-run.toml',Path.home()/'.config/niwoe/rooms.toml',Path.home()/'.config/mimeapps.list')}
before=p.snapshot()
assert not before['window-snapshot']['windows']
assert len(before['output-workspace-snapshot']['outputs'])==2
expected=Path('target/r8-first-run-error-gates/release.sha256').read_text().split()[0]
pid=subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()
assert hashlib.sha256(Path('/proc/'+pid+'/exe').read_bytes()).hexdigest()==expected
compositor_pid=subprocess.check_output(['pgrep','-x','niwoe'],text=True).strip()
compositor_sha=hashlib.sha256(Path('/proc/'+compositor_pid+'/exe').read_bytes()).hexdigest()
assert compositor_sha=='c09985e39b77bea259fb81aaae8ab92f4933bf50a0a8971756bde146457d6f65'
first,=[o for o in before['output-workspace-snapshot']['outputs'] if o['primary']]
assert first['output_name']=='drm-0' and first['scale_millis']==1000
second,=[o for o in before['output-workspace-snapshot']['outputs'] if o['output_name']=='drm-1']
assert second['transform']=='Normal' and second['scale_millis']==1000
(data/'config-original.toml').write_bytes(original)
(data/'before.json').write_text(json.dumps(before,indent=2))
(data/'other-original.json').write_text(json.dumps({str(f):v.hex() if v is not None else None for f,v in others.items()},indent=2))
owned=original
records=[]
def state():
    snapshot=p.snapshot()
    assert len(snapshot['output-workspace-snapshot']['outputs'])==2
    assert {o['output_id'] for o in snapshot['output-workspace-snapshot']['outputs']}=={o['output_id'] for o in before['output-workspace-snapshot']['outputs']}
    assert not snapshot['window-snapshot']['windows']
    return snapshot
def output(name):
    return next(o for o in state()['output-workspace-snapshot']['outputs'] if o['output_name']==name)
def guard():
    assert config.read_bytes()==owned,'concurrent config edit; stop'
    assert all((f.read_bytes() if f.exists() else None)==v for f,v in others.items())
def click(x,y):
    guard()
    primary=next(o for o in state()['output-workspace-snapshot']['outputs'] if o['output_name']=='drm-0')
    scale=primary['scale_millis']/1000
    width,height=round(primary['width']/scale),round(primary['height']/scale)
    source_width,source_height=max(width,1366),max(height-48,720)
    ui['pointer']('click',[round(primary['x']+x*width/source_width),round(primary['y']+48+y*(height-48)/source_height)])
def shot(label):
    guard()
    capture('r9-primary-before-'+label)
    guard()
def open_display(second_page=True):
    guard()
    key(['escape']*5)
    p.send(type='open-system-settings')
    time.sleep(3)
    primary=next(o for o in state()['output-workspace-snapshot']['outputs'] if o['output_name']=='drm-0')
    scale=primary['scale_millis']/1000
    sw,sh=max(round(primary['width']/scale),1366),max(round(primary['height']/scale)-48,720)
    row=min(48,max(38,(sh-60-20-96-24)//11))
    click(80,84+10.5*row+56)
    time.sleep(3)
    click(sw-200,100)
    if second_page:click(536,220)
def action(label,run,config_assert,output_assert):
    global owned
    guard()
    run()
    time.sleep(3)
    current=config.read_bytes()
    parsed=tomllib.loads(current.decode())
    assert {k:v for k,v in parsed.items() if k!='outputs'}==base,'unexpected non-output edit; do not take ownership'
    assert set(parsed.get('outputs',{}))<= {'drm-0','drm-1'}
    owned=current
    (data/'config-latest.toml').write_bytes(owned)
    snap=state()
    records.append(dict(case=label,outputs=snap['output-workspace-snapshot']['outputs'],config_outputs=parsed.get('outputs',{}),shell_sha256=expected,compositor_sha256=compositor_sha,compositor_pid=compositor_pid))
    (data/'cases.json').write_text(json.dumps(records,indent=2))
    config_assert(parsed.get('outputs',{}))
    output_assert(snap['output-workspace-snapshot']['outputs'])
    shot(label)
    print('PASS',label,flush=True)
def equal(actual,expected_value):
    assert actual==expected_value,(actual,expected_value)
def target(outputs,name='drm-1'):
    return next(o for o in outputs if o['output_name']==name)
def mode_is(outputs,width,height,refresh):
    cur=[m for m in target(outputs)['modes'] if m['current']]
    assert cur and all((m['width'],m['height'],m['refresh_millihz'])==(width,height,refresh) for m in cur),cur
    equal(target(outputs,'drm-0')['primary'],True)
try:
    open_display()
    shot('before-second-output')
    action('second-primary-saved',lambda:click(550,385),
        lambda c:equal(c['drm-1']['primary'],True),
        lambda o:equal(target(o)['primary'],True))
    capture('r9-primary-before-old-output-after-save','drm-0')
finally:
    key(['escape']*5)
    guard()
    temporary=original.decode()
    for out in before['output-workspace-snapshot']['outputs']:
        temporary+='\n[outputs.'+json.dumps(out['output_name'])+']\nenabled = true\nprimary = '+str(out['primary']).lower()
        temporary+='\nscale = '+str(out['scale_millis']/1000)
        temporary+='\nmode = { width = %d, height = %d, refresh_millihz = %d }\n' % (out['width'],out['height'],out['refresh_millihz'])
        assert out['transform']=='Normal'
    try:
        config.write_text(temporary)
        p.send(type='reload-config')
        time.sleep(3)
    finally:
        config.write_bytes(original)
        p.send(type='reload-config')
        time.sleep(3)
    old=subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()
    os.kill(int(old),signal.SIGTERM)
    deadline=time.monotonic()+25
    while time.monotonic()<deadline:
        try:
            pid=subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()
            if pid!=old:break
        except subprocess.CalledProcessError:pass
        time.sleep(.2)
    else:raise AssertionError('watchdog deadline')
    time.sleep(6)
    assert hashlib.sha256(Path('/proc/'+pid+'/exe').read_bytes()).hexdigest()==expected
    focused,=[o for o in before['output-workspace-snapshot']['outputs'] if o['focused']]
    ui['pointer']('move',[focused['x']+focused['width']//2,focused['y']+focused['height']//2])
    time.sleep(1)
    after=p.snapshot()
    result=dict(config_bytes_equal=config.read_bytes()==original,
        other_files_equal=all((f.read_bytes() if f.exists() else None)==v for f,v in others.items()),
        rooms_equal=before['room-snapshot']==after['room-snapshot'],
        outputs_equal=before['output-workspace-snapshot']==after['output-workspace-snapshot'],
        windows_empty=not after['window-snapshot']['windows'],sha256=expected,shell_pid=pid)
    (data/'after.json').write_text(json.dumps(after,indent=2))
    (data/'cleanup.json').write_text(json.dumps(result,indent=2))
    assert all(result[k] for k in ('config_bytes_equal','other_files_equal','rooms_equal','outputs_equal','windows_empty'))
    print(json.dumps(result),flush=True)
