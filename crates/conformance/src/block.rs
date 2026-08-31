//! Validated borrowed planar blocks.

use engine::{SampleRateHz, is_extended_compatibility_sample_rate, is_launch_sample_rate};

/// `PlanarBlock` construction errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockError {
    /// Rate is neither a launch rate nor an extended compatibility corpus rate.
    InvalidRate,
    /// Channels or frames is zero.
    Empty,
    /// Channel/frame multiplication overflowed.
    Overflow,
    /// Samples do not match the shape.
    WrongLength,
}

/// Borrowed channel-major planar samples with a validated shape.
#[derive(Clone, Copy, Debug)]
pub struct PlanarBlock<'a, T> {
    rate: SampleRateHz,
    channels: usize,
    frames: usize,
    samples: &'a [T],
}

impl<'a, T> PlanarBlock<'a, T> {
    /// Validates and constructs a planar block.
    pub fn try_new(
        rate: SampleRateHz,
        channels: usize,
        frames: usize,
        samples: &'a [T],
    ) -> Result<Self, BlockError> {
        if !(is_launch_sample_rate(rate) || is_extended_compatibility_sample_rate(rate)) {
            return Err(BlockError::InvalidRate);
        }
        if channels == 0 || frames == 0 {
            return Err(BlockError::Empty);
        }
        let length = channels.checked_mul(frames).ok_or(BlockError::Overflow)?;
        if length != samples.len() {
            return Err(BlockError::WrongLength);
        }
        Ok(Self {
            rate,
            channels,
            frames,
            samples,
        })
    }
    /// Returns the validated sample rate.
    pub const fn rate(&self) -> SampleRateHz {
        self.rate
    }
    /// Returns channels.
    pub const fn channels(&self) -> usize {
        self.channels
    }
    /// Returns frames per channel.
    pub const fn frames(&self) -> usize {
        self.frames
    }
    /// Returns channel-major samples.
    pub const fn samples(&self) -> &'a [T] {
        self.samples
    }
    /// Returns one channel when in range.
    pub fn channel(&self, channel: usize) -> Option<&'a [T]> {
        let start = channel.checked_mul(self.frames)?;
        let end = channel.checked_add(1)?.checked_mul(self.frames)?;
        self.samples.get(start..end)
    }
}
