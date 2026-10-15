//! Event generation: one cell, one block, pulled in offset order.

use crate::{CellProgram, Completion, JumpLength, value_at};

/// A cell with no ramp in flight.
const IDLE: i64 = i64::MIN;
/// A cursor not yet placed in the discontinuity list.
const UNPOSITIONED: u32 = u32::MAX;

/// The kind of one event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventKind {
    /// A retarget on the node-time grid, along the curve.
    Grid,
    /// A retarget at a discontinuity of the table, or the ramped landing of a seek.
    Jump,
    /// An exact value, no ramp: the landing of a seek in [`SeekMode::Exact`].
    Set,
}

/// One event of a block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event {
    /// The sample of the block the event applies at.
    pub offset: u32,
    /// The target value.
    pub value: f32,
    /// The ramp length in samples; `0` for [`EventKind::Set`].
    pub ramp: u32,
    /// What produced the event.
    pub kind: EventKind,
    /// A grid event inside a ramp that completes later: the stage remembers `value` and does not
    /// retarget, and the cell's current target does not change.
    pub held: bool,
}

/// How a seek lands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeekMode {
    /// An exact [`EventKind::Set`] (builtin cells).
    Exact,
    /// A [`EventKind::Jump`] over the jump length (effect cells).
    Ramp,
}

/// Why a [`NodeSpan`] is refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpanError {
    /// The seek step's offset is not inside the block.
    StepOutsideBlock,
}

// REALTIME_POLICY_BEGIN
/// One node block's mapping from offset to node time: the node time at offset 0, at most one step
/// (a seek) and the block length.
///
/// Between the start and the step, and from the step on, node time advances by one per sample.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeSpan {
    start: i64,
    step: Option<(u32, i64)>,
    frames: u32,
}

impl NodeSpan {
    /// A span of `frames` samples whose offset 0 is node time `start`, with an optional seek step
    /// `(offset, node_time)`. A step at offset 0 replaces `start`.
    ///
    /// # Errors
    ///
    /// [`SpanError::StepOutsideBlock`] when the step offset is not below `frames`.
    pub const fn new(start: i64, step: Option<(u32, i64)>, frames: u32) -> Result<Self, SpanError> {
        if let Some((offset, _)) = step
            && offset >= frames
        {
            return Err(SpanError::StepOutsideBlock);
        }
        Ok(Self {
            start,
            step,
            frames,
        })
    }
}

/// The ramp in flight, the current target and the cursor of one cell, kept across blocks.
///
/// Its size is a fixed 32 bytes. A fresh state has no target, so the first retarget always
/// emits.
#[derive(Clone, Copy, Debug)]
pub struct CellState {
    /// Index of the next unconsumed discontinuity; [`UNPOSITIONED`] before the first block.
    cursor: u32,
    /// The bits of the current target, valid when `has_target`.
    target: u32,
    has_target: bool,
    /// A held event is outstanding, so the next retarget emits even when its value is unchanged.
    held: bool,
    /// The node time at which the ramp in flight completes, or [`IDLE`].
    ramp_done: i64,
    /// The node time at which a deferred jump fires (a `no_restart` cell), or [`IDLE`].
    deferred: i64,
}

const _: () = assert!(size_of::<CellState>() <= 48);

impl Default for CellState {
    fn default() -> Self {
        Self::new()
    }
}

impl CellState {
    /// A state with no target, no ramp in flight and an unplaced cursor.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cursor: UNPOSITIONED,
            target: 0,
            has_target: false,
            held: false,
            ramp_done: IDLE,
            deferred: IDLE,
        }
    }

    /// Places the cursor for node time `t` (an adoption): the next block's discontinuities start
    /// at `t`. The in-flight ramp and a deferred jump belong to the old position and are dropped;
    /// the current target is kept.
    pub fn reposition(&mut self, program: &CellProgram<'_>, t: i64) {
        self.cursor = to_cursor(program.discontinuities.partition_point(|&time| time < t));
        self.ramp_done = IDLE;
        self.deferred = IDLE;
        self.held = false;
    }

    /// Reports a ramp the caller started on this lane (a mute ramp, a live record's ramp) that
    /// completes at node time `completion`, so the grid holds inside it.
    ///
    /// On a `no_restart` cell the caller must not restart a busy lane, so `completion` is never
    /// earlier than the ramp in flight: a deferred jump keeps the absolute time it was deferred
    /// to, and a shorter report would land it late (at the next grid point instead).
    pub const fn hold_until(&mut self, completion: i64) {
        self.ramp_done = completion;
    }

    /// The bits of the current target, if the cell has one.
    #[must_use]
    pub const fn target(&self) -> Option<f32> {
        if self.has_target {
            Some(f32::from_bits(self.target))
        } else {
            None
        }
    }
}

fn to_cursor(index: usize) -> u32 {
    u32::try_from(index).unwrap_or(UNPOSITIONED - 1)
}

