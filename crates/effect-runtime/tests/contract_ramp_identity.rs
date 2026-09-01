//! D11 is one law: the contract's control-plane smoother and this crate's render-path ramp agree
//! bit for bit.
//!
//! Issue #95 finding F2 was that `effect_contract::ParameterSmoother::next_value`
//! divided by `remaining` on **every** sample, while decision D11 (and therefore
//! [`effect_runtime::ramp::LinearRamp`], and therefore every effect in the workspace)
//! precomputes the increment once. Two implementations of a frozen numeric law are only
//! acceptable while a test says they are the same implementation. They cannot be one function:
//! the contract is `std` and control-plane, this crate is `no_std` and lane-generic, and neither
//! may depend on the other (`scripts/check-effect-runtime-policy.sh` pins both boundaries).
//!
//! Red mutation: restore the audited `current + (target - current) / remaining as f32` in
//! `ParameterSmoother::next_value` — `linear_smoother_is_bit_identical_to_the_linear_ramp` fails
//! at the second update of the first non-power-of-two ramp length.

use effect_contract::{ParameterSmoother, SmoothingRule};
use effect_runtime::ramp::LinearRamp;

/// Ramp lengths that are not powers of two, so `(target - current) / remaining` and
/// `current + step` genuinely differ in `f32`.
const LENGTHS: [u32; 9] = [1, 2, 3, 4, 5, 7, 37, 64, 129];

/// Endpoint pairs, including sign changes and a zero endpoint.
const ENDPOINTS: [(f32, f32); 7] = [
    (0.0, 1.0),
    (1.0, 0.0),
    (-24.0, 6.0),
    (6.0, -24.0),
    (0.1, 0.3),
    (-0.0, 1.0e-7),
    (1_000.0, 1_000.000_1),
];

#[test]
fn linear_smoother_is_bit_identical_to_the_linear_ramp() {
    for samples in LENGTHS {
        for (from, to) in ENDPOINTS {
            let mut smoother = ParameterSmoother::new(from, SmoothingRule::Linear, samples)
                .expect("finite initial value and a nonzero length");
            let mut ramp = LinearRamp::fixed(if from == 0.0 { 0.0 } else { from });

            assert!(smoother.set_target(to), "finite target");
            ramp.set_target(if to == 0.0 { 0.0 } else { to }, samples);

            assert_eq!(
                smoother.step().to_bits(),
                ramp.step.to_bits(),
                "the one division must produce the same increment ({from} -> {to} over {samples})"
            );

            // Two samples past the end: the resting value must agree too.
            for update in 0..samples + 2 {
                let a = smoother.next_value();
                let b = ramp.next_value();
                assert_eq!(
                    a.to_bits(),
                    b.to_bits(),
                    "update {update} of {samples} ({from} -> {to}): {a} vs {b}"
                );
            }
            assert_eq!(smoother.remaining(), 0);
            assert_eq!(smoother.current().to_bits(), ramp.current.to_bits());
        }
    }
}

#[test]
fn the_precomputed_increment_is_the_declared_one_and_the_last_update_assigns() {
    // The observable signature of D11: every update adds the *same* precomputed increment, so the
    // trajectory is the iterated addition of one constant. The audited form recomputed
    // `(target - current) / remaining` each update, whose increment grows as the rounding error
    // accumulates, so it leaves this trajectory at the second update for a length of 5.
    const SAMPLES: u32 = 7;
    const FROM: f32 = 0.1;
    const TO: f32 = 0.3;
    let mut smoother =
        ParameterSmoother::new(FROM, SmoothingRule::Linear, SAMPLES).expect("nonzero length");
    assert!(smoother.set_target(TO));

    // Derived here, not read back from the smoother: an independent statement of the law.
    let step = (TO - FROM) / SAMPLES as f32;
    assert_eq!(smoother.step().to_bits(), step.to_bits());
    let mut expected = FROM;
    for update in 1..SAMPLES {
        expected += step;
        assert_eq!(
            smoother.next_value().to_bits(),
            expected.to_bits(),
            "update {update} must be the iterated addition of one constant increment"
        );
    }
    // The last update is the exact assignment, not an addition.
    assert_eq!(smoother.next_value().to_bits(), TO.to_bits());
}

#[test]
fn no_smoothing_snaps_and_one_pole_uses_a_precomputed_coefficient() {
    let mut none = ParameterSmoother::new(2.0, SmoothingRule::None, 0).expect("length zero");
    assert!(none.set_target(-3.0));
    assert_eq!(none.current().to_bits(), (-3.0_f32).to_bits());
    assert_eq!(none.step().to_bits(), 0.0_f32.to_bits());

    let mut pole = ParameterSmoother::new(0.0, SmoothingRule::OnePole99, 4).expect("nonzero");
    assert!(pole.set_target(1.0));
    for _ in 0..3 {
        let _ = pole.next_value();
    }
    // Update N assigns the target exactly, as the contract text says.
    assert_eq!(pole.next_value().to_bits(), 1.0_f32.to_bits());
}
