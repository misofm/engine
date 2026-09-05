//! Plan-owned disjoint audio arena for the sequential render executor.
//!
//! The render executor consumes one prepared lease sequentially. Before issue #100, plans used
//! separate arenas and copied edges between them. The pull model instead gives the whole plan
//! **one** arena and lets consumers read producer buffers in place.
//!
//! That requires one allocation to be reachable through several leases, which is why this module
//! owns the only `unsafe` on the render path. [`ArenaLeaseSetBuilder::finish`] proves the two
//! structural invariants:
//!
//! * **I1 — writes are globally unique.** Every buffer is writable by at most one lease, for the
//!   whole life of the plan. Buffers are never recycled, so no two leases (in the same wave or in
//!   different waves) can ever address the same words mutably.
//! * **I2 — reads are strictly earlier.** A lease may read a buffer only if that buffer is
//!   written by a lease of a strictly smaller wave, or by the reading lease itself. Wave order is
//!   a dependency relation; it is not by itself synchronization.
//!
//! All execution must also satisfy **E1 — ordered access**: a foreign writer's exclusive access
//! ends and happens-before a consuming lease reads that buffer. The production executor
//! discharges E1 by using its single prepared lease exclusively and sequentially. Any retained
//! multi-lease use must provide the same happens-before edge and must not overlap a foreign write
//! with a read. Concurrent leases may write their I1-disjoint sets and join before inspection.
//!
//! Buffer `0` is the silence buffer. No lease may write it, so it stays zero for the life of the
//! arena.

#![allow(unsafe_code)]

use core::{cell::UnsafeCell, num::NonZeroUsize};
use std::sync::Arc;

/// The silence buffer every arena reserves: always zero, writable by nobody.
pub const ARENA_SILENCE_BUFFER: u32 = 0;

/// A bind-time rejection of an unsound or oversized lease set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisjointArenaError {
    /// Two leases declared a write to the same buffer, breaking I1.
    Overlap {
        /// The buffer both leases claimed.
        buffer: u32,
        /// The lease that claimed it first.
        first: usize,
        /// The lease that claimed it again.
        second: usize,
    },
    /// A lease reads a buffer that is not produced by a strictly earlier wave, breaking I2.
    ReadNotEarlier {
        /// The reading lease.
        lease: usize,
        /// The buffer it tried to read.
        buffer: u32,
    },
    /// A lease declared a write to the reserved silence buffer.
    SilenceWrite {
        /// The offending lease.
        lease: usize,
    },
    /// A declared buffer was never reserved.
    UnknownBuffer {
        /// The offending lease.
        lease: usize,
        /// The buffer it named.
        buffer: u32,
    },
    /// Exact byte accounting overflowed `usize`.
    CapacityOverflow,
}

/// Flat planar `f32` storage shared by every lease of one prepared plan.
///
/// See the module documentation for the invariants that make the shared mutable access sound.
/// Instances are produced only by [`ArenaLeaseSetBuilder::finish`].
pub struct DisjointArena {
    cells: Box<[UnsafeCell<f32>]>,
    planes: usize,
    buffers: usize,
    frames: usize,
}

// SAFETY: the arena hands out access only through `ArenaLease`, and a lease set is constructed
// only by `ArenaLeaseSetBuilder::finish`, which proves I1 and I2 (module documentation). I1
// prevents write/write aliasing. E1 is the separate execution obligation that prevents a foreign
// write from overlapping a read; the production executor meets it through exclusive sequential
// lease use, and any retained multi-lease executor must establish the documented happens-before.
unsafe impl Sync for DisjointArena {}
// SAFETY: `f32` is `Send`, and a lease carries no thread-affine state.
unsafe impl Send for DisjointArena {}

impl core::fmt::Debug for DisjointArena {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("DisjointArena")
            .field("planes", &self.planes)
            .field("buffers", &self.buffers)
            .field("frames", &self.frames)
            .finish()
    }
}

impl DisjointArena {
    /// Number of planes (2 for the stereo render graph).
    #[must_use]
    pub const fn planes(&self) -> usize {
        self.planes
    }

    /// Number of reserved buffers, including the silence buffer.
    #[must_use]
    pub const fn buffers(&self) -> usize {
        self.buffers
    }

