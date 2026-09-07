from pathlib import Path
import hashlib
import json
import subprocess
import sys

prefix = Path(sys.argv[1])
cwd = sys.argv[2]
argv = sys.argv[3:]
if not argv:
    raise SystemExit("missing command")

with open(str(prefix) + ".command.json", "x") as stream:
    json.dump({"argv": argv, "cwd": cwd}, stream, indent=2)

with open(str(prefix) + ".stdout", "xb") as stdout, open(
    str(prefix) + ".stderr", "xb"
) as stderr:
    result = subprocess.run(argv, cwd=cwd, stdout=stdout, stderr=stderr)

Path(str(prefix) + ".status").write_text(f"{result.returncode}\n")
for suffix in (".command.json", ".stdout", ".stderr", ".status"):
    path = Path(str(prefix) + suffix)
    Path(str(path) + ".sha256").write_text(
        hashlib.sha256(path.read_bytes()).hexdigest() + "\n"
    )
raise SystemExit(result.returncode)
