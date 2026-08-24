#!/usr/bin/env bash
# One fixed invocation, exactly two internal rounds, descriptive output only; never retries or tunes.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }

target_features="${RUSTFLAGS:-target-default}"
output="$(
    MISO_ENGINE_BENCH_POWER_SOURCE="${MISO_ENGINE_BENCH_POWER_SOURCE:-unknown}" \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="${MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE:-unknown}" \
    MISO_ENGINE_BENCH_OPT_LEVEL=3 \
    MISO_ENGINE_BENCH_LTO=off \
    MISO_ENGINE_BENCH_CODEGEN_UNITS=16 \
    MISO_ENGINE_BENCH_TARGET_CPU=target-default \
    MISO_ENGINE_BENCH_TARGET_FEATURES="$target_features" \
    MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER=native-cli \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="not measured; descriptive baseline" \
    cargo run --locked --release -q -p miso-engine-bench -- session
)"

[[ "$(printf '%s\n' "$output" | wc -l | tr -d ' ')" == 4 ]] || {
    printf 'session benchmark record count mismatch\n' >&2
    exit 1
}
command -v jq >/dev/null || { printf 'jq is required for JSONL verification\n' >&2; exit 1; }
printf '%s\n' "$output" | jq -s -e '
    length == 4 and
    (map(.benchmark_id) | unique | sort) ==
        ["session_compile_256_tracks", "session_parse_canonical_256_tracks"] and
    ([.[] | (.benchmark_id + ":" + (.round | tostring))] | unique | length) == 4 and
    (group_by(.benchmark_id) | all(map(.round) == [1, 2])) and
    ([.[] | .timestamp_epoch_seconds] | unique | length) == 1 and
    ([.[] | .fixture_sha256] | unique | length) == 1 and
    ([.[] | .fixture_size_bytes] | unique | length) == 1 and
    all(
        .schema_version == 2 and .rounds == 2 and
        (.round == 1 or .round == 2) and
        (["timestamp_epoch_seconds", "cpu_model", "architecture", "physical_cores",
          "logical_cores", "os", "kernel", "power_source", "governor_or_power_mode",
          "rustc_version", "llvm_version", "cargo_profile", "opt_level", "lto",
          "codegen_units", "target_triple", "target_cpu", "compile_target_features",
          "runtime_or_browser", "sample_rate_hz", "quantum_frames", "fixture_path",
          "fixture_sha256", "fixture_size_bytes", "fixture_counts", "warmup_batches",
          "measured_batches", "operations_per_batch", "total_operations",
          "total_duration_ns", "timer", "unit", "percentile_method", "p50", "p95",
          "p99", "p99_9", "min", "max", "background_load_note",
          "metadata_incomplete", "missing_metadata", "descriptive_only",
          "decision_threshold"] - keys) == [] and
        .cargo_profile == "release" and .opt_level == "3" and
        .lto == "off" and .codegen_units == "16" and
        .runtime_or_browser == "native-cli" and
        .sample_rate_hz == 48000 and .quantum_frames == 128 and
        .fixture_path == "generated:canonical-v1-256-tracks" and
        (.fixture_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
        (.fixture_size_bytes | type == "number" and . > 0) and
        .fixture_counts == {
            "sources": 1,
            "tracks": 256,
            "submixes": 0,
            "outputs": 1,
            "routes": 256,
            "automation_programs": 256,
            "effects": 256,
            "effect_parameters": 256,
            "automation_segments": 256
        } and
        .warmup_batches == 64 and .measured_batches == 512 and
        .operations_per_batch == 8 and .total_operations == 4096 and
        .timer == "std::time::Instant" and .unit == "ns/operation" and
        .percentile_method == "nearest-rank over 512 batch durations divided by 8 operations" and
        (.missing_metadata | type == "array" and (unique | length) == length) and
        (.metadata_incomplete == ((.missing_metadata | length) > 0)) and
        (.min > 0 and .min <= .p50 and .p50 <= .p95 and .p95 <= .p99 and
         .p99 <= .p99_9 and .p99_9 <= .max) and
        .descriptive_only == true and .decision_threshold == null
    )
' >/dev/null
printf '%s\n' "$output"
