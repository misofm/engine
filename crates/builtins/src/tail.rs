//! The builtin input section's certified tail and exact-rest bounds (issue #1329; decision 15
//! D15-4(b)).
//!
//! Control-plane only. When a session is prepared (`crate::input_section_bounds`),
//! [`fixed_input_bound`] runs at most once per distinct design, and only while the preparation's
//! budget covers it and no [`InputBoundCache`] given to the preparation holds the design (#1457);
//! a design past the budget reports the live bound. The compiler reads the live bound from
//! [`input_section_live_bound_table`] and computes [`input_section_live_bound`] nowhere; the table's
//! gate computes it. Neither computation is reachable from a render path, both allocate (through
//! `math::tail`), and nothing render owns stores their results (#1329 Amendment 4, R5).
//!
//! Every bound is computed under the canonical floating-point environment
//! (`lane::CanonicalFpEnv`): round to nearest with full subnormals, as on every target, whatever
//! flush-to-zero mode the calling host left installed, and the caller's control and status word is
//! restored bit for bit afterwards (the `f64` decays can underflow, which raises a status flag).
//!
//! The derivation is `docs/derivations/1329-input-section-tail-and-rest.md`; the sums and the
//! kernel propagation are `math::tail`. This module supplies the input section's own terms: its
//! designed `f32` words (D4), and for a live input lane the per-section suprema over every word a
//! control history can reach (D5).

use effect_contract::{CompositionBound, NodeTailBound, RestBound, RestSamples, TailSamples};
use math::tail::{
    CascadeBound, EnvelopeSection, FlushLaw, LiveCascade, PoleDomain, SvfWords,
    fixed_cascade_within, live_cascade, output_rounding, state_rounding, v_dual_norm, v_norm,
    word_box_norm,
};

use crate::filter_control::INPUT_FILTER_RAMP_SAMPLES;
use crate::{BUTTERWORTH_K, InputLane, SvfSection, builtin_filter_cutoff_maximum_hz, db_gain};

/// A builtin input section's bound from a cascade's: its tail, tail over every peak and exact-rest
/// bound (#1329 D1, D2, Amendment 3). Its composition is unstated until #1465 and #1467 derive it.
fn bound_from_cascade(bound: &CascadeBound) -> NodeTailBound {
    NodeTailBound {
        tail: TailSamples::Finite(bound.tail),
        tail_every_peak: TailSamples::Finite(bound.tail_every_peak),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: bound.rest_peak,
            any_sanitized_input: bound.rest_any,
        }),
        composition: CompositionBound::Unstated,
    }
}

/// What a fixed input section's bound depends on: the rate, and per channel the seven words of each
/// section and the trim magnitude (the bound reads nothing else). `crate::input_section_bounds`
/// computes each distinct key's bound once per preparation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct InputBoundKey {
    sample_rate: u32,
    channels: [([u32; 7], [u32; 7], u32); 2],
}

pub(crate) fn input_bound_key(sample_rate: u32, lanes: [&InputLane; 2]) -> InputBoundKey {
    InputBoundKey {
        sample_rate,
        channels: lanes.map(|lane| {
            (
                lane.hpf.words(),
                lane.lpf.words(),
                lane.trim_signed.abs().to_bits(),
            )
        }),
    }
}

/// `2^-24`, the `f32` unit roundoff.
const U: f64 = 1.0 / 16_777_216.0;

/// The flush law of #1328 at `sample_rate`, from the engine's own constants.
#[must_use]
pub fn input_section_flush_law(sample_rate: u32) -> FlushLaw {
    FlushLaw {
        flush_eps: f64::from(lane::FLUSH_EPS),
        rest_eps: f64::from(lane::REST_EPS),
        silence_frames: u64::from(lane::silence_frames(sample_rate)),
    }
}

/// The two peaks D2 states exact rest for: +24 dBFS (as the `f32` gain word of +24 dB, which is
/// at or above `10^(24/20)`) and the input sanitizer's limit.
fn rest_peaks() -> [f64; 2] {
    [
        f64::from(max_trim_gain()),
        f64::from(lane::kernels::builtins::NONFINITE_LIMIT),
    ]
}

