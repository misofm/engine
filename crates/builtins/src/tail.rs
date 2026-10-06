//! The builtin input section's certified tail and exact-rest bounds (issue #1329; decision 15
//! D15-4(b)).
//!
//! Control-plane only. [`fixed_input_bound`] runs once per distinct design when a session is
//! prepared (`crate::input_section_bounds`); [`input_section_live_bound`] is called by the compiler
//! for a session with a live input lane. Neither is reachable from a render path, both allocate
//! (through `math::tail`), and nothing render owns stores their results (#1329 Amendment 4, R5).
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

use effect_contract::{RestSamples, TailSamples};
use math::tail::{
    CascadeBound, EnvelopeSection, FlushLaw, LiveCascade, PoleDomain, SvfWords, fixed_cascade,
    live_cascade, output_rounding, state_rounding, v_dual_norm, v_norm, word_box_norm,
};

use crate::filter_control::INPUT_FILTER_RAMP_SAMPLES;
use crate::{BUTTERWORTH_K, InputLane, SvfSection, builtin_filter_cutoff_maximum_hz, db_gain};

/// The tail and exact-rest bounds of one builtin input section (#1329 D1, D2, Amendment 3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputSectionBound {
    /// `T_decay`: the tail every tail report uses (PDC, the C ABI, the browser). Valid for every
    /// input peak at or above the section's flush floor `P*`.
    pub tail: TailSamples,
    /// `T_rest = max(T_decay, R(P*))`: the tail over every peak. From it on the output is below
    /// `P * 10^(-144/20)` for `P >= P*` and exactly zero for `P < P*`.
    pub tail_every_peak: TailSamples,
    /// D2's exact-rest bound, the one silence skipping uses (#1107). `None` states no bound.
    pub rest: Option<RestSamples>,
}

impl InputSectionBound {
    /// A memoryless section (both filters disabled): trim and polarity have no tail.
    pub const ZERO: Self = Self {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Finite(0),
        rest: Some(RestSamples::ZERO),
    };

    /// No bound stated: what a section reports if the derivation cannot bound it.
    pub const UNBOUNDED: Self = Self {
        tail: TailSamples::Infinite,
        tail_every_peak: TailSamples::Infinite,
        rest: None,
    };

    fn from_cascade(bound: &CascadeBound) -> Self {
        Self {
            tail: TailSamples::Finite(bound.tail),
            tail_every_peak: TailSamples::Finite(bound.tail_every_peak),
            rest: Some(RestSamples {
                peak_plus_24_dbfs: bound.rest_peak,
                any_sanitized_input: bound.rest_any,
            }),
        }
    }

