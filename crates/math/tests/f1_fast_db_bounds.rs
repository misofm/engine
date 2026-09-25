#![allow(clippy::disallowed_methods)] // this is the bound gate the sealed fast tier is proven against
//! Gate F1 — the sealed fast dB tier's error bounds, exactness anchors and domain behaviour.
//!
//! The oracle is the crate's own vendored `f64` `exp2`/`log2`/`pow`: a table-driven musl
//! reduction, a completely different algorithm from the minimax polynomials under test, so
//! agreement is evidence rather than a tautology. `f64` carries 29 more significand bits than
//! `f32`, so the oracle is exact to far better than the error being measured.
//!
//! Two forms of every bound check, following gate M1:
//!
//! * `f1_*_exhaustive` walks **every** `f32` bit pattern in the function's operating domain.
//!   `#[ignore]`d, because it wants release codegen; its sweep workers are capped at four to
//!   leave CPU capacity for parallel qualification work. Run it with
//!   `cargo test --locked --release -p math --features lane --test f1_fast_db_bounds -- --ignored`.
//! * `f1_*_subsample` strides the bit pattern by 4099 (prime, so it walks every exponent and a
//!   dense spread of significands), adds the anchors and the measured worst points, and runs in
//!   the default `cargo test`.
//!
//! # The operating domains, and why these and not others
//!
//! These are not chosen for convenience; they are what the named crossings can actually produce.
//!
//! * [`fast_gain_from_db`] is called on `smoothed + makeup` (compressor, multiband) and on the
//!   gate's smoothed `gain_db`. The compressor clamps its reduction to `[-100, 0]` and its makeup
//!   parameter to `[-24, 24]`; multiband is the same; the gate's range parameter caps its target
//!   at `[-96, 0]`. The union is `[-124, 24]`. The sweep uses `[-160, 24]`, the wider interval the
//!   detector level clamps already guarantee, so the bound covers more than the crossings can ask
//!   for.
//! * [`fast_level_db`] is called on a rectified detector floored to `1e-8` by every crossing, and
//!   its result is immediately clamped into `[-160, 24]` dB, i.e. amplitudes up to about `15.85`.
//!   The sweep uses `[1e-8, 16]`.
//!
//! Behaviour *outside* those domains is a separate claim, checked by `f1_domain_and_clamping`
//! rather than by a bound: the functions must stay finite and well-defined for every `f32`,
//! including zero, negatives, subnormals, both infinities and NaN.
//!
//! # Measured on the delivery host, exhaustively, through the real `impl Lane for f32`
//!
//! | sweep | inputs | exact tier | **fast tier** | gate |
//! |---|---|---|---|---|
//! | `gain_from_db`, `[-160, -0]` dB | 1,126,170,625 | `7.020e-6` dB | **`7.431e-6` dB** | `1.0e-5` |
//! | `gain_from_db`, `[0, 24]` dB | 1,103,101,953 | `1.517e-6` dB | **`2.183e-6` dB** | `1.0e-5` |
//! | `level_db`, `[1e-8, 16]` | 257,176,458 | `1.5382e-5` dB | **`2.810e-5` dB** | `4.0e-5` |
//!
//! # Red mutations
//!
//! Each was run against the same oracle over the same domain, and each must push the measured
//! error over the gate — a bound with no red mutation is decoration. These are measured, not
//! predicted. The degree-drop mutations below deliberately truncate the shipped polynomial;
//! they do not establish the bound for a refitted lower-degree polynomial. That comparison is
//! measured separately by `f1_lower_degree_refits_exhaustive`.
//!
//! | mutation | measured | gate |
//! |---|---|---|
//! | `EXP2_P[0]` += 1e-5 | `fast_gain_from_db` `5.003e-5` dB at `-150.789` | `1.0e-5` — red |
//! | `LOG2_Q[0]` += 1e-5 | `fast_level_db` `8.524e-5` dB at `2.903e-8` | `4.0e-5` — red |
//! | drop `EXP2_P[4]` (degree 3, same coefficients) | `fast_gain_from_db` `8.115e-3` dB | `1.0e-5` — red |
//! | drop `LOG2_Q[5]` (degree 4, same coefficients) | `fast_level_db` `1.593e-1` dB | `4.0e-5` — red |
//!
//! The last two show why truncating a minimax polynomial is not a lower-degree fit. The refitted
//! candidate measurements below are the evidence about whether a lower degree can satisfy F1.
//!
//! The monotonicity assertion also has a real red mutation: changing `LOG2_Q[0]` from
//! `0x3fb8_a595` to `0x3fb8_a8dc` changed the exhaustive count from 77 to 95 and failed the
//! pinned-count assertion.
//!
//! Memory note: the sweeps iterate ranges of `u32`. Never collect the patterns.

use std::thread;

use lane::Lane;
use math::fast_db::{fast_gain_from_db, fast_level_db};

/// `20 * log10(2)`, the same constant the tier itself uses, recomputed here from `f64`.
const DB_PER_LOG2: f64 = 20.0 * core::f64::consts::LOG10_2;

/// The gate on [`fast_gain_from_db`], in decibels of the returned gain.
///
/// The exact tier's own qualified bound, as the compressor specification states it, is 2 ulp of a
/// `log2` result — about `4.6e-5` dB. The fast tier is asserted *inside* that, which is the
/// claim that makes the observation taps' semantics survive the crossing (#143).
const GAIN_MAX_DB: f64 = 1.0e-5;

/// The gate on [`fast_level_db`], in decibels. See [`GAIN_MAX_DB`].
const LEVEL_MAX_DB: f64 = 4.0e-5;

/// Decibels of error implied by a relative error on an amplitude: `20 / ln(10)`.
const DB_PER_RELATIVE: f64 = 8.685_889_638_065_035;

fn oracle_gain(db: f64) -> f64 {
    math::exp2(db * (core::f64::consts::LOG2_10 / 20.0))
}

fn oracle_level_db(x: f64) -> f64 {
    math::log2(x) * DB_PER_LOG2
}

