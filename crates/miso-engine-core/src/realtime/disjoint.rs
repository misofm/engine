//! Plan-owned disjoint audio arena for the native dependency-wave executor.
//!
//! The native executor renders one dependency wave at a time. Inside a wave, several parcels run
//! concurrently on auxiliary workers; between waves the coordinator owns every parcel. Before
//! issue #100 each parcel owned a private arena and the coordinator copied every inter-parcel
//! edge between waves, which serialised all data movement onto the render thread. The pull model
//! instead gives the whole plan **one** arena and lets each consuming parcel read its producers'
//! buffers in place, on the worker that needs them.
//!
//! That requires one allocation to be reachable, mutably, from several threads at once, which is
//! why this module owns the only `unsafe` on the render path. Soundness does not depend on any
//! worker being on time; it is a property of the lease set, proved once at bind by
//! [`ArenaLeaseSetBuilder::finish`]:
//!
//! * **I1 — writes are globally unique.** Every buffer is writable by at most one lease, for the
//!   whole life of the plan. Buffers are never recycled, so no two leases (in the same wave or in
//!   different waves) can ever address the same words mutably.
//! * **I2 — reads are strictly earlier.** A lease may read a buffer only if that buffer is
//!   written by a lease of a strictly smaller wave, or by the reading lease itself. A producer of
//!   an earlier wave has finished, and its parcel has been recovered by the coordinator, before
//!   any consumer of a later wave is issued.
//! * **I3 — waves are separated by a happens-before edge.** The scheduler issues one wave at a
//!   time and recovers every issued parcel through the SPSC release/acquire pair before it issues
//!   the next, so an earlier wave's writes are visible to a later wave's reads.
//! * **I4 — a parcel the coordinator does not own is never read.** When a worker misses its
//!   deadline the coordinator marks its buffers muted ([`ArenaLeaseV1::set_muted`]); a muted read
//!   returns the always-zero silence buffer instead. A late worker can therefore only ever write
//!   its own unique slots, which nobody reads until its parcel is reaped.
//!
//! Buffer `0` is the silence buffer. No lease may write it, so it stays zero for the life of the
//! arena and is what every muted read observes.
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

/// Flat planar `f32` storage shared by every parcel of one prepared plan.
///
/// See the module documentation for the invariants that make the shared mutable access sound.
/// Instances are produced only by [`ArenaLeaseSetBuilder::finish`].
pub struct DisjointArena {
    cells: Box<[UnsafeCell<f32>]>,
    planes: usize,
    buffers: usize,
    frames: usize,
}

// SAFETY: the arena hands out access only through `ArenaLeaseV1`, and a lease set is constructed
// only by `ArenaLeaseSetBuilder::finish`, which proves I1 and I2 (module documentation). I3 and
// I4 are discharged by the scheduler: one wave at a time with an acquire/release edge between
// waves, and no read of a parcel the coordinator does not own. Under I1 no two leases can
// mutably alias, and under I2/I3/I4 no read can overlap a concurrent write, so sending leases to
// worker threads never produces a data race.
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

/// One parcel's checked view of the shared arena.
///
/// The lease is the only way to reach arena storage. It is `Send` (it travels with its parcel to
/// an auxiliary worker) and deliberately not `Sync`.
pub struct ArenaLeaseV1 {
    arena: Arc<DisjointArena>,
    /// Per buffer: bit 0 writable by this lease, bit 1 muted for this lease.
    access: Box<[u8]>,
    wave: usize,
}

const ACCESS_WRITE: u8 = 0b01;
const ACCESS_MUTED: u8 = 0b10;

impl core::fmt::Debug for ArenaLeaseV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ArenaLeaseV1")
            .field("wave", &self.wave)
            .field("buffers", &self.arena.buffers)
            .finish()
    }
}

impl ArenaLeaseV1 {
    /// The wave this lease belongs to.
    #[must_use]
    pub const fn wave(&self) -> usize {
        self.wave
    }

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

    /// Redirect (or restore) this lease's reads of `buffer` to the silence buffer.
    ///
    /// The coordinator calls this while it owns the parcel, after a worker misses its deadline
    /// and its outputs become unreadable (I4), and again once the parcel has been reaped.
    pub fn set_muted(&mut self, buffer: u32, muted: bool) {
        if let Some(access) = self.access.get_mut(buffer as usize) {
            if muted {
                *access |= ACCESS_MUTED;
            } else {
                *access &= !ACCESS_MUTED;
            }
        }
    }

