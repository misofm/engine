//! Gate M1 — the lane-wide `exp2`/`log2` are within 2 ulp everywhere, monotone, and exact at the
//! anchors.
//!
//! The oracle is the crate's own vendored `f64` `exp2`/`log2`: a completely different algorithm
//! (musl's table-driven reduction) from the Cephes polynomials under test, so agreement is
//! evidence rather than a tautology. `f64` carries 29 more significand bits than `f32`, so its
//! result is exact to far better than the `f32` ulp being measured.
//!
//! Two forms of every check:
//!
//! * `m1_*_exhaustive` walks every one of the 2^32 `f32` bit patterns. `#[ignore]`d, because it
//!   wants release codegen and all cores; run it with
//!   `cargo test --locked --release -p math --features lane --test m1_exhaustive -- --ignored`.
//! * `m1_*_subsample` strides the bit pattern by 4099 (prime, so it walks every exponent and a
//!   dense spread of significands), adds the anchors and the neighbourhood of the measured worst
//!   points, and runs in the default `cargo test`.
//!
//! **Measured on the delivery host**, exhaustively, through the real `impl Lane for f32`:
//!
//! | function | max error | at | inputs checked | monotone |
//! |---|---|---|---|---|
//! | `exp2_lane::<f32>` | **1.4615 ulp** | `x = -0.4910151` (bits `0xbefb6655`) | 2,247,753,730 | yes |
//! | `log2_lane::<f32>` | **1.4667 ulp** | `x = 1.4082463` (bits `0x3fb4416a`) | 2,130,706,432 | yes |
//!
//! The gate is 2 ulp, and the margin is what pays for `mul`/`add` instead of `fma`
//! (`fma` would give `exp2` 1.191 ulp and `log2` no change; see `lane_math.rs`).
//!
//! **Red mutations**, each run exhaustively against the same oracle:
//!
//! | mutation | result |
//! |---|---|
//! | `EXP2_P[5]` + 1e-6 | `exp2_lane` 2.103 ulp at `x = -0.48641285`, 4 decreasing steps — over the gate |
//! | `LOG2_P[8]` + 1e-5 | `log2_lane` 35.61 ulp at `x = 1.4136208`, 64 decreasing steps — over the gate |
//! | Cephes fold removed (reduce to `[0, 1)` keeping the same coefficients) | `exp2_lane` 95.01 ulp at `x = -0.00026169422` — over the gate |
//! | Cephes summation reassociated to `(y + x) * LOG2EA + (y + x)` | `log2_lane` 1.938 ulp — still inside the gate, but two thirds of the margin gone, which is why the summation order is frozen above |
//!
//! The first of those is also caught by `m1_measured_worst_points`, which runs in the default
//! `cargo test` (2.072 ulp in the recorded neighbourhood) — the exhaustive sweep is the proof, not
//! the tripwire.
//!
//! `LOG2_P[8]` + 1e-6 reaches only 1.722 ulp and stays inside the gate; that is a property of the
//! polynomial, not a hole in the sweep, and it is why the gate is a bound rather than a pin.
//!
//! Memory note: the sweep iterates ranges of `u32`. Never collect the patterns — 2^31 `u32`s is
//! 8 GB. Each thread starts its monotonicity chain fresh, so one consecutive pair per thread
//! boundary goes unchecked — a handful out of four billion.

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

use math::{exp2_lane, log2_lane};

/// The gate. Not a knob: master plan §5.2 freezes it.
const MAX_ULP: f64 = 2.0;

/// Spacing of `f32` at `|value|`, in `f64`, clamped at the subnormal spacing.
fn f32_ulp(value: f64) -> f64 {
    let magnitude = value.abs() as f32;
    if magnitude == 0.0 || magnitude.is_subnormal() {
        return f64::from(f32::from_bits(1));
    }
    let next = f32::from_bits(magnitude.to_bits() + 1);
    f64::from(next) - f64::from(magnitude)
}

/// Result of one sweep: the worst error seen, where, how many points were in domain, and how many
/// consecutive pairs went the wrong way.
#[derive(Clone, Copy, Debug, Default)]
struct Sweep {
    worst_ulp: f64,
    worst_bits: u32,
    checked: u64,
    non_monotone: u64,
}

/// Whether `bits` is an input the function under test is specified for, and its value.
type Domain = fn(f32) -> bool;

fn exp2_domain(x: f32) -> bool {
    x.is_finite() && (-126.0..=127.0).contains(&x)
}

fn log2_domain(x: f32) -> bool {
    x.is_finite() && x > 0.0 && !x.is_subnormal()
}