/// The largest trim word magnitude: the `f32` gain of +24 dB, the top of the trim domain. A trim
/// ramp stays inside its endpoints (#1408), so no trim word exceeds it.
fn max_trim_gain() -> f32 {
    db_gain(24.0).expect("+24 dB is inside the trim domain")
}

fn section_words(section: &SvfSection) -> SvfWords {
    SvfWords::from_f32([
        section.c1, section.a2, section.a3, section.m0, section.m1, section.m2,
    ])
}

/// One design's bound and its charge (#1457 D1, D3, Amendment 3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChargedInputBound {
    /// The design's own certified bound ([`crate::input_section_bound`]).
    pub bound: NodeTailBound,
    /// The frames `math::tail::fixed_cascade` walked to compute it, over both channels.
    pub frames: u64,
    /// What it charges a preparation's budget, in frame-equivalents, whether computed or read from
    /// an [`InputBoundCache`]: its [`frames`](Self::frames) plus [`INPUT_BOUND_SECTION_CHARGE`]
    /// for each enabled section it walks (both channels' when they differ, one channel's when
    /// they are the same).
    pub charge: u64,
}

/// D4: the bound of a prepared input section whose designs never change, the maximum over its
/// two channels. Each channel is its trim (a gain of `|trim|` on the peak) followed by its enabled
/// sections in order, HPF then LPF; a disabled section is the exact identity and is left out.
///
/// #1457 D3: the computation charges at most `budget` frame-equivalents over both channels
/// ([`ChargedInputBound::charge`]); `None` when it would charge more. Each cascade's section
/// charges are reserved before its walk, so the frames walked never pass the budget, a stopped
/// design included. Under a budget at or above its charge the result is the same, bit for bit.
pub(crate) fn fixed_input_bound(
    sample_rate: u32,
    lanes: [&InputLane; 2],
    budget: u64,
) -> Option<ChargedInputBound> {
    let (bound, walked) = fixed_input_walk(sample_rate, lanes, budget);
    #[cfg(any(test, feature = "test-support"))]
    {
        if let Some(bound) = bound {
            FIXED_INPUT_BOUNDS.with(|count| count.set(count.get() + 1));
            FIXED_INPUT_CHARGED.with(|count| count.set(count.get() + bound.charge));
        }
        // The frames really walked, a stopped walk's included (#1457 attempt 1, MJ2).
        FIXED_INPUT_FRAMES.with(|count| count.set(count.get() + walked));
    }
    // Only the test-support counters read the frames walked.
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = walked;
    bound
}

/// The bound under `budget` and the frames walked, finished or stopped.
fn fixed_input_walk(
    sample_rate: u32,
    lanes: [&InputLane; 2],
    budget: u64,
) -> (Option<ChargedInputBound>, u64) {
    let _environment = lane::CanonicalFpEnv::enter();
    let law = input_section_flush_law(sample_rate);
    let peaks = rest_peaks();
    let channel = |lane: &InputLane, budget: u64| -> (Option<ChargedInputBound>, u64) {
        let mut sections = [section_words(&lane.hpf); 2];
        let mut count = 0;
        for section in [&lane.hpf, &lane.lpf] {
            if section.enabled {
                sections[count] = section_words(section);
                count += 1;
            }
        }
        if count == 0 {
            let zero = ChargedInputBound {
                bound: NodeTailBound::ZERO,
                frames: 0,
                charge: 0,
            };
            return (Some(zero), 0);
        }
        // The sections' fixed charge is reserved first: what is left is the walk's horizon.
        let fixed = INPUT_BOUND_SECTION_CHARGE * count as u64;
        let Some(horizon) = budget.checked_sub(fixed) else {
            return (None, 0);
        };
        let gain = f64::from(lane.trim_signed.abs());
        let walk = fixed_cascade_within(&sections[..count], gain, &law, peaks, horizon);
        let charged = walk.result.map(|result| ChargedInputBound {
            bound: result.map_or(NodeTailBound::UNBOUNDED, |bound| bound_from_cascade(&bound)),
            frames: walk.frames,
            charge: walk.frames + fixed,
        });
        (charged, walk.frames)
    };
    // A memoryless design (no section enabled on either channel) walks and charges nothing.
    if lanes
        .iter()
        .all(|lane| !lane.hpf.enabled && !lane.lpf.enabled)
    {
        return channel(lanes[0], 0);
    }
    let (left, left_walked) = channel(lanes[0], budget);
    let Some(left) = left else {
        return (None, left_walked);
    };
    // The common case, two channels with the same design and trim magnitude, is bounded once.
    let same = |a: &SvfSection, b: &SvfSection| a.words() == b.words() && a.enabled == b.enabled;
    let (right, right_walked) = if same(&lanes[0].hpf, &lanes[1].hpf)
        && same(&lanes[0].lpf, &lanes[1].lpf)
        && lanes[0].trim_signed.abs().to_bits() == lanes[1].trim_signed.abs().to_bits()
    {
        let none = ChargedInputBound {
            bound: left.bound,
            frames: 0,
            charge: 0,
        };
        (Some(none), 0)
    } else {
        channel(lanes[1], budget - left.charge)
    };
    let walked = left_walked + right_walked;
    let charged = right.map(|right| ChargedInputBound {
        bound: left.bound.max(right.bound),
        frames: left.frames + right.frames,
        charge: left.charge + right.charge,
    });
    (charged, walked)
}

