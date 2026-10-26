//! Independent offline `f64` oracle for BS.1770-5 loudness and EBU Tech 3342 loudness range, and
//! the EBU Tech 3341/3342 minimum-requirement test signals (issue #2115).
//!
//! Nothing here is shared with `crates/loudness`. The K-weighting is designed from the published
//! analog-prototype constants by its own code, the filters run as direct form I over the whole
//! signal, and gating is exact: every block level is kept and compared, with no histogram.
//!
//! # K-weighting design
//!
//! BS.1770-5 Annex 1 tabulates the two K-weighting biquads at 48 kHz only. Both stages are the
//! bilinear transform, prewarped at the stage's `f0`, of a normalized analog prototype
//! (`w0 = 1`, `K = tan(pi f0 / fs)`):
//!
//! ```text
//! H1(s) = (Vh s^2 + (Vb / Q1) s + 1) / (s^2 + s / Q1 + 1)      Vh = 10^(G/20), Vb = Vh^E
//! H2(s) = g48 s^2 / (s^2 + s / Q2 + 1)                          g48 = 1 + K48/Q2 + K48^2
//! ```
//!
//! with the constants of [`REFERENCE_K_PROTOTYPE`]. `g48` is stage 2's denominator `a0` at 48 kHz,
//! so stage 2's numerator is exactly `1, -2, 1` at 48 kHz, as the standard's table has it, and the
//! passband gain stays the 48 kHz one (+0.0433 dB) at every rate.
//!
//! # Levels
//!
//! A block of `N` frames with K-weighted squared sums `E_L`, `E_R` has the mean square
//! `z = (E_L + E_R) / N` (both channel weights 1) and the level `-0.691 + 10 log10 z` LUFS. A
//! zero-energy block has the level `-inf` here; the oracle is never asked to print one.
//!
//! # Gating
//!
//! Integrated loudness is BS.1770-5's: 400 ms blocks every 100 ms, absolute gate `l > -70`,
//! relative gate 10 LU below the mean of the absolute-gated blocks, `l > gamma_r`. Loudness range
//! is Tech 3342 section 5's MATLAB function line for line: 3 s blocks every 100 ms, `l >= -70`,
//! `l >= power mean - 20`, sort, and ranks `round((n - 1) p / 100 + 1)`.

/// The analog-prototype constants that reproduce the BS.1770-5 48 kHz table to about `1e-15`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceKPrototype {
    /// Stage 1 (high shelf) centre frequency, Hz.
    pub shelf_hz: f64,
    /// Stage 1 high-frequency gain, dB.
    pub shelf_gain_db: f64,
    /// Stage 1 quality factor.
    pub shelf_q: f64,
    /// Stage 1 band-gain exponent: `Vb = Vh^E`.
    pub shelf_band_exponent: f64,
    /// Stage 2 (high-pass) corner frequency, Hz.
    pub highpass_hz: f64,
    /// Stage 2 quality factor.
    pub highpass_q: f64,
}

/// The constants of the K-weighting prototype, typed from the #2115 decision record.
pub const REFERENCE_K_PROTOTYPE: ReferenceKPrototype = ReferenceKPrototype {
    shelf_hz: 1_681.974_450_955_533,
    shelf_gain_db: 3.999_843_853_973_347,
    shelf_q: 0.707_175_236_955_419_6,
    shelf_band_exponent: 0.499_666_774_154_541_6,
    highpass_hz: 38.135_470_876_024_44,
    highpass_q: 0.500_327_037_323_877_3,
};

/// One normalized biquad: numerator `b0, b1, b2`, denominator `1, a1, a2`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceKStage {
    /// Numerator.
    pub b: [f64; 3],
    /// Denominator after the leading 1.
    pub a: [f64; 2],
}

/// Both K-weighting stages at one rate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceKWeighting {
    /// The high shelf.
    pub shelf: ReferenceKStage,
    /// The high-pass.
    pub highpass: ReferenceKStage,
}

/// Errors of the loudness oracle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceLoudnessError {
    /// The rate is zero, or a plane is empty, or the two planes differ in length.
    InvalidInput,
    /// A sample is not finite.
    NonFinite,
}

impl core::fmt::Display for ReferenceLoudnessError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidInput => "invalid loudness reference input",
            Self::NonFinite => "non-finite loudness reference sample",
        })
    }
}

