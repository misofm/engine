//! Planar offline reference buffers.

/// Errors returned while constructing a reference buffer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceBlockError {
    /// A planar buffer requires at least one channel and frame.
    EmptyShape,
    /// Channel count times frames overflowed `usize`.
    ShapeOverflow,
    /// The supplied storage does not exactly match the declared shape.
    WrongLength,
}

/// Owned planar `f64` samples in channel-major order.
#[derive(Clone, Debug, PartialEq)]
pub struct F64PlanarBuffer {
    channels: usize,
    frames: usize,
    samples: Vec<f64>,
}

impl F64PlanarBuffer {
    /// Creates a zero-filled planar buffer.
    pub fn zeros(channels: usize, frames: usize) -> Result<Self, ReferenceBlockError> {
        let length = checked_length(channels, frames)?;
        Ok(Self {
            channels,
            frames,
            samples: vec![0.0; length],
        })
    }

    /// Creates a buffer from channel-major samples.
    pub fn from_samples(
        channels: usize,
        frames: usize,
        samples: Vec<f64>,
    ) -> Result<Self, ReferenceBlockError> {
        if checked_length(channels, frames)? != samples.len() {
            return Err(ReferenceBlockError::WrongLength);
        }
        Ok(Self {
            channels,
            frames,
            samples,
        })
    }

    /// Returns the channel count.
    pub const fn channels(&self) -> usize {
        self.channels
    }
    /// Returns the frame count.
    pub const fn frames(&self) -> usize {
        self.frames
    }
    /// Returns all channel-major samples.
    pub fn samples(&self) -> &[f64] {
        &self.samples
    }
    /// Returns mutable channel-major samples.
    pub fn samples_mut(&mut self) -> &mut [f64] {
        &mut self.samples
    }
    /// Returns one channel.
    pub fn channel(&self, channel: usize) -> Option<&[f64]> {
        let start = channel.checked_mul(self.frames)?;
        self.samples.get(start..start.checked_add(self.frames)?)
    }
    /// Returns one mutable channel.
    pub fn channel_mut(&mut self, channel: usize) -> Option<&mut [f64]> {
        let start = channel.checked_mul(self.frames)?;
        self.samples.get_mut(start..start.checked_add(self.frames)?)
    }
}

fn checked_length(channels: usize, frames: usize) -> Result<usize, ReferenceBlockError> {
    if channels == 0 || frames == 0 {
        return Err(ReferenceBlockError::EmptyShape);
    }
    channels
        .checked_mul(frames)
        .ok_or(ReferenceBlockError::ShapeOverflow)
}
