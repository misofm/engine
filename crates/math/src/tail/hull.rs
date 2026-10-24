//! The word-hull cascade bound (issue #1372, tranche a): certified tail and exact-rest bounds for
//! a cascade of TPT SVF sections whose damping `k` varies, under one common quadratic norm.
//!
//! Control-plane only, like the rest of [`crate::tail`]. The derivation is
//! `docs/derivations/1372-parametric-eq-tail-and-rest.md`, part 1.
//!
//! # The norm
//!
//! The `V`-norm of the parent module is the eigenbasis of every Butterworth (`k = sqrt(2)`)
//! design. A section family whose `k` varies has no shared eigenbasis, so this module states a
//! norm per hull: `||x||_Q = ||L x||_2` with `L = [[1, l12], [0, l22]]`, `l22 > 0`
//! ([`HullNorm`]; `Q = L^T L`). The `V`-norm is `l12 = l22 = 1/sqrt(2)`. For one design of damping
//! `k < 2`, `l12 = k / 2`, `l22 = sqrt(1 - k^2 / 4)` makes `L A L^-1` a scaled rotation, so the
//! norm of `A^n` is the pole radius to the `n`-th power.
//!
//! # The certificate
//!
//! `A(w) = I - 2 N(w)` with `N(w) = [[c1, a2], [-a2, a3]]` is affine in the words
//! `w = (c1, a2, a3)`, so `w -> ||A(w)||_Q` is convex: over the convex hull of a set of designed
//! words it is largest at one of them, and a per-word box of half-widths `m` around the hull adds
//! at most `sup_box 2 ||N(e)||_Q`, itself largest at one of the box's eight corners
//! ([`HullNorm::contraction`]). [`common_lyapunov`] searches `L` and certifies the result this
//! way.
//!
//! # The identity is out of scope
//!
//! The identity section (`w = 0`, `A = I`) has norm `1` in every norm, and a ramp between it and a
//! design does not contract at small design weight once the kernel's rounding is counted (the
//! derivation's "The identity is out of scope"). A section whose words can ramp toward the
//! identity is refused ([`HullSection::identity_reachable`],
//! [`TailBoundError::IdentityReachable`]); its hull contains no identity vertex, so a uniform
//! contraction below `1` covers every word it can load.

use std::vec::Vec;

use super::{
    CascadeBound, Conversion, EnvelopeSection, F32_MAX, FlushLaw, HORIZON_LIMIT, HalfSystem,
    LIVE_STATE_LIMIT, LiveComposition, Propagator, SLACK, SquareMatrix, SvfWords, TAIL_FLOOR, TAU,
    TailBoundError, U, U64, carry, dot, nan_max, output_rounding_in, rest_frames,
    spectral_norm_bound, state_rounding_in,
};

/// A state norm `||x||_Q = ||L x||_2`, `L = [[1, l12], [0, l22]]`, with its certified conversion
/// constants (issue #1372).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HullNorm {
    l12: f64,
    l22: f64,
    conversion: Conversion,
}

impl HullNorm {
    /// The norm of `L = [[1, l12], [0, l22]]`.
    ///
    /// # Errors
    ///
    /// [`TailBoundError::Invalid`] when `l12` is not finite or `l22` is not finite and positive.
    pub fn new(l12: f64, l22: f64) -> Result<Self, TailBoundError> {
        if !(l12.is_finite() && l22.is_finite() && l22 > 0.0) {
            return Err(TailBoundError::Invalid);
        }
        // `L` is exact; `L^-1 = [[1, -l12 / l22], [0, 1 / l22]]` rounds once per entry.
        let from_words = spectral_norm_bound([[1.0, l12], [0.0, l22]], [[0.0; 2]; 2]);
        let inverse = [[1.0, -l12 / l22], [0.0, 1.0 / l22]];
        let error = inverse.map(|row| row.map(|value| 2.0 * U64 * value.abs()));
        let to_words = spectral_norm_bound(inverse, error);
        let kappa = from_words * to_words * SLACK;
        if !(from_words.is_finite() && to_words.is_finite() && kappa.is_finite()) {
            return Err(TailBoundError::Invalid);
        }
        Ok(Self {
            l12,
            l22,
            conversion: Conversion {
                from_words,
                to_words,
                kappa,
            },
        })
    }

