#!/usr/bin/env bash
# Sole exactly-once Issue 108 benchmark entrypoint.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: run-effect-interchange-benchmark-108.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
cd "$root"

for tool in awk git ln mkdir mktemp python3 rm rustc sha256sum stat wc; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange benchmark runner: missing tool %s\n' "$tool" >&2
        exit 1
    }
done
artifact_dir="$root/target/issue108"
seal="$artifact_dir/benchmark-preflight.seal.json"
repair_seal="$artifact_dir/repair.seal.json"
binary="$artifact_dir/miso_engine_effect_interchange_bench"
raw="$artifact_dir/benchmark.raw.jsonl"
accepted="$artifact_dir/benchmark.accepted.jsonl"
disposition="$artifact_dir/benchmark.disposition.json"
prelaunch_disposition="$artifact_dir/benchmark.prelaunch.disposition.json"
stderr_log="$artifact_dir/benchmark.stderr.log"
mkdir -p "$artifact_dir"
[[ ! -L "$artifact_dir" ]] || { printf 'effect interchange benchmark runner: artifact directory symlink\n' >&2; exit 1; }
for terminal_artifact in "$disposition" "$prelaunch_disposition"; do
    [[ ! -e "$terminal_artifact" && ! -L "$terminal_artifact" ]] || {
        printf 'effect interchange benchmark runner: refusing existing artifact %s\n' \
            "$terminal_artifact" >&2
        exit 1
    }
done

scratch="$(mktemp -d "$artifact_dir/.benchmark-run.XXXXXX")"
trap 'rm -rf -- "$scratch"' EXIT
launch_attempted=0
workload_invocations=0
timed_invocations=0
warmup_passes=0
measured_rounds=0
failure_reason=prelaunch_failure
failed=1
commit=
tree=
binary_sha=
seal_sha=

artifact_json() {
    local path=$1
    if [[ -f "$path" && ! -L "$path" ]]; then
        printf '"%s",%s' "$(sha256sum "$path" | awk '{print $1}')" "$(wc -c <"$path")"
    else
        printf 'null,0'
    fi
}

refresh_phases() {
    workload_invocations=0
    timed_invocations=0
    warmup_passes=0
    measured_rounds=0
    if [[ -f "$stderr_log" && ! -L "$stderr_log" ]]; then
        workload_invocations="$(awk '$0 == "MISO_INTERCHANGE_BENCH_PHASE workload_started" {count++} END {print count + 0}' "$stderr_log")"
        timed_invocations="$(awk '$0 == "MISO_INTERCHANGE_BENCH_PHASE timed_started" {count++} END {print count + 0}' "$stderr_log")"
        warmup_passes="$(awk '$0 == "MISO_INTERCHANGE_BENCH_PHASE warmup_complete" {count++} END {print count + 0}' "$stderr_log")"
        measured_rounds="$(awk '$0 == "MISO_INTERCHANGE_BENCH_PHASE round_1_complete" || $0 == "MISO_INTERCHANGE_BENCH_PHASE round_2_complete" {count++} END {print count + 0}' "$stderr_log")"
    fi
}

publish_disposition() {
    local destination=$1 kind=$2 status=$3 reason=$4 raw_id accepted_id stderr_id
    raw_id="$(artifact_json "$raw")"
    accepted_id="$(artifact_json "$accepted")"
    stderr_id="$(artifact_json "$stderr_log")"
    local raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    IFS=, read -r raw_sha raw_bytes <<<"$raw_id"
    IFS=, read -r accepted_sha accepted_bytes <<<"$accepted_id"
    IFS=, read -r stderr_sha stderr_bytes <<<"$stderr_id"
    local commit_json=null tree_json=null binary_json=null seal_json=null
    [[ -z "$commit" ]] || commit_json="\"$commit\""
    [[ -z "$tree" ]] || tree_json="\"$tree\""
    [[ -z "$binary_sha" ]] || binary_json="\"$binary_sha\""
    [[ -z "$seal_sha" ]] || seal_json="\"$seal_sha\""
    local tmp="$scratch/disposition.json"
    printf '{"schema_version":1,"issue":108,"kind":"%s","status":"%s","reason":"%s","benchmark_runner_invocations":1,"benchmark_workload_invocations":%s,"timed_benchmark_invocations":%s,"warmup_passes_completed":%s,"measured_rounds_completed":%s,"candidate_commit":%s,"candidate_tree":%s,"binary_sha256":%s,"preflight_seal_sha256":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$kind" "$status" "$reason" "$workload_invocations" "$timed_invocations" \
        "$warmup_passes" "$measured_rounds" \
        "$commit_json" "$tree_json" "$binary_json" "$seal_json" \
        "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" "$stderr_sha" "$stderr_bytes" \
        >"$tmp"
    ln "$tmp" "$destination"
}

