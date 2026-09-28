#!/usr/bin/env bash
# Consolidated native/wasm cross-target build matrix.
#
# Native AArch64 is an official target again (#1017, owner ruling 2026-09-28,
# docs/rulings/engine-footprint-2026-09-28.md; it reverses #378): iOS arm64 and Android arm64-v8a
# embed the engine through the C ABI. The two AArch64 rows below check and lint every crate a
# mobile app links. The tests themselves run on arm64 hardware in qualification.yml's
# `aarch64-debug` and `aarch64-release` jobs; this script only compiles.
#
# 32-bit targets are refused at compile time (#1041, owner ruling 2026-09-28: mobile ships 64-bit
# only). The armv7-linux-androideabi row below is a refusal row: it passes only while `lane`
# fails to compile for that target with its 64-bit-only message.
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

[[ $# -eq 0 ]] || { printf 'usage: check-cross-targets.sh\n' >&2; exit 2; }

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
for target in x86_64-unknown-linux-gnu wasm32-unknown-unknown armv7-linux-androideabi \
    aarch64-apple-ios aarch64-linux-android; do
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

# --- 64-bit only: lane refuses 32-bit ARM Android (#1041) ----------------------------------------
# Owner ruling 2026-09-28 (docs/rulings/engine-footprint-2026-09-28.md): iOS arm64 and Android
# arm64-v8a ship, armeabi-v7a does not, and a 32-bit build is refused at compile time. Deleting or
# loosening the guard in crates/lane/src/lib.rs turns this row red. `cargo check` links nothing, so
# no NDK is needed; only the target's standard library.
refusal_log="$base_target_dir/armv7-linux-androideabi.refusal.log"
mkdir -p "$base_target_dir"
if CARGO_TARGET_DIR="$base_target_dir/armv7-linux-androideabi" \
    cargo check --quiet --locked --target armv7-linux-androideabi -p lane 2>"$refusal_log"; then
    fail 'lane compiled for armv7-linux-androideabi: the 64-bit-only guard (#1041) is gone'
fi
rg -qF 'lane supports 64-bit targets only' "$refusal_log" ||
    fail "armv7-linux-androideabi failed without lane's 64-bit-only message (#1041); see $refusal_log"

# --- native AArch64: iOS arm64 and Android arm64-v8a (#1017) ----------------------------------
# The product crates (scripts/lib/product-crates.sh): every workspace crate in `capi`'s
# normal-dependency closure, what an iOS or Android app links through the C ABI. Each row checks
# them with `--all-targets --all-features` and then lints them with clippy `-D warnings`.
# `cargo check` links nothing, so neither Xcode nor the NDK is needed; only each target's standard
# library.
source "$root/scripts/lib/product-crates.sh"
product_list="$(product_crates "$root")" || fail "could not derive the product crates from capi"
product_packages=()
while read -r crate; do
    product_packages+=(-p "$crate")
done <<<"$product_list"

aarch64_row() {
    local target=$1
    CARGO_TARGET_DIR="$base_target_dir/$target" \
        cargo check --quiet --locked --all-targets --all-features --target "$target" \
        "${product_packages[@]}"
    CARGO_TARGET_DIR="$base_target_dir/$target" \
        cargo clippy --quiet --locked --all-targets --all-features --target "$target" \
        "${product_packages[@]}" -- -D warnings
}

aarch64_row aarch64-apple-ios
aarch64_row aarch64-linux-android

# --- known defect #1018, an expected failure by name: `ios-asm-memset-pattern16` ----------------
# On Apple targets LLVM lowers a stored splat (the SVF flush's `L::splat(FLUSH_EPS)` among others) to
# `bl _memset_pattern16`, a libc call inside the EQ and builtin render kernels, which the realtime
# rules forbid. This check counts every such call in the iOS release assembly of `parametric-eq`
# and `builtins`. While #1018 is open the count must be nonzero; when it reaches zero the check
# fails and says so, so that the fix removes this marker and the register entry in
# docs/TARGET_MATRIX.md together. #1018 owns the render-precise replacement gate. A fresh `--emit`
# path per run keeps cargo from treating the crate as fresh and skipping the assembly.
asm_out="$(mktemp -d)"
trap 'rm -rf "$asm_out"' EXIT
memset_calls=0
for crate in parametric-eq builtins; do
    CARGO_TARGET_DIR="$base_target_dir/aarch64-apple-ios-asm" \
        cargo rustc --quiet --locked --release --target aarch64-apple-ios -p "$crate" --lib \
        -- --emit "asm=$asm_out/$crate.s"
    [[ -s "$asm_out/$crate.s" ]] || fail "no iOS release assembly for $crate"
    count="$(rg -c '^\tbl\t_memset_pattern16$' "$asm_out/$crate.s" || true)"
    count="${count:-0}"
    printf 'ios-asm-memset-pattern16: %s: %s calls\n' "$crate" "$count"
    memset_calls=$((memset_calls + count))
done
if ((memset_calls == 0)); then
    fail 'expected failure ios-asm-memset-pattern16 (#1018) now passes: remove it from scripts/check-cross-targets.sh and from the register in docs/TARGET_MATRIX.md'
fi
printf 'ios-asm-memset-pattern16: expected failure (#1018), %d calls\n' "$memset_calls"

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
        cargo build --quiet --locked --release --target wasm32-unknown-unknown \
        -p builtins -p builtins-compiler

    # effect-package + effect-compiler: `check --all-targets`, debug (issue #081). Split from the
    # conformance row below (N1): effect-package ships a cdylib, so one invocation naming it
    # together with the evidence crate conformance would unify conformance's feature/dependency
    # edges into the artifact build -- exactly what scripts/check-artifact-evidence-leak.sh exists
    # to catch. Both invocations share $target_dir, so the split costs no extra fetch/compile work
    # on an incremental rerun.
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --all-targets --target wasm32-unknown-unknown \
        -p effect-package -p effect-compiler
    # conformance: `check --all-targets`, debug -- evidence-only, no shipped package (N1).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --all-targets --target wasm32-unknown-unknown \
        -p conformance

    # effect-package cdylib object + export/SIMD assertions (issue #081's wasm row).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo rustc --quiet --locked -p effect-package --features c-abi \
        --target wasm32-unknown-unknown --lib -- --crate-type=cdylib
    # Named explicitly rather than globbed (N3): `cargo rustc --crate-type=cdylib -p effect-package`
    # always produces exactly this path, and a persistent (non-mktemp) target dir means a stale
    # object from a different commit could otherwise win a `find -print -quit` race.
    wasm="$target_dir/wasm32-unknown-unknown/debug/effect_package.wasm"
    [[ -f "$wasm" ]] || fail "missing effect-package Wasm object ($mode)"
    metadata="$base_target_dir/$mode.wasm-metadata.txt"
    wasm-objdump -x "$wasm" >"$metadata"
    validate_wasm_exports "$metadata" "$mode"
    if [[ "$mode" == scalar ]]; then
        # Captured to a variable first (N2) so `set -e` semantics apply to `wasm-objdump` itself --
        # inside an `if` condition (or the left side of a pipeline under only `pipefail`) a failing
        # objdump would otherwise make the SIMD ban silently vacuous rather than fatal.
        disasm="$(wasm-objdump -d "$wasm")"
        if printf '%s' "$disasm" | rg -q 'v128|f32x4|f64x2|i8x16|i16x8|i32x4|i64x2'; then
            fail 'SIMD opcode in scalar effect-package object'
        fi
    fi
done

printf 'cross-target matrix: PASS (x86-64-v3; aarch64 iOS and Android product crates checked and linted (#1017), ios-asm-memset-pattern16 an expected failure (#1018); wasm scalar/simd128; armv7 refused (#1041); parametric-eq, builtins, effect-interchange rows deduplicated)\n'