    /// The eigenbasis norm of a design of damping `k` (`0 <= k < 2`): `l12 = k / 2`,
    /// `l22 = sqrt(1 - k^2 / 4)`, in which every design of that damping, at any cutoff, is a
    /// scaled rotation.
    ///
    /// # Errors
    ///
    /// [`TailBoundError::Invalid`] when `k` is not in `[0, 2)`.
    pub fn of_damping(k: f64) -> Result<Self, TailBoundError> {
        if !(0.0..2.0).contains(&k) {
            return Err(TailBoundError::Invalid);
        }
        let l12 = 0.5 * k;
        Self::new(l12, crate::sqrt((1.0 - l12) * (1.0 + l12)))
    }

    /// `l12`.
    #[must_use]
    pub fn l12(&self) -> f64 {
        self.l12
    }

    /// `l22`.
    #[must_use]
    pub fn l22(&self) -> f64 {
        self.l22
    }

    /// At least `||L||_2`: `||x||_Q <= from_words ||x||_2`.
    #[must_use]
    pub fn from_words(&self) -> f64 {
        self.conversion.from_words
    }

    /// At least `||L^-1||_2`: every word of a state is at most `to_words ||x||_Q` in magnitude.
    #[must_use]
    pub fn to_words(&self) -> f64 {
        self.conversion.to_words
    }

    /// At least `from_words * to_words`.
    #[must_use]
    pub fn kappa(&self) -> f64 {
        self.conversion.kappa
    }

    /// A certified upper bound on `||x||_Q`. `x1 + l12 x2` may cancel, so its error is charged
    /// to the magnitudes of its terms.
    #[must_use]
    pub fn norm(&self, x: [f64; 2]) -> f64 {
        let first = x[0] + self.l12 * x[1];
        let second = self.l22 * x[1];
        let error = 4.0 * U64 * (x[0].abs() + (self.l12 * x[1]).abs());
        (crate::sqrt(first * first + second * second) + error) * SLACK
    }

    /// A certified upper bound on the dual norm of an output row `c`: the smallest `gamma` with
    /// `|c . x| <= gamma ||x||_Q`, `||c L^-1||_2` with `c L^-1 = (c1, (c2 - l12 c1) / l22)`.
    #[must_use]
    pub fn dual_norm(&self, c: [f64; 2]) -> f64 {
        let second = (c[1] - self.l12 * c[0]) / self.l22;
        let error = 4.0 * U64 * (c[1].abs() + (self.l12 * c[0]).abs()) / self.l22;
        (crate::sqrt(c[0] * c[0] + second * second) + error) * SLACK
    }

