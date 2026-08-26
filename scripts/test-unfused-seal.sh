#!/usr/bin/env bash
# A policy script with no red mutation is decoration (master plan section 10, "POL").
#
# Every rule in `check-unfused-seal.sh` gets a mutation that must turn it red, and the structural
# exemption gets mutations that must stay green -- otherwise the seal could be "hardened" into
# refusing the tree it is supposed to describe.
#
# The green cases matter more here than they do for the fast dB seal. This seal's vocabulary
# includes the word `mul_add`, which appears in prose all over the workspace precisely *because*
# the operation was removed: `lane/src/lib.rs` explains why the fusion is gone, `lane_math.rs`
# records the precedent, and the checker itself lists every spelling. A seal that could be tripped
# by documenting it would be unusable.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
policy="$root/scripts/check-unfused-seal.sh"

scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

passed=0
failed=0

# Builds a minimal synthetic workspace that the real checker passes: the two dispatch points
# stating the unfused contract, a retired `softfma.rs`, and both registered exemptions -- the
# audit's seven marked fused calls and gate G5's one.
create_fixture() {
    local tree
    tree="$(mktemp -d "$scratch_root/fixture-XXXXXX")"

    mkdir -p "$tree/crates/miso-engine-lane/src" "$tree/tools/miso-engine-audit/src" "$tree/hosts"

    cat >"$tree/crates/miso-engine-lane/src/wide_impl.rs" <<'EOF'
//! One `Lane` body for both `wide` widths. `mul_add` is never forwarded.
macro_rules! impl_lane_for_wide {
    ($simd:ty, $uint:ty, $width:literal, $cascade_depth:literal) => {
        impl $crate::Lane for $simd {
            #[inline(always)]
            fn fma(self, b: Self, c: Self) -> Self {
                // Two roundings, natively, on every backend.
                (self * b) + c
            }
        }
    };
}
EOF

    cat >"$tree/crates/miso-engine-lane/src/scalar.rs" <<'EOF'
//! The scalar oracle. `f32::mul_add` is deliberately not called.
impl Lane for f32 {
    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self {
        (self * b) + c
    }
}
EOF

    cat >"$tree/crates/miso-engine-lane/src/softfma.rs" <<'EOF'
//! The MXCSR helpers gate G6 needs. The software FMA was retired in #163 phase 2.
pub const MXCSR_FTZ: u32 = 0x8000;
EOF

    # Seven fused calls, two of them on one line, each within six lines of a marker.
    cat >"$tree/tools/miso-engine-audit/src/unfused_fma.rs" <<'EOF'
//! The audit. Keeps the retired fused arm so the unfused contract can be measured against it.
fn step_fused(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    let d1 = a.mul_add(b, c);
    // UNFUSED-SEAL-EXEMPT
    let d2 = a.mul_add(c, b);
    // UNFUSED-SEAL-EXEMPT (two calls)
    let d3 = a.mul_add(d1, b.mul_add(d2, c));
    d3
}
fn one_pole_fused(c: f32, x: f32, y: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    c.mul_add(x - y, y)
}
fn mix_fused(x: f32, g: f32, m: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    m.mul_add(g, x)
}
fn matrix_fused(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    a.mul_add(b, c)
}
EOF
    mkdir -p "$tree/tools/miso-engine-wasm-gates/tests"
    cat >"$tree/tools/miso-engine-wasm-gates/tests/g5_native_corpus.rs" <<'EOF'
//! Gate G5's native leg. Keeps a fused reference so the `lane_fma` case cannot pass vacuously.
fn fused_reference(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    a.mul_add(b, c)
}
EOF
    printf '%s\n' "$tree"
}

expect_success() {
    local name=$1 tree=$2
    if bash "$policy" "$tree" >/dev/null 2>&1; then
        printf 'green ok   %s\n' "$name"
        passed=$((passed + 1))
    else
        printf 'GREEN FAIL %s -- the seal refuses a tree it should accept\n' "$name" >&2
        bash "$policy" "$tree" 2>&1 | sed 's/^/           /' >&2
        failed=$((failed + 1))
    fi
}

expect_failure() {
    local name=$1 tree=$2
    if bash "$policy" "$tree" >/dev/null 2>&1; then
        printf 'RED FAIL   %s -- the seal accepted a tree it should refuse\n' "$name" >&2
        failed=$((failed + 1))
    else
        printf 'red ok     %s\n' "$name"
        passed=$((passed + 1))
    fi
}

