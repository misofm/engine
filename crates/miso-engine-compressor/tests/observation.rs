//! Issue #143 P1: the gain-reduction observation tap.
//!
//! This file is #140 D's `gain_reduction.rs`, re-expressed on the declared tap after R5 removed
//! `PreparedNativeEffect::gain_reduction`. The reason the method went away is worth keeping: two
//! routes to one value diverge, and a defaulted `Option<GainReductionV1>` accessor that nothing in
//! the runtime called was the second route. Every assertion below is the same assertion it was --
//! the reading is the kernel's own smoother state, in the contract's negative-for-reduction
//! convention -- addressed through `observe_resident` instead.

mod support;

use miso_engine_effect_contract::{
    EffectProcessBlock, ObservationSample, PreparedNativeEffect, ResetKind,
    validate_descriptor,
};

use support::{initial_values, prepare, request};

fn prepared() -> Box<dyn PreparedNativeEffect> {
    let values = initial_values();
    prepare(request(&values))
}

fn observe(effect: &dyn PreparedNativeEffect) -> ObservationSample {
    let mut sample = ObservationSample::default();
    assert!(
        effect.observe_resident(0, &mut sample),
        "the compressor implements its one declared tap"
    );
    sample
}

fn render(effect: &mut dyn PreparedNativeEffect, value: f32, blocks: usize) -> ObservationSample {
    let quantum = effect.metadata().quantum;
    let mut reduction = ObservationSample::default();
    for block in 0..blocks {
        let mut left = vec![value; quantum as usize];
        let mut right = vec![value; quantum as usize];
        let first_sample = block as u64 * u64::from(quantum);
        let block =
            EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &[], quantum)
                .expect("block");
        let _ = effect.process(block);
        reduction = observe(&*effect);
    }
    reduction
}

/// The declared menu is exactly one `Resident` tap, and it validates.
#[test]
fn the_compressor_declares_one_resident_gain_reduction_tap() {
    use miso_engine_effect_contract::{
        ObservationCadenceV1, ObservationChannelsV1, ObservationCostV1, ObservationFoldV1,
        ObservationKindV1, ObservationTapId, ParameterUnit,
    };
    let descriptor = miso_engine_compressor::COMPRESSOR_DESCRIPTOR_V1;
    validate_descriptor(&miso_engine_compressor::COMPRESSOR_DESCRIPTOR_V1).unwrap();
    assert_eq!(descriptor.observations.len(), 1);
    let tap = descriptor.observations[0];
    assert_eq!(tap.id, ObservationTapId(1));
    assert_eq!(tap.display_name, "Gain Reduction");
    assert_eq!(tap.display_unit, "dB");
    assert_eq!(tap.kind, ObservationKindV1::GainReductionDb);
    assert_eq!(tap.unit, ParameterUnit::Db);
    assert_eq!(tap.cost, ObservationCostV1::Resident);
    assert_eq!(tap.cadence, ObservationCadenceV1::PerBlock);
    assert_eq!(tap.fold, ObservationFoldV1::PeakMagnitude);
    assert_eq!(tap.channels, ObservationChannelsV1::PerLane);
    assert_eq!(tap.minimum.to_bits(), 0.0_f32.to_bits());
    assert_eq!(tap.maximum, 100.0);
    // The addressing rule: a tap the descriptor does not declare is refused, never answered with
    // a stale or zeroed reading.
    let effect = prepared();
    let mut sample = ObservationSample {
        left: f32::NAN,
        right: f32::NAN,
    };
    assert!(!effect.observe_resident(1, &mut sample));
    assert!(
        sample.left.is_nan() && sample.right.is_nan(),
        "out untouched"
    );
}

/// Red mutation: return `true` after writing `0.0` into both lanes instead of reading
/// `Channel::gain_reduction_db` -> the loud case below reports no reduction and fails.
#[test]
fn the_compressor_reports_the_reduction_its_kernel_smoothed() {
    let mut effect = prepared();
    // Well below the prepared threshold: nothing to reduce.
    let quiet = render(effect.as_mut(), 0.000_1, 8);
    assert!(
        quiet.left > -0.5 && quiet.left <= 0.0,
        "a signal under the threshold is barely reduced: {}",
        quiet.left
    );

    effect.reset(ResetKind::FullToDefaults);
    let loud = render(effect.as_mut(), 0.9, 32);
    assert!(
        loud.left < -1.0,
        "a signal well over the threshold is audibly reduced: {}",
        loud.left
    );
    assert!(
        loud.left >= -100.0,
        "the kernel's own clamp bounds the reading: {}",
        loud.left
    );
    assert_eq!(
        loud.left.to_bits(),
        loud.right.to_bits(),
        "a dual-mono compressor fed identical lanes reduces them identically"
    );
}

