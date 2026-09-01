#!/usr/bin/env bash
# Run one bounded descriptive issue-146 benchmark invocation: what the canonical floating-point
# environment costs per rendered block.
#
# The measurement is a difference of two loops over one `PreparedRenderPlan` at 48 kHz and a
# 128-frame quantum -- one wrapping each block in the render entry's `CanonicalFpEnv` guard, one
# not -- so the reported `guard_ns_per_block` is the guard and nothing else. Descriptive only: no
# threshold, one invocation, one warmup, one or two measured rounds.
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
    target/release/audit fp-env \
        --blocks "$blocks" --benchmark-rounds "$rounds"
)"

[[ "$(printf '%s\n' "$output" | wc -l | tr -d ' ')" == "1" ]] || {
    printf 'fp environment benchmark did not emit exactly one JSONL record\n' >&2
    exit 1
}

command -v jq >/dev/null 2>&1 || {
    printf 'jq is required to validate fp environment benchmark JSON\n' >&2
    exit 1
}
printf '%s\n' "$output" | jq -e \
    --argjson expected_rounds "$rounds" \
    --argjson expected_blocks "$blocks" '
    .schema_version == 1 and
    .benchmark == "fp_environment_guard" and
    .rounds == $expected_rounds and
    .blocks_per_round == $expected_blocks and
    .quantum_frames == 128 and
    (.bare_round_duration_ns | type == "array" and length == $expected_rounds) and
    (.guarded_round_duration_ns | type == "array" and length == $expected_rounds) and
    (.bare_ns_per_block | type == "array" and length == $expected_rounds) and
    (.guarded_ns_per_block | type == "array" and length == $expected_rounds) and
    (.guard_ns_per_block | type == "array" and length == $expected_rounds) and
    (.guard_ns_per_block | all(type == "number")) and
    (.statistical_method | startswith("per-round ns/block, guarded minus bare"))
' >/dev/null

printf '%s\n' "$output"
