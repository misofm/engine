//! Plan-owned planar audio storage for the sequential render executor.
//!
//! One non-cloneable owner holds every buffer. Shared access borrows that owner; all mutable
//! access requires its exclusive borrow, so no foreign writer can overlap a read. Buffer `0` is
//! the immutable silence buffer. Other reserved buffers remain writable for the life of the plan.
//! Combined borrows validate bounds and spatial disjointness before forming any references.

#![allow(unsafe_code)]

use core::num::NonZeroUsize;

/// The silence buffer every arena reserves: always zero, writable by nobody.
pub const ARENA_SILENCE_BUFFER: u32 = 0;

/// A bind-time rejection of an oversized arena.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisjointArenaError {
    /// Buffer count, word count, or slice allocation bytes exceed the platform capacity.
    CapacityOverflow,
}

/// Flat planar `f32` storage exclusively owned by one prepared plan.
pub struct DisjointArena {
    cells: Box<[f32]>,
    planes: usize,
    buffers: usize,
    frames: usize,
}

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

/// Both planes of one written buffer plus both planes of one read buffer.
pub type ArenaStereoPair<'a> = ((&'a mut [f32], &'a mut [f32]), (&'a [f32], &'a [f32]));
/// Both writable planes of one arena buffer.
pub type ArenaStereoPlanes<'a> = (&'a mut [f32], &'a mut [f32]);