/// Issue #143 E6: resident means resident.
///
/// Two calls with no `process` between them return identical bits, because `observe_resident`
/// takes `&self` and there is nothing it could have advanced. The `&self` half of the statement is
/// enforced by the trait signature; this is the behavioural half.
///
/// Red mutation: make the read "freshen" the smoother by one release step -> the second call
/// differs from the first.
#[test]
fn a_resident_read_is_repeatable_to_the_bit() {
    let mut effect = prepared();
    let _ = render(effect.as_mut(), 0.9, 12);
    let first = observe(&*effect);
    let second = observe(&*effect);
    let third = observe(&*effect);
    assert_eq!(first.left.to_bits(), second.left.to_bits());
    assert_eq!(first.right.to_bits(), second.right.to_bits());
    assert_eq!(second.left.to_bits(), third.left.to_bits());
    assert!(first.left < 0.0, "the case is not vacuous: {}", first.left);
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
        after_many.left < after_one.left,
        "the envelope is still attacking after one block: one={} many={}",
        after_one.left,
        after_many.left
    );
}

/// A full reset returns the observation to zero, because it returns the smoother to zero.
#[test]
fn a_full_reset_returns_the_reading_to_zero() {
    let mut effect = prepared();
    let loud = render(effect.as_mut(), 0.9, 16);
    assert!(loud.left < 0.0);
    effect.reset(ResetKind::FullToDefaults);
    let after = observe(&*effect);
    assert_eq!(after.left.to_bits(), 0.0_f32.to_bits());
    assert_eq!(after.right.to_bits(), 0.0_f32.to_bits());
}

/// Issue #143 E2, contract half: a bank lane's resident read is that lane's own state.
///
/// Four (or eight) compressors in one cohort, fed four different signals -- one well under the
/// threshold and three biting at different depths -- and each lane's `observe_resident_bank`
/// reading equals an independent scalar instance's `observe_resident` reading **to the bit**. The
/// published half of this eval lives with the transport; what is gated here is the read itself,
/// which is where a lane mix-up would originate.
///
/// Red mutation: broadcast lane 0's reading to every lane (`sample.left = left[0]`) -> lanes 1..W
/// stop matching their scalar twins.
#[test]
fn every_bank_lane_reads_its_own_reduction() {
    use support::{bind_bank, native_bank_width, render_bank, render_scalar, values_with};

    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    // -40 dBFS is far under the prepared threshold and does not bite; the rest do, at three
    // different depths, so a broadcast reading cannot pass by coincidence.
    let amplitudes: Vec<f32> = (0..lanes)
        .map(|lane| match lane % 4 {
            0 => 0.01,
            1 => 0.501_187,
            2 => 0.251_189,
            _ => 0.125_893,
        })
        .collect();

    let values = values_with(&[]);
    let requests: Vec<_> = (0..lanes).map(|_| request(&values)).collect();
    let mut bank = bind_bank(&requests).expect("a bank at this build's width");

    // 8 192 frames is many attack constants (the default attack is 10 ms at 48 kHz), so every
    // biting lane has reached a settled reduction rather than a point on its attack curve.
    let frames = 8_192;
    let mut bank_left = vec![0.0_f32; frames * lanes];
    let mut bank_right = vec![0.0_f32; frames * lanes];
    for frame in 0..frames {
        for lane in 0..lanes {
            bank_left[frame * lanes + lane] = amplitudes[lane];
            bank_right[frame * lanes + lane] = amplitudes[lane];
        }
    }
    render_bank(
        bank.as_mut(),
        &mut bank_left,
        &mut bank_right,
        lanes,
        width,
        128,
        128,
        &[],
    );

    let mut published = vec![ObservationSample::default(); lanes];
    assert!(
        bank.observe_resident_bank(0, &mut published),
        "the bank implements its one declared tap"
    );
    assert!(
        !bank.observe_resident_bank(1, &mut published),
        "an undeclared tap is refused"
    );
    assert!(
        !bank.observe_resident_bank(0, &mut published[..lanes - 1]),
        "a short output slice is refused rather than partially filled"
    );

    let mut biting = 0;
    for (lane, amplitude) in amplitudes.iter().copied().enumerate() {
        let mut scalar = prepare(request(&values));
        let mut left = vec![amplitude; frames];
        let mut right = vec![amplitude; frames];
        render_scalar(scalar.as_mut(), &mut left, &mut right, 128, 128, &[]);
        let expected = observe(&*scalar);
        assert_eq!(
            published[lane].left.to_bits(),
            expected.left.to_bits(),
            "lane {lane} left reading is its own, not a neighbour's"
        );
        assert_eq!(
            published[lane].right.to_bits(),
            expected.right.to_bits(),
            "lane {lane} right reading is its own, not a neighbour's"
        );
        if expected.left < -0.05 {
            biting += 1;
        }
    }
    // Exactly the lanes fed above the threshold reduce; the -40 dBFS lanes do not.
    assert_eq!(
        biting,
        lanes - lanes.div_ceil(4),
        "the case is not vacuous: {biting} of {lanes} lanes are reduced"
    );
    let distinct: std::collections::BTreeSet<u32> =
        published.iter().map(|s| s.left.to_bits()).collect();
    assert!(
        distinct.len() >= 4.min(lanes),
        "the lanes report distinct reductions, so a broadcast cannot pass"
    );
}
