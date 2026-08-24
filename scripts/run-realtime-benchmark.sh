#!/usr/bin/env bash
# Run one bounded descriptive issue-003 benchmark invocation with one or two internal rounds.
set -euo pipefail

rounds="${1:-2}"
blocks="${2:-1000000}"
[[ "$rounds" == "1" || "$rounds" == "2" ]] || {
    printf 'usage: %s [1|2] [blocks]\n' "$0" >&2
    exit 2
}

cpu="unknown"
if command -v lscpu >/dev/null 2>&1; then
    cpu="$(lscpu | awk -F: '/Model name/ { sub(/^[[:space:]]+/, "", $2); print $2; exit }')"
fi

power_mode="unknown"
if [[ -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]]; then
    power_mode="$(< /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)"
fi

compiler="$(rustc -V 2>/dev/null || printf unknown)"
llvm_version="$(rustc -Vv 2>/dev/null | awk -F': ' '/LLVM version/ { print $2; exit }')"
llvm_version="${llvm_version:-unknown}"

output="$(
    MISO_ENGINE_BENCH_CPU_MODEL="$cpu" \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="$power_mode" \
    MISO_ENGINE_BENCH_RUST_VERSION="$compiler" \
    MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
    MISO_ENGINE_BENCH_TARGET_TRIPLE="$(rustc -vV | awk '/host/ { print $2; exit }')" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="runtime-dispatch-baseline" \
    MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER="native-cli" \
    target/release/miso_engine_realtime_audit \
        --blocks "$blocks" --benchmark-rounds "$rounds"
)"

[[ "$(printf '%s\n' "$output" | wc -l | tr -d ' ')" == "1" ]] || {
    printf 'realtime benchmark did not emit exactly one JSONL record\n' >&2
    exit 1
}

command -v jq >/dev/null 2>&1 || {
    printf 'jq is required to validate realtime benchmark JSON\n' >&2
    exit 1
}
printf '%s\n' "$output" | jq -e \
    --argjson expected_rounds "$rounds" \
    --argjson expected_blocks "$blocks" '
    .schema_version == 1 and
    .benchmark == "realtime_plan_lifetime" and
    .rounds == $expected_rounds and
    .blocks_per_round == $expected_blocks and
    (.round_duration_ns | type == "array" and length == $expected_rounds) and
    (.ns_per_block | type == "array" and length == $expected_rounds) and
    (.ns_per_block | all(type == "number")) and
    .statistical_method == "per-round ns/block; descriptive only; no threshold"
' >/dev/null

printf '%s\n' "$output"
