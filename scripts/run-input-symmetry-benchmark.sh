#!/usr/bin/env bash
# Sole Issue #600 capture. It consumes one preflight seal and never retries or resumes it.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(cd "$script_directory/.." && pwd)
artifact_directory="$repository_root/artifacts/issue600-input-symmetry"
prepared_directory="$repository_root/target/issue600-input-symmetry"
binary="$prepared_directory/bench"
seal="$artifact_directory/input-symmetry-benchmark.preflight.json"
raw="$artifact_directory/input-symmetry-benchmark.raw.jsonl"
accepted="$artifact_directory/input-symmetry-benchmark.jsonl"
stdout_log="$artifact_directory/input-symmetry-benchmark.stdout"
stderr_log="$artifact_directory/input-symmetry-benchmark.stderr"
validator_stderr="$artifact_directory/input-symmetry-benchmark.validator.stderr"
disposition="$artifact_directory/input-symmetry-benchmark.disposition.json"
validator="$script_directory/input-symmetry-benchmark-validator.py"
runner="$script_directory/run-input-symmetry-benchmark.sh"
preflight="$script_directory/preflight-input-symmetry-benchmark.sh"
subject="$repository_root/tools/bench/src/input_symmetry.rs"
dispatcher="$repository_root/tools/bench/src/main.rs"
fixture="$repository_root/fixtures/session/v1/parametric-eq-bank-console.json"
lock_file="$repository_root/Cargo.lock"

fail() { printf 'Issue-600 input-symmetry runner failure: %s\n' "$1" >&2; exit 1; }
for tool in awk cp git ln mktemp mv python3 rm sha256sum stat uname wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool unavailable: $tool"
done
[[ -d "$artifact_directory" && ! -L "$artifact_directory" ]] || fail 'artifact namespace unavailable'
[[ -d "$prepared_directory" && ! -L "$prepared_directory" ]] || fail 'prepared namespace unavailable'
for path in "$seal" "$binary" "$validator" "$runner" "$preflight" "$subject" "$dispatcher" "$fixture" "$lock_file"; do
    [[ -f "$path" && ! -L "$path" ]] || fail "required input unavailable: $path"