impl core::error::Error for ReferenceLoudnessError {}

/// The bilinear denominator of a normalized second-order prototype `s^2 + s/q + 1`, before
/// normalisation: `(a0, a1, a2)` in `z^-1` powers.
fn bilinear_denominator(prewarp: f64, q: f64) -> (f64, f64, f64) {
    let squared = prewarp * prewarp;
    (
        squared + prewarp / q + 1.0,
        2.0 * squared - 2.0,
        squared - prewarp / q + 1.0,
    )
}

impl ReferenceKWeighting {
    /// Designs both stages at `rate_hz`.
    #[must_use]
    pub fn design(rate_hz: f64) -> Self {
        let p = REFERENCE_K_PROTOTYPE;

        let k1 = (core::f64::consts::PI * p.shelf_hz / rate_hz).tan();
        let vh = 10.0_f64.powf(p.shelf_gain_db / 20.0);
        let vb = vh.powf(p.shelf_band_exponent);
        let (d0, d1, d2) = bilinear_denominator(k1, p.shelf_q);
        // Numerator of Vh s^2 + (Vb/Q) s + 1 with s = (1 - z^-1) / (K (1 + z^-1)), times K^2 (1 + z^-1)^2.
        let n0 = vh + vb * k1 / p.shelf_q + k1 * k1;
        let n1 = 2.0 * k1 * k1 - 2.0 * vh;
        let n2 = vh - vb * k1 / p.shelf_q + k1 * k1;
        let shelf = ReferenceKStage {
            b: [n0 / d0, n1 / d0, n2 / d0],
            a: [d1 / d0, d2 / d0],
        };

        let k48 = (core::f64::consts::PI * p.highpass_hz / 48_000.0).tan();
        let gain = bilinear_denominator(k48, p.highpass_q).0;
        let k2 = (core::f64::consts::PI * p.highpass_hz / rate_hz).tan();
        let (e0, e1, e2) = bilinear_denominator(k2, p.highpass_q);
        let highpass = ReferenceKStage {
            b: [gain / e0, -2.0 * gain / e0, gain / e0],
            a: [e1 / e0, e2 / e0],
        };
        Self { shelf, highpass }
    }

    /// The whole signal through both stages, direct form I, from rest.
    #[must_use]
    pub fn filter(&self, signal: &[f64]) -> Vec<f64> {
        let mut output = signal.to_vec();
        for stage in [self.shelf, self.highpass] {
            let (mut x1, mut x2, mut y1, mut y2) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
            for sample in &mut output {
                let x = *sample;
                let y = stage.b[0] * x + stage.b[1] * x1 + stage.b[2] * x2
                    - stage.a[0] * y1
                    - stage.a[1] * y2;
                x2 = x1;
                x1 = x;
                y2 = y1;
                y1 = y;
                *sample = y;
            }
        }
        output
    }
}

/// The level of a mean square, LUFS: `-0.691 + 10 log10 z` (`-inf` for zero).
#[must_use]
pub fn reference_loudness_level(mean_square: f64) -> f64 {
    -0.691 + 10.0 * mean_square.log10()
}

/// A whole stereo signal reduced to K-weighted energy per 10 ms segment.
///
/// Every window and step of BS.1770 and Tech 3342 is a whole number of 10 ms segments, and a
/// segment is `rate / 100` frames, exact at the four launch rates. A trailing partial segment is
/// not measured.
#[derive(Clone, Debug)]
pub struct ReferenceLoudnessTrace {
    segment_frames: usize,
    /// `E_L + E_R` of each segment.
    energy: Vec<f64>,
}

/// Segments in a momentary (400 ms) window.
pub const REFERENCE_MOMENTARY_SEGMENTS: usize = 40;
/// Segments in a short-term (3 s) window.
pub const REFERENCE_SHORT_TERM_SEGMENTS: usize = 300;
/// Segments in the 100 ms gating step.
pub const REFERENCE_GATING_STEP_SEGMENTS: usize = 10;

