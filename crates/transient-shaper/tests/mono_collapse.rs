//! Issue #1873: the transient shaper's collapsed body renders the dual body's left plane, to the
//! bit, and its disengage copy hands the right channel the dual run's state.
//!
//! Everything goes through the shipped contract calls: `bind_homogeneous_bank`, `process_bank`,
//! `process_bank_mono`, `desymmetrize_channels`, `channels_agree`, `lane_channel_symmetry`, the
//! scalar `channel_symmetry` and the state payload restore.

mod common;

use common::{
    PARAMETER_COUNT, bank_snapshot, initial_values, native_bank, point, prepare, request_full,
    snapshot, state_sizes, values_of,
};
use effect_contract::{
    BankProcessReport, EffectBankProcessBlock, InitialParameterValue, LinkMode,
    NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PreparedAutomationSpan,
    PreparedNativeEffectBank, StatePayloadInput,
};
use effect_runtime::state_payload::{write_f32, write_u32};
use transient_shaper::TransientShaperFactory;

const FRAMES: usize = 32;
const BLOCKS: usize = 48;
/// The first dual block: the disengage lands here, with the spans of block 23 (64-sample ramps)
/// 32 samples from their targets.
const DISENGAGE: usize = BLOCKS / 2;
/// Words of one lane's state section: `fast`, `slow`, then each ramp's four words.
const LANE_WORDS: usize = 14;
/// The first designed (ramp) word; the two before it are the envelopes.
const DESIGNED_FROM: usize = 2;

type Values = [InitialParameterValue; PARAMETER_COUNT * 2];

fn lanes() -> usize {
    native_bank().expect("a native bank width").1.lanes() as usize
}

/// Per-lane values, left equal to right, off the identity path on every lane.
fn lane_values(lane: usize) -> Values {
    let attack = 0.9 - 0.2 * lane as f32;
    let sustain = -0.6 + 0.15 * lane as f32;
    let mix = 1.0 - 0.05 * lane as f32;
    values_of(attack, sustain, mix)
}

/// A bank whose lane `l` runs `links[l % links.len()]`: one mode per bank, or a mixed bank.
fn bank(links: &[LinkMode]) -> Box<dyn PreparedNativeEffectBank> {
    let (backend, width) = native_bank().expect("a native bank width");
    let values: Vec<Values> = (0..lanes()).map(lane_values).collect();
    let requests: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(lane, values)| request_full(values, 48_000, 128, false, links[lane % links.len()]))
        .collect();
    TransientShaperFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("bank binding")
        .expect("the native width is available")
        .processor
}

/// A small deterministic generator in `[-1, 1)`.
fn noise(seed: u64, frame: usize) -> f32 {
    let mut x = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(frame as u64);
    x ^= x >> 33;
    x = x.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    x ^= x >> 29;
    ((x >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
}

/// One lane's input word at `frame`: onsets (a quiet bed, then loud bursts, one in flight at the
/// disengage), signed zeros and subnormals. The lanes differ by their seed.
fn sample(lane: usize, frame: usize) -> f32 {
    let value = noise(0x1873 + lane as u64, frame);
    let loud = (100..300).contains(&frame) || (700..800).contains(&frame) || frame >= 1200;
    match frame % 13 {
        0 => -0.0,
        1 => f32::from_bits(1),
        2 => -f32::from_bits(0x007F_FFFF),
        _ if loud => value * 0.9,
        _ => value * 0.01,
    }
}

/// Packed AoSoA words of block `step`. One non-finite word while collapsed: D7 recovers lane 1.
fn block(step: usize) -> Vec<f32> {
    let lanes = lanes();
    let mut words = vec![0.0; FRAMES * lanes];
    for frame in 0..FRAMES {
        for lane in 0..lanes {
            words[frame * lanes + lane] = sample(lane, step * FRAMES + frame);
        }
    }
    if step == 10 {
        words[5 * lanes + 1] = f32::NAN;
    }
    words
}

/// The same spans on both channels of every lane: attack and mix at block 3, and attack, sustain
/// and mix at block 23, whose 64-sample ramps are in flight at the disengage.
fn automation(step: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let points: &[(u32, f32)] = match step {
        3 => &[(0, -0.7), (2, 0.6)],
        23 => &[(0, 0.8), (1, 0.9), (2, 0.3)],
        _ => &[],
    };
    let first = (step * FRAMES) as u64;
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes() + 1];
    for lane in 0..lanes() {
        for &(parameter, value) in points {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                spans.push(point(channel, parameter, first, value));
            }
        }
        offsets[lane + 1] = spans.len() as u32;
    }
    (spans, offsets)
}

