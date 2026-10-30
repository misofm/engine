//! `miso.reverb`: a 32-line feedback-delay-network stereo reverb (#2114).
//!
//! # Shape
//!
//! The design is `dsp-research/reverb.md` ("Algorithm and equations", R1-R12): a pre-delay per
//! channel, four pipelined Schroeder allpass diffusers per channel (one 8-lane group, outside the
//! loop), thirty-two delay lines read with clear-on-read taps, a TPT one-pole low-pass (damping)
//! and a TPT first-order high-pass (low cut) per slot, the feedback matrix
//! `diag(g) (H32 / sqrt 32) P` as a fixed five-stage butterfly (the `1 / sqrt 32` folded into each
//! line gain), two output taps scaled by `fl32(1 / sqrt 32)`, and an explicit smoothed M/S width
//! matrix.
//!
//! The kernel (`PreparedReverb::render`) is one body generic over [`lane::Lane`]: the thirty-two
//! slot values are `32 / W` vectors, the diffusers `8 / W`. The butterfly runs its stages in the
//! fixed order 4, 3, 2, 1, 0 and re-lays the vectors out through a 32-word stack scratch whenever a
//! stage's bit is a lane bit, so `W = 1`, `4` and `8` compute the same `f32` values (a re-layout
//! moves words and rounds nothing). Ring reads and writes are scalar per line, because every line
//! has its own length; the slot permutation `P` is applied at the read for free. Production runs
//! `lane::Native`.
//!
//! The effect never banks: its SIMD is inside one instance, across its own lines.
//!
//! # Where the numerics live
//!
//! * Multiply-add is `Lane::fma`, two roundings on every target; never the standard library's
//!   fused form.
//! * Denormals: `lane::flush` on the 32 line writes, the 64 filter states and the 8 diffuser
//!   writes, once per sample (D7). Nothing else is classified per value.
//! * Finiteness: once per block over both outputs, every ring's write window, the filter states
//!   and the pipeline registers. A failing block zeroes both outputs, drops all signal history
//!   (`history`, the delay's `valid_history`) and counts one non-finite block per channel.
//! * Transcendentals: `math` designs the line lengths, gains and filter coefficients at prepare
//!   and at accepted block-start events; the per-sample kernel has none.

use effect_contract::{
    AutomationRate, EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality,
    LatencySamples, LinkModeSet, NativeEffectFactory, NodeTailBound, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedEffect, PreparedEffectMetadata,
    PreparedNativeEffect, ProcessReport, ResetKind, SmoothingRule, StatePayloadError,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, expected_prepared_metadata,
};
use effect_runtime::params::{ParameterSpec, normalize_zero, parameter_value_valid};
use effect_runtime::ramp::LinearRamp;
use lane::Lane;

pub mod corpus;
mod kernel;
mod state;
mod tail;
#[cfg(test)]
mod tests;

pub use tail::tail_bound_at;

/// Number of feedback lines.
pub const LINES: usize = 32;
/// Butterfly stages: `log2(LINES)`.
const STAGES: usize = LINES.trailing_zeros() as usize;
const _: () = assert!(1 << STAGES == LINES);
/// Number of input diffusers: four per channel.
const DIFFUSERS: usize = 8;
const PARAMETER_COUNT: usize = 8;
/// Length of every coefficient ramp.
const RAMP_SAMPLES: u32 = 64;
/// Length of every tap crossfade (pre-delay and lines).
const TRANSITION_SAMPLES: u32 = 128;
/// Ring words beyond the longest tap, as the note's capacity rule (`D_i(2, fs) + 128`).
const RING_SLACK: u32 = 128;

/// Line base lengths at 48 kHz and size 1 (the note's `BASE`, line 0..31): `RANKED[k] =
/// Prime(floor(960 * 4.5^(k / 31) + 1/2))`, line `i < 16` takes `RANKED[2 i]` and line `16 + i`
/// takes `RANKED[2 i + 1]` (R3).
const BASE: [u32; LINES] = [
    953, 1061, 1163, 1283, 1409, 1559, 1721, 1889, 2087, 2297, 2531, 2791, 3079, 3389, 3733, 4111,
    1009, 1109, 1223, 1361, 1487, 1637, 1801, 1987, 2179, 2411, 2659, 2927, 3229, 3557, 3919, 4327,
];

/// The shortest and the longest base length.
const BASE_MIN: u32 = const_min_max(&BASE).0;
const BASE_MAX: u32 = const_min_max(&BASE).1;

const fn const_min_max(values: &[u32; LINES]) -> (u32, u32) {
    let (mut low, mut high) = (values[0], values[0]);
    let mut i = 1;
    while i < LINES {
        if values[i] < low {
            low = values[i];
        }
        if values[i] > high {
            high = values[i];
        }
        i += 1;
    }
    (low, high)
}

/// Diffuser lengths at 48 kHz: lanes 0-3 are the left channel's stages 0-3, lanes 4-7 the right's.
const DIFFUSER_BASE: [u32; DIFFUSERS] = [229, 173, 613, 449, 233, 179, 617, 457];

/// Input gain per line (the note's `B`, exact).
const INPUT_GAIN: f32 = 0.5;

/// `1 / sqrt(32)` in `f64`, the matrix normalisation.
const INV_SQRT_LINES: f64 = 0.176_776_695_296_636_87;

