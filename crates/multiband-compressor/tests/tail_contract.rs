#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the test's own f64 impulse response of the designed words.
//! Issue #1373 tranche b: the multiband compressor's stated tail and exact-rest bound
//! (`docs/derivations/1373-multiband-compressor-tail-and-rest.md`, part 2), against its own
//! recompute, the real kernel and the smoother word's brute-force count.
//!
//! Every launch rate in release; 48 kHz only in debug, where the kernel run takes the deeper start
//! (`1e29`) alone. Evidence lines print with `--nocapture`.
#![allow(missing_docs)]

mod support;

use effect_contract::{
    EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory, NodeTailBound,
    ParameterChannel, PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort, RestBound, StatePayloadSizes, TailSamples,
};
use multiband_compressor::tail::{
    multiband_live_bound, multiband_live_bound_table, smoother_rest_samples,
};
use multiband_compressor::{
    MULTIBAND_COMPRESSOR_DESCRIPTOR, MULTIBAND_COMPRESSOR_PARAMETERS, MultibandCompressorFactory,
    design_lr4_words,
};
use support::{PARAMETER_COUNT, point, process, signal, snapshot};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// Frames of worst-sign input before `N` (spec gate 2).
const LOUD: usize = 1_000_000;
/// Frames before `N` at which the glide starts: in flight at `N`.
const LEAD: usize = 32;

const CROSSOVER: usize = 0;

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn minimum(parameter: usize) -> f32 {
    MULTIBAND_COMPRESSOR_PARAMETERS[parameter]
        .minimum
        .expect("a minimum")
}

fn maximum(parameter: usize) -> f32 {
    MULTIBAND_COMPRESSOR_PARAMETERS[parameter]
        .maximum
        .expect("a maximum")
}

/// T1: the pinned bound the descriptor states is the live recompute at every launch rate, all four
/// fields, and nothing is stated off the launch rates.
#[test]
fn the_pinned_bound_is_the_computed_live_bound_at_every_launch_rate() {
    for &rate in rates() {
        let computed = multiband_live_bound(rate);
        assert_eq!(multiband_live_bound_table(rate), computed, "{rate} Hz");
        assert_eq!(
            (MULTIBAND_COMPRESSOR_DESCRIPTOR.tail_and_rest)(rate, EffectQuality::Normal),
            computed.expect("a launch rate"),
            "{rate} Hz"
        );
        assert!(
            matches!(computed, Some(bound) if bound.tail != TailSamples::Infinite),
            "{rate} Hz: {computed:?}"
        );
    }
    for rate in [8_000, 32_000, 176_400, 192_000] {
        assert_eq!(multiband_live_bound_table(rate), None, "{rate} Hz");
        assert_eq!(multiband_live_bound(rate), None, "{rate} Hz");
    }
}

/// The deepest curve, the fastest attack, the slowest release and the largest makeup on both bands
/// and both channels, at `crossover`.
fn worst_values(crossover: f32) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    let mut row = [0.0_f32; PARAMETER_COUNT];
    row[CROSSOVER] = crossover;
    for band in [1, 6] {
        row[band] = minimum(band);
        row[band + 1] = maximum(band + 1);
        row[band + 2] = minimum(band + 2);
        row[band + 3] = maximum(band + 3);
        row[band + 4] = maximum(band + 4);
    }
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: row[index / 2],
    })
}

fn prepare(
    rate: u32,
    values: &[InitialParameterValue],
) -> (Box<dyn PreparedNativeEffect>, StatePayloadSizes) {
    let prepared = MultibandCompressorFactory
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
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: 32,
            },
            tail_bound: conformance::tail_bound_of(
                Box::new(MultibandCompressorFactory),
                rate,
                EffectQuality::Normal,
            ),
        })
        .expect("prepare");
    (prepared.processor, prepared.metadata.state_sizes)
}

/// The `f64` impulse response of the split's band sum `low + high = ap1` (the first stage's
/// all-pass tap) at the designed `f32` words: `h[0] = d`, `h[j] = c A^(j-1) b`.
fn band_sum_response(words: [f32; 3], frames: usize) -> Vec<f64> {
    let [c1, a2, a3] = words.map(f64::from);
    let m1 = -2.0 * f64::from(core::f64::consts::SQRT_2 as f32);
    let a = [[1.0 - 2.0 * c1, -2.0 * a2], [2.0 * a2, 1.0 - 2.0 * a3]];
    let c = [m1 * (1.0 - c1), -m1 * a2];
    let mut response = vec![1.0 + m1 * a2];
    let mut s = [2.0 * a2, 2.0 * a3];
    for _ in 1..frames {
        response.push(c[0] * s[0] + c[1] * s[1]);
        s = [
            a[0][0] * s[0] + a[0][1] * s[1],
            a[1][0] * s[0] + a[1][1] * s[1],
        ];
    }
    response
}

