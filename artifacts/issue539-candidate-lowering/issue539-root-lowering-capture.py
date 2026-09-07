from pathlib import Path
import hashlib
import json
import os
import subprocess
import sys

label = sys.argv[1]
argv = sys.argv[2:]
assert label and all(character.isalnum() or character == "-" for character in label)
assert argv
root = Path("/home/bl/misofm/engine-limiter-stationary-dispatch")
prefix = Path("/tmp/issue539-root-lowering") / label
prefix.parent.mkdir(parents=True, exist_ok=True)
paths = [
    "crates/true-peak-limiter/src/lib.rs",
    "crates/true-peak-limiter/tests/mono_collapse.rs",
    "crates/true-peak-limiter/tests/allocation.rs",
    "Cargo.toml",
    "Cargo.lock",
    ".cargo/config.toml",
    "rust-toolchain.toml",
]
paths = [path for path in paths if (root / path).exists()]
environment = os.environ.copy()
environment["PATH"] = "/home/bl/.cargo/bin:" + environment.get("PATH", "")
environment["CARGO_TARGET_DIR"] = "/tmp/issue539-root-lowering-target"
metadata = {
    "argv": argv,
    "cwd": str(root),
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
    "branch": subprocess.check_output(["git", "branch", "--show-current"], cwd=root, text=True).strip(),
    "status": subprocess.check_output(["git", "status", "--porcelain=v1"], cwd=root, text=True),
    "env": {key: environment.get(key) for key in ["PATH", "RUSTFLAGS", "CARGO_TARGET_DIR"]},
    "source": {
        path: {
            "sha256": hashlib.sha256((root / path).read_bytes()).hexdigest(),
            "git_blob": subprocess.check_output(["git", "hash-object", path], cwd=root, text=True).strip(),
        }
        for path in paths
    },
}
with prefix.with_suffix(".command.json").open("x") as command_file:
    json.dump(metadata, command_file, indent=2)
    command_file.write("\n")
with prefix.with_suffix(".stdout").open("xb") as stdout_file, prefix.with_suffix(".stderr").open("xb") as stderr_file:
    result = subprocess.run(argv, cwd=root, env=environment, stdout=stdout_file, stderr=stderr_file, check=False)
with prefix.with_suffix(".status").open("x") as status_file:
    status_file.write(f"{result.returncode}\n")
print(label, result.returncode, flush=True)
sys.exit(result.returncode)
