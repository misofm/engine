//! Small direct DFT oracle; bounded to fixtures rather than a production FFT.

/// Maximum accepted direct-DFT input frames.
pub(crate) const MAX_DIRECT_DFT_FRAMES: usize = 4_096;

/// Direct DFT validation errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpectrumError {
    /// Input was empty.
    EmptyInput,
    /// Input exceeds the bounded oracle size.
    TooManyFrames,
    /// Bin is outside the input spectrum.
    BinOutOfRange,
    /// Frequency or rate is invalid.
    InvalidFrequency,
    /// An input sample or complex component is non-finite.
    NonFiniteInput,
    /// The requested magnitude floor is not finite and non-positive.
    InvalidFloor,
}

/// A complex `f64` value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex64 {
    /// Real component.
    pub re: f64,
    /// Imaginary component.
    pub im: f64,
}

/// Computes one unnormalized DFT bin directly.
pub fn direct_dft_bin(samples: &[f64], bin: usize) -> Result<Complex64, SpectrumError> {
    if samples.is_empty() {
        return Err(SpectrumError::EmptyInput);
    }
    if samples.len() > MAX_DIRECT_DFT_FRAMES {
        return Err(SpectrumError::TooManyFrames);
    }
    if bin >= samples.len() {
        return Err(SpectrumError::BinOutOfRange);
    }
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(SpectrumError::NonFiniteInput);
    }
    let length = samples.len() as f64;
    let (mut re, mut im) = (0.0, 0.0);
    for (index, sample) in samples.iter().enumerate() {
        let phase = -core::f64::consts::TAU * bin as f64 * index as f64 / length;
        re += sample * phase.cos();
        im += sample * phase.sin();
    }
    Ok(Complex64 { re, im })
}

/// Computes a direct frequency sample at an arbitrary finite frequency.
pub fn direct_dft_frequency(
    samples: &[f64],
    rate_hz: f64,
    frequency_hz: f64,
) -> Result<Complex64, SpectrumError> {
    if samples.is_empty() {
        return Err(SpectrumError::EmptyInput);
    }
    if samples.len() > MAX_DIRECT_DFT_FRAMES {
        return Err(SpectrumError::TooManyFrames);
    }
    if !rate_hz.is_finite()
        || !frequency_hz.is_finite()
        || rate_hz <= 0.0
        || !(0.0..=rate_hz * 0.5).contains(&frequency_hz)
    {
        return Err(SpectrumError::InvalidFrequency);
    }
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(SpectrumError::NonFiniteInput);
    }
    let (mut re, mut im) = (0.0, 0.0);
    for (index, sample) in samples.iter().enumerate() {
        let phase = -core::f64::consts::TAU * frequency_hz * index as f64 / rate_hz;
        re += sample * phase.cos();
        im += sample * phase.sin();
    }
    Ok(Complex64 { re, im })
}

/// Converts magnitude to dBFS with a finite floor.
pub fn magnitude_db(value: Complex64, floor_db: f64) -> Result<f64, SpectrumError> {
    if !value.re.is_finite() || !value.im.is_finite() {
        return Err(SpectrumError::NonFiniteInput);
    }
    if !floor_db.is_finite() || floor_db > 0.0 {
        return Err(SpectrumError::InvalidFloor);
    }
    let magnitude = value.re.hypot(value.im);
    if magnitude == 0.0 {
        Ok(floor_db)
    } else {
        Ok((20.0 * magnitude.log10()).max(floor_db))
    }
}