/// Renders `frames` frames of `input` (both channels) in blocks of at most `block`, from sample
/// `first`, and returns the left and right outputs.
fn render(
    effect: &mut dyn PreparedNativeEffect,
    input: impl Fn(usize) -> f32,
    first: usize,
    frames: usize,
    block: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut out = (Vec::with_capacity(frames), Vec::with_capacity(frames));
    let mut done = 0;
    while done < frames {
        let length = block.min(frames - done);
        let mut left: Vec<f32> = (done..done + length).map(&input).collect();
        let mut right = left.clone();
        process(
            effect,
            &mut left,
            &mut right,
            (first + done) as u64,
            &[],
            QUANTUM as u32,
        );
        out.0.extend_from_slice(&left);
        out.1.extend_from_slice(&right);
        done += length;
    }
    out
}

fn stated(rate: u32) -> (u64, [u64; 2]) {
    let bound: NodeTailBound = conformance::tail_bound_of(
        Box::new(MultibandCompressorFactory),
        rate,
        EffectQuality::Normal,
    )
    .bound();
    let TailSamples::Finite(tail) = bound.tail else {
        panic!("{rate} Hz: no finite tail stated");
    };
    let RestBound::Bounded(rest) = bound.rest else {
        panic!("{rate} Hz: no exact-rest bound stated");
    };
    (tail, [rest.peak_plus_24_dbfs, rest.any_sanitized_input])
}

/// T2: the real kernel's worst case. The slowest crossover (80 Hz), the deepest curve with the
/// fastest attack, the slowest release and +24 dB makeup on both bands, and a worst-sign input of
/// peak `P` over the last 1,000,000 frames before `N` (the signs of the band sum's response, aimed
/// at `N + T`); settled at 80 Hz, and with a crossover glide 8 kHz -> 80 Hz started 32 frames
/// before `N`. For `P = 10^(24/20)` and `1e29`: `|y| < P eps` on both channels from `N + T`; at
/// `N + R(P)` the state payload equals, word for word, that of a fresh instance with the same
/// parameters fed the same zeros (`Z`); and then both render the same noise to the bit.
#[test]
fn the_kernel_meets_the_stated_tail_and_rests_by_the_stated_bound() {
    for &rate in rates() {
        let (tail, rests) = stated(rate);
        let slowest = design_lr4_words(rate, minimum(CROSSOVER));
        let response = band_sum_response(slowest, tail as usize + 1 + LOUD);
        let signs: Vec<f32> = response[tail as usize + 1..]
            .iter()
            .map(|h| if *h < 0.0 { -1.0 } else { 1.0 })
            .collect();
        for (peak, rest) in [(10.0_f32.powf(24.0 / 20.0), rests[0]), (1.0e29, rests[1])] {
            if cfg!(debug_assertions) && peak < 1.0e28 {
                continue;
            }
            // `Z` and the reference noise: a fresh instance with the settled parameters fed zeros
            // from `N` to `N + R(P)`, then the noise. It does not depend on the glide, so both
            // glides compare against one render of it (#1972).
            let noise = signal(4 * QUANTUM, u64::from(rate));
            let after = LOUD + rest as usize;
            let (z, reference) = {
                let (mut fresh, fresh_sizes) = prepare(rate, &worst_values(minimum(CROSSOVER)));
                render(fresh.as_mut(), |_| 0.0, LOUD, rest as usize, QUANTUM);
                let z = snapshot(fresh.as_ref(), fresh_sizes);
                let reference = render(fresh.as_mut(), |n| noise[n], after, noise.len(), QUANTUM);
                (z, reference)
            };
            for glide in [false, true] {
                let start = if glide {
                    maximum(CROSSOVER)
                } else {
                    minimum(CROSSOVER)
                };
                let (mut effect, sizes) = prepare(rate, &worst_values(start));
                let loud = |n: usize| peak * signs[LOUD - 1 - n];
                let lead = LOUD - LEAD;
                render(effect.as_mut(), loud, 0, lead, QUANTUM);
                if glide {
                    let spans = [
                        point(0, ParameterChannel::Left, lead as u64, minimum(CROSSOVER)),
                        point(0, ParameterChannel::Right, lead as u64, minimum(CROSSOVER)),
                    ];
                    let mut left: Vec<f32> = (lead..LOUD).map(loud).collect();
                    let mut right = left.clone();
                    process(
                        effect.as_mut(),
                        &mut left,
                        &mut right,
                        lead as u64,
                        &spans,
                        QUANTUM as u32,
                    );
                } else {
                    render(effect.as_mut(), |n| loud(lead + n), lead, LEAD, QUANTUM);
                }
                let (left, right) = render(effect.as_mut(), |_| 0.0, LOUD, rest as usize, QUANTUM);
                let floor = f64::from(peak) * math::tail::TAIL_FLOOR;
                let last_loud = left
                    .iter()
                    .zip(&right)
                    .rposition(|(l, r)| f64::from(l.abs().max(r.abs())) >= floor);
                assert!(
                    last_loud.is_none_or(|frame| (frame as u64) < tail),
                    "{rate} Hz, P = {peak:e}, glide {glide}: |y| >= P eps at N + {last_loud:?}, T = {tail}"
                );
                assert_eq!(
                    snapshot(effect.as_ref(), sizes),
                    z,
                    "{rate} Hz, P = {peak:e}, glide {glide}: not at Z by N + {rest}"
                );
                let rendered = render(effect.as_mut(), |n| noise[n], after, noise.len(), QUANTUM);
                let bits = |out: &(Vec<f32>, Vec<f32>)| -> Vec<u32> {
                    out.0.iter().chain(&out.1).map(|y| y.to_bits()).collect()
                };
                assert_eq!(
                    bits(&rendered),
                    bits(&reference),
                    "{rate} Hz, P = {peak:e}, glide {glide}"
                );
                eprintln!(
                    "{rate} Hz, P = {peak:e}, glide {glide}: last |y| >= P eps at N + {last_loud:?} (T = {tail}), Z by N + {rest}"
                );
            }
        }
    }
}

