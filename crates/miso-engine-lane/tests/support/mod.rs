//! Shared corpus and drivers for the lane gates.
//!
//! Everything here exists to make one statement checkable: a `Lane` operation at any width is the
//! scalar operation applied lane by lane, bit for bit. The scalar implementation is the oracle
//! (master plan §3.2), so every gate compares against it and never against a tolerance.

// Each gate binary includes this module and uses a different part of it: `dead_code` and
// `unreachable_pub` would fire for the parts the current binary does not reach.
#![allow(dead_code, unreachable_pub)]

use miso_engine_lane::Lane;
use miso_engine_lane::kernels::{
    OnePoleCoef, OnePoleState, RampSegment, SvfCoef, SvfCoefStep, SvfState, gain_block,
    gain_mix_block, one_pole_block, ramp_block, sum_into_block, sum2_block, svf_block,
    svf_block_ramped,
};

/// Widest lane count the gates instantiate; every corpus length is a multiple of it.
pub const MAX_WIDTH: usize = 8;

/// Xorshift64\* (Vigna 2016): a seeded, portable, reproducible bit source.
///
/// The gates need the same inputs on every host and every run, so they never use a system random
/// number generator.
pub struct Xorshift64Star {
    /// Generator state; never zero.
    state: u64,
}

impl Xorshift64Star {
    /// Starts the generator from `seed` (zero is replaced, since it is the fixed point).
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    /// Next 64 bits.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Next 32 bits.
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// A uniform bit pattern reinterpreted as `f32`: covers NaN, infinity and subnormals densely.
    pub fn next_bit_pattern(&mut self) -> f32 {
        f32::from_bits(self.next_u32())
    }

    /// A value of moderate exponent, `(1 + m) * 2^e` with `e` in `[-30, 30)` and a random sign:
    /// the shape audio actually carries, where the bit-pattern sweep is mostly NaN and infinity.
    pub fn next_moderate(&mut self) -> f32 {
        let bits = self.next_u32();
        let mantissa = bits & 0x007F_FFFF;
        let exponent = ((bits >> 23) % 60) as i32 - 30 + 127;
        let sign = bits & 0x8000_0000;
        f32::from_bits(sign | ((exponent as u32) << 23) | mantissa)
    }

    /// Alternates [`Self::next_bit_pattern`] and [`Self::next_moderate`].
    pub fn next_mixed(&mut self, index: usize) -> f32 {
        if index.is_multiple_of(2) {
            self.next_bit_pattern()
        } else {
            self.next_moderate()
        }
    }
}

/// The directed edge pool of master plan §3.6, in the order the gates report failures.
///
/// Signed zeros and both NaN payloads are in here because they are where a backend substitution
/// hides: `wide`'s `max` is `maxps` plus a fix-up on x86 and IEEE `maxNum` on NEON, and the two
/// disagree on `max(+0.0, -0.0)` and on NaN.
pub const EDGES: &[f32] = &[
    0.0,
    -0.0,
    1.0,
    -1.0,
    2.0,
    0.5,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN,
    -f32::NAN,
    f32::from_bits(0x7FC0_0001),
    f32::from_bits(0xFFC0_0002),
    f32::MIN_POSITIVE,
    -f32::MIN_POSITIVE,
    f32::from_bits(0x0000_0001),
    f32::from_bits(0x8000_0001),
    f32::from_bits(0x007F_FFFF),
    f32::from_bits(0x807F_FFFF),
    f32::MAX,
    f32::MIN,
    1.0e-20,
    -1.0e-20,
    f32::from_bits(0x1E3C_E508_u32 + 1),
    f32::from_bits(0x1E3C_E508_u32 - 1),
    16_777_216.0,
    16_777_215.0,
    -16_777_216.0,
    1.5,
    -1.5,
    2.5,
    f32::from_bits(0x3F80_0800),
    f32::from_bits(0xBF80_1000),
    8_388_608.0,
    1.0e30,
    -1.0e30,
];

