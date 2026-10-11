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

/// `payload`, the snapshot of `source` after `frames` samples, restores into a fresh instance
/// with different initial values, snapshots back to itself, and the two continue bit for bit
/// over `blocks` further 128-sample blocks, with equal reports per block.
fn restores_and_continues(
    mut source: Box<dyn effect_contract::PreparedNativeEffect>,
    payload: &(Vec<u8>, Vec<u8>, Vec<u8>),
    frames: usize,
    blocks: usize,
) {
    let mut destination = prepare(&values_from([(0.0, 0.0), (0.0, 0.0), (1.0, 1.0)]));
    destination
        .restore_state_payload(1, as_input(payload))
        .expect("the effect's own snapshot restores");
    assert_eq!(&support::snapshot(destination.as_ref()), payload);

    for block in 0..blocks {
        let start = frames + block * 128;
        let mut expected_left: Vec<f32> = (0..128).map(|index| signal(index + start, 0)).collect();
        let mut expected_right: Vec<f32> = (0..128).map(|index| signal(index + start, 1)).collect();
        let mut actual_left = expected_left.clone();
        let mut actual_right = expected_right.clone();
        let first_sample = start as u64;
        let expected_report = process(
            source.as_mut(),
            &mut expected_left,
            &mut expected_right,
            first_sample,
            &[],
        );
        let actual_report = process(
            destination.as_mut(),
            &mut actual_left,
            &mut actual_right,
            first_sample,
            &[],
        );
        assert_eq!(bits(&actual_left), bits(&expected_left), "block {block}");
        assert_eq!(bits(&actual_right), bits(&expected_right), "block {block}");
        assert_eq!(actual_report, expected_report, "block {block}");
    }
    assert_eq!(
        support::snapshot(source.as_ref()),
        support::snapshot(destination.as_ref())
    );
}

/// One planted history state of [`a_snapshot_holding_non_finite_history_restores_and_continues_bit_for_bit`].
struct NonFiniteRow {
    name: &'static str,
    output_db: (f32, f32),
    mix: (f32, f32),
    left: &'static [(usize, f32)],
    right: &'static [(usize, f32)],
    expect: fn(&str, &[u8]),
}

/// The values of the 31 `X`, 30 `e` and 31 dry words of one payload section.
fn history_words(section: &[u8]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let range = |words: std::ops::Range<usize>| -> Vec<f32> {
        words
            .map(|index| support::word_f32(section, index))
            .collect()
    };
    (range(12..43), range(43..73), range(73..104))
}

