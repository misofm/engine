//! The frozen cross-target corpus of gate G5.
//!
//! One definition, compiled twice: natively by `miso-engine-wasm-gates`, and to
//! `wasm32-unknown-unknown` where wasmtime executes it. Master plan #83 D5 claims a rendered block
//! is bit-identical across `Scalar`/`Simd4`/`Simd8` **and** across `x86_64`/`aarch64`/`wasm32`.
//! Gate G2 proves the width half on one target; this module proves the target half, by digesting
//! the same computation on both sides and comparing digests to one set of pins.
//!
//! Three properties make that comparison mean something, and each is a rule this file keeps:
//!
//! 1. **The byte stream is width independent.** A lane case is [`LANES`] independent single-lane
//!    signals of [`FRAMES`] frames. At width `W` the corpus is processed in `LANES / W` groups of
//!    an AoSoA block and read back *lane-major* before hashing, so the digest describes the
//!    arithmetic and not the layout. `W = 1`, `W = 4` and `W = 8` must produce the same 32 bytes,
//!    on every target — which is why every case is digested at all three widths on both legs.
//! 2. **No NaN reaches a digest.** D5 excludes NaN payloads because wasm canonicalises them. Every
//!    case is built so its outputs are finite, and the host crate asserts that rather than assuming
//!    it.
//! 3. **The corpus is frozen.** [`LANE_DIGESTS`] pins this case list, these seeds and this
//!    operation order. Changing one of them is a re-pin, permitted only from the scalar `Lane`
//!    oracle and only with the change stated in the commit message (master plan §8).
//!
//! Two halves of the corpus are not redefined here. Cases past [`LANE_CASE_COUNT`] delegate to
//! [`miso_engine_math::corpus`] (gate M3) and [`miso_engine_effect_runtime::corpus`] (gate D1) and
//! are compared against those crates' own pins, so the wasm run replays exactly what M3 and D1
//! pinned natively rather than a transcription of them. The `effect-runtime` cases are lane
//! generic — the dB curve a compressor rides, the followers that ride it — so they too are
//! digested at every width.

use miso_engine_delay::corpus as delay_corpus;
use miso_engine_effect_runtime::corpus as runtime_corpus;
use miso_engine_lane::Lane;
use miso_engine_lane::kernels::{
    OnePoleCoef, OnePoleState, RampSegment, SvfCoef, SvfCoefStep, SvfState, gain_block,
    gain_mix_block, one_pole_block, ramp_block, sum_into_block, sum2_block, svf_block,
    svf_block_ramped,
};
use miso_engine_math::corpus as math_corpus;
use miso_engine_math::{exp2_lane, log2_lane};
use miso_engine_multiband_compressor::corpus as multiband_corpus;
use miso_engine_transient_shaper::corpus as transient_shaper_corpus;
use sha2::{Digest, Sha256};

/// Independent single-lane signals in every lane case; a multiple of the widest backend.
pub const LANES: usize = 8;

/// Frames per signal: long enough for a recursive filter to settle and for the 512-frame ramp to
/// finish and snap, short enough that the wasm leg costs well under a second.
pub const FRAMES: usize = 1024;

/// The lane widths every case is digested at. `Simd4` and `Simd8` are implemented on every target
/// (`wide` lowers `f32x8` to two four-lane values where AVX2 is absent), so all three run on both
/// legs and a width difference cannot hide behind a target difference.
pub const WIDTHS: usize = 3;

/// Cases built from the `Lane` trait and the block kernels.
pub const LANE_CASE_COUNT: usize = KERNELS.len() * SIGNALS.len() + ELEMENTWISE.len();

/// Cases delegated to [`miso_engine_math::corpus`] (gate M3, replayed under wasm).
pub const MATH_CASE_COUNT: usize = math_corpus::CASE_COUNT;

/// Cases delegated to [`miso_engine_effect_runtime::corpus`] (gate D1, replayed under wasm).
///
/// These are lane-generic — the dB curve a compressor rides, the followers that ride it, and the
/// level conversions between them — so unlike the math cases they are digested at every width.
pub const RUNTIME_CASE_COUNT: usize = runtime_corpus::CASE_COUNT;

/// Cases delegated to [`miso_engine_transient_shaper::corpus`], replayed under wasm.
///
/// The first effect crate to join this gate. Its cases are whole rendered blocks of the production
/// effect — followers, the `log2`/`exp2` dB chain, the D11 ramp prefix and the identity selects —
/// so they exercise the composition the lane and runtime cases only cover a piece at a time. They
/// are lane generic, and their pins live in that crate.
pub const TRANSIENT_SHAPER_CASE_COUNT: usize = transient_shaper_corpus::CASE_COUNT;