fn render(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    spans: (&[PreparedAutomationSpan], &[u32]),
    mono: bool,
) -> BankProcessReport {
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        FRAMES as u32,
        native_bank().expect("a native bank width").1,
        first_sample,
        spans.0,
        spans.1,
        128,
    )
    .expect("bank block");
    if mono {
        bank.process_bank_mono(block)
    } else {
        bank.process_bank(block)
    }
}

fn assert_words(actual: &[f32], expected: &[f32], context: &str) {
    for (word, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(a.to_bits(), e.to_bits(), "{context} word {word}");
    }
}

/// The link configurations: each uniform mode, and a mixed bank whose lanes cycle through the
/// three, so its blocks take the `LINK_MIXED` arm.
fn link_configurations() -> [&'static [LinkMode]; 4] {
    [
        &[LinkMode::DualMono],
        &[LinkMode::Maximum],
        &[LinkMode::Average],
        &[LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average],
    ]
}

/// For each uniform link mode and for a mixed-link bank, a bank that runs collapsed and then
/// disengages renders, block for block, the left plane, the left state and the reports (a D7
/// recovery included) of a never-collapsed bank on `L == R` input, and after the disengage the
/// right plane and the right state too, with attack, sustain and mix ramps in flight at the
/// disengage; the collapsed body never writes the right plane. A collapsed lane's link partner is
/// its own left word, so each lane of the mixed bank takes its own mode's law of the pair
/// `(x, x)`. Then one dual block with different L and R words makes `channels_agree` false
/// exactly when some lane is unlinked, and the copy makes it true.
#[test]
fn a_collapsed_then_desymmetrized_bank_is_a_never_collapsed_bank() {
    let lanes = lanes();
    let sizes = state_sizes();
    for links in link_configurations() {
        let mut mixed = bank(links);
        let mut never = bank(links);
        assert!(mixed.supports_mono_collapse());
        for step in 0..BLOCKS {
            let collapsed = step < DISENGAGE;
            if step == DISENGAGE {
                mixed.desymmetrize_channels();
                assert!(mixed.channels_agree(), "{links:?}: after the copy");
            }
            let context = format!("{links:?} block {step}");
            let (spans, offsets) = automation(step);
            let first = (step * FRAMES) as u64;
            let mut never_left = block(step);
            let mut never_right = never_left.clone();
            let mut mixed_left = never_left.clone();
            let stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
            let mut mixed_right = if collapsed {
                stale.clone()
            } else {
                never_left.clone()
            };
            let never_report = render(
                never.as_mut(),
                &mut never_left,
                &mut never_right,
                first,
                (&spans, &offsets),
                false,
            );
            let mixed_report = render(
                mixed.as_mut(),
                &mut mixed_left,
                &mut mixed_right,
                first,
                (&spans, &offsets),
                collapsed,
            );
            assert_eq!(mixed_report, never_report, "{context}: reports");
            assert_words(&mixed_left, &never_left, &format!("{context}: left plane"));
            if collapsed {
                assert_words(&mixed_right, &stale, &format!("{context}: untouched right"));
            } else {
                assert_words(
                    &mixed_right,
                    &never_right,
                    &format!("{context}: right plane"),
                );
            }
            for track in 0..lanes as u32 {
                let (mixed_l, mixed_r) = bank_snapshot(mixed.as_ref(), track, sizes);
                let (never_l, never_r) = bank_snapshot(never.as_ref(), track, sizes);
                assert_eq!(mixed_l, never_l, "{context} track {track}: left state");
                if !collapsed {
                    assert_eq!(mixed_r, never_r, "{context} track {track}: right state");
                }
            }
        }

        // Different L and R words: an unlinked lane's followers see different detectors, a linked
        // lane's see one, so the channels come apart exactly when some lane is unlinked.
        let mut apart = bank(links);
        let mut left = block(30);
        let mut right: Vec<f32> = left.iter().map(|word| word * 0.25).collect();
        let offsets = vec![0_u32; lanes + 1];
        render(
            apart.as_mut(),
            &mut left,
            &mut right,
            0,
            (&[], &offsets),
            false,
        );
        let unlinked = (0..lanes).any(|lane| links[lane % links.len()] == LinkMode::DualMono);
        assert_eq!(
            apart.channels_agree(),
            !unlinked,
            "{links:?}: after different L and R words"
        );
        apart.desymmetrize_channels();
        assert!(apart.channels_agree(), "{links:?}: re-equalized");
    }
}

