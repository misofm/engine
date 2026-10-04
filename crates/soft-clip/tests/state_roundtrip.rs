#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E10 — state layout 1 round-trips exactly, into a scalar instance and into one lane of a bank.
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
use support::{as_input, bits, prepare, prepare_bank, process, process_bank, values_from, word};

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

/// #1071: every word the effect itself can hold restores, subnormals included, and the restored
/// instance continues bit for bit. A subnormal mix is inside its `[0, 1]` domain, a ramp between
/// two subnormal mixes steps by a subnormal, and the dry history holds the input unflushed.
#[test]
fn a_snapshot_holding_subnormal_words_restores_and_continues_bit_for_bit() {
    const MIX_CURRENT: usize = 8;
    const MIX_STEP: usize = 10;
    const NEWEST_DRY: usize = 73;
    let tiny = |ulps: u32| f32::from_bits(ulps);
    let values = values_from([(6.0, -6.0), (0.0, 3.0), (tiny(200), tiny(7))]);
    let mut source = prepare(&values);
    let input = |index: usize, lane: usize| {
        if index.is_multiple_of(3) {
            tiny(5 + index as u32 + lane as u32)
        } else {
            signal(index, lane)
        }
    };
    // 16 frames in, the last of them subnormal, with the left mix ramping toward zero.
    let mut left: Vec<f32> = (0..16).map(|index| input(index, 0)).collect();
    let mut right: Vec<f32> = (0..16).map(|index| input(index, 1)).collect();
    let spans = [support::point(2, ParameterChannel::Left, 0.0, 0)];
    process(source.as_mut(), &mut left, &mut right, 0, &spans);
    let payload = support::snapshot(source.as_ref());
    for (section, name) in [(&payload.1, "left"), (&payload.2, "right")] {
        assert!(
            support::word_f32(section, MIX_CURRENT).is_subnormal(),
            "{name} mix current"
        );
        assert!(
            support::word_f32(section, NEWEST_DRY).is_subnormal(),
            "{name} newest dry"
        );
    }
    assert!(
        support::word_f32(&payload.1, MIX_STEP).is_subnormal(),
        "left mix step"
    );

    let mut destination = prepare(&values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]));
    destination
        .restore_state_payload(1, as_input(&payload))
        .expect("the effect's own snapshot restores");
    assert_eq!(support::snapshot(destination.as_ref()), payload);

    let mut expected_left: Vec<f32> = (0..128).map(|index| input(index + 16, 0)).collect();
    let mut expected_right: Vec<f32> = (0..128).map(|index| input(index + 16, 1)).collect();
    let mut actual_left = expected_left.clone();
    let mut actual_right = expected_right.clone();
    process(
        source.as_mut(),
        &mut expected_left,
        &mut expected_right,
        16,
        &[],
    );
    process(
        destination.as_mut(),
        &mut actual_left,
        &mut actual_right,
        16,
        &[],
    );
    assert_eq!(bits(&actual_left), bits(&expected_left));
    assert_eq!(bits(&actual_right), bits(&expected_right));
}

/// One parameter ramped from `start` to a domain edge, snapshotted `frames` samples in, where the
/// D11 ramp has crossed that edge; the snapshot must restore into a fresh instance and continue
/// bit for bit (#1071 attempt 2).
fn an_overshooting_ramp_restores_and_continues(
    parameter: u32,
    start: f32,
    edge: f32,
    frames: usize,
    outside: impl Fn(f32) -> bool,
) {
    let mut initial = [(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)];
    initial[parameter as usize] = (start, start);
    let values = values_from(initial);
    let mut source = prepare(&values);
    let mut left: Vec<f32> = (0..frames).map(|index| signal(index, 0)).collect();
    let mut right: Vec<f32> = (0..frames).map(|index| signal(index, 1)).collect();
    let spans = [support::point(parameter, ParameterChannel::Left, edge, 0)];
    process(source.as_mut(), &mut left, &mut right, 0, &spans);
    let payload = support::snapshot(source.as_ref());
    let base = parameter as usize * 4;
    let current = support::word_f32(&payload.1, base);
    assert!(
        word(&payload.1, base + 3) > 0,
        "the ramp is still in flight"
    );
    assert!(
        outside(current),
        "the ramp has crossed its edge: current {current:e} ({:#010x})",
        current.to_bits()
    );
    restores_and_continues(source, &payload, frames);
}