/// Soft-clip has no recursive state, so every non-finite word it writes itself lives at most 31
/// samples without D7: it reaches the output (D7 resets the lane), `cubic` clamps it, or only the
/// path the identity select discards reads it. Its own snapshot
/// of such a state must restore and continue bit for bit, D7 reports included (#1300, option C):
///
/// * R1, a finite `1e37` overflows `2 * drive * x` to an infinity in `X`, which the cubic clamps
///   (#1278; #1071 attempt 1 MINOR-2);
/// * R2, two infinite `X` words whose half-band tap products have opposite signs make the
///   interpolated `u` and so `e` NaN, while the identity path outputs the finite dry sample;
/// * R3 and R4, a non-finite input in a block's last 31 samples sits in the dry and `X` histories
///   (and `e` for a NaN) before the 31-sample dry delay brings it to the output.
#[test]
fn a_snapshot_holding_non_finite_history_restores_and_continues_bit_for_bit() {
    let rows = [
        NonFiniteRow {
            name: "R1",
            output_db: (-6.0, 3.0),
            mix: (0.5, 1.0),
            left: &[(120, 1.0e37)],
            right: &[(118, -1.0e37)],
            expect: |name, section| {
                let (x, _, _) = history_words(section);
                let infinite = x.iter().filter(|word| word.is_infinite()).count();
                assert_eq!(infinite, 1, "{name}: one infinity in the X history");
            },
        },
        NonFiniteRow {
            name: "R2",
            output_db: (0.0, 0.0),
            mix: (0.0, 0.0),
            left: &[(118, 1.0e37), (120, -1.0e37)],
            right: &[],
            expect: |name, section| {
                let (x, e, _) = history_words(section);
                assert!(
                    x.iter().any(|word| word.is_infinite()),
                    "{name}: X infinity"
                );
                assert!(e.iter().any(|word| word.is_nan()), "{name}: e NaN");
            },
        },
        NonFiniteRow {
            name: "R3",
            output_db: (0.0, 0.0),
            mix: (1.0, 1.0),
            left: &[(120, f32::INFINITY)],
            right: &[],
            expect: |name, section| {
                let (x, _, dry) = history_words(section);
                assert!(x.contains(&f32::INFINITY), "{name}: X +inf");
                assert!(dry.contains(&f32::INFINITY), "{name}: dry +inf");
            },
        },
        NonFiniteRow {
            name: "R4",
            output_db: (0.0, 0.0),
            mix: (0.0, 0.0),
            left: &[(120, f32::NAN)],
            right: &[],
            expect: |name, section| {
                let (x, e, dry) = history_words(section);
                assert!(x.iter().any(|word| word.is_nan()), "{name}: X NaN");
                assert!(e.iter().any(|word| word.is_nan()), "{name}: e NaN");
                assert!(dry.iter().any(|word| word.is_nan()), "{name}: dry NaN");
            },
        },
    ];
    for row in rows {
        let values = values_from([(36.0, 36.0), row.output_db, row.mix]);
        let mut source = prepare(&values);
        let mut left: Vec<f32> = (0..128).map(|index| signal(index, 0)).collect();
        let mut right: Vec<f32> = (0..128).map(|index| signal(index, 1)).collect();
        for &(index, value) in row.left {
            left[index] = value;
        }
        for &(index, value) in row.right {
            right[index] = value;
        }
        let report = process(source.as_mut(), &mut left, &mut right, 0, &[]);
        let name = row.name;
        assert_eq!(report.nonfinite_left_blocks, 0, "{name}: D7 did not fire");
        assert_eq!(report.nonfinite_right_blocks, 0, "{name}: D7 did not fire");
        assert!(
            left.iter().chain(&right).all(|sample| sample.is_finite()),
            "{name}: the snapshot block's output is finite"
        );
        let payload = support::snapshot(source.as_ref());
        (row.expect)(name, &payload.1);
        if !row.right.is_empty() {
            (row.expect)(name, &payload.2);
        }
        restores_and_continues(source, &payload, 128, 3);
    }
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
    // A subnormal history word: the kernel flushes `e` before it enters the history.
    bad(43, 1, "effect.state.history");
    // #1071 accepts only the subnormals the effect can hold: never in the flushed `X` history,
    // never as a gain (the gains' converted range starts at -24 dB). Word 0 is the in-flight drive
    // ramp's current: a subnormal there is outside the converted range, which holds an in-flight
    // current as it holds one at rest (issue #1411 D1).
    bad(12, 1, "effect.state.history");
    bad(0, 1, "effect.state.parameter");
    // `X` may hold an infinity (an overflowed `2 * drive * x`), and `X`, `e` and dry may hold a
    // NaN (#1300), but the shaped `e` is never infinite, of either sign: the cubic is bounded.
    bad(43, f32::INFINITY.to_bits(), "effect.state.history");
    bad(43, f32::NEG_INFINITY.to_bits(), "effect.state.history");

    // An output at rest at its +24 dB top with its current one ulp above the top: a ramp at rest
    // holds exactly its target, so this is never the effect's own word (#1071 attempt 2 review,
    // MINOR-2; the in-flight case is `a_moving_ramp_word_past_its_domain_is_refused`).
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
    // And `-0.0` is never a current, even on an in-flight mix ramp toward `0.0`: a mix from 40
    // subnormal units to zero, 48 frames in.
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
        .expect("prepare")
        .processor;
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

/// Issue #1411 D1: a ramp word one ulp outside its converted range is refused even while the ramp
/// moves, and even when it lies on its own ramp's line.
///
/// From the effect's own snapshot with all three ramps in flight (drive, output and mix, on both
/// channels), each left ramp in turn is rewritten to a one-sample ramp onto an edge of its
/// converted range (`target` the edge, `step` the exact one-ulp distance, `remaining` 1), with its
/// `current` one ulp outside that edge: the point `target - remaining * step` itself, which the
/// old overshoot allowance (#1071 attempt 2) admitted. The restore refuses it with
/// `effect.state.parameter` and leaves the scalar instance and a bank track unchanged; the same
/// payload with `current` on the edge restores. Red when `decode_lane_words` admits an in-flight
/// `current` outside the converted range again.
#[test]
fn a_moving_ramp_word_past_its_domain_is_refused() {
    let values = values_from([(6.0, -6.0), (0.0, 3.0), (1.0, 0.5)]);
    let mut effect = prepare(&values);
    let mut left: Vec<f32> = (0..8).map(|index| signal(index, 0)).collect();
    let mut right: Vec<f32> = (0..8).map(|index| signal(index, 1)).collect();
    let targets = [(18.0, -12.0), (-6.0, 9.0), (0.25, 0.75)];
    let spans: Vec<_> = targets
        .iter()
        .enumerate()
        .flat_map(|(parameter, (left, right))| {
            [
                support::point(parameter as u32, ParameterChannel::Left, *left, 0),
                support::point(parameter as u32, ParameterChannel::Right, *right, 0),
            ]
        })
        .collect();
    process(effect.as_mut(), &mut left, &mut right, 0, &spans);
    let saved = support::snapshot(effect.as_ref());
    for parameter in 0..3 {
        assert!(
            word(&saved.1, parameter * 4 + 3) > 0,
            "ramp {parameter} is in flight"
        );
    }
    let parameters = soft_clip::SOFT_CLIP_PARAMETERS;
    let converted = |parameter: usize, value: f32| {
        if parameter < 2 {
            math::db_to_gain_f32(value)
        } else {
            value
        }
    };
    let ramp = |parameter: usize, current: f32, target: f32| {
        let mut sections = saved.clone();
        let base = parameter * 16;
        for (offset, bits) in [
            current.to_bits(),
            target.to_bits(),
            (target - current).to_bits(),
            1,
        ]
        .into_iter()
        .enumerate()
        {
            sections.1[base + offset * 4..base + offset * 4 + 4]
                .copy_from_slice(&bits.to_le_bytes());
        }
        sections
    };

    let width = BankWidth::for_backend(lane::Backend::current()).expect("a vector build");
    let per_lane: Vec<Vec<_>> = (0..width.lanes()).map(|_| values.to_vec()).collect();
    let mut bank = prepare_bank(width, &per_lane).expect("bank binds");
    bank.restore_track_state_payload(0, 1, as_input(&saved))
        .expect("the scalar snapshot restores into a bank track");
    let bank_saved = support::snapshot_bank(bank.as_ref(), 0);

    for (parameter, row) in parameters.iter().enumerate() {
        let low = converted(parameter, row.minimum.expect("min"));
        let high = converted(parameter, row.maximum.expect("max"));
        for (outside, edge) in [(low.next_down(), low), (high.next_up(), high)] {
            let case = format!(
                "ramp {parameter}: current {outside:e} ({:#010x})",
                outside.to_bits()
            );
            let crafted = ramp(parameter, outside, edge);
            assert_eq!(
                effect.restore_state_payload(1, as_input(&crafted)),
                Err(StatePayloadError {
                    code: "effect.state.parameter"
                }),
                "{case}"
            );
            assert_eq!(support::snapshot(effect.as_ref()), saved, "{case}");
            assert_eq!(
                bank.restore_track_state_payload(0, 1, as_input(&crafted)),
                Err(StatePayloadError {
                    code: "effect.state.parameter"
                }),
                "bank {case}"
            );
            assert_eq!(
                support::snapshot_bank(bank.as_ref(), 0),
                bank_saved,
                "bank {case}"
            );
            let crafted = ramp(parameter, edge, edge);
            effect
                .restore_state_payload(1, as_input(&crafted))
                .unwrap_or_else(|error| panic!("{case}: on the edge: {}", error.code));
            effect
                .restore_state_payload(1, as_input(&saved))
                .expect("own snapshot");
            bank.restore_track_state_payload(0, 1, as_input(&crafted))
                .unwrap_or_else(|error| panic!("bank {case}: on the edge: {}", error.code));
            bank.restore_track_state_payload(0, 1, as_input(&saved))
                .expect("own snapshot");
        }
    }
}
