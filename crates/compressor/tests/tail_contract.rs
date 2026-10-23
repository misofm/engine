//! Issue #1375's gates for the compressor: the stated zero tail beyond latency, the exact-rest
//! bound and the composition values, against the real kernel.
//!
//! Every launch rate in release; 48 kHz only in debug, where gate 2's slowest-release runs are a
//! few million samples each. Evidence lines print with `--nocapture`.
#![allow(missing_docs)]

mod support;

use compressor::{COMPRESSOR_PARAMETERS, CompressorFactory};
use effect_contract::{
    CompositionBound, EffectProcessBlock, EffectQuality, FlushStall, InitialParameterValue,
    LinkMode, NativeEffectFactory, NodeTailBound, ParameterChannel, PeakGain, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedPorts,
    PreparedSidechainPort, RestBound, StatePayloadOutput, StatePayloadSizes, TailDecay,
    TailSamples,
};
use support::{noise, sidechain_port, values_with};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// Samples of signal before the silence (#1375 gates 1 and 2).
const SIGNAL: usize = 4_096;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;

const THRESHOLD: usize = 0;
const RATIO: usize = 1;
const ATTACK: usize = 3;
const RELEASE: usize = 4;
const MAKEUP: usize = 5;
const MIX: usize = 6;

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn domain(parameter: usize) -> (f32, f32) {
    let row = &COMPRESSOR_PARAMETERS[parameter];
    (
        row.minimum.expect("a minimum"),
        row.maximum.expect("a maximum"),
    )
}