/// The worst error seen over a sweep, and how many inputs produced it.
#[derive(Clone, Copy)]
struct Sweep {
    /// Worst error, in decibels, scaled by 2^40 for stable integer comparisons.
    worst_scaled: u64,
    /// The input bit pattern that produced it.
    worst_bits: u32,
    /// How many inputs were actually checked.
    checked: u64,
    /// Number of adjacent input pairs where the output moved opposite to the input.
    decreasing_steps: u64,
}

const SCALE: f64 = (1_u64 << 40) as f64;

/// Sweeps `[first_bits, last_bits]` (inclusive, monotone in the `f32` value) with `stride`.
struct Measurement {
    error_db: Option<f64>,
    value: f32,
}

#[derive(Clone, Copy)]
struct Edge {
    x: f32,
    y: f32,
}

struct ChunkSweep {
    worst_scaled: u64,
    worst_bits: u32,
    checked: u64,
    decreasing_steps: u64,
    first: Option<Edge>,
    last: Option<Edge>,
}

fn decreases(previous: Edge, current: Edge) -> bool {
    (previous.x < current.x && previous.y > current.y)
        || (previous.x > current.x && previous.y < current.y)
}

fn sweep(first_bits: u32, last_bits: u32, stride: u64, measure: fn(f32) -> Measurement) -> Sweep {
    // Keep exhaustive CPU work bounded while other qualification agents are active.
    let threads = thread::available_parallelism().map_or(1, |value| value.get().min(4));
    let total = u64::from(last_bits - first_bits) + 1;
    let span = total.div_ceil(threads as u64);
    let chunks = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(threads);
        for index in 0..threads {
            handles.push(scope.spawn(move || {
                let start = index as u64 * span;
                if start >= total {
                    return ChunkSweep {
                        worst_scaled: 0,
                        worst_bits: 0,
                        checked: 0,
                        decreasing_steps: 0,
                        first: None,
                        last: None,
                    };
                }
                let end = (start + span).min(total);
                // Align this thread's start to the stride so the union is exactly the strided set.
                let mut offset = start.div_ceil(stride) * stride;
                let mut local_worst = 0_u64;
                let mut local_bits = 0_u32;
                let mut local_checked = 0_u64;
                let mut local_decreasing = 0_u64;
                let mut first = None;
                let mut previous = None;
                let mut last = None;
                while offset < end {
                    let bits = first_bits + offset as u32;
                    let x = f32::from_bits(bits);
                    let Measurement { error_db, value } = measure(x);
                    let edge = Edge { x, y: value };
                    if first.is_none() {
                        first = Some(edge);
                    }
                    if stride == 1 {
                        if let Some(previous) = previous
                            && decreases(previous, edge)
                        {
                            local_decreasing += 1;
                        }
                        previous = Some(edge);
                    }
                    if let Some(error) = error_db {
                        local_checked += 1;
                        let scaled = (error * SCALE) as u64;
                        if scaled > local_worst {
                            local_worst = scaled;
                            local_bits = bits;
                        }
                    }
                    // Decreasing counts are enabled exclusively for stride 1, so every compared
                    // pair is adjacent in the original domain. `last` also marks strided chunks.
                    last = Some(edge);
                    offset += stride;
                }
                ChunkSweep {
                    worst_scaled: local_worst,
                    worst_bits: local_bits,
                    checked: local_checked,
                    decreasing_steps: local_decreasing,
                    first,
                    last,
                }
            }));
        }

        handles
            .into_iter()
            .map(|handle| handle.join().expect("F1 sweep worker panicked"))
            .collect::<Vec<_>>()
    });

    let mut result = Sweep {
        worst_scaled: 0,
        worst_bits: 0,
        checked: 0,
        decreasing_steps: 0,
    };
    let mut previous_edge = None;
    for chunk in chunks {
        if chunk.worst_scaled > result.worst_scaled {
            result.worst_scaled = chunk.worst_scaled;
            result.worst_bits = chunk.worst_bits;
        }
        result.checked += chunk.checked;
        if stride == 1 {
            result.decreasing_steps += chunk.decreasing_steps;
            // Chunks are joined in bit-range order, so this includes every worker-boundary pair.
            if let (Some(previous), Some(first)) = (previous_edge, chunk.first)
                && decreases(previous, first)
            {
                result.decreasing_steps += 1;
            }
        }
        if chunk.last.is_some() {
            previous_edge = chunk.last;
        }
    }

    result
}

/// Error of `fast_gain_from_db` at one decibel value, in decibels of the returned gain.
fn gain_error_db(db: f32) -> Measurement {
    let got = f64::from(fast_gain_from_db::<f32>(db));
    let want = oracle_gain(f64::from(db));
    let error_db = (want > 0.0 && want.is_finite() && got.is_finite())
        .then_some(((got - want) / want).abs() * DB_PER_RELATIVE);
    Measurement {
        error_db,
        value: got as f32,
    }
}

/// Error of `fast_level_db` at one amplitude, in decibels.
fn level_error_db(x: f32) -> Measurement {
    let got = f64::from(fast_level_db::<f32>(x));
    let want = oracle_level_db(f64::from(x));
    let error_db = (want.is_finite() && got.is_finite()).then_some((got - want).abs());
    Measurement {
        error_db,
        value: got as f32,
    }
}

fn assert_sweep(name: &str, sweep: Sweep, gate_db: f64, minimum_checked: u64) {
    let worst = sweep.worst_scaled as f64 / SCALE;
    assert!(
        sweep.checked >= minimum_checked,
        "{name}: only {} inputs checked, expected at least {minimum_checked}",
        sweep.checked
    );
    assert!(
        worst <= gate_db,
        "{name}: max error {worst:.6e} dB exceeds the {gate_db:.1e} dB gate, at bits {:#010x} \
         (value {})",
        sweep.worst_bits,
        f32::from_bits(sweep.worst_bits)
    );
    println!(
        "{name}: max {worst:.6e} dB over {} inputs, worst at {:#010x} ({})",
        sweep.checked,
        sweep.worst_bits,
        f32::from_bits(sweep.worst_bits)
    );
}