/// #1457 D1 (Amendments 3 and 4): the fixed charge of each enabled section a design's computation
/// walks, in frame-equivalents (both channels' sections when they differ, one channel's when they
/// are the same). A design has no fixed cost of its own beyond its sections' (the calibration's
/// per-design term is negative at every rate), so it carries no other charge.
///
/// A frame-equivalent is 17.5 ns on the CI-class runner: the slowest frame class's measured
/// maximum, rounded up (#1474 and root's rulings: every class, and a stated worst case is a bound,
/// so the maximum, not the median). Since the walk's certified flush (`math::tail`'s `tau`, #1474)
/// no propagated quantity is subnormal, and a frame's cost depends on the walk's length, not its
/// cutoffs: net of the design's fixed cost, long near-top walks take 13.6-13.7 ns a frame, gate 2's
/// cheap two-section designs 12.8-16.1 ns and typical designs (an HPF at 20-80 Hz into an LPF at
/// 16-20 kHz) 15.0-16.5 ns, the slowest an 80 Hz HPF into an 18 kHz LPF at 44.1 kHz (16.54 ns, the
/// median of five runs, spread 16.39-17.10; 17.5 ns is above its maximum). The largest measured
/// fixed cost per section is 7.03 us (a two-section cascade at 96 kHz, 401.7 frame-equivalents), so
/// 410 frame-equivalents (the next ten up, 7.18 us) bound the fixed cost of every class measured.
/// Calibrated with `examples/input_bound_budget.rs calibrate` and confirmed against every gate-2
/// family; the classes, rates and runs are in #1474's attempt record. A change to the walk reruns
/// the calibration and restates the frame-equivalent and this charge.
pub const INPUT_BOUND_SECTION_CHARGE: u64 = 410;

/// #1457 D1: the budget of one preparation's design bounds, in frame-equivalents, charged in strip
/// order (`crate::input_section_bounds`). Each distinct design computed charges the frames it walks
/// plus [`INPUT_BOUND_SECTION_CHARGE`] per section; a cache hit charges the same stored amount.
///
/// At 17.5 ns a frame-equivalent ([`INPUT_BOUND_SECTION_CHARGE`]) it is 26.43 ms of design-bound
/// work on the CI-class runner; this workstation (x86-64-v3, release, one pinned core) stood in
/// for that runner. Its purpose is that typical sessions bound every distinct design exactly: the
/// 64-track console documents (64 distinct designs) fit at every launch rate. Every frame and every
/// section is charged at or above its measured cost, so the budget bounds the real work of every
/// design family measured (gate 2, `examples/input_bound_budget.rs`), the slowest frame class
/// included; the attempt record states the measured worst case and its spread. A design past the
/// budget reports the rate's live bound (D3).
pub const INPUT_BOUND_BUDGET_FRAMES: u64 = 1_510_000;

