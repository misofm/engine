//! Issue #2114 S3's gates for `miso.reverb`: the stated tail and exact-rest bound against the
//! real kernel. The derivation is `docs/derivations/2114-reverb-tail-and-rest.md`; the stated
//! values are its computation (`reverb::tail_bound_at`), checked here by soundness and
//! non-vacuity, not re-derived.
//!
//! Gate 1 renders about 110 million frames (about 10 s) and runs in release only. Gate 2 renders
//! about 0.6 million frames and runs in both profiles. Evidence lines print with `--nocapture`.
#![allow(missing_docs)]

mod common;

use common::{DECAY, DIFFUSION, PRE_DELAY, QUANTUM, SIZE};
use effect_contract::{
    EffectProcessBlock, EffectQuality, ParameterChannel, PreparedAutomationSpan,
    PreparedNativeEffect, RestBound, RestSamples, StatePayloadOutput, StatePayloadSizes,
    TailSamples,
};
use reverb::{REVERB_DESCRIPTOR, REVERB_PARAMETERS, ReverbFactory};

/// `10^(-144/20)`.
const EPS: f64 = 6.309_573_444_801_932e-8;
const DAMPING: usize = 3;
const LOW_CUT: usize = 4;

fn maximum(index: usize) -> f32 {
    REVERB_PARAMETERS[index].maximum.expect("bounded")
}

fn minimum(index: usize) -> f32 {
    REVERB_PARAMETERS[index].minimum.expect("bounded")
}

/// The payload sizes at `rate`: the prepared metadata's are the quality row's.
fn sizes(rate: u32) -> StatePayloadSizes {
    REVERB_DESCRIPTOR
        .qualities
        .iter()
        .find(|q| q.sample_rate == rate)
        .expect("launch rate")
        .maximum_state
}

/// Renders `left`/`right` in quantum blocks from sample `first`; `spans` apply at their sample,
/// which must be a block start. No block may fail its finiteness check.
fn render(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first: u64,
    spans: &[PreparedAutomationSpan],
) {
    let quantum = QUANTUM as usize;
    let mut offset = 0;
    while offset < left.len() {
        let end = (offset + quantum).min(left.len());
        let block: Vec<PreparedAutomationSpan> = spans
            .iter()
            .filter(|span| span.sample >= first + offset as u64 && span.sample < first + end as u64)
            .copied()
            .collect();
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                first + offset as u64,
                &block,
                QUANTUM,
            )
            .expect("block"),
        );
        assert_eq!(report.invalid_spans, 0, "every span is legal");
        assert_eq!(report.nonfinite_left_blocks, 0, "no block fails its check");
        offset = end;
    }
}

/// Deterministic uniform noise in `[-peak, peak]`, one stream per channel.
fn noise(frames: usize, peak: f32, seed: u64) -> (Vec<f32>, Vec<f32>) {
    let mut state = seed;
    let mut next = || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((state >> 40) as f32 / (1_u64 << 23) as f32 - 1.0) * peak
    };
    let left = (0..frames).map(|_| next()).collect();
    let right = (0..frames).map(|_| next()).collect();
    (left, right)
}

/// Renders zeros from sample `first` for `frames` frames in chunks, calling `each` per chunk with
/// the chunk's offset into the silence.
fn render_silence(
    effect: &mut dyn PreparedNativeEffect,
    first: u64,
    frames: u64,
    mut each: impl FnMut(u64, &[f32], &[f32]),
) {
    let chunk = 128 * QUANTUM as usize;
    let mut left = vec![0.0_f32; chunk];
    let mut right = vec![0.0_f32; chunk];
    let mut done = 0_u64;
    while done < frames {
        let n = (chunk as u64).min(frames - done) as usize;
        left[..n].fill(0.0);
        right[..n].fill(0.0);
        render(effect, &mut left[..n], &mut right[..n], first + done, &[]);
        each(done, &left[..n], &right[..n]);
        done += n as u64;
    }
}