/// `1 + 2^-12`, the first half of the triple where a fused multiply-add differs from a multiply
/// followed by an add.
pub const FUSED_WITNESS_A: f32 = f32::from_bits(0x3F80_0800);

/// `-(1 + 2^-11)`, the addend of that triple: `a * a + c` is `2^-24` fused and `0` unfused.
pub const FUSED_WITNESS_C: f32 = f32::from_bits(0xBF80_1000);

/// One operation of the [`Lane`] surface, applied uniformly at every width.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    /// `a + b`.
    Add,
    /// `a - b`.
    Sub,
    /// `a * b`.
    Mul,
    /// `a / b`.
    Div,
    /// `sqrt(a)`.
    Sqrt,
    /// `fma(a, b, c)`.
    Fma,
    /// `-a`.
    Neg,
    /// `|a|`.
    Abs,
    /// `floor(a)`.
    Floor,
    /// `a < b`, as `select(mask, 1.0, 0.0)`.
    Lt,
    /// `a <= b`.
    Le,
    /// `a > b`.
    Gt,
    /// `a >= b`.
    Ge,
    /// `a == b`.
    Eq,
    /// `(a < b) & (b < c)`.
    MaskAnd,
    /// `(a < b) | (b < c)`.
    MaskOr,
    /// `!(a < b)`.
    MaskNot,
    /// `select(a < b, b, c)`.
    Select,
    /// `andnot(a, b < c)`.
    Andnot,
    /// `max(a, b)` (D8).
    Max,
    /// `min(a, b)` (D8).
    Min,
    /// `exp2_int(a)`.
    Exp2Int,
    /// Significand of `frexp(a)`.
    FrexpSignificand,
    /// Exponent of `frexp(a)`.
    FrexpExponent,
    /// `flush(a)` (D7).
    Flush,
}

/// Every operation the gates sweep.
pub const ALL_OPS: &[Op] = &[
    Op::Add,
    Op::Sub,
    Op::Mul,
    Op::Div,
    Op::Sqrt,
    Op::Fma,
    Op::Neg,
    Op::Abs,
    Op::Floor,
    Op::Lt,
    Op::Le,
    Op::Gt,
    Op::Ge,
    Op::Eq,
    Op::MaskAnd,
    Op::MaskOr,
    Op::MaskNot,
    Op::Select,
    Op::Andnot,
    Op::Max,
    Op::Min,
    Op::Exp2Int,
    Op::FrexpSignificand,
    Op::FrexpExponent,
    Op::Flush,
];

impl Op {
    /// Name used in failure reports.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Sqrt => "sqrt",
            Self::Fma => "fma",
            Self::Neg => "neg",
            Self::Abs => "abs",
            Self::Floor => "floor",
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Gt => "gt",
            Self::Ge => "ge",
            Self::Eq => "eq",
            Self::MaskAnd => "mask_and",
            Self::MaskOr => "mask_or",
            Self::MaskNot => "mask_not",
            Self::Select => "select",
            Self::Andnot => "andnot",
            Self::Max => "max",
            Self::Min => "min",
            Self::Exp2Int => "exp2_int",
            Self::FrexpSignificand => "frexp.significand",
            Self::FrexpExponent => "frexp.exponent",
            Self::Flush => "flush",
        }
    }

    /// `true` for the operations whose NaN result depends on which operand was NaN.
    ///
    /// The directed pool skips pairs with more than one NaN for these: the packed and the scalar
    /// instruction both return "the second operand if it is NaN, else the first", but the *choice*
    /// is operand-order dependent, so a pair of different payloads proves nothing about the lane
    /// contract (master plan §3.6). Comparisons, `select`, `max`, `min` and `andnot` are swept with
    /// every NaN combination, because there the NaN behaviour *is* the contract.
    #[must_use]
    pub fn nan_payload_sensitive(self) -> bool {
        matches!(
            self,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Fma
        )
    }

    /// How many of `(a, b, c)` the operation reads.
    #[must_use]
    pub fn arity(self) -> usize {
        match self {
            Self::Sqrt
            | Self::Neg
            | Self::Abs
            | Self::Floor
            | Self::Exp2Int
            | Self::FrexpSignificand
            | Self::FrexpExponent
            | Self::Flush => 1,
            Self::Fma | Self::MaskAnd | Self::MaskOr | Self::Select | Self::Andnot => 3,
            _ => 2,
        }
    }
}

