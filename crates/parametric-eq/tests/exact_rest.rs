//! Issue #1328 gate 2: an EQ section that used to stick at a non-zero fixed point reaches rest.
//!
//! A low shelf at 10 Hz, +24 dB, slope 0.1, at 96 kHz, driven by a unit impulse, used to settle at
//! `ic1 = +0.0`, `ic2 = 6.01e-20` under the per-word flush: `2 * d2` is below half an ulp of `ic2`,
//! so the word never moves again and the output stays near -361 dBFS forever, which also keeps the
//! EQ's silent fast path from ever engaging. The joint SVF flush (`lane::flush_pair`) zeroes the
//! pair once both words are below `REST_EPS` and the EQ's input has been silent for `N_SILENCE`
//! frames, so the band's integrators reach `+0.0` and the output reaches exactly `+0.0`.
//!
//! Amendment A8 and A9 gates: the same rule must leave a live input alone, however small, and a
//! sparse one too (silence is a time property, A9). Four boosting low shelves fed a square, or a
//! sparse train, far below any rest threshold must apply all four boosts, exactly as they do to
//! the same input at an ordinary level; and a section fed an exact zero by the section before it
//! must not end its decay while the EQ's own input is live.

mod support;

use std::f32::consts::FRAC_1_SQRT_2;

use effect_contract::{
    EffectProcessBlock, InitialParameterValue, NativeEffectFactory, ParameterChannel,
    PreparedNativeEffect,
};
use parametric_eq::{EqBandKind, ParametricEqFactory};
use support::{band_word, request_at_rate, set_initial, single_section_values, snapshot, values};

const RATE: u32 = 96_000;
const FRAMES: usize = 128;
/// The bound the issue states: rest within 9,600,000 samples (100 s) of the impulse.
const REST_BOUND_SAMPLES: usize = 9_600_000;
/// Blocks rendered after rest that must stay at rest.
const HELD_BLOCKS: usize = 4;

fn at_rest(effect: &dyn PreparedNativeEffect, left: &[f32], right: &[f32]) -> bool {
    let (_, left_state, right_state) = snapshot(effect);
    [&left_state[..], &right_state[..]]
        .iter()
        .all(|payload| band_word(payload, 0, 0) == 0 && band_word(payload, 0, 1) == 0)
        && left.iter().chain(right).all(|sample| sample.to_bits() == 0)
}

#[test]
fn the_low_shelf_fixed_point_reaches_exact_rest() {
    let configured = single_section_values(EqBandKind::LowShelf, 10.0, 24.0, FRAC_1_SQRT_2, 0.1);
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, RATE))
        .expect("the shelf prepares");

    let mut left = [0.0_f32; FRAMES];
    let mut right = [0.0_f32; FRAMES];
    left[0] = 1.0;
    right[0] = 1.0;
    let mut rest_at = None;
    let mut block = 0_usize;
    while block * FRAMES < REST_BOUND_SAMPLES {
        let first_sample = (block * FRAMES) as u64;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first_sample,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        if block > 0 && at_rest(effect.as_ref(), &left, &right) {
            rest_at = Some(block);
            break;
        }
        left.fill(0.0);
        right.fill(0.0);
        block += 1;
    }
    let rest_at = rest_at.unwrap_or_else(|| {
        let (_, left_state, _) = snapshot(effect.as_ref());
        panic!(
            "no exact rest within {REST_BOUND_SAMPLES} samples: band 0 left ic1 {:e}, ic2 {:e}",
            f32::from_bits(band_word(&left_state, 0, 0)),
            f32::from_bits(band_word(&left_state, 0, 1)),
        )
    });
    for held in 1..=HELD_BLOCKS {
        left.fill(0.0);
        right.fill(0.0);
        let first_sample = ((rest_at + held) * FRAMES) as u64;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first_sample,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        assert!(
            at_rest(effect.as_ref(), &left, &right),
            "rest reached at block {rest_at} must hold, broke {held} blocks later"
        );
    }
}