/// One lane section from two envelope words and three in-flight ramps `(current, target,
/// remaining)`, with the step `set_target` would give.
fn lane_section(envelopes: [f32; 2], ramps: [(f32, f32, u32); PARAMETER_COUNT]) -> Vec<u8> {
    let mut bytes = vec![0_u8; LANE_WORDS * 4];
    write_f32(&mut bytes, 0, envelopes[0]);
    write_f32(&mut bytes, 1, envelopes[1]);
    for (index, (current, target, remaining)) in ramps.into_iter().enumerate() {
        let word = DESIGNED_FROM + index * 4;
        write_f32(&mut bytes, word, current);
        write_f32(&mut bytes, word + 1, target);
        write_f32(&mut bytes, word + 2, (target - current) / remaining as f32);
        write_u32(&mut bytes, word + 3, remaining);
    }
    bytes
}

/// Two valid lane sections that differ in every one of the fourteen words, every ramp in flight,
/// so a section taking any one word of `b` into `a` is still a valid restore.
fn two_sections() -> (Vec<u8>, Vec<u8>) {
    (
        lane_section(
            [0.25, 0.125],
            [(0.5, -0.25, 40), (-0.75, 0.5, 17), (0.5, 0.875, 9)],
        ),
        lane_section(
            [0.375, 0.0625],
            [(-0.5, 0.75, 33), (0.25, -0.5, 50), (0.125, 0.625, 3)],
        ),
    )
}

fn with_word(base: &[u8], other: &[u8], word: usize) -> Vec<u8> {
    let mut bytes = base.to_vec();
    bytes[word * 4..word * 4 + 4].copy_from_slice(&other[word * 4..word * 4 + 4]);
    bytes
}

fn restore(bank: &mut dyn PreparedNativeEffectBank, track: u32, left: &[u8], right: &[u8]) {
    bank.restore_track_state_payload(
        track,
        1,
        StatePayloadInput {
            common: &[],
            left,
            right,
        },
    )
    .expect("restore");
}

/// The word lists. For every state word of one lane, a right channel that differs from the left
/// in that word alone makes `channels_agree` false, and makes the `DESIGNED` witness decline on
/// exactly that lane when the word is a ramp word and on no lane when it is an envelope word;
/// equal channels agree and every lane's witness holds, past the width it declines. The scalar
/// witness answers the same for a lone instance.
#[test]
fn the_word_lists_are_complete_and_the_witness_declines_on_exactly_the_asymmetric_lane() {
    let lanes = lanes();
    let (a, b) = two_sections();
    for word in 0..LANE_WORDS {
        assert_ne!(
            a[word * 4..word * 4 + 4],
            b[word * 4..word * 4 + 4],
            "fixture word {word} must differ"
        );
    }
    let track = 1.min(lanes - 1);
    let mut equal = bank(&[LinkMode::DualMono]);
    restore(equal.as_mut(), track as u32, &a, &a);
    assert!(equal.channels_agree(), "equal channels");
    assert!((0..lanes).all(|lane| equal.lane_channel_symmetry(lane)));
    assert!(!equal.lane_channel_symmetry(lanes));
    for word in 0..LANE_WORDS {
        let right = with_word(&a, &b, word);
        let mut odd = bank(&[LinkMode::DualMono]);
        restore(odd.as_mut(), track as u32, &a, &right);
        assert!(!odd.channels_agree(), "word {word}: channels_agree");
        let designed = word >= DESIGNED_FROM;
        for lane in 0..lanes {
            assert_eq!(
                odd.lane_channel_symmetry(lane),
                !(designed && lane == track),
                "word {word}, lane {lane}: witness"
            );
        }
    }

    let mut scalar = prepare(&initial_values());
    assert!(scalar.channel_symmetry(), "a symmetric instance");
    assert_eq!(snapshot(scalar.as_ref()).0.len(), LANE_WORDS * 4);
    for word in 0..LANE_WORDS {
        let right = with_word(&a, &b, word);
        scalar
            .restore_state_payload(
                1,
                StatePayloadInput {
                    common: &[],
                    left: &a,
                    right: &right,
                },
            )
            .expect("restore");
        assert_eq!(
            scalar.channel_symmetry(),
            word < DESIGNED_FROM,
            "scalar, word {word}"
        );
    }
}

