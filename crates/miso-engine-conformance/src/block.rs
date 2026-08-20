//! Validated borrowed planar blocks.

use miso_engine_core::SampleRateHz;

/// `PlanarBlock` construction errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockError {
    /// Rate is zero or not an engine-supported rate.
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
        if !matches!(
            rate.0,
            44_100 | 48_000 | 88_200 | 96_000 | 176_400 | 192_000 | 352_800 | 384_000
        ) {
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