    /// `P = L N(w) L^-1` for `N(w) = [[c1, a2], [-a2, a3]]`, in closed form with componentwise
    /// error bounds. With `L^-1 = adj(L) / l22`, `adj(L) = [[l22, -l12], [0, 1]]`:
    /// `P11 = c1 - l12 a2`, `P12 = (a2 + l12 a3 - l12 c1 + l12^2 a2) / l22`, `P21 = -l22 a2`,
    /// `P22 = l12 a2 + a3`. Each entry passes through at most seven roundings, so it errs by at
    /// most `16 u` times the sum of its terms' magnitudes.
    fn transformed(&self, words: [f64; 3]) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
        let [c1, a2, a3] = words;
        let (l12, l22) = (self.l12, self.l22);
        let p = [
            [
                c1 - l12 * a2,
                (a2 + l12 * a3 - l12 * c1 + l12 * l12 * a2) / l22,
            ],
            [-l22 * a2, l12 * a2 + a3],
        ];
        let terms = [
            [
                c1.abs() + (l12 * a2).abs(),
                (a2.abs() + (l12 * a3).abs() + (l12 * c1).abs() + (l12 * l12 * a2).abs()) / l22,
            ],
            [(l22 * a2).abs(), (l12 * a2).abs() + a3.abs()],
        ];
        (p, terms.map(|row| row.map(|value| 16.0 * U64 * value)))
    }

    /// A certified upper bound on `||A(w)||_Q`, `A(w) = I - 2 N(w)` (exact arithmetic on the
    /// words `w`, no kernel rounding). The largest singular value of `M = I - 2 P` is
    /// `(sqrt((a + d)^2 + (c - b)^2) + sqrt((a - d)^2 + (b + c)^2)) / 2`, formed from `P` without
    /// forming `1 - 2 P_ii`: `a + d = 2 - 2 (P11 + P22)`, `c - b = 2 (P12 - P21)`,
    /// `a - d = 2 (P22 - P11)`, `b + c = -2 (P12 + P21)`. Each operation acts on computed values
    /// it takes as exact, so the result is within `(1 + u)^8` of the singular value of the computed
    /// `P`; the error of `P` adds at most `2 ||E||_F`.
    #[must_use]
    pub fn operator_norm(&self, words: [f64; 3]) -> f64 {
        let (p, error) = self.transformed(words);
        let [[p11, p12], [p21, p22]] = p;
        let trace = 2.0 - 2.0 * (p11 + p22);
        let skew = 2.0 * (p12 - p21);
        let spread = 2.0 * (p22 - p11);
        let shear = 2.0 * (p12 + p21);
        let sum = crate::sqrt(trace * trace + skew * skew);
        let difference = crate::sqrt(spread * spread + shear * shear);
        let largest = 0.5 * (sum + difference) * (1.0 + 32.0 * U64);
        let perturbation = 2.0 * frobenius(error);
        (largest + perturbation) * (1.0 + 32.0 * U64)
    }

    /// A certified upper bound on `sup 2 ||N(e)||_Q` over the box `|e_i| <= box_[i]`: how much a
    /// per-word perturbation of the words can add to `||A||_Q`. The norm is convex in `e`, so the
    /// supremum is at one of the box's eight corners.
    #[must_use]
    pub fn box_norm(&self, box_: [f64; 3]) -> f64 {
        let mut worst = 0.0_f64;
        for signs in 0..8_u32 {
            let sign = |bit: u32| if signs & (1 << bit) == 0 { 1.0 } else { -1.0 };
            let e = [sign(0) * box_[0], sign(1) * box_[1], sign(2) * box_[2]];
            let (p, error) = self.transformed(e);
            worst = nan_max(worst, 2.0 * spectral_norm_bound(p, error) * SLACK);
        }
        worst
    }

    /// A certified upper bound on `||A(w)||_Q` over every word `w` within the box `margin` of the
    /// convex hull of `vertices` (exact arithmetic, no kernel rounding): the largest vertex value
    /// plus [`Self::box_norm`] (the module note). A NaN when a value is not finite; `0` for no
    /// vertex.
    #[must_use]
    pub fn contraction(&self, vertices: &[[f64; 3]], margin: [f64; 3]) -> f64 {
        let largest = vertices.iter().fold(0.0_f64, |sup, words| {
            nan_max(sup, self.operator_norm(*words))
        });
        (largest + self.box_norm(margin)) * SLACK
    }

    /// [`crate::tail::state_rounding`] in this norm: the `f32` state step's rounding, relative to
    /// the state and to the input, for words of the stated magnitudes, contraction `q` and input
    /// column `beta` in this norm.
    #[must_use]
    pub fn state_rounding(&self, words: [f64; 3], q: f64, beta: f64) -> (f64, f64) {
        state_rounding_in(words, q, beta, &self.conversion)
    }

    /// [`crate::tail::output_rounding`] in this norm.
    #[must_use]
    pub fn output_rounding(&self, words: &SvfWords) -> (f64, f64) {
        output_rounding_in(words, &self.conversion)
    }
}