/// Cases delegated to [`miso_engine_delay::corpus`] (issue #93's G5 row, replayed under wasm).
///
/// The delay is a `W = 1` effect -- a gathered two-second ring has no `W4`/`W8` kernel -- so like
/// the math cases these are run once rather than at every width. What they carry across the target
/// boundary is the six `Lane::fma` sites of its kernel, which on wasm are the software FMA.
pub const DELAY_CASE_COUNT: usize = delay_corpus::CASE_COUNT;

/// Cases delegated to [`miso_engine_multiband_compressor::corpus`] (audit #94 E5), replayed under
/// wasm.
///
/// Lane generic, and their pins live in that crate: the Linkwitz-Riley split with its fused
/// all-pass tap and the D7 flush of its four filter words, the band's dB chain through
/// `log2_lane`/`exp2_lane`, the branching gain smoother and the stereo detector link.
pub const MULTIBAND_CASE_COUNT: usize = multiband_corpus::CASE_COUNT;

/// Total cases the guest exports.
pub const CASE_COUNT: usize = LANE_CASE_COUNT
    + MATH_CASE_COUNT
    + RUNTIME_CASE_COUNT
    + TRANSIENT_SHAPER_CASE_COUNT
    + DELAY_CASE_COUNT
    + MULTIBAND_CASE_COUNT;

/// The block kernels the corpus drives, in pin order.
const KERNELS: [Kernel; 12] = [
    Kernel::SvfLow,
    Kernel::SvfHigh,
    Kernel::SvfBand,
    Kernel::SvfBell,
    Kernel::SvfRamped,
    Kernel::SvfRampedIdle,
    Kernel::OnePole,
    Kernel::Gain,
    Kernel::GainMix,
    Kernel::Ramp,
    Kernel::Sum2,
    Kernel::SumInto,
];

/// The signals each kernel is driven with, in pin order.
const SIGNALS: [Signal; 4] = [
    Signal::Noise,
    Signal::Impulse,
    Signal::Dc,
    Signal::Subnormal,
];

/// The element-wise `Lane` cases, in pin order.
///
/// [`Elementwise::Fma`] is the one that carries this gate: on wasm `Lane::fma` is the exact
/// software FMA of master plan §3.5, whose `v128` body is the only part of the lane crate no
/// native gate can execute. Its inputs are built so a fused and an unfused evaluation disagree, so
/// a wasm build that quietly stopped fusing moves this digest.
const ELEMENTWISE: [Elementwise; 3] = [Elementwise::Fma, Elementwise::Exp2, Elementwise::Log2];

/// One block kernel of `miso_engine_lane::kernels`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kernel {
    /// `svf_block` with a low-pass output mix.
    SvfLow,
    /// `svf_block` with a high-pass output mix.
    SvfHigh,
    /// `svf_block` with a band-pass output mix.
    SvfBand,
    /// `svf_block` with a `+6 dB` bell output mix.
    SvfBell,
    /// `svf_block_ramped` with a non-zero coefficient ramp.
    SvfRamped,
    /// `svf_block_ramped` with `ramp_frames = 0`, which must equal `SvfLow` exactly.
    SvfRampedIdle,
    /// `one_pole_block`.
    OnePole,
    /// `gain_block`.
    Gain,
    /// `gain_mix_block`.
    GainMix,
    /// `ramp_block` across the ramp end, so the D11 snap is inside the case.
    Ramp,
    /// `sum2_block`.
    Sum2,
    /// `sum_into_block`.
    SumInto,
}