on_exit() {
    local status=$?
    trap - EXIT
    if [[ $failed == 1 ]]; then
        set +e
        refresh_phases
        if [[ $launch_attempted == 1 ]]; then
            if [[ ! -e "$disposition" && ! -L "$disposition" ]]; then
                publish_disposition "$disposition" effect_interchange_benchmark_disposition FAIL "$failure_reason"
            fi
        elif [[ ! -e "$prelaunch_disposition" && ! -L "$prelaunch_disposition" ]]; then
            publish_disposition "$prelaunch_disposition" effect_interchange_benchmark_prelaunch_disposition FAIL "$failure_reason"
        fi
    fi
    rm -rf -- "$scratch"
    exit "$status"
}
trap on_exit EXIT

for path in "$raw" "$accepted" "$stderr_log"; do
    [[ ! -e "$path" && ! -L "$path" ]] || {
        failure_reason=existing_output
        printf 'effect interchange benchmark runner: refusing existing artifact %s\n' "$path" >&2
        exit 1
    }
done

[[ -f "$repair_seal" && ! -L "$repair_seal" && "$(stat -c %h "$repair_seal")" == 1 && \
   -f "$seal" && ! -L "$seal" && "$(stat -c %h "$seal")" == 1 && \
   -f "$binary" && -x "$binary" && ! -L "$binary" && "$(stat -c %h "$binary")" == 1 ]] || {
    failure_reason=missing_preflight_artifact
    exit 1
}
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    failure_reason=dirty_candidate
    exit 1
}
branch="$(git branch --show-current)"
commit="$(git rev-parse --verify HEAD)"
tree="$(git rev-parse HEAD^{tree})"
binary_sha="$(sha256sum "$binary" | awk '{print $1}')"
seal_sha="$(sha256sum "$seal" | awk '{print $1}')"
source_sha="$(sha256sum tools/miso-engine-effect-interchange-bench/src/main.rs | awk '{print $1}')"
tool_manifest_sha="$(sha256sum tools/miso-engine-effect-interchange-bench/Cargo.toml | awk '{print $1}')"
fixture_sha="$(sha256sum fixtures/effect-interchange/v1/ACCEPTED.sha256 | awk '{print $1}')"
[[ "$fixture_sha" == 6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5 ]] || {
    failure_reason=fixture_manifest_changed
    exit 1
}
while read -r name bytes digest; do
    inherited="$root/target/issue081/$name"
    [[ -f "$inherited" && ! -L "$inherited" && "$(stat -c %h "$inherited")" == 1 && \
       "$(wc -c <"$inherited")" == "$bytes" && \
       "$(sha256sum "$inherited" | awk '{print $1}')" == "$digest" ]] || {
        failure_reason=inherited_evidence_changed
        exit 1
    }
done <<'EOF'
nonbenchmark.seal.json 833 6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c
miso_engine_effect_interchange_bench 827232 fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c
benchmark-preflight.seal.json 1577 da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3
benchmark.raw.jsonl 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
benchmark.stderr.log 361 442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93
benchmark.disposition.json 817 8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de
EOF
for name in benchmark.accepted.jsonl benchmark.prelaunch.disposition.json; do
    [[ ! -e "$root/target/issue081/$name" && ! -L "$root/target/issue081/$name" ]] || {
        failure_reason=inherited_evidence_changed
        exit 1
    }