/// `payload`, the snapshot of `source` after `frames` samples, restores into a fresh instance
/// with different initial values, snapshots back to itself, and the two continue bit for bit.
fn restores_and_continues(
    mut source: Box<dyn effect_contract::PreparedNativeEffect>,
    payload: &(Vec<u8>, Vec<u8>, Vec<u8>),
    frames: usize,
) {
    let mut destination = prepare(&values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]));
    destination
        .restore_state_payload(1, as_input(payload))
        .expect("the effect's own mid-ramp snapshot restores");
    assert_eq!(&support::snapshot(destination.as_ref()), payload);

    let mut expected_left: Vec<f32> = (0..128).map(|index| signal(index + frames, 0)).collect();
    let mut expected_right: Vec<f32> = (0..128).map(|index| signal(index + frames, 1)).collect();
    let mut actual_left = expected_left.clone();
    let mut actual_right = expected_right.clone();
    let first_sample = frames as u64;
    process(
        source.as_mut(),
        &mut expected_left,
        &mut expected_right,
        first_sample,
        &[],
    );
    process(
        destination.as_mut(),
        &mut actual_left,
        &mut actual_right,
        first_sample,
        &[],
    );
    assert_eq!(bits(&actual_left), bits(&expected_left));
    assert_eq!(bits(&actual_right), bits(&expected_right));
    assert_eq!(
        support::snapshot(source.as_ref()),
        support::snapshot(destination.as_ref())
    );
}

/// A decibel value next to `edge_db` (below it when `below`) whose gain is 34 to 60 ulps from the
/// edge's gain: far enough for the ramp's rounded step to carry it past the edge.
fn decibels_near(edge_db: f32, below: bool) -> f32 {
    let edge = math::db_to_gain_f32(edge_db).to_bits();
    let mut decibels = edge_db;
    loop {
        decibels = if below {
            decibels.next_down()
        } else {
            decibels.next_up()
        };
        let distance = math::db_to_gain_f32(decibels).to_bits().abs_diff(edge);
        if (34..=60).contains(&distance) {
            return decibels;
        }
        assert!(distance < 60, "no start value near {edge_db} dB");
    }
}

/// A subnormal mix ramped to `0.0` steps by a whole negative unit and crosses zero: the effect
/// holds a negative subnormal mix for the rest of the ramp.
#[test]
fn a_mix_ramp_to_zero_that_crosses_into_negative_subnormals_restores() {
    an_overshooting_ramp_restores_and_continues(2, f32::from_bits(40), 0.0, 48, |current| {
        current < 0.0 && current.is_subnormal()
    });
}

/// A mix ramped to `1.0` from 103 ulps below it ends above `1.0`.
#[test]
fn a_mix_ramp_to_one_that_crosses_above_one_restores() {
    an_overshooting_ramp_restores_and_continues(
        2,
        f32::from_bits(1.0_f32.to_bits() - 103),
        1.0,
        60,
        |current| current > 1.0,
    );
}

/// A drive ramped to its `+36 dB` top from just below it ends above the top's gain.
#[test]
fn a_drive_ramp_to_its_top_that_crosses_above_it_restores() {
    let top = math::db_to_gain_f32(36.0);
    an_overshooting_ramp_restores_and_continues(0, decibels_near(36.0, true), 36.0, 60, |gain| {
        gain > top
    });
}

/// An output ramped to its `-24 dB` bottom from just above it ends below the bottom's gain.
#[test]
fn an_output_ramp_to_its_bottom_that_crosses_below_it_restores() {
    let bottom = math::db_to_gain_f32(-24.0);
    an_overshooting_ramp_restores_and_continues(
        1,
        decibels_near(-24.0, false),
        -24.0,
        60,
        |gain| gain < bottom,
    );
}

/// A drive ramp that has overshot `+36 dB` is retargeted one frame inward, so a new ramp starts
/// from the overshoot and its current is still past the top with 63 samples to go (#1071 attempt 2
/// review, MINOR-1). Its distance from its own line `target - remaining * step` is a fraction of an
/// ulp, but `remaining * step` is about three times the restore's tolerance, so a restore that
/// flips the line's sign or drops its slope (`line = target`) refuses this, the effect's own
/// snapshot.
#[test]
fn a_drive_overshoot_retargeted_inward_restores_and_continues() {
    let top = math::db_to_gain_f32(36.0);
    let first_block = |start: f32| {
        let values = values_from([(start, -6.0), (0.0, 3.0), (1.0, 0.5)]);
        let mut source = prepare(&values);
        let mut left: Vec<f32> = (0..63).map(|index| signal(index, 0)).collect();
        let mut right: Vec<f32> = (0..63).map(|index| signal(index, 1)).collect();
        let spans = [support::point(0, ParameterChannel::Left, 36.0, 0)];
        process(source.as_mut(), &mut left, &mut right, 0, &spans);
        source
    };
    // The start (a few dozen decibel ulps below the top) whose ramp ends furthest above it.
    let overshoot =
        |start: f32| support::word_f32(&support::snapshot(first_block(start).as_ref()).1, 0);
    let mut start = 36.0_f32;
    let mut best = start;
    for _ in 0..40 {
        start = start.next_down();
        if overshoot(start) > overshoot(best) {
            best = start;
        }
    }
    let mut source = first_block(best);
    let mut inward = 36.0_f32;
    for _ in 0..50 {
        inward = inward.next_down();
    }
    let mut left = [signal(63, 0)];
    let mut right = [signal(63, 1)];
    let spans = [support::point(0, ParameterChannel::Left, inward, 63)];
    process(source.as_mut(), &mut left, &mut right, 63, &spans);
    let payload = support::snapshot(source.as_ref());
    let current = support::word_f32(&payload.1, 0);
    let step = support::word_f32(&payload.1, 2);
    assert_eq!(
        word(&payload.1, 3),
        63,
        "the inward ramp has 63 samples to go"
    );
    assert!(step < 0.0, "the ramp moves inward");
    assert!(
        current > top,
        "the current is still past the top: {current:e} against {top:e}"
    );
    restores_and_continues(source, &payload, 64);
}