/// The output scale `S = fl32(1 / sqrt 32)` (R8): with `B = 1/2` it keeps the 16-line design's wet
/// level. Not a power of two, so it rounds once.
const OUTPUT_SCALE: f32 = INV_SQRT_LINES as f32;

/// Butterfly output index the left output taps (R8). A named constant so the pair can change
/// without touching the kernel.
const OUTPUT_LEFT: usize = 14;
/// Butterfly output index the right output taps (R8).
const OUTPUT_RIGHT: usize = 15;

/// The line slot `j` holds: `pi(j) = (29 j + 31) mod 32` (R4).
const fn line_of_slot(slot: usize) -> usize {
    (29 * slot + 31) % LINES
}

/// The slot a line feeds: `pi^-1(i) = (21 i + 21) mod 32` (`21 = 29^-1 mod 32`).
const fn slot_of_line(line: usize) -> usize {
    (21 * line + 21) % LINES
}

const _: () = {
    let mut j = 0;
    while j < LINES {
        assert!(slot_of_line(line_of_slot(j)) == j);
        j += 1;
    }
};

/// `0.45 * 44_100`, the damping cap shared with the delay (#1374).
const DAMPING_MAX_CUTOFF_HZ: f64 = 19_845.0;

/// The damping cutoff cap at `sample_rate`: `min(19_845 Hz, fs / 4)` (#1374, owner ruling of
/// 2026-10-25), so the designed low-pass coefficient is at most `1/2`.
fn damping_cutoff_max_hz(sample_rate: u32) -> f64 {
    DAMPING_MAX_CUTOFF_HZ.min(f64::from(sample_rate) / 4.0)
}

const fn effect_id(value: &'static str) -> effect_contract::EffectId {
    match effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect identifier"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port identifier"),
    }
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero static parameter identifier"),
    }
}

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    name: &'static str,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    mapping: ParameterMapping,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    smoothing_samples: u32,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: parameter_id(id),
        name,
        display_name,
        display_unit,
        unit,
        domain: ParameterDomain::Continuous,
        minimum: Some(minimum),
        maximum: Some(maximum),
        default_value,
        mapping,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::Linear,
        smoothing_samples,
        readable: true,
        automatable: true,
        enum_choices: &[],
        lattice: effect_contract::default_parameter_lattice(
            unit,
            ParameterDomain::Continuous,
            mapping,
        ),
        prepared_reason: None,
    }
}

/// Overrides one row's unit-class lattice (#242 S1): `size`'s bounds `0.25` and `2.0` need two
/// decimals, which the ratio class's one-decimal default cannot spell.
const fn with_lattice(
    mut parameter: ParameterDescriptor,
    lattice: effect_contract::ParameterLattice,
) -> ParameterDescriptor {
    parameter.lattice = lattice;
    parameter
}

/// Index of `pre_delay_ms` in [`REVERB_PARAMETERS`].
const PRE_DELAY: usize = 0;
/// Index of `decay_ms`.
const DECAY: usize = 1;
/// Index of `size`.
const SIZE: usize = 2;
/// Index of `damping_hz`.
const DAMPING: usize = 3;
/// Index of `low_cut_hz`.
const LOW_CUT: usize = 4;
/// Index of `diffusion`.
const DIFFUSION: usize = 5;
/// Index of `width`.
const WIDTH: usize = 6;
/// Index of `mix`.
const MIX: usize = 7;

/// The eight `miso.reverb` parameters in descriptor and stable-ID order (the note's table). Every
/// row is `Shared`, block-rate and live.
pub const REVERB_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "pre_delay_ms",
        "pre-delay",
        "ms",
        ParameterUnit::Milliseconds,
        ParameterMapping::Linear,
        0.0,
        250.0,
        20.0,
        TRANSITION_SAMPLES,
    ),
    parameter(
        2,
        "decay_ms",
        "decay",
        "ms",
        ParameterUnit::Milliseconds,
        ParameterMapping::Logarithmic,
        100.0,
        20_000.0,
        1_800.0,
        RAMP_SAMPLES,
    ),
    with_lattice(
        parameter(
            3,
            "size",
            "size",
            "ratio",
            ParameterUnit::Ratio,
            ParameterMapping::Linear,
            0.25,
            2.0,
            1.0,
            TRANSITION_SAMPLES,
        ),
        effect_contract::ParameterLattice::arithmetic(0.01, 2),
    ),
    parameter(
        4,
        "damping_hz",
        "damping",
        "Hz",
        ParameterUnit::Hz,
        ParameterMapping::Logarithmic,
        1_000.0,
        20_000.0,
        6_000.0,
        RAMP_SAMPLES,
    ),
    parameter(
        5,
        "low_cut_hz",
        "low cut",
        "Hz",
        ParameterUnit::Hz,
        ParameterMapping::Logarithmic,
        20.0,
        500.0,
        20.0,
        RAMP_SAMPLES,
    ),
    parameter(
        6,
        "diffusion",
        "diffusion",
        "linear",
        ParameterUnit::Linear,
        ParameterMapping::Linear,
        0.0,
        1.0,
        0.7,
        RAMP_SAMPLES,
    ),
    parameter(
        7,
        "width",
        "width",
        "linear",
        ParameterUnit::Linear,
        ParameterMapping::Linear,
        0.0,
        1.0,
        1.0,
        RAMP_SAMPLES,
    ),
    parameter(
        8,
        "mix",
        "mix",
        "linear",
        ParameterUnit::Linear,
        ParameterMapping::Linear,
        0.0,
        1.0,
        1.0,
        RAMP_SAMPLES,
    ),
];

const PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];

/// `true` if `n` is prime (trial division; prepare and `const` only).
const fn const_is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

/// Largest distance [`nearest_prime`] searches. The largest prime gap below 17,500 is 44, so
/// every argument the descriptor domain can produce has a prime within this distance.
const PRIME_SEARCH: u32 = 64;

/// The nearest prime to `n`, the smaller on a tie (the note's `Prime(n)`), by trial division.
const fn const_nearest_prime(n: u32) -> u32 {
    let mut d = 0;
    while d <= PRIME_SEARCH {
        if d <= n && const_is_prime(n - d) {
            return n - d;
        }
        if const_is_prime(n + d) {
            return n + d;
        }
        d += 1;
    }
    panic!("no prime within the search distance")
}

/// `floor(BASE_i * 2 * fs / 48000 + 1/2)` in exact integers: `floor((2a + b) / 2b)` with
/// `a = 2 BASE_i fs` and `b = 48000`. The `f64` form [`line_length`] evaluates rounds to the same
/// integer: its only rounding errors are relative `2^-52`, and a rational with denominator
/// 48,000 is either exactly a half-integer (computed exactly) or at least `1 / 48000` from one.
const fn const_line_capacity(line: usize, sample_rate: u32) -> u32 {
    let doubled = 2 * BASE[line] as u64 * sample_rate as u64;
    let rounded = ((2 * doubled + 48_000) / 96_000) as u32;
    const_nearest_prime(rounded) + RING_SLACK
}

/// Diffuser length `M = floor(DIFF * fs / 48000 + 1/2)`, in exact integers.
const fn diffuser_length(lane: usize, sample_rate: u32) -> u32 {
    ((2 * DIFFUSER_BASE[lane] as u64 * sample_rate as u64 + 48_000) / 96_000) as u32
}

/// Largest pre-delay in samples: `floor(0.25 fs)`.
const fn max_pre_delay(sample_rate: u32) -> u32 {
    sample_rate / 4
}

/// Pre-delay ring words per channel: `floor(0.25 fs) + 1 + 128`.
const fn pre_delay_capacity(sample_rate: u32) -> u32 {
    max_pre_delay(sample_rate) + 1 + RING_SLACK
}

/// Words of the common state header (see `state.rs`).
const COMMON_HEADER_WORDS: usize = 324;
/// Words of one channel's state header (see `state.rs`).
const LANE_HEADER_WORDS: usize = 8;

/// The ring geometry of one sample rate, all of it fixed by the rate.
#[derive(Clone, Copy, Debug)]
struct Geometry {
    /// Ring words of each line: `D_i(2, fs) + 128`.
    line_len: [u32; LINES],
    /// First word of each line in the concatenated line store.
    line_offset: [usize; LINES],
    /// Words of the concatenated line store.
    line_words: usize,
    /// Ring words of each diffuser (its delay `M`).
    diffuser_len: [u32; DIFFUSERS],
    /// First word of each diffuser in the concatenated diffuser store.
    diffuser_offset: [usize; DIFFUSERS],
    /// Words of the concatenated diffuser store.
    diffuser_words: usize,
    /// Pre-delay ring words per channel.
    pre_len: u32,
    /// Largest pre-delay in samples.
    pre_max: u32,
    /// The saturation of the signal-history counter: the longest ring.
    history_cap: u32,
}

const fn geometry(sample_rate: u32) -> Geometry {
    let mut line_len = [0; LINES];
    let mut line_offset = [0; LINES];
    let mut line_words = 0;
    let mut cap = 0;
    let mut i = 0;
    while i < LINES {
        line_len[i] = const_line_capacity(i, sample_rate);
        line_offset[i] = line_words;
        line_words += line_len[i] as usize;
        if line_len[i] > cap {
            cap = line_len[i];
        }
        i += 1;
    }
    let mut diffuser_len = [0; DIFFUSERS];
    let mut diffuser_offset = [0; DIFFUSERS];
    let mut diffuser_words = 0;
    let mut l = 0;
    while l < DIFFUSERS {
        diffuser_len[l] = diffuser_length(l, sample_rate);
        diffuser_offset[l] = diffuser_words;
        diffuser_words += diffuser_len[l] as usize;
        if diffuser_len[l] > cap {
            cap = diffuser_len[l];
        }
        l += 1;
    }
    let pre_len = pre_delay_capacity(sample_rate);
    if pre_len > cap {
        cap = pre_len;
    }
    Geometry {
        line_len,
        line_offset,
        line_words,
        diffuser_len,
        diffuser_offset,
        diffuser_words,
        pre_len,
        pre_max: max_pre_delay(sample_rate),
        history_cap: cap,
    }
}