/// `||E||_F`, rounded up.
fn frobenius(e: [[f64; 2]; 2]) -> f64 {
    crate::sqrt(e[0][0] * e[0][0] + e[0][1] * e[0][1] + e[1][0] * e[1][0] + e[1][1] * e[1][1])
        * SLACK
}

/// A norm and the contraction it certifies over a word hull ([`common_lyapunov`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HullCertificate {
    /// The norm.
    pub norm: HullNorm,
    /// [`HullNorm::contraction`] over the hull and its margin: below `1`.
    pub contraction: f64,
}

/// The most eigenbasis candidates [`common_lyapunov`] starts from.
const CANDIDATES: usize = 64;

/// The most objective evaluations of [`common_lyapunov`]'s pattern search.
const SEARCH_EVALUATIONS: usize = 4096;

/// A common quadratic Lyapunov norm for every word within the box `margin` of the convex hull of
/// `vertices` (issue #1372; Shorten and Narendra 2003 for the existence of such a function for
/// 2x2 systems): the norm, and the certified contraction [`HullNorm::contraction`] it gives,
/// below `1`.
///
/// The search only chooses among norms; the certificate is [`HullNorm::contraction`]'s, so the
/// search affects the bound's tightness, never its soundness. It starts from the identity and
/// from the eigenbasis norm ([`HullNorm::of_damping`]) of up to 64 vertices, keeps the best, and
/// refines `(l12, ln l22)` by a compass search whose step halves from `1/4` to `2^-30`.
///
/// # Errors
///
/// [`TailBoundError::Invalid`] for no vertex or a word or margin that is not finite (a margin also
/// non-negative); [`TailBoundError::NotContracting`] when no norm found certifies a contraction
/// below `1` (a hull that holds the identity never does).
pub fn common_lyapunov(
    vertices: &[[f64; 3]],
    margin: [f64; 3],
) -> Result<HullCertificate, TailBoundError> {
    let finite = vertices.iter().flatten().all(|word| word.is_finite());
    let margin_valid = margin
        .iter()
        .all(|value| value.is_finite() && *value >= 0.0);
    if vertices.is_empty() || !finite || !margin_valid {
        return Err(TailBoundError::Invalid);
    }
    let evaluate = |l12: f64, log_l22: f64| -> Option<(HullNorm, f64)> {
        let norm = HullNorm::new(l12, crate::exp(log_l22)).ok()?;
        let q = norm.contraction(vertices, margin);
        q.is_finite().then_some((norm, q))
    };
    let mut best: Option<(f64, f64, HullNorm, f64)> = None;
    let consider = |l12: f64, log_l22: f64, best: &mut Option<(f64, f64, HullNorm, f64)>| {
        if let Some((norm, q)) = evaluate(l12, log_l22)
            && best.as_ref().is_none_or(|(_, _, _, least)| q < *least)
        {
            *best = Some((l12, log_l22, norm, q));
        }
    };
    consider(0.0, 0.0, &mut best);
    let stride = vertices.len().div_ceil(CANDIDATES).max(1);
    for words in vertices.iter().step_by(stride) {
        let [c1, a2, a3] = *words;
        // The design's damping: `(c1 - a3) / a2 = k` for every TPT design.
        let l12 = (c1 - a3) / (2.0 * a2);
        if a2 > 0.0 && l12.abs() < 1.0 {
            let l22 = crate::sqrt((1.0 - l12) * (1.0 + l12));
            consider(l12, crate::log(l22), &mut best);
        }
    }
    let Some((mut x, mut y, _, _)) = best else {
        return Err(TailBoundError::NotContracting);
    };
    let mut step = 0.25_f64;
    let mut evaluations = 0_usize;
    let directions = [
        (1.0, 0.0),
        (-1.0, 0.0),
        (0.0, 1.0),
        (0.0, -1.0),
        (1.0, 1.0),
        (1.0, -1.0),
        (-1.0, 1.0),
        (-1.0, -1.0),
    ];
    while step > 1.0 / 1_073_741_824.0 && evaluations < SEARCH_EVALUATIONS {
        let current = best.as_ref().map_or(f64::INFINITY, |entry| entry.3);
        for (dx, dy) in directions {
            consider(x + dx * step, y + dy * step, &mut best);
            evaluations += 1;
        }
        let entry = best.as_ref().expect("a candidate was kept");
        if entry.3 < current {
            (x, y) = (entry.0, entry.1);
        } else {
            step *= 0.5;
        }
    }
    let (_, _, norm, contraction) = best.expect("a candidate was kept");
    if contraction.partial_cmp(&1.0) != Some(core::cmp::Ordering::Less) {
        return Err(TailBoundError::NotContracting);
    }
    Ok(HullCertificate { norm, contraction })
}