impl ReferenceLoudnessTrace {
    /// K-weights `left` and `right` at `rate_hz` and sums each 10 ms segment.
    ///
    /// # Errors
    ///
    /// [`ReferenceLoudnessError::InvalidInput`] for a rate that is not a multiple of 100 or planes
    /// of different lengths; [`ReferenceLoudnessError::NonFinite`] for a non-finite sample.
    pub fn measure(
        left: &[f64],
        right: &[f64],
        rate_hz: u32,
    ) -> Result<Self, ReferenceLoudnessError> {
        if rate_hz == 0 || !rate_hz.is_multiple_of(100) || left.len() != right.len() {
            return Err(ReferenceLoudnessError::InvalidInput);
        }
        if left.iter().chain(right).any(|sample| !sample.is_finite()) {
            return Err(ReferenceLoudnessError::NonFinite);
        }
        let weighting = ReferenceKWeighting::design(f64::from(rate_hz));
        let segment_frames = (rate_hz / 100) as usize;
        let weighted_left = weighting.filter(left);
        let weighted_right = weighting.filter(right);
        let energy = weighted_left
            .chunks_exact(segment_frames)
            .zip(weighted_right.chunks_exact(segment_frames))
            .map(|(l, r)| {
                l.iter().map(|x| x * x).sum::<f64>() + r.iter().map(|x| x * x).sum::<f64>()
            })
            .collect();
        Ok(Self {
            segment_frames,
            energy,
        })
    }

    /// Number of whole 10 ms segments measured.
    #[must_use]
    pub fn segments(&self) -> usize {
        self.energy.len()
    }

    /// Mean square of the `length` segments that end at segment `end` (exclusive), or `None` when
    /// the window does not fit.
    #[must_use]
    pub fn window_mean_square(&self, end: usize, length: usize) -> Option<f64> {
        if length == 0 || end < length || end > self.energy.len() {
            return None;
        }
        let energy: f64 = self.energy[end - length..end].iter().sum();
        Some(energy / (length * self.segment_frames) as f64)
    }

    /// Momentary loudness of the 400 ms ending at segment `end`.
    #[must_use]
    pub fn momentary(&self, end: usize) -> Option<f64> {
        self.window_mean_square(end, REFERENCE_MOMENTARY_SEGMENTS)
            .map(reference_loudness_level)
    }

    /// Short-term loudness of the 3 s ending at segment `end`.
    #[must_use]
    pub fn short_term(&self, end: usize) -> Option<f64> {
        self.window_mean_square(end, REFERENCE_SHORT_TERM_SEGMENTS)
            .map(reference_loudness_level)
    }

    /// Mean squares of every `length`-segment block that starts at `from + k * step` and ends at
    /// or before `to`.
    fn blocks(&self, from: usize, to: usize, length: usize) -> Vec<f64> {
        let mut blocks = Vec::new();
        let mut end = from + length;
        while end <= to.min(self.energy.len()) {
            if let Some(z) = self.window_mean_square(end, length) {
                blocks.push(z);
            }
            end += REFERENCE_GATING_STEP_SEGMENTS;
        }
        blocks
    }

    /// BS.1770-5 integrated loudness over segments `[from, to)`, exactly gated; `None` when no
    /// block passes the absolute gate.
    #[must_use]
    pub fn integrated(&self, from: usize, to: usize) -> Option<f64> {
        let blocks = self.blocks(from, to, REFERENCE_MOMENTARY_SEGMENTS);
        let absolute: Vec<f64> = blocks
            .into_iter()
            .filter(|z| reference_loudness_level(*z) > -70.0)
            .collect();
        if absolute.is_empty() {
            return None;
        }
        let gamma =
            reference_loudness_level(absolute.iter().sum::<f64>() / absolute.len() as f64) - 10.0;
        let relative: Vec<f64> = absolute
            .into_iter()
            .filter(|z| reference_loudness_level(*z) > gamma)
            .collect();
        if relative.is_empty() {
            return None;
        }
        Some(reference_loudness_level(
            relative.iter().sum::<f64>() / relative.len() as f64,
        ))
    }

    /// EBU Tech 3342 loudness range over segments `[from, to)`, as its MATLAB function computes
    /// it; `None` when no short-term block passes the gates.
    #[must_use]
    pub fn loudness_range(&self, from: usize, to: usize) -> Option<f64> {
        let levels: Vec<f64> = self
            .blocks(from, to, REFERENCE_SHORT_TERM_SEGMENTS)
            .into_iter()
            .map(reference_loudness_level)
            .collect();
        reference_loudness_range_of_levels(&levels)
    }
}