impl Kernel {
    /// Name used in reports and in the case list.
    const fn name(self) -> &'static str {
        match self {
            Self::SvfLow => "svf_block/low",
            Self::SvfHigh => "svf_block/high",
            Self::SvfBand => "svf_block/band",
            Self::SvfBell => "svf_block/bell",
            Self::SvfRamped => "svf_block_ramped",
            Self::SvfRampedIdle => "svf_block_ramped/idle",
            Self::OnePole => "one_pole_block",
            Self::Gain => "gain_block",
            Self::GainMix => "gain_mix_block",
            Self::Ramp => "ramp_block",
            Self::Sum2 => "sum2_block",
            Self::SumInto => "sum_into_block",
        }
    }

    /// `(c1, a2, a3, m0, m1, m2)`: the master plan §4.2 `c1` storage of a TPT state-variable
    /// filter at 1 kHz, `Q = 0.707`, 48 kHz, designed in `f64` and rounded once to `f32`.
    const fn svf_coefficients(self) -> [f32; 6] {
        // g = tan(pi * 1000 / 48000) = 0.065_543_46, k = 1 / 0.707 = 1.414_427_2,
        // t = g * (g + k) = 0.096_985_49, c1 = t / (1 + t) = 0.088_412_71,
        // a1 = 1 - c1 = 0.911_587_3, a2 = g * a1 = 0.059_749_45, a3 = g * a2 = 0.003_916_28.
        let (c1, a2, a3) = (0.088_412_71_f32, 0.059_749_45_f32, 0.003_916_28_f32);
        match self {
            Self::SvfHigh => [c1, a2, a3, 1.0, -1.414_427_2, -1.0],
            Self::SvfBand => [c1, a2, a3, 0.0, 1.0, 0.0],
            // Bell at +6 dB: A = 10^(6/40) = 1.412_537_5, m1 = k * (A^2 - 1), k = 1 / (Q * A).
            Self::SvfBell => [c1, a2, a3, 1.0, 1.001_204_5, 0.0],
            _ => [c1, a2, a3, 0.0, 0.0, 1.0],
        }
    }
}

/// The signal a kernel case is fed, and the state it starts from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Signal {
    /// Seeded pseudo-random noise in `[-1, 1)`.
    Noise,
    /// A unit impulse in the first frame, silence after it.
    Impulse,
    /// Constant `0.5`.
    Dc,
    /// Subnormal-magnitude noise, with the recurrence seeded subnormal too, so the D7 flush has to
    /// remove it identically on every target.
    Subnormal,
}

impl Signal {
    /// Name used in reports.
    const fn name(self) -> &'static str {
        match self {
            Self::Noise => "noise",
            Self::Impulse => "impulse",
            Self::Dc => "dc",
            Self::Subnormal => "subnormal",
        }
    }

    /// The value every recurrence word starts at for this signal.
    const fn state_seed(self) -> f32 {
        match self {
            Self::Subnormal => 1.0e-40,
            _ => 0.0,
        }
    }

    /// Fills one lane's `FRAMES` samples.
    fn fill(self, lane: &mut [f32], seed: u64) {
        let mut random = Xorshift64Star::new(seed);
        for (frame, sample) in lane.iter_mut().enumerate() {
            *sample = match self {
                Self::Noise => f32::from((random.next_u32() >> 16) as u16) * (2.0 / 65_536.0) - 1.0,
                Self::Impulse => {
                    if frame == 0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                Self::Dc => 0.5,
                // Never zero, so the flush has something to remove in every frame.
                Self::Subnormal => f32::from_bits((random.next_u32() & 0x007F_FFFF) | 1),
            };
        }
    }
}

/// An element-wise `Lane` or lane-`math` operation applied to a corpus of values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Elementwise {
    /// `Lane::fma`: hardware `vfmadd`/`fmla` natively, the exact software FMA on wasm.
    Fma,
    /// `miso_engine_math::exp2_lane`.
    Exp2,
    /// `miso_engine_math::log2_lane`.
    Log2,
}

impl Elementwise {
    /// Name used in reports.
    const fn name(self) -> &'static str {
        match self {
            Self::Fma => "lane_fma",
            Self::Exp2 => "exp2_lane",
            Self::Log2 => "log2_lane",
        }
    }
}

/// Which computation a case index selects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Case {
    /// A block kernel driven with one signal.
    Kernel(Kernel, Signal),
    /// An element-wise lane operation.
    Elementwise(Elementwise),
    /// One case of the `miso-engine-math` M3 corpus.
    Math(usize),
    /// One case of the `miso-engine-effect-runtime` D1 corpus.
    Runtime(usize),
    /// One case of the `miso-engine-transient-shaper` cross-target corpus.
    TransientShaper(usize),
    /// One case of the `miso-engine-delay` G5 corpus.
    Delay(usize),
    /// One case of the `miso-engine-multiband-compressor` cross-target corpus.
    Multiband(usize),
}

/// Decodes a case index. This order is part of the pin.
///
/// # Panics
///
/// Panics if `index >= CASE_COUNT`.
fn case_of(index: usize) -> Case {
    assert!(index < CASE_COUNT, "corpus case index out of range");
    let kernel_cases = KERNELS.len() * SIGNALS.len();
    if index < kernel_cases {
        return Case::Kernel(
            KERNELS[index / SIGNALS.len()],
            SIGNALS[index % SIGNALS.len()],
        );
    }
    let index = index - kernel_cases;
    if index < ELEMENTWISE.len() {
        return Case::Elementwise(ELEMENTWISE[index]);
    }
    let index = index - ELEMENTWISE.len();
    if index < MATH_CASE_COUNT {
        return Case::Math(index);
    }
    let index = index - MATH_CASE_COUNT;
    if index < RUNTIME_CASE_COUNT {
        return Case::Runtime(index);
    }
    let index = index - RUNTIME_CASE_COUNT;
    if index < TRANSIENT_SHAPER_CASE_COUNT {
        return Case::TransientShaper(index);
    }
    let index = index - TRANSIENT_SHAPER_CASE_COUNT;
    if index < DELAY_CASE_COUNT {
        return Case::Delay(index);
    }
    Case::Multiband(index - DELAY_CASE_COUNT)
}

