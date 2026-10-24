#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the test's own f64 recomputation of the stated values.
//! Issue #1375's gates for the true-peak limiter: the stated zero tail beyond latency, the
//! exact-rest bound and the composition values, against the real kernel.
//!
//! Every launch rate in release; 48 kHz only in debug, where gate 2 runs its slowest-release cases
//! (a few million samples each) only at the worst start. Evidence lines print with `--nocapture`.
#![allow(missing_docs)]

use effect_contract::{
    CompositionBound, EffectProcessBlock, EffectQuality, FlushStall, InitialParameterValue,
    LinkMode, NativeEffectFactory, NodeTailBound, ParameterChannel, PeakGain, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedPorts,
    PreparedSidechainPort, RestBound, StatePayloadOutput, StatePayloadSizes, TailDecay,
    TailSamples,
};
use true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR, TRUE_PEAK_LIMITER_PARAMETERS, TruePeakLimiterFactory,
};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// Samples of signal before the silence (#1375 gates 1 and 2).
const SIGNAL: usize = 4_096;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;

const CEILING: usize = 0;
const RELEASE: usize = 1;
const LOOKAHEAD: usize = 2;

/// Payload words of a channel section that are cursors, not signal state: the bank's main-delay
/// and gain-ring cursors and the van Herk phase (`words::MAIN_CURSOR`, `RING_CURSOR`, `PHASE`).
const CURSOR_WORDS: [usize; 3] = [0, 1, 4];

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn domain(parameter: usize) -> (f32, f32) {
    let row = &TRUE_PEAK_LIMITER_PARAMETERS[parameter];
    (
        row.minimum.expect("a minimum"),
        row.maximum.expect("a maximum"),
    )
}

fn latency(rate: u32) -> usize {
    TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == rate)
        .expect("launch rate")
        .latency
        .0 as usize
}

fn stated(rate: u32) -> NodeTailBound {
    conformance::tail_bound_of(
        Box::new(TruePeakLimiterFactory),
        rate,
        EffectQuality::Normal,
    )
    .bound()
}

fn stated_rest(rate: u32) -> u64 {
    let bound = stated(rate);
    let RestBound::Bounded(rest) = bound.rest else {
        panic!("{rate} Hz: no exact-rest bound stated");
    };
    assert_eq!(
        rest.peak_plus_24_dbfs, rest.any_sanitized_input,
        "{rate} Hz"
    );
    assert_eq!(
        bound.tail_every_peak,
        TailSamples::Finite(rest.any_sanitized_input),
        "{rate} Hz"
    );
    rest.any_sanitized_input
}

/// Deterministic SplitMix64 bipolar noise.
fn noise(samples: usize, seed: u64, scale: f32) -> Vec<f32> {
    let mut generator = conformance::SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * scale)
        .collect()
}

/// `(ceiling, release, lookahead)` on both channels.
fn values_of(case: [f32; 3]) -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index.is_multiple_of(2) {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: case[index / 2],
    })
}

fn prepare(
    rate: u32,
    values: &[InitialParameterValue],
    link_mode: LinkMode,
) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let request = PrepareEffectRequest {
        sample_rate: rate,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 16,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(TruePeakLimiterFactory),
            rate,
            EffectQuality::Normal,
        ),
    };
    let prepared = TruePeakLimiterFactory.prepare(request).expect("prepare");
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

