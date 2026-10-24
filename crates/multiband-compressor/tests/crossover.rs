#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E2: the `f32` two-stage crossover against the independent `f64` oracle, and its flatness.
//!
//! The `f64` mapping is pinned separately, before any production line moved, in
//! `tests/lr4_two_section_mapping_f64.rs`. This gate is the `f32` production form: the same
//! `lr4_step` the render path runs, at `WIDTH = 1`.
//!
//! Issue #1338 adds the live crossover's two claims here: a glide keeps the band sum the first
//! stage's all-pass at every sample and adds no click (gate 2), and a restore refuses crossover
//! words the engine would not hold (gate 4). The bank half of the glide (gate 3) is the randomized
//! differential's reach (`tests/randomized.rs`): its generator draws crossover spans on any lane.

mod support;

use effect_contract::{
    LinkMode, NativeEffectFactory, ParameterChannel, PreparedNativeEffect, StatePayloadSizes,
};
use effect_runtime::ramp::LinearRamp;
use lane::Lane;
use lane::kernels::{SvfState, svf_step};
use multiband_compressor::{
    Lr4State, MultibandCompressorFactory, design_lr4_words, lr4_coefficients, lr4_step,
};
use support::{point, process, request_with, restore, snapshot, values};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// The design guards its domain and its own half-power self-check.
#[test]
fn the_design_guards_its_domain() {
    assert!(lr4_coefficients::<f32>(0, 1_000.0).is_none());
    for crossover in [79.0f32, 8_001.0, f32::NAN, f32::INFINITY, -0.0] {
        assert!(
            lr4_coefficients::<f32>(48_000, crossover).is_none(),
            "{crossover}"
        );
    }
    for rate in RATES {
        for crossover in [80.0f32, 1_000.0, 8_000.0] {
            let coefficients = lr4_coefficients::<f32>(rate, crossover).expect("design");
            // `nk2` is exactly `-2 * sqrt(2)`, rounded once, on every design.
            assert_eq!(
                coefficients.nk2.to_bits(),
                (-2.0 * core::f32::consts::SQRT_2).to_bits()
            );
            assert!(coefficients.a2 > 0.0 && coefficients.a3 > 0.0);
        }
    }
}

/// A subnormal-seeded state reaches exactly zero, and stays there, at every width (D7).
#[test]
fn the_recursive_words_are_flushed() {
    fn check<L: Lane>() {
        let coefficients = lr4_coefficients::<L>(48_000, 1_000.0).expect("design");
        let mut state = Lr4State::<L> {
            a: lane::kernels::SvfState {
                ic1: L::splat(1.0e-40),
                ic2: L::splat(-1.0e-40),
            },
            b: lane::kernels::SvfState {
                ic1: L::splat(1.0e-40),
                ic2: L::splat(-1.0e-40),
            },
        };
        let mut run = L::zero();
        for _ in 0..4 {
            let rest = lane::silence_step(L::zero(), &mut run, L::splat(4_096.0));
            let _ = lr4_step(L::zero(), rest, &coefficients, &mut state);
        }
        let mut words = [0u32; 8];
        for value in [state.a.ic1, state.a.ic2, state.b.ic1, state.b.ic2] {
            value.store_bits(&mut words[..L::WIDTH]);
            for word in &words[..L::WIDTH] {
                assert_eq!(*word, 0, "a flushed state word must be exactly +0.0");
            }
        }
    }
    lane::each_lane!(|L| check::<L>());
}

/// Sample rate of the live-crossover gates.
const RATE: u32 = 48_000;

/// Block length of the live-crossover gates: shorter than the 64-sample glide, so every glide is
/// cut by a block boundary.
const BLOCK: usize = 40;

/// The block whose first sample carries the crossover span.
const SPAN_BLOCK: usize = 50;

/// Blocks rendered: the warm-up, the glide and well past its end.
const BLOCKS: usize = 60;

/// A 1 kHz sine at 0.5, phase-offset so no frame is exactly zero.
fn sine(sample: usize) -> f32 {
    (0.5 * math::sin(2.0 * core::f64::consts::PI * 1_000.0 * sample as f64 / f64::from(RATE) + 0.3))
        as f32
}

/// A scalar instance at `crossover` with both bands at unity: ratio 1 and makeup 0.
fn unity(crossover: f32) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let mut initial = values();
    for lane in 0..2 {
        initial[lane].value = crossover;
        initial[4 + lane].value = 1.0;
        initial[14 + lane].value = 1.0;
    }
    let mut request = request_with(&initial, LinkMode::DualMono, BLOCK as u32, false);
    request.sample_rate = RATE;
    let prepared = MultibandCompressorFactory
        .prepare(request)
        .expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