    /// Frames in one buffer.
    #[must_use]
    pub const fn frames(&self) -> usize {
        self.frames
    }

    /// Exact retained payload bytes, excluding allocator headers.
    #[must_use]
    pub const fn total_bytes(&self) -> usize {
        self.cells.len() * core::mem::size_of::<f32>()
    }

    #[inline]
    const fn offset(&self, plane: usize, buffer: usize) -> usize {
        (plane * self.buffers + buffer) * self.frames
    }
}

/// Both planes of one written buffer plus both planes of one read buffer.
pub type ArenaStereoPair<'a> = ((&'a mut [f32], &'a mut [f32]), (&'a [f32], &'a [f32]));
/// Both writable planes of one arena buffer.
pub type ArenaStereoPlanes<'a> = (&'a mut [f32], &'a mut [f32]);

/// One executor step's checked view of the shared arena.
///
/// The lease is the only way to reach arena storage. It is `Send` and deliberately not `Sync`.
pub struct ArenaLease {
    arena: Arc<DisjointArena>,
    /// Per buffer: bit 0 writable by this lease.
    access: Box<[u8]>,
}

const ACCESS_WRITE: u8 = 0b01;

impl core::fmt::Debug for ArenaLease {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ArenaLease")
            .field("buffers", &self.arena.buffers)
            .finish()
    }
}

impl ArenaLease {
    /// Frames in one buffer.
    #[must_use]
    pub fn frames(&self) -> usize {
        self.arena.frames
    }

    /// Whether this lease may write `buffer`.
    #[must_use]
    pub fn writes(&self, buffer: u32) -> bool {
        self.access
            .get(buffer as usize)
            .is_some_and(|access| access & ACCESS_WRITE != 0)
    }

    // REALTIME_POLICY_BEGIN

    #[inline]
    fn effective(&self, buffer: u32) -> usize {
        let index = buffer as usize;
        // This checked access is a release-mode read-ID guard. Keep it before forming the
        // unchecked arena slice below; debug_assert alone would remove the safety boundary.
        let _ = self.access[index];
        index
    }

    #[inline]
    fn checked_write(&self, buffer: u32) -> usize {
        let index = buffer as usize;
        debug_assert!(
            self.access
                .get(index)
                .is_some_and(|access| access & ACCESS_WRITE != 0),
            "write outside this lease's unique write set"
        );
        index
    }

    /// One buffer's frames in `plane`, shared.
    #[inline]
    #[must_use]
    pub fn read(&self, plane: usize, buffer: u32) -> &[f32] {
        let start = self.arena.offset(plane, self.effective(buffer));
        // SAFETY: I1/I2/E1. `start..start + frames` is in bounds by construction (the builder
        // reserved every buffer and sized the allocation `planes * buffers * frames`), and no
        // other lease may write those words while this shared reference lives: either the buffer
        // is this lease's own, E1 has ended and ordered its foreign writer's access, or it is the
        // never-written silence buffer.
        unsafe {
            core::slice::from_raw_parts(
                self.arena.cells[start].get().cast_const().cast::<f32>(),
                self.arena.frames,
            )
        }
    }

    /// Both planes of one buffer, shared.
    #[inline]
    #[must_use]
    pub fn read_stereo(&self, buffer: u32) -> (&[f32], &[f32]) {
        (self.read(0, buffer), self.read(1, buffer))
    }

    /// One buffer's frames in `plane`, exclusively. `buffer` must be in this lease's write set.
    #[inline]
    pub fn write(&mut self, plane: usize, buffer: u32) -> &mut [f32] {
        let start = self.arena.offset(plane, self.checked_write(buffer));
        // SAFETY: I1 — `buffer` is writable by this lease alone, for the life of the plan, so no
        // other lease can produce a reference to these words; `&mut self` excludes any other live
        // reference from this lease.
        unsafe {
            core::slice::from_raw_parts_mut(
                self.arena.cells[start].get().cast::<f32>(),
                self.arena.frames,
            )
        }
    }

