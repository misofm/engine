//! Certified tail and exact-rest bounds for cascades of TPT SVF sections (issue #1329).
//!
//! Control-plane only: every function here allocates at most a few small vectors and runs in
//! `f64`. Nothing here is called on the render thread.
//!
//! # The kernel this bounds
//!
//! One section is the stored-form TPT state-variable filter [SIMPER-SVF] [ZAVALISHIN-TPT] the
//! engine renders in `f32` (`lane::kernels::svf_step_when` and its output mix):
//!
//! ```text
//! v3 = v0 - ic2;  d1 = -c1 ic1 + a2 v3;  d2 = a3 v3 + a2 ic1
//! v1 = ic1 + d1;  v2 = ic2 + d2;         (ic1, ic2) = flush(ic1 + 2 d1, ic2 + 2 d2)
//! y  = m2 v2 + (m1 v1 + m0 v0)
//! ```
//!
//! With state `s = (ic1, ic2)` this is `s' = A s + b x`, `y = c . s + d x`, with
//! `A = [[1 - 2 c1, -2 a2], [2 a2, 1 - 2 a3]]`, `b = (2 a2, 2 a3)`,
//! `c = (m1 (1 - c1) + m2 a2, -m1 a2 + m2 (1 - a3))` and `d = m0 + m1 a2 + m2 a3`.
//!
//! # The norm
//!
//! Every Butterworth (`k = sqrt(2)`) design shares one eigenbasis, so the bounds use one norm,
//! `||x||_V = ||R x||_2` with `R = [[1, r], [0, r]]`, `r = 1 / sqrt(2)`
//! (`||x||_V^2 = x1^2 + sqrt(2) x1 x2 + x2^2`). In it an exact design's `A` is a scaled rotation,
//! so `||A^n||_V` is the pole radius to the `n`-th power with no Putzer factor. `||R||_2` and
//! `||R^-1||_2` convert to and from the per-word magnitudes; their product is
//! `kappa = 1 + sqrt(2)`. Other section families reuse the machinery with the same norm: it is
//! sound for any words, only less tight where the words are far from `k = sqrt(2)`.
//!
//! # What is bounded
//!
//! The derivation is `docs/derivations/1329-input-section-tail-and-rest.md`. In short:
//!
//! * [`fixed_cascade`]: a cascade whose words never change. The exact half of the tail is an
//!   exact-arithmetic **reference** that resets its sections at the same instants as the kernel
//!   (the joint flush of #1328 and the per-block non-finite recovery). Its states and output are
//!   bounded, for every reset pattern, by triangle-inequality suffix sums of impulse majorants,
//!   summed exactly to a horizon and closed by a contraction remainder. The `f32` half is the
//!   kernel's deviation from that reference: relative rounding (this module's rounding count) and
//!   the absolute per-word flush, never modelled as relative error.
//! * [`live_cascade`]: a two-section cascade whose words may change under a control law (issue
//!   #1433), given per-section suprema over every reachable word ([`EnvelopeSection`]) and where
//!   those words lie as poles ([`PoleDomain`]). It is frequency-aware: the first section's output
//!   is `c . s = 1/2 (m1, m2) (I + A) s`, which a slow pole near `-1` makes small, and its state
//!   is bounded per zone of pole position ([`LiveZones`]), so the second section is charged only
//!   for what the first can deliver near its own slow pole.
//! * Exact rest, for both: proven section by section, first section first. A section rests on the
//!   first armed frame (its effect input exactly zero for `N_SILENCE` frames, #1328 amendment A9)
//!   on which both its words are below `REST_EPS`; once it rests its output is exactly zero, so
//!   the next section decays freely. Each section in sequence adds `N_SILENCE`.

use std::vec::Vec;

/// `10^(-144/20)`: the tail floor of decision 15 D15-4(b), relative to the input's peak.
pub const TAIL_FLOOR: f64 = 6.309_573_444_801_932e-8;

/// The `f32` unit roundoff, `2^-24`.
const U: f64 = 1.0 / 16_777_216.0;

/// `2^-126`, the smallest normal `f32`: the absolute error one `f32` operation can make by
/// underflowing (gradual underflow loses less, flush-to-zero at most this).
const UNDERFLOW: f64 = 1.0 / 85_070_591_730_234_615_865_843_651_857_942_052_864.0;

/// Absolute underflow allowance per section step, per word and on the output: sixteen
/// operations' worth, more than the step or the mix performs.
const UNDERFLOW_PER_STEP: f64 = 16.0 * UNDERFLOW;

/// The largest finite `f32`, as `f64`.
const F32_MAX: f64 = 3.402_823_466_385_288_6e38;

/// `1 / sqrt(2)`.
const R: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// `||R||_2 = sqrt(1 + r)`, rounded up.
const R_NORM: f64 = 1.306_562_964_876_377_2;

/// `||R^-1||_2 = 1 / sqrt(1 - r)`, rounded up: the largest per-word magnitude of a state of unit
/// `V`-norm.
pub const R_INV_NORM: f64 = 1.847_759_065_022_573_6;

/// `kappa = ||R||_2 ||R^-1||_2 = 1 + sqrt(2)`, rounded up.
const KAPPA: f64 = 2.414_213_562_373_095_5;

/// The relative inflation applied to a computed constant, and to every step of a propagation, so
/// that the `f64` rounding of that one evaluation cannot make it optimistic: `2^-30`, far above the
/// few `f64` units of roundoff (`2^-53` each) one evaluation of a short, non-cancelling expression
/// makes. It covers only such evaluations. Where an evaluation can cancel (the operator norm) or a
/// rounding compounds over many steps (the impulse response, the majorant recursions, the long
/// sums), the error is bounded explicitly instead ([`spectral_norm_bound`], [`Majorants`],
/// [`accumulation`]).
const SLACK: f64 = 1.0 + 1.0 / 1_073_741_824.0;

/// The `f64` unit roundoff, `2^-53`.
const U64: f64 = 1.0 / 9_007_199_254_740_992.0;

/// One step of a non-negative recursion evaluated in `f64`, made an upper bound. Each summand of
/// the step reaches the result through at most two roundings (a rounded product, then the
/// rounded additions it passes through: in a left-to-right sum `(x + y) + z`, `x` and `y` pass
/// through two additions and `z` through one), and the product with `1 + 4u` rounds once more:
/// every summand is at least `(1 - u)^3 (1 + 4u) = 1 + u - 9u^2 + ... >= 1` times its exact
/// value, so by induction the inflated recursion bounds the exact one at every step, however many
/// steps it runs. The one summand with a third rounding before the product, the first section's
/// error growth `nu (|s_1| + |s_2|)` in [`Majorants`] (a rounded sum, then a product, then the
/// outer addition), falls short of its exact value by at most `10 u^2` relative; `nu` exceeds the
/// step error it bounds by a factor above 1.8 ([`FIRST_STEP_ERROR`]), which absorbs it.
const STEP_UP: f64 = 1.0 + 4.0 * U64;

/// The relative error of a sum of `terms` non-negative `f64` values, as an inflation:
/// `(n - 1) u / (1 - (n - 1) u)` at most, bounded here by `2 n u` (with `n u` far below one).
fn accumulation(terms: u64) -> f64 {
    1.0 + 2.0 * (terms as f64) * U64
}

/// The longest sum this module evaluates before it gives up: no bound above it is stated.
const HORIZON_LIMIT: u64 = 1 << 26;

/// Frames per checkpoint block of the suffix-sum search.
const BLOCK: usize = 256;

/// One section's six words, read from the `f32` words the kernel loads and widened exactly.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SvfWords {
    /// Damping coefficient.
    pub c1: f64,
    /// `g (1 - c1)`.
    pub a2: f64,
    /// `g a2`.
    pub a3: f64,
    /// Direct mix.
    pub m0: f64,
    /// Band mix.
    pub m1: f64,
    /// Low mix.
    pub m2: f64,
}

