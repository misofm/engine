//! Preallocated planar PCM storage.

use crate::QuantumFrames;
use core::num::NonZeroUsize;

/// Index of a buffer prepared in an arena.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BufferIndex(pub usize);

/// Immutable shape of one planar PCM buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBufferSpec {
    /// Number of independent planes.
    pub channels: NonZeroUsize,
    /// Capacity of every plane.
    pub frame_capacity: QuantumFrames,
}

/// A buffer-arena validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferArenaError {
    /// Zero frames cannot form a render buffer.
    ZeroFrames,
    /// A channel/frame/count arithmetic operation overflowed `usize`.
    CapacityOverflow,
    /// An index does not identify a prepared buffer.
    InvalidIndex,
    /// A requested plane is outside the buffer shape.
    InvalidPlane,
    /// A borrowed buffer shape is invalid for its storage.
    InvalidBorrow,
}

/// One allocation containing every prepared planar buffer.
pub struct BufferArena {
    storage: Box<[f32]>,
    offsets: Box<[usize]>,
    specs: Box<[PlanarBufferSpec]>,
}

impl BufferArena {
    /// Preallocate all buffer planes with checked arithmetic.
    pub fn try_new(specs: &[PlanarBufferSpec]) -> Result<Self, BufferArenaError> {
        let mut total = 0usize;
        let mut offsets = Vec::with_capacity(specs.len());
        for spec in specs {
            let frames = usize::try_from(spec.frame_capacity.0)
                .map_err(|_| BufferArenaError::CapacityOverflow)?;
            if frames == 0 {
                return Err(BufferArenaError::ZeroFrames);
            }
            let samples = spec
                .channels
                .get()
                .checked_mul(frames)
                .ok_or(BufferArenaError::CapacityOverflow)?;
            offsets.push(total);
            total = total
                .checked_add(samples)
                .ok_or(BufferArenaError::CapacityOverflow)?;
        }
        // The PCM allocation is the sole large allocation; metadata is prepared off render.
        Ok(Self {
            storage: vec![0.0; total].into_boxed_slice(),
            offsets: offsets.into_boxed_slice(),
            specs: specs.to_vec().into_boxed_slice(),
        })
    }

    /// Number of logical buffers.
    #[must_use]
    pub fn count(&self) -> usize {
        self.specs.len()
    }
    /// Total preallocated PCM samples.
    #[must_use]
    pub fn total_samples(&self) -> usize {
        self.storage.len()
    }
    /// Shape of one buffer.
    pub fn spec(&self, index: BufferIndex) -> Result<PlanarBufferSpec, BufferArenaError> {
        self.specs
            .get(index.0)
            .copied()
            .ok_or(BufferArenaError::InvalidIndex)
    }
    fn range(
        &self,
        index: BufferIndex,
        channel: usize,
    ) -> Result<core::ops::Range<usize>, BufferArenaError> {
        let spec = self.spec(index)?;
        if channel >= spec.channels.get() {
            return Err(BufferArenaError::InvalidPlane);
        }
        let frames = spec.frame_capacity.0 as usize;
        let start = self.offsets[index.0]
            .checked_add(
                channel
                    .checked_mul(frames)
                    .ok_or(BufferArenaError::CapacityOverflow)?,
            )
            .ok_or(BufferArenaError::CapacityOverflow)?;
        let end = start
            .checked_add(frames)
            .ok_or(BufferArenaError::CapacityOverflow)?;
        Ok(start..end)
    }
    /// Borrow one immutable plane.
    pub fn plane(&self, index: BufferIndex, channel: usize) -> Result<&[f32], BufferArenaError> {
        let range = self.range(index, channel)?;
        Ok(&self.storage[range])
    }
    /// Borrow one mutable plane.
    pub fn plane_mut(
        &mut self,
        index: BufferIndex,
        channel: usize,
    ) -> Result<&mut [f32], BufferArenaError> {
        let range = self.range(index, channel)?;
        Ok(&mut self.storage[range])
    }
    /// Clear all prepared PCM samples.
    pub fn clear(&mut self) {
        self.storage.fill(0.0);
    }
}