done
for path in "$raw" "$accepted" "$stdout_log" "$stderr_log" "$validator_stderr" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "refusing protected output: $path"
done
scratch=$(mktemp -d "$artifact_directory/.run.XXXXXX")
cleanup() { rm -rf -- "$scratch"; }
trap cleanup EXIT
hash_file() { sha256sum "$1" | awk '{print $1}'; }
seal_field() { python3 - "$seal" "$1" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
field = sys.argv[2]
item = value.get(field)
if not isinstance(item, (str, int)):
    raise SystemExit(1)
print(item)
PY
}
candidate_commit=$(seal_field candidate_commit) || fail 'preflight seal candidate commit missing'
candidate_tree=$(seal_field candidate_tree) || fail 'preflight seal candidate tree missing'
[[ "$(git -C "$repository_root" rev-parse --verify HEAD)" == "$candidate_commit" ]] || fail 'candidate commit changed'
[[ "$(git -C "$repository_root" rev-parse 'HEAD^{tree}')" == "$candidate_tree" ]] || fail 'candidate tree changed'
[[ "$(hash_file "$binary")" == "$(seal_field binary_sha256)" ]] || fail 'binary identity mismatch'
[[ "$(hash_file "$fixture")" == "$(seal_field fixture_sha256)" ]] || fail 'fixture identity mismatch'
[[ "$(hash_file "$lock_file")" == "$(seal_field cargo_lock_sha256)" ]] || fail 'Cargo.lock identity mismatch'
source_sha256=$(sha256sum "$subject" "$dispatcher" | sha256sum | awk '{print $1}')
[[ "$source_sha256" == "$(seal_field source_sha256)" ]] || fail 'source identity mismatch'
[[ "$(hash_file "$validator")" == "$(seal_field validator_sha256)" ]] || fail 'validator identity mismatch'
[[ "$(hash_file "$runner")" == "$(seal_field runner_sha256)" ]] || fail 'runner identity mismatch'
[[ "$(hash_file "$preflight")" == "$(seal_field preflight_sha256)" ]] || fail 'preflight identity mismatch'
[[ "$(seal_field status)" == READY ]] || fail 'preflight seal is not READY'
[[ "$(set -o pipefail; python3 - "$seal" <<'PY'
import json, pathlib, sys
value = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
print('true' if value.get('records_required') == 2 and value.get('warmup_blocks') == 512 and value.get('measured_blocks') == 4096 else 'false')
PY
)" == true ]] || fail 'preflight workload contract mismatch'

if (set -o noclobber; : >"$disposition"); then :; else fail 'invocation reservation unavailable'; fi
publish_disposition() {
    local status=$1 reason=$2 runner_status=$3 child_status=${4:-null} validator_status=${5:-null}
    python3 - "$disposition" "$status" "$reason" "$runner_status" "$child_status" "$validator_status" "$raw" "$accepted" "$stdout_log" "$stderr_log" "$validator_stderr" "$candidate_commit" "$candidate_tree" <<'PY'
import hashlib, json, pathlib, sys
destination = pathlib.Path(sys.argv[1])
def artifact(path):
    item = pathlib.Path(path)
    if not item.is_file() or item.is_symlink():
        return None
    return {"sha256": hashlib.sha256(item.read_bytes()).hexdigest(), "bytes": item.stat().st_size}
def integer_or_null(value):
    return None if value == "null" else int(value)
value = {
    "schema_version": 1, "issue": 600, "kind": "input_symmetry_benchmark_disposition",
    "status": sys.argv[2], "reason": sys.argv[3], "runner_status": int(sys.argv[4]),
    "child_status": integer_or_null(sys.argv[5]), "validator_status": integer_or_null(sys.argv[6]),
    "preflight_invocations": 1, "runner_invocations": 1,
    "workload_invocations": 1 if sys.argv[5] != "null" else 0,
    "timed_benchmark_invocations": 1 if sys.argv[5] == "0" else 0,
    "candidate_commit": sys.argv[12], "candidate_tree": sys.argv[13],
    "raw_stdout": artifact(sys.argv[7]), "stdout": artifact(sys.argv[9]),
    "raw_stderr": artifact(sys.argv[10]),
    "validator_stderr": artifact(sys.argv[11]), "accepted": artifact(sys.argv[8]),
}
temporary = destination.with_name('.disposition.tmp')
temporary.write_text(json.dumps(value, sort_keys=True, separators=(',', ':')) + '\n', encoding='utf-8')
temporary.replace(destination)
PY
}

set +e
cpu_model=${MISO_ENGINE_BENCH_CPU_MODEL:-unknown}
metadata_missing='[]'
[[ "$cpu_model" == unknown ]] && metadata_missing='["cpu"]'
env \
    MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
    MISO_ENGINE_BENCH_CANDIDATE_TREE="$candidate_tree" \
    MISO_ENGINE_BENCH_BINARY_SHA256="$(seal_field binary_sha256)" \
    MISO_ENGINE_BENCH_FIXTURE_SHA256="$(seal_field fixture_sha256)" \
    MISO_ENGINE_BENCH_ARGV="$binary input-symmetry" \
    MISO_ENGINE_BENCH_CWD="$repository_root" \
    MISO_ENGINE_BENCH_RUST_VERSION="$(seal_field rust_version)" \
    MISO_ENGINE_BENCH_COMPILER="$(seal_field compiler)" \
    MISO_ENGINE_BENCH_TARGET_TRIPLE="$(seal_field target_triple)" \
    MISO_ENGINE_BENCH_BUILD_FLAGS="$(seal_field build_flags)" \
    MISO_ENGINE_BENCH_CPU_MODEL="$cpu_model" \
    MISO_ENGINE_BENCH_OS="${MISO_ENGINE_BENCH_OS:-$(uname -s 2>/dev/null || printf unknown)}" \
    MISO_ENGINE_BENCH_METADATA_MISSING="$metadata_missing" \
    "$binary" input-symmetry >"$scratch/stdout" 2>"$scratch/stderr"
child_status=$?
set -e
mv -n -- "$scratch/stdout" "$stdout_log"
mv -n -- "$scratch/stderr" "$stderr_log"
cp -- "$stdout_log" "$raw"
if ((child_status != 0)); then
    publish_disposition FAIL child_failed 1 "$child_status" null
    printf 'Issue-600 input-symmetry runner: child failed (status=%s)\n' "$child_status" >&2
    exit "$child_status"
fi
set +e
python3 -I -B "$validator" "$raw" "$seal" >"$scratch/validator.stdout" 2>"$scratch/validator.stderr"
validator_status=$?
set -e
mv -n -- "$scratch/validator.stderr" "$validator_stderr"
if ((validator_status != 0)); then
    publish_disposition FAIL validator_rejected 1 "$child_status" "$validator_status"
    cat "$validator_stderr" >&2
    exit 1
fi
temporary_accepted=$(mktemp "$artifact_directory/.accepted.XXXXXX")
cp -- "$raw" "$temporary_accepted"
ln -- "$temporary_accepted" "$accepted" || { rm -f -- "$temporary_accepted"; publish_disposition FAIL accepted_publication 1 "$child_status" 0; exit 1; }
rm -f -- "$temporary_accepted"
publish_disposition PASS accepted 0 "$child_status" 0
printf 'Issue-600 input-symmetry runner: PASS (runner/workload/timed=1/1/1)\n'