/// `true` when the case has a lane instantiation, so its digest must be identical at all three
/// widths and is compared at each of them.
///
/// The math and delay cases are excluded: the math functions are scalar `f64`/`f32` with no lane
/// instantiation, and the delay is a `W = 1` effect, so both are run once and their digests cannot
/// depend on a width.
///
/// # Panics
///
/// Panics if `index >= CASE_COUNT`.
#[must_use]
pub fn is_width_dependent(index: usize) -> bool {
    !matches!(case_of(index), Case::Math(_) | Case::Delay(_))
}

/// `true` when the case is one of this crate's own kernel or element-wise cases, whose per-lane
/// `f32` results [`lane_case_values`] can return.
///
/// The delegated `math` and `effect-runtime` cases produce result *words* through their own
/// crates' corpus APIs, and the assertions those crates make about their own corpora belong there.
///
/// # Panics
///
/// Panics if `index >= CASE_COUNT`.
#[must_use]
pub fn has_lane_values(index: usize) -> bool {
    matches!(case_of(index), Case::Kernel(..) | Case::Elementwise(_))
}

/// Human-readable name of a case, used in the failure reports of both legs.
///
/// # Panics
///
/// Panics if `index >= CASE_COUNT`.
#[must_use]
pub fn case_name(index: usize) -> String {
    match case_of(index) {
        Case::Kernel(kernel, signal) => format!("{}/{}", kernel.name(), signal.name()),
        Case::Elementwise(operation) => operation.name().to_string(),
        Case::Math(case) => format!("math/{}", math_corpus::CASE_NAMES[case]),
        Case::Runtime(case) => format!("runtime/{}", runtime_corpus::CASE_NAMES[case]),
        Case::TransientShaper(case) => transient_shaper_corpus::CASE_NAMES[case].to_string(),
        Case::Delay(case) => format!("delay/{}", delay_corpus::CASE_NAMES[case]),
        Case::Multiband(case) => format!("multiband/{}", multiband_corpus::CASE_NAMES[case]),
    }
}

/// Name of a width index, as the widths appear in [`digest_case`].
///
/// # Panics
///
/// Panics if `width >= WIDTHS`.
#[must_use]
pub fn width_name(width: usize) -> &'static str {
    match width {
        0 => "scalar",
        1 => "simd4",
        2 => "simd8",
        _ => panic!("width index out of range"),
    }
}

/// The pinned digest of a case.
///
/// This crate's own cases are pinned in [`LANE_DIGESTS`]; the delegated cases return the pins gates
/// M3 and D1 wrote in `miso-engine-math` and `miso-engine-effect-runtime`. Keeping them there is
/// the point: this crate replays those gates on a second target, and a second copy of their pins
/// could drift away from the gates it is meant to replay.
///
/// # Panics
///
/// Panics if `index >= CASE_COUNT`.
#[must_use]
pub fn expected_digest(index: usize) -> [u8; 32] {
    match case_of(index) {
        Case::Kernel(..) | Case::Elementwise(_) => LANE_DIGESTS[index],
        Case::Math(case) => math_corpus::M3_DIGESTS[case],
        Case::Runtime(case) => runtime_corpus::D1_DIGESTS[case],
        Case::TransientShaper(case) => transient_shaper_corpus::CROSS_TARGET_DIGESTS[case],
        Case::Delay(case) => delay_corpus::G5_DIGESTS[case],
        Case::Multiband(case) => multiband_corpus::DIGESTS[case],
    }
}