/// Parameters per EQ band in the descriptor: enabled, kind, frequency, gain, Q, slope.
const BAND_PARAMETERS: usize = 6;
/// The four general bands, all used by the chain gate.
const BANDS: usize = 4;
/// Half a period of the chain gate's square: 12,488 samples, 3.84 Hz at 96 kHz (the attempt-3
/// verdict's measurement input).
const SQUARE_HALF_PERIOD: usize = 12_488;
/// Samples the chain gate renders: four half periods of the square, so every shelf has settled.
const CHAIN_SAMPLES: usize = 4 * SQUARE_HALF_PERIOD;
/// The ordinary input level the tiny runs are scaled from.
const REFERENCE_LEVEL: f32 = 0.03;

/// Every general band a low shelf at `frequency`, +24 dB, slope 1, on both channels.
fn four_boosting_shelves(frequency: f32) -> Vec<InitialParameterValue> {
    let mut configured = values();
    for band in 0..BANDS {
        let base = band * BAND_PARAMETERS;
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            set_initial(&mut configured, base, channel, 1.0);
            set_initial(
                &mut configured,
                base + 1,
                channel,
                EqBandKind::LowShelf as u32 as f32,
            );
            set_initial(&mut configured, base + 2, channel, frequency);
            set_initial(&mut configured, base + 3, channel, 24.0);
            set_initial(&mut configured, base + 4, channel, FRAC_1_SQRT_2);
            set_initial(&mut configured, base + 5, channel, 1.0);
        }
    }
    configured
}

/// `2^exponent`, exactly, for a normal-range exponent: built from its bits.
fn power_of_two(exponent: i32) -> f32 {
    f32::from_bits(u32::try_from(127 + exponent).expect("a normal exponent") << 23)
}

/// Renders the square at `level` through a fresh EQ and returns the left output.
fn render_square(configured: &[InitialParameterValue], level: f32) -> Vec<f32> {
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(configured, false, RATE))
        .expect("the shelves prepare");
    let mut output = Vec::with_capacity(CHAIN_SAMPLES);
    for first in (0..CHAIN_SAMPLES).step_by(FRAMES) {
        let mut left = [0.0_f32; FRAMES];
        for (offset, sample) in left.iter_mut().enumerate() {
            let polarity = if ((first + offset) / SQUARE_HALF_PERIOD).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            *sample = polarity * level;
        }
        let mut right = left;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                first as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        output.extend_from_slice(&left);
    }
    output
}

/// Amendment A8: a tiny non-zero input through four +24 dB low shelves gets its full boost.
///
/// The joint flush ends a decay only on a sample whose input is exactly zero. A rule that ignored
/// the input (attempt 3's) also zeroed a section's state whenever both words sat below `REST_EPS`
/// while a tiny signal drove it: such a section passed only its direct term, so each following
/// section saw the unboosted signal and stayed at rest too, and four +24 dB shelves at 10 Hz lost
/// about 96 dB of response below about `3e-11` (the attempt-3 verdict measured `1.5e-6` of change at
/// the EQ's output, -116 dBFS).
///
/// Every input here is the same 3.84 Hz square at `REFERENCE_LEVEL * 2^-k`, at 10 Hz and 100 Hz:
/// `k = 30` and `34` sit just under one section's old rest limit at those frequencies (`2.8e-11`
/// and `1.7e-12`), `k = 36` (`4.4e-13`) under both. Scaling an input by a power of two scales every
/// product and sum of the recurrence exactly while no word reaches the subnormal range or
/// `FLUSH_EPS`, so the tiny run, scaled back up by `2^k`, must be the ordinary run. The comparison
/// allows `1e-4` of the ordinary run's peak, because a state word that crosses zero can land inside
/// the per-word law's `FLUSH_EPS` band (measured up to `1.3e-6` of the peak at these levels; the
/// per-word law's own limit, `3e-17` at 10 Hz, is far below them). A lost boost misses by nearly
/// the whole peak.
#[test]
fn a_tiny_input_through_four_boosting_shelves_gets_every_boost() {
    for frequency in [10.0_f32, 100.0] {
        let configured = four_boosting_shelves(frequency);
        let ordinary = render_square(&configured, REFERENCE_LEVEL);
        let peak = ordinary.iter().fold(0.0_f32, |peak, y| peak.max(y.abs()));
        // About +96 dB of boost at 3.84 Hz, so the ordinary run peaks far above its input.
        assert!(
            peak > 1_000.0 * REFERENCE_LEVEL,
            "{frequency} Hz: four +24 dB shelves must boost the ordinary square (peak {peak:e})"
        );
        for k in [30_i32, 34, 36] {
            let level = REFERENCE_LEVEL * power_of_two(-k);
            let tiny = render_square(&configured, level);
            let scale = power_of_two(k);
            let (worst, at) = tiny
                .iter()
                .zip(&ordinary)
                .enumerate()
                .map(|(index, (small, big))| ((small * scale - big).abs(), index))
                .fold(
                    (0.0_f32, 0),
                    |best, next| if next.0 > best.0 { next } else { best },
                );
            assert!(
                worst <= 1.0e-4 * peak,
                "{frequency} Hz, input {level:e}: the tiny run, scaled by 2^{k}, misses the \
                 ordinary run by {worst:e} at sample {at} (ordinary peak {peak:e}); \
                 the shelves must apply every boost to a tiny non-zero input"
            );
        }
    }
}