impl SvfWords {
    /// Widens the six `f32` words `[c1, a2, a3, m0, m1, m2]` exactly.
    #[must_use]
    pub fn from_f32(words: [f32; 6]) -> Self {
        let [c1, a2, a3, m0, m1, m2] = words.map(f64::from);
        Self {
            c1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// `A`, the zero-input state step.
    #[must_use]
    pub fn a(&self) -> [[f64; 2]; 2] {
        [
            [1.0 - 2.0 * self.c1, -2.0 * self.a2],
            [2.0 * self.a2, 1.0 - 2.0 * self.a3],
        ]
    }

    /// `b`, the input column.
    #[must_use]
    pub fn b(&self) -> [f64; 2] {
        [2.0 * self.a2, 2.0 * self.a3]
    }

    /// `c`, the output row on the state.
    #[must_use]
    pub fn c(&self) -> [f64; 2] {
        [
            self.m1 * (1.0 - self.c1) + self.m2 * self.a2,
            -self.m1 * self.a2 + self.m2 * (1.0 - self.a3),
        ]
    }

    /// `d`, the direct feedthrough.
    #[must_use]
    pub fn d(&self) -> f64 {
        self.m0 + self.m1 * self.a2 + self.m2 * self.a3
    }
}

/// `||x||_V` of a state.
#[must_use]
pub fn v_norm(x: [f64; 2]) -> f64 {
    let first = x[0] + R * x[1];
    let second = R * x[1];
    crate::sqrt(first * first + second * second)
}

/// The dual norm of an output row `c`: the smallest `gamma` with `|c . x| <= gamma ||x||_V` for
/// every `x`, `||c R^-1||_2` with `R^-1 = [[1, -1], [0, sqrt(2)]]`.
#[must_use]
pub fn v_dual_norm(c: [f64; 2]) -> f64 {
    let second = -c[0] + core::f64::consts::SQRT_2 * c[1];
    crate::sqrt(c[0] * c[0] + second * second)
}

/// A certified upper bound on the spectral norm of a 2x2 matrix `M`, given an `f64` matrix `m` and
/// componentwise bounds `error[i][j] >= |M_ij - m_ij|`.
///
/// For `m = [[a, b], [c, d]]` the singular values satisfy `s1 + s2 = sqrt((a + d)^2 + (c - b)^2)`
/// and `s1 - s2 = sqrt((a - d)^2 + (b + c)^2)`, so `s1` is half their sum. Every operation of that
/// form is non-cancelling (one rounded sum or difference squared, sums of squares, square roots,
/// one sum of non-negative values), so the computed value is within `(1 +- u)^4` of the exact `s1`
/// of `m`. The textbook form `sqrt((F + sqrt(F^2 - 4 det^2)) / 2)` is not: for a near-rotation (two
/// almost equal singular values, every Butterworth design in the `V`-basis) `F^2 - 4 det^2`
/// cancels, and its rounding can put the result below the exact norm by far more than `2^-30`
/// (#1329 attempt 3, M1). The perturbation adds at most `||M - m||_2 <= ||E||_F`, the square root
/// of the sum of the squared componentwise bounds (within `(1 +- u)^4` as computed). The two terms
/// and their sum are inflated by `1 + 16 u`, which covers all three relative errors and the final
/// rounding.
fn spectral_norm_bound(m: [[f64; 2]; 2], error: [[f64; 2]; 2]) -> f64 {
    let [[a, b], [c, d]] = m;
    let sum = crate::sqrt((a + d) * (a + d) + (c - b) * (c - b));
    let difference = crate::sqrt((a - d) * (a - d) + (b + c) * (b + c));
    let perturbation = crate::sqrt(
        error[0][0] * error[0][0]
            + error[0][1] * error[0][1]
            + error[1][0] * error[1][0]
            + error[1][1] * error[1][1],
    );
    (0.5 * (sum + difference) + perturbation) * (1.0 + 16.0 * U64)
}

/// `||M||_V`, the operator norm `M` induces in the `V`-norm, as a certified upper bound: the
/// spectral norm of `R M R^-1`, computed by a non-cancelling closed form with its own error term.
///
/// For `M = [[al, be], [ga, de]]` and `sqrt(2) r = 1`, `R M R^-1` is exactly
/// `[[al + r ga, sqrt(2) be + de - al - r ga], [r ga, de - r ga]]`; the `1`s of a design's diagonal
/// cancel in exact arithmetic, never in rounding. Each entry is evaluated in `f64` with the rounded
/// constants `r` and `sqrt(2)`, and each entry of `M` is taken as known only to within one rounding
/// (a caller's `1 - 2 c1`): every term passes through at most six roundings (the entry, the
/// constant, the product, three additions), so an entry's error is at most `gamma_6` times the sum
/// of its terms' magnitudes, bounded here by `16 u` times that sum (which also covers the bound's
/// own rounding).
#[must_use]
pub fn v_operator_norm(m: [[f64; 2]; 2]) -> f64 {
    let sqrt2 = core::f64::consts::SQRT_2;
    let [[al, be], [ga, de]] = m;
    let r_ga = R * ga;
    let transformed = [[al + r_ga, sqrt2 * be + de - al - r_ga], [r_ga, de - r_ga]];
    let magnitude = [
        [
            al.abs() + r_ga.abs(),
            (sqrt2 * be).abs() + de.abs() + al.abs() + r_ga.abs(),
        ],
        [r_ga.abs(), de.abs() + r_ga.abs()],
    ];
    let error = magnitude.map(|row| row.map(|terms| 16.0 * U64 * terms));
    spectral_norm_bound(transformed, error)
}

/// The sup of `2 ||[[e1, e2], [-e2, e3]]||_V` over the box `|e_i| <= box_[i]`: how much a
/// componentwise perturbation of the recursion words `(c1, a2, a3)` can add to `||A||_V`.
///
/// `A` is affine in the words with `A(w + e) - A(w) = -2 [[e1, e2], [-e2, e3]]`, and the norm is
/// convex, so the supremum over the box is at one of its eight vertices (#1407's `P`).
#[must_use]
pub fn word_box_norm(box_: [f64; 3]) -> f64 {
    let mut worst = 0.0_f64;
    for signs in 0..8_u32 {
        let sign = |bit: u32| if signs & (1 << bit) == 0 { 1.0 } else { -1.0 };
        let (e1, e2, e3) = (sign(0) * box_[0], sign(1) * box_[1], sign(2) * box_[2]);
        worst = worst.max(2.0 * v_operator_norm([[e1, e2], [-e2, e3]]));
    }
    worst
}

/// The rounding of the `f32` state step, as a relative bound on the next state:
/// `||fl-step(s, x) - (A s + b x)||_V <= mu_state ||s||_V + mu_input |x|`, before the flush.
///
/// Counting every rounding of the frozen order (`gamma_n = n u / (1 - n u)`):
/// `|d1 - d1e| <= gamma2 c1 |s1| + gamma3 a2 (|x| + |s2|)`,
/// `|d2 - d2e| <= gamma3 a3 (|x| + |s2|) + gamma2 a2 |s1|`, and the final addition
/// `n = fl(s + 2 d)` adds `2 (1 + u) |d - de| + u |n_e|`. So the componentwise error is at most
/// `G |s| + g_x |x| + u |n_e|`; in the `V`-norm, `||R|| ||G|| ||R^-1|| + kappa u ||A||_V` on the
/// state and `||R|| ||g_x|| + kappa u ||b||_V` on the input. The last term, the final rounding of
/// `n1` and `n2`, is what makes the rate inflation `7 * 2^-24 * kappa` at the top of the domain
/// (Amendment 2).
#[must_use]
pub fn state_rounding(words: [f64; 3], q: f64, beta: f64) -> (f64, f64) {
    let [c1, a2, a3] = words.map(f64::abs);
    let k = 1.0 + U;
    let gamma2 = 2.0 * U / (1.0 - 2.0 * U);
    let gamma3 = 3.0 * U / (1.0 - 3.0 * U);
    let state = [
        [2.0 * k * gamma2 * c1, 2.0 * k * gamma3 * a2],
        [2.0 * k * gamma2 * a2, 2.0 * k * gamma3 * a3],
    ];
    let input = [2.0 * k * gamma3 * a2, 2.0 * k * gamma3 * a3];
    let mu_state = R_NORM * spectral_norm_bound(state, [[0.0; 2]; 2]) * R_INV_NORM + KAPPA * U * q;
    let mu_input =
        R_NORM * crate::sqrt(input[0] * input[0] + input[1] * input[1]) + KAPPA * U * beta;
    (mu_state * SLACK, mu_input * SLACK)
}

/// The rounding of the `f32` output mix `y = m2 v2 + (m1 v1 + m0 v0)`, as a bound
/// `|fl-y - (c . s + d x)| <= omega_state ||s||_V + omega_input |x|`.
///
/// `v1 = fl(s1 + d1)` is within `(1 + u) |d1 - d1e| + u |v1e|` of `v1e = (1 - c1) s1 - a2 s2 + a2 x`
/// and `v2` likewise of `v2e = a2 s1 + (1 - a3) s2 + a3 x`; the mix adds `gamma3 |m1 v1|`,
/// `gamma2 |m2 v2|` and `gamma3 |m0 x|`.
#[must_use]
pub fn output_rounding(words: &SvfWords) -> (f64, f64) {
    let (c1, a2, a3) = (words.c1.abs(), words.a2.abs(), words.a3.abs());
    let (one_c1, one_a3) = ((1.0 - words.c1).abs(), (1.0 - words.a3).abs());
    let (m0, m1, m2) = (words.m0.abs(), words.m1.abs(), words.m2.abs());
    let k = 1.0 + U;
    let gamma2 = 2.0 * U / (1.0 - 2.0 * U);
    let gamma3 = 3.0 * U / (1.0 - 3.0 * U);
    let v1_error = [
        k * gamma2 * c1 + U * one_c1,
        k * gamma3 * a2 + U * a2,
        k * gamma3 * a2 + U * a2,
    ];
    let v2_error = [
        k * gamma2 * a2 + U * a2,
        k * gamma3 * a3 + U * one_a3,
        k * gamma3 * a3 + U * a3,
    ];
    let v1_magnitude = [one_c1, a2, a2];
    let v2_magnitude = [a2, one_a3, a3];
    let w: [f64; 3] = core::array::from_fn(|i| {
        m1 * (1.0 + gamma3) * v1_error[i]
            + m2 * (1.0 + gamma2) * v2_error[i]
            + gamma3 * m1 * v1_magnitude[i]
            + gamma2 * m2 * v2_magnitude[i]
    });
    let omega_state = crate::sqrt(w[0] * w[0] + w[1] * w[1]) * R_INV_NORM;
    let omega_input = w[2] + gamma3 * m0;
    (omega_state * SLACK, omega_input * SLACK)
}

/// One section's bound constants, all in the `V`-norm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionConstants {
    /// `||A||_V`, the exact contraction.
    pub q: f64,
    /// `||b||_V`.
    pub beta: f64,
    /// The dual norm of `c`.
    pub gamma: f64,
    /// `|d|`.
    pub delta: f64,
    /// Relative state-step rounding on the state ([`state_rounding`]).
    pub mu_state: f64,
    /// Relative state-step rounding on the input.
    pub mu_input: f64,
    /// Relative output rounding on the state ([`output_rounding`]).
    pub omega_state: f64,
    /// Relative output rounding on the input.
    pub omega_input: f64,
}

impl SectionConstants {
    /// The constants of one fixed section. `q` is [`v_operator_norm`]'s certified bound, which
    /// already carries its own rounding; the other norms, short non-cancelling evaluations (the
    /// dual norm's `-c1 + sqrt(2) c2` is at most `2 gamma` in magnitude per term), carry the
    /// `2^-30` inflation.
    #[must_use]
    pub fn of(words: &SvfWords) -> Self {
        let q = v_operator_norm(words.a());
        let beta = v_norm(words.b()) * SLACK;
        let (mu_state, mu_input) = state_rounding([words.c1, words.a2, words.a3], q, beta);
        let (omega_state, omega_input) = output_rounding(words);
        Self {
            q,
            beta,
            gamma: v_dual_norm(words.c()) * SLACK,
            delta: words.d().abs() * SLACK,
            mu_state,
            mu_input,
            omega_state,
            omega_input,
        }
    }

    /// The kernel's contraction, rounding included: `q + mu_state`.
    #[must_use]
    pub fn rho_kernel(&self) -> f64 {
        self.q + self.mu_state
    }

    /// The kernel envelope of this one fixed section ([`EnvelopeSection`]).
    #[must_use]
    pub fn envelope(&self) -> EnvelopeSection {
        let rho = self.rho_kernel();
        EnvelopeSection {
            rho_ramp: rho,
            rho_settled: rho,
            input: self.beta + self.mu_input,
            output_state: self.gamma + self.omega_state,
            output_input: self.delta + self.omega_input,
        }
    }
}

/// The flush law of #1328: `FLUSH_EPS`, `REST_EPS` and `N_SILENCE` at the node's rate. The caller
/// passes the engine's own constants (`lane::FLUSH_EPS`, `lane::REST_EPS`,
/// `lane::silence_frames`), never copies of their values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlushLaw {
    /// The per-word flush threshold: a word below it is zeroed, so the flush moves a word by less
    /// than this.
    pub flush_eps: f64,
    /// The joint threshold: both words below it on an armed frame are zeroed together.
    pub rest_eps: f64,
    /// `N_SILENCE`: frames of exactly zero effect input before a frame is armed.
    pub silence_frames: u64,
}

impl FlushLaw {
    /// The `V`-norm of the largest per-step absolute perturbation of one section's state: the
    /// per-word flush (`< flush_eps` per word) plus the underflow allowance.
    fn per_step(&self) -> f64 {
        R_NORM * core::f64::consts::SQRT_2 * (self.flush_eps + UNDERFLOW_PER_STEP) * SLACK
    }
}

/// Why no bound is stated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TailBoundError {
    /// A section's kernel contraction (rounding included) is not below one.
    NotContracting,
    /// A section's flush stall radius is not below `REST_EPS`: the joint flush cannot be proven to
    /// fire.
    StallAboveRest,
    /// A bound would exceed the longest horizon this module evaluates.
    Horizon,
}

/// A certified tail and exact-rest bound, in samples beyond latency (#1329 D1, D2, Amendment 3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CascadeBound {
    /// `T_decay`, D1's `T` for every input peak at or above [`Self::flush_floor`]: from `N + T`
    /// on the output is below `P * TAIL_FLOOR`.
    pub tail: u64,
    /// `T_rest = max(T_decay, R(P*))`, the tail over every peak: from `N + T_rest` on the output
    /// is below `P * TAIL_FLOOR` for `P >= P*` and exactly zero for `P < P*`.
    pub tail_every_peak: u64,
    /// D2's `R` for an input peak up to the first stated peak (+24 dBFS).
    pub rest_peak: u64,
    /// D2's `R` for an input peak up to the second stated peak (any sanitized input).
    pub rest_any: u64,
    /// `P*`, the flush floor: the smallest peak for which the flush's absolute deviation fits the
    /// tail floor's budget. Below it the tail is proven by exact rest.
    pub flush_floor: f64,
    /// `R(P*)`: exact rest for every input peak at or below [`Self::flush_floor`].
    pub rest_at_flush_floor: u64,
    /// The exact half alone: the first frame `t0` from which the reference's impulse-response
    /// suffix is below `eps / 2` (evidence only). A live bound ([`live_cascade`]) has no
    /// reference split, its decay being propagated directly on the kernel's envelope, so there it
    /// equals [`Self::tail`].
    pub tail_reference: u64,
}

impl CascadeBound {
    /// The bound of an empty cascade (every section disabled): memoryless.
    pub const ZERO: Self = Self {
        tail: 0,
        tail_every_peak: 0,
        rest_peak: 0,
        rest_any: 0,
        flush_floor: 0.0,
        rest_at_flush_floor: 0,
        tail_reference: 0,
    };
}

/// Per-section suprema of the kernel over every word a control history can reach.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvelopeSection {
    /// The kernel contraction (`||A(w)||_V` plus the state rounding) over every word reachable
    /// while a ramp may be in flight.
    pub rho_ramp: f64,
    /// The kernel contraction over every word a settled section can hold.
    pub rho_settled: f64,
    /// `||b||_V` plus the input rounding.
    pub input: f64,
    /// The output row's dual norm plus the output rounding on the state.
    pub output_state: f64,
    /// `|d|` plus the output rounding on the input.
    pub output_input: f64,
}

/// The direct kernel bound of a cascade at zero input: `z_k` bounds section `k`'s state in the
/// `V`-norm, `v_k` the magnitude of its kernel input (`v_1 = 0`), and
///
/// ```text
/// z_k' = rho_k z_k + input_k v_k + F,   v_{k+1} = output_state_k z_k + output_input_k v_k + F
/// ```
///
/// (`F` on the output covers the mix's underflow allowance, which it exceeds by far). Linear in
/// `w = (z_1, ..., z_K, 1)`: `M` is `(K + 1)`-square and non-negative. A section before `from` is
/// at rest: its state stays zero and its output is exactly zero. [`live_cascade`]'s settled
/// systems use the same shape (a non-negative `(K + 1)`-square matrix whose last component is the
/// constant `1`).
struct Propagator {
    k: usize,
    m: Vec<f64>,
}

impl Propagator {
    /// `alpha[i]`: the coefficients of `v_i` on `w`, for `i = 0..=K`.
    fn input_rows(sections: &[EnvelopeSection], from: usize, f: f64) -> Vec<Vec<f64>> {
        let k = sections.len();
        let mut rows = std::vec![std::vec![0.0; k + 1]; k + 1];
        for i in 0..k {
            if i < from {
                continue;
            }
            let (previous, next) = rows.split_at_mut(i + 1);
            let next = &mut next[0];
            for (value, earlier) in next.iter_mut().zip(&previous[i]) {
                *value = sections[i].output_input * earlier;
            }
            next[i] += sections[i].output_state;
            next[k] += f;
        }
        rows
    }

    /// The system with contractions `rho`, sections before `from` at rest, absolute drive `f`.
    fn new(sections: &[EnvelopeSection], rho: &[f64], from: usize, f: f64) -> Self {
        let k = sections.len();
        let n = k + 1;
        let rows = Self::input_rows(sections, from, f);
        let mut m = std::vec![0.0; n * n];
        for i in from..k {
            for j in 0..n {
                m[i * n + j] = sections[i].input * rows[i][j];
            }
            m[i * n + i] += rho[i];
            m[i * n + k] += f;
        }
        m[k * n + k] = 1.0;
        Self { k, m }
    }

    fn apply(&self, z: &[f64]) -> Vec<f64> {
        let n = self.k + 1;
        (0..n)
            .map(|i| (0..n).map(|j| self.m[i * n + j] * z[j]).sum::<f64>() * SLACK)
            .collect()
    }