// The operating-domain endpoints, as bit patterns.
//
// Positive floats are monotone in their bit patterns; negative floats are monotone in *magnitude*,
// so the negative sweep runs from `-0.0` (`0x8000_0000`) up to `-160.0`, not the other way round.
// Getting this backwards underflows the span, which is how the first version of this file
// "measured" 1.7e7 dB at -884 dB -- a value outside the domain entirely.
fn gain_domain() -> (u32, u32) {
    ((-0.0_f32).to_bits(), (-160.0_f32).to_bits())
}
fn gain_domain_positive() -> (u32, u32) {
    ((0.0_f32).to_bits(), (24.0_f32).to_bits())
}
fn level_domain() -> (u32, u32) {
    ((1.0e-8_f32).to_bits(), (16.0_f32).to_bits())
}

// ---------------------------------------------------------------------------------------------
// Exactness anchors. These are properties of the *form*, not of the coefficients.
// ---------------------------------------------------------------------------------------------

#[test]
fn f1_identity_anchors_are_exact() {
    // A unity stage must be a true identity, or a bypassed dynamics slot changes the audio.
    assert_eq!(
        fast_gain_from_db::<f32>(0.0).to_bits(),
        1.0_f32.to_bits(),
        "fast_gain_from_db(+0.0) must be exactly 1.0"
    );
    assert_eq!(
        fast_gain_from_db::<f32>(-0.0).to_bits(),
        1.0_f32.to_bits(),
        "fast_gain_from_db(-0.0) must be exactly 1.0"
    );

    // frexp returns a mantissa of exactly 1 at every power of two, so `t` is +0.0 and the
    // polynomial term vanishes structurally.
    assert_eq!(
        fast_level_db::<f32>(1.0).to_bits(),
        0.0_f32.to_bits(),
        "fast_level_db(1.0) must be exactly +0.0"
    );
    let db_per_log2 = (20.0_f64 * core::f64::consts::LOG10_2) as f32;
    assert_eq!(fast_level_db::<f32>(2.0), db_per_log2);
    assert_eq!(fast_level_db::<f32>(0.5), -db_per_log2);
    assert_eq!(fast_level_db::<f32>(4.0), 2.0 * db_per_log2);
}

#[test]
fn f1_domain_and_clamping() {
    // Every f32 must produce a finite result: the dynamics path multiplies these into audio.
    for bits in [
        0x0000_0000_u32, // +0
        0x8000_0000,     // -0
        0x0000_0001,     // smallest subnormal
        0x0080_0000,     // smallest normal
        0x7f7f_ffff,     // f32::MAX
        0x7f80_0000,     // +inf
        0xff80_0000,     // -inf
        0x7fc0_0000,     // quiet NaN
        0x7f80_0001,     // signalling NaN
        0xbf80_0000,     // -1.0
    ] {
        let x = f32::from_bits(bits);
        let level = fast_level_db::<f32>(x);
        assert!(
            level.is_finite(),
            "fast_level_db({x}) [bits {bits:#010x}] returned {level}, which is not finite"
        );
        let gain = fast_gain_from_db::<f32>(x);
        assert!(
            gain.is_finite() && gain > 0.0,
            "fast_gain_from_db({x}) [bits {bits:#010x}] returned {gain}"
        );
    }

    // The documented floors, exactly.
    let floor_db = -126.0 * (20.0_f64 * core::f64::consts::LOG10_2) as f32;
    for x in [
        0.0_f32,
        -0.0,
        -1.0,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::MIN_POSITIVE,
    ] {
        assert_eq!(
            fast_level_db::<f32>(x),
            floor_db,
            "fast_level_db({x}) must floor at -126 octaves"
        );
    }
    // A NaN decibel value is swallowed by the D8 clamp to 2^-126, never propagated.
    assert_eq!(
        fast_gain_from_db::<f32>(f32::NAN),
        f32::from_bits(0x0080_0000)
    );
    assert_eq!(
        fast_gain_from_db::<f32>(f32::NEG_INFINITY),
        f32::from_bits(0x0080_0000)
    );
    assert!(fast_gain_from_db::<f32>(f32::INFINITY).is_finite());
}

// ---------------------------------------------------------------------------------------------
// Subsampled bounds — these run in the default `cargo test`.
// ---------------------------------------------------------------------------------------------

#[test]
fn f1_gain_from_db_subsample() {
    let (lo, hi) = gain_domain();
    assert_sweep(
        "fast_gain_from_db negative subsample",
        sweep(lo, hi, 4099, gain_error_db),
        GAIN_MAX_DB,
        200_000,
    );
    let (lo, hi) = gain_domain_positive();
    assert_sweep(
        "fast_gain_from_db positive subsample",
        sweep(lo, hi, 4099, gain_error_db),
        GAIN_MAX_DB,
        200_000,
    );
}

#[test]
fn f1_level_db_subsample() {
    let (lo, hi) = level_domain();
    assert_sweep(
        "fast_level_db subsample",
        sweep(lo, hi, 4099, level_error_db),
        LEVEL_MAX_DB,
        50_000,
    );
}

// ---------------------------------------------------------------------------------------------
// Exhaustive bounds — the proof. Ignored by default; see the module documentation.
// ---------------------------------------------------------------------------------------------

#[test]
#[ignore = "full-domain sweep: run with --release -- --ignored"]
fn f1_gain_from_db_exhaustive() {
    let (lo, hi) = gain_domain();
    let negative = sweep(lo, hi, 1, gain_error_db);
    assert_sweep(
        "fast_gain_from_db negative exhaustive",
        negative,
        GAIN_MAX_DB,
        1_000_000_000,
    );
    println!(
        "fast_gain_from_db negative domain: {} decreasing steps",
        negative.decreasing_steps
    );
    assert_eq!(negative.decreasing_steps, 0);
    let (lo, hi) = gain_domain_positive();
    let positive = sweep(lo, hi, 1, gain_error_db);
    assert_sweep(
        "fast_gain_from_db positive exhaustive",
        positive,
        GAIN_MAX_DB,
        1_000_000_000,
    );
    println!(
        "fast_gain_from_db positive domain: {} decreasing steps",
        positive.decreasing_steps
    );
    assert_eq!(positive.decreasing_steps, 0);
}

