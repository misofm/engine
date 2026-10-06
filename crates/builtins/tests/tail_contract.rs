//! Issue #1329's objective gates: the builtin input section's certified tail (`T_decay`), its tail
//! over every peak (`T_rest`) and its exact-rest bound (`RestSamples`), recomputed against an
//! independent `f64` brute force and against the real `f32` kernel.
//!
//! Every launch rate in release; 48 kHz only in debug. The real-kernel soundness gate at the
//! domain extreme (gate 2) runs in release only. Evidence lines are printed with `--nocapture`.
#![allow(missing_docs)]

use builtins::test_support::{input_section_words, input_state_words, input_trim_words};
use builtins::{
    BuiltinChain, BuiltinParameters, ChannelParameters, DualMonoBlock, InputBuiltins,
    PreparedInputFilterTarget, builtin_filter_cutoff_maximum_hz, input_section_flush_law,
    input_section_live_bound, input_section_live_cascade, input_section_live_envelope,
    input_section_worst_case_pair, prepare_input_filter_pair,
};
use effect_contract::{RestSamples, TailSamples};
use math::tail::{CascadeBound, SectionConstants, SvfWords, TAIL_FLOOR, fixed_cascade};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

fn rates() -> &'static [u32] {
    if cfg!(debug_assertions) {
        &[48_000]
    } else {
        &LAUNCH_RATES
    }
}

fn below(value: f32) -> f32 {
    f32::from_bits(value.to_bits() - 1)
}

fn maximum(rate: u32) -> f32 {
    builtin_filter_cutoff_maximum_hz(rate).expect("launch rate")
}

/// Gate 1(a)'s cutoffs: disabled, 10 Hz, 1 kHz, one `f32` below the maximum, the maximum.
fn cutoffs(rate: u32) -> [f32; 5] {
    [0.0, 10.0, 1_000.0, below(maximum(rate)), maximum(rate)]
}

/// Every valid HPF/LPF pair of [`cutoffs`] (the HPF below the LPF when both are enabled).
fn pairs(rate: u32) -> Vec<(f32, f32)> {
    let cutoffs = cutoffs(rate);
    let mut pairs = Vec::new();
    for hpf in cutoffs {
        for lpf in cutoffs {
            if hpf == 0.0 || lpf == 0.0 || hpf < lpf {
                pairs.push((hpf, lpf));
            }
        }
    }
    pairs
}

fn chain(rate: u32, hpf: f32, lpf: f32, trim_db: f32, polarity_invert: bool) -> BuiltinChain {
    let channel = ChannelParameters {
        polarity_invert,
        trim_db,
        hpf_hz: hpf,
        lpf_hz: lpf,
        ..ChannelParameters::default()
    };
    BuiltinChain::new(
        rate,
        BuiltinParameters {
            left: channel,
            right: channel,
            ..BuiltinParameters::default()
        },
    )
    .expect("valid parameters")
}

fn input(rate: u32, hpf: f32, lpf: f32, trim_db: f32, polarity_invert: bool) -> InputBuiltins {
    chain(rate, hpf, lpf, trim_db, polarity_invert).into_input_builtins()
}

fn finite(tail: TailSamples) -> u64 {
    match tail {
        TailSamples::Finite(samples) => samples,
        TailSamples::Infinite => panic!("expected a finite tail"),
    }
}

/// The left channel's enabled sections, read from the words the kernel loads.
fn kernel_sections(input: &InputBuiltins) -> Vec<SvfWords> {
    input_section_words(input)[..2]
        .iter()
        .filter_map(|words| {
            let [c1, a2, a3, _k, m0, m1, m2] = words.map(f32::from_bits);
            let identity = m0 == 1.0 && m1 == 0.0 && m2 == 0.0;
            (!identity).then(|| SvfWords::from_f32([c1, a2, a3, m0, m1, m2]))
        })
        .collect()
}

fn trim_gain(input: &InputBuiltins) -> f64 {
    f64::from(f32::from_bits(input_trim_words(input)[0]).abs())
}

/// The two peaks D2 states its rest for, as preparation states them: the `f32` gain of +24 dB and
/// the input sanitizer's limit.
fn rest_peaks() -> [f64; 2] {
    [
        f64::from(math::pow(10.0, 24.0 / 20.0) as f32),
        f64::from(lane::kernels::builtins::NONFINITE_LIMIT),
    ]
}

