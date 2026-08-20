#!/usr/bin/env bash
# Run each issue-005 native libFuzzer target exactly once with its fixed bounded run count.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
readonly FUZZ_TOOLCHAIN="nightly-2026-08-20"
readonly FUZZ_RUNS=10000
readonly FUZZ_VERSION="0.13.2"
readonly OUTPUT_ROOT="$repository_root/target/ci/protocol-fuzz"

cd "$repository_root"

[[ "$(cargo fuzz --version)" == "cargo-fuzz $FUZZ_VERSION" ]] || {
    printf 'expected cargo-fuzz %s, found: %s\n' "$FUZZ_VERSION" "$(cargo fuzz --version)" >&2
    exit 1
}

run_target() {
    local target_name="$1"
    local seed_directory="$repository_root/fuzz/corpus/$target_name"
    local run_directory="$OUTPUT_ROOT/$target_name"

    mkdir -p "$run_directory"
    cp "$seed_directory"/*.hex "$run_directory/"
    cargo "+$FUZZ_TOOLCHAIN" fuzz run "$target_name" "$run_directory" -- \
        "-runs=$FUZZ_RUNS" \
        "-seed=$2" \
        "-artifact_prefix=$OUTPUT_ROOT/artifacts/$target_name/"
}

mkdir -p "$OUTPUT_ROOT/artifacts/protocol_command" \
    "$OUTPUT_ROOT/artifacts/protocol_session_transaction" \
    "$OUTPUT_ROOT/artifacts/protocol_event" \
    "$OUTPUT_ROOT/artifacts/protocol_response"
run_target protocol_command 557075001
run_target protocol_session_transaction 557075002
run_target protocol_event 557075003
run_target protocol_response 557075004

printf '{"issue":"005","invocation":3,"toolchain":"%s","cargo_fuzz":"%s","runs_per_target":%s,"targets":4,"new_executions":40000,"prior_executions":60000,"cumulative_executions":100000}\n' \
    "$FUZZ_TOOLCHAIN" "$FUZZ_VERSION" "$FUZZ_RUNS" > "$OUTPUT_ROOT/evidence.json"
