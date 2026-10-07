//! Issue #1329's objective gates: the builtin input section's certified tail (`T_decay`), its tail
//! over every peak (`T_rest`) and its exact-rest bound (`RestSamples`), recomputed against an
//! independent `f64` brute force and against the real `f32` kernel; and issue #1433's gates for
//! the frequency-aware live cascade bound (the live scan, its tightness against the real kernel's
//! rest, and an independent recomputation of its derivation).
//!
//! Every launch rate in release; 48 kHz only in debug. The real-kernel soundness gate at the
//! domain extreme (gate 2) runs in release only. Evidence lines are printed with `--nocapture`.
#![allow(missing_docs)]

use builtins::INPUT_FILTER_RAMP_SAMPLES;
use builtins::test_support::{input_section_words, input_state_words, input_trim_words};
use builtins::{
    BuiltinChain, BuiltinParameters, ChannelParameters, DualMonoBlock, InputBoundCache,
    InputBuiltins, InputSectionBound, PreparedInputFilterTarget, builtin_filter_cutoff_maximum_hz,
    input_section_bound, input_section_bound_charged, input_section_bounds,
    input_section_bounds_within, input_section_flush_law, input_section_live_bound,
    input_section_live_bound_table, input_section_live_cascade, input_section_live_envelope,
    input_section_worst_case_pair, prepare_input_filter_pair,
};
use effect_contract::{RestSamples, TailSamples};
use math::tail::{
    CascadeBound, LiveZones, SectionConstants, SvfWords, TAIL_FLOOR, fixed_cascade, live_cascade,
    live_cascade_groups, live_zones, pole_real, v_operator_norm,
};

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

fn parameters(hpf: f32, lpf: f32, trim_db: f32, polarity_invert: bool) -> BuiltinParameters {
    let channel = ChannelParameters {
        polarity_invert,
        trim_db,
        hpf_hz: hpf,
        lpf_hz: lpf,
        ..ChannelParameters::default()
    };
    BuiltinParameters {
        left: channel,
        right: channel,
        ..BuiltinParameters::default()
    }
}

fn input(rate: u32, hpf: f32, lpf: f32, trim_db: f32, polarity_invert: bool) -> InputBuiltins {
    BuiltinChain::new(rate, parameters(hpf, lpf, trim_db, polarity_invert))
        .expect("valid parameters")
        .into_input_builtins()
}