/// The exact-arithmetic impulse response of a cascade of the designed `f32` words, evaluated in
/// `f64`: an independent brute force, sharing no code with `math::tail`.
fn impulse_response(sections: &[SvfWords], frames: usize) -> Vec<f64> {
    let mut state = vec![[0.0_f64; 2]; sections.len()];
    let mut out = vec![0.0_f64; frames];
    for (frame, value) in out.iter_mut().enumerate() {
        let mut x = if frame == 0 { 1.0 } else { 0.0 };
        for (words, state) in sections.iter().zip(state.iter_mut()) {
            let v3 = x - state[1];
            let d1 = -words.c1 * state[0] + words.a2 * v3;
            let d2 = words.a3 * v3 + words.a2 * state[0];
            let v1 = state[0] + d1;
            let v2 = state[1] + d2;
            state[0] += 2.0 * d1;
            state[1] += 2.0 * d2;
            x = words.m0 * x + words.m1 * v1 + words.m2 * v2;
        }
        *value = x;
    }
    out
}

/// `T_b(e)` for each level: the smallest `j` with `g * sum_{j <= m < len} |h[m]| < e`.
fn suffix_crossings(response: &[f64], gain: f64, levels: [f64; 2]) -> [usize; 2] {
    let mut crossings = [0_usize; 2];
    let mut sum = 0.0_f64;
    for index in (0..response.len()).rev() {
        sum += response[index].abs();
        for (crossing, level) in crossings.iter_mut().zip(levels) {
            if *crossing == 0 && gain * sum >= level {
                *crossing = index + 1;
            }
        }
    }
    crossings
}

struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `[-1, 1)`.
    fn signed(&mut self) -> f32 {
        ((self.next() >> 40) as f32 / (1_u64 << 24) as f32).mul_add(2.0, -1.0)
    }
}

fn process(input: &mut InputBuiltins, left: &mut [f32], right: &mut [f32], first: u64) {
    input.process(DualMonoBlock::new(left, right, first).expect("block"));
}

fn state_is_rest(input: &InputBuiltins) -> bool {
    input_state_words(input)
        .iter()
        .all(|word| f32::from_bits(*word) == 0.0)
}

/// What the real kernel did after the input stopped at `N`.
#[derive(Debug, Default)]
struct Observed {
    /// One past the last frame after `N` with `|y| >= P * eps` (`0` if none).
    above: u64,
    /// The first 64-frame block end after `N` at which every integrator is `+0.0` or `-0.0`
    /// (exact rest is reached at or before it, and after the previous block end).
    rest: Option<u64>,
    /// Whether a `-0.0` output was seen after rest.
    negative_zero: bool,
}

/// Runs `input`, whose drive has already been processed up to `N = first`, on silence for up to
/// `limit` frames in blocks of at most 64, with a block boundary at every `checkpoint`, and checks
/// `at_checkpoint` there. Stops 64 frames after the state rests (from then on it stays at rest:
/// zero input into zero state is exactly zero).
fn observe_silence(
    input: &mut InputBuiltins,
    first: u64,
    peak: f32,
    limit: u64,
    checkpoints: &[u64],
    mut at_checkpoint: impl FnMut(u64, &InputBuiltins),
) -> Observed {
    let floor = peak * TAIL_FLOOR as f32;
    let mut observed = Observed::default();
    let mut left = [0.0_f32; 64];
    let mut right = [0.0_f32; 64];
    let mut done = 0_u64;
    let mut after_rest = 0_u64;
    while done < limit {
        let mut frames = 64.min(limit - done);
        if let Some(next) = checkpoints.iter().copied().find(|point| *point > done) {
            frames = frames.min(next - done);
        }
        let count = frames as usize;
        left[..count].fill(0.0);
        right[..count].fill(0.0);
        process(input, &mut left[..count], &mut right[..count], first + done);
        for (index, (l, r)) in left[..count].iter().zip(&right[..count]).enumerate() {
            if l.abs() >= floor || r.abs() >= floor {
                observed.above = done + index as u64 + 1;
            }
            if observed.rest.is_some() {
                assert!(*l == 0.0 && *r == 0.0, "an output after rest is not zero");
                observed.negative_zero |=
                    l.to_bits() == (-0.0_f32).to_bits() || r.to_bits() == (-0.0_f32).to_bits();
            }
        }
        done += frames;
        if checkpoints.contains(&done) {
            at_checkpoint(done, input);
        }
        if observed.rest.is_none() && state_is_rest(input) {
            observed.rest = Some(done);
        }
        if observed.rest.is_some() {
            after_rest += frames;
            if after_rest > 64 && checkpoints.iter().all(|point| *point <= done) {
                break;
            }
        }
    }
    observed
}

