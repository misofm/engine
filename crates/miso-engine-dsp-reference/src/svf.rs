//! Independent `f64` Simper/Zavalishin TPT state-variable filter: design, recurrence, transfer.
//!
//! This module is the single home of the TPT/SVF transfer model in this crate. It models the
//! realization frozen by the master plan (#83 revision 4) §4.2 amendment A1 — the `c1`-storage
//! form — in `f64`:
//!
//! * `g = tan(pi * f0 / fs)`, `k = 1/Q` (per-kind prewarp for the shelves),
//! * `t = g * (g + k)`, `c1 = t / (1 + t)`, `a1 = 1 - c1`, `a2 = g * a1`, `a3 = g * a2`,
//! * output mix `(m0, m1, m2)` per kind (Simper 2013 table),
//! * recurrence `d1 = a2*v3 - c1*ic1`, `v1 = ic1 + d1`, `d2 = a2*ic1 + a3*v3`, `v2 = ic2 + d2`,
//!   `ic1 += d1 + d1`, `ic2 += d2 + d2`, `y = m0*v0 + m1*v1 + m2*v2`.
//!
//! The realization stores `c1` rather than `a1` because the `f32` production kernel loses ~0.6 %
//! of the pole damping at 10 Hz / Q = 18 / 88.2 kHz when `a1 = 1/(1 + t)` is the stored word
//! (master plan §4.2 A1). The `f64` oracle keeps the same storage so its state-space model is the
//! model of the shipped graph and not of an algebraically equivalent one.
//!
//! Nothing here runs on a render path: this is offline `f64` evidence code, so it uses `std`
//! transcendentals, performs no FMA, and applies no denormal flush.
//!
//! Sources: A. Simper, *Solving the continuous SVF equations using trapezoidal integration and
//! equivalent currents* (2013); V. Zavalishin, *The Art of VA Filter Design* §4.4; RBJ Audio EQ
//! Cookbook (the analog prototypes both mappings transcribe).

use crate::Complex64;

/// The seven independent state-variable filter responses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceSvfKind {
    /// Second-order low-pass, mix `(0, 0, 1)`.
    LowPass,
    /// Second-order high-pass, mix `(1, -k, -1)`.
    HighPass,
    /// Second-order band-pass with peak gain `Q`, mix `(0, 1, 0)`.
    BandPass,
    /// Second-order notch, mix `(1, -k, 0)`.
    Notch,
    /// Peaking (bell) equalizer, `k = 1/(Q*A)`, mix `(1, k*(A^2 - 1), 0)`.
    Bell,
    /// Low shelving equalizer, `g = tan(pi f0/fs)/sqrt(A)`, mix `(1, k*(A - 1), A^2 - 1)`.
    LowShelf,
    /// High shelving equalizer, `g = tan(pi f0/fs)*sqrt(A)`, mix `(A^2, k*(1 - A)*A, 1 - A^2)`.
    HighShelf,
}

/// Invalid reference-SVF design input, coefficient calculation, or stability proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceSvfError {
    /// Rate, frequency, gain, or Q was nonfinite or outside its mathematical domain.
    InvalidInput,
    /// An intermediate or final coefficient was nonfinite.
    NonFinite,
    /// The realized state matrix failed strict Jury stability.
    Unstable,
}

/// Simper 2013 coefficients in the master-plan §4.2 A1 storage form.
///
/// `g` and `k` are retained for inspection; the recurrence and the transfer use only
/// `c1`, `a1`, `a2`, `a3` and the output mix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceSvfCoefficients {
    /// Prewarped, kind-specific integrator gain `g`.
    pub g: f64,
    /// Damping word `k` (`1/Q`, or `1/(Q*A)` for the bell).
    pub k: f64,
    /// Stored feedback word `c1 = t/(1 + t)` with `t = g*(g + k)`.
    pub c1: f64,
    /// Derived word `a1 = 1 - c1`.
    pub a1: f64,
    /// Derived word `a2 = g * a1`.
    pub a2: f64,
    /// Derived word `a3 = g * a2`.
    pub a3: f64,
    /// Direct output mix term.
    pub m0: f64,
    /// Band-state output mix term.
    pub m1: f64,
    /// Low-state output mix term.
    pub m2: f64,
}

