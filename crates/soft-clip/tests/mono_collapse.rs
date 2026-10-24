//! Issue #1872: the soft clip's collapsed body renders the dual body's left plane, to the bit, and
//! its disengage copy hands the right channel the dual run's state.
//!
//! Everything goes through the shipped contract calls: `bind_homogeneous_bank`, `process_bank`,
//! `process_bank_mono`, `desymmetrize_channels`, `channels_agree`, `lane_channel_symmetry`, the
//! scalar `channel_symmetry` and the state payload restore.

mod support;

use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, InitialParameterValue, ParameterChannel,
    PreparedAutomationSpan, PreparedNativeEffectBank, StatePayloadInput,
};
use lane::Backend;
use support::{QUANTUM, as_input, point, prepare, prepare_bank, snapshot, snapshot_bank};

/// Not a divisor of the 32-row ring, so the cursor lands somewhere new every block and a copy that
/// skipped `pos` would hand the right channel a rotated window.
const FRAMES: usize = 37;
/// The collapsed blocks before the disengage.
const DISENGAGE: usize = 24;
/// The payload words of one lane and channel; the first twelve are the ramps (state word, target,
/// step, remaining per parameter), the rest the `x`, `e` and dry histories by age.
const LANE_WORDS: usize = 104;
const DESIGNED_WORDS: usize = 12;

fn width() -> BankWidth {
    BankWidth::for_backend(Backend::current()).expect("a native bank width")
}

fn lanes() -> usize {
    width().lanes() as usize
}

/// Per-lane values, left equal to right: drive, output and mix differ by lane, and every lane is
/// off the identity path, so the wet path renders.
fn lane_values(lane: usize) -> Vec<InitialParameterValue> {
    let drive = 6.0 + 3.0 * lane as f32;
    let output = 0.0 - 1.5 * lane as f32;
    let mix = 1.0 - 0.1 * lane as f32;
    support::values_from([(drive, drive), (output, output), (mix, mix)]).to_vec()
}

