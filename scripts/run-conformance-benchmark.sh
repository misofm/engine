#!/usr/bin/env bash
# Runs exactly one bounded descriptive invocation; it never retries or tunes.
set -euo pipefail
rounds="${1:-2}"
[[ "$rounds" == 1 || "$rounds" == 2 ]] || { printf 'usage: %s [1|2]\n' "$0" >&2; exit 2; }
output="$(
    MISO_ENGINE_BENCH_POWER_SOURCE="${MISO_ENGINE_BENCH_POWER_SOURCE:-unknown}" \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="${MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE:-unknown}" \
    MISO_ENGINE_BENCH_OPT_LEVEL=3 \
    MISO_ENGINE_BENCH_LTO=off \
    MISO_ENGINE_BENCH_CODEGEN_UNITS=16 \
    MISO_ENGINE_BENCH_TARGET_CPU=baseline \
    MISO_ENGINE_BENCH_TARGET_FEATURES=runtime-dispatch-baseline \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="not measured; descriptive baseline" \
    cargo run --locked --release -q -p miso-engine-conformance-bench -- --rounds "$rounds"
)"
[[ "$(printf '%s\n' "$output" | wc -l | tr -d ' ')" == "$((rounds * 2))" ]] || { printf 'benchmark record count mismatch\n' >&2; exit 1; }
command -v jq >/dev/null || { printf 'jq is required for JSONL verification\n' >&2; exit 1; }
printf '%s\n' "$output" | jq -s -e --argjson expected_rounds "$rounds" '
    length == ($expected_rounds * 2) and
    (map(.benchmark_id) | unique | sort) ==
        ["compare_f32_f64_2x4096", "fixture_decode_crc32c_2x4096"] and
    all(
        .schema_version == 1 and
        .rounds == $expected_rounds and
        (.round >= 1 and .round <= $expected_rounds) and
        (.timestamp_epoch_seconds | type == "number") and
        (["git_commit", "workspace_dirty", "cpu_model", "architecture", "physical_cores",
          "logical_cores", "os", "kernel", "power_source", "governor_or_power_mode",
          "compiler", "llvm_version", "cargo_profile", "opt_level", "lto", "codegen_units",
          "target_triple", "target_cpu", "compile_target_features", "runtime_or_browser",
          "sample_rate_hz", "quantum_frames", "channels", "fixture_path", "fixture_crc32c",
          "prng_algorithm", "prng_seed", "warmup_batches", "measured_batches", "batch_samples",
          "total_duration_ns", "timer", "unit", "percentile_method", "p50", "p95", "p99",
          "p99_9", "min", "max", "sample_count", "background_load_note",
          "metadata_incomplete", "missing_metadata"] - (keys)) == [] and
        .sample_count == 4096 and .measured_batches == 4096 and
        (.missing_metadata | type == "array") and
        (.metadata_incomplete == ((.missing_metadata | length) > 0)) and
        (.min > 0 and .min <= .p50 and .p50 <= .p95 and .p95 <= .p99 and
         .p99 <= .p99_9 and .p99_9 <= .max)
    )
' >/dev/null
printf '%s\n' "$output"