/// #1457 D2: the entry cap of [`InputBoundCache::new`]. The cache holds only designs with an
/// enabled section, and such a design charges at least 667 frame-equivalents (a walk of at least
/// 257 frames, one 256-frame block of the majorant pass and one deviation frame, plus one section's
/// charge). So one preparation computes at most `INPUT_BOUND_BUDGET_FRAMES / 667` = 2,263 designs
/// exactly, and stops one more: the cap holds a whole preparation's designs. A rebuild of an
/// unchanged session is served entirely from the cache only while the cache has not been cleared
/// since that session's designs were inserted: a cache shared by several sessions (an engine's,
/// #1471) can fill and clear in the middle of a preparation, and a cleared design is computed
/// again, with the same result. A full cache of 8,192 entries holds about 2.4 MiB (2,486,520 bytes
/// measured, about 304 bytes an entry; the attempt record).
pub const INPUT_BOUND_CACHE_ENTRIES: usize = 8192;

/// A control-side cache of input-section design bounds across preparations (#1457 D2), keyed by
/// what a design's bound depends on (the rate, both channels' section words and trim magnitudes).
///
/// An entry keeps either its design's bound with its charge ([`ChargedInputBound`]), or, for a
/// design whose computation a preparation's remaining budget stopped, that remaining budget (its
/// charge is above it). A hit charges the preparation's budget exactly what the computation would:
/// the stored charge, or, when the remaining budget is at most the stored one, the whole remainder
/// and the live bound, which is what the stopped computation did. So every reported bound is the
/// same with and without the cache, cold or warm (gate 3), and a rebuild of an unchanged session
/// walks no frame while the cache still holds its designs. When an insertion would pass the entry
/// cap the cache is cleared first (clear-on-full): the cap bounds its memory, and a cleared design
/// is computed again with the same result ([`INPUT_BOUND_CACHE_ENTRIES`] states when a rebuild is
/// served entirely from the cache). Render never reads it.
#[derive(Clone, Debug)]
pub struct InputBoundCache {
    entries: std::collections::BTreeMap<InputBoundKey, CachedDesign>,
    capacity: core::num::NonZeroUsize,
}

/// One cached design (#1457 D2).
#[derive(Clone, Copy, Debug)]
pub(crate) enum CachedDesign {
    /// The design's bound, the frames its walk took and its charge.
    Bound(ChargedInputBound),
    /// The design charges more than this many frame-equivalents (the remaining budget that
    /// stopped it, above zero).
    ChargeAbove(u64),
}

impl InputBoundCache {
    /// An empty cache holding at most `capacity` designs.
    #[must_use]
    pub const fn with_capacity(capacity: core::num::NonZeroUsize) -> Self {
        Self {
            entries: std::collections::BTreeMap::new(),
            capacity,
        }
    }

    /// An empty cache holding at most [`INPUT_BOUND_CACHE_ENTRIES`] designs.
    #[must_use]
    pub const fn new() -> Self {
        Self::with_capacity(
            core::num::NonZeroUsize::new(INPUT_BOUND_CACHE_ENTRIES).expect("a nonzero cap"),
        )
    }

    /// The designs cached.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no design is cached.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entry cap.
    #[must_use]
    pub const fn capacity(&self) -> core::num::NonZeroUsize {
        self.capacity
    }

    pub(crate) fn get(&self, key: &InputBoundKey) -> Option<CachedDesign> {
        self.entries.get(key).copied()
    }

    pub(crate) fn insert(&mut self, key: InputBoundKey, value: CachedDesign) {
        if self.entries.len() >= self.capacity.get() && !self.entries.contains_key(&key) {
            self.entries.clear();
        }
        self.entries.insert(key, value);
    }
}

