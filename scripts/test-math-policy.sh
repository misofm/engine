#!/usr/bin/env bash
# Mutation tests proving `check-math-policy.sh` actually rejects what it claims to (master plan
# #83 §10 "POL"). A policy script with no red mutation is decoration.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-math-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

# Build a fixture workspace that mirrors the real allowlist exactly: every allowlisted file exists
# with its pinned number of platform calls, and nothing else calls the platform libm.
create_fixture() {
    local root="$1"
    mkdir -p "$root/crates/miso-engine-math/src" \
        "$root/crates/miso-engine-dsp-reference/src" \
        "$root/crates/miso-engine-clean-effect/src" \
        "$root/crates/miso-engine-clean-effect/tests" \
        "$root/hosts/miso-engine-host-native/src"

    printf '%s\n' 'pub fn exp(x: f64) -> f64 { x }' \
        >"$root/crates/miso-engine-math/src/lib.rs"
    # The oracle is meant to use the platform libm.
    printf '%s\n' 'pub fn oracle(x: f64) -> f64 { x.exp() + x.ln() }' \
        >"$root/crates/miso-engine-dsp-reference/src/lib.rs"
    # Production code that already migrated: free-function calls, not methods.
    printf '%s\n' 'pub fn gain(db: f32) -> f32 { miso_engine_math::exp2f(db) }' \
        >"$root/crates/miso-engine-clean-effect/src/lib.rs"
    # Tests are allowed to compare against the platform.
    printf '%s\n' 'fn check(x: f64) -> f64 { x.exp() }' \
        >"$root/crates/miso-engine-clean-effect/tests/accuracy.rs"
    printf '%s\n' 'pub fn boot() {}' \
        >"$root/hosts/miso-engine-host-native/src/lib.rs"

    while read -r file limit _owner; do
        [[ -n "$file" ]] || continue
        mkdir -p "$root/$(dirname "$file")"
        {
            printf '%s\n' '// Pre-migration file: exactly the pinned number of platform calls.'
            local index=0
            while [[ "$index" -lt "$limit" ]]; do
                printf 'pub fn legacy_%s(x: f64) -> f64 { x.exp() }\n' "$index"
                index=$((index + 1))
            done
        } >"$root/$file"
    done < <(bash "$policy_script" --print-allowlist)
}

expect_failure() {
    local name="$1"
    local mutation="$2"
    local root="$scratch_root/$name"
    create_fixture "$root"
    eval "$mutation"
    if bash "$policy_script" "$root" >/dev/null 2>&1; then
        printf 'math policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

valid="$scratch_root/valid"
create_fixture "$valid"
bash "$policy_script" "$valid" >/dev/null

expect_failure powf-in-a-clean-crate \
    'printf "%s\n" "pub fn bad(x: f32) -> f32 { x.powf(2.0) }" >>"$root/crates/miso-engine-clean-effect/src/lib.rs"'
expect_failure ln-in-the-effect-runtime \
    'mkdir -p "$root/crates/miso-engine-effect-runtime/src"; printf "%s\n" "pub fn bad(x: f64) -> f64 { x.ln() }" >"$root/crates/miso-engine-effect-runtime/src/x.rs"'
expect_failure sin-in-a-host \
    'printf "%s\n" "pub fn bad(x: f64) -> f64 { x.sin() }" >>"$root/hosts/miso-engine-host-native/src/lib.rs"'
expect_failure new-file-next-to-an-allowlisted-one \
    'printf "%s\n" "pub fn bad(x: f64) -> f64 { x.sin() }" >"$root/crates/miso-engine-graph/src/other.rs"'
expect_failure allowlisted-file-gains-a-call \
    'printf "%s\n" "pub fn extra(x: f64) -> f64 { x.tanh() }" >>"$root/crates/miso-engine-graph/src/lib.rs"'
# Issue #95 cleared the `miso-engine-effect-contract` row. A platform call coming back to it must
# now be rejected as an unallowlisted file, not merely ratcheted.
expect_failure the-cleared-contract-row-cannot-come-back \
    'mkdir -p "$root/crates/miso-engine-effect-contract/src"; printf "%s\n" "pub fn bad(x: f32) -> f32 { x.powf(2.0) }" >"$root/crates/miso-engine-effect-contract/src/lib.rs"'
expect_failure allowlist-entry-gone-stale \
    'mkdir -p "$root/crates/miso-engine-graph-compiler/src"; printf "%s\n" "pub fn bad(x: f32) -> f32 { x.exp() }" >"$root/crates/miso-engine-graph-compiler/src/lib.rs"'
expect_failure allowlisted-file-deleted \
    'rm "$root/crates/miso-engine-conformance/src/compare.rs"'
expect_failure powi-in-a-clean-crate \
    'printf "%s\n" "pub fn bad(x: f64) -> f64 { x.powi(3) }" >>"$root/crates/miso-engine-clean-effect/src/lib.rs"'
# The sidecars/ tree ships as its own delivery artifact (issue: FLAC decoder sidecar move) and
# is scanned the same as crates/ and hosts/ (scripts/check-math-policy.sh:63) -- a platform
# transcendental in a sidecar is exactly the same cross-target bit-identity hole.
expect_failure sin-in-a-sidecar \
    'mkdir -p "$root/sidecars/probe-decoder/src"; printf "%s\n" "pub fn bad(x: f64) -> f64 { x.sin() }" >"$root/sidecars/probe-decoder/src/lib.rs"'

# The structural exemptions must keep working, or the script would be unusable.
expect_pass() {
    local name="$1"
    local mutation="$2"
    local root="$scratch_root/$name"
    create_fixture "$root"
    eval "$mutation"
    if ! bash "$policy_script" "$root" >/dev/null 2>&1; then
        printf 'math policy rejected legitimate code: %s\n' "$name" >&2
        bash "$policy_script" "$root" >&2 || true
        exit 1
    fi
}

expect_pass oracle-may-use-the-platform \
    'printf "%s\n" "pub fn more(x: f64) -> f64 { x.tanh() }" >>"$root/crates/miso-engine-dsp-reference/src/lib.rs"'
expect_pass tests-may-use-the-platform \
    'printf "%s\n" "fn other(x: f64) -> f64 { x.log10() }" >>"$root/crates/miso-engine-clean-effect/tests/accuracy.rs"'
expect_pass sqrt-stays-legal \
    'printf "%s\n" "pub fn ok(x: f64) -> f64 { x.sqrt() }" >>"$root/crates/miso-engine-clean-effect/src/lib.rs"'
expect_pass allowlisted-file-may-shrink \
    'sed -i "1a // migrated one call site" "$root/crates/miso-engine-graph/src/lib.rs"; sed -i "s/pub fn legacy_0(x: f64) -> f64 { x.exp() }/pub fn legacy_0(x: f64) -> f64 { miso_engine_math::exp(x) }/" "$root/crates/miso-engine-graph/src/lib.rs"'

printf 'math policy mutation tests: ok\n'
