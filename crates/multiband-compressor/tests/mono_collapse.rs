//! Issue #1871: the multiband compressor's collapsed body renders the dual body's left plane, to
//! the bit, and its disengage copy hands the right channel the dual run's state.
//!
//! Everything goes through the shipped contract calls: `bind_homogeneous_bank`, `process_bank`,
//! `process_bank_mono`, `desymmetrize_channels`, `channels_agree`, `lane_channel_symmetry`, the
//! scalar `channel_symmetry` and the track state payload.

mod support;

use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, InitialParameterValue, LinkMode,
    NativeEffectFactory, ParameterChannel, PreparedAutomationSpan, PreparedEffectBank,
    PreparedNativeEffect, StatePayloadInput, StatePayloadSizes,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{bank, point, request, request_with, signal, snapshot_track, values};

const QUANTUM: u32 = 128;
const BLOCKS: usize = 56;
/// The first dual block of the mixed arm: the disengage lands here, with the block-29 spans'
/// ramps half-way.
const DISENGAGE: usize = 30;
/// Blocks 12..48 are exact `+0.0`: 4,128 frames from block 13 on, past `N_SILENCE` (4,096 at
/// 48 kHz), so the joint flush arms after the disengage on counters the copy carried.
const SILENT: core::ops::Range<usize> = 12..48;

/// Short blocks where spans land, so a second span lands mid-ramp; long ones elsewhere.
fn frames_of(step: usize) -> usize {
    if step < 12 || step == 29 { 32 } else { 128 }
}

fn first_sample(step: usize) -> usize {
    (0..step).map(frames_of).sum()
}

/// One lane's input word at absolute `frame` of block `step`: two blocks of exact `+0.0` (the
/// bank earns its silent fixed point on block 0 and takes the skip on block 1), a loud burst with
/// subnormal and signed-zero words in it, the silent run, and a second burst.
fn sample(step: usize, frame: usize, noise: &[f32]) -> f32 {
    if step < 2 || SILENT.contains(&step) {
        return 0.0;
    }
    match frame % 11 {
        0 => f32::from_bits(1),
        1 => -0.0,
        2 => -f32::from_bits(0x007F_FFFF),
        _ => noise[frame],
    }
}

fn block(step: usize, lanes: usize, noise: &[Vec<f32>]) -> Vec<f32> {
    let frames = frames_of(step);
    let start = first_sample(step);
    let mut words = vec![0.0; frames * lanes];
    for frame in 0..frames {
        for lane in 0..lanes {
            words[frame * lanes + lane] = sample(step, start + frame, &noise[lane]);
        }
    }
    // One non-finite word while collapsed: D7 recovers lane 1 on both channels, and the report
    // counts both channels on both arms.
    if step == 10 {
        words[5 * lanes + 1] = f32::NAN;
    }
    words
}

/// The same spans on both channels of every lane. Block 2 moves the crossover, the low
/// threshold and the low attack; block 3 retargets the crossover and the threshold mid-ramp;
/// block 8 moves a release; block 29 starts ramps the disengage copy must carry.
fn automation(step: usize, lanes: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let points: &[(u32, f32)] = match step {
        2 => &[(0, 2_000.0), (1, -40.0), (3, 1.0)],
        3 => &[(0, 500.0), (1, -30.0)],
        8 => &[(9, 20.0)],
        29 => &[(0, 3_000.0), (8, 50.0)],
        _ => &[],
    };
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; lanes + 1];
    for lane in 0..lanes {
        for &(parameter_index, value) in points {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                spans.push(point(
                    parameter_index,
                    channel,
                    first_sample(step) as u64,
                    value,
                ));
            }
        }
        offsets[lane + 1] = spans.len() as u32;
    }
    (spans, offsets)
}

fn render(
    bank: &mut PreparedEffectBank,
    width: BankWidth,
    left: &mut [f32],
    right: &mut [f32],
    step: usize,
    mono: bool,
) -> BankProcessReport {
    let (spans, offsets) = automation(step, width.lanes() as usize);
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        frames_of(step) as u32,
        width,
        first_sample(step) as u64,
        &spans,
        &offsets,
        QUANTUM,
    )
    .expect("bank block");
    if mono {
        bank.processor.process_bank_mono(block)
    } else {
        bank.processor.process_bank(block)
    }
}

