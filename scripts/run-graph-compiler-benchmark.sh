#!/usr/bin/env bash
# Sole future Issue-006 benchmark entry point. This script is never an Issue-030 workload action.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
record_fixture="$script_directory/fixtures/graph-benchmark-validator-record.json"
record_validator="$script_directory/graph-benchmark-record-validator.jq"
validator="$script_directory/graph-benchmark-validator.jq"
artifact_directory="$repository_root/target/issue6"
raw_output="$artifact_directory/graph-compiler-benchmark.raw.jsonl"
accepted_output="$artifact_directory/graph-compiler-benchmark.jsonl"

report_identity() {
    local path=$1
    if [[ -f "$path" && ! -L "$path" ]]; then
        printf 'raw artifact: bytes=%s sha256=%s path=%s\n' \
            "$(wc -c <"$path" | tr -d ' ')" \
            "$(sha256sum "$path" | awk '{print $1}')" "$path" >&2
    fi
}

publish_copy() {
    local source=$1 destination=$2 directory temporary
    directory="$(dirname "$destination")"
    [[ -f "$source" && ! -L "$source" ]] || return 1
    [[ ! -e "$destination" && ! -L "$destination" ]] || return 1
    temporary="$(mktemp "$directory/.graph-compiler-benchmark.XXXXXX")"
    trap 'rm -f -- "$temporary"' RETURN
    cp -- "$source" "$temporary"
    cmp -s -- "$source" "$temporary"
    jq -s -e -L "$script_directory" -f "$validator" "$temporary" >/dev/null
    [[ ! -e "$destination" && ! -L "$destination" ]] || return 1
    mv -n -- "$temporary" "$destination"
    [[ ! -e "$temporary" && -f "$destination" && ! -L "$destination" ]] || return 1
    cmp -s -- "$source" "$destination"
    trap - RETURN
}

# Preflight is deliberately inline here rather than a separate scripts/preflight-*.sh: this is a
# descriptive run whose outputs live under a rebuildable target/ path, so there is no consumed
# one-shot authority for a sealed pre-check to protect. The AGENTS.md preflight duties -- tools,
# schema, output persistence and overwrite refusal before any workload launch -- are all
# discharged by the block below.
command -v jq >/dev/null || { printf 'jq is required for graph benchmark validation\n' >&2; exit 1; }
command -v cargo >/dev/null || { printf 'cargo is required for graph benchmark workload\n' >&2; exit 1; }
[[ -f "$record_fixture" && -f "$record_validator" && -f "$validator" ]] || {
    printf 'graph benchmark validators are missing\n' >&2
    exit 1
}
jq -e -L "$script_directory" \
    'include "graph-benchmark-record-validator"; graph_benchmark_record_valid' \
    "$record_fixture" >/dev/null || {
    printf 'graph benchmark validator preflight failed\n' >&2
    exit 1
}
[[ ! -e "$raw_output" && ! -L "$raw_output" && ! -e "$accepted_output" && ! -L "$accepted_output" ]] || {
    printf 'refusing to overwrite an existing Issue-006 benchmark artifact\n' >&2
    exit 1
}
mkdir -p "$artifact_directory"

status_file="$(mktemp "$artifact_directory/.graph-compiler-benchmark-status.XXXXXX")"
if ! (
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="${MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE:-unknown}" \
    MISO_ENGINE_BENCH_POWER_SOURCE="${MISO_ENGINE_BENCH_POWER_SOURCE:-unknown}" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="${MISO_ENGINE_BENCH_TARGET_FEATURES:-${RUSTFLAGS:-target-default}}" \
    MISO_ENGINE_BENCH_OPT_LEVEL="${MISO_ENGINE_BENCH_OPT_LEVEL:-3}" \
    MISO_ENGINE_BENCH_LTO="${MISO_ENGINE_BENCH_LTO:-off}" \
    MISO_ENGINE_BENCH_CODEGEN_UNITS="${MISO_ENGINE_BENCH_CODEGEN_UNITS:-default}" \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="${MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE:-not measured}" \
    cargo run --locked --release --quiet -p bench -- graph >"$raw_output"
    status=$?
    printf '%s\n' "$status" >"$status_file"
    exit "$status"
); then
    status="$(<"$status_file")"
    rm -f -- "$status_file"
    printf 'graph benchmark workload failed; raw output preserved at %s\n' "$raw_output" >&2
    report_identity "$raw_output"
    exit "$status"
fi
rm -f -- "$status_file"

if ! jq -s -e -L "$script_directory" -f "$validator" "$raw_output" >/dev/null; then
    printf 'graph benchmark validation failed; rejected output preserved at %s\n' "$raw_output" >&2
    report_identity "$raw_output"
    exit 1
fi
if ! publish_copy "$raw_output" "$accepted_output"; then
    printf 'graph benchmark accepted-artifact publication failed; raw output preserved at %s\n' "$raw_output" >&2
    report_identity "$raw_output"
    exit 1
fi
printf '%s\n' "$accepted_output"
