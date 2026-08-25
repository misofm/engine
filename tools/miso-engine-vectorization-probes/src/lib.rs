//! Named, non-inlined probe instantiations of every production lane-kernel family.
//!
//! Issue #144 item 3. The block kernels of `miso_engine_lane::kernels` are `#[inline(always)]`
//! generic bodies: in a shipped artifact they carry no symbol of their own, so there is nothing to
//! disassemble and nothing to name. This crate is the smallest construction that gives each family
//! a name without changing it: one `#[inline(never)]` wrapper per family, generic over
//! [`Lane`] exactly as production is, calling the production body with production arguments.
//! `miso_engine_audit vectorization` instantiates them at the backend width, emits fresh LLVM IR
//! and a fresh object, and certifies the named bodies.
//!
//! Three properties are deliberate and load-bearing:
//!
//! - **One body per family, not one per width.** The probes are generic over [`Lane`]; the width
//!   comes from the instantiation, so `probe_svf_block::<Simd8>` and `probe_svf_block::<Simd4>`
//!   are the same source text and cannot drift apart.
//! - **Block lengths are compile-time constants.** [`PROBE_WORDS`] is 256, a multiple of both
//!   production widths, so every kernel's scalar tail is provably dead at every instantiation and
//!   a surviving scalar floating-point instruction in a probe body is a real vectorization
//!   failure. Production keeps its scalar tail for track counts that are not a multiple of the
//!   width; the probes remove it from the *evidence*, not from the engine.
//! - **Nothing here is reachable from a render path.** This crate is a build-QA subject. No
//!   shipped host, the C ABI, or any effect crate depends on it.

use miso_engine_lane::Lane;
use miso_engine_lane::kernels::builtins::{
    InputChainCoef, InputChainReport, InputChainState, Matrix2x2Coef, Matrix2x2Ramp, all_lanes,
    gain_mute_block, input_chain_block, matrix2x2_block, matrix2x2_ramp_block,
    nonfinite_lanes_block, sanitize_gain_block, zero_lanes_block,
};
use miso_engine_lane::kernels::halfband::{
    HALFBAND63_BASE, HALFBAND63_ROWS, halfband2x_decim_even, halfband2x_interp_even, history_push,
};
use miso_engine_lane::kernels::{
    OnePoleCoef, OnePoleState, RampSegment, SvfCoef, SvfCoefStep, SvfState, gain_block,
    gain_mix_block, gain_mix_step, mix2x2_block, one_pole_block, pdc_delay_block, ramp_block,
    sum_into_block, sum2_block, svf_block, svf_block_ramped, svf_step,
};

/// Words in one probe block: 256, a multiple of both production widths (4 and 8).
///
/// The constant is what makes a scalar instruction inside a probe body a finding rather than a
/// legitimate tail: at every instantiation `PROBE_WORDS % L::WIDTH == 0`, so the tail loop of every
/// kernel that has one is statically dead.
pub const PROBE_WORDS: usize = 256;

/// Words in one probe half-band history: [`HALFBAND63_ROWS`] rows at the widest production width.
pub const PROBE_HISTORY_WORDS: usize = HALFBAND63_ROWS * 8;

/// One probe block: a planar AoSoA buffer of [`PROBE_WORDS`] words.
pub type Block = [f32; PROBE_WORDS];

/// One probe half-band history buffer.
pub type History = [f32; PROBE_HISTORY_WORDS];

/// Frames in one probe block at width `L`.
#[inline(always)]
fn frames<L: Lane>() -> usize {
    PROBE_WORDS / L::WIDTH
}

/// Probe for the recursive TPT state-variable filter over a whole block.
#[inline(never)]
pub fn probe_svf_block<L: Lane>(io: &mut Block, c: &SvfCoef<L>, s: &mut SvfState<L>) {
    svf_block::<L>(io.as_mut_slice(), frames::<L>(), c, s);
}

/// Probe for one state-variable filter sample step.
#[inline(never)]
pub fn probe_svf_step<L: Lane>(v0: L, nc1: L, a2: L, a3: L, s: &mut SvfState<L>) -> (L, L) {
    svf_step::<L>(v0, nc1, a2, a3, s)
}

