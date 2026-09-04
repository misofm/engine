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
    # lane's D4 guard requires x86-64-v3; cargo-fuzz rebuilds RUSTFLAGS and would drop the
    # workspace pin, so thread it through explicitly (#84).
    RUSTFLAGS="-C target-feature=+avx2,+fma${RUSTFLAGS:+ $RUSTFLAGS}" \
    cargo "+$FUZZ_TOOLCHAIN" fuzz run "$target_name" "$run_directory" -- \
        "-runs=$FUZZ_RUNS" \
        "-seed=$2" \
        "-artifact_prefix=$OUTPUT_ROOT/artifacts/$target_name/"
}

mkdir -p "$OUTPUT_ROOT/artifacts/protocol_command" \
    "$OUTPUT_ROOT/artifacts/protocol_session_transaction" \
    "$OUTPUT_ROOT/artifacts/protocol_event" \
    "$OUTPUT_ROOT/artifacts/protocol_response"
readonly TARGET_COUNT=4
run_target protocol_command 557075001
run_target protocol_session_transaction 557075002
run_target protocol_event 557075003
run_target protocol_response 557075004

# `new_executions` is measured, not asserted: it is exactly `-runs` times the number of targets
# this invocation actually ran above. The prior fields ("invocation", "prior_executions",
# "cumulative_executions") were constants presented as measurements -- this script has no
# persisted counter of prior runs to derive them from, so they are dropped rather than faked.
printf '{"issue":"005","toolchain":"%s","cargo_fuzz":"%s","runs_per_target":%s,"targets":%s,"new_executions":%s}\n' \
    "$FUZZ_TOOLCHAIN" "$FUZZ_VERSION" "$FUZZ_RUNS" "$TARGET_COUNT" "$((FUZZ_RUNS * TARGET_COUNT))" \
    > "$OUTPUT_ROOT/evidence.json"
