#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the test's own f64 recomputation of the stated values.
//! Issue #1376's gates for the gate/expander: the stated zero tail, the exact-rest bound with its
//! rest state at the gain word's stall word, and the composition values, against the real kernel.
//!
//! Every launch rate in release; 48 kHz only in debug, where gate 2 runs one range. Evidence lines
//! print with `--nocapture`.
#![allow(missing_docs)]

mod support;

use effect_contract::{
    CompositionBound, EffectProcessBlock, EffectQuality, FlushStall, InitialParameterValue,
    NativeEffectFactory, NodeTailBound, ParameterChannel, PeakGain, PreparedAutomationSpan,
    PreparedNativeEffect, PreparedSidechainPort, RestBound, StatePayloadOutput, StatePayloadSizes,
    TailDecay, TailSamples,
};
use effect_runtime::envelope::attack_release_coefficient;
use gate_expander::{GATE_EXPANDER_PARAMETERS, GateExpanderFactory};
use support::{Values, initial_values, request_at, set_parameter, sidechain_port};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// Samples of signal before the silence (gates 1 and 2).
const SIGNAL: usize = 4_096;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;

const THRESHOLD: usize = 0;
const RATIO: usize = 1;
const RANGE: usize = 2;
const HYSTERESIS: usize = 3;
const ATTACK: usize = 4;
const HOLD: usize = 5;
const RELEASE: usize = 6;

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn domain(parameter: usize) -> (f32, f32) {
    let row = &GATE_EXPANDER_PARAMETERS[parameter];
    (
        row.minimum.expect("a minimum"),
        row.maximum.expect("a maximum"),
    )
}

fn stated(rate: u32) -> NodeTailBound {
    conformance::tail_bound_of(Box::new(GateExpanderFactory), rate, EffectQuality::Normal).bound()
}

fn stated_rest(rate: u32) -> u64 {
    let bound = stated(rate);
    // A fixed point, not equality with `Z` (#1376 root option (a)): an edited history can rest on
    // another stall word.
    let RestBound::FixedPoint(rest) = bound.rest else {
        panic!("{rate} Hz: no fixed-point rest bound stated");
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

/// The longest hold, in samples: `round(1000 ms * fs)`.
fn hold_max(rate: u32) -> u64 {
    (f64::from(domain(HOLD).1) * f64::from(rate) / 1000.0 + 0.5).floor() as u64
}

fn noise(samples: usize, seed: u64, scale: f32) -> Vec<f32> {
    let mut generator = conformance::SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * scale)
        .collect()
}

fn values_of(settings: &[(usize, f32)]) -> Values {
    let mut values = initial_values();
    for (parameter, value) in settings.iter().copied() {
        set_parameter(&mut values, parameter, value, value);
    }
    values
}

/// A prepared scalar instance, the sidechain connected or not.
fn prepare(
    rate: u32,
    values: &[InitialParameterValue],
    sidechain: bool,
) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let mut request = request_at(values, rate, QUANTUM as u32);
    if sidechain {
        request.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
    }
    let prepared = GateExpanderFactory.prepare(request).expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

/// Renders `left`/`right` (and the sidechain, when given) in quantum blocks from `first`.
fn render(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    sidechain: Option<(&[f32], &[f32])>,
    first: u64,
) {
    render_with(effect, left, right, sidechain, first, &[]);
}

/// [`render`], delivering each span in the block that holds its sample.
fn render_with(
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
        let block: Vec<PreparedAutomationSpan> = spans
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
                &block,
                QUANTUM as u32,
            )
            .expect("block"),
        );
        offset = end;
    }
}