/// State payload sizes at `sample_rate`: the common section holds the thirty-two line rings, each
/// channel section its pre-delay ring and its four diffuser rings.
const fn state_sizes(sample_rate: u32) -> StatePayloadSizes {
    let g = geometry(sample_rate);
    let mut left = 0;
    let mut right = 0;
    let mut l = 0;
    while l < DIFFUSERS {
        if l < DIFFUSERS / 2 {
            left += g.diffuser_len[l];
        } else {
            right += g.diffuser_len[l];
        }
        l += 1;
    }
    // The contract requires equal channel sections; the shorter channel's diffusers leave
    // padding words at the end of its section, always `+0.0`.
    let diffusers = if left > right { left } else { right };
    let lane_bytes = (LANE_HEADER_WORDS as u32 + g.pre_len + diffusers) * 4;
    StatePayloadSizes {
        common_bytes: (COMMON_HEADER_WORDS as u32 + g.line_words as u32) * 4,
        left_bytes: lane_bytes,
        right_bytes: lane_bytes,
    }
}

const fn quality(sample_rate: u32) -> effect_contract::QualityDescriptor {
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: state_sizes(sample_rate),
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// This effect's tail, tail over every peak and exact-rest bound, the one place they are stated
/// (decision 15 D15-4(b), #2114 S3): [`tail_bound_at`] at the descriptor's longest decay.
fn tail_and_rest(sample_rate: u32, _quality: EffectQuality) -> NodeTailBound {
    match REVERB_PARAMETERS[DECAY].maximum {
        Some(decay_max_ms) => tail_bound_at(sample_rate, decay_max_ms),
        None => NodeTailBound::UNBOUNDED,
    }
}

/// Immutable descriptor of `miso.reverb`.
pub const REVERB_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.reverb"),
    display_name: "Stereo Reverb",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    lane_link: false,
    parameters: &REVERB_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &[],
};

/// Factory for `miso.reverb`. There is intentionally no homogeneous bank kernel.
#[derive(Clone, Copy, Debug, Default)]
pub struct ReverbFactory;

const fn spec_of(parameter: &ParameterDescriptor) -> ParameterSpec {
    let (minimum, maximum) = match (parameter.minimum, parameter.maximum) {
        (Some(minimum), Some(maximum)) => (minimum, maximum),
        _ => panic!("every reverb parameter is a bounded continuous domain"),
    };
    ParameterSpec::continuous(minimum, maximum, parameter.default_value)
}

/// Frozen parameter domains, in descriptor order.
const PARAMETER_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    spec_of(&REVERB_PARAMETERS[0]),
    spec_of(&REVERB_PARAMETERS[1]),
    spec_of(&REVERB_PARAMETERS[2]),
    spec_of(&REVERB_PARAMETERS[3]),
    spec_of(&REVERB_PARAMETERS[4]),
    spec_of(&REVERB_PARAMETERS[5]),
    spec_of(&REVERB_PARAMETERS[6]),
    spec_of(&REVERB_PARAMETERS[7]),
];

/// A sieve of primes up to a bound, built at prepare, read at accepted `size` events.
#[derive(Debug)]
struct Sieve {
    composite: Box<[bool]>,
}

impl Sieve {
    fn new(limit: usize) -> Self {
        let mut composite = vec![false; limit + 1].into_boxed_slice();
        composite[0] = true;
        if limit >= 1 {
            composite[1] = true;
        }
        let mut p = 2;
        while p * p <= limit {
            if !composite[p] {
                let mut m = p * p;
                while m <= limit {
                    composite[m] = true;
                    m += p;
                }
            }
            p += 1;
        }
        Self { composite }
    }

    fn is_prime(&self, n: usize) -> bool {
        self.composite.get(n).is_some_and(|composite| !composite)
    }

    /// The nearest prime to `n`, the smaller on a tie; `None` only outside the sieve. A bounded
    /// loop of at most `2 * PRIME_SEARCH + 2` lookups.
    fn nearest_prime(&self, n: usize) -> Option<u32> {
        for d in 0..=PRIME_SEARCH as usize {
            if d <= n && self.is_prime(n - d) {
                return u32::try_from(n - d).ok();
            }
            if self.is_prime(n + d) {
                return u32::try_from(n + d).ok();
            }
        }
        None
    }
}

/// `D_i(size, fs) = Prime(floor(BASE_i * size * fs / 48000 + 1/2))`, `size` widened to `f64`.
fn line_length(line: usize, size: f32, sample_rate: u32, sieve: &Sieve) -> Option<u32> {
    let scaled =
        (f64::from(BASE[line]) * f64::from(size) * f64::from(sample_rate) / 48_000.0 + 0.5).floor();
    if !(1.0..=1.0e9).contains(&scaled) {
        return None;
    }
    sieve.nearest_prime(scaled as usize)
}

/// All thirty-two line lengths at `size`.
fn line_lengths(size: f32, sample_rate: u32, sieve: &Sieve) -> Option<[u32; LINES]> {
    let mut lengths = [0; LINES];
    for (line, length) in lengths.iter_mut().enumerate() {
        *length = line_length(line, size, sample_rate, sieve)?;
    }
    Some(lengths)
}

/// The per-line gains `c_i = fl32(g_i / sqrt 32)`, `g_i = exp(-3 ln(10) D_i / (fs T))`, `g_i` and
/// the quotient in `f64`, rounded once (D2).
fn line_gains(lengths: &[u32; LINES], decay_ms: f32, sample_rate: u32) -> [f32; LINES] {
    let t = f64::from(decay_ms) / 1000.0;
    let mut gains = [0.0; LINES];
    for (gain, length) in gains.iter_mut().zip(lengths) {
        let g = math::exp(
            -3.0 * core::f64::consts::LN_10 * f64::from(*length) / (f64::from(sample_rate) * t),
        );
        *gain = (g / (LINES as f64).sqrt()) as f32;
    }
    gains
}