/// One section of a hull cascade: the kernel's suprema over every word it can load, in the
/// cascade's [`HullNorm`], and whether its words can ramp toward the identity section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HullSection {
    /// The kernel's suprema ([`EnvelopeSection`]), every norm the cascade's: `rho_ramp` over every
    /// word reachable while a ramp may be in flight (for a hull, [`HullCertificate::contraction`]
    /// with the ramp words' margin plus the state rounding), `rho_settled` over every settled
    /// design.
    pub envelope: EnvelopeSection,
    /// Whether a control history can ramp this section's words toward the identity section (an
    /// enable or kind change that ramps to `c1 = a2 = a3 = 0`). Such a section is refused
    /// ([`TailBoundError::IdentityReachable`]): no hull bound covers it.
    ///
    /// This is the caller's attestation, not decided from the hull: the module cannot see the
    /// kernel's enable, kind and retarget rules, so each caller crate tests its own claim. The
    /// envelope carries the companion obligation: every word triple the kernel can load under its
    /// own per-word ramp and retarget rule lies in hull + margin, and the state rounding is taken
    /// at word magnitudes that are suprema over hull + margin (the derivation's "The caller's
    /// obligation").
    pub identity_reachable: bool,
}

/// The hull cascade's propagation, shared by its bounds.
struct HullBound<'a> {
    sections: Vec<EnvelopeSection>,
    conversion: Conversion,
    law: &'a FlushLaw,
    ramp_frames: u64,
    /// The zero-input step while a ramp may be in flight, on `(z_1, ..., z_K, 1)`, no drive.
    window: Propagator,
    /// The zero-input step of the settled words, no drive.
    settled: Propagator,
    /// The output's coefficients on `(z, 1)` with the input zero.
    row: Vec<f64>,
    /// The state invariant before `N` per unit of the input's peak (times the gain).
    relative: Vec<f64>,
    /// The output invariant per unit of the input's peak.
    relative_output: f64,
    /// The state invariant of the flush drive.
    flush: Vec<f64>,
    /// `sigma`: the output invariant of the flush drive, which bounds the flush part of the output
    /// at every frame.
    stall: f64,
}

/// The invariant `z_k = (input_k v_k + f) / (1 - rho_k)`, `v_(k+1) = output_state_k z_k +
/// output_input_k v_k + f`, from `v_1`, at the ramp contractions: per section and at the output.
fn invariant(sections: &[EnvelopeSection], first: f64, f: f64) -> (Vec<f64>, f64) {
    let mut v = first;
    let mut z = Vec::with_capacity(sections.len());
    for section in sections {
        let state = (section.input * v + f) / (1.0 - section.rho_ramp) * SLACK;
        z.push(state);
        v = (section.output_state * state + section.output_input * v + f) * SLACK;
    }
    (z, v)
}

