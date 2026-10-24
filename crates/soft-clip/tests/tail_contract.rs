#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the test's own f64 recomputation of the stated values.
//! Issue #1376's gates for the soft clip: the stated tail, the exact-rest bound and the
//! composition values, against the real kernel.
//!
//! The soft clip's design is the same at every rate; release runs every launch rate, debug
//! 48 kHz. Evidence lines print with `--nocapture`.
#![allow(missing_docs)]

mod support;

use effect_contract::{
    BankWidth, CompositionBound, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    FlushStall, InitialParameterValue, LinkMode, NativeEffectFactory, NodeTailBound, PeakGain,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect,
    PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort, RestBound, RestSamples,
    TailDecay, TailSamples,
};
use lane::Backend;
use lane::kernels::halfband::{HALFBAND63_EVEN, HALFBAND63_EVEN_TAPS, HALFBAND63_LIVE_ROWS};
use soft_clip::{SOFT_CLIP_DESCRIPTOR, SoftClipFactory};
use support::{snapshot_bank, total_state_bytes, values_from};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTUM: usize = 1_024;
/// Samples of signal before the silence (gates 1 and 2).
const SIGNAL: usize = 4_096;
/// `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn stated(rate: u32) -> NodeTailBound {
    conformance::tail_bound_of(Box::new(SoftClipFactory), rate, EffectQuality::Normal).bound()
}

/// The declared latency, `31`.
fn latency() -> usize {
    SOFT_CLIP_DESCRIPTOR.qualities[1].latency.0 as usize
}

fn noise(samples: usize, seed: u64, scale: f32) -> Vec<f32> {
    let mut generator = conformance::SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * scale)
        .collect()
}

fn request(values: &[InitialParameterValue], rate: u32, bypass: bool) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: rate,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: total_state_bytes(),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(SoftClipFactory),
            rate,
            EffectQuality::Normal,
        ),
    }
}

fn prepare(
    rate: u32,
    values: &[InitialParameterValue],
    bypass: bool,
) -> Box<dyn PreparedNativeEffect> {
    SoftClipFactory
        .prepare(request(values, rate, bypass))
        .expect("prepare")
        .processor
}

/// Renders `left`/`right` in quantum blocks.
fn render(effect: &mut dyn PreparedNativeEffect, left: &mut [f32], right: &mut [f32]) {
    let mut offset = 0;
    while offset < left.len() {
        let end = (offset + QUANTUM).min(left.len());
        let _ = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                &[],
                QUANTUM as u32,
            )
            .expect("block"),
        );
        offset = end;
    }
}

/// `ceil(2000 log10 g)`, independently of the crate's helper.
fn millibels(g: f64) -> i32 {
    (2000.0 * math::log(g) / core::f64::consts::LN_10).ceil() as i32
}

/// The domain extremes gates 1 and 3 run, as `(drive dB, output dB, mix)`.
fn extremes() -> Vec<[f32; 3]> {
    let mut cases = Vec::new();
    for drive in [-24.0, 0.0, 36.0] {
        for output in [-24.0, 24.0] {
            for mix in [0.0, 0.5, 1.0] {
                cases.push([drive, output, mix]);
            }
        }
    }
    cases
}

