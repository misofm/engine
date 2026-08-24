//! Issue #140 D: the gain-reduction observation point.
//!
//! What is gated here is that the reading is the kernel's own smoother state, in the sign
//! convention the contract fixes -- negative for reduction -- and that it moves only when the
//! compressor actually reduces. The transport from here to a console's meter frame does not exist
//! yet and is recorded as the remaining half of #140 D.

mod support;

use miso_engine_effect_contract::{
    EffectProcessBlock, GainReductionV1, PreparedNativeEffect, ResetKind,
};

use support::{initial_values, prepare, request};

fn prepared() -> Box<dyn PreparedNativeEffect> {
    let values = initial_values();
    prepare(request(&values))
}

fn render(effect: &mut dyn PreparedNativeEffect, value: f32, blocks: usize) -> GainReductionV1 {
    let quantum = effect.metadata().quantum;
    let mut reduction = GainReductionV1 {
        left_db: 0.0,
        right_db: 0.0,
    };
    for block in 0..blocks {
        let mut left = vec![value; quantum as usize];
        let mut right = vec![value; quantum as usize];
        let first_sample = block as u64 * u64::from(quantum);
        let block =
            EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], quantum)
                .expect("block");
        let _ = effect.process(block);
        reduction = effect.gain_reduction().expect("the compressor reports GR");
    }
    reduction
}

/// Red mutation: return `Some(GainReductionV1 { left_db: 0.0, right_db: 0.0 })` instead of reading
/// `Channel::gain_reduction_db` -> the loud case below reports no reduction and fails.
#[test]
fn the_compressor_reports_the_reduction_its_kernel_smoothed() {
    let mut effect = prepared();
    // Well below the prepared threshold: nothing to reduce.
    let quiet = render(effect.as_mut(), 0.000_1, 8);
    assert!(
        quiet.left_db > -0.5 && quiet.left_db <= 0.0,
        "a signal under the threshold is barely reduced: {}",
        quiet.left_db
    );

    effect.reset(ResetKind::FullToDefaults);
    let loud = render(effect.as_mut(), 0.9, 32);
    assert!(
        loud.left_db < -1.0,
        "a signal well over the threshold is audibly reduced: {}",
        loud.left_db
    );
    assert!(
        loud.left_db >= -100.0,
        "the kernel's own clamp bounds the reading: {}",
        loud.left_db
    );
    assert_eq!(
        loud.left_db.to_bits(),
        loud.right_db.to_bits(),
        "a dual-mono compressor fed identical lanes reduces them identically"
    );
}

/// The reading follows the smoother across blocks rather than jumping: the attack is what the
/// kernel says it is, so a single block of a loud signal has not yet reached the steady state that
/// many blocks reach.
#[test]
fn the_reading_follows_the_smoother_rather_than_the_instantaneous_curve() {
    let mut effect = prepared();
    let after_one = render(effect.as_mut(), 0.9, 1);
    effect.reset(ResetKind::FullToDefaults);
    let after_many = render(effect.as_mut(), 0.9, 32);
    assert!(
        after_many.left_db < after_one.left_db,
        "the envelope is still attacking after one block: one={} many={}",
        after_one.left_db,
        after_many.left_db
    );
}

/// A full reset returns the observation to zero, because it returns the smoother to zero.
#[test]
fn a_full_reset_returns_the_reading_to_zero() {
    let mut effect = prepared();
    let loud = render(effect.as_mut(), 0.9, 16);
    assert!(loud.left_db < 0.0);
    effect.reset(ResetKind::FullToDefaults);
    let after = effect.gain_reduction().expect("GR");
    assert_eq!(after.left_db.to_bits(), 0.0_f32.to_bits());
    assert_eq!(after.right_db.to_bits(), 0.0_f32.to_bits());
}