    fn multiply(&self, other: &Self) -> Self {
        let n = self.k + 1;
        let mut m = std::vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                m[i * n + j] = (0..n)
                    .map(|l| self.m[i * n + l] * other.m[l * n + j])
                    .sum::<f64>()
                    * SLACK;
            }
        }
        Self { k: self.k, m }
    }

    /// `M^steps z`, by repeated squaring; every product is inflated by [`SLACK`].
    fn power_apply(&self, mut steps: u64, z: &[f64]) -> Vec<f64> {
        let mut result = z.to_vec();
        let mut base = Self {
            k: self.k,
            m: self.m.clone(),
        };
        while steps > 0 {
            if steps & 1 == 1 {
                result = base.apply(&result);
            }
            steps >>= 1;
            if steps > 0 {
                base = base.multiply(&base);
            }
        }
        result
    }
}

/// The first `m` with `rho^m (z0 - stall) + stall < limit`, `stall = f / (1 - rho)`, for a
/// section decaying freely from `z0` (`None` when the stall itself is not below `limit`).
fn free_decay_frames(z0: f64, rho: f64, f: f64, limit: f64) -> Option<u64> {
    let stall = f / (1.0 - rho) * SLACK;
    if stall >= limit {
        return None;
    }
    if z0 < limit {
        return Some(0);
    }
    // `rho^m < (limit - stall) / (z0 - stall)`; one frame added for the rounding of `log`.
    let ratio = (limit - stall) / (z0 - stall);
    let frames = crate::log(ratio) / crate::log(rho);
    if !frames.is_finite() || frames >= HORIZON_LIMIT as f64 {
        return Some(HORIZON_LIMIT);
    }
    Some(crate::floor(frames) as u64 + 2)
}

/// Exact rest, section by section, from the kernel state bounds `z` at the input's end (frame
/// `N`): returns `R`, the frames from `N` after which every section is at rest.
///
/// The first `ramp_frames` frames use each section's `rho_ramp`, later frames `rho_settled`. A
/// section `k` rests once its state bound, as per-word magnitudes (`||R^-1|| z_k`), is below
/// `REST_EPS` on an armed frame; the bound adds `N_SILENCE` per section in sequence (#1329
/// Amendment 2). From then on its output is exactly zero and the next section decays freely.
fn rest_frames(
    sections: &[EnvelopeSection],
    z: &[f64],
    law: &FlushLaw,
    ramp_frames: u64,
) -> Result<u64, TailBoundError> {
    let f = law.per_step();
    let k = sections.len();
    let limit = law.rest_eps / R_INV_NORM;
    let ramp: Vec<f64> = sections.iter().map(|s| s.rho_ramp).collect();
    let settled: Vec<f64> = sections.iter().map(|s| s.rho_settled).collect();
    let mut state: Vec<f64> = z.iter().copied().chain([1.0]).collect();
    let mut now = 0_u64;
    for section in 0..k {
        // Bring the bound to the end of the ramp window if it is still ahead.
        if now < ramp_frames {
            let window = Propagator::new(sections, &ramp, section, f);
            state = window.power_apply(ramp_frames - now, &state);
            now = ramp_frames;
        }
        let rho = settled[section];
        let frames = free_decay_frames(state[section], rho, f, limit)
            .ok_or(TailBoundError::StallAboveRest)?;
        let rested_at = now
            .checked_add(frames)
            .and_then(|value| value.checked_add(law.silence_frames))
            .filter(|value| *value < HORIZON_LIMIT)
            .ok_or(TailBoundError::Horizon)?;
        // Every section, this one still driving, propagated to `rested_at`.
        let system = Propagator::new(sections, &settled, section, f);
        state = system.power_apply(rested_at - now, &state);
        state[section] = 0.0;
        now = rested_at;
    }
    Ok(now)
}

// ---- The frequency-aware live cascade (issue #1433) ------------------------------------------

/// Where the recursion words a live section can load lie, read as poles (issue #1433).
///
/// For any words `w = (c1, a2, a3)`, in the `V`-basis
/// `R A(w) R^-1 = (Re p) I + (Im p) J + kappa [[-1, 1], [1, 1]]` exactly, with
/// `Re p = 1 - c1 - a3`, `Im p = 2 sqrt(2) a2 - c1 + a3`, `kappa = c1 - a3 - sqrt(2) a2` and
/// `J = [[0, -1], [1, 0]]`. The matrices `a I + b J` are the complex numbers `a + i b` (their
/// spectral norm is `|a + i b|`, `J^2 = -I`), and the last term has norm `sqrt(2) |kappa|`, so
/// `||A(w)||_V <= |p| + sqrt(2) |kappa|`, and likewise `||I + A||_V <= |1 + p| + sqrt(2) |kappa|`,
/// `||I - A||_V <= |1 - p| + sqrt(2) |kappa|`. All three are affine in the words.
///
/// An exact Butterworth design (`k = sqrt(2)`) has `kappa = 0` and its pole on the circle
/// `|p + i| = sqrt(2)`: the bilinear image of the analog ray `g e^(i 3 pi / 4)`, through `1`, `-1`
/// and `(sqrt(2) - 1) i`. Over a cutoff interval the poles are an arc of it, and the convex hull of
/// the exact designs maps onto the region between that arc and its chord: at `Re p = x` the
/// imaginary part lies in `[0, h(x)]`, `h(x) = sqrt(2 - x^2) - 1`. A word the kernel loads is such
/// a hull point moved by at most a box per word (#1407's ramp allowance, or an `f32` design's own
/// rounding), which moves `Re p`, `Im p` and `kappa` by at most the box's image.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PoleDomain {
    /// A lower bound on `Re p` over the exact designs of the cutoff domain (the top design's).
    pub low: f64,
    /// An upper bound on `Re p` over the exact designs (the lowest cutoff's).
    pub high: f64,
    /// Per recursion word `(c1, a2, a3)`, how far any word the kernel can load lies from the
    /// convex hull of the exact designs (#1407: `E + h`).
    pub ramp_box: [f64; 3],
    /// The same for a settled word, an `f32` design (`h`).
    pub design_box: [f64; 3],
    /// [`state_rounding`]'s `mu_state` over every reachable word.
    pub state_rounding: f64,
    /// `mu_state` over every settled word.
    pub settled_state_rounding: f64,
    /// [`state_rounding`]'s `mu_input` over every reachable word.
    pub input_rounding: f64,
}

/// What [`live_cascade`] bounds: two live sections that share one domain (issue #1433).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiveCascade {
    /// Both sections' suprema over every reachable word ([`EnvelopeSection`]).
    pub envelope: EnvelopeSection,
    /// Where both sections' recursion words lie.
    pub poles: PoleDomain,
    /// The supremum of `||(m1, m2)||_V*` ([`v_dual_norm`]) over the first section's mix words:
    /// its output row on the state is `1/2 (m1, m2) (I + A)`.
    pub first_mix_row: f64,
    /// [`output_rounding`]'s `omega_state` over the first section's words: the output mix's
    /// rounding relative to the state.
    pub first_output_rounding: f64,
}

/// One zone of pole positions: every reachable word whose `Re p` lies in `[low, high]`, with its
/// certified constants and the state and potential bounds of [`live_zones`] (issue #1433).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PoleZone {
    /// The zone's range of `Re p`.
    pub low: f64,
    /// The zone's range of `Re p`.
    pub high: f64,
    /// At least `||A(w)||_V + mu_state` over the zone's words.
    pub contraction: f64,
    /// At least `||I + A(w)||_V` over the zone's words.
    pub sum_norm: f64,
    /// At least `||b(w)||_V + mu_input` over the zone's words (`b = (I - A) e2`).
    pub input: f64,
    /// The zone's constants over settled words, `None` when no design lies in it.
    pub settled: Option<SettledPoleZone>,
    /// `Phi`: the first section's state while its words are in this zone is at most
    /// `state * gain * peak + flush_state * F` (`F` [`FlushLaw`]'s per-step perturbation).
    pub state: f64,
    /// `Phi`'s part per unit of `F`.
    pub flush_state: f64,
    /// `Psi`, the potential: `potential >= q + rho * potential'` for every neighbouring zone
    /// (`q` the zone's [`Self::sum_norm`], zero for a direct zone).
    pub potential: f64,
    /// Whether the zone's output is charged directly (its pole lies at or right of `Re p = 0`)
    /// rather than through the potential.
    pub direct: bool,
    /// The zones a frame can move to from this one: indices `first_neighbour..=last_neighbour`.
    pub first_neighbour: usize,
    /// See [`Self::first_neighbour`].
    pub last_neighbour: usize,
}

/// A zone's constants over the settled words (`f32` designs) in it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SettledPoleZone {
    /// At least `||A(w)||_V + mu_state` over the settled words.
    pub contraction: f64,
    /// At least `||I + A(w)||_V` over the settled words.
    pub sum_norm: f64,
    /// At least `||A(w)^2 - I||_V` over the settled words.
    pub square_norm: f64,
}

/// The zones of a live section's pole domain and the suprema [`live_cascade`] reads (issue #1433).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveZones {
    /// The zones, in increasing `Re p`, covering every reachable word's `Re p`.
    pub zones: Vec<PoleZone>,
    /// The largest change of `Re p` from one frame's word to the next's, under any history.
    pub step: f64,
    /// `sup Psi Phi` over the zones: per unit of `gain * peak`, and per unit of `F`.
    pub potential_state: [f64; 2],
    /// `sup Psi' beta` over every move from a zone to a neighbour (per unit of `gain * peak`) and
    /// `sup Psi` (per unit of `F`): the potential's charge per frame.
    pub charge: [f64; 2],
    /// `sup q Phi` over the direct zones, per unit of `gain * peak` and of `F`.
    pub direct_output: [f64; 2],
    /// `sup Phi` over the zones, per unit of `gain * peak` and of `F`.
    pub largest_state: [f64; 2],
    /// `sup q` over the zones.
    pub largest_sum_norm: f64,
}

impl LiveZones {
    /// The zone that holds a word whose pole has real part `re` (the first whose closed range
    /// contains it), or `None` when no reachable word has that real part.
    #[must_use]
    pub fn zone_of(&self, re: f64) -> Option<&PoleZone> {
        self.zones
            .iter()
            .find(|zone| zone.low <= re && re <= zone.high)
    }
}

/// The real part of a word set's pole, `Re p = 1 - c1 - a3` ([`PoleDomain`]).
#[must_use]
pub fn pole_real(words: &SvfWords) -> f64 {
    1.0 - words.c1 - words.a3
}

/// The first zone's width at either end of the domain; widths then grow by [`ZONE_GROWTH`] up to
/// [`ZONE_WIDEST`]. The zones only partition the domain: any partition is sound, a finer one is
/// usually tighter (#1433 attempt 1's prototype put `R` 121 frames above a partition of 629 zones
/// instead of 146, at 44.1 kHz).
const ZONE_FIRST: f64 = 1.0e-6;
/// See [`ZONE_FIRST`].
const ZONE_GROWTH: f64 = 1.5;
/// See [`ZONE_FIRST`].
const ZONE_WIDEST: f64 = 0.02;
/// The relative margin of the potential's and the state bound's starting values above their
/// own-zone fixed points ([`live_zones`]), far above the few `f64` roundings each update makes.
const FIXED_POINT_MARGIN: f64 = 1.0 / 1_099_511_627_776.0;
/// The most passes [`live_zones`] makes before it gives up: no bound above it is stated.
const ZONE_PASSES: usize = 4096;

/// `h(x) = sqrt(2 - x^2) - 1`, the exact pole circle's height at `Re p = x`, in a form without
/// cancellation (`|x| < 1`).
fn arc_height(x: f64) -> f64 {
    (1.0 - x) * (1.0 + x) / (crate::sqrt(2.0 - x * x) + 1.0)
}

/// The largest `|p|` over the exact designs' hull at `Re p = x`: `sqrt(x^2 + h(x)^2)`.
fn hull_radius(x: f64) -> f64 {
    let h = arc_height(x);
    crate::sqrt(x * x + h * h)
}

/// The largest `|1 + p|` over the hull at `Re p = x`, increasing in `x`.
fn hull_sum(x: f64) -> f64 {
    let h = arc_height(x);
    crate::sqrt((1.0 + x) * (1.0 + x) + h * h)
}

/// The largest `|1 - p|` over the hull at `Re p = x`, decreasing in `x`.
fn hull_difference(x: f64) -> f64 {
    let h = arc_height(x);
    crate::sqrt((1.0 - x) * (1.0 - x) + h * h)
}

/// The image of a per-word box `|e_i| <= box_[i]` on the pole: the largest change of `Re p`, of
/// `|p|` and of `kappa` ([`PoleDomain`]).
fn pole_shift(box_: [f64; 3]) -> (f64, f64, f64) {
    let sqrt2 = core::f64::consts::SQRT_2;
    let real = (box_[0] + box_[2]) * SLACK;
    let imaginary = 2.0 * sqrt2 * box_[1] + box_[0] + box_[2];
    let shift = crate::sqrt(real * real + imaginary * imaginary) * SLACK;
    let kappa = (box_[0] + box_[2] + sqrt2 * box_[1]) * SLACK;
    (real, shift, kappa)
}

/// The zone boundaries: from `first` up to `0` and from `last` down to `0`, the widths growing
/// from [`ZONE_FIRST`] by [`ZONE_GROWTH`] up to [`ZONE_WIDEST`] (`first < 0 < last` clamps the
/// split point into the domain otherwise).
fn zone_boundaries(first: f64, last: f64) -> Vec<f64> {
    let middle = 0.0_f64.clamp(first, last);
    let mut left = std::vec![first];
    let (mut x, mut width) = (first, ZONE_FIRST);
    loop {
        x += width;
        width = (width * ZONE_GROWTH).min(ZONE_WIDEST);
        if x >= middle {
            break;
        }
        left.push(x);
    }
    let mut right = std::vec![last];
    let (mut x, mut width) = (last, ZONE_FIRST);
    loop {
        x -= width;
        width = (width * ZONE_GROWTH).min(ZONE_WIDEST);
        if x <= middle {
            break;
        }
        right.push(x);
    }
    if middle > first && middle < last {
        left.push(middle);
    }
    left.extend(right.into_iter().rev());
    left.dedup();
    left
}