#[test]
#[ignore = "full-domain sweep: run with --release -- --ignored"]
fn f1_level_db_exhaustive() {
    let (lo, hi) = level_domain();
    let level = sweep(lo, hi, 1, level_error_db);
    println!(
        "fast_level_db domain: {} decreasing steps",
        level.decreasing_steps
    );
    assert_eq!(level.decreasing_steps, 77);
    assert_sweep("fast_level_db exhaustive", level, LEVEL_MAX_DB, 250_000_000);
}

// Refitted lower-degree rows recorded by issue #880 F-3. Coefficients are in Horner order:
// highest order first, matching the coefficient words printed in the issue.
const REFIT_GAIN_DEGREE3: [f32; 4] = [
    f32::from_bits(0x3c5b_f2e2),
    f32::from_bits(0x3d55_ffe6),
    f32::from_bits(0x3e77_11ca),
    f32::from_bits(0x3f31_6b63),
];
const REFIT_LEVEL_DEGREE4: [f32; 5] = [
    f32::from_bits(0x3d3e_0145),
    f32::from_bits(0xbe48_fcca),
    f32::from_bits(0x3ed5_d00c),
    f32::from_bits(0xbf35_aca2),
    f32::from_bits(0x3fb8_9252),
];

const LOG2_PER_DB_F32: f32 = (core::f64::consts::LOG2_10 / 20.0) as f32;
const DB_PER_LOG2_F32: f32 = (20.0_f64 * core::f64::consts::LOG10_2) as f32;

fn refit_gain_degree3(db: f32) -> f32 {
    let x = db.mul(f32::splat(LOG2_PER_DB_F32)).clamp(-126.0, 127.0);
    let xi = x.floor();
    let f = x.sub(xi);
    let mut p = REFIT_GAIN_DEGREE3[0];
    for coefficient in &REFIT_GAIN_DEGREE3[1..] {
        p = p.mul(f).add(*coefficient);
    }
    f32::splat(1.0)
        .add(f.mul(p))
        .mul(f32::exp2_int_in_range(xi))
}

fn refit_level_degree4(x: f32) -> f32 {
    let (m, e) = x.max(f32::MIN_POSITIVE).frexp();
    let t = m.sub(1.0);
    let mut q = REFIT_LEVEL_DEGREE4[0];
    for coefficient in &REFIT_LEVEL_DEGREE4[1..] {
        q = q.mul(t).add(*coefficient);
    }
    e.add(t.mul(q)).mul(DB_PER_LOG2_F32)
}

fn refit_gain_error_db(db: f32) -> Measurement {
    let got = f64::from(refit_gain_degree3(db));
    let want = oracle_gain(f64::from(db));
    Measurement {
        error_db: Some(((got - want) / want).abs() * DB_PER_RELATIVE),
        value: got as f32,
    }
}

fn refit_level_error_db(x: f32) -> Measurement {
    let got = f64::from(refit_level_degree4(x));
    let want = oracle_level_db(f64::from(x));
    Measurement {
        error_db: Some((got - want).abs()),
        value: got as f32,
    }
}

/// Remeasures the recorded refits over F1's actual domains and rejects both against its gates.
#[test]
#[ignore = "full-domain refit measurement: run with --release -- --ignored"]
fn f1_lower_degree_refits_exhaustive() {
    let (lo, hi) = gain_domain();
    let gain_negative = sweep(lo, hi, 1, refit_gain_error_db);
    let (lo, hi) = gain_domain_positive();
    let gain_positive = sweep(lo, hi, 1, refit_gain_error_db);
    let (lo, hi) = level_domain();
    let level = sweep(lo, hi, 1, refit_level_error_db);

    let gain_negative_db = gain_negative.worst_scaled as f64 / SCALE;
    let gain_positive_db = gain_positive.worst_scaled as f64 / SCALE;
    let level_db = level.worst_scaled as f64 / SCALE;
    assert_eq!(gain_negative.checked, 1_126_170_625);
    assert_eq!(gain_positive.checked, 1_103_101_953);
    assert_eq!(level.checked, 257_176_458);
    println!(
        "refit P degree 3: negative {gain_negative_db:.6e} dB, positive {gain_positive_db:.6e} dB"
    );
    println!("refit Q degree 4: {level_db:.6e} dB");
    assert!(gain_negative_db > GAIN_MAX_DB);
    assert!(gain_positive_db > GAIN_MAX_DB);
    assert!(level_db > LEVEL_MAX_DB);
}

// ---------------------------------------------------------------------------------------------
// The comparison that makes the crossing safe: the fast tier against the exact tier, same domain.
// ---------------------------------------------------------------------------------------------

/// The exact tier's `level_db`, spelled exactly as `effect_runtime::dynamics::level_db` spells it.
fn exact_level_db(x: f32) -> f32 {
    math::log2_lane::<f32>(x) * (20.0_f64 * core::f64::consts::LOG10_2) as f32
}

/// The exact tier's `gain_from_db`, spelled as `effect_runtime::dynamics::gain_from_db` spells it.
fn exact_gain_from_db(db: f32) -> f32 {
    math::exp2_lane::<f32>(db * (core::f64::consts::LOG2_10 / 20.0) as f32)
}

fn exact_gain_error_db(db: f32) -> Measurement {
    let got = f64::from(exact_gain_from_db(db));
    let want = oracle_gain(f64::from(db));
    let error_db = (want > 0.0 && want.is_finite() && got.is_finite())
        .then_some(((got - want) / want).abs() * DB_PER_RELATIVE);
    Measurement {
        error_db,
        value: got as f32,
    }
}

fn exact_level_error_db(x: f32) -> Measurement {
    let got = f64::from(exact_level_db(x));
    let want = oracle_level_db(f64::from(x));
    let error_db = (want.is_finite() && got.is_finite()).then_some((got - want).abs());
    Measurement {
        error_db,
        value: got as f32,
    }
}