/// Applies one operation at lane type `L`.
///
/// Masks are compared through a `select`, which turns them into ordinary lane bits: a mask type is
/// not required to have any particular representation, only the semantics of the trait.
#[inline(always)]
pub fn apply<L: Lane>(op: Op, a: L, b: L, c: L) -> L {
    let one = L::splat(1.0);
    let zero = L::zero();
    match op {
        Op::Add => a.add(b),
        Op::Sub => a.sub(b),
        Op::Mul => a.mul(b),
        Op::Div => a.div(b),
        Op::Sqrt => a.sqrt(),
        Op::Fma => a.fma(b, c),
        Op::Neg => a.neg(),
        Op::Abs => a.abs(),
        Op::Floor => a.floor(),
        Op::Lt => L::select(a.lt(b), one, zero),
        Op::Le => L::select(a.le(b), one, zero),
        Op::Gt => L::select(a.gt(b), one, zero),
        Op::Ge => L::select(a.ge(b), one, zero),
        Op::Eq => L::select(a.eq(b), one, zero),
        Op::MaskAnd => L::select(L::mask_and(a.lt(b), b.lt(c)), one, zero),
        Op::MaskOr => L::select(L::mask_or(a.lt(b), b.lt(c)), one, zero),
        Op::MaskNot => L::select(L::mask_not(a.lt(b)), one, zero),
        Op::Select => L::select(a.lt(b), b, c),
        Op::Andnot => a.andnot(b.lt(c)),
        Op::Max => a.max(b),
        Op::Min => a.min(b),
        Op::Exp2Int => L::exp2_int(a),
        Op::FrexpSignificand => a.frexp().0,
        Op::FrexpExponent => a.frexp().1,
        Op::Flush => miso_engine_lane::flush(a),
    }
}

/// Runs one operation over a whole corpus at lane type `L`, writing result bits.
pub fn run_op_bits<L: Lane>(op: Op, a: &[f32], b: &[f32], c: &[f32], out: &mut [u32]) {
    assert_eq!(
        a.len() % MAX_WIDTH,
        0,
        "corpus length must suit every width"
    );
    assert_eq!(a.len(), out.len());
    let mut index = 0;
    while index < a.len() {
        // The three operands are read from the same offset in three parallel corpora.
        let value = apply::<L>(
            op,
            L::load(&a[index..]),
            L::load(&b[index..]),
            L::load(&c[index..]),
        );
        value.store_bits(&mut out[index..]);
        index += L::WIDTH;
    }
}

/// A kernel of the `kernels` module, driven identically at every width by [`run_kernel`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kernel {
    /// [`svf_block`] with a low-pass coefficient set.
    SvfLow,
    /// [`svf_block`] with a high-pass coefficient set.
    SvfHigh,
    /// [`svf_block`] with a band-pass coefficient set.
    SvfBand,
    /// [`svf_block`] with a bell coefficient set.
    SvfBell,
    /// [`svf_block_ramped`] with a non-zero ramp.
    SvfRamped,
    /// [`svf_block_ramped`] with `ramp_frames = 0`, which must equal [`Kernel::SvfLow`] exactly.
    SvfRampedIdle,
    /// [`one_pole_block`].
    OnePole,
    /// [`gain_block`].
    Gain,
    /// [`gain_mix_block`].
    GainMix,
    /// [`ramp_block`].
    Ramp,
    /// [`sum2_block`].
    Sum2,
    /// [`sum_into_block`].
    SumInto,
}