impl<'a> HullBound<'a> {
    fn new(
        sections: &[HullSection],
        norm: &HullNorm,
        law: &'a FlushLaw,
        ramp_frames: u64,
    ) -> Result<Self, TailBoundError> {
        if sections.iter().any(|section| section.identity_reachable) {
            return Err(TailBoundError::IdentityReachable);
        }
        let envelopes: Vec<EnvelopeSection> =
            sections.iter().map(|section| section.envelope).collect();
        let valid = |value: f64| value.is_finite() && value >= 0.0;
        let finite = envelopes.iter().all(|e| {
            valid(e.rho_ramp)
                && valid(e.rho_settled)
                && valid(e.input)
                && valid(e.output_state)
                && valid(e.output_input)
        });
        if !finite {
            return Err(TailBoundError::Invalid);
        }
        if !envelopes
            .iter()
            .all(|e| e.rho_ramp < 1.0 && e.rho_settled <= e.rho_ramp)
        {
            return Err(TailBoundError::NotContracting);
        }
        let k = envelopes.len();
        let ramp: Vec<f64> = envelopes.iter().map(|e| e.rho_ramp).collect();
        let settled: Vec<f64> = envelopes.iter().map(|e| e.rho_settled).collect();
        let window = Propagator::new(&envelopes, &ramp, 0, 0.0);
        let settled = Propagator::new(&envelopes, &settled, 0, 0.0);
        let row = Propagator::input_rows(&envelopes, 0, 0.0).swap_remove(k);
        let (relative, relative_output) = invariant(&envelopes, 1.0, 0.0);
        let f = law.per_step_in(&norm.conversion);
        let (flush, stall) = invariant(&envelopes, 0.0, f);
        if !(relative_output.is_finite() && stall.is_finite()) {
            return Err(TailBoundError::Horizon);
        }
        Ok(Self {
            sections: envelopes,
            conversion: norm.conversion,
            law,
            ramp_frames,
            window,
            settled,
            row,
            relative,
            relative_output,
            flush,
            stall,
        })
    }

    /// The relative state bound at `N` for the input scale `g`, with the constant component.
    fn start(&self, g: f64) -> Vec<f64> {
        self.relative
            .iter()
            .map(|z| z * g * SLACK)
            .chain([1.0])
            .collect()
    }

    /// The first frame (from `N`) from which the relative output bound stays below `limit`, for
    /// the input scale `g`.
    ///
    /// Frame by frame from the invariant at `N`: the window's step for frames `0..=ramp_frames`,
    /// the settled step after. Once a settled step does not raise any component of the computed
    /// bound `x` (`fl(S x) <= x`), the exact bounds of every later frame `j` steps on are at most
    /// `S^j x`, which does not increase in `j` (`S^(j+1) x = S^j (S x) <= S^j x`, `S >= 0`); its
    /// first value below `limit` is found by an exponential search and a bisection over powers
    /// that round up, and any power found below `limit` holds for every later frame.
    fn crossing(&self, g: f64, limit: f64) -> Result<u64, TailBoundError> {
        let k = self.sections.len();
        let below = |value: f64| value.partial_cmp(&limit) == Some(core::cmp::Ordering::Less);
        let mut state = self.start(g);
        let mut tail = 0_u64;
        let mut frame = 0_u64;
        loop {
            if !below(dot(&self.row, &state)) {
                tail = frame + 1;
            }
            let settled = frame > self.ramp_frames;
            let next = if settled {
                self.settled.apply(&state)
            } else {
                self.window.apply(&state)
            };
            if settled && next[..k].iter().zip(&state[..k]).all(|(n, s)| n <= s) {
                if tail <= frame {
                    return Ok(tail);
                }
                let at = |steps: u64| {
                    let mut carried = self.settled.power_apply(steps, &state);
                    carried[..k].iter_mut().for_each(|value| *value += TAU);
                    dot(&self.row, &carried)
                };
                let (mut low, mut high) = (0_u64, 1_u64);
                while !below(at(high)) {
                    low = high;
                    high = high
                        .checked_mul(2)
                        .filter(|value| frame + value < HORIZON_LIMIT)
                        .ok_or(TailBoundError::Horizon)?;
                }
                while high - low > 1 {
                    let middle = low + (high - low) / 2;
                    if below(at(middle)) {
                        high = middle;
                    } else {
                        low = middle;
                    }
                }
                return Ok(frame + high);
            }
            state = next;
            frame += 1;
            if frame >= HORIZON_LIMIT {
                return Err(TailBoundError::Horizon);
            }
        }
    }

