//! Lane-wide `2^x` and `log2(x)` (master plan #83 §5.2).
//!
//! Per-sample dB↔gain conversion inside a SIMD bank cannot call the scalar layer: that would be a
//! branchy, table-driven, per-lane function call in the middle of a vector kernel. These two are
//! polynomial evaluations in `Lane` basic operations only — multiply, add, subtract, compare,
//! select, floor, `exp2_int`, `frexp` — so they run at full width, on every backend, with the same
//! operation sequence, and are bit-identical across Scalar/Simd4/Simd8 (gate M2) and across
//! targets (D5).
//!
//! **No fused multiply-add.** The Horner chains are written `p.mul(f).add(c)`, not `p.fma(f, c)`.
//! This crate reached that conclusion before the workspace did, and for the same reason: when
//! `Lane::fma` was still fused, it was an exact software emulation on wasm costing far more than
//! the accuracy it bought here — measured, an `fma` Horner improved `exp2_lane` from 1.462 to
//! 1.191 ulp, left `log2_lane` unchanged, and was weighed against a gate of 2 ulp. Mul/add was the
//! cheaper way to stay inside the gate.
//!
//! Since issue #163 phase 2 the two spellings compute the same thing — `Lane::fma` is `(a * b) + c`
//! on every backend — so this note is now a record of precedent rather than a live distinction.
//! The Horner chains keep their explicit `mul`/`add` spelling because it says what happens, and
//! because these bits are pinned by gate M2 and by the M3 corpus digests: they did **not** move
//! when the contract changed, which is one of the phase's control groups.
//!
//! **Coefficients.** Both sets are Moshier's published Cephes single-precision sets
//! (`cephes/single/exp2f.c` and `cephes/single/logf.c`), used rather than newly fitted ones so the
//! provenance is a citable reference rather than this crate's own optimiser run:
//!
//! > Stephen L. Moshier, *Cephes Mathematical Library*, single-precision routines `exp2f.c` and
//! > `logf.c`. Moshier, *Methods and Programs for Mathematical Functions*, Ellis Horwood, 1989.
//!
//! Cephes' `exp2f` set is fitted on `[-0.5, 0.5]`, so the argument reduction is Cephes' (floor,
//! then fold when the fraction exceeds one half) rather than a reduction to `[0, 1)`. Using the
//! published coefficients with a `[0, 1)` reduction costs about 10 ulp; the coefficient choice and
//! the reduction are one decision.
//!
//! Every reduction step here is exact. `x - floor(x)` is exact for `|x| < 2^23`, and `f - 1` and
//! `0.5 * m - 1` are exact by Sterbenz's lemma, so the only error is the polynomial's.
//!
//! **Accuracy (gate M1).** Exhaustively measured against the vendored `f64` `exp2`/`log2` oracle
//! over every `f32` input: `exp2_lane` at most **1.4615 ulp** (at `x = -0.4910151`, over all
//! 2,247,753,730 inputs in `[-126, 127]`), `log2_lane` at most **1.4667 ulp** (at
//! `x = 1.4082463`, over all 2,130,706,432 positive normals). Both are monotone, and
//! `exp2_lane(0) == 1.0`, `exp2_lane(1) == 2.0`, `log2_lane(1) == 0.0`, `log2_lane(2) == 1.0`,
//! `log2_lane(0.5) == -1.0` exactly. `tests/m1_exhaustive.rs` is the gate.
//!
//! **Width independence (gate M2).** `tests/m2_lane_identity.rs` checks that `Simd4` and `Simd8`
//! produce the scalar oracle's bits exactly, over a corpus that reaches both sides of the `exp2`
//! fold, both clamp rails, the `sqrt(2)` mantissa split and the NaN and subnormal inputs the
//! clamps have to absorb.

use lane::Lane;

/// Cephes `exp2f.c` polynomial for `2^f` on `[-0.5, 0.5]`, highest order first.
///
/// Transcribed verbatim from Moshier's file; the decimal strings carry more digits than `f32`
/// holds, which is how they are published.
#[allow(clippy::excessive_precision)]
const EXP2_P: [f32; 6] = [
    1.535336188319500E-4,
    1.339887440266574E-3,
    9.618437357674640E-3,
    5.550332471162809E-2,
    2.402264791363012E-1,
    6.931472028550421E-1,
];

