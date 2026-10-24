//! Issue #973: the gate's collapsed body renders the dual body's left plane, to the bit, and its
//! disengage copy hands the right channel the dual run's state.
//!
//! Everything goes through the shipped contract calls: `bind_homogeneous_bank`, `process_bank`,
//! `process_bank_mono`, `desymmetrize_channels`, `channels_agree`, `lane_channel_symmetry` and the
//! scalar `channel_symmetry`.

mod support;

use effect_contract::{
    BankProcessReport, EffectBankProcessBlock, LinkMode, ParameterChannel, PreparedAutomationSpan,
    PreparedEffectBank,
};
use support::{
    NATIVE_LANES, Values, active_values, native_width, noise, prepare, prepare_bank_native,
    request, set_parameter, snapshot_bank,
};

const FRAMES: usize = 32;
const BLOCKS: usize = 48;
/// The first dual block of the mixed arm: the disengage lands here.
const DISENGAGE: usize = BLOCKS / 2;
/// The hold is 10 ms, 480 samples at 48 kHz. The loud burst ends at frame 600, so at the
/// disengage (frame 768) every lane is open and counting its hold down.
const HOLD_MS: f32 = 10.0;
const LOUD_END: usize = 600;

/// One lane's input word at `frame`: a loud burst with subnormal and signed-zero words in it, a
/// quiet stretch of subnormals, both zeros and words near the threshold, and a second burst after
/// the disengage. The lanes differ by their noise seed.
fn sample(lane: usize, frame: usize, noise: &[f32]) -> f32 {
    let loud = frame < LOUD_END || (1100..1200).contains(&frame);
    let quiet = if lane.is_multiple_of(2) {
        1.0e-4
    } else {
        3.0e-3
    };
    match frame % 9 {
        0 => f32::from_bits(1),
        1 => -0.0,
        2 => -f32::from_bits(0x007F_FFFF),
        3 => 0.0,
        _ if loud => noise[frame] * 0.8,
        _ => noise[frame] * quiet,
    }
}

/// Packed AoSoA words of block `step`.
fn block(step: usize, signals: &[Vec<f32>]) -> Vec<f32> {
    let mut words = vec![0.0; FRAMES * NATIVE_LANES];
    for frame in 0..FRAMES {
        for lane in 0..NATIVE_LANES {
            words[frame * NATIVE_LANES + lane] =
                sample(lane, step * FRAMES + frame, &signals[lane]);
        }
    }
    // One non-finite word while collapsed: D7 recovers lane 1, and the report counts both
    // channels on both arms.
    if step == 10 {
        words[5 * NATIVE_LANES + 1] = f32::NAN;
    }
    words
}

/// The same spans on both channels of every lane: a threshold point and an attack time at block
/// 2, and a hold retime at block 20, while every lane is holding, so the right channel's retime
/// reads its frozen countdown.
fn automation(step: usize) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let points: &[(u32, f32)] = match step {
        2 => &[(0, -36.0), (4, 3.0)],
        20 => &[(5, 7.0)],
        _ => &[],
    };
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32; NATIVE_LANES + 1];
    for lane in 0..NATIVE_LANES {
        for &(parameter_index, value) in points {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                spans.push(PreparedAutomationSpan {
                    channel,
                    parameter_index,
                    sample: (step * FRAMES) as u64,
                    value,
                });
            }
        }
        offsets[lane + 1] = spans.len() as u32;
    }
    (spans, offsets)
}

fn values() -> [Values; 8] {
    let mut values = active_values();
    set_parameter(&mut values, 0, -40.0, -40.0);
    set_parameter(&mut values, 5, HOLD_MS, HOLD_MS);
    set_parameter(&mut values, 6, 50.0, 50.0);
    [values; 8]
}

fn render(
    bank: &mut PreparedEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    step: usize,
    mono: bool,
) -> BankProcessReport {
    let (spans, offsets) = automation(step);
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        FRAMES as u32,
        native_width(),
        (step * FRAMES) as u64,
        &spans,
        &offsets,
        128,
    )
    .expect("bank block");
    if mono {
        bank.processor.process_bank_mono(block)
    } else {
        bank.processor.process_bank(block)
    }
}

fn assert_words(actual: &[f32], expected: &[f32], context: &str) {
    for (word, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(a.to_bits(), e.to_bits(), "{context} word {word}");
    }
}