impl ReferenceSvfCoefficients {
    /// Designs one section from the frozen §4.2 mapping.
    ///
    /// `q` is the RBJ Q for every kind; shelves specified by slope `S` convert with
    /// [`shelf_slope_to_q`] first. `gain_db` is ignored for
    /// [`LowPass`](ReferenceSvfKind::LowPass), [`HighPass`](ReferenceSvfKind::HighPass),
    /// [`BandPass`](ReferenceSvfKind::BandPass) and [`Notch`](ReferenceSvfKind::Notch).
    ///
    /// # Errors
    ///
    /// Returns [`ReferenceSvfError::InvalidInput`] for a nonfinite or out-of-domain argument,
    /// [`ReferenceSvfError::NonFinite`] if any coefficient is nonfinite, and
    /// [`ReferenceSvfError::Unstable`] if the realized state matrix fails the Jury test.
    pub fn design(
        kind: ReferenceSvfKind,
        sample_rate_hz: f64,
        frequency_hz: f64,
        q: f64,
        gain_db: f64,
    ) -> Result<Self, ReferenceSvfError> {
        if !sample_rate_hz.is_finite()
            || sample_rate_hz <= 0.0
            || !frequency_hz.is_finite()
            || frequency_hz <= 0.0
            || frequency_hz >= sample_rate_hz * 0.5
            || !gain_db.is_finite()
            || !q.is_finite()
            || q <= 0.0
        {
            return Err(ReferenceSvfError::InvalidInput);
        }
        let amplitude = 10.0_f64.powf(gain_db / 40.0);
        let warped = (core::f64::consts::PI * frequency_hz / sample_rate_hz).tan();
        let (g, k, m0, m1, m2) = match kind {
            ReferenceSvfKind::LowPass => (warped, 1.0 / q, 0.0, 0.0, 1.0),
            ReferenceSvfKind::HighPass => (warped, 1.0 / q, 1.0, -(1.0 / q), -1.0),
            ReferenceSvfKind::BandPass => (warped, 1.0 / q, 0.0, 1.0, 0.0),
            ReferenceSvfKind::Notch => (warped, 1.0 / q, 1.0, -(1.0 / q), 0.0),
            ReferenceSvfKind::Bell => {
                let k = 1.0 / (q * amplitude);
                (warped, k, 1.0, k * (amplitude * amplitude - 1.0), 0.0)
            }
            ReferenceSvfKind::LowShelf => {
                let k = 1.0 / q;
                (
                    warped / amplitude.sqrt(),
                    k,
                    1.0,
                    k * (amplitude - 1.0),
                    amplitude * amplitude - 1.0,
                )
            }
            ReferenceSvfKind::HighShelf => {
                let k = 1.0 / q;
                (
                    warped * amplitude.sqrt(),
                    k,
                    amplitude * amplitude,
                    k * (1.0 - amplitude) * amplitude,
                    1.0 - amplitude * amplitude,
                )
            }
        };
        // Master plan §4.2 amendment A1 storage: c1 = t/(1 + t), a1 = 1 - c1.
        let t = g * (g + k);
        let c1 = t / (1.0 + t);
        let a1 = 1.0 - c1;
        let a2 = g * a1;
        let a3 = g * a2;
        let coefficients = Self {
            g,
            k,
            c1,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        };
        if ![g, k, c1, a1, a2, a3, m0, m1, m2]
            .into_iter()
            .all(f64::is_finite)
        {
            return Err(ReferenceSvfError::NonFinite);
        }
        if !coefficients.is_strictly_jury_stable() {
            return Err(ReferenceSvfError::Unstable);
        }
        Ok(coefficients)
    }

