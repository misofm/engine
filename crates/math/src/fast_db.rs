//! The sealed fast dB tier: `20 log10` and `10^(x/20)` for the dynamics gain path only.
//!
//! # Why this module exists, and why it is sealed
//!
//! [`crate::exp2_lane`] and [`crate::log2_lane`] are the engine's *exact* tier: Cephes
//! polynomials qualified at 2 ulp over all 2^32 `f32` inputs (gate M1). They are the right
//! answer for coefficient design, route gains and anything whose result is a pinned
//! coefficient word.
//!
//! They are the wrong answer for a dynamics detector. A compressor converts an amplitude to
//! decibels, applies a static curve, smooths the result and converts back — twice per frame per
//! channel — and every one of those decibel values is then multiplied into audio. What that
//! path needs is not 2 ulp of a logarithm; it is an error small compared with the smallest
//! change a listener or a meter can resolve. The exact tier buys ulp accuracy with polynomial
//! degree, and polynomial degree is the cost.
//!
//! This module buys the accuracy the dynamics path actually needs, and no more. It is sealed —
//! `scripts/check-fast-db-seal.sh` refuses the two names below anywhere except this file and an
//! enumerated list of *named crossings* — because a cheaper-but-less-accurate conversion is
//! exactly the kind of function that spreads. A crossing is a deliberate, reviewed decision that
//! a particular call site is a dynamics gain path; the seal is what makes adding a new one an
//! act rather than an accident.
//!
//! # The functions
//!
//! Both are `Lane`-generic, built from `Lane` basic operations only — multiply, add, subtract,
//! compare/select (inside `max`/`min`), `floor`, `exp2_int`, `frexp` — with **no fused
//! multiply-add**, exactly like the exact tier. So they run at full width on every backend, are
//! bit-identical across `Scalar`/`Simd4`/`Simd8`, and are bit-identical across x86-64, AArch64
//! and wasm32. That property is not weakened by being fast: it is the same property, proven the
//! same way, and the three wasm-gate legs check it on the rendered audio.
//!
//! # Derivation
//!
//! Both polynomials are fresh minimax (Remez) fits, not truncations of a published set.
//! Truncating a minimax polynomial destroys the property that makes it minimax, so a
//! lower-degree tier has to be refitted rather than shortened.
//!
//! Neither function folds its reduced argument. Cephes folds `exp2`'s fraction into
//! `[-0.5, 0.5]` and `log`'s mantissa about `sqrt(2)` because its published coefficients are
//! fitted there. Fitting fresh coefficients on the *unfolded* interval `[0, 1)` is both cheaper
//! (the fold is a compare, two selects and, for `log2`, a multiply) and **more accurate** at
//! equal degree: for `2^f` the relative-error weight `2^-f` is kinder on `[0, 1)` than on
//! `[-0.5, 0.5]`, and for `log2(1+t)` the interval `[0, 1)` stays further from the singularity
//! at `t = -1` than the folded `[-0.293, 0.414]` does. Dropping the fold is therefore not a
//! corner cut; it is the reason the degree could come down at all.
//!
//! ## Error bounds
//!
//! Proven by exhaustive sweep, not by sampling — `tests/f1_fast_db_bounds.rs` walks **every**
//! `f32` bit pattern in each function's operating domain and compares against the vendored `f64`
//! oracle. The same sweep is run against the *exact* tier over the same domains, because the
//! number that matters is not the fast tier's error in isolation but how much worse it is than
//! what it replaces:
//!
//! | conversion | domain | exact tier | **fast tier** | ratio |
//! |---|---|---|---|---|
//! | `gain_from_db` | `[-160, -0]` dB, 1,126,170,625 inputs | `7.020e-6` dB | **`7.431e-6` dB** | 1.06x |
//! | `gain_from_db` | `[0, 24]` dB, 1,103,101,953 inputs | `1.517e-6` dB | **`2.183e-6` dB** | 1.44x |
//! | `level_db` | `[1e-8, 16]`, 257,176,458 inputs | `1.538e-5` dB | **`2.810e-5` dB** | 1.83x |
//!
//! So the tier is under a factor of two worse than the exact tier everywhere the dynamics path
//! can reach, and never worse than `2.9e-5` dB. That is the number the observation taps'
//! semantics rest on (#143): a gain-reduction reading crossing this tier means what it meant, to
//! within `2.81e-5` dB, and `f1_fast_tier_stays_within_twice_the_exact_tier` asserts the ratio
//! rather than leaving it to be assumed.
//!
//! Both figures include the decibel scaling multiply, which is where most of the error at large
//! magnitudes comes from and which the exact tier pays identically: at `-150` dB one ulp of the
//! `exp2` argument is already worth about `1.2e-5` dB, so the two tiers converge there. The
//! polynomial's own contribution is `1.66e-7` relative for `exp2` and `2.19e-6` absolute for
//! `log2`.
//!
//! The tier is faster because it is a better-conditioned approximation problem, not because it
//! is a worse approximation.
//!
//! # Boundaries
//!
//! `docs/rulings/fast-db-tier-boundaries.md` records what this tier does *not* do: the true-peak
//! limiter, which had no decibel conversion left to replace; the shared
//! `effect_runtime::dynamics` helpers, deliberately left exact; and the isolated throughput
//! microbenchmark that under-predicted this tier's console win by a factor of seventeen.
//!
//! ## Exactness at the identity points, preserved
//!
//! The dynamics path relies on a unity stage being a *true* identity, so these hold exactly and
//! are asserted:
//!
//! * `fast_gain_from_db(+0.0) == 1.0` — `0 * LOG2_PER_DB` is `+0.0`, `floor(+0.0)` is `+0.0`,
//!   the fraction is `+0.0`, so the polynomial term vanishes structurally and the result is
//!   `1.0 * exp2_int(0)`.
//! * `fast_level_db(1.0) == +0.0`, `fast_level_db(2.0) == DB_PER_LOG2`,
//!   `fast_level_db(0.5) == -DB_PER_LOG2` — `frexp` returns a mantissa of exactly `1` at every
//!   power of two, so `t` is `+0.0` and the polynomial term vanishes structurally.
//!
//! These are properties of the *form*, not of the coefficients: no coefficient value can break
//! them, which is why they survive a refit.
//!
//! ## Behaviour at and beyond the domain
//!
//! * **`fast_gain_from_db`**: the `exp2` argument is clamped to `[-126, 127]` with the D8
//!   select form of `max`/`min`, which *swallows* NaN (`select(NaN > -126, NaN, -126)` is
//!   `-126`). The result is therefore always a finite positive gain — never `inf`, never a NaN
//!   payload wasm would canonicalise. A NaN decibel value produces `2^-126`, not a NaN.
//! * **`fast_level_db`**: the input is clamped up to `f32::MIN_POSITIVE` by the same select
//!   form, so zero, negative and NaN inputs all floor to the smallest positive normal and the
//!   result floors at `-126 * DB_PER_LOG2`, about `-758.6` dB, rather than `-inf` or NaN.
//!   Callers pass an already-rectified detector level; the clamp is the guard, not the contract.
//! * **Subnormals**: the clamp above means no subnormal ever reaches `frexp`. In
//!   `fast_gain_from_db` a subnormal *result* is possible only below about `-745` dB, far under
//!   the `-124` dB the dynamics path can request, and the canonical floating-point environment
//!   installed at every render entry (#146) fixes the flush behaviour in any case.
//! * The reductions themselves are exact: `x - floor(x)` is exact for `|x| < 2^23` (and the
//!   clamp holds `|x| <= 127`), `frexp` is exact, and `exp2_int` is an exact power of two. The
//!   only error in either function is the polynomial's plus the final rounding.

