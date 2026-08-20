#!/usr/bin/env bash
# Run the fixed-work bootstrap benchmark once. The binary itself performs exactly two rounds by
# default and prints a single JSONL record; this script does not write a benchmark result file.
set -euo pipefail

rounds="${1:-2}"
[[ "$rounds" == "1" || "$rounds" == "2" ]] || {
    printf 'usage: %s [1|2]\n' "$0" >&2
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
    MISO_ENGINE_BENCH_CPU="$cpu" \
    MISO_ENGINE_BENCH_POWER_MODE="$power_mode" \
    MISO_ENGINE_BENCH_COMPILER="$compiler" \
    MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
    MISO_ENGINE_BENCH_TARGET_TRIPLE="$(rustc -vV | awk '/host/ { print $2; exit }')" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="runtime-dispatch-baseline" \
    MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER="native-cli" \
    cargo run --locked --release -q -p miso-engine-bootstrap-bench -- --rounds "$rounds"
)"

[[ "$(printf '%s\n' "$output" | wc -l | tr -d ' ')" == "1" ]] || {
    printf 'benchmark did not emit exactly one JSONL record\n' >&2
    exit 1
}

for field in schema_version benchmark cpu os power_mode compiler llvm_version target_triple \
    compile_target_features runtime_or_browser sample_rate_hz quantum_frames fixture \
    warmup_iterations iterations_per_round rounds round_duration_ns ns_per_call \
    statistical_method capabilities; do
    [[ "$output" == *"\"$field\""* ]] || {
        printf 'benchmark record is missing %s\n' "$field" >&2
        exit 1
    }
done

command -v jq >/dev/null 2>&1 || {
    printf 'jq is required to validate the benchmark JSONL record\n' >&2
    exit 1
}

printf '%s\n' "$output" | jq -e --argjson expected_rounds "$rounds" '
    type == "object" and
    .rounds == $expected_rounds and
    (.round_duration_ns | type == "array" and length == $expected_rounds) and
    (.ns_per_call | type == "array" and length == $expected_rounds) and
    (.ns_per_call | all(type == "number"))
' >/dev/null || {
    printf 'benchmark did not emit a valid JSONL record with the requested rounds\n' >&2
    exit 1
}

printf '%s\n' "$output"