/// Every kernel the gates sweep.
pub const ALL_KERNELS: &[Kernel] = &[
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

impl Kernel {
    /// Name used in failure reports.
    #[must_use]
    pub fn name(self) -> &'static str {
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

    /// The per-lane coefficient set, as a scalar broadcast to every lane.
    ///
    /// `(c1, a2, a3, m0, m1, m2)` for the state-variable filter forms. The values are the `f64`
    /// design of master plan §4.2 rounded once to `f32`: `g = tan(pi * f0 / fs)`, `k = 1 / Q`,
    /// `t = g * (g + k)`, `c1 = t / (1 + t)`, `a1 = 1 - c1`, `a2 = g * a1`, `a3 = g * a2`, at
    /// 1 kHz and Q = 0.707 for 48 kHz.
    fn svf_coefficients(self) -> [f32; 6] {
        // g = tan(pi * 1000 / 48000) = 0.065_543_46, k = 1 / 0.707 = 1.414_427_2,
        // t = g * (g + k) = 0.096_985_49, c1 = t / (1 + t) = 0.088_412_71,
        // a1 = 0.911_587_3, a2 = g * a1 = 0.059_749_45, a3 = g * a2 = 0.003_916_28.
        let (c1, a2, a3) = (0.088_412_71_f32, 0.059_749_45_f32, 0.003_916_28_f32);
        match self {
            Self::SvfHigh => [c1, a2, a3, 1.0, -1.414_427_2, -1.0],
            Self::SvfBand => [c1, a2, a3, 0.0, 1.0, 0.0],
            // Bell at +6 dB: A = 10^(6/40) = 1.412_537_5, m1 = k * (A^2 - 1) with k = 1 / (Q * A).
            Self::SvfBell => [c1, a2, a3, 1.0, 1.001_204_5, 0.0],
            _ => [c1, a2, a3, 0.0, 0.0, 1.0],
        }
    }
}

/// Runs one kernel over `io` at lane type `L`, in blocks of `partition` frames.
///
/// `io` is an AoSoA block of `frames * L::WIDTH` samples. Coefficients and state are created once,
/// before the first block, and carried across the blocks exactly as a prepared plan carries them:
/// `partition == frames` is the one-shot run and any smaller partition must produce the same bits
/// (gate P1). `state_seed` seeds every recurrence word; a subnormal seed is one of the G2 cases,
/// because the D7 flush has to remove it identically at every width.
pub fn run_kernel<L: Lane>(
    kernel: Kernel,
    io: &mut [f32],
    frames: usize,
    state_seed: f32,
    partition: usize,
) {
    assert!(partition > 0, "a partition is at least one frame");
    assert_eq!(io.len(), frames * L::WIDTH);
    let coefficients = kernel.svf_coefficients();
    let width = L::WIDTH;
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
    /// Frames of the coefficient ramp window, counted from the first frame of the whole run.
    const RAMP_WINDOW: usize = 64;
    /// Frames of the gain ramp of [`Kernel::Ramp`], counted the same way.
    const GAIN_RAMP_FRAMES: usize = 512;
    let mut svf_state = SvfState::<L> {
        ic1: L::splat(state_seed),
        ic2: L::splat(state_seed),
    };
    let mut one_pole_state = OnePoleState::<L> {
        y: L::splat(state_seed),
    };
    let one_pole_coef = OnePoleCoef::<L> {
        c: L::splat(0.002_083_333_3),
    };
    let mut gain = L::zero();

    let mut offset = 0;
    while offset < frames {
        let block_frames = core::cmp::min(partition, frames - offset);
        let block = &mut io[offset * width..(offset + block_frames) * width];
        match kernel {
            Kernel::SvfLow | Kernel::SvfHigh | Kernel::SvfBand | Kernel::SvfBell => {
                svf_block::<L>(block, block_frames, &svf_coef, &mut svf_state);
            }
            Kernel::SvfRamped | Kernel::SvfRampedIdle => {
                let window = if ramping {
                    RAMP_WINDOW.saturating_sub(offset).min(block_frames)
                } else {
                    0
                };
                svf_block_ramped::<L>(
                    block,
                    block_frames,
                    &mut svf_coef,
                    &svf_step,
                    window,
                    &mut svf_state,
                );
            }
            Kernel::OnePole => {
                one_pole_block::<L>(block, block_frames, &one_pole_coef, &mut one_pole_state);
            }
            Kernel::Gain => gain_block::<L>(block, block_frames, L::splat(0.501_187_2)),
            Kernel::GainMix => {
                gain_mix_block::<L>(block, block_frames, L::splat(0.501_187_2), L::splat(0.25));
            }
            Kernel::Ramp => {
                let segment = RampSegment::<L> {
                    start: gain,
                    step: L::splat(1.0 / GAIN_RAMP_FRAMES as f32),
                    target: L::splat(1.0),
                    ramp_frames: GAIN_RAMP_FRAMES.saturating_sub(offset),
                };
                gain = ramp_block::<L>(block, block_frames, &segment);
            }
            Kernel::Sum2 => {
                let other: std::vec::Vec<f32> = block.iter().map(|x| x * 0.5).collect();
                let mut out = std::vec![0.0f32; block.len()];
                sum2_block::<L>(&mut out, block, &other);
                block.copy_from_slice(&out);
            }
            Kernel::SumInto => {
                let other: std::vec::Vec<f32> = block.iter().map(|x| x * 0.25).collect();
                sum_into_block::<L>(block, &other);
            }
        }
        offset += block_frames;
    }
}

/// Interleaves per-lane signals into one AoSoA block: `block[f * width + l] = lanes[l][f]`.
pub fn interleave(lanes: &[std::vec::Vec<f32>], width: usize, frames: usize) -> std::vec::Vec<f32> {
    let mut block = std::vec![0.0f32; frames * width];
    for (lane_index, lane) in lanes.iter().take(width).enumerate() {
        for frame in 0..frames {
            block[frame * width + lane_index] = lane[frame];
        }
    }
    block
}

/// Reads lane `lane_index` back out of an AoSoA block.
pub fn deinterleave(
    block: &[f32],
    width: usize,
    frames: usize,
    lane_index: usize,
) -> std::vec::Vec<f32> {
    (0..frames)
        .map(|frame| block[frame * width + lane_index])
        .collect()
}

/// The G2 signal corpora: what a kernel is fed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Signal {
    /// Seeded pseudo-random noise in `[-1, 1)`.
    Noise,
    /// A unit impulse in the first frame, silence after it.
    Impulse,
    /// Constant `0.5`.
    Dc,
    /// Subnormal-magnitude noise, which the D7 flush must remove from every recurrence.
    Subnormal,
}

