#!/usr/bin/env python3
"""Summarize raw P12 measurements without exporting personal room snapshots."""
import hashlib, json, os, statistics
from pathlib import Path
d = Path(__file__).resolve().parents[1] / 'target/p12-evidence'
load = lambda name: json.loads((d / name).read_text())
before = load('before-idle.json')
replacement = d / 'before-idle-replacement.json'
if replacement.exists():
    # This P12 run's third baseline sample was interrupted by physical hotplug.
    before = before[:2] + load(replacement.name)
after = load('after-idle.json')
assert len(before) == len(after) == 3
out = {'idle': {}, 'cycles': {}, 'raw_sha256': {}}
for label, samples in [('before', before), ('after', after)]:
    assert all(s['seconds'] >= 300 for s in samples)
    if label == 'after': assert all(s['state_unchanged'] for s in samples)
    out['idle'][label] = [{k:v for k,v in s.items() if k not in ('before_state','after_state')} for s in samples]
for label in ('before','after'):
    runs = load(label + '-cycles.json')
    assert len(runs) == 3 and all(r['cycles'] == 20 for r in runs)
    out['cycles'][label] = [dict(seconds=r['seconds'], gpu_engine_percent=r['gpu_engine_percent'],
        processes={n:dict(cpu_percent=100*(r['after'][n][0]-r['before'][n][0])/os.sysconf('SC_CLK_TCK')/r['seconds'],
        rss_before=r['before'][n][1], rss_after=r['after'][n][1],
        rss_min=min(s[n] for s in r['rss_samples']), rss_max=max(s[n] for s in r['rss_samples']))
        for n in r['before']}) for r in runs]
out['median_idle'] = {}
for name in ('niwoe', 'niwoe-shell'):
    a,b = [statistics.median(s['processes'][name]['cpu_percent'] for s in samples) for samples in (before,after)]
    maximum_delta = max(s['processes'][name]['cpu_percent'] for s in after) - a
    out['median_idle'][name] = dict(before=a, after=b, delta_percentage_points=b-a,
        maximum_delta_percentage_points=maximum_delta, budget_pass=maximum_delta<=.5)
out['median_idle']['gpu_engine_percent'] = {label:statistics.median(s['gpu_engine_percent'] for s in samples) for label,samples in [('before',before),('after',after)]}
files = ['before-idle.json','after-idle.json','before-cycles.json','after-cycles.json']
if replacement.exists(): files.append(replacement.name)
for file in files:
    out['raw_sha256'][file] = hashlib.sha256((d/file).read_bytes()).hexdigest()
(d/'performance-summary.json').write_text(json.dumps(out,indent=2))
print(json.dumps(out,indent=2))
assert all(out['median_idle'][n]['budget_pass'] for n in ('niwoe','niwoe-shell')), 'CPU budget exceeded; summary retained'
