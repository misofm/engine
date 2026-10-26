#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the test's own f64 recomputation of the stated values.
//! Issue #1374's gates for the delay: the stated tail, exact-rest bound and composition values,
//! recomputed and against the real kernel. The derivation is
//! `docs/derivations/1374-delay-tail-and-rest.md`.
//!
//! Gate 1 runs every launch rate in release and 48 kHz in debug. Gate 2 renders about 40 million
//! frames per case and runs in release only. Gate 3 runs 48 kHz in both profiles. Evidence lines
//! print with `--nocapture`.
#![allow(missing_docs)]

use delay::{DELAY_PARAMETERS, DelayFactory};
use effect_contract::{
    CompositionBound, EffectProcessBlock, EffectQuality, FlushStall, InitialParameterValue,
    LinkMode, NativeEffectFactory, NodeTailBound, ParameterChannel, PeakGain, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedPorts,
    PreparedSidechainPort, RestBound, RestSamples, StatePayloadOutput, StatePayloadSizes,
    TailDecay, TailSamples,
};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;
/// `2^-126`.
const UNDERFLOW: f64 = 1.0 / 85_070_591_730_234_615_865_843_651_857_942_052_864.0;
/// `10^(-144/20)`.
const EPS: f64 = 6.309_573_444_801_932e-8;
/// `lane::FLUSH_EPS`.
const FLUSH: f32 = 1.0e-20;

const TIME: usize = 0;
const FEEDBACK: usize = 1;
const DAMPING: usize = 2;
const CROSS: usize = 4;

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn stated(rate: u32) -> NodeTailBound {
    conformance::tail_bound_of(Box::new(DelayFactory), rate, EffectQuality::Normal).bound()
}

/// Every lane at `time` ms, `feedback`, `damping` and `mix`; the matrix at `cross`.
fn values(
    time: f32,
    feedback: f32,
    damping: f32,
    mix: f32,
    cross: f32,
) -> [InitialParameterValue; 9] {
    let per_lane = [time, feedback, damping, mix];
    core::array::from_fn(|index| {
        if index == 8 {
            return InitialParameterValue {
                parameter_index: CROSS as u32,
                channel: ParameterChannel::Both,
                value: cross,
            };
        }
        InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: per_lane[index / 2],
        }
    })
}

fn prepare(
    rate: u32,
    values: &[InitialParameterValue],
) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let prepared = DelayFactory
        .prepare(PrepareEffectRequest {
            sample_rate: rate,
            quantum: QUANTUM as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 24,
                maximum_scratch_bytes: 1 << 16,
                maximum_automation_spans_per_block: 16,
            },
            tail_bound: conformance::tail_bound_of(
                Box::new(DelayFactory),
                rate,
                EffectQuality::Normal,
            ),
        })
        .expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

/// Renders `left`/`right` in quantum blocks from sample `first`; `spans` apply at their sample,
/// which must be a block start.
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
                QUANTUM as u32,
            )
            .expect("block"),
        );
        assert_eq!(report.invalid_spans, 0, "every span is legal");
        offset = end;
    }
}

/// `1 + k u`.
fn grow(k: f64) -> f64 {
    1.0 + k * U
}

