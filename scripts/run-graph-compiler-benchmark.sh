#!/usr/bin/env bash
# Exactly one fixed Issue-006 descriptive invocation. Never retry, tune, or gate on timing.
set -euo pipefail

[[ "$#" == 0 ]] || {
    printf 'usage: %s\n' "$0" >&2
    exit 2
}

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
record_fixture="$script_directory/fixtures/graph-benchmark-validator-record.json"
record_validator="$script_directory/graph-benchmark-record-validator.jq"
validator="$script_directory/graph-benchmark-validator.jq"
command -v jq >/dev/null || { printf 'jq is required for graph benchmark validation\n' >&2; exit 1; }
jq -e -L "$script_directory" \
    'include "graph-benchmark-record-validator"; graph_benchmark_record_valid' \
    "$record_fixture" >/dev/null || {
    printf 'graph benchmark validator preflight failed\n' >&2
    exit 1
}
[[ -f "$record_validator" && -f "$validator" ]] || {
    printf 'graph benchmark validators are missing\n' >&2
    exit 1
}

cd "$repository_root"
raw_output="target/issue6/graph-compiler-benchmark.raw.jsonl"
accepted_output="target/issue6/graph-compiler-benchmark.jsonl"
[[ ! -e "$raw_output" && ! -e "$accepted_output" ]] || {
    printf 'refusing to overwrite an existing Issue-006 benchmark artifact\n' >&2
    exit 1
}
mkdir -p "target/issue6"

if !
    MISO_ENGINE_BENCH_POWER_MODE="${MISO_ENGINE_BENCH_POWER_MODE:-unknown}" \
    MISO_ENGINE_BENCH_POWER_SOURCE="${MISO_ENGINE_BENCH_POWER_SOURCE:-unknown}" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="${MISO_ENGINE_BENCH_TARGET_FEATURES:-${RUSTFLAGS:-target-default}}" \
    MISO_ENGINE_BENCH_OPT_LEVEL="${MISO_ENGINE_BENCH_OPT_LEVEL:-3}" \
    MISO_ENGINE_BENCH_LTO="${MISO_ENGINE_BENCH_LTO:-off}" \
    MISO_ENGINE_BENCH_CODEGEN_UNITS="${MISO_ENGINE_BENCH_CODEGEN_UNITS:-default}" \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD="${MISO_ENGINE_BENCH_BACKGROUND_LOAD:-not measured}" \
    cargo run --locked --release --quiet -p miso-engine-graph-bench >"$raw_output"
then
    printf 'graph benchmark workload failed; partial raw output preserved at %s\n' "$raw_output" >&2
    exit 1
fi

if ! jq -s -e -L "$script_directory" -f "$validator" "$raw_output" >/dev/null; then
    printf 'graph benchmark validation failed; rejected output preserved at %s\n' "$raw_output" >&2
    sha256sum "$raw_output" >&2
    exit 1
fi
mv "$raw_output" "$accepted_output"
cat "$accepted_output"