/// Probe for the ramped state-variable filter over a whole block.
#[inline(never)]
pub fn probe_svf_block_ramped<L: Lane>(
    io: &mut Block,
    c: &mut SvfCoef<L>,
    step: &SvfCoefStep<L>,
    s: &mut SvfState<L>,
) {
    let count = frames::<L>();
    svf_block_ramped::<L>(io.as_mut_slice(), count, c, step, count, s);
}

/// Probe for the recursive one-pole smoother over a whole block.
#[inline(never)]
pub fn probe_one_pole_block<L: Lane>(io: &mut Block, c: &OnePoleCoef<L>, s: &mut OnePoleState<L>) {
    one_pole_block::<L>(io.as_mut_slice(), frames::<L>(), c, s);
}

/// Probe for the feed-forward constant-gain kernel.
#[inline(never)]
pub fn probe_gain_block<L: Lane>(io: &mut Block, g: L) {
    gain_block::<L>(io.as_mut_slice(), frames::<L>(), g);
}

/// Probe for the feed-forward gain-and-dry/wet-mix kernel.
#[inline(never)]
pub fn probe_gain_mix_block<L: Lane>(io: &mut Block, g: L, mix: L) {
    gain_mix_block::<L>(io.as_mut_slice(), frames::<L>(), g, mix);
}

/// Probe for one gain-and-mix sample step.
#[inline(never)]
pub fn probe_gain_mix_step<L: Lane>(x: L, g: L, mix: L) -> L {
    gain_mix_step::<L>(x, g, mix)
}

/// Probe for the ramped-gain kernel.
#[inline(never)]
pub fn probe_ramp_block<L: Lane>(io: &mut Block, seg: &RampSegment<L>) -> L {
    ramp_block::<L>(io.as_mut_slice(), frames::<L>(), seg)
}

/// Probe for the two-input summing kernel.
#[inline(never)]
pub fn probe_sum2_block<L: Lane>(out: &mut Block, a: &Block, b: &Block) {
    sum2_block::<L>(out.as_mut_slice(), a.as_slice(), b.as_slice());
}

/// Probe for the accumulating summing kernel.
#[inline(never)]
pub fn probe_sum_into_block<L: Lane>(acc: &mut Block, x: &Block) {
    sum_into_block::<L>(acc.as_mut_slice(), x.as_slice());
}

/// Probe for the integer-sample plugin-delay-compensation ring.
///
/// This family is not generic over [`Lane`]: it is an integer-indexed swap of whole words and
/// performs no floating-point arithmetic at any width. Its rule class says exactly that.
#[inline(never)]
pub fn probe_pdc_delay_block(ring: &mut Block, cursor: &mut usize, io: &mut Block) {
    pdc_delay_block(ring.as_mut_slice(), cursor, io.as_mut_slice());
}

/// Probe for the 2x2 route/pan matrix kernel.
#[inline(never)]
pub fn probe_mix2x2_block<L: Lane>(left: &mut Block, right: &mut Block, c: [f32; 4]) {
    mix2x2_block::<L>(left.as_mut_slice(), right.as_mut_slice(), c);
}

/// Probe for the builtins sanitize-and-gain kernel.
#[inline(never)]
pub fn probe_sanitize_gain_block<L: Lane>(io: &mut Block, gain: L) -> L {
    sanitize_gain_block::<L>(io.as_mut_slice(), frames::<L>(), gain)
}

/// Probe for the builtins non-finite lane detector.
#[inline(never)]
pub fn probe_nonfinite_lanes_block<L: Lane>(io: &Block) -> L::Mask {
    nonfinite_lanes_block::<L>(io.as_slice(), frames::<L>())
}

/// Probe for the builtins masked lane-zeroing kernel.
#[inline(never)]
pub fn probe_zero_lanes_block<L: Lane>(io: &mut Block, m: L::Mask) {
    zero_lanes_block::<L>(io.as_mut_slice(), frames::<L>(), m);
}

/// Probe for the builtins gain-and-mute kernel.
#[inline(never)]
pub fn probe_gain_mute_block<L: Lane>(io: &mut Block, gain: L, mute: L::Mask) {
    gain_mute_block::<L>(io.as_mut_slice(), frames::<L>(), gain, mute);
}