/// The stated values, re-derived from the descriptor's domain bounds, the rate and the derivation's
/// formulas, independently of the crate's code: `(T, D, G, sigma, R(+24 dBFS), R(1e30), R(P*))`,
/// with `G` and `sigma` linear. Every `f64` value is rounded by `2^-24` toward its bound.
fn recomputed(rate: u32, delay_max: u32) -> (u64, u64, f64, f64, u64, u64, u64) {
    let slack = |x: f64| x * (1.0 + U);
    let shrink = |x: f64| x * (1.0 - U);
    let ceil = |x: f64| {
        assert!(x.is_finite() && x >= 0.0);
        x.ceil() as u64
    };
    let samples = |x: f64, y: f64, nu: f64| {
        if x < y {
            0
        } else {
            ceil(slack(slack(math::log(slack(x / y))) / nu))
        }
    };
    let f_max = f64::from(DELAY_PARAMETERS[FEEDBACK].maximum.unwrap())
        .max(-f64::from(DELAY_PARAMETERS[FEEDBACK].minimum.unwrap()));
    // The slowest non-zero damping design: `c = 0.995` at its 48 kHz cutoff, re-designed at `rate`.
    let cutoff = -math::log(f64::from(DELAY_PARAMETERS[DAMPING].maximum.unwrap())) * 48_000.0
        / (2.0 * core::f64::consts::PI);
    let big_g = math::tan(core::f64::consts::PI * cutoff / f64::from(rate));
    let g_lo = f64::from((big_g / (1.0 + big_g)) as f32) * (1.0 - 1.0 / 1_048_576.0);
    // (N1), the hull: W = ((1 + u) X + 16 U) / (1 - f kappa), S_b = ((1 + 6u) W + 7 U) / (1 - 7u).
    let kappa = slack(grow(1.0).powi(5) * grow(3.0) * grow(8.0) * grow(6.0));
    let hull = shrink(1.0 - slack(f_max * kappa));
    let state = shrink(1.0 - 7.0 * U);
    let g_p = slack(grow(6.0) * grow(6.0) * grow(1.0) / hull);
    let sigma_p = slack((grow(6.0) * grow(6.0) * 16.0 / hull + grow(6.0) * 3.0 + 3.0) * UNDERFLOW);
    let c_rel = slack(grow(6.0) * grow(1.0) / (hull * state));
    let c_abs = slack((grow(6.0) * 16.0 / hull + 7.0) / state) * UNDERFLOW;
    // The settled loop: F(r) = A r^-D (beta + alpha b / (r - a)) <= 1, bisected to adjacent f64s.
    let (a, b) = (1.0 - 2.0 * g_lo + 7.0 * U, 2.0 * g_lo + 7.0 * U);
    let (alpha, beta) = (1.0 - g_lo + 3.0 * U, g_lo + 3.0 * U);
    let gain = slack(f_max * grow(1.0).powi(5));
    let d = f64::from(delay_max);
    let f =
        |r: f64| slack(gain * math::exp(-d * math::log(r)) * (beta + alpha * b / shrink(r - a)));
    assert!(f(1.0) < 1.0, "{rate} Hz: the settled loop contracts");
    let (mut lo, mut hi) = (a, 1.0_f64);
    loop {
        let mid = lo + (hi - lo) / 2.0;
        if mid <= lo || mid >= hi {
            break;
        }
        if f(mid) <= 1.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let nu = shrink(-math::log(hi));
    let loop_gain =
        slack(1.0 / shrink(1.0 - slack(gain * slack(beta + alpha * b / shrink(1.0 - a)))));
    let w_u = slack(slack(8.0 + 4.0 * gain * alpha / shrink(1.0 - a)) * loop_gain) * UNDERFLOW;
    let s_u = slack((b * w_u + 4.0 * UNDERFLOW) / shrink(1.0 - a));
    let g_t = slack(grow(6.0) * grow(1.0) * loop_gain);
    let sigma_t = slack(grow(6.0) * (c_abs + w_u) + 3.0 * UNDERFLOW);
    let settle = 256 + u64::from(delay_max);
    let tail = settle + samples(c_rel, EPS / 2.0, nu);
    let decay = samples(10.0, 1.0, nu);
    let floor_peak = slack(2.0 * (c_abs + w_u) / EPS);
    // Rest: the writes rest below FLUSH_EPS - W_U; the damping state then walks from at most
    // `FLUSH_EPS b / (r - a) + S_U` at the follower rate `2 g_lo - 3u`.
    let headroom = shrink(f64::from(FLUSH) - w_u);
    let start = slack(f64::from(FLUSH) * slack(b / shrink(hi - a)) + s_u);
    let mut start_f32 = start as f32;
    if f64::from(start_f32) < start {
        start_f32 = start_f32.next_up();
    }
    let target = shrink(2.0 * g_lo - 3.0 * U);
    let mut walk_rate = target as f32;
    if f64::from(walk_rate) > target {
        walk_rate = walk_rate.next_down();
    }
    let walk = effect_runtime::tail::rate_follower_rest_samples(
        walk_rate,
        start_f32,
        effect_runtime::tail::RestSnap::Below(FLUSH),
    )
    .expect("the damping state rests");
    let ring_words = 2 * u64::from(rate) + 3;
    let rest = |peak: f64| {
        256 + samples(slack(c_rel * peak + c_abs), headroom, nu)
            + ring_words.max(u64::from(delay_max) + walk)
    };
    (
        tail,
        decay,
        g_p.max(g_t),
        sigma_p.max(sigma_t),
        rest(slack(math::db_to_gain(24.0))),
        rest(f64::from(1.0e30_f32)),
        rest(floor_peak),
    )
}

/// `ceil(2000 log10 g)` with the `2^-30` margins of `math::tail::ceil_millibels` for a computed
/// value.
fn millibels(g: f64) -> i32 {
    let margin = 1.0 / 1_073_741_824.0;
    let x = 2000.0 * math::log(g) / core::f64::consts::LN_10;
    (x + x.abs() * margin + margin).ceil() as i32
}

/// **Gate 1: the stated values are the derivation's, recomputed** at every rate (48 kHz in debug)
/// from the descriptor's domain bounds and the rate, in integers and millibels.
#[test]
fn the_stated_bound_is_the_derivations() {
    for &rate in rates() {
        let (tail, decay, gain, stall, rest_24, rest_any, rest_floor) = recomputed(rate, 2 * rate);
        let gain = PeakGain::Millibels(millibels(gain));
        let stall = FlushStall::Level(millibels(stall));
        let expected = NodeTailBound {
            tail: TailSamples::Finite(tail),
            tail_every_peak: TailSamples::Finite(tail.max(rest_floor)),
            rest: RestBound::Bounded(RestSamples {
                peak_plus_24_dbfs: rest_24,
                any_sanitized_input: rest_any,
            }),
            composition: CompositionBound::Stated {
                decay: TailDecay(decay),
                peak_gain: gain,
                tail_gain: gain,
                peak_stall: stall,
                tail_stall: stall,
            },
        };
        assert_eq!(stated(rate), expected, "{rate} Hz");
        assert_eq!(delay::tail_bound_at(rate, 2 * rate), expected, "{rate} Hz");
        eprintln!("{rate} Hz: {expected:?}");
    }
}

/// **Gate 2: soundness against the real kernel** at 48 kHz: both taps at 2,000 ms, feedback 0.95,
/// mix 1, cross feedback 0 and 1, damping 0 and 0.995, fed the in-phase constant `P = 1` on both
/// lanes for 60 s (the case that drives the ring toward D1's `20 P`), then zeros to the stated
/// `T`. Every output sample is within (N1)'s `G_p P + sigma_p`, and every sample of the last 2 s
/// before `T` is below `eps P`. A fifth run starts at feedback 0.5 and 1,000 ms, and has a 2,000 ms
/// time change (a crossfade) and a feedback ramp to 0.95 both in flight when the input stops.
#[test]
#[cfg_attr(debug_assertions, ignore = "about 200 million frames: release only")]
fn the_kernel_decays_within_the_stated_tail() {
    let rate = 48_000;
    let bound = stated(rate);
    let TailSamples::Finite(tail) = bound.tail else {
        panic!("a finite tail");
    };
    let (PeakGain::Millibels(gain), FlushStall::Level(stall)) =
        bound.composition.peak_clause().expect("stated")
    else {
        panic!("a finite gain and stall");
    };
    let limit = 10.0_f64.powf(f64::from(gain) / 2000.0) + 10.0_f64.powf(f64::from(stall) / 2000.0);
    let fill = 60 * rate as usize;
    let window = 2 * rate as u64;
    let (feedback_max, damping_max) = (
        DELAY_PARAMETERS[FEEDBACK].maximum.unwrap(),
        DELAY_PARAMETERS[DAMPING].maximum.unwrap(),
    );
    let mut cases: Vec<(f32, f32, bool)> = Vec::new();
    for cross in [0.0, 1.0] {
        for damping in [0.0, damping_max] {
            cases.push((cross, damping, false));
        }
    }
    cases.push((1.0, damping_max, true));
    for (cross, damping, in_flight) in cases {
        let (time, feedback) = if in_flight {
            (1_000.0, 0.5)
        } else {
            (2_000.0, feedback_max)
        };
        let (mut effect, _) = prepare(rate, &values(time, feedback, damping, 1.0, cross));
        let split = fill - 32;
        let spans: Vec<PreparedAutomationSpan> = if in_flight {
            // Slot order: the time of both lanes, then the feedback of both.
            [(TIME, 2_000.0), (FEEDBACK, feedback_max)]
                .into_iter()
                .flat_map(|(parameter, value)| {
                    [ParameterChannel::Left, ParameterChannel::Right].map(|channel| {
                        PreparedAutomationSpan {
                            channel,
                            parameter_index: parameter as u32,
                            sample: split as u64,
                            value,
                        }
                    })
                })
                .collect()
        } else {
            Vec::new()
        };
        let mut worst = 0.0_f32;
        let mut check = |out: &[f32], first: u64| {
            for (n, y) in out.iter().enumerate() {
                assert!(
                    f64::from(y.abs()) <= limit,
                    "cross {cross} damping {damping} in flight {in_flight}: |y| = {y} above {limit} at {}",
                    first + n as u64
                );
                worst = worst.max(y.abs());
            }
        };
        let mut left = vec![1.0_f32; fill];
        let mut right = vec![1.0_f32; fill];
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
        check(&left, 0);
        check(&right, 0);
        // Zeros from N = `fill` to N + T, in chunks; the last sample at or above `eps P`.
        let mut last_loud = 0_u64;
        let mut silent = 0_u64;
        let chunk = 64 * QUANTUM;
        let mut left = vec![0.0_f32; chunk];
        let mut right = vec![0.0_f32; chunk];
        while silent < tail {
            let frames = (chunk as u64).min(tail - silent) as usize;
            left[..frames].fill(0.0);
            right[..frames].fill(0.0);
            render(
                effect.as_mut(),
                &mut left[..frames],
                &mut right[..frames],
                fill as u64 + silent,
                &[],
            );
            check(&left[..frames], fill as u64 + silent);
            check(&right[..frames], fill as u64 + silent);
            for n in 0..frames {
                if f64::from(left[n].abs().max(right[n].abs())) >= EPS {
                    last_loud = silent + n as u64;
                }
            }
            silent += frames as u64;
        }
        assert!(
            last_loud + window < tail,
            "cross {cross} damping {damping} in flight {in_flight}: |y| >= eps at {last_loud} samples into the silence, inside the last 2 s before T = {tail}"
        );
        eprintln!(
            "48000 Hz cross {cross} damping {damping} in flight {in_flight}: largest |y| {worst} (G_p P + sigma_p = {limit}); last |y| >= eps at {last_loud}, T = {tail}"
        );
    }
}

/// The signal-state words of a payload: each lane's damping state (word 0) and its ring (the last
/// `ring_words` words of the lane section). Cursors, tap geometry, counters and ramps are not
/// signal state (#1374 D2).
fn signal_words(
    effect: &dyn PreparedNativeEffect,
    sizes: StatePayloadSizes,
    ring: usize,
) -> Vec<f32> {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    let mut words = Vec::new();
    for lane in [&left, &right] {
        let lane: Vec<f32> = lane
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| f32::from_le_bytes(*word))
            .collect();
        words.push(lane[0]);
        words.extend_from_slice(&lane[lane.len() - ring..]);
    }
    words
}

