#!/usr/bin/env bash
# Sole authorized Issue-035 descriptive invocation. Never rebuild, retry, or resume this workload.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
artifact_directory="$repository_root/target/issue35"
raw_output="$artifact_directory/builtins-benchmark.raw.jsonl"
accepted_output="$artifact_directory/builtins-benchmark.jsonl"
stderr_output="$artifact_directory/builtins-benchmark.validator.stderr"
disposition="$artifact_directory/builtins-benchmark.disposition.json"
seal="$artifact_directory/builtins-benchmark.preflight.json"
binary="$artifact_directory/miso_engine_builtins_bench"
record_validator="$script_directory/builtins-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-benchmark-validator.jq"

for tool in jq sha256sum wc cmp mktemp mv git; do
    command -v "$tool" >/dev/null || { printf 'required tool is unavailable: %s\n' "$tool" >&2; exit 1; }
done
for path in "$raw_output" "$accepted_output" "$stderr_output" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || {
        printf 'refusing to overwrite Issue-035 artifact: %s\n' "$path" >&2
        exit 1
    }
done
[[ -f "$seal" && ! -L "$seal" && -x "$binary" && ! -L "$binary" ]] || {
    printf 'Issue-035 sealed binary or preflight seal is unavailable\n' >&2
    exit 1
}
[[ -f "$record_validator" && ! -L "$record_validator" &&
   -f "$aggregate_validator" && ! -L "$aggregate_validator" ]] || {
    printf 'Issue-035 validators are unavailable\n' >&2
    exit 1
}
candidate_commit="$(git -C "$repository_root" rev-parse --verify HEAD)"
[[ -z "$(git -C "$repository_root" status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-035 runner requires a clean sealed candidate\n' >&2
    exit 1
}
binary_sha256="$(sha256sum "$binary" | awk '{print $1}')"
runner_sha256="$(sha256sum "$script_directory/run-builtins-benchmark.sh" | awk '{print $1}')"
record_validator_sha256="$(sha256sum "$record_validator" | awk '{print $1}')"
aggregate_validator_sha256="$(sha256sum "$aggregate_validator" | awk '{print $1}')"
preflight_sha256="$(sha256sum "$seal" | awk '{print $1}')"
jq -e \
    --arg candidate "$candidate_commit" \
    --arg binary "$binary_sha256" \
    --arg runner "$runner_sha256" \
    --arg record_validator "$record_validator_sha256" \
    --arg aggregate_validator "$aggregate_validator_sha256" \
    'type == "object" and
     .schema_version == 2 and .issue == 58 and .kind == "builtins_benchmark_preflight" and
     .candidate_commit == $candidate and .binary_sha256 == $binary and
     .runner_sha256 == $runner and .record_validator_sha256 == $record_validator and
     .aggregate_validator_sha256 == $aggregate_validator and
     .runner_invocations == 0 and .workload_invocations == 0 and
     .timed_benchmark_invocations == 0' \
    "$seal" >/dev/null || {
    printf 'Issue-035 preflight seal does not match this candidate\n' >&2
    exit 1
}

artifact_identity() {
    local path=$1
    if [[ -f "$path" && ! -L "$path" ]]; then
        printf '"%s" %s' "$(sha256sum "$path" | awk '{print $1}')" "$(wc -c <"$path" | tr -d ' ')"
    else
        printf 'null 0'
    fi
}
write_disposition() {
    local status=$1 reason=$2 exit_status=$3
    local raw_identity accepted_identity stderr_identity
    local raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    raw_identity="$(artifact_identity "$raw_output")"
    accepted_identity="$(artifact_identity "$accepted_output")"
    stderr_identity="$(artifact_identity "$stderr_output")"
    read -r raw_sha raw_bytes <<<"$raw_identity"
    read -r accepted_sha accepted_bytes <<<"$accepted_identity"
    read -r stderr_sha stderr_bytes <<<"$stderr_identity"
    printf '{"schema_version":2,"issue":58,"status":"%s","reason":"%s","runner_invocations":1,"workload_invocations":1,"warmup_passes":1,"measured_rounds_completed":2,"timed_benchmark_invocations":1,"candidate_commit":"%s","binary_sha256":"%s","runner_sha256":"%s","record_validator_sha256":"%s","aggregate_validator_sha256":"%s","preflight_sha256":"%s","workload_exit_status":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$status" "$reason" "$candidate_commit" "$binary_sha256" "$runner_sha256" \
        "$record_validator_sha256" "$aggregate_validator_sha256" "$preflight_sha256" \
        "$exit_status" "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" \
        "$stderr_sha" "$stderr_bytes" >"$disposition"
}
publish_accepted_copy() {
    local temporary
    temporary="$(mktemp "$artifact_directory/.builtins-benchmark.XXXXXX")"
    trap 'rm -f -- "$temporary"' RETURN
    cp -- "$raw_output" "$temporary"
    cmp -s -- "$raw_output" "$temporary"
    [[ ! -e "$accepted_output" && ! -L "$accepted_output" ]] || return 1
    mv -n -- "$temporary" "$accepted_output"
    [[ ! -e "$temporary" && -f "$accepted_output" && ! -L "$accepted_output" ]] || return 1
    cmp -s -- "$raw_output" "$accepted_output"
    trap - RETURN
}

umask 077
mkdir -p "$artifact_directory"
set -o noclobber
exec {raw_fd}>"$raw_output"
exec {stderr_fd}>"$stderr_output"
failure_reason=workload_failed
workload_status=1
completed=0
on_exit() {
    local status=$?
    trap - EXIT INT TERM
    if [[ "$completed" == 0 && ! -e "$disposition" && ! -L "$disposition" ]]; then
        set +e
        write_disposition FAIL "$failure_reason" "$workload_status"
    fi
    exit "$status"
}
on_signal() {
    failure_reason=interrupted
    workload_status=130
    exit 130
}
trap on_exit EXIT
trap on_signal INT TERM

set +e
MISO_ENGINE_BUILTINS_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
MISO_ENGINE_BUILTINS_BENCH_BINARY_SHA256="$binary_sha256" \
"$binary" >&"$raw_fd" 2>&"$stderr_fd"
workload_status=$?
set -e
exec {raw_fd}>&-
exec {stderr_fd}>&-
if [[ "$workload_status" != 0 ]]; then
    if (( workload_status >= 128 )); then
        failure_reason=workload_interrupted
    fi
    exit "$workload_status"
fi
failure_reason=validation_failed
if ! jq -s -e -L "$script_directory" -f "$aggregate_validator" "$raw_output" >/dev/null 2>>"$stderr_output"; then
    printf 'Issue-035 aggregate validator rejected raw output\n' >>"$stderr_output"
    workload_status=1
    exit 1
fi
failure_reason=accepted_promotion_failed
if ! publish_accepted_copy; then
    workload_status=1
    exit 1
fi
write_disposition PASS complete 0
completed=1
trap - EXIT INT TERM
printf '%s\n' "$accepted_output"