/// Sweep `[lo, hi)` of the `f32` bit space, checking `lane` against `oracle`.
fn sweep_range(
    lo: u64,
    hi: u64,
    stride: u64,
    domain: Domain,
    lane: fn(f32) -> f32,
    oracle: fn(f64) -> f64,
) -> Sweep {
    let mut result = Sweep::default();
    let mut previous: Option<(f32, f32)> = None;

    let mut pattern = lo;
    while pattern < hi {
        let bits = pattern as u32;
        pattern += stride;

        let x = f32::from_bits(bits);
        if !domain(x) {
            continue;
        }

        let got = lane(x);
        let want = oracle(f64::from(x));
        result.checked += 1;

        let error = ((f64::from(got) - want) / f32_ulp(want)).abs();
        if error > result.worst_ulp {
            result.worst_ulp = error;
            result.worst_bits = bits;
        }

        // Monotonicity. Both functions are non-decreasing on their whole domain. Increasing bit
        // patterns mean increasing values in the positive half and *decreasing* values in the
        // negative half, so both directions are checked: testing only `previous_x < x` would leave
        // every negative argument unchecked.
        if let Some((previous_x, previous_y)) = previous {
            let decreased =
                (previous_x < x && previous_y > got) || (previous_x > x && previous_y < got);
            if decreased {
                result.non_monotone += 1;
            }
        }
        previous = Some((x, got));
    }

    result
}