use lane::Lane;

/// `20 * log10(2)`: decibels per octave of amplitude. Rounded once, from the `f64` constant.
///
/// Bit-identical to `effect_runtime::dynamics::DB_PER_LOG2` and to the gate and
/// transient-shaper spellings of the same number; `0x40c0_a8c1`.
const DB_PER_LOG2: f32 = (20.0_f64 * core::f64::consts::LOG10_2) as f32;

/// `log2(10) / 20`: the inverse of [`DB_PER_LOG2`]. Rounded once, from the `f64` constant.
///
/// `0x3e2a_152d`.
const LOG2_PER_DB: f32 = (core::f64::consts::LOG2_10 / 20.0) as f32;

/// Minimax coefficients for `P` in `2^f = 1 + f * P(f)` on `[0, 1)`, **ascending** power.
///
/// Degree 4. Fresh Remez fit minimising *relative* error of the assembled `1 + f * P(f)`, which
/// is why the basis carries the `2^-f` weight and the constant term is not free: forcing the
/// value at `f = 0` to be exactly `1` is what makes `fast_gain_from_db(0)` exactly `1`.
const EXP2_P: [f32; 5] = [
    0.693_151_3,   // 0x3f31_725d
    0.240_164_44,  // 0x3e75_edab
    0.055_799_913, // 0x3d64_8e73
    0.009_017_031, // 0x3c13_bc2b
    0.001_867_13,  // 0x3af4_ba7d
];