/// The four launch rates (`LAUNCH_SAMPLE_RATES`).
const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// Renders `input` through a fresh EQ prepared at `rate` and returns the left output.
fn render_at(configured: &[InitialParameterValue], rate: u32, input: &[f32]) -> Vec<f32> {
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(configured, false, rate))
        .expect("the EQ prepares");
    let mut output = Vec::with_capacity(input.len());
    for (index, chunk) in input.chunks(FRAMES).enumerate() {
        let mut left = [0.0_f32; FRAMES];
        left[..chunk.len()].copy_from_slice(chunk);
        let mut right = left;
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                (index * FRAMES) as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        output.extend_from_slice(&left[..chunk.len()]);
    }
    output
}

/// The largest `|tiny * 2^k - ordinary|`, and where.
fn scaled_miss(tiny: &[f32], ordinary: &[f32], k: i32) -> (f32, usize) {
    let scale = power_of_two(k);
    tiny.iter()
        .zip(ordinary)
        .enumerate()
        .map(|(index, (small, big))| ((small * scale - big).abs(), index))
        .fold(
            (0.0_f32, 0),
            |best, next| if next.0 > best.0 { next } else { best },
        )
}

/// A sparse train: every `period`-th sample carries the 3.84 Hz square at `level`, the rest are
/// exactly `+0.0` (at 96 kHz; the same sample pattern at every rate).
///
/// The train starts after `lead` samples of exact silence: long enough that the joint flush has
/// armed and the EQ's rest planes hold armed thresholds when the train begins, so a block of the
/// train that read the last armed plane instead of the unarmed one is caught.
fn sparse_square(period: usize, level: f32, samples: usize, lead: usize) -> Vec<f32> {
    (0..lead)
        .map(|_| 0.0)
        .chain((0..samples).map(|index| {
            if index % period != 0 {
                0.0
            } else if (index / SQUARE_HALF_PERIOD).is_multiple_of(2) {
                level
            } else {
                -level
            }
        }))
        .collect()
}

/// Amendment A9: a sparse input -- non-zero samples separated by exact zeros, each far below one
/// section's old rest limit -- through four +24 dB low shelves gets every boost, at every launch
/// rate.
///
/// `[a, 0, a, 0, ...]` and `[a, 0, 0, 0, ...]`, with `a` the 3.84 Hz square at
/// `REFERENCE_LEVEL * 2^-k`, at 10 Hz and 100 Hz: `k = 36` puts `a` (`4.4e-13`) below one section's
/// `L* = REST_EPS / (2 max(a2, a3))` at both frequencies and all four rates (the smallest, 100 Hz at
/// 44.1 kHz, is about `1.4e-12`). Its zeros are never `N_SILENCE` frames long, so the silence
/// counter never arms the joint rule and the tiny run, scaled back by `2^k`, is the ordinary run
/// up to the per-word law's own `FLUSH_EPS` crossings (the tolerance, `1e-6` of the peak;
/// measured exactly `0` at every rate, frequency and period). Amendment A8's rule (armed by any zero section input) zeroes the first shelf's state at
/// every zero and loses its boost (`[a, 0, a, 0]`) or all four (`[a, 0, 0, 0]`); attempt 3's rule
/// loses all four. Both miss by about the whole peak.
#[test]
fn a_sparse_input_through_four_boosting_shelves_gets_every_boost_at_every_rate() {
    const K: i32 = 36;
    for rate in LAUNCH_RATES {
        // Silence until the joint flush is armed, and two blocks more.
        let lead = lane::silence_frames(rate) as usize + 2 * FRAMES;
        for frequency in [10.0_f32, 100.0] {
            let configured = four_boosting_shelves(frequency);
            for period in [2_usize, 4] {
                let ordinary = render_at(
                    &configured,
                    rate,
                    &sparse_square(period, REFERENCE_LEVEL, CHAIN_SAMPLES, lead),
                );
                let peak = ordinary.iter().fold(0.0_f32, |peak, y| peak.max(y.abs()));
                assert!(
                    peak > 1_000.0 * REFERENCE_LEVEL / period as f32,
                    "{rate} Hz, {frequency} Hz, period {period}: the shelves must boost the \
                     ordinary train (peak {peak:e})"
                );
                let level = REFERENCE_LEVEL * power_of_two(-K);
                let tiny = render_at(
                    &configured,
                    rate,
                    &sparse_square(period, level, CHAIN_SAMPLES, lead),
                );
                let (worst, at) = scaled_miss(&tiny, &ordinary, K);
                assert!(
                    worst <= SPARSE_TOLERANCE * peak,
                    "{rate} Hz, {frequency} Hz shelves, period {period}, input {level:e}: the tiny \
                     run, scaled by 2^{K}, misses the ordinary run by {worst:e} at sample {at} \
                     (ordinary peak {peak:e}); a sparse input must get every boost"
                );
            }
        }
    }
}