/// A bank of the descriptor defaults at `link`, and the state sizes its lanes' payloads take.
fn prepare(width: BankWidth, link: LinkMode) -> (PreparedEffectBank, StatePayloadSizes) {
    prepare_links(&vec![link; width.lanes() as usize], width)
}

/// [`prepare`] with lane `l` at `links[l]` (issue #1367: one bank mixes link modes).
fn prepare_links(links: &[LinkMode], width: BankWidth) -> (PreparedEffectBank, StatePayloadSizes) {
    let values = values();
    let requests: Vec<_> = links
        .iter()
        .map(|link| request_with(&values, *link, QUANTUM, false))
        .collect();
    (bank(width, &requests), scalar(&values).1)
}

/// A lone scalar instance over `values`, and its state sizes.
fn scalar(values: &[InitialParameterValue]) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let prepared = MultibandCompressorFactory
        .prepare(request(values))
        .expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

fn assert_words(actual: &[f32], expected: &[f32], context: &str) {
    assert_eq!(actual.len(), expected.len(), "{context}: length");
    for (word, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(a.to_bits(), e.to_bits(), "{context} word {word}");
    }
}

/// For every bound width and every link mode, and for a bank whose lanes mix the three modes
/// (issue #1367: a collapsed lane links with its own `L == R` pair), a bank that runs collapsed and
/// then disengages
/// renders, block for block, the left plane, the left state, the reports and (after the
/// disengage) the right plane and right state of a never-collapsed bank on `L == R` input; the
/// collapsed body never touches the right plane; and `channels_agree` is false after one dual
/// block of different L and R words and true after the copy.
#[test]
fn a_collapsed_then_desymmetrized_bank_is_a_never_collapsed_bank() {
    let total = first_sample(BLOCKS);
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let noise: Vec<Vec<f32>> = (0..lanes)
            .map(|lane| signal(total, 0x1871 + lane as u64))
            .collect();
        let modes = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];
        let mut banks: Vec<Vec<LinkMode>> = modes.iter().map(|link| vec![*link; lanes]).collect();
        banks.push((0..lanes).map(|lane| modes[lane % modes.len()]).collect());
        for link in &banks {
            let (mut mixed, sizes) = prepare_links(link, width);
            let (mut never, _) = prepare_links(link, width);
            assert!(mixed.processor.supports_mono_collapse());
            for step in 0..BLOCKS {
                let collapsed = step < DISENGAGE;
                if step == DISENGAGE {
                    mixed.processor.desymmetrize_channels();
                    assert!(mixed.processor.channels_agree(), "{link:?}: after the copy");
                }
                let context = format!("{width:?} {link:?} block {step}");
                let mut never_left = block(step, lanes, &noise);
                let mut never_right = never_left.clone();
                let mut mixed_left = never_left.clone();
                let stale = vec![f32::from_bits(0x7F7F_FFFF); never_left.len()];
                let mut mixed_right = if collapsed {
                    stale.clone()
                } else {
                    never_left.clone()
                };
                let never_report = render(
                    &mut never,
                    width,
                    &mut never_left,
                    &mut never_right,
                    step,
                    false,
                );
                let mixed_report = render(
                    &mut mixed,
                    width,
                    &mut mixed_left,
                    &mut mixed_right,
                    step,
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
                    let (_, mixed_l, mixed_r) =
                        snapshot_track(mixed.processor.as_ref(), track, sizes);
                    let (_, never_l, never_r) =
                        snapshot_track(never.processor.as_ref(), track, sizes);
                    assert_eq!(mixed_l, never_l, "{context} track {track}: left state");
                    if !collapsed {
                        assert_eq!(mixed_r, never_r, "{context} track {track}: right state");
                    }
                }
            }

            // Channels driven apart by the render disagree in every link mode (the crossover
            // state is per channel even when the detector is linked); the copy makes them agree.
            let (mut apart, _) = prepare_links(link, width);
            let mut left = vec![0.5_f32; 32 * lanes];
            let mut right = vec![0.25_f32; 32 * lanes];
            let offsets = vec![0_u32; lanes + 1];
            apart.processor.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    32,
                    width,
                    0,
                    &[],
                    &offsets,
                    QUANTUM,
                )
                .expect("bank block"),
            );
            assert!(
                !apart.processor.channels_agree(),
                "{link:?}: after different words"
            );
            apart.processor.desymmetrize_channels();
            assert!(apart.processor.channels_agree(), "{link:?}: re-equalized");
        }
    }
}