/// Both channel sections of the payload as words, the cursor words cleared.
fn snapshot(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Vec<f32> {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    let section = |bytes: &[u8]| -> Vec<f32> {
        bytes
            .chunks_exact(4)
            .enumerate()
            .map(|(word, bytes)| {
                if CURSOR_WORDS.contains(&word) {
                    0.0
                } else {
                    f32::from_le_bytes(bytes.try_into().expect("a word"))
                }
            })
            .collect()
    };
    let mut words = section(&left);
    words.extend(section(&right));
    words
}

fn at_rest(state: &[f32], rest: &[f32]) -> bool {
    state.iter().zip(rest).all(|(a, b)| a == b)
}

/// `ceil(2000 log10 g)`, independently of the crate's helper.
fn millibels(g: f64) -> i32 {
    (2000.0 * math::log(g) / core::f64::consts::LN_10).ceil() as i32
}

/// **Gates 1 and 3: zero output from `latency` after the last non-zero input, and (N1) at every
/// sample,** at every rate, over the domain extremes (the lowest and highest ceiling, the fastest
/// and slowest release, lookahead 0 and maximum, both link modes), with full-scale noise. The
/// stated values are `Finite(0)`, decay `0`, and D4's gain and stall recomputed here: `G_p = G_t =
/// ceil_mB(1 + u)` (0 dB and the one rounded product) and `sigma_p = sigma_t = ceil_mB(2^-126)`.
#[test]
fn the_output_is_zero_after_the_latency_and_within_the_stated_peak_gain() {
    let gain = millibels(1.0 + U);
    let stall = millibels(2.0_f64.powi(-126));
    let g_p = 10.0_f64.powf(f64::from(gain) / 2000.0);
    let sigma_p = 10.0_f64.powf(f64::from(stall) / 2000.0);
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
        let delay = latency(rate);
        let mut seed = 0x1375_5000_u64;
        for ceiling in [domain(CEILING).0, domain(CEILING).1] {
            for release in [domain(RELEASE).0, domain(RELEASE).1] {
                for lookahead in [domain(LOOKAHEAD).0, domain(LOOKAHEAD).1] {
                    for link in [LinkMode::DualMono, LinkMode::Maximum] {
                        seed += 2;
                        let case = [ceiling, release, lookahead];
                        let (mut effect, _) = prepare(rate, &values_of(case), link);
                        let total = 2 * SIGNAL + delay;
                        let mut left = noise(total, seed, 1.0);
                        let mut right = noise(total, seed + 1, 1.0);
                        left[SIGNAL..].fill(0.0);
                        right[SIGNAL..].fill(0.0);
                        let peak = left
                            .iter()
                            .chain(&right)
                            .fold(0.0_f32, |p, x| p.max(x.abs()));
                        render(effect.as_mut(), &mut left, &mut right, 0, &[]);
                        let limit = g_p * f64::from(peak) + sigma_p;
                        for (channel, out) in [&left, &right].into_iter().enumerate() {
                            if let Some(n) = out.iter().position(|y| f64::from(y.abs()) > limit) {
                                panic!(
                                    "{rate} Hz {case:?} {link:?} channel {channel}: |y| = {} above {limit} at {n}",
                                    out[n]
                                );
                            }
                            if let Some(n) = out[SIGNAL + delay..].iter().position(|y| *y != 0.0) {
                                panic!(
                                    "{rate} Hz {case:?} {link:?} channel {channel}: output {} at {n} samples after the latency",
                                    out[SIGNAL + delay + n]
                                );
                            }
                        }
                    }
                }
            }
        }
        eprintln!("{rate} Hz: G_p {gain} mB, sigma_p {stall} mB");
    }
}

/// **Gate 4: the stated rest is the ring drain plus the real reduction word's count from its
/// largest distance**: `R = N + 5 + B`, `N = Fs / 100`, with `B` the steps the word
/// `flush(max(+0, fma(c, 0 - d, d)))` takes in `f32` from `d = 1` at the slowest release
/// coefficient (2,000 ms, one `f32` below its design) to `+0.0`:
/// `B <= R - N - 5 <= B + max(2, ceil(B / 100))`.
#[test]
fn the_stated_rest_is_the_ring_drain_and_the_reduction_words_count() {
    for &rate in rates() {
        let tau = 0.001 * f64::from(domain(RELEASE).1) * f64::from(rate);
        let c = f32::from_bits(((1.0 - math::exp(-1.0 / tau)) as f32).to_bits() - 1);
        let mut d = 1.0_f32;
        let mut count = 0_u64;
        while d != 0.0 {
            // `Lane::fma`'s two roundings: Rust never fuses `a * b + c`.
            let next = (c * (0.0 - d) + d).max(0.0);
            d = if next.abs() < lane::FLUSH_EPS {
                0.0
            } else {
                next
            };
            count += 1;
        }
        let rest = stated_rest(rate);
        let drain = u64::from(rate / 100) + 5;
        let follower = rest - drain;
        assert!(
            count <= follower && follower <= count + 2.max(count.div_ceil(100)),
            "{rate} Hz: the word rests in {count} steps, stated {rest} = {drain} + {follower}"
        );
        eprintln!("{rate} Hz: R = {rest} = {drain} + {follower}, brute force {count}");
    }
}