/// Every section of the payload, as words (the gate has no cursor words).
fn snapshot(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Vec<u32> {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    common
        .chunks_exact(4)
        .chain(left.chunks_exact(4))
        .chain(right.chunks_exact(4))
        .map(|word| u32::from_le_bytes(word.try_into().expect("a word")))
        .collect()
}

/// `ceil(2000 log10 g)`, independently of the crate's helper.
fn millibels(g: f64) -> i32 {
    (2000.0 * math::log(g) / core::f64::consts::LN_10).ceil() as i32
}

/// **Gates 1 and 3: zero output from the first silent sample, and (N1) at every sample,** at
/// every rate, over the domain extremes (the deepest curve and range, no curve, a mid setting;
/// fastest and slowest attack and release; no hold and the longest), the sidechain unconnected or
/// connected with full-scale noise that keeps playing through the silence. The stated values are
/// `Finite(0)`, decay `0`, and D4's gain and stall recomputed here: `G_p = G_t =
/// ceil_mB(10^(1e-5 / 20) (1 + u))` (the gain word is at most 0 dB, the fast tier's proven `1e-5`
/// dB error, one rounded product) and `sigma_p = sigma_t = ceil_mB(2^-126)`.
#[test]
fn the_output_is_zero_after_the_input_and_within_the_stated_peak_gain() {
    let gain = millibels(math::db_to_gain(1.0e-5) * (1.0 + U));
    let stall = millibels(2.0_f64.powi(-126));
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
        for curve in [[-80.0, 20.0, 96.0], [0.0, 1.0, 0.0], [-40.0, 4.0, 48.0]] {
            for attack in [domain(ATTACK).0, domain(ATTACK).1] {
                for hold in [0.0, domain(HOLD).1] {
                    for release in [domain(RELEASE).0, domain(RELEASE).1] {
                        for sidechain in [false, true] {
                            index += 1;
                            let values = values_of(&[
                                (THRESHOLD, curve[0]),
                                (RATIO, curve[1]),
                                (RANGE, curve[2]),
                                (ATTACK, attack),
                                (HOLD, hold),
                                (RELEASE, release),
                            ]);
                            let seed = 0x1376_9000 + index;
                            let (mut effect, _) = prepare(rate, &values, sidechain);
                            let total = 2 * SIGNAL;
                            // Bursts, so the gate opens and closes inside the signal.
                            let envelope = |n: usize| {
                                if (n / 700).is_multiple_of(2) {
                                    1.0
                                } else {
                                    1.0e-4
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
                            let keys = (noise(total, !seed, 1.0), noise(total, !seed ^ 0xff, 1.0));
                            let keys = sidechain.then_some((keys.0.as_slice(), keys.1.as_slice()));
                            render(effect.as_mut(), &mut left, &mut right, keys, 0);
                            let limit = g_p * f64::from(peak) + sigma_p;
                            for (n, y) in left.iter().chain(&right).enumerate() {
                                assert!(
                                    f64::from(y.abs()) <= limit,
                                    "{rate} Hz {curve:?} {attack} {hold} {release} sidechain {sidechain}: |y| = {y} above {limit} (frame {n})"
                                );
                                worst = worst.max(f64::from(y.abs()) / f64::from(peak));
                            }
                            for (channel, out) in [&left, &right].into_iter().enumerate() {
                                if let Some(n) = out[SIGNAL..].iter().position(|y| *y != 0.0) {
                                    panic!(
                                        "{rate} Hz {curve:?} sidechain {sidechain} channel {channel}: output {} at {n} samples into the silence",
                                        out[SIGNAL + n]
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        eprintln!("{rate} Hz: G_p {gain} mB, sigma_p {stall} mB, largest |y| / max|x| {worst}");
    }
}

/// The helper's coefficient: the longest release's design less `2^-24`.
fn slowest_release(rate: u32) -> f32 {
    attack_release_coefficient(domain(RELEASE).1, rate) - 1.0 / 16_777_216.0
}

/// The real gain word's walk from `+0.0` toward `t`: `flush(fma(c, t - e, e))` at `f32` (two
/// roundings), counted until a step leaves it unchanged.
fn walk(c: f32, t: f32) -> (u64, f32) {
    walk_from(c, t, 0.0)
}

/// [`walk`] from the word `e`.
fn walk_from(c: f32, t: f32, e: f32) -> (u64, f32) {
    let mut e = e;
    let mut steps = 0;
    loop {
        let next = c * (t - e) + e;
        let next = if next.abs() < lane::FLUSH_EPS {
            0.0
        } else {
            next
        };
        if next == e {
            return (steps, e);
        }
        e = next;
        steps += 1;
    }
}

/// Targets `-(32 + L ulp(32))` over the derivation's three classes: on the power of two, above it
/// by fewer and more than the sure-stall distance (about `1 / (2 c)` ulps), mid-binade and just
/// below the next power.
fn targets(c: f32) -> Vec<f32> {
    let sure = (0.5 / f64::from(c)) as u32;
    let mut ls = vec![0, 1, 2, 1 << 20, 1 << 22, 3 << 21, (1 << 23) - 1];
    for step in 0..24 {
        ls.push(sure / 8 + step * sure / 12);
    }
    ls.into_iter()
        .map(|l| -f32::from_bits(32.0_f32.to_bits() + l))
        .collect()
}

/// **Gate 4: the stated rest is the longest hold plus the real gain word's longest walk**: the walk
/// from `+0.0` at the slowest release coefficient (2,000 ms, one `2^-24` step below its design)
/// toward targets across the derivation's classes, in `B` steps for the longest:
/// `B <= R - H_max <= B + max(2, ceil(B / 100))`, and every walk within the bound.
#[test]
fn the_stated_rest_is_the_hold_and_the_gain_words_walk() {
    for &rate in rates() {
        let walk_bound = stated_rest(rate) - hold_max(rate).max(64);
        let c = slowest_release(rate);
        let mut worst = (0, 0.0_f32);
        for t in targets(c) {
            let (steps, _) = walk(c, t);
            assert!(
                steps <= walk_bound,
                "{rate} Hz t = {t}: the word stalls in {steps} steps, stated {walk_bound}"
            );
            if steps > worst.0 {
                worst = (steps, t);
            }
        }
        // From below (#1376 root option (a)): the word an edit left at the -96 dB floor rises at
        // the slowest attack toward a shallow target, at the flush threshold and under it (where
        // it flushes to `+0.0`) and an ordinary one; every walk ends within the stated bound.
        let attack = attack_release_coefficient(domain(ATTACK).1, rate) - 1.0 / 16_777_216.0;
        for t in [-1.000_001e-20_f32, -1.0e-25, -3.0] {
            let (steps, word) = walk_from(attack, t, -96.0);
            assert!(
                steps <= walk_bound,
                "{rate} Hz t = {t:e}: from below the word stops in {steps} steps, stated {walk_bound}"
            );
            eprintln!("{rate} Hz: from below toward {t:e}, {steps} steps to {word:e}");
        }
        let count = worst.0;
        assert!(
            walk_bound <= count + 2.max(count.div_ceil(100)),
            "{rate} Hz: the longest walk is {count} steps (t = {}), stated {walk_bound}",
            worst.1
        );
        eprintln!(
            "{rate} Hz: R = {} (hold {} + walk {walk_bound}), longest walk {count} at t = {}",
            stated_rest(rate),
            hold_max(rate),
            worst.1
        );
    }
}

/// **Gate 2: rest by the stated `R`, real kernel: a fixed point from every history, and `Z` from
/// every history with settled parameters**, at the deepest curve, the longest hold, the fastest attack and the slowest release,
/// for a range of 96 dB and a range just above 32 dB that puts the target in the walk's slowest
/// class. Histories: full-scale noise at both peak cases (`10^(24/20)` and `1e29`: the gate open,
/// its hold loaded) and sub-threshold noise longer than the hold (the gate closed, the gain word
/// released toward a partial target first). After `R` zeros the output is `+-0.0` and the payload
/// equals, word for word, the payload of a freshly reset instance fed `R` zeros (`Z`, which is
/// itself a fixed point), and the two then render the same seeded noise to the bit. An edited
/// history (a range automation still ramping at the silence makes the target shallower than the
/// closed gate's word) is at a zero-input fixed point by `R`, below its target and not equal to `Z`.
#[test]
fn the_state_reaches_one_rest_by_the_stated_bound() {
    for &rate in rates() {
        let rest_bound = stated_rest(rate);
        let c = slowest_release(rate);
        let slow_class = 32.0 * (1.0 + (0.25 / f64::from(c)) * U * 2.0);
        for range in [96.0_f32, slow_class as f32] {
            if cfg!(debug_assertions) && range > 90.0 {
                continue;
            }
            let values = values_of(&[
                (THRESHOLD, domain(THRESHOLD).0),
                (RATIO, domain(RATIO).1),
                (RANGE, range),
                // No hysteresis, so the sub-threshold history cannot re-arm the hold.
                (HYSTERESIS, 0.0),
                (ATTACK, domain(ATTACK).0),
                (HOLD, domain(HOLD).1),
                (RELEASE, domain(RELEASE).1),
            ]);
            let zeros = |effect: &mut dyn PreparedNativeEffect, first: u64| {
                let mut left = vec![0.0_f32; rest_bound as usize];
                let mut right = vec![0.0_f32; rest_bound as usize];
                render(effect, &mut left, &mut right, None, first);
                left.iter().chain(&right).all(|y| *y == 0.0)
            };
            let (mut fresh, sizes) = prepare(rate, &values, false);
            assert!(zeros(fresh.as_mut(), 0), "{rate} Hz: Z renders non-zero");
            let rest = snapshot(fresh.as_ref(), sizes);
            // Z is a fixed point: one more block of zeros leaves it.
            let (mut l, mut r) = (vec![0.0_f32; QUANTUM], vec![0.0_f32; QUANTUM]);
            render(fresh.as_mut(), &mut l, &mut r, None, rest_bound);
            assert_eq!(
                rest,
                snapshot(fresh.as_ref(), sizes),
                "{rate} Hz: Z is not a fixed point"
            );
            // Below the -80 dB threshold, above `threshold - range / (ratio - 1)` for both ranges, so
            // the curve's targets are partial.
            let quiet = 10.0_f32.powf(-80.5 / 20.0);
            let gain_word = sizes.common_bytes as usize / 4;
            let histories = [
                ("open, +24 dBFS", SIGNAL, 10.0_f32.powf(24.0 / 20.0)),
                ("open, 1e29", SIGNAL, 1.0e29),
                ("closed, partial target", 2 * rate as usize, quiet),
            ];
            // An edited history (#1376 root option (a)): the gate is closed with its word near
            // -10 dB when a range automation toward 3 dB, still ramping at the silence, makes the
            // target shallower than the word. The word rises from below and rests on a fixed point
            // that is not `Z`.
            {
                let length = 2 * rate as usize;
                let (mut effect, _) = prepare(rate, &values, false);
                let mut left = noise(length, 0x1376_a000, quiet);
                let mut right = noise(length, 0x1376_a001, quiet);
                let spans: Vec<PreparedAutomationSpan> =
                    [ParameterChannel::Left, ParameterChannel::Right]
                        .into_iter()
                        .map(|channel| PreparedAutomationSpan {
                            channel,
                            parameter_index: RANGE as u32,
                            sample: (length - 16) as u64,
                            value: 3.0,
                        })
                        .collect();
                let split = length - 16;
                render(
                    effect.as_mut(),
                    &mut left[..split],
                    &mut right[..split],
                    None,
                    0,
                );
                render_with(
                    effect.as_mut(),
                    &mut left[split..],
                    &mut right[split..],
                    None,
                    split as u64,
                    &spans,
                );
                assert!(
                    zeros(effect.as_mut(), length as u64),
                    "{rate} Hz range {range} edited: non-zero output in the silence"
                );
                let state = snapshot(effect.as_ref(), sizes);
                let (mut l, mut r) = (vec![0.0_f32; QUANTUM], vec![0.0_f32; QUANTUM]);
                render(
                    effect.as_mut(),
                    &mut l,
                    &mut r,
                    None,
                    length as u64 + rest_bound,
                );
                assert_eq!(
                    state,
                    snapshot(effect.as_ref(), sizes),
                    "{rate} Hz range {range} edited: not at a fixed point after {rest_bound} zeros"
                );
                let word = f32::from_bits(state[gain_word]);
                assert!(
                    word < -3.0 && word != f32::from_bits(rest[gain_word]),
                    "{rate} Hz range {range} edited: the word {word} did not rest below its target"
                );
                eprintln!(
                    "{rate} Hz range {range} edited: at a fixed point ({word} dB, Z has {} dB) after {rest_bound} zeros",
                    f32::from_bits(rest[gain_word])
                );
            }
            for (name, length, peak) in histories {
                let (mut effect, _) = prepare(rate, &values, false);
                let mut left = noise(length, 0x1376_a000, peak);
                let mut right = noise(length, 0x1376_a001, peak);
                render(effect.as_mut(), &mut left, &mut right, None, 0);
                let before = f32::from_bits(snapshot(effect.as_ref(), sizes)[gain_word]);
                if peak < 1.0 {
                    let stall = f32::from_bits(rest[gain_word]);
                    assert!(
                        before < 0.0 && before != stall,
                        "{rate} Hz range {range} {name}: the word {before} is not released toward a partial target"
                    );
                }
                assert!(
                    zeros(effect.as_mut(), length as u64),
                    "{rate} Hz range {range} {name}: non-zero output in the silence"
                );
                let state = snapshot(effect.as_ref(), sizes);
                assert!(
                    state
                        .iter()
                        .zip(&rest)
                        .all(|(a, b)| f32::from_bits(*a) == f32::from_bits(*b)),
                    "{rate} Hz range {range} {name}: not at Z after {rest_bound} zeros"
                );
                let mut a = (
                    noise(SIGNAL, 0x1376_b000, 0.5),
                    noise(SIGNAL, 0x1376_b001, 0.5),
                );
                let mut b = a.clone();
                let mut z = fresh_copy(rate, &values);
                render(
                    effect.as_mut(),
                    &mut a.0,
                    &mut a.1,
                    None,
                    (length as u64) + rest_bound,
                );
                render(z.as_mut(), &mut b.0, &mut b.1, None, rest_bound);
                assert!(
                    a.0.iter()
                        .zip(&b.0)
                        .chain(a.1.iter().zip(&b.1))
                        .all(|(x, y)| x.to_bits() == y.to_bits()),
                    "{rate} Hz range {range} {name}: the rested instance renders unlike Z"
                );
                eprintln!(
                    "{rate} Hz range {range} {name}: gain word {before} dB before the silence, at Z ({} dB) after {rest_bound} zeros",
                    f32::from_bits(rest[gain_word])
                );
            }
        }
    }
}

/// A fresh instance fed the stated `R` zeros: `Z`, to render beside a rested instance.
fn fresh_copy(rate: u32, values: &[InitialParameterValue]) -> Box<dyn PreparedNativeEffect> {
    let rest_bound = stated_rest(rate) as usize;
    let (mut fresh, _) = prepare(rate, values, false);
    let mut left = vec![0.0_f32; rest_bound];
    let mut right = vec![0.0_f32; rest_bound];
    render(fresh.as_mut(), &mut left, &mut right, None, 0);
    fresh
}