    /// The componentwise maximum: the bound of a section whose channels are bounded by `self` and
    /// `other` (D4: max over left and right).
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        let tail = |left: TailSamples, right: TailSamples| match (left, right) {
            (TailSamples::Finite(left), TailSamples::Finite(right)) => {
                TailSamples::Finite(left.max(right))
            }
            _ => TailSamples::Infinite,
        };
        Self {
            tail: tail(self.tail, other.tail),
            tail_every_peak: tail(self.tail_every_peak, other.tail_every_peak),
            rest: match (self.rest, other.rest) {
                (Some(left), Some(right)) => Some(RestSamples {
                    peak_plus_24_dbfs: left.peak_plus_24_dbfs.max(right.peak_plus_24_dbfs),
                    any_sanitized_input: left.any_sanitized_input.max(right.any_sanitized_input),
                }),
                _ => None,
            },
        }
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

/// D4: the bound of a prepared input section whose designs never change, the maximum over its
/// two channels. Each channel is its trim (a gain of `|trim|` on the peak) followed by its enabled
/// sections in order, HPF then LPF; a disabled section is the exact identity and is left out.
pub(crate) fn fixed_input_bound(sample_rate: u32, lanes: [&InputLane; 2]) -> InputSectionBound {
    #[cfg(any(test, feature = "test-support"))]
    FIXED_INPUT_BOUNDS.with(|count| count.set(count.get() + 1));
    let _environment = lane::CanonicalFpEnv::enter();
    let law = input_section_flush_law(sample_rate);
    let peaks = rest_peaks();
    let channel = |lane: &InputLane| -> InputSectionBound {
        let mut sections = [section_words(&lane.hpf); 2];
        let mut count = 0;
        for section in [&lane.hpf, &lane.lpf] {
            if section.enabled {
                sections[count] = section_words(section);
                count += 1;
            }
        }
        if count == 0 {
            return InputSectionBound::ZERO;
        }
        let gain = f64::from(lane.trim_signed.abs());
        fixed_cascade(&sections[..count], gain, &law, peaks)
            .map_or(InputSectionBound::UNBOUNDED, |bound| {
                InputSectionBound::from_cascade(&bound)
            })
    };
    let left = channel(lanes[0]);
    // The common case, two channels with the same design and trim magnitude, is bounded once.
    let same = |a: &SvfSection, b: &SvfSection| a.words() == b.words() && a.enabled == b.enabled;
    if same(&lanes[0].hpf, &lanes[1].hpf)
        && same(&lanes[0].lpf, &lanes[1].lpf)
        && lanes[0].trim_signed.abs().to_bits() == lanes[1].trim_signed.abs().to_bits()
    {
        return left;
    }
    left.max(channel(lanes[1]))
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times [`fixed_input_bound`] ran on this thread: one per design bound computed
    /// (#1329 Amendment 5, MJ1). Read through `crate::test_support::fixed_input_bounds_computed`.
    pub(crate) static FIXED_INPUT_BOUNDS: core::cell::Cell<u64> =
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
pub fn input_section_live_bound(sample_rate: u32) -> Option<InputSectionBound> {
    let terms = input_section_live_envelope(sample_rate)?;
    Some(
        live_bound(sample_rate, &terms).map_or(InputSectionBound::UNBOUNDED, |bound| {
            InputSectionBound::from_cascade(&bound)
        }),
    )
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
///   (#1407); an `f32` design is within the half-ulp box `h = (u/2, u/4, u/2)` of the exact
///   design (`c1`, `a3 < 1`, `a2 < 1/2`). The kernel's contraction `||A(w)||_V + mu_state(w)` is
///   convex in the words, so over the hull it is largest at a design. The exact designs split at
///   `g1 = g_max / 2`: above it the norm is at most `rho(g_max)` and `a2` at most `a2(g1)`
///   (`a2 = g / (1 + g (g + sqrt(2)))` falls for `g > 1`); below it the norm is at most
///   `max(rho(g1), rho(g_min))` and `a2` at most its global maximum `1 / (2 + sqrt(2))`. The
///   `f32` box adds `P(h)`, the allowance `P(E)` ([`word_box_norm`]), and the rounding counts take
///   every word at its largest magnitude plus `E + h`.
/// * **Input column.** `||b||_V = 2 g / sqrt(1 + g (g + sqrt(2))) < 2`, plus `E + h` on `(a2, a3)`
///   and the input rounding.
/// * **Output row.** For both mixes `||c||_V* = sqrt(2) / sqrt(1 + g (g + sqrt(2)))`, largest at
///   10 Hz; a mix ramp toward or from the identity scales the row by its weight. The words' `E + h`,
///   the mix words' own ramp allowance and `|fl(sqrt(2)) - sqrt(2)|` (the HPF's band mix) add to it.
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
///   output rounding on the state, `omega_state`, is taken over every word (below).
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
    let (omega_state, omega_input) = output_rounding(&largest);
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
    // `omega_state` as a supremum: [`output_rounding`] is a norm of terms each increasing in one
    // of `|c1|`, `|1 - c1|`, `|a2|`, `|a3|`, `|1 - a3|` and the mix words, so its value at the
    // largest words plus its value at the opposite corner (`c1`, `a3` at `-(E + h)`, where
    // `|1 - c1|` and `|1 - a3|` are largest) bounds it for every word.
    let corner = SvfWords {
        c1: -off_design[0],
        a3: -off_design[2],
        ..largest
    };
    let first_output_rounding = omega_state + output_rounding(&corner).0;
    Some(LiveCascade {
        envelope,
        poles,
        first_mix_row,
        first_output_rounding,
    })
}
