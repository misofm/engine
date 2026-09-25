#!/usr/bin/env bash
# Issue-880 MQ-1 preflight and exactly-once timing entrypoint.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/issue880-mq1-benchmark-lib.sh
source scripts/issue880-mq1-benchmark-lib.sh

readonly engine_effect_commit=6f662fee7b47a5eb38b67e0ddc6d007edd438cfa
mode=
output_arg=
usage() {
    printf 'usage: %s (--preflight|--run) --output RECORD.json\n' "$0" >&2
    printf '       %s --recover-preserved --raw-input RAW.log --failure-input FAILURE.json --output RECOVERED.json\n' "$0" >&2
}

while (($#)); do
    case "$1" in
        --preflight|--run)
            [[ -z "$mode" ]] || { usage; exit 2; }
            mode=${1#--}
            shift
            ;;
        --recover-preserved)
            [[ -z "$mode" ]] || { usage; exit 2; }
            mode=recover-preserved
            shift
            ;;
        --output)
            [[ -z "$output_arg" && $# -ge 2 && -n "$2" ]] || { usage; exit 2; }
            output_arg=$2
            shift 2
            ;;
        --raw-input)
            [[ -z "${raw_input_arg:-}" && $# -ge 2 && -n "$2" ]] || { usage; exit 2; }
            raw_input_arg=$2
            shift 2
            ;;
        --failure-input)
            [[ -z "${failure_input_arg:-}" && $# -ge 2 && -n "$2" ]] || { usage; exit 2; }
            failure_input_arg=$2
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
            printf 'refusing to overwrite MQ-1 output: %s\n' "$path" >&2
            return 1
        }
    done
}

check_machine() {
    [[ "$(uname -m)" == x86_64 ]] || {
        printf 'Issue-880 MQ-1 requires the native x86-64-v3 Simd8 target\n' >&2
        return 1
    }
    grep -qm1 -w avx2 /proc/cpuinfo && grep -qm1 -w fma /proc/cpuinfo || {
        printf 'Issue-880 MQ-1 requires AVX2 and FMA hardware\n' >&2
        return 1
    }
}

preflight() {
    local scratch failure_status persisted occupied
    refuse_existing_outputs
    check_machine
    git merge-base --is-ancestor "$engine_effect_commit" HEAD || {
        printf 'MQ-1 candidate is not based on the MA-3 E1 checkpoint\n' >&2
        return 1
    }
    git diff --quiet "$engine_effect_commit" HEAD -- \
        crates/math/src/lane_math.rs crates/transient-shaper/src || {
        printf 'MQ-1 baseline effect sources differ from the E1 checkpoint\n' >&2
        return 1
    }
    jq -e -L scripts -f scripts/issue880-mq1-record-validator.jq \
        scripts/fixtures/issue880-mq1-record.json >/dev/null

    scratch=$(mktemp -d "$output_dir/.mq1-preflight.XXXXXXXX")
    printf 'persist-probe\n' >"$scratch/source"
    persist_no_clobber "$scratch/source" "$scratch/persisted"
    cmp -s "$scratch/source" "$scratch/persisted"
    printf 'existing-sentinel\n' >"$scratch/occupied"
    if persist_no_clobber "$scratch/source" "$scratch/occupied" 2>/dev/null; then
        rm -rf -- "$scratch"
        printf 'MQ-1 persistence helper overwrote an existing path\n' >&2
        return 1
    fi
    [[ "$(<"$scratch/occupied")" == existing-sentinel ]]

    if run_and_capture "$scratch/status-probe" bash -c 'printf exit-probe; exit 37'; then
        rm -rf -- "$scratch"
        printf 'MQ-1 capture helper accepted a failing command\n' >&2
        return 1
    else
        failure_status=$?
    fi
    [[ "$failure_status" == 37 && "$(<"$scratch/status-probe")" == exit-probe ]]
    rm -rf -- "$scratch"

    # Build the ignored integration target without selecting --ignored or executing its workload.
    cargo test --locked --release --quiet -p transient-shaper --test bench --no-run
}

recover_preserved() (
    set -euo pipefail
    local raw_input failure_input raw_sha256 failure_sha256 scratch result_stage record_stage

    [[ -n "${raw_input_arg:-}" && -n "${failure_input_arg:-}" ]] || { usage; exit 2; }
    if [[ "$raw_input_arg" == /* ]]; then
        raw_input=$(realpath -e -- "$raw_input_arg")
    else
        raw_input=$(realpath -e -- "$root/$raw_input_arg")
    fi
    if [[ "$failure_input_arg" == /* ]]; then
        failure_input=$(realpath -e -- "$failure_input_arg")
    else
        failure_input=$(realpath -e -- "$root/$failure_input_arg")
    fi
    [[ -f "$raw_input" && -f "$failure_input" ]] || {
        printf 'MQ-1 recovery inputs must be regular files\n' >&2
        exit 1
    }
    [[ "$output" != "$raw_input" && "$output" != "$failure_input" ]] || {
        printf 'MQ-1 recovery output must be separate from its source files\n' >&2
        exit 1
    }
    refuse_existing_outputs

    raw_sha256=$(sha256sum "$raw_input" | awk '{print $1}')
    failure_sha256=$(sha256sum "$failure_input" | awk '{print $1}')
    jq -e --arg raw_sha256 "$raw_sha256" '
        type == "object" and
        .schema_version == 1 and .issue == 880 and .task == "MQ-1" and
        .kind == "transient_shaper_benchmark" and
        .status == "postprocess_failed_unaccepted" and
        (.candidate_commit | type == "string" and test("^[0-9a-f]{40}$")) and
        (.runner_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
        (.raw_sha256 == $raw_sha256) and
        (.runner_report.kind == "transient_shaper_benchmark_failure") and
        (.runner_report.reason == "result_line_count") and
        (.runner_report.exit_status == 1)
    ' "$failure_input" >/dev/null || {
        printf 'MQ-1 recovery failure record does not match the preserved parser-failure disposition\n' >&2
        exit 1
    }

    scratch=$(mktemp -d "$output_dir/.mq1-recover.XXXXXXXX")
    trap 'rm -rf -- "$scratch"' EXIT
    result_stage="$scratch/result.json"
    record_stage="$scratch/record.json"
    if ! extract_mq1_result "$raw_input" "$result_stage"; then
        printf 'MQ-1 recovery found zero, duplicate, or invalid result records\n' >&2
        exit 1
    fi

    jq -e --slurpfile result "$result_stage" '
        .fixture_sha256 == $result[0].fixture_sha256 and
        .bank_ns_per_lane_sample == $result[0].bank_ns_per_lane_sample and
        .scalar_ns_per_lane_sample == $result[0].scalar_ns_per_lane_sample
    ' "$failure_input" >/dev/null || {
        printf 'MQ-1 extracted values disagree with the preserved failed disposition\n' >&2
        exit 1
    }

    jq --slurpfile result "$result_stage" \
        --arg failure_input "$failure_input_arg" \
        --arg failure_sha256 "$failure_sha256" \
        --arg raw_input "$raw_input_arg" \
        --arg raw_sha256 "$raw_sha256" \
        --arg extracted_result_sha256 "$(sha256sum "$result_stage" | awk '{print $1}')" '
        .status = "measured" |
        .fixture_sha256 = $result[0].fixture_sha256 |
        .bank_ns_per_lane_sample = $result[0].bank_ns_per_lane_sample |
        .scalar_ns_per_lane_sample = $result[0].scalar_ns_per_lane_sample |
        .recovery = {
            schema_version: 1,
            method: "offline_mq1_result_extraction",
            recovered_from_status: "postprocess_failed_unaccepted",
            source_failure_record: $failure_input,
            source_failure_record_sha256: $failure_sha256,
            source_raw_log: $raw_input,
            source_raw_log_sha256: $raw_sha256,
            source_failure_reason: .failure_reason,
            source_runner_report: .runner_report,
            extracted_result_sha256: $extracted_result_sha256,
            workload_invocations_during_recovery: 0
        } |
        del(.failure_reason, .runner_report)
    ' "$failure_input" >"$record_stage"

    jq -e -L scripts -f scripts/issue880-mq1-record-validator.jq "$record_stage" >/dev/null || {
        printf 'MQ-1 recovered record failed schema validation\n' >&2
        exit 1
    }
    persist_no_clobber "$record_stage" "$output"
    printf 'MQ-1 recovered record: %s\n' "$output"
)

if [[ "$mode" == recover-preserved ]]; then
    recover_preserved
    exit 0
fi
[[ -z "${raw_input_arg:-}" && -z "${failure_input_arg:-}" ]] || { usage; exit 2; }
preflight
if [[ "$mode" == preflight ]]; then
    printf 'Issue-880 MQ-1 preflight: PASS (workload launches 0)\n'
    exit 0
fi

[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-880 MQ-1 timing requires a clean committed candidate\n' >&2
    exit 1
}
candidate_commit=$(git rev-parse --verify HEAD)
[[ "$candidate_commit" =~ ^[0-9a-f]{40}$ ]] || exit 1

cpu_model=$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)
architecture=$(uname -m)
os=$(uname -s)
kernel=$(uname -r)
rust_version=$(rustc -V)
llvm_version=$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')
target_triple=$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')
benchmark_source_sha256=$(sha256sum crates/transient-shaper/tests/bench.rs | awk '{print $1}')
runner_sha256=$(sha256sum "$0" | awk '{print $1}')

raw_stage=$(mktemp "$output.run.XXXXXXXX")
record_stage=$(mktemp "$output.record.XXXXXXXX")
failure_stage=$(mktemp "$output.failure.XXXXXXXX")
cleanup() { rm -f -- "$raw_stage" "$record_stage" "$failure_stage"; }
trap cleanup EXIT

write_failure() {
    local reason=$1 status=$2
    jq -n \
        --arg candidate_commit "$candidate_commit" \
        --arg reason "$reason" \
        --arg raw_output "$raw_output" \
        --argjson exit_status "$status" \
        '{schema_version:1,issue:880,task:"MQ-1",kind:"transient_shaper_benchmark_failure",candidate_commit:$candidate_commit,reason:$reason,exit_status:$exit_status,raw_output:$raw_output}' \
        >"$failure_stage"
    persist_no_clobber "$failure_stage" "$failure_output"
}

if run_and_capture "$raw_stage" cargo test --locked --release -p transient-shaper --test bench -- \
    --ignored --exact mq1_transient_shaper_ns_per_lane_sample --nocapture --test-threads=1; then
    :
else
    workload_status=$?
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure workload_failed "$workload_status"
    printf 'MQ-1 workload failed with status %s; raw output: %s\n' "$workload_status" "$raw_output" >&2
    exit "$workload_status"
fi

mapfile -t result_lines < <(sed -n 's/^MQ1_RESULT //p' "$raw_stage")
if [[ "${#result_lines[@]}" != 1 ]]; then
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure result_line_count 1
    printf 'MQ-1 emitted %s result records; raw output: %s\n' "${#result_lines[@]}" "$raw_output" >&2
    exit 1
fi
measurement=${result_lines[0]}
if ! jq -e '
    type == "object" and
    (keys | sort) == ["bank_ns_per_lane_sample", "fixture_sha256", "scalar_ns_per_lane_sample"] and
    (.fixture_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
    (.bank_ns_per_lane_sample | type == "array" and length == 2 and all(.[]; type == "number" and . > 0)) and
    (.scalar_ns_per_lane_sample | type == "array" and length == 2 and all(.[]; type == "number" and . > 0))
' <<<"$measurement" >/dev/null; then
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure result_schema 1
    printf 'MQ-1 result failed schema validation; raw output: %s\n' "$raw_output" >&2
    exit 1
fi

jq -n \
    --arg candidate_commit "$candidate_commit" \
    --arg engine_effect_commit "$engine_effect_commit" \
    --arg benchmark_source_sha256 "$benchmark_source_sha256" \
    --arg runner_sha256 "$runner_sha256" \
    --arg fixture_sha256 "$(jq -r '.fixture_sha256' <<<"$measurement")" \
    --arg cpu_model "$cpu_model" \
    --arg architecture "$architecture" \
    --arg os "$os" \
    --arg kernel "$kernel" \
    --arg rust_version "$rust_version" \
    --arg llvm_version "$llvm_version" \
    --arg target_triple "$target_triple" \
    --argjson bank_ns_per_lane_sample "$(jq -c '.bank_ns_per_lane_sample' <<<"$measurement")" \
    --argjson scalar_ns_per_lane_sample "$(jq -c '.scalar_ns_per_lane_sample' <<<"$measurement")" \
    '{schema_version:1,issue:880,task:"MQ-1",kind:"transient_shaper_benchmark",status:"measured",candidate_commit:$candidate_commit,engine_effect_commit:$engine_effect_commit,engine_effect_revision:"E1 (MA-3)",benchmark_source_sha256:$benchmark_source_sha256,runner_sha256:$runner_sha256,sample_rate_hz:48000,duration_seconds:4,frames:192000,block_frames:128,track_count:8,link_mode:"dual_mono",attack:0.75,sustain:-0.5,mix:1.0,programme_seeds:{left:"0x88000001",right:"0x88000002"},fixture_sha256:$fixture_sha256,workload_invocations:1,warmups_per_arm:1,measured_rounds_per_arm:2,bank_width:8,bank_ns_per_lane_sample:$bank_ns_per_lane_sample,scalar_width:1,scalar_ns_per_lane_sample:$scalar_ns_per_lane_sample,cpu_model:$cpu_model,architecture:$architecture,os:$os,kernel:$kernel,rust_version:$rust_version,llvm_version:$llvm_version,target_triple:$target_triple}' \
    >"$record_stage"
if ! jq -e -L scripts -f scripts/issue880-mq1-record-validator.jq "$record_stage" >/dev/null; then
    persist_no_clobber "$raw_stage" "$raw_output"
    write_failure final_record_schema 1
    printf 'MQ-1 final record failed schema validation; raw output: %s\n' "$raw_output" >&2
    exit 1
fi

persist_no_clobber "$raw_stage" "$raw_output"
persist_no_clobber "$record_stage" "$output"
printf 'MQ-1 record: %s\nMQ-1 raw output: %s\n' "$output" "$raw_output"