/// Probe for the builtins static 2x2 matrix kernel.
#[inline(never)]
pub fn probe_matrix2x2_block<L: Lane>(left: &mut Block, right: &mut Block, c: &Matrix2x2Coef<L>) {
    matrix2x2_block::<L>(left.as_mut_slice(), right.as_mut_slice(), frames::<L>(), c);
}

/// Probe for the builtins ramped 2x2 matrix kernel.
#[inline(never)]
pub fn probe_matrix2x2_ramp_block<L: Lane>(
    left: &mut Block,
    right: &mut Block,
    r: &mut Matrix2x2Ramp<L>,
) {
    matrix2x2_ramp_block::<L>(left.as_mut_slice(), right.as_mut_slice(), frames::<L>(), r);
}

/// Probe for the fused builtins input chain.
#[inline(never)]
pub fn probe_input_chain_block<L: Lane>(
    left: &mut Block,
    right: &mut Block,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
) -> InputChainReport<L> {
    input_chain_block::<L>(
        left.as_mut_slice(),
        right.as_mut_slice(),
        frames::<L>(),
        c,
        s,
    )
}

/// Probe for the half-band history write.
#[inline(never)]
pub fn probe_history_push<L: Lane>(history: &mut History, pos: usize, value: L) {
    history_push::<L>(&mut history[..HALFBAND63_ROWS * L::WIDTH], pos, value);
}

/// Probe for the half-band 2x interpolation tap loop.
#[inline(never)]
pub fn probe_halfband2x_interp_even<L: Lane>(history: &History) -> L {
    halfband2x_interp_even::<L>(&history[..HALFBAND63_ROWS * L::WIDTH], HALFBAND63_BASE)
}

/// Probe for the half-band 2x decimation tap loop.
#[inline(never)]
pub fn probe_halfband2x_decim_even<L: Lane>(history: &History, odd: L) -> L {
    halfband2x_decim_even::<L>(&history[..HALFBAND63_ROWS * L::WIDTH], HALFBAND63_BASE, odd)
}