/// The sparse gate's tolerance, as a fraction of the ordinary run's peak.
const SPARSE_TOLERANCE: f32 = 1.0e-6;

/// Amendment A9: while the EQ's input is live, the joint flush moves nothing, even where a section
/// inside the cascade is fed exact zeros.
///
/// The EQ's own high-pass at 40 Hz blocks a DC input and, once its integrator has converged on the
/// DC word, outputs exactly `+0.0`, so the four +24 dB shelves after it see silence while the EQ's
/// input never is (and a slow square makes the same happen on every flat half period). A rule armed
/// by a section's own input (amendment A8's) then zeroes the shelves' small decaying states while
/// the effect is live; the A9 rule is armed by the EQ's input only, which here is never zero. The
/// tiny run (`2^-K` of the ordinary one) scaled back by `2^K` must therefore be the ordinary run up
/// to `FLUSH_EPS` crossings, on every sample.
#[test]
fn an_exact_zero_inside_the_cascade_moves_nothing_while_the_input_is_live() {
    const K: i32 = 20;
    const RATE_HZ: u32 = 96_000;
    const SAMPLES: usize = 192_000;
    let mut configured = four_boosting_shelves(100.0);
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut configured, HPF_ENABLED, channel, 1.0);
        set_initial(&mut configured, HPF_ENABLED + 1, channel, 40.0);
        set_initial(&mut configured, HPF_ENABLED + 2, channel, 0.7);
    }
    for (name, input) in [
        ("DC", vec![1.0_f32; SAMPLES]),
        (
            "square",
            (0..SAMPLES)
                .map(|index| {
                    if (index / SQUARE_HALF_PERIOD).is_multiple_of(2) {
                        1.0
                    } else {
                        -1.0
                    }
                })
                .collect(),
        ),
    ] {
        let ordinary_level = 1.0e-6 * power_of_two(K);
        let ordinary_input: Vec<f32> = input.iter().map(|x| x * ordinary_level).collect();
        let tiny_input: Vec<f32> = input.iter().map(|x| x * 1.0e-6).collect();
        let ordinary = render_at(&configured, RATE_HZ, &ordinary_input);
        let tiny = render_at(&configured, RATE_HZ, &tiny_input);
        let zeros = tiny.iter().filter(|y| y.to_bits() == 0).count();
        let peak = ordinary.iter().fold(0.0_f32, |peak, y| peak.max(y.abs()));
        let (worst, at) = scaled_miss(&tiny, &ordinary, K);
        assert!(
            worst <= CANCELLATION_TOLERANCE * peak,
            "{name} at 1e-6: the tiny run, scaled by 2^{K}, misses the ordinary run by {worst:e} \
             at sample {at} (peak {peak:e}, {zeros} exact-zero outputs); a section fed an exact \
             zero inside a live EQ must not end its decay"
        );
    }
}

