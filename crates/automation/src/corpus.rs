//! The cross-target determinism corpus of the automation events (gate G5).
//!
//! Each case builds a fixed curve set, runs [`CellEvents`] over a fixed sequence of node-time
//! mappings and returns one `u32` word group per event: the block index, the offset, the kind
//! (with the held flag in bit 8), the ramp and the value bits. Gate G5
//! (`tools/wasm-gates/tests/g5_native_corpus.rs`) digests the words with SHA-256 and compares
//! them with [`G5_DIGESTS`] natively; `tools/wasm-gate-corpus` replays the identical cases on
//! wasm32 against the same pins, and the AArch64 release job runs G5 unfiltered.
//!
//! The case is scalar: curve values are `f64` through `math::pow`, rounded once to `f32`, with no
//! lane type, so a digest does not depend on a lane width. Every table and mapping is built from
//! integer constants, and no value is NaN (the builder refuses a non-positive exponential).
//!
//! The pins were generated once from this scalar code on x86-64 and frozen; a mismatch is never
//! fixed by re-pinning. AArch64 cannot be run on the authoring host: the pins describe scalar
//! `f64` arithmetic, which the three targets must agree on.

use alloc::vec::Vec;

use crate::{
    CellDescription, CellEvents, CellState, Completion, EventKind, JumpLength, NodeSpan, Program,
    ProgramBuilder, SeekMode, Segment, Shape,
};

/// Number of corpus cases.
pub const CASE_COUNT: usize = 6;

/// Human-readable name of each case, indexed by case number.
pub const CASE_NAMES: [&str; CASE_COUNT] = [
    "automation/linear_db_fade",
    "automation/exponential_sweep",
    "automation/steps_gaps_and_a_reported_ramp",
    "automation/exact_seeks",
    "automation/ramped_seeks",
    "automation/no_restart",
];

const fn seg(start: u64, end: u64, v0: f32, v1: f32, shape: Shape) -> Segment {
    Segment {
        start,
        end,
        start_value: v0,
        end_value: v1,
        shape,
    }
}

/// One case's curve table and cell parameters.
pub(crate) struct Case {
    pub(crate) table: Vec<Segment>,
    pub(crate) ramp: u32,
    pub(crate) jump: JumpLength,
    pub(crate) completion: Completion,
    pub(crate) no_restart: bool,
    pub(crate) mode: SeekMode,
}

pub(crate) fn case_of(case: usize) -> Case {
    let (table, ramp, jump, completion, no_restart, mode) = match case {
        // A 60 dB linear fade over one second at 48 kHz, a fader cell.
        0 => (
            alloc::vec![seg(0, 48_000, 0.0, -60.0, Shape::Linear)],
            64,
            JumpLength::Block,
            Completion::End,
            false,
            SeekMode::Exact,
        ),
        // A 20 Hz to 20 kHz exponential sweep over one second, a filter cell.
        1 => (
            alloc::vec![seg(0, 48_000, 20.0, 20_000.0, Shape::Exponential)],
            64,
            JumpLength::Fixed(64),
            Completion::After,
            false,
            SeekMode::Exact,
        ),
        // Steps, gaps, adjacent segments, an equal-valued join and an exponential, with a ramp
        // the caller reports.
        2 => (
            alloc::vec![
                seg(100, 1000, 0.5, 0.75, Shape::Step),
                seg(1000, 1500, 0.75, 0.25, Shape::Linear),
                seg(2000, 2400, 0.25, 0.25, Shape::Step),
                seg(2400, 3000, 2.0, 8.0, Shape::Exponential),
                seg(3180, 3250, 1.0, 0.0, Shape::Step),
                seg(3250, 9000, 0.0, 1.0, Shape::Linear),
            ],
            64,
            JumpLength::Block,
            Completion::End,
            false,
            SeekMode::Exact,
        ),
        3 | 4 => (
            alloc::vec![
                seg(0, 6000, -30.0, 6.0, Shape::Linear),
                seg(6000, 6100, 0.0, 0.0, Shape::Step),
                seg(6500, 20_000, 1.0, 1000.0, Shape::Exponential),
            ],
            64,
            JumpLength::Block,
            Completion::End,
            false,
            if case == 3 {
                SeekMode::Exact
            } else {
                SeekMode::Ramp
            },
        ),
        // The delay time: a grid and ramp of 128, a jump that waits for its ramp.
        _ => (
            alloc::vec![
                seg(0, 100, 10.0, 10.0, Shape::Step),
                seg(100, 190, 20.0, 20.0, Shape::Step),
                seg(190, 300, 30.0, 30.0, Shape::Step),
                seg(300, 5000, 5.0, 500.0, Shape::Exponential),
                seg(5050, 5400, 40.0, 41.0, Shape::Step),
            ],
            128,
            JumpLength::Fixed(128),
            Completion::End,
            true,
            SeekMode::Exact,
        ),
    };
    Case {
        table,
        ramp,
        jump,
        completion,
        no_restart,
        mode,
    }
}