/// The crossing is safe because the fast tier is *not much worse* than what it replaces.
///
/// This is the assertion the observation taps rest on (#143 / issue #149 requirement C4). A
/// gain-reduction reading that crosses the fast tier still means what it meant, and the amount by
/// which it may differ is bounded here against the exact tier over the identical domain, rather
/// than assumed from the absolute gates above.
#[test]
#[ignore = "full-domain sweep: run with --release -- --ignored"]
fn f1_fast_tier_stays_within_twice_the_exact_tier() {
    /// How much worse than the exact tier the fast tier is allowed to be, over the same domain.
    const RATIO: f64 = 2.0;

    for (name, (lo, hi), fast, exact) in [
        (
            "gain_from_db negative",
            gain_domain(),
            gain_error_db as fn(f32) -> Measurement,
            exact_gain_error_db as fn(f32) -> Measurement,
        ),
        (
            "gain_from_db positive",
            gain_domain_positive(),
            gain_error_db,
            exact_gain_error_db,
        ),
        (
            "level_db",
            level_domain(),
            level_error_db,
            exact_level_error_db,
        ),
    ] {
        let fast = sweep(lo, hi, 1, fast).worst_scaled as f64 / SCALE;
        let exact = sweep(lo, hi, 1, exact).worst_scaled as f64 / SCALE;
        println!(
            "{name}: exact {exact:.6e} dB, fast {fast:.6e} dB, ratio {:.3}",
            fast / exact
        );
        assert!(
            fast <= RATIO * exact,
            "{name}: fast tier {fast:.6e} dB is more than {RATIO}x the exact tier's              {exact:.6e} dB -- the crossing's semantics claim no longer holds"
        );
    }
}

// Issue #880 MA-5 restatements from transient-shaper `src/lib.rs`; parameter IDs 1 and 2 are both
// in [-1, 1]. Keep this prospective evidence out of the six admitted-crossing count until R2.
const SHAPER_FLOOR: f32 = f32::from_bits(0x322b_cc77);
const SHAPER_CONTRAST_LIMIT_DB: f32 = 24.0;
const SHAPER_SHAPE_LIMIT_DB: f32 = 18.0;
const SHAPER_AMOUNT_MIN: f32 = -1.0;
const SHAPER_AMOUNT_MAX: f32 = 1.0;
const F1_SHAPER_LEVEL_MIN: f32 = 1.0e-8;
const F1_SHAPER_LEVEL_MAX: f32 = 16.0;

fn shaper_shape_f32(contrast_db: f32, attack: f32, sustain: f32) -> f32 {
    attack
        .mul(contrast_db.max(0.0))
        .add(sustain.mul((-contrast_db).max(0.0)))
        .clamp(-SHAPER_SHAPE_LIMIT_DB, SHAPER_SHAPE_LIMIT_DB)
}

fn shaper_shape_f64(contrast_db: f64, attack: f64, sustain: f64) -> f64 {
    (attack * contrast_db.max(0.0) + sustain * (-contrast_db).max(0.0)).clamp(-18.0, 18.0)
}

#[test]
fn f1_shaper_x7_x8_domains_are_covered() {
    assert_eq!(SHAPER_FLOOR.to_bits(), F1_SHAPER_LEVEL_MIN.to_bits());

    // If the ±24 dB contrast clamp does not saturate, the amplitude ratio is within these bounds.
    let unsaturated_ratio_min = math::pow(10.0, -24.0 / 20.0) as f32;
    let unsaturated_ratio_max = math::pow(10.0, 24.0 / 20.0) as f32;
    assert!(unsaturated_ratio_min >= F1_SHAPER_LEVEL_MIN);
    assert!(unsaturated_ratio_max <= F1_SHAPER_LEVEL_MAX);

    // For every corner of the parameter domains and contrast range, the final shape is clamped
    // inside F1's applied-gain domain [-160, 24] dB.
    for attack in [SHAPER_AMOUNT_MIN, SHAPER_AMOUNT_MAX] {
        for sustain in [SHAPER_AMOUNT_MIN, SHAPER_AMOUNT_MAX] {
            for contrast in [-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB] {
                let shape = shaper_shape_f32(contrast, attack, sustain);
                assert!((-18.0..=18.0).contains(&shape));
                assert!((-160.0..=24.0).contains(&shape));
            }
        }
    }
}

fn prospective_low_ratio_rail(x: f32) -> Measurement {
    let fast = fast_level_db::<f32>(x);
    let exact = exact_level_db(x);
    let fast_rail = fast.clamp(-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB);
    let exact_rail = exact.clamp(-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB);
    let on_rail = fast <= -SHAPER_CONTRAST_LIMIT_DB
        && exact <= -SHAPER_CONTRAST_LIMIT_DB
        && fast_rail == -SHAPER_CONTRAST_LIMIT_DB
        && exact_rail == -SHAPER_CONTRAST_LIMIT_DB;
    Measurement {
        error_db: Some(if on_rail { 0.0 } else { 1.0 }),
        value: fast,
    }
}

fn prospective_high_ratio_rail(x: f32) -> Measurement {
    let fast = fast_level_db::<f32>(x);
    let exact = exact_level_db(x);
    let fast_rail = fast.clamp(-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB);
    let exact_rail = exact.clamp(-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB);
    let on_rail = fast >= SHAPER_CONTRAST_LIMIT_DB
        && exact >= SHAPER_CONTRAST_LIMIT_DB
        && fast_rail == SHAPER_CONTRAST_LIMIT_DB
        && exact_rail == SHAPER_CONTRAST_LIMIT_DB;
    Measurement {
        error_db: Some(if on_rail { 0.0 } else { 1.0 }),
        value: fast,
    }
}