    /// The frame (from `N`) at which every section is proven at rest for the input scale `g`:
    /// the kernel state at `N` is at most `g z_rel + z_flush` (capped at the norm of finite `f32`
    /// words), then rest section by section ([`rest_frames`]).
    fn rest(&self, g: f64) -> Result<u64, TailBoundError> {
        let cap = self.conversion.from_words * core::f64::consts::SQRT_2 * F32_MAX;
        let state: Vec<f64> = self
            .relative
            .iter()
            .zip(&self.flush)
            .map(|(relative, flush)| (relative * g + flush).min(cap) * SLACK)
            .collect();
        rest_frames(
            &self.sections,
            &state,
            self.law,
            self.ramp_frames,
            &self.conversion,
        )
    }
}

/// The tail and rest bound of a cascade of hull sections (issue #1372;
/// `docs/derivations/1372-parametric-eq-tail-and-rest.md`, part 1), with the meaning of every
/// [`CascadeBound`] field. `sections` are in signal order, every envelope in `norm`; `gain` bounds
/// the magnitude of the input word ahead of them; `ramp_frames` is the longest a ramp can stay in
/// flight after the input's end; `peaks` are the two stated rest peaks.
///
/// * **Before `N`.** Every section's kernel state obeys `z' <= rho_ramp z + input v + F` with its
///   input `v` at most the previous section's output bound, so `z <= (input v + F) / (1 - rho)`
///   is invariant from the zero state, for every history.
/// * **After `N`.** The input is zero; the same system, the window's contractions for
///   `ramp_frames + 1` frames, then the settled ones (the derivation's "The cascade bound").
/// * **Tail, stall, rest.** `T_decay` where the relative output bound stays below `eps / 2`;
///   `P*` from the flush's invariant output `sigma` (the flush part never exceeds it); rest section
///   by section, each adding `N_SILENCE`.
///
/// # Errors
///
/// [`TailBoundError::IdentityReachable`] when a section can ramp toward the identity;
/// [`TailBoundError::Invalid`] for a constant that is not finite and non-negative;
/// [`TailBoundError::NotContracting`] when a ramp contraction is not below `1` or a settled one
/// is above it; otherwise as [`crate::tail::live_cascade`].
pub fn hull_cascade(
    sections: &[HullSection],
    norm: &HullNorm,
    gain: f64,
    law: &FlushLaw,
    ramp_frames: u64,
    peaks: [f64; 2],
) -> Result<CascadeBound, TailBoundError> {
    if !(gain.is_finite() && gain >= 0.0) {
        return Err(TailBoundError::Invalid);
    }
    let bound = HullBound::new(sections, norm, law, ramp_frames)?;
    if sections.is_empty() {
        return Ok(CascadeBound::ZERO);
    }
    let gain = gain * (1.0 + U);
    let tail = bound.crossing(gain, TAIL_FLOOR / 2.0)?;
    let p_star = bound.stall / (TAIL_FLOOR / 2.0) * SLACK;
    if !p_star.is_finite() {
        return Err(TailBoundError::Horizon);
    }
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
        composition: None,
    })
}