/// The zones of a live section's pole domain, with the state invariant `Phi` and the potential
/// `Psi` (issue #1433; `docs/derivations/1329-input-section-tail-and-rest.md`, "#1433").
///
/// * **Zones.** `Re p` of every reachable word lies in `[low - eta, high + eta]` (`eta` the ramp
///   box's image). A zone's constants hold for every word whose `Re p` lies in it: such a word is a
///   hull point at `Re p` within `eta` of the zone, moved by the box, so `|p|` is at most the hull
///   radius at the zone's extended end farthest from `0` plus the box's image, and so on
///   ([`PoleDomain`]). `contraction`, `input` are also capped by the envelope's suprema.
/// * **Moves.** From one frame to the next a word moves by one ramp step (#1407: a fraction
///   `1/64` of a target minus a reachable word, `c1` and `a3` each within a few `f32` roundings of
///   it), holds, or is replaced with zero state (a rule-3 enable, a completed disable, a reset), so
///   `Re p` moves by at most [`LiveZones::step`]: zones within it of each other are neighbours.
/// * **`Phi`.** The least solution, per unit of `gain * peak` and of `F`, of
///   `Phi_k >= rho_i Phi_i + beta_i` for every neighbour `i` of `k` (itself included), computed
///   upward from each zone's own fixed point; the first section's state then never exceeds its
///   zone's `Phi` (induction over frames; a reset only lowers it).
/// * **`Psi`.** The least solution of `Psi_k >= q_k + rho_k Psi_i` for every neighbour `i`, with
///   `q_k` zero for a direct zone (`Re p >= 0`). Then for every frame
///   `q_k E_j [k not direct] <= Psi_k E_j - Psi_k' E_(j+1) + Psi_k' (beta_k |x_j| + F)`: the
///   first section's output mass telescopes against its state.
///
/// Every update is made an upper bound by `1 + 4 u64` and checked again when the iteration stops,
/// so the stated inequalities hold for the computed values.
///
/// # Errors
///
/// [`TailBoundError::NotContracting`] when a zone does not contract or the domain is not inside
/// `(-1, 1)`; [`TailBoundError::Horizon`] when the iteration does not settle within 4,096
/// passes.
pub fn live_zones(live: &LiveCascade) -> Result<LiveZones, TailBoundError> {
    let poles = &live.poles;
    let envelope = &live.envelope;
    let interior = |x: f64| x > -1.0 && x < 1.0;
    if !(interior(poles.low) && interior(poles.high) && poles.low <= poles.high) {
        return Err(TailBoundError::NotContracting);
    }
    let sqrt2 = core::f64::consts::SQRT_2;
    let (ramp_real, ramp_shift, ramp_kappa) = pole_shift(poles.ramp_box);
    let (design_real, design_shift, design_kappa) = pole_shift(poles.design_box);
    let ramp_norm = (ramp_shift + sqrt2 * ramp_kappa) * SLACK;
    let design_norm = (design_shift + sqrt2 * design_kappa) * SLACK;
    // A margin far above the rounding of `x +- eta` (`|x| <= 1`).
    let pad = 1.0e-15;
    let first = poles.low - ramp_real - pad;
    let last = poles.high + ramp_real + pad;
    let boundaries = zone_boundaries(first, last);
    // `Re p` of consecutive words: one ramp step of `c1` and `a3` together is at most `1/64` of the
    // distance between a design and a reachable word, plus each word's rounding.
    let step = ((poles.high - poles.low + 2.0 * ramp_real) / 64.0 + 32.0 * U) * SLACK;
    let mut zones = Vec::with_capacity(boundaries.len());
    for window in boundaries.windows(2) {
        let (low, high) = (window[0], window[1]);
        let reach = |eta: f64| {
            let from = (low - eta - pad).max(poles.low);
            let to = (high + eta + pad).min(poles.high);
            (from <= to).then_some((from, to))
        };
        let Some((from, to)) = reach(ramp_real) else {
            continue;
        };
        let radius = hull_radius(from).max(hull_radius(to));
        let contraction =
            ((radius + ramp_norm + poles.state_rounding) * SLACK).min(envelope.rho_ramp);
        let sum_norm = (hull_sum(to) + ramp_norm) * SLACK;
        let input = ((hull_difference(from) + ramp_norm + poles.input_rounding) * SLACK)
            .min(envelope.input);
        let settled = reach(design_real).map(|(from, to)| {
            let radius = hull_radius(from).max(hull_radius(to));
            // `||A^2 - I|| <= |p^2 - 1| + 2 |p| sqrt(2) |kappa| + 2 kappa^2`, with `p` a hull
            // point `p0` moved by at most `d`: `|p^2 - 1| <= |1 - p0| |1 + p0| + 2 d + d^2`.
            let square_norm = (hull_difference(from) * hull_sum(to)
                + 2.0 * design_shift
                + design_shift * design_shift
                + 2.0 * sqrt2 * design_kappa * (1.0 + design_shift)
                + 2.0 * design_kappa * design_kappa)
                * SLACK;
            SettledPoleZone {
                contraction: ((radius + design_norm + poles.settled_state_rounding) * SLACK)
                    .min(envelope.rho_settled),
                sum_norm: (hull_sum(to) + design_norm) * SLACK,
                square_norm,
            }
        });
        // The own-zone fixed points below need `rho (1 + margin) < 1`.
        let contracts = |rho: f64| {
            (rho * (1.0 + FIXED_POINT_MARGIN)).partial_cmp(&1.0) == Some(core::cmp::Ordering::Less)
        };
        if !contracts(contraction) {
            return Err(TailBoundError::NotContracting);
        }
        zones.push(PoleZone {
            low,
            high,
            contraction,
            sum_norm,
            input,
            settled,
            state: 0.0,
            flush_state: 0.0,
            potential: 0.0,
            direct: low >= 0.0,
            first_neighbour: 0,
            last_neighbour: 0,
        });
    }
    let count = zones.len();
    for k in 0..count {
        let (low, high) = (zones[k].low, zones[k].high);
        let first = zones
            .iter()
            .position(|zone| zone.high >= low - step)
            .expect("a zone is its own neighbour");
        let last = zones
            .iter()
            .rposition(|zone| zone.low <= high + step)
            .expect("a zone is its own neighbour");
        zones[k].first_neighbour = first;
        zones[k].last_neighbour = last;
    }
    // `Phi`, upward from each zone's own fixed point (with the margin), until no update.
    let own = |drive: f64, rho: f64| {
        drive * (1.0 + FIXED_POINT_MARGIN) / (1.0 - rho * (1.0 + FIXED_POINT_MARGIN)) * STEP_UP
    };
    let mut state: Vec<f64> = zones.iter().map(|z| own(z.input, z.contraction)).collect();
    let mut flush: Vec<f64> = zones.iter().map(|z| own(1.0, z.contraction)).collect();
    let mut settled = false;
    for _ in 0..ZONE_PASSES {
        let mut changed = false;
        for k in 0..count {
            let zone = zones[k];
            for i in zone.first_neighbour..=zone.last_neighbour {
                let from = zones[i];
                let relative = (from.contraction * state[i] + from.input) * STEP_UP;
                let absolute = (from.contraction * flush[i] + 1.0) * STEP_UP;
                if relative > state[k] {
                    state[k] = relative;
                    changed = true;
                }
                if absolute > flush[k] {
                    flush[k] = absolute;
                    changed = true;
                }
            }
        }
        if !changed {
            settled = true;
            break;
        }
    }
    // `Psi`, upward from each zone's own fixed point, until no update.
    let reward = |zone: &PoleZone| if zone.direct { 0.0 } else { zone.sum_norm };
    let mut potential: Vec<f64> = zones
        .iter()
        .map(|z| own(reward(z), z.contraction))
        .collect();
    let mut potential_settled = false;
    for _ in 0..ZONE_PASSES {
        let mut changed = false;
        for k in 0..count {
            let zone = zones[k];
            let best = potential[zone.first_neighbour..=zone.last_neighbour]
                .iter()
                .fold(0.0_f64, |best, value| best.max(*value));
            let candidate = (reward(&zone) + zone.contraction * best) * STEP_UP;
            if candidate > potential[k] {
                potential[k] = candidate;
                changed = true;
            }
        }
        if !changed {
            potential_settled = true;
            break;
        }
    }
    if !(settled && potential_settled) {
        return Err(TailBoundError::Horizon);
    }
    // Check every stated inequality on the computed values.
    for k in 0..count {
        let zone = zones[k];
        for i in zone.first_neighbour..=zone.last_neighbour {
            let from = zones[i];
            let holds = state[k] >= (from.contraction * state[i] + from.input) * STEP_UP
                && flush[k] >= (from.contraction * flush[i] + 1.0) * STEP_UP
                && potential[k] >= (reward(&zone) + zone.contraction * potential[i]) * STEP_UP;
            if !holds {
                return Err(TailBoundError::Horizon);
            }
        }
    }
    let mut potential_state = [0.0_f64; 2];
    let mut charge = [0.0_f64; 2];
    let mut direct_output = [0.0_f64; 2];
    let mut largest_state = [0.0_f64; 2];
    let mut largest_sum_norm = 0.0_f64;
    for k in 0..count {
        let zone = &mut zones[k];
        zone.state = state[k];
        zone.flush_state = flush[k];
        zone.potential = potential[k];
        potential_state[0] = potential_state[0].max(potential[k] * state[k] * SLACK);
        potential_state[1] = potential_state[1].max(potential[k] * flush[k] * SLACK);
        for value in &potential[zone.first_neighbour..=zone.last_neighbour] {
            charge[0] = charge[0].max(value * zone.input * SLACK);
        }
        charge[1] = charge[1].max(potential[k]);
        if zone.direct {
            direct_output[0] = direct_output[0].max(zone.sum_norm * state[k] * SLACK);
            direct_output[1] = direct_output[1].max(zone.sum_norm * flush[k] * SLACK);
        }
        largest_state[0] = largest_state[0].max(state[k]);
        largest_state[1] = largest_state[1].max(flush[k]);
        largest_sum_norm = largest_sum_norm.max(zone.sum_norm);
    }
    Ok(LiveZones {
        zones,
        step,
        potential_state,
        charge,
        direct_output,
        largest_state,
        largest_sum_norm,
    })
}

/// The second section's bound through the window `N ..= N + ramp_frames` (a ramp may still be in
/// flight), for one input scale `g = gain * peak` and flush drive `f`.
struct LiveWindow {
    /// `sigma` at `N + ramp_frames + 1`: the second section's state bound.
    state: f64,
    /// The first section's state bound at `N + ramp_frames`.
    energy: f64,
    /// The cascade's output bound at each window frame.
    outputs: Vec<f64>,
}

/// The settled-phase terms of one zone a first section's settled design can lie in (or of the
/// first section at the identity).
#[derive(Clone, Copy, PartialEq)]
struct SettledTerms {
    /// The first section's settled contraction `r`.
    contraction: f64,
    /// `c_y`: its output is at most `c_y H + F`.
    output: f64,
    /// `c_d`: its output's frame-to-frame change is at most `c_d H + (gamma + 2 + omega) F`.
    change: f64,
    /// `Phi` per unit of `gain * peak` and of `F`.
    state: [f64; 2],
}

impl SettledTerms {
    /// Whether every term of `self` is at least the same term of `other`. The settled system is
    /// non-negative and increasing in every term (its entries, its start and its rows are), so a
    /// bound computed with `self`'s terms holds for `other`'s zone too: its first section rests no
    /// later than `self`'s rest bound and the second section's bound is larger at every frame.
    fn covers(&self, other: &Self) -> bool {
        self.contraction >= other.contraction
            && self.output >= other.output
            && self.change >= other.change
            && self.state[0] >= other.state[0]
            && self.state[1] >= other.state[1]
    }
}

/// A non-negative 4x4 matrix, row major.
type Square4 = [[f64; 4]; 4];

/// `m v`, each component inflated by [`SLACK`] (every operand is non-negative).
fn apply4(m: &Square4, v: &[f64; 4]) -> [f64; 4] {
    core::array::from_fn(|i| {
        (m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2] + m[i][3] * v[3]) * SLACK
    })
}

/// `a b`, each entry inflated by [`SLACK`].
fn multiply4(a: &Square4, b: &Square4) -> Square4 {
    core::array::from_fn(|i| {
        core::array::from_fn(|j| {
            (a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j]) * SLACK
        })
    })
}

/// `row . v`, inflated by [`SLACK`].
fn dot4(row: &[f64; 4], v: &[f64; 4]) -> f64 {
    (row[0] * v[0] + row[1] * v[1] + row[2] * v[2] + row[3] * v[3]) * SLACK
}

/// The powers `M^(2^j)` of one settled system, squared on demand up to `j < 26`: `M^m v` for
/// any `m` below [`HORIZON_LIMIT`] by one product per set bit (powers of one matrix commute, and
/// every product is rounded up, so the order does not matter for an upper bound). Only the powers a
/// call needs are formed: the high powers of a contraction near `1` underflow, and subnormal
/// arithmetic is slow.
struct Powers4 {
    table: core::cell::RefCell<Vec<Square4>>,
}