/// The prepared bounds of one design, as preparation computes them.
fn bound(rate: u32, hpf: f32, lpf: f32, trim_db: f32) -> InputSectionBound {
    input_section_bound(rate, parameters(hpf, lpf, trim_db, false)).expect("valid parameters")
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
        // `x * 2` is exact, so this rounds once, as a fused form would.
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32 * 2.0 - 1.0
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
                let prepared = bound(rate, hpf, lpf, trim_db);
                let tail = finite(prepared.tail);
                let Some(response) = &response else {
                    assert_eq!(tail, 0, "{rate} Hz: disabled filters have no tail");
                    assert_eq!(prepared.tail_every_peak, TailSamples::Finite(0));
                    assert_eq!(prepared.rest, Some(RestSamples::ZERO));
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

/// D7 and Amendment 4 R7: a session's strips are bounded once per distinct design, and each strip
/// still gets its own design's bound. The designs differ from the first in one key term each (the
/// HPF words, the trim magnitude, the LPF words), the first design repeats with its polarity
/// inverted (the same key: the bound reads the trim's magnitude), and the last two strips differ
/// from the first only in one channel's HPF, the right (Amendment 5, m2) and then the left (#1329
/// follow-up C): the key reads both channels.
#[test]
fn each_strip_is_bounded_by_its_own_design_when_designs_are_shared() {
    let rate = 48_000;
    let right_only = {
        let mut strip = parameters(1_000.0, 0.0, 0.0, false);
        strip.right.hpf_hz = 10.0;
        strip
    };
    let left_only = {
        let mut strip = parameters(1_000.0, 0.0, 0.0, false);
        strip.left.hpf_hz = 10.0;
        strip
    };
    let strips = [
        parameters(1_000.0, 0.0, 0.0, false),
        parameters(10.0, 0.0, 0.0, false),
        parameters(1_000.0, 0.0, 24.0, false),
        parameters(1_000.0, 2_000.0, 0.0, false),
        parameters(1_000.0, 0.0, 0.0, true),
        right_only,
        left_only,
    ];
    let own: Vec<InputSectionBound> = strips
        .iter()
        .map(|strip| input_section_bound(rate, *strip).expect("bound"))
        .collect();
    for (index, earlier) in own.iter().enumerate().take(4) {
        for later in &own[index + 1..4] {
            assert_ne!(earlier, later, "the designs must bound differently");
        }
    }
    assert_eq!(own[4], own[0]);
    assert_ne!(own[5], own[0], "the right channel must bound differently");
    assert_ne!(own[6], own[0], "the left channel must bound differently");
    assert_eq!(
        input_section_bounds(rate, strips, None).expect("bounds"),
        own,
        "a strip took another design's bound"
    );
}

// ---- #1457 gates 5, 3 and 6: the preparation budget, the cache and the live-bound table --------

/// #1457 gate 5 (and gate 3 at the bound level): the live bound is reported exactly from the
/// design whose charge, summed with the charges before it in strip order, crosses the budget; a
/// design repeated later charges nothing and reports its first strip's value; and a cache, cold,
/// warm or holding a stopped walk's horizon, changes no reported value. The budgets sit on each
/// side of every crossing, pairs of designs run in both strip orders, and a design that passes the
/// budget is followed by cheaper ones, which must still report the live bound.
#[test]
fn the_live_bound_is_taken_exactly_when_the_budget_is_exhausted() {
    let rate = 48_000;
    let designs = [
        parameters(0.0, 1_000.0, 0.0, false),
        parameters(1_000.0, 0.0, 6.0, false),
        parameters(1_000.0, 2_000.0, 0.0, false),
        parameters(10.0, 0.0, 0.0, false),
    ];
    let charged: Vec<_> = designs
        .iter()
        .map(|design| input_section_bound_charged(rate, *design).expect("bound"))
        .collect();
    let live = input_section_live_bound_table(rate).expect("launch rate");
    for (index, design) in charged.iter().enumerate() {
        assert!(design.frames > 0, "design {index} walks");
        assert_ne!(
            design.bound, live,
            "design {index} must bound below the live bound"
        );
        assert_eq!(
            Some(design.bound),
            input_section_bound(rate, designs[index]).ok()
        );
    }
    // Strips in order, a design repeated after others; the last order puts designs cheaper than
    // the first after it, so a budget the first passes leaves room a later one would fit.
    let orders: [&[usize]; 4] = [&[0, 1, 0, 2], &[2, 1, 2, 0], &[1, 0], &[3, 0, 1]];
    for order in orders {
        let strips: Vec<BuiltinParameters> = order.iter().map(|&index| designs[index]).collect();
        // Each design's first strip, in order, and the charges before it.
        let mut firsts: Vec<usize> = Vec::new();
        for &index in order {
            if !firsts.contains(&index) {
                firsts.push(index);
            }
        }
        let mut budgets = std::vec![0, u64::MAX];
        let mut before = 0_u64;
        for &index in &firsts {
            let after = before + charged[index].frames;
            budgets.extend([after - 1, after, after + 1]);
            before = after;
        }
        for budget in budgets {
            // The reference: the first design whose cumulative charge passes the budget, and every
            // design after it, report the live bound.
            let mut covered = 0_u64;
            let mut exhausted = false;
            let mut value = std::collections::BTreeMap::new();
            for &index in &firsts {
                let fits = !exhausted && covered + charged[index].frames <= budget;
                if fits {
                    covered += charged[index].frames;
                } else {
                    exhausted = true;
                }
                value.insert(index, if fits { charged[index].bound } else { live });
            }
            let expected: Vec<InputSectionBound> = order.iter().map(|index| value[index]).collect();
            let what = format!("order {order:?}, budget {budget}");
            let none = input_section_bounds_within(rate, strips.clone(), budget, None);
            assert_eq!(none.expect("bounds"), expected, "{what}, no cache");
            // A cold cache, then the same cache warm.
            let mut cache = InputBoundCache::new();
            for pass in ["cold", "warm"] {
                let cached =
                    input_section_bounds_within(rate, strips.clone(), budget, Some(&mut cache));
                assert_eq!(cached.expect("bounds"), expected, "{what}, {pass} cache");
            }
            // Caches warmed under every other budget: each holds stored charges and stopped
            // walks' horizons this budget has to honour.
            for other in [
                0,
                u64::MAX,
                budget.saturating_sub(1),
                budget.saturating_add(1),
            ] {
                let mut cache = InputBoundCache::new();
                input_section_bounds_within(rate, strips.clone(), other, Some(&mut cache))
                    .expect("bounds");
                let cached =
                    input_section_bounds_within(rate, strips.clone(), budget, Some(&mut cache));
                assert_eq!(
                    cached.expect("bounds"),
                    expected,
                    "{what}, cache warmed under budget {other}"
                );
            }
        }
    }
    // The production budget leaves every one of these designs exact.
    assert_eq!(
        input_section_bounds(rate, designs, Some(&mut InputBoundCache::new())).expect("bounds"),
        charged
            .iter()
            .map(|design| design.bound)
            .collect::<Vec<_>>()
    );
}

/// #1457 gate 3 (D2): the cache's entry cap. A cache of two designs given three keeps at most two,
/// and a design it cleared is computed again with the same value.
#[test]
fn a_full_bound_cache_is_cleared_and_reports_the_same_values() {
    let rate = 48_000;
    let designs = [
        parameters(0.0, 1_000.0, 0.0, false),
        parameters(1_000.0, 0.0, 6.0, false),
        parameters(1_000.0, 2_000.0, 0.0, false),
    ];
    let expected = input_section_bounds(rate, designs, None).expect("bounds");
    let mut cache = InputBoundCache::with_capacity(core::num::NonZeroUsize::new(2).expect("two"));
    for pass in 0..3 {
        let cached = input_section_bounds(rate, designs, Some(&mut cache)).expect("bounds");
        assert_eq!(cached, expected, "pass {pass}");
        assert!(cache.len() <= 2, "pass {pass}: {} entries", cache.len());
    }
}

/// #1457 gate 6 (D2): the live-bound table preparation reads is the computed live bound at every
/// launch rate, bit for bit, and states nothing off them.
#[test]
fn live_bound_table_is_the_computed_live_bound_at_every_launch_rate() {
    for rate in LAUNCH_RATES {
        assert_eq!(
            input_section_live_bound_table(rate),
            input_section_live_bound(rate),
            "{rate} Hz"
        );
        assert!(input_section_live_bound_table(rate).is_some(), "{rate} Hz");
    }
    for rate in [8_000, 32_000, 176_400, 192_000] {
        assert_eq!(input_section_live_bound_table(rate), None, "{rate} Hz");
        assert_eq!(input_section_live_bound(rate), None, "{rate} Hz");
    }
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

/// `||I + A||_V` and `||A^2 - I||_V` of one recursion word set (certified upper bounds).
fn sum_and_square_norms(words: &SvfWords) -> (f64, f64) {
    let a = words.a();
    let sum = [[1.0 + a[0][0], a[0][1]], [a[1][0], 1.0 + a[1][1]]];
    let square = [
        [
            a[0][0] * a[0][0] + a[0][1] * a[1][0] - 1.0,
            a[0][0] * a[0][1] + a[0][1] * a[1][1],
        ],
        [
            a[1][0] * a[0][0] + a[1][1] * a[1][0],
            a[1][0] * a[0][1] + a[1][1] * a[1][1] - 1.0,
        ],
    ];
    (v_operator_norm(sum), v_operator_norm(square))
}

/// #1433's premises for one word the kernel can load: it lies in a zone, and the zone's constants
/// (contraction, `||I + A||`, input column; for a settled design also the settled ones and
/// `||A^2 - I||`) hold for it.
fn assert_in_zone(zones: &LiveZones, words: &SvfWords, settled: bool, what: &str) {
    let re = pole_real(words);
    let zone = zones
        .zone_of(re)
        .unwrap_or_else(|| panic!("{what}: Re p {re} lies in no zone"));
    let constants = SectionConstants::of(words);
    let (sum, square) = sum_and_square_norms(words);
    assert!(
        constants.rho_kernel() <= zone.contraction
            && sum <= zone.sum_norm
            && constants.beta + constants.mu_input <= zone.input,
        "{what}: Re p {re}: contraction {}, ||I + A|| {sum}, input {} against zone {zone:?}",
        constants.rho_kernel(),
        constants.beta + constants.mu_input
    );
    if settled {
        let at = zone
            .settled
            .unwrap_or_else(|| panic!("{what}: a design in a zone without settled constants"));
        assert!(
            constants.rho_kernel() <= at.contraction
                && sum <= at.sum_norm
                && square <= at.square_norm,
            "{what}: Re p {re}: contraction {}, ||I + A|| {sum}, ||A^2 - I|| {square} against \
             settled {at:?}",
            constants.rho_kernel()
        );
    }
}

/// Gate 1(b) of #1329, extended by #1433's gate 1. Over 100,000 log-spaced cutoffs and the last
/// 65,536 `f32` cutoffs below each rate's maximum (a tenth of each in debug), for both sections:
/// no design exceeds the live envelope the live bound is computed from (contraction, input column,
/// output row, feedthrough, rounding included), every design lies in a pole zone whose constants
/// hold for it (#1433), the slowest designs are the worst-case pair's, a sampled design's own
/// bound never exceeds the live bound, and every recursion word the real kernel loads while
/// ramping between scanned designs (and under per-frame restarts at the top of the domain)
/// contracts within the live envelope's `rho_ramp < 1`, lies in a zone whose constants hold for
/// it, and moves its pole's real part by at most the zones' stated step from one frame to the
/// next.
#[test]
fn live_bound_covers_every_scanned_design_and_ramp_word() {
    for &rate in rates() {
        let terms = input_section_live_envelope(rate).expect("launch rate");
        let envelope = terms.envelope;
        let zones = live_zones(&terms).expect("live zones");
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
                assert_in_zone(
                    &zones,
                    &words,
                    true,
                    &format!("{rate} Hz section {section} at {hz} Hz"),
                );
                if section == 0 {
                    assert!(
                        math::tail::v_dual_norm([words.m1, words.m2]) <= terms.first_mix_row
                            && math::tail::output_rounding(&words).0 <= terms.first_output_rounding,
                        "{rate} Hz HPF at {hz} Hz: its mix row exceeds the live terms"
                    );
                }
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
                let own = bound(rate, hpf, lpf, 24.0);
                let rest = own.rest.expect("fixed rest");
                assert!(
                    finite(own.tail) <= finite(live.tail)
                        && finite(own.tail_every_peak) <= finite(live.tail_every_peak)
                        && rest.peak_plus_24_dbfs <= live_rest.peak_plus_24_dbfs
                        && rest.any_sanitized_input <= live_rest.any_sanitized_input,
                    "{rate} Hz HPF {hpf} LPF {lpf}: a design exceeds the live bound"
                );
            }
        }
        // Interior ramp words, read from the real kernel frame by frame: each inside the envelope
        // and its zone, and its pole's real part within the zones' step of the previous frame's
        // (an identity word, at rest, starts a new run: a rule-3 enable jumps with zero state).
        let mut worst_ramp_word = 0.0_f64;
        let mut largest_step = 0.0_f64;
        let mut check = |input: &InputBuiltins, section: usize, previous: &mut Option<f64>| {
            let words = input_section_words(input)[section];
            let [c1, a2, a3, _k, m0, m1, m2] = words.map(f32::from_bits);
            if m0 == 1.0 && m1 == 0.0 && m2 == 0.0 && c1 == 0.0 && a2 == 0.0 && a3 == 0.0 {
                *previous = None;
                return;
            }
            let words = SvfWords::from_f32([c1, a2, a3, m0, m1, m2]);
            let rho = kernel_contraction(&words);
            worst_ramp_word = worst_ramp_word.max(rho);
            assert!(
                rho <= envelope.rho_ramp,
                "{rate} Hz: a ramp word contracts by {rho} > rho_ramp {}",
                envelope.rho_ramp
            );
            assert_in_zone(&zones, &words, false, &format!("{rate} Hz ramp word"));
            if section == 0 {
                assert!(
                    math::tail::v_dual_norm([words.m1, words.m2]) <= terms.first_mix_row,
                    "{rate} Hz: an HPF ramp word's mix row exceeds the live terms"
                );
            }
            let re = pole_real(&words);
            if let Some(last) = *previous {
                let step = (re - last).abs();
                largest_step = largest_step.max(step);
                assert!(
                    step <= zones.step,
                    "{rate} Hz: a ramp word's pole moved by {step} > the zones' step {}",
                    zones.step
                );
            }
            *previous = Some(re);
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
            let mut previous = None;
            for &hz in to {
                input
                    .apply_prepared_filter(target(rate, section, hz))
                    .expect("retarget");
                for _ in 0..frames_per_target {
                    process(&mut input, &mut silence, &mut silence_right, sample);
                    sample += 1;
                    check(&input, section, &mut previous);
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
            // Disable, re-enable and retarget mid-crossfade (rules 2 to 4), per frame and every
            // 32 frames: frozen recursion words under a mix ramp, and ramps from them.
            let toggles: Vec<f32> = (0..3_072)
                .map(|index| [0.0, top, 10.0][index % 3])
                .collect();
            ramp(top, &toggles, section, 1);
            ramp(top, &toggles[..384], section, 32);
        }
        eprintln!(
            "1(b) {rate} Hz: worst scanned ramp word contraction - 1 = {:.5e} (rho_ramp - 1 = {:.5e}); \
             largest pole step {largest_step:.6} (zones' step {:.6}, {} zones)",
            worst_ramp_word - 1.0,
            envelope.rho_ramp - 1.0,
            zones.step,
            zones.zones.len()
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
                let kernel = input(rate, hpf, lpf, trim_db, false);
                let sections = kernel_sections(&kernel);
                let gain = trim_gain(&kernel);
                let prepared = self::bound(rate, hpf, lpf, trim_db);
                let bound: CascadeBound =
                    fixed_cascade(&sections, gain, &law, rest_peaks()).expect("bound");
                // (i) The prepared values, and T_rest's formula.
                assert_eq!(prepared.tail, TailSamples::Finite(bound.tail));
                assert_eq!(
                    prepared.tail_every_peak,
                    TailSamples::Finite(bound.tail_every_peak)
                );
                assert_eq!(
                    prepared.rest,
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

/// Gate 1(c) for the live bound (verdict m2). The live `T_rest` is `max(T_decay, R(P*))` with
/// `R(P*)` recomputed through the live rest bound at the peak `P*`, and at least `N_SILENCE`
/// (every live section can be enabled, so the A9 term applies).
#[test]
fn live_tail_every_peak_is_the_rest_at_the_flush_floor() {
    for rate in LAUNCH_RATES {
        let law = input_section_flush_law(rate);
        let cascade = input_section_live_cascade(rate).expect("launch rate");
        let live = input_section_live_bound(rate).expect("launch rate");
        let terms = input_section_live_envelope(rate).expect("launch rate");
        let p_star = cascade.flush_floor;
        let at_floor = live_cascade(
            &terms,
            rest_peaks()[0],
            &law,
            u64::from(INPUT_FILTER_RAMP_SAMPLES),
            [p_star, p_star],
        )
        .expect("live bound at the flush floor")
        .rest_peak;
        let t_rest = cascade.tail.max(at_floor);
        eprintln!(
            "1(c) live {rate} Hz: T_decay {}, P* {p_star:.4e}, R(P*) {at_floor}, T_rest {t_rest}",
            cascade.tail
        );
        assert_eq!(cascade.tail_every_peak, t_rest);
        assert_eq!(live.tail_every_peak, TailSamples::Finite(t_rest));
        assert!(t_rest >= law.silence_frames);
    }
}

// ---- Gate 2's terms and gate 3's A9 term, by an independent recomputation -------------------

/// `2 ||[[e1, e2], [-e2, e3]]||_V` at the worst vertex of the box `|e_i| <= box_[i]`, the
/// operator norm taken by the non-cancelling closed form on `R M R^-1` (`R = [[1, r], [0, r]]`).
fn independent_box_norm(box_: [f64; 3]) -> f64 {
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let mut worst = 0.0_f64;
    for signs in 0..8_u32 {
        let sign = |bit: u32| if signs & (1 << bit) == 0 { 1.0 } else { -1.0 };
        let (e1, e2, e3) = (sign(0) * box_[0], sign(1) * box_[1], sign(2) * box_[2]);
        // R M R^-1 for M = [[e1, e2], [-e2, e3]], with sqrt(2) r = 1.
        let (a, b, c, d) = (
            e1 - r * e2,
            core::f64::consts::SQRT_2 * e2 + e3 - e1 + r * e2,
            -r * e2,
            e3 + r * e2,
        );
        let sum = math::sqrt((a + d) * (a + d) + (c - b) * (c - b));
        let difference = math::sqrt((a - d) * (a - d) + (b + c) * (b + c));
        worst = worst.max(sum + difference);
    }
    worst
}

/// The live bound recomputed from #1433's derivation in plain `f64`, sharing no code with
/// `math::tail` (`docs/derivations/1329-input-section-tail-and-rest.md`, "#1433"): the pole
/// domain and the first section's mix row from first principles, the zones, the state bound
/// `Phi` and the potential `Psi` by plain fixed-point iteration, the window frame by frame, and
/// the settled phase per zone group in closed form. It takes #1329's envelope and rounding counts
/// from the module (#1329's gates check those).
struct LiveOracle {
    t_decay: u64,
    p_star: f64,
    rest_at_p_star: u64,
    rest_peak: u64,
    rest_any: u64,
    /// `sup Psi Phi`, `sup Psi' beta` and `sup Psi`, `sup q Phi` over the direct zones and
    /// `sup Phi`, each per unit of `gain * peak` and of `F` ([`math::tail::LiveZones`]).
    potential_state: [f64; 2],
    charge: [f64; 2],
    direct_output: [f64; 2],
    largest_state: [f64; 2],
    /// Every zone group's rest at +24 dBFS, with the quantities it reads.
    groups_peak: Vec<OracleGroupRest>,
}

/// One zone group's rest ([`math::tail::LiveGroupRest`]): `r`, `c_y`, `c_d`, the window's
/// second-section state, `H_0`, `X_0`, the first section's exact rest frame, the second section's
/// state bound at any later frame (from `N`) and the group's `R`.
struct OracleGroupRest {
    r: f64,
    cy: f64,
    cd: f64,
    window_state: f64,
    h0: f64,
    x0: f64,
    first_rested: u64,
    second_state_at: Box<dyn Fn(u64) -> f64>,
    rested: u64,
}

/// One zone of the oracle: `[a, b]`, ramp constants, settled constants, direct.
#[derive(Clone, Copy, Debug)]
struct OracleZone {
    a: f64,
    b: f64,
    rho: f64,
    q: f64,
    beta: f64,
    settled: Option<(f64, f64, f64)>,
    direct: bool,
}

/// One zone group's settled terms: `r`, `c_y`, `c_d`, `Phi` relative and per unit `F`.
#[derive(Clone, Copy, Debug)]
struct OracleTerms {
    r: f64,
    cy: f64,
    cd: f64,
    phi: [f64; 2],
}

fn live_oracle(rate: u32) -> LiveOracle {
    let terms = input_section_live_envelope(rate).expect("launch rate");
    let envelope = terms.envelope;
    let law = input_section_flush_law(rate);
    let sqrt2 = core::f64::consts::SQRT_2;
    let u = 1.0 / 16_777_216.0;
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let (r_norm, r_inv) = (math::sqrt(1.0 + r), 1.0 / math::sqrt(1.0 - r));
    let f_step =
        r_norm * core::f64::consts::SQRT_2 * (law.flush_eps + 16.0 * f64::from(f32::MIN_POSITIVE));
    let cap = r_norm * sqrt2 * f64::from(f32::MAX);
    let limit_rest = law.rest_eps / r_inv;
    let ramp = u64::from(INPUT_FILTER_RAMP_SAMPLES);
    let start = ramp + 1;
    let silence = law.silence_frames;
    let (rr, rs) = (envelope.rho_ramp, envelope.rho_settled);
    let (iota, os, oi) = (envelope.input, envelope.output_state, envelope.output_input);
    let (mu_ramp, mu_settled, mu_x) = (
        terms.poles.state_rounding,
        terms.poles.settled_state_rounding,
        terms.poles.input_rounding,
    );
    let omega = terms.first_output_rounding;

    // The pole domain: `Re p` of the exact designs, and the boxes (#1407's floor `E = 64 h + u D`).
    let g_max = math::tan(core::f64::consts::PI * f64::from(maximum(rate)) / f64::from(rate));
    let g_min = math::tan(core::f64::consts::PI * 10.0 / f64::from(rate));
    let real = |g: f64| (1.0 - g * g) / (1.0 + sqrt2 * g + g * g);
    let (low, high) = (real(g_max), real(g_min));
    let half = [u / 2.0, u / 4.0, u / 2.0];
    let ramp_box: [f64; 3] = core::array::from_fn(|i| 65.0 * half[i] + u * [1.0, 0.3, 1.0][i]);
    assert!(
        terms.poles.low <= low
            && terms.poles.high >= high
            && (0..3)
                .all(|i| terms.poles.ramp_box[i] >= ramp_box[i]
                    && terms.poles.design_box[i] >= half[i]),
        "{rate} Hz: the module's pole domain is narrower than the derivation's"
    );
    // The HPF's mix row `theta (-k, -1)` plus the mix ramp allowance `64 h + u D` per word.
    // The HPF band mix word `fl(sqrt(2))` (the kernel's `BUTTERWORTH_K`).
    let k = f64::from(core::f64::consts::SQRT_2 as f32);
    let dual = |c: [f64; 2]| {
        let second = -c[0] + sqrt2 * c[1];
        math::sqrt(c[0] * c[0] + second * second)
    };
    let mix = [64.0 * u + u * k, 32.0 * u + u];
    let mix_box = [[1.0, 1.0], [1.0, -1.0]]
        .iter()
        .map(|s| dual([s[0] * mix[0], s[1] * mix[1]]))
        .fold(0.0_f64, f64::max);
    let mix_row = dual([-k, -1.0]) + mix_box;
    assert!(
        terms.first_mix_row >= mix_row,
        "{rate} Hz: the module's mix row {} is below the derivation's {mix_row}",
        terms.first_mix_row
    );
    let half_row = 0.5 * mix_row;

    // The box's image on the pole: `Re p`, `|p|` and `kappa`.
    let shift = |b: [f64; 3]| {
        let dre = b[0] + b[2];
        let dim = 2.0 * sqrt2 * b[1] + b[0] + b[2];
        let dk = b[0] + b[2] + sqrt2 * b[1];
        (dre, math::sqrt(dre * dre + dim * dim), dk)
    };
    let (dre_r, dp_r, dk_r) = shift(ramp_box);
    let (dre_s, dp_s, dk_s) = shift(half);
    let (norm_r, norm_s) = (dp_r + sqrt2 * dk_r, dp_s + sqrt2 * dk_s);
    let arc = |x: f64| math::sqrt(2.0 - x * x) - 1.0;
    let radius = |x: f64| math::sqrt(x * x + arc(x) * arc(x));
    let plus = |x: f64| math::sqrt((1.0 + x) * (1.0 + x) + arc(x) * arc(x));
    let minus = |x: f64| math::sqrt((1.0 - x) * (1.0 - x) + arc(x) * arc(x));

    // The zone boundaries: widths from 1e-6, growing by 1.5 up to 0.02, from each end to 0.
    let (first, last) = (low - dre_r, high + dre_r);
    let mut left = vec![first];
    let (mut x, mut w) = (first, 1.0e-6);
    while x + w < 0.0 {
        x += w;
        left.push(x);
        w = (w * 1.5).min(0.02);
    }
    let mut right = vec![last];
    let (mut x, mut w) = (last, 1.0e-6);
    while x - w > 0.0 {
        x -= w;
        right.push(x);
        w = (w * 1.5).min(0.02);
    }
    left.push(0.0);
    left.extend(right.into_iter().rev());
    let zones: Vec<OracleZone> = left
        .windows(2)
        .filter_map(|edge| {
            let (a, b) = (edge[0], edge[1]);
            let (lo, hi) = ((a - dre_r).max(low), (b + dre_r).min(high));
            if lo > hi {
                return None;
            }
            let (ls, hs) = ((a - dre_s).max(low), (b + dre_s).min(high));
            let settled = (ls <= hs).then(|| {
                (
                    (radius(ls).max(radius(hs)) + norm_s + mu_settled).min(rs),
                    plus(hs) + norm_s,
                    minus(ls) * plus(hs)
                        + 2.0 * dp_s
                        + dp_s * dp_s
                        + 2.0 * sqrt2 * dk_s * (1.0 + dp_s)
                        + 2.0 * dk_s * dk_s,
                )
            });
            Some(OracleZone {
                a,
                b,
                rho: (radius(lo).max(radius(hi)) + norm_r + mu_ramp).min(rr),
                q: plus(hi) + norm_r,
                beta: (minus(lo) + norm_r + mu_x).min(iota),
                settled,
                direct: a >= 0.0,
            })
        })
        .collect();
    let step = (high - low + 2.0 * dre_r) / 64.0 + 32.0 * u;
    let neighbours: Vec<Vec<usize>> = zones
        .iter()
        .map(|z| {
            (0..zones.len())
                .filter(|&j| zones[j].b >= z.a - step && zones[j].a <= z.b + step)
                .collect()
        })
        .collect();

    // `Phi` (relative, per unit `F`) and `Psi`: least fixed points, plain iteration.
    let mut phi: Vec<[f64; 2]> = zones
        .iter()
        .map(|z| [z.beta / (1.0 - z.rho), 1.0 / (1.0 - z.rho)])
        .collect();
    let reward = |z: &OracleZone| if z.direct { 0.0 } else { z.q };
    let mut psi: Vec<f64> = zones.iter().map(|z| reward(z) / (1.0 - z.rho)).collect();
    for _ in 0..100_000 {
        let mut changed = false;
        for kz in 0..zones.len() {
            for &j in &neighbours[kz] {
                for (c, drive) in [(0, zones[j].beta), (1, 1.0)] {
                    let value = zones[j].rho * phi[j][c] + drive;
                    if value > phi[kz][c] * (1.0 + 1.0e-14) {
                        phi[kz][c] = value;
                        changed = true;
                    }
                }
                let value = reward(&zones[kz]) + zones[kz].rho * psi[j];
                if value > psi[kz] * (1.0 + 1.0e-14) {
                    psi[kz] = value;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let n = zones.len();
    let sup = |values: &mut dyn Iterator<Item = f64>| values.fold(0.0_f64, f64::max);
    let v = [
        sup(&mut (0..n).map(|i| psi[i] * phi[i][0])),
        sup(&mut (0..n).map(|i| psi[i] * phi[i][1])),
    ];
    let charge = [
        sup(&mut (0..n)
            .flat_map(|i| neighbours[i].iter().map(move |&j| (i, j)))
            .map(|(i, j)| psi[j] * zones[i].beta)),
        sup(&mut psi.iter().copied()),
    ];
    let direct = [
        sup(&mut (0..n)
            .filter(|&i| zones[i].direct)
            .map(|i| zones[i].q * phi[i][0])),
        sup(&mut (0..n)
            .filter(|&i| zones[i].direct)
            .map(|i| zones[i].q * phi[i][1])),
    ];
    let phi_max = [
        sup(&mut phi.iter().map(|p| p[0])),
        sup(&mut phi.iter().map(|p| p[1])),
    ];
    let q_max = sup(&mut zones.iter().map(|z| z.q));

    // The window: `(sigma at N + ramp + 1, E at N + ramp, outputs)`.
    let window = |g: f64, f: f64| -> (f64, f64, Vec<f64>) {
        let one = 1.0 - rr;
        let pot = v[0] * g + v[1] * f;
        let dir = direct[0] * g + direct[1] * f;
        let mut e = phi_max[0] * g + phi_max[1] * f;
        let mut kk = (charge[0] * g + charge[1] * f) / one;
        let mut kl = dir / one;
        let mut dd = (oi * g + f) / one;
        let mut om = omega * e / one;
        let mut fs = f / one;
        let sigma = |kk: f64, kl: f64, dd: f64, om: f64, fs: f64| {
            (iota * (half_row * (pot + kk + kl) + dd + om) + fs).min(cap)
        };
        let mut outputs = Vec::new();
        for frame in 0..=ramp {
            outputs.push(
                os * sigma(kk, kl, dd, om, fs) + oi * ((half_row * q_max + omega) * e + f) + f,
            );
            kk = rr * kk + charge[1] * f;
            kl = rr * kl + dir;
            dd = rr * dd + f;
            om = rr * om + omega * e;
            fs = rr * fs + f;
            if frame < ramp {
                e = rr * e + f;
            }
        }
        (sigma(kk, kl, dd, om, fs), e.min(cap), outputs)
    };

    // Settled groups: consecutive zones within one quarter octave of `1 - r`, plus the identity.
    let mut groups: Vec<OracleTerms> = Vec::new();
    let mut band = None;
    for (i, z) in zones.iter().enumerate() {
        let Some((rz, qs, q2)) = z.settled else {
            continue;
        };
        let t = OracleTerms {
            r: rz,
            cy: half_row * qs + omega,
            cd: half_row * q2 + os * mu_settled + omega * (1.0 + rz),
            phi: phi[i],
        };
        let this = (4.0 * math::log2(1.0 - rz)) as i32;
        match groups.last_mut() {
            Some(g) if band == Some(this) => {
                g.r = g.r.max(t.r);
                g.cy = g.cy.max(t.cy);
                g.cd = g.cd.max(t.cd);
                g.phi = [g.phi[0].max(t.phi[0]), g.phi[1].max(t.phi[1])];
            }
            _ => groups.push(t),
        }
        band = Some(this);
    }
    groups.push(OracleTerms {
        r: 0.0,
        cy: 0.0,
        cd: 0.0,
        phi: [0.0, 0.0],
    });

    // The settled phase in closed form, `m` frames after `N + ramp + 1`: returns
    // `(tau, H_(n-1), X)` with and without the flush drive.
    let closed =
        move |t: &OracleTerms, tau0: f64, h0: f64, x0: f64, f: f64, m: u64| -> (f64, f64, f64) {
            let mf = m as f64;
            let a_h = rs * t.cd + mu_settled * t.cy + mu_x * t.cy * t.r;
            let a_f = rs * (os + 2.0 + omega) + mu_settled + mu_x * (t.cy + 1.0) + 1.0;
            let st = f / (1.0 - t.r);
            let rho_m = math::pow(rs, mf);
            let r_m = math::pow(t.r, mf);
            let geometric = if t.r == rs {
                mf * math::pow(rs, mf - 1.0)
            } else {
                (rho_m - r_m) / (rs - t.r)
            };
            let tau = rho_m * tau0
                + a_h * ((h0 - st) * geometric + st * (1.0 - rho_m) / (1.0 - rs))
                + a_f * f * (1.0 - rho_m) / (1.0 - rs);
            (tau, r_m * (h0 - st) + st, math::pow(rs.max(t.r), mf) * x0)
        };
    let frames_to = |z: f64, rho: f64, f: f64| -> u64 {
        let st = f / (1.0 - rho);
        assert!(st < limit_rest, "{rate} Hz: a stall above REST_EPS");
        if z < limit_rest {
            return 0;
        }
        // The first `m` with `rho^m (z - st) + st < limit`.
        let mut m = (math::log((limit_rest - st) / (z - st)) / math::log(rho)).floor() as u64;
        while math::pow(rho, m as f64) * (z - st) + st >= limit_rest {
            m += 1;
        }
        m
    };
    let group_rests = |g: f64| -> Vec<OracleGroupRest> {
        let f = f_step;
        let (sigma, energy, _) = window(g, f);
        groups
            .iter()
            .map(|t| {
                let t = *t;
                let h0 = energy.min(t.phi[0] * g + t.phi[1] * f).min(cap);
                let x0 = 2.0 * t.cy * h0;
                let first = frames_to(h0, t.r, f);
                let first_rested = ramp + first + silence;
                let st = f / (1.0 - t.r);
                let state_at = move |frame: u64| {
                    let (tau, h, x) = closed(&t, sigma + t.cy * h0 + f, h0, x0, f, frame - start);
                    (tau + t.cy * h + f + x + 2.0 * (t.cy * st + f)).min(cap)
                };
                let state = state_at(first_rested);
                OracleGroupRest {
                    r: t.r,
                    cy: t.cy,
                    cd: t.cd,
                    window_state: sigma,
                    h0,
                    x0,
                    first_rested,
                    second_state_at: Box::new(state_at),
                    rested: first_rested + frames_to(state, rs, f) + silence,
                }
            })
            .collect()
    };
    let rest = |g: f64| -> u64 {
        group_rests(g)
            .iter()
            .map(|group| group.rested)
            .max()
            .expect("groups")
    };
    // The relative output in the settled phase (no flush), `m` frames after `N + ramp + 1`.
    let settled_output = |t: &OracleTerms, tau0: f64, h0: f64, m: u64| -> f64 {
        let (tau, h, x) = closed(t, tau0, h0, 2.0 * t.cy * h0, 0.0, m);
        os * (tau + t.cy * h + x) + oi * t.cy * t.r * h
    };
    let gain = rest_peaks()[0] * (1.0 + u);
    let limit = TAIL_FLOOR / 2.0;
    let (sigma, energy, outputs) = window(gain, 0.0);
    let mut t_decay = outputs
        .iter()
        .rposition(|value| *value >= limit)
        .map_or(0, |frame| frame as u64 + 1);
    for t in &groups {
        let h0 = energy.min(t.phi[0] * gain);
        let tau0 = sigma + t.cy * h0;
        // `G(m)` stops rising once `r^m <= (1 - rho) G(m)`; from there every term falls.
        let falling = |m: u64| {
            let (tau, h, _) = closed(t, tau0, h0, 0.0, 0.0, m);
            let next = closed(t, tau0, h0, 0.0, 0.0, m + 1);
            next.0 <= tau && next.1 <= h
        };
        let (mut lo, mut hi) = (0_u64, 1_u64);
        if falling(0) {
            hi = 0;
        }
        while hi > 0 && !falling(hi) {
            lo = hi;
            hi *= 2;
        }
        while hi > lo + 1 {
            let mid = (lo + hi) / 2;
            if falling(mid) { hi = mid } else { lo = mid }
        }
        let from = hi;
        if settled_output(t, tau0, h0, from) < limit {
            continue;
        }
        let (mut lo, mut hi) = (from, from + 1);
        while settled_output(t, tau0, h0, hi) >= limit {
            lo = hi;
            hi = from + 2 * (hi - from);
        }
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if settled_output(t, tau0, h0, mid) >= limit {
                lo = mid
            } else {
                hi = mid
            }
        }
        t_decay = t_decay.max(start + hi);
    }
    // The flush's part of the output from `T_decay` on: the fixed point plus the decaying start.
    let (sigma_a, energy_a, outputs_a) = window(0.0, f_step);
    let mut stall = outputs_a
        .iter()
        .skip(t_decay as usize)
        .fold(0.0_f64, |s, value| s.max(*value));
    for t in &groups {
        let f = f_step;
        let a_h = rs * t.cd + mu_settled * t.cy + mu_x * t.cy * t.r;
        let a_f = rs * (os + 2.0 + omega) + mu_settled + mu_x * (t.cy + 1.0) + 1.0;
        let st = f / (1.0 - t.r);
        let tau_star = (a_h * st + a_f * f) / (1.0 - rs);
        let constant = f + 2.0 * (t.cy * st + f);
        let fixed = os * (tau_star + t.cy * st + constant) + oi * (t.cy * (t.r * st + f) + f) + f;
        let h0 = energy_a.min(t.phi[1] * f);
        let decaying = settled_output(
            t,
            sigma_a + t.cy * h0 + f,
            h0,
            t_decay.saturating_sub(start),
        );
        stall = stall.max(fixed + decaying);
    }
    let p_star = stall / limit;
    LiveOracle {
        t_decay,
        p_star,
        rest_at_p_star: rest(gain * p_star),
        rest_peak: rest(gain * rest_peaks()[0]),
        rest_any: rest(gain * rest_peaks()[1]),
        potential_state: v,
        charge,
        direct_output: direct,
        largest_state: phi_max,
        groups_peak: group_rests(gain * rest_peaks()[0]),
    }
}

/// The relative tolerance of the intermediate quantities' comparison with the recomputation: far
/// above the module's own inflation and the recomputation's iteration stop (at most `2.1e-6`, the
/// direct zones' flush charge `K_L`), far below what a dropped neighbour or term moves (the pole
/// step's use: `K_L` by `1.8e-3`, the window's second-section state by `4.3e-4`).
const INTERMEDIATE_TOLERANCE: f64 = 1.0e-5;
/// The same for the second section's state at the first section's rest frame, which the module
/// carries through a table of squared powers rounded up at every product (at most `1.3e-3` above
/// the recomputation's closed form at the launch rates): the module may only be larger.
const REST_STATE_TOLERANCE: f64 = 4.0e-3;

/// Gate 2's terms and gate 3's A9 term (#1329 Amendment 4, R3), and #1433's independent
/// recomputation. The real kernel cannot show that the live bound omits the `f32` rounding, the
/// ramp in flight, the flush stall or the A9 term: the bound leaves about 260,000 samples between
/// the live `T_decay` and the kernel's last output above `P eps`, and about 85,000 between `R` and
/// its rest. So this test recomputes the derivation independently.
///
/// * The envelope's contractions carry at least the exact top design's radius, the `f32` design
///   box `P(h)`, the state step's final rounding `kappa u rho` (Amendment 2) and, for `rho_ramp`,
///   the ramp allowance `P(E)` with `E = 64 h + u D` (#1407).
/// * `P*`, `T_decay`, `T_rest` and both rest bounds lie between the recomputation of #1433's
///   derivation ([`live_oracle`]: pole domain, zones, `Phi`, `Psi`, the window frame by frame and
///   the settled phase in closed form, no shared code) and that value plus `0.01 %` and 64 frames:
///   the module may only be more conservative, by its own rounding inflation, never optimistic,
///   and never loose.
/// * The envelope's output row carries the 10 Hz design's `||c||_V*`, its first-order word and
///   mix perturbations and the output rounding's supremum over every word (#1433 verdict m1).
/// * The intermediate quantities agree within [`INTERMEDIATE_TOLERANCE`]: `sup Psi Phi`, the
///   charges `K`, the direct zones' `K_L` and `sup Phi` ([`math::tail::LiveZones`]), and per zone
///   group at +24 dBFS ([`math::tail::live_cascade_groups`]) `r`, `c_y`, `c_d`, the window's
///   second-section state, `H_0` and the joint flush's `X_0`; the first section's rest frame and
///   the second section's state there within [`REST_STATE_TOLERANCE`]. Terms such as `X_0` and the
///   neighbour radius' use of the pole step move the final figures by at most a frame (#1433
///   verdict m2), so only these comparisons defend them.
#[test]
fn live_bound_carries_every_term_an_independent_recomputation_requires() {
    let u = 1.0 / 16_777_216.0;
    let kappa = 1.0 + core::f64::consts::SQRT_2;
    for rate in LAUNCH_RATES {
        let envelope = input_section_live_envelope(rate)
            .expect("launch rate")
            .envelope;
        let g_max = math::tan(core::f64::consts::PI * f64::from(maximum(rate)) / f64::from(rate));
        let radius = math::sqrt(1.0 + g_max * g_max * g_max * g_max)
            / (1.0 + core::f64::consts::SQRT_2 * g_max + g_max * g_max);
        let design_box = independent_box_norm([u / 2.0, u / 4.0, u / 2.0]);
        let ramp_box = independent_box_norm([33.0 * u, 16.3 * u, 33.0 * u]);
        let rounding = kappa * u * radius;
        let settled = radius + design_box + rounding;
        let ramp = settled + ramp_box;
        eprintln!(
            "R3 {rate} Hz: rho_settled - 1 {:.6e} (needs {:.6e}), rho_ramp - 1 {:.6e} (needs \
             {:.6e})",
            envelope.rho_settled - 1.0,
            settled - 1.0,
            envelope.rho_ramp - 1.0,
            ramp - 1.0
        );
        assert!(
            envelope.rho_settled >= settled && envelope.rho_ramp >= ramp,
            "{rate} Hz: the live envelope omits a term the derivation requires"
        );
        assert!(
            envelope.rho_ramp - ramp <= 8.0 * u * kappa,
            "{rate} Hz: the live envelope carries more than its rounding count"
        );
        // The output row (#1329 D5, #1433 verdict m1): the 10 Hz design's `||c||_V*`, the
        // first-order row perturbation of the words' `E + h`, the mix words' ramp allowance and
        // `|fl(sqrt(2)) - sqrt(2)|`, and the output rounding's supremum over every word, which the
        // scan above checks against every design (the value at the largest words alone is not
        // one).
        let terms = input_section_live_envelope(rate).expect("launch rate");
        let sqrt2 = core::f64::consts::SQRT_2;
        let k = f64::from(sqrt2 as f32);
        let k_error = (k - sqrt2).abs();
        let a2_max = 1.0 / (2.0 + sqrt2);
        let half = [u / 2.0, u / 4.0, u / 2.0];
        let off: [f64; 3] = core::array::from_fn(|i| 65.0 * half[i] + u * [1.0, 0.3, 1.0][i]);
        let (mix_m1, mix_m2) = (64.0 * u + u * k, 32.0 * u + u);
        let row = [
            k * off[0] + off[1] + mix_m1 + mix_m2 * a2_max + k_error,
            k * off[1] + off[2] + mix_m1 * a2_max + mix_m2 + k_error * a2_max,
        ];
        let second = row[0] + sqrt2 * row[1];
        let row_norm = math::sqrt(row[0] * row[0] + second * second);
        let g_min = math::tan(core::f64::consts::PI * 10.0 / f64::from(rate));
        let gamma_10 = sqrt2 / math::sqrt(1.0 + g_min * (g_min + sqrt2));
        let output_row = gamma_10 + row_norm + terms.first_output_rounding;
        eprintln!(
            "R3 {rate} Hz: output row {:.12e} (needs {output_row:.12e}; omega {:.6e})",
            envelope.output_state, terms.first_output_rounding
        );
        assert!(
            envelope.output_state >= output_row * (1.0 - 1.0e-12),
            "{rate} Hz: the live envelope's output row omits a term the derivation requires"
        );
        let oracle = live_oracle(rate);
        let cascade = input_section_live_cascade(rate).expect("launch rate");
        let live = input_section_live_bound(rate).expect("launch rate");
        let rest = live.rest.expect("live rest");
        let t_rest = oracle.t_decay.max(oracle.rest_at_p_star);
        eprintln!(
            "R3 {rate} Hz: recomputed T_decay {} (module {}), P* {:.6e} ({:.6e}), R(P*) {} ({}), \
             T_rest {t_rest} ({}), peak_plus_24_dbfs {} ({}), any_sanitized_input {} ({})",
            oracle.t_decay,
            cascade.tail,
            oracle.p_star,
            cascade.flush_floor,
            oracle.rest_at_p_star,
            cascade.rest_at_flush_floor,
            cascade.tail_every_peak,
            oracle.rest_peak,
            rest.peak_plus_24_dbfs,
            oracle.rest_any,
            rest.any_sanitized_input
        );
        assert!(
            cascade.flush_floor >= oracle.p_star
                && cascade.flush_floor <= oracle.p_star * 1.000_001,
            "{rate} Hz: P* {} against the recomputed {}",
            cascade.flush_floor,
            oracle.p_star
        );
        // The intermediate quantities, at a tight relative tolerance: terms that move the final
        // figures by at most a frame at the launch configuration (the neighbour radius' use of
        // the pole step, the joint flush's one-off `X_0`) still move these.
        let zones = live_zones(&terms).expect("live zones");
        let close = |name: &str, module: f64, recomputed: f64| {
            eprintln!(
                "R3 {rate} Hz {name}: module {module:.12e}, recomputed {recomputed:.12e}, ratio \
                 {:.3e}",
                module / recomputed - 1.0
            );
            assert!(
                (module - recomputed).abs() <= INTERMEDIATE_TOLERANCE * recomputed.abs(),
                "{rate} Hz {name}: module {module:e} against the recomputed {recomputed:e}"
            );
        };
        for c in 0..2 {
            close(
                "sup Psi Phi",
                zones.potential_state[c],
                oracle.potential_state[c],
            );
            close("K", zones.charge[c], oracle.charge[c]);
            close("K_L", zones.direct_output[c], oracle.direct_output[c]);
            close("sup Phi", zones.largest_state[c], oracle.largest_state[c]);
        }
        let groups = live_cascade_groups(
            &terms,
            rest_peaks()[0],
            &input_section_flush_law(rate),
            u64::from(INPUT_FILTER_RAMP_SAMPLES),
            rest_peaks()[0],
        )
        .expect("live groups");
        assert!(groups.len() > 1, "{rate} Hz: fewer than two zone groups");
        for group in &groups {
            // The module keeps only the groups no other group covers; each is one of the
            // oracle's, which keeps them all.
            let found = oracle
                .groups_peak
                .iter()
                .min_by(|a, b| {
                    (a.r - group.contraction)
                        .abs()
                        .total_cmp(&(b.r - group.contraction).abs())
                })
                .expect("oracle groups");
            close("group r", group.contraction, found.r);
            close("group c_y", group.output, found.cy);
            close("group c_d", group.change, found.cd);
            close("window sigma", group.window_state, found.window_state);
            close("group H_0", group.energy, found.h0);
            close("group X_0", group.one_off, found.x0);
            // The module's frame count may only be later, by its contraction's inflation and
            // the rounding of its `log`.
            assert!(
                group.first_rested >= found.first_rested
                    && group.first_rested <= found.first_rested + found.first_rested / 10_000 + 2,
                "{rate} Hz: the group at r = {} rests its first section at {} against the \
                 recomputed {}",
                group.contraction,
                group.first_rested,
                found.first_rested
            );
            let recomputed = (found.second_state_at)(group.first_rested);
            eprintln!(
                "R3 {rate} Hz sigma at the HPF's rest frame {}: module {:.12e}, recomputed \
                 {recomputed:.12e}, ratio {:.3e}",
                group.first_rested,
                group.second_state,
                group.second_state / recomputed - 1.0
            );
            assert!(
                group.second_state >= recomputed
                    && group.second_state <= recomputed * (1.0 + REST_STATE_TOLERANCE),
                "{rate} Hz: the group at r = {} has the second section's state {:e} at the first \
                 section's rest against the recomputed {recomputed:e}",
                group.contraction,
                group.second_state
            );
            assert!(
                group.rested >= found.rested && group.rested <= found.rested + 64,
                "{rate} Hz: the group at r = {} rests at {} against the recomputed {}",
                group.contraction,
                group.rested,
                found.rested
            );
        }
        for (name, module, recomputed) in [
            ("T_decay", finite(live.tail), oracle.t_decay),
            ("T_rest", finite(live.tail_every_peak), t_rest),
            ("R(P*)", cascade.rest_at_flush_floor, oracle.rest_at_p_star),
            (
                "peak_plus_24_dbfs",
                rest.peak_plus_24_dbfs,
                oracle.rest_peak,
            ),
            (
                "any_sanitized_input",
                rest.any_sanitized_input,
                oracle.rest_any,
            ),
        ] {
            assert!(
                module >= recomputed && module <= recomputed + recomputed / 10_000 + 64,
                "{rate} Hz {name}: module {module} against the recomputed {recomputed}"
            );
        }
    }
}

// ---- Gate 2: the live bound on the real kernel at the domain extreme -------------------------

/// The extreme pair's `f64` impulse response, for the adversarial input's signs.
fn extreme_response(rate: u32, frames: usize) -> Vec<f64> {
    let (hpf, lpf) = input_section_worst_case_pair(rate).expect("launch rate");
    impulse_response(&kernel_sections(&input(rate, hpf, lpf, 0.0, false)), frames)
}

/// Gate 2 of #1329, with #1433's gate 2. At the domain extreme of D5, every launch rate, trim
/// +24 dB, input peaks 1, +24 dBFS
/// and `1e29`: the worst-case pair designed for the whole history, driven by an input that
/// maximises the output at `N + T_decay` (the reversed signs of the impulse response there, over
/// the last 1,000,000 samples), with a live HPF target (one `f32` lower) applied 32 samples before
/// `N`; and a block-size-1 history that re-sends and alternates the worst-case pair with disable,
/// a disable in flight at `N`. Every history keeps a finite state (no non-finite recovery fires,
/// so no run is at rest by the recovery's reset). From `N + T_decay` until exact rest every output
/// is below `P * eps`; from `N + R` on (`R` the live rest bound for the peak) every output is
/// `+-0.0` and every integrator equals a freshly reset section's. One run inverts polarity: the
/// section's output mix normalizes `-0.0`, so it shows `+0.0`, which D2 admits (#1329 Amendment
/// 4, R4). Also measures the fixed top pair's real rest under an alternating input (gate 3's
/// lower side), and at +24 dBFS its exact rest frame `R_meas`: the certified `peak_plus_24_dbfs`
/// lies in `[R_meas, 1.15 R_meas]` (#1433 gate 2, the stated margin).
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
            let mut section = input(rate, hpf, lpf, 24.0, polarity_invert);
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
            assert_eq!(
                section.lifetime_recovered_state(),
                (0, 0),
                "{rate} Hz P {peak}: the history overflowed, so the run would test the recovery"
            );
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
            assert!(
                quantum_one || observed.above > 0,
                "{rate} Hz P {peak}: the history left no tail to bound"
            );
        }
        // Gate 3's lower side: the fixed top pair's real rest under an alternating input through
        // the +24 dB trim stays at or below the certified bound, at +24 dBFS
        // (`peak_plus_24_dbfs`) and at `1e29` (`any_sanitized_input`). The alternating drive
        // holds the state near its largest finite values (about `4e34`) for the whole history.
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
            if peak == plus_24 {
                // #1433 gate 2: the exact rest frame (the same history again, the last blocks
                // frame by frame), and the certified bound within 15 % above it.
                let exact = exact_rest_of_top_pair(rate, peak, measured);
                let ratio = bound as f64 / exact as f64;
                eprintln!(
                    "1433-2 {rate} Hz: R_meas {exact} (exact frame), peak_plus_24_dbfs {bound}, \
                     ratio {ratio:.4}"
                );
                assert!(
                    exact <= bound && bound * 100 <= exact * 115,
                    "{rate} Hz: peak_plus_24_dbfs {bound} against the real rest {exact} \
                     (ratio {ratio:.4}, margin 1.15)"
                );
            }
        }
    }
}

/// The exact frame (counted from `N`) from which the fixed top pair at the +24 dB trim, after an
/// alternating input of `peak` for `HISTORY` frames, is at rest: every integrator `+-0.0`, so every
/// later output is zero. `block_rest` is the 64-frame block end at which the same run was first
/// seen at rest; the run is repeated (the kernel is deterministic and partition-invariant) and its
/// last blocks are processed frame by frame.
fn exact_rest_of_top_pair(rate: u32, peak: f32, block_rest: u64) -> u64 {
    const HISTORY: usize = 1_000_000;
    let (hpf, lpf) = input_section_worst_case_pair(rate).expect("launch rate");
    let mut section = input(rate, hpf, lpf, 24.0, false);
    let mut left: Vec<f32> = (0..HISTORY)
        .map(|index| if index % 2 == 0 { peak } else { -peak })
        .collect();
    let mut right = left.clone();
    process(&mut section, &mut left, &mut right, 0);
    // The rest frame lies after the block end before `block_rest`, so after `safe`.
    let safe = block_rest.saturating_sub(128) / 64 * 64;
    let mut done = 0_u64;
    let (mut l, mut r) = ([0.0_f32; 64], [0.0_f32; 64]);
    while done < safe {
        l.fill(0.0);
        r.fill(0.0);
        process(&mut section, &mut l, &mut r, HISTORY as u64 + done);
        done += 64;
    }
    assert!(
        !state_is_rest(&section),
        "{rate} Hz: at rest before the block it was seen in"
    );
    while !state_is_rest(&section) {
        let (mut l, mut r) = ([0.0_f32; 1], [0.0_f32; 1]);
        process(&mut section, &mut l, &mut r, HISTORY as u64 + done);
        done += 1;
        assert!(
            done <= block_rest,
            "{rate} Hz: not at rest by the block it was seen in"
        );
    }
    done
}

// ---- Gate 3: the restated contract figures ---------------------------------------------------

/// #1329's certified live values per launch rate (44.1 / 48 / 88.2 / 96 kHz, its Amendment 3 and
/// attempt-5 record): `T_decay`, `peak_plus_24_dbfs`, `any_sanitized_input`. #1433's tighter bound
/// may only lower them (its gate 1).
const CRUDE_T_DECAY: [u64; 4] = [904_785, 899_524, 904_795, 899_533];
const CRUDE_PEAK_PLUS_24_DBFS: [u64; 4] = [1_264_736, 1_257_840, 1_268_585, 1_262_029];
const CRUDE_ANY_SANITIZED_INPUT: [u64; 4] = [2_583_197, 2_569_016, 2_587_000, 2_573_156];

/// Gate 3 of #1329 and gate 1's ceilings of #1433. The live exact-rest bounds are at or below
/// decision 15's restated D15-4 figures (#1433 D3: about 1.07M at +24 dBFS and 2.39M for any
/// sanitized input, each including `2 * N_SILENCE`), and the live `T_decay`,
/// `peak_plus_24_dbfs` and `any_sanitized_input` are each at most #1329's certified values at
/// the same rate. The lower side, against the real kernel's rest, is gate 2's run.
#[test]
fn live_rest_bounds_are_within_the_restated_contract_figures() {
    for (index, rate) in LAUNCH_RATES.into_iter().enumerate() {
        let cascade = input_section_live_cascade(rate).expect("launch rate");
        let live = input_section_live_bound(rate).expect("launch rate");
        let rest = live.rest.expect("live rest");
        eprintln!(
            "3 {rate} Hz: T_decay {} (crude {}), T_rest {}, peak_plus_24_dbfs {} (crude {}), \
             any_sanitized_input {} (crude {}), P* {:.4e}",
            finite(live.tail),
            CRUDE_T_DECAY[index],
            finite(live.tail_every_peak),
            rest.peak_plus_24_dbfs,
            CRUDE_PEAK_PLUS_24_DBFS[index],
            rest.any_sanitized_input,
            CRUDE_ANY_SANITIZED_INPUT[index],
            cascade.flush_floor
        );
        assert!(rest.peak_plus_24_dbfs <= 1_075_000);
        assert!(rest.any_sanitized_input <= 2_390_000);
        assert!(rest.peak_plus_24_dbfs >= 2 * u64::from(lane::silence_frames(rate)));
        assert!(
            finite(live.tail) <= CRUDE_T_DECAY[index]
                && rest.peak_plus_24_dbfs <= CRUDE_PEAK_PLUS_24_DBFS[index]
                && rest.any_sanitized_input <= CRUDE_ANY_SANITIZED_INPUT[index],
            "{rate} Hz: the live bound is above #1329's crude bound"
        );
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