// ---- Gate 1(a): fixed designs against an independent brute force ---------------------------

/// Gate 1(a). For every HPF/LPF pair of {disabled, 10 Hz, 1 kHz, one `f32` below the maximum, the
/// maximum} at trims 0 and +24 dB, the prepared `T_decay` covers the exact-arithmetic half of D3
/// (`T_b(eps / 2) <= T`) and is never longer than the exact tail at a floor 30 dB lower
/// (`T <= T_b(eps / 32)`). Disabled filters report `0`.
#[test]
fn fixed_design_tail_is_sound_and_within_thirty_db_of_the_exact_tail() {
    const FRAMES: usize = 4_000_000;
    for &rate in rates() {
        for (hpf, lpf) in pairs(rate) {
            let response = {
                let probe = input(rate, hpf, lpf, 0.0, false);
                let sections = kernel_sections(&probe);
                (!sections.is_empty()).then(|| impulse_response(&sections, FRAMES))
            };
            for trim_db in [0.0, 24.0] {
                let input = input(rate, hpf, lpf, trim_db, false);
                let tail = finite(input.tail());
                let Some(response) = &response else {
                    assert_eq!(tail, 0, "{rate} Hz: disabled filters have no tail");
                    assert_eq!(input.tail_every_peak(), TailSamples::Finite(0));
                    assert_eq!(input.rest(), Some(RestSamples::ZERO));
                    continue;
                };
                let gain = trim_gain(&input);
                let [sound, tight] =
                    suffix_crossings(response, gain, [TAIL_FLOOR / 2.0, TAIL_FLOOR / 32.0]);
                eprintln!(
                    "1(a) {rate} Hz HPF {hpf} LPF {lpf} trim {trim_db} dB: T_decay {tail}, \
                     T_b(eps/2) {sound}, T_b(eps/32) {tight}, ratio {:.4}",
                    tail as f64 / sound.max(1) as f64
                );
                assert!(
                    sound as u64 <= tail,
                    "{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db}: T_b(eps/2) {sound} > T {tail}"
                );
                assert!(
                    tail <= tight as u64,
                    "{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db}: T {tail} > T_b(eps/32) {tight}"
                );
            }
        }
    }
}

/// D7's ahead-of-time path: a bound computed by `prepare_input_bound` is stored only by a chain of
/// the same design (rate, both channels' section words and trim magnitude); any other chain
/// computes its own, so a compiler that pairs a bound with the wrong strip still reports each
/// strip's own tail.
#[test]
fn a_prepared_bound_is_stored_only_for_its_own_design() {
    let rate = 48_000;
    let parameters = |hpf: f32, trim_db: f32| {
        let channel = ChannelParameters {
            trim_db,
            hpf_hz: hpf,
            ..ChannelParameters::default()
        };
        BuiltinParameters {
            left: channel,
            right: channel,
            ..BuiltinParameters::default()
        }
    };
    let own = |parameters| BuiltinChain::new(rate, parameters).expect("chain");
    let base = parameters(1_000.0, 0.0);
    let prepared = builtins::prepare_input_bound(rate, base).expect("prepared");
    let reused = BuiltinChain::with_prepared_bound(rate, base, &prepared).expect("chain");
    assert_eq!(reused.tail(), own(base).tail());
    assert_eq!(reused.rest(), own(base).rest());
    for other in [parameters(10.0, 0.0), parameters(1_000.0, 24.0)] {
        let chain = BuiltinChain::with_prepared_bound(rate, other, &prepared).expect("chain");
        assert_ne!(own(other).tail(), own(base).tail());
        assert_eq!(chain.tail(), own(other).tail());
        assert_eq!(chain.tail_every_peak(), own(other).tail_every_peak());
        assert_eq!(chain.rest(), own(other).rest());
    }
    let other_rate = BuiltinChain::new(44_100, base).expect("chain").tail();
    assert_ne!(other_rate, own(base).tail());
    assert_eq!(
        BuiltinChain::with_prepared_bound(44_100, base, &prepared)
            .expect("chain")
            .tail(),
        other_rate
    );
}

// ---- Gate 1(b): the live bound covers the domain ---------------------------------------------

