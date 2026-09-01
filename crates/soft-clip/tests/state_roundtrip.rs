#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E10 — state layout 2 round-trips exactly, into a scalar instance and into one lane of a bank.
//!
//! The layout lost the per-lane cursor word (D10: one cursor per bank) and gained a `step` word per
//! ramp (D11: the increment is precomputed). What replaces the cursor is *age ordering*: the
//! payload carries `X[n] .. X[n-30]`, `e[n] .. e[n-29]` and `x[n] .. x[n-30]` newest first, and a
//! restore places them relative to whatever position the bank it joins happens to be at. That is
//! the property this file exists for: restoring the same payload into two banks at different
//! positions must give two banks that render the same next block.

mod support;

use effect_contract::{
    BankWidth, NativeEffectFactory, ParameterChannel, PreparedNativeEffectBank, ResetKind,
    StatePayloadError, StatePayloadInput,
};
use soft_clip::SoftClipFactory;
use support::{
    as_input, bank_available, bits, initial_values, prepare, prepare_bank, process, process_bank,
    values_from, word,
};

const FRAMES: usize = 64;

fn signal(index: usize, lane: usize) -> f32 {
    ((index + lane * 13) as f32 * 0.043).sin() * 0.75
}

/// The scalar payload of an instance driven `frames` samples into a live ramp.
fn live_instance(frames: usize) -> (Box<dyn effect_contract::PreparedNativeEffect>, u64) {
    let values = values_from([(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)]);
    let mut effect = prepare(&values);
    let mut left: Vec<f32> = (0..frames).map(|index| signal(index, 0)).collect();
    let mut right: Vec<f32> = (0..frames).map(|index| signal(index, 1)).collect();
    let spans = [
        support::point(0, ParameterChannel::Left, 18.0, 0),
        support::point(2, ParameterChannel::Right, 0.25, 0),
    ];
    process(effect.as_mut(), &mut left, &mut right, 0, &spans);
    (effect, frames as u64)
}

#[test]
fn a_snapshot_restores_into_a_fresh_instance_and_continues_bit_for_bit() {
    // 17 frames in: mid-ramp (remaining 47) and mid-history, at cursor position 17.
    let (mut source, first_sample) = live_instance(17);
    let payload = support::snapshot(source.as_ref());
    assert_eq!(word(&payload.0, 0), 1, "layout version in the header");
    assert_eq!(word(&payload.0, 1), 208, "data word count in the header");
    assert_eq!(word(&payload.1, 3), 47, "drive ramp is still live");

    let values = values_from([(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)]);
    let mut destination = prepare(&values);
    destination
        .restore_state_payload(1, as_input(&payload))
        .expect("restore");
    assert_eq!(support::snapshot(destination.as_ref()), payload);

    let mut expected_left: Vec<f32> = (0..256).map(|index| signal(index + 17, 0)).collect();
    let mut expected_right: Vec<f32> = (0..256).map(|index| signal(index + 17, 1)).collect();
    let mut actual_left = expected_left.clone();
    let mut actual_right = expected_right.clone();
    for offset in (0..256).step_by(FRAMES) {
        process(
            source.as_mut(),
            &mut expected_left[offset..offset + FRAMES],
            &mut expected_right[offset..offset + FRAMES],
            first_sample + offset as u64,
            &[],
        );
        process(
            destination.as_mut(),
            &mut actual_left[offset..offset + FRAMES],
            &mut actual_right[offset..offset + FRAMES],
            first_sample + offset as u64,
            &[],
        );
    }
    assert_eq!(bits(&actual_left), bits(&expected_left));
    assert_eq!(bits(&actual_right), bits(&expected_right));
    assert_eq!(
        support::snapshot(source.as_ref()),
        support::snapshot(destination.as_ref())
    );
}

/// The same payload restored into a bank at a *different* cursor position renders the same block.
#[test]
fn a_bank_track_restore_is_position_independent_and_lane_local() {
    let width = if bank_available(BankWidth::Eight) {
        BankWidth::Eight
    } else if bank_available(BankWidth::Four) {
        BankWidth::Four
    } else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = values_from([(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)]);
    let per_lane: Vec<Vec<_>> = (0..lanes).map(|_| values.to_vec()).collect();
    let (source, _) = live_instance(17);
    let payload = support::snapshot(source.as_ref());

    // Two banks, driven a different number of frames, so their shared cursors differ.
    let advance = |bank: &mut dyn PreparedNativeEffectBank, frames: usize| {
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        for frame in 0..frames {
            for lane in 0..lanes {
                left[frame * lanes + lane] = signal(frame, lane);
                right[frame * lanes + lane] = signal(frame + 3, lane);
            }
        }
        let offsets = vec![0_u32; lanes + 1];
        process_bank(bank, width, &mut left, &mut right, frames, 0, &[], &offsets);
    };

    let mut first = prepare_bank(width, &per_lane).expect("bank binds");
    let mut second = prepare_bank(width, &per_lane).expect("bank binds");
    advance(first.as_mut(), 5);
    advance(second.as_mut(), 23);
    let sibling_before = support::snapshot_bank(second.as_ref(), 0);

    let track = (lanes - 1) as u32;
    first
        .restore_track_state_payload(track, 1, as_input(&payload))
        .expect("restore into bank at position 5");
    second
        .restore_track_state_payload(track, 1, as_input(&payload))
        .expect("restore into bank at position 23");
    assert_eq!(support::snapshot_bank(first.as_ref(), track), payload);
    assert_eq!(support::snapshot_bank(second.as_ref(), track), payload);
    assert_eq!(
        support::snapshot_bank(second.as_ref(), 0),
        sibling_before,
        "a track restore is lane-local"
    );

    // Both banks now render the restored track identically, and identically to the scalar source.
    let mut source = source;
    let render_bank = |bank: &mut dyn PreparedNativeEffectBank| {
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                left[frame * lanes + lane] = signal(frame + 17, 0);
                right[frame * lanes + lane] = signal(frame + 17, 1);
            }
        }
        let offsets = vec![0_u32; lanes + 1];
        process_bank(
            bank,
            width,
            &mut left,
            &mut right,
            FRAMES,
            17,
            &[],
            &offsets,
        );
        let lane = track as usize;
        (
            (0..FRAMES)
                .map(|frame| left[frame * lanes + lane])
                .collect::<Vec<_>>(),
            (0..FRAMES)
                .map(|frame| right[frame * lanes + lane])
                .collect::<Vec<_>>(),
        )
    };
    let mut scalar_left: Vec<f32> = (0..FRAMES).map(|index| signal(index + 17, 0)).collect();
    let mut scalar_right: Vec<f32> = (0..FRAMES).map(|index| signal(index + 17, 1)).collect();
    process(
        source.as_mut(),
        &mut scalar_left,
        &mut scalar_right,
        17,
        &[],
    );
    for bank in [first.as_mut(), second.as_mut()] {
        let (left, right) = render_bank(bank);
        assert_eq!(bits(&left), bits(&scalar_left));
        assert_eq!(bits(&right), bits(&scalar_right));
    }
}