/// T3: the smoother word's stated rest against its brute-force count. The word
/// `flush(fl(fl(c y) + 0))` from `-100` dB at the slowest release's own coefficient reaches `+0.0`
/// in `B` steps; the stated count `R_s`, at the largest retention `smoother_retention_max` scans
/// (at every launch rate that is the slowest design's coefficient itself, so `R_s = B`), satisfies
/// `B <= R_s <= B + B / 20`. The 5 % allowance would also accept a retention one ulp high
/// (about 2.7 % at 88.2 kHz): sound, only looser.
///
/// The scan's premise, `|expf(x) - exp(x)| <= 2^-24` on `[1/2, 1)`, is checked exhaustively in
/// release over every `f32` argument a legal release time gives at a launch rate
/// (`x = -1 / fl(fl(t 0.001) fs)`, `t` in `[5, 5000]` ms), against the `f64` `exp`; debug checks the
/// 48 kHz scan window alone.
#[test]
fn the_smoother_rest_is_the_word_count_from_its_largest_distance() {
    let argument = |ms: f32, rate: u32| -1.0 / (ms * 0.001 * rate as f32);
    let (lowest, highest) = if cfg!(debug_assertions) {
        let top = argument(maximum(4), 48_000);
        (f64::from(top) - 1.0 / 2_097_152.0, top)
    } else {
        (
            f64::from(argument(minimum(4), 44_100)),
            argument(maximum(4), 96_000),
        )
    };
    let mut x = highest;
    let mut checked = 0_u64;
    while f64::from(x) >= lowest {
        let error = (f64::from(math::expf(x)) - math::exp(f64::from(x))).abs();
        assert!(error <= 1.0 / 16_777_216.0, "expf({x:e}) errs by {error:e}");
        checked += 1;
        x = f32::from_bits(x.to_bits() + 1);
    }
    eprintln!("expf within 2^-24 of exp at {checked} arguments");
    for &rate in rates() {
        let c = effect_runtime::envelope::retention_coefficient(maximum(4), rate);
        let mut y = -100.0_f32;
        let mut count = 0_u64;
        while y.to_bits() != 0 {
            // `branching_smooth` at a `+0.0` target: the release arm, `Lane::fma`'s two roundings
            // (Rust never fuses `a * b + c`), then `lane::flush`.
            let next = c * (y - 0.0) + 0.0;
            y = if next.abs() < lane::FLUSH_EPS {
                0.0
            } else {
                next
            };
            count += 1;
        }
        let stated = smoother_rest_samples(rate).expect("a bound");
        assert!(
            count <= stated && stated <= count + count / 20,
            "{rate} Hz: the word rests in {count} steps, stated {stated}"
        );
        eprintln!("{rate} Hz: R_s = {stated}, brute force {count}");
    }
}
