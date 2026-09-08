#!/usr/bin/env bash
set -euo pipefail

if (($#)); then echo "usage: $0" >&2; exit 2; fi
repo=$(git rev-parse --show-toplevel)
cd "$repo"
for name in RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_TARGET_DIR CARGO_BUILD_TARGET CARGO_BUILD_RUSTC CARGO_BUILD_RUSTFLAGS RUSTC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER RUSTDOC RUSTDOCFLAGS CARGO_PROFILE_RELEASE_CODEGEN_UNITS CARGO_PROFILE_RELEASE_LTO CARGO_PROFILE_RELEASE_OPT_LEVEL CARGO_PROFILE_RELEASE_DEBUG CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS CARGO_PROFILE_RELEASE_PANIC CARGO_PROFILE_RELEASE_STRIP CARGO_PROFILE_RELEASE_INCREMENTAL CARGO_PROFILE_RELEASE_RPATH; do
  [[ -z "${!name-}" ]] || { echo "refusing inherited $name" >&2; exit 1; }
done
[[ -n "${MISO_ENGINE_603_EXPECTED_HEAD-}" && "$(git rev-parse HEAD)" == "$MISO_ENGINE_603_EXPECTED_HEAD" ]] || { echo "exact approved candidate head required" >&2; exit 1; }
git diff --quiet && git diff --cached --quiet || { echo "worktree must be clean" >&2; exit 1; }
[[ -z "$(git status --porcelain --untracked-files=all)" ]] || { echo "worktree must have no untracked files" >&2; exit 1; }

root=artifacts/issue-603-input-symmetry-capture
qualification="$root/qualification"
prepared="$root/prepared-release"
seal="$root/input-symmetry-capture.seal.json"
[[ ! -e "$seal" && ! -L "$seal" && ! -e "$root/input-symmetry-capture.jsonl" && ! -L "$root/input-symmetry-capture.jsonl" ]] || { echo "protected output already exists" >&2; exit 1; }
[[ ! -L "$root" && ! -L "$qualification" && ! -L "$prepared" ]] || { echo "artifact paths may not be symlinks" >&2; exit 1; }
[[ ! -e "$qualification" || -d "$qualification" ]] || { echo "qualification path must be a directory" >&2; exit 1; }
[[ ! -e "$prepared" && ! -L "$prepared" ]] || { echo "prepared directory already exists" >&2; exit 1; }
mkdir -p "$qualification" "$prepared"

export CARGO_TARGET_DIR="$repo/$prepared/target"
export CARGO_ENCODED_RUSTFLAGS=$'-Ctarget-feature=+avx2,+fma\x1f-Copt-level=3\x1f-Clto=fat\x1f-Ccodegen-units=1\x1f-Cpanic=abort\x1f-Cdebuginfo=1'
cargo build --locked --release -p bench --bin bench
binary="$CARGO_TARGET_DIR/release/bench"
[[ -x "$binary" ]] || { echo "release binary missing" >&2; exit 1; }
cp -- "$binary" "$prepared/bench"
binary="$prepared/bench"

commit=$(git rev-parse HEAD)
tree=$(git rev-parse HEAD^{tree})
fixture=fixtures/session/v1/parametric-eq-bank-console.json
timed=tools/bench/src/input_symmetry_capture.rs
untimed=tools/bench/src/input_symmetry.rs
dispatcher=tools/bench/src/main.rs
fixture_hash=$(sha256sum "$fixture" | cut -d' ' -f1)
timed_hash=$(sha256sum "$timed" | cut -d' ' -f1)
untimed_hash=$(sha256sum "$untimed" | cut -d' ' -f1)
dispatcher_hash=$(sha256sum "$dispatcher" | cut -d' ' -f1)
source_hash=$(cat "$timed" "$untimed" "$dispatcher" | sha256sum | cut -d' ' -f1)
binary_hash=$(sha256sum "$binary" | cut -d' ' -f1)
lock_hash=$(sha256sum Cargo.lock | cut -d' ' -f1)
validator_hash=$(sha256sum scripts/input-symmetry-capture-validator.py | cut -d' ' -f1)
runner_hash=$(sha256sum scripts/run-input-symmetry-capture.sh | cut -d' ' -f1)
preflight_hash=$(sha256sum scripts/preflight-input-symmetry-capture.sh | cut -d' ' -f1)
target=$(rustc -vV | sed -n 's/^host: //p')
compiler=$(rustc --version)
flags='-C target-feature=+avx2,+fma -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort -C debug=1'
cwd="$repo"
export commit tree fixture_hash timed_hash untimed_hash dispatcher_hash source_hash binary_hash lock_hash validator_hash runner_hash preflight_hash target compiler flags cwd

trap 'if [[ -e "$seal.scratch" ]]; then unlink "$seal.scratch"; fi' EXIT
SEAL="$seal" python3 - <<'PY'
import json, os
seal = {
 "schema_version":1,"issue":603,"kind":"input_symmetry_capture_seal","status":"READY",
 "candidate_commit":os.environ["commit"],"candidate_tree":os.environ["tree"],
 "binary_sha256":os.environ["binary_hash"],"fixture_sha256":os.environ["fixture_hash"],"source_sha256":os.environ["source_hash"],
 "timed_source_sha256":os.environ["timed_hash"],"untimed_source_sha256":os.environ["untimed_hash"],
 "dispatcher_sha256":os.environ["dispatcher_hash"],"validator_sha256":os.environ["validator_hash"],
 "runner_sha256":os.environ["runner_hash"],"preflight_sha256":os.environ["preflight_hash"],
 "cargo_lock_sha256":os.environ["lock_hash"],"argv":os.environ["cwd"] + "/artifacts/issue-603-input-symmetry-capture/prepared-release/bench input-symmetry-capture","cwd":os.environ["cwd"],
 "compiler":os.environ["compiler"],"target_triple":os.environ["target"],"effective_build_flags":os.environ["flags"],
 "sample_rate_hz":48000,"quantum_frames":128,"lane_width":8,"track_count":8,"records_per_block":8,
 "preparation_blocks_per_owner":512,"measured_blocks_per_owner":4096,"expected_records":2,"expected_attempted_records":65536,
 "expected_renders_per_round":8192,"expected_rounds":2,"expected_timed_render_calls":16384,
 "expected_workload_processes":1,"target_pair_db":[-6.0,-12.0],"smoothing_samples":256,"owners":2,
}
with open(os.environ["SEAL"] + ".scratch", "x", encoding="utf-8") as handle:
    json.dump(seal, handle, sort_keys=True, separators=(",", ":")); handle.write("\n")
PY
python3 scripts/input-symmetry-capture-validator.py --seal-only "$seal.scratch" >/dev/null
if ! ln -- "$seal.scratch" "$seal"; then
  echo "seal publication failed; scratch retained at $seal.scratch" >&2
  exit 1
fi
rm -- "$seal.scratch"
trap - EXIT
echo "READY #603 preflight seal: $seal"