impl Powers4 {
    fn new(m: Square4) -> Self {
        Self {
            table: core::cell::RefCell::new(std::vec![m]),
        }
    }

    /// `M^steps v`; `steps` below [`HORIZON_LIMIT`].
    fn apply(&self, steps: u64, v: &[f64; 4]) -> [f64; 4] {
        let needed = (u64::BITS - steps.leading_zeros()) as usize;
        let mut table = self.table.borrow_mut();
        while table.len() < needed {
            let last = table[table.len() - 1];
            table.push(multiply4(&last, &last));
        }
        let mut result = *v;
        for (bit, power) in table.iter().enumerate().take(needed) {
            if steps & (1 << bit) != 0 {
                result = apply4(power, &result);
            }
        }
        result
    }
}

/// The bound machinery of [`live_cascade`] for one cascade.
struct LiveBound<'a> {
    live: &'a LiveCascade,
    zones: &'a LiveZones,
    terms: Vec<SettledTerms>,
    ramp_frames: u64,
    law: &'a FlushLaw,
    cap: f64,
}

impl<'a> LiveBound<'a> {
    fn new(live: &'a LiveCascade, zones: &'a LiveZones, law: &'a FlushLaw, ramp: u64) -> Self {
        let half_row = 0.5 * live.first_mix_row;
        let omega = live.first_output_rounding;
        let mu = live.poles.settled_state_rounding;
        let gamma = live.envelope.output_state;
        let mut every: Vec<SettledTerms> = zones
            .zones
            .iter()
            .filter_map(|zone| {
                let settled = zone.settled?;
                let r = settled.contraction;
                Some(SettledTerms {
                    contraction: r,
                    output: (half_row * settled.sum_norm + omega) * SLACK,
                    change: (half_row * settled.square_norm + gamma * mu + omega * (1.0 + r))
                        * SLACK,
                    state: [zone.state, zone.flush_state],
                })
            })
            .collect();
        // The first section at the identity: its output is the (zero) input.
        every.push(SettledTerms {
            contraction: 0.0,
            output: 0.0,
            change: 0.0,
            state: [0.0, 0.0],
        });
        // Consecutive zones whose settled contractions lie in one quarter octave of `1 - r` (the
        // same integer part of `4 log2(1 - r)`) share one system, with the largest of each term
        // (it covers each of them).
        let quarter_octave = |terms: &SettledTerms| {
            let gap = 1.0 - terms.contraction;
            if gap > 0.0 {
                (4.0 * crate::log2(gap)) as i32
            } else {
                i32::MIN
            }
        };
        let mut grouped: Vec<SettledTerms> = Vec::new();
        let mut previous = None;
        for terms in &every {
            let band = quarter_octave(terms);
            match grouped.last_mut() {
                Some(last) if previous == Some(band) => {
                    last.contraction = last.contraction.max(terms.contraction);
                    last.output = last.output.max(terms.output);
                    last.change = last.change.max(terms.change);
                    last.state[0] = last.state[0].max(terms.state[0]);
                    last.state[1] = last.state[1].max(terms.state[1]);
                }
                _ => grouped.push(*terms),
            }
            previous = Some(band);
        }
        let every = grouped;
        // Only the zones no other zone covers need their own system.
        let mut terms: Vec<SettledTerms> = Vec::new();
        for (index, candidate) in every.iter().enumerate() {
            let covered = every.iter().enumerate().any(|(other, terms)| {
                other != index
                    && terms.covers(candidate)
                    && (!candidate.covers(terms) || other < index)
            });
            if !covered {
                terms.push(*candidate);
            }
        }
        Self {
            live,
            zones,
            terms,
            ramp_frames: ramp,
            law,
            cap: R_NORM * core::f64::consts::SQRT_2 * F32_MAX,
        }
    }

    /// The window, frame by frame, from the bounds at `N`:
    /// `sigma <= iota (1/2 m (V + K + K_L) + D + Omega) + F_sum` with the potential's telescoped
    /// mass `V`, its per-frame charges `K`, the direct zones' output `K_L`, the feedthrough `D`,
    /// the output rounding `Omega` and the second section's own flush `F_sum`, each a weighted sum
    /// with the second section's contraction as the weight's ratio.
    fn window(&self, g: f64, f: f64) -> LiveWindow {
        let envelope = &self.live.envelope;
        let zones = self.zones;
        let rho = envelope.rho_ramp;
        let one_minus = 1.0 - rho;
        let half_row = 0.5 * self.live.first_mix_row;
        let omega = self.live.first_output_rounding;
        let potential = (zones.potential_state[0] * g + zones.potential_state[1] * f) * SLACK;
        let direct = (zones.direct_output[0] * g + zones.direct_output[1] * f) * SLACK;
        let mut energy = (zones.largest_state[0] * g + zones.largest_state[1] * f) * SLACK;
        let mut charge = (zones.charge[0] * g + zones.charge[1] * f) / one_minus * SLACK;
        let mut direct_sum = direct / one_minus * SLACK;
        let mut feedthrough = (envelope.output_input * g + f) / one_minus * SLACK;
        let mut rounding = omega * energy / one_minus * SLACK;
        let mut flush_sum = f / one_minus * SLACK;
        let output_coefficient = (half_row * zones.largest_sum_norm + omega) * SLACK;
        let state =
            |charge: f64, direct_sum: f64, feedthrough: f64, rounding: f64, flush_sum: f64| {
                ((envelope.input
                    * (half_row * (potential + charge + direct_sum) + feedthrough + rounding)
                    + flush_sum)
                    * SLACK)
                    .min(self.cap)
            };
        let mut outputs = Vec::with_capacity(self.ramp_frames as usize + 1);
        for frame in 0..=self.ramp_frames {
            let sigma = state(charge, direct_sum, feedthrough, rounding, flush_sum);
            outputs.push(
                (envelope.output_state * sigma
                    + envelope.output_input * (output_coefficient * energy + f)
                    + f)
                    * SLACK,
            );
            charge = (rho * charge + zones.charge[1] * f) * SLACK;
            direct_sum = (rho * direct_sum + direct) * SLACK;
            feedthrough = (rho * feedthrough + f) * SLACK;
            rounding = (rho * rounding + omega * energy) * SLACK;
            flush_sum = (rho * flush_sum + f) * SLACK;
            if frame < self.ramp_frames {
                energy = (rho * energy + f) * SLACK;
            }
        }
        LiveWindow {
            state: state(charge, direct_sum, feedthrough, rounding, flush_sum),
            energy: energy.min(self.cap),
            outputs,
        }
    }

