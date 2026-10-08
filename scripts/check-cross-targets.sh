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
# fails to compile for that target with its 64-bit-only message. wasm32 without `simd128` has the
# same kind of row (#1062, decision 7 of the same ruling): its scalar rows are retired, and the
# wasm rows run only the `simd128` build that ships.
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
for target in x86_64-unknown-linux-gnu wasm32-unknown-unknown armv7-linux-androideabi \
    aarch64-apple-ios aarch64-linux-android; do
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

# --- known defect #1018, expected failures by crate: `ios-asm-memset-pattern16` -----------------
# On Apple targets LLVM's loop-idiom pass turns a loop that stores one constant `f32` pattern into
# `bl _memset_pattern16`: a Darwin-only libSystem routine reached through a lazily bound stub, the
# render-thread call #1018 forbids. (`bzero` and `memcpy` are bounded memory routines the compiler
# emits on every target and are not this defect; #1456 Amendment 1, Q3.) Until #1451 every
# lane splat was such a loop (`wide`'s `splat` is `transmute([elem; N])`, an array-repeat store
# loop); #1451 builds splats from an array literal in `Lane::splat` and `Lane::zero`, which removed
# that cause. #1456 removed the `true-peak-limiter`'s, the only ones reachable from render.
#
# The counted artifact (#1472) is the library an iPhone app links: `capi` as the release
# staticlib, which the release profile's fat LTO makes one module, so its assembly is every
# function the app ships, after inlining. A call that LTO adds or keeps is counted; code that no
# app links is not. A staticlib links nothing, so no Xcode is needed. Each call is charged to a
# crate by the assembly's DWARF inline records (the release profile keeps line tables): to the
# innermost function holding it that belongs to a product crate, so a fill inlined from `builtins`
# into `builtins-compiler` stays `builtins`'. The 5 calls left are preparation code, none
# reachable from render (rows in scripts/lib/aarch64-known-defects.py). The judge reads the
# counts: a row at zero, a count above its ceiling, a crate with calls and no row, calls charged to
# a crate outside the product closure, or a row for a crate outside it each fail here. The defect
# reads fixed only when every row is gone. Until #1472 the ratchet read each product crate's
# pre-link rlib assembly; that form counted code that LTO removes (`host-core` 4, `soft-clip` 1)
# and missed code LTO inlines across crates, and it caught nothing this count does not, since
# every product crate ships only through `capi`.
#
# The same assembly also proves that no eight-lane code is back in the iOS library (#1112).
# The phones run the 4-lane (NEON) width only, and the eight-lane items (`lane::Simd8`,
# `BankWidth::Eight` and everything instantiated at them) exist only where `avx2` is enabled. Any
# line that spells an eight-lane instantiation -- a function label, a call, or the name of a
# function inlined into a four-lane caller, which the line tables keep -- fails the row.
# `EIGHT_LANE` is the browser module's rule (scripts/check-web-audioworklet-callgraph.py).
# On 089ef456, before #1112, the per-crate scan matched eight crates (builtins 451 lines,
# multiband-compressor 255).
eight_lane='(?:f32|f64|u32|i32)x8|(?i:simd8)|transpose_tile_8'
known_defects=(python3 -B "$root/scripts/lib/aarch64-known-defects.py")
"${known_defects[@]}" --self-test >/dev/null || fail 'the known-defect judges failed their self-test'
asm_out="$(mktemp -d)"
trap 'rm -rf "$asm_out"' EXIT
CARGO_TARGET_DIR="$base_target_dir/aarch64-apple-ios-asm" \
    cargo rustc --quiet --locked --release --target aarch64-apple-ios -p capi --lib \
    --crate-type staticlib -- --emit "asm=$asm_out/ios-capi.s"
[[ -s "$asm_out/ios-capi.s" ]] || fail 'no iOS release assembly for capi'
printf '%s\n' "$product_list" >"$asm_out/products"
"${known_defects[@]}" count-memset "$asm_out/ios-capi.s" "$asm_out/products" "$asm_out/counts" ||
    fail 'could not charge the iOS memset_pattern16 calls to crates; see above'
"${known_defects[@]}" judge-memset "$asm_out/counts" "$asm_out/products" ||
    fail 'ios-asm-memset-pattern16 moved; see above'
if ios_eight="$(rg -n "$eight_lane" "$asm_out/ios-capi.s")"; then
    head -n 20 <<<"$ios_eight" >&2
    fail "eight-lane code is back in the iOS library (#1112): $(wc -l <<<"$ios_eight") lines; the phones run four lanes only, so it can never execute; gate it with target_feature = \"avx2\""