/// The frame (from `N`) from which [`hull_cascade`]'s relative output bound stays below
/// `(TAIL_FLOOR / 2) 10^-decades` per unit of the peak (`decades = 0` gives `T_decay`): evidence
/// that [`hull_cascade_composition`]'s `D` covers the bound's own crossings.
///
/// # Errors
///
/// As [`hull_cascade`].
pub fn hull_cascade_crossing(
    sections: &[HullSection],
    norm: &HullNorm,
    gain: f64,
    law: &FlushLaw,
    ramp_frames: u64,
    decades: u64,
) -> Result<u64, TailBoundError> {
    if !(gain.is_finite() && gain >= 0.0) {
        return Err(TailBoundError::Invalid);
    }
    let bound = HullBound::new(sections, norm, law, ramp_frames)?;
    if sections.is_empty() {
        return Ok(0);
    }
    let limit = TAIL_FLOOR / 2.0 * crate::pow(10.0, -(decades as f64));
    bound.crossing(gain * (1.0 + U), limit)
}

/// The composition values of a hull cascade (#1379 Amendment 1 H1, with the meaning of
/// [`LiveComposition`]'s fields), for the input word's magnitude `gain`:
///
/// * `g_p = g_t`: the output invariant per unit of the peak (a late input from a zero state is
///   bounded by the same invariant); `sigma_p = sigma_t`: the flush's invariant output.
/// * `D`: one decay certificate on the settled system (#1465's, lower triangular in section
///   order), anchored at `T` or at the window's end, whichever is later.
///
/// `Ok(None)` when the certificate is not verified, its start is outside the carry's rounding
/// argument, or `D` is not below the horizon. An empty cascade has no values (its caller states
/// a gain-only section's).
///
/// # Errors
///
/// As [`hull_cascade`].
pub fn hull_cascade_composition(
    sections: &[HullSection],
    norm: &HullNorm,
    gain: f64,
    law: &FlushLaw,
    ramp_frames: u64,
) -> Result<Option<LiveComposition>, TailBoundError> {
    if !(gain.is_finite() && gain >= 0.0) {
        return Err(TailBoundError::Invalid);
    }
    let bound = HullBound::new(sections, norm, law, ramp_frames)?;
    let k = sections.len();
    if k == 0 {
        return Ok(None);
    }
    let gain = gain * (1.0 + U);
    let tail = bound.crossing(gain, TAIL_FLOOR / 2.0)?;
    // The bound at the window's end, frame `ramp_frames + 1`.
    let mut state = bound.start(gain);
    for _ in 0..=ramp_frames {
        state = bound.window.apply(&state);
    }
    let start = &state[..k];
    if start
        .iter()
        .any(|value| value.is_nan() || *value > LIVE_STATE_LIMIT)
    {
        return Ok(None);
    }
    let n = k + 1;
    let step = SquareMatrix {
        n: k,
        m: (0..k)
            .flat_map(|i| bound.settled.m[i * n..i * n + k].iter().copied())
            .collect(),
    };
    let anchor = tail.max(ramp_frames + 1);
    let at_anchor: Vec<f64> = carry(&step, anchor - (ramp_frames + 1), start)
        .into_iter()
        .map(|value| value.max(TAU))
        .collect();
    let order: Vec<usize> = (0..k).collect();
    let half = HalfSystem {
        step: &step,
        order: &order,
        state: &at_anchor,
        row: &bound.row[..k],
        log_threshold: crate::log(TAIL_FLOOR / 2.0),
        offset: anchor - tail,
    };
    let Some(group) = half.certificate() else {
        return Ok(None);
    };
    let decay = group.decay();
    let peak_gain = gain * bound.relative_output * SLACK;
    if decay >= HORIZON_LIMIT || !peak_gain.is_finite() {
        return Ok(None);
    }
    Ok(Some(LiveComposition {
        decay,
        groups: std::vec![group],
        peak_gain,
        tail_gain: peak_gain,
        peak_stall: bound.stall,
        tail_stall: bound.stall,
    }))
}