/// Payload word offsets of one channel section (the crate's frozen layout): two smoother words,
/// the ten parameter ramps, four crossover words, the silence counter, then the coefficient and
/// crossover word ramps.
const GAIN_WORD: usize = 1;
const LOW_ATTACK_RAMP_WORD: usize = 3 + 3 * 4;
const FILTER_WORD: usize = 3 + 10 * 4;
const SILENCE_WORD: usize = FILTER_WORD + 4;

fn set_word(bytes: &mut [u8], word: usize, value: u32) {
    bytes[word * 4..word * 4 + 4].copy_from_slice(&value.to_le_bytes());
}

/// `channels_agree` reads every word the disengage copy moves: on a bank whose channels agree,
/// one restored right-channel word of one lane that differs -- each crossover word, the silence
/// counter, each smoother word, a ramp's `remaining` -- makes it decline. The same restore of a
/// ramp's `remaining` alone makes the `DESIGNED` witness decline on exactly that lane.
#[test]
fn channels_agree_and_the_witness_read_every_copied_word() {
    let width = BankWidth::ALL[0];
    let lanes = width.lanes() as usize;
    let odd = lanes - 1;
    let low_attack_remaining = LOW_ATTACK_RAMP_WORD + 3;
    let edits: [(&str, usize, u32); 8] = [
        ("crossover a.ic1", FILTER_WORD, 0.125_f32.to_bits()),
        ("crossover a.ic2", FILTER_WORD + 1, 0.125_f32.to_bits()),
        ("crossover b.ic1", FILTER_WORD + 2, 0.125_f32.to_bits()),
        ("crossover b.ic2", FILTER_WORD + 3, 0.125_f32.to_bits()),
        ("silence", SILENCE_WORD, 7.0_f32.to_bits()),
        ("low gain", GAIN_WORD, (-1.0_f32).to_bits()),
        ("high gain", GAIN_WORD + 1, (-1.0_f32).to_bits()),
        ("low attack remaining", low_attack_remaining, 1),
    ];
    for (name, word, value) in edits {
        let (mut prepared, sizes) = prepare(width, LinkMode::DualMono);
        assert!(prepared.processor.channels_agree(), "{name}: prepared");
        let (common, left, mut right) =
            snapshot_track(prepared.processor.as_ref(), odd as u32, sizes);
        set_word(&mut right, word, value);
        prepared
            .processor
            .restore_track_state_payload(
                odd as u32,
                1,
                StatePayloadInput::new(&common, &left, &right, sizes).expect("payload"),
            )
            .unwrap_or_else(|error| panic!("{name}: restore refused: {error:?}"));
        assert!(!prepared.processor.channels_agree(), "{name}");
        let designed = word == low_attack_remaining;
        for lane in 0..lanes {
            assert_eq!(
                prepared.processor.lane_channel_symmetry(lane),
                !(designed && lane == odd),
                "{name}: witness on lane {lane}"
            );
        }
        prepared.processor.desymmetrize_channels();
        assert!(
            prepared.processor.channels_agree(),
            "{name}: after the copy"
        );
    }
}

/// The `DESIGNED` witness declines on exactly the lane whose right channel carries a different
/// kernel-read word (a crossover word ramp, a rate ramp), accepts every other lane and declines
/// past the width; the scalar witness answers the same for a lone instance.
#[test]
fn the_witness_declines_on_exactly_the_asymmetric_lane() {
    let base = values();
    // (parameter, right value): the crossover (its three word ramps), the low attack (its rate
    // ramp and its parameter ramp).
    for (parameter, right) in [(0_usize, 1_200.0_f32), (3, 2.0)] {
        let mut asymmetric = base;
        asymmetric[parameter * 2 + 1].value = right;
        for &width in BankWidth::ALL {
            let lanes = width.lanes() as usize;
            for odd in 0..lanes {
                let requests: Vec<_> = (0..lanes)
                    .map(|lane| {
                        if lane == odd {
                            request(&asymmetric)
                        } else {
                            request(&base)
                        }
                    })
                    .collect();
                let prepared = bank(width, &requests);
                for lane in 0..lanes {
                    assert_eq!(
                        prepared.processor.lane_channel_symmetry(lane),
                        lane != odd,
                        "parameter {parameter}, {width:?}, asymmetric lane {odd}, lane {lane}"
                    );
                }
                assert!(!prepared.processor.lane_channel_symmetry(lanes));
            }
        }
        assert!(
            !scalar(&asymmetric).0.channel_symmetry(),
            "parameter {parameter}"
        );
    }
    assert!(scalar(&base).0.channel_symmetry());
}