impl DisjointArena {
    /// Allocate zeroed planar storage on the control thread.
    ///
    /// Buffer `0` is reserved for silence; writable buffers have IDs `1..=writable_buffers`.
    ///
    /// # Errors
    /// Returns [`DisjointArenaError::CapacityOverflow`] before allocation when the silence slot,
    /// word/byte products, or the platform's maximum slice allocation cannot be represented.
    pub fn try_new(
        planes: NonZeroUsize,
        frames: NonZeroUsize,
        writable_buffers: u32,
    ) -> Result<Self, DisjointArenaError> {
        let buffers = usize::try_from(writable_buffers)
            .ok()
            .and_then(|count| count.checked_add(1))
            .ok_or(DisjointArenaError::CapacityOverflow)?;
        let words = planes
            .get()
            .checked_mul(buffers)
            .and_then(|value| value.checked_mul(frames.get()))
            .ok_or(DisjointArenaError::CapacityOverflow)?;
        let bytes = words
            .checked_mul(core::mem::size_of::<f32>())
            .ok_or(DisjointArenaError::CapacityOverflow)?;
        if bytes > isize::MAX as usize {
            return Err(DisjointArenaError::CapacityOverflow);
        }
        Ok(Self {
            cells: vec![0.0; words].into_boxed_slice(),
            planes: planes.get(),
            buffers,
            frames: frames.get(),
        })
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

    /// Whether `buffer` is a reserved writable buffer.
    #[must_use]
    pub fn writes(&self, buffer: u32) -> bool {
        buffer != ARENA_SILENCE_BUFFER && (buffer as usize) < self.buffers
    }

    // REALTIME_POLICY_BEGIN

    #[inline]
    fn checked_buffer(&self, buffer: u32) -> usize {
        let index = buffer as usize;
        assert!(index < self.buffers, "unreserved arena buffer");
        index
    }

    #[inline]
    fn checked_write(&self, buffer: u32) -> usize {
        assert!(self.writes(buffer), "arena buffer is not writable");
        buffer as usize
    }

    #[inline]
    fn offset(&self, plane: usize, buffer: usize) -> usize {
        assert!(plane < self.planes, "invalid arena plane");
        assert!(buffer < self.buffers, "unreserved arena buffer");
        // Checked construction bounds the full product. Valid indices keep every intermediate
        // and the complete frames-long range within that product, before references are formed.
        (plane * self.buffers + buffer) * self.frames
    }

    /// One buffer's frames in `plane`, shared.
    ///
    /// Panics on an invalid plane or unreserved buffer, before borrowing storage.
    #[inline]
    #[must_use]
    pub fn read(&self, plane: usize, buffer: u32) -> &[f32] {
        let start = self.offset(plane, self.checked_buffer(buffer));
        &self.cells[start..start + self.frames]
    }

    /// Both planes of one buffer, shared.
    ///
    /// Panics unless the arena has two planes and the buffer is reserved.
    #[inline]
    #[must_use]
    pub fn read_stereo(&self, buffer: u32) -> (&[f32], &[f32]) {
        assert!(self.planes >= 2, "stereo access requires two arena planes");
        let index = self.checked_buffer(buffer);
        let left = self.offset(0, index);
        let right = self.offset(1, index);
        (
            &self.cells[left..left + self.frames],
            &self.cells[right..right + self.frames],
        )
    }

    /// One buffer's frames in `plane`, exclusively.
    ///
    /// Panics on an invalid plane, unreserved buffer, or silence write before borrowing storage.
    #[inline]
    pub fn write(&mut self, plane: usize, buffer: u32) -> &mut [f32] {
        let start = self.offset(plane, self.checked_write(buffer));
        &mut self.cells[start..start + self.frames]
    }

    /// Both planes of one writable buffer, exclusively.
    ///
    /// Panics unless the arena has two planes and the buffer is writable.
    #[inline]
    pub fn write_stereo(&mut self, buffer: u32) -> (&mut [f32], &mut [f32]) {
        assert!(self.planes >= 2, "stereo access requires two arena planes");
        let index = self.checked_write(buffer);
        let start = self.offset(0, index);
        let stride = self.buffers * self.frames;
        let (left, right) = self.cells.split_at_mut(stride);
        (
            &mut left[start..start + self.frames],
            &mut right[start..start + self.frames],
        )
    }

    /// Borrow pairwise-disjoint stereo outputs for a full four- or eight-lane bank.
    ///
    /// Invalid shape, non-writable or repeated outputs return `None` before any reference forms.
    pub fn write_stereo_many<const W: usize>(
        &mut self,
        buffers: &[u32; W],
        frames: usize,
    ) -> Option<[ArenaStereoPlanes<'_>; W]> {
        if self.planes < 2 || (W != 4 && W != 8) || frames > self.frames {
            return None;
        }
        for (index, buffer) in buffers.iter().copied().enumerate() {
            if !self.writes(buffer) || buffers[..index].contains(&buffer) {
                return None;
            }
        }
        let stride = self.buffers * self.frames;
        let starts = buffers.map(|buffer| buffer as usize * self.frames);
        let cells = self.cells.as_mut_ptr();
        Some(core::array::from_fn(|lane| {
            let left = starts[lane];
            let right = stride + left;
            // SAFETY: checked construction and the preceding bounds/shape checks keep every
            // requested range within the allocation. Distinct nonzero buffer IDs separate all
            // lane ranges; the two planes are separated by stride. Every pointer derives from
            // the complete Box allocation, and every returned borrow is tied to this exclusive
            // owner borrow, which excludes other reads/writes for its lifetime.
            unsafe {
                (
                    core::slice::from_raw_parts_mut(cells.add(left), frames),
                    core::slice::from_raw_parts_mut(cells.add(right), frames),
                )
            }
        }))
    }

    /// One writable output and one shared input in `plane`.
    ///
    /// Panics on invalid bounds, a silence output or an output/input alias before borrowing.
    #[inline]
    pub fn write_read(&mut self, plane: usize, out: u32, input: u32) -> (&mut [f32], &[f32]) {
        let (output, [input]) = self.write_read_checked(plane, out, &[input]);
        (output, input)
    }

    /// One writable output and two shared inputs in `plane`. Shared inputs may repeat.
    ///
    /// Panics on invalid bounds, a silence output or an output/input alias before borrowing.
    #[inline]
    pub fn write_read2(
        &mut self,
        plane: usize,
        out: u32,
        first: u32,
        second: u32,
    ) -> (&mut [f32], &[f32], &[f32]) {
        let (output, [first, second]) = self.write_read_checked(plane, out, &[first, second]);
        (output, first, second)
    }

    /// Split around the output after checking every request. The prefix and suffix are shared,
    /// allowing repeated inputs while the complete output buffer remains exclusively borrowed.
    #[inline]
    fn write_read_checked<const N: usize>(
        &mut self,
        plane: usize,
        out: u32,
        inputs: &[u32; N],
    ) -> (&mut [f32], [&[f32]; N]) {
        let out_start = self.offset(plane, self.checked_write(out));
        let in_starts = inputs.map(|input| {
            assert_ne!(input, out, "a read may not alias its own output");
            self.offset(plane, self.checked_buffer(input))
        });
        let frames = self.frames;
        let out_end = out_start + frames;
        let (before, remainder) = self.cells.split_at_mut(out_start);
        let (output, after) = remainder.split_at_mut(frames);
        let before: &[f32] = before;
        let after: &[f32] = after;
        let reads = in_starts.map(|start| {
            if start < out_start {
                &before[start..start + frames]
            } else {
                // Every buffer has the same frame extent and inputs differ from the output,
                // so a later input starts at or beyond out_end.
                let relative = start - out_end;
                &after[relative..relative + frames]
            }
        });
        (output, reads)
    }

    /// One writable output and one to eight shared inputs in `plane`, acquired once per call.
    ///
    /// Shared inputs may repeat or name silence; none may be the output. Invalid planes, IDs or
    /// output/input aliases return `None` before any reference is formed.
    #[inline]
    pub fn write_read_many<const N: usize>(
        &mut self,
        plane: usize,
        out: u32,
        inputs: &[u32; N],
    ) -> Option<(&mut [f32], [&[f32]; N])> {
        const { assert!(N >= 1 && N <= 8, "write_read_many forms one to eight reads") };
        if plane >= self.planes || !self.writes(out) {
            return None;
        }
        if inputs
            .iter()
            .any(|&input| input == out || input as usize >= self.buffers)
        {
            return None;
        }
        let frames = self.frames;
        let out_start = self.offset(plane, out as usize);
        let in_starts = inputs.map(|input| self.offset(plane, input as usize));
        let cells = self.cells.as_mut_ptr();
        // SAFETY: constructor products and validated plane/IDs bound every frames-long range.
        // Each input ID differs from the output, so none of the shared ranges overlaps the
        // mutable one. Shared ranges may overlap one another. Whole-allocation pointers derive
        // from the single exclusive Box owner, and the returned lifetimes retain that borrow.
        unsafe {
            Some((
                core::slice::from_raw_parts_mut(cells.add(out_start), frames),
                in_starts.map(|start| {
                    core::slice::from_raw_parts(cells.add(start).cast_const(), frames)
                }),
            ))
        }
    }

    /// Both planes of one writable output plus both planes of one shared input.
    ///
    /// Panics on invalid stereo shape/IDs, silence output or output/input alias before borrowing.
    #[inline]
    pub fn write_read_stereo(&mut self, out: u32, input: u32) -> ArenaStereoPair<'_> {
        assert!(self.planes >= 2, "stereo access requires two arena planes");
        let out_index = self.checked_write(out);
        let in_index = self.checked_buffer(input);
        assert_ne!(out_index, in_index, "a read may not alias its own output");
        let offsets = [
            self.offset(0, out_index),
            self.offset(1, out_index),
            self.offset(0, in_index),
            self.offset(1, in_index),
        ];
        let frames = self.frames;
        let cells = self.cells.as_mut_ptr();
        // SAFETY: validated stereo shape/IDs and checked construction bound all four ranges.
        // Distinct buffers separate output from input, while plane-major layout separates each
        // pair. The raw pointers derive from the complete Box allocation; exclusive borrowing
        // of the single owner prevents any other access during these returned lifetimes.
        unsafe {
            (
                (
                    core::slice::from_raw_parts_mut(cells.add(offsets[0]), frames),
                    core::slice::from_raw_parts_mut(cells.add(offsets[1]), frames),
                ),
                (
                    core::slice::from_raw_parts(cells.add(offsets[2]).cast_const(), frames),
                    core::slice::from_raw_parts(cells.add(offsets[3]).cast_const(), frames),
                ),
            )
        }
    }
    // REALTIME_POLICY_END
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arena() -> DisjointArena {
        DisjointArena::try_new(
            NonZeroUsize::new(2).expect("planes"),
            NonZeroUsize::new(4).expect("frames"),
            2,
        )
        .expect("test arena")
    }

