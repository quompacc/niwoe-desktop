import hashlib,json,os,signal,subprocess,time
from pathlib import Path
expected='f5de3bdcd398259dacf250ad2d1d0cb5382fc8f4bbea255ccd3079e055676448'
def pid(name):return subprocess.check_output(['pgrep','-x',name],text=True).strip()
def digest(name):return hashlib.sha256(Path('/proc/'+pid(name)+'/exe').read_bytes()).hexdigest()
paths=[Path.home()/'.config/niwoe'/f for f in ('config.toml','panel.toml','rooms.toml','first-run.toml')]
paths += [Path.home()/'.config'/f for f in ('mimeapps.list','kdeglobals','gtk-3.0/settings.ini','gtk-4.0/settings.ini','gtk-3.0/gtk.css','gtk-4.0/gtk.css')]
original={f:f.read_bytes() if f.exists() else None for f in paths}
old=pid('niwoe-shell');compositor=pid('niwoe')
assert digest('niwoe-shell')=='010d2ac4b1afd1fe1b4f27270ee1ef1b07b019b4c97a0df9a21580031837da17'
assert digest('niwoe')=='8f1e15e3678860e9160cc00ee17e61c0650e9c15ae1ecb086fef53e48c3290e3'
assert hashlib.sha256(Path('/usr/local/bin/niwoe-shell').read_bytes()).hexdigest()==expected
os.kill(int(old),signal.SIGTERM)
for _ in range(100):
    time.sleep(.2)
    if pid('niwoe-shell')!=old:break
assert pid('niwoe-shell')!=old and digest('niwoe-shell')==expected
assert pid('niwoe')==compositor
assert all((f.read_bytes() if f.exists() else None)==value for f,value in original.items())
result=dict(shell_pid=pid('niwoe-shell'),shell_sha256=expected,compositor_pid=compositor,compositor_sha256=digest('niwoe'),configuration_bytes_equal=True,shared_kde_gtk_bytes_equal=True)
Path('target/r9-calm-gates/activation.json').write_text(json.dumps(result,indent=2))
print(json.dumps(result))