# -------------------------------------------------------------------------------------------
# Green: the synthetic tree, and the real one.
# -------------------------------------------------------------------------------------------
expect_success baseline-synthetic "$(create_fixture)"
expect_success baseline-real-tree "$root"

# Green: prose may name the vocabulary freely. This is the case the seal must not regress on.
tree=$(create_fixture)
cat >>"$tree/crates/miso-engine-lane/src/lib.rs" <<'EOF'
//! Fusion exists nowhere. `mul_add(` is not called; `f32::mul_add(a, b, c)` would be a call, and
//! `_mm256_fmadd_ps(x, y, z)` would be another. Writing about them is not calling them.
EOF
expect_success prose-may-name-the-vocabulary "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 1 -- a dispatch point that stops stating the unfused contract.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
sed -i 's/(self \* b) + c/self.something(b, c)/' "$tree/crates/miso-engine-lane/src/scalar.rs"
expect_failure scalar-dispatch-no-longer-unfused "$tree"

tree=$(create_fixture)
sed -i 's/(self \* b) + c/self * b + c/' "$tree/crates/miso-engine-lane/src/wide_impl.rs"
expect_failure wide-dispatch-loses-its-parentheses "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 2 -- the contract split by target. This is the mutation that matters most: it is the
# tempting change, and no single-target test would catch it.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
python3 - "$tree" <<'PY'
import sys, pathlib
p = pathlib.Path(sys.argv[1]) / "crates/miso-engine-lane/src/wide_impl.rs"
s = p.read_text().replace(
    "                (self * b) + c\n",
    '                #[cfg(target_arch = "x86_64")]\n'
    "                {\n"
    "                    self.mul_add(b, c)\n"
    "                }\n"
    "                (self * b) + c\n",
)
p.write_text(s)
PY
expect_failure contract-split-by-target "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 3 -- a fused call in an unregistered file.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
mkdir -p "$tree/crates/miso-engine-compressor/src"
cat >"$tree/crates/miso-engine-compressor/src/kernel.rs" <<'EOF'
fn detector(a: f32, b: f32, c: f32) -> f32 { a.mul_add(b, c) }
EOF
expect_failure unregistered-fused-call "$tree"

# The same, through a raw intrinsic rather than the method.
tree=$(create_fixture)
mkdir -p "$tree/crates/miso-engine-compressor/src"
cat >"$tree/crates/miso-engine-compressor/src/kernel.rs" <<'EOF'
unsafe fn detector(a: __m256, b: __m256, c: __m256) -> __m256 { _mm256_fmadd_ps(a, b, c) }
EOF
expect_failure unregistered-fused-intrinsic "$tree"

# And through wasm relaxed SIMD, which may fuse at the engine's discretion.
tree=$(create_fixture)
mkdir -p "$tree/crates/miso-engine-compressor/src"
cat >"$tree/crates/miso-engine-compressor/src/kernel.rs" <<'EOF'
fn detector(a: v128, b: v128, c: v128) -> v128 { f32x4_relaxed_madd(a, b, c) }
EOF
expect_failure unregistered-relaxed-madd "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 4 -- registry rot, wrong count, missing marker.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
rm "$tree/tools/miso-engine-audit/src/unfused_fma.rs"
expect_failure registered-exemption-deleted "$tree"

tree=$(create_fixture)
cat >>"$tree/tools/miso-engine-audit/src/unfused_fma.rs" <<'EOF'
fn extra(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    a.mul_add(b, c)
}
EOF
expect_failure exemption-call-count-grew "$tree"

tree=$(create_fixture)
sed -i '0,/    \/\/ UNFUSED-SEAL-EXEMPT$/{/    \/\/ UNFUSED-SEAL-EXEMPT$/d}' \
    "$tree/tools/miso-engine-audit/src/unfused_fma.rs"
expect_failure exemption-call-without-marker "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 6 -- the retired emulation regrows in the file that kept its name.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
cat >>"$tree/crates/miso-engine-lane/src/softfma.rs" <<'EOF'
pub fn fma_f32_via_f64(a: f32, b: f32, c: f32) -> f32 {
    ((f64::from(a) * f64::from(b)) + f64::from(c)) as f32
}
EOF
expect_failure software-fma-restored "$tree"

# -------------------------------------------------------------------------------------------
printf '\nunfused seal mutations: %s passed, %s failed\n' "$passed" "$failed"
[[ "$failed" == "0" ]] || exit 1