    /// Whether reads of `buffer` currently return silence.
    #[must_use]
    pub fn is_muted(&self, buffer: u32) -> bool {
        self.access
            .get(buffer as usize)
            .is_some_and(|access| access & ACCESS_MUTED != 0)
    }

    // REALTIME_POLICY_BEGIN

    #[inline]
    fn effective(&self, buffer: u32) -> usize {
        let index = buffer as usize;
        debug_assert!(index < self.arena.buffers, "read of an unreserved buffer");
        if self.access[index] & ACCESS_MUTED == 0 {
            index
        } else {
            ARENA_SILENCE_BUFFER as usize
        }
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

    /// One buffer's frames in `plane`, shared. A muted buffer reads as silence.
    #[inline]
    #[must_use]
    pub fn read(&self, plane: usize, buffer: u32) -> &[f32] {
        let start = self.arena.offset(plane, self.effective(buffer));
        // SAFETY: I1/I2/I3/I4. `start..start + frames` is in bounds by construction (the builder
        // reserved every buffer and sized the allocation `planes * buffers * frames`), and no
        // other lease may write those words while this shared reference lives: either the buffer
        // is this lease's own, or it belongs to a strictly earlier wave whose parcels have been
        // recovered, or it is the never-written silence buffer.
        unsafe {
            core::slice::from_raw_parts(
                self.arena.cells[start].get().cast_const().cast::<f32>(),
                self.arena.frames,
            )
        }
    }

    /// Both planes of one buffer, shared. A muted buffer reads as silence.
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
        // SAFETY: I1 for the mutable range and I2/I3/I4 for the shared range, as in `write` and
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
        // SAFETY: I1 for the mutable range, I2/I3/I4 for the two shared ranges. The two shared
        // ranges may be the same buffer (two muted reads both resolve to silence), which is
        // sound: they are shared references. Neither can be the output buffer.
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
    pub fn write_read_stereo(
        &mut self,
        out: u32,
        input: u32,
    ) -> ((&mut [f32], &mut [f32]), (&[f32], &[f32])) {
        let out_index = self.checked_write(out);
        let in_index = self.effective(input);
        debug_assert_ne!(out_index, in_index, "a read may not alias its own output");
        let offsets = [
            self.arena.offset(0, out_index),
            self.arena.offset(1, out_index),
            self.arena.offset(0, in_index),
            self.arena.offset(1, in_index),
        ];
        // SAFETY: I1 for the two mutable ranges (same buffer, two planes: disjoint), I2/I3/I4 for
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

/// Builds one plan's arena and its per-parcel leases, proving I1 and I2 before publication.
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

    /// Declare one parcel's lease and return its index in the finished set.
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
    pub fn finish(self) -> Result<(Arc<DisjointArena>, Vec<ArenaLeaseV1>), DisjointArenaError> {
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
                ArenaLeaseV1 {
                    arena: Arc::clone(&arena),
                    access,
                    wave: lease.wave,
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
    fn a_muted_read_is_silence_and_unmuting_restores_it() {
        let mut build = builder();
        let produced = build.reserve();
        let consumed = build.reserve();
        build.lease(0, vec![produced], Vec::new());
        build.lease(1, vec![consumed], vec![produced]);
        let (_arena, mut leases) = build.finish().expect("valid lease set");
        let mut consumer = leases.pop().expect("consumer lease");
        let mut producer = leases.pop().expect("producer lease");
        producer.write(0, produced).fill(7.0);
        assert_eq!(consumer.read(0, produced), &[7.0; 4]);
        consumer.set_muted(produced, true);
        assert!(consumer.is_muted(produced));
        assert_eq!(consumer.read(0, produced), &[0.0; 4]);
        consumer.set_muted(produced, false);
        assert_eq!(consumer.read(0, produced), &[7.0; 4]);
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

    #[test]
    fn leases_travel_to_other_threads() {
        let mut build = builder();
        let a = build.reserve();
        let b = build.reserve();
        build.lease(0, vec![a], Vec::new());
        build.lease(0, vec![b], Vec::new());
        let (_arena, leases) = build.finish().expect("valid lease set");
        let buffers = [a, b];
        let handles: Vec<_> = leases
            .into_iter()
            .zip(buffers)
            .map(|(mut lease, buffer)| {
                std::thread::spawn(move || {
                    lease.write(0, buffer).fill(buffer as f32);
                    lease
                })
            })
            .collect();
        for (index, handle) in handles.into_iter().enumerate() {
            let lease = handle.join().expect("worker");
            assert_eq!(lease.read(0, buffers[index]), &[buffers[index] as f32; 4]);
        }
    }
}
