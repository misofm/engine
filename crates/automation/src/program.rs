//! The control-side builder and the immutable program render reads.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::{Completion, JumpLength, Segment, Shape, value_at};

/// Times are exact in `f64` below this bound, which the evaluator relies on: a segment's start and
/// end sample are both below it, so the largest accepted sample is `TIME_LIMIT - 1`. The session
/// validator refuses a segment at or past it with the same constant (`numeric.out_of_schema_range`
/// at the sample's path), so a validated session never reaches [`BuildError::Segment`] on it.
pub const TIME_LIMIT: u64 = 1 << 53;

/// The least distance, in node-time samples, between two discontinuities of one cell: the session
/// refuses closer ones (`automation.discontinuity_spacing`, #1626 D3), and the builder refuses
/// them again, typed, because the per-block event bound ([`CellProgram::max_events`]) rests on it.
pub const DISCONTINUITY_SPACING: u32 = 64;

/// One cell, in neutral terms: its table and the parameters of its events.
#[derive(Clone, Copy, Debug)]
pub struct CellDescription<'a> {
    /// The cell's segments, ordered and disjoint, at least one.
    pub segments: &'a [Segment],
    /// `L`: the length of a grid ramp, in samples.
    pub grid_ramp: u32,
    /// `G`: the grid period in node-time samples. A multiple of 64 and exactly the larger of 64
    /// and `grid_ramp`.
    pub grid: u32,
    /// Where a jump's ramp length comes from.
    pub jump: JumpLength,
    /// Where a ramp started at `τ` completes.
    pub completion: Completion,
    /// A target whose ramp cannot restart while one runs (the delay time).
    pub no_restart: bool,
}

/// Why a description is refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildError {
    /// The cell has no segment.
    NoSegments {
        /// The cell's index.
        cell: usize,
    },
    /// A segment has `end <= start`, a time at or past `2^53`, or a non-finite value.
    Segment {
        /// The cell's index.
        cell: usize,
        /// The segment's index.
        segment: usize,
    },
    /// A segment starts before the previous one ends.
    Order {
        /// The cell's index.
        cell: usize,
        /// The later segment's index.
        segment: usize,
    },
    /// An `exponential` segment has a value that is not positive.
    NonPositiveExponential {
        /// The cell's index.
        cell: usize,
        /// The segment's index.
        segment: usize,
    },
    /// `grid` is not a multiple of 64, or differs from the larger of 64 and `grid_ramp`.
    Grid {
        /// The cell's index.
        cell: usize,
    },
    /// Two discontinuities of the cell are closer than [`DISCONTINUITY_SPACING`] samples.
    Spacing {
        /// The cell's index.
        cell: usize,
        /// The node time of the later discontinuity of the first close pair.
        time: i64,
    },
    /// A count does not fit the program's `u32` indexes.
    TooLarge,
    /// The allocator refused the reservation.
    Allocation,
}

/// What one cell stores: ranges into the program's flat arrays and its event parameters.
#[derive(Clone, Copy, Debug)]
struct CellRecord {
    segments: (u32, u32),
    discontinuities: (u32, u32),
    grid: u32,
    ramp: u32,
    jump: JumpLength,
    completion: Completion,
    no_restart: bool,
}

/// The built, immutable cells of one plan.
#[derive(Debug)]
pub struct Program {
    segments: Box<[Segment]>,
    discontinuities: Box<[i64]>,
    cells: Box<[CellRecord]>,
}

/// A cell as render reads it: borrowed slices and parameters.
#[derive(Clone, Copy, Debug)]
pub struct CellProgram<'a> {
    pub(crate) segments: &'a [Segment],
    /// The node times at which the table is discontinuous, ascending and distinct.
    pub(crate) discontinuities: &'a [i64],
    pub(crate) grid: u32,
    pub(crate) ramp: u32,
    pub(crate) jump: JumpLength,
    pub(crate) completion: Completion,
    pub(crate) no_restart: bool,
}