    /// Both planes of one buffer, exclusively.
    #[inline]
    pub fn write_stereo(&mut self, buffer: u32) -> (&mut [f32], &mut [f32]) {
        let index = self.checked_write(buffer);
        let (left, right) = (self.arena.offset(0, index), self.arena.offset(1, index));
        // SAFETY: I1 as in `write`; the two planes are disjoint ranges of the allocation because
        // `offset` is plane-major and `index` is the same buffer in both.
        unsafe {
            (
                core::slice::from_raw_parts_mut(
                    self.arena.cells[left].get().cast::<f32>(),
                    self.arena.frames,
                ),
                core::slice::from_raw_parts_mut(
                    self.arena.cells[right].get().cast::<f32>(),
                    self.arena.frames,
                ),
            )
        }
    }

    /// Borrow the complete set of pairwise-disjoint stereo outputs for a full bank.
    ///
    /// All access and shape checks happen before any reference is formed. This is the sole
    /// mutable multi-plane seam used by the direct bank scatter path.
    pub fn write_stereo_many<const W: usize>(
        &mut self,
        buffers: &[u32; W],
        frames: usize,
    ) -> Option<[ArenaStereoPlanes<'_>; W]> {
        // Stereo access is part of this safe API's shape, not a convention imposed on callers.
        // Reject it before computing either plane offset or forming any reference.
        if self.arena.planes < 2 || (W != 4 && W != 8) || frames > self.arena.frames {
            return None;
        }
        for (i, buffer) in buffers.iter().copied().enumerate() {
            let index = buffer as usize;
            if index == 0 || index >= self.arena.buffers || !self.writes(buffer) {
                return None;
            }
            if buffers[..i].contains(&buffer) {
                return None;
            }
        }
        let cells = self.arena.cells.as_ptr();
        let buffer_count = self.arena.buffers;
        let arena_frames = self.arena.frames;
        let pairs = core::array::from_fn(|lane| {
            let buffer = buffers[lane] as usize;
            let left = buffer * arena_frames;
            let right = buffer_count * arena_frames + left;
            // SAFETY: all buffers were checked writable, in bounds, and pairwise distinct above;
            // W <= 8 bounds this fixed construction. `finish` allocated
            // `planes * buffer_count * arena_frames` cells, `planes >= 2`, buffer is strictly
            // below buffer_count, and frames <= arena_frames, so both [offset, offset + frames)
            // ranges lie inside that allocation. Distinct buffer IDs make every lane range
            // spatially disjoint (I1); different planes are disjoint too. `&mut self` ties every
            // returned lifetime to this exclusive lease borrow, while the lease's checked write
            // set retains I1, while exclusive borrowing prevents references from this lease from
            // overlapping these writes; E1 governs any foreign reads and writes.
            unsafe {
                (
                    core::slice::from_raw_parts_mut((*cells.add(left)).get(), frames),
                    core::slice::from_raw_parts_mut((*cells.add(right)).get(), frames),
                )
            }
        });
        Some(pairs)
    }

    /// One written buffer and one read buffer in `plane`. The two must be distinct.
    #[inline]
    pub fn write_read(&mut self, plane: usize, out: u32, input: u32) -> (&mut [f32], &[f32]) {
        let out_index = self.checked_write(out);
        let in_index = self.effective(input);
        debug_assert_ne!(out_index, in_index, "a read may not alias its own output");
        let (out_start, in_start) = (
            self.arena.offset(plane, out_index),
            self.arena.offset(plane, in_index),
        );
        // SAFETY: I1 for the mutable range and I2/E1 for the shared range, as in `write` and
        // `read`; the `debug_assert` above plus distinct buffer indices make the two ranges
        // disjoint (buffers never overlap within a plane).
        unsafe {
            (
                core::slice::from_raw_parts_mut(
                    self.arena.cells[out_start].get().cast::<f32>(),
                    self.arena.frames,
                ),
                core::slice::from_raw_parts(
                    self.arena.cells[in_start].get().cast_const().cast::<f32>(),
                    self.arena.frames,
                ),
            )
        }
    }