/// TPT one-pole coefficient `G / (1 + G)`, `G = tan(pi fc / fs)`.
fn tpt_coefficient(cutoff_hz: f64, sample_rate: u32) -> f32 {
    let big_g = math::tan(core::f64::consts::PI * cutoff_hz / f64::from(sample_rate));
    (big_g / (1.0 + big_g)) as f32
}

/// The damping low-pass coefficient `g1`, its cutoff capped at `min(19,845, fs / 4)`.
fn damping_coefficient(damping_hz: f32, sample_rate: u32) -> f32 {
    tpt_coefficient(
        f64::from(damping_hz).min(damping_cutoff_max_hz(sample_rate)),
        sample_rate,
    )
}

/// The low-cut high-pass coefficient `g2`.
fn low_cut_coefficient(low_cut_hz: f32, sample_rate: u32) -> f32 {
    tpt_coefficient(f64::from(low_cut_hz), sample_rate)
}

/// The diffuser coefficient `k = fl32(0.7 * diffusion)`.
fn diffusion_coefficient(diffusion: f32) -> f32 {
    (0.7 * f64::from(diffusion)) as f32
}

/// Pre-delay samples `P = floor(pre_delay_ms * fs / 1000 + 1/2)`.
fn pre_delay_samples(milliseconds: f32, sample_rate: u32) -> Option<u32> {
    let rounded = (f64::from(milliseconds) * f64::from(sample_rate) / 1000.0 + 0.5).floor();
    if !(0.0..=f64::from(max_pre_delay(sample_rate))).contains(&rounded) {
        return None;
    }
    Some(rounded as u32)
}

/// Everything a set of control values designs: the line taps, their gains, the filter and
/// diffuser coefficients and the pre-delay.
#[derive(Clone, Copy, Debug)]
struct Design {
    lengths: [u32; LINES],
    gains: [f32; LINES],
    damping: f32,
    low_cut: f32,
    diffusion: f32,
    pre_delay: u32,
}

impl Design {
    fn new(targets: &[f32; PARAMETER_COUNT], sample_rate: u32, sieve: &Sieve) -> Option<Self> {
        let lengths = line_lengths(targets[SIZE], sample_rate, sieve)?;
        Some(Self {
            lengths,
            gains: line_gains(&lengths, targets[DECAY], sample_rate),
            damping: damping_coefficient(targets[DAMPING], sample_rate),
            low_cut: low_cut_coefficient(targets[LOW_CUT], sample_rate),
            diffusion: diffusion_coefficient(targets[DIFFUSION]),
            pre_delay: pre_delay_samples(targets[PRE_DELAY], sample_rate)?,
        })
    }
}

/// The thirty-two per-line gain ramps: one shared length, because every event that moves one of
/// them (`decay_ms`, `size`) moves all of them. Same law as [`LinearRamp`] elementwise.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GainRamps {
    current: [f32; LINES],
    target: [f32; LINES],
    step: [f32; LINES],
    remaining: u32,
}

impl GainRamps {
    const fn fixed(gains: [f32; LINES]) -> Self {
        Self {
            current: gains,
            target: gains,
            step: [0.0; LINES],
            remaining: 0,
        }
    }

    /// Points every gain at its target over [`RAMP_SAMPLES`]: `step = (target - current) / 64`,
    /// the [`LinearRamp::set_target`] division. A retarget that moves no word settles at once.
    fn set_target(&mut self, target: [f32; LINES]) {
        self.target = target;
        if (0..LINES).all(|j| LinearRamp::stationary_at(self.current[j], target[j])) {
            self.current = target;
            self.step = [0.0; LINES];
            self.remaining = 0;
            return;
        }
        for ((step, target), current) in self.step.iter_mut().zip(target).zip(self.current) {
            *step = (target - current) / RAMP_SAMPLES as f32;
        }
        self.remaining = RAMP_SAMPLES;
    }

    fn snap(&mut self) {
        self.current = self.target;
        self.step = [0.0; LINES];
        self.remaining = 0;
    }
}

/// The five scalar coefficient ramps, in this order.
const RAMP_DAMPING: usize = 0;
const RAMP_LOW_CUT: usize = 1;
const RAMP_DIFFUSION: usize = 2;
const RAMP_WIDTH: usize = 3;
const RAMP_MIX: usize = 4;
const SCALAR_RAMPS: usize = 5;

/// Prepared `miso.reverb` state. The geometry and every allocation are fixed at prepare.
#[derive(Debug)]
pub struct PreparedReverb {
    bypass: bool,
    sample_rate: u32,
    automation_capacity: u32,
    state_sizes: StatePayloadSizes,
    geometry: Geometry,
    sieve: Sieve,
    /// Prepared default control values, for `FullToDefaults`.
    default_targets: [f32; PARAMETER_COUNT],
    /// Their design, computed once at prepare.
    default_design: Design,
    /// The line lengths at size 0.25 and 2.0, the domain a restored tap must lie in.
    min_lengths: [u32; LINES],
    max_lengths: [u32; LINES],

    /// Last accepted control value of every parameter.
    targets: [f32; PARAMETER_COUNT],
    /// Samples of signal history written since the last fault or reset, saturating at
    /// `geometry.history_cap`. A tap at delay `d` reads `+0.0` unless `d <= history`.
    history: u32,