/// Issue #1894: a collapsed bank whose left channel falls silent is at rest, and a skipped block
/// is exact.
///
/// Dual signal first, so the collapse freezes a right channel that is **not** at rest; then
/// collapsed signal, then collapsed `+0.0` until the left channel rests. The bank must then answer
/// every lane at rest (a predicate that still ANDs the frozen right channel answers `0` and never
/// gets there), `advance_clocks` stands in for three zero blocks the reference renders, and after
/// the disengage the states and the next dual blocks are the never-collapsed bank's, bit for bit.
#[test]
fn a_collapsed_bank_that_falls_silent_is_at_rest_and_skips_exactly() {
    const SKIPPED: usize = 3;
    const REST_BOUND: usize = 20_000;
    let lanes = lanes();
    let all = (1_u32 << lanes) - 1;
    let sizes = state_sizes();
    let offsets = vec![0_u32; lanes + 1];
    // Dual mono: a linked detector would move the left channel with the right one below and hide
    // a flag the disengage failed to clear.
    let mut mixed = bank(&[LinkMode::DualMono]);
    let mut never = bank(&[LinkMode::DualMono]);
    let mut step = 0_usize;
    // `skip`: the reference renders the `+0.0` block, the collapsed bank advances its clocks.
    let mut run = |mixed: &mut dyn PreparedNativeEffectBank,
                   never: &mut dyn PreparedNativeEffectBank,
                   signal: bool,
                   mono: bool,
                   skip: bool| {
        let mut never_left = if signal {
            block(step)
        } else {
            vec![0.0; FRAMES * lanes]
        };
        let mut never_right = never_left.clone();
        let mut mixed_left = never_left.clone();
        let mut mixed_right = if mono {
            vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes]
        } else {
            never_left.clone()
        };
        let first = (step * FRAMES) as u64;
        let spans = (&[][..], &offsets[..]);
        render(never, &mut never_left, &mut never_right, first, spans, false);
        step += 1;
        if skip {
            mixed.advance_clocks(all, FRAMES as u32);
            return;
        }
        render(mixed, &mut mixed_left, &mut mixed_right, first, spans, mono);
        assert_words(&mixed_left, &never_left, &format!("block {step}: left"));
        if !mono {
            assert_words(&mixed_right, &never_right, &format!("block {step}: right"));
        }
    };
    for mono in [false, true] {
        for _ in 0..8 {
            run(mixed.as_mut(), never.as_mut(), true, mono, false);
        }
    }
    let rested = (0..REST_BOUND).any(|_| {
        run(mixed.as_mut(), never.as_mut(), false, true, false);
        mixed.lanes_at_rest() == all
    });
    assert!(
        rested,
        "the collapsed bank's left channel rests within the bound"
    );
    assert_eq!(
        never.lanes_at_rest(),
        all,
        "the reference rests on the same block"
    );
    for _ in 0..SKIPPED {
        run(mixed.as_mut(), never.as_mut(), false, true, true);
    }
    mixed.desymmetrize_channels();
    for track in 0..lanes as u32 {
        assert_eq!(
            bank_snapshot(mixed.as_ref(), track, sizes),
            bank_snapshot(never.as_ref(), track, sizes),
            "track {track}: the state after the skip and the disengage"
        );
    }
    // The flag the disengage clears: one dual block whose left plane is `+0.0` and whose right
    // carries signal moves the right channel off rest, so no lane may answer at rest. A flag left
    // set answers from the rested left channel and the right tail would be skipped.
    for bank in [&mut mixed, &mut never] {
        let mut left = vec![0.0; FRAMES * lanes];
        let mut right = vec![0.25; FRAMES * lanes];
        render(
            bank.as_mut(),
            &mut left,
            &mut right,
            1 << 30,
            (&[], &offsets),
            false,
        );
    }
    assert_eq!(
        mixed.lanes_at_rest() & all,
        0,
        "the right channel is live again"
    );
    for _ in 0..2 {
        run(mixed.as_mut(), never.as_mut(), true, false, false);
    }
}
