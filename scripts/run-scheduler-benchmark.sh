#!/usr/bin/env bash
# Sole exactly-once Issue-009 timing entrypoint. Do not invoke the benchmark binary directly.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
artifact_dir="$root/artifacts/issue009"
raw="$artifact_dir/scheduler-benchmark.raw.jsonl"
accepted="$artifact_dir/scheduler-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/scheduler-benchmark.stderr.log"
disposition="$artifact_dir/scheduler-benchmark.disposition.json"
for path in "$raw" "$accepted" "$stderr_log" "$disposition"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite issue-009 artifact: %s\n' "$path" >&2; exit 1; }
done
[[ "$(uname -s)-$(uname -m)" == "Linux-x86_64" ]] || { printf 'Issue-009 timing requires Linux x86_64\n' >&2; exit 1; }
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || { printf 'Issue-009 timing requires a clean candidate\n' >&2; exit 1; }
umask 077
mkdir -p "$artifact_dir"
set -o noclobber
: >"$stderr_log"
failed=1
reason=unexpected_failure
workload_process_launches=0
warmup_launches=0
measured_rounds_completed=0
write_disposition() {
    local status=$1 why=$2
    printf '{"schema_version":1,"issue":9,"status":"%s","reason":"%s","runner_invocations":1,"workload_process_launches":%s,"warmup_launches":%s,"measured_rounds_completed":%s}\n' \
        "$status" "$why" "$workload_process_launches" "$warmup_launches" "$measured_rounds_completed" >"$disposition"
}
on_exit() {
    local status=$?
    trap - EXIT
    if [[ "$failed" == 1 && ! -e "$disposition" ]]; then set +e; write_disposition FAIL "$reason"; fi
    exit "$status"
}
trap on_exit EXIT
candidate=$(git rev-parse --verify HEAD 2>>"$stderr_log")
candidate_sha=$(printf '%s' "$candidate" | sha256sum | awk '{print $1}')
reason=build_failed
cargo build --locked --release --quiet -p miso-engine-scheduler-bench 2>>"$stderr_log"
binary="$root/target/release/miso_engine_scheduler_bench"
[[ -x "$binary" ]] || { reason=missing_binary; exit 1; }
binary_sha=$(sha256sum "$binary" | awk '{print $1}')
cpu_model=$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)
os=$(uname -s)
kernel=$(uname -r)
rust_version=$(rustc -V)
llvm_version=$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')
governor=not-reported
[[ ! -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]] || governor=$(< /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
run_round() {
    local round=$1
    workload_process_launches=$((workload_process_launches + 1))
    MISO_ENGINE_SCHEDULER_BENCH_ROUND="$round" \
    MISO_ENGINE_SCHEDULER_BENCH_CANDIDATE_SHA256="$candidate_sha" \
    MISO_ENGINE_SCHEDULER_BENCH_BINARY_SHA256="$binary_sha" \
    MISO_ENGINE_BENCH_CPU_MODEL="$cpu_model" MISO_ENGINE_BENCH_OS="$os" \
    MISO_ENGINE_BENCH_KERNEL="$kernel" MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" \
    MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" MISO_ENGINE_BENCH_GOVERNOR="$governor" \
    "$binary"
}
reason=warmup_failed
run_round warmup >/dev/null 2>>"$stderr_log" || exit 1
warmup_launches=1
reason=round_1_failed
run_round 1 >"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=1
reason=round_2_failed
run_round 2 >>"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=2
reason=record_count
[[ "$(wc -l <"$raw")" == 6 ]] || exit 1
reason=validation_failed
jq -s -e -L scripts -f scripts/scheduler-benchmark-validator.jq "$raw" >/dev/null || exit 1
: >"$accepted"
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || exit 1
write_disposition PASS complete
failed=0
trap - EXIT
printf '%s\n' "$accepted"