fn stated(rate: u32) -> NodeTailBound {
    conformance::tail_bound_of(Box::new(CompressorFactory), rate, EffectQuality::Normal).bound()
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

/// A prepared scalar instance at `rate`, the sidechain connected or not.
fn prepare(
    rate: u32,
    values: &[InitialParameterValue],
    sidechain: bool,
) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let id = sidechain_port();
    let request = PrepareEffectRequest {
        sample_rate: rate,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: if sidechain {
                PreparedSidechainPort::Connected {
                    id,
                    required: false,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id,
                    required: false,
                }
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 16,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(CompressorFactory),
            rate,
            EffectQuality::Normal,
        ),
    };
    let prepared = CompressorFactory.prepare(request).expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

/// Renders `left`/`right` (and the sidechain, when given) in quantum blocks from `first`.
fn render(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    sidechain: Option<(&[f32], &[f32])>,
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
                sidechain.map(|(l, r)| (&l[offset..end], &r[offset..end])),
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
fn snapshot(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Vec<f32> {
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    left.chunks_exact(4)
        .chain(right.chunks_exact(4))
        .map(|word| f32::from_le_bytes(word.try_into().expect("a word")))
        .collect()
}

/// Every word equal under `f32` `==` (the compressor has no cursor words).
fn at_rest(state: &[f32], rest: &[f32]) -> bool {
    state.iter().zip(rest).all(|(a, b)| a == b)
}

/// The domain extremes gates 1 and 3 run, as `(threshold, ratio, attack, release, makeup, mix)`.
fn extremes() -> Vec<[f32; 6]> {
    let mut cases = Vec::new();
    for (threshold, ratio) in [(domain(THRESHOLD).0, domain(RATIO).1), (0.0, 1.0)] {
        for attack in [domain(ATTACK).0, domain(ATTACK).1] {
            for release in [domain(RELEASE).0, domain(RELEASE).1] {
                for makeup in [domain(MAKEUP).0, domain(MAKEUP).1] {
                    for mix in [0.0, 0.5, 1.0] {
                        cases.push([threshold, ratio, attack, release, makeup, mix]);
                    }
                }
            }
        }
    }
    cases
}

fn values_of(case: [f32; 6]) -> [InitialParameterValue; 14] {
    values_with(&[
        (THRESHOLD, case[0]),
        (RATIO, case[1]),
        (ATTACK, case[2]),
        (RELEASE, case[3]),
        (MAKEUP, case[4]),
        (MIX, case[5]),
    ])
}

/// `ceil(2000 log10 g)`, independently of the crate's helper.
fn millibels(g: f64) -> i32 {
    (2000.0 * math::log(g) / core::f64::consts::LN_10).ceil() as i32
}

/// **Gates 1 and 3: zero output from the first silent sample, and (N1) at every sample,** at
/// every rate, over the domain extremes (fastest and slowest attack and release, minimum and
/// maximum makeup, `mix` 0, 0.5 and 1, the deepest curve and no curve), the sidechain unconnected
/// or connected with full-scale noise that keeps playing through the silence. The stated values
/// are `Finite(0)`, decay `0`, and D4's gain and stall recomputed here: `G_p = G_t =
/// ceil_mB(10^((24 + 1e-5) / 20) (1 + u)^4)` (the fast tier's proven `1e-5` dB gain error) and
/// `sigma_p = sigma_t = ceil_mB(4 2^-126 (1 + u)^3)`.
#[test]
fn the_output_is_zero_after_the_input_and_within_the_stated_peak_gain() {
    let makeup_max = f64::from(domain(MAKEUP).1);
    let gain = millibels(math::db_to_gain(makeup_max + 1.0e-5) * (1.0 + U).powi(4));
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
        for (index, case) in extremes().into_iter().enumerate() {
            for sidechain in [false, true] {
                let seed = 0x1375_0000 + index as u64 * 2 + u64::from(sidechain);
                let (mut effect, _) = prepare(rate, &values_of(case), sidechain);
                let total = 2 * SIGNAL;
                let mut left = noise(total, seed, 1.0);
                let mut right = noise(total, seed ^ 0xff, 1.0);
                left[SIGNAL..].fill(0.0);
                right[SIGNAL..].fill(0.0);
                let peak = left
                    .iter()
                    .chain(&right)
                    .fold(0.0_f32, |p, x| p.max(x.abs()));
                let input = (left.clone(), right.clone());
                let keys = (noise(total, !seed, 1.0), noise(total, !seed ^ 0xff, 1.0));
                let keys = sidechain.then_some((keys.0.as_slice(), keys.1.as_slice()));
                render(effect.as_mut(), &mut left, &mut right, keys, 0, &[]);
                for (n, (y, x)) in left
                    .iter()
                    .zip(&input.0)
                    .chain(right.iter().zip(&input.1))
                    .enumerate()
                {
                    let limit = g_p * f64::from(peak) + sigma_p;
                    assert!(
                        f64::from(y.abs()) <= limit,
                        "{rate} Hz {case:?} sidechain {sidechain}: |y| = {y} above {limit} (x = {x}, frame {n})"
                    );
                    if peak > 0.0 {
                        worst = worst.max(f64::from(y.abs()) / f64::from(peak));
                    }
                }
                for (channel, out) in [&left, &right].into_iter().enumerate() {
                    if let Some(n) = out[SIGNAL..].iter().position(|y| *y != 0.0) {
                        panic!(
                            "{rate} Hz {case:?} sidechain {sidechain} channel {channel}: output {} at {} samples into the silence",
                            out[SIGNAL + n],
                            n
                        );
                    }
                }
            }
        }
        eprintln!("{rate} Hz: G_p {gain} mB, sigma_p {stall} mB, largest |y| / max|x| {worst}");
    }
}

/// **Gate 4: the stated rest is the real gain-reduction word's count from its largest distance**:
/// the word `snap_reduction(fma(c, 0 - g, g))`, iterated in `f32` from `100` dB plus one ulp at
/// the slowest release coefficient (5,000 ms, one `f32` below its design) until it is `+0.0`, in
/// `B` steps: `B <= R <= B + max(2, ceil(B / 100))`.
#[test]
fn the_stated_rest_is_the_gain_words_count() {
    let floor = f32::from_bits(0x3510_7e90); // `REDUCTION_REST_DB`, #1784 B1.
    for &rate in rates() {
        let tau = 0.001 * f64::from(domain(RELEASE).1) * f64::from(rate);
        let c = f32::from_bits(((1.0 - math::exp(-1.0 / tau)) as f32).to_bits() - 1);
        let mut g = -f32::from_bits(100.0_f32.to_bits() + 1);
        let mut count = 0_u64;
        while g != 0.0 {
            // `Lane::fma`'s two roundings: Rust never fuses `a * b + c`.
            let next = c * (0.0 - g) + g;
            g = if next.abs() <= floor { 0.0 } else { next };
            count += 1;
        }
        let rest = stated_rest(rate);
        assert!(
            count <= rest && rest <= count + 2.max(count.div_ceil(100)),
            "{rate} Hz: the word rests in {count} steps, stated {rest}"
        );
        eprintln!("{rate} Hz: R = {rest}, brute force {count}");
    }
}

/// **Gate 2: exact rest by the stated `R`, real kernel**, at both peak cases (`10^(24/20)` and
/// `1e29`), the deepest curve, maximum makeup and the fastest attack, at the slowest and the
/// fastest release and with a release ramp from the fastest to the slowest still in flight when
/// the input stops. From `R` on the payload equals, word for word under `==`, the payload of a
/// freshly reset instance with the same settled parameters fed the same zeros (`Z`, a fixed
/// point), and the two then render the same seeded noise to the bit.
#[test]
fn the_state_reaches_its_rest_by_the_stated_bound() {
    let (release_min, release_max) = domain(RELEASE);
    for &rate in rates() {
        let rest_bound = stated_rest(rate);
        for peak in [10.0_f32.powf(24.0 / 20.0), 1.0e29] {
            for (release, ramp) in [
                (release_max, false),
                (release_min, false),
                (release_min, true),
            ] {
                let mut case = [
                    domain(THRESHOLD).0,
                    domain(RATIO).1,
                    domain(ATTACK).0,
                    release,
                    domain(MAKEUP).1,
                    1.0,
                ];
                let (mut effect, sizes) = prepare(rate, &values_of(case), false);
                let spans: Vec<PreparedAutomationSpan> = if ramp {
                    [ParameterChannel::Left, ParameterChannel::Right]
                        .into_iter()
                        .map(|channel| PreparedAutomationSpan {
                            channel,
                            parameter_index: RELEASE as u32,
                            sample: (SIGNAL - 16) as u64,
                            value: release_max,
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                if ramp {
                    case[3] = release_max;
                }
                let mut left = noise(SIGNAL, 0x1375_2000, peak);
                let mut right = noise(SIGNAL, 0x1375_2001, peak);
                // The last signal block starts where the ramp does: a `Block`-rate point applies
                // at its block's first sample.
                let split = SIGNAL - 16;
                render(
                    effect.as_mut(),
                    &mut left[..split],
                    &mut right[..split],
                    None,
                    0,
                    &[],
                );
                render(
                    effect.as_mut(),
                    &mut left[split..],
                    &mut right[split..],
                    None,
                    split as u64,
                    &spans,
                );
                let (mut fresh, _) = prepare(rate, &values_of(case), false);
                let rest = snapshot(fresh.as_ref(), sizes);
                let mut silent = 0_u64;
                let reached = loop {
                    if at_rest(&snapshot(effect.as_ref(), sizes), &rest) {
                        break silent;
                    }
                    assert!(
                        silent < rest_bound,
                        "{rate} Hz peak {peak:e} release {release} ramp {ramp}: not at rest after {silent} samples, bound {rest_bound}"
                    );
                    let frames = (QUANTUM as u64).min(rest_bound - silent) as usize;
                    let mut left = vec![0.0_f32; frames];
                    let mut right = vec![0.0_f32; frames];
                    render(
                        effect.as_mut(),
                        &mut left,
                        &mut right,
                        None,
                        SIGNAL as u64 + silent,
                        &[],
                    );
                    assert!(
                        left.iter().chain(&right).all(|y| *y == 0.0),
                        "{rate} Hz: non-zero output in the silence"
                    );
                    let mut zl = vec![0.0_f32; frames];
                    let mut zr = vec![0.0_f32; frames];
                    render(fresh.as_mut(), &mut zl, &mut zr, None, silent, &[]);
                    assert_eq!(
                        rest,
                        snapshot(fresh.as_ref(), sizes),
                        "Z is not a fixed point"
                    );
                    silent += frames as u64;
                };
                let mut a = (
                    noise(SIGNAL, 0x1375_3000, 0.5),
                    noise(SIGNAL, 0x1375_3001, 0.5),
                );
                let mut b = a.clone();
                render(
                    effect.as_mut(),
                    &mut a.0,
                    &mut a.1,
                    None,
                    SIGNAL as u64 + reached,
                    &[],
                );
                render(fresh.as_mut(), &mut b.0, &mut b.1, None, reached, &[]);
                assert!(
                    a.0.iter()
                        .zip(&b.0)
                        .chain(a.1.iter().zip(&b.1))
                        .all(|(x, y)| x == y),
                    "{rate} Hz peak {peak:e} release {release} ramp {ramp}: the rested instance renders unlike Z"
                );
                eprintln!(
                    "{rate} Hz peak {peak:e} release {release} ramp {ramp}: rest after {reached} samples, bound {rest_bound}"
                );
            }
        }
    }
}