#[test]
#[ignore = "full positive f32 range proof: run with --release -- --ignored"]
fn f1_exhaustive_shaper_x7_outside_level_domain_lands_on_clamp_rails() {
    let floor_bits = SHAPER_FLOOR.to_bits();
    let below = sweep(0, floor_bits - 1, 1, prospective_low_ratio_rail);
    assert_eq!(below.checked, u64::from(floor_bits));
    assert_eq!(
        below.worst_scaled, 0,
        "a ratio below the F1 floor missed -24 dB"
    );
    println!(
        "prospective shaper low ratios: {} values including zero/subnormals, exact and fast clamp to -24 dB",
        below.checked
    );

    let first_high_bits = F1_SHAPER_LEVEL_MAX.to_bits();
    let last_finite_bits = f32::MAX.to_bits();
    let above = sweep(
        first_high_bits,
        last_finite_bits,
        1,
        prospective_high_ratio_rail,
    );
    assert_eq!(
        above.checked,
        u64::from(last_finite_bits - first_high_bits) + 1
    );
    assert_eq!(
        above.worst_scaled, 0,
        "a finite ratio at or above 16 missed +24 dB"
    );
    println!(
        "prospective shaper high ratios: {} finite values, exact and fast clamp to +24 dB",
        above.checked
    );

    let positive_infinity = f32::INFINITY;
    assert!(fast_level_db::<f32>(positive_infinity) >= SHAPER_CONTRAST_LIMIT_DB);
    assert!(exact_level_db(positive_infinity) >= SHAPER_CONTRAST_LIMIT_DB);
}

#[derive(Clone, Copy, Default)]
struct Maximum {
    value: f64,
    bits: u32,
    attack: f32,
    sustain: f32,
}

#[derive(Default)]
struct ShaperPipelineSweep {
    checked: u64,
    contrast_difference: Maximum,
    gain_difference_db: Maximum,
    fast_oracle_gain_error: Maximum,
    exact_oracle_gain_error: Maximum,
}

fn record_maximum(maximum: &mut Maximum, value: f64, bits: u32, attack: f32, sustain: f32) {
    if value > maximum.value {
        *maximum = Maximum {
            value,
            bits,
            attack,
            sustain,
        };
    }
}