impl Default for InputBoundCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many design bounds [`fixed_input_bound`] computed on this thread (#1329 Amendment 5,
    /// MJ1): one per call whose walk finished inside its horizon (#1457); a stopped walk computes
    /// no bound. Read through `crate::test_support::fixed_input_bounds_computed`.
    pub(crate) static FIXED_INPUT_BOUNDS: core::cell::Cell<u64> =
        const { core::cell::Cell::new(0) };
    /// How many frames [`fixed_input_bound`]'s walks took on this thread, stopped walks included
    /// (#1457). Read through `crate::test_support::fixed_input_frames_walked`.
    pub(crate) static FIXED_INPUT_FRAMES: core::cell::Cell<u64> =
        const { core::cell::Cell::new(0) };
    /// The charges of the bounds [`fixed_input_bound`] computed on this thread, in
    /// frame-equivalents (#1457 Amendment 3). Read through
    /// `crate::test_support::fixed_input_charged`.
    pub(crate) static FIXED_INPUT_CHARGED: core::cell::Cell<u64> =
        const { core::cell::Cell::new(0) };
}

/// The worst-case pair of D5 at each launch rate: the HPF one `f32` below the maximum cutoff into
/// the LPF at the maximum cutoff, the slowest-decaying pair the domain admits (both sections at the
/// slowest pole). `None` off the launch rates. #1262's gate 4 enables this pair live.
#[must_use]
pub const fn input_section_worst_case_pair(sample_rate: u32) -> Option<(f32, f32)> {
    match builtin_filter_cutoff_maximum_hz(sample_rate) {
        Some(maximum) => Some((f32::from_bits(maximum.to_bits() - 1), maximum)),
        None => None,
    }
}

/// D5: the bound of an input section whose trim, polarity and filter targets may change live,
/// over every history: each section disabled or designed anywhere in `[10 Hz, maximum]`, the trim
/// anywhere up to +24 dB, and any history of 64-frame filter ramps (#1407), at any block size.
/// The HPF-to-LPF cascade is bounded frequency-aware (#1433, `math::tail::live_cascade`).
/// `None` off the launch rates (only a launch rate has a cutoff domain).
#[must_use]
pub fn input_section_live_bound(sample_rate: u32) -> Option<NodeTailBound> {
    let terms = input_section_live_envelope(sample_rate)?;
    Some(
        live_bound(sample_rate, &terms)
            .map_or(NodeTailBound::UNBOUNDED, |bound| bound_from_cascade(&bound)),
    )
}

/// [`input_section_live_bound`] at each launch rate, as a table (#1457 D2): the live bound depends
/// only on the rate, so preparation reads it here and computes it nowhere (it costs about 0.25 ms
/// per computation, #1433). `None` off the launch rates, as there.
///
/// The test `live_bound_table_is_the_computed_live_bound_at_every_launch_rate`
/// (`tests/tail_contract.rs`) holds every entry equal to [`input_section_live_bound`], so a change
/// to the derivation, the flush law or the cutoff domain turns it red until the table is restated.
#[must_use]
pub const fn input_section_live_bound_table(sample_rate: u32) -> Option<NodeTailBound> {
    const fn bound(tail: u64, every_peak: u64, rest_peak: u64, rest_any: u64) -> NodeTailBound {
        NodeTailBound {
            tail: TailSamples::Finite(tail),
            tail_every_peak: TailSamples::Finite(every_peak),
            rest: RestBound::Bounded(RestSamples {
                peak_plus_24_dbfs: rest_peak,
                any_sanitized_input: rest_any,
            }),
            composition: CompositionBound::Unstated,
        }
    }
    match sample_rate {
        44_100 => Some(bound(704_010, 704_010, 1_067_207, 2_384_997)),
        48_000 => Some(bound(699_952, 699_952, 1_061_497, 2_372_008)),
        88_200 => Some(bound(704_018, 704_018, 1_071_057, 2_388_800)),
        96_000 => Some(bound(699_960, 699_960, 1_065_688, 2_376_147)),
        _ => None,
    }
}

/// The full `math::tail` result behind [`input_section_live_bound`], with its flush floor `P*`
/// and its exact half: evidence for the derivation and the gates. `None` off the launch rates or
/// when no bound is proven.
#[must_use]
pub fn input_section_live_cascade(sample_rate: u32) -> Option<CascadeBound> {
    live_bound(sample_rate, &input_section_live_envelope(sample_rate)?).ok()
}

fn live_bound(
    sample_rate: u32,
    terms: &LiveCascade,
) -> Result<CascadeBound, math::tail::TailBoundError> {
    let _environment = lane::CanonicalFpEnv::enter();
    live_cascade(
        terms,
        f64::from(max_trim_gain()),
        &input_section_flush_law(sample_rate),
        u64::from(INPUT_FILTER_RAMP_SAMPLES),
        rest_peaks(),
    )
}