    /// One written buffer and two read buffers in `plane`. All three must be distinct.
    #[inline]
    pub fn write_read2(
        &mut self,
        plane: usize,
        out: u32,
        first: u32,
        second: u32,
    ) -> (&mut [f32], &[f32], &[f32]) {
        let out_index = self.checked_write(out);
        let (first_index, second_index) = (self.effective(first), self.effective(second));
        debug_assert!(out_index != first_index && out_index != second_index);
        let (out_start, first_start, second_start) = (
            self.arena.offset(plane, out_index),
            self.arena.offset(plane, first_index),
            self.arena.offset(plane, second_index),
        );
        // SAFETY: I1 for the mutable range, I2/E1 for the two shared ranges. The two shared
        // ranges may be the same buffer, which is sound: they are shared references. Neither
        // can be the output buffer.
        unsafe {
            (
                core::slice::from_raw_parts_mut(
                    self.arena.cells[out_start].get().cast::<f32>(),
                    self.arena.frames,
                ),
                core::slice::from_raw_parts(
                    self.arena.cells[first_start]
                        .get()
                        .cast_const()
                        .cast::<f32>(),
                    self.arena.frames,
                ),
                core::slice::from_raw_parts(
                    self.arena.cells[second_start]
                        .get()
                        .cast_const()
                        .cast::<f32>(),
                    self.arena.frames,
                ),
            )
        }
    }

    /// Both planes of one written buffer plus both planes of one read buffer.
    #[inline]
    pub fn write_read_stereo(&mut self, out: u32, input: u32) -> ArenaStereoPair<'_> {
        let out_index = self.checked_write(out);
        let in_index = self.effective(input);
        debug_assert_ne!(out_index, in_index, "a read may not alias its own output");
        let offsets = [
            self.arena.offset(0, out_index),
            self.arena.offset(1, out_index),
            self.arena.offset(0, in_index),
            self.arena.offset(1, in_index),
        ];
        // SAFETY: I1 for the two mutable ranges (same buffer, two planes: disjoint), I2/E1 for
        // the two shared ranges, and `out_index != in_index` keeps the two pairs apart.
        unsafe {
            (
                (
                    core::slice::from_raw_parts_mut(
                        self.arena.cells[offsets[0]].get().cast::<f32>(),
                        self.arena.frames,
                    ),
                    core::slice::from_raw_parts_mut(
                        self.arena.cells[offsets[1]].get().cast::<f32>(),
                        self.arena.frames,
                    ),
                ),
                (
                    core::slice::from_raw_parts(
                        self.arena.cells[offsets[2]]
                            .get()
                            .cast_const()
                            .cast::<f32>(),
                        self.arena.frames,
                    ),
                    core::slice::from_raw_parts(
                        self.arena.cells[offsets[3]]
                            .get()
                            .cast_const()
                            .cast::<f32>(),
                        self.arena.frames,
                    ),
                ),
            )
        }
    }
    // REALTIME_POLICY_END
}

struct PendingLease {
    wave: usize,
    writes: Vec<u32>,
    reads: Vec<u32>,
}

/// Builds one plan's arena and its execution leases, proving I1 and I2 before publication.
pub struct ArenaLeaseSetBuilder {
    planes: usize,
    frames: usize,
    reserved: usize,
    leases: Vec<PendingLease>,
}

impl ArenaLeaseSetBuilder {
    /// Start an arena of `planes` planes and `frames` frames per buffer.
    ///
    /// Buffer [`ARENA_SILENCE_BUFFER`] is reserved immediately and is never writable.
    #[must_use]
    pub fn new(planes: NonZeroUsize, frames: NonZeroUsize) -> Self {
        Self {
            planes: planes.get(),
            frames: frames.get(),
            reserved: 1,
            leases: Vec::new(),
        }
    }

    /// Reserve one fresh buffer. Buffers are never recycled, which is what makes I1 structural.
    pub fn reserve(&mut self) -> u32 {
        let buffer = u32::try_from(self.reserved).expect("arena buffer index fits in u32");
        self.reserved += 1;
        buffer
    }

    /// Number of buffers reserved so far, including silence.
    #[must_use]
    pub const fn reserved(&self) -> usize {
        self.reserved
    }

    /// Declare one execution lease and return its index in the finished set.
    pub fn lease(&mut self, wave: usize, writes: Vec<u32>, reads: Vec<u32>) -> usize {
        self.leases.push(PendingLease {
            wave,
            writes,
            reads,
        });
        self.leases.len() - 1
    }

    /// Exact retained payload bytes of the arena this builder would produce.
    #[must_use]
    pub fn total_bytes(&self) -> Option<usize> {
        self.planes
            .checked_mul(self.reserved)?
            .checked_mul(self.frames)?
            .checked_mul(core::mem::size_of::<f32>())
    }