/// Borrowed planar input/view with an explicit per-plane stride.
#[derive(Clone, Copy)]
pub struct PlanarBufferRef<'a> {
    storage: &'a [f32],
    channels: usize,
    frames: usize,
    stride: usize,
}
/// Borrowed mutable planar output/view with an explicit per-plane stride.
pub struct PlanarBufferMut<'a> {
    storage: &'a mut [f32],
    channels: usize,
    frames: usize,
    stride: usize,
}

impl<'a> PlanarBufferRef<'a> {
    /// Validate a borrowed planar layout without allocation.
    pub fn try_new(
        storage: &'a [f32],
        channels: usize,
        frames: usize,
        stride: usize,
    ) -> Result<Self, BufferArenaError> {
        validate_borrow(storage.len(), channels, frames, stride)?;
        Ok(Self {
            storage,
            channels,
            frames,
            stride,
        })
    }
    /// Number of planes.
    #[must_use]
    pub const fn channels(&self) -> usize {
        self.channels
    }
    /// Frames valid in each plane.
    #[must_use]
    pub const fn frames(&self) -> usize {
        self.frames
    }
    /// Borrow one plane.
    pub fn plane(&self, channel: usize) -> Result<&'a [f32], BufferArenaError> {
        plane_range(
            self.storage,
            self.channels,
            self.frames,
            self.stride,
            channel,
        )
    }
}
// REALTIME_POLICY_BEGIN
impl<'a> PlanarBufferMut<'a> {
    /// Validate a borrowed mutable planar layout without allocation.
    pub fn try_new(
        storage: &'a mut [f32],
        channels: usize,
        frames: usize,
        stride: usize,
    ) -> Result<Self, BufferArenaError> {
        validate_borrow(storage.len(), channels, frames, stride)?;
        Ok(Self {
            storage,
            channels,
            frames,
            stride,
        })
    }
    /// Number of planes.
    #[must_use]
    pub const fn channels(&self) -> usize {
        self.channels
    }
    /// Frames valid in each plane.
    #[must_use]
    pub const fn frames(&self) -> usize {
        self.frames
    }
    /// Borrow one mutable plane.
    pub fn plane_mut(&mut self, channel: usize) -> Result<&mut [f32], BufferArenaError> {
        if channel >= self.channels {
            return Err(BufferArenaError::InvalidPlane);
        }
        let start = channel
            .checked_mul(self.stride)
            .ok_or(BufferArenaError::CapacityOverflow)?;
        let end = start
            .checked_add(self.frames)
            .ok_or(BufferArenaError::CapacityOverflow)?;
        Ok(&mut self.storage[start..end])
    }
    /// Borrow both planes of a two-channel buffer at once, each exactly `frames` words long.
    ///
    /// The planes are the words [`Self::plane_mut`] returns for channels 0 and 1, split at the
    /// stride, so a caller can hold both for as long as it holds this buffer. The `stride - frames`
    /// padding words after each plane are part of neither borrow.
    ///
    /// # Errors
    ///
    /// [`BufferArenaError::InvalidPlane`] unless this buffer has exactly two channels. The
    /// validated layout already guarantees the split and both trims, so their
    /// [`BufferArenaError::InvalidBorrow`] refusal is unreachable; it is returned, not panicked.
    pub fn stereo_planes_mut(&mut self) -> Result<(&mut [f32], &mut [f32]), BufferArenaError> {
        if self.channels != 2 {
            return Err(BufferArenaError::InvalidPlane);
        }
        let (left, right) = self
            .storage
            .split_at_mut_checked(self.stride)
            .ok_or(BufferArenaError::InvalidBorrow)?;
        let left = left
            .get_mut(..self.frames)
            .ok_or(BufferArenaError::InvalidBorrow)?;
        let right = right
            .get_mut(..self.frames)
            .ok_or(BufferArenaError::InvalidBorrow)?;
        Ok((left, right))
    }
}
// REALTIME_POLICY_END
fn validate_borrow(
    len: usize,
    channels: usize,
    frames: usize,
    stride: usize,
) -> Result<(), BufferArenaError> {
    if channels == 0 || frames == 0 || stride < frames {
        return Err(BufferArenaError::InvalidBorrow);
    }
    let required = channels
        .checked_sub(1)
        .and_then(|n| n.checked_mul(stride))
        .and_then(|n| n.checked_add(frames))
        .ok_or(BufferArenaError::CapacityOverflow)?;
    if required > len {
        return Err(BufferArenaError::InvalidBorrow);
    }
    Ok(())
}
fn plane_range(
    storage: &[f32],
    channels: usize,
    frames: usize,
    stride: usize,
    channel: usize,
) -> Result<&[f32], BufferArenaError> {
    if channel >= channels {
        return Err(BufferArenaError::InvalidPlane);
    }
    let start = channel
        .checked_mul(stride)
        .ok_or(BufferArenaError::CapacityOverflow)?;
    let end = start
        .checked_add(frames)
        .ok_or(BufferArenaError::CapacityOverflow)?;
    Ok(&storage[start..end])
}

