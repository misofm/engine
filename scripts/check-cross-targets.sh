#!/usr/bin/env bash
# Consolidated android/ios/wasm cross-target build matrix.
#
# Replaces the cargo/wasm-objdump halves of scripts/check-parametric-eq-targets.sh,
# scripts/check-builtins-targets.sh and scripts/check-effect-interchange-targets.sh with one script
# that runs each distinct package/target/feature combination exactly once, under one cached target
# dir per target triple (`target/ci/cross-target/<triple>`, or under `$CARGO_TARGET_DIR` if the
# caller has set it). The three original scripts are now thin wrappers that call this one, so any
# remaining caller by the old name keeps working; scripts/check-parametric-eq-targets.sh's hermetic
# render-contract half moved to scripts/check-parametric-eq-render-contract.sh instead.
#
# Not moved here (still owned by their original scripts, or already run elsewhere):
#   * scripts/check-effect-interchange-targets.sh's `cargo test -p effect-package -p effect-compiler
#     -p conformance --lib --tests` -- an exact subset of the workspace test run.
#   * scripts/test-effect-descriptor-capi.sh -- already run by scripts/check-effect-descriptor-v1.sh.
#   * The second scripts/check-effect-interchange-qualification.sh run scripts/
#     check-effect-interchange-targets.sh used to make at its own end -- this script runs it once,
#     first.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

fail() {
    printf 'cross-target check failure: %s\n' "$1" >&2
    exit 1
}

for tool in cargo rustc rustup wasm-objdump rg uname; do
    command -v "$tool" >/dev/null 2>&1 || fail "missing tool $tool"
done
[[ "$(uname -s)" == Linux ]] || fail 'native row requires Linux'
host_triple="$(rustc -vV | sed -n 's/^host: //p')"
[[ "$host_triple" == x86_64-unknown-linux-gnu ]] || fail 'native row requires x86_64 Linux host'
for target in x86_64-unknown-linux-gnu aarch64-linux-android aarch64-apple-ios wasm32-unknown-unknown; do
    rustup target list --installed | rg -qx "$target" || fail "required target unavailable: $target"
done

# Issue #081's static/qualification half, exactly once (scripts/check-effect-interchange-targets.sh
# used to run this at both its own start and its own end).
bash scripts/check-effect-interchange-qualification.sh . >/dev/null

# scripts/check-effect-interchange-targets.sh's `validate_wasm_exports` Wasm-export parser: sourced
# rather than duplicated, so scripts/test-effect-interchange-target-export-parser.sh's synthetic
# regression keeps exercising the one live implementation. Sourcing only defines the function --
# the file's own `[[ "${BASH_SOURCE[0]}" != "$0" ]]` guard returns before any of its own work runs.
source "$root/scripts/check-effect-interchange-targets.sh"

base_target_dir="${CARGO_TARGET_DIR:-target}/ci/cross-target"

# --- native x86-64-v3 release check: parametric-eq, builtins, builtins-compiler -----------------
# `.cargo/config.toml` pins `+avx2,+fma` for every x86_64 build in this workspace (master plan #83
# D4), so no explicit RUSTFLAGS is needed or set here. effect-package/effect-compiler/conformance
# have no native `check` row in the original three scripts -- their native coverage is the
# workspace test run, not this matrix.
CARGO_TARGET_DIR="$base_target_dir/x86_64-unknown-linux-gnu" \
    cargo check --quiet --locked --release \
    -p parametric-eq -p builtins -p builtins-compiler

for target in aarch64-linux-android aarch64-apple-ios; do
    target_dir="$base_target_dir/$target"
    # parametric-eq + builtins + builtins-compiler: release `check` (issue #087, issue #007).
    CARGO_TARGET_DIR="$target_dir" \
        cargo check --quiet --locked --release --target "$target" \
        -p parametric-eq -p builtins -p builtins-compiler
    # effect-package + effect-compiler + conformance: `check --all-targets`, debug (issue #081's
    # android/ios rows; the original script never passed --release here).
    CARGO_TARGET_DIR="$target_dir" \
        cargo check --locked --all-targets --target "$target" \
        -p effect-package -p effect-compiler -p conformance
done

for mode in scalar simd; do
    if [[ "$mode" == scalar ]]; then
        feature=-simd128
    else
        feature=+simd128
    fi
    # Every distinct RUSTFLAGS variant gets its own target dir under the wasm triple's directory,
    # so switching between scalar and simd128 never thrashes the other's fingerprints.
    target_dir="$base_target_dir/wasm32-unknown-unknown/$mode"
    flags="-C target-feature=$feature"

    # parametric-eq: release `check` (issue #087).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --release --target wasm32-unknown-unknown -p parametric-eq

    # builtins + builtins-compiler: release `build` -- the original script links here, not just
    # checks (issue #007).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo build --locked --release --target wasm32-unknown-unknown \
        -p builtins -p builtins-compiler

    # effect-package + effect-compiler + conformance: `check --all-targets`, debug (issue #081).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo check --locked --all-targets --target wasm32-unknown-unknown \
        -p effect-package -p effect-compiler -p conformance

    # effect-package cdylib object + export/SIMD assertions (issue #081's wasm row).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo rustc --locked -p effect-package --features c-abi \
        --target wasm32-unknown-unknown --lib -- --crate-type=cdylib
    wasm="$(find "$target_dir/wasm32-unknown-unknown/debug" -maxdepth 1 -name '*.wasm' -type f -print -quit)"
    [[ -n "$wasm" ]] || fail "missing effect-package Wasm object ($mode)"
    metadata="$base_target_dir/$mode.wasm-metadata.txt"
    wasm-objdump -x "$wasm" >"$metadata"
    validate_wasm_exports "$metadata" "$mode"
    if [[ "$mode" == scalar ]] && wasm-objdump -d "$wasm" | rg -q \
        'v128|f32x4|f64x2|i8x16|i16x8|i32x4|i64x2'; then
        fail 'SIMD opcode in scalar effect-package object'
    fi
done

printf 'cross-target matrix: PASS (x86-64-v3; android/ios; wasm scalar/simd128; parametric-eq, builtins, effect-interchange rows deduplicated)\n'
