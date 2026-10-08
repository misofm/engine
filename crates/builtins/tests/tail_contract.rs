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
#[cfg(feature = "test-support")]
use builtins::input_section_bound_charged;
use builtins::test_support::{input_section_words, input_state_words, input_trim_words};
use builtins::{
    BuiltinChain, BuiltinLaneSelector, BuiltinParameters, ChannelParameters, DualMonoBlock,
    InputBoundCache, InputBuiltins, PreparedInputFilterTarget, builtin_filter_cutoff_maximum_hz,
    input_section_bound, input_section_bounds, input_section_flush_law, input_section_live_bound,
    input_section_live_bound_table, input_section_live_cascade, input_section_live_envelope,
    input_section_worst_case_pair, prepare_input_filter_pair,
};
use effect_contract::{
    FlushStall, NodeTailBound, PeakGain, RestBound, RestSamples, TailDecay, TailSamples,
};
use math::tail::{
    CascadeBound, LiveComposition, LiveZones, SectionConstants, SvfWords, TAIL_FLOOR,
    fixed_cascade, live_cascade, live_cascade_composition, live_cascade_crossing,
    live_cascade_groups, live_zones, pole_real, v_operator_norm,
};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// The stated exact-rest bound of `rest`; panics with `message` when it is unstated.
fn stated_rest(rest: RestBound, message: &str) -> RestSamples {
    match rest {
        RestBound::Bounded(rest) => rest,
        RestBound::Unstated => panic!("{message}"),
    }
}

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
fn bound(rate: u32, hpf: f32, lpf: f32, trim_db: f32) -> NodeTailBound {
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
                    assert_eq!(prepared.rest, RestBound::Bounded(RestSamples::ZERO));
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
    let own: Vec<NodeTailBound> = strips
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
#[cfg(feature = "test-support")]
#[test]
fn the_live_bound_is_taken_exactly_when_the_budget_is_exhausted() {
    use builtins::test_support::input_section_bounds_within;
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
        assert!(
            design.charge > design.frames,
            "design {index} has a fixed charge"
        );
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
            let after = before + charged[index].charge;
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
                let fits = !exhausted && covered + charged[index].charge <= budget;
                if fits {
                    covered += charged[index].charge;
                } else {
                    exhausted = true;
                }
                value.insert(index, if fits { charged[index].bound } else { live });
            }
            let expected: Vec<NodeTailBound> = order.iter().map(|index| value[index]).collect();
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

/// #1457 gate 7 at the bound level (attempt 1, MJ2): a design's computation walks at most the
/// budget it is given, counted frame by frame by the walk itself, finished or stopped. Each
/// channel cascade reserves `INPUT_BOUND_SECTION_CHARGE` per section before it walks to the rest
/// of the budget: a stopped cascade walks exactly what is left after its reservation, and nothing
/// when the budget does not cover that. The right channel of a design whose channels differ gets what the left
/// channel's charge left. The near-top pairs' majorant passes are hundreds of thousands of frames,
/// so a pass that counted frames but did not stop at the horizon would walk far past every small
/// budget here.
#[cfg(feature = "test-support")]
#[test]
fn a_design_walks_at_most_the_budget_it_is_given() {
    use builtins::INPUT_BOUND_SECTION_CHARGE;
    use builtins::test_support::{fixed_input_frames_walked, input_section_bounds_within};
    let cascade = 2 * INPUT_BOUND_SECTION_CHARGE;
    for &rate in rates() {
        let maximum = builtin_filter_cutoff_maximum_hz(rate).expect("launch rate");
        let live = input_section_live_bound_table(rate).expect("launch rate");
        let left = parameters(below(maximum), maximum, 24.0, false);
        let right = parameters(below(below(maximum)), maximum, 12.0, false);
        let split = BuiltinParameters {
            left: left.left,
            right: right.right,
            ..left
        };
        let [left_alone, right_alone, split_charged] = [left, right, split]
            .map(|design| input_section_bound_charged(rate, design).expect("bound"));
        // A channel cascade's charge is its frames plus two sections, and a design whose channels
        // differ is its two cascades.
        assert_eq!(left_alone.charge, cascade + left_alone.frames);
        assert_eq!(
            (split_charged.frames, split_charged.charge),
            (
                left_alone.frames + right_alone.frames,
                left_alone.charge + right_alone.charge
            )
        );
        // The frames a design of these cascades walks under `budget` when it is stopped.
        let stopped_walk = |budget: u64, channels: &[u64]| -> u64 {
            let mut left_over = budget;
            let mut walked = 0;
            for &frames in channels {
                let horizon = left_over.saturating_sub(cascade);
                if horizon < frames {
                    return walked + horizon;
                }
                walked += frames;
                left_over -= cascade + frames;
            }
            unreachable!("the design is stopped");
        };
        let cases = [
            (left, left_alone, std::vec![left_alone.frames]),
            (
                split,
                split_charged,
                std::vec![left_alone.frames, right_alone.frames],
            ),
        ];
        for (design, charged, channels) in cases {
            let mut budgets = std::vec![
                1,
                cascade - 1,
                cascade,
                cascade + 1,
                cascade + 1_000,
                charged.charge / 2,
                charged.charge - 1,
                charged.charge,
            ];
            if channels.len() == 2 {
                budgets.extend([
                    left_alone.charge - 1,
                    left_alone.charge,
                    left_alone.charge + cascade + 1_000,
                ]);
            }
            for budget in budgets {
                let what = format!(
                    "{rate} Hz, {} channel walks, budget {budget}",
                    channels.len()
                );
                let before = fixed_input_frames_walked();
                let bounds =
                    input_section_bounds_within(rate, [design], budget, None).expect("bounds");
                let walked = fixed_input_frames_walked() - before;
                assert!(walked <= budget, "{what}: walked {walked}");
                if budget >= charged.charge {
                    assert_eq!(
                        (bounds[0], walked),
                        (charged.bound, charged.frames),
                        "{what}"
                    );
                } else {
                    assert_eq!(bounds[0], live, "{what}");
                    assert_eq!(walked, stopped_walk(budget, &channels), "{what}");
                }
            }
        }
    }
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
        let live_rest = stated_rest(live.rest, "live rest");
        let sampled = scan
            .iter()
            .step_by(scan.len() / 16)
            .copied()
            .chain(scan.iter().rev().take(4).copied());
        for hz in sampled {
            for (hpf, lpf) in [(hz, 0.0), (0.0, hz)] {
                let own = bound(rate, hpf, lpf, 24.0);
                let rest = stated_rest(own.rest, "fixed rest");
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
                    RestBound::Bounded(RestSamples {
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

/// The settled contraction the derivation requires at `rate` (#1329 Amendment 2, D5): the exact
/// top design's radius, the `f32` design box `P(h)` and the state step's final rounding
/// `kappa u rho`.
fn settled_contraction(rate: u32) -> f64 {
    let u = 1.0 / 16_777_216.0;
    let kappa = 1.0 + core::f64::consts::SQRT_2;
    let g_max = math::tan(core::f64::consts::PI * f64::from(maximum(rate)) / f64::from(rate));
    let radius = math::sqrt(1.0 + g_max * g_max * g_max * g_max)
        / (1.0 + core::f64::consts::SQRT_2 * g_max + g_max * g_max);
    radius + independent_box_norm([u / 2.0, u / 4.0, u / 2.0]) + kappa * u * radius
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
    /// #1466's composition values (`docs/derivations/1379-graph-tail-composition.md`, "The live
    /// input section"): `D` by each group's certificate over the full rate grid, `g_p` along the
    /// LPF's own zones (#1485), `sigma_p` the flush part with the tail at frame `0`, `sigma_t` the
    /// flush part from `T_decay` on.
    decay: u64,
    peak_gain: f64,
    peak_stall: f64,
    tail_stall: f64,
    /// The tail gain (`docs/derivations/1379-graph-tail-composition.md`, "The live tail gain
    /// `G_t`", #1485): the larger of its window part (the late part along the in-flight ramps)
    /// and its settled part (every pair of settled zones), both kept as evidence.
    tail_gain: f64,
    tail_window: f64,
    tail_settled: f64,
    /// `G_p`'s split (evidence): `m0`, `Gamma(1)`, `Gammabar(m0)`, `A` and `C`.
    peak_split: (usize, f64, f64, f64, f64),
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
    // #1467: the LPF's mix row `theta (0, 1)` plus the same allowance, and its output rounding on
    // the state (the module's supremum over every word, as `omega`).
    let second_row = dual([0.0, 1.0]) + mix_box;
    assert!(
        terms.second_mix_row >= second_row && terms.second_output_rounding >= omega,
        "{rate} Hz: the module's second mix row {} is below the derivation's {second_row}",
        terms.second_mix_row
    );
    let half_second_row = 0.5 * second_row;

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

    // #1485 (N1): the LPF along its own zones. `Gamma(m)`, the largest coefficient of the HPF's
    // output at one frame in the LPF's output `m` frames later, by the exposure `B_r` over
    // neighbouring zones (1024 frames, then a geometric tail at `rho_ramp`, whose shape is
    // checked); the non-increasing envelope split at every `m0`: the fast part on the HPF's
    // largest output `A`, the slow part telescoped through the potential with each frame's charge
    // `C` at one zone. The smallest bound over `m0`.
    let horizon = 1024_usize;
    let omega_second = terms.second_output_rounding;
    let nb_max = |values: &[f64], i: usize| {
        neighbours[i]
            .iter()
            .map(|&j| values[j])
            .fold(0.0_f64, f64::max)
    };
    let mut exposure: Vec<f64> = zones
        .iter()
        .map(|z| half_second_row * z.q + omega_second)
        .collect();
    let mut gamma = Vec::with_capacity(horizon + 1);
    for m in 1..=horizon + 1 {
        gamma.push(
            (0..n)
                .map(|i| zones[i].beta * nb_max(&exposure, i))
                .fold(0.0_f64, f64::max),
        );
        if m <= horizon {
            exposure = (0..n)
                .map(|i| zones[i].rho * nb_max(&exposure, i))
                .collect();
        }
    }
    assert!(
        (0..n).all(|i| zones[i].rho * nb_max(&exposure, i) <= rr * exposure[i] * (1.0 + 1.0e-9)),
        "{rate} Hz: the exposure's shape is not settled at its horizon"
    );
    let tail = gamma[horizon] * rr / (1.0 - rr);
    let mut envelope = gamma.clone();
    for m in (0..horizon).rev() {
        envelope[m] = envelope[m].max(envelope[m + 1]);
    }
    let peak_output = (0..n)
        .map(|i| (half_row * zones[i].q + omega) * phi[i][0])
        .fold(0.0_f64, f64::max)
        + oi;
    let charge_per_frame = (0..n)
        .map(|i| {
            let direct = if zones[i].direct {
                zones[i].q * phi[i][0]
            } else {
                0.0
            };
            half_row * (nb_max(&psi, i) * zones[i].beta + direct) + omega * phi[i][0] + oi
        })
        .fold(0.0_f64, f64::max);
    let mut best = (f64::INFINITY, 0_usize);
    for m0 in 1..=horizon + 1 {
        let fast: f64 = envelope[..m0 - 1]
            .iter()
            .map(|e| e - envelope[m0 - 1])
            .sum();
        let slow =
            (m0 - 1) as f64 * envelope[m0 - 1] + envelope[m0 - 1..].iter().sum::<f64>() + tail;
        let bound = oi * peak_output
            + peak_output * fast
            + half_row * envelope[m0 - 1] * v[0]
            + charge_per_frame * slow;
        if bound < best.0 {
            best = (bound, m0);
        }
    }
    let peak_gain = gain * best.0;
    let peak_split = (
        best.1,
        gamma[0],
        envelope[best.1 - 1],
        peak_output,
        charge_per_frame,
    );
    // `sigma_p`: the flush part with the tail at frame `0`, the flush window over every frame and
    // per group the fixed point plus the decaying start's peak, frame by frame until it falls.
    let mut peak_stall = outputs_a.iter().fold(0.0_f64, |s, value| s.max(*value));
    for t in &groups {
        let f = f_step;
        let a_h = rs * t.cd + mu_settled * t.cy + mu_x * t.cy * t.r;
        let a_f = rs * (os + 2.0 + omega) + mu_settled + mu_x * (t.cy + 1.0) + 1.0;
        let st = f / (1.0 - t.r);
        let tau_star = (a_h * st + a_f * f) / (1.0 - rs);
        let constant = f + 2.0 * (t.cy * st + f);
        let fixed = os * (tau_star + t.cy * st + constant) + oi * (t.cy * (t.r * st + f) + f) + f;
        let h0 = energy_a.min(t.phi[1] * f);
        let (mut tau, mut h, mut x) = (sigma_a + t.cy * h0 + f, h0, 2.0 * t.cy * h0);
        let mut peak = 0.0_f64;
        loop {
            peak = peak.max(os * (tau + t.cy * h + x) + oi * t.cy * t.r * h);
            let next = (rs * tau + a_h * h, t.r * h, rs.max(t.r) * x);
            if next.0 <= tau && next.1 <= h && next.2 <= x {
                break;
            }
            (tau, h, x) = next;
        }
        peak_stall = peak_stall.max(fixed + peak);
    }
    // `sigma_t`: the smaller of the stall from `T_decay` and the stall at every frame.
    let tail_stall = stall.min(peak_stall);
    // #1485 `G_t`, per unit of the late input's peak: each section along every in-flight ramp
    // of #1407 (from a word in zone `s` toward a design in zone `t`, `r` of its frames left at
    // `M`, then the design), or a frozen recursion (a disable) for `r` frames, the HPF from zero
    // with a unit input and the LPF from zero driven by the HPF's largest output at each window
    // frame; the word at ramp frame `i` lies in a zone that meets the interval
    // `(1 - i/64) [a_s, b_s] + (i/64) [a_t, b_t] +- eta`. Then per pair of settled zones in
    // closed form.
    let ramp_frames = ramp as usize;
    let edges_a: Vec<f64> = zones.iter().map(|z| z.a).collect();
    let edges_b: Vec<f64> = zones.iter().map(|z| z.b).collect();
    let constants_over = |from: f64, to: f64| -> [f64; 3] {
        let first = edges_b.partition_point(|b| *b < from).min(n - 1);
        let last = edges_a.partition_point(|a| *a <= to).max(first + 1) - 1;
        (first..=last).fold([0.0_f64; 3], |sup, i| {
            [
                sup[0].max(zones[i].rho),
                sup[1].max(zones[i].beta),
                sup[2].max(zones[i].q),
            ]
        })
    };
    let ramp_constants: Vec<Vec<Vec<[f64; 3]>>> = (0..n)
        .map(|s| {
            (0..n)
                .map(|t| {
                    (0..ramp_frames)
                        .map(|i| {
                            let alpha = i as f64 / ramp_frames as f64;
                            constants_over(
                                (1.0 - alpha) * zones[s].a + alpha * zones[t].a - dre_r,
                                (1.0 - alpha) * zones[s].b + alpha * zones[t].b + dre_r,
                            )
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    let run_section = |row: f64, omega_s: f64, drive: &[f64]| -> (Vec<f64>, Vec<f64>) {
        let mut output = vec![0.0_f64; ramp_frames];
        let mut end = vec![0.0_f64; n];
        for t in 0..n {
            let Some((rt, qt, _)) = zones[t].settled else {
                continue;
            };
            for from_start in &ramp_constants {
                for r in 0..=ramp_frames {
                    let mut state = 0.0_f64;
                    for f in 0..ramp_frames {
                        let [rho, beta, q] = if f < r {
                            from_start[t][ramp_frames - r + f]
                        } else {
                            [rt, zones[t].beta, qt]
                        };
                        output[f] = output[f].max((row * q + omega_s) * state + oi * drive[f]);
                        state = rho * state + beta * drive[f];
                    }
                    end[t] = end[t].max(state);
                }
            }
        }
        for (s, from_start) in ramp_constants.iter().enumerate() {
            let [rho, beta, q] = from_start[s][0];
            for r in 1..=ramp_frames {
                let mut state = 0.0_f64;
                for f in 0..r {
                    output[f] = output[f].max((row * q + omega_s) * state + oi * drive[f]);
                    state = rho * state + beta * drive[f];
                }
            }
        }
        (output, end)
    };
    let (hpf_output, hpf_end) = run_section(half_row, omega, &vec![1.0_f64; ramp_frames]);
    let (lpf_output, lpf_end) = run_section(half_second_row, omega_second, &hpf_output);
    let tail_window = lpf_output.iter().copied().fold(0.0_f64, f64::max);
    let mut tail_settled = 0.0_f64;
    for k in 0..n {
        let Some((rk, qk, _)) = zones[k].settled else {
            continue;
        };
        let ck = half_row * qk + omega;
        let e_bar = zones[k].beta / (1.0 - rk);
        for l in 0..n {
            let Some((rl, ql, _)) = zones[l].settled else {
                continue;
            };
            let state = lpf_end[l]
                + zones[l].beta * ck * hpf_end[k] / (1.0 - rk.min(rl))
                + zones[l].beta * (ck * e_bar + oi) / (1.0 - rl);
            tail_settled = tail_settled.max(
                (half_second_row * ql + omega_second) * state
                    + oi * (ck * hpf_end[k].max(e_bar) + oi),
            );
        }
    }
    let tail_window = gain * tail_window;
    let tail_settled = gain * tail_settled;
    let tail_gain = tail_window.max(tail_settled);
    // `D`: per group, the relative state carried to `T_decay` in closed form, then the
    // certificate `v_H = z_H`, `v_tau = max(z_tau, a_H v_H / (lambda - rho_s))`, `v_X = z_X` at
    // every rate `lambda_j = rho + (1 - rho) 2^(-j/2)`, `j = 1..=32`, `rho` the largest diagonal
    // entry; `D_c = floor(max(a, 0) + b) + 1` at the best rate, `D` the largest over the groups.
    let h_log = math::log(limit);
    let decay = groups
        .iter()
        .map(|t| {
            assert!(t_decay >= start, "{rate} Hz: T_decay lies in the window");
            let h0 = energy.min(t.phi[0] * gain);
            let (tau, h, x) = closed(
                t,
                sigma + t.cy * h0,
                h0,
                2.0 * t.cy * h0,
                0.0,
                t_decay - start,
            );
            let a_h = rs * t.cd + mu_settled * t.cy + mu_x * t.cy * t.r;
            let row = [os, os * t.cy + oi * t.cy * t.r, os];
            let rho = rs.max(t.r);
            (1..=32)
                .map(|j| {
                    let lambda = rho + (1.0 - rho) * math::pow(2.0, -f64::from(j) / 2.0);
                    let v = [tau.max(a_h * h / (lambda - rs)), h, x];
                    let b_value = row[0] * v[0] + row[1] * v[1] + row[2] * v[2];
                    let rate_log = -math::log(lambda);
                    let a = (math::log(b_value) - h_log) / rate_log;
                    let b = core::f64::consts::LN_10 / rate_log;
                    math::floor(a.max(0.0) + b) as u64 + 1
                })
                .min()
                .expect("rates")
        })
        .max()
        .expect("groups");
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
        decay,
        peak_gain,
        peak_stall,
        tail_stall,
        tail_gain,
        tail_window,
        tail_settled,
        peak_split,
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
/// * #1466 L3: the live composition values ([`live_composition`]) against the recomputation of
///   `docs/derivations/1379-graph-tail-composition.md`, "The live input section" ([`LiveOracle`]'s
///   `decay`, `peak_gain`, `peak_stall`, `tail_stall`): `D` within `0.01 %` and 64 frames above it,
///   each gain and stall within 1 mB above it. Nothing else reads these values.
#[test]
fn live_bound_carries_every_term_an_independent_recomputation_requires() {
    let u = 1.0 / 16_777_216.0;
    let kappa = 1.0 + core::f64::consts::SQRT_2;
    for rate in LAUNCH_RATES {
        let envelope = input_section_live_envelope(rate)
            .expect("launch rate")
            .envelope;
        let settled = settled_contraction(rate);
        let ramp_box = independent_box_norm([33.0 * u, 16.3 * u, 33.0 * u]);
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
        let rest = stated_rest(live.rest, "live rest");
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
        // #1466 L3: the live composition values against the recomputation of
        // `docs/derivations/1379-graph-tail-composition.md`, "The live input section": `D` within
        // `0.01 %` and 64 frames above it, each gain and stall within 1 mB above it.
        let composition = live_composition(rate);
        eprintln!(
            "L3 {rate} Hz: D {} (recomputed {}), G_p {:.9e} ({:.9e}), sigma_p {:.9e} ({:.9e}), \
             sigma_t {:.9e} ({:.9e})",
            composition.decay,
            oracle.decay,
            composition.peak_gain,
            oracle.peak_gain,
            composition.peak_stall,
            oracle.peak_stall,
            composition.tail_stall,
            oracle.tail_stall
        );
        // #1484 rule (g) for the live section, which no registry checks (builtins are not
        // registry rows): `sigma_t <= sigma_p`, raw and in millibels (equal stalls are admissible).
        assert!(
            composition.tail_stall <= composition.peak_stall
                && ceil_mb(composition.tail_stall) <= ceil_mb(composition.peak_stall),
            "{rate} Hz: sigma_t {:e} above sigma_p {:e}",
            composition.tail_stall,
            composition.peak_stall
        );
        assert!(
            composition.decay >= oracle.decay
                && composition.decay <= oracle.decay + oracle.decay / 10_000 + 64,
            "{rate} Hz D: module {} against the recomputed {}",
            composition.decay,
            oracle.decay
        );
        // #1485: `G_p` and `G_t` against the recomputation of "(N1): `G_p`" and "The live tail
        // gain `G_t`".
        let (split, gamma_1, gamma_split, peak_output, charge) = oracle.peak_split;
        eprintln!(
            "L3 {rate} Hz: G_p {:.9e} ({:.3} dB; recomputed {:.9e}: m0 {split}, Gamma(1) \
             {gamma_1:.6e}, Gammabar(m0) {gamma_split:.6e}, A {peak_output:.6e}, C {charge:.6e})",
            composition.peak_gain,
            20.0 * math::log10(composition.peak_gain),
            oracle.peak_gain
        );
        eprintln!(
            "L3' {rate} Hz: G_t {:.9e} ({:.9e}: window {:.6e}, settled {:.6e}), {:.3} dB",
            composition.tail_gain,
            oracle.tail_gain,
            oracle.tail_window,
            oracle.tail_settled,
            20.0 * math::log10(composition.tail_gain)
        );
        for (name, module, recomputed) in [
            ("G_p", composition.peak_gain, oracle.peak_gain),
            ("G_t", composition.tail_gain, oracle.tail_gain),
            ("sigma_p", composition.peak_stall, oracle.peak_stall),
            ("sigma_t", composition.tail_stall, oracle.tail_stall),
        ] {
            assert!(
                module >= recomputed && module <= recomputed * linear(1),
                "{rate} Hz {name}: module {module:e} against the recomputed {recomputed:e}"
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
        let rest = stated_rest(live.rest, "live rest");
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
        let rest = stated_rest(live.rest, "live rest");
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
        let rest = stated_rest(live.rest, "live rest");
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

// ---- #1465: a fixed input section's composition values (F1-F3) --------------------------------

/// `2^-24`, the `f32` unit roundoff.
const U: f64 = 1.0 / 16_777_216.0;

/// The five composition values of a bound, `(D, G_p, G_t, sigma_p, sigma_t)`, read through the
/// clause readers (#1484 S-D2: `G_p` and `sigma_p` from (N1)'s pair, `D`, `G_t` and `sigma_t`
/// from (N2)'s triple); panics unless every one is stated with a level (no `Zero`).
fn composition_values(bound: NodeTailBound, what: &str) -> (u64, i32, i32, i32, i32) {
    match (
        bound.composition.peak_clause(),
        bound.composition.tail_clause(),
    ) {
        (
            Some((PeakGain::Millibels(peak), FlushStall::Level(peak_stall))),
            Some((TailDecay(decay), PeakGain::Millibels(tail), FlushStall::Level(tail_stall))),
        ) => (decay, peak, tail, peak_stall, tail_stall),
        _ => panic!("{what}: composition {:?}", bound.composition),
    }
}

/// `ceil(2000 log10 g)`, plain.
fn ceil_mb(g: f64) -> i32 {
    -math::floor(-2000.0 * math::log10(g)) as i32
}

/// The linear value of `millibels`.
fn linear(millibels: i32) -> f64 {
    math::pow(10.0, f64::from(millibels) / 2000.0)
}

/// `T_b(level)` for each level: the smallest `j` with `g * sum_{j <= m < len} |h[m]| < level`.
fn suffix_crossings_at(response: &[f64], gain: f64, levels: &[f64]) -> Vec<usize> {
    let mut crossings = vec![0_usize; levels.len()];
    let mut found = vec![false; levels.len()];
    let mut sum = 0.0_f64;
    for index in (0..response.len()).rev() {
        sum += response[index].abs();
        for ((crossing, found), level) in crossings.iter_mut().zip(&mut found).zip(levels) {
            if !*found && gain * sum >= *level {
                *crossing = index + 1;
                *found = true;
            }
        }
    }
    crossings
}

fn l1(response: &[f64]) -> f64 {
    response.iter().map(|value| value.abs()).sum()
}

/// One section's constants by #1329's closed forms (`docs/derivations/1329-input-section-tail-
/// and-rest.md`, "The kernel and its norm", "Rounding and the flush"), recomputed in plain `f64`
/// from the designed words: no code shared with `math::tail`.
#[derive(Clone, Copy, Debug)]
struct PlainSection {
    q: f64,
    beta: f64,
    gamma: f64,
    delta: f64,
    mu_state: f64,
    mu_input: f64,
    omega_state: f64,
    omega_input: f64,
}

/// The largest singular value of `[[a, b], [c, d]]`, by the non-cancelling closed form.
fn singular(a: f64, b: f64, c: f64, d: f64) -> f64 {
    (math::sqrt((a + d) * (a + d) + (c - b) * (c - b))
        + math::sqrt((a - d) * (a - d) + (b + c) * (b + c)))
        / 2.0
}

fn plain_section(words: &SvfWords) -> PlainSection {
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let sqrt2 = core::f64::consts::SQRT_2;
    let (c1, a2, a3, m0, m1, m2) = (words.c1, words.a2, words.a3, words.m0, words.m1, words.m2);
    // `A`, `b`, `c`, `d` of the stored-form step.
    let (al, be, ga, de) = (1.0 - 2.0 * c1, -2.0 * a2, 2.0 * a2, 1.0 - 2.0 * a3);
    let b = [2.0 * a2, 2.0 * a3];
    let c = [m1 * (1.0 - c1) + m2 * a2, -m1 * a2 + m2 * (1.0 - a3)];
    let d = m0 + m1 * a2 + m2 * a3;
    // `||A||_V = ||R A R^-1||_2`, `R = [[1, r], [0, r]]`.
    let q = singular(
        al + r * ga,
        sqrt2 * be + de - al - r * ga,
        r * ga,
        de - r * ga,
    );
    let v_norm =
        |x: [f64; 2]| math::sqrt((x[0] + r * x[1]) * (x[0] + r * x[1]) + r * x[1] * r * x[1]);
    let beta = v_norm(b);
    let gamma = math::sqrt(c[0] * c[0] + (sqrt2 * c[1] - c[0]) * (sqrt2 * c[1] - c[0]));
    let (r_norm, r_inverse, kappa) = (math::sqrt(1.0 + r), 1.0 / math::sqrt(1.0 - r), 1.0 + sqrt2);
    let k = 1.0 + U;
    let gamma2 = 2.0 * U / (1.0 - 2.0 * U);
    let gamma3 = 3.0 * U / (1.0 - 3.0 * U);
    let (c1a, a2a, a3a) = (c1.abs(), a2.abs(), a3.abs());
    let g = [
        [2.0 * k * gamma2 * c1a, 2.0 * k * gamma3 * a2a],
        [2.0 * k * gamma2 * a2a, 2.0 * k * gamma3 * a3a],
    ];
    let mu_state =
        r_norm * singular(g[0][0], g[0][1], g[1][0], g[1][1]) * r_inverse + kappa * U * q;
    let input = [2.0 * k * gamma3 * a2a, 2.0 * k * gamma3 * a3a];
    let mu_input =
        r_norm * math::sqrt(input[0] * input[0] + input[1] * input[1]) + kappa * U * beta;
    let (one_c1, one_a3) = ((1.0 - c1).abs(), (1.0 - a3).abs());
    let (m0a, m1a, m2a) = (m0.abs(), m1.abs(), m2.abs());
    let v1_error = [
        k * gamma2 * c1a + U * one_c1,
        k * gamma3 * a2a + U * a2a,
        k * gamma3 * a2a + U * a2a,
    ];
    let v2_error = [
        k * gamma2 * a2a + U * a2a,
        k * gamma3 * a3a + U * one_a3,
        k * gamma3 * a3a + U * a3a,
    ];
    let v1_magnitude = [one_c1, a2a, a2a];
    let v2_magnitude = [a2a, one_a3, a3a];
    let w: [f64; 3] = core::array::from_fn(|i| {
        m1a * (1.0 + gamma3) * v1_error[i]
            + m2a * (1.0 + gamma2) * v2_error[i]
            + gamma3 * m1a * v1_magnitude[i]
            + gamma2 * m2a * v2_magnitude[i]
    });
    PlainSection {
        q,
        beta,
        gamma,
        delta: d.abs(),
        mu_state,
        mu_input,
        omega_state: math::sqrt(w[0] * w[0] + w[1] * w[1]) * r_inverse,
        omega_input: w[2] + gamma3 * m0a,
    }
}

/// #1465 F1. For every enabled pair of gate 1(a) at trims 0 and +24 dB, the stated `D` against the
/// independent brute force and the module's own crossings:
/// (a) `T_b(eps 10^-k / (2 |trim|)) <= T + k D` for `k = 0..8`;
/// (b) `D <= 1.5 max(D_mean, D_floor)`, `D_mean` the brute force's mean decade over eight decades;
/// (c) the module's certified crossing `T(k)` is at most `T + k D` for `k = 0..64`;
/// (d) each half's certificate rate is no faster than its propagation's spectral radius allows,
/// recomputed from the words: the deviation half's frames per decade at least
/// `D_floor = ceil(ln 10 / -ln(max_s (q_s + mu_s)))`, the reference half's at least that of
/// `max_s q_s`, and so `D_inf >= D_floor`.
#[test]
fn fixed_design_decay_law_is_sound_tight_and_above_its_floor() {
    const FRAMES: usize = 4_000_000;
    for &rate in rates() {
        let law = input_section_flush_law(rate);
        for (hpf, lpf) in pairs(rate) {
            if hpf == 0.0 && lpf == 0.0 {
                continue;
            }
            let probe = input(rate, hpf, lpf, 0.0, false);
            let sections = kernel_sections(&probe);
            let response = impulse_response(&sections, FRAMES);
            // `ceil(ln 10 / -ln rho)`: the frames per decade of a rate `rho`.
            let decade =
                |rho: f64| -math::floor(-core::f64::consts::LN_10 / -math::log(rho)) as u64;
            let plain: Vec<PlainSection> = sections.iter().map(plain_section).collect();
            // The deviation's spectral radius `max_s (q_s + mu_s)` and the reference's `max_s q_s`.
            let d_floor = decade(plain.iter().map(|c| c.q + c.mu_state).fold(0.0, f64::max));
            let reference_floor = decade(plain.iter().map(|c| c.q).fold(0.0, f64::max));
            for trim_db in [0.0, 24.0] {
                let what = format!("{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db}");
                let kernel = input(rate, hpf, lpf, trim_db, false);
                let gain = trim_gain(&kernel);
                let prepared = bound(rate, hpf, lpf, trim_db);
                let tail = finite(prepared.tail);
                let (decay, ..) = composition_values(prepared, &what);
                let levels: Vec<f64> = (0..=8)
                    .map(|k| TAIL_FLOOR / 2.0 * math::pow(10.0, -f64::from(k)))
                    .collect();
                let crossings = suffix_crossings_at(&response, gain, &levels);
                // (a) The exact half of the decade law.
                for (k, crossing) in crossings.iter().enumerate() {
                    assert!(
                        *crossing as u64 <= tail + k as u64 * decay,
                        "{what}: T_b at k = {k} is {crossing} > T + k D = {tail} + {k} {decay}"
                    );
                }
                // (b) Tightness.
                let d_mean = (crossings[8] - crossings[0]) as f64 / 8.0;
                let line = d_mean.max(d_floor as f64);
                let ratio = decay as f64 / line;
                // (c) The module's own crossings.
                let cascade = fixed_cascade(&sections, gain, &law, rest_peaks()).expect("bound");
                let composition = cascade.composition.expect("a stated composition");
                assert_eq!(
                    composition.decay, decay,
                    "{what}: the prepared D is the module's"
                );
                for k in 0..=64 {
                    let crossing = cascade.tail + composition.certificate.crossing(k);
                    assert!(
                        crossing <= tail + k * decay,
                        "{what}: T({k}) = {crossing} > T + k D = {tail} + {k} {decay}"
                    );
                }
                // (d) The asymptotic floor.
                let d_inf = composition.certificate.asymptotic_decay();
                eprintln!(
                    "F1 {what}: T {tail}, D {decay}, D_mean {d_mean:.1}, D_floor {d_floor}, \
                     D_inf {d_inf}, ratio {ratio:.3}, T_lambda - T {}",
                    composition.certificate.floor_crossing()
                );
                let half = |decade: f64| -math::floor(-decade) as u64;
                let certificate = composition.certificate;
                assert!(
                    half(certificate.deviation.decade) >= d_floor
                        && half(certificate.reference.decade) >= reference_floor
                        && d_inf >= d_floor,
                    "{what}: the deviation half's rate {} against D_floor {d_floor}, the \
                     reference half's {} against {reference_floor}",
                    certificate.deviation.decade,
                    certificate.reference.decade
                );
                assert!(
                    ratio <= 1.5,
                    "{what}: D {decay} > 1.5 max(D_mean, D_floor) = 1.5 {line}"
                );
            }
        }
    }
}

/// The impulse response of one enabled section alone, its `l1` norm.
fn section_l1(sections: &[SvfWords], frames: usize) -> Vec<f64> {
    sections
        .iter()
        .map(|words| l1(&impulse_response(core::slice::from_ref(words), frames)))
        .collect()
}

/// #1465 F2(a), (b), (d). For every enabled pair of gate 1(a) at trims 0 and +24 dB, and at every
/// launch rate the top pair and the HPF at 10 Hz into the LPF one `f32` above 10 Hz:
/// (a) `|trim| ||h||_1 <= g_t <= g_p` against the brute-force cascade;
/// (b) `G_p <= 20 log10(|trim| ||h_HPF||_1 ||h_LPF||_1) + 6.02 dB`, the margin recorded;
/// (d) the module's `dev_loud` lies within 1 mB above a plain recomputation of the loud fixed
/// point, and `G_p = ceil_mB(|trim| (1 + u) (O + dev_loud))` from the module's values.
#[test]
fn fixed_design_gains_bound_the_cascade_and_stay_near_its_section_norms() {
    const FRAMES: usize = 4_000_000;
    for &rate in rates() {
        let law = input_section_flush_law(rate);
        let mut rows = pairs(rate);
        rows.push((10.0, f32::from_bits(10.0_f32.to_bits() + 1)));
        for (hpf, lpf) in rows {
            if hpf == 0.0 && lpf == 0.0 {
                continue;
            }
            let probe = input(rate, hpf, lpf, 0.0, false);
            let sections = kernel_sections(&probe);
            let cascade_l1 = l1(&impulse_response(&sections, FRAMES));
            let norms = section_l1(&sections, FRAMES);
            let product: f64 = norms.iter().product();
            // (d)'s recomputation: the first section's output supremum from the brute force.
            let plain: Vec<PlainSection> = sections.iter().map(plain_section).collect();
            let mut state_sup = vec![plain[0].beta / (1.0 - plain[0].q)];
            let mut input_sup = vec![1.0, norms[0]];
            if plain.len() == 2 {
                state_sup.push(plain[1].beta * input_sup[1] / (1.0 - plain[1].q));
            }
            input_sup.truncate(plain.len());
            let mut difference = 0.0;
            for (i, c) in plain.iter().enumerate() {
                let error = ((c.beta + c.mu_input) * difference
                    + c.mu_state * state_sup[i]
                    + c.mu_input * input_sup[i])
                    / (1.0 - (c.q + c.mu_state));
                difference = (c.gamma + c.omega_state) * error
                    + (c.delta + c.omega_input) * difference
                    + c.omega_state * state_sup[i]
                    + c.omega_input * input_sup[i];
            }
            for trim_db in [0.0, 24.0] {
                let what = format!("{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db}");
                let kernel = input(rate, hpf, lpf, trim_db, false);
                let gain = trim_gain(&kernel);
                let prepared = bound(rate, hpf, lpf, trim_db);
                let (_, peak, tail, ..) = composition_values(prepared, &what);
                // (a) Soundness against the brute force.
                assert!(tail <= peak, "{what}: G_t {tail} > G_p {peak}");
                assert!(
                    gain * cascade_l1 <= linear(tail),
                    "{what}: |trim| ||h||_1 = {} > g_t = {}",
                    gain * cascade_l1,
                    linear(tail)
                );
                // (b) Tightness against the sections' own norms.
                let line = 20.0 * math::log10(gain * product) + 6.02;
                let margin = line - f64::from(peak) / 100.0;
                eprintln!(
                    "F2 {what}: G_p {peak} mB, |trim| ||h||_1 {:.3} dB, line {line:.3} dB, \
                     margin {margin:.3} dB",
                    20.0 * math::log10(gain * cascade_l1)
                );
                assert!(
                    margin >= 0.0,
                    "{what}: G_p {peak} mB above the line {line:.3} dB"
                );
                // (d) `dev_loud` and its use.
                let composition = fixed_cascade(&sections, gain, &law, rest_peaks())
                    .expect("bound")
                    .composition
                    .expect("a stated composition");
                let dev_loud = composition.dev_loud;
                assert!(
                    difference <= dev_loud
                        && dev_loud <= difference * math::pow(10.0, 1.0 / 2000.0),
                    "{what}: dev_loud {dev_loud} against the recomputation {difference}"
                );
                let recomputed =
                    ceil_mb(gain * (1.0 + U) * (composition.output_majorant + dev_loud));
                assert_eq!(peak, recomputed, "{what}: G_p from O and dev_loud");
            }
        }
    }
}

/// #1465 F3, #1484 S4. Both stalls are the module's stall rounded up to a millibel (`F a` holds at
/// every frame, so `sigma_p = sigma_t`), `P^ = 4 sigma_t / eps` (an (N2) use, read through
/// (N2)'s triple) is at least the module's `P*`, and every enabled design states a positive stall
/// level.
#[test]
fn fixed_design_stall_is_the_module_stall_rounded_up_and_covers_the_flush_floor() {
    for &rate in rates() {
        let law = input_section_flush_law(rate);
        for (hpf, lpf) in pairs(rate) {
            if hpf == 0.0 && lpf == 0.0 {
                continue;
            }
            for trim_db in [0.0, 24.0] {
                let what = format!("{rate} Hz HPF {hpf} LPF {lpf} trim {trim_db}");
                let kernel = input(rate, hpf, lpf, trim_db, false);
                let sections = kernel_sections(&kernel);
                let gain = trim_gain(&kernel);
                let (.., peak_stall, tail_stall) =
                    composition_values(bound(rate, hpf, lpf, trim_db), &what);
                let cascade = fixed_cascade(&sections, gain, &law, rest_peaks()).expect("bound");
                let raw = cascade.composition.expect("a stated composition").stall;
                assert!(raw > 0.0, "{what}: a positive stall");
                assert_eq!(
                    peak_stall,
                    ceil_mb(raw),
                    "{what}: sigma_p is the stall rounded up"
                );
                assert_eq!(
                    tail_stall,
                    ceil_mb(raw),
                    "{what}: sigma_t is the stall rounded up"
                );
                let p_hat = 4.0 * linear(tail_stall) / TAIL_FLOOR;
                eprintln!(
                    "F3 {what}: sigma_t {tail_stall} mB, P^ {p_hat:.4e}, P* {:.4e}",
                    cascade.flush_floor
                );
                assert!(
                    p_hat >= cascade.flush_floor,
                    "{what}: P^ {p_hat} < P* {}",
                    cascade.flush_floor
                );
            }
        }
    }
}

/// #1465 F2(c), and the both-disabled branch of preparation (root's named exception, 2026-10-08).
/// A design with both filters disabled states `D = 0`, `G_p = G_t = ceil_mB(|trim|)` for a
/// power-of-two trim (0 dB, `|trim| = 2`) and `ceil_mB(|trim| (1 + u))` otherwise (+6 dB, whose
/// word lies below `10^(6/20)`, so the two differ), and one underflow as both stalls, through both
/// `input_section_bound` and `input_section_bounds`; with the channels' trims different, the larger
/// gain; and one disabled channel beside an enabled one still leaves the design stated.
#[test]
fn a_design_with_filters_disabled_states_its_trim_gain() {
    let rate = 48_000;
    // `+6.0206 dB`: the `f32` gain word is exactly 2.
    let two_db = 6.020_6_f32;
    let underflow = ceil_mb(f64::from(f32::MIN_POSITIVE));
    for (trim_db, exact) in [(0.0_f32, true), (two_db, true), (6.0, false), (-3.0, false)] {
        let strip = parameters(0.0, 0.0, trim_db, false);
        let kernel = input(rate, 0.0, 0.0, trim_db, false);
        let trim = trim_gain(&kernel);
        let expected = if exact {
            ceil_mb(trim)
        } else {
            ceil_mb(trim * (1.0 + U))
        };
        if trim_db == two_db {
            assert_eq!(trim, 2.0, "the +6.0206 dB word");
        }
        if !exact {
            assert_ne!(
                expected,
                ceil_mb(trim),
                "{trim_db} dB: the rounding moves the value"
            );
        }
        let single = input_section_bound(rate, strip).expect("bound");
        let session = input_section_bounds(rate, [strip], None).expect("bounds")[0];
        for (path, value) in [
            ("input_section_bound", single),
            ("input_section_bounds", session),
        ] {
            let what = format!("{path} at {trim_db} dB");
            assert_eq!(value.tail, TailSamples::Finite(0), "{what}");
            assert_eq!(value.rest, RestBound::Bounded(RestSamples::ZERO), "{what}");
            assert_eq!(
                composition_values(value, &what),
                (0, expected, expected, underflow, underflow),
                "{what}"
            );
        }
    }
    // Different trims, both disabled: the larger gain, on both paths.
    let channel = |trim_db: f32, lpf_hz: f32| ChannelParameters {
        trim_db,
        lpf_hz,
        ..ChannelParameters::default()
    };
    let strip = BuiltinParameters {
        left: channel(0.0, 0.0),
        right: channel(12.0, 0.0),
        ..BuiltinParameters::default()
    };
    let louder = ceil_mb(f64::from(math::pow(10.0, 12.0 / 20.0) as f32) * (1.0 + U));
    let single = input_section_bound(rate, strip).expect("bound");
    let session = input_section_bounds(rate, [strip], None).expect("bounds")[0];
    for (path, value) in [
        ("input_section_bound", single),
        ("input_section_bounds", session),
    ] {
        let (_, peak, tail, ..) = composition_values(value, path);
        assert_eq!(
            (peak, tail),
            (louder, louder),
            "{path}: the louder channel's gain"
        );
    }
    // One channel disabled, the other enabled: stated, at least the disabled channel's gain.
    let mixed = BuiltinParameters {
        left: channel(24.0, 0.0),
        right: channel(0.0, 1_000.0),
        ..BuiltinParameters::default()
    };
    let (_, peak, ..) =
        composition_values(input_section_bound(rate, mixed).expect("bound"), "mixed");
    assert!(peak >= ceil_mb(f64::from(math::pow(10.0, 24.0 / 20.0) as f32)));
}

// ---- #1466: the live input section's decay, peak gain and stalls (gate 1, L4; L3 above) -----

/// The live composition values at `rate`, as the gates read them: `math::tail`'s accessor at the
/// +24 dB trim word, the flush law and the ramp the live bound uses.
fn live_composition(rate: u32) -> LiveComposition {
    live_cascade_composition(
        &input_section_live_envelope(rate).expect("launch rate"),
        rest_peaks()[0],
        &input_section_flush_law(rate),
        u64::from(INPUT_FILTER_RAMP_SAMPLES),
    )
    .expect("live composition")
    .expect("a certified live composition")
}

/// One control event of a gate 1 history ([`peak_history`]), at a frame after the drive.
#[derive(Clone, Copy)]
enum PeakEvent {
    /// Retarget one section (`0` the HPF, `1` the LPF) to a cutoff.
    Retarget(usize, f32),
    /// Flip the polarity of both channels at once.
    Flip,
}

/// The largest `|y|` of a real input section, trim +24 dB, both sections designed at the
/// worst-case pair, driven by an alternating `+-1` input for 1,000,000 frames and 200,000 more
/// with `events` (frames counted from the end of the drive), in blocks of 64 frames, or of
/// `block` frames from 128 frames before the first event to 512 after it. Returns the peak
/// before the first event and over the run.
fn peak_history(rate: u32, events: &[(usize, PeakEvent)], block: usize) -> (f32, f32) {
    const DRIVE: usize = 1_000_000;
    const AFTER: usize = 200_000;
    let (hpf, lpf) = input_section_worst_case_pair(rate).expect("launch rate");
    let mut section = input(rate, hpf, lpf, 24.0, false);
    let (mut before, mut peak) = (0.0_f32, 0.0_f32);
    let mut left = [0.0_f32; 64];
    let mut right = [0.0_f32; 64];
    let mut inverted = false;
    let mut frame = 0;
    while frame < DRIVE + AFTER {
        for (at, event) in events {
            if DRIVE + at == frame {
                match *event {
                    PeakEvent::Retarget(index, hz) => section
                        .apply_prepared_filter(target(rate, index, hz))
                        .expect("retarget"),
                    PeakEvent::Flip => {
                        inverted = !inverted;
                        section.set_polarity_invert(BuiltinLaneSelector::Both, inverted, 0);
                    }
                }
            }
        }
        let mut len = 64.min(DRIVE + AFTER - frame);
        if frame + 128 >= DRIVE && frame < DRIVE + 512 {
            len = len.min(block);
        }
        for (at, _) in events {
            if DRIVE + at > frame {
                len = len.min(DRIVE + at - frame);
            }
        }
        for index in 0..len {
            let value = if (frame + index) % 2 == 0 { 1.0 } else { -1.0 };
            left[index] = value;
            right[index] = value;
        }
        process(
            &mut section,
            &mut left[..len],
            &mut right[..len],
            frame as u64,
        );
        let block_peak = left[..len]
            .iter()
            .chain(&right[..len])
            .fold(0.0_f32, |sup, value| sup.max(value.abs()));
        if frame < DRIVE {
            before = before.max(block_peak);
        }
        peak = peak.max(block_peak);
        frame += len;
    }
    (before, peak)
}

/// #1485 gate 1 (#1466 L1, widened). The real kernel's largest `|y|` per unit of the input's peak
/// is at most `g_p + sigma_p`, the live peak gain and peak stall at `X = 1`, on five histories
/// after the Nyquist drive of [`peak_history`] (#1379 H9: the drive builds a large first
/// integrator in both sections at the top of the domain while the output stays near 1, and a
/// retarget exposes it). Each history, with its reason:
///
/// * L1: the HPF to 10 Hz, the largest peak the attempt found (its record lists the others it
///   measured, all lower);
/// * L1 in blocks of one frame around the retarget: #1407's ramp allowance is largest at block
///   size 1;
/// * L1 with the polarity flipped 8 frames into the ramp window: a trim or polarity change while
///   the state is exposed;
/// * the LPF to 10 Hz: the second section's state at the top, which the bound's slow exposure
///   charges;
/// * both sections moving: the HPF to 10 Hz, then the LPF to 10 Hz 4 frames later.
///
/// Prints, per history and rate, the peak (`g_meas` of the largest) and the ratio
/// `g_p / peak` (issue #1485, V-D3).
#[test]
fn live_peak_gain_bounds_the_real_kernel_on_its_worst_histories() {
    use PeakEvent::{Flip, Retarget};
    type History = (&'static str, Vec<(usize, PeakEvent)>, usize);
    let histories: [History; 5] = [
        ("L1, HPF to 10 Hz", vec![(0, Retarget(0, 10.0))], 64),
        ("L1 in blocks of 1", vec![(0, Retarget(0, 10.0))], 1),
        (
            "L1, polarity flipped at +8",
            vec![(0, Retarget(0, 10.0)), (8, Flip)],
            64,
        ),
        ("LPF to 10 Hz", vec![(0, Retarget(1, 10.0))], 64),
        (
            "HPF to 10 Hz, LPF to 10 Hz at +4",
            vec![(0, Retarget(0, 10.0)), (4, Retarget(1, 10.0))],
            64,
        ),
    ];
    for &rate in rates() {
        let composition = live_composition(rate);
        let stated = composition.peak_gain + composition.peak_stall;
        let mut largest = 0.0_f32;
        for (name, events, block) in &histories {
            let (before, peak) = peak_history(rate, events, *block);
            largest = largest.max(peak);
            eprintln!(
                "G1 {rate} Hz {name}: peak before the event {before:.4e}, over the run {peak:.4e} \
                 ({:.2} dB), g_p {:.4e} ({:.2} dB), ratio g_p / peak {:.2} ({:.2} dB)",
                20.0 * math::log10(f64::from(peak)),
                composition.peak_gain,
                20.0 * math::log10(composition.peak_gain),
                composition.peak_gain / f64::from(peak),
                20.0 * math::log10(composition.peak_gain / f64::from(peak))
            );
            assert!(
                f64::from(peak) <= stated,
                "{rate} Hz {name}: the real kernel's peak {peak:e} exceeds g_p + sigma_p = \
                 {stated:e}"
            );
        }
        eprintln!(
            "G1 {rate} Hz: g_meas {largest:.6e}, g_p {:.6e}, r_p {:.4}",
            composition.peak_gain,
            composition.peak_gain / f64::from(largest)
        );
    }
}

/// #1466 L4. The live decade law against the module's own crossings: the directly searched
/// crossing of `(eps / 2) 10^-k` (`math::tail::live_cascade_crossing`, #1433's search at the lower
/// threshold) is `T` itself at `k = 0`, strictly increasing in `k`, and for `k = 1..64` at most
/// the certificates' own crossing `T + crossing(k)` and at most `T + k D`; and `D_inf` is at
/// least the floor `ceil(ln 10 / -ln rho_s)` of the settled contraction the derivation requires
/// ([`settled_contraction`], as #1465's F1(d)).
#[test]
fn live_decay_covers_the_module_crossings_and_its_floor() {
    for &rate in rates() {
        let terms = input_section_live_envelope(rate).expect("launch rate");
        let law = input_section_flush_law(rate);
        let ramp = u64::from(INPUT_FILTER_RAMP_SAMPLES);
        let composition = live_composition(rate);
        let decay = composition.decay;
        let tail = input_section_live_cascade(rate).expect("launch rate").tail;
        let mut worst = 0.0_f64;
        let mut previous = tail;
        for k in 0..=64_u64 {
            let crossing = live_cascade_crossing(&terms, rest_peaks()[0], &law, ramp, k)
                .expect("live crossing");
            if k == 0 {
                assert_eq!(crossing, tail, "{rate} Hz: the crossing at k = 0 is T");
            } else {
                // Non-vacuity: each decade's crossing lies past the one before, so the search
                // reads `k` (a search that ignores it gives `T(k) = T` and checks nothing).
                assert!(
                    crossing > previous,
                    "{rate} Hz: T({k}) = {crossing} is not past T({}) = {previous}",
                    k - 1
                );
                // The certificate's own per-decade claim, which is stronger than `T + k D`.
                let certified = tail + composition.crossing(k);
                assert!(
                    crossing <= certified,
                    "{rate} Hz: T({k}) = {crossing} > T + crossing({k}) = {certified}"
                );
                worst = worst.max((crossing - tail) as f64 / k as f64);
            }
            previous = crossing;
            assert!(
                crossing <= tail + k * decay,
                "{rate} Hz: T({k}) = {crossing} > T + k D = {tail} + {k} {decay}"
            );
        }
        let floor =
            -math::floor(-core::f64::consts::LN_10 / -math::log(settled_contraction(rate))) as u64;
        let d_inf = composition.asymptotic_decay();
        eprintln!(
            "L4 {rate} Hz: T {tail}, D {decay}, D_inf {d_inf}, floor {floor}, max (T(k) - T) / k \
             {worst:.1}, lambda {:.12}, groups {}, T_lambda - T {}",
            composition.lambda(),
            composition.groups.len(),
            composition.floor_crossing()
        );
        assert!(
            d_inf >= floor,
            "{rate} Hz: D_inf {d_inf} below the settled contraction's floor {floor}"
        );
    }
}

/// #1466 follow-up (verdict NITs 3 and 4). The live composition is refused (`Ok(None)`) when a
/// window state bound took the cap, a NaN included, and when a settled group leaves the carry's
/// rounding argument. Neither guard is reached at the launch configuration; both are reached here
/// through `math::tail`'s public accessor: an input scale far above any trim word (`1e35`,
/// infinity), a NaN scale, and the launch cascade with its first section's mix row scaled tenfold.
#[test]
fn live_composition_refuses_a_capped_window_and_an_unbounded_carry() {
    for &rate in rates() {
        let live = input_section_live_envelope(rate).expect("launch rate");
        let law = input_section_flush_law(rate);
        let ramp = u64::from(INPUT_FILTER_RAMP_SAMPLES);
        for gain in [1e35, f64::INFINITY, f64::NAN] {
            let composition = live_cascade_composition(&live, gain, &law, ramp);
            assert!(
                matches!(composition, Ok(None)),
                "{rate} Hz: a capped window at the input scale {gain:e} gave {composition:?}"
            );
        }
        let mut doctored = live;
        doctored.first_mix_row *= 10.0;
        let composition = live_cascade_composition(&doctored, rest_peaks()[0], &law, ramp);
        assert!(
            matches!(composition, Ok(None)),
            "{rate} Hz: a carry outside its rounding argument gave {composition:?}"
        );
    }
}

// ---- #1467: the live tail gain and the statement (L2, L5'; L3' above) -------------------------

/// The cascade of one frame as one 4-state system, `z' = A z + b x`, `y = c . z + d x`, with
/// `z = (s_hpf, s_lpf)` and the HPF's output the LPF's input, in exact (`f64`) arithmetic.
#[derive(Clone, Copy)]
struct CascadeFrame {
    a: [[f64; 4]; 4],
    b: [f64; 4],
    c: [f64; 4],
    d: f64,
}

fn cascade_frame(hpf: &SvfWords, lpf: &SvfWords) -> CascadeFrame {
    let (a1, b1, c1, d1) = (hpf.a(), hpf.b(), hpf.c(), hpf.d());
    let (a2, b2, c2, d2) = (lpf.a(), lpf.b(), lpf.c(), lpf.d());
    CascadeFrame {
        a: [
            [a1[0][0], a1[0][1], 0.0, 0.0],
            [a1[1][0], a1[1][1], 0.0, 0.0],
            [b2[0] * c1[0], b2[0] * c1[1], a2[0][0], a2[0][1]],
            [b2[1] * c1[0], b2[1] * c1[1], a2[1][0], a2[1][1]],
        ],
        b: [b1[0], b1[1], b2[0] * d1, b2[1] * d1],
        c: [d2 * c1[0], d2 * c1[1], c2[0], c2[1]],
        d: d2 * d1,
    }
}

fn apply_frame(a: &[[f64; 4]; 4], z: &[f64; 4]) -> [f64; 4] {
    core::array::from_fn(|i| a[i][0] * z[0] + a[i][1] * z[1] + a[i][2] * z[2] + a[i][3] * z[3])
}

fn row_times(row: &[f64; 4], a: &[[f64; 4]; 4]) -> [f64; 4] {
    core::array::from_fn(|j| {
        row[0] * a[0][j] + row[1] * a[1][j] + row[2] * a[2][j] + row[3] * a[3][j]
    })
}

fn dot_frame(row: &[f64; 4], z: &[f64; 4]) -> f64 {
    row[0] * z[0] + row[1] * z[1] + row[2] * z[2] + row[3] * z[3]
}

fn square_frame(a: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    core::array::from_fn(|i| core::array::from_fn(|j| (0..4).map(|k| a[i][k] * a[k][j]).sum()))
}

/// `A^steps z`, by squaring.
fn power_apply_frame(a: &[[f64; 4]; 4], mut steps: u64, z: &[f64; 4]) -> [f64; 4] {
    let (mut power, mut result) = (*a, *z);
    while steps > 0 {
        if steps & 1 == 1 {
            result = apply_frame(&power, &result);
        }
        power = square_frame(&power);
        steps >>= 1;
    }
    result
}

/// One retarget of one section of a recorded history (#1467 L2): `target(rate, section, hz)`
/// applied to the real input section before frame `N - before` is processed.
#[derive(Clone, Copy, Debug)]
struct Retarget {
    before: usize,
    section: usize,
    hz: f32,
}

/// Frames recorded before and from `N`: every retarget is at most 63 frames before `N`, and the
/// kernel's words are fixed from `N + 64` on at the latest.
const HISTORY_LEAD: usize = 72;
const HISTORY_AFTER: usize = 80;

/// One history of the real input section from `N`, recorded from the kernel (#1467 L2): the HPF's
/// and the LPF's words the kernel loads at frame `N + j` (read from [`InputBuiltins`] before that
/// frame is processed, with zero input throughout), and the sections whose integrators the kernel
/// clears before frame `N + j`; the words are the same from `N + fixed_from` on.
struct KernelHistory {
    frames: Vec<(SvfWords, SvfWords)>,
    clears: Vec<[bool; 2]>,
    fixed_from: usize,
}

impl KernelHistory {
    /// The words at frame `N + j`, for any `j`.
    fn words(&self, j: usize) -> (SvfWords, SvfWords) {
        self.frames[j.min(self.fixed_from)]
    }

    /// Whether the kernel clears `section`'s integrators before frame `N + j`.
    fn cleared(&self, j: usize, section: usize) -> bool {
        j <= self.fixed_from && self.clears[j][section]
    }
}

/// The words the kernel loads for the left channel's HPF and LPF, the identity included.
fn loaded_words(input: &InputBuiltins) -> [SvfWords; 2] {
    let words = input_section_words(input);
    core::array::from_fn(|section| {
        let [c1, a2, a3, _k, m0, m1, m2] = words[section].map(f32::from_bits);
        SvfWords::from_f32([c1, a2, a3, m0, m1, m2])
    })
}

fn is_identity(words: &SvfWords) -> bool {
    *words == SvfWords::from_f32([0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
}

/// The input section of `start = (hpf, lpf)` at `trim_db`, driven one frame per block up to `N`
/// with zero input and `events` applied before their frames.
fn drive_to_n(rate: u32, start: (f32, f32), trim_db: f32, events: &[Retarget]) -> InputBuiltins {
    let mut section = input(rate, start.0, start.1, trim_db, false);
    for frame in 0..HISTORY_LEAD {
        apply_retargets(rate, &mut section, events, frame);
        let (mut left, mut right) = ([0.0_f32], [0.0_f32]);
        process(&mut section, &mut left, &mut right, frame as u64);
    }
    section
}

fn apply_retargets(rate: u32, section: &mut InputBuiltins, events: &[Retarget], frame: usize) {
    for event in events {
        if frame + event.before == HISTORY_LEAD {
            section
                .apply_prepared_filter(target(rate, event.section, event.hz))
                .expect("retarget");
        }
    }
}

/// Records the history of `start = (hpf, lpf)` with `events` from the real kernel (#1407's rules
/// as the kernel runs them: an in-flight re-send, a disable that freezes the recursion, ramps the
/// mix and completes with the identity and cleared integrators, an enable from rest that jumps the
/// recursion and ramps the mix, and the all-six ramp otherwise; its own event timing). A clear is
/// read where a section's words become the identity: only a disable's completion writes the
/// identity over other words, and it clears the integrators in the same step. (Once a section is
/// the identity, its state neither moves nor reaches the output; in L2's scan every event comes
/// before `N` and every clear at `N + 1` or later, so a cleared section stays the identity to the
/// end of the history and no row depends on the clear. An enable after a completed disable would
/// need it, since rule 3 starts from zero integrators; the oracle applies it as the kernel does.)
fn record_history(rate: u32, start: (f32, f32), events: &[Retarget]) -> KernelHistory {
    let mut section = input(rate, start.0, start.1, 0.0, false);
    let mut frames = Vec::with_capacity(HISTORY_AFTER);
    let mut clears = Vec::with_capacity(HISTORY_AFTER);
    let mut previous = loaded_words(&section);
    for frame in 0..HISTORY_LEAD + HISTORY_AFTER {
        apply_retargets(rate, &mut section, events, frame);
        let words = loaded_words(&section);
        if frame >= HISTORY_LEAD {
            clears.push(core::array::from_fn(|s| {
                is_identity(&words[s]) && !is_identity(&previous[s])
            }));
            frames.push((words[0], words[1]));
        }
        previous = words;
        let (mut left, mut right) = ([0.0_f32], [0.0_f32]);
        process(&mut section, &mut left, &mut right, frame as u64);
    }
    assert!(
        state_is_rest(&section),
        "zero input keeps the state at rest"
    );
    let last = frames[frames.len() - 1];
    let mut fixed_from = frames.len() - 1;
    while fixed_from > 0
        && frames[fixed_from - 1] == last
        && !clears[fixed_from].iter().any(|clear| *clear)
    {
        fixed_from -= 1;
    }
    assert!(
        fixed_from <= 64,
        "{rate} Hz {start:?} {events:?}: words fixed only from N + {fixed_from}"
    );
    KernelHistory {
        frames,
        clears,
        fixed_from,
    }
}

/// The cascade frames `0 ..= fixed_from` of `history`, each clear folded in: a clear before frame
/// `j` zeroes the cleared section's state rows (its rows of `A` and entries of `b`) of frame
/// `j - 1`. A clear before `N` acts on the zero state at `N` and changes nothing.
fn history_frames(history: &KernelHistory) -> Vec<CascadeFrame> {
    let mut frames: Vec<CascadeFrame> = (0..=history.fixed_from)
        .map(|j| {
            let (hpf, lpf) = history.words(j);
            cascade_frame(&hpf, &lpf)
        })
        .collect();
    for j in 1..=history.fixed_from {
        for section in 0..2 {
            if history.cleared(j, section) {
                for row in 2 * section..2 * section + 2 {
                    frames[j - 1].a[row] = [0.0; 4];
                    frames[j - 1].b[row] = 0.0;
                }
            }
        }
    }
    frames
}

/// Bounds of a free response of the fixed cascade `frame` (zero input) from the state
/// `z = (s_hpf, s_lpf)`, by each section's `V`-norm contraction (`math::tail::v_operator_norm`,
/// an upper bound of `||A||_V`): `(sup_t |y(t)|, sum_t |y(t)|)`. With `q` each section's
/// contraction, `gamma = ||c||_V*`, `beta = ||b||_V`: `|y_1(t)| <= gamma_1 q_1^t ||s_1||`,
/// `||s_2(t)|| <= q_2^t ||s_2|| + beta_2 sum_{i < t} q_2^(t - 1 - i) |y_1(i)|`, and
/// `|y(t)| <= gamma_2 ||s_2(t)|| + |d_2| |y_1(t)|`. A section with a zero output row and input
/// column (the identity) contributes no state term.
fn free_response_bounds(hpf: &SvfWords, lpf: &SvfWords, z: &[f64; 4]) -> (f64, f64) {
    use math::tail::{v_dual_norm, v_norm};
    let (s1, s2) = (v_norm([z[0], z[1]]), v_norm([z[2], z[3]]));
    let (gamma1, gamma2) = (v_dual_norm(hpf.c()), v_dual_norm(lpf.c()));
    let beta2 = v_norm(lpf.b());
    let d2 = lpf.d().abs();
    let first = if gamma1 == 0.0 { 0.0 } else { gamma1 * s1 };
    let q1 = v_operator_norm(hpf.a());
    let q2 = v_operator_norm(lpf.a());
    assert!(first == 0.0 || q1 < 1.0, "a contracting first section");
    let second_state = if gamma2 == 0.0 && beta2 == 0.0 {
        0.0
    } else {
        assert!(q2 < 1.0, "a contracting second section");
        s2 + beta2 * first / (1.0 - q2)
    };
    let sup = gamma2 * second_state + d2 * first;
    let first_sum = if first == 0.0 {
        0.0
    } else {
        first / (1.0 - q1)
    };
    let second_sum = if gamma2 == 0.0 && beta2 == 0.0 {
        0.0
    } else {
        (s2 + beta2 * first_sum) / (1.0 - q2)
    };
    (sup, gamma2 * second_sum + d2 * first_sum)
}

/// The oracle's stopping level: frames are computed exactly until each rigorous remainder is below
/// it, and the remainders are added to the returned supremum.
const ORACLE_REMAINDER: f64 = 1.0e-12;
/// How many frames of a settled system's prefix `l1` are kept.
const SETTLED_PREFIX: usize = 1 << 17;

/// The settled cascade `(hpf, lpf)`'s impulse response `h_s` in sums: `prefix[k]` the sum of
/// `|h_s(i)|` over `i <= k` (for `k < SETTLED_PREFIX`), `total` the sum over the frames computed,
/// and `remainder` a rigorous bound of the sum over the rest (`free_response_bounds` from the state
/// `A^k b`).
struct SettledSum {
    prefix: Vec<f64>,
    total: f64,
    remainder: f64,
}

impl SettledSum {
    fn of(hpf: &SvfWords, lpf: &SvfWords) -> Self {
        const CHECK: usize = 65_536;
        let settled = cascade_frame(hpf, lpf);
        let mut prefix = Vec::with_capacity(SETTLED_PREFIX);
        let mut row = settled.c;
        let mut total = settled.d.abs();
        let mut steps = 0_usize;
        loop {
            if prefix.len() < SETTLED_PREFIX {
                prefix.push(total);
            }
            total += dot_frame(&row, &settled.b).abs();
            row = row_times(&row, &settled.a);
            steps += 1;
            if steps.is_multiple_of(CHECK) {
                // The terms not yet summed: the free response from `A^steps b`.
                let z = power_apply_frame(&settled.a, steps as u64, &settled.b);
                let remainder = free_response_bounds(hpf, lpf, &z).1;
                if remainder <= ORACLE_REMAINDER {
                    return Self {
                        prefix,
                        total,
                        remainder,
                    };
                }
                assert!(steps < 64 * CHECK, "the settled sum does not converge");
            }
        }
    }

    /// At least the sum of `|h_s(i)|` over `i <= k`: exact inside the prefix, the whole sum's
    /// bound beyond it.
    fn up_to(&self, k: usize) -> f64 {
        self.prefix
            .get(k)
            .copied()
            .unwrap_or(self.total + self.remainder)
    }
}

/// The settled sums of every settled pair one thread meets, by the words' bits.
#[derive(Default)]
struct SettledSums(std::collections::HashMap<[u64; 12], std::sync::Arc<SettledSum>>);

impl SettledSums {
    fn of(&mut self, (hpf, lpf): (SvfWords, SvfWords)) -> std::sync::Arc<SettledSum> {
        let key = [
            hpf.c1, hpf.a2, hpf.a3, hpf.m0, hpf.m1, hpf.m2, lpf.c1, lpf.a2, lpf.a3, lpf.m0, lpf.m1,
            lpf.m2,
        ]
        .map(f64::to_bits);
        self.0
            .entry(key)
            .or_insert_with(|| std::sync::Arc::new(SettledSum::of(&hpf, &lpf)))
            .clone()
    }
}

/// #1467 L2's oracle: the exact supremum, over every frame `n >= N` and every input with
/// `|x| <= 1` from `N` on (zero state at `N`), of the time-varying cascade's output on a recorded
/// kernel history. At frame `n` that supremum is the row `l1` `sum_m |h(n, m)|` (the input
/// `sign h(n, m)` attains it), computed by the backward (adjoint) recursion: inside the history's
/// changing frames (`N .. N + fixed_from`) the row `c_n A_(n-1) ... A_(m+1)` frame by frame, the
/// kernel's clears folded in ([`history_frames`]); after them `h(n, m) = c A^(n - r) v_m` for each
/// earlier input `m` (`v_m` its state at `N + r`, `r = fixed_from`) with the row `c A^i` stepped
/// forward, plus the settled system's prefix `l1` ([`SettledSum`]). The rows after `N + r` are
/// exact until the window's rigorous remainder (`free_response_bounds` from the columns' states)
/// is below [`ORACLE_REMAINDER`]; beyond, every row is at most that remainder plus the settled
/// sum's bound, which the returned value includes. Returns `(supremum, remainder)`; `record` sees
/// each computed row `(n - N, l1)`.
fn exact_row_supremum(
    history: &KernelHistory,
    settled_sum: &SettledSum,
    record: &mut dyn FnMut(usize, f64),
) -> (f64, f64) {
    const CHECK: usize = 4_096;
    let frames = history_frames(history);
    let ramp = history.fixed_from;
    let (settled_hpf, settled_lpf) = history.words(ramp);
    let settled = frames[ramp];
    let mut supremum = 0.0_f64;
    // Inside the changing frames: rows by the backward recursion.
    for n in 0..ramp {
        let mut row = frames[n].c;
        let mut l1 = frames[n].d.abs();
        for m in (0..n).rev() {
            l1 += dot_frame(&row, &frames[m].b).abs();
            row = row_times(&row, &frames[m].a);
        }
        record(n, l1);
        supremum = supremum.max(l1);
    }
    // Each earlier input's state at `N + ramp`.
    let columns: Vec<[f64; 4]> = (0..ramp)
        .map(|m| {
            let mut z = frames[m].b;
            for frame in &frames[m + 1..ramp] {
                z = apply_frame(&frame.a, &z);
            }
            z
        })
        .collect();
    // From `N + ramp`: `row = c A^steps`, the settled system's prefix `l1`.
    let mut row = settled.c;
    let mut steps = 0_usize;
    let window_remainder = loop {
        let window: f64 = columns.iter().map(|v| dot_frame(&row, v).abs()).sum();
        let value = window + settled_sum.up_to(steps);
        record(ramp + steps, value);
        supremum = supremum.max(value);
        row = row_times(&row, &settled.a);
        steps += 1;
        if steps.is_multiple_of(CHECK) {
            // Past this frame each column's output is a free response from its state here.
            let remainder: f64 = columns
                .iter()
                .map(|v| {
                    let z = power_apply_frame(&settled.a, steps as u64, v);
                    free_response_bounds(&settled_hpf, &settled_lpf, &z).0
                })
                .sum();
            if remainder <= ORACLE_REMAINDER {
                break remainder;
            }
            assert!(steps < 1 << 22, "the oracle's window does not converge");
        }
    };
    let remainder = window_remainder + settled_sum.remainder;
    (supremum.max(settled_sum.total + remainder), remainder)
}

/// The output at frame `N + n` of a unit impulse at frame `N + m <= N + n` (zero state before it),
/// through the kernel's equations (as [`impulse_response`]) with the history's words and clears
/// frame by frame, in `f64`.
fn forward_response(history: &KernelHistory, m: usize, n: usize) -> f64 {
    let mut state = [[0.0_f64; 2]; 2];
    let mut out = 0.0;
    for frame in m..=n {
        let (hpf, lpf) = history.words(frame);
        for (section, state) in state.iter_mut().enumerate() {
            if frame > m && history.cleared(frame, section) {
                *state = [0.0; 2];
            }
        }
        let mut x = if frame == m { 1.0 } else { 0.0 };
        for (words, state) in [hpf, lpf].iter().zip(state.iter_mut()) {
            let v3 = x - state[1];
            let d1 = -words.c1 * state[0] + words.a2 * v3;
            let d2 = words.a3 * v3 + words.a2 * state[0];
            let v1 = state[0] + d1;
            let v2 = state[1] + d2;
            state[0] += 2.0 * d1;
            state[1] += 2.0 * d2;
            x = words.m0 * x + words.m1 * v1 + words.m2 * v2;
        }
        out = x;
    }
    out
}

/// The oracle's coefficients `h(n, m)`, `m = 0 ..= n`, of the row at `N + n`: the backward
/// recursion of [`exact_row_supremum`] over the history's frames (fixed after `fixed_from`).
fn row_coefficients(history: &KernelHistory, n: usize) -> Vec<f64> {
    let frames = history_frames(history);
    let frame = |j: usize| frames[j.min(history.fixed_from)];
    let mut coefficients = vec![0.0_f64; n + 1];
    let mut row = frame(n).c;
    coefficients[n] = frame(n).d;
    for m in (0..n).rev() {
        coefficients[m] = dot_frame(&row, &frame(m).b);
        row = row_times(&row, &frame(m).a);
    }
    coefficients
}

/// The real `f32` kernel's `|y(N + n)|` for the input `amplitude sign h(n, m)` at `N + m` (zero
/// before `N`), trim +24 dB, on the history `start`/`events`, rendered from `N` in one block; and
/// the oracle's value for it, `trim amplitude sum_m |h(n, m)|`.
fn kernel_against_oracle(
    rate: u32,
    start: (f32, f32),
    events: &[Retarget],
    history: &KernelHistory,
    n: usize,
) -> (f64, f64) {
    let amplitude = 1.0_f32 / 4_096.0;
    let coefficients = row_coefficients(history, n);
    let mut section = drive_to_n(rate, start, 24.0, events);
    let mut left: Vec<f32> = coefficients
        .iter()
        .map(|h| if *h >= 0.0 { amplitude } else { -amplitude })
        .collect();
    let mut right = left.clone();
    let trim = trim_gain(&section);
    process(&mut section, &mut left, &mut right, HISTORY_LEAD as u64);
    let expected = trim * f64::from(amplitude) * coefficients.iter().map(|h| h.abs()).sum::<f64>();
    (f64::from(left[n].abs()), expected)
}

/// One history of L2's scan: its label, start pair `(hpf, lpf)` and retargets.
type ScanHistory = (String, (f32, f32), Vec<Retarget>);

/// L2's scan at `rate` (see the test).
fn l2_scan(rate: u32) -> Vec<ScanHistory> {
    let top = maximum(rate);
    let worst = below(top);
    let points = [
        0.0, 10.0, 100.0, 200.0, 500.0, 1_000.0, 10_000.0, worst, top,
    ];
    let valid = |hpf: f32, lpf: f32| hpf == 0.0 || lpf == 0.0 || hpf < lpf;
    let at = |before: usize, section: usize, hz: f32| Retarget {
        before,
        section,
        hz,
    };
    let pair = |section: usize, moving: f32, other: f32| {
        if section == 0 {
            (moving, other)
        } else {
            (other, moving)
        }
    };
    let mut scan: Vec<ScanHistory> = Vec::new();
    // (a) One section retargeted once.
    for section in 0..2 {
        let others = if section == 0 {
            [top, 0.0]
        } else {
            [10.0, 0.0]
        };
        for other in others {
            for from in points {
                for to in points {
                    let (start, end) = (pair(section, from, other), pair(section, to, other));
                    if from == to || !valid(start.0, start.1) || !valid(end.0, end.1) {
                        continue;
                    }
                    for before in [1, 2, 8, 16, 32, 48, 63] {
                        scan.push((
                            format!("section {section} {from} -> {to} Hz at N - {before}, other {other} Hz"),
                            start,
                            vec![at(before, section, to)],
                        ));
                    }
                }
            }
        }
    }
    // (b) Every settled pair.
    for hpf in points {
        for lpf in points {
            if valid(hpf, lpf) {
                scan.push((
                    format!("settled {hpf} Hz into {lpf} Hz"),
                    (hpf, lpf),
                    Vec::new(),
                ));
            }
        }
    }
    // (c) One section retargeted twice (the second in flight of the first, a re-send included).
    for section in 0..2 {
        let (other, chain) = if section == 0 {
            (top, [0.0, 10.0, 200.0, 1_000.0, worst])
        } else {
            (10.0, [0.0, 100.0, 1_000.0, 10_000.0, top])
        };
        for a in chain {
            for b in chain {
                for c in chain {
                    if a == b {
                        continue;
                    }
                    for (first, second) in [(63, 1), (40, 8), (16, 15), (2, 1)] {
                        scan.push((
                            format!(
                                "section {section} {a} -> {b} Hz at N - {first} -> {c} Hz at N - {second}"
                            ),
                            pair(section, a, other),
                            vec![at(first, section, b), at(second, section, c)],
                        ));
                    }
                }
            }
        }
    }
    // (d) Both sections retargeted.
    for a in [0.0, 10.0, 200.0, 1_000.0] {
        for b in [0.0, 10.0, 200.0, 1_000.0] {
            for c in [0.0, 10_000.0, worst, top] {
                for d in [0.0, 10_000.0, worst, top] {
                    if a == b || c == d {
                        continue;
                    }
                    for (first, second) in [(1, 1), (1, 63), (63, 1), (32, 8)] {
                        scan.push((
                            format!("HPF {a} -> {b} Hz at N - {first}, LPF {c} -> {d} Hz at N - {second}"),
                            (a, c),
                            vec![at(first, 0, b), at(second, 1, d)],
                        ));
                    }
                }
            }
        }
    }
    scan
}

/// #1467 L2. The live tail gain is at least the exact supremum of the late input's part on the
/// time-varying cascade, over a scan of histories recorded from the real kernel. With zero state
/// at `N`, the whole output from `N` on is the part `G_t` bounds (H1), and its supremum over every
/// input `|x| <= 1` from `N` is the row-`l1` supremum ([`exact_row_supremum`]) times the trim word
/// of +24 dB; an input from `M > N` is one of these with zeros before `M`, so it covers every `M`.
/// The words are the real kernel's, frame by frame ([`record_history`]): #1407's rules 1 to 4 and
/// the kernel's event timing (an event before frame `N - s` has `64 - s` frames in flight at `N`).
/// The scan at every launch rate ([`l2_scan`]), on the cutoffs `{the identity, 10 Hz, 100 Hz,
/// 200 Hz, 500 Hz, 1 kHz, 10 kHz, one f32 below the maximum, the maximum}` (HPF below LPF when
/// both are enabled):
///
/// * (a) one section retargeted between every ordered pair of the cutoffs (a disable, an enable
///   from rest or an all-six ramp), the other section fixed (LPF at the maximum or the identity;
///   HPF at 10 Hz or the identity), the event before `N - s`, `s` in `{1, 2, 8, 16, 32, 48, 63}`;
/// * (b) every settled pair of the cutoffs;
/// * (c) one section retargeted twice, `a -> b` then `-> c`, on five cutoffs per section (the
///   identity among them; `a != b`, `c` any of the five, `c = b` a re-send), the events before
///   `(N - 63, N - 1)`,
///   `(N - 40, N - 8)`, `(N - 16, N - 15)` and `(N - 2, N - 1)`;
/// * (d) both sections retargeted, the HPF on `{the identity, 10 Hz, 200 Hz, 1 kHz}` and the LPF
///   on `{the identity, 10 kHz, one f32 below the maximum, the maximum}`, the events before
///   `(N - 1, N - 1)`, `(N - 1, N - 63)`, `(N - 63, N - 1)` and `(N - 32, N - 8)`.
///
/// It covers those histories only, the input at `N` and later in exact arithmetic on the kernel's
/// `f32` words (not the rounding deviation: `G_t`'s derivation bounds the late input's relative
/// rounding, and the flush part is B1's `sigma_t`). The oracle checks itself against
/// independent brute force (a settled impulse response; forward impulse sums) and against the
/// real `f32` kernel, which a worst-sign input of `2^-12` drives to the oracle's row.
#[test]
#[cfg_attr(debug_assertions, ignore = "release scale (#1467 L2)")]
fn live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps() {
    let trim = f64::from(math::pow(10.0, 24.0 / 20.0) as f32);
    std::thread::scope(|scope| {
        let handles: Vec<_> = rates()
            .iter()
            .map(|&rate| scope.spawn(move || l2_at_rate(rate, trim)))
            .collect();
        for handle in handles {
            for line in handle.join().expect("L2 rate") {
                eprintln!("{line}");
            }
        }
    });
}

/// L2 at one rate; returns the evidence lines.
fn l2_at_rate(rate: u32, trim: f64) -> Vec<String> {
    const THREADS: usize = 6;
    let (_, _, tail_gain, _, _) = composition_values(
        input_section_live_bound(rate).expect("launch rate"),
        "live bound",
    );
    let g_t = linear(tail_gain);
    let scan = l2_scan(rate);
    let count = scan.len();
    // Per thread: (worst trim x supremum and its label, largest remainder, histories with a
    // disable completed, with an enable from rest, with a retarget in flight at `N`).
    type Partial = ((f64, String), f64, [usize; 3]);
    let partials: Vec<Partial> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..THREADS)
            .map(|thread| {
                let scan = &scan;
                scope.spawn(move || {
                    let mut sums = SettledSums::default();
                    let mut worst = (0.0_f64, String::new());
                    let mut largest_remainder = 0.0_f64;
                    let mut kinds = [0_usize; 3];
                    for (label, start, events) in scan.iter().skip(thread).step_by(THREADS) {
                        let history = record_history(rate, *start, events);
                        kinds[0] += usize::from(history.clears.iter().flatten().any(|c| *c));
                        // A design target on a section that starts disabled and at rest.
                        kinds[1] += usize::from(events.iter().any(|event| {
                            event.hz != 0.0
                                && (if event.section == 0 { start.0 } else { start.1 }) == 0.0
                        }));
                        kinds[2] += usize::from(history.fixed_from > 0);
                        let settled = sums.of(history.words(history.fixed_from));
                        let (supremum, remainder) =
                            exact_row_supremum(&history, &settled, &mut |_, _| {});
                        largest_remainder = largest_remainder.max(remainder);
                        let reached = trim * supremum;
                        assert!(
                            reached <= g_t,
                            "{rate} Hz {label}: the exact supremum {reached:e} exceeds g_t {g_t:e}"
                        );
                        if reached > worst.0 {
                            worst = (reached, label.clone());
                        }
                    }
                    (worst, largest_remainder, kinds)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("L2 thread"))
            .collect()
    });
    let mut worst = (0.0_f64, String::new());
    let mut largest_remainder = 0.0_f64;
    let mut kinds = [0_usize; 3];
    for (thread_worst, remainder, thread_kinds) in partials {
        if thread_worst.0 > worst.0 {
            worst = thread_worst;
        }
        largest_remainder = largest_remainder.max(remainder);
        for (kind, more) in kinds.iter_mut().zip(thread_kinds) {
            *kind += more;
        }
    }
    let mut lines = Vec::new();
    // The oracle against independent brute force: a settled pair's supremum is its impulse
    // response's `l1` (the partial sums only rise).
    let (hpf, lpf) = (
        design_words(rate, 0, 10.0),
        design_words(rate, 1, maximum(rate)),
    );
    let settled_history = record_history(rate, (10.0, maximum(rate)), &[]);
    let settled = exact_row_supremum(
        &settled_history,
        &SettledSum::of(&hpf, &lpf),
        &mut |_, _| {},
    )
    .0;
    let brute = l1(&impulse_response(&[hpf, lpf], 2_000_000));
    assert!(
        (settled - brute).abs() <= 1.0e-9 * brute,
        "{rate} Hz: the oracle's settled supremum {settled} against the impulse response's l1 \
         {brute}"
    );
    lines.push(format!(
        "L2 {rate} Hz: settled 10 Hz into the maximum {settled:.10} (impulse response l1 \
         {brute:.10})"
    ));
    // On recorded histories (a retarget, a disable and an enable at `N - 1`, a disable
    // interrupted by a retarget, an enable interrupted by a disable): the rows inside and after
    // the changing frames are the sums of `|y(n)|` over unit impulses at every earlier frame,
    // each run forward through the kernel's equations with the frame's words and clears; and the
    // real `f32` kernel, driven by the input `2^-12 sign h(n, m)` from `N`, gives `|y(N + n)|`
    // within `1e-3` of the oracle's `trim 2^-12 row`.
    let checks: [(&str, (f32, f32), Vec<Retarget>); 6] = [
        (
            "retarget 10 -> 100 Hz at N - 1",
            (10.0, maximum(rate)),
            vec![Retarget {
                before: 1,
                section: 0,
                hz: 100.0,
            }],
        ),
        (
            "HPF disable 100 Hz at N - 1",
            (100.0, maximum(rate)),
            vec![Retarget {
                before: 1,
                section: 0,
                hz: 0.0,
            }],
        ),
        (
            "HPF enable 100 Hz at N - 1",
            (0.0, maximum(rate)),
            vec![Retarget {
                before: 1,
                section: 0,
                hz: 100.0,
            }],
        ),
        (
            "LPF disable 1 kHz at N - 1",
            (10.0, 1_000.0),
            vec![Retarget {
                before: 1,
                section: 1,
                hz: 0.0,
            }],
        ),
        (
            "HPF disable 100 Hz at N - 40, retarget 1 kHz at N - 8",
            (100.0, maximum(rate)),
            vec![
                Retarget {
                    before: 40,
                    section: 0,
                    hz: 0.0,
                },
                Retarget {
                    before: 8,
                    section: 0,
                    hz: 1_000.0,
                },
            ],
        ),
        (
            "HPF enable 200 Hz at N - 16, disable at N - 15",
            (0.0, maximum(rate)),
            vec![
                Retarget {
                    before: 16,
                    section: 0,
                    hz: 200.0,
                },
                Retarget {
                    before: 15,
                    section: 0,
                    hz: 0.0,
                },
            ],
        ),
    ];
    for (what, start, events) in &checks {
        let history = record_history(rate, *start, events);
        let settled = SettledSum::of(
            &history.words(history.fixed_from).0,
            &history.words(history.fixed_from).1,
        );
        let mut rows = [0.0_f64; 2];
        exact_row_supremum(&history, &settled, &mut |n, value| match n {
            40 => rows[0] = value,
            200 => rows[1] = value,
            _ => {}
        });
        for (index, n) in [40_usize, 200].into_iter().enumerate() {
            let forward: f64 = (0..=n)
                .map(|m| forward_response(&history, m, n).abs())
                .sum();
            assert!(
                (rows[index] - forward).abs() <= 1.0e-12 * forward.max(1.0),
                "{rate} Hz {what}: the oracle's row at N + {n} {} against the forward sum \
                 {forward}",
                rows[index]
            );
            let (kernel, oracle) = kernel_against_oracle(rate, *start, events, &history, n);
            assert!(
                (kernel / oracle - 1.0).abs() <= 1.0e-3,
                "{rate} Hz {what}: the real kernel's |y(N + {n})| {kernel:e} against the \
                 oracle's {oracle:e}"
            );
            lines.push(format!(
                "L2 {rate} Hz {what}: row at N + {n} {:.10} (forward {forward:.10}); kernel \
                 against oracle {:.6}",
                rows[index],
                kernel / oracle
            ));
        }
    }
    lines.push(format!(
        "L2 {rate} Hz: {count} histories ({} with a disable completed after N, {} with an enable \
         from rest, {} with a retarget in flight at N); largest trim x supremum {:.6e} ({:.2} dB, \
         {}), g_t {g_t:.6e} ({:.2} dB), margin {:.2} dB; largest remainder \
         {largest_remainder:.1e}",
        kinds[0],
        kinds[1],
        kinds[2],
        worst.0,
        20.0 * math::log10(worst.0),
        worst.1,
        20.0 * math::log10(g_t),
        20.0 * math::log10(g_t / worst.0)
    ));
    lines
}

/// #1467 L5'. The live bound states all five values: `decay` is the accessor's `D`, and each gain
/// and stall is the accessor's raw value rounded up to millibels (at least `ceil(2000 log10)` of
/// it and at most that of the value one part in a million above, which covers the module's
/// rounding margin); and the two orders the registry would check, which never sees this bound:
/// `tail_gain <= peak_gain` and `tail_stall <= peak_stall` (#1484 rule (g)), raw and in millibels,
/// at every launch rate.
#[test]
fn live_bound_states_the_composition_rounded_up_with_its_orders() {
    for &rate in rates() {
        let composition = live_composition(rate);
        let (decay, peak_gain, tail_gain, peak_stall, tail_stall) = composition_values(
            input_section_live_bound(rate).expect("launch rate"),
            "live bound",
        );
        eprintln!(
            "L5' {rate} Hz: D {decay}, G_p {peak_gain} mB, G_t {tail_gain} mB, sigma_p {peak_stall} \
             mB, sigma_t {tail_stall} mB"
        );
        assert_eq!(decay, composition.decay, "{rate} Hz: D");
        for (name, stated, raw) in [
            ("G_p", peak_gain, composition.peak_gain),
            ("G_t", tail_gain, composition.tail_gain),
            ("sigma_p", peak_stall, composition.peak_stall),
            ("sigma_t", tail_stall, composition.tail_stall),
        ] {
            assert!(
                ceil_mb(raw) <= stated && stated <= ceil_mb(raw * (1.0 + 1.0e-6)),
                "{rate} Hz {name}: stated {stated} mB is not the raw {raw:e} rounded up"
            );
        }
        assert!(
            composition.tail_gain <= composition.peak_gain && tail_gain <= peak_gain,
            "{rate} Hz: G_t above G_p"
        );
        assert!(
            composition.tail_stall <= composition.peak_stall && tail_stall <= peak_stall,
            "{rate} Hz: sigma_t above sigma_p"
        );
    }
}