/// Cephes `logf.c` polynomial, highest order first. Transcribed verbatim; see [`EXP2_P`].
#[allow(clippy::excessive_precision)]
const LOG2_P: [f32; 9] = [
    7.0376836292E-2,
    -1.1514610310E-1,
    1.1676998740E-1,
    -1.2420140846E-1,
    1.4249322787E-1,
    -1.6668057665E-1,
    2.0000714765E-1,
    -2.4999993993E-1,
    3.3333331174E-1,
];

/// `log2(e) - 1`, Cephes' `LOG2EA`.
#[allow(clippy::excessive_precision)]
const LOG2EA: f32 = 0.44269504088896340735992;

/// `sqrt(2)`, the fold point of Cephes' `logf` range reduction.
const SQRT2: f32 = core::f32::consts::SQRT_2;

/// `2^x`, lane-wide.
///
/// Inputs are clamped to `[-126, 127]` with the D8 select form of `max`/`min`, which sends NaN to
/// `-126`; the result is therefore always a finite positive number and never a NaN payload that
/// wasm would canonicalise (D5).
///
/// Operation order, frozen (any change re-opens gate M1):
/// clamp; `xi = floor(x)`; `f = x - xi`; fold `f > 0.5` into `xi + 1`, `f - 1`; six-term Horner in
/// `f` with mul/add; `p = 1 + f * p`; `p * exp2_int_in_range(xi)`.
#[inline(always)]
pub fn exp2_lane<L: Lane>(x: L) -> L {
    let x = x.max(L::splat(-126.0)).min(L::splat(127.0));
    let xi = x.floor();
    let f = x.sub(xi);

    // Cephes folds the fraction into [-0.5, 0.5], which is where its coefficients are fitted.
    let fold = f.gt(L::splat(0.5));
    let xi = L::select(fold, xi.add(L::splat(1.0)), xi);
    let f = L::select(fold, f.sub(L::splat(1.0)), f);

    let mut p = L::splat(EXP2_P[0]);
    let mut index = 1;
    while index < EXP2_P.len() {
        p = p.mul(f).add(L::splat(EXP2_P[index]));
        index += 1;
    }
    let p = L::splat(1.0).add(f.mul(p));

    p.mul(L::exp2_int_in_range(xi))
}

/// `log2(x)`, lane-wide, for positive `x`.
///
/// Inputs at or below `f32::MIN_POSITIVE` clamp to `f32::MIN_POSITIVE` (so the result floors at
/// `-126`), which is what keeps a silent detector from producing `-inf` or NaN. Callers guarantee
/// positive detectors; the clamp is the guard, not the contract.
///
/// Operation order, frozen (any change re-opens gate M1):
/// clamp; `(m, e) = frexp(x)` with `m` in `[1, 2)`; fold `m > sqrt(2)` into `e + 1` and
/// `0.5 * m - 1`, otherwise `m - 1`; `z = x * x`; nine-term Horner in `x` with mul/add;
/// `y = x * (z * p)`; `y = y - 0.5 * z`; then the Cephes summation
/// `((y * LOG2EA + x * LOG2EA) + y + x) + e`, whose order is load-bearing.
#[inline(always)]
pub fn log2_lane<L: Lane>(x: L) -> L {
    let x = x.max(L::splat(f32::MIN_POSITIVE));
    let (m, e) = x.frexp();

    // Cephes folds the mantissa about sqrt(2) so the polynomial argument stays near zero.
    let fold = m.gt(L::splat(SQRT2));
    let e = L::select(fold, e.add(L::splat(1.0)), e);
    let x = L::select(
        fold,
        L::splat(0.5).mul(m).sub(L::splat(1.0)),
        m.sub(L::splat(1.0)),
    );

    let z = x.mul(x);
    let mut p = L::splat(LOG2_P[0]);
    let mut index = 1;
    while index < LOG2_P.len() {
        p = p.mul(x).add(L::splat(LOG2_P[index]));
        index += 1;
    }

    let y = x.mul(z.mul(p));
    let y = y.sub(L::splat(0.5).mul(z));

    let r = y.mul(L::splat(LOG2EA));
    let r = r.add(x.mul(L::splat(LOG2EA)));
    let r = r.add(y);
    let r = r.add(x);
    r.add(e)
}