#[cfg(test)]
mod tests {
    use super::{BufferArenaError, PlanarBufferMut};

    /// Issue #916: both planes at once, each trimmed to `frames`, at a stride wider than the
    /// block, with the padding words after each plane outside both borrows.
    #[test]
    fn stereo_planes_are_the_two_strided_planes_and_leave_the_padding_alone() {
        const FRAMES: usize = 5;
        const STRIDE: usize = FRAMES + 3;
        let pad = f32::from_bits(0x7fc0_0916);
        let mut storage = [pad; STRIDE + FRAMES + 2];
        let mut buffer =
            PlanarBufferMut::try_new(&mut storage, 2, FRAMES, STRIDE).expect("stereo layout");
        {
            let (left, right) = buffer.stereo_planes_mut().expect("two planes");
            assert_eq!((left.len(), right.len()), (FRAMES, FRAMES));
            for (index, word) in left.iter_mut().enumerate() {
                *word = index as f32;
            }
            for (index, word) in right.iter_mut().enumerate() {
                *word = -(index as f32) - 1.0;
            }
        }
        // The same words `plane_mut` names, so the two accessors agree plane for plane.
        assert_eq!(
            buffer.plane_mut(0).expect("left"),
            &[0.0, 1.0, 2.0, 3.0, 4.0]
        );
        assert_eq!(
            buffer.plane_mut(1).expect("right"),
            &[-1.0, -2.0, -3.0, -4.0, -5.0]
        );
        for (index, word) in storage.iter().enumerate() {
            let in_left = index < FRAMES;
            let in_right = (STRIDE..STRIDE + FRAMES).contains(&index);
            if !in_left && !in_right {
                assert_eq!(
                    word.to_bits(),
                    pad.to_bits(),
                    "padding word {index} was written"
                );
            }
        }
    }

    #[test]
    fn stereo_planes_refuse_every_other_channel_count() {
        let mut mono = [0.0_f32; 4];
        let mut buffer = PlanarBufferMut::try_new(&mut mono, 1, 4, 4).expect("mono layout");
        assert_eq!(
            buffer.stereo_planes_mut().map(|_| ()),
            Err(BufferArenaError::InvalidPlane)
        );
        let mut three = [0.0_f32; 12];
        let mut buffer = PlanarBufferMut::try_new(&mut three, 3, 4, 4).expect("three planes");
        assert_eq!(
            buffer.stereo_planes_mut().map(|_| ()),
            Err(BufferArenaError::InvalidPlane)
        );
        // Stride equal to frames, the layout every host but the C ABI builds.
        let mut tight = [0.0_f32; 8];
        let mut buffer = PlanarBufferMut::try_new(&mut tight, 2, 4, 4).expect("tight stereo");
        let (left, right) = buffer.stereo_planes_mut().expect("two planes");
        assert_eq!((left.len(), right.len()), (4, 4));
    }
}
