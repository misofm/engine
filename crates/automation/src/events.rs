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

    /// The block length.
    pub(crate) const fn frames(&self) -> u32 {
        self.frames
    }

    /// The seek step's offset, if the block has one.
    pub(crate) const fn step_offset(&self) -> Option<u32> {
        match self.step {
            Some((offset, _)) => Some(offset),
            None => None,
        }
    }

    /// The node time at `offset` (the step's time from the step on); `None` past the block or the
    /// `i64` range.
    pub(crate) fn time_at(&self, offset: u32) -> Option<i64> {
        if offset >= self.frames {
            return None;
        }
        match self.step {
            Some((step, time)) if offset >= step => time.checked_add(i64::from(offset - step)),
            _ => self.start.checked_add(i64::from(offset)),
        }
    }

    /// The offsets `[from, to)` of this block as a span of their own, whose offset 0 is `from`:
    /// the same node times, and the seek step when it lies inside the window. `None` for an empty
    /// or out-of-block window, or past the `i64` range. A cell's events over the window are the
    /// block's events at those offsets, shifted by `from`, for a state advanced to `from`: the
    /// event generator keeps nothing across offsets but its [`CellState`].
    pub(crate) fn window(&self, from: u32, to: u32) -> Option<Self> {
        if from >= to || to > self.frames {
            return None;
        }
        let frames = to - from;
        let (start, step) = match self.step {
            Some((step, time)) if from > step => (time.checked_add(i64::from(from - step))?, None),
            Some((step, time)) if step < to => (
                self.start.checked_add(i64::from(from))?,
                Some((step - from, time)),
            ),
            _ => (self.start.checked_add(i64::from(from))?, None),
        };
        Some(Self {
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
    /// The node time at which a deferred jump fires, or [`IDLE`]: a `no_restart` cell's jump that
    /// waits for its lane, or a jump the caller deferred ([`Self::defer_jump`]).
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
    /// at `t`. The in-flight ramp and a deferred jump belong to the old position and are dropped,
    /// and an outstanding held event is cleared, so the next retarget to an unchanged value emits
    /// nothing; the current target is kept.
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

    /// Shortens the hold of the ramp in flight so that it completes no later than node time `t`:
    /// the caller's stage stopped holding the lane earlier than the hold it reported (a ramp that
    /// a later operation ended). Shorten-only: a hold that already completes by `t`, or no ramp in
    /// flight, is left as it is: it never lengthens a hold or starts one, as
    /// [`Self::hold_until`] would.
    pub const fn release_hold_by(&mut self, t: i64) {
        if self.ramp_done != IDLE && self.ramp_done > t {
            self.ramp_done = t;
        }
    }

    /// Defers the cell's jump to node time `at` (#1711 D1): at the first sample at or after `at`,
    /// the cell compares its curve with its current target, as at a discontinuity (a
    /// [`EventKind::Jump`] over the jump length to the curve at the jump's completion, change
    /// only, or always when a held event is outstanding). It replaces a deferral already pending.
    ///
    /// The caller defers the jump of a lane whose table changed while a ramp of its own runs (a
    /// live move adopted at a plan swap) to that ramp's first free sample, so the ramp keeps its
    /// own length. Until the deferral fires, every grid event of a restartable cell is held (the
    /// stage remembers it and does not retarget), so no grid ramp cuts the running ramp short; a
    /// `no_restart` cell holds them inside the ramp in flight as it always does. Cleared by
    /// [`Self::clear_deferred_jump`], [`Self::reposition`], a seek landing and, on a restartable
    /// cell, a discontinuity jump that emits (it replaces the ramp the deferral waits for; one
    /// that emits nothing replaces no ramp and keeps the deferral). Control-free and
    /// allocation-free.
    pub const fn defer_jump(&mut self, at: i64) {
        self.deferred = at;
    }

    /// Drops a pending deferred jump ([`Self::defer_jump`]), and with it the grid hold it keeps;
    /// with none pending it does nothing. The caller clears it where the ramp the deferral waits
    /// for is replaced (a newer live move keeps its own length) or ends early (a release of the
    /// lane's hold), so the cell's grid moves the lane again from its next grid point.
    pub const fn clear_deferred_jump(&mut self) {
        self.deferred = IDLE;
    }

    /// The node time of the pending deferred jump, if any ([`Self::defer_jump`], or a
    /// `no_restart` cell's jump that waits for its lane).
    #[must_use]
    pub const fn deferred_jump(&self) -> Option<i64> {
        if self.deferred == IDLE {
            None
        } else {
            Some(self.deferred)
        }
    }

    /// Sets the current target to `value`, as if a retarget to it had completed: the next grid or
    /// jump event to the same bits emits nothing (change only). It clears an outstanding held
    /// event and keeps the cursor, the ramp in flight and a deferred jump.
    ///
    /// A stage that starts a lane at a value it already holds seeds the cell with it: preparation
    /// with the lane's prepared curve value (#1646 D2), a plan swap with the carried stage target
    /// (#1648 D2). Control side; it allocates nothing.
    pub const fn seed_target(&mut self, value: f32) {
        self.target = value.to_bits();
        self.has_target = true;
        self.held = false;
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

    /// `true` when a retarget whose target has the same bits as the current one (`same`, which
    /// is `false` with no target) changes nothing the stage holds: no held event is outstanding.
    /// The change-only rule of every grid and jump event, shared with the merged walk
    /// ([`crate::compose`]).
    pub(crate) const fn unchanged_with(&self, same: bool) -> bool {
        same && !self.held
    }

    /// `true` when the ramp in flight completes after a ramp of `ramp` samples started at `time`
    /// would ([`completion_of`] under `completion`): it is still short of its target there, so
    /// change only must not drop a jump.
    pub(crate) const fn lagging(&self, completion: Completion, time: i64, ramp: u32) -> bool {
        self.ramp_done != IDLE && self.ramp_done > completion_of(completion, time, ramp)
    }

    /// The held-event rule: a held event whose value `differs` from the current target (or with
    /// no target) is outstanding; one that does not is emitted only to clear an outstanding one.
    /// Returns whether the held event is emitted.
    pub(crate) const fn hold_event(&mut self, differs: bool) -> bool {
        if !differs && !self.held {
            return false;
        }
        self.held = differs;
        true
    }

    /// A retarget of `n` samples from `start` committed: no held event is outstanding and the
    /// ramp in flight completes at [`completion_of`] (none for `n = 0`).
    pub(crate) const fn commit_ramp(&mut self, completion: Completion, start: i64, n: u32) {
        self.held = false;
        self.ramp_done = if n == 0 {
            IDLE
        } else {
            completion_of(completion, start, n)
        };
    }

    /// A seek landing's reset: a deferred jump belongs to the old position and is dropped, and no
    /// held event stays outstanding.
    pub(crate) const fn clear_for_landing(&mut self) {
        self.deferred = IDLE;
        self.held = false;
    }

    /// The first node time at which the ramp in flight no longer runs under `completion`, if a
    /// ramp is in flight.
    pub(crate) const fn free_after(&self, completion: Completion) -> Option<i64> {
        if self.ramp_done == IDLE {
            return None;
        }
        Some(match completion {
            Completion::End => self.ramp_done.saturating_add(1),
            Completion::After => self.ramp_done,
        })
    }
}

/// The value a ramp of `n` samples started at `start` aims for on `program`, with `state`'s
/// cursor: the curve at its completion, but never past the next discontinuity, which a jump of its
/// own retargets at its exact time. The one rule of every grid, jump and landing target, and of
/// the merged walk's record target ([`crate::compose`]).
pub(crate) fn target_toward(
    program: &CellProgram<'_>,
    state: &CellState,
    start: i64,
    n: u32,
) -> f32 {
    let mut at = completion_of(program.completion, start, n);
    let ahead = program
        .discontinuities
        .get(state.cursor as usize..)
        .unwrap_or(&[]);
    if let Some(&next) = ahead.get(ahead.partition_point(|&time| time <= start))
        && next <= at
    {
        at = next - 1;
    }
    value_at(program.segments, at)
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

    /// Ends the block's events early and returns the cell's state, so the caller can report a ramp
    /// with [`CellState::hold_until`] and continue the block with a second span (#1647 D1.6: a
    /// span cut at a held ramp's end). Consume the events up to the cut first: the state has
    /// advanced only through the events already pulled.
    #[must_use]
    pub fn into_state(self) -> &'s mut CellState {
        self.state
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
        self.state
            .free_after(self.program.completion)
            .unwrap_or(IDLE)
    }

    /// The value a ramp of `n` samples started at `start` aims for: the curve at its completion,
    /// but never past the next discontinuity, which a jump of its own retargets at its exact time.
    fn target_at(&self, start: i64, n: u32) -> f32 {
        target_toward(&self.program, self.state, start, n)
    }

    /// `true` when a retarget to `value` changes nothing the stage holds.
    fn unchanged(&self, value: f32) -> bool {
        self.state
            .unchanged_with(self.state.has_target && self.state.target == value.to_bits())
    }

    fn commit(&mut self, value: f32, start: i64, n: u32) {
        self.state.target = value.to_bits();
        self.state.has_target = true;
        self.state.commit_ramp(self.program.completion, start, n);
    }

    /// The landing of the seek step at `offset`, node time `time`.
    fn land(&mut self, offset: u32, time: i64) -> Event {
        let cursor = self.program.discontinuities.partition_point(|&d| d < time);
        let on_time = self.program.discontinuities.get(cursor) == Some(&time);
        self.state.cursor = to_cursor(cursor + usize::from(on_time));
        self.state.clear_for_landing();
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
        let lagging = self.state.lagging(self.program.completion, time, ramp);
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
        // A restartable cell holds its grid until a pending deferral fires (#1711 D1): a grid
        // ramp that completes after the ramp in flight would otherwise restart it with the new
        // curve and cut it short. A `no_restart` cell's deferral always lies at its lane's free
        // time, so its rule is unchanged.
        let deferring =
            !self.program.no_restart && self.state.deferred != IDLE && time < self.state.deferred;
        if (in_flight && (self.program.no_restart || done < self.state.ramp_done)) || deferring {
            let differs = !self.state.has_target || self.state.target != value.to_bits();
            if !self.state.hold_event(differs) {
                return None;
            }
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
            // A discontinuity jump on a restartable cell that emits drops a pending deferral
            // (#1711 U1 fold 1): it replaces the ramp the deferral waits for, so the deferral and
            // its grid hold are stale (they would freeze the lane on the jump's target until the
            // deferral's time). A jump that emits nothing (#1711 U1 fold 2: the curve already has
            // its value and no ramp lags) replaces no ramp, and the deferral stays owed.
            if is_boundary && !self.program.no_restart && event.is_some() {
                self.state.deferred = IDLE;
            }
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

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::{
        CellDescription, CellEvents, CellState, Completion, EventKind, JumpLength, NodeSpan,
        ProgramBuilder, SeekMode, Segment, Shape,
    };

    fn flat(value: f32) -> crate::Program {
        let segments = [Segment {
            start: 0,
            end: 4_096,
            start_value: value,
            end_value: value,
            shape: Shape::Linear,
        }];
        ProgramBuilder::build(&[CellDescription {
            segments: &segments,
            grid_ramp: 64,
            grid: 64,
            jump: JumpLength::Block,
            completion: Completion::End,
            no_restart: false,
        }])
        .expect("valid cell")
    }

    /// A restartable cell riding linearly from 0 to -12 over 4,096 samples (a grid event at
    /// every 64th sample moves it).
    fn riding() -> crate::Program {
        let segments = [Segment {
            start: 0,
            end: 4_096,
            start_value: 0.0,
            end_value: -12.0,
            shape: Shape::Linear,
        }];
        ProgramBuilder::build(&[CellDescription {
            segments: &segments,
            grid_ramp: 64,
            grid: 64,
            jump: JumpLength::Block,
            completion: Completion::End,
            no_restart: false,
        }])
        .expect("valid cell")
    }

    /// The `(offset, kind, held)` of every event of the block of 256 samples from node time 0.
    fn block_events(
        program: &crate::Program,
        state: &mut CellState,
    ) -> Vec<(u32, EventKind, bool)> {
        let span = NodeSpan::new(0, None, 256).expect("span");
        CellEvents::new(
            program.cell(0).expect("cell"),
            state,
            span,
            SeekMode::Exact,
            480,
        )
        .map(|event| (event.offset, event.kind, event.held))
        .collect()
    }

    /// A cell seeded at its curve's start whose caller reported a ramp completing at node time
    /// 200 (a live move), so its first free sample is 201.
    fn in_a_move() -> CellState {
        let mut state = CellState::new();
        state.seed_target(0.0);
        state.hold_until(200);
        state
    }

    /// #1711 D1: a jump deferred to the first free sample of a ramp in flight fires there over the
    /// jump length, and every grid event before it is held, also the one whose grid ramp would
    /// complete after the ramp in flight (which restarts the ramp with no deferral).
    ///
    /// Red if the deferral is not fired on a restartable cell, or if the grid event at 192 (its
    /// ramp completes at 255, after 200) is not held while the deferral is pending.
    #[test]
    fn a_deferred_jump_fires_at_its_time_and_holds_the_grid_before_it() {
        let program = riding();
        let mut plain = in_a_move();
        let held = |offset| (offset, EventKind::Grid, true);
        assert_eq!(
            block_events(&program, &mut plain),
            [held(0), held(64), held(128), (192, EventKind::Grid, false)],
            "no deferral: the grid restarts the ramp at 192"
        );
        let mut deferred = in_a_move();
        deferred.defer_jump(201);
        assert_eq!(deferred.deferred_jump(), Some(201));
        assert_eq!(
            block_events(&program, &mut deferred),
            [
                held(0),
                held(64),
                held(128),
                held(192),
                (201, EventKind::Jump, false)
            ]
        );
        assert_eq!(deferred.deferred_jump(), None, "a fired deferral is spent");
    }

    /// #1711 D1: a cleared deferral fires nothing and holds no grid event: the cell's events are
    /// those of a cell that never deferred; a deferral whose target the curve already has emits
    /// nothing (change only), and `reposition` drops it.
    ///
    /// Red if `clear_deferred_jump` leaves the deferral or its grid hold in place (a held grid
    /// would freeze the lane after an early release, #1651 U1 MAJOR-1).
    #[test]
    fn a_cleared_deferral_fires_nothing_and_holds_no_grid() {
        let program = riding();
        let mut cleared = in_a_move();
        cleared.defer_jump(201);
        cleared.clear_deferred_jump();
        assert_eq!(cleared.deferred_jump(), None);
        assert_eq!(
            block_events(&program, &mut cleared),
            block_events(&program, &mut in_a_move())
        );

        let flat = flat(-7.25);
        let mut settled = CellState::new();
        settled.seed_target(-7.25);
        settled.defer_jump(100);
        assert_eq!(first_block(&flat, &mut settled), [], "change only");

        let mut repositioned = in_a_move();
        repositioned.defer_jump(201);
        repositioned.reposition(&program.cell(0).expect("cell"), 0);
        assert_eq!(repositioned.deferred_jump(), None);
    }

    /// #1711 U1 fold 2: a discontinuity jump on a restartable cell that emits nothing keeps a
    /// pending deferral. The curve is flat at -6 to 50, then linear from -4 to -8 over 50..562,
    /// with a fixed jump length of 257 samples: the jump at 50 aims at the curve at 306 (-6, the
    /// seeded target), so it emits nothing and replaces no ramp. The deferral to 201 stays owed:
    /// the grid is held until 201, where the deferred jump fires.
    ///
    /// Red if a discontinuity whose jump emits nothing drops the deferral (the fold-1 rule): the
    /// grid at 64, 128 and 192 then retargets unheld and nothing fires at 201.
    #[test]
    fn a_discontinuity_whose_jump_emits_nothing_keeps_a_pending_deferral() {
        let segments = [
            Segment {
                start: 0,
                end: 50,
                start_value: -6.0,
                end_value: -6.0,
                shape: Shape::Linear,
            },
            Segment {
                start: 50,
                end: 562,
                start_value: -4.0,
                end_value: -8.0,
                shape: Shape::Linear,
            },
            Segment {
                start: 562,
                end: 4_096,
                start_value: -8.0,
                end_value: -8.0,
                shape: Shape::Linear,
            },
        ];
        let program = ProgramBuilder::build(&[CellDescription {
            segments: &segments,
            grid_ramp: 64,
            grid: 64,
            jump: JumpLength::Fixed(257),
            completion: Completion::End,
            no_restart: false,
        }])
        .expect("valid cell");
        assert_eq!(program.cell(0).expect("cell").discontinuities(), [50]);
        let mut state = CellState::new();
        state.seed_target(-6.0);
        state.defer_jump(201);
        let held = |offset| (offset, EventKind::Grid, true);
        assert_eq!(
            block_events(&program, &mut state),
            [
                held(64),
                held(128),
                held(192),
                (201, EventKind::Jump, false)
            ]
        );
        assert_eq!(state.deferred_jump(), None, "a fired deferral is spent");
    }

    fn first_block(program: &crate::Program, state: &mut CellState) -> Vec<(u32, EventKind)> {
        let span = NodeSpan::new(0, None, 256).expect("span");
        CellEvents::new(
            program.cell(0).expect("cell"),
            state,
            span,
            SeekMode::Exact,
            480,
        )
        .map(|event| (event.offset, event.kind))
        .collect()
    }
}
