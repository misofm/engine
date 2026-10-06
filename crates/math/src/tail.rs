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
//! * [`envelope_cascade`]: a cascade whose words may change under a control law, given per-section
//!   suprema of the contraction and gains over every reachable word. The state at the input's end
//!   is bounded by the invariant ball of each section in turn; the free response is propagated
//!   directly on the kernel.
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

/// The relative inflation applied to every bound computed in `f64`, so that the `f64` rounding
/// of the bound's own arithmetic cannot make it optimistic (`2^-30`, far above any accumulated
/// `f64` rounding of the sums here and far below anything that moves a sample count).
const SLACK: f64 = 1.0 + 1.0 / 1_073_741_824.0;

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

/// The spectral norm of a 2x2 matrix.
fn spectral_norm(m: [[f64; 2]; 2]) -> f64 {
    let frobenius = m[0][0] * m[0][0] + m[0][1] * m[0][1] + m[1][0] * m[1][0] + m[1][1] * m[1][1];
    let determinant = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    let discriminant = (frobenius * frobenius - 4.0 * determinant * determinant).max(0.0);
    crate::sqrt(0.5 * (frobenius + crate::sqrt(discriminant)))
}

/// `||M||_V`, the operator norm `M` induces in the `V`-norm: the spectral norm of `R M R^-1`.
#[must_use]
pub fn v_operator_norm(m: [[f64; 2]; 2]) -> f64 {
    let rm = [
        [m[0][0] + R * m[1][0], m[0][1] + R * m[1][1]],
        [R * m[1][0], R * m[1][1]],
    ];
    spectral_norm([
        [rm[0][0], -rm[0][0] + core::f64::consts::SQRT_2 * rm[0][1]],
        [rm[1][0], -rm[1][0] + core::f64::consts::SQRT_2 * rm[1][1]],
    ])
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
    worst * SLACK
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
    let mu_state = R_NORM * spectral_norm(state) * R_INV_NORM + KAPPA * U * q;
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
    /// The constants of one fixed section.
    #[must_use]
    pub fn of(words: &SvfWords) -> Self {
        let q = v_operator_norm(words.a()) * SLACK;
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
    /// suffix is below `eps / 2` (evidence only).
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
/// at rest: its state stays zero and its output is exactly zero.
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

    /// The coefficients of the output bound `v_{K+1}` on `w`.
    fn output_row(sections: &[EnvelopeSection], from: usize, f: f64) -> Vec<f64> {
        let mut rows = Self::input_rows(sections, from, f);
        rows.pop().expect("K + 1 rows")
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

/// For a non-negative system, the first `m >= start` from which the bound `row . M^m z` stays
/// below `limit`. `M^m z` is propagated frame by frame until it is componentwise non-increasing
/// (after which it stays so, `M` being non-negative) and below `limit`.
fn decay_frames(
    system: &Propagator,
    row: &[f64],
    z: &[f64],
    limit: f64,
) -> Result<u64, TailBoundError> {
    let mut current = z.to_vec();
    let mut last_above = None::<u64>;
    let mut frame = 0_u64;
    loop {
        let value: f64 = row.iter().zip(&current).map(|(a, b)| a * b).sum::<f64>() * SLACK;
        if value >= limit {
            last_above = Some(frame);
        }
        let next = system.apply(&current);
        let falling = next.iter().zip(&current).all(|(next, now)| next <= now);
        if falling && value < limit {
            return Ok(last_above.map_or(0, |frame| frame + 1));
        }
        current = next;
        frame += 1;
        if frame >= HORIZON_LIMIT {
            return Err(TailBoundError::Horizon);
        }
    }
}

/// [`decay_frames`] for a long decay: jumps ahead with powers of the system while the bound is
/// still falling and above `limit`, then finishes frame by frame.
fn decay_frames_fast(
    system: &Propagator,
    row: &[f64],
    z: &[f64],
    limit: f64,
) -> Result<u64, TailBoundError> {
    let evaluate = |state: &[f64]| row.iter().zip(state).map(|(a, b)| a * b).sum::<f64>() * SLACK;
    let mut current = z.to_vec();
    let mut base = 0_u64;
    // Walk frame by frame until the propagation is componentwise falling.
    loop {
        let next = system.apply(&current);
        if next.iter().zip(&current).all(|(next, now)| next <= now) {
            break;
        }
        current = next;
        base += 1;
        if base >= HORIZON_LIMIT {
            return Err(TailBoundError::Horizon);
        }
    }
    // From `base` on the bound is non-increasing; before it, every frame was walked, and a frame
    // above `limit` before `base` is covered because the answer is at least `base` whenever the
    // value at `base` is still above `limit`, and is found by [`decay_frames`] otherwise.
    if evaluate(&current) < limit {
        return decay_frames(system, row, z, limit);
    }
    // Exponential search for a power that brings the bound below `limit`, then bisection.
    let mut step = 1_u64;
    let mut low = 0_u64; // above the limit at `base + low`
    let high = loop {
        let candidate = low + step;
        if base + candidate >= HORIZON_LIMIT {
            return Err(TailBoundError::Horizon);
        }
        if evaluate(&system.power_apply(candidate, &current)) < limit {
            break candidate;
        }
        low = candidate;
        step *= 2;
    };
    let (mut low, mut high) = (low, high);
    while high - low > 1 {
        let middle = low + (high - low) / 2;
        if evaluate(&system.power_apply(middle, &current)) < limit {
            high = middle;
        } else {
            low = middle;
        }
    }
    Ok(base + high)
}

/// The tail and rest bound of a cascade whose words may change under a control law, from
/// per-section suprema over every reachable word ([`EnvelopeSection`]).
///
/// * Before the input's end `N` the kernel state of section `k` lies in the invariant ball
///   `B_k = (input_k V_k + F) / (1 - rho_ramp_k)`, `V_1 = gain * peak` and
///   `V_{k+1} = output_state_k B_k + output_input_k V_k`, capped at the `V`-norm of a state of
///   finite `f32` words (a state that overflows is cleared by the per-block recovery).
/// * After `N` the input is exactly zero; for `ramp_frames` frames a ramp may still be in flight
///   (`rho_ramp`), then every section is settled (`rho_settled`).
/// * The output bound is `peak * r(m) + a`, `a` the flush stall the absolute drive `F` sustains.
///   The tail is the first `m` with `gain * r(m) < TAIL_FLOOR / 2` for every later frame, and for
///   a peak below `p_star = a / (TAIL_FLOOR / 2)` the tail is proven by exact rest instead, so the
///   tail is at least the rest bound at `p_star`.
///
/// # Errors
///
/// [`TailBoundError`] when a section does not contract, the stall is not below `REST_EPS`, or a
/// bound exceeds the horizon.
pub fn envelope_cascade(
    sections: &[EnvelopeSection],
    gain: f64,
    law: &FlushLaw,
    ramp_frames: u64,
    peaks: [f64; 2],
) -> Result<CascadeBound, TailBoundError> {
    if sections.is_empty() {
        return Ok(CascadeBound::ZERO);
    }
    // A NaN contraction is refused with the rest: only `rho < 1` passes.
    let contracts = |rho: f64| rho.partial_cmp(&1.0) == Some(core::cmp::Ordering::Less);
    if sections.iter().any(|s| {
        !(contracts(s.rho_ramp) && contracts(s.rho_settled) && s.rho_settled <= s.rho_ramp)
    }) {
        return Err(TailBoundError::NotContracting);
    }
    // `gain` is the trim word's magnitude; the kernel's `fl(x * trim)` is at most `(1 + u)` times
    // the product.
    let gain = gain * (1.0 + U);
    let f = law.per_step();
    let cap = R_NORM * core::f64::consts::SQRT_2 * F32_MAX;
    // The balls per unit of `gain * peak` (relative) and from `F` alone (absolute): the fixed
    // point of the ramp system, section by section.
    let balls = |drive: f64, absolute: f64| -> Vec<f64> {
        let mut v = drive;
        sections
            .iter()
            .map(|s| {
                let ball = (s.input * v + absolute) / (1.0 - s.rho_ramp) * SLACK;
                v = (s.output_state * ball + s.output_input * v + absolute) * SLACK;
                ball
            })
            .collect()
    };
    let relative = balls(1.0, 0.0);
    let absolute = balls(0.0, f);
    // The absolute part of the output bound never exceeds its value at the ramp system's fixed
    // point, which the settled system (smaller contractions) maps below itself.
    let stall: f64 = {
        let row = Propagator::output_row(sections, 0, f);
        let fixed: Vec<f64> = absolute.iter().copied().chain([1.0]).collect();
        row.iter().zip(&fixed).map(|(a, b)| a * b).sum::<f64>() * SLACK
    };
    let p_star = stall / (TAIL_FLOOR / 2.0) * SLACK;
    let ramp: Vec<f64> = sections.iter().map(|s| s.rho_ramp).collect();
    let settled: Vec<f64> = sections.iter().map(|s| s.rho_settled).collect();
    // Relative tail: the system without `F`, from the relative balls at `gain`, through the ramp
    // window frame by frame, then settled.
    let relative_state: Vec<f64> = relative.iter().map(|b| b * gain).chain([0.0]).collect();
    let window = Propagator::new(sections, &ramp, 0, 0.0);
    let settled_system = Propagator::new(sections, &settled, 0, 0.0);
    let row = Propagator::output_row(sections, 0, 0.0);
    let limit = TAIL_FLOOR / 2.0;
    let mut tail_relative = 0_u64;
    let mut state = relative_state;
    for frame in 0..ramp_frames {
        let value: f64 = row.iter().zip(&state).map(|(a, b)| a * b).sum();
        if value * SLACK >= limit {
            tail_relative = frame + 1;
        }
        state = window.apply(&state);
    }
    let after = decay_frames_fast(&settled_system, &row, &state, limit)?;
    if after > 0 {
        tail_relative = ramp_frames + after;
    }
    let ball_at = |peak: f64| -> Vec<f64> {
        relative
            .iter()
            .zip(&absolute)
            .map(|(r, a)| (r * gain * peak + a).min(cap))
            .collect()
    };
    let rest_star = rest_frames(sections, &ball_at(p_star), law, ramp_frames)?;
    let rest_peak = rest_frames(sections, &ball_at(peaks[0]), law, ramp_frames)?;
    let rest_any = rest_frames(sections, &ball_at(peaks[1]), law, ramp_frames)?;
    Ok(CascadeBound {
        tail: tail_relative,
        tail_every_peak: tail_relative.max(rest_star),
        rest_peak,
        rest_any,
        flush_floor: p_star,
        rest_at_flush_floor: rest_star,
        tail_reference: tail_relative,
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
#[derive(Clone)]
struct Majorants<'a> {
    constants: &'a [SectionConstants],
    /// The first section's `A`, `b`, `c` and `d`, from its words.
    a: [[f64; 2]; 2],
    b: [f64; 2],
    c: [f64; 2],
    d: f64,
    first: [f64; 2],
    later: Vec<f64>,
    frame: u64,
}

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
        Self {
            constants,
            a: words.a(),
            b: words.b(),
            c: words.c(),
            d: words.d(),
            first: [0.0, 0.0],
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
        let c = self.c;
        out.input[1] = (c[0] * self.first[0] + c[1] * self.first[1] + self.d * impulse).abs();
        for i in 1..k {
            out.state[i] = self.later[i - 1];
            out.input[i + 1] =
                self.constants[i].gamma * out.state[i] + self.constants[i].delta * out.input[i];
        }
        // Advance.
        let (a, b) = (self.a, self.b);
        self.first = [
            a[0][0] * self.first[0] + a[0][1] * self.first[1] + b[0] * impulse,
            a[1][0] * self.first[0] + a[1][1] * self.first[1] + b[1] * impulse,
        ];
        for i in 1..k {
            self.later[i - 1] =
                self.constants[i].q * self.later[i - 1] + self.constants[i].beta * out.input[i];
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
        let mut current = std::vec![v_norm(self.first)];
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
///   `Ref(m) = sum_{t > m} o(t)` ([`Majorants`]), summed exactly to a horizon and closed by the
///   contraction remainder. The tail is at least the first `m` with `gain * Ref(m) < eps / 2`.
/// * **`f32` half.** The kernel's output differs from the reference's by at most
///   `gain * peak * dev(m) + F * a` ([`Deviation`]): relative rounding plus the flush stall. For
///   every peak at or above `p_star = F a / (eps / 2 - gain * dev_sup)` the deviation fits the other
///   `eps / 2` from the tail on; below `p_star` exact rest is proven by the tail instead.
/// * **Rest.** The kernel state of section `k` at `N` is at most
///   `gain * peak * (x_k + E_k) + F * E_abs_k`, capped at the `V`-norm of finite `f32` words; rest
///   follows section by section ([`rest_frames`]).
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
    if sections.is_empty() {
        return Ok(CascadeBound::ZERO);
    }
    let constants: Vec<SectionConstants> = sections.iter().map(SectionConstants::of).collect();
    // A NaN contraction is refused with the rest: only `rho < 1` passes.
    let contracts = |rho: f64| rho.partial_cmp(&1.0) == Some(core::cmp::Ordering::Less);
    if !constants.iter().all(|c| contracts(c.rho_kernel())) {
        return Err(TailBoundError::NotContracting);
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
            return Err(TailBoundError::Horizon);
        }
        checkpoints.push(majorants.clone());
        let mut sum = 0.0;
        for _ in 0..BLOCK {
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
    // The first section's state majorant in closed form ([`Majorants::step`]).
    state_sup[0] = constants[0].beta / (1.0 - constants[0].q);
    let state_sup: Vec<f64> = state_sup.iter().map(|value| value * SLACK).collect();
    let input_sup: Vec<f64> = input_sup.iter().map(|value| value * SLACK).collect();

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
        .find(|&block| suffix[block] * SLACK < threshold)
        .expect("the remainder is below the threshold");
    let t0 = if first_block == 0 {
        0
    } else {
        let block = first_block - 1;
        let mut replay = checkpoints[block].clone();
        let values: Vec<f64> = (0..BLOCK)
            .map(|_| {
                replay.step(&mut frame);
                frame.input[k]
            })
            .collect();
        let mut running = suffix[block + 1];
        let mut first = (block + 1) * BLOCK;
        for index in (0..BLOCK).rev() {
            running += values[index];
            if running * SLACK < threshold {
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
            return Err(TailBoundError::Horizon);
        }
    }
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
                ((state_sup[i] + at_end.error[i]) * gain * peak + f * absolute_error[i]).min(cap)
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
}