/// The proven rounding allowance of #1407's live retarget law at block size 1, its worst: how far,
/// per word, a word the kernel loads can lie from the convex hull of the `f32` words its history
/// used.
///
/// A ramp from `c` to a target `t` has step `s = fl(t - c) / 64`; its word after frame `j` is the
/// mixture `(1 - j/64) c + (j/64) t` up to `rho_j = j h + (j/64) |t - c| u` for `j <= 4` (stepped
/// as `fl(w + s)`) and `rho_j = (1 - j/64) |t - c| (2u + u^2) + h` for `5 <= j <= 63` (computed as
/// `fl(t - fl(s * (64 - j)))`), `h` the word's half-ulp and `|t - c|` at most the word's `range`.
/// A restart from a word off the hull carries `(1 - j/64)` of its error, so with restarts every
/// frame the start error is `E_start = max_j (64 / j) rho_j` and any word's error at most
/// `E = max_j (1 - j/64) E_start + rho_j`: `64 h + u D` (#1407 "Numerical limits").
fn ramp_word_allowance(range: [f64; 3], half_ulp: [f64; 3]) -> [f64; 3] {
    core::array::from_fn(|index| {
        let rho = |j: u32| {
            let j = f64::from(j);
            if j <= 4.0 {
                j * half_ulp[index] + j / 64.0 * range[index] * U
            } else {
                (64.0 - j) / 64.0 * range[index] * (2.0 * U + U * U) + half_ulp[index]
            }
        };
        let start = (1..64)
            .map(|j| 64.0 / f64::from(j) * rho(j))
            .fold(0.0_f64, f64::max);
        (1..64)
            .map(|j| f64::from(64 - j) / 64.0 * start + rho(j))
            .fold(0.0_f64, f64::max)
    })
}

/// The exact Butterworth design's pole radius at prewarped frequency `g`, which is its `V`-norm:
/// `rho(g) = sqrt(1 + g^4) / (1 + sqrt(2) g + g^2)`. Symmetric under `g -> 1 / g` and smallest at
/// `g = 1`, so over an interval of `g` it is largest at an endpoint.
fn design_radius(g: f64) -> f64 {
    math::sqrt(1.0 + g * g * g * g) / (1.0 + core::f64::consts::SQRT_2 * g + g * g)
}