    /// Returns the analytic state-space model of the realized recurrence.
    #[must_use]
    pub fn state_space(self) -> ReferenceSvfStateSpace {
        ReferenceSvfStateSpace::new(self.c1, self.a2, self.a3, [self.m0, self.m1, self.m2])
    }

    /// Returns whether the realized state matrix meets all strict Jury inequalities.
    ///
    /// The characteristic polynomial of `A` is `z^2 - tr(A) z + det(A)`, so the second-order
    /// strict Jury conditions are `|det| < 1`, `1 - tr + det > 0` and `1 + tr + det > 0`.
    #[must_use]
    pub fn is_strictly_jury_stable(self) -> bool {
        let a00 = 1.0 - 2.0 * self.c1;
        let a01 = -2.0 * self.a2;
        let a10 = 2.0 * self.a2;
        let a11 = 1.0 - 2.0 * self.a3;
        let trace = a00 + a11;
        let determinant = a00 * a11 - a01 * a10;
        determinant.abs() < 1.0
            && 1.0 - trace + determinant > 0.0
            && 1.0 + trace + determinant > 0.0
    }
}

/// Exact-real `H(z) = D + C (zI - A)^-1 B` of the Simper recurrence.
///
/// This is the only TPT/SVF transfer model in this crate. It is derived from the realized
/// recurrence, not from a second hand-written closed form: substituting
/// `v3 = u - ic2`, `d1 = a2*v3 - c1*ic1` and `d2 = a2*ic1 + a3*v3` into `ic += 2d` gives
///
/// ```text
/// A = [[1 - 2*c1, -2*a2], [2*a2, 1 - 2*a3]]        B = [2*a2, 2*a3]
/// C = [m1*a1 + m2*a2, -m1*a2 + m2*(1 - a3)]        D = m0 + m1*a2 + m2*a3
/// ```
///
/// with `a1 = 1 - c1`, because `v1 = a1*ic1 - a2*ic2 + a2*u` and
/// `v2 = a2*ic1 + (1 - a3)*ic2 + a3*u`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceSvfStateSpace {
    a00: f64,
    a01: f64,
    a10: f64,
    a11: f64,
    b0: f64,
    b1: f64,
    c0: f64,
    c1: f64,
    d: f64,
}

impl ReferenceSvfStateSpace {
    /// Builds the model from the stored realization words and the output mix `[m0, m1, m2]`.
    #[must_use]
    pub fn new(c1: f64, a2: f64, a3: f64, mix: [f64; 3]) -> Self {
        let [m0, m1, m2] = mix;
        let a1 = 1.0 - c1;
        Self {
            a00: 1.0 - 2.0 * c1,
            a01: -2.0 * a2,
            a10: 2.0 * a2,
            a11: 1.0 - 2.0 * a3,
            b0: 2.0 * a2,
            b1: 2.0 * a3,
            c0: m1 * a1 + m2 * a2,
            c1: -m1 * a2 + m2 * (1.0 - a3),
            d: m0 + m1 * a2 + m2 * a3,
        }
    }