/// **Gates 1, 3 and the tail recompute: zero output from `latency + T`, non-zero at
/// `latency + T - 1` for some input, and (N1) at every sample,** over the domain extremes
/// (drive -24, 0 and 36 dB, output -24 and 24 dB, `mix` 0, 0.5 and 1, bypassed and not). The
/// stated values are recomputed here: `T = 2 * 30 - 31 = 29` from the half-band's 30 even taps
/// read twice (interpolator and decimator) and the latency; `R = 30 + 32 - 31 = 31` from the last
/// non-zero `e` push and the 32-row ring; `G_p = G_t = ceil_mB(g_out max(1, 2 g_drive (l1e^2 + 1/4))
/// (1 + u)^129)` and `sigma = ceil_mB(129 2^-126 g_out max(1, l1e + 1/2)^2 (1 + u)^129)`, at the
/// largest converted drive and output words.
#[test]
fn the_output_is_zero_after_the_tail_and_within_the_stated_peak_gain() {
    let taps = HALFBAND63_EVEN_TAPS;
    let tail = (2 * taps).max(latency()) - latency();
    let rest = (taps + HALFBAND63_LIVE_ROWS) - latency();
    let l1e: f64 = HALFBAND63_EVEN.iter().map(|h| f64::from(h.abs())).sum();
    let drive = f64::from(math::db_to_gain_f32(36.0));
    let output = f64::from(math::db_to_gain_f32(24.0));
    let rounding = (1.0 + U).powi(129);
    let gain = millibels(output * (2.0 * drive * (l1e * l1e + 0.25)).max(1.0) * rounding);
    let stage = (l1e + 0.5).max(1.0);
    let stall = millibels(129.0 * 2.0_f64.powi(-126) * output * stage * stage * rounding);
    for &rate in rates() {
        assert_eq!(
            stated(rate),
            NodeTailBound {
                tail: TailSamples::Finite(tail as u64),
                tail_every_peak: TailSamples::Finite(tail.max(rest) as u64),
                rest: RestBound::Bounded(RestSamples {
                    peak_plus_24_dbfs: rest as u64,
                    any_sanitized_input: rest as u64,
                }),
                composition: CompositionBound::Stated {
                    decay: TailDecay(0),
                    peak_gain: PeakGain::Millibels(gain),
                    tail_gain: PeakGain::Millibels(gain),
                    peak_stall: FlushStall::Level(stall),
                    tail_stall: FlushStall::Level(stall),
                },
            },
            "{rate} Hz"
        );
        let g_p = 10.0_f64.powf(f64::from(gain) / 2000.0);
        let sigma_p = 10.0_f64.powf(f64::from(stall) / 2000.0);
        let quiet = latency() + tail;
        let (mut worst, mut last_live) = (0.0_f64, false);
        for (index, case) in extremes().into_iter().enumerate() {
            for bypass in [false, true] {
                let seed = 0x1376_5000 + 2 * index as u64 + u64::from(bypass);
                let values =
                    values_from([(case[0], case[0]), (case[1], case[1]), (case[2], case[2])]);
                let mut effect = prepare(rate, &values, bypass);
                let total = 2 * SIGNAL;
                let mut left = noise(total, seed, 1.0);
                let mut right = noise(total, seed ^ 0xff, 1.0);
                left[SIGNAL..].fill(0.0);
                right[SIGNAL..].fill(0.0);
                let peak = left
                    .iter()
                    .chain(&right)
                    .fold(0.0_f32, |p, x| p.max(x.abs()));
                render(effect.as_mut(), &mut left, &mut right);
                let limit = g_p * f64::from(peak) + sigma_p;
                for (n, y) in left.iter().chain(&right).enumerate() {
                    assert!(
                        f64::from(y.abs()) <= limit,
                        "{rate} Hz {case:?} bypass {bypass}: |y| = {y} above {limit} (frame {n})"
                    );
                    worst = worst.max(f64::from(y.abs()) / f64::from(peak));
                }
                for (channel, out) in [&left, &right].into_iter().enumerate() {
                    if let Some(n) = out[SIGNAL + quiet..].iter().position(|y| *y != 0.0) {
                        panic!(
                            "{rate} Hz {case:?} bypass {bypass} channel {channel}: output {} at {} samples into the silence",
                            out[SIGNAL + quiet + n],
                            quiet + n
                        );
                    }
                    last_live |= out[SIGNAL + quiet - 1] != 0.0;
                }
            }
        }
        assert!(
            last_live,
            "{rate} Hz: no run is non-zero at latency + T - 1 = {} samples into the silence",
            quiet - 1
        );
        eprintln!(
            "{rate} Hz: T {tail}, R {rest}, G_p {gain} mB, sigma_p {stall} mB, largest |y| / max|x| {worst}"
        );
    }
}

/// This build's bank width, and a bank of `requests` bound at it.
fn bind(requests: &[PrepareEffectRequest<'_>]) -> (BankWidth, Box<dyn PreparedNativeEffectBank>) {
    let backend = Backend::current();
    let width = BankWidth::for_backend(backend).expect("a bank width on every launch target");
    let bank = SoftClipFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
            active_mask: width.full_mask(),
        })
        .expect("bind")
        .expect("the bank binds");
    (width, bank.processor)
}

/// Renders an AoSoA block (`frames * lanes` words per plane) in one call.
fn render_bank(
    bank: &mut dyn PreparedNativeEffectBank,
    width: BankWidth,
    left: &mut [f32],
    right: &mut [f32],
    first: u64,
) {
    let lanes = width.lanes() as usize;
    let frames = left.len() / lanes;
    let offsets = vec![0_u32; lanes + 1];
    bank.process_bank(
        EffectBankProcessBlock::new(
            left,
            right,
            None,
            frames as u32,
            width,
            first,
            &[],
            &offsets,
            QUANTUM as u32,
        )
        .expect("bank block"),
    );
}

/// Interleaves per-lane planes into one AoSoA plane.
fn interleave(planes: &[Vec<f32>]) -> Vec<f32> {
    let frames = planes[0].len();
    (0..frames * planes.len())
        .map(|i| planes[i % planes.len()][i / planes.len()])
        .collect()
}