/// Digests one case at one width.
///
/// `width` selects `f32` (0), `Simd4` (1) or `Simd8` (2). A math case ignores it.
///
/// # Panics
///
/// Panics if `index >= CASE_COUNT` or `width >= WIDTHS`.
#[must_use]
pub fn digest_case(index: usize, width: usize) -> [u8; 32] {
    assert!(width < WIDTHS, "width index out of range");
    match case_of(index) {
        Case::Kernel(..) | Case::Elementwise(_) => digest_lanes(&lane_values(index, width, true)),
        Case::Math(case) => digest_math(case),
        Case::Runtime(case) => match width {
            0 => digest_runtime::<f32>(case),
            1 => digest_runtime::<miso_engine_lane::Simd4>(case),
            _ => digest_runtime::<miso_engine_lane::Simd8>(case),
        },
        Case::TransientShaper(case) => digest_transient_shaper(case, width),
        Case::Delay(case) => digest_delay(case),
        Case::Multiband(case) => match width {
            0 => digest_multiband::<f32>(case),
            1 => digest_multiband::<miso_engine_lane::Simd4>(case),
            _ => digest_multiband::<miso_engine_lane::Simd8>(case),
        },
    }
}

/// The per-lane results of a lane case, lane-major, for the assertions the gate makes about the
/// corpus itself (finiteness, distinctness) rather than about a digest.
///
/// # Panics
///
/// Panics if `index` is not a lane case, or if `width >= WIDTHS`.
#[must_use]
pub fn lane_case_values(index: usize, width: usize) -> Vec<f32> {
    lane_values(index, width, true)
        .into_iter()
        .flatten()
        .collect()
}

/// The `lane_fma` case evaluated with a multiply and an add instead of `Lane::fma`.
///
/// Gate G5 asserts this digest differs from the pinned one at every width. Without that assertion
/// the `lane_fma` case would only prove that both legs computed *something* identically; with it,
/// the case is known to separate a fused evaluation from an unfused one, which is the difference a
/// wasm build that stopped using the software FMA would show.
///
/// # Panics
///
/// Panics if `width >= WIDTHS`.
#[must_use]
pub fn unfused_fma_digest(width: usize) -> [u8; 32] {
    assert!(width < WIDTHS, "width index out of range");
    let lanes = match width {
        0 => elementwise_values::<f32>(Elementwise::Fma, false),
        1 => elementwise_values::<miso_engine_lane::Simd4>(Elementwise::Fma, false),
        _ => elementwise_values::<miso_engine_lane::Simd8>(Elementwise::Fma, false),
    };
    digest_lanes(&lanes)
}

/// Index of the `lane_fma` case, which [`unfused_fma_digest`] is the counterpart of.
#[must_use]
pub const fn fma_case() -> usize {
    KERNELS.len() * SIGNALS.len()
}

/// Runs one lane case at one width.
fn lane_values(index: usize, width: usize, fused: bool) -> [[f32; FRAMES]; LANES] {
    match case_of(index) {
        Case::Kernel(kernel, signal) => match width {
            0 => kernel_values::<f32>(kernel, signal),
            1 => kernel_values::<miso_engine_lane::Simd4>(kernel, signal),
            _ => kernel_values::<miso_engine_lane::Simd8>(kernel, signal),
        },
        Case::Elementwise(operation) => match width {
            0 => elementwise_values::<f32>(operation, fused),
            1 => elementwise_values::<miso_engine_lane::Simd4>(operation, fused),
            _ => elementwise_values::<miso_engine_lane::Simd8>(operation, fused),
        },
        Case::Math(_)
        | Case::Runtime(_)
        | Case::TransientShaper(_)
        | Case::Delay(_)
        | Case::Multiband(_) => {
            panic!("case {index} is not a lane-kernel case")
        }
    }
}

/// Xorshift64\* (Vigna 2016): a seeded, portable bit source, so the corpus is the same on every
/// host and every run and never touches a system random number generator.
struct Xorshift64Star {
    /// Generator state; never zero.
    state: u64,
}

impl Xorshift64Star {
    /// Starts the generator from `seed` (zero is replaced, since it is the fixed point).
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    /// Next 64 bits.
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Next 32 bits.
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
}

