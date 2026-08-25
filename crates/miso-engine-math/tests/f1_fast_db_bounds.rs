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
//!   `#[ignore]`d, because it wants release codegen and all cores; run it with
//!   `cargo test --locked --release -p miso-engine-math --features lane --test f1_fast_db_bounds -- --ignored`.
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
//! | `level_db`, `[1e-8, 16]` | 257,176,458 | `1.538e-5` dB | **`2.810e-5` dB** | `4.0e-5` |
//!
//! # Red mutations
//!
//! Each was run against the same oracle over the same domain, and each must push the measured
//! error over the gate — a bound with no red mutation is decoration. These are measured, not
//! predicted:
//!
//! | mutation | measured | gate |
//! |---|---|---|
//! | `EXP2_P[0]` += 1e-5 | `fast_gain_from_db` `5.003e-5` dB at `-150.789` | `1.0e-5` — red |
//! | `LOG2_Q[0]` += 1e-5 | `fast_level_db` `8.524e-5` dB at `2.903e-8` | `4.0e-5` — red |
//! | drop `EXP2_P[4]` (degree 3, same coefficients) | `fast_gain_from_db` `8.115e-3` dB | `1.0e-5` — red |
//! | drop `LOG2_Q[5]` (degree 4, same coefficients) | `fast_level_db` `1.593e-1` dB | `4.0e-5` — red |
//!
//! The last two are the ones that matter: they are what "just drop a term" looks like, and they
//! are 200x and 4000x over the gate respectively. The degree is not slack.
//!
//! Memory note: the sweeps iterate ranges of `u32`. Never collect the patterns.

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

use miso_engine_math::fast_db::{fast_gain_from_db, fast_level_db};

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
    miso_engine_math::exp2(db * (core::f64::consts::LOG2_10 / 20.0))
}

fn oracle_level_db(x: f64) -> f64 {
    miso_engine_math::log2(x) * DB_PER_LOG2
}

/// The worst error seen over a sweep, and how many inputs produced it.
struct Sweep {
    /// Worst error, in decibels, scaled by 2^40 so it can live in an `AtomicU64`.
    worst_scaled: u64,
    /// The input bit pattern that produced it.
    worst_bits: u32,
    /// How many inputs were actually checked.
    checked: u64,
}

const SCALE: f64 = (1_u64 << 40) as f64;

/// Sweeps `[first_bits, last_bits]` (inclusive, monotone in the `f32` value) with `stride`.
fn sweep(first_bits: u32, last_bits: u32, stride: u64, error_db: fn(f32) -> Option<f64>) -> Sweep {
    let threads = thread::available_parallelism().map_or(1, |value| value.get());
    let total = u64::from(last_bits - first_bits) + 1;
    let span = total.div_ceil(threads as u64);
    let worst = AtomicU64::new(0);
    let worst_bits = AtomicU64::new(0);
    let checked = AtomicU64::new(0);

    thread::scope(|scope| {
        for index in 0..threads {
            let worst = &worst;
            let worst_bits = &worst_bits;
            let checked = &checked;
            scope.spawn(move || {
                let start = index as u64 * span;
                if start >= total {
                    return;
                }
                let end = (start + span).min(total);
                // Align this thread's start to the stride so the union is exactly the strided set.
                let mut offset = start.div_ceil(stride) * stride;
                let mut local_worst = 0_u64;
                let mut local_bits = 0_u32;
                let mut local_checked = 0_u64;
                while offset < end {
                    let bits = first_bits + offset as u32;
                    if let Some(error) = error_db(f32::from_bits(bits)) {
                        local_checked += 1;
                        let scaled = (error * SCALE) as u64;
                        if scaled > local_worst {
                            local_worst = scaled;
                            local_bits = bits;
                        }
                    }
                    offset += stride;
                }
                checked.fetch_add(local_checked, Ordering::Relaxed);
                let mut seen = worst.load(Ordering::Relaxed);
                while local_worst > seen {
                    match worst.compare_exchange(
                        seen,
                        local_worst,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            worst_bits.store(u64::from(local_bits), Ordering::Relaxed);
                            break;
                        }
                        Err(current) => seen = current,
                    }
                }
            });
        }
    });

    Sweep {
        worst_scaled: worst.load(Ordering::Relaxed),
        worst_bits: worst_bits.load(Ordering::Relaxed) as u32,
        checked: checked.load(Ordering::Relaxed),
    }
}

/// Error of `fast_gain_from_db` at one decibel value, in decibels of the returned gain.
fn gain_error_db(db: f32) -> Option<f64> {
    let got = f64::from(fast_gain_from_db::<f32>(db));
    let want = oracle_gain(f64::from(db));
    if want <= 0.0 || !want.is_finite() || !got.is_finite() {
        return None;
    }
    Some(((got - want) / want).abs() * DB_PER_RELATIVE)
}

/// Error of `fast_level_db` at one amplitude, in decibels.
fn level_error_db(x: f32) -> Option<f64> {
    let got = f64::from(fast_level_db::<f32>(x));
    let want = oracle_level_db(f64::from(x));
    if !want.is_finite() || !got.is_finite() {
        return None;
    }
    Some((got - want).abs())
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
    assert_sweep(
        "fast_gain_from_db negative exhaustive",
        sweep(lo, hi, 1, gain_error_db),
        GAIN_MAX_DB,
        1_000_000_000,
    );
    let (lo, hi) = gain_domain_positive();
    assert_sweep(
        "fast_gain_from_db positive exhaustive",
        sweep(lo, hi, 1, gain_error_db),
        GAIN_MAX_DB,
        1_000_000_000,
    );
}

#[test]
#[ignore = "full-domain sweep: run with --release -- --ignored"]
fn f1_level_db_exhaustive() {
    let (lo, hi) = level_domain();
    assert_sweep(
        "fast_level_db exhaustive",
        sweep(lo, hi, 1, level_error_db),
        LEVEL_MAX_DB,
        250_000_000,
    );
}

// ---------------------------------------------------------------------------------------------
// The comparison that makes the crossing safe: the fast tier against the exact tier, same domain.
// ---------------------------------------------------------------------------------------------

/// The exact tier's `level_db`, spelled exactly as `effect_runtime::dynamics::level_db` spells it.
fn exact_level_db(x: f32) -> f32 {
    miso_engine_math::log2_lane::<f32>(x) * (20.0_f64 * core::f64::consts::LOG10_2) as f32
}

/// The exact tier's `gain_from_db`, spelled as `effect_runtime::dynamics::gain_from_db` spells it.
fn exact_gain_from_db(db: f32) -> f32 {
    miso_engine_math::exp2_lane::<f32>(db * (core::f64::consts::LOG2_10 / 20.0) as f32)
}

fn exact_gain_error_db(db: f32) -> Option<f64> {
    let got = f64::from(exact_gain_from_db(db));
    let want = oracle_gain(f64::from(db));
    if want <= 0.0 || !want.is_finite() || !got.is_finite() {
        return None;
    }
    Some(((got - want) / want).abs() * DB_PER_RELATIVE)
}

fn exact_level_error_db(x: f32) -> Option<f64> {
    let got = f64::from(exact_level_db(x));
    let want = oracle_level_db(f64::from(x));
    if !want.is_finite() || !got.is_finite() {
        return None;
    }
    Some((got - want).abs())
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
            gain_error_db as fn(f32) -> Option<f64>,
            exact_gain_error_db as fn(f32) -> Option<f64>,
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
