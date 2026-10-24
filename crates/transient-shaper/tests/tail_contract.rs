#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the test's own f64 recomputation of the stated values.
//! Issue #1376's gates for the transient shaper: the stated zero tail, the exact-rest bound and
//! the composition values, against the real kernel.
//!
//! Every launch rate in release; 48 kHz only in debug, where gate 2 runs only the deeper start.
//! Evidence lines print with `--nocapture`.
#![allow(missing_docs)]

mod common;

use common::{request_full, values_of};
use effect_contract::{
    CompositionBound, EffectProcessBlock, EffectQuality, FlushStall, LinkMode, NativeEffectFactory,
    NodeTailBound, ParameterChannel, PeakGain, PreparedAutomationSpan, PreparedNativeEffect,
    RestBound, RestSamples, StatePayloadOutput, StatePayloadSizes, TailDecay, TailSamples,
};
use transient_shaper::{TRANSIENT_SHAPER_COEFFICIENT_BITS, TransientShaperFactory};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// Samples of signal before the silence (gates 1 and 2).
const SIGNAL: usize = 4_096;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;
/// The follower rest floor, `FOLLOWER_REST` (#1784 B2).
const FOLLOWER_REST: f32 = 1.0e-12;
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn row(rate: u32) -> [f32; 4] {
    let index = LAUNCH_RATES
        .iter()
        .position(|r| *r == rate)
        .expect("a rate");
    TRANSIENT_SHAPER_COEFFICIENT_BITS[index].map(f32::from_bits)
}

fn stated(rate: u32) -> NodeTailBound {
    conformance::tail_bound_of(
        Box::new(TransientShaperFactory),
        rate,
        EffectQuality::Normal,
    )
    .bound()
}

fn stated_rest(rate: u32) -> RestSamples {
    let bound = stated(rate);
    let RestBound::Bounded(rest) = bound.rest else {
        panic!("{rate} Hz: no exact-rest bound stated");
    };
    assert_eq!(
        bound.tail_every_peak,
        TailSamples::Finite(rest.peak_plus_24_dbfs.max(rest.any_sanitized_input)),
        "{rate} Hz"
    );
    rest
}

fn noise(samples: usize, seed: u64, scale: f32) -> Vec<f32> {
    let mut generator = conformance::SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * scale)
        .collect()
}

fn prepare(
    rate: u32,
    attack: f32,
    sustain: f32,
    mix: f32,
    link: LinkMode,
) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let values = values_of(attack, sustain, mix);
    let prepared = TransientShaperFactory
        .prepare(request_full(&values, rate, QUANTUM as u32, false, link))
        .expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

/// Renders `left`/`right` in quantum blocks from `first`.
fn render(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first: u64,
    spans: &[PreparedAutomationSpan],
) {
    let mut offset = 0;
    while offset < left.len() {
        let end = (offset + QUANTUM).min(left.len());
        let block_spans: Vec<PreparedAutomationSpan> = spans
            .iter()
            .filter(|span| span.sample >= first + offset as u64 && span.sample < first + end as u64)
            .copied()
            .collect();
        let _ = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                first + offset as u64,
                &block_spans,
                QUANTUM as u32,
            )
            .expect("block"),
        );
        offset = end;
    }
}

