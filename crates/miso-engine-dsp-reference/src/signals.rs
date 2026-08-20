//! Deterministic `f64` signal generators.

use crate::{F64PlanarBuffer, ReferenceBlockError};

/// Errors returned by deterministic reference-signal generators.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceSignalError {
    /// The requested buffer shape is invalid.
    Block(ReferenceBlockError),
    /// A rate, frequency, or impulse position is outside its finite domain.
    InvalidParameter,
}

impl From<ReferenceBlockError> for ReferenceSignalError {
    fn from(value: ReferenceBlockError) -> Self {
        Self::Block(value)
    }
}

/// Produces a channel-asymmetric impulse at `frame`.
pub fn deterministic_impulse(
    channels: usize,
    frames: usize,
    frame: usize,
) -> Result<F64PlanarBuffer, ReferenceSignalError> {
    if frame >= frames {
        return Err(ReferenceSignalError::InvalidParameter);
    }
    let mut result = F64PlanarBuffer::zeros(channels, frames)?;
    for channel in 0..channels {
        result.channel_mut(channel).expect("validated channel")[frame] =
            1.0 - channel as f64 * 0.125;
    }
    Ok(result)
}

/// Produces a deterministic sine per channel with a channel-dependent phase.
pub fn deterministic_sine(
    channels: usize,
    frames: usize,
    rate_hz: f64,
    frequency_hz: f64,
) -> Result<F64PlanarBuffer, ReferenceSignalError> {
    if !rate_hz.is_finite()
        || !frequency_hz.is_finite()
        || rate_hz <= 0.0
        || !(0.0..=rate_hz * 0.5).contains(&frequency_hz)
    {
        return Err(ReferenceSignalError::InvalidParameter);
    }
    let mut result = F64PlanarBuffer::zeros(channels, frames)?;
    for channel in 0..channels {
        let phase = channel as f64 * 0.31;
        for (index, sample) in result
            .channel_mut(channel)
            .expect("validated channel")
            .iter_mut()
            .enumerate()
        {
            *sample =
                (core::f64::consts::TAU * frequency_hz * index as f64 / rate_hz + phase).sin();
        }
    }
    Ok(result)
}

/// Produces deterministic SplitMix64-derived bipolar noise without depending on the conformance PRNG.
pub fn deterministic_bipolar_noise(
    channels: usize,
    frames: usize,
    mut state: u64,
) -> Result<F64PlanarBuffer, ReferenceBlockError> {
    let mut result = F64PlanarBuffer::zeros(channels, frames)?;
    for sample in result.samples_mut() {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut mixed = state;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        mixed ^= mixed >> 31;
        *sample = ((mixed >> 40) as f64 * (1.0 / 16_777_216.0)) * 2.0 - 1.0;
    }
    Ok(result)
}