/// Every signal the gates sweep.
pub const ALL_SIGNALS: &[Signal] = &[
    Signal::Noise,
    Signal::Impulse,
    Signal::Dc,
    Signal::Subnormal,
];

impl Signal {
    /// Name used in failure reports.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Noise => "noise",
            Self::Impulse => "impulse",
            Self::Dc => "dc",
            Self::Subnormal => "subnormal",
        }
    }

    /// The state seed that goes with this signal: subnormal cases seed the recurrence too.
    #[must_use]
    pub fn state_seed(self) -> f32 {
        match self {
            Self::Subnormal => 1.0e-40,
            _ => 0.0,
        }
    }

    /// Fills `block` with `samples` values of this signal.
    pub fn fill(self, block: &mut [f32], seed: u64) {
        let mut random = Xorshift64Star::new(seed);
        for (index, sample) in block.iter_mut().enumerate() {
            *sample = match self {
                Self::Noise => {
                    f32::from((random.next_u32() >> 16) as u16) * (2.0 / 65_536.0_f32) - 1.0
                }
                Self::Impulse => {
                    if index < MAX_WIDTH {
                        1.0
                    } else {
                        0.0
                    }
                }
                Self::Dc => 0.5,
                Self::Subnormal => f32::from_bits((random.next_u32() & 0x007F_FFFF) | 1),
            };
        }
    }
}
