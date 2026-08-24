#!/usr/bin/env bash
# Sole exactly-once Issue-038 timing entrypoint. Do not invoke its binary directly.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

artifact_dir="$root/artifacts/issue038"
raw="$artifact_dir/rack-benchmark.raw.jsonl"
accepted="$artifact_dir/rack-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/rack-benchmark.stderr.log"
disposition="$artifact_dir/rack-benchmark.disposition.json"
for path in "$raw" "$accepted" "$stderr_log" "$disposition"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite issue-038 artifact: %s\n' "$path" >&2; exit 1; }
done
[[ "$(uname -m)" == "x86_64" ]] || { printf 'Issue-038 qualification requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'Issue-038 qualification requires AVX2\n' >&2; exit 1; }
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-038 qualification requires a clean committed candidate\n' >&2
    exit 1
}

umask 077
mkdir -p "$artifact_dir"
set -o noclobber
: >"$stderr_log"

failed=1
failure_reason=unexpected_failure
workload_process_launches=0
warmup_launches=0
measured_rounds_completed=0
candidate_commit=
candidate_commit_sha256=
binary_sha256=

artifact_identity() {
    local path=$1
    if [[ -e "$path" ]]; then
        printf '"%s" %s' "$(sha256sum "$path" | awk '{print $1}')" "$(wc -c <"$path")"
    else
        printf 'null 0'
    fi
}
write_disposition() {
    local status=$1 reason=$2 raw_identity accepted_identity stderr_identity
    raw_identity=$(artifact_identity "$raw")
    accepted_identity=$(artifact_identity "$accepted")
    stderr_identity=$(artifact_identity "$stderr_log")
    local raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    read -r raw_sha raw_bytes <<<"$raw_identity"
    read -r accepted_sha accepted_bytes <<<"$accepted_identity"
    read -r stderr_sha stderr_bytes <<<"$stderr_identity"
    local candidate_json=null candidate_sha_json=null binary_sha_json=null
    [[ -n "$candidate_commit" ]] && candidate_json="\"$candidate_commit\""
    [[ -n "$candidate_commit_sha256" ]] && candidate_sha_json="\"$candidate_commit_sha256\""
    [[ -n "$binary_sha256" ]] && binary_sha_json="\"$binary_sha256\""
    printf '{"schema_version":2,"issue":38,"status":"%s","reason":"%s","runner_invocations":1,"workload_process_launches":%s,"warmup_launches":%s,"measured_rounds_completed":%s,"candidate_commit":%s,"candidate_commit_sha256":%s,"binary_sha256":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$status" "$reason" "$workload_process_launches" "$warmup_launches" \
        "$measured_rounds_completed" "$candidate_json" "$candidate_sha_json" \
        "$binary_sha_json" "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" \
        "$stderr_sha" "$stderr_bytes" >"$disposition"
}
on_exit() {
    local status=$?
    trap - EXIT
    if [[ "$failed" == 1 && ! -e "$disposition" ]]; then
        set +e
        write_disposition FAIL "$failure_reason"
    fi
    exit "$status"
}
on_signal() {
    failure_reason=interrupted
    exit 130
}
trap on_exit EXIT
trap on_signal INT TERM

failure_reason=candidate_identity_failed
candidate_commit=$(git rev-parse --verify HEAD 2>>"$stderr_log")
candidate_commit_sha256=$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')

# Freeze the release profile so the recorded build metadata describes the binary actually run.
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3
export CARGO_PROFILE_RELEASE_LTO=false
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
failure_reason=build_failed
cargo build --locked --release --quiet -p miso-engine-bench 2>>"$stderr_log"
binary="$root/target/release/miso_engine_bench"
[[ -x "$binary" ]] || { failure_reason=missing_binary; exit 1; }
failure_reason=binary_identity_failed
binary_sha256=$(sha256sum "$binary" | awk '{print $1}')

failure_reason=metadata_failed
cpu_model=$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)
architecture=$(uname -m)
logical_cores=$(getconf _NPROCESSORS_ONLN)
physical_cores=$(lscpu -p=CORE,SOCKET | awk -F, '!/^#/ {seen[$1 FS $2]=1} END {print length(seen)}')
os=$(uname -s)
kernel=$(uname -r)
rust_version=$(rustc -V)
llvm_version=$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')
target_triple=$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')
target_features="runtime-avx2$(grep -qm1 ' fma ' /proc/cpuinfo && printf '%s' ',fma' || true);baseline"
background_load_note="not-controlled; pre-run loadavg $(awk '{print $1","$2","$3}' /proc/loadavg)"
governor_or_power_mode=
if [[ -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]]; then
    governor_or_power_mode=$(< /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
fi

run_round() {
    local round=$1
    workload_process_launches=$((workload_process_launches + 1))
    MISO_ENGINE_BENCH_ROUND="$round" \
    MISO_ENGINE_BENCH_CANDIDATE_SHA256="$candidate_commit_sha256" \
    MISO_ENGINE_BENCH_BINARY_SHA256="$binary_sha256" \
    MISO_ENGINE_BENCH_CPU_MODEL="$cpu_model" \
    MISO_ENGINE_BENCH_CPU_ARCHITECTURE="$architecture" \
    MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT="$logical_cores" \
    MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT="$physical_cores" \
    MISO_ENGINE_BENCH_OS="$os" \
    MISO_ENGINE_BENCH_KERNEL="$kernel" \
    MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" \
    MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
    MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="$target_features" \
    MISO_ENGINE_BENCH_PROFILE=release \
    MISO_ENGINE_BENCH_OPT_LEVEL=3 \
    MISO_ENGINE_BENCH_LTO=false \
    MISO_ENGINE_BENCH_CODEGEN_UNITS=16 \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="$background_load_note" \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="$governor_or_power_mode" \
    "$binary" rack
}

# One untimed warmup, then exactly the two frozen measured rounds. Raw stdout is append-only after
# its exclusive creation; failures preserve every byte emitted by the failed process.
failure_reason=warmup_failed
run_round warmup >/dev/null 2>>"$stderr_log" || exit 1
warmup_launches=1
failure_reason=round_1_failed
run_round 1 >"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=1
failure_reason=round_2_failed
run_round 2 >>"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=2
failure_reason=record_count
[[ "$(wc -l <"$raw")" == 6 ]] || exit 1
failure_reason=validation_failed
jq -s -e -L scripts -f scripts/rack-benchmark-validator.jq "$raw" >/dev/null || exit 1
failure_reason=accepted_promotion_failed
: >"$accepted"
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || exit 1
write_disposition PASS complete
failed=0
trap - EXIT INT TERM
printf '%s\n' "$accepted"