/// Tech 3342 section 5 on a vector of short-term levels (LUFS).
#[must_use]
pub fn reference_loudness_range_of_levels(levels: &[f64]) -> Option<f64> {
    let absolute: Vec<f64> = levels.iter().copied().filter(|l| *l >= -70.0).collect();
    if absolute.is_empty() {
        return None;
    }
    let power = absolute
        .iter()
        .map(|l| 10.0_f64.powf(l / 10.0))
        .sum::<f64>()
        / absolute.len() as f64;
    let integrated = 10.0 * power.log10();
    let mut relative: Vec<f64> = absolute
        .into_iter()
        .filter(|l| *l >= integrated - 20.0)
        .collect();
    if relative.is_empty() {
        return None;
    }
    relative.sort_by(f64::total_cmp);
    let n = relative.len();
    // MATLAB `round` rounds half away from zero; the argument is positive.
    let rank = |percent: f64| ((n - 1) as f64 * percent / 100.0 + 1.0).round() as usize;
    Some(relative[rank(95.0) - 1] - relative[rank(10.0) - 1])
}

/// The Tech 3341/3342 tone frequency, Hz.
pub const REFERENCE_TONE_HZ: f64 = 1_000.0;

/// A sequence of 1 kHz tone and silence segments, one plane (Tech 3341 cases 1-5, 9-14 and Tech
/// 3342 cases 1-4 apply it in phase to both channels).
///
/// Each segment is `(seconds, peak dBFS)`, `None` for silence. A segment's length is
/// `round(seconds * rate)` frames. The sine is phase-continuous across every level change and
/// silence: frame `n` of the whole signal is `A(n) sin(2 pi 1000 n / rate)`, with `A(n)` the
/// segment's linear peak. At the four launch rates every recipe duration is a whole number of
/// 1 kHz periods, so a restarted phase would give the same samples.
#[must_use]
pub fn reference_tone_sequence(rate_hz: u32, segments: &[(f64, Option<f64>)]) -> Vec<f64> {
    let rate = f64::from(rate_hz);
    let mut signal = Vec::new();
    for (seconds, level) in segments {
        let frames = (seconds * rate).round() as usize;
        let amplitude = level.map_or(0.0, |db| 10.0_f64.powf(db / 20.0));
        for _ in 0..frames {
            let n = signal.len() as f64;
            signal.push(amplitude * (core::f64::consts::TAU * REFERENCE_TONE_HZ * n / rate).sin());
        }
    }
    signal
}

/// The Tech 3341 taper length, seconds.
pub const REFERENCE_TAPER_SECONDS: f64 = 0.010;

/// Applies a raised-cosine (half-Hann) fade of `frames` frames at both ends of `signal`.
///
/// Frame `k < frames` of the fade-in is scaled by `(1 - cos(pi (k + 0.5) / frames)) / 2`, and the
/// fade-out is its mirror image, so neither end has a step.
pub fn reference_taper(signal: &mut [f64], frames: usize) {
    let frames = frames.min(signal.len() / 2);
    let length = signal.len();
    for k in 0..frames {
        let gain = 0.5 * (1.0 - (core::f64::consts::PI * (k as f64 + 0.5) / frames as f64).cos());
        signal[k] *= gain;
        signal[length - 1 - k] *= gain;
    }
}

/// Tech 3341 cases 15-19: a sine at `rate / divisor` with linear peak `amplitude` (FFS) and start
/// phase `phase_degrees`, `seconds` long, tapered over 10 ms at each end.
#[must_use]
pub fn reference_true_peak_sine(
    rate_hz: u32,
    divisor: u32,
    amplitude: f64,
    phase_degrees: f64,
    seconds: f64,
) -> Vec<f64> {
    let rate = f64::from(rate_hz);
    let frames = (seconds * rate).round() as usize;
    let phase = phase_degrees.to_radians();
    let mut signal: Vec<f64> = (0..frames)
        .map(|n| amplitude * (core::f64::consts::TAU * n as f64 / f64::from(divisor) + phase).sin())
        .collect();
    reference_taper(
        &mut signal,
        (REFERENCE_TAPER_SECONDS * rate).round() as usize,
    );
    signal
}

/// Taps of the Tech 3341 cases 20-23 anti-aliasing filter at `4 fs`.
pub const REFERENCE_DECIMATION_TAPS: usize = 257;

