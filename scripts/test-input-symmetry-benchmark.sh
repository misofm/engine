#!/usr/bin/env bash
# Hermetic Issue #600 validator/preflight/runner lifecycle. No real benchmark is launched here.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(cd "$script_directory/.." && pwd)
cd "$root"
python3 -I -B scripts/input-symmetry-benchmark-validator.py --self-test
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template/scripts" "$template/tools/bench/src" "$template/fixtures/session/v1" "$template/target" "$template/artifacts" "$template/bin"
cp scripts/input-symmetry-benchmark-validator.py scripts/preflight-input-symmetry-benchmark.sh \
    scripts/run-input-symmetry-benchmark.sh "$template/scripts/"
cp tools/bench/src/input_symmetry.rs tools/bench/src/main.rs "$template/tools/bench/src/"
cp fixtures/session/v1/parametric-eq-bank-console.json "$template/fixtures/session/v1/"
cp Cargo.lock "$template/Cargo.lock"
cat >"$template/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == -C ]]; then shift 2; fi
case "${1:-}" in
  status) exit 0 ;;
  rev-parse) [[ "${2:-}" == --verify ]] && printf '%040d\n' 1 || printf '%040d\n' 2 ;;
  *) exit 91 ;;
esac
EOF
cat >"$template/bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == -vV ]]; then
  printf '%s\n' 'rustc 1.99.0 (synthetic)' 'host: x86_64-unknown-linux-gnu' 'release: 1.99.0' 'LLVM version: 99.0.0'
else
  printf '%s\n' 'rustc 1.99.0 (synthetic)'
fi
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "${CARGO_ENCODED_RUSTFLAGS:-}" == '-Ctarget-feature=+avx2,+fma' ]] || exit 75
mkdir -p "$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release"
cp "$MISO_ENGINE_TEST_FAKE_BENCH" "$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/bench"
chmod 0755 "$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/bench"
EOF
cat >"$template/fake-bench.py" <<'PY'
#!/usr/bin/env python3
import json, os, pathlib, sys
if sys.argv[1:] != ['input-symmetry']:
    raise SystemExit(2)
pathlib.Path(os.environ['MISO_ENGINE_TEST_LAUNCH_LOG']).open('a', encoding='utf-8').write('child=1 timing=0\n')
if os.environ.get('MISO_ENGINE_TEST_CHILD_MODE') == 'fail':
    print('synthetic child failure', file=sys.stderr)
    raise SystemExit(71)
for phase in ('workload_started', 'warmup_complete', 'timed_started', 'round_1_complete', 'round_2_complete'):
    print('MISO_ENGINE_BENCH_PHASE ' + phase, file=sys.stderr)
record = {
 'schema_version': 1, 'issue': 600, 'record': 'input_symmetry', 'round': 1, 'sample_rate_hz': 48000,
 'quantum_frames': 128, 'lane_width': 8, 'track_count': 8, 'records_per_block': 8,
 'target_pair_db': [-6.0, -12.0], 'smoothing_samples': 256, 'warmup_blocks': 512,
 'measured_blocks': 4096, 'attempted_records': 32768, 'accepted_records': 32768,
 'rendered_blocks': 4096, 'render_errors': 0, 'output_words': 1048576,
 'output_sha256': '1' * 64, 'owner_digests': ['1' * 64, '1' * 64],
 'owner_attempted_records': [32768, 32768], 'owner_accepted_records': [32768, 32768],
 'owner_rendered_blocks': [4096, 4096], 'output_witness_sha256': '', 'traffic_witness_sha256': '',
 'elapsed_ns': 409600, 'nanoseconds_per_block': 100,
 'backend': 'Simd8', 'source_commit': os.environ['MISO_ENGINE_BENCH_CANDIDATE_COMMIT'],
 'source_tree': os.environ['MISO_ENGINE_BENCH_CANDIDATE_TREE'],
 'binary_sha256': os.environ['MISO_ENGINE_BENCH_BINARY_SHA256'],
 'fixture_sha256': os.environ['MISO_ENGINE_BENCH_FIXTURE_SHA256'],
 'fixture_id': 'fixtures/session/v1/parametric-eq-bank-console.json',
 'argv': os.environ['MISO_ENGINE_BENCH_ARGV'], 'cwd': os.environ['MISO_ENGINE_BENCH_CWD'],
 'rust_version': os.environ['MISO_ENGINE_BENCH_RUST_VERSION'],
 'compiler': os.environ['MISO_ENGINE_BENCH_COMPILER'],
 'target_triple': os.environ['MISO_ENGINE_BENCH_TARGET_TRIPLE'],
 'build_flags': os.environ['MISO_ENGINE_BENCH_BUILD_FLAGS'], 'cpu': 'unknown', 'os': 'linux',
 'metadata_missing': ['cpu'], 'descriptive_only': True,
}
import hashlib
record['output_witness_sha256'] = hashlib.sha256(f"issue600-output-v1|{'1' * 64}|{'1' * 64}|1".encode()).hexdigest()
record['traffic_witness_sha256'] = hashlib.sha256(b'issue600-traffic-v1|1|48000|128|8|8|256|512|4096|32768|32768|4096|0|1048576').hexdigest()
if os.environ.get('MISO_ENGINE_TEST_CHILD_MODE') == 'invalid':
    record.pop('accepted_records')
