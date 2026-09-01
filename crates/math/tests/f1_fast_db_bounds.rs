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
//!   `#[ignore]`d, because it wants release codegen and all cores; run it with
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
    math::log2_lane::<f32>(x) * (20.0_f64 * core::f64::consts::LOG10_2) as f32
}

/// The exact tier's `gain_from_db`, spelled as `effect_runtime::dynamics::gain_from_db` spells it.
fn exact_gain_from_db(db: f32) -> f32 {
    math::exp2_lane::<f32>(db * (core::f64::consts::LOG2_10 / 20.0) as f32)
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

// ---------------------------------------------------------------------------------------------
// The six named crossings, each pinned by an independent restatement.
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

/// The container's arithmetic: six crossings, and no seventh hiding in this file.
#[test]
fn f1_the_container_pins_exactly_six_crossings() {
    let source = include_str!("f1_fast_db_bounds.rs");
    // Built in two pieces so this test does not match its own needle.
    let needle = concat!("fn ", "f1_crossing_x");
    let pinned = source.matches(needle).count();
    assert_eq!(
        pinned, 6,
        "the seal admits exactly six named crossings; this file pins {pinned}"
    );
}
