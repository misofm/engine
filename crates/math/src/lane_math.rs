//! Lane-wide `2^x` and `log2(x)` (master plan #83 §5.2).
//!
//! Per-sample dB↔gain conversion inside a SIMD bank cannot call the scalar layer: that would be a
//! branchy, table-driven, per-lane function call in the middle of a vector kernel. These two are
//! polynomial evaluations in `Lane` basic operations only — multiply, add, subtract, divide,
//! compare, select, floor, `exp2_int`, `frexp` — so they run at full width, on every backend,
//! with the same operation sequence, and are bit-identical across Scalar/Simd4/Simd8 (gate M2)
//! and across targets (D5).
//!
//! **No fused multiply-add.** Polynomial stages stay explicit as `p.mul(f).add(c)` and never call
//! `Lane::fma`. Issue #163 phase 2 made `Lane::fma` two roundings on every backend, but retaining
//! the direct multiply/add spelling keeps each algorithm's frozen operation sequence visible.
//!
//! **Coefficients.** `exp2_lane` uses Moshier's published Cephes single-precision set
//! (`cephes/single/exp2f.c`):
//!
//! > Stephen L. Moshier, *Cephes Mathematical Library*, single-precision routine `exp2f.c`.
//! > Moshier, *Methods and Programs for Mathematical Functions*, Ellis Horwood, 1989.
//!
//! Cephes' `exp2f` set is fitted on `[-0.5, 0.5]`, so the argument reduction is Cephes' (floor,
//! then move the rounded fraction to the nearest integer) rather than a reduction to `[0, 1)`.
//! Using the published coefficients with a `[0, 1)` reduction costs about 10 ulp; the coefficient
//! choice and the reduction are one decision.
//!
//! `x - floor(x)` can round for negative non-integers in `(-0.5, 0)`, producing exactly `1.0` for
//! some inputs; the rounded fraction remains in `[0, 1]`. The magic-constant round maps it to the
//! nearest integer, with ties-to-even matching the old strict `f > 0.5` fold. When that result is
//! one, `f - 1` is exact by Sterbenz's lemma. The former `0.5 * m - 1` reduction in `log2_lane`
//! was also exact by Sterbenz's lemma.
//!
//! `log2_lane` now uses the owner-approved L3 form. After `frexp` and the `sqrt(2)` fold, let
//! `t = m - 1` and `s = t / (t + 2)`. Since `1 + t = (1 + s) / (1 - s)`,
//! `ln(1 + t) = 2 atanh(s) = 2s + 2s^3/3 + 2s^5/5 + …`. The committed degree-3 polynomial
//! `P3(z)`, where `z = s²`, approximates the residual
//! `2 atanh(s) - 2s = s · z · P3(z)`. With `r = z · P3(z)` and `u = s · (t - r)`, this gives
//! `ln(1 + t) = t - u`. The return is evaluated in the frozen unfused order
//! `t·(log2(e)-1) - u·log2(e) + t + e` so exact powers of two keep exact results.
//!
//! L3's f32 coefficients were derived by an LP minimax fit on that residual, rounded and refit in
//! sequential f32 arithmetic, then coordinate-searched by the full two-rounding evaluator. The
//! committed words and fit method are recorded in `docs/issue880-mb1.md`; the fit is provenance,
//! while the exhaustive M1 sweep is the accuracy proof. Its reduced `t` interval is
//! `[-0.292893…, 0.414214…]`, keeping `s` bounded away from the `atanh` singularity at ±1.
//!
//! **Division audit.** L3 adds one `Lane::div` per `log2_lane` call for `t / (t + 2)`; the
//! denominator lies in `[1.7071…, 2.4143…]`. A source census of production lane divisions at
//! this checkpoint found this call, true-peak limiter required-gain and box-mean divisions,
//! soft-clip's cubic `/ 3`, and the transient-shaper fast/slow envelope ratio. The gate-expander
//! corpus also calls `div`, but only while constructing test inputs. No other production lane
//! division was found; each render-path use should be audited before adding another.
//!
//! **Accuracy (gate M1).** Exhaustively measured against the vendored `f64` `exp2`/`log2` oracle
//! over every `f32` input: `exp2_lane` at most **1.4615 ulp** (at `x = -0.4910151`, over all
//! 2,247,753,730 inputs in `[-126, 127]`), `log2_lane` at most **1.2983 ulp** (at
//! `x = 0.7106287`, over all 2,130,706,432 positive normals). Both are monotone, and
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