/// For every link mode, a bank that runs collapsed and then disengages renders, block for block,
/// the left plane, the left state, the reports and (after the disengage) the right plane and
/// right state of a never-collapsed bank on `L == R` input; the collapsed body never touches the
/// right plane; and `channels_agree` is false after one dual block of different L and R words and
/// true after the copy.
#[test]
fn a_collapsed_then_desymmetrized_bank_is_a_never_collapsed_bank() {
    let signals: Vec<Vec<f32>> = (0..NATIVE_LANES)
        .map(|lane| noise(0x973 + lane as u64, FRAMES * BLOCKS, 1.0))
        .collect();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let mut mixed = prepare_bank_native(&values(), link).expect("bank");
        let mut never = prepare_bank_native(&values(), link).expect("bank");
        assert!(mixed.processor.supports_mono_collapse());
        for step in 0..BLOCKS {
            let collapsed = step < DISENGAGE;
            if step == DISENGAGE {
                mixed.processor.desymmetrize_channels();
                assert!(mixed.processor.channels_agree(), "{link:?}: after the copy");
            }
            let context = format!("{link:?} block {step}");
            let mut never_left = block(step, &signals);
            let mut never_right = never_left.clone();
            let mut mixed_left = never_left.clone();
            let stale = vec![f32::from_bits(0x7F7F_FFFF); FRAMES * NATIVE_LANES];
            let mut mixed_right = if collapsed {
                stale.clone()
            } else {
                never_left.clone()
            };
            let never_report = render(&mut never, &mut never_left, &mut never_right, step, false);
            let mixed_report = render(
                &mut mixed,
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
            for track in 0..NATIVE_LANES as u32 {
                let (_, mixed_l, mixed_r) = snapshot_bank(&mixed, track);
                let (_, never_l, never_r) = snapshot_bank(&never, track);
                assert_eq!(mixed_l, never_l, "{context} track {track}: left state");
                if !collapsed {
                    assert_eq!(mixed_r, never_r, "{context} track {track}: right state");
                }
            }
        }

        // Under DualMono, channels driven apart by the render disagree, here in the hold
        // countdown alone: both channels stay open with `gain_db` at exact `0.0`, the left
        // reloads its 480-sample hold every frame, and the right last reloads at frame 7 (a hold
        // longer than the block keeps it open). The copy makes them agree. A linked detector
        // reads one level on both channels, so their state stays bit-equal without the copy.
        let mut apart = prepare_bank_native(&values(), link).expect("bank");
        let mut left = vec![0.5_f32; FRAMES * NATIVE_LANES];
        let mut right: Vec<f32> = (0..FRAMES * NATIVE_LANES)
            .map(|word| if word / NATIVE_LANES < 8 { 0.5 } else { 0.0 })
            .collect();
        let offsets = [0_u32; NATIVE_LANES + 1];
        apart.processor.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                FRAMES as u32,
                native_width(),
                0,
                &[],
                &offsets,
                128,
            )
            .expect("bank block"),
        );
        assert_eq!(
            apart.processor.channels_agree(),
            link != LinkMode::DualMono,
            "{link:?}: after different L and R words"
        );
        apart.processor.desymmetrize_channels();
        assert!(apart.processor.channels_agree(), "{link:?}: re-equalized");
    }
}

/// The `DESIGNED` witness declines on exactly the lane whose right channel carries a different
/// kernel-read word (a threshold ramp, an attack rate, a hold count), accepts every other lane and
/// declines past the width; the scalar witness answers the same for a lone instance.
#[test]
fn the_witness_declines_on_exactly_the_asymmetric_lane() {
    let base = values()[0];
    // (parameter, right value): a ramp word, a rate word (the time alone differs), a hold count.
    for (parameter, right) in [(0, -41.0), (4, 2.0), (5, 11.0)] {
        for odd in 0..NATIVE_LANES {
            let mut sets = values();
            set_parameter(&mut sets[odd], parameter, base[parameter * 2].value, right);
            let bank = prepare_bank_native(&sets, LinkMode::DualMono).expect("bank");
            for lane in 0..NATIVE_LANES {
                assert_eq!(
                    bank.processor.lane_channel_symmetry(lane),
                    lane != odd,
                    "parameter {parameter}, asymmetric lane {odd}, lane {lane}"
                );
            }
            assert!(!bank.processor.lane_channel_symmetry(NATIVE_LANES));
        }
        let mut asymmetric = base;
        set_parameter(&mut asymmetric, parameter, base[parameter * 2].value, right);
        assert!(!prepare(request(&asymmetric)).processor.channel_symmetry());
    }
    assert!(prepare(request(&base)).processor.channel_symmetry());
}

