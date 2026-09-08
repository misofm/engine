#!/usr/bin/env bash
# Prepare exactly one protected Issue #600 capture. This command never runs the subject.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(cd "$script_directory/.." && pwd)
artifact_directory="$repository_root/artifacts/issue600-input-symmetry"
prepared_directory="$repository_root/target/issue600-input-symmetry"
build_directory="$prepared_directory/build"
binary="$prepared_directory/bench"
seal="$artifact_directory/input-symmetry-benchmark.preflight.json"
validator="$script_directory/input-symmetry-benchmark-validator.py"
runner="$script_directory/run-input-symmetry-benchmark.sh"
subject="$repository_root/tools/bench/src/input_symmetry.rs"
dispatcher="$repository_root/tools/bench/src/main.rs"
fixture="$repository_root/fixtures/session/v1/parametric-eq-bank-console.json"

fail() { printf 'Issue-600 input-symmetry preflight failure: %s\n' "$1" >&2; exit 1; }
for tool in awk cargo cp git mktemp mv python3 rustc sha256sum stat; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool unavailable: $tool"
done
for override in RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS CARGO_BUILD_TARGET \
    CARGO_INCREMENTAL CARGO_BUILD_INCREMENTAL RUSTC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER \
    CARGO_BUILD_RUSTC CARGO_BUILD_RUSTC_WRAPPER CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER; do
    [[ ! -v "$override" ]] || fail "incompatible build environment: $override"
done
while IFS= read -r override; do
    [[ "$override" == CARGO_PROFILE_* ]] && fail "incompatible build environment: $override"
done < <(compgen -e)
[[ ! -e "$artifact_directory" || -d "$artifact_directory" ]] || fail 'artifact namespace is not a directory'
[[ ! -e "$prepared_directory" || -d "$prepared_directory" ]] || fail 'prepared namespace is not a directory'
mkdir -p "$artifact_directory" "$prepared_directory"
for path in "$seal" "$artifact_directory/input-symmetry-benchmark.raw.jsonl" \
    "$artifact_directory/input-symmetry-benchmark.jsonl" "$artifact_directory/input-symmetry-benchmark.stdout" \
    "$artifact_directory/input-symmetry-benchmark.stderr" "$artifact_directory/input-symmetry-benchmark.validator.stderr" \
    "$artifact_directory/input-symmetry-benchmark.disposition.json" \
    "$binary"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "refusing protected output: $path"
done
for path in "$validator" "$runner" "$subject" "$dispatcher" "$fixture" "$repository_root/Cargo.lock"; do
    [[ -f "$path" && ! -L "$path" ]] || fail "required input unavailable: $path"
done
candidate_commit=$(git -C "$repository_root" rev-parse --verify HEAD) || fail 'candidate commit unavailable'
candidate_tree=$(git -C "$repository_root" rev-parse 'HEAD^{tree}') || fail 'candidate tree unavailable'
[[ -z "$(git -C "$repository_root" status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate is not clean'
target_triple=$(rustc -vV | awk '$1 == "host:" { print $2; found=1 } END { if (!found) exit 1 }') || fail 'target unavailable'
[[ "$target_triple" == x86_64-unknown-linux-gnu ]] || fail "unsupported target: $target_triple"
rust_version=$(rustc -Vv | awk 'NR == 1 { print }')
compiler=$(rustc -V)
build_flags='-Ctarget-feature=+avx2,+fma;release;opt-level=3;lto=fat;codegen-units=1;panic=abort;debug=1;debug-assertions=false;overflow-checks=false;incremental=false;rpath=false;strip=none;split-debuginfo=off'
rm -rf -- "$build_directory"
mkdir -p "$build_directory"
if ! (cd "$repository_root" && CARGO_TARGET_DIR="$build_directory" CARGO_INCREMENTAL=0 \
    CARGO_PROFILE_RELEASE_OPT_LEVEL=3 CARGO_PROFILE_RELEASE_LTO=fat \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 CARGO_PROFILE_RELEASE_PANIC=abort \
    CARGO_PROFILE_RELEASE_DEBUG=1 CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false \
    CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=false CARGO_PROFILE_RELEASE_INCREMENTAL=false \
    CARGO_PROFILE_RELEASE_RPATH=false CARGO_PROFILE_RELEASE_STRIP=none \
    CARGO_PROFILE_RELEASE_SPLIT_DEBUGINFO=off CARGO_ENCODED_RUSTFLAGS='-Ctarget-feature=+avx2,+fma' \
    cargo build --locked --release -p bench --target "$target_triple" >/dev/null); then
    fail 'isolated release build failed'
fi
built_binary="$build_directory/$target_triple/release/bench"
[[ -x "$built_binary" ]] || fail 'release binary unavailable'
temporary_binary=$(mktemp "$prepared_directory/.bench.XXXXXX")
cp -- "$built_binary" "$temporary_binary"
chmod 0755 "$temporary_binary"
mv -n -- "$temporary_binary" "$binary"
[[ -x "$binary" && ! -L "$binary" ]] || fail 'binary publication failed'
hash_file() { sha256sum "$1" | awk '{print $1}'; }
python3 -I -B "$validator" --self-test >/dev/null || fail 'validator self-test failed'
source_sha256=$(sha256sum "$subject" "$dispatcher" | sha256sum | awk '{print $1}')
binary_sha256=$(hash_file "$binary")
fixture_sha256=$(hash_file "$fixture")
validator_sha256=$(hash_file "$validator")
runner_sha256=$(hash_file "$runner")
preflight_sha256=$(hash_file "$script_directory/preflight-input-symmetry-benchmark.sh")
lock_sha256=$(hash_file "$repository_root/Cargo.lock")
cwd="$repository_root"
python3 - "$seal" "$candidate_commit" "$candidate_tree" "$binary_sha256" "$fixture_sha256" \
    "$source_sha256" "$validator_sha256" "$runner_sha256" "$preflight_sha256" "$lock_sha256" \
    "$target_triple" "$rust_version" "$compiler" "$build_flags" "$cwd" "$binary" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
value = {
    "schema_version": 1, "issue": 600, "kind": "input_symmetry_benchmark_preflight", "status": "READY",
    "candidate_commit": sys.argv[2], "candidate_tree": sys.argv[3], "binary_sha256": sys.argv[4],
    "fixture_sha256": sys.argv[5], "source_sha256": sys.argv[6], "validator_sha256": sys.argv[7],
    "runner_sha256": sys.argv[8], "preflight_sha256": sys.argv[9], "cargo_lock_sha256": sys.argv[10],
    "target_triple": sys.argv[11], "rust_version": sys.argv[12], "compiler": sys.argv[13],
    "build_flags": sys.argv[14], "cwd": sys.argv[15], "argv": sys.argv[16] + " input-symmetry",
    "warmup_blocks": 512, "measured_blocks": 4096,
    "records_required": 2, "preflight_invocations": 1, "runner_invocations": 0,
    "workload_invocations": 0, "timed_benchmark_invocations": 0,
}
temporary = path.with_name('.preflight.tmp')
temporary.write_text(json.dumps(value, sort_keys=True, separators=(',', ':')) + '\n', encoding='utf-8')
temporary.replace(path)
PY
printf 'Issue-600 input-symmetry preflight: READY (preflight/runner/workload/timed=1/0/0/0)\n'
