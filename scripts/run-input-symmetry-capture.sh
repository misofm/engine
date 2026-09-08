#!/usr/bin/env bash
set -euo pipefail

if (($#)); then echo "usage: $0" >&2; exit 2; fi
repo=$(git rev-parse --show-toplevel)
cd "$repo"
if [[ -n "${MISO_ENGINE_603_ROOT-}" && "${MISO_ENGINE_603_SELF_TEST-}" != 1 ]]; then
  echo "artifact-root override is self-test-only" >&2; exit 2
fi
self_test_fault=
if [[ "${MISO_ENGINE_603_SELF_TEST-}" == 1 ]]; then self_test_fault="${MISO_ENGINE_603_SELF_TEST_FAULT-}"; fi
root=${MISO_ENGINE_603_ROOT:-"$repo/artifacts/issue-603-input-symmetry-capture"}
seal="$root/input-symmetry-capture.seal.json"
prepared="$root/prepared-release/bench"
raw="$root/raw"
accepted="$root/input-symmetry-capture.jsonl"
disposition="$root/disposition.json"
[[ -f "$seal" && -x "$prepared" && ! -L "$seal" && ! -L "$prepared" ]] || { echo "READY seal and prepared binary required" >&2; exit 1; }
[[ ! -e "$accepted" && ! -L "$accepted" && ! -e "$disposition" && ! -L "$disposition" && ! -e "$raw" && ! -L "$raw" && ! -L "$root" ]] || { echo "protected outputs already reserved" >&2; exit 1; }
reservation="$root/.capture-reservation"
[[ ! -e "$reservation" && ! -L "$reservation" ]] || { echo "capture reservation already exists" >&2; exit 1; }
mkdir "$raw" "$reservation"
: >"$reservation/accepted.jsonl"; : >"$reservation/disposition.json"
write_prelaunch_failure() {
  python3 - "$disposition.tmp" <<'PY'
import json, sys
with open(sys.argv[1], "x", encoding="utf-8") as handle:
    json.dump({"schema_version":1,"issue":603,"runner_status":"FAIL","child_status":"not_run","validator_status":"not_run","accepted_status":"not_attempted","workload_process_invocations":0,"capture_started_markers":0,"round_completion_markers":0,"timed_render_calls":0,"recovery_path":"raw"}, handle, separators=(",", ":")); handle.write("\n")
PY
  if ! ln -- "$disposition.tmp" "$disposition"; then
    echo "FAIL: prelaunch disposition publication failed; recovery retained at $raw" >&2
    return 1
  fi
  rm -- "$disposition.tmp"
}
if ! python3 scripts/input-symmetry-capture-validator.py --seal-only "$seal" >"$raw/validator.stdout" 2>"$raw/validator.stderr"; then
  write_prelaunch_failure; echo "FAIL: invalid prelaunch seal; evidence retained at $raw" >&2; exit 1
fi
seal_get() { python3 - "$seal" "$1" <<'PY'
import json, sys
s=json.load(open(sys.argv[1], encoding='utf-8'))
print(s[sys.argv[2]])
PY
}
candidate_commit=$(seal_get candidate_commit); candidate_tree=$(seal_get candidate_tree)
binary_sha256=$(seal_get binary_sha256); fixture_sha256=$(seal_get fixture_sha256)
source_sha256=$(seal_get source_sha256); argv=$(seal_get argv); cwd=$(seal_get cwd)
compiler=$(seal_get compiler); target_triple=$(seal_get target_triple); effective_build_flags=$(seal_get effective_build_flags)
timed_hash=$(seal_get timed_source_sha256); untimed_hash=$(seal_get untimed_source_sha256); dispatcher_hash=$(seal_get dispatcher_sha256)
validator_file_hash=$(seal_get validator_sha256); runner_file_hash=$(seal_get runner_sha256); preflight_file_hash=$(seal_get preflight_sha256); lock_hash=$(seal_get cargo_lock_sha256)
if ! {
  [[ "$argv" == "$prepared input-symmetry-capture" && "$cwd" == "$repo" ]] &&
  [[ "$compiler" == "$(rustc --version)" && "$target_triple" == "$(rustc -vV | sed -n 's/^host: //p')" ]] &&
  [[ "$effective_build_flags" == "-C target-feature=+avx2,+fma -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort -C debug=1" ]] &&
  [[ "$(git rev-parse HEAD)" == "$candidate_commit" && "$(git rev-parse HEAD^{tree})" == "$candidate_tree" ]] &&
  [[ "$(sha256sum "$prepared" | cut -d' ' -f1)" == "$binary_sha256" ]] &&
  [[ "$(sha256sum fixtures/session/v1/parametric-eq-bank-console.json | cut -d' ' -f1)" == "$fixture_sha256" ]] &&
  [[ "$(sha256sum tools/bench/src/input_symmetry_capture.rs | cut -d' ' -f1)" == "$timed_hash" ]] &&
  [[ "$(sha256sum tools/bench/src/input_symmetry.rs | cut -d' ' -f1)" == "$untimed_hash" ]] &&
  [[ "$(sha256sum tools/bench/src/main.rs | cut -d' ' -f1)" == "$dispatcher_hash" ]] &&
  [[ "$(sha256sum scripts/input-symmetry-capture-validator.py | cut -d' ' -f1)" == "$validator_file_hash" ]] &&
  [[ "$(sha256sum scripts/run-input-symmetry-capture.sh | cut -d' ' -f1)" == "$runner_file_hash" ]] &&
  [[ "$(sha256sum scripts/preflight-input-symmetry-capture.sh | cut -d' ' -f1)" == "$preflight_file_hash" ]] &&
  [[ "$(sha256sum Cargo.lock | cut -d' ' -f1)" == "$lock_hash" ]] &&
  [[ "$(cat tools/bench/src/input_symmetry_capture.rs tools/bench/src/input_symmetry.rs tools/bench/src/main.rs | sha256sum | cut -d' ' -f1)" == "$source_sha256" ]]
}; then
  write_prelaunch_failure; echo "FAIL: candidate identity changed; evidence retained at $raw" >&2; exit 1
fi
export MISO_ENGINE_CAPTURE_COMMIT="$candidate_commit"
export MISO_ENGINE_CAPTURE_TREE="$candidate_tree"
export MISO_ENGINE_CAPTURE_BINARY_SHA256="$binary_sha256"
export MISO_ENGINE_CAPTURE_FIXTURE_SHA256="$fixture_sha256"
export MISO_ENGINE_CAPTURE_SOURCE_SHA256="$source_sha256"
export MISO_ENGINE_CAPTURE_COMPILER="$compiler"
export MISO_ENGINE_CAPTURE_TARGET="$target_triple"
export MISO_ENGINE_CAPTURE_FLAGS="$effective_build_flags"
export MISO_ENGINE_CAPTURE_ARGV="$argv"
export MISO_ENGINE_CAPTURE_CWD="$cwd"
export MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple"
export MISO_ENGINE_BENCH_RUST_VERSION="$compiler"
export MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$candidate_commit"
export MISO_ENGINE_BENCH_PROFILE=release
export MISO_ENGINE_BENCH_TARGET_FEATURES="$effective_build_flags"

set +e
(cd "$cwd" && "$prepared" input-symmetry-capture >"$raw/stdout.jsonl" 2>"$raw/stderr.log")
child_status=$?
set -e
stdout_hash=$(sha256sum "$raw/stdout.jsonl" | cut -d' ' -f1)
stderr_hash=$(sha256sum "$raw/stderr.log" | cut -d' ' -f1)
stdout_bytes=$(wc -c <"$raw/stdout.jsonl")
stderr_bytes=$(wc -c <"$raw/stderr.log")
validator_status=not_run
accepted_status=not_attempted
marker_status=not_run
started=0
rounds=0
timed_calls=0
if [[ -f "$raw/stderr.log" ]]; then
  IFS=$'\t' read -r marker_status started rounds timed_calls < <(python3 - "$raw/stderr.log" <<'PY'
import sys
lines = [line.rstrip("\n") for line in open(sys.argv[1], encoding="utf-8")]
markers = [line for line in lines if line.startswith("MISO_ENGINE_CAPTURE_PHASE")]
expected = ["MISO_ENGINE_CAPTURE_PHASE capture_started", "MISO_ENGINE_CAPTURE_PHASE round_1_complete", "MISO_ENGINE_CAPTURE_PHASE round_2_complete"]
prefix = 0
for actual, wanted in zip(markers, expected):
    if actual != wanted:
        break
    prefix += 1
started = sum(line == expected[0] for line in markers)
rounds = sum(line in expected[1:] for line in markers)
status = "PASS" if markers == expected else "FAIL"
print(status, started, rounds, max(0, prefix - 1) * 8192, sep="\t")
PY
  )
fi
if ((child_status == 0)); then
  set +e
  python3 scripts/input-symmetry-capture-validator.py "$seal" "$raw/stdout.jsonl" >"$raw/validator.stdout" 2>"$raw/validator.stderr"
  validator_status=$?
  set -e
  if ((validator_status == 0)) && [[ "$marker_status" == PASS ]]; then
    if [[ "$self_test_fault" == publication ]]; then
      accepted_status=publication_failed
    else
      set +e
      ln -- "$raw/stdout.jsonl" "$accepted"
      publish_status=$?
      set -e
      if ((publish_status == 0)); then accepted_status=published; else accepted_status=publication_failed; fi
    fi
  else
    accepted_status=rejected
  fi
fi
validator_hash_value=missing; validator_bytes=0; validator_err_hash=missing; validator_err_bytes=0
if [[ -f "$raw/validator.stdout" ]]; then validator_hash_value=$(sha256sum "$raw/validator.stdout" | cut -d' ' -f1); validator_bytes=$(wc -c <"$raw/validator.stdout"); fi
if [[ -f "$raw/validator.stderr" ]]; then validator_err_hash=$(sha256sum "$raw/validator.stderr" | cut -d' ' -f1); validator_err_bytes=$(wc -c <"$raw/validator.stderr"); fi
accepted_hash=missing; accepted_bytes=0
if [[ -f "$accepted" ]]; then accepted_hash=$(sha256sum "$accepted" | cut -d' ' -f1); accepted_bytes=$(wc -c <"$accepted"); fi
python3 - "$disposition.tmp" "$child_status" "$validator_status" "$accepted_status" "$stdout_hash" "$stderr_hash" "$stdout_bytes" "$stderr_bytes" "$validator_hash_value" "$validator_bytes" "$validator_err_hash" "$validator_err_bytes" "$accepted_hash" "$accepted_bytes" "$started" "$rounds" "$timed_calls" "$marker_status" <<'PY'
import json, sys
path, child, validator, accepted, sh, eh, sb, eb, vh, vb, veh, ve_bytes, ah, ab, started, rounds, calls, markers = sys.argv[1:]
record={"schema_version":1,"issue":603,"runner_status":"PASS" if child == "0" and validator == "0" and accepted == "published" and markers == "PASS" else "FAIL",
 "child_status":int(child),"validator_status":validator,"accepted_status":accepted,"workload_process_invocations":1,
 "capture_started_markers":int(started),"round_completion_markers":int(rounds),"timed_render_calls":int(calls),
 "marker_status":markers,
 "raw_stdout_sha256":sh,"raw_stderr_sha256":eh,"validator_stdout_sha256":vh,"validator_stderr_sha256":veh,"accepted_output_sha256":ah,
 "raw_stdout_bytes":int(sb),"raw_stderr_bytes":int(eb),"validator_stdout_bytes":int(vb),"validator_stderr_bytes":int(ve_bytes),"accepted_output_bytes":int(ab),"recovery_path":"raw"}
with open(path,"x",encoding="utf-8") as f: json.dump(record,f,sort_keys=True,separators=(",",":")); f.write("\n")
PY
if [[ "$self_test_fault" == persistence ]]; then
  echo "injected disposition persistence failure" >"$disposition.tmp"
  echo "FAIL: disposition persistence failed; recovery retained at $raw" >&2
  exit 1
fi
if ! ln -- "$disposition.tmp" "$disposition"; then
  echo "FAIL: disposition publication failed; recovery retained at $raw" >&2
  exit 1
fi
rm -- "$disposition.tmp"
if [[ "$accepted_status" == published ]]; then echo "PASS: accepted capture published"; else echo "FAIL: capture retained at $raw" >&2; exit 1; fi