fn design_words(rate: u32, section: usize, hz: f32) -> SvfWords {
    let words = builtins::test_support::section_words(rate, hz, section == 0).expect("design");
    let [c1, a2, a3, _k, m0, m1, m2] = words.map(f32::from_bits);
    SvfWords::from_f32([c1, a2, a3, m0, m1, m2])
}

fn target(rate: u32, section: usize, hz: f32) -> PreparedInputFilterTarget {
    let pair = if section == 0 {
        prepare_input_filter_pair(rate, hz, 0.0)
    } else {
        prepare_input_filter_pair(rate, 0.0, hz)
    }
    .expect("target");
    pair.targets[section]
}

/// The kernel contraction of one recursion word set, rounding included.
fn kernel_contraction(words: &SvfWords) -> f64 {
    SectionConstants::of(words).rho_kernel()
}

/// Gate 1(b). Over 100,000 log-spaced cutoffs and the last 65,536 `f32` cutoffs below each rate's
/// maximum (a tenth of each in debug), for both sections: no design exceeds the live envelope the live bound is computed
/// from (contraction, input column, output row, feedthrough, rounding included), the slowest
/// designs are the worst-case pair's, a sampled design's own bound never exceeds the live bound,
/// and every recursion word the real kernel loads while ramping between scanned designs (and under
/// per-frame restarts at the top of the domain) contracts within the live envelope's `rho_ramp < 1`.
#[test]
fn live_bound_covers_every_scanned_design_and_ramp_word() {
    for &rate in rates() {
        let envelope = input_section_live_envelope(rate).expect("launch rate");
        let live = input_section_live_bound(rate).expect("launch rate");
        assert!(envelope.rho_settled <= envelope.rho_ramp && envelope.rho_ramp < 1.0);
        let top = maximum(rate);
        // The release scale is the gate's; debug runs a tenth of it (48 kHz only).
        let (log_points, top_points, near_top_ramps, log_ramps) = if cfg!(debug_assertions) {
            (10_000_u32, 4_096, 256, 64)
        } else {
            (100_000, 65_536, 2_048, 512)
        };
        let mut scan: Vec<f32> = (0..log_points)
            .map(|index| {
                let fraction = f64::from(index) / f64::from(log_points - 1);
                (10.0 * math::pow(f64::from(top) / 10.0, fraction)) as f32
            })
            .map(|hz| hz.clamp(10.0, top))
            .collect();
        let mut hz = top;
        for _ in 0..top_points {
            hz = below(hz);
            scan.push(hz);
        }
        scan.push(top);
        scan.sort_by(f32::total_cmp);
        scan.dedup();
        // Every design inside the envelope, and the slowest design per section.
        let mut slowest = [(0.0_f64, 0.0_f32); 2];
        for &hz in &scan {
            for (section, slowest) in slowest.iter_mut().enumerate() {
                let words = design_words(rate, section, hz);
                let constants = SectionConstants::of(&words);
                assert!(
                    constants.rho_kernel() <= envelope.rho_settled
                        && constants.beta + constants.mu_input <= envelope.input
                        && constants.gamma + constants.omega_state <= envelope.output_state
                        && constants.delta + constants.omega_input <= envelope.output_input,
                    "{rate} Hz section {section} at {hz} Hz exceeds the live envelope: \
                     {constants:?} against {envelope:?}"
                );
                if constants.rho_kernel() >= slowest.0 {
                    *slowest = (constants.rho_kernel(), hz);
                }
            }
        }
        // The worst-case pair: the two slowest designs the HPF-below-LPF order admits.
        let pair = input_section_worst_case_pair(rate).expect("launch rate");
        assert_eq!(pair, (below(top), top));
        assert_eq!(
            slowest[1].1, top,
            "{rate} Hz: the slowest LPF is not the maximum"
        );
        let slowest_hpf_below = scan
            .iter()
            .filter(|hz| **hz < top)
            .map(|hz| (kernel_contraction(&design_words(rate, 0, *hz)), *hz))
            .fold((0.0_f64, 0.0_f32), |best, next| {
                if next.0 >= best.0 { next } else { best }
            });
        assert_eq!(
            slowest_hpf_below.1, pair.0,
            "{rate} Hz: a slower HPF below the maximum exists"
        );
        eprintln!(
            "1(b) {rate} Hz: {} designs per section; slowest HPF {} Hz ({:.7e}), slowest LPF {} Hz \
             ({:.7e}); envelope rho_ramp - 1 = {:.5e}, rho_settled - 1 = {:.5e}",
            scan.len(),
            slowest[0].1,
            slowest[0].0 - 1.0,
            slowest[1].1,
            slowest[1].0 - 1.0,
            envelope.rho_ramp - 1.0,
            envelope.rho_settled - 1.0
        );
        // A sampled design's own certified bound, at the top trim, never exceeds the live bound.
        let live_rest = live.rest.expect("live rest");
        let sampled = scan
            .iter()
            .step_by(scan.len() / 16)
            .copied()
            .chain(scan.iter().rev().take(4).copied());
        for hz in sampled {
            for (hpf, lpf) in [(hz, 0.0), (0.0, hz)] {
                let own = input(rate, hpf, lpf, 24.0, false);
                let rest = own.rest().expect("fixed rest");
                assert!(
                    finite(own.tail()) <= finite(live.tail)
                        && finite(own.tail_every_peak()) <= finite(live.tail_every_peak)
                        && rest.peak_plus_24_dbfs <= live_rest.peak_plus_24_dbfs
                        && rest.any_sanitized_input <= live_rest.any_sanitized_input,
                    "{rate} Hz HPF {hpf} LPF {lpf}: a design exceeds the live bound"
                );
            }
        }
        // Interior ramp words, read from the real kernel frame by frame.
        let mut worst_ramp_word = 0.0_f64;
        let mut check = |input: &InputBuiltins, section: usize| {
            let words = input_section_words(input)[section];
            let [c1, a2, a3, _k, m0, m1, m2] = words.map(f32::from_bits);
            if m0 == 1.0 && m1 == 0.0 && m2 == 0.0 && c1 == 0.0 && a2 == 0.0 && a3 == 0.0 {
                return;
            }
            let rho = kernel_contraction(&SvfWords::from_f32([c1, a2, a3, m0, m1, m2]));
            worst_ramp_word = worst_ramp_word.max(rho);
            assert!(
                rho <= envelope.rho_ramp,
                "{rate} Hz: a ramp word contracts by {rho} > rho_ramp {}",
                envelope.rho_ramp
            );
        };
        let mut silence = [0.0_f32; 1];
        let mut silence_right = [0.0_f32; 1];
        let mut ramp = |from: f32, to: &[f32], section: usize, frames_per_target: usize| {
            let (hpf, lpf) = if section == 0 {
                (from, 0.0)
            } else {
                (0.0, from)
            };
            let mut input = input(rate, hpf, lpf, 0.0, false);
            let mut sample = 0_u64;
            for &hz in to {
                input
                    .apply_prepared_filter(target(rate, section, hz))
                    .expect("retarget");
                for _ in 0..frames_per_target {
                    process(&mut input, &mut silence, &mut silence_right, sample);
                    sample += 1;
                    check(&input, section);
                }
            }
        };
        let near_top: Vec<f32> = scan.iter().rev().take(near_top_ramps).copied().collect();
        let log_sample: Vec<f32> = scan
            .iter()
            .step_by(scan.len() / log_ramps)
            .copied()
            .collect();
        for section in 0..2 {
            // Consecutive designs at the top of the domain, each ramp run to completion.
            ramp(top, &near_top, section, 64);
            // From the maximum to log-spaced designs and back.
            for &hz in &log_sample {
                ramp(top, &[hz, top], section, 64);
            }
            // Per-frame restarts: the maximum against its neighbour and against 10 Hz.
            let alternate: Vec<f32> = (0..4_096)
                .map(|index| if index % 2 == 0 { below(top) } else { top })
                .collect();
            ramp(top, &alternate, section, 1);
            let wide: Vec<f32> = (0..4_096)
                .map(|index| if index % 2 == 0 { 10.0 } else { top })
                .collect();
            ramp(top, &wide, section, 1);
        }
        eprintln!(
            "1(b) {rate} Hz: worst scanned ramp word contraction - 1 = {:.5e} (rho_ramp - 1 = {:.5e})",
            worst_ramp_word - 1.0,
            envelope.rho_ramp - 1.0
        );
    }
}

