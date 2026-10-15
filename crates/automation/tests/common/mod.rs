//! Shared builders for the automation tests.
#![allow(dead_code, unreachable_pub, missing_docs)]

use automation::{
    CellDescription, CellEvents, CellState, Completion, Event, JumpLength, NodeSpan, Program,
    ProgramBuilder, SeekMode, Segment, Shape,
};

pub fn seg(start: u64, end: u64, v0: f32, v1: f32, shape: Shape) -> Segment {
    Segment {
        start,
        end,
        start_value: v0,
        end_value: v1,
        shape,
    }
}

pub fn description(
    segments: &[Segment],
    ramp: u32,
    jump: JumpLength,
    completion: Completion,
    no_restart: bool,
) -> CellDescription<'_> {
    CellDescription {
        segments,
        grid_ramp: ramp,
        grid: ramp.max(64),
        jump,
        completion,
        no_restart,
    }
}

/// A one-cell program: a fader-like cell (ramp 64, `End`, block-keyed jump).
pub fn fader(segments: &[Segment]) -> Program {
    ProgramBuilder::build(&[description(
        segments,
        64,
        JumpLength::Block,
        Completion::End,
        false,
    )])
    .expect("valid cell")
}

pub fn block(
    program: &Program,
    state: &mut CellState,
    span: NodeSpan,
    mode: SeekMode,
    jump: u32,
) -> Vec<Event> {
    CellEvents::new(program.cell(0).expect("cell 0"), state, span, mode, jump).collect()
}

/// Events over `blocks` consecutive blocks of `quantum` samples from node time `start`, each as
/// `(node time, event)`.
pub fn run(
    program: &Program,
    state: &mut CellState,
    start: i64,
    quantum: u32,
    blocks: u32,
    mode: SeekMode,
    jump: u32,
) -> Vec<(i64, Event)> {
    let mut out = Vec::new();
    for index in 0..blocks {
        let first = start + i64::from(index) * i64::from(quantum);
        let span = NodeSpan::new(first, None, quantum).expect("span");
        out.extend(
            block(program, state, span, mode, jump)
                .into_iter()
                .map(|event| (first + i64::from(event.offset), event)),
        );
    }
    out
}