    /// Check I1 and I2 and allocate the arena.
    ///
    /// # Errors
    /// Returns the first violated invariant; nothing is allocated on failure.
    pub fn finish(self) -> Result<(Arc<DisjointArena>, Vec<ArenaLease>), DisjointArenaError> {
        let buffers = self.reserved;
        // I1: at most one writer per buffer, and never the silence buffer.
        let mut owner: Vec<Option<usize>> = vec![None; buffers];
        for (index, lease) in self.leases.iter().enumerate() {
            for buffer in &lease.writes {
                if *buffer == ARENA_SILENCE_BUFFER {
                    return Err(DisjointArenaError::SilenceWrite { lease: index });
                }
                let slot =
                    owner
                        .get_mut(*buffer as usize)
                        .ok_or(DisjointArenaError::UnknownBuffer {
                            lease: index,
                            buffer: *buffer,
                        })?;
                match slot {
                    Some(first) => {
                        return Err(DisjointArenaError::Overlap {
                            buffer: *buffer,
                            first: *first,
                            second: index,
                        });
                    }
                    None => *slot = Some(index),
                }
            }
        }
        // I2: a read resolves to the reader's own lease, or to a strictly earlier wave.
        for (index, lease) in self.leases.iter().enumerate() {
            for buffer in &lease.reads {
                if *buffer == ARENA_SILENCE_BUFFER {
                    continue;
                }
                let producer =
                    *owner
                        .get(*buffer as usize)
                        .ok_or(DisjointArenaError::UnknownBuffer {
                            lease: index,
                            buffer: *buffer,
                        })?;
                let ok = match producer {
                    Some(producer) if producer == index => true,
                    Some(producer) => self.leases[producer].wave < lease.wave,
                    None => false,
                };
                if !ok {
                    return Err(DisjointArenaError::ReadNotEarlier {
                        lease: index,
                        buffer: *buffer,
                    });
                }
            }
        }
        let words = self
            .planes
            .checked_mul(buffers)
            .and_then(|value| value.checked_mul(self.frames))
            .ok_or(DisjointArenaError::CapacityOverflow)?;
        words
            .checked_mul(core::mem::size_of::<f32>())
            .ok_or(DisjointArenaError::CapacityOverflow)?;
        let mut cells = Vec::new();
        cells.reserve_exact(words);
        cells.resize_with(words, || UnsafeCell::new(0.0));
        let arena = Arc::new(DisjointArena {
            cells: cells.into_boxed_slice(),
            planes: self.planes,
            buffers,
            frames: self.frames,
        });
        let leases = self
            .leases
            .into_iter()
            .map(|lease| {
                let mut access = vec![0_u8; buffers].into_boxed_slice();
                for buffer in lease.writes {
                    access[buffer as usize] |= ACCESS_WRITE;
                }
                ArenaLease {
                    arena: Arc::clone(&arena),
                    access,
                }
            })
            .collect();
        Ok((arena, leases))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builder() -> ArenaLeaseSetBuilder {
        ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(2).expect("planes"),
            NonZeroUsize::new(4).expect("frames"),
        )
    }

    #[test]
    fn overlapping_writes_are_rejected() {
        let mut build = builder();
        let shared = build.reserve();
        build.lease(0, vec![shared], Vec::new());
        build.lease(0, vec![shared], Vec::new());
        assert_eq!(
            build.finish().err(),
            Some(DisjointArenaError::Overlap {
                buffer: shared,
                first: 0,
                second: 1
            })
        );
    }

    #[test]
    fn a_read_from_the_same_wave_is_rejected() {
        let mut build = builder();
        let produced = build.reserve();
        let consumed = build.reserve();
        build.lease(1, vec![produced], Vec::new());
        build.lease(1, vec![consumed], vec![produced]);
        assert_eq!(
            build.finish().err(),
            Some(DisjointArenaError::ReadNotEarlier {
                lease: 1,
                buffer: produced
            })
        );
    }

    #[test]
    fn a_write_to_the_silence_buffer_is_rejected() {
        let mut build = builder();
        build.lease(0, vec![ARENA_SILENCE_BUFFER], Vec::new());
        assert_eq!(
            build.finish().err(),
            Some(DisjointArenaError::SilenceWrite { lease: 0 })
        );
    }

