#!/usr/bin/env bash
# Consolidated native/wasm cross-target build matrix.
#
# Native AArch64 (android/ios) is unsupported, no claim (#378, owner ruling 2026-09-04): the
# android/ios rows this script used to build were removed. A future revival must reopen the
# deferred-defect register in docs/TARGET_MATRIX.md before restoring them.
#
# 32-bit targets are refused at compile time (#1041, owner ruling 2026-09-28: mobile ships 64-bit
# only). The armv7-linux-androideabi row below is a refusal row: it passes only while `lane`
# fails to compile for that target with its 64-bit-only message.
#
# Replaces the cargo/wasm-objdump halves of scripts/check-parametric-eq-targets.sh,
# scripts/check-builtins-targets.sh and scripts/check-effect-interchange-targets.sh with one script
# that runs each distinct package/target/feature combination exactly once, under one cached target
# dir per target triple (`target/ci/cross-target/<triple>`, or under `$CARGO_TARGET_DIR` if the
# caller has set it). The original scripts became thin wrappers that call this one, so any
# remaining caller by the old name keeps working; #1026 deleted the builtins wrapper and #1027 the
# parametric EQ wrapper (scripts/check-parametric-eq-targets.sh), neither of which had a caller left,
# and #1037 the interchange one. The parametric EQ wrapper's hermetic render-contract half moved to
# scripts/check-parametric-eq-render-contract.sh instead.
# #1037 (owner ruling R6) also removed the interchange qualification precondition and the
# effect-package cdylib object row; effect-compiler keeps its wasm `check --all-targets` row below.
set -euo pipefail

[[ $# -eq 0 ]] || { printf 'usage: check-cross-targets.sh\n' >&2; exit 2; }

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

fail() {
    printf 'cross-target check failure: %s\n' "$1" >&2
    exit 1
}

for tool in cargo rustc rustup rg uname; do
    command -v "$tool" >/dev/null 2>&1 || fail "missing tool $tool"
done
[[ "$(uname -s)" == Linux ]] || fail 'native row requires Linux'
host_triple="$(rustc -vV | sed -n 's/^host: //p')"
[[ "$host_triple" == x86_64-unknown-linux-gnu ]] || fail 'native row requires x86_64 Linux host'
for target in x86_64-unknown-linux-gnu wasm32-unknown-unknown armv7-linux-androideabi; do
    rustup target list --installed | rg -qx "$target" || fail "required target unavailable: $target"
done

base_target_dir="${CARGO_TARGET_DIR:-target}/ci/cross-target"

# --- native x86-64-v3 release check: parametric-eq, builtins, builtins-compiler -----------------
# `.cargo/config.toml` pins `+avx2,+fma` for every x86_64 build in this workspace (master plan #83
# D4), so no explicit RUSTFLAGS is needed or set here. effect-compiler/conformance have no native
# `check` row -- their native coverage is the workspace test run, not this matrix.
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

    # effect-compiler: `check --all-targets`, debug (issue #081). Kept apart from the conformance
    # row below (N1) so the evidence crate's feature/dependency edges never unify into a shipped
    # crate's build -- what scripts/check-artifact-evidence-leak.sh exists to catch. Both
    # invocations share $target_dir, so the split costs no extra fetch/compile work on an
    # incremental rerun.
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --all-targets --target wasm32-unknown-unknown \
        -p effect-compiler
    # conformance: `check --all-targets`, debug -- evidence-only, no shipped package (N1).
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --all-targets --target wasm32-unknown-unknown \
        -p conformance
done

printf 'cross-target matrix: PASS (x86-64-v3; wasm scalar/simd128; armv7 refused (#1041); parametric-eq, builtins, effect-compiler rows deduplicated; native aarch64 unsupported, see #378)\n'
