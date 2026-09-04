#!/usr/bin/env bash
# Prove the parametric EQ's render path keeps the issue-087 render contract. Hermetic: no cargo, no
# network, no clock.
#
# Split out of scripts/check-parametric-eq-targets.sh, which now only builds the cross-target
# matrix (delegated to scripts/check-cross-targets.sh) and calls this script. See that file's
# header for the full history of what issue #87's audit removed from this crate.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() {
    printf 'parametric-EQ render contract failure: %s\n' "$1" >&2
    exit 1
}

source=crates/parametric-eq/src

[[ -f "$source/lib.rs" ]] || fail 'crate source is missing'

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

printf 'parametric-EQ render contract: PASS (construct bans; frozen acceptance gates)\n'