// ---- Gate 1(c): the tail over every peak -----------------------------------------------------

/// Gate 1(c). (i) For every enabled pair of gate 1(a) at both trims, the prepared values are the
/// module's for the kernel's own words, and `T_rest` is `max(T_decay, R(P*))` with `R(P*)`
/// recomputed through the rest bound at the peak `P*`, and at least `N_SILENCE`. (ii) A real
/// input section driven at peaks `P*`, `P* / 10`, `1e-13` and `1e-20` (random and alternating),
/// then silent, reaches exact rest (outputs `+-0.0`, integrators at the reset state) by
/// `N + T_rest`, and its last output at or above `P * eps` falls before `N + T_rest`. (ii) runs in
/// release only.
#[test]
fn tail_every_peak_is_the_rest_at_the_flush_floor_and_holds_on_the_real_kernel() {
    const DRIVE: usize = 8_192;
    for &rate in rates() {
        let law = input_section_flush_law(rate);
        for (hpf, lpf) in pairs(rate) {
            if hpf == 0.0 && lpf == 0.0 {
                continue;
            }
            for trim_db in [0.0, 24.0] {
                let prepared = input(rate, hpf, lpf, trim_db, false);
                let sections = kernel_sections(&prepared);
                let gain = trim_gain(&prepared);
                let bound: CascadeBound =
                    fixed_cascade(&sections, gain, &law, rest_peaks()).expect("bound");
                // (i) The prepared values, and T_rest's formula.
                assert_eq!(prepared.tail(), TailSamples::Finite(bound.tail));
                assert_eq!(
                    prepared.tail_every_peak(),
                    TailSamples::Finite(bound.tail_every_peak)
                );
                assert_eq!(
                    prepared.rest(),
                    Some(RestSamples {
                        peak_plus_24_dbfs: bound.rest_peak,
                        any_sanitized_input: bound.rest_any,
                    })
                );
                let p_star = bound.flush_floor;
                let at_floor = fixed_cascade(&sections, gain, &law, [p_star, p_star])
                    .expect("bound at the flush floor")
                    .rest_peak;
                let t_rest = bound.tail.max(at_floor);
                assert_eq!(bound.tail_every_peak, t_rest);
                assert!(t_rest >= law.silence_frames);
                if cfg!(debug_assertions) {
                    // (ii) is release-scale: every launch rate on the real kernel.
                    continue;
                }
                // (ii) The real kernel below and at the flush floor.
                let mut draw = SplitMix(u64::from(rate) ^ u64::from(hpf.to_bits()) << 7);
                let peaks = [p_star as f32, (p_star / 10.0) as f32, 1.0e-13, 1.0e-20];
                let mut worst = (0_u64, 0_u64);
                for peak in peaks {
                    for alternating in [false, true] {
                        let mut input = input(rate, hpf, lpf, trim_db, false);
                        let mut left: Vec<f32> = (0..DRIVE)
                            .map(|index| {
                                if alternating {
                                    if index % 2 == 0 { peak } else { -peak }
                                } else {
                                    peak * draw.signed()
                                }
                            })
                            .collect();
                        let mut right = left.clone();
                        process(&mut input, &mut left, &mut right, 0);
                        let observed = observe_silence(
                            &mut input,
                            DRIVE as u64,
                            peak,
                            t_rest + 64,
                            &[t_rest],
                            |_, input| {
                                assert!(
                                    state_is_rest(input),
                                    "{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db} P {peak}: \
                                     not at rest at N + T_rest = {t_rest}"
                                );
                            },
                        );
                        let rest = observed.rest.expect("rest reached");
                        assert!(
                            observed.above < t_rest && rest <= t_rest,
                            "{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db} P {peak}: \
                             {observed:?} against T_rest {t_rest}"
                        );
                        worst = (worst.0.max(observed.above), worst.1.max(rest));
                    }
                }
                eprintln!(
                    "1(c) {rate} Hz HPF {hpf} LPF {lpf} trim {trim_db} dB: T_decay {}, P* {:.3e}, \
                     R(P*) {at_floor}, T_rest {t_rest}; real kernel at P <= P*: last |y| >= P eps \
                     {}, rest by {}",
                    bound.tail, p_star, worst.0, worst.1
                );
            }
        }
    }
}