/// Renders the sine on both channels, in [`BLOCK`]-frame blocks, with a crossover span to `to` on
/// both channels at the first sample of [`SPAN_BLOCK`] (none when `to` is `None`). Returns the
/// left channel.
fn render(from: f32, to: Option<f32>) -> Vec<f32> {
    let (mut effect, _) = unity(from);
    let mut output = Vec::with_capacity(BLOCKS * BLOCK);
    for block in 0..BLOCKS {
        let first = block * BLOCK;
        let mut left = (first..first + BLOCK).map(sine).collect::<Vec<_>>();
        let mut right = left.clone();
        let spans = match to {
            Some(value) if block == SPAN_BLOCK => vec![
                point(0, ParameterChannel::Left, first as u64, value),
                point(0, ParameterChannel::Right, first as u64, value),
            ],
            _ => Vec::new(),
        };
        process(
            effect.as_mut(),
            &mut left,
            &mut right,
            first as u64,
            &spans,
            BLOCK as u32,
        );
        assert_eq!(left, right, "both channels carry the same glide");
        output.extend_from_slice(&left);
    }
    output
}

/// The oracle: the first stage alone, `ap = x - 2k v1`, driven by the same word ramp the effect
/// walks (`LinearRamp::set_target(word, 64)` at the span, one `next_value` per frame), and the
/// second stage beside it for the band magnitudes. Returns `(ap, low, high)` per frame.
fn oracle(from: f32, to: f32) -> Vec<(f32, f32, f32)> {
    let mut words = design_lr4_words(RATE, from).map(LinearRamp::fixed);
    let target = design_lr4_words(RATE, to);
    let nk2 = -2.0 * core::f32::consts::SQRT_2;
    let mut first = SvfState::<f32>::default();
    let mut second = SvfState::<f32>::default();
    (0..BLOCKS * BLOCK)
        .map(|sample| {
            if sample == SPAN_BLOCK * BLOCK {
                for (ramp, word) in words.iter_mut().zip(target) {
                    ramp.set_target(word, 64);
                }
            }
            let [c1, a2, a3] = words.each_mut().map(LinearRamp::next_value);
            let x = sine(sample);
            let (v1, lp1) = svf_step(x, -c1, a2, a3, 0.0, &mut first);
            let ap = nk2.fma(v1, x);
            let (_, low) = svf_step(lp1, -c1, a2, a3, 0.0, &mut second);
            (ap, low, ap - low)
        })
        .collect()
}

/// The largest `|y[n] - y[n-1]|` over `window`.
fn largest_step(output: &[f32], window: core::ops::Range<usize>) -> f32 {
    window
        .map(|index| (output[index] - output[index - 1]).abs())
        .fold(0.0, f32::max)
}

/// Gate 2: a crossover glide keeps the band sum the first stage's all-pass at every sample, and is
/// no rougher than the static renders at its two ends.
///
/// Test value: words assigned at once instead of ramped (a click), or a second stage fed other
/// words than the first, turns it red; no other test moves the crossover while it plays.
#[test]
fn a_crossover_glide_stays_the_all_pass_and_adds_no_click() {
    let start = SPAN_BLOCK * BLOCK;
    let window = start..start + 64 + 128;
    for (from, to) in [(200.0f32, 4_000.0f32), (4_000.0, 200.0), (80.0, 8_000.0)] {
        let glide = render(from, Some(to));
        let reference = oracle(from, to);
        // Unity gain, asserted: before the span the output is the two bands' sum bit for bit.
        for (index, &(_, low, high)) in reference[..start].iter().enumerate() {
            assert_eq!(
                glide[index].to_bits(),
                (low + high).to_bits(),
                "{from} -> {to}: sample {index} is not at unity gain"
            );
        }
        // (a) The band sum is the all-pass of the first stage on the same word ramp.
        let mut worst = 0.0f64;
        for (index, &(ap, low, high)) in reference.iter().enumerate() {
            let scale = low.abs().max(high.abs()).max(ap.abs());
            let error = (glide[index] - ap).abs();
            assert!(
                error <= scale * f32::from_bits(0x3480_0000),
                "{from} -> {to}: sample {index}: {} against the all-pass {ap}",
                glide[index]
            );
            if scale > 0.0 {
                worst = worst.max(f64::from(error) / f64::from(scale));
            }
        }
        // (b) No click: the glide's largest sample step against both static renders'.
        let still = largest_step(&render(from, None), window.clone())
            .max(largest_step(&render(to, None), window.clone()));
        let moving = largest_step(&glide, window.clone());
        assert!(
            moving <= 2.0 * still,
            "{from} -> {to}: largest step {moving} against {still} static"
        );
        eprintln!(
            "{from} -> {to}: worst relative error {worst:e} (bound 2^-22), largest step {moving} \
             against {still} static, ratio {}",
            moving / still
        );
    }
}