    #[test]
    fn an_unproduced_read_is_rejected() {
        let mut build = builder();
        let own = build.reserve();
        let ghost = build.reserve();
        build.lease(1, vec![own], vec![ghost]);
        assert_eq!(
            build.finish().err(),
            Some(DisjointArenaError::ReadNotEarlier {
                lease: 0,
                buffer: ghost
            })
        );
    }

    #[test]
    fn a_write_then_a_later_wave_read_carries_the_audio() {
        let mut build = builder();
        let produced = build.reserve();
        let consumed = build.reserve();
        build.lease(0, vec![produced], Vec::new());
        build.lease(1, vec![consumed], vec![produced]);
        let (arena, mut leases) = build.finish().expect("valid lease set");
        assert_eq!(arena.buffers(), 3);
        assert_eq!(arena.total_bytes(), 2 * 3 * 4 * 4);
        let mut consumer = leases.pop().expect("consumer lease");
        let mut producer = leases.pop().expect("producer lease");
        producer
            .write(0, produced)
            .copy_from_slice(&[1.0, 2.0, 3.0, 4.0]);
        producer
            .write(1, produced)
            .copy_from_slice(&[-1.0, -2.0, -3.0, -4.0]);
        assert_eq!(consumer.read(0, produced), &[1.0, 2.0, 3.0, 4.0]);
        let (out, input) = consumer.write_read(1, consumed, produced);
        out.copy_from_slice(input);
        assert_eq!(consumer.read(1, consumed), &[-1.0, -2.0, -3.0, -4.0]);
    }