print(json.dumps(record, sort_keys=True))
record['round'] = 2
record['output_witness_sha256'] = hashlib.sha256(f"issue600-output-v1|{'1' * 64}|{'1' * 64}|2".encode()).hexdigest()
record['traffic_witness_sha256'] = hashlib.sha256(b'issue600-traffic-v1|2|48000|128|8|8|256|512|4096|32768|32768|4096|0|1048576').hexdigest()
print(json.dumps(record, sort_keys=True))
PY
chmod 0755 "$template/bin/git" "$template/bin/rustc" "$template/bin/cargo" "$template/fake-bench.py"

case_root="$scratch/case"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
set +e
(cd / && PATH="$case_root/bin:$PATH" bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" bad >/dev/null 2>&1)
invalid_status=$?
set -e
[[ "$invalid_status" == 2 ]] || { printf 'preflight invalid-argument test failed\n' >&2; exit 1; }
set +e
CARGO_PROFILE_RELEASE_LTO=inherited PATH="$case_root/bin:$PATH" bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null 2>&1
profile_status=$?
set -e
[[ "$profile_status" == 1 ]] || { printf 'inherited profile override escaped\n' >&2; exit 1; }
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
seal="$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.preflight.json"
raw="$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.raw.jsonl"
accepted="$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.jsonl"
disposition="$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.disposition.json"
run_runner() {
  MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" MISO_ENGINE_TEST_CHILD_MODE="${1:-success}" \
      PATH="$case_root/bin:$PATH" bash "$case_root/scripts/run-input-symmetry-benchmark.sh"
}
run_runner success >/dev/null
[[ -f "$raw" && -f "$accepted" && -f "$disposition" ]]
cmp -s "$raw" "$accepted"
python3 - "$disposition" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
assert value['status'] == 'PASS' and value['child_status'] == 0 and value['validator_status'] == 0
assert value['workload_invocations'] == 1 and value['timed_benchmark_invocations'] == 1
PY
[[ "$(wc -l <"$launch_log" | tr -d ' ')" == 1 ]]
if run_runner success >/dev/null 2>&1; then
  printf 'runner overwrite refusal escaped\n' >&2
  exit 1
fi

case_root="$scratch/case-failure"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" MISO_ENGINE_TEST_CHILD_MODE=fail PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
failure_status=$?
set -e
[[ "$failure_status" == 71 ]] || { printf 'child failure propagation escaped\n' >&2; exit 1; }
[[ -f "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.raw.jsonl" ]]
[[ ! -e "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.jsonl" ]]
python3 - "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.disposition.json" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
assert value['status'] == 'FAIL' and value['reason'] == 'child_failed'
assert value['child_status'] == 71 and value['runner_status'] == 71
assert value['workload_invocations'] == 1 and value['timed_benchmark_invocations'] == 0
PY
[[ "$(wc -l <"$launch_log" | tr -d ' ')" == 1 ]]

case_root="$scratch/case-invalid"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" MISO_ENGINE_TEST_CHILD_MODE=invalid PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
invalid_status=$?
set -e
[[ "$invalid_status" == 1 ]] || { printf 'invalid-schema rejection escaped\n' >&2; exit 1; }
[[ -f "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.raw.jsonl" ]]
[[ -f "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.validator.stderr" ]]
[[ ! -e "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.jsonl" ]]
python3 - "$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.disposition.json" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
assert value['status'] == 'FAIL' and value['reason'] == 'validator_rejected'
assert value['child_status'] == 0 and value['runner_status'] == 1 and value['validator_status'] == 1
assert value['workload_invocations'] == 1 and value['timed_benchmark_invocations'] == 1
PY
[[ "$(wc -l <"$launch_log" | tr -d ' ')" == 1 ]]

case_root="$scratch/case-identity"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
printf 'fixture mutation\n' >>"$case_root/fixtures/session/v1/parametric-eq-bank-console.json"
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
identity_status=$?
set -e
[[ "$identity_status" == 1 ]] || { printf 'fixture identity mismatch escaped\n' >&2; exit 1; }
[[ ! -e "$launch_log" ]]

case_root="$scratch/case-source"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
printf 'source mutation\n' >>"$case_root/tools/bench/src/input_symmetry.rs"
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
source_status=$?
set -e
[[ "$source_status" == 1 && ! -e "$launch_log" ]] || { printf 'source identity mismatch escaped\n' >&2; exit 1; }

case_root="$scratch/case-binary"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
printf 'binary mutation\n' >>"$case_root/target/issue600-input-symmetry/bench"
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
binary_status=$?
set -e
[[ "$binary_status" == 1 && ! -e "$launch_log" ]] || { printf 'binary identity mismatch escaped\n' >&2; exit 1; }

case_root="$scratch/case-persistence"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
cat >"$case_root/bin/cp" <<'EOF'
#!/usr/bin/env bash
for argument in "$@"; do
  if [[ "${MISO_ENGINE_TEST_PERSIST_FAIL:-0}" == 1 && "$argument" == *raw.jsonl ]]; then exit 73; fi
done
exec /usr/bin/cp "$@"
EOF
chmod 0755 "$case_root/bin/cp"
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" MISO_ENGINE_TEST_PERSIST_FAIL=1 PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
persistence_status=$?
set -e
persist_disposition="$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.disposition.json"
[[ "$persistence_status" == 1 ]]
python3 - "$persist_disposition" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
assert value['status'] == 'FAIL' and value['reason'] == 'post_workload_persistence'
assert value['child_status'] == 0 and value['runner_status'] == 1
assert value['preserved_scratch'] is not None and pathlib.Path(value['preserved_scratch']).is_dir()
PY
[[ "$(wc -l <"$launch_log" | tr -d ' ')" == 1 ]]

case_root="$scratch/case-publication"
cp -a "$template" "$case_root"
launch_log="$case_root/launch.log"
PATH="$case_root/bin:$PATH" MISO_ENGINE_TEST_FAKE_BENCH="$case_root/fake-bench.py" \
    bash "$case_root/scripts/preflight-input-symmetry-benchmark.sh" >/dev/null
cat >"$case_root/bin/ln" <<'EOF'
#!/usr/bin/env bash
if [[ "${MISO_ENGINE_TEST_PUBLICATION_FAIL:-0}" == 1 ]]; then exit 74; fi
exec /usr/bin/ln "$@"
EOF
chmod 0755 "$case_root/bin/ln"
set +e
MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" MISO_ENGINE_TEST_PUBLICATION_FAIL=1 PATH="$case_root/bin:$PATH" \
    bash "$case_root/scripts/run-input-symmetry-benchmark.sh" >/dev/null 2>&1
publication_status=$?
set -e
publication_disposition="$case_root/artifacts/issue600-input-symmetry/input-symmetry-benchmark.disposition.json"
[[ "$publication_status" == 1 ]]
python3 - "$publication_disposition" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
assert value['status'] == 'FAIL' and value['reason'] == 'accepted_publication'
assert value['child_status'] == 0 and value['runner_status'] == 1
assert value['raw_stdout'] is not None and value['accepted'] is None
assert value['preserved_scratch'] is not None and pathlib.Path(value['preserved_scratch']).is_dir()
PY
[[ "$(wc -l <"$launch_log" | tr -d ' ')" == 1 ]]
printf 'Issue-600 input-symmetry lifecycle self-test passed (real_subject/workload/timing=0/0/0; synthetic_child=5)\n'