/// The seed of one lane, so the eight signals of a case are genuinely different from each other
/// and a cross-lane leak cannot pass as agreement.
fn lane_seed(lane: usize) -> u64 {
    0xA5A5_5A5A_1234_0001 ^ (lane as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

/// Hashes lane-major result bits: lane 0 frame 0..FRAMES, lane 1 frame 0..FRAMES, and so on.
///
/// This is the step that makes the digest width independent: whatever AoSoA grouping produced the
/// values, they are hashed in the same order.
fn digest_lanes(lanes: &[[f32; FRAMES]; LANES]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for lane in lanes {
        for sample in lane {
            hasher.update(sample.to_bits().to_le_bytes());
        }
    }
    hasher.finalize().into()
}

/// Runs one kernel over the whole corpus at width `L::WIDTH` and returns the per-lane results.
fn kernel_values<L: Lane>(kernel: Kernel, signal: Signal) -> [[f32; FRAMES]; LANES] {
    let mut lanes = [[0.0_f32; FRAMES]; LANES];
    for (index, lane) in lanes.iter_mut().enumerate() {
        signal.fill(lane, lane_seed(index));
    }

    let width = L::WIDTH;
    assert!(LANES.is_multiple_of(width), "LANES must divide by a width");
    let mut block = vec![0.0_f32; FRAMES * width];

    for group in 0..LANES / width {
        for frame in 0..FRAMES {
            for offset in 0..width {
                block[frame * width + offset] = lanes[group * width + offset][frame];
            }
        }
        run_kernel::<L>(kernel, &mut block, signal.state_seed());
        for frame in 0..FRAMES {
            for offset in 0..width {
                lanes[group * width + offset][frame] = block[frame * width + offset];
            }
        }
    }

    lanes
}

/// One kernel invocation over one AoSoA group, one shot over all [`FRAMES`] frames.
///
/// Coefficients and state are created here and consumed here: partition invariance is gate P1's
/// property and is proven in the lane crate, not re-proven across targets.
fn run_kernel<L: Lane>(kernel: Kernel, block: &mut [f32], state_seed: f32) {
    /// Frames of the coefficient ramp of [`Kernel::SvfRamped`].
    const RAMP_WINDOW: usize = 64;
    /// Frames of the gain ramp of [`Kernel::Ramp`]; shorter than `FRAMES`, so the D11 snap to the
    /// target happens inside the case.
    const GAIN_RAMP_FRAMES: usize = 512;

    let coefficients = kernel.svf_coefficients();
    let mut svf_coef = SvfCoef::<L> {
        c1: L::splat(coefficients[0]),
        a2: L::splat(coefficients[1]),
        a3: L::splat(coefficients[2]),
        m0: L::splat(coefficients[3]),
        m1: L::splat(coefficients[4]),
        m2: L::splat(coefficients[5]),
    };
    let ramping = kernel == Kernel::SvfRamped;
    let svf_step = SvfCoefStep::<L> {
        c1: L::splat(if ramping { 1.0e-6 } else { 0.0 }),
        a2: L::splat(if ramping { 2.0e-7 } else { 0.0 }),
        a3: L::splat(if ramping { 3.0e-8 } else { 0.0 }),
        m0: L::zero(),
        m1: L::zero(),
        m2: L::zero(),
    };
    let mut svf_state = SvfState::<L> {
        ic1: L::splat(state_seed),
        ic2: L::splat(state_seed),
    };

    match kernel {
        Kernel::SvfLow | Kernel::SvfHigh | Kernel::SvfBand | Kernel::SvfBell => {
            svf_block::<L>(block, FRAMES, &svf_coef, &mut svf_state);
        }
        Kernel::SvfRamped | Kernel::SvfRampedIdle => {
            let window = if ramping { RAMP_WINDOW } else { 0 };
            svf_block_ramped::<L>(
                block,
                FRAMES,
                &mut svf_coef,
                &svf_step,
                window,
                &mut svf_state,
            );
        }
        Kernel::OnePole => {
            let coef = OnePoleCoef::<L> {
                c: L::splat(0.002_083_333_3),
            };
            let mut state = OnePoleState::<L> {
                y: L::splat(state_seed),
            };
            one_pole_block::<L>(block, FRAMES, &coef, &mut state);
        }
        Kernel::Gain => gain_block::<L>(block, FRAMES, L::splat(0.501_187_2)),
        Kernel::GainMix => {
            gain_mix_block::<L>(block, FRAMES, L::splat(0.501_187_2), L::splat(0.25));
        }
        Kernel::Ramp => {
            // The ramp starts at 0.25, not at zero: a ramp starting at zero multiplies the first
            // frame by zero, and on the impulse signal -- whose only non-zero sample is that
            // frame -- the whole case would then be `+0.0` and prove nothing.
            const RAMP_START: f32 = 0.25;
            let segment = RampSegment::<L> {
                start: L::splat(RAMP_START),
                step: L::splat((1.0 - RAMP_START) / GAIN_RAMP_FRAMES as f32),
                target: L::splat(1.0),
                ramp_frames: GAIN_RAMP_FRAMES,
            };
            let _final_gain = ramp_block::<L>(block, FRAMES, &segment);
        }
        Kernel::Sum2 => {
            // `x * 0.5` and `x * 0.25` are exact, so the second operand is a scaling of the
            // signal and not a second source of rounding.
            let other: Vec<f32> = block.iter().map(|x| x * 0.5).collect();
            let mut out = vec![0.0_f32; block.len()];
            sum2_block::<L>(&mut out, block, &other);
            block.copy_from_slice(&out);
        }
        Kernel::SumInto => {
            let other: Vec<f32> = block.iter().map(|x| x * 0.25).collect();
            sum_into_block::<L>(block, &other);
        }
    }
}

/// Builds the three operand corpora of an element-wise case, per lane.
///
/// Every triple is constructed to be finite (D5) and, for [`Elementwise::Fma`], to separate a
/// fused evaluation from an unfused one.
fn elementwise_operands(operation: Elementwise, lane: usize) -> [[f32; FRAMES]; 3] {
    let mut random = Xorshift64Star::new(lane_seed(lane));
    let mut operands = [[0.0_f32; FRAMES]; 3];
    for frame in 0..FRAMES {
        let (a, b, c) = match operation {
            Elementwise::Fma => match frame % 3 {
                0 => {
                    // The witness triple of master plan §3.6, scaled by exact powers of two:
                    // `a = b = 1 + 2^-12`, `c = -(1 + 2^-11)`. Fused this is `2^-24`; unfused it
                    // is exactly zero, so an unfused build cannot pass.
                    let exponent = (frame % 41) as i32 - 20;
                    let scale = exact_pow2(exponent);
                    let a = f32::from_bits(0x3F80_0800) * scale;
                    let c = f32::from_bits(0xBF80_1000) * exact_pow2(2 * exponent);
                    (a, a, c)
                }
                1 => {
                    // The midpoint family, which is the only thing that exercises the *direction*
                    // of the software FMA's round-to-odd step (master plan hazard H1: an
                    // unconditional `bits | 1` is wrong whenever the rounded `f64` sum lies above
                    // the exact value).
                    //
                    // `a` is an odd mantissa in `[1, 4/3)` and `b` is exactly `1.5`, so the `f64`
                    // product `a * b` is exact and lands precisely halfway between two `f32`
                    // values. `c = ±2^-60` is far below half an `f64` ulp there, so the `f64` sum
                    // rounds back to the product, is flagged inexact, and the round-to-odd
                    // adjustment alone decides which of the two neighbouring `f32` values the
                    // demote produces. Getting its direction wrong changes the result for exactly
                    // the negative half of the family.
                    let mantissa = (random.next_u32() % ODD_MANTISSA_LIMIT) | 1;
                    let a = f32::from_bits(0x3F80_0000 | mantissa);
                    let c = if random.next_u32() & 1 == 0 {
                        exact_pow2(-60)
                    } else {
                        -exact_pow2(-60)
                    };
                    (a, 1.5, c)
                }
                _ => {
                    // Near-total cancellation: `c` is the negated `f32`-rounded product, so the
                    // fused result is the rounding error of that product and the unfused result is
                    // a different, coarser value.
                    let a = moderate(random.next_u32());
                    let b = moderate(random.next_u32());
                    let c = -((f64::from(a) * f64::from(b)) as f32);
                    (a, b, c)
                }
            },
            Elementwise::Exp2 => {
                // Spread across the whole exponent range and past both clamp rails, plus the exact
                // anchors `exp2_lane(0) == 1` and `exp2_lane(1) == 2`.
                let unit = f64::from(random.next_u32()) / f64::from(u32::MAX);
                let value = match frame % 8 {
                    0 => 0.0,
                    1 => 1.0,
                    2 => -1.0,
                    3 => (unit * 300.0 - 150.0) as f32,
                    _ => (unit * 40.0 - 20.0) as f32,
                };
                (value, 0.0, 0.0)
            }
            Elementwise::Log2 => {
                // Strictly positive and finite, reaching the subnormal clamp rail and the exact
                // anchors `log2_lane(1) == 0` and `log2_lane(2) == 1`.
                let value = match frame % 8 {
                    0 => 1.0,
                    1 => 2.0,
                    2 => 0.5,
                    3 => f32::from_bits(random.next_u32() & 0x007F_FFFF),
                    _ => moderate(random.next_u32()).abs(),
                };
                (value, 0.0, 0.0)
            }
        };
        for (slot, value) in operands.iter_mut().zip([a, b, c]) {
            slot[frame] = value;
        }
    }
    operands
}

/// Largest mantissa keeping `a * 1.5` below `2.0`, so the product's exponent -- and therefore the
/// `f32` ulp it is a midpoint of -- does not change under the multiply.
const ODD_MANTISSA_LIMIT: u32 = 0x0055_5555;

/// `2^n` for `n` in `[-126, 127]`, built from the exponent field: exact, and identical on every
/// target because no rounding happens.
fn exact_pow2(n: i32) -> f32 {
    let biased = n.clamp(-126, 127) + 127;
    f32::from_bits((biased as u32) << 23)
}

/// A finite value of moderate exponent: `(1 + m) * 2^e` with `e` in `[-30, 30)` and a random sign.
///
/// This is the shape audio carries. A raw bit pattern would be mostly NaN and infinity, which D5
/// excludes from the determinism claim anyway.
fn moderate(bits: u32) -> f32 {
    let mantissa = bits & 0x007F_FFFF;
    let exponent = ((bits >> 23) % 60) as i32 - 30 + 127;
    let sign = bits & 0x8000_0000;
    f32::from_bits(sign | ((exponent as u32) << 23) | mantissa)
}

/// Runs one element-wise case at width `L::WIDTH` and returns the per-lane results.
///
/// `fused` selects `Lane::fma` (the production path) or an explicit multiply followed by an add.
/// The unfused form exists for one assertion -- that the two disagree -- which is what makes the
/// `lane_fma` case a real check of `Lane::fma` and not a test that happens to pass.
fn elementwise_values<L: Lane>(operation: Elementwise, fused: bool) -> [[f32; FRAMES]; LANES] {
    let mut operands = [[[0.0_f32; FRAMES]; 3]; LANES];
    for (lane, slot) in operands.iter_mut().enumerate() {
        *slot = elementwise_operands(operation, lane);
    }

    let width = L::WIDTH;
    let mut a = vec![0.0_f32; width];
    let mut b = vec![0.0_f32; width];
    let mut c = vec![0.0_f32; width];
    let mut out = vec![0.0_f32; width];
    let mut lanes = [[0.0_f32; FRAMES]; LANES];

    for group in 0..LANES / width {
        for frame in 0..FRAMES {
            for offset in 0..width {
                let source = &operands[group * width + offset];
                a[offset] = source[0][frame];
                b[offset] = source[1][frame];
                c[offset] = source[2][frame];
            }
            let x = L::load(&a);
            let value = match operation {
                Elementwise::Fma if fused => x.fma(L::load(&b), L::load(&c)),
                Elementwise::Fma => x.mul(L::load(&b)).add(L::load(&c)),
                Elementwise::Exp2 => exp2_lane::<L>(x),
                Elementwise::Log2 => log2_lane::<L>(x),
            };
            value.store(&mut out);
            for offset in 0..width {
                lanes[group * width + offset][frame] = out[offset];
            }
        }
    }

    lanes
}

/// Digests one `miso-engine-effect-runtime` D1 case at width `L::WIDTH`, exactly as that crate's
/// `tests/determinism.rs` does natively.
fn digest_runtime<L: Lane>(case: usize) -> [u8; 32] {
    let mut out = vec![0_u32; runtime_corpus::POINTS];
    runtime_corpus::run_case::<L>(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// Digests one `miso-engine-multiband-compressor` case, exactly as that crate's
/// `tests/cross_target_digest.rs` does natively.
fn digest_multiband<L: Lane>(case: usize) -> [u8; 32] {
    let mut out = vec![0_u32; multiband_corpus::POINTS];
    multiband_corpus::run_case::<L>(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// Digests one `miso-engine-transient-shaper` case, exactly as that crate's `tests/cross_target.rs`
/// does natively.
fn digest_transient_shaper(case: usize, width: usize) -> [u8; 32] {
    let mut out = vec![0_u32; transient_shaper_corpus::WORDS];
    transient_shaper_corpus::run_case(case, transient_shaper_corpus::WIDTHS[width], &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// Digests one `miso-engine-delay` G5 case, exactly as that crate's `tests/determinism.rs` does
/// natively.
fn digest_delay(case: usize) -> [u8; 32] {
    let mut out = vec![0_u32; delay_corpus::POINTS];
    delay_corpus::run_case(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// Digests one `miso-engine-math` M3 case, exactly as `tests/m3_determinism.rs` does natively.
fn digest_math(case: usize) -> [u8; 32] {
    let mut out = vec![0_u64; math_corpus::POINTS];
    math_corpus::run_case(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// SHA-256 of every lane case, generated once from the scalar `Lane` oracle on `x86_64` and frozen
/// (master plan §8: a pin comes from the oracle, never from copying production output).
///
/// A mismatch here is never fixed by re-pinning. It means either the corpus changed, or a target
/// stopped agreeing with the oracle — and the second is the thing this gate exists to catch.
pub const LANE_DIGESTS: [[u8; 32]; LANE_CASE_COUNT] = include!("lane_digests.in");