/// The cancellation gate's tolerance, as a fraction of the ordinary run's peak.
const CANCELLATION_TOLERANCE: f32 = 1.0e-9;

/// The HPF's enable parameter index; its frequency and Q follow it.
const HPF_ENABLED: usize = 24;

/// Issue #1328 amendment A9: the EQ's silent fast path skips a block only once the joint flush is
/// armed on every lane, so a state the joint rule must clear is still cleared.
///
/// Every band disabled, and band one's integrators restored to `(1e-15, -1e-15)`: a pair inside
/// the joint band that an identity section holds unchanged on a zero input, with an output of
/// exactly `+0.0`. That is a fixed point the fast path's induction would accept, but the joint
/// flush zeroes it once the input has been silent for `N_SILENCE` (8,192 frames at 96 kHz); a
/// flag earned before the counter arms would skip every later block and keep the pair forever.
#[test]
fn a_fixed_point_inside_the_joint_band_is_cleared_once_the_flush_arms() {
    use effect_contract::StatePayloadInput;
    use support::{SILENCE_WORD, WORDS_PER_BAND};
    const N_SILENCE_96K: usize = 8_192;
    let configured = values();
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, RATE))
        .expect("the EQ prepares");
    let mut payload = snapshot(effect.as_ref());
    for section in [&mut payload.1, &mut payload.2] {
        let at = WORDS_PER_BAND * 4;
        section[at..at + 4].copy_from_slice(&1.0e-15_f32.to_bits().to_le_bytes());
        section[at + 4..at + 8].copy_from_slice(&(-1.0e-15_f32).to_bits().to_le_bytes());
        assert_eq!(support::word(section, SILENCE_WORD), 0, "a fresh counter");
    }
    let sizes = effect.metadata().state_sizes;
    effect
        .restore_state_payload(
            1,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("input"),
        )
        .expect("the payload restores");
    let blocks = N_SILENCE_96K / FRAMES + 4;
    for block in 0..blocks {
        let mut left = [0.0_f32; FRAMES];
        let mut right = [0.0_f32; FRAMES];
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                (block * FRAMES) as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        assert!(
            left.iter().chain(&right).all(|y| y.to_bits() == 0),
            "block {block}: an identity cascade on silence outputs +0.0"
        );
        let (_, left_state, _) = snapshot(effect.as_ref());
        let pair = [band_word(&left_state, 0, 0), band_word(&left_state, 0, 1)];
        if (block + 1) * FRAMES < N_SILENCE_96K {
            assert_ne!(
                pair,
                [0, 0],
                "block {block}: the pair is held before the flush arms"
            );
        } else {
            assert_eq!(
                pair,
                [0, 0],
                "block {block}: the armed joint flush clears the pair"
            );
        }
    }
    // The blocks the fast path skips still count: the counter is the input's, rendered or not.
    let (_, left_state, right_state) = snapshot(effect.as_ref());
    for state in [&left_state, &right_state] {
        assert_eq!(
            support::word(state, SILENCE_WORD),
            ((blocks * FRAMES) as f32).to_bits(),
            "the silence counter after {blocks} silent blocks"
        );
    }
}