/// L3's f32 minimax fit for the atanh residual, highest order first.
///
/// `log2_lane`'s fold and transform are derived in the module documentation. These exact f32
/// words are the committed LP-minimax/refit/coordinate-search result recorded in
/// `docs/issue880-mb1.md`.
const LOG2_L3_P: [f32; 3] = [
    f32::from_bits(0x3e99_004a),
    f32::from_bits(0x3ecc_aefc),
    f32::from_bits(0x3f2a_aab1),
];

/// `log2(e) - 1`, rounded to its committed f32 word.
const LOG2_E_MINUS_ONE: f32 = f32::from_bits(0x3ee2_a8ed);

/// `log2(e)`, rounded to its committed f32 word.
const LOG2_E: f32 = f32::from_bits(0x3fb8_aa3b);

/// `sqrt(2)`, the fold point of L3's mantissa range reduction.
const SQRT2: f32 = core::f32::consts::SQRT_2;

/// `2^x`, lane-wide.
///
/// Inputs are clamped to `[-126, 127]` with the D8 select form of `max`/`min`, which sends NaN to
/// `-126`; the result is therefore always a finite positive number and never a NaN payload that
/// wasm would canonicalise (D5).
///
/// Operation order, frozen (any change re-opens gate M1):
/// clamp; `xi = floor(x)`; `f = x - xi`; `r = (f + 12_582_912) - 12_582_912`; `xi = xi + r`;
/// `f = f - r`; six-term Horner in `f` with mul/add; `p = 1 + f * p`;
/// `p * exp2_int_in_range(xi)`.
#[inline(always)]
pub fn exp2_lane<L: Lane>(x: L) -> L {
    let x = x.max(L::splat(-126.0)).min(L::splat(127.0));
    let xi = x.floor();
    let f = x.sub(xi);

    // 1.5·2^23 rounds f ∈ [0, 1] to 0 or 1 using ties-to-even; this is exactly f > 0.5.
    // Cephes' coefficients are fitted on [-0.5, 0.5], so move the rounded unit into xi and fold f.
    let round = f.add(L::splat(12_582_912.0)).sub(L::splat(12_582_912.0));
    let xi = xi.add(round);
    let f = f.sub(round);

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
/// clamp; `(m, e) = frexp(x)` with `m` in `[1, 2)`; when `m > sqrt(2)`, fold with `0.5 * m` and
/// add one to `e`; `t = m - 1`; `s = t / (t + 2)`; `z = s * s`; degree-2 Horner over `z` with
/// `LOG2_L3_P`; `r = z * p`; `u = s * (t - r)`; then
/// `((t * LOG2_E_MINUS_ONE - u * LOG2_E) + t) + e`, whose order is load-bearing.
#[inline(always)]
pub fn log2_lane<L: Lane>(x: L) -> L {
    let x = x.max(L::splat(f32::MIN_POSITIVE));
    let (m, e) = x.frexp();

    // Fold the mantissa about sqrt(2) so the atanh argument stays in a compact interval.
    let fold = m.gt(L::splat(SQRT2));
    let t = m
        .mul(L::select(fold, L::splat(0.5), L::splat(1.0)))
        .sub(L::splat(1.0));
    let e = e.add(L::select(fold, L::splat(1.0), L::splat(0.0)));

    let s = t.div(t.add(L::splat(2.0)));
    let z = s.mul(s);
    let mut p = L::splat(LOG2_L3_P[0]);
    let mut index = 1;
    while index < LOG2_L3_P.len() {
        p = p.mul(z).add(L::splat(LOG2_L3_P[index]));
        index += 1;
    }

    let r = z.mul(p);
    let u = s.mul(t.sub(r));
    let result = t
        .mul(L::splat(LOG2_E_MINUS_ONE))
        .sub(u.mul(L::splat(LOG2_E)));
    result.add(t).add(e)
}
