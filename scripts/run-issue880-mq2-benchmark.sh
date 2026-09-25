#!/usr/bin/env bash
# Issue-880 MQ-2 preflight and exactly-once timing entrypoint.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

mode=
output_arg=
usage() { printf 'usage: %s (--preflight|--run) --output RECORD.json\n' "$0" >&2; }
while (($#)); do
    case "$1" in
        --preflight|--run)
            [[ -z "$mode" ]] || { usage; exit 2; }
            mode=${1#--}
            shift
            ;;
        --output)
            [[ -z "$output_arg" && $# -ge 2 && -n "$2" ]] || { usage; exit 2; }
            output_arg=$2
            shift 2
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            usage
            exit 2
            ;;
    esac
done
[[ -n "$mode" && -n "$output_arg" ]] || { usage; exit 2; }

if [[ "$output_arg" == /* ]]; then
    output=$(realpath -m -- "$output_arg")
else
    output=$(realpath -m -- "$root/$output_arg")
fi
output_dir=$(dirname -- "$output")
mkdir -p -- "$output_dir"
raw_output="$output.raw.log"
failure_output="$output.failure.json"

refuse_existing_outputs() {
    local path
    for path in "$output" "$raw_output" "$failure_output"; do
        [[ ! -e "$path" && ! -L "$path" ]] || {
            printf 'refusing to overwrite MQ-2 output: %s\n' "$path" >&2
            return 1
        }
    done
}

run_and_capture() {
    local capture=$1
    shift
    if "$@" >"$capture" 2>&1; then
        return 0
    else
        local status=$?
        return "$status"
    fi
}

persist_no_clobber() { ln -- "$1" "$2"; }

check_machine() {
    [[ "$(uname -m)" == x86_64 ]] || {
        printf 'Issue-880 MQ-2 requires native x86-64-v3 Simd8\n' >&2
        return 1
    }
    grep -qm1 -w avx2 /proc/cpuinfo && grep -qm1 -w fma /proc/cpuinfo || {
        printf 'Issue-880 MQ-2 requires AVX2 and FMA hardware\n' >&2
        return 1
    }
}

preflight() {
    local scratch status probe_lines
    refuse_existing_outputs
    check_machine
    jq -e -f scripts/issue880-mq2-record-validator.jq \
        scripts/fixtures/issue880-mq2-record.json >/dev/null
    if jq '.arms[1].rate_coefficient_calls_per_block = 1' \
        scripts/fixtures/issue880-mq2-record.json |
        jq -e -f scripts/issue880-mq2-record-validator.jq >/dev/null; then
        printf 'MQ-2 record validator accepted a deliberately wrong call count\n' >&2
        return 1
    fi
    if ! jq '.schema_version = 2 |
        .rate_coefficient_calls_per_bank_per_parameter = 16 |
        .arms |= map(.rate_coefficient_calls_per_bank_per_parameter = 16 |
            .rate_coefficient_calls_per_block = (8 * 8 * 2 * .ramping_parameters))' \
        scripts/fixtures/issue880-mq2-record.json |
        jq -e -f scripts/issue880-mq2-record-validator.jq >/dev/null; then
        printf 'MQ-2 record validator rejected the MC-2 event-rate call-count profile\n' >&2
        return 1
    fi
    if jq '.schema_version = 2 |
        .rate_coefficient_calls_per_bank_per_parameter = 16 |
        .arms |= map(.rate_coefficient_calls_per_bank_per_parameter = 16 |
            .rate_coefficient_calls_per_block = (8 * 8 * 2 * .ramping_parameters)) |
        .arms[1].rate_coefficient_calls_per_block = 1' \
        scripts/fixtures/issue880-mq2-record.json |
        jq -e -f scripts/issue880-mq2-record-validator.jq >/dev/null; then
        printf 'MQ-2 record validator accepted a wrong MC-2 event-rate call count\n' >&2
        return 1
    fi

    scratch=$(mktemp -d "$output_dir/.mq2-preflight.XXXXXXXX")
    printf 'persist-probe\n' >"$scratch/source"
    persist_no_clobber "$scratch/source" "$scratch/persisted"
    cmp -s "$scratch/source" "$scratch/persisted"
    printf 'existing-sentinel\n' >"$scratch/occupied"
    if persist_no_clobber "$scratch/source" "$scratch/occupied" 2>/dev/null; then
        rm -rf -- "$scratch"
        printf 'MQ-2 persistence helper overwrote an existing path\n' >&2
        return 1
    fi
    [[ "$(<"$scratch/occupied")" == existing-sentinel ]]

    if run_and_capture "$scratch/status-probe" bash -c 'printf exit-probe; exit 37'; then
        rm -rf -- "$scratch"
        printf 'MQ-2 capture helper accepted a failing command\n' >&2
        return 1
    else
        status=$?
    fi
    [[ "$status" == 37 && "$(<"$scratch/status-probe")" == exit-probe ]]

    # Compile the ignored target, but do not select it or execute its timed workload.
    cargo test --locked --release --quiet -p compressor --test bench_ramp --no-run

    # Verify extraction using actual libtest output from the harmless marker test.
    if ! run_and_capture "$scratch/libtest-output" cargo test --locked --release --quiet \
        -p compressor --test bench_ramp -- mq2_libtest_result_marker_probe --exact \
        --nocapture --test-threads=1; then
        rm -rf -- "$scratch"
        printf 'MQ-2 libtest marker probe failed\n' >&2
        return 1
    fi
    mapfile -t probe_lines < <(sed -n 's/.*MQ2_RESULT_PROBE /MQ2_RESULT_PROBE /p' "$scratch/libtest-output")
    if [[ "${#probe_lines[@]}" != 1 ]] ||
        ! jq -e '.probe == true and .simd8 == true' <<<"${probe_lines[0]#MQ2_RESULT_PROBE }" >/dev/null; then
        cat "$scratch/libtest-output" >&2
        rm -rf -- "$scratch"
        printf 'MQ-2 result extraction did not match actual libtest output\n' >&2
        return 1
    fi
    rm -rf -- "$scratch"
}

preflight
if [[ "$mode" == preflight ]]; then
    printf 'Issue-880 MQ-2 preflight: PASS (timed workload launches 0)\n'
    exit 0
fi

[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-880 MQ-2 timing requires a clean committed candidate\n' >&2
    exit 1
}
candidate_commit=$(git rev-parse --verify HEAD)
candidate_tree=$(git rev-parse --verify "$candidate_commit^{tree}")
[[ "$candidate_commit" =~ ^[0-9a-f]{40}$ && "$candidate_tree" =~ ^[0-9a-f]{40}$ ]] || exit 1

cpu_model=$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)
architecture=$(uname -m)
os=$(uname -s)
kernel=$(uname -r)
rust_version=$(rustc -V)
llvm_version=$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')
target_triple=$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')
benchmark_source_sha256=$(sha256sum crates/compressor/tests/bench_ramp.rs | awk '{print $1}')
runner_sha256=$(sha256sum "$0" | awk '{print $1}')

raw_stage=$(mktemp "$output.run.XXXXXXXX")
record_stage=$(mktemp "$output.record.XXXXXXXX")
measurement_stage=$(mktemp "$output.measurement.XXXXXXXX")
failure_stage=$(mktemp "$output.failure.XXXXXXXX")
cleanup() { rm -f -- "$raw_stage" "$record_stage" "$measurement_stage" "$failure_stage"; }
trap cleanup EXIT

write_failure() {
    local reason=$1 status=$2
    jq -n \
        --arg candidate_commit "$candidate_commit" \
        --arg candidate_tree "$candidate_tree" \
        --arg reason "$reason" \
        --arg raw_output "$raw_output" \
        --argjson exit_status "$status" \
        '{schema_version:1,issue:880,task:"MQ-2",kind:"compressor_ramp_benchmark_failure",candidate_commit:$candidate_commit,candidate_tree:$candidate_tree,reason:$reason,exit_status:$exit_status,raw_output:$raw_output}' \
        >"$failure_stage"
    persist_no_clobber "$failure_stage" "$failure_output"
}

if run_and_capture "$raw_stage" cargo test --locked --release -p compressor --test bench_ramp -- \
    --ignored --exact mq2_compressor_ramp_spike --nocapture --test-threads=1; then
    :
else
    workload_status=$?
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure workload_failed "$workload_status"
    printf 'MQ-2 workload failed with status %s; raw output: %s\n' "$workload_status" "$raw_output" >&2
    exit "$workload_status"
fi

mapfile -t result_lines < <(sed -n 's/.*MQ2_RESULT /MQ2_RESULT /p' "$raw_stage")
if [[ "${#result_lines[@]}" != 1 ]]; then
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure result_line_count 1
    printf 'MQ-2 emitted %s result records; raw output: %s\n' "${#result_lines[@]}" "$raw_output" >&2
    exit 1
fi
printf '%s\n' "${result_lines[0]#MQ2_RESULT }" >"$measurement_stage"
if ! jq -e 'type == "object" and .task == "MQ-2"' "$measurement_stage" >/dev/null; then
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure result_json 1
    printf 'MQ-2 result line was not valid JSON; raw output: %s\n' "$raw_output" >&2
    exit 1
fi

jq -n \
    --slurpfile measurement "$measurement_stage" \
    --arg candidate_commit "$candidate_commit" \
    --arg candidate_tree "$candidate_tree" \
    --arg benchmark_source_sha256 "$benchmark_source_sha256" \
    --arg runner_sha256 "$runner_sha256" \
    --arg cpu_model "$cpu_model" \
    --arg architecture "$architecture" \
    --arg os "$os" \
    --arg kernel "$kernel" \
    --arg rust_version "$rust_version" \
    --arg llvm_version "$llvm_version" \
    --arg target_triple "$target_triple" \
    '{schema_version:2,issue:880,task:"MQ-2",kind:"compressor_ramp_benchmark",status:"measured",candidate_commit:$candidate_commit,candidate_tree:$candidate_tree,benchmark_source_sha256:$benchmark_source_sha256,runner_sha256:$runner_sha256,sample_rate_hz:48000,quantum_frames:128,bank_width:8,bank_count:8,track_count:64,warmup_blocks_per_arm:32,measured_blocks_per_round:32,measured_rounds_per_arm:2,workload_invocations:1,ramp_frames:64,coefficient_channels:2,rate_coefficient_calls_per_bank_per_parameter:16,programme_seeds:{left:"0x88000001",right:"0x88000002"},cpu_model:$cpu_model,architecture:$architecture,os:$os,kernel:$kernel,rust_version:$rust_version,llvm_version:$llvm_version,target_triple:$target_triple,arms:$measurement[0].arms}' \
    >"$record_stage"
if ! jq -e -f scripts/issue880-mq2-record-validator.jq "$record_stage" >/dev/null; then
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure final_record_schema 1
    printf 'MQ-2 record failed schema validation; raw output: %s\n' "$raw_output" >&2
    exit 1
fi

persist_no_clobber "$raw_stage" "$raw_output"
persist_no_clobber "$record_stage" "$output"
printf 'MQ-2 record: %s\nMQ-2 raw output: %s\n' "$output" "$raw_output"