/// Issue #1894: a collapsed bank whose left side falls silent is at rest, and a skipped block is
/// exact, at every bound width.
///
/// Dual signal first, so the collapse freezes a right side that is **not** at rest; then
/// collapsed signal, then collapsed `+0.0` until the left side rests. The bank must then answer
/// every lane at rest (a predicate that still ANDs the frozen right side answers `0` and never
/// gets there), `advance_clocks` stands in for three zero blocks the reference renders, and after
/// the disengage the payloads and the next dual blocks are the never-collapsed bank's, bit for bit
/// (red if a skipped collapsed block advances the wrong silence counter).
#[test]
fn a_collapsed_bank_that_falls_silent_is_at_rest_and_skips_exactly() {
    const FRAMES: usize = QUANTUM as usize;
    const SIGNAL: usize = 6;
    const SKIPPED: usize = 3;
    const REST_BOUND: usize = 5_000;
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let all = (1_u32 << lanes) - 1;
        let noise: Vec<Vec<f32>> = (0..lanes)
            .map(|lane| signal(FRAMES * 2 * SIGNAL, 0x1894 + lane as u64))
            .collect();
        let (mut mixed, sizes) = prepare(width, LinkMode::DualMono);
        let (mut never, _) = prepare(width, LinkMode::DualMono);
        let offsets = vec![0_u32; lanes + 1];
        let mut step = 0_usize;
        // `skip`: the reference renders the `+0.0` block, the collapsed bank advances its clocks.
        let mut run = |mixed: &mut PreparedEffectBank,
                       never: &mut PreparedEffectBank,
                       signal: bool,
                       mono: bool,
                       skip: bool| {
            let mut never_left: Vec<f32> = (0..FRAMES * lanes)
                .map(|word| {
                    let frame = (step % (2 * SIGNAL)) * FRAMES + word / lanes;
                    if signal {
                        noise[word % lanes][frame]
                    } else {
                        0.0
                    }
                })
                .collect();
            let mut never_right = never_left.clone();
            let mut mixed_left = never_left.clone();
            let mut mixed_right = if mono {
                vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes]
            } else {
                never_left.clone()
            };
            let first = (step * FRAMES) as u64;
            let context = format!("{width:?} block {step}");
            step += 1;
            for (bank, left, right, mono) in [
                (&mut *never, &mut never_left, &mut never_right, false),
                (&mut *mixed, &mut mixed_left, &mut mixed_right, mono),
            ] {
                if skip && mono {
                    bank.processor.advance_clocks(all, FRAMES as u32);
                    return;
                }
                let block = EffectBankProcessBlock::new(
                    left,
                    right,
                    None,
                    FRAMES as u32,
                    width,
                    first,
                    &[],
                    &offsets,
                    QUANTUM,
                )
                .expect("bank block");
                if mono {
                    bank.processor.process_bank_mono(block);
                } else {
                    bank.processor.process_bank(block);
                }
            }
            assert_words(&mixed_left, &never_left, &format!("{context}: left plane"));
            if !mono {
                assert_words(&mixed_right, &never_right, &format!("{context}: right"));
            }
        };
        for mono in [false, true] {
            for _ in 0..SIGNAL {
                run(&mut mixed, &mut never, true, mono, false);
            }
        }
        let rested = (0..REST_BOUND).any(|_| {
            run(&mut mixed, &mut never, false, true, false);
            mixed.processor.lanes_at_rest() == all
        });
        assert!(
            rested,
            "{width:?}: the collapsed left side rests within the bound"
        );
        assert_eq!(
            never.processor.lanes_at_rest(),
            all,
            "{width:?}: the reference"
        );
        for _ in 0..SKIPPED {
            run(&mut mixed, &mut never, false, true, true);
        }
        mixed.processor.desymmetrize_channels();
        for track in 0..lanes as u32 {
            assert_eq!(
                snapshot_track(mixed.processor.as_ref(), track, sizes),
                snapshot_track(never.processor.as_ref(), track, sizes),
                "{width:?} track {track}: the payload after the skip and the disengage"
            );
        }
        for _ in 0..2 {
            run(&mut mixed, &mut never, true, false, false);
        }
    }
}
