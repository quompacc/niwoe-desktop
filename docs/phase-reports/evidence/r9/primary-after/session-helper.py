"""Finish the two session captures after confirmed exact primary-review cleanup."""
import hashlib,json,runpy,subprocess,sys,time
from pathlib import Path
sys.argv=['review','snapshot']
ui=runpy.run_path('target/ui-visual-review-20261002.py');p,key,capture=ui['p'],ui['key'],ui['capture']
data=Path('target/r9-session-tail-review');data.mkdir(exist_ok=False)
previous=Path('target/r9-primary-after-review')
clean=json.loads((previous/'cleanup.json').read_text())
assert all(clean[k] for k in ('config_bytes_equal','other_files_equal','rooms_equal','outputs_equal','windows_empty','neutral_foyer','login_marker_unchanged'))
baseline=json.loads((previous/'after.json').read_text())
config=Path.home()/'.config/niwoe/config.toml'
original=(previous/'config-original.toml').read_bytes()
others={Path(f):(bytes.fromhex(v) if v is not None else None) for f,v in json.loads((previous/'other-original.json').read_text()).items()}
pid=clean['shell_pid']
assert subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()==pid
sha=hashlib.sha256(Path('/proc/'+pid+'/exe').read_bytes()).hexdigest();assert sha==clean['sha256']
_,env=p.environment()
marker=Path(env['XDG_RUNTIME_DIR'])/'niwoe'/('first-login-hub-shown-'+env['XDG_SESSION_ID'])
stamp=marker.stat().st_mtime_ns
records=[]

def guard():
    assert config.read_bytes()==original
    assert all((f.read_bytes() if f.exists() else None)==v for f,v in others.items())
    assert p.snapshot()==baseline
    assert marker.stat().st_mtime_ns==stamp
    assert subprocess.check_output(['pgrep','-x','niwoe-shell'],text=True).strip()==pid

def shot(label):
    guard();capture('r9-session-tail-'+label);guard()
    image=ui['DATA']/('r9-session-tail-'+label+'.png')
    records.append(dict(label=label,pid=pid,shell_sha256=sha,
                        neutral_foyer=True,login_marker_unchanged=True,
                        image_sha256=hashlib.sha256(image.read_bytes()).hexdigest()))
    (data/'cases.json').write_text(json.dumps(records,indent=2))

try:
    shot('watchdog-neutral-desktop')
    key(['hub']);time.sleep(2)
    shot('regular-hub-after-watchdog')
finally:
    key(['escape']*2);guard()
    result=dict(config_bytes_equal=True,other_files_equal=True,state_equal=True,
                neutral_foyer=True,login_marker_unchanged=True,pid=pid,shell_sha256=sha)
    (data/'cleanup.json').write_text(json.dumps(result,indent=2))
    print(json.dumps(result),flush=True)
