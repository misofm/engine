#!/usr/bin/env bash
# Prove the parametric EQ builds for every launch target and keeps the issue-087 render contract.
#
# What this replaces: until #87 the EQ ran through `engine::arch`'s per-sample
# `process_delta_*` kernels, and this script disassembled those five symbols to confirm which
# instructions each target selected. The EQ no longer has a kernel of its own -- it composes
# `lane::kernels::svf_block`, one generic body per width -- so the instruction
# assertions now belong to `scripts/check-lane-policy.sh`, which owns that crate. What is left here
# is what is still this crate's: it builds everywhere, and its render path contains none of the
# constructs the audit found (issue #87 F1/F3/F4/F5/F10).
#
# The script name is unchanged so CI keeps calling it.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() {
    printf 'parametric-EQ target failure: %s\n' "$1" >&2
    exit 1
}

package=parametric-eq
source=crates/parametric-eq/src

[[ -f "$source/lib.rs" ]] || fail 'crate source is missing'

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

# Cross-target builds. `lane` refuses to compile on x86 without AVX2+FMA (master plan
# D4), so the old `-avx2,-fma` probe is gone with the runtime dispatch it protected; the x86 leg is
# the workspace's own x86-64-v3 pin.
cargo check --quiet --locked --release -p "$package"
for target in aarch64-linux-android aarch64-apple-ios; do
    CARGO_TARGET_DIR="$scratch/check-$target" \
        cargo check --quiet --locked --release --target "$target" -p "$package"
done
for feature in scalar simd128; do
    if [[ "$feature" == scalar ]]; then
        flags='-C target-feature=-simd128'
    else
        flags='-C target-feature=+simd128'
    fi
    CARGO_TARGET_DIR="$scratch/check-wasm-$feature" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --release --target wasm32-unknown-unknown -p "$package"
done

# Render-path constructs the audit removed. Production source only: the acceptance gates in
# `tests/` legitimately name the bank backend enumeration and the reference oracle.
forbidden=(
    'PreparedDeltaBankKernelV1'
    'DeltaBankKernelError'
    'KernelBackendV1'
    'process_delta'
    'sanitize_sample'
    'is_normal'
    'is_subnormal'
    'mul_add'
    'core::arch'
    'std::arch'
    'is_x86_feature_detected'
)
for pattern in "${forbidden[@]}"; do
    ! rg -n --fixed-strings "$pattern" "$source" \
        || fail "render path still references $pattern"
done

# D6: transcendentals come from `math`, never the platform libm. `sqrt` stays legal.
! rg -n '\.(exp|exp2|ln|log2|log10|powf|powi|sin|cos|tan|atan|atan2|sinh|cosh|tanh)\(' "$source" \
    || fail 'render path calls a platform transcendental'

# Issue #87 F4: the delta kernel divided once per section per sample (1,024 `vdivps ymm` per W=8
# bank block). The SVF has no division at all, so the lane division must not be reachable: the only
# `/` this crate writes is in the f64 control-plane design, which runs at event rate.
! rg -n --fixed-strings '.div(' "$source" || fail 'a lane division is reachable from the render path'
! rg -n --fixed-strings 'Lane::div' "$source" || fail 'a lane division is reachable from the render path'

# The frozen acceptance evals must stay in the crate and keep their thresholds and row counts.
declare -A gates=(
    ['rows, 1_488']='the 1,488-row analytic grid'
    ['searches, 1_104']='the 1,104 frequency searches'
    ['cases, 48']='the 48 one-second impulses'
    ['sequences, 48']='the 48 million-sample sequences'
    ['RESPONSE_TOLERANCE_DB: f64 = 0.005']='the 0.005 dB analytic tolerance'
    ['ONE_SECOND_DFT_TOLERANCE_DB: f64 = 0.05']='the 0.05 dB impulse tolerance'
)
for gate in "${!gates[@]}"; do
    rg -q --fixed-strings "$gate" crates/parametric-eq/tests \
        || fail "${gates[$gate]} is no longer asserted"
done

printf 'parametric-EQ targets: PASS (x86-64-v3; android/ios; wasm scalar/simd128; render contract)\n'