    #[test]
    fn release_read_id_bounds_check_rejects_unreserved_buffer() {
        let mut build = builder();
        let owned = build.reserve();
        build.lease(0, vec![owned], vec![owned]);
        let (_arena, mut leases) = build.finish().expect("valid lease set");
        let lease = leases.pop().expect("the lease");
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = lease.read(0, 2);
        }));
        assert!(result.is_err(), "unreserved read ID must be rejected");
    }

    #[test]
    fn a_lease_may_read_the_buffers_it_writes_itself() {
        let mut build = builder();
        let own = build.reserve();
        let other = build.reserve();
        build.lease(3, vec![own, other], vec![own, other]);
        let (_arena, mut leases) = build.finish().expect("valid lease set");
        let lease = &mut leases[0];
        assert!(lease.writes(own) && lease.writes(other));
        lease.write(0, own).fill(0.5);
        let (out, input) = lease.write_read(0, other, own);
        out.copy_from_slice(input);
        assert_eq!(lease.read(0, other), &[0.5; 4]);
    }

    fn many_lease(planes: usize, frames: usize) -> (Vec<u32>, u32, ArenaLease) {
        let mut build = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(planes).expect("nonzero test planes"),
            NonZeroUsize::new(frames).expect("nonzero test frames"),
        );
        let writable: Vec<u32> = (0..8).map(|_| build.reserve()).collect();
        let reserved_unwritable = build.reserve();
        build.lease(0, writable.clone(), Vec::new());
        let (_arena, mut leases) = build.finish().expect("test arena");
        (writable, reserved_unwritable, leases.remove(0))
    }

    fn assert_many_rejection_keeps_plane_zero(
        lease: &mut ArenaLease,
        observed: &[u32],
        attempted: &[u32; 4],
        frames: usize,
    ) {
        for buffer in observed {
            lease.write(0, *buffer).fill(f32::from_bits(0x7fc0_3990));
            lease.write(1, *buffer).fill(f32::from_bits(0xffc0_3990));
        }
        assert!(lease.write_stereo_many(attempted, frames).is_none());
        for buffer in observed {
            assert!(
                lease
                    .read(0, *buffer)
                    .iter()
                    .all(|word| word.to_bits() == 0x7fc0_3990)
            );
            assert!(
                lease
                    .read(1, *buffer)
                    .iter()
                    .all(|word| word.to_bits() == 0xffc0_3990)
            );
        }
    }

    /// RT-1: every safe multi-borrow rejection happens before a reference or write is produced.
    #[test]
    fn stereo_many_rejects_every_invalid_shape_without_partial_writes() {
        let (ids, _, mut mono) = many_lease(1, 8);
        let four: [u32; 4] = ids[..4].try_into().expect("four ids");
        for buffer in &four {
            mono.write(0, *buffer).fill(f32::from_bits(0x7fc0_3990));
        }
        assert!(mono.write_stereo_many(&four, 8).is_none());
        for buffer in &four {
            assert!(
                mono.read(0, *buffer)
                    .iter()
                    .all(|word| word.to_bits() == 0x7fc0_3990)
            );
        }

        let (ids, unwritable, mut lease) = many_lease(2, 8);
        let four: [u32; 4] = ids[..4].try_into().expect("four ids");
        assert_many_rejection_keeps_plane_zero(
            &mut lease,
            &ids,
            &[four[0], four[0], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(
            &mut lease,
            &ids,
            &[0, four[1], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(
            &mut lease,
            &ids,
            &[unwritable, four[1], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(
            &mut lease,
            &ids,
            &[u32::MAX, four[1], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(&mut lease, &ids, &four, 9);

        let unsupported = [ids[0], ids[1], ids[2]];
        assert!(lease.write_stereo_many(&unsupported, 8).is_none());
        let eight: [u32; 8] = ids[..8].try_into().expect("eight ids");
        assert!(lease.write_stereo_many(&eight, 8).is_some());
    }

    /// E8. Concurrent leases never touch each other's words.
    ///
    /// Every lease fills every word it owns with its own tag, on its own thread, with staggered
    /// spins so the writes genuinely overlap. Once all writers join, the test checks the
    /// whole arena word by word: each buffer must carry exactly its owner's tag, and the silence
    /// buffer must still be zero. Under Miri or a thread sanitiser this is also the data-race
    /// probe for `unsafe impl Sync`.
    ///
    /// Red mutation (`MUTATIONS.md`): give two leases the same reserved buffer -- the builder
    /// rejects it (`overlapping_writes_are_rejected`); bypass the builder by widening one lease's
    /// write set -- this stress reports the foreign tag.
    #[test]
    fn concurrent_leases_never_write_a_foreign_word() {
        const LEASES: usize = 6;
        const BUFFERS_PER_LEASE: usize = 5;
        const ROUNDS: usize = 200;
        let mut build = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(2).expect("planes"),
            NonZeroUsize::new(17).expect("frames"),
        );
        let owned: Vec<Vec<u32>> = (0..LEASES)
            .map(|_| (0..BUFFERS_PER_LEASE).map(|_| build.reserve()).collect())
            .collect();
        for buffers in &owned {
            build.lease(0, buffers.clone(), Vec::new());
        }
        let (arena, leases) = build.finish().expect("valid lease set");
        let mut leases = leases;
        for round in 0..ROUNDS {
            let handles: Vec<_> = leases
                .drain(..)
                .enumerate()
                .map(|(index, mut lease)| {
                    let buffers = owned[index].clone();
                    std::thread::spawn(move || {
                        let tag = (round * LEASES + index) as f32;
                        for (position, buffer) in buffers.iter().enumerate() {
                            // Stagger the writes so they overlap rather than serialise.
                            for _ in 0..(index * 32 + position * 8) {
                                core::hint::spin_loop();
                            }
                            for plane in 0..2 {
                                lease.write(plane, *buffer).fill(tag);
                            }
                        }
                        lease
                    })
                })
                .collect();
            leases = handles
                .into_iter()
                .map(|handle| handle.join().expect("lease writer"))
                .collect();
            for (index, buffers) in owned.iter().enumerate() {
                let tag = (round * LEASES + index) as f32;
                for buffer in buffers {
                    for plane in 0..2 {
                        assert!(
                            leases[0]
                                .read(plane, *buffer)
                                .iter()
                                .all(|word| *word == tag),
                            "round {round}: buffer {buffer} carries a foreign write"
                        );
                    }
                }
            }
            assert!(
                leases[0]
                    .read(0, ARENA_SILENCE_BUFFER)
                    .iter()
                    .chain(leases[0].read(1, ARENA_SILENCE_BUFFER))
                    .all(|word| *word == 0.0),
                "the silence buffer is never written"
            );
        }
        assert_eq!(arena.buffers(), LEASES * BUFFERS_PER_LEASE + 1);
    }
}