    /// The settled phase from `n0 = N + ramp_frames + 1` for one zone, as the non-negative system
    /// on `(tau_n, H_(n-1), X_n, 1)`:
    ///
    /// ```text
    /// tau' = rho tau + (rho c_d + mu c_y + mu_x c_y r) H + a_F F
    /// H'   = r H + F
    /// X'   = max(rho, r) X
    /// ```
    ///
    /// `tau = sigma - e2 y_prev` (the second section's deviation from its input's DC state), `H`
    /// the first section's state bound, `X` the joint flush's one-off terms. Returns the system,
    /// the row of `sigma` and the row of the cascade's output.
    fn settled_system(&self, terms: &SettledTerms, f: f64) -> (Square4, [f64; 4], [f64; 4]) {
        let envelope = &self.live.envelope;
        let rho = envelope.rho_settled;
        let mu = self.live.poles.settled_state_rounding;
        let mu_x = self.live.poles.input_rounding;
        let omega = self.live.first_output_rounding;
        let gamma = envelope.output_state;
        let r = terms.contraction;
        let (c_y, c_d) = (terms.output, terms.change);
        let a_h = (rho * c_d + mu * c_y + mu_x * c_y * r) * SLACK;
        let a_f = (rho * (gamma + 2.0 + omega) + mu + mu_x * (c_y + 1.0) + 1.0) * SLACK;
        let stall = f / (1.0 - r) * SLACK;
        let m = [
            [rho, a_h, 0.0, a_f * f * SLACK],
            [0.0, r, 0.0, f],
            [0.0, 0.0, rho.max(r), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let constant = (f + 2.0 * (c_y * stall + f)) * SLACK;
        let state_row = [1.0, c_y, 1.0, constant];
        let (out_state, out_input) = (envelope.output_state, envelope.output_input);
        let output_row = [
            out_state,
            (out_state * c_y + out_input * c_y * r) * SLACK,
            out_state,
            (out_state * constant + out_input * (c_y * f + f) + f) * SLACK,
        ];
        (m, state_row, output_row)
    }

    /// The settled phase's state at `n0` for one zone: `tau <= sigma + |y|`, `H` the smaller of
    /// the window's state bound and the zone's `Phi`, `X = 2 c_y H`.
    fn settled_start(&self, terms: &SettledTerms, window: &LiveWindow, g: f64, f: f64) -> [f64; 4] {
        let h = window
            .energy
            .min((terms.state[0] * g + terms.state[1] * f) * SLACK)
            .min(self.cap);
        [
            (window.state + terms.output * h + f) * SLACK,
            h,
            2.0 * terms.output * h * SLACK,
            1.0,
        ]
    }

    /// `R` for one input scale `g = gain * peak`: the first section rests (its state bound below
    /// `REST_EPS` per word, plus `N_SILENCE`), then the second, from its bound at that frame.
    fn rest(&self, g: f64) -> Result<u64, TailBoundError> {
        let mut worst = 0_u64;
        for group in self.group_rests(g)? {
            worst = worst.max(group.rested);
        }
        Ok(worst)
    }

    /// [`Self::rest`] per zone group, with the quantities it reads.
    fn group_rests(&self, g: f64) -> Result<Vec<LiveGroupRest>, TailBoundError> {
        let f = self.law.per_step();
        let limit = self.law.rest_eps / R_INV_NORM;
        let rho = self.live.envelope.rho_settled;
        let window = self.window(g, f);
        let start = self.ramp_frames + 1;
        let mut groups = Vec::with_capacity(self.terms.len());
        for terms in &self.terms {
            let u0 = self.settled_start(terms, &window, g, f);
            let first = free_decay_frames(u0[1], terms.contraction, f, limit)
                .ok_or(TailBoundError::StallAboveRest)?;
            let first_rested = self
                .ramp_frames
                .checked_add(first)
                .and_then(|value| value.checked_add(self.law.silence_frames))
                .filter(|value| *value < HORIZON_LIMIT)
                .ok_or(TailBoundError::Horizon)?;
            let (system, state_row, _) = self.settled_system(terms, f);
            let at = Powers4::new(system).apply(first_rested - start, &u0);
            let sigma = dot4(&state_row, &at).min(self.cap);
            let second =
                free_decay_frames(sigma, rho, f, limit).ok_or(TailBoundError::StallAboveRest)?;
            let rested = first_rested
                .checked_add(second)
                .and_then(|value| value.checked_add(self.law.silence_frames))
                .filter(|value| *value < HORIZON_LIMIT)
                .ok_or(TailBoundError::Horizon)?;
            groups.push(LiveGroupRest {
                contraction: terms.contraction,
                output: terms.output,
                change: terms.change,
                window_state: window.state,
                energy: u0[1],
                one_off: u0[2],
                first_rested,
                second_state: sigma,
                rested,
            });
        }
        Ok(groups)
    }

    /// A frame `m` from which `row . M^m u` stays below `limit`, for a system without constant
    /// drive.
    ///
    /// First a frame `m_f` at which the computed `w >= M^m_f u` satisfies `M w <= w` (checked
    /// with the product rounded up): then `M^j w` is non-increasing in `j` and bounds every later
    /// frame (`M` is non-negative). Whether a frame passes is found by an exponential search and a
    /// bisection that keeps a passing upper end. If the output is already below `limit` there,
    /// `m_f` is stated; otherwise the first `j` with `row . M^j w < limit`, found the same way,
    /// is added. A computed value below `limit` bounds the exact one, which bounds every later
    /// one, so the search needs no monotonicity of the computed values.
    fn settled_decay(
        m: &Square4,
        powers: &Powers4,
        row: &[f64; 4],
        u: [f64; 4],
        limit: f64,
    ) -> Result<u64, TailBoundError> {
        let falling = |state: &[f64; 4]| {
            apply4(m, state)
                .iter()
                .zip(state)
                .all(|(next, now)| next <= now)
        };
        // The first passing frame of a search over `0..HORIZON_LIMIT`: `test(0)` first, then
        // doubling, then bisection keeping a passing upper end.
        let search = |test: &dyn Fn(u64) -> bool| -> Result<u64, TailBoundError> {
            if test(0) {
                return Ok(0);
            }
            let (mut low, mut high) = (0_u64, 1_u64);
            while !test(high) {
                low = high;
                high *= 2;
                if high >= HORIZON_LIMIT {
                    return Err(TailBoundError::Horizon);
                }
            }
            while high - low > 1 {
                let middle = low + (high - low) / 2;
                if test(middle) {
                    high = middle;
                } else {
                    low = middle;
                }
            }
            Ok(high)
        };
        let settle = search(&|steps| falling(&powers.apply(steps, &u)))?;
        let w = powers.apply(settle, &u);
        let after = search(&|steps| dot4(row, &powers.apply(steps, &w)) < limit)?;
        settle
            .checked_add(after)
            .filter(|value| *value < HORIZON_LIMIT)
            .ok_or(TailBoundError::Horizon)
    }

    /// `T_decay`: the first frame from which the relative output bound (per unit of the peak,
    /// without the flush) stays below `TAIL_FLOOR / 2`, over the window and every zone.
    fn tail(&self, gain: f64) -> Result<u64, TailBoundError> {
        let limit = TAIL_FLOOR / 2.0;
        let window = self.window(gain, 0.0);
        let mut tail = window
            .outputs
            .iter()
            .rposition(|value| *value >= limit)
            .map_or(0, |frame| frame as u64 + 1);
        let start = self.ramp_frames + 1;
        for terms in &self.terms {
            // Without the flush the constant component is unused; it is zero so that every
            // component can fall.
            let mut u0 = self.settled_start(terms, &window, gain, 0.0);
            u0[3] = 0.0;
            let (system, _, output_row) = self.settled_system(terms, 0.0);
            let powers = Powers4::new(system);
            // A zone already falling and below the limit at the tail found so far cannot raise
            // it (from there on its bound is non-increasing).
            if let Some(pivot) = tail.checked_sub(start) {
                let at = powers.apply(pivot, &u0);
                let next = apply4(&system, &at);
                if next.iter().zip(&at).all(|(next, now)| next <= now)
                    && dot4(&output_row, &at) < limit
                {
                    continue;
                }
            }
            let after = Self::settled_decay(&system, &powers, &output_row, u0, limit)?;
            if after > 0 {
                tail = tail.max(start + after);
            }
        }
        Ok(tail)
    }

    /// The flush's part of the output bound from frame `tail` on (per unit `F`, then scaled): the
    /// window's from there, and per zone the settled system's fixed point plus the decaying part
    /// of its start (`u_n = U + M_h^m (u0 - U) <= U + M_h^m u0`, `M_h` the system without its
    /// constant).
    fn stall(&self, tail: u64) -> Result<f64, TailBoundError> {
        let f = self.law.per_step();
        let window = self.window(0.0, f);
        let mut stall = window
            .outputs
            .iter()
            .skip(tail as usize)
            .fold(0.0_f64, |sup, value| sup.max(*value));
        let start = self.ramp_frames + 1;
        let rho = self.live.envelope.rho_settled;
        for terms in &self.terms {
            let mut u0 = self.settled_start(terms, &window, 0.0, f);
            u0[3] = 0.0;
            let (homogeneous, _, output_row) = self.settled_system(terms, 0.0);
            let (driven, _, driven_row) = self.settled_system(terms, f);
            // The fixed point `U`, rounded up.
            let h_star = f / (1.0 - terms.contraction) * SLACK;
            let tau_star = (driven[0][1] * h_star + driven[0][3]) / (1.0 - rho) * SLACK;
            let fixed = dot4(&driven_row, &[tau_star, h_star, 0.0, 1.0]);
            let mut current = Powers4::new(homogeneous).apply(tail.saturating_sub(start), &u0);
            let mut peak = 0.0_f64;
            let mut frames = 0_u64;
            loop {
                peak = peak.max(dot4(&output_row, &current));
                let next = apply4(&homogeneous, &current);
                if next.iter().zip(&current).all(|(next, now)| next <= now) {
                    break;
                }
                current = next;
                frames += 1;
                if frames >= HORIZON_LIMIT {
                    return Err(TailBoundError::Horizon);
                }
            }
            stall = stall.max(fixed + peak);
        }
        Ok(stall * SLACK)
    }
}

/// One settled zone group of [`live_cascade`]'s rest at one input scale, with the quantities the
/// rest reads (evidence for the derivation's independent recomputation; issue #1433).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiveGroupRest {
    /// The group's settled contraction `r` (the largest of its zones').
    pub contraction: f64,
    /// `c_y`: the first section's settled output is at most `c_y H + F`.
    pub output: f64,
    /// `c_d`: the first section's settled output change is at most
    /// `c_d H + (gamma + 2 + omega) F`.
    pub change: f64,
    /// The second section's state bound at `N + ramp_frames + 1`, from the window (shared by
    /// every group).
    pub window_state: f64,
    /// `H_0`: the first section's state bound at `N + ramp_frames`, the smaller of the window's
    /// and the group's `Phi`.
    pub energy: f64,
    /// `X_0 = 2 c_y H_0`: the joint flush's one-off terms at `N + ramp_frames + 1`.
    pub one_off: f64,
    /// The frame (from `N`) at which the first section is proven at rest, `N_SILENCE` included.
    pub first_rested: u64,
    /// The second section's state bound at [`Self::first_rested`] (`tau + c_y H + X` plus the
    /// flush's constant).
    pub second_state: f64,
    /// `R` for this group: the frame at which the second section is proven at rest.
    pub rested: u64,
}

/// [`live_cascade`]'s rest at one input peak, per settled zone group, with the quantities it reads
/// (the groups no other group covers, in increasing `Re p`; issue #1433). The cascade's `R` at
/// `peak` is the largest [`LiveGroupRest::rested`]. Evidence for the independent recomputation:
/// terms such as the joint flush's `X_0` move the final figures by at most a frame at the launch
/// configuration, so only a comparison of these intermediate quantities defends them.
///
/// # Errors
///
/// As [`live_cascade`].
pub fn live_cascade_groups(
    live: &LiveCascade,
    gain: f64,
    law: &FlushLaw,
    ramp_frames: u64,
    peak: f64,
) -> Result<Vec<LiveGroupRest>, TailBoundError> {
    let zones = live_cascade_zones(live)?;
    let bound = LiveBound::new(live, &zones, law, ramp_frames);
    bound.group_rests(gain * (1.0 + U) * peak)
}

/// [`live_zones`], after checking that both of the envelope's contractions are below `1` (a NaN
/// is refused with the rest) and the settled one at most the ramp one.
fn live_cascade_zones(live: &LiveCascade) -> Result<LiveZones, TailBoundError> {
    let envelope = &live.envelope;
    let contracts = |rho: f64| rho.partial_cmp(&1.0) == Some(core::cmp::Ordering::Less);
    if !(contracts(envelope.rho_ramp)
        && contracts(envelope.rho_settled)
        && envelope.rho_settled <= envelope.rho_ramp)
    {
        return Err(TailBoundError::NotContracting);
    }
    live_zones(live)
}

/// The tail and rest bound of a two-section live cascade, frequency-aware (issue #1433;
/// `docs/derivations/1329-input-section-tail-and-rest.md`, "#1433: the frequency-aware cascade").
///
/// * **Before `N`.** The first section's state is at most its zone's `Phi`; its output is
///   `1/2 (m1, m2) (I + A) s + d x` up to rounding, so the second section's state is at most
///   `iota sum_j W_j |y_j|` (`W_j` its contraction's powers) with
///   `|c . s_j| <= 1/2 |m| q_k E_j`. The weighted mass of `q_k E_j` telescopes through the
///   potential (Abel summation with non-decreasing weights): at most `sup Psi Phi` plus the
///   charges `Psi' (beta |x| + F)` per frame; direct zones are charged `q Phi` per frame.
/// * **Window.** For `ramp_frames + 1` frames a ramp may still be in flight: the same sums, with
///   the input zero.
/// * **Settled.** From `N + ramp_frames + 1` the words are fixed. With `b = (I - B) e2` exactly,
///   `tau = sigma - e2 y_prev` obeys `tau' = B (tau - e2 (y - y_prev)) + rounding`: the second
///   section is driven by the first's output *change*, `1/2 (m1, m2) (A^2 - I) s`, small at
///   both ends of the domain. Per zone the first section's settled design can lie in, a
///   four-term non-negative system bounds it; a joint-flush reset of either section adds at most
///   one output's worth (`X`).
/// * **Tail, stall, rest.** `T_decay` from the relative output bound; `P*` from the flush's
///   part of the output bound from `T_decay` on; rest section by section, each adding
///   `N_SILENCE`, maximised over the zones.
///
/// # Errors
///
/// [`TailBoundError`] when a section does not contract, a stall is not below `REST_EPS`, or a
/// bound exceeds the horizon.
pub fn live_cascade(
    live: &LiveCascade,
    gain: f64,
    law: &FlushLaw,
    ramp_frames: u64,
    peaks: [f64; 2],
) -> Result<CascadeBound, TailBoundError> {
    let zones = live_cascade_zones(live)?;
    let bound = LiveBound::new(live, &zones, law, ramp_frames);
    // `gain` is the trim word's magnitude; the kernel's `fl(x * trim)` is at most `(1 + u)` times
    // the product.
    let gain = gain * (1.0 + U);
    let tail = bound.tail(gain)?;
    let p_star = bound.stall(tail)? / (TAIL_FLOOR / 2.0) * SLACK;
    let rest_star = bound.rest(gain * p_star)?;
    let rest_peak = bound.rest(gain * peaks[0])?;
    let rest_any = bound.rest(gain * peaks[1])?;
    Ok(CascadeBound {
        tail,
        tail_every_peak: tail.max(rest_star),
        rest_peak,
        rest_any,
        flush_floor: p_star,
        rest_at_flush_floor: rest_star,
        tail_reference: tail,
    })
}

/// Impulse majorants of a fixed cascade, one frame at a time: the first section's exact state
/// response `G_1(t) = A_1^(t-1) b_1` and output `h_1(t)`, and for every later section `k` the
/// state majorant `Gbar_k(t+1) = q_k Gbar_k(t) + beta_k a_k(t)` with input majorant
/// `a_{k+1}(t) = gamma_k Gbar_k(t) + |d_k| a_k(t)`, `a_2 = |h_1|`.
///
/// For every reset pattern of the reference (a section's state set to zero at any frame), the
/// reference's state of section `k` at `N + m` is at most `gain * peak * sum_{t > m} Gbar_k(t)`
/// in the `V`-norm (`Gbar_1 = ||G_1||_V`), its input at most `gain * peak * sum_{t > m} a_k(t)`,
/// and its output at most `gain * peak * sum_{t > m} o(t)`, `o = a_{K+1}`: a reset only drops
/// terms of each sum, and the triangle inequality covers the rest.
///
/// **`f64` rounding.** The first section's response is computed in `f64`, so it carries an error
/// radius `e(t) >= ||G_1(t) - computed||_V` with `e(t + 1) = q_1 e(t) + nu (|s_1| + |s_2|)`. At
/// frame 0 the state is zero and the step returns `b` exactly (`2 a2` and `2 a3` scale a word by
/// two). At every later frame the input is zero, and each word of the step is
/// `A_i0 s_1 + A_i1 s_2` (plus an exact `+0`): the entry `1 - 2 c1` or `1 - 2 a3` rounds once
/// (`2 a2` is exact), each product once and the sum once, so each word errs by at most
/// `((1 + u)^3 - 1) (|A_i0| |s_1| + |A_i1| |s_2|) <= 3.01 u max|A_ij| (|s_1| + |s_2|)`. For a
/// TPT section of damping `k >= 0` (`t = g (g + k)`, `c1 = t / (1 + t)`, `a2 = g / (1 + t)`,
/// `a3 = g a2`), `c1` and `a3` lie in `[0, 1)` and `2 a2 <= 2 g / (1 + g^2) <= 1`, so
/// `|1 - 2 c1|`, `|2 a2|` and `|1 - 2 a3|` are at most `1` (to within the words' own `f32`
/// rounding, a relative `2^-24`). The error vector's two words then have a 2-norm of at most
/// `sqrt(2) 3.01 u (|s_1| + |s_2|)`, and in the `V`-norm at most `||R||_2` times that:
/// `4.26 u ||R||_2 (|s_1| + |s_2|)`, which `nu = 8 u ||R||_2` exceeds by a factor above 1.8. The
/// premise is the section's: words outside these ranges need their own `nu`. The output
/// majorant adds `gamma_1 e(t)` and the output's own rounding, at most `8 u` times its terms'
/// magnitudes: forming `c` and `d` from the words and the output from the state rounds each term
/// at most six times (`(1 + u)^6 - 1 <= 6.01 u`), and the remaining `1.99 u` covers this term's
/// own evaluation. The later sections' recursions are non-negative and step up by [`STEP_UP`] each
/// frame (its doc states the per-summand argument), so none of the three can fall below its exact
/// value however long it runs.
#[derive(Clone)]
struct Majorants<'a> {
    constants: &'a [SectionConstants],
    /// The first section's `A`, `b`, `c` and `d`, from its words.
    a: [[f64; 2]; 2],
    b: [f64; 2],
    c: [f64; 2],
    d: f64,
    /// `|m1 (1 - c1)| + |m2 a2|` and `|m1 a2| + |m2 (1 - a3)|`: the magnitudes of `c`'s terms,
    /// and `|d|`'s, for the output rounding.
    c_terms: [f64; 2],
    d_terms: f64,
    first: [f64; 2],
    /// `e(t)`, the first section's `f64` error radius in the `V`-norm.
    first_error: f64,
    later: Vec<f64>,
    frame: u64,
}

/// `nu`: the first section's `f64` step error per unit of `|s_1| + |s_2|`, in the `V`-norm.
const FIRST_STEP_ERROR: f64 = 8.0 * U64 * R_NORM;

/// The values of one frame of [`Majorants`], written in place by [`Majorants::step`].
struct MajorantFrame {
    /// `Gbar_k(t)` for every section.
    state: Vec<f64>,
    /// `a_k(t)` for `k = 1..=K + 1` (`a_1` the impulse itself).
    input: Vec<f64>,
}

impl MajorantFrame {
    fn new(sections: usize) -> Self {
        Self {
            state: std::vec![0.0; sections],
            input: std::vec![0.0; sections + 1],
        }
    }
}