/// **Gate 2: exact rest by `latency + R`, real kernel**, at both peak cases (`10^(24/20)` and
/// `1e29`), the lowest ceiling, lookahead 0 and maximum, at the slowest and the fastest release
/// and with a release ramp from the fastest to the slowest still in flight when the input stops.
/// From `latency + R` on the payload equals, word for word under `==` and except the cursor
/// words, the payload of a freshly reset instance with the same settled parameters fed the same
/// zeros (`Z`, a fixed point: rings `1.0`, box sum `Wb`, `d = +0.0`), and the two then render the
/// same seeded noise to the bit.
#[test]
fn the_state_reaches_its_rest_by_the_stated_bound() {
    let (release_min, release_max) = domain(RELEASE);
    for &rate in rates() {
        let bound = stated_rest(rate) + latency(rate) as u64;
        for peak in [10.0_f32.powf(24.0 / 20.0), 1.0e29] {
            for lookahead in [domain(LOOKAHEAD).0, domain(LOOKAHEAD).1] {
                for (release, ramp) in [
                    (release_max, false),
                    (release_min, false),
                    (release_min, true),
                ] {
                    // Debug runs one slowest-release pair, at the worst start (`d = 1` exactly).
                    let slow = release == release_max || ramp;
                    if cfg!(debug_assertions)
                        && slow
                        && (peak < 1.0e28 || lookahead < domain(LOOKAHEAD).1)
                    {
                        continue;
                    }
                    let mut case = [domain(CEILING).0, release, lookahead];
                    let link = LinkMode::DualMono;
                    let (mut effect, sizes) = prepare(rate, &values_of(case), link);
                    let mut left = noise(SIGNAL, 0x1375_6000, peak);
                    let mut right = noise(SIGNAL, 0x1375_6001, peak);
                    // The last signal block starts where the ramp does: a `Block`-rate point
                    // applies at its block's first sample.
                    let split = SIGNAL - 16;
                    render(
                        effect.as_mut(),
                        &mut left[..split],
                        &mut right[..split],
                        0,
                        &[],
                    );
                    let spans: Vec<PreparedAutomationSpan> = if ramp {
                        case[1] = release_max;
                        [ParameterChannel::Left, ParameterChannel::Right]
                            .into_iter()
                            .map(|channel| PreparedAutomationSpan {
                                channel,
                                parameter_index: RELEASE as u32,
                                sample: split as u64,
                                value: release_max,
                            })
                            .collect()
                    } else {
                        Vec::new()
                    };
                    render(
                        effect.as_mut(),
                        &mut left[split..],
                        &mut right[split..],
                        split as u64,
                        &spans,
                    );
                    let (mut fresh, _) = prepare(rate, &values_of(case), link);
                    let rest = snapshot(fresh.as_ref(), sizes);
                    let context = format!(
                        "{rate} Hz peak {peak:e} lookahead {lookahead} release {release} ramp {ramp}"
                    );
                    let mut silent = 0_u64;
                    let reached = loop {
                        if at_rest(&snapshot(effect.as_ref(), sizes), &rest) {
                            break silent;
                        }
                        assert!(
                            silent < bound,
                            "{context}: not at rest after {silent} samples, bound {bound}"
                        );
                        let frames = (QUANTUM as u64).min(bound - silent) as usize;
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames];
                        render(
                            effect.as_mut(),
                            &mut left,
                            &mut right,
                            SIGNAL as u64 + silent,
                            &[],
                        );
                        let skip =
                            ((latency(rate) as u64).saturating_sub(silent) as usize).min(frames);
                        assert!(
                            left[skip..].iter().chain(&right[skip..]).all(|y| *y == 0.0),
                            "{context}: non-zero output after the latency"
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
                        noise(SIGNAL, 0x1375_7000, 0.9),
                        noise(SIGNAL, 0x1375_7001, 0.9),
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
                            .all(|(x, y)| x == y),
                        "{context}: the rested instance renders unlike Z"
                    );
                    eprintln!("{context}: rest after {reached} samples, bound {bound}");
                }
            }
        }
    }
}