/// Everything a restore has to reject.
#[test]
fn a_restore_rejects_a_stale_version_a_wrong_length_and_every_invalid_word() {
    let (source, _) = live_instance(17);
    let payload = support::snapshot(source.as_ref());
    let values = values_from([(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)]);
    let mut effect = prepare(&values);

    // An invalid declared version is rejected outright; issue #080's registry owns any converting edge.
    assert_eq!(
        effect.restore_state_payload(0, as_input(&payload)),
        Err(StatePayloadError {
            code: "effect.state.version"
        })
    );
    // A header from another layout, at the right length.
    let mut stale = payload.clone();
    stale.0[0] = 0;
    assert_eq!(
        effect.restore_state_payload(1, as_input(&stale)),
        Err(StatePayloadError {
            code: "effect.state.version"
        })
    );
    // A truncated section.
    let short = payload.1[..payload.1.len() - 4].to_vec();
    assert_eq!(
        effect.restore_state_payload(
            1,
            StatePayloadInput {
                common: &payload.0,
                left: &short,
                right: &payload.2,
            },
        ),
        Err(StatePayloadError {
            code: "effect.state.length"
        })
    );

    let bad = |word_index: usize, bits: u32, code: &'static str| {
        let mut broken = payload.clone();
        broken.1[word_index * 4..word_index * 4 + 4].copy_from_slice(&bits.to_le_bytes());
        assert_eq!(
            prepare(&values).restore_state_payload(1, as_input(&broken)),
            Err(StatePayloadError { code }),
            "word {word_index} = {bits:#010x}"
        );
    };
    // A drive gain outside the converted domain (+36 dB is the top).
    bad(0, 1.0e6_f32.to_bits(), "effect.state.parameter");
    // A negative-zero ramp target.
    bad(1, (-0.0_f32).to_bits(), "effect.state.parameter");
    // A non-finite step.
    bad(2, f32::NAN.to_bits(), "effect.state.parameter");
    // `remaining` beyond the smoothing window.
    bad(3, 65, "effect.state.parameter");
    // A NaN history word, and a subnormal one.
    bad(12, f32::NAN.to_bits(), "effect.state.history");
    bad(43, 1, "effect.state.history");
    bad(103, f32::INFINITY.to_bits(), "effect.state.history");

    // A rejected restore leaves the effect untouched.
    let mut untouched = prepare(&values);
    let before = support::snapshot(untouched.as_ref());
    let mut broken = payload.clone();
    broken.2[0..4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
    assert!(
        untouched
            .restore_state_payload(1, as_input(&broken))
            .is_err()
    );
    assert_eq!(support::snapshot(untouched.as_ref()), before);
}

/// Both resets are word-exact: a full reset is the prepared instance, a discontinuity reset keeps
/// the parameters and clears the histories.
#[test]
fn both_resets_are_word_exact() {
    let values = values_from([(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)]);
    let fresh = support::snapshot(prepare(&values).as_ref());
    let (mut effect, _) = live_instance(40);

    let mut discontinuity = SoftClipFactory
        .prepare(support::request(&values))
        .expect("prepare");
    let payload = support::snapshot(effect.as_ref());
    discontinuity
        .restore_state_payload(1, as_input(&payload))
        .expect("restore");
    discontinuity.reset(ResetKind::DiscontinuityKeepParameters);
    let after = support::snapshot(discontinuity.as_ref());
    // Every history word is zero, every ramp rests at its target, and nothing else moved.
    for index in 12..104 {
        assert_eq!(word(&after.1, index), 0, "left history word {index}");
        assert_eq!(word(&after.2, index), 0, "right history word {index}");
    }
    for parameter in 0..3 {
        assert_eq!(
            word(&after.1, parameter * 4),
            word(&payload.1, parameter * 4 + 1),
            "parameter {parameter} snapped to its target"
        );
        assert_eq!(word(&after.1, parameter * 4 + 2), 0, "step cleared");
        assert_eq!(word(&after.1, parameter * 4 + 3), 0, "countdown cleared");
    }

    effect.reset(ResetKind::FullToDefaults);
    assert_eq!(support::snapshot(effect.as_ref()), fresh);
    assert_eq!(initial_values().len(), 6);
}