// REALTIME_POLICY_BEGIN
impl<'a> CellProgram<'a> {
    /// The cell's segment slice, the argument of [`value_at`].
    #[must_use]
    pub const fn segments(&self) -> &'a [Segment] {
        self.segments
    }

    /// The node times at which the table is discontinuous, ascending.
    #[must_use]
    pub const fn discontinuities(&self) -> &'a [i64] {
        self.discontinuities
    }

    /// `G`, the grid period.
    #[must_use]
    pub const fn grid(&self) -> u32 {
        self.grid
    }

    /// The most events one block of `frames` samples yields for this cell: the capacity a
    /// per-cell event buffer needs.
    ///
    /// A block is at most two runs (before and after a seek step) of `a + b = frames - 1` samples,
    /// or one run of `frames`. Each run holds at most `⌈run/64⌉` grid points (`G` is at least 64)
    /// and, with discontinuities at least [`DISCONTINUITY_SPACING`] apart, `⌈run/64⌉` of them, and
    /// `⌈a/64⌉ + ⌈b/64⌉ <= ⌈frames/64⌉ + 1`. The seek adds its landing. A `no_restart` cell can
    /// also fire, in the run before the step, one jump deferred from an earlier block (it carries
    /// every discontinuity its ramp covered, so later discontinuities of the run still come at
    /// least 64 apart): `2⌈frames/64⌉ + 3`, plus one for a `no_restart` cell.
    #[must_use]
    pub const fn max_events(&self, frames: u32) -> u32 {
        let per_kind = frames.div_ceil(64) + 1;
        2 * per_kind + 1 + if self.no_restart { 1 } else { 0 }
    }
}

impl Program {
    /// The number of cells.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// `true` when the program has no cell.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Cell `index` as render reads it.
    #[must_use]
    pub fn cell(&self, index: usize) -> Option<CellProgram<'_>> {
        let record = self.cells.get(index)?;
        let segments = self.segments.get(range(record.segments))?;
        let discontinuities = self.discontinuities.get(range(record.discontinuities))?;
        Some(CellProgram {
            segments,
            discontinuities,
            grid: record.grid,
            ramp: record.ramp,
            jump: record.jump,
            completion: record.completion,
            no_restart: record.no_restart,
        })
    }
}

fn range((start, len): (u32, u32)) -> core::ops::Range<usize> {
    let start = start as usize;
    start..start + len as usize
}
// REALTIME_POLICY_END

/// Builds a [`Program`] from cell descriptions.
#[derive(Clone, Copy, Debug)]
pub struct ProgramBuilder;