    /// Returns the complex transfer response at a finite frequency inside Nyquist.
    #[must_use]
    pub fn response(self, rate_hz: f64, frequency_hz: f64) -> Option<Complex64> {
        if !rate_hz.is_finite()
            || !frequency_hz.is_finite()
            || rate_hz <= 0.0
            || !(0.0..=rate_hz * 0.5).contains(&frequency_hz)
        {
            return None;
        }
        let phase = core::f64::consts::TAU * frequency_hz / rate_hz;
        let (zr, zi) = (phase.cos(), phase.sin());
        let m00r = zr - self.a00;
        let m00i = zi;
        let m01r = -self.a01;
        let m10r = -self.a10;
        let m11r = zr - self.a11;
        let m11i = zi;
        let detr = m00r * m11r - m00i * m11i - m01r * m10r;
        let deti = m00r * m11i + m00i * m11r;
        let denominator = detr * detr + deti * deti;
        if denominator == 0.0 || !denominator.is_finite() {
            return None;
        }
        let inv00r = (m11r * detr + m11i * deti) / denominator;
        let inv00i = (m11i * detr - m11r * deti) / denominator;
        let inv01r = (-m01r * detr) / denominator;
        let inv01i = (m01r * deti) / denominator;
        let inv10r = (-m10r * detr) / denominator;
        let inv10i = (m10r * deti) / denominator;
        let inv11r = (m00r * detr + m00i * deti) / denominator;
        let inv11i = (m00i * detr - m00r * deti) / denominator;
        let state0r = inv00r * self.b0 + inv01r * self.b1;
        let state0i = inv00i * self.b0 + inv01i * self.b1;
        let state1r = inv10r * self.b0 + inv11r * self.b1;
        let state1i = inv10i * self.b0 + inv11i * self.b1;
        Some(Complex64 {
            re: self.d + self.c0 * state0r + self.c1 * state1r,
            im: self.c0 * state0i + self.c1 * state1i,
        })
    }

    /// Returns response magnitude in dB, floored only by IEEE zero behavior.
    #[must_use]
    pub fn magnitude_db(self, rate_hz: f64, frequency_hz: f64) -> Option<f64> {
        let response = self.response(rate_hz, frequency_hz)?;
        let magnitude = response.re.hypot(response.im);
        Some(if magnitude == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * magnitude.log10()
        })
    }

    /// Iterates `x' = A x + B u`, `y = C x + D u` from zero state.
    ///
    /// This is the second realization the recurrence self-test compares against: it uses the
    /// state-space words, never the Simper intermediates.
    #[must_use]
    pub fn impulse_response(self, frames: usize) -> Vec<f64> {
        let mut output = Vec::with_capacity(frames);
        let (mut x0, mut x1) = (0.0_f64, 0.0_f64);
        for frame in 0..frames {
            let input = if frame == 0 { 1.0 } else { 0.0 };
            output.push(self.d * input + self.c0 * x0 + self.c1 * x1);
            let next0 = self.a00 * x0 + self.a01 * x1 + self.b0 * input;
            let next1 = self.a10 * x0 + self.a11 * x1 + self.b1 * input;
            x0 = next0;
            x1 = next1;
        }
        output
    }
}

/// `f64` time-domain recurrence in the frozen master-plan §4.2 A1 operation order.
///
/// The `f32` kernel fuses `a2*v3 - c1*ic1` and flushes the retained words; this `f64` twin does
/// neither (there is no `f64` denormal risk at oracle amplitudes and no FMA in the reference).
#[derive(Clone, Copy, Debug)]
pub struct ReferenceSvf {
    coefficients: ReferenceSvfCoefficients,
    ic1: f64,
    ic2: f64,
}

impl ReferenceSvf {
    /// Starts a zero-state section with immutable coefficients.
    #[must_use]
    pub const fn new(coefficients: ReferenceSvfCoefficients) -> Self {
        Self {
            coefficients,
            ic1: 0.0,
            ic2: 0.0,
        }
    }

    /// Applies one frozen recurrence step.
    pub fn process(&mut self, input: f64) -> f64 {
        let c = self.coefficients;
        let v3 = input - self.ic2;
        let d1 = c.a2 * v3 - c.c1 * self.ic1;
        let v1 = self.ic1 + d1;
        let d2 = c.a2 * self.ic1 + c.a3 * v3;
        let v2 = self.ic2 + d2;
        self.ic1 += d1 + d1;
        self.ic2 += d2 + d2;
        c.m0 * input + c.m1 * v1 + c.m2 * v2
    }