/// The node time at which a ramp of `n` samples started at `start` completes.
///
/// `start + n - 1` for [`Completion::End`] and `start + n` for [`Completion::After`]; both are
/// `start` for `n = 0`.
#[must_use]
pub const fn completion_of(completion: Completion, start: i64, n: u32) -> i64 {
    if n == 0 {
        return start;
    }
    let after = start.saturating_add(n as i64);
    match completion {
        Completion::End => after - 1,
        Completion::After => after,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    /// The samples before the seek step, or the whole block.
    Before,
    /// The landing of the seek step.
    Seek,
    /// The samples after the seek step.
    After,
    Done,
}

/// The events of one block of one cell, in increasing offset, at most one per offset
/// (`Set > Jump > Grid`). Consume it to the end: the state advances as events come out.
#[derive(Debug)]
pub struct CellEvents<'a, 's> {
    program: CellProgram<'a>,
    state: &'s mut CellState,
    mode: SeekMode,
    jump: u32,
    span: NodeSpan,
    stage: Stage,
    /// First offset of the current run, its end, the node time at its first offset, and the next
    /// offset to examine.
    run_start: u32,
    run_end: u32,
    run_time: i64,
    position: u32,
}

impl<'a, 's> CellEvents<'a, 's> {
    /// The events of the block `span` of `program`.
    ///
    /// `jump` is the block's jump length for a [`JumpLength::Block`] cell (the caller resolves the
    /// cell's row against the session's ramp lengths); a [`JumpLength::Fixed`] cell ignores it.
    #[must_use]
    pub fn new(
        program: CellProgram<'a>,
        state: &'s mut CellState,
        span: NodeSpan,
        mode: SeekMode,
        jump: u32,
    ) -> Self {
        let before_end = span.step.map_or(span.frames, |(offset, _)| offset);
        if state.cursor == UNPOSITIONED && before_end > 0 {
            // Only the cursor: a ramp the caller reported before the first block stays reported.
            state.cursor = to_cursor(program.discontinuities.partition_point(|&t| t < span.start));
        }
        let stage = if before_end > 0 {
            Stage::Before
        } else if span.step.is_some() {
            Stage::Seek
        } else {
            Stage::Done
        };
        Self {
            program,
            state,
            mode,
            jump,
            span,
            stage,
            run_start: 0,
            run_end: before_end,
            run_time: span.start,
            position: 0,
        }
    }

    fn jump_length(&self) -> u32 {
        match self.program.jump {
            JumpLength::Block => self.jump,
            JumpLength::Fixed(length) => length,
        }
    }

    fn completion(&self, start: i64, n: u32) -> i64 {
        completion_of(self.program.completion, start, n)
    }

    /// The first node time at which the lane's ramp in flight no longer runs.
    fn free_at(&self) -> i64 {
        if self.state.ramp_done == IDLE {
            return IDLE;
        }
        match self.program.completion {
            Completion::End => self.state.ramp_done.saturating_add(1),
            Completion::After => self.state.ramp_done,
        }
    }

    /// The value a ramp of `n` samples started at `start` aims for: the curve at its completion,
    /// but never past the next discontinuity, which a jump of its own retargets at its exact time.
    fn target_at(&self, start: i64, n: u32) -> f32 {
        let mut at = self.completion(start, n);
        let ahead = self
            .program
            .discontinuities
            .get(self.state.cursor as usize..)
            .unwrap_or(&[]);
        if let Some(&next) = ahead.get(ahead.partition_point(|&time| time <= start))
            && next <= at
        {
            at = next - 1;
        }
        value_at(self.program.segments, at)
    }

    /// `true` when a retarget to `value` changes nothing the stage holds.
    fn unchanged(&self, value: f32) -> bool {
        self.state.has_target && self.state.target == value.to_bits() && !self.state.held
    }

    fn commit(&mut self, value: f32, start: i64, n: u32) {
        self.state.target = value.to_bits();
        self.state.has_target = true;
        self.state.held = false;
        self.state.ramp_done = if n == 0 {
            IDLE
        } else {
            self.completion(start, n)
        };
    }

    /// The landing of the seek step at `offset`, node time `time`.
    fn land(&mut self, offset: u32, time: i64) -> Event {
        let cursor = self.program.discontinuities.partition_point(|&d| d < time);
        let on_time = self.program.discontinuities.get(cursor) == Some(&time);
        self.state.cursor = to_cursor(cursor + usize::from(on_time));
        self.state.deferred = IDLE;
        self.state.held = false;
        match self.mode {
            SeekMode::Exact => {
                let value = value_at(self.program.segments, time);
                self.commit(value, time, 0);
                Event {
                    offset,
                    value,
                    ramp: 0,
                    kind: EventKind::Set,
                    held: false,
                }
            }
            SeekMode::Ramp => {
                let ramp = self.jump_length();
                let value = self.target_at(time, ramp);
                self.commit(value, time, ramp);
                Event {
                    offset,
                    value,
                    ramp,
                    kind: EventKind::Jump,
                    held: false,
                }
            }
        }
    }

    fn jump_event(&mut self, offset: u32, time: i64) -> Option<Event> {
        let ramp = self.jump_length();
        let value = self.target_at(time, ramp);
        // A ramp in flight that completes after this jump would is still short of its target at
        // the discontinuity: its target can equal the jump's (a grid ramp aimed at `d - 1` where
        // `value(d - 1) == value(d)` but the left limit differs), and change only must not drop
        // the jump then.
        let lagging =
            self.state.ramp_done != IDLE && self.state.ramp_done > self.completion(time, ramp);
        if self.unchanged(value) && !lagging {
            return None;
        }
        self.commit(value, time, ramp);
        Some(Event {
            offset,
            value,
            ramp,
            kind: EventKind::Jump,
            held: false,
        })
    }

    fn grid_event(&mut self, offset: u32, time: i64) -> Option<Event> {
        let ramp = self.program.ramp;
        let value = self.target_at(time, ramp);
        let done = self.completion(time, ramp);
        let in_flight = time < self.free_at();
        if in_flight && (self.program.no_restart || done < self.state.ramp_done) {
            let differs = !self.state.has_target || self.state.target != value.to_bits();
            if !differs && !self.state.held {
                return None;
            }
            self.state.held = differs;
            return Some(Event {
                offset,
                value,
                ramp,
                kind: EventKind::Grid,
                held: true,
            });
        }
        if self.unchanged(value) {
            return None;
        }
        self.commit(value, time, ramp);
        Some(Event {
            offset,
            value,
            ramp,
            kind: EventKind::Grid,
            held: false,
        })
    }

    /// Steps the cursor over the discontinuities before `here`, which the caller passed by moving
    /// node time forward with no seek step. Their jumps are dropped: the next grid event retargets
    /// along the curve. A binary search, so a long skip costs `O(log n)`.
    fn skip_behind(&mut self, here: i64) {
        let cursor = self.state.cursor as usize;
        if let Some(ahead) = self.program.discontinuities.get(cursor..)
            && ahead.first().is_some_and(|&time| time < here)
        {
            self.state.cursor = to_cursor(cursor + ahead.partition_point(|&time| time < here));
        }
    }

    /// The next event of the current run, or `None` at its end.
    fn next_in_run(&mut self) -> Option<Event> {
        loop {
            if self.position >= self.run_end {
                return None;
            }
            let here = self.run_time + i64::from(self.position - self.run_start);
            self.skip_behind(here);
            let period = i64::from(self.program.grid);
            let grid = match here.rem_euclid(period) {
                0 => here,
                rest => here.saturating_add(period - rest),
            };
            let boundary = self
                .program
                .discontinuities
                .get(self.state.cursor as usize)
                .copied();
            // A deferral behind the run (the caller moved node time forward with no seek step) is
            // still owed: it is due at the run's first sample. Inside a run a deferral is always
            // ahead, since it is set to a free time past the sample that deferred it.
            let deferred = (self.state.deferred != IDLE).then(|| self.state.deferred.max(here));
            let jump = boundary.into_iter().chain(deferred).min();
            let next = jump.map_or(grid, |jump| jump.min(grid));
            let remaining = i64::from(self.run_end - self.position);
            // Every candidate is at or after `here`, so `distance` is not negative.
            let distance = next.saturating_sub(here);
            if distance >= remaining {
                return None;
            }
            let Ok(distance) = u32::try_from(distance) else {
                return None;
            };
            let offset = self.position + distance;
            let is_boundary = boundary == Some(next);
            let is_deferred = deferred == Some(next);
            if (is_boundary || is_deferred) && self.program.no_restart && next < self.free_at() {
                // The ramp cannot restart: the jump waits for the lane to be free. A deferral due
                // now waits again when the caller reported a ramp after it was deferred
                // (`hold_until` between blocks), including one due at a run's first sample.
                if is_boundary {
                    self.state.cursor += 1;
                }
                self.state.deferred = self.free_at();
                continue;
            }
            let is_jump = is_boundary || is_deferred;
            if is_boundary {
                self.state.cursor += 1;
            }
            if is_deferred {
                self.state.deferred = IDLE;
            }
            self.position = offset + 1;
            let event = if is_jump {
                self.jump_event(offset, next)
            } else {
                self.grid_event(offset, next)
            };
            if event.is_some() {
                return event;
            }
        }
    }
}

impl Iterator for CellEvents<'_, '_> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        loop {
            match self.stage {
                Stage::Done => return None,
                Stage::Seek => {
                    let (offset, time) = self.span.step?;
                    self.stage = Stage::After;
                    self.run_start = offset;
                    self.run_end = self.span.frames;
                    self.run_time = time;
                    self.position = offset + 1;
                    return Some(self.land(offset, time));
                }
                Stage::Before | Stage::After => {
                    if let Some(event) = self.next_in_run() {
                        return Some(event);
                    }
                    self.stage = if self.stage == Stage::Before && self.span.step.is_some() {
                        Stage::Seek
                    } else {
                        Stage::Done
                    };
                }
            }
        }
    }
}
// REALTIME_POLICY_END