    /// The thirty-two line rings, concatenated.
    lines: Box<[f32]>,
    line_cursor: [u32; LINES],
    line_active: [u32; LINES],
    line_transition: [u32; LINES],
    line_pending: [u32; LINES],
    /// Updates left in the running line crossfade; `0` when idle.
    line_fade: u32,
    /// `line_pending != line_active` on some line: a crossfade waits to start.
    line_queued: bool,

    /// The two pre-delay rings, left then right.
    pre: Box<[f32]>,
    pre_cursor: u32,
    pre_active: u32,
    pre_transition: u32,
    pre_pending: u32,
    pre_fade: u32,

    /// The eight diffuser rings, concatenated.
    diffusers: Box<[f32]>,
    diffuser_cursor: [u32; DIFFUSERS],
    /// Each diffuser's output of the previous sample (the pipeline registers).
    pipe: [f32; DIFFUSERS],

    /// Damping `g1`, low cut `g2`, diffusion `k`, width, mix.
    ramps: [LinearRamp; SCALAR_RAMPS],
    /// The per-line gains `c_i`, in line order.
    gains: GainRamps,
    /// Low-pass states `a_j`, in slot order.
    lp: [f32; LINES],
    /// High-pass states `b_j`, in slot order.
    hp: [f32; LINES],
}

impl NativeEffectFactory for ReverbFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &REVERB_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        size_of::<PreparedReverb>() as u64
    }

    fn prepared_undeclared_heap_bytes(&self, quality: &effect_contract::QualityDescriptor) -> u64 {
        // The prime sieve `prepare_reverb` keeps for its line lengths, one flag per sample up to
        // the longest line plus the search window; the payload does not carry it.
        let largest = (f64::from(BASE_MAX) * 2.0 * f64::from(quality.sample_rate) / 48_000.0 + 0.5)
            .floor() as u64;
        largest + 2 * u64::from(PRIME_SEARCH) + 1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let (processor, metadata) = prepare_reverb(request)?;
        Ok(PreparedEffect {
            processor: Box::new(processor),
            metadata,
        })
    }

    /// Issue #2199: `false` by design, as the prepared processor answers (its two channels'
    /// diffusers have different lengths), after the design `prepare` would refuse a request in.
    /// The design holds the prime sieve, not the instance's rings.
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        design_reverb(request).map(|_| false)
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<effect_contract::PreparedEffectBank>, EffectPrepareError> {
        request.validate_shape()?;
        for member in request.requests.iter().copied() {
            let _ = validate_inputs(member)?;
        }
        // The reverb's SIMD runs across its own thirty-two lines, inside one instance; there is no
        // cross-instance bank kernel. Every validated request stays an ordered scalar member, and
        // a padded request is declined like every other one (decision 12, "Eligibility").
        Ok(None)
    }
}

const fn resource_error() -> EffectPrepareError {
    EffectPrepareError {
        code: "effect.resource.limit",
    }
}

const fn initial_error() -> EffectPrepareError {
    EffectPrepareError {
        code: "effect.parameter.initial",
    }
}

type ValidatedInputs = (PreparedEffectMetadata, [f32; PARAMETER_COUNT]);

fn validate_inputs(
    request: PrepareEffectRequest<'_>,
) -> Result<ValidatedInputs, EffectPrepareError> {
    let metadata = expected_prepared_metadata(&REVERB_DESCRIPTOR, request)?;
    if metadata.state_sizes != state_sizes(metadata.sample_rate) || metadata.scratch_bytes != 0 {
        return Err(resource_error());
    }
    let values = request.initial_values;
    if values.len() != PARAMETER_COUNT {
        return Err(initial_error());
    }
    let mut targets = [0.0; PARAMETER_COUNT];
    for (index, value) in values.iter().enumerate() {
        if value.parameter_index != index as u32
            || value.channel != ParameterChannel::Both
            || !parameter_value_valid(&PARAMETER_SPECS[index], value.value)
            || effect_runtime::params::is_negative_zero(value.value)
        {
            return Err(initial_error());
        }
        targets[index] = normalize_zero(value.value);
    }
    Ok((metadata, targets))
}

/// Everything [`prepare_reverb`] designs from a request before it allocates the instance's rings:
/// every way preparation can refuse a request is here (issue #2199).
struct ReverbDesign {
    metadata: PreparedEffectMetadata,
    targets: [f32; PARAMETER_COUNT],
    geometry: Geometry,
    sieve: Sieve,
    min_lengths: [u32; LINES],
    max_lengths: [u32; LINES],
    design: Design,
}

fn design_reverb(request: PrepareEffectRequest<'_>) -> Result<ReverbDesign, EffectPrepareError> {
    let (metadata, targets) = validate_inputs(request)?;
    let sample_rate = metadata.sample_rate;
    let geometry = geometry(sample_rate);
    let largest =
        (f64::from(BASE_MAX) * 2.0 * f64::from(sample_rate) / 48_000.0 + 0.5).floor() as usize;
    let sieve = Sieve::new(largest + 2 * PRIME_SEARCH as usize);
    let min_lengths = line_lengths(0.25, sample_rate, &sieve).ok_or(resource_error())?;
    let max_lengths = line_lengths(2.0, sample_rate, &sieve).ok_or(resource_error())?;
    // The ring capacities the descriptor declared (`const`) must be the ones the runtime law
    // gives at size 2; otherwise a size-2 tap could read past its ring.
    if max_lengths
        .iter()
        .zip(geometry.line_len)
        .any(|(length, capacity)| length + RING_SLACK != capacity)
    {
        return Err(resource_error());
    }
    let design = Design::new(&targets, sample_rate, &sieve).ok_or(initial_error())?;
    Ok(ReverbDesign {
        metadata,
        targets,
        geometry,
        sieve,
        min_lengths,
        max_lengths,
        design,
    })
}

