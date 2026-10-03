import hashlib, json
from pathlib import Path
expected = json.loads(Path('target/r9-primary-source-hashes.json').read_text())
actual = {name: hashlib.sha256(Path(name).read_bytes().replace(b'\r\n', b'\n')).hexdigest() for name in expected}
Path('target/r9-primary-source-verified.json').write_text(json.dumps(actual, indent=2, sort_keys=True))
different = [name for name in expected if expected[name] != actual[name]]
print('Source files', len(actual), 'mismatches', different)
assert not different