fn shaper_pipeline_sweep() -> ShaperPipelineSweep {
    let (first_bits, last_bits) = level_domain();
    let total = u64::from(last_bits - first_bits) + 1;
    let threads = thread::available_parallelism().map_or(1, |value| value.get().min(4));
    let span = total.div_ceil(threads as u64);

    let chunks = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(threads);
        for index in 0..threads {
            handles.push(scope.spawn(move || {
                let start = index as u64 * span;
                let end = (start + span).min(total);
                let mut local = ShaperPipelineSweep::default();
                if start >= total {
                    return local;
                }
                for offset in start..end {
                    let bits = first_bits + offset as u32;
                    let ratio = f32::from_bits(bits);
                    let exact_contrast = exact_level_db(ratio)
                        .clamp(-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB);
                    let fast_contrast = fast_level_db::<f32>(ratio)
                        .clamp(-SHAPER_CONTRAST_LIMIT_DB, SHAPER_CONTRAST_LIMIT_DB);
                    record_maximum(
                        &mut local.contrast_difference,
                        f64::from((fast_contrast - exact_contrast).abs()),
                        bits,
                        0.0,
                        0.0,
                    );

                    let oracle_contrast = restate_level_db(f64::from(ratio)).clamp(
                        -f64::from(SHAPER_CONTRAST_LIMIT_DB),
                        f64::from(SHAPER_CONTRAST_LIMIT_DB),
                    );
                    for attack in [SHAPER_AMOUNT_MIN, SHAPER_AMOUNT_MAX] {
                        for sustain in [SHAPER_AMOUNT_MIN, SHAPER_AMOUNT_MAX] {
                            let exact_shape = shaper_shape_f32(exact_contrast, attack, sustain);
                            let fast_shape = shaper_shape_f32(fast_contrast, attack, sustain);
                            let oracle_shape = shaper_shape_f64(
                                oracle_contrast,
                                f64::from(attack),
                                f64::from(sustain),
                            );
                            let exact_gain = exact_gain_from_db(exact_shape);
                            let fast_gain = fast_gain_from_db::<f32>(fast_shape);
                            let oracle_gain = restate_gain(oracle_shape);
                            let gain_difference_db = (20.0
                                * math::log10(f64::from(fast_gain) / f64::from(exact_gain)))
                            .abs();
                            record_maximum(
                                &mut local.gain_difference_db,
                                gain_difference_db,
                                bits,
                                attack,
                                sustain,
                            );
                            record_maximum(
                                &mut local.fast_oracle_gain_error,
                                (f64::from(fast_gain) - oracle_gain).abs(),
                                bits,
                                attack,
                                sustain,
                            );
                            record_maximum(
                                &mut local.exact_oracle_gain_error,
                                (f64::from(exact_gain) - oracle_gain).abs(),
                                bits,
                                attack,
                                sustain,
                            );
                        }
                    }
                    local.checked += 1;
                }
                local
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().expect("MA-5 shaper worker panicked"))
            .collect::<Vec<_>>()
    });

    let mut total_metrics = ShaperPipelineSweep::default();
    for chunk in chunks {
        total_metrics.checked += chunk.checked;
        for (target, candidate) in [
            (
                &mut total_metrics.contrast_difference,
                chunk.contrast_difference,
            ),
            (
                &mut total_metrics.gain_difference_db,
                chunk.gain_difference_db,
            ),
            (
                &mut total_metrics.fast_oracle_gain_error,
                chunk.fast_oracle_gain_error,
            ),
            (
                &mut total_metrics.exact_oracle_gain_error,
                chunk.exact_oracle_gain_error,
            ),
        ] {
            record_maximum(
                target,
                candidate.value,
                candidate.bits,
                candidate.attack,
                candidate.sustain,
            );
        }
    }
    total_metrics
}

#[test]
#[ignore = "full shaper-pipeline sweep: run with --release -- --ignored"]
fn f1_exhaustive_shaper_x7_x8_pipeline_error_and_oracle() {
    let measured = shaper_pipeline_sweep();
    assert_eq!(measured.checked, 257_176_458);
    println!(
        "prospective shaper max |Δcontrast|: {:.6e} dB at ratio {:#010x}",
        measured.contrast_difference.value, measured.contrast_difference.bits
    );
    println!(
        "prospective shaper max |Δgain|: {:.6e} dB at ratio {:#010x}, attack {}, sustain {}",
        measured.gain_difference_db.value,
        measured.gain_difference_db.bits,
        measured.gain_difference_db.attack,
        measured.gain_difference_db.sustain
    );
    println!(
        "unit-input absolute gain error vs f64 oracle: fast {:.6e} (ratio {:#010x}), exact {:.6e}",
        measured.fast_oracle_gain_error.value,
        measured.fast_oracle_gain_error.bits,
        measured.exact_oracle_gain_error.value
    );
    assert!(measured.contrast_difference.value <= 4.3487e-5);
    assert!(measured.gain_difference_db.value <= 5.8e-5);
    assert!(measured.fast_oracle_gain_error.value <= 2.0e-5);
    assert!(measured.exact_oracle_gain_error.value <= 2.0e-5);
}

// ---------------------------------------------------------------------------------------------
// The eight named crossings, each pinned by an independent restatement.
//
// The sweeps above bound the tier over the union of the domains. These bound it over *each
// crossing's own* domain, named, so that the container's claim -- "exactly six crossings, and this
// is what each one costs" -- is checked one site at a time rather than in aggregate.
//
// The restatement is deliberately not the `exp2`/`log2` oracle used above. It is
// `20 * log10(x)` and `pow(10, db/20)` in `f64`, through the vendored `log10` and `pow`, which are
// different algorithms from the vendored `exp2`/`log2` -- `log10` is vendored separately precisely
// because `log2(x) * LOG10_2` is not bit-equal to it. So the expected value is computed from the
// decibel definition by an independent route, not by running the fast path and recording what it
// said, and not by re-spelling the same reduction.
// ---------------------------------------------------------------------------------------------

/// `20 * log10(x)` in `f64`, through the vendored `log10`. Independent of `log2`.
fn restate_level_db(x: f64) -> f64 {
    20.0 * math::log10(x)
}

/// `10^(db / 20)` in `f64`, through the vendored `pow`. Independent of `exp2`.
fn restate_gain(db: f64) -> f64 {
    math::pow(10.0, db / 20.0)
}

/// Worst error over one same-sign run of `f32` bit patterns, `first` to `last` inclusive.
///
/// Same-sign is the precondition, not a detail: `f32` bit patterns are monotone in *magnitude*,
/// so a range whose endpoints straddle zero is not a contiguous run of values at all -- walking it
/// passes through the infinities and the NaNs. Crossing domains that span zero are therefore
/// swept as two runs and combined by the caller.
fn worst_over_run(first: u32, last: u32, level: bool) -> f64 {
    let (first, last) = if first <= last {
        (first, last)
    } else {
        (last, first)
    };
    let mut worst = 0.0_f64;
    let mut bits = first;
    // Stride chosen so every crossing checks a few hundred thousand points in the default run.
    let stride = 1 + (u64::from(last - first) / 400_000) as u32;
    while bits <= last {
        let x = f32::from_bits(bits);
        let error = if level {
            let got = f64::from(fast_level_db::<f32>(x));
            (got - restate_level_db(f64::from(x))).abs()
        } else {
            let got = f64::from(fast_gain_from_db::<f32>(x));
            let want = restate_gain(f64::from(x));
            ((got - want) / want).abs() * DB_PER_RELATIVE
        };
        if error.is_finite() && error > worst {
            worst = error;
        }
        bits = match bits.checked_add(stride) {
            Some(next) => next,
            None => break,
        };
    }
    worst
}

/// Sweeps a crossing's own domain `[lo, hi]` and returns its worst error in decibels.
///
/// Split at zero into at most two same-sign runs, because that is the only shape
/// [`worst_over_run`] can walk. A domain that merely *ends* at zero (crossing X4's `[-96, 0]`) is
/// still a negative run: its far end is `-0.0`, not `+0.0`, and taking the `+0.0` bit pattern as
/// the endpoint sends the walk through every positive float instead.
fn crossing_worst_db(lo: f32, hi: f32, level: bool) -> f64 {
    assert!(lo <= hi, "a crossing domain is given low to high");
    let mut worst = 0.0_f64;
    if lo < 0.0 {
        let negative_end = if hi < 0.0 { hi } else { -0.0 };
        worst = worst.max(worst_over_run(lo.to_bits(), negative_end.to_bits(), level));
    }
    if hi >= 0.0 {
        let positive_start = if lo > 0.0 { lo } else { 0.0 };
        worst = worst.max(worst_over_run(
            positive_start.to_bits(),
            hi.to_bits(),
            level,
        ));
    }
    worst
}

/// Crossing X1 — the compressor's detector level (`kernel.rs`, step 4).
///
/// Domain: the detector is rectified and floored to `LEVEL_FLOOR = 1e-8`; the result is clamped
/// into `[-160, 24]` dB, so amplitudes above about `15.85` cannot affect the output.
#[test]
fn f1_crossing_x1_compressor_detector_level() {
    let worst = crossing_worst_db(1.0e-8, 16.0, true);
    assert!(
        worst <= LEVEL_MAX_DB,
        "crossing X1: {worst:.6e} dB exceeds the {LEVEL_MAX_DB:.1e} dB gate"
    );
    println!("crossing X1 (compressor detector level): {worst:.6e} dB");
}

/// Crossing X2 — the compressor's applied gain (`kernel.rs`, step 7).
///
/// Domain: `smoothed + makeup`, where the reduction is clamped to `[-100, 0]` and the makeup
/// parameter's domain is `[-24, 24]`, so the argument lies in `[-124, 24]` dB.
#[test]
fn f1_crossing_x2_compressor_applied_gain() {
    let worst = crossing_worst_db(-124.0, 24.0, false);
    assert!(
        worst <= GAIN_MAX_DB,
        "crossing X2: {worst:.6e} dB exceeds the {GAIN_MAX_DB:.1e} dB gate"
    );
    println!("crossing X2 (compressor applied gain): {worst:.6e} dB");
}

/// Crossing X3 — the gate/expander's detector level (`kernel.rs`).
///
/// Same floor and clamps as X1; the clamp *order* differs (`min` then `max`) but the conversion's
/// domain does not.
#[test]
fn f1_crossing_x3_gate_detector_level() {
    let worst = crossing_worst_db(1.0e-8, 16.0, true);
    assert!(
        worst <= LEVEL_MAX_DB,
        "crossing X3: {worst:.6e} dB exceeds the {LEVEL_MAX_DB:.1e} dB gate"
    );
    println!("crossing X3 (gate detector level): {worst:.6e} dB");
}

/// Crossing X4 — the gate/expander's applied gain (`kernel.rs`).
///
/// Domain: the smoothed `gain_db` tracks a target clamped to `[-range, 0]`, and the range
/// parameter's maximum is 96 dB, so the argument lies in `[-96, 0]`.
#[test]
fn f1_crossing_x4_gate_applied_gain() {
    let worst = crossing_worst_db(-96.0, 0.0, false);
    assert!(
        worst <= GAIN_MAX_DB,
        "crossing X4: {worst:.6e} dB exceeds the {GAIN_MAX_DB:.1e} dB gate"
    );
    println!("crossing X4 (gate applied gain): {worst:.6e} dB");
}

/// Crossing X5 — one multiband band's detector level (`lib.rs`, `band_amplitude`).
///
/// Domain: `DETECTOR_FLOOR = 1e-8`, clamped into `[-160, 24]` dB. Same as X1, reached four times
/// per frame rather than twice because there are two bands.
#[test]
fn f1_crossing_x5_multiband_detector_level() {
    let worst = crossing_worst_db(1.0e-8, 16.0, true);
    assert!(
        worst <= LEVEL_MAX_DB,
        "crossing X5: {worst:.6e} dB exceeds the {LEVEL_MAX_DB:.1e} dB gate"
    );
    println!("crossing X5 (multiband detector level): {worst:.6e} dB");
}

/// Crossing X6 — one multiband band's applied gain (`lib.rs`, `band_amplitude`).
///
/// Domain: `smoothed + makeup`, reduction clamped to `[-100, 0]`, makeup domain `[-24, 24]`.
#[test]
fn f1_crossing_x6_multiband_applied_gain() {
    let worst = crossing_worst_db(-124.0, 24.0, false);
    assert!(
        worst <= GAIN_MAX_DB,
        "crossing X6: {worst:.6e} dB exceeds the {GAIN_MAX_DB:.1e} dB gate"
    );
    println!("crossing X6 (multiband applied gain): {worst:.6e} dB");
}

/// Crossing X7 — the transient shaper's fast/slow detector contrast.
///
/// Domain: both envelopes are floored to `1e-8`, their ratio stays within `[1e-8, 16]` whenever
/// contrast is not already clamped, and the result is clamped to ±24 dB.
#[test]
fn f1_crossing_x7_transient_shaper_detector_contrast() {
    let worst = crossing_worst_db(1.0e-8, 16.0, true);
    assert!(
        worst <= LEVEL_MAX_DB,
        "crossing X7: {worst:.6e} dB exceeds the {LEVEL_MAX_DB:.1e} dB gate"
    );
    println!("crossing X7 (transient-shaper detector contrast): {worst:.6e} dB");
}

/// Crossing X8 — the transient shaper's applied gain.
///
/// Domain: the shape law's output is clamped to `[-18, 18]` dB before conversion.
#[test]
fn f1_crossing_x8_transient_shaper_applied_gain() {
    let worst = crossing_worst_db(-18.0, 18.0, false);
    assert!(
        worst <= GAIN_MAX_DB,
        "crossing X8: {worst:.6e} dB exceeds the {GAIN_MAX_DB:.1e} dB gate"
    );
    println!("crossing X8 (transient-shaper applied gain): {worst:.6e} dB");
}

/// Full-domain site-specific proof for X7, separate from the shared F1 tier sweep.
#[test]
#[ignore = "full X7 crossing sweep: run with --release -- --ignored"]
fn f1_exhaustive_x7_transient_shaper_detector_contrast() {
    let (lo, hi) = level_domain();
    let measured = sweep(lo, hi, 1, level_error_db);
    assert_sweep(
        "crossing X7 transient-shaper detector contrast",
        measured,
        LEVEL_MAX_DB,
        257_176_458,
    );
}

/// Full-domain site-specific proof for X8, split at zero because signed `f32` bit patterns are
/// monotone in magnitude on each side and not across zero.
#[test]
#[ignore = "full X8 crossing sweep: run with --release -- --ignored"]
fn f1_exhaustive_x8_transient_shaper_applied_gain() {
    let negative = sweep(
        (-0.0_f32).to_bits(),
        (-18.0_f32).to_bits(),
        1,
        gain_error_db,
    );
    let positive = sweep(0.0_f32.to_bits(), 18.0_f32.to_bits(), 1, gain_error_db);
    let negative_count = u64::from((-18.0_f32).to_bits() - (-0.0_f32).to_bits()) + 1;
    let positive_count = u64::from(18.0_f32.to_bits()) + 1;
    assert_sweep(
        "crossing X8 negative transient-shaper applied gain",
        negative,
        GAIN_MAX_DB,
        negative_count,
    );
    assert_sweep(
        "crossing X8 positive transient-shaper applied gain",
        positive,
        GAIN_MAX_DB,
        positive_count,
    );
}

/// The container's arithmetic: eight crossings, and no ninth hiding in this file.
#[test]
fn f1_the_container_pins_exactly_eight_crossings() {
    let source = include_str!("f1_fast_db_bounds.rs");
    // Built in two pieces so this test does not match its own needle.
    let needle = concat!("fn ", "f1_crossing_x");
    let pinned = source.matches(needle).count();
    assert_eq!(
        pinned, 8,
        "the seal admits exactly eight named crossings; this file pins {pinned}"
    );
}