/// One block of a case's fixed mapping.
pub(crate) struct Block {
    pub(crate) span: NodeSpan,
    pub(crate) jump: u32,
    /// A ramp to report to the state before the block, as a completion node time.
    pub(crate) hold: Option<i64>,
}

pub(crate) fn blocks(case: usize) -> Vec<Block> {
    let mut out = Vec::new();
    let mut push = |start: i64, step: Option<(u32, i64)>, frames: u32, jump: u32, hold| {
        if let Ok(span) = NodeSpan::new(start, step, frames) {
            out.push(Block { span, jump, hold });
        }
    };
    match case {
        0 => {
            // Quantum 100 from before the curve, so the grid straddles blocks.
            for index in 0..490_i64 {
                push(index * 100 - 50, None, 100, 480, None);
            }
        }
        1 => {
            for index in 0..400_i64 {
                push(index * 128, None, 128, 64, None);
            }
        }
        2 => {
            for index in 0..80_i64 {
                let hold = (index % 9 == 4).then_some(index * 128 + 300);
                let jump = if index % 2 == 0 { 480 } else { 960 };
                push(index * 128, None, 128, jump, hold);
            }
        }
        3 | 4 => {
            // Seeks at offset 0, mid-block and the last sample, forward and backward, each
            // followed by plain blocks.
            let mut node = 0_i64;
            for round in 0..24_i64 {
                let target = (round * 4_813 + 217) % 21_000;
                let offset = [0, 37, 99, 63][(round % 4) as usize];
                let jump = [480, 960, 240][(round % 3) as usize];
                push(node, Some((offset, target)), 100, jump, None);
                node = target + i64::from(100 - offset);
                for _ in 0..6 {
                    push(node, None, 100, jump, None);
                    node += 100;
                }
            }
        }
        _ => {
            for index in 0..60_i64 {
                push(index * 100, None, 100, 0, None);
            }
        }
    }
    out
}

/// The result words of one case: five per event, then the event count.
///
/// # Panics
///
/// Panics if `case >= CASE_COUNT`, or if the frozen description is refused (a corpus bug).
#[must_use]
pub fn run_case(case: usize) -> Vec<u32> {
    assert!(case < CASE_COUNT, "corpus case index out of range");
    let spec = case_of(case);
    let description = CellDescription {
        segments: &spec.table,
        grid_ramp: spec.ramp,
        grid: spec.ramp.max(64),
        jump: spec.jump,
        completion: spec.completion,
        no_restart: spec.no_restart,
    };
    let program: Program =
        ProgramBuilder::build(&[description]).expect("the frozen corpus description is valid");
    let cell = program.cell(0).expect("one cell");
    let mut state = CellState::new();
    let mut words = Vec::new();
    for (index, block) in blocks(case).into_iter().enumerate() {
        if let Some(completion) = block.hold {
            state.hold_until(completion);
        }
        for event in CellEvents::new(cell, &mut state, block.span, spec.mode, block.jump) {
            let kind = match event.kind {
                EventKind::Grid => 0,
                EventKind::Jump => 1,
                EventKind::Set => 2,
            };
            words.extend_from_slice(&[
                index as u32,
                event.offset,
                kind | (u32::from(event.held) << 8),
                event.ramp,
                event.value.to_bits(),
            ]);
        }
    }
    let count = (words.len() / 5) as u32;
    words.push(count);
    words
}

