#!/usr/bin/env bash
# Sole exactly-once Issue 081 benchmark entrypoint.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: run-effect-interchange-benchmark.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
cd "$root"

for tool in awk git ln mkdir mktemp python3 rm rustc sha256sum wc; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange benchmark runner: missing tool %s\n' "$tool" >&2
        exit 1
    }
done
artifact_dir="$root/target/issue081"
seal="$artifact_dir/benchmark-preflight.seal.json"
binary="$artifact_dir/miso_engine_bench"
raw="$artifact_dir/benchmark.raw.jsonl"
accepted="$artifact_dir/benchmark.accepted.jsonl"
disposition="$artifact_dir/benchmark.disposition.json"
prelaunch_disposition="$artifact_dir/benchmark.prelaunch.disposition.json"
stderr_log="$artifact_dir/benchmark.stderr.log"
mkdir -p "$artifact_dir"
[[ ! -L "$artifact_dir" ]] || { printf 'effect interchange benchmark runner: artifact directory symlink\n' >&2; exit 1; }
[[ ! -e "$disposition" && ! -L "$disposition" ]] || {
    printf 'effect interchange benchmark runner: refusing existing artifact %s\n' "$disposition" >&2
    exit 1
}

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
        workload_invocations="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE workload_started" {count++} END {print count + 0}' "$stderr_log")"
        timed_invocations="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE timed_started" {count++} END {print count + 0}' "$stderr_log")"
        warmup_passes="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE warmup_complete" {count++} END {print count + 0}' "$stderr_log")"
        measured_rounds="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE round_1_complete" || $0 == "MISO_ENGINE_BENCH_PHASE round_2_complete" {count++} END {print count + 0}' "$stderr_log")"
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
    printf '{"schema_version":1,"issue":81,"kind":"%s","status":"%s","reason":"%s","benchmark_runner_invocations":1,"benchmark_workload_invocations":%s,"timed_benchmark_invocations":%s,"warmup_passes_completed":%s,"measured_rounds_completed":%s,"candidate_commit":%s,"candidate_tree":%s,"binary_sha256":%s,"preflight_seal_sha256":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
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

[[ -f "$seal" && ! -L "$seal" && -f "$binary" && -x "$binary" && ! -L "$binary" ]] || {
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
source_sha="$(sha256sum tools/miso-engine-bench/src/effect_interchange.rs | awk '{print $1}')"
tool_manifest_sha="$(sha256sum tools/miso-engine-bench/Cargo.toml | awk '{print $1}')"
fixture_sha="$(sha256sum fixtures/effect-interchange/v1/ACCEPTED.sha256 | awk '{print $1}')"
[[ "$fixture_sha" == e3896726979aa746cfda50fc10c1985c0ecef117f87b39e692f18226b7b4fa14 ]] || {
    failure_reason=fixture_manifest_changed
    exit 1
}
lock_sha="$(sha256sum Cargo.lock | awk '{print $1}')"
validator_sha="$(sha256sum scripts/effect-interchange-benchmark-validator.py | awk '{print $1}')"
preflight_sha="$(sha256sum scripts/preflight-effect-interchange-benchmark.sh | awk '{print $1}')"
runner_sha="$(sha256sum scripts/run-effect-interchange-benchmark.sh | awk '{print $1}')"
lifecycle_sha="$(sha256sum scripts/test-effect-interchange-benchmark.sh | awk '{print $1}')"
python3 -I -B - "$seal" "$branch" "$commit" "$tree" "$binary_sha" "$tool_manifest_sha" "$source_sha" "$fixture_sha" "$lock_sha" "$validator_sha" "$preflight_sha" "$runner_sha" "$lifecycle_sha" <<'PY'
import json, pathlib, sys
actual = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
keys = {
    "schema_version", "issue", "kind", "branch", "candidate_commit", "candidate_tree",
    "binary_sha256", "tool_manifest_sha256", "tool_source_sha256", "fixture_manifest_sha256", "cargo_lock_sha256",
    "validator_sha256", "preflight_sha256", "runner_sha256", "lifecycle_sha256",
    "output_sha256",
    "runner_invocations", "workload_invocations", "timed_benchmark_invocations",
    "warmup_passes", "measured_rounds", "records_required",
}
expected = {
    "schema_version": 1, "issue": 81, "kind": "effect_interchange_benchmark_preflight",
    "branch": sys.argv[2], "candidate_commit": sys.argv[3], "candidate_tree": sys.argv[4],
    "binary_sha256": sys.argv[5], "tool_manifest_sha256": sys.argv[6],
    "tool_source_sha256": sys.argv[7], "fixture_manifest_sha256": sys.argv[8],
    "cargo_lock_sha256": sys.argv[9], "validator_sha256": sys.argv[10],
    "preflight_sha256": sys.argv[11], "runner_sha256": sys.argv[12], "lifecycle_sha256": sys.argv[13],
    "output_sha256": {
        "descriptor_verify_identity_a": "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1",
        "package_verify_cid_select_a": "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f",
        "state_verify_reencode_current": "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48",
        "migration_two_step_bank_restore": "350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441",
    },
    "runner_invocations": 0, "workload_invocations": 0, "timed_benchmark_invocations": 0,
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
printf '{"candidate_commit":"%s","candidate_tree":"%s","binary_sha256":"%s","tool_manifest_sha256":"%s","tool_source_sha256":"%s","fixture_manifest_sha256":"%s","output_sha256":{"descriptor_verify_identity_a":"865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1","package_verify_cid_select_a":"02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f","state_verify_reencode_current":"b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48","migration_two_step_bank_restore":"350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441"}}\n' \
    "$commit" "$tree" "$binary_sha" "$tool_manifest_sha" "$source_sha" "$fixture_sha" >"$expected"
umask 077
set -o noclobber
: >"$raw"
: >"$stderr_log"
failure_reason=workload_failed
launch_attempted=1
MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$commit" \
MISO_ENGINE_BENCH_CANDIDATE_TREE="$tree" \
MISO_ENGINE_BENCH_BINARY_SHA256="$binary_sha" \
MISO_ENGINE_BENCH_TOOL_MANIFEST_SHA256="$tool_manifest_sha" \
MISO_ENGINE_BENCH_TOOL_SOURCE_SHA256="$source_sha" \
MISO_ENGINE_BENCH_FIXTURE_MANIFEST_SHA256="$fixture_sha" \
MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" \
MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple" \
MISO_ENGINE_BENCH_PROFILE=release \
CPU_MODEL="$cpu_model" LOGICAL_CORES="$logical_cores" PHYSICAL_CORES="$physical_cores" \
OS="$os" KERNEL="$kernel" POWER_MODE="$power_mode" GOVERNOR="$governor" \
BACKGROUND_LOAD="$background_load" \
    "$binary" effect-interchange >>"$raw" 2>>"$stderr_log" || exit 1
refresh_phases
failure_reason=phase_handshake_failed
python3 -I -B - "$stderr_log" <<'PY'
import pathlib, sys
expected = [
    "MISO_ENGINE_BENCH_PHASE workload_started",
    "MISO_ENGINE_BENCH_PHASE warmup_complete",
    "MISO_ENGINE_BENCH_PHASE timed_started",
    "MISO_ENGINE_BENCH_PHASE round_1_complete",
    "MISO_ENGINE_BENCH_PHASE round_2_complete",
]
if pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").splitlines() != expected:
    raise SystemExit("benchmark phase handshake mismatch")
PY
failure_reason=validation_failed
python3 -I -B scripts/effect-interchange-benchmark-validator.py "$raw" "$expected" || exit 1
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