/// Issue #1894: a collapsed bank whose left channel falls silent is at rest, and a skipped block
/// is exact.
///
/// Dual signal first, so the collapse freezes a right channel that is **not** at rest; then
/// collapsed signal, then collapsed `+0.0` until the left channel rests. The bank must then answer
/// every lane at rest (a predicate that still ANDs the frozen right channel answers `0` and never
/// gets there), `advance_clocks` stands in for three zero blocks the reference renders, and after
/// the disengage the payloads and the next dual blocks are the never-collapsed bank's, bit for bit.
#[test]
fn a_collapsed_bank_that_falls_silent_is_at_rest_and_skips_exactly() {
    const SIGNAL: usize = 16;
    const SKIPPED: usize = 3;
    const REST_BOUND: usize = 20_000;
    let all = (1_u32 << NATIVE_LANES) - 1;
    let signals: Vec<Vec<f32>> = (0..NATIVE_LANES)
        .map(|lane| noise(0x1894 + lane as u64, FRAMES * BLOCKS, 1.0))
        .collect();
    let mut mixed = prepare_bank_native(&values(), LinkMode::DualMono).expect("bank");
    let mut never = prepare_bank_native(&values(), LinkMode::DualMono).expect("bank");
    let mut step = 0_usize;
    // The first signal blocks carry the file's automation; every later block carries none, so a
    // skip is admissible. `skip`: the reference renders the `+0.0` block, the collapsed bank
    // advances its clocks.
    let mut run = |mixed: &mut PreparedEffectBank,
                   never: &mut PreparedEffectBank,
                   signal: bool,
                   mono: bool,
                   skip: bool| {
        let mut never_left = if signal {
            block(step % BLOCKS, &signals)
        } else {
            vec![0.0; FRAMES * NATIVE_LANES]
        };
        let mut never_right = never_left.clone();
        let mut mixed_left = never_left.clone();
        let mut mixed_right = if mono {
            vec![f32::from_bits(0x7F7F_FFFF); FRAMES * NATIVE_LANES]
        } else {
            never_left.clone()
        };
        let context = format!("block {step}");
        if step < 2 * SIGNAL {
            render(never, &mut never_left, &mut never_right, step, false);
            render(mixed, &mut mixed_left, &mut mixed_right, step, mono);
        } else {
            let offsets = [0_u32; NATIVE_LANES + 1];
            let first = (step * FRAMES) as u64;
            for (bank, left, right, mono) in [
                (&mut *never, &mut never_left, &mut never_right, false),
                (&mut *mixed, &mut mixed_left, &mut mixed_right, mono),
            ] {
                if skip && mono {
                    bank.processor.advance_clocks(all, FRAMES as u32);
                    continue;
                }
                let block = EffectBankProcessBlock::new(
                    left,
                    right,
                    None,
                    FRAMES as u32,
                    native_width(),
                    first,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block");
                if mono {
                    bank.processor.process_bank_mono(block);
                } else {
                    bank.processor.process_bank(block);
                }
            }
        }
        step += 1;
        if skip {
            return;
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
        "the collapsed bank's left channel rests within the bound"
    );
    assert_eq!(
        never.processor.lanes_at_rest(),
        all,
        "the reference rests too"
    );
    for _ in 0..SKIPPED {
        run(&mut mixed, &mut never, false, true, true);
    }
    mixed.processor.desymmetrize_channels();
    for track in 0..NATIVE_LANES as u32 {
        assert_eq!(
            snapshot_bank(&mixed, track),
            snapshot_bank(&never, track),
            "track {track}: the payload after the skip and the disengage"
        );
    }
    // The flag the disengage clears: one dual block whose left plane is `+0.0` and whose right
    // carries signal moves the right channel off rest, so no lane may answer at rest. A flag left
    // set answers from the rested left channel and the right tail would be skipped.
    for bank in [&mut mixed, &mut never] {
        let mut left = vec![0.0; FRAMES * NATIVE_LANES];
        let mut right = vec![0.25; FRAMES * NATIVE_LANES];
        let offsets = [0_u32; NATIVE_LANES + 1];
        let block = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            native_width(),
            1 << 30,
            &[],
            &offsets,
            128,
        )
        .expect("bank block");
        bank.processor.process_bank(block);
    }
    assert_eq!(
        mixed.processor.lanes_at_rest() & all,
        0,
        "the right channel is live again"
    );
    for _ in 0..2 {
        run(&mut mixed, &mut never, true, false, false);
    }
}