/// SHA-256 of each case's result words, little-endian, generated once from this scalar code on
/// `x86_64` and frozen. A mismatch is never fixed by re-pinning: it means the corpus changed or a
/// target stopped agreeing on scalar `f64` arithmetic.
pub const G5_DIGESTS: [[u8; 32]; CASE_COUNT] = [
    // automation/linear_db_fade
    [
        0xBD, 0xF6, 0xDF, 0xE1, 0x91, 0x89, 0x30, 0xC6, 0x3A, 0x42, 0xB2, 0xA2, 0xE5, 0x3B, 0xB4,
        0xA9, 0x3B, 0x35, 0x53, 0x52, 0xAF, 0x5E, 0xE9, 0xE3, 0x85, 0x33, 0x84, 0xA9, 0xD6, 0x64,
        0x7B, 0xCB,
    ],
    // automation/exponential_sweep
    [
        0x91, 0x6D, 0x5F, 0x88, 0xB6, 0x70, 0x99, 0x85, 0x30, 0x35, 0xCB, 0xD0, 0x17, 0x72, 0x3B,
        0xAA, 0x39, 0x69, 0xEB, 0x59, 0xFB, 0x30, 0x95, 0xF3, 0x8F, 0xF5, 0x54, 0xD2, 0xDF, 0xC5,
        0xD7, 0xD2,
    ],
    // automation/steps_gaps_and_a_reported_ramp
    [
        0x94, 0x5C, 0x0A, 0xC9, 0xF1, 0x3B, 0x56, 0xED, 0xE8, 0xBA, 0xDF, 0x99, 0xCE, 0xAC, 0x30,
        0x00, 0x71, 0x06, 0x79, 0x60, 0x76, 0x55, 0x25, 0x73, 0xF4, 0xCF, 0x65, 0xF8, 0x4E, 0x4F,
        0xA3, 0x22,
    ],
    // automation/exact_seeks
    [
        0x54, 0xCF, 0xA6, 0xB9, 0x9B, 0x82, 0x02, 0xE8, 0x61, 0x64, 0xB8, 0x9F, 0x98, 0xF6, 0x1F,
        0x42, 0x64, 0x95, 0xBF, 0xFE, 0x27, 0xDC, 0xAD, 0xE8, 0x34, 0x53, 0xE4, 0xDE, 0x2F, 0x37,
        0xA4, 0xDA,
    ],
    // automation/ramped_seeks
    [
        0x55, 0xC7, 0x97, 0x52, 0xE1, 0x5A, 0xC2, 0x7A, 0xB0, 0xEB, 0xBC, 0xCC, 0xA6, 0xCF, 0x9C,
        0x71, 0xF8, 0x36, 0xD5, 0x0D, 0x1C, 0x5B, 0x56, 0x8D, 0x9F, 0x3F, 0x1A, 0x58, 0x0E, 0xA5,
        0xE6, 0x45,
    ],
    // automation/no_restart
    [
        0xA5, 0xB7, 0x56, 0xAB, 0xAB, 0x96, 0xF5, 0x0D, 0x2A, 0x58, 0x9C, 0x9E, 0xB2, 0x36, 0x26,
        0x29, 0x2F, 0xDB, 0x2B, 0x96, 0xC1, 0x68, 0xEC, 0xCE, 0xAB, 0x38, 0x50, 0xCE, 0x4E, 0x68,
        0xB0, 0x8F,
    ],
];