/// Run `sweep_range` across all available cores and reduce.
fn sweep_all(stride: u64, domain: Domain, lane: fn(f32) -> f32, oracle: fn(f64) -> f64) -> Sweep {
    let threads = thread::available_parallelism().map_or(1, |value| value.get());
    let total = 1u64 << 32;
    let span = total.div_ceil(threads as u64);

    let worst = AtomicU64::new(0);
    let worst_bits = AtomicU64::new(0);
    let checked = AtomicU64::new(0);
    let non_monotone = AtomicU64::new(0);

    thread::scope(|scope| {
        for index in 0..threads {
            let worst = &worst;
            let worst_bits = &worst_bits;
            let checked = &checked;
            let non_monotone = &non_monotone;
            scope.spawn(move || {
                // Align each thread's start to the stride so the union is exactly the strided set.
                let raw_lo = index as u64 * span;
                let lo = raw_lo.div_ceil(stride) * stride;
                let hi = (raw_lo + span).min(total);
                let local = sweep_range(lo, hi, stride, domain, lane, oracle);

                checked.fetch_add(local.checked, Ordering::Relaxed);
                non_monotone.fetch_add(local.non_monotone, Ordering::Relaxed);
                loop {
                    let current = worst.load(Ordering::Relaxed);
                    if f64::from_bits(current) >= local.worst_ulp {
                        break;
                    }
                    if worst
                        .compare_exchange(
                            current,
                            local.worst_ulp.to_bits(),
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                    {
                        worst_bits.store(u64::from(local.worst_bits), Ordering::Relaxed);
                        break;
                    }
                }
            });
        }
    });

    Sweep {
        worst_ulp: f64::from_bits(worst.load(Ordering::Relaxed)),
        worst_bits: worst_bits.load(Ordering::Relaxed) as u32,
        checked: checked.load(Ordering::Relaxed),
        non_monotone: non_monotone.load(Ordering::Relaxed),
    }
}

fn assert_sweep(name: &str, sweep: Sweep, minimum_checked: u64) {
    println!(
        "M1 {name}: max {:.6} ulp at x = {} (bits {:#010x}), checked {}, non-monotone {}",
        sweep.worst_ulp,
        f32::from_bits(sweep.worst_bits),
        sweep.worst_bits,
        sweep.checked,
        sweep.non_monotone
    );
    assert!(
        sweep.checked >= minimum_checked,
        "{name}: only {} inputs were in domain; the sweep is not covering the range",
        sweep.checked
    );
    assert!(
        sweep.worst_ulp <= MAX_ULP,
        "{name}: {:.6} ulp at x = {} exceeds the {MAX_ULP} ulp gate",
        sweep.worst_ulp,
        f32::from_bits(sweep.worst_bits)
    );
    assert_eq!(
        sweep.non_monotone, 0,
        "{name}: {} decreasing steps",
        sweep.non_monotone
    );
}

fn exp2_lane_f32(x: f32) -> f32 {
    exp2_lane::<f32>(x)
}

fn log2_lane_f32(x: f32) -> f32 {
    log2_lane::<f32>(x)
}

/// Exact values the conversions depend on: a 0 dB trim must be bit-exactly unity gain, and unity
/// gain must be bit-exactly 0 dB, or every identity test downstream drifts.
fn assert_anchors() {
    assert_eq!(
        exp2_lane_f32(0.0).to_bits(),
        1.0_f32.to_bits(),
        "exp2_lane(0) must be exactly 1"
    );
    assert_eq!(
        exp2_lane_f32(1.0).to_bits(),
        2.0_f32.to_bits(),
        "exp2_lane(1) must be exactly 2"
    );
    assert_eq!(exp2_lane_f32(-1.0).to_bits(), 0.5_f32.to_bits());
    assert_eq!(exp2_lane_f32(10.0).to_bits(), 1024.0_f32.to_bits());
    assert_eq!(
        log2_lane_f32(1.0).to_bits(),
        0.0_f32.to_bits(),
        "log2_lane(1) must be exactly 0"
    );
    assert_eq!(
        log2_lane_f32(2.0).to_bits(),
        1.0_f32.to_bits(),
        "log2_lane(2) must be exactly 1"
    );
    assert_eq!(log2_lane_f32(0.5).to_bits(), (-1.0_f32).to_bits());
    assert_eq!(log2_lane_f32(1024.0).to_bits(), 10.0_f32.to_bits());
}

/// Out-of-domain inputs must saturate rather than produce infinities or NaN: D5 excludes NaN
/// payloads, and a NaN reaching a bank would poison a whole cohort.
fn assert_clamping() {
    assert_eq!(
        exp2_lane_f32(f32::NAN).to_bits(),
        exp2_lane_f32(-126.0).to_bits()
    );
    assert_eq!(
        exp2_lane_f32(f32::NEG_INFINITY).to_bits(),
        exp2_lane_f32(-126.0).to_bits()
    );
    assert_eq!(
        exp2_lane_f32(f32::INFINITY).to_bits(),
        exp2_lane_f32(127.0).to_bits()
    );
    assert_eq!(
        exp2_lane_f32(1.0e30).to_bits(),
        exp2_lane_f32(127.0).to_bits()
    );
    assert!(exp2_lane_f32(f32::NAN).is_finite());

    assert_eq!(
        log2_lane_f32(0.0).to_bits(),
        log2_lane_f32(f32::MIN_POSITIVE).to_bits()
    );
    assert_eq!(
        log2_lane_f32(-1.0).to_bits(),
        log2_lane_f32(f32::MIN_POSITIVE).to_bits()
    );
    assert_eq!(
        log2_lane_f32(f32::MIN_POSITIVE).to_bits(),
        (-126.0_f32).to_bits()
    );
    assert!(log2_lane_f32(0.0).is_finite());
}

#[test]
fn m1_anchors_and_clamping() {
    assert_anchors();
    assert_clamping();
}

/// The neighbourhood of the exhaustively measured worst points, so a regression that moves the
/// maximum shows up in the default test run and not only in the ignored sweep.
#[test]
fn m1_measured_worst_points() {
    for (name, bits, lane, oracle, domain) in [
        (
            "exp2_lane",
            0xbefb_6655_u32,
            exp2_lane_f32 as fn(f32) -> f32,
            math::exp2 as fn(f64) -> f64,
            exp2_domain as Domain,
        ),
        (
            "log2_lane",
            0x3fb4_416a,
            log2_lane_f32,
            math::log2,
            log2_domain as Domain,
        ),
    ] {
        let mut worst = 0.0_f64;
        for offset in -64_i64..=64 {
            let neighbour = (i64::from(bits) + offset) as u32;
            let x = f32::from_bits(neighbour);
            if !domain(x) {
                continue;
            }
            let want = oracle(f64::from(x));
            worst = worst.max(((f64::from(lane(x)) - want) / f32_ulp(want)).abs());
        }
        assert!(
            worst <= MAX_ULP,
            "{name}: {worst:.6} ulp near the measured worst point"
        );
        assert!(
            worst >= 1.4,
            "{name}: only {worst:.6} ulp near the recorded worst point {bits:#010x}; the recorded \
             maximum no longer describes this implementation, so re-run the exhaustive sweep"
        );
    }
}

#[test]
fn m1_exp2_lane_subsample() {
    let sweep = sweep_all(4099, exp2_domain, exp2_lane_f32, math::exp2);
    assert_sweep("exp2_lane subsample", sweep, 500_000);
    assert_anchors();
}

#[test]
fn m1_log2_lane_subsample() {
    let sweep = sweep_all(4099, log2_domain, log2_lane_f32, math::log2);
    assert_sweep("log2_lane subsample", sweep, 500_000);
    assert_anchors();
}

/// Every `f32` in `[-126, 127]`. Measured: max 1.4615 ulp at `x = -0.4910151`, 2,247,753,730
/// inputs, monotone.
#[test]
#[ignore = "2^32 sweep: run with --release -- --ignored"]
fn m1_exp2_lane_exhaustive() {
    let sweep = sweep_all(1, exp2_domain, exp2_lane_f32, math::exp2);
    assert_sweep("exp2_lane exhaustive", sweep, 2_247_753_730);
    assert_anchors();
    assert_clamping();
}

/// Every positive normal `f32`. Measured: max 1.4667 ulp at `x = 1.4082463`, 2,130,706,432 inputs,
/// monotone.
#[test]
#[ignore = "2^32 sweep: run with --release -- --ignored"]
fn m1_log2_lane_exhaustive() {
    let sweep = sweep_all(1, log2_domain, log2_lane_f32, math::log2);
    assert_sweep("log2_lane exhaustive", sweep, 2_130_706_432);
    assert_anchors();
    assert_clamping();
}