/// Calls every generic probe once at width `L`, so nothing is dropped as unreachable.
///
/// The return value folds one word from every family, so no probe can be optimized away as dead.
#[must_use]
#[expect(
    clippy::too_many_lines,
    reason = "one straight-line call of every kernel family; splitting it would hide the roster"
)]
pub fn run_all<L: Lane>(seed: f32) -> f32 {
    let mut left: Block = [seed; PROBE_WORDS];
    let mut right: Block = [seed * 0.5; PROBE_WORDS];
    let a: Block = [seed * 0.25; PROBE_WORDS];
    let b: Block = [seed * 0.125; PROBE_WORDS];
    let mut history: History = [seed * 0.0625; PROBE_HISTORY_WORDS];

    let coefficients = SvfCoef {
        c1: L::splat(0.1),
        a2: L::splat(0.1),
        a3: L::splat(0.01),
        m0: L::splat(0.0),
        m1: L::splat(0.0),
        m2: L::splat(1.0),
    };
    let step = SvfCoefStep {
        c1: L::splat(1.0e-6),
        a2: L::splat(1.0e-6),
        a3: L::splat(1.0e-7),
        m0: L::splat(0.0),
        m1: L::splat(0.0),
        m2: L::splat(0.0),
    };
    let mut state = SvfState {
        ic1: L::zero(),
        ic2: L::zero(),
    };

    probe_svf_block::<L>(&mut left, &coefficients, &mut state);
    let mut ramped = coefficients;
    probe_svf_block_ramped::<L>(&mut left, &mut ramped, &step, &mut state);
    let (v1, v2) = probe_svf_step::<L>(
        L::splat(seed),
        L::splat(-0.1),
        L::splat(0.1),
        L::splat(0.01),
        &mut state,
    );

    let mut one_pole = OnePoleState { y: L::zero() };
    probe_one_pole_block::<L>(&mut left, &OnePoleCoef { c: L::splat(0.5) }, &mut one_pole);

    probe_gain_block::<L>(&mut left, L::splat(0.75));
    probe_gain_mix_block::<L>(&mut left, L::splat(0.75), L::splat(0.25));
    let mixed = probe_gain_mix_step::<L>(L::splat(seed), L::splat(0.75), L::splat(0.25));

    let segment = RampSegment {
        start: L::splat(0.0),
        step: L::splat(1.0e-3),
        target: L::splat(1.0),
        ramp_frames: frames::<L>() / 2,
    };
    let ramp_end = probe_ramp_block::<L>(&mut left, &segment);

    probe_sum2_block::<L>(&mut left, &a, &b);
    probe_sum_into_block::<L>(&mut left, &a);

    let mut ring: Block = [0.0; PROBE_WORDS];
    let mut cursor = 0usize;
    probe_pdc_delay_block(&mut ring, &mut cursor, &mut right);

    probe_mix2x2_block::<L>(&mut left, &mut right, [0.7, 0.3, 0.3, 0.7]);

    let sanitized = probe_sanitize_gain_block::<L>(&mut left, L::splat(0.9));
    let nonfinite = probe_nonfinite_lanes_block::<L>(&left);
    probe_zero_lanes_block::<L>(&mut left, nonfinite);
    probe_gain_mute_block::<L>(&mut left, L::splat(0.8), nonfinite);

    let matrix = Matrix2x2Coef {
        ll: L::splat(0.7),
        lr: L::splat(0.3),
        rl: L::splat(0.3),
        rr: L::splat(0.7),
        identity: all_lanes::<L>(),
    };
    probe_matrix2x2_block::<L>(&mut left, &mut right, &matrix);

    let mut matrix_ramp = Matrix2x2Ramp {
        current: [L::splat(0.7), L::splat(0.3), L::splat(0.3), L::splat(0.7)],
        target: [L::splat(0.6), L::splat(0.4), L::splat(0.4), L::splat(0.6)],
        step: [L::splat(1.0e-4); 4],
        remaining: L::splat(64.0),
    };
    probe_matrix2x2_ramp_block::<L>(&mut left, &mut right, &mut matrix_ramp);

    let chain = InputChainCoef {
        trim: [L::splat(1.0), L::splat(1.0)],
        section: [[coefficients, coefficients], [coefficients, coefficients]],
    };
    let mut chain_state = InputChainState {
        section: [
            [
                SvfState {
                    ic1: L::zero(),
                    ic2: L::zero(),
                },
                SvfState {
                    ic1: L::zero(),
                    ic2: L::zero(),
                },
            ],
            [
                SvfState {
                    ic1: L::zero(),
                    ic2: L::zero(),
                },
                SvfState {
                    ic1: L::zero(),
                    ic2: L::zero(),
                },
            ],
        ],
    };
    let report = probe_input_chain_block::<L>(&mut left, &mut right, &chain, &mut chain_state);

    probe_history_push::<L>(&mut history, 1, L::splat(seed));
    let interpolated = probe_halfband2x_interp_even::<L>(&history);
    let decimated = probe_halfband2x_decim_even::<L>(&history, L::splat(seed));

    let mut word = [0.0f32; 8];
    let mut total = left[0] + right[0] + ring[0] + history[0];
    for value in [
        v1,
        v2,
        mixed,
        ramp_end,
        sanitized,
        report.sanitized[0],
        interpolated,
        decimated,
    ] {
        value.store(&mut word[..L::WIDTH]);
        total += word[0];
    }
    total
}

/// Instantiates every probe at the eight-lane production width.
///
/// The x86-64-v3 backend (`Simd8`, one `__m256`). Non-generic, so an `rlib` build emits the whole
/// roster into its object without a linked binary to force instantiation.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[must_use]
pub fn run_simd8(seed: f32) -> f32 {
    run_all::<miso_engine_lane::Simd8>(seed)
}

/// Instantiates every probe at the four-lane production width.
///
/// The AArch64 NEON and wasm `simd128` backend (`Simd4`).
#[cfg(any(target_arch = "aarch64", target_arch = "wasm32"))]
#[must_use]
pub fn run_simd4(seed: f32) -> f32 {
    run_all::<miso_engine_lane::Simd4>(seed)
}