fn prepare_reverb(
    request: PrepareEffectRequest<'_>,
) -> Result<(PreparedReverb, PreparedEffectMetadata), EffectPrepareError> {
    let ReverbDesign {
        metadata,
        targets,
        geometry,
        sieve,
        min_lengths,
        max_lengths,
        design,
    } = design_reverb(request)?;
    let sample_rate = metadata.sample_rate;
    let mut processor = PreparedReverb {
        bypass: metadata.bypass,
        sample_rate,
        automation_capacity: metadata.automation_capacity,
        state_sizes: metadata.state_sizes,
        geometry,
        sieve,
        default_targets: targets,
        default_design: design,
        min_lengths,
        max_lengths,
        targets,
        history: 0,
        lines: vec![0.0; geometry.line_words].into_boxed_slice(),
        line_cursor: [0; LINES],
        line_active: design.lengths,
        line_transition: design.lengths,
        line_pending: design.lengths,
        line_fade: 0,
        line_queued: false,
        pre: vec![0.0; 2 * geometry.pre_len as usize].into_boxed_slice(),
        pre_cursor: 0,
        pre_active: design.pre_delay,
        pre_transition: design.pre_delay,
        pre_pending: design.pre_delay,
        pre_fade: 0,
        diffusers: vec![0.0; geometry.diffuser_words].into_boxed_slice(),
        diffuser_cursor: [0; DIFFUSERS],
        pipe: [0.0; DIFFUSERS],
        ramps: [LinearRamp::fixed(0.0); SCALAR_RAMPS],
        gains: GainRamps::fixed(design.gains),
        lp: [0.0; LINES],
        hp: [0.0; LINES],
    };
    processor.full_reset();
    Ok((processor, metadata))
}

impl PreparedReverb {
    /// Restores the prepared defaults, keeping every allocation (rings are not cleared: the
    /// history counter makes their old words unreadable).
    fn full_reset(&mut self) {
        let design = self.default_design;
        self.targets = self.default_targets;
        self.ramps = [
            LinearRamp::fixed(design.damping),
            LinearRamp::fixed(design.low_cut),
            LinearRamp::fixed(design.diffusion),
            LinearRamp::fixed(self.default_targets[WIDTH]),
            LinearRamp::fixed(self.default_targets[MIX]),
        ];
        self.gains = GainRamps::fixed(design.gains);
        self.line_pending = design.lengths;
        self.pre_pending = design.pre_delay;
        self.drop_history();
    }

    /// Keeps the parameters (every ramp snapped to its target), drops all signal history.
    fn discontinuity_reset(&mut self) {
        for ramp in &mut self.ramps {
            ramp.snap();
        }
        self.gains.snap();
        self.drop_history();
    }

    /// Settles every tap on its pending value, rewinds the cursors and empties the history.
    fn drop_history(&mut self) {
        self.line_active = self.line_pending;
        self.line_transition = self.line_pending;
        self.line_fade = 0;
        self.line_queued = false;
        self.pre_active = self.pre_pending;
        self.pre_transition = self.pre_pending;
        self.pre_fade = 0;
        self.line_cursor = [0; LINES];
        self.pre_cursor = 0;
        self.diffuser_cursor = [0; DIFFUSERS];
        self.clear_signal_state();
    }

    /// Drops all signal history in bounded work: the history counter and the recursive words.
    fn clear_signal_state(&mut self) {
        self.history = 0;
        self.lp = [0.0; LINES];
        self.hp = [0.0; LINES];
        self.pipe = [0.0; DIFFUSERS];
    }

    /// Applies this block's automation points (all at block start, one per parameter at most).
    fn apply_automation(&mut self, spans: &[PreparedAutomationSpan], first_sample: u64) -> u64 {
        let mut invalid = 0_u64;
        let mut pending: [Option<f32>; PARAMETER_COUNT] = [None; PARAMETER_COUNT];
        let mut last: Option<usize> = None;
        for (span_index, span) in spans.iter().enumerate() {
            let index = span.parameter_index as usize;
            let accepted = index < PARAMETER_COUNT
                && span.channel == ParameterChannel::Both
                && span_index < self.automation_capacity as usize
                && span.sample == first_sample
                && parameter_value_valid(&PARAMETER_SPECS[index], span.value)
                && last.is_none_or(|previous| index > previous);
            if accepted {
                last = Some(index);
                pending[index] = Some(normalize_zero(span.value));
            } else {
                invalid = invalid.saturating_add(1);
            }
        }
        let sample_rate = self.sample_rate;
        if let Some(value) = pending[PRE_DELAY]
            && let Some(samples) = pre_delay_samples(value, sample_rate)
        {
            self.targets[PRE_DELAY] = value;
            self.pre_pending = samples;
        }
        let mut redesign_gains = false;
        if let Some(value) = pending[SIZE]
            && let Some(lengths) = line_lengths(value, sample_rate, &self.sieve)
        {
            self.targets[SIZE] = value;
            self.line_pending = lengths;
            self.line_queued = self.line_pending != self.line_active;
            redesign_gains = true;
        }
        if let Some(value) = pending[DECAY] {
            self.targets[DECAY] = value;
            redesign_gains = true;
        }
        if redesign_gains {
            let gains = line_gains(&self.line_pending, self.targets[DECAY], sample_rate);
            self.gains.set_target(gains);
        }
        for (parameter, ramp) in [
            (DAMPING, RAMP_DAMPING),
            (LOW_CUT, RAMP_LOW_CUT),
            (DIFFUSION, RAMP_DIFFUSION),
            (WIDTH, RAMP_WIDTH),
            (MIX, RAMP_MIX),
        ] {
            if let Some(value) = pending[parameter] {
                self.targets[parameter] = value;
                let target = match parameter {
                    DAMPING => damping_coefficient(value, sample_rate),
                    LOW_CUT => low_cut_coefficient(value, sample_rate),
                    DIFFUSION => diffusion_coefficient(value),
                    _ => value,
                };
                self.ramps[ramp].set_target(target, RAMP_SAMPLES);
            }
        }
        invalid
    }