// ---- Gate 2: the live bound on the real kernel at the domain extreme -------------------------

/// The extreme pair's `f64` impulse response, for the adversarial input's signs.
fn extreme_response(rate: u32, frames: usize) -> Vec<f64> {
    let (hpf, lpf) = input_section_worst_case_pair(rate).expect("launch rate");
    impulse_response(&kernel_sections(&input(rate, hpf, lpf, 0.0, false)), frames)
}

/// Gate 2. At the domain extreme of D5, every launch rate, trim +24 dB, input peaks 1, +24 dBFS
/// and `1e29`: an input that maximises the output at `N + T_decay` (the reversed signs of the
/// impulse response there, over the last 1,000,000 samples), with a live HPF target applied 32
/// samples before `N`; and a block-size-1 history that re-sends and alternates the worst-case
/// pair with disable, a disable in flight at `N`. From `N + T_decay` until exact rest every output
/// is below `P * eps`; from `N + R` on (`R` the live rest bound for the peak) every output is
/// `+-0.0` and every integrator equals a freshly reset section's. One run inverts polarity. Also
/// measures the fixed top pair's real rest under an alternating +24 dBFS input (gate 3's lower
/// side).
#[test]
#[cfg_attr(debug_assertions, ignore = "release scale (#1329 gate 2)")]
fn live_bound_holds_on_the_real_kernel_at_the_domain_extreme() {
    const HISTORY: usize = 1_000_000;
    for &rate in rates() {
        let live = input_section_live_bound(rate).expect("launch rate");
        let rest = live.rest.expect("live rest");
        let t_decay = finite(live.tail);
        let (hpf, lpf) = input_section_worst_case_pair(rate).expect("launch rate");
        let response = extreme_response(rate, t_decay as usize + 1 + HISTORY);
        let plus_24 = math::pow(10.0, 24.0 / 20.0) as f32;
        let fresh_state = input_state_words(&input(rate, hpf, lpf, 24.0, false));
        let runs: [(f32, bool, bool); 5] = [
            (1.0, false, false),
            (plus_24, false, false),
            (1.0e29, false, false),
            (plus_24, true, false),
            (plus_24, false, true),
        ];
        for (peak, polarity_invert, quantum_one) in runs {
            let r = if peak <= plus_24 {
                rest.peak_plus_24_dbfs
            } else {
                rest.any_sanitized_input
            };
            let start_hpf = if quantum_one { hpf } else { 1_000.0 };
            let mut section = input(rate, start_hpf, lpf, 24.0, polarity_invert);
            // x[N - 1 - i] = P sign(h[T + 1 + i]): y[N + T] = P sum |h[T + 1 + i]|.
            let mut left: Vec<f32> = (0..HISTORY)
                .rev()
                .map(|i| {
                    if response[t_decay as usize + 1 + i] < 0.0 {
                        -peak
                    } else {
                        peak
                    }
                })
                .collect();
            let mut right = left.clone();
            if quantum_one {
                let tail = 4_096;
                let head = HISTORY - tail;
                process(&mut section, &mut left[..head], &mut right[..head], 0);
                let worst = prepare_input_filter_pair(rate, hpf, lpf).expect("pair");
                let disabled = prepare_input_filter_pair(rate, 0.0, 0.0).expect("disabled");
                for frame in head..HISTORY {
                    let pair = if (frame - head) % 2 == 1 || frame == HISTORY - 1 {
                        &disabled
                    } else {
                        &worst
                    };
                    for target in pair.targets {
                        section.apply_prepared_filter(target).expect("retarget");
                    }
                    process(
                        &mut section,
                        &mut left[frame..=frame],
                        &mut right[frame..=frame],
                        frame as u64,
                    );
                }
            } else {
                let head = HISTORY - 32;
                process(&mut section, &mut left[..head], &mut right[..head], 0);
                section
                    .apply_prepared_filter(target(rate, 0, below(hpf)))
                    .expect("live HPF target in flight at N");
                process(
                    &mut section,
                    &mut left[head..],
                    &mut right[head..],
                    head as u64,
                );
            }
            let reset = if quantum_one {
                input_state_words(&input(rate, 0.0, 0.0, 24.0, false))
            } else {
                fresh_state
            };
            let floor = peak * TAIL_FLOOR as f32;
            let mut late = 0_u64;
            let observed = observe_silence(
                &mut section,
                HISTORY as u64,
                peak,
                r + 64,
                &[t_decay, r],
                |done, section| {
                    if done == r {
                        let state = input_state_words(section);
                        assert!(
                            state
                                .iter()
                                .zip(reset)
                                .all(|(word, z)| f32::from_bits(*word) == f32::from_bits(z)),
                            "{rate} Hz P {peak}: integrators not at the rest state at N + R = {r}"
                        );
                    }
                },
            );
            if observed.above > t_decay {
                late = observed.above;
            }
            let measured = observed.rest.expect("rest reached");
            eprintln!(
                "2 {rate} Hz P {peak} invert {polarity_invert} quantum-1 {quantum_one}: last \
                 |y| >= P eps at {} (T_decay {t_decay}), rest by {measured} (R {r}), -0.0 seen {}",
                observed.above, observed.negative_zero
            );
            assert_eq!(
                late, 0,
                "{rate} Hz P {peak}: |y| >= {floor} after N + T_decay"
            );
            assert!(measured <= r);
        }
        // Gate 3's lower side: the fixed top pair's real rest under an alternating input through
        // the +24 dB trim stays at or below the certified bound, at +24 dBFS
        // (`peak_plus_24_dbfs`) and at `1e29` (`any_sanitized_input`). At `1e29` the sign-pattern
        // run above is already at rest by `N + 64`; the alternating drive is the one that leaves
        // the state at its largest finite values (about `4e34`) and lets it decay.
        for (peak, bound) in [
            (plus_24, rest.peak_plus_24_dbfs),
            (1.0e29, rest.any_sanitized_input),
        ] {
            let mut top_pair = input(rate, hpf, lpf, 24.0, false);
            let mut left: Vec<f32> = (0..HISTORY)
                .map(|index| if index % 2 == 0 { peak } else { -peak })
                .collect();
            let mut right = left.clone();
            process(&mut top_pair, &mut left, &mut right, 0);
            let observed = observe_silence(
                &mut top_pair,
                HISTORY as u64,
                peak,
                bound + 64,
                &[t_decay, bound],
                |done, section| {
                    if done == bound {
                        assert!(
                            state_is_rest(section),
                            "{rate} Hz P {peak}: not at rest by R"
                        );
                    }
                },
            );
            let measured = observed.rest.expect("rest reached");
            eprintln!(
                "3 {rate} Hz: top pair real rest at P {peak} (alternating) by {measured}, last \
                 |y| >= P eps at {}, certified R {bound}",
                observed.above
            );
            assert!(measured <= bound && observed.above <= t_decay);
        }
    }
}