fn bank() -> Box<dyn PreparedNativeEffectBank> {
    let per_lane: Vec<_> = (0..lanes()).map(lane_values).collect();
    prepare_bank(width(), &per_lane).expect("native bank")
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

/// One lane's input word at `frame`: signed zeros, subnormals, a burst past +-1 and ordinary
/// noise. The lanes differ by their seed.
fn sample(lane: usize, frame: usize) -> f32 {
    let value = noise(0x1872 + lane as u64, frame);
    match frame % 11 {
        0 => -0.0,
        1 => f32::from_bits(1),
        2 => -f32::from_bits(0x007F_FFFF),
        _ if (300..500).contains(&frame) || (1000..1100).contains(&frame) => value * 2.5,
        _ => value * 0.4,
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

/// The same spans on both channels of every lane: drive and mix at block 3, and drive, output and
/// mix at block 23, whose 64-sample ramps are 27 samples from their targets at the disengage.
fn automation(step: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let points: &[(u32, f32)] = match step {
        3 => &[(0, 18.0), (2, 0.7)],
        23 => &[(0, -3.0), (1, 4.0), (2, 0.35)],
        _ => &[],
    };
    let first = (step * FRAMES) as u64;
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes() + 1];
    for lane in 0..lanes() {
        for &(parameter, value) in points {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                spans.push(point(parameter, channel, value, first));
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
    frames: usize,
    first_sample: u64,
    spans: (&[PreparedAutomationSpan], &[u32]),
    mono: bool,
) -> BankProcessReport {
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        frames as u32,
        width(),
        first_sample,
        spans.0,
        spans.1,
        QUANTUM,
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

/// The checks the randomized collapsed-against-dual oracle (`tests/randomized.rs`, `witness:
/// true`) does not make; that oracle owns the left plane, the reports and the state. A collapsed
/// block never writes the right plane, a D7 recovery included. After collapsed blocks with drive,
/// output and mix ramps in flight, the disengage copy leaves the channels agreeing (red on a copy
/// that skips `pos` or a ring's mirror rows). `channels_agree` is false after one dual block with a
/// Left-only span and true after the copy, and false when the channels differ in the cursor alone.
#[test]
fn a_collapsed_bank_spares_the_right_plane_and_its_copy_makes_the_channels_agree() {
    let lanes = lanes();
    let mut mixed = bank();
    assert!(mixed.supports_mono_collapse());
    for step in 0..DISENGAGE {
        let (spans, offsets) = automation(step);
        let mut left = block(step);
        let stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
        let mut right = stale.clone();
        render(
            mixed.as_mut(),
            &mut left,
            &mut right,
            FRAMES,
            (step * FRAMES) as u64,
            (&spans, &offsets),
            true,
        );
        assert_words(&right, &stale, &format!("block {step}: untouched right"));
    }
    mixed.desymmetrize_channels();
    assert!(mixed.channels_agree(), "after the copy");

    // One dual block with a Left-only drive span on lane 0: the channels disagree, and the copy
    // makes them agree again.
    let first = (DISENGAGE * FRAMES) as u64;
    let spans = [point(0, ParameterChannel::Left, 12.0, first)];
    let mut offsets = vec![spans.len() as u32; lanes + 1];
    offsets[0] = 0;
    let mut left = block(DISENGAGE);
    let mut right = left.clone();
    render(
        mixed.as_mut(),
        &mut left,
        &mut right,
        FRAMES,
        first,
        (&spans, &offsets),
        false,
    );
    assert!(!mixed.channels_agree(), "after a Left-only span");
    mixed.desymmetrize_channels();
    assert!(mixed.channels_agree(), "re-equalized");

    // A fresh bank, one silent collapsed block: every lane is at rest, so only the left cursor
    // moves, and the channels then differ in `pos` alone.
    let mut fresh = bank();
    assert!(fresh.channels_agree(), "a fresh bank");
    let offsets = vec![0_u32; lanes + 1];
    let mut silent = vec![0.0; FRAMES * lanes];
    let mut stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes];
    render(
        fresh.as_mut(),
        &mut silent,
        &mut stale,
        FRAMES,
        0,
        (&[], &offsets),
        true,
    );
    assert!(!fresh.channels_agree(), "the cursors differ");
    fresh.desymmetrize_channels();
    assert!(fresh.channels_agree(), "the cursor copied");
}

/// Two distinct lane payloads: `a` a lane mid-ramp, `b` another lane at rest, after different
/// plain noise (no signed zeros or subnormals, which two lanes could share). Every one of the 104
/// words differs between them.
fn two_payloads() -> (Vec<u8>, Vec<u8>) {
    let lanes = lanes();
    let mut source = bank();
    let mut offsets = vec![0_u32; lanes + 1];
    for step in 0..3 {
        let mut left: Vec<f32> = (0..FRAMES * lanes)
            .map(|word| {
                noise(
                    0x7_1872 + (word % lanes) as u64,
                    step * FRAMES + word / lanes,
                )
            })
            .collect();
        let mut right = left.clone();
        let first = (step * FRAMES) as u64;
        let spans: Vec<_> = if step == 2 {
            vec![
                point(0, ParameterChannel::Left, 20.0, first),
                point(1, ParameterChannel::Left, -6.0, first),
                point(2, ParameterChannel::Left, 0.25, first),
            ]
        } else {
            Vec::new()
        };
        for offset in &mut offsets[1..] {
            *offset = spans.len() as u32;
        }
        render(
            source.as_mut(),
            &mut left,
            &mut right,
            FRAMES,
            first,
            (&spans, &offsets),
            false,
        );
    }
    let a = snapshot_bank(source.as_ref(), 0).1;
    let b = snapshot_bank(source.as_ref(), lanes as u32 - 1).1;
    (a, b)
}

/// The restore of one lane whose left section is `left` and right section `right`.
fn restore(bank: &mut dyn PreparedNativeEffectBank, track: u32, left: &[u8], right: &[u8]) {
    let (common, _, _) = snapshot_bank(bank, track);
    bank.restore_track_state_payload(
        track,
        1,
        StatePayloadInput {
            common: &common,
            left,
            right,
        },
    )
    .expect("restore");
}

fn with_word(base: &[u8], other: &[u8], word: usize) -> Vec<u8> {
    let mut bytes = base.to_vec();
    bytes[word * 4..word * 4 + 4].copy_from_slice(&other[word * 4..word * 4 + 4]);
    bytes
}

/// The word lists. For every payload word of one lane, a right channel that differs from the left
/// in that word alone makes `channels_agree` false, and makes the `DESIGNED` witness decline on
/// exactly that lane when the word is a ramp or state word (the first twelve) and on no lane when
/// it is a history word; equal channels agree and every lane's witness holds. The scalar witness
/// answers the same for a lone instance, and both decline past the width.
#[test]
fn the_word_lists_are_complete_and_the_witness_declines_on_exactly_the_asymmetric_lane() {
    let lanes = lanes();
    let (a, b) = two_payloads();
    for word in 0..LANE_WORDS {
        assert_ne!(
            a[word * 4..word * 4 + 4],
            b[word * 4..word * 4 + 4],
            "fixture word {word} must differ"
        );
    }
    let track = 1.min(lanes - 1);
    let mut equal = bank();
    restore(equal.as_mut(), track as u32, &a, &a);
    assert!(equal.channels_agree(), "equal channels");
    assert!((0..lanes).all(|lane| equal.lane_channel_symmetry(lane)));
    assert!(!equal.lane_channel_symmetry(lanes));
    for word in 0..LANE_WORDS {
        let right = with_word(&a, &b, word);
        let mut odd = bank();
        restore(odd.as_mut(), track as u32, &a, &right);
        assert!(!odd.channels_agree(), "word {word}: channels_agree");
        let designed = word < DESIGNED_WORDS;
        for lane in 0..lanes {
            assert_eq!(
                odd.lane_channel_symmetry(lane),
                !(designed && lane == track),
                "word {word}, lane {lane}: witness"
            );
        }
    }

    let scalar_values = support::initial_values();
    let mut scalar = prepare(&scalar_values);
    assert!(scalar.channel_symmetry(), "a symmetric instance");
    let sections = snapshot(scalar.as_ref());
    for word in [0, 3, 11, DESIGNED_WORDS, LANE_WORDS - 1] {
        let input = (sections.0.clone(), a.clone(), with_word(&a, &b, word));
        scalar
            .restore_state_payload(1, as_input(&input))
            .expect("restore");
        assert_eq!(
            scalar.channel_symmetry(),
            word >= DESIGNED_WORDS,
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
/// gets there), `advance_clocks` stands in for three zero blocks the reference renders (red on a
/// skip that leaves the cursor behind), and after the disengage the payloads and the next dual
/// blocks are the never-collapsed bank's, bit for bit.
#[test]
fn a_collapsed_bank_that_falls_silent_is_at_rest_and_skips_exactly() {
    const SKIPPED: usize = 3;
    const REST_BOUND: usize = 1_000;
    let lanes = lanes();
    let all = (1_u32 << lanes) - 1;
    let offsets = vec![0_u32; lanes + 1];
    let mut mixed = bank();
    let mut never = bank();
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
        render(
            never,
            &mut never_left,
            &mut never_right,
            FRAMES,
            first,
            spans,
            false,
        );
        step += 1;
        if skip {
            mixed.advance_clocks(all, FRAMES as u32);
            return;
        }
        render(
            mixed,
            &mut mixed_left,
            &mut mixed_right,
            FRAMES,
            first,
            spans,
            mono,
        );
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
    for lane in 0..lanes as u32 {
        assert_eq!(
            snapshot_bank(mixed.as_ref(), lane),
            snapshot_bank(never.as_ref(), lane),
            "lane {lane}: the payload after the skip and the disengage"
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
            FRAMES,
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
