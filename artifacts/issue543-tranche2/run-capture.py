from pathlib import Path
import hashlib
import json
import os
import subprocess
import sys

prefix = Path(sys.argv[1])
cwd = sys.argv[2]
argv = sys.argv[3:]
if not argv:
    raise SystemExit("missing command")

git_head = subprocess.run(
    ["git", "rev-parse", "HEAD"], cwd=cwd, check=True, capture_output=True, text=True
).stdout.strip()
git_status = subprocess.run(
    ["git", "status", "--short"],
    cwd=cwd,
    check=True,
    capture_output=True,
    text=True,
).stdout.splitlines()
selected_sources = {}
for relative in filter(None, os.environ.get("ISSUE543_HASH_PATHS", "").split(",")):
    path = Path(cwd) / relative
    selected_sources[relative] = (
        hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
    )
safe_environment = {
    name: os.environ[name]
    for name in (
        "CARGO_TARGET_DIR",
        "CARGO_TERM_COLOR",
        "RUSTFLAGS",
        "RUSTUP_TOOLCHAIN",
    )
    if name in os.environ
}
with open(str(prefix) + ".command.json", "x") as stream:
    json.dump(
        {
            "argv": argv,
            "cwd": cwd,
            "environment": safe_environment,
            "git_head_before": git_head,
            "git_status_before": git_status,
            "selected_source_sha256_before": selected_sources,
        },
        stream,
        indent=2,
    )

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
