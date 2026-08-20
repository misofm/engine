//! Stable numerical metrics for `f32` implementation output versus an `f64` oracle.

use crate::PlanarBlock;

/// Error conditions for comparison.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComparisonError {
    /// Sample rates differ.
    RateMismatch,
    /// Shapes differ.
    ShapeMismatch,
    /// A tolerance is negative or non-finite.
    InvalidTolerance,
    /// Input contains a non-finite value.
    NonFiniteInput,
    /// A finite input/tolerance combination overflowed a derived metric.
    NonFiniteComputation,
}

/// Absolute plus reference-scaled tolerance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComparisonTolerance {
    /// Constant permitted absolute error.
    pub absolute: f64,
    /// Permitted error proportional to absolute reference sample.
    pub relative: f64,
    /// Minimum denominator for relative-error reporting.
    pub relative_floor: f64,
}

/// The location of a reported sample in a planar block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SampleLocation {
    /// Channel index.
    pub channel: usize,
    /// Frame index.
    pub frame: usize,
}

/// Complete deterministic comparison measurements.
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonReport {
    /// Samples compared.
    pub sample_count: usize,
    /// Actual peak absolute sample.
    pub actual_peak: f64,
    /// Reference peak absolute sample.
    pub reference_peak: f64,
    /// Actual stable RMS.
    pub actual_rms: f64,
    /// Reference stable RMS.
    pub reference_rms: f64,
    /// Peak absolute error.
    pub peak_error: f64,
    /// RMS error.
    pub rms_error: f64,
    /// Largest relative error using `relative_floor`.
    pub max_relative_error: f64,
    /// Signal-to-noise ratio with explicit exact/silent boundary cases.
    pub snr_db: SnrDb,
    /// Location of greatest normalized tolerance error.
    pub worst_sample: SampleLocation,
    /// Largest error divided by sample tolerance.
    pub worst_normalized_error: f64,
    /// Whether every sample passed tolerance.
    pub within_tolerance: bool,
}

/// Explicit signal-to-noise result; no sentinel dB values are used.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnrDb {
    /// A finite dB result.
    Finite(f64),
    /// Reference is nonzero and error is exactly zero.
    PositiveInfinity,
    /// Reference is zero and error is nonzero.
    NegativeInfinity,
    /// Both reference and error are exactly zero.
    Undefined,
}

/// Compares an implementation block with an independent `f64` planar reference.
pub fn compare_f32_to_f64(
    actual: PlanarBlock<'_, f32>,
    reference: PlanarBlock<'_, f64>,
    tolerance: ComparisonTolerance,
) -> Result<ComparisonReport, ComparisonError> {
    if !tolerance.absolute.is_finite()
        || !tolerance.relative.is_finite()
        || !tolerance.relative_floor.is_finite()
        || tolerance.absolute < 0.0
        || tolerance.relative < 0.0
        || tolerance.relative_floor <= 0.0
    {
        return Err(ComparisonError::InvalidTolerance);
    }
    if actual.rate() != reference.rate() {
        return Err(ComparisonError::RateMismatch);
    }
    if actual.channels() != reference.channels() || actual.frames() != reference.frames() {
        return Err(ComparisonError::ShapeMismatch);
    }
    let mut actual_peak = 0.0_f64;
    let mut reference_peak = 0.0_f64;
    let mut peak_error = 0.0_f64;
    let mut max_relative_error = 0.0_f64;
    let mut worst = SampleLocation {
        channel: 0,
        frame: 0,
    };
    let mut worst_normalized = 0.0_f64;
    let mut within = true;
    let mut actual_squares = ScaledSumSquares::default();
    let mut reference_squares = ScaledSumSquares::default();
    let mut error_squares = ScaledSumSquares::default();
    for (index, (actual_value, reference_value)) in
        actual.samples().iter().zip(reference.samples()).enumerate()
    {
        if !actual_value.is_finite() || !reference_value.is_finite() {
            return Err(ComparisonError::NonFiniteInput);
        }
        let actual_value = f64::from(*actual_value);
        let error = (actual_value - reference_value).abs();
        let limit = tolerance.absolute + tolerance.relative * reference_value.abs();
        let relative_error = error / reference_value.abs().max(tolerance.relative_floor);
        if !error.is_finite() || !limit.is_finite() || !relative_error.is_finite() {
            return Err(ComparisonError::NonFiniteComputation);
        }
        let normalized = if limit == 0.0 {
            if error == 0.0 { 0.0 } else { f64::INFINITY }
        } else {
            error / limit
        };
        actual_peak = actual_peak.max(actual_value.abs());
        reference_peak = reference_peak.max(reference_value.abs());
        peak_error = peak_error.max(error);
        max_relative_error = max_relative_error.max(relative_error);
        if normalized > worst_normalized {
            worst_normalized = normalized;
            worst = SampleLocation {
                channel: index / actual.frames(),
                frame: index % actual.frames(),
            };
        }
        if error > limit {
            within = false;
        }
        actual_squares.add(actual_value);
        reference_squares.add(*reference_value);
        error_squares.add(error);
    }
    let sample_count = actual.samples().len();
    let actual_rms = actual_squares.rms(sample_count);
    let reference_rms = reference_squares.rms(sample_count);
    let rms_error = error_squares.rms(sample_count);
    let snr_db = if rms_error == 0.0 && reference_rms == 0.0 {
        SnrDb::Undefined
    } else if rms_error == 0.0 {
        SnrDb::PositiveInfinity
    } else if reference_rms == 0.0 {
        SnrDb::NegativeInfinity
    } else {
        let value = 20.0 * (reference_rms / rms_error).log10();
        if !value.is_finite() {
            return Err(ComparisonError::NonFiniteComputation);
        }
        SnrDb::Finite(value)
    };
    Ok(ComparisonReport {
        sample_count,
        actual_peak,
        reference_peak,
        actual_rms,
        reference_rms,
        peak_error,
        rms_error,
        max_relative_error,
        snr_db,
        worst_sample: worst,
        worst_normalized_error: worst_normalized,
        within_tolerance: within,
    })
}

#[derive(Default)]
struct ScaledSumSquares {
    scale: f64,
    scaled_sum: f64,
}

impl ScaledSumSquares {
    fn add(&mut self, value: f64) {
        let absolute = value.abs();
        if absolute == 0.0 {
            return;
        }
        if self.scale < absolute {
            let ratio = self.scale / absolute;
            self.scaled_sum = 1.0 + self.scaled_sum * ratio * ratio;
            self.scale = absolute;
        } else {
            let ratio = absolute / self.scale;
            self.scaled_sum += ratio * ratio;
        }
    }

    fn rms(&self, count: usize) -> f64 {
        if self.scale == 0.0 {
            0.0
        } else {
            self.scale * (self.scaled_sum / count as f64).sqrt()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScaledSumSquares;

    #[test]
    fn scaled_sum_squares_handles_extreme_finite_values() {
        let mut sum = ScaledSumSquares::default();
        sum.add(f64::MAX / 2.0);
        sum.add(f64::MAX / 2.0);
        assert_eq!(sum.rms(2), f64::MAX / 2.0);
    }
}
