import hashlib, json, os, tarfile
from pathlib import Path

root = Path.cwd().resolve()
expected = json.loads(Path('target/r9-primary-source-hashes.json').read_text())
with tarfile.open('target/r9-primary-source.tar') as archive:
    assert {member.name for member in archive.getmembers()} == set(expected)
    for member in archive.getmembers():
        destination = (root / member.name).resolve()
        assert destination.is_relative_to(root / 'crates') and member.isfile()
        stream = archive.extractfile(member)
        raw = stream.read().replace(b'\r\n', b'\n')
        assert hashlib.sha256(raw).hexdigest() == expected[member.name]
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(raw)
        os.utime(destination, None)
print('PASS exact checked source archive, canonical LF bytes and fresh build timestamps', len(expected))