done
lock_sha="$(sha256sum Cargo.lock | awk '{print $1}')"
validator_sha="$(sha256sum scripts/effect-interchange-benchmark-108-validator.py | awk '{print $1}')"
preflight_sha="$(sha256sum scripts/preflight-effect-interchange-benchmark-108.sh | awk '{print $1}')"
runner_sha="$(sha256sum scripts/run-effect-interchange-benchmark-108.sh | awk '{print $1}')"
lifecycle_sha="$(sha256sum scripts/test-effect-interchange-benchmark-108.sh | awk '{print $1}')"
checker_sha="$(sha256sum scripts/check-effect-interchange-benchmark-108.sh | awk '{print $1}')"
mutation_sha="$(sha256sum scripts/test-effect-interchange-benchmark-108-policy.sh | awk '{print $1}')"
repair_sha="$(sha256sum "$repair_seal" | awk '{print $1}')"
python3 -I -B - "$seal" "$branch" "$commit" "$tree" "$binary_sha" "$tool_manifest_sha" "$source_sha" "$fixture_sha" "$lock_sha" "$validator_sha" "$checker_sha" "$mutation_sha" "$preflight_sha" "$runner_sha" "$lifecycle_sha" "$repair_sha" <<'PY'
import json, pathlib, sys
actual = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
keys = {
    "schema_version", "issue", "kind", "branch", "candidate_commit", "candidate_tree",
    "binary_sha256", "tool_manifest_sha256", "tool_source_sha256", "fixture_manifest_sha256", "cargo_lock_sha256",
    "validator_sha256", "checker_sha256", "mutation_sha256", "preflight_sha256",
    "runner_sha256", "lifecycle_sha256", "repair_seal_sha256",
    "output_sha256",
    "preflight_invocations", "runner_invocations", "workload_invocations", "timed_benchmark_invocations",
    "warmup_passes", "measured_rounds", "records_required",
}
expected = {
    "schema_version": 1, "issue": 108, "kind": "effect_interchange_benchmark_preflight",
    "branch": sys.argv[2], "candidate_commit": sys.argv[3], "candidate_tree": sys.argv[4],
    "binary_sha256": sys.argv[5], "tool_manifest_sha256": sys.argv[6],
    "tool_source_sha256": sys.argv[7], "fixture_manifest_sha256": sys.argv[8],
    "cargo_lock_sha256": sys.argv[9], "validator_sha256": sys.argv[10],
    "checker_sha256": sys.argv[11], "mutation_sha256": sys.argv[12],
    "preflight_sha256": sys.argv[13], "runner_sha256": sys.argv[14],
    "lifecycle_sha256": sys.argv[15], "repair_seal_sha256": sys.argv[16],
    "output_sha256": {
        "descriptor_verify_identity_a": "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1",
        "package_verify_cid_select_a": "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f",
        "state_verify_reencode_current": "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48",
        "migration_two_step_bank_restore": "5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777",
    },
    "preflight_invocations": 1, "runner_invocations": 0, "workload_invocations": 0,
    "timed_benchmark_invocations": 0,
    "warmup_passes": 1, "measured_rounds": 2, "records_required": 8,
}
if set(actual) != keys or actual != expected:
    raise SystemExit("preflight seal mismatch")
PY

rust_version="$(rustc -V)"
llvm_version="$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')"
target_triple="$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')"
cpu_model="$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo 2>/dev/null || true)"
logical_cores=""
physical_cores=""
os=""
kernel=""
if command -v getconf >/dev/null 2>&1; then
    logical_cores="$(getconf _NPROCESSORS_ONLN 2>/dev/null || true)"
fi
if command -v lscpu >/dev/null 2>&1; then
    physical_cores="$(lscpu -p=CORE,SOCKET 2>/dev/null | awk -F, '!/^#/ {seen[$1 FS $2]=1} END {if (length(seen)) print length(seen)}' || true)"