/// **Gate 2: exact rest by `latency + R`, real kernel, judged by the rest predicate itself**: a
/// bank whose lanes alternate the two peak cases (`10^(24/20)` and `1e29`), at the largest drive
/// and output, renders noise and then `latency + R` zeros. From there the output is `+-0.0`,
/// `lanes_at_rest` sets every lane (every ring row `+0.0`, mirrors included: the payload's read
/// ages alone would rest two samples sooner), every track's payload equals, word for word, the
/// payload of a freshly bound bank fed the same zeros (`Z`, the reset state; the shared cursor
/// `pos` is the one word the payload leaves out), and the two banks then render the same seeded
/// noise to the bit.
#[test]
fn the_state_reaches_its_rest_by_the_stated_bound() {
    for &rate in rates() {
        let RestBound::Bounded(rest) = stated(rate).rest else {
            panic!("{rate} Hz: no exact-rest bound stated");
        };
        assert_eq!(rest.peak_plus_24_dbfs, rest.any_sanitized_input);
        let silence = latency() + rest.any_sanitized_input as usize;
        for case in [[36.0_f32, 24.0, 1.0], [36.0, 24.0, 0.5], [0.0, 0.0, 1.0]] {
            let values = values_from([(case[0], case[0]), (case[1], case[1]), (case[2], case[2])]);
            let probe = BankWidth::for_backend(Backend::current()).expect("a bank width");
            let lanes = probe.lanes() as usize;
            let requests: Vec<PrepareEffectRequest<'_>> =
                (0..lanes).map(|_| request(&values, rate, false)).collect();
            let (width, mut bank) = bind(&requests);
            let (_, mut fresh) = bind(&requests);
            let peak = |lane: usize| {
                if lane.is_multiple_of(2) {
                    10.0_f32.powf(24.0 / 20.0)
                } else {
                    1.0e29
                }
            };
            let planes = |channel: u64| -> Vec<Vec<f32>> {
                (0..lanes)
                    .map(|lane| noise(SIGNAL, 0x1376_6000 + 2 * lane as u64 + channel, peak(lane)))
                    .collect()
            };
            let (mut left, mut right) = (interleave(&planes(0)), interleave(&planes(1)));
            for start in (0..SIGNAL).step_by(QUANTUM) {
                let span = start * lanes..(start + QUANTUM) * lanes;
                render_bank(
                    bank.as_mut(),
                    width,
                    &mut left[span.clone()],
                    &mut right[span],
                    start as u64,
                );
            }
            let mut zl = vec![0.0_f32; silence * lanes];
            let mut zr = vec![0.0_f32; silence * lanes];
            render_bank(bank.as_mut(), width, &mut zl, &mut zr, SIGNAL as u64);
            let tail_start = (latency() + 29) * lanes;
            assert!(
                zl[tail_start..]
                    .iter()
                    .chain(&zr[tail_start..])
                    .all(|y| *y == 0.0),
                "{rate} Hz {case:?}: non-zero output after the tail"
            );
            let mut fl = vec![0.0_f32; silence * lanes];
            let mut fr = vec![0.0_f32; silence * lanes];
            render_bank(fresh.as_mut(), width, &mut fl, &mut fr, 0);
            assert_eq!(
                bank.lanes_at_rest(),
                (1_u32 << lanes) - 1,
                "{rate} Hz {case:?}: not every lane at rest {silence} samples into the silence"
            );
            for track in 0..lanes as u32 {
                assert_eq!(
                    snapshot_bank(bank.as_ref(), track),
                    snapshot_bank(fresh.as_ref(), track),
                    "{rate} Hz {case:?} track {track}: payload unlike Z"
                );
            }
            let mut a = noise(QUANTUM * lanes, 0x1376_7000, 0.5);
            let mut b = noise(QUANTUM * lanes, 0x1376_7001, 0.5);
            let (mut c, mut d) = (a.clone(), b.clone());
            render_bank(
                bank.as_mut(),
                width,
                &mut a,
                &mut b,
                (SIGNAL + silence) as u64,
            );
            render_bank(fresh.as_mut(), width, &mut c, &mut d, silence as u64);
            assert!(
                a.iter()
                    .zip(&c)
                    .chain(b.iter().zip(&d))
                    .all(|(x, y)| x.to_bits() == y.to_bits()),
                "{rate} Hz {case:?}: the rested bank renders unlike Z"
            );
            eprintln!("{rate} Hz {case:?}: every lane at rest {silence} samples into the silence");
        }
    }
}