    #[test]
    fn a_write_then_a_read_carries_the_audio() {
        let mut arena = arena();
        assert_eq!(arena.buffers(), 3);
        assert!(arena.writes(1) && arena.writes(2));
        assert!(!arena.writes(0) && !arena.writes(3));
        assert!(
            arena.total_bytes() <= 128,
            "payload fits the configured ceiling"
        );
        arena.write(0, 1).copy_from_slice(&[1.0, 2.0, 3.0, 4.0]);
        arena.write(1, 1).copy_from_slice(&[-1.0, -2.0, -3.0, -4.0]);
        assert_eq!(arena.read(0, 1), &[1.0, 2.0, 3.0, 4.0]);
        let (out, input) = arena.write_read(1, 2, 1);
        out.copy_from_slice(input);
        assert_eq!(arena.read(1, 2), &[-1.0, -2.0, -3.0, -4.0]);
        let (out, first, second) = arena.write_read2(0, 2, 1, 1);
        assert_eq!(first, &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(first.as_ptr(), second.as_ptr());
        out.copy_from_slice(first);
        assert_eq!(arena.read(0, 2), &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn direct_borrows_reject_invalid_ids_planes_and_aliases_in_release() {
        let mut arena = arena();
        for plane in 0..2 {
            for buffer in 1..=2 {
                arena
                    .write(plane, buffer)
                    .fill(f32::from_bits(0x7fc0_1154 + buffer));
            }
        }
        for plane in [2, usize::MAX, usize::MAX / 4 + 1] {
            refused(&mut arena, |a| {
                let _ = a.read(plane, 1);
            });
            refused(&mut arena, |a| {
                let _ = a.write(plane, 1);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read(plane, 2, 1);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read2(plane, 2, 1, 1);
            });
        }
        for buffer in [3, u32::MAX] {
            refused(&mut arena, |a| {
                let _ = a.read(0, buffer);
            });
            refused(&mut arena, |a| {
                let _ = a.write(0, buffer);
            });
            refused(&mut arena, |a| {
                let _ = a.read_stereo(buffer);
            });
            refused(&mut arena, |a| {
                let _ = a.write_stereo(buffer);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read(0, buffer, 1);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read(0, 1, buffer);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read2(0, buffer, 1, 1);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read2(0, 1, buffer, 2);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read2(0, 1, 2, buffer);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read_stereo(buffer, 1);
            });
            refused(&mut arena, |a| {
                let _ = a.write_read_stereo(1, buffer);
            });
        }
        refused(&mut arena, |a| {
            let _ = a.write(0, 0);
        });
        refused(&mut arena, |a| {
            let _ = a.write_stereo(0);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read(0, 0, 1);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read2(0, 0, 1, 2);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read_stereo(0, 1);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read(0, 1, 1);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read2(0, 1, 1, 2);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read2(0, 1, 2, 1);
        });
        refused(&mut arena, |a| {
            let _ = a.write_read_stereo(1, 1);
        });

        let mut mono = DisjointArena::try_new(
            NonZeroUsize::new(1).expect("one plane"),
            NonZeroUsize::new(4).expect("frames"),
            2,
        )
        .expect("mono arena");
        mono.write(0, 1).fill(f32::from_bits(0x7fc0_1154));
        refused(&mut mono, |a| {
            let _ = a.read_stereo(1);
        });
        refused(&mut mono, |a| {
            let _ = a.write_stereo(1);
        });
        refused(&mut mono, |a| {
            let _ = a.write_read_stereo(1, 2);
        });
    }

    fn refused(arena: &mut DisjointArena, request: impl FnOnce(&mut DisjointArena)) {
        let before: Vec<_> = (0..arena.planes)
            .flat_map(|plane| (0..arena.buffers as u32).map(move |buffer| (plane, buffer)))
            .map(|(plane, buffer)| (plane, buffer, bits(arena.read(plane, buffer))))
            .collect();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| request(arena)));
        assert!(result.is_err(), "invalid direct borrow must be rejected");
        for (plane, buffer, words) in before {
            assert_eq!(
                bits(arena.read(plane, buffer)),
                words,
                "refusal kept plane {plane}, buffer {buffer}"
            );
        }
    }

    #[test]
    fn oversized_arenas_are_rejected_before_allocation() {
        for (planes, frames, buffers) in [
            (usize::MAX, 1, u32::MAX),
            (usize::MAX, 2, 0),
            (1, usize::MAX, 0),
            (1, isize::MAX as usize / core::mem::size_of::<f32>() + 1, 0),
        ] {
            assert_eq!(
                DisjointArena::try_new(
                    NonZeroUsize::new(planes).expect("nonzero planes"),
                    NonZeroUsize::new(frames).expect("nonzero frames"),
                    buffers,
                )
                .err(),
                Some(DisjointArenaError::CapacityOverflow),
            );
        }
        #[cfg(target_pointer_width = "32")]
        assert_eq!(
            DisjointArena::try_new(
                NonZeroUsize::new(1).expect("one plane"),
                NonZeroUsize::new(1).expect("one frame"),
                u32::MAX,
            )
            .err(),
            Some(DisjointArenaError::CapacityOverflow),
        );
        let silence = DisjointArena::try_new(
            NonZeroUsize::new(1).expect("one plane"),
            NonZeroUsize::new(1).expect("one frame"),
            0,
        )
        .expect("silence-only arena");
        assert_eq!(silence.buffers(), 1);
        assert!(!silence.writes(0));
        assert_eq!(silence.read(0, 0), &[0.0]);
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    /// Borrows `inputs` and `out` through one `write_read_many` call and checks every slice
    /// against the single-buffer borrow of the same buffer: same address, same length, same
    /// words. It then writes a marker through the output and checks that it landed in `out`'s
    /// `plane` and nowhere else.
    fn assert_many_is_the_single_borrows<const N: usize>(
        arena: &mut DisjointArena,
        frames: usize,
        plane: usize,
        out: u32,
        inputs: [u32; N],
    ) {
        const MARKER: u32 = 0x7fc0_0898;
        let expected: Vec<(usize, Vec<u32>)> = inputs
            .iter()
            .map(|&input| {
                let words = arena.read(plane, input);
                (words.as_ptr() as usize, bits(words))
            })
            .collect();
        let out_address = arena.write(plane, out).as_ptr() as usize;
        let other_plane = bits(arena.read(1 - plane, out));
        {
            let (output, reads) = arena
                .write_read_many(plane, out, &inputs)
                .expect("a sound borrow set");
            assert_eq!(output.as_ptr() as usize, out_address, "output address");
            assert_eq!(output.len(), frames, "output length");
            for (index, (read, (address, words))) in reads.iter().zip(&expected).enumerate() {
                assert_eq!(read.as_ptr() as usize, *address, "read {index} address");
                assert_eq!(read.len(), frames, "read {index} length");
                assert_eq!(bits(read), *words, "read {index} words");
            }
            output.fill(f32::from_bits(MARKER));
        }
        assert!(
            bits(arena.read(plane, out))
                .iter()
                .all(|&word| word == MARKER)
        );
        assert_eq!(bits(arena.read(1 - plane, out)), other_plane);
        for (input, (_, words)) in inputs.iter().zip(&expected) {
            assert_eq!(&bits(arena.read(plane, *input)), words, "read {input} kept");
        }
    }

    /// Issue #898: the slices one `write_read_many` call forms are exactly the buffers the
    /// single-buffer borrows name, in the requested plane, at every supported count, with
    /// repeated reads and the silence buffer among them.
    ///
    /// Red mutations: compute a read's offset in the other plane, or off by one buffer -- the
    /// address and word checks fail.
    #[test]
    fn write_read_many_forms_the_slices_the_single_borrows_form() {
        const FRAMES: usize = 5;
        let mut storage = DisjointArena::try_new(
            NonZeroUsize::new(2).expect("planes"),
            NonZeroUsize::new(FRAMES).expect("frames"),
            10,
        )
        .expect("test arena");
        let owned: Vec<u32> = (1..=10).collect();
        let arena = &mut storage;
        for plane in 0..2_u32 {
            for &buffer in &owned {
                for (frame, word) in arena.write(plane as usize, buffer).iter_mut().enumerate() {
                    *word = f32::from_bits(0x4000_0000 | plane << 12 | buffer << 4 | frame as u32);
                }
            }
        }
        let out = owned[9];
        for plane in 0..2 {
            assert_many_is_the_single_borrows(arena, FRAMES, plane, out, [owned[2]]);
            assert_many_is_the_single_borrows(arena, FRAMES, plane, out, [owned[1], owned[1]]);
            assert_many_is_the_single_borrows(
                arena,
                FRAMES,
                plane,
                out,
                [ARENA_SILENCE_BUFFER, owned[3], ARENA_SILENCE_BUFFER],
            );
            assert_many_is_the_single_borrows(
                arena,
                FRAMES,
                plane,
                out,
                [
                    owned[0], owned[1], owned[2], owned[3], owned[4], owned[5], owned[6], owned[7],
                ],
            );
            assert_many_is_the_single_borrows(
                arena,
                FRAMES,
                plane,
                out,
                [
                    owned[8],
                    ARENA_SILENCE_BUFFER,
                    owned[8],
                    owned[1],
                    owned[1],
                    ARENA_SILENCE_BUFFER,
                    owned[4],
                    owned[2],
                ],
            );
        }
    }

    /// Issue #898: `write_read_many` checks every premise of its disjointness argument in
    /// release and refuses, forming nothing, when one fails.
    ///
    /// Red mutations: drop the read-is-output check, the reserved-read check, the writable-output check
    /// or the plane check -- the matching `is_none` assertion fails.
    #[test]
    fn write_read_many_refuses_every_unsound_borrow() {
        let mut storage = DisjointArena::try_new(
            NonZeroUsize::new(2).expect("planes"),
            NonZeroUsize::new(4).expect("frames"),
            3,
        )
        .expect("test arena");
        let (owned, first, other, unreserved) = (1, 2, 3, 4);
        let arena = &mut storage;
        assert!(
            arena
                .write_read_many(0, owned, &[first, other, ARENA_SILENCE_BUFFER])
                .is_some()
        );
        assert!(arena.write_read_many(1, owned, &[first]).is_some());
        // A read that is the output would alias the one mutable slice.
        assert!(arena.write_read_many(0, owned, &[first, owned]).is_none());
        assert!(arena.write_read_many(1, owned, &[owned]).is_none());
        // The silence buffer and buffers that were never reserved are not writable.
        assert!(
            arena
                .write_read_many(0, ARENA_SILENCE_BUFFER, &[first])
                .is_none()
        );
        assert!(arena.write_read_many(0, unreserved, &[first]).is_none());
        assert!(arena.write_read_many(0, u32::MAX, &[first]).is_none());
        // A read that was never reserved.
        assert!(
            arena
                .write_read_many(0, owned, &[first, unreserved])
                .is_none()
        );
        assert!(arena.write_read_many(0, owned, &[u32::MAX]).is_none());
        // A plane the arena does not have.
        assert!(arena.write_read_many(2, owned, &[first]).is_none());
        assert!(arena.write_read_many(usize::MAX, owned, &[first]).is_none());
    }

    fn many_arena(planes: usize, frames: usize) -> (Vec<u32>, DisjointArena) {
        let arena = DisjointArena::try_new(
            NonZeroUsize::new(planes).expect("nonzero test planes"),
            NonZeroUsize::new(frames).expect("nonzero test frames"),
            8,
        )
        .expect("test arena");
        ((1..=8).collect(), arena)
    }

    fn assert_many_rejection_keeps_plane_zero(
        arena: &mut DisjointArena,
        observed: &[u32],
        attempted: &[u32; 4],
        frames: usize,
    ) {
        for buffer in observed {
            arena.write(0, *buffer).fill(f32::from_bits(0x7fc0_3990));
            arena.write(1, *buffer).fill(f32::from_bits(0xffc0_3990));
        }
        assert!(arena.write_stereo_many(attempted, frames).is_none());
        for buffer in observed {
            assert!(
                arena
                    .read(0, *buffer)
                    .iter()
                    .all(|word| word.to_bits() == 0x7fc0_3990)
            );
            assert!(
                arena
                    .read(1, *buffer)
                    .iter()
                    .all(|word| word.to_bits() == 0xffc0_3990)
            );
        }
    }

    /// RT-1: every safe multi-borrow rejection happens before a reference or write is produced.
    #[test]
    fn stereo_many_rejects_every_invalid_shape_without_partial_writes() {
        let (ids, mut mono) = many_arena(1, 8);
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

        let (ids, mut arena) = many_arena(2, 8);
        let four: [u32; 4] = ids[..4].try_into().expect("four ids");
        assert_many_rejection_keeps_plane_zero(
            &mut arena,
            &ids,
            &[four[0], four[0], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(
            &mut arena,
            &ids,
            &[0, four[1], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(
            &mut arena,
            &ids,
            &[u32::MAX, four[1], four[2], four[3]],
            8,
        );
        assert_many_rejection_keeps_plane_zero(&mut arena, &ids, &four, 9);

        let unsupported = [ids[0], ids[1], ids[2]];
        assert!(arena.write_stereo_many(&unsupported, 8).is_none());
        let eight: [u32; 8] = ids[..8].try_into().expect("eight ids");
        assert!(arena.write_stereo_many(&eight, 8).is_some());
    }
}