impl<'a> Majorants<'a> {
    fn new(constants: &'a [SectionConstants], words: &SvfWords) -> Self {
        let (m0, m1, m2) = (words.m0.abs(), words.m1.abs(), words.m2.abs());
        let (a2, a3) = (words.a2.abs(), words.a3.abs());
        Self {
            constants,
            a: words.a(),
            b: words.b(),
            c: words.c(),
            d: words.d(),
            c_terms: [
                m1 * (1.0 - words.c1).abs() + m2 * a2,
                m1 * a2 + m2 * (1.0 - words.a3).abs(),
            ],
            d_terms: m0 + m1 * a2 + m2 * a3,
            first: [0.0, 0.0],
            first_error: 0.0,
            later: std::vec![0.0; constants.len().saturating_sub(1)],
            frame: 0,
        }
    }

    /// Writes the majorants at the current frame `t` into `out`, then advances to `t + 1`.
    ///
    /// The first section's state majorant `Gbar_1(t) = ||G_1(t)||_V` is not evaluated per frame:
    /// its sum is closed in [`fixed_cascade`] (`G_1(t) = A_1^(t-1) b_1`, so the sum over `t >= 1`
    /// is at most `beta_1 / (1 - q_1)`), and `out.state[0]` stays `0`.
    fn step(&mut self, out: &mut MajorantFrame) {
        let k = self.constants.len();
        let impulse = if self.frame == 0 { 1.0 } else { 0.0 };
        out.input[0] = impulse;
        out.state[0] = 0.0;
        let (c, first) = (self.c, self.first);
        // `|h_1(t)|` as computed, plus the error radius through the output row and the output's
        // own rounding: at least the exact `|h_1(t)|`.
        let output_rounding = 8.0
            * U64
            * (self.c_terms[0] * first[0].abs()
                + self.c_terms[1] * first[1].abs()
                + self.d_terms * impulse);
        out.input[1] = ((c[0] * first[0] + c[1] * first[1] + self.d * impulse).abs()
            + output_rounding
            + self.constants[0].gamma * self.first_error)
            * STEP_UP;
        for i in 1..k {
            out.state[i] = self.later[i - 1];
            out.input[i + 1] = (self.constants[i].gamma * out.state[i]
                + self.constants[i].delta * out.input[i])
                * STEP_UP;
        }
        // Advance.
        let (a, b) = (self.a, self.b);
        self.first_error = (self.constants[0].q * self.first_error
            + FIRST_STEP_ERROR * (first[0].abs() + first[1].abs()))
            * STEP_UP;
        self.first = [
            a[0][0] * first[0] + a[0][1] * first[1] + b[0] * impulse,
            a[1][0] * first[0] + a[1][1] * first[1] + b[1] * impulse,
        ];
        for i in 1..k {
            self.later[i - 1] = (self.constants[i].q * self.later[i - 1]
                + self.constants[i].beta * out.input[i])
                * STEP_UP;
        }
        self.frame += 1;
    }

    /// Closed-form remainders `sum_{t >= now}` of every state majorant, every input majorant and
    /// the output majorant, by contraction from the current frame: for `t >= now` the first
    /// section's state shrinks by `q_1` per frame, `|h_1| <= gamma_1 Gbar_1`, and the later
    /// majorants follow their own recursions, so the vector of state majorants obeys
    /// `m(t + 1) <= M m(t)` with `M` lower triangular and non-negative; the sums are
    /// `(I - M)^-1 m(now)`, solved by forward substitution.
    fn remainders(&self) -> (Vec<f64>, Vec<f64>) {
        let k = self.constants.len();
        // The first section's exact state is within `first_error` of the computed one.
        let mut current = std::vec![v_norm(self.first) * SLACK + self.first_error];
        current.extend(self.later.iter().copied());
        // `alpha[i][j]`: coefficient of `Gbar_j` in `a_i` for `t >= now >= 1`.
        let mut alpha = std::vec![std::vec![0.0; k]; k + 1];
        alpha[1][0] = self.constants[0].gamma;
        for i in 1..k {
            let (earlier, later) = alpha.split_at_mut(i + 1);
            for (next, previous) in later[0].iter_mut().zip(&earlier[i]).take(i) {
                *next = self.constants[i].delta * previous;
            }
            later[0][i] = self.constants[i].gamma;
        }
        // Solve `(I - M) s = current`: `s_0 = current_0 / (1 - q_0)`,
        // `s_i = (current_i + beta_i sum_j alpha[i][j] s_j) / (1 - q_i)`.
        let mut sums = std::vec![0.0; k];
        for i in 0..k {
            let drive: f64 = if i == 0 {
                0.0
            } else {
                self.constants[i].beta * (0..i).map(|j| alpha[i][j] * sums[j]).sum::<f64>()
            };
            sums[i] = (current[i] + drive) / (1.0 - self.constants[i].q) * SLACK;
        }
        let inputs: Vec<f64> = (0..=k)
            .map(|i| (0..k).map(|j| alpha[i][j] * sums[j]).sum::<f64>() * SLACK)
            .collect();
        (sums, inputs)
    }
}

/// A non-negative square matrix, row major.
struct SquareMatrix {
    n: usize,
    m: Vec<f64>,
}

impl SquareMatrix {
    fn apply(&self, z: &[f64]) -> Vec<f64> {
        (0..self.n)
            .map(|i| dot(&self.m[i * self.n..(i + 1) * self.n], z))
            .collect()
    }

    fn multiply(&self, other: &Self) -> Self {
        let n = self.n;
        let mut m = std::vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                m[i * n + j] = (0..n)
                    .map(|l| self.m[i * n + l] * other.m[l * n + j])
                    .sum::<f64>()
                    * SLACK;
            }
        }
        Self { n, m }
    }
}

/// `a . b`, inflated by [`SLACK`] (every operand here is non-negative).
fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(a, b)| a * b).sum::<f64>() * SLACK
}

/// `M^steps z` by repeated squaring, every product inflated by [`SLACK`].
fn power_apply(matrix: &SquareMatrix, mut steps: u64, z: &[f64]) -> Vec<f64> {
    let mut result = z.to_vec();
    let mut base = SquareMatrix {
        n: matrix.n,
        m: matrix.m.clone(),
    };
    while steps > 0 {
        if steps & 1 == 1 {
            result = base.apply(&result);
        }
        steps >>= 1;
        if steps > 0 {
            base = base.multiply(&base);
        }
    }
    result
}

/// The deviation of the `f32` kernel from the reset-aware reference, per unit of `gain * peak`
/// (relative) and from the flush alone (absolute), propagated after the input's end.
///
/// With `E_k` bounding `||s_kernel - s_ref||_V` of section `k` and `D_k` the magnitude of the
/// difference of their inputs (`D_1 = 0`):
///
/// ```text
/// E_k' <= (q_k + mu_k) E_k + (beta_k + mu_x_k) D_k + mu_k x_k + mu_x_k v_k + F
/// D_{k+1} <= (gamma_k + omega_k) E_k + (|d_k| + omega_x_k) D_k + omega_k x_k + omega_x_k v_k
/// ```
///
/// with `x_k`, `v_k` the reference's state and input bounds. A joint-flush reset clears the
/// kernel and the reference together, so it only lowers the deviation. Before `N` the drives are at
/// their suprema and the deviation is at most the fixed point; after `N` the reference contracts
/// (`x_1' <= q_1 x_1`, `v_{k+1} <= gamma_k x_k + |d_k| v_k`, `x_k' <= q_k x_k + beta_k v_k`).
struct Deviation<'a> {
    constants: &'a [SectionConstants],
    /// Relative deviation `E_k` and reference state `x_k`, per unit of `gain * peak`.
    error: Vec<f64>,
    reference: Vec<f64>,
    /// The previous frame's `error` and `reference` after a [`Deviation::step`] (the step's
    /// scratch before it).
    previous_error: Vec<f64>,
    previous_reference: Vec<f64>,
}

impl<'a> Deviation<'a> {
    /// The deviation at `N`, from the reference's suprema `state_sup` and `input_sup` (per unit
    /// of `gain * peak`; `input_sup[0] = 1`).
    fn at_end(constants: &'a [SectionConstants], state_sup: &[f64], input_sup: &[f64]) -> Self {
        let k = constants.len();
        let mut error = std::vec![0.0; k];
        let mut difference = 0.0;
        for i in 0..k {
            let c = &constants[i];
            error[i] = ((c.beta + c.mu_input) * difference
                + c.mu_state * state_sup[i]
                + c.mu_input * input_sup[i])
                / (1.0 - c.rho_kernel())
                * SLACK;
            difference = (c.gamma + c.omega_state) * error[i]
                + (c.delta + c.omega_input) * difference
                + c.omega_state * state_sup[i]
                + c.omega_input * input_sup[i];
        }
        Self {
            constants,
            previous_error: std::vec![0.0; k],
            previous_reference: std::vec![0.0; k],
            error,
            reference: state_sup.to_vec(),
        }
    }

    /// The absolute (flush) deviation's fixed point per section and at the output, per unit of
    /// the per-step drive `F` (the output also carries the mix's underflow allowance).
    fn absolute(constants: &[SectionConstants]) -> (Vec<f64>, f64) {
        let mut error = std::vec![0.0; constants.len()];
        let mut difference = 0.0;
        for (i, c) in constants.iter().enumerate() {
            error[i] = ((c.beta + c.mu_input) * difference + 1.0) / (1.0 - c.rho_kernel()) * SLACK;
            // `+ 1`: the mix's underflow allowance, far below one unit of `F`.
            difference =
                (c.gamma + c.omega_state) * error[i] + (c.delta + c.omega_input) * difference + 1.0;
        }
        (error, difference * SLACK)
    }

    /// The relative output deviation at the current frame, then advance one frame.
    fn step(&mut self) -> f64 {
        let k = self.constants.len();
        let mut difference = 0.0;
        let mut input = 0.0;
        for i in 0..k {
            let c = &self.constants[i];
            self.previous_error[i] = (c.rho_kernel() * self.error[i]
                + (c.beta + c.mu_input) * difference
                + c.mu_state * self.reference[i]
                + c.mu_input * input)
                * SLACK;
            self.previous_reference[i] = (c.q * self.reference[i] + c.beta * input) * SLACK;
            difference = (c.gamma + c.omega_state) * self.error[i]
                + (c.delta + c.omega_input) * difference
                + c.omega_state * self.reference[i]
                + c.omega_input * input;
            input = c.gamma * self.reference[i] + c.delta * input;
        }
        // The new frame becomes current; the old one stays readable for [`Self::falling`].
        core::mem::swap(&mut self.error, &mut self.previous_error);
        core::mem::swap(&mut self.reference, &mut self.previous_reference);
        difference * SLACK
    }

    /// The current state `(E_1..E_K, x_1..x_K)`.
    fn state(&self) -> Vec<f64> {
        self.error.iter().chain(&self.reference).copied().collect()
    }

    /// The step as a matrix on [`Self::state`] and the output value as a row: the step is linear
    /// (no constant term), so its columns are its images of the unit vectors.
    fn linear_map(&self) -> (SquareMatrix, Vec<f64>) {
        let k = self.constants.len();
        let n = 2 * k;
        let mut m = std::vec![0.0; n * n];
        let mut row = std::vec![0.0; n];
        for j in 0..n {
            let mut probe = Deviation {
                constants: self.constants,
                error: std::vec![0.0; k],
                reference: std::vec![0.0; k],
                previous_error: std::vec![0.0; k],
                previous_reference: std::vec![0.0; k],
            };
            if j < k {
                probe.error[j] = 1.0;
            } else {
                probe.reference[j - k] = 1.0;
            }
            row[j] = probe.step();
            for (i, value) in probe.state().into_iter().enumerate() {
                m[i * n + j] = value;
            }
        }
        (SquareMatrix { n, m }, row)
    }

    /// Whether the last [`Self::step`] lowered (or kept) every component. The propagation is a
    /// non-negative linear map, so from such a frame on every component, and the output bound,
    /// stays non-increasing.
    fn falling(&self) -> bool {
        self.error
            .iter()
            .zip(&self.previous_error)
            .chain(self.reference.iter().zip(&self.previous_reference))
            .all(|(next, now)| next <= now)
    }
}

/// The certified tail and exact-rest bound of a fixed cascade (issue #1329 D3, D4).
///
/// `sections` are the enabled sections in signal order (a disabled section is the exact identity
/// with zero state and is left out); `gain` bounds the magnitude of the trim word ahead of them;
/// `peaks` are the input peaks the two rest bounds are stated for (+24 dBFS and the sanitizer's
/// limit).
///
/// * **Exact half.** The reference output at `N + m` is at most `gain * peak * Ref(m)`,
///   `Ref(m) = sum_{t > m} o(t)` (the impulse majorants), summed exactly to a horizon and closed by the
///   contraction remainder. The tail is at least the first `m` with `gain * Ref(m) < eps / 2`.
/// * **`f32` half.** The kernel's output differs from the reference's by at most
///   `gain * peak * dev(m) + F * a` (the deviation propagation): relative rounding plus the flush stall. For
///   every peak at or above `p_star = F a / (eps / 2 - gain * dev_sup)` the deviation fits the other
///   `eps / 2` from the tail on; below `p_star` exact rest is proven by the tail instead.
/// * **Rest.** The kernel state of section `k` at `N` is at most
///   `gain * peak * (x_k + E_k) + F * E_abs_k`, capped at the `V`-norm of finite `f32` words; rest
///   follows section by section.
///
/// # Errors
///
/// [`TailBoundError`] when a section does not contract, the stall is not below `REST_EPS`, or a
/// bound exceeds the horizon.
pub fn fixed_cascade(
    sections: &[SvfWords],
    gain: f64,
    law: &FlushLaw,
    peaks: [f64; 2],
) -> Result<CascadeBound, TailBoundError> {
    fixed_cascade_within(sections, gain, law, peaks, u64::MAX)
        .result
        .expect("no walk reaches u64::MAX frames: the horizon limit stops it first")
}