/// **Gate 1: soundness against the real kernel.** At 48 and 96 kHz, the descriptor's slowest
/// decay: decay 20 s, size 2, damping 20 kHz, diffusion 1, low cut 20 Hz, pre-delay 250 ms, fed
/// 5 s of noise at `P = 1` on both channels (the stored energy settles in about
/// `1 / (2 (1 - r0))` samples, 1.4 s), then zeros to the stated `T`. Every sample of the last 2 s
/// before `T` is below `eps P`. A second run starts at decay 10 s, size 1, damping 6 kHz,
/// diffusion 0.5, low cut 500 Hz, pre-delay 0 and moves all of them to the first run's values 32
/// samples before the input stops, so the tap and pre-delay crossfades and every ramp are in
/// flight at `N`.
#[test]
#[cfg_attr(debug_assertions, ignore = "about 110 million frames: release only")]
fn the_kernel_decays_within_the_stated_tail() {
    let fill_seconds = 5;
    for rate in [48_000_u32, 96_000] {
        let bound =
            conformance::tail_bound_of(Box::new(ReverbFactory), rate, EffectQuality::Normal)
                .bound();
        let TailSamples::Finite(tail) = bound.tail else {
            panic!("{rate} Hz: a finite tail");
        };
        let mut slowest = common::defaults();
        slowest[PRE_DELAY] = maximum(PRE_DELAY);
        slowest[DECAY] = maximum(DECAY);
        slowest[SIZE] = maximum(SIZE);
        slowest[DAMPING] = maximum(DAMPING);
        slowest[LOW_CUT] = minimum(LOW_CUT);
        slowest[DIFFUSION] = maximum(DIFFUSION);
        for in_flight in [false, true] {
            let start = if in_flight {
                let mut values = common::defaults();
                values[PRE_DELAY] = 0.0;
                values[DECAY] = 10_000.0;
                values[SIZE] = 1.0;
                values[DAMPING] = 6_000.0;
                values[LOW_CUT] = maximum(LOW_CUT);
                values[DIFFUSION] = 0.5;
                values
            } else {
                slowest
            };
            let mut effect = common::prepare(&start, rate);
            let fill = fill_seconds * rate as usize;
            let split = fill - 32;
            let spans: Vec<PreparedAutomationSpan> = (0..8)
                .filter(|&index| slowest[index] != start[index])
                .map(|index| PreparedAutomationSpan {
                    channel: ParameterChannel::Both,
                    parameter_index: index as u32,
                    sample: split as u64,
                    value: slowest[index],
                })
                .collect();
            assert_eq!(spans.is_empty(), !in_flight);
            let (mut left, mut right) = noise(fill, 1.0, u64::from(rate) + u64::from(in_flight));
            render(
                effect.as_mut(),
                &mut left[..split],
                &mut right[..split],
                0,
                &[],
            );
            render(
                effect.as_mut(),
                &mut left[split..],
                &mut right[split..],
                split as u64,
                &spans,
            );
            let window = 2 * u64::from(rate);
            let mut last_loud = None;
            render_silence(effect.as_mut(), fill as u64, tail, |done, left, right| {
                for (n, (l, r)) in left.iter().zip(right).enumerate() {
                    if f64::from(l.abs().max(r.abs())) >= EPS {
                        last_loud = Some(done + n as u64);
                    }
                }
            });
            let last_loud = last_loud.expect("the tail is audible at first");
            assert!(
                last_loud + window < tail,
                "{rate} Hz in flight {in_flight}: |y| >= eps P at {last_loud} samples into the silence, inside the last 2 s before T = {tail}"
            );
            eprintln!(
                "{rate} Hz in flight {in_flight}: last |y| >= eps P at {last_loud} samples ({:.1} s), T = {tail} ({:.1} s)",
                last_loud as f64 / f64::from(rate),
                tail as f64 / f64::from(rate),
            );
        }
    }
}

/// The signal-state words of a payload (`src/state.rs`): the 32 filter states that end the
/// 180-word common header and the line rings after it; each channel's four pipeline registers
/// (words 4-7 of its 8-word header) and its pre-delay and diffuser rings after it. A ring cell
/// the history counter makes unreadable is written `+0.0`. Cursors, taps, the history counter,
/// ramps and crossfade state are not signal state.
fn signal_words(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Vec<f32> {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    let floats = |bytes: &[u8]| -> Vec<f32> {
        bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| f32::from_le_bytes(*word))
            .collect()
    };
    let mut words = floats(&common)[180 - 32..].to_vec();
    for lane in [&left, &right] {
        words.extend_from_slice(&floats(lane)[4..]);
    }
    words
}