/// Both channel sections of the payload, as words.
fn snapshot(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Vec<u32> {
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    left.chunks_exact(4)
        .chain(right.chunks_exact(4))
        .map(|word| u32::from_le_bytes(word.try_into().expect("a word")))
        .collect()
}

/// Every word equal under `f32` `==` (the shaper has no cursor words; the ramp `remaining` words
/// are integers and compare as bits).
fn at_rest(state: &[u32], rest: &[u32]) -> bool {
    state
        .iter()
        .zip(rest)
        .all(|(a, b)| f32::from_bits(*a) == f32::from_bits(*b))
}

/// `ceil(2000 log10 g)`, independently of the crate's helper.
fn millibels(g: f64) -> i32 {
    (2000.0 * math::log(g) / core::f64::consts::LN_10).ceil() as i32
}

/// **Gates 1 and 3: zero output from the first silent sample, and (N1) at every sample,** at
/// every rate, over the domain extremes (attack and sustain at `-1` and `1`, `mix` 0, 0.5 and 1)
/// in every link mode. The stated values are `Finite(0)`, decay `0`, and D4's gain and stall
/// recomputed here: `G_p = G_t = ceil_mB(10^((18 + 1e-5) / 20) (1 + u)^4)` (the shape clamp and the
/// fast tier's proven `1e-5` dB gain error) and `sigma_p = sigma_t = ceil_mB(4 2^-126 (1 + u)^3)`.
#[test]
fn the_output_is_zero_after_the_input_and_within_the_stated_peak_gain() {
    let gain = millibels(math::db_to_gain(18.0 + 1.0e-5) * (1.0 + U).powi(4));
    let stall = millibels(4.0 * 2.0_f64.powi(-126) * (1.0 + U).powi(3));
    for &rate in rates() {
        let bound = stated(rate);
        assert_eq!(bound.tail, TailSamples::Finite(0), "{rate} Hz");
        assert_eq!(
            bound.composition,
            CompositionBound::Stated {
                decay: TailDecay(0),
                peak_gain: PeakGain::Millibels(gain),
                tail_gain: PeakGain::Millibels(gain),
                peak_stall: FlushStall::Level(stall),
                tail_stall: FlushStall::Level(stall),
            },
            "{rate} Hz"
        );
        let g_p = 10.0_f64.powf(f64::from(gain) / 2000.0);
        let sigma_p = 10.0_f64.powf(f64::from(stall) / 2000.0);
        let mut worst = 0.0_f64;
        let mut index = 0_u64;
        for link in LINKS {
            for attack in [-1.0, 1.0] {
                for sustain in [-1.0, 1.0] {
                    for mix in [0.0, 0.5, 1.0] {
                        index += 1;
                        let seed = 0x1376_0000 + index;
                        let (mut effect, _) = prepare(rate, attack, sustain, mix, link);
                        let total = 2 * SIGNAL;
                        // Bursts, so the followers see attacks and releases, not only noise.
                        let envelope = |n: usize| {
                            if (n / 512).is_multiple_of(2) {
                                1.0
                            } else {
                                0.05
                            }
                        };
                        let mut left = noise(total, seed, 1.0);
                        let mut right = noise(total, seed ^ 0xff, 1.0);
                        for (n, (l, r)) in left.iter_mut().zip(&mut right).enumerate() {
                            *l *= envelope(n);
                            *r *= envelope(n);
                        }
                        left[SIGNAL..].fill(0.0);
                        right[SIGNAL..].fill(0.0);
                        let peak = left
                            .iter()
                            .chain(&right)
                            .fold(0.0_f32, |p, x| p.max(x.abs()));
                        render(effect.as_mut(), &mut left, &mut right, 0, &[]);
                        let limit = g_p * f64::from(peak) + sigma_p;
                        for (n, y) in left.iter().chain(&right).enumerate() {
                            assert!(
                                f64::from(y.abs()) <= limit,
                                "{rate} Hz {link:?} {attack} {sustain} {mix}: |y| = {y} above {limit} (frame {n})"
                            );
                            worst = worst.max(f64::from(y.abs()) / f64::from(peak));
                        }
                        for (channel, out) in [&left, &right].into_iter().enumerate() {
                            if let Some(n) = out[SIGNAL..].iter().position(|y| *y != 0.0) {
                                panic!(
                                    "{rate} Hz {link:?} {attack} {sustain} {mix} channel {channel}: output {} at {n} samples into the silence",
                                    out[SIGNAL + n]
                                );
                            }
                        }
                    }
                }
            }
        }
        eprintln!("{rate} Hz: G_p {gain} mB, sigma_p {stall} mB, largest |y| / max|x| {worst}");
    }
}

/// The largest envelope word for a detector of at most `peak`, recomputed independently of the
/// crate: `U (1 + 3 u / k_min)` rounded up to `f32` plus one ulp, `U` the `f32` at or above `peak`
/// and `k_min` the slow release's `1 - c`.
fn distance(rate: u32, peak: f64) -> f32 {
    let k_min = 1.0 - f64::from(row(rate)[3]);
    let bound = peak * (1.0 + 3.0 * U / k_min);
    let mut word = bound as f32;
    if f64::from(word) < bound {
        word = f32::from_bits(word.to_bits() + 1);
    }
    f32::from_bits(word.to_bits() + 1)
}

/// **Gate 4: the stated rest is the real slow follower's count from its largest value**: the
/// word `snap(fl(c e))` at the slow release retention (the slower of the two words), iterated in
/// `f32` from [`distance`] at both peak cases until it is `+0.0`, in `B` steps:
/// `B <= R <= B + max(2, ceil(B / 100))`.
#[test]
fn the_stated_rest_is_the_slow_followers_count() {
    for &rate in rates() {
        let rest = stated_rest(rate);
        let c = row(rate)[3];
        for (peak, stated) in [
            (math::db_to_gain(24.0), rest.peak_plus_24_dbfs),
            (f64::from(1.0e30_f32), rest.any_sanitized_input),
        ] {
            let mut e = distance(rate, peak);
            let mut count = 0_u64;
            while e != 0.0 {
                let next = c * e;
                e = if next.abs() < FOLLOWER_REST {
                    0.0
                } else {
                    next
                };
                count += 1;
            }
            assert!(
                count <= stated && stated <= count + 2.max(count.div_ceil(100)),
                "{rate} Hz peak {peak:e}: the word rests in {count} steps, stated {stated}"
            );
            eprintln!("{rate} Hz peak {peak:e}: R = {stated}, brute force {count}");
        }
    }
}

/// **Gate 2: exact rest by the stated `R`, real kernel**, at both peak cases (`10^(24/20)` and
/// `1e29`) in every link mode, and once per rate with an attack-amount ramp still in flight when
/// the input stops. From `R` on the output is `+-0.0` and the payload equals, word for word under
/// `==`, the payload of a freshly reset instance with the same settled parameters fed the same
/// zeros (`Z`, a fixed point), and the two then render the same seeded noise to the bit.
#[test]
fn the_state_reaches_its_rest_by_the_stated_bound() {
    for &rate in rates() {
        let rest_bounds = stated_rest(rate);
        for (peak, rest_bound) in [
            (10.0_f32.powf(24.0 / 20.0), rest_bounds.peak_plus_24_dbfs),
            (1.0e29, rest_bounds.any_sanitized_input),
        ] {
            for (link, ramp) in [
                (LinkMode::DualMono, false),
                (LinkMode::Maximum, false),
                (LinkMode::Average, false),
                (LinkMode::DualMono, true),
            ] {
                if cfg!(debug_assertions) && (peak < 1.0e28 || link != LinkMode::DualMono) {
                    continue;
                }
                let (mut effect, sizes) = prepare(rate, 1.0, -1.0, 1.0, link);
                let spans: Vec<PreparedAutomationSpan> = if ramp {
                    [ParameterChannel::Left, ParameterChannel::Right]
                        .into_iter()
                        .map(|channel| PreparedAutomationSpan {
                            channel,
                            parameter_index: 0,
                            sample: (SIGNAL - 16) as u64,
                            value: -1.0,
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                let mut left = noise(SIGNAL, 0x1376_2000, peak);
                let mut right = noise(SIGNAL, 0x1376_2001, peak);
                // The last signal block starts where the ramp does: a `Block`-rate point applies
                // at its block's first sample.
                let split = SIGNAL - 16;
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
                let attack = if ramp { -1.0 } else { 1.0 };
                let (mut fresh, _) = prepare(rate, attack, -1.0, 1.0, link);
                let rest = snapshot(fresh.as_ref(), sizes);
                let mut silent = 0_u64;
                let reached = loop {
                    if at_rest(&snapshot(effect.as_ref(), sizes), &rest) {
                        break silent;
                    }
                    assert!(
                        silent < rest_bound,
                        "{rate} Hz peak {peak:e} {link:?} ramp {ramp}: not at rest after {silent} samples, bound {rest_bound}"
                    );
                    let frames = (QUANTUM as u64).min(rest_bound - silent) as usize;
                    let mut left = vec![0.0_f32; frames];
                    let mut right = vec![0.0_f32; frames];
                    render(
                        effect.as_mut(),
                        &mut left,
                        &mut right,
                        SIGNAL as u64 + silent,
                        &[],
                    );
                    assert!(
                        left.iter().chain(&right).all(|y| *y == 0.0),
                        "{rate} Hz: non-zero output in the silence"
                    );
                    let mut zl = vec![0.0_f32; frames];
                    let mut zr = vec![0.0_f32; frames];
                    render(fresh.as_mut(), &mut zl, &mut zr, silent, &[]);
                    assert_eq!(
                        rest,
                        snapshot(fresh.as_ref(), sizes),
                        "Z is not a fixed point"
                    );
                    silent += frames as u64;
                };
                let mut a = (
                    noise(SIGNAL, 0x1376_3000, 0.5),
                    noise(SIGNAL, 0x1376_3001, 0.5),
                );
                let mut b = a.clone();
                render(
                    effect.as_mut(),
                    &mut a.0,
                    &mut a.1,
                    SIGNAL as u64 + reached,
                    &[],
                );
                render(fresh.as_mut(), &mut b.0, &mut b.1, reached, &[]);
                assert!(
                    a.0.iter()
                        .zip(&b.0)
                        .chain(a.1.iter().zip(&b.1))
                        .all(|(x, y)| x.to_bits() == y.to_bits()),
                    "{rate} Hz peak {peak:e} {link:?} ramp {ramp}: the rested instance renders unlike Z"
                );
                eprintln!(
                    "{rate} Hz peak {peak:e} {link:?} ramp {ramp}: rest after {reached} samples, bound {rest_bound}"
                );
            }
        }
    }
}