// ---- Gate 3: the restated contract figures ---------------------------------------------------

/// Gate 3. The live exact-rest bounds are at or below decision 15's restated D15-4 figures
/// (#1329 Amendment 3, G4: 1,270,000 at +24 dBFS and 2,590,000 for any sanitized input, each
/// including `2 * N_SILENCE`). The lower side, against the real kernel's rest, is gate 2's run.
#[test]
fn live_rest_bounds_are_within_the_restated_contract_figures() {
    for rate in LAUNCH_RATES {
        let cascade = input_section_live_cascade(rate).expect("launch rate");
        let live = input_section_live_bound(rate).expect("launch rate");
        let rest = live.rest.expect("live rest");
        eprintln!(
            "3 {rate} Hz: T_decay {}, T_rest {}, peak_plus_24_dbfs {}, any_sanitized_input {}, \
             P* {:.4e}",
            finite(live.tail),
            finite(live.tail_every_peak),
            rest.peak_plus_24_dbfs,
            rest.any_sanitized_input,
            cascade.flush_floor
        );
        assert!(rest.peak_plus_24_dbfs <= 1_270_000);
        assert!(rest.any_sanitized_input <= 2_590_000);
        assert!(rest.peak_plus_24_dbfs >= 2 * u64::from(lane::silence_frames(rate)));
    }
}

// ---- Gate 7: headroom to the tail cap --------------------------------------------------------

/// Gate 7. Every live bound is below 10,000,000, the smallest finite
/// `maximum_finite_tail_samples` the tree configures (`graph_fixture`, the audit tools and
/// `console-workload`); a strip's builtin path has one input section, so one term.
#[test]
fn live_bounds_leave_headroom_to_the_tail_cap() {
    const CAP: u64 = 10_000_000;
    for rate in LAUNCH_RATES {
        let live = input_section_live_bound(rate).expect("launch rate");
        let rest = live.rest.expect("live rest");
        for (name, value) in [
            ("T_decay", finite(live.tail)),
            ("T_rest", finite(live.tail_every_peak)),
            ("peak_plus_24_dbfs", rest.peak_plus_24_dbfs),
            ("any_sanitized_input", rest.any_sanitized_input),
        ] {
            eprintln!(
                "7 {rate} Hz {name}: {value}, headroom {} ({:.1} %)",
                CAP - value.min(CAP),
                100.0 * (CAP - value.min(CAP)) as f64 / CAP as f64
            );
            assert!(value < CAP, "{rate} Hz {name} {value} reaches the cap");
        }
    }
}