/// **Gate 3: exact rest by the stated bound, real kernel**, at 48 kHz with both taps at 1 ms
/// (`D = 48` samples), so the bound is [`delay::tail_bound_at`] at `D = 48` (the function the
/// descriptor evaluates at `2 fs`): the in-phase constant `P = 10^(24/20)` for 2 s at feedback
/// 0.95, mix 1, cross feedback 0 and 1 (each lane's own ring, and the ping-pong through the other
/// lane's), damping 0 and 0.995, then zeros. By the bound the output is
/// `+-0.0` and every ring word and damping state equals a freshly prepared delay's (`+0.0`) under
/// `==`, and one more block leaves them there. At damping 0.995, the slowest design, the state is
/// not yet at rest at 80 % of the bound.
#[test]
fn the_state_rests_by_the_stated_bound() {
    let rate = 48_000;
    let ring = 2 * rate as usize + 3;
    let RestBound::Bounded(RestSamples {
        peak_plus_24_dbfs: bound,
        ..
    }) = delay::tail_bound_at(rate, 48).rest
    else {
        panic!("a bounded rest");
    };
    let peak = 10.0_f32.powf(24.0 / 20.0);
    let damping_max = DELAY_PARAMETERS[DAMPING].maximum.unwrap();
    let feedback_max = DELAY_PARAMETERS[FEEDBACK].maximum.unwrap();
    for (cross, damping) in [
        (0.0, 0.0),
        (0.0, damping_max),
        (1.0, 0.0),
        (1.0, damping_max),
    ] {
        let parameters = values(1.0, feedback_max, damping, 1.0, cross);
        let (mut effect, sizes) = prepare(rate, &parameters);
        let (fresh, _) = prepare(rate, &parameters);
        let rest = signal_words(fresh.as_ref(), sizes, ring);
        assert!(rest.iter().all(|word| word.to_bits() == 0), "Z is +0.0");
        let fill = 2 * rate as usize;
        let mut left = vec![peak; fill];
        let mut right = vec![peak; fill];
        render(effect.as_mut(), &mut left, &mut right, 0, &[]);
        let at_rest = |effect: &dyn PreparedNativeEffect| {
            signal_words(effect, sizes, ring)
                .iter()
                .zip(&rest)
                .all(|(a, b)| a == b)
        };
        let mut silent = 0_u64;
        let mut reached = None;
        let probe = bound * 4 / 5;
        while silent <= bound {
            if reached.is_none() && at_rest(effect.as_ref()) {
                reached = Some(silent);
            }
            if silent == probe && damping == damping_max {
                assert!(
                    !at_rest(effect.as_ref()),
                    "cross {cross} damping {damping}: already at rest at 80 % of the bound {bound}: the gate is vacuous"
                );
            }
            if silent == bound {
                break;
            }
            let next = if silent < probe { probe } else { bound };
            let frames = (QUANTUM as u64).min(next - silent) as usize;
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
        }
        assert!(
            at_rest(effect.as_ref()),
            "cross {cross} damping {damping}: not at rest after the bound {bound}"
        );
        let mut left = vec![0.0_f32; QUANTUM];
        let mut right = vec![0.0_f32; QUANTUM];
        render(
            effect.as_mut(),
            &mut left,
            &mut right,
            fill as u64 + bound,
            &[],
        );
        assert!(
            left.iter().chain(&right).all(|y| *y == 0.0),
            "cross {cross} damping {damping}: non-zero output after the bound"
        );
        assert!(
            at_rest(effect.as_ref()),
            "cross {cross} damping {damping}: the rest state moved"
        );
        eprintln!(
            "48000 Hz D = 48 cross {cross} damping {damping}: at rest after {} samples, bound {bound}",
            reached.expect("reached")
        );
    }
}