/// D5's per-section suprema over every word a live input section can load, at `sample_rate`, and
/// where those words lie as poles (#1433): everything [`input_section_live_bound`] reads. Both
/// sections share it: each may be disabled or designed anywhere in the cutoff domain.
///
/// * **Recursion words.** Every word is a design, the identity at rest, or within the allowance
///   `E` (the ramp word allowance below) of the convex hull of the `f32` designs its history used
///   (#1407); an `f32` design is within the half-ulp box `h = (u/2, u/4, u/2)` plus the design's
///   `f64` evaluation error (a few `f64` ulps, which the pole domain's pads and the scan's checks
///   cover) of the exact design (`c1`, `a3 < 1`, `a2 < 1/2`). The kernel's contraction `||A(w)||_V + mu_state(w)` is
///   convex in the words, so over the hull it is largest at a design. The exact designs split at
///   `g1 = g_max / 2`: above it the norm is at most `rho(g_max)` and `a2` at most `a2(g1)`
///   (`a2 = g / (1 + g (g + sqrt(2)))` falls for `g > 1`); below it the norm is at most
///   `max(rho(g1), rho(g_min))` and `a2` at most its global maximum `1 / (2 + sqrt(2))`. The
///   `f32` box adds `P(h)`, the allowance `P(E)` ([`word_box_norm`]), and the state and input rounding
///   counts, which increase in every word's magnitude, take every word at its largest magnitude
///   plus `E + h`.
/// * **Input column.** `||b||_V = 2 g / sqrt(1 + g (g + sqrt(2))) < 2`, plus `E + h` on `(a2, a3)`
///   and the input rounding.
/// * **Output row.** For both mixes `||c||_V* = sqrt(2) / sqrt(1 + g (g + sqrt(2)))`, largest at
///   10 Hz; a mix ramp toward or from the identity scales the row by its weight. The words' `E + h`,
///   the mix words' own ramp allowance and `|fl(sqrt(2)) - sqrt(2)|` (the HPF's band mix) add to it,
///   and so does the output rounding on the state, `omega_state` ([`output_rounding`]), as a
///   supremum over every word: its value at the largest words plus its value at the corner where
///   `c1` and `a3` are `-(E + h)`. Its `|1 - c1|` and `|1 - a3|` terms peak at small `c1` and
///   `a3`, so its value at the largest words alone is not a supremum (#1433).
///   `|d|` is at most `1` (identity `1`, HPF `1 / (1 + g (g + sqrt(2)))`, LPF `a3`), with the same
///   corrections.
/// * **Pole domain (#1433, `math::tail::PoleDomain`).** An exact design's pole has real part
///   `Re p = (1 - g^2) / (1 + sqrt(2) g + g^2)`, falling in `g`, so over the domain it lies in
///   `[Re p(g_max), Re p(g_min)]`; both ends are padded by `1e-12`, far above the `f64` error of
///   `g` (near Nyquist `tan` amplifies its argument's rounding to about `1e-11` relative in `g`,
///   which moves `Re p = -1 + O(1 / g)` by below `1e-15`). A ramp word lies within `E + h` of the
///   exact designs' hull, a settled word (an `f32` design) within `h`. The rounding counts are
///   the ones above.
/// * **First section's mix row.** The HPF's mix words are `theta (-k, -1)` (`theta` in `[0, 1]`,
///   a crossfade toward or from the identity) within the mix allowance, so
///   `||(m1, m2)||_V* <= ||(-k, -1)||_V* + ` the allowance box's largest dual norm; the box term
///   (above `1e-6`) and the final `1 + 2^-30` cover this evaluation's few `f64` roundings. Its
///   output rounding on the state is the output row's supremum `omega_state`.
#[must_use]
pub fn input_section_live_envelope(sample_rate: u32) -> Option<LiveCascade> {
    let _environment = lane::CanonicalFpEnv::enter();
    let maximum = f64::from(builtin_filter_cutoff_maximum_hz(sample_rate)?);
    let rate = f64::from(sample_rate);
    let sqrt2 = core::f64::consts::SQRT_2;
    let g_max = math::tan(core::f64::consts::PI * maximum / rate);
    let g_min = math::tan(core::f64::consts::PI * 10.0 / rate);
    let half_ulp = [U / 2.0, U / 4.0, U / 2.0];
    let allowance = ramp_word_allowance([1.0, 0.3, 1.0], half_ulp);
    let off_design: [f64; 3] = core::array::from_fn(|i| allowance[i] + half_ulp[i]);
    let ramp_norm = word_box_norm(allowance);
    let design_box_norm = word_box_norm(half_ulp);

    // The two groups of designs: the top (`g >= g1`) and the rest.
    let g1 = g_max / 2.0;
    let a2_top = g1 / (1.0 + g1 * (g1 + sqrt2));
    let a2_max = 1.0 / (2.0 + sqrt2);
    let q_top = design_radius(g_max) + design_box_norm;
    let q_other = design_radius(g1).max(design_radius(g_min)) + design_box_norm;
    let beta_sup = 2.0;
    let rounding = |a2: f64, extra: [f64; 3], q: f64| {
        state_rounding([1.0 + extra[0], a2 + extra[1], 1.0 + extra[2]], q, beta_sup)
    };
    let ramp_top = rounding(a2_top, off_design, q_top + ramp_norm);
    let ramp_other = rounding(a2_max, off_design, q_other + ramp_norm);
    let settled_top = rounding(a2_top, half_ulp, q_top);
    let settled_other = rounding(a2_max, half_ulp, q_other);
    let rho_ramp = (q_top + ramp_top.0).max(q_other + ramp_other.0) + ramp_norm;
    let rho_settled = (q_top + settled_top.0).max(q_other + settled_other.0);

    // The input column, off the design by `E + h` on `(a2, a3)`.
    let mut column = 0.0_f64;
    for signs in 0..4_u32 {
        let sign = |bit: u32| if signs & bit == 0 { 1.0 } else { -1.0 };
        column = column.max(v_norm([
            2.0 * off_design[1] * sign(1),
            2.0 * off_design[2] * sign(2),
        ]));
    }
    let input_rounding = ramp_top.1.max(ramp_other.1);

    // The output row and feedthrough. The mix words `(m0, m1, m2)` ramp over `(1, k, 1)` with
    // half-ulps `(u/2, u, u/2)` (`|m1| = fl(sqrt(2)) < 2`).
    let k = f64::from(BUTTERWORTH_K);
    let k_error = (k - sqrt2).abs();
    let mix = ramp_word_allowance([1.0, k, 1.0], [U / 2.0, U, U / 2.0]);
    let t_min = g_min * (g_min + sqrt2);
    let gamma_design = sqrt2 / math::sqrt(1.0 + t_min);
    let row = [
        k * off_design[0] + off_design[1] + mix[1] + mix[2] * a2_max + k_error,
        k * off_design[1] + off_design[2] + mix[1] * a2_max + mix[2] + k_error * a2_max,
    ];
    let second = row[0] + sqrt2 * row[1];
    let row_correction = math::sqrt(row[0] * row[0] + second * second);
    let delta_sup = 1.0
        + mix[0]
        + (k + mix[1]) * off_design[1]
        + (1.0 + mix[2]) * off_design[2]
        + mix[1] * a2_max
        + mix[2]
        + k_error * a2_max;
    let largest = SvfWords {
        c1: 1.0 + off_design[0],
        a2: a2_max + off_design[1],
        a3: 1.0 + off_design[2],
        m0: 1.0 + mix[0],
        m1: k + mix[1],
        m2: 1.0 + mix[2],
    };
    // `omega_state` as a supremum (#1433): [`output_rounding`] is a norm of terms each increasing
    // in one of `|c1|`, `|1 - c1|`, `|a2|`, `|a3|`, `|1 - a3|` and the mix words, so its value at
    // the largest words plus its value at the opposite corner (`c1`, `a3` at `-(E + h)`, where
    // `|1 - c1|` and `|1 - a3|` are largest) bounds it for every word. Its value at the largest
    // words alone is not a supremum: the `U |1 - c1|` and `U |1 - a3|` terms are largest at small
    // `c1` and `a3`, which mid-band designs reach. `omega_input` has no such term.
    let (omega_largest, omega_input) = output_rounding(&largest);
    let corner = SvfWords {
        c1: -off_design[0],
        a3: -off_design[2],
        ..largest
    };
    let omega_state = omega_largest + output_rounding(&corner).0;
    let envelope = EnvelopeSection {
        rho_ramp,
        rho_settled,
        input: beta_sup + column + input_rounding,
        output_state: gamma_design + row_correction + omega_state,
        output_input: delta_sup + omega_input,
    };

    // #1433: where the words lie as poles, and the first section's mix row.
    const POLE_PAD: f64 = 1.0e-12;
    let real_pole = |g: f64| (1.0 - g * g) / (1.0 + sqrt2 * g + g * g);
    let poles = PoleDomain {
        low: real_pole(g_max) - POLE_PAD,
        high: real_pole(g_min) + POLE_PAD,
        ramp_box: off_design,
        design_box: half_ulp,
        state_rounding: ramp_top.0.max(ramp_other.0),
        settled_state_rounding: settled_top.0.max(settled_other.0),
        input_rounding,
    };
    let mut mix_box = 0.0_f64;
    for signs in 0..4_u32 {
        let sign = |bit: u32| if signs & bit == 0 { 1.0 } else { -1.0 };
        mix_box = mix_box.max(v_dual_norm([mix[1] * sign(1), mix[2] * sign(2)]));
    }
    let first_mix_row = (v_dual_norm([-k, -1.0]) + mix_box) * (1.0 + 1.0 / 1_073_741_824.0);
    // The first section's output rounding on the state is the supremum above.
    let first_output_rounding = omega_state;
    Some(LiveCascade {
        envelope,
        poles,
        first_mix_row,
        first_output_rounding,
    })
}
