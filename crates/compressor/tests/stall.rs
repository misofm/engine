#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E14 — descriptive: where an `f32` gain-reduction word stops moving on a long release.
//!
//! Not a gate. The one-pole increment is `c * (C - G)`; when that drops below `ulp(G)` the state
//! stops changing and the envelope parks short of its target. At the release parameter's maximum,
//! 5,000 ms at 96 kHz, `c` is about `2.08e-6`, and `ulp(G)` near `-20` dB is about `1.9e-6`, so
//! the recursion stalls while `|C - G|` is still of order `1` dB.
//!
//! This is inherent to an `f32` state, not to the operation order: master plan D2 leaves an `f64`
//! `Lane64` family open, and BRIEFS/013's 0.005 dB envelope gate *at the release maximum* needs
//! that decision. Measured here, reported, handed to issue 046. The number is printed, never
//! asserted against a threshold, so it cannot be tuned toward.

mod support;

use effect_runtime::state_payload::read_f32;

use support::{prepare, render_scalar, request, snapshot, values_with};

/// Prints the residual `|C - G|` at which the envelope stops moving, for the longest release.
#[test]
fn f32_release_stall_floor_is_reported() {
    // 96 kHz is not the fixture rate the rest of the suite uses; the stall is worst there, which
    // is the point of measuring it.
    let values = values_with(&[
        (0, -40.0),
        (1, 20.0),
        (2, 0.0),
        (3, 0.1),
        (4, 5_000.0),
        (5, 0.0),
        (6, 1.0),
        (7, 0.0),
    ]);
    let mut preparation = request(&values);
    preparation.sample_rate = 96_000;
    preparation.limits.maximum_total_state_bytes = 64_000;
    let mut effect = prepare(preparation);

    // Drive the envelope down with a loud burst, then step to a quieter level that is still above
    // the threshold. Releasing toward *zero* never stalls — `ulp(G)` shrinks as fast as the
    // increment does — so the experiment that matters is a release toward a non-zero target.
    let mut loud = vec![0.9_f32; 16_384];
    let mut loud_right = vec![0.9_f32; 16_384];
    render_scalar(effect.as_mut(), &mut loud, &mut loud_right, 128, 128, &[]);
    let settled = read_f32(&snapshot(effect.as_ref()).0, 2);

    // Level 0.05 is -26.02 dB; with T = -40 and R = 20 the static curve asks for
    // (1/20 - 1) * (-26.02 + 40) = -13.28 dB.
    let quiet_level = 0.05_f32;
    let target_db = (1.0 / 20.0 - 1.0) * (20.0 * f64::from(quiet_level).log10() + 40.0);

    // The detector is aligned with the output (lookahead 0, so `D = N`), so the step does not reach
    // it until the latency has run out: 1,920 samples at 96 kHz.
    let latency_blocks = (1_920 / 128) + 2;
    let mut previous = settled;
    let mut samples = 0_usize;
    let mut stalled_at = None;
    for block in 0..40_000 {
        let mut quiet = vec![quiet_level; 128];
        let mut quiet_right = vec![quiet_level; 128];
        render_scalar(effect.as_mut(), &mut quiet, &mut quiet_right, 128, 128, &[]);
        samples += 128;
        let current = read_f32(&snapshot(effect.as_ref()).0, 2);
        if block > latency_blocks && current.to_bits() == previous.to_bits() {
            stalled_at = Some((current, samples));
            break;
        }
        previous = current;
    }

    println!(
        "E14: 96 kHz, release 5,000 ms (c = 2.08e-6). Settled under the burst at G = {settled:.4} dB; the static curve then asks for {target_db:.4} dB."
    );
    match stalled_at {
        Some((gain_reduction_db, after)) => println!(
            "E14: G stopped moving at {gain_reduction_db:.4} dB after {after} samples — a residual of {:.4} dB. The increment c*(C - G) has fallen below half an ulp of G, which is inherent to an f32 state (master plan D2 leaves an f64 Lane64 family open). BRIEFS/013's 0.005 dB envelope gate at the release maximum needs that decision; handed to issue 046.",
            (f64::from(gain_reduction_db) - target_db).abs()
        ),
        None => println!(
            "E14: no stall within 5,120,000 samples; G reached {previous:.6} dB, a residual of \
 {:.6} dB.",
            (f64::from(previous) - target_db).abs()
        ),
    }
}