/// Issue #1328 (the root's cost ruling, (a) and (b)): a block runs the cascades' armed form when
/// some lane of **either** channel can arm within it and holds a non-zero integrator word in
/// **any** section, and the unarmed form otherwise. One channel's one section is put in the joint
/// band (`(1e-15, -1e-15)`, a pair the identity section holds on a zero input) with that channel's
/// counter one frame short of its window, while the other channel's input is live and its counter
/// fresh: the first block must clear exactly that pair, for each channel and each of the six
/// sections. Red if the block's form is decided from one channel, or from a subset of the sections'
/// words, or never armed.
#[test]
fn the_joint_flush_arms_for_whichever_channel_and_section_holds_the_band() {
    use effect_contract::StatePayloadInput;
    use support::{SILENCE_WORD, WORDS_PER_BAND};
    const N_SILENCE_96K: f32 = 8_192.0;
    let configured = values();
    for silent in 0..2 {
        for section in 0..6 {
            let mut effect = ParametricEqFactory
                .prepare(request_at_rate(&configured, false, RATE))
                .expect("the EQ prepares");
            let mut payload = snapshot(effect.as_ref());
            let target = if silent == 0 {
                &mut payload.1
            } else {
                &mut payload.2
            };
            // Byte offset of the section's first integrator word.
            let at = WORDS_PER_BAND * section * 4;
            target[at..at + 4].copy_from_slice(&1.0e-15_f32.to_bits().to_le_bytes());
            target[at + 4..at + 8].copy_from_slice(&(-1.0e-15_f32).to_bits().to_le_bytes());
            target[SILENCE_WORD * 4..SILENCE_WORD * 4 + 4]
                .copy_from_slice(&(N_SILENCE_96K - 1.0).to_bits().to_le_bytes());
            let sizes = effect.metadata().state_sizes;
            effect
                .restore_state_payload(
                    1,
                    StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                        .expect("input"),
                )
                .expect("the payload restores");
            let mut planes = [[0.25_f32; FRAMES]; 2];
            planes[silent] = [0.0; FRAMES];
            let [left, right] = &mut planes;
            effect.process(
                EffectProcessBlock::new(left, right, None, 0, &[], FRAMES as u32).expect("block"),
            );
            let (_, left_state, right_state) = snapshot(effect.as_ref());
            let state = if silent == 0 {
                &left_state
            } else {
                &right_state
            };
            assert_eq!(
                [
                    support::word(state, WORDS_PER_BAND * section),
                    support::word(state, WORDS_PER_BAND * section + 1),
                ],
                [0, 0],
                "channel {silent}, section {section}: the armed block clears the band pair"
            );
        }
    }
}

/// Issue #1328 (the root's cost ruling, (b)): a block longer than the silence window must arm a
/// lane that starts it at rest. The unarmed form is exact for a lane at rest only when the window
/// reaches back to the block's first frame; inside a longer block a lane can take a sample, rest's
/// opposite, and then a whole window of zeros. Prepared with an 8,192-frame quantum at 48 kHz
/// (window 4,096), a tiny impulse through a +24 dB 10 Hz low shelf, rendered as one block, must
/// give the bits the same input gives in 128-frame blocks, where the joint flush clears the tail
/// on frame 4,096. Also at the boundary: a 4,097-frame block, one frame longer than the window,
/// whose tail arms on its last frame, against the same 4,097 frames in 128-frame parts. Red if the
/// decision drops the block-length term or moves it by one frame (`frames > armed_after + 1`).
#[test]
fn a_block_longer_than_the_window_renders_as_its_quantum_sized_parts() {
    const QUANTUM: u32 = 8_192;
    const WINDOW_48K: usize = 4_096;
    let configured = single_section_values(EqBandKind::LowShelf, 10.0, 24.0, FRAC_1_SQRT_2, 1.0);
    let render = |block: usize, total: usize| {
        let mut request = request_at_rate(&configured, false, 48_000);
        request.quantum = QUANTUM;
        request.limits.maximum_scratch_bytes =
            u64::from(QUANTUM) * parametric_eq::REST_PLANE_BYTES_PER_FRAME;
        let mut effect = ParametricEqFactory
            .prepare(request)
            .expect("the EQ prepares");
        let mut left = vec![0.0_f32; total];
        left[0] = 1.0e-12;
        let mut right = left.clone();
        for (index, (left, right)) in left
            .chunks_mut(block)
            .zip(right.chunks_mut(block))
            .enumerate()
        {
            let frames = left.len() as u32;
            effect.process(
                EffectProcessBlock::new(left, right, None, (index * block) as u64, &[], frames)
                    .expect("block"),
            );
        }
        (left, snapshot(effect.as_ref()))
    };
    for total in [QUANTUM as usize, WINDOW_48K + 1] {
        let (whole, whole_state) = render(total, total);
        let (parts, parts_state) = render(FRAMES, total);
        assert_eq!(
            band_word(&parts_state.1, 0, 0) | band_word(&parts_state.1, 0, 1),
            0,
            "{total} frames: the tail is in the joint band when the window ends, and is cleared"
        );
        assert!(
            whole
                .iter()
                .zip(&parts)
                .all(|(a, b)| a.to_bits() == b.to_bits()),
            "one {total}-frame block renders the bits of its 128-frame parts"
        );
        assert!(
            whole_state == parts_state,
            "one {total}-frame block leaves the state of its 128-frame parts"
        );
    }
}