else
    status=$?
    ((status == 1)) || fail "the iOS eight-lane scan could not read the assembly (rg exit $status)"
fi

# --- no eight-lane code in the Android library (#1112) -----------------------------------------
# The same rule over the library an Android app links: `capi` as the release staticlib, which the
# release profile's fat LTO makes one module, so its assembly is every function the app ships,
# after inlining. A staticlib links nothing, so no NDK is needed. iOS and Android set the same
# width predicates, so this row differs from the iOS scan only if a regression keys eight lanes
# on the operating system. On 089ef456 it matched 832 lines and 28 function labels.
CARGO_TARGET_DIR="$base_target_dir/aarch64-linux-android-asm" \
    cargo rustc --quiet --locked --release --target aarch64-linux-android -p capi --lib \
    --crate-type staticlib -- --emit "asm=$asm_out/android-capi.s"
[[ -s "$asm_out/android-capi.s" ]] || fail 'no Android release assembly for capi'
if android_eight="$(rg -n "$eight_lane" "$asm_out/android-capi.s")"; then
    head -n 20 <<<"$android_eight" >&2
    fail "eight-lane code is back in the Android library (#1112): $(wc -l <<<"$android_eight") lines; gate it with target_feature = \"avx2\""
else
    status=$?
    ((status == 1)) || fail "the Android eight-lane scan could not read the assembly (rg exit $status)"
fi

# --- wasm32 without simd128 is refused (#1062) ----------------------------------------------------
# Owner ruling 2026-09-28, decision 7: the scalar (non-`simd128`) wasm builds and CI legs are
# retired now that #1017's AArch64 legs run, and `lane` refuses that build with the same 64-bit-only
# guard, whose supported set names wasm32 only with `simd128`. Restoring the scalar-wasm exception
# in crates/lane/src/lib.rs turns this row red.
scalar_wasm_log="$base_target_dir/wasm32-unknown-unknown.scalar-refusal.log"
if CARGO_TARGET_DIR="$base_target_dir/wasm32-unknown-unknown/scalar-refusal" \
    RUSTFLAGS='-C target-feature=-simd128' \
    cargo check --quiet --locked --target wasm32-unknown-unknown -p lane 2>"$scalar_wasm_log"; then
    fail 'lane compiled for wasm32 without simd128: the scalar-wasm exception is back (#1062)'
fi
rg -qF 'lane supports 64-bit targets only' "$scalar_wasm_log" ||
    fail "wasm32 without simd128 failed without lane's refusal message (#1062); see $scalar_wasm_log"

# --- wasm32 with simd128, the one shipped wasm build (W4-D1) -------------------------------------
# Its own target dir under the wasm triple's directory, so the refusal row's RUSTFLAGS never
# thrash these fingerprints.
wasm_target_dir="$base_target_dir/wasm32-unknown-unknown/simd"
wasm_flags='-C target-feature=+simd128'

# parametric-eq: release `check` (issue #087).
CARGO_TARGET_DIR="$wasm_target_dir" RUSTFLAGS="$wasm_flags" \
    cargo check --quiet --locked --release --target wasm32-unknown-unknown -p parametric-eq

# builtins + builtins-compiler: release `build` -- the original script links here, not just
# checks (issue #007).
CARGO_TARGET_DIR="$wasm_target_dir" RUSTFLAGS="$wasm_flags" \
    cargo build --quiet --locked --release --target wasm32-unknown-unknown \
    -p builtins -p builtins-compiler

# effect-compiler: `check --all-targets`, debug (issue #081). Kept apart from the conformance
# row below (N1) so the evidence crate's feature/dependency edges never unify into a shipped
# crate's build -- what scripts/check-artifact-evidence-leak.sh exists to catch. Both
# invocations share $wasm_target_dir, so the split costs no extra fetch/compile work on an
# incremental rerun.
CARGO_TARGET_DIR="$wasm_target_dir" RUSTFLAGS="$wasm_flags" \
    cargo check --quiet --locked --all-targets --target wasm32-unknown-unknown \
    -p effect-compiler
# conformance: `check --all-targets`, debug -- evidence-only, no shipped package (N1).
CARGO_TARGET_DIR="$wasm_target_dir" RUSTFLAGS="$wasm_flags" \
    cargo check --quiet --locked --all-targets --target wasm32-unknown-unknown \
    -p conformance

printf 'cross-target matrix: PASS (x86-64-v3; aarch64 iOS and Android product crates checked and linted (#1017), ios-asm-memset-pattern16 expected failures (#1018); no eight-lane code in the iOS or Android library (#1112); wasm simd128; armv7 and scalar wasm refused (#1041, #1062); parametric-eq, builtins, effect-compiler rows deduplicated)\n'
