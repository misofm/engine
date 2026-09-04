#!/usr/bin/env bash
# Usage: check-effect-contract.sh [path/to/bench]
#
# WP-3 (#359): drops `cargo test --locked -p effect-contract -p effect-compiler -p conformance` and
# the per-effect `cargo test --locked -p <effect> --test conformance` loop that used to close this
# script -- both are exact subsets of the workspace test run. The `bench effect-contract
# --conformance` step stays (nothing else runs it) and now accepts a prebuilt `bench` binary as this
# script's first positional argument, falling back to `cargo run --locked --release -p bench` when
# none is given.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# S3: resolve a relative bench path against the caller's cwd BEFORE `cd "$root"` below.
bench_binary="${1:-}"
if [[ -n "$bench_binary" ]]; then
    case "$bench_binary" in
        /*) : ;;
        *) bench_binary="$(realpath -m -- "$bench_binary")" ;;
    esac
    # S1/S2: an explicit path must be an existing executable file, never a directory or a missing
    # path -- and being explicit-but-missing is a hard error, never a fallback trigger (the only
    # fallback this script has, `cargo run`, only ever fires when no path is given at all).
    [[ -f "$bench_binary" && -x "$bench_binary" ]] || {
        printf 'effect contract failure: missing bench binary %s\n' "$bench_binary" >&2
        exit 1
    }
fi

cd "$root"
bash scripts/check-effect-runtime-policy.sh .
bash scripts/test-effect-runtime-policy.sh .
bash scripts/check-effect-runtime-fixtures.sh .
bash scripts/test-effect-runtime-fixtures.sh .

# #105 phase 2 F1: the conformance harness runs against EVERY production `NativeEffectFactory`,
# not just against its own reference mock. This loop is what stops the ninth effect from shipping
# without one -- the completeness statement lives here rather than in a hand-maintained list, so a
# new effect crate is failing until it carries the test.
#
# Two directories are deliberately not products:
#   * `crates/conformance` owns the reference mock the harness validates itself with;
#   * `crates/graph-compiler`'s factories are `#[cfg(test)]` mocks inside its own unit
#     tests (bank-bind failure and scalar-only fallbacks), not effects anybody can instantiate.
conformance_crates=()
while IFS= read -r source; do
    crate_dir="${source%%/src/*}"
    case "$crate_dir" in
        crates/conformance | crates/graph-compiler) continue ;;
    esac
    conformance_crates+=("$crate_dir")
    if [[ ! -f "$crate_dir/tests/conformance.rs" ]]; then
        printf 'effect contract failure: missing %s/tests/conformance.rs\n' "$crate_dir" >&2
        printf 'every production NativeEffectFactory must run the shared harness (#105 F1)\n' >&2
        exit 1
    fi
    if ! grep -q 'effect_conformance_test!' "$crate_dir/tests/conformance.rs"; then
        printf 'effect contract failure: %s/tests/conformance.rs does not invoke effect_conformance_test!\n' \
            "$crate_dir" >&2
        exit 1
    fi
done < <(rg -l 'impl NativeEffectFactory for' crates --glob 'crates/*/src/**.rs' | LC_ALL=C sort)
[[ ${#conformance_crates[@]} -ge 8 ]] || {
    printf 'effect contract failure: found only %s production effect factories\n' \
        "${#conformance_crates[@]}" >&2
    exit 1
}

# B1: `bench effect-contract --conformance` prints one JSON record on success; assert its shape
# so a stand-in binary (or a stale one) that exits 0 with nothing meaningful is caught rather than
# trusted.
if [[ -n "$bench_binary" ]]; then
    conformance_output="$("$bench_binary" effect-contract --conformance)"
else
    conformance_output="$(cargo run --locked --release -q -p bench -- effect-contract --conformance)"
fi
printf '%s\n' "$conformance_output"
jq -e '.schema_version == 1 and .kind == "effect_conformance" and .launch_failed_gates == 0' \
    <<<"$conformance_output" >/dev/null || {
    printf 'effect contract failure: bench conformance output missing expected record\n' >&2
    exit 1
}
printf 'effect runtime contract/conformance: ok (%s production factories on the harness)\n' \
    "${#conformance_crates[@]}"