/// The cases 20-23 anti-aliasing filter: a 257-tap linear-phase windowed sinc at `4 fs`, cutoff
/// `fs / 2` (normalized `0.125` cycles per sample), Blackman window, scaled to unity DC gain.
///
/// The Blackman window puts the first sidelobe near -74 dB and a transition about
/// `5.5 / 257 = 0.021` cycles per sample wide, centred on the cutoff: the passband is flat to
/// about `0.46 fs` and the stopband starts near `0.54 fs`. The group delay is 128 samples at
/// `4 fs`, exactly 32 base-rate frames, so the decimated phases stay on the base grid.
#[must_use]
pub fn reference_decimation_filter() -> Vec<f64> {
    let centre = (REFERENCE_DECIMATION_TAPS / 2) as f64;
    let cutoff = 0.125;
    let mut taps: Vec<f64> = (0..REFERENCE_DECIMATION_TAPS)
        .map(|k| {
            let t = k as f64 - centre;
            let sinc = if t == 0.0 {
                2.0 * cutoff
            } else {
                (core::f64::consts::TAU * cutoff * t).sin() / (core::f64::consts::PI * t)
            };
            let phase = core::f64::consts::TAU * k as f64 / (REFERENCE_DECIMATION_TAPS - 1) as f64;
            let window = 0.42 - 0.5 * phase.cos() + 0.08 * (2.0 * phase).cos();
            sinc * window
        })
        .collect();
    let sum: f64 = taps.iter().sum();
    for tap in &mut taps {
        *tap /= sum;
    }
    taps
}

/// Tech 3341 cases 20-23 (`offset` 0, 1, 2, 3): synthesized at `4 fs`, filtered by
/// [`reference_decimation_filter`] and decimated by 4 at `offset`.
///
/// At `4 fs` the carrier is a sine of period 24 samples (`fs / 6`) and amplitude 0.5. One period
/// of a sine of period 16 samples (`fs / 4`) and amplitude 1.0 replaces the 16 samples that start
/// at `4 fs` sample `m0`, the first multiple of 24 at or after the middle of the signal: the
/// carrier is at phase 0 (a rising zero crossing) there, the burst starts and ends at phase 0
/// rising, and after it the carrier resumes at phase 0, so the signal is continuous in value and
/// in phase on both sides. The `4 fs` signal is `seconds` long and tapered over 10 ms (40 ms of
/// `4 fs` samples is 10 ms) at each end before filtering; the filter's 128-sample delay is removed
/// so output frame `n` is the filtered `4 fs` sample `4 n + offset`.
#[must_use]
pub fn reference_true_peak_burst(rate_hz: u32, offset: usize, seconds: f64) -> Vec<f64> {
    let high_rate = 4.0 * f64::from(rate_hz);
    let length = (seconds * high_rate).round() as usize;
    let m0 = (length / 2).div_ceil(24) * 24;
    let mut high: Vec<f64> = (0..length)
        .map(|m| {
            if m < m0 {
                0.5 * (core::f64::consts::TAU * m as f64 / 24.0).sin()
            } else if m < m0 + 16 {
                (core::f64::consts::TAU * (m - m0) as f64 / 16.0).sin()
            } else {
                0.5 * (core::f64::consts::TAU * (m - m0 - 16) as f64 / 24.0).sin()
            }
        })
        .collect();
    reference_taper(
        &mut high,
        (REFERENCE_TAPER_SECONDS * high_rate).round() as usize,
    );
    let taps = reference_decimation_filter();
    let delay = REFERENCE_DECIMATION_TAPS / 2;
    let frames = length / 4;
    (0..frames)
        .map(|n| {
            let centre = 4 * n + offset;
            let mut sum = 0.0;
            for (k, tap) in taps.iter().enumerate() {
                // y[c] = sum_k h[k] x[c + delay - k], zero outside the signal.
                if let Some(index) = (centre + delay).checked_sub(k)
                    && index < length
                {
                    sum += tap * high[index];
                }
            }
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matlab_rank_rule_picks_the_documented_elements() {
        // n = 11 levels 0..10: ranks round(10 * 0.1 + 1) = 2 and round(10 * 0.95 + 1) = 11 (10.5
        // rounds away from zero), so the range is element 11 minus element 2.
        let levels: Vec<f64> = (0..11).map(|k| -30.0 + f64::from(k)).collect();
        assert_eq!(
            reference_loudness_range_of_levels(&levels),
            Some(10.0 - 1.0)
        );
    }
}