/// What a [`fixed_cascade_within`] walk returns: the bound (or why none is stated) when the walk
/// finished inside its horizon, and the frames it actually walked, finished or stopped.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CascadeWalk {
    /// [`fixed_cascade`]'s result, bit for bit, when the walk finished inside its horizon; `None`
    /// when it stopped because one more frame would pass the horizon.
    pub result: Option<Result<CascadeBound, TailBoundError>>,
    /// The frames walked: every frame-by-frame step of the majorant pass, of the replay of its
    /// crossing block and of the deviation walk (issue #1457 D1), counted as taken, so a stopped
    /// walk reports what it really took. The closed-form steps (powers of a step matrix, the rest
    /// bound) are not frames walked. For a finished walk, a function of the sections, the gain,
    /// the law and the peaks only.
    pub frames: u64,
}

/// One more frame of a walk under a horizon of `horizon` frames: `false` once the walk has taken
/// `horizon` frames, so a walk of exactly `horizon` frames finishes and a longer one stops.
fn take_frame(walked: &mut u64, horizon: u64) -> bool {
    if *walked >= horizon {
        return false;
    }
    *walked += 1;
    true
}

/// [`fixed_cascade`] under a horizon of `horizon` frames walked (issue #1457 D1, D3). The result
/// is `None` when the walk would take more than `horizon` frames, and otherwise the same result as
/// [`fixed_cascade`], bit for bit; [`CascadeWalk::frames`] is the frames walked either way, never
/// more than `horizon`. The walk is the same whatever the horizon, so a walk that finishes takes
/// the same frames under every horizon at or above them, and stops under every smaller one.
#[must_use]
pub fn fixed_cascade_within(
    sections: &[SvfWords],
    gain: f64,
    law: &FlushLaw,
    peaks: [f64; 2],
    horizon: u64,
) -> CascadeWalk {
    let mut walked = 0_u64;
    let result = fixed_cascade_walk(sections, gain, law, peaks, &mut walked, horizon);
    CascadeWalk {
        result,
        frames: walked,
    }
}

#[allow(clippy::too_many_lines)]
fn fixed_cascade_walk(
    sections: &[SvfWords],
    gain: f64,
    law: &FlushLaw,
    peaks: [f64; 2],
    walked: &mut u64,
    horizon: u64,
) -> Option<Result<CascadeBound, TailBoundError>> {
    if sections.is_empty() {
        return Some(Ok(CascadeBound::ZERO));
    }
    let constants: Vec<SectionConstants> = sections.iter().map(SectionConstants::of).collect();
    // A NaN contraction is refused with the rest: only `rho < 1` passes.
    let contracts = |rho: f64| rho.partial_cmp(&1.0) == Some(core::cmp::Ordering::Less);
    if !constants.iter().all(|c| contracts(c.rho_kernel())) {
        return Some(Err(TailBoundError::NotContracting));
    }
    let k = constants.len();
    // `gain` is the trim word's magnitude; the kernel's `fl(x * trim)` is at most `(1 + u)` times
    // the product.
    let gain = gain * (1.0 + U);
    let threshold = TAIL_FLOOR / 2.0 / gain;

    // Pass 1: block sums of the output majorant and checkpoints, to a horizon whose remainder is
    // far below the threshold; the suprema of every state and input majorant.
    let mut majorants = Majorants::new(&constants, &sections[0]);
    let mut frame = MajorantFrame::new(k);
    let mut checkpoints = Vec::new();
    let mut block_sums = Vec::new();
    let mut state_sup = std::vec![0.0; k];
    let mut input_sup = std::vec![0.0; k + 1];
    let remainder = loop {
        if majorants.frame >= HORIZON_LIMIT {
            return Some(Err(TailBoundError::Horizon));
        }
        checkpoints.push(majorants.clone());
        let mut sum = 0.0;
        for _ in 0..BLOCK {
            if !take_frame(walked, horizon) {
                return None;
            }
            majorants.step(&mut frame);
            sum += frame.input[k];
            for i in 0..k {
                state_sup[i] += frame.state[i];
                input_sup[i] += frame.input[i];
            }
            input_sup[k] += frame.input[k];
        }
        block_sums.push(sum);
        let (states, inputs) = majorants.remainders();
        if inputs[k] < threshold / 16.0 {
            for i in 0..k {
                state_sup[i] += states[i];
                input_sup[i] += inputs[i];
            }
            break inputs[k];
        }
    };
    // Every sum here has at most `majorants.frame` non-negative terms (block sums, then the sum of
    // the blocks and the remainder): their rounding is bounded by `accumulation`.
    let summed = accumulation(majorants.frame + 1);
    // The first section's state majorant in closed form ([`Majorants::step`]).
    state_sup[0] = constants[0].beta / (1.0 - constants[0].q);
    let state_sup: Vec<f64> = state_sup
        .iter()
        .map(|value| value * SLACK * summed)
        .collect();
    let input_sup: Vec<f64> = input_sup
        .iter()
        .map(|value| value * SLACK * summed)
        .collect();

    // The suffix `Ref(t0) = sum_{t >= t0} o(t)` at each block start, from the back.
    let blocks = block_sums.len();
    let mut suffix = std::vec![0.0; blocks + 1];
    suffix[blocks] = remainder;
    for block in (0..blocks).rev() {
        suffix[block] = suffix[block + 1] + block_sums[block];
    }
    // The first `t0` with `sum_{t >= t0} o(t) < threshold`. The reference output at `N + m` sums
    // from `t = m + 1`, so D1 needs only `m >= t0 - 1`; the exact half is stated as `t0`, one
    // frame more, so that it reads on the same index as the impulse-response suffix
    // `S(j) = sum_{m >= j} |h[m]|` gate 1(a) compares it with (`T_b(eps / 2) <= T`).
    let first_block = (0..=blocks)
        .find(|&block| suffix[block] * SLACK * summed < threshold)
        .expect("the remainder is below the threshold");
    let t0 = if first_block == 0 {
        0
    } else {
        let block = first_block - 1;
        let mut replay = checkpoints[block].clone();
        let mut values = Vec::with_capacity(BLOCK);
        for _ in 0..BLOCK {
            if !take_frame(walked, horizon) {
                return None;
            }
            replay.step(&mut frame);
            values.push(frame.input[k]);
        }
        let mut running = suffix[block + 1];
        let mut first = (block + 1) * BLOCK;
        for index in (0..BLOCK).rev() {
            running += values[index];
            if running * SLACK * summed < threshold {
                first = block * BLOCK + index;
            } else {
                break;
            }
        }
        first as u64
    };
    let tail_reference = t0;

    // The `f32` half: the relative deviation, frame by frame from `N`, until it is falling for
    // good below its share; the flush's stall sets `p_star`.
    let (absolute_error, absolute_output) = Deviation::absolute(&constants);
    let f = law.per_step();
    let stall = f * absolute_output;
    let mut deviation = Deviation::at_end(&constants, &state_sup, &input_sup);
    // The relative deviation must fall below half of its share (`eps / 4`) for good, leaving the
    // other half for the flush stall at `p_star`. The propagated system is non-negative, so once
    // every component falls it falls for good.
    let quarter = TAIL_FLOOR / 4.0 / gain;
    let mut tail_deviation = 0_u64;
    let mut frame = 0_u64;
    let mut values = Vec::new();
    // Walk frame by frame until the propagation is falling: from that frame on it is
    // componentwise non-increasing, and so is every value.
    loop {
        if !take_frame(walked, horizon) {
            return None;
        }
        let value = deviation.step();
        values.push(value);
        if value >= quarter {
            tail_deviation = frame + 1;
        }
        frame += 1;
        if deviation.falling() {
            break;
        }
        if frame >= HORIZON_LIMIT {
            return Some(Err(TailBoundError::Horizon));
        }
    }
    // The rest is closed-form (powers of the step, the rest bound): no frame is walked.
    let mut finish = || -> Result<CascadeBound, TailBoundError> {
        let (step, row) = deviation.linear_map();
        let at = |later: u64| dot(&row, &power_apply(&step, later, &deviation.state()));
        if tail_deviation == frame {
            // Still at or above `quarter` on the last walked frame: the first later frame below it,
            // by an exponential search and a bisection over the non-increasing values.
            let (mut low, mut high) = (0_u64, 1_u64);
            while at(high) >= quarter {
                low = high;
                high = high
                    .checked_mul(2)
                    .filter(|value| frame + value < HORIZON_LIMIT)
                    .ok_or(TailBoundError::Horizon)?;
            }
            if at(low) < quarter {
                high = low;
            } else {
                while high - low > 1 {
                    let middle = low + (high - low) / 2;
                    if at(middle) < quarter {
                        high = middle;
                    } else {
                        low = middle;
                    }
                }
            }
            tail_deviation = frame + high;
        }
        let tail_decay = tail_reference.max(tail_deviation);
        // The deviation's supremum from `tail_decay` on: the walked values from there, and beyond the
        // walk the value at `max(tail_decay, frame)`, which bounds every later one. That value comes
        // from the current state by powers of the (linear, non-negative) step.
        let mut dev_sup_from_core = values
            .iter()
            .skip(tail_decay as usize)
            .fold(0.0_f64, |sup, value| sup.max(*value));
        dev_sup_from_core = dev_sup_from_core.max(at(tail_decay.max(frame) - frame));
        let budget = TAIL_FLOOR / 2.0 - gain * dev_sup_from_core;
        let p_star = stall / budget * SLACK;

        // Rest: the kernel state at `N`, section by section.
        let envelopes: Vec<EnvelopeSection> =
            constants.iter().map(SectionConstants::envelope).collect();
        let cap = R_NORM * core::f64::consts::SQRT_2 * F32_MAX;
        let at_end = Deviation::at_end(&constants, &state_sup, &input_sup);
        let kernel_state = |peak: f64| -> Vec<f64> {
            (0..k)
                .map(|i| {
                    ((state_sup[i] + at_end.error[i]) * gain * peak + f * absolute_error[i])
                        .min(cap)
                        * SLACK
                })
                .collect()
        };
        let rest_star = rest_frames(&envelopes, &kernel_state(p_star), law, 0)?;
        let rest_peak = rest_frames(&envelopes, &kernel_state(peaks[0]), law, 0)?;
        let rest_any = rest_frames(&envelopes, &kernel_state(peaks[1]), law, 0)?;
        Ok(CascadeBound {
            tail: tail_decay,
            tail_every_peak: tail_decay.max(rest_star),
            rest_peak,
            rest_any,
            flush_floor: p_star,
            rest_at_flush_floor: rest_star,
            tail_reference,
        })
    };
    Some(finish())
}

#[cfg(test)]
mod tests {
    use super::{SvfWords, v_operator_norm};

    /// The certified operator norm is never below the exact `||A||_V` of a near-rotation, and
    /// within `1e-13` of it. The references are the exact norms of the kernel's own `f32` words,
    /// evaluated in 60-digit decimal arithmetic (`R A R^-1`, then `s1` by the non-cancelling
    /// closed form) and rounded to the nearest `f64`; a bound strictly above that `f64` is above the
    /// exact value. The cancelling form `sqrt((F + sqrt(F^2 - 4 det^2)) / 2)` falls below the first
    /// two by `6.0e-9` and `1.1e-9` (#1329 attempt 3, M1).
    #[test]
    fn the_operator_norm_bounds_the_exact_norm_of_every_near_top_design() {
        // (c1, a2, a3) bits, exact `||A||_V` to 25 digits.
        let rows: [([u32; 3], f64); 6] = [
            // 44.1 kHz HPF one `f32` below the maximum: 0.9999476754147092331719515.
            (
                [0x3f80_0000, 0x381b_3976, 0x3f7f_fc92],
                0.999_947_675_414_709_2,
            ),
            // 44.1 kHz LPF at 1 kHz: 0.9041639287076296876045586.
            (
                [0x3dc4_4bda, 0x3d84_2298, 0x3b96_dd23],
                0.904_163_928_707_629_7,
            ),
            // 44.1 kHz LPF at the maximum: 0.9999478657377627828961350.
            (
                [0x3f80_0000, 0x381a_a414, 0x3f7f_fc95],
                0.999_947_865_737_762_8,
            ),
            // 48 kHz LPF at the maximum: 0.9999475892230697675331340.
            (
                [0x3f80_0000, 0x381b_7ad0, 0x3f7f_fc90],
                0.999_947_589_223_069_8,
            ),
            // 48 kHz HPF one `f32` below the maximum: 0.9999474076590467282723153.
            (
                [0x3f80_0000, 0x381c_040f, 0x3f7f_fc8d],
                0.999_947_407_659_046_7,
            ),
            // 44.1 kHz HPF at 10 Hz: 0.9989930509006467528090886.
            (
                [0x3a83_fb9b, 0x3a3a_8ed5, 0x3508_16f3],
                0.998_993_050_900_646_8,
            ),
        ];
        for (bits, exact) in rows {
            let [c1, a2, a3] = bits.map(f32::from_bits);
            let words = SvfWords::from_f32([c1, a2, a3, 0.0, 0.0, 0.0]);
            let bound = v_operator_norm(words.a());
            assert!(
                bound > exact && bound - exact < 1.0e-13,
                "words {bits:08x?}: bound {bound:.17} against exact {exact:.17}"
            );
        }
    }
}