impl ProgramBuilder {
    /// The exact bytes [`Self::build`] allocates for `cells`, which preparation charges first.
    #[must_use]
    pub fn bytes(cells: &[CellDescription<'_>]) -> u64 {
        let segments: u64 = cells.iter().map(|cell| cell.segments.len() as u64).sum();
        let discontinuities: u64 = cells
            .iter()
            .map(|cell| count_discontinuities(cell.segments) as u64)
            .sum();
        segments * size_of::<Segment>() as u64
            + discontinuities * size_of::<i64>() as u64
            + cells.len() as u64 * size_of::<CellRecord>() as u64
    }

    /// Validates `cells` and builds their program, reserving each array once and fallibly.
    ///
    /// # Errors
    ///
    /// The first [`BuildError`] found; nothing is allocated before validation passes.
    pub fn build(cells: &[CellDescription<'_>]) -> Result<Program, BuildError> {
        let mut segment_count = 0_usize;
        let mut discontinuity_count = 0_usize;
        for (index, cell) in cells.iter().enumerate() {
            validate(index, cell)?;
            segment_count += cell.segments.len();
            discontinuity_count += count_discontinuities(cell.segments);
        }
        // Positions are `u32`, and the unpositioned cursor sentinel is `u32::MAX`.
        if u32::try_from(segment_count).is_err() || u32::try_from(discontinuity_count + 1).is_err()
        {
            return Err(BuildError::TooLarge);
        }
        let mut segments = reserve::<Segment>(segment_count)?;
        let mut discontinuities = reserve::<i64>(discontinuity_count)?;
        let mut records = reserve::<CellRecord>(cells.len())?;
        for cell in cells {
            let discontinuity_start = discontinuities.len();
            for_each_discontinuity(cell.segments, |time| discontinuities.push(time));
            records.push(CellRecord {
                segments: (to_u32(segments.len()), to_u32(cell.segments.len())),
                discontinuities: (
                    to_u32(discontinuity_start),
                    to_u32(discontinuities.len() - discontinuity_start),
                ),
                grid: cell.grid,
                ramp: cell.grid_ramp,
                jump: cell.jump,
                completion: cell.completion,
                no_restart: cell.no_restart,
            });
            segments.extend_from_slice(cell.segments);
        }
        Ok(Program {
            segments: segments.into_boxed_slice(),
            discontinuities: discontinuities.into_boxed_slice(),
            cells: records.into_boxed_slice(),
        })
    }
}

/// Counts were checked against `u32` before any conversion.
fn to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn reserve<T>(count: usize) -> Result<Vec<T>, BuildError> {
    let mut vector = Vec::new();
    vector
        .try_reserve_exact(count)
        .map_err(|_| BuildError::Allocation)?;
    Ok(vector)
}

fn validate(cell: usize, description: &CellDescription<'_>) -> Result<(), BuildError> {
    if description.segments.is_empty() {
        return Err(BuildError::NoSegments { cell });
    }
    if !description.grid.is_multiple_of(64) || description.grid != description.grid_ramp.max(64) {
        return Err(BuildError::Grid { cell });
    }
    let mut previous_end = 0;
    for (segment, value) in description.segments.iter().enumerate() {
        if value.start >= value.end
            || value.end >= TIME_LIMIT
            || !value.start_value.is_finite()
            || !value.end_value.is_finite()
        {
            return Err(BuildError::Segment { cell, segment });
        }
        if value.start < previous_end {
            return Err(BuildError::Order { cell, segment });
        }
        if value.shape == Shape::Exponential && !(value.start_value > 0.0 && value.end_value > 0.0)
        {
            return Err(BuildError::NonPositiveExponential { cell, segment });
        }
        previous_end = value.end;
    }
    let mut previous = None;
    let mut close = None;
    for_each_discontinuity(description.segments, |time| {
        if close.is_none()
            && previous.is_some_and(|before: i64| time - before < i64::from(DISCONTINUITY_SPACING))
        {
            close = Some(time);
        }
        previous = Some(time);
    });
    if let Some(time) = close {
        return Err(BuildError::Spacing { cell, time });
    }
    Ok(())
}

fn count_discontinuities(segments: &[Segment]) -> usize {
    let mut count = 0;
    for_each_discontinuity(segments, |_| count += 1);
    count
}

/// Calls `emit` with each node time at which the table is discontinuous, ascending: where the
/// value just before the time differs, bit for bit, from the value at it.
///
/// A discontinuity is a step the hold rule makes, not a fast ramp: a segment's start when the
/// value held before it is not its `start_value`, and a `step`'s end when `end_value` differs from
/// `start_value`. A `linear` or `exponential` segment is continuous at its end.
fn for_each_discontinuity(segments: &[Segment], mut emit: impl FnMut(i64)) {
    let mut last = None;
    for segment in segments {
        for time in [segment.start, segment.end] {
            let time = i64::try_from(time).unwrap_or(i64::MAX);
            if last == Some(time) {
                continue;
            }
            last = Some(time);
            if value_before(segments, time).to_bits() != value_at(segments, time).to_bits() {
                emit(time);
            }
        }
    }
}

/// The value the curve approaches from the left of `time`.
fn value_before(segments: &[Segment], time: i64) -> f32 {
    let Some(first) = segments.first() else {
        return 0.0;
    };
    let previous = time.saturating_sub(1);
    let Ok(previous) = u64::try_from(previous) else {
        return first.start_value;
    };
    let Some(segment) = segments
        .partition_point(|segment| segment.start <= previous)
        .checked_sub(1)
        .and_then(|index| segments.get(index))
    else {
        return first.start_value;
    };
    if previous >= segment.end || segment.shape != Shape::Step {
        segment.end_value
    } else {
        segment.start_value
    }
}