/// **Gate 2: exact rest by the stated bound, real kernel**, at 48 kHz with the decay domain
/// restricted to 100 ms (`reverb::tail_bound_at(48_000, 100.0)`; the descriptor evaluates the same
/// function at 20 s), so the bound is short enough to reach. The slowest designs for rest:
/// diffusion 1 (the diffusers decay slowest), pre-delay 250 ms, damping 1 kHz and low cut 20 Hz
/// (the slowest filter walks), size 0.25 and 2, fed 1 s of noise at `P = 1e29` (just below the
/// block limit the bound starts from; no block fails) then zeros. By the bound the output is
/// `+-0.0` and every signal-state word equals a freshly prepared reverb's under `==`, and one more
/// block leaves them there. At size 2 the state is not yet at rest at 80 % of the bound.
#[test]
fn the_state_rests_by_the_stated_bound() {
    let rate = 48_000;
    let sizes = sizes(rate);
    let RestBound::Bounded(RestSamples {
        any_sanitized_input: bound,
        ..
    }) = reverb::tail_bound_at(rate, minimum(DECAY)).rest
    else {
        panic!("a bounded rest");
    };
    for size in [minimum(SIZE), maximum(SIZE)] {
        let mut values = common::defaults();
        values[PRE_DELAY] = maximum(PRE_DELAY);
        values[DECAY] = minimum(DECAY);
        values[SIZE] = size;
        values[DAMPING] = minimum(DAMPING);
        values[LOW_CUT] = minimum(LOW_CUT);
        values[DIFFUSION] = maximum(DIFFUSION);
        let mut effect = common::prepare(&values, rate);
        let rest = signal_words(common::prepare(&values, rate).as_ref(), sizes);
        assert!(rest.iter().all(|word| word.to_bits() == 0), "Z is +0.0");
        let at_rest = |effect: &dyn PreparedNativeEffect| {
            signal_words(effect, sizes)
                .iter()
                .zip(&rest)
                .all(|(a, b)| a == b)
        };
        let fill = rate as usize;
        let (mut left, mut right) = noise(fill, 1.0e29, u64::from(size.to_bits()));
        render(effect.as_mut(), &mut left, &mut right, 0, &[]);
        let probe = bound * 4 / 5;
        let mut silent = 0_u64;
        let mut reached = None;
        while silent < bound {
            let next = if silent < probe { probe } else { bound };
            let frames = u64::from(QUANTUM).min(next - silent) as usize;
            let mut left = vec![0.0_f32; frames];
            let mut right = vec![0.0_f32; frames];
            render(
                effect.as_mut(),
                &mut left,
                &mut right,
                fill as u64 + silent,
                &[],
            );
            silent += frames as u64;
            if reached.is_none() && at_rest(effect.as_ref()) {
                reached = Some(silent);
            }
            if silent == probe && size == maximum(SIZE) {
                assert!(
                    !at_rest(effect.as_ref()),
                    "size {size}: already at rest at 80 % of the bound {bound}: the gate is vacuous"
                );
            }
        }
        assert!(
            at_rest(effect.as_ref()),
            "size {size}: not at rest after the bound {bound}"
        );
        let mut left = vec![0.0_f32; QUANTUM as usize];
        let mut right = vec![0.0_f32; QUANTUM as usize];
        render(
            effect.as_mut(),
            &mut left,
            &mut right,
            fill as u64 + bound,
            &[],
        );
        assert!(
            left.iter().chain(&right).all(|y| *y == 0.0),
            "size {size}: non-zero output after the bound"
        );
        assert!(
            at_rest(effect.as_ref()),
            "size {size}: the rest state moved"
        );
        let reached = reached.expect("reached");
        eprintln!(
            "48000 Hz decay 100 ms size {size}: at rest after {reached} samples, bound {bound} ({:.1} %)",
            100.0 * reached as f64 / bound as f64
        );
    }
}