fi
if command -v uname >/dev/null 2>&1; then
    os="$(uname -s 2>/dev/null || true)"
    kernel="$(uname -r 2>/dev/null || true)"
fi
power_mode=""
governor=""
[[ ! -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]] || governor="$(</sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)"
background_load=""
[[ ! -r /proc/loadavg ]] || background_load="$(awk '{print $1","$2","$3}' /proc/loadavg)"

expected="$scratch/expected.json"
printf '{"candidate_commit":"%s","candidate_tree":"%s","binary_sha256":"%s","tool_manifest_sha256":"%s","tool_source_sha256":"%s","fixture_manifest_sha256":"%s","output_sha256":{"descriptor_verify_identity_a":"865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1","package_verify_cid_select_a":"02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f","state_verify_reencode_current":"b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48","migration_two_step_bank_restore":"5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777"}}\n' \
    "$commit" "$tree" "$binary_sha" "$tool_manifest_sha" "$source_sha" "$fixture_sha" >"$expected"
umask 077
set -o noclobber
: >"$raw"
: >"$stderr_log"
failure_reason=workload_failed
launch_attempted=1
MISO_INTERCHANGE_CANDIDATE_COMMIT="$commit" \
MISO_INTERCHANGE_CANDIDATE_TREE="$tree" \
MISO_INTERCHANGE_BINARY_SHA256="$binary_sha" \
MISO_INTERCHANGE_TOOL_MANIFEST_SHA256="$tool_manifest_sha" \
MISO_INTERCHANGE_TOOL_SOURCE_SHA256="$source_sha" \
MISO_INTERCHANGE_FIXTURE_MANIFEST_SHA256="$fixture_sha" \
MISO_INTERCHANGE_RUST_VERSION="$rust_version" \
MISO_INTERCHANGE_LLVM_VERSION="$llvm_version" \
MISO_INTERCHANGE_TARGET_TRIPLE="$target_triple" \
MISO_INTERCHANGE_PROFILE=release \
CPU_MODEL="$cpu_model" LOGICAL_CORES="$logical_cores" PHYSICAL_CORES="$physical_cores" \
OS="$os" KERNEL="$kernel" POWER_MODE="$power_mode" GOVERNOR="$governor" \
BACKGROUND_LOAD="$background_load" \
    "$binary" >>"$raw" 2>>"$stderr_log" || exit 1
refresh_phases
failure_reason=phase_handshake_failed
python3 -I -B - "$stderr_log" <<'PY'
import pathlib, sys
expected = [
    "MISO_INTERCHANGE_BENCH_PHASE workload_started",
    "MISO_INTERCHANGE_BENCH_PHASE warmup_complete",
    "MISO_INTERCHANGE_BENCH_PHASE timed_started",
    "MISO_INTERCHANGE_BENCH_PHASE round_1_complete",
    "MISO_INTERCHANGE_BENCH_PHASE round_2_complete",
]
if pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").splitlines() != expected:
    raise SystemExit("benchmark phase handshake mismatch")
PY
failure_reason=validation_failed
python3 -I -B scripts/effect-interchange-benchmark-108-validator.py "$raw" "$expected" || exit 1
failure_reason=accepted_publication_failed
accepted_copy="$scratch/benchmark.accepted.jsonl"
python3 -I -B - "$raw" "$accepted_copy" <<'PY'
import os, pathlib, shutil, sys
source = pathlib.Path(sys.argv[1])
destination = pathlib.Path(sys.argv[2])
with source.open("rb") as input_stream, destination.open("xb") as output_stream:
    shutil.copyfileobj(input_stream, output_stream)
    output_stream.flush()
    os.fsync(output_stream.fileno())
PY
ln "$accepted_copy" "$accepted" || exit 1
refresh_phases
publish_disposition "$disposition" effect_interchange_benchmark_disposition PASS complete
failed=0
trap - EXIT
rm -rf -- "$scratch"
printf '%s\n' "$accepted"