/// Minimax coefficients for `Q` in `log2(1 + t) = t * Q(t)` on `[0, 1)`, **ascending** power.
///
/// Degree 5. Fresh Remez fit minimising *absolute* error of the assembled `t * Q(t)` — absolute
/// rather than relative because the result is added to an exact integer exponent and then scaled
/// to decibels, so it is the absolute error that becomes a decibel error. The leading `t` factor
/// is what makes `fast_level_db` exact at every power of two.
const LOG2_Q: [f32; 6] = [
    1.442_553_2,   // 0x3fb8_a595
    -0.718_281_9,  // 0xbf37_e153
    0.458_270_82,  // 0x3eea_a279
    -0.279_538_15, // 0xbe8f_1fa0
    0.123_451_486, // 0x3dfc_d422
    -0.026_457_45, // 0xbcd8_bd4b
];

/// `2^x`, lane-wide, fast tier. Private: the seal exposes only the decibel conversions.
///
/// Operation order, frozen (any change re-opens the exhaustive bound test):
/// clamp to `[-126, 127]`; `xi = floor(x)`; `f = x - xi`; five-term Horner in `f` with mul/add;
/// `p = 1 + f * p`; `p * exp2_int(xi)`.
#[inline(always)]
fn fast_exp2<L: Lane>(x: L) -> L {
    let x = x.max(L::splat(-126.0)).min(L::splat(127.0));
    let xi = x.floor();
    let f = x.sub(xi);

    let mut p = L::splat(EXP2_P[4]);
    let mut index = 3;
    loop {
        p = p.mul(f).add(L::splat(EXP2_P[index]));
        if index == 0 {
            break;
        }
        index -= 1;
    }
    let p = L::splat(1.0).add(f.mul(p));

    p.mul(L::exp2_int(xi))
}

/// `log2(x)`, lane-wide, fast tier, for positive `x`. Private: see [`fast_exp2`].
///
/// Operation order, frozen (any change re-opens the exhaustive bound test):
/// clamp up to `f32::MIN_POSITIVE`; `(m, e) = frexp(x)` with `m` in `[1, 2)`; `t = m - 1`;
/// six-term Horner in `t` with mul/add; `e + t * q`.
#[inline(always)]
fn fast_log2<L: Lane>(x: L) -> L {
    let x = x.max(L::splat(f32::MIN_POSITIVE));
    let (m, e) = x.frexp();
    let t = m.sub(L::splat(1.0));

    let mut q = L::splat(LOG2_Q[5]);
    let mut index = 4;
    loop {
        q = q.mul(t).add(L::splat(LOG2_Q[index]));
        if index == 0 {
            break;
        }
        index -= 1;
    }

    e.add(t.mul(q))
}

/// Amplitude to level: `20 * log10(|x|)`, lane-wide, **fast tier**.
///
/// A named crossing of the seal is required to call this. See the module documentation for the
/// error bound, the domain analysis and the exactness guarantees.
///
/// Frozen operation order: `fast_log2(x_abs) * DB_PER_LOG2`.
#[inline(always)]
pub fn fast_level_db<L: Lane>(x_abs: L) -> L {
    fast_log2(x_abs).mul(L::splat(DB_PER_LOG2))
}

/// Level to amplitude: `10^(db / 20)`, lane-wide, **fast tier**.
///
/// A named crossing of the seal is required to call this. See the module documentation for the
/// error bound, the domain analysis and the exactness guarantees.
///
/// Frozen operation order: `fast_exp2(db * LOG2_PER_DB)`.
#[inline(always)]
pub fn fast_gain_from_db<L: Lane>(db: L) -> L {
    fast_exp2(db.mul(L::splat(LOG2_PER_DB)))
}