/// Word offset of the first crossover word ramp in a channel's payload, and the ramp's width.
const CROSSOVER_WORD: usize = 64;
const RAMP_WORDS: usize = 4;

fn word(bytes: &[u8], index: usize) -> [u8; 4] {
    bytes[index * 4..index * 4 + 4].try_into().expect("word")
}

fn set_word(bytes: &mut [u8], index: usize, value: [u8; 4]) {
    bytes[index * 4..index * 4 + 4].copy_from_slice(&value);
}

/// Gate 4: a restore refuses crossover words the engine would not hold, and a refused restore
/// leaves the instance as it was.
///
/// The honest mid-glide snapshot restores (the randomized differential's restore leg proves that it
/// renders on bit for bit); a target triple one ulp off the design of the payload's crossover, and
/// a current triple outside `RAMP_PATH_NORM_TOLERANCE`, are refused.
///
/// Test value: a restore that accepts forged unstable or undesigned words, or that omits the word
/// ramps, turns it red.
#[test]
fn a_restore_refuses_crossover_words_the_engine_would_not_hold() {
    let block = |first: usize| {
        let left = (first..first + BLOCK).map(sine).collect::<Vec<_>>();
        (left.clone(), left)
    };
    let (mut effect, sizes) = unity(200.0);
    let (mut left, mut right) = block(0);
    let spans = [
        point(0, ParameterChannel::Left, 0, 4_000.0),
        point(0, ParameterChannel::Right, 0, 4_000.0),
    ];
    process(
        effect.as_mut(),
        &mut left,
        &mut right,
        0,
        &spans,
        BLOCK as u32,
    );
    // 40 samples into the 64-sample glide.
    let honest = snapshot(effect.as_ref(), sizes);
    let c1 = CROSSOVER_WORD;
    assert_ne!(
        word(&honest.1, c1 + 3),
        0u32.to_le_bytes(),
        "the snapshot is mid-glide"
    );
    restore(effect.as_mut(), 1, &honest, sizes).expect("the honest mid-glide snapshot restores");

    let mut undesigned = honest.clone();
    let target = f32::from_le_bytes(word(&undesigned.1, c1 + 1));
    set_word(&mut undesigned.1, c1 + 1, target.next_up().to_le_bytes());

    let mut unstable = honest.clone();
    // `M = [[1 - 2c1, -2a2], [2a2, 1 - 2a3]]` near `[[1, -1], [1, 1]]`: norm about sqrt(2).
    for (index, value) in [1.0e-3f32, 0.5, 1.0e-3].into_iter().enumerate() {
        set_word(
            &mut unstable.1,
            c1 + index * RAMP_WORDS,
            value.to_le_bytes(),
        );
    }

    let (expected, _) = unity(200.0);
    let mut expected = expected;
    restore(expected.as_mut(), 1, &honest, sizes).expect("restore");
    for (name, forged) in [
        ("undesigned target", &undesigned),
        ("unstable current", &unstable),
    ] {
        let refused = restore(effect.as_mut(), 1, forged, sizes);
        assert_eq!(
            refused.map_err(|error| error.code),
            Err("effect.state.coefficient"),
            "{name}"
        );
        assert_eq!(
            snapshot(effect.as_ref(), sizes),
            honest,
            "{name}: state changed"
        );
    }
    let (mut ours, mut ours_right) = block(BLOCK);
    let (mut theirs, mut theirs_right) = block(BLOCK);
    process(
        effect.as_mut(),
        &mut ours,
        &mut ours_right,
        BLOCK as u64,
        &[],
        BLOCK as u32,
    );
    process(
        expected.as_mut(),
        &mut theirs,
        &mut theirs_right,
        BLOCK as u64,
        &[],
        BLOCK as u32,
    );
    assert_eq!(ours, theirs, "a refused restore moved the render");
}