/// The same payload restored into a bank at a *different* cursor position renders the same block.
#[test]
fn a_bank_track_restore_is_position_independent_and_lane_local() {
    let width = BankWidth::for_backend(lane::Backend::current()).expect("a vector build");
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

    // An invalid declared version is rejected outright.
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
    let short = &payload.1[..payload.1.len() - 4];
    assert_eq!(
        effect.restore_state_payload(
            1,
            StatePayloadInput {
                common: &payload.0,
                left: short,
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
    // A subnormal gain step, on the left output's ramp at rest (so only the step is wrong): only
    // the mix ever steps by a subnormal.
    bad(6, 1, "effect.state.parameter");
    // `remaining` beyond the smoothing window.
    bad(3, 65, "effect.state.parameter");
    // A NaN history word, and a subnormal one.
    bad(12, f32::NAN.to_bits(), "effect.state.history");
    bad(43, 1, "effect.state.history");
    // #1071 accepts only the subnormals the effect can hold: never in the flushed `X` history,
    // never as a gain (the gains' converted range starts at -24 dB). Word 0 is the in-flight drive
    // ramp's current: a subnormal there is outside the range and far off the ramp's own line, so
    // the overshoot allowance (#1071 attempt 2) does not admit it.
    bad(12, 1, "effect.state.history");
    bad(0, 1, "effect.state.parameter");
    bad(73, f32::NAN.to_bits(), "effect.state.history");
    bad(103, f32::INFINITY.to_bits(), "effect.state.history");

    // The overshoot allowance is for in-flight ramps only (#1071 attempt 2 review, MINOR-2). An
    // output at rest at its +24 dB top with its current one ulp above the top: a ramp at rest
    // holds exactly its target, so this is never the effect's own word.
    let top = values_from([(6.0, -6.0), (24.0, 3.0), (1.0, 0.5)]);
    let mut at_rest = support::snapshot(prepare(&top).as_ref());
    assert_eq!(word(&at_rest.1, 7), 0, "the output ramp is at rest");
    let above = math::db_to_gain_f32(24.0).next_up();
    at_rest.1[16..20].copy_from_slice(&above.to_bits().to_le_bytes());
    assert_eq!(
        prepare(&top).restore_state_payload(1, as_input(&at_rest)),
        Err(StatePayloadError {
            code: "effect.state.parameter"
        }),
        "an output at rest one ulp above its top"
    );
    // And `-0.0` is never a current, even on an in-flight mix ramp toward `0.0` whose line is
    // within the tolerance of zero: a mix from 40 subnormal units to zero, 48 frames in.
    let subnormal = values_from([(6.0, -6.0), (0.0, 3.0), (f32::from_bits(40), 0.5)]);
    let mut source = prepare(&subnormal);
    let mut left = vec![0.25_f32; 48];
    let mut right = left.clone();
    let spans = [support::point(2, ParameterChannel::Left, 0.0, 0)];
    process(source.as_mut(), &mut left, &mut right, 0, &spans);
    let mut negative_zero = support::snapshot(source.as_ref());
    assert!(word(&negative_zero.1, 11) > 0, "the mix ramp is in flight");
    negative_zero.1[32..36].copy_from_slice(&(-0.0_f32).to_bits().to_le_bytes());
    assert_eq!(
        prepare(&subnormal).restore_state_payload(1, as_input(&negative_zero)),
        Err(StatePayloadError {
            code: "effect.state.parameter"
        }),
        "an in-flight mix current of -0.0"
    );

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
}
