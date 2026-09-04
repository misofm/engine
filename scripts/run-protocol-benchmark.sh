#!/usr/bin/env bash
# Runs the one fixed Issue-005 descriptive comparison. It never retries, tunes, or gates results.
set -euo pipefail

[[ "$#" == 2 && "$1" == "--rounds" && "$2" == 2 ]] || {
    printf 'usage: %s --rounds 2\n' "$0" >&2
    exit 2
}

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
record_fixture="$script_directory/fixtures/protocol-benchmark-validator-record.json"
validator="$script_directory/protocol-benchmark-validator.jq"
# Preflight is deliberately inline here rather than a separate scripts/preflight-*.sh: this is a
# descriptive comparison whose outputs live under a rebuildable target/ path, so there is no
# consumed one-shot authority for a sealed pre-check to protect. The AGENTS.md preflight duties
# -- tools, schema, output persistence and overwrite refusal before any workload launch -- are
# all discharged by the block below and the artifact guard further down.
command -v jq >/dev/null || { printf 'jq is required for JSONL verification\n' >&2; exit 1; }
jq -e -L "$script_directory" \
    'include "protocol-benchmark-record-validator"; protocol_benchmark_record_valid' \
    "$record_fixture" >/dev/null || {
    printf 'protocol benchmark record validator preflight failed\n' >&2
    exit 1
}

cd "$repository_root"
scalar_artifact="target/ci/issue005-protocol-bench-wasm-scalar/wasm32-unknown-unknown/release/bench.wasm"
simd_artifact="target/ci/issue005-protocol-bench-wasm-simd128/wasm32-unknown-unknown/release/bench.wasm"
[[ -f "$scalar_artifact" && -f "$simd_artifact" ]] || {
    printf 'build Wasm parity artifacts with scripts/check-protocol-wasm-parity.sh before benchmarking\n' >&2
    exit 1
}

raw_output="target/issue005-protocol-benchmark.raw.jsonl"
accepted_output="target/issue005-protocol-benchmark.jsonl"
[[ ! -e "$raw_output" && ! -e "$accepted_output" ]] || {
    printf 'refusing to overwrite an existing Issue-005 benchmark artifact\n' >&2
    exit 1
}

if !
    MISO_ENGINE_BENCH_TARGET_CPU="${MISO_ENGINE_BENCH_TARGET_CPU:-target-default}" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="${MISO_ENGINE_BENCH_TARGET_FEATURES:-${RUSTFLAGS:-target-default}}" \
    MISO_ENGINE_BENCH_WASM_HOST="${MISO_ENGINE_BENCH_WASM_HOST:-wasm-interp}" \
    MISO_ENGINE_BENCH_WASM_HOST_VERSION="${MISO_ENGINE_BENCH_WASM_HOST_VERSION:-$(wasm-interp --version 2>/dev/null || printf unknown)}" \
    MISO_ENGINE_BENCH_WASM_SCALAR_BYTES="$(wc -c < "$scalar_artifact" | tr -d ' ')" \
    MISO_ENGINE_BENCH_WASM_SIMD_BYTES="$(wc -c < "$simd_artifact" | tr -d ' ')" \
    cargo run --locked --release -q -p bench -- protocol --rounds 2 >"$raw_output"
then
    printf 'benchmark workload failed; partial raw output preserved at %s\n' "$raw_output" >&2
    exit 1
fi

if ! jq -s -e -L "$script_directory" -f "$validator" "$raw_output" >/dev/null; then
    printf 'benchmark JSONL validation failed; complete raw output preserved at %s\n' "$raw_output" >&2
    exit 1
fi
mv "$raw_output" "$accepted_output"
cat "$accepted_output"