    /// One block at lane type `L`: automation, the kernel, the finiteness check.
    fn process_at<L: Lane>(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport {
            invalid_spans: self.apply_automation(block.automation, block.first_sample),
            ..ProcessReport::default()
        };
        let EffectProcessBlock {
            left: io_left,
            right: io_right,
            ..
        } = block;
        let windows = self.write_windows();
        if self.history >= self.geometry.history_cap {
            self.render::<L, true>(io_left, io_right);
        } else {
            self.render::<L, false>(io_left, io_right);
        }
        if !self.block_is_finite::<L>(io_left, io_right, &windows) {
            io_left.fill(0.0);
            io_right.fill(0.0);
            self.clear_signal_state();
            report.nonfinite_left_blocks = 1;
            report.nonfinite_right_blocks = 1;
        }
        report
    }
}

/// The ring cursors at block start: where this block's writes begin.
#[derive(Clone, Copy, Debug)]
struct WriteWindows {
    lines: [u32; LINES],
    pre: u32,
    diffusers: [u32; DIFFUSERS],
}

/// `true` when every word is finite and below the block limit: the vector body at `L`, the tail
/// at one lane.
fn finite_words<L: Lane>(words: &[f32]) -> bool {
    use effect_runtime::bank::check_block;
    let split = words.len() - words.len() % L::WIDTH;
    check_block::<L>(&words[..split]) && check_block::<f32>(&words[split..])
}

/// `true` when the `frames` words written from `start` on, around a ring, are finite.
fn ring_window_finite<L: Lane>(ring: &[f32], start: u32, frames: usize) -> bool {
    let len = ring.len();
    let written = frames.min(len);
    let start = start as usize;
    let first = written.min(len - start);
    finite_words::<L>(&ring[start..start + first]) && finite_words::<L>(&ring[..written - first])
}

impl PreparedReverb {
    fn write_windows(&self) -> WriteWindows {
        WriteWindows {
            lines: self.line_cursor,
            pre: self.pre_cursor,
            diffusers: self.diffuser_cursor,
        }
    }

    /// The once-per-block check: outputs, every ring's write window, the filter states and the
    /// pipeline registers. The gains and coefficients are designed finite.
    fn block_is_finite<L: Lane>(&self, left: &[f32], right: &[f32], w: &WriteWindows) -> bool {
        let g = &self.geometry;
        let frames = left.len();
        if !(finite_words::<L>(left)
            && finite_words::<L>(right)
            && finite_words::<L>(&self.lp)
            && finite_words::<L>(&self.hp)
            && finite_words::<L>(&self.pipe))
        {
            return false;
        }
        for line in 0..LINES {
            let ring = &self.lines[g.line_offset[line]..][..g.line_len[line] as usize];
            if !ring_window_finite::<L>(ring, w.lines[line], frames) {
                return false;
            }
        }
        let pre_len = g.pre_len as usize;
        for channel in 0..2 {
            let ring = &self.pre[channel * pre_len..][..pre_len];
            if !ring_window_finite::<L>(ring, w.pre, frames) {
                return false;
            }
        }
        for lane in 0..DIFFUSERS {
            let ring = &self.diffusers[g.diffuser_offset[lane]..][..g.diffuser_len[lane] as usize];
            if !ring_window_finite::<L>(ring, w.diffusers[lane], frames) {
                return false;
            }
        }
        true
    }
}

impl PreparedNativeEffect for PreparedReverb {
    /// The three ring stores and the prime sieve; every other field is inline.
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::slice(&self.lines)
            + effect_contract::retained::slice(&self.pre)
            + effect_contract::retained::slice(&self.diffusers)
            + effect_contract::retained::slice(&self.sieve.composite)
    }

    /// `false` by design: the two channels' diffusers have different lengths (`DIFF_L` and
    /// `DIFF_R`), so the designed per-channel words never compare equal.
    fn channel_symmetry(&self) -> bool {
        false
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => self.full_reset(),
            ResetKind::DiscontinuityKeepParameters => self.discontinuity_reset(),
        }
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        self.process_at::<lane::Native>(block)
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot(output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.restore(state_layout_version, input)
    }
}