    /// Resets both retained words to positive zero.
    pub fn reset(&mut self) {
        self.ic1 = 0.0;
        self.ic2 = 0.0;
    }

    /// Returns the retained `(ic1, ic2)` words.
    #[must_use]
    pub const fn state(self) -> (f64, f64) {
        (self.ic1, self.ic2)
    }

    /// Returns the immutable coefficients of this section.
    #[must_use]
    pub const fn coefficients(self) -> ReferenceSvfCoefficients {
        self.coefficients
    }
}

/// Converts an RBJ shelf slope `S` to the Q the Simper shelf mapping takes.
///
/// `1/Q = sqrt((A + 1/A)*(1/S - 1) + 2)` with `A = 10^(gain_db/40)` — the identity that makes
/// RBJ's `alpha_S` equal `alpha_Q`. Returns `None` for nonfinite or non-positive inputs.
#[must_use]
pub fn shelf_slope_to_q(gain_db: f64, shelf_slope: f64) -> Option<f64> {
    if !gain_db.is_finite() || !shelf_slope.is_finite() || shelf_slope <= 0.0 {
        return None;
    }
    let amplitude = 10.0_f64.powf(gain_db / 40.0);
    let inverse = ((amplitude + 1.0 / amplitude) * (1.0 / shelf_slope - 1.0) + 2.0).sqrt();
    if !inverse.is_finite() || inverse <= 0.0 {
        return None;
    }
    let q = 1.0 / inverse;
    if q.is_finite() && q > 0.0 {
        Some(q)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ReferenceParametricEqCoefficients, ReferenceParametricEqError, ReferenceParametricEqKind,
    };
    use core::f64::consts::{FRAC_1_SQRT_2, PI, TAU};

    // The frozen Issue-042/045 design grid, taken verbatim from the constants that used to sit at
    // `parametric_eq_recurrence_proof.rs:17-21` (now `dsp-research/archive/issue-045/`). It is the
    // 1,488-row grid recorded in `.github/ISSUE_SPECS/045-*.md` (`rows=1488`).
    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    const FREQUENCIES: [f64; 6] = [10.0, 20.0, 100.0, 1_000.0, 10_000.0, 20_000.0];
    const QS: [f64; 4] = [0.1, FRAC_1_SQRT_2, 1.0, 18.0];
    const GAINS: [f64; 5] = [-24.0, -6.0, 0.0, 6.0, 24.0];
    const SLOPES: [f64; 3] = [0.1, 0.5, 1.0];
    const PROBES: usize = 64;
    /// Conditioning constant of the E1 bound; see `RbjProbe::absolute_bound`.
    const CONDITIONING: f64 = 256.0;
    /// `20 / ln 10`, relative error to dB.
    const DECIBELS_PER_NEPER: f64 = 8.685_889_638_065_035;
    /// Flat gate for every row whose design frequency is at or above 1 kHz.
    const FLAT_DECIBEL_TOLERANCE: f64 = 1e-9;
    const IMPULSE_FRAMES: usize = 4_096;

    /// One RBJ cookbook evaluation plus the terms of its own conditioning bound.
    #[derive(Clone, Copy, Debug)]
    struct RbjProbe {
        magnitude: f64,
        numerator: f64,
        denominator: f64,
        b_sum: f64,
        a_sum: f64,
    }

    impl RbjProbe {
        /// Absolute rounding bound on `|H| = |N(z)|/|D(z)|`.
        ///
        /// Each polynomial is a sum of three products, so its absolute rounding error is
        /// `O(eps * sum|coefficient|)`; dividing gives a relative error of
        /// `eps * (sum|b|/|N| + sum|a|/|D|)` and an absolute error of `|H|` times that, i.e.
        /// `eps * (sum|b| + |H| * sum|a|) / |D|`. Both realisations contribute the same order;
        /// `CONDITIONING` covers the constant.
        fn absolute_bound(self) -> f64 {
            CONDITIONING * f64::EPSILON * (self.b_sum + self.magnitude * self.a_sum)
                / self.denominator
        }

        /// The same bound expressed in dB (`20/ln 10` per unit of relative error).
        fn decibel_bound(self) -> f64 {
            DECIBELS_PER_NEPER
                * CONDITIONING
                * f64::EPSILON
                * (self.b_sum / self.numerator + self.a_sum / self.denominator)
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct Row {
        rate: u32,
        kind: ReferenceSvfKind,
        frequency: f64,
        q: f64,
        gain: f64,
        slope: f64,
    }

    impl Row {
        /// The Q the Simper mapping takes for this row (shelves convert their RBJ slope).
        fn svf_q(self) -> f64 {
            match self.kind {
                ReferenceSvfKind::LowShelf | ReferenceSvfKind::HighShelf => {
                    shelf_slope_to_q(self.gain, self.slope).expect("legal shelf slope")
                }
                _ => self.q,
            }
        }

        fn coefficients(self) -> ReferenceSvfCoefficients {
            ReferenceSvfCoefficients::design(
                self.kind,
                f64::from(self.rate),
                self.frequency,
                self.svf_q(),
                self.gain,
            )
            .expect("the frozen grid is legal for the Simper mapping")
        }

        fn rbj_kind(self) -> Option<ReferenceParametricEqKind> {
            Some(match self.kind {
                ReferenceSvfKind::LowPass => ReferenceParametricEqKind::LowPass,
                ReferenceSvfKind::HighPass => ReferenceParametricEqKind::HighPass,
                ReferenceSvfKind::Notch => ReferenceParametricEqKind::Notch,
                ReferenceSvfKind::Bell => ReferenceParametricEqKind::Bell,
                ReferenceSvfKind::LowShelf => ReferenceParametricEqKind::LowShelf,
                ReferenceSvfKind::HighShelf => ReferenceParametricEqKind::HighShelf,
                ReferenceSvfKind::BandPass => return None,
            })
        }

        /// The independent RBJ cookbook evaluation at `frequency_hz`.
        ///
        /// Six kinds reuse `ReferenceParametricEqCoefficients` (its own closed forms and its own
        /// polynomial-in-z evaluation). The band-pass has no parametric-EQ family, so the
        /// constant-skirt cookbook section is written out here and evaluated the same way.
        /// The returned polynomial magnitudes and coefficient sums are used only to derive this
        /// probe's conditioning bound, never as the comparison value.
        fn rbj_probe(self, frequency_hz: f64) -> Result<RbjProbe, ReferenceParametricEqError> {
            let rate = f64::from(self.rate);
            let omega = TAU * frequency_hz / rate;
            let (magnitude, values) = match self.rbj_kind() {
                Some(kind) => {
                    let design = ReferenceParametricEqCoefficients::design(
                        kind,
                        rate,
                        self.frequency,
                        self.gain,
                        self.q,
                        self.slope,
                    )?;
                    (design.magnitude_at_hz(frequency_hz)?, design.values())
                }
                None => {
                    let design_omega = TAU * self.frequency / rate;
                    let alpha = design_omega.sin() / (2.0 * self.q);
                    let a0 = 1.0 + alpha;
                    let values = (
                        self.q * alpha / a0,
                        0.0,
                        -self.q * alpha / a0,
                        -2.0 * design_omega.cos() / a0,
                        (1.0 - alpha) / a0,
                    );
                    (biquad_magnitude(values, omega), values)
                }
            };
            let (b0, b1, b2, a1, a2) = values;
            let (cosine, sine) = (omega.cos(), omega.sin());
            let (cosine2, sine2) = ((2.0 * omega).cos(), (2.0 * omega).sin());
            Ok(RbjProbe {
                magnitude,
                numerator: (b0 + b1 * cosine + b2 * cosine2).hypot(-b1 * sine - b2 * sine2),
                denominator: (1.0 + a1 * cosine + a2 * cosine2).hypot(-a1 * sine - a2 * sine2),
                b_sum: b0.abs() + b1.abs() + b2.abs(),
                a_sum: 1.0 + a1.abs() + a2.abs(),
            })
        }

        fn probes(self) -> Vec<f64> {
            let nyquist_probe = 0.45 * f64::from(self.rate);
            let low = 10.0_f64;
            let mut probes: Vec<f64> = (0..PROBES)
                .map(|index| {
                    let ratio = index as f64 / (PROBES - 1) as f64;
                    low * (nyquist_probe / low).powf(ratio)
                })
                .collect();
            probes.push(self.frequency);
            probes.sort_by(f64::total_cmp);
            probes.dedup_by(|left, right| *left == *right);
            probes
        }
    }

    fn biquad_magnitude(coefficients: (f64, f64, f64, f64, f64), omega: f64) -> f64 {
        let (b0, b1, b2, a1, a2) = coefficients;
        let (cosine, sine) = (omega.cos(), omega.sin());
        let (cosine2, sine2) = ((2.0 * omega).cos(), (2.0 * omega).sin());
        let numerator = (b0 + b1 * cosine + b2 * cosine2).hypot(-b1 * sine - b2 * sine2);
        let denominator = (1.0 + a1 * cosine + a2 * cosine2).hypot(-a1 * sine - a2 * sine2);
        numerator / denominator
    }

    /// The frozen 1,488-row grid plus the band-pass rows the RBJ EQ families do not cover.
    fn rows() -> Vec<Row> {
        let mut output = Vec::new();
        for rate in RATES {
            for frequency in FREQUENCIES {
                for q in QS {
                    for gain in GAINS {
                        output.push(Row {
                            rate,
                            kind: ReferenceSvfKind::Bell,
                            frequency,
                            q,
                            gain,
                            slope: 1.0,
                        });
                    }
                    for kind in [
                        ReferenceSvfKind::LowPass,
                        ReferenceSvfKind::HighPass,
                        ReferenceSvfKind::Notch,
                    ] {
                        output.push(Row {
                            rate,
                            kind,
                            frequency,
                            q,
                            gain: 0.0,
                            slope: 1.0,
                        });
                    }
                }
                for gain in GAINS {
                    for slope in SLOPES {
                        for kind in [ReferenceSvfKind::LowShelf, ReferenceSvfKind::HighShelf] {
                            output.push(Row {
                                rate,
                                kind,
                                frequency,
                                q: 1.0,
                                gain,
                                slope,
                            });
                        }
                    }
                }
            }
        }
        assert_eq!(output.len(), 1_488, "frozen Issue-042/045 grid size");
        for rate in RATES {
            for frequency in FREQUENCIES {
                for q in QS {
                    output.push(Row {
                        rate,
                        kind: ReferenceSvfKind::BandPass,
                        frequency,
                        q,
                        gain: 0.0,
                        slope: 1.0,
                    });
                }
            }
        }
        assert_eq!(output.len(), 1_488 + 96);
        output
    }

    /// E1: the Simper mapping and the RBJ cookbook transcribe the same analog prototype.
    ///
    /// Two derivations meet here: `ReferenceParametricEqCoefficients` (cookbook closed forms
    /// evaluated as a polynomial in `z`) and `ReferenceSvfCoefficients` (the master-plan §4.2
    /// mapping evaluated as `D + C(zI - A)^-1 B`). They share nothing but the `f64`
    /// transcendentals of their inputs.
    ///
    /// Gates: (i) every probe is inside `max(1e-9 dB, this probe's conditioning bound)`;
    /// (ii) every row designed at or above 1 kHz is flat inside 1e-9 dB; (iii) probes where the
    /// cookbook magnitude is below 1e-3 (notch nulls and deep stop bands, where the dB ratio is
    /// meaningless) are compared as absolute magnitudes inside `max(1e-12, the same bound)`.
    #[test]
    fn svf_transfer_matches_rbj_cookbook() {
        let mut worst_db = 0.0_f64;
        let mut worst_db_row = String::new();
        let mut worst_flat = 0.0_f64;
        let mut worst_flat_row = String::new();
        let mut worst_null = 0.0_f64;
        let mut worst_ratio = 0.0_f64;
        let mut compared = 0_usize;
        for row in rows() {
            let rate = f64::from(row.rate);
            let space = row.coefficients().state_space();
            for probe in row.probes() {
                let reference = row.rbj_probe(probe).expect("legal RBJ probe");
                let response = space
                    .response(rate, probe)
                    .expect("legal state-space probe");
                let actual = response.re.hypot(response.im);
                compared += 1;
                if reference.magnitude >= 1e-3 {
                    let error = (DECIBELS_PER_NEPER * (actual / reference.magnitude).ln()).abs();
                    let bound = reference.decibel_bound().max(FLAT_DECIBEL_TOLERANCE);
                    assert!(
                        error <= bound,
                        "dB mismatch {error:e} > {bound:e}: {row:?} probe={probe}"
                    );
                    worst_ratio = worst_ratio.max(error / bound);
                    if error > worst_db {
                        worst_db = error;
                        worst_db_row = format!("{row:?} probe={probe}");
                    }
                    if row.frequency >= 1_000.0 && error > worst_flat {
                        worst_flat = error;
                        worst_flat_row = format!("{row:?} probe={probe}");
                    }
                } else {
                    let error = (actual - reference.magnitude).abs();
                    let bound = reference.absolute_bound().max(1e-12);
                    assert!(
                        error <= bound,
                        "null mismatch {error:e} > {bound:e}: {row:?} probe={probe}"
                    );
                    worst_ratio = worst_ratio.max(error / bound);
                    worst_null = worst_null.max(error);
                }
            }
        }
        assert!(
            worst_flat <= FLAT_DECIBEL_TOLERANCE,
            "rows at or above 1 kHz must be flat inside {FLAT_DECIBEL_TOLERANCE:e} dB, worst {worst_flat:e} at {worst_flat_row}"
        );
        eprintln!(
            "E1 probes={compared} worst_db={worst_db:e} at {worst_db_row}\n\
             E1 worst_flat={worst_flat:e} at {worst_flat_row}\n\
             E1 worst_null={worst_null:e} worst_error_over_bound={worst_ratio:e}"
        );
    }

    #[test]
    fn svf_recurrence_matches_state_space_impulse() {
        let mut worst = 0.0_f64;
        for row in rows() {
            let coefficients = row.coefficients();
            let mut section = ReferenceSvf::new(coefficients);
            let modelled = coefficients.state_space().impulse_response(IMPULSE_FRAMES);
            for (frame, expected) in modelled.into_iter().enumerate() {
                let input = if frame == 0 { 1.0 } else { 0.0 };
                worst = worst.max((section.process(input) - expected).abs());
            }
        }
        eprintln!("E2 worst_abs={worst:e}");
        assert!(worst <= 1e-12, "recurrence vs state space worst {worst:e}");
    }

    #[test]
    fn zero_gain_bell_and_shelves_are_exact_identity() {
        for kind in [
            ReferenceSvfKind::Bell,
            ReferenceSvfKind::LowShelf,
            ReferenceSvfKind::HighShelf,
        ] {
            let space = ReferenceSvfCoefficients::design(kind, 48_000.0, 1_000.0, 1.0, 0.0)
                .expect("legal design")
                .state_space();
            assert_eq!(space.magnitude_db(48_000.0, 1_000.0), Some(0.0));
            assert_eq!(space.magnitude_db(48_000.0, 20.0), Some(0.0));
        }
        let _ = PI;
    }
}
