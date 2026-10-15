//! Stored automation, evaluated and turned into per-cell events in node time (#1637).
//!
//! A stored curve is a table of [`Segment`]s. This crate does two things with one: it evaluates
//! the curve at a node time ([`value_at`]), and it turns a cell's table into the retarget events a
//! stage applies, block by block ([`CellEvents`]). It reads no session, descriptor or plan, and no
//! stage uses it yet.
//!
//! # The law
//!
//! The curve holds the first segment's `start_value` before the first segment, a segment's
//! `end_value` after it and in a gap, and a `step` holds its `start_value` on `[start, end)`.
//! Inside a `linear` or `exponential` segment, `x = (t - t0) / (t1 - t0)` and the value is
//! `v0 + (v1 - v0)·x` or `v0·(v1/v0)^x` (the Web Audio ramp formulas). Every value is computed in
//! scalar `f64` through [`math::pow`] and rounded once to `f32`, so the same table and the same
//! node-time mapping give the same events, bit for bit, on x86-64, AArch64 and wasm, at any
//! quantum, wherever a seek lands. There is no lane math and no platform math here: events are
//! sparse control arithmetic, and vector lane math diverges on AArch64 release builds.
//!
//! # Events
//!
//! A cell has a fixed grid in node time (a multiple of `G` samples) and three kinds of event:
//! [`EventKind::Grid`] retargets along the curve, [`EventKind::Jump`] retargets at a
//! discontinuity of the table, and [`EventKind::Set`] sets the exact value at a seek. A block's
//! events come out of an iterator, in increasing offset, with no buffer. Because the grid is in
//! node time, not render time, the events do not depend on the quantum or on the render sample a
//! seek landed at.
//!
//! A block of `q` samples yields at most [`CellProgram::max_events`] events: `2⌈q/64⌉ + 3`, and
//! one more for a `no_restart` cell, whose jump deferred from an earlier block can fire in it. The
//! bound rests on the session's spacing rule (discontinuities at least 64 samples apart, #1626),
//! which [`ProgramBuilder`] enforces again ([`BuildError::Spacing`]).
//!
//! # Realtime
//!
//! Evaluation and event generation are render-reachable: they allocate, lock and block on
//! nothing. Only [`ProgramBuilder`] allocates; it runs on the control thread and reports its exact
//! bytes first.

#![no_std]

extern crate alloc;

mod evaluate;
mod events;
mod program;

pub mod corpus;

pub use evaluate::value_at;
pub use events::{
    CellEvents, CellState, Event, EventKind, NodeSpan, SeekMode, SpanError, completion_of,
};
pub use program::{
    BuildError, CellDescription, CellProgram, DISCONTINUITY_SPACING, Program, ProgramBuilder,
};

/// The shape of one segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Shape {
    /// Holds `start_value` on `[start, end)`, then `end_value`.
    Step = 0,
    /// `v0 + (v1 - v0)·x`.
    Linear = 1,
    /// `v0·(v1/v0)^x`; both values are positive.
    Exponential = 2,
}

/// One stored segment, the same fields as a session `AutomationSegment` in a neutral type.
///
/// Times are node-time samples; the hold rule (see the crate documentation) fills the gaps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    /// First sample of the segment.
    pub start: u64,
    /// First sample after the segment.
    pub end: u64,
    /// The value at `start`.
    pub start_value: f32,
    /// The value at `end`, and after it.
    pub end_value: f32,
    /// How the value moves from `start_value` to `end_value`.
    pub shape: Shape,
}

const _: () = assert!(size_of::<Segment>() == 32);

/// Where a ramp of `n` samples started at node time `τ` completes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Completion {
    /// At `τ + n - 1`: the fader, matrix, trim and effect ramps assign the target on their last
    /// frame.
    End,
    /// At `τ + n`: the EQ and input-filter coefficient ramps reach the target one sample later.
    After,
}

/// Where a cell's jump ramp length comes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JumpLength {
    /// The length the caller passes for the block: `controls::LiveRamps::for_row` of the cell's
    /// live-ramp row (`effect_contract::control::LiveRampRow`). The row's rule lives in
    /// `controls` alone, so this crate neither names a row nor writes its rule a second time, and
    /// it stays a leaf that depends on `math` only. The program stores no copy of the length, so a
    /// `control_smoothing` edit reaches the next jump with no rebuild.
    Block,
    /// A length fixed by the target: 64 for a filter, the smoothing of an effect parameter.
    Fixed(u32),
}
