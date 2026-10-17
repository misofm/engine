//! The merged walk's gates (#1654 K1).

use alloc::vec::Vec;

use super::{Composed, ComposedEvent, CursorKind, Heard};
use crate::corpus::{CASE_COUNT, blocks, case_of};
use crate::{
    CellDescription, CellEvents, CellState, Completion, Event, EventKind, JumpLength, NodeSpan,
    Program, ProgramBuilder, SeekMode, Segment, Shape,
};

const fn seg(start: u64, end: u64, v0: f32, v1: f32, shape: Shape) -> Segment {
    Segment {
        start,
        end,
        start_value: v0,
        end_value: v1,
        shape,
    }
}

/// A fader-rule cell (grid and ramp 64, `End`, block jump length, restartable).
const fn fader_cell(segments: &[Segment]) -> CellDescription<'_> {
    CellDescription {
        segments,
        grid_ramp: 64,
        grid: 64,
        jump: JumpLength::Block,
        completion: Completion::End,
        no_restart: false,
    }
}

/// `(offset, value bits, ramp, kind, held)`: what a differential compares.
type Word = (u32, u32, u32, EventKind, bool);

const fn word(event: &Event) -> Word {
    (
        event.offset,
        event.value.to_bits(),
        event.ramp,
        event.kind,
        event.held,
    )
}

const fn composed_word(event: &ComposedEvent<f32>) -> Word {
    (
        event.offset,
        event.value.to_bits(),
        event.ramp,
        event.kind,
        event.held,
    )
}

fn identity(heard: Heard<'_>) -> f32 {
    heard.value(0)
}

/// A seeded reference state, as [`Composed::new`] seeds its cursors.
fn seeded(program: &Program, cell: usize) -> CellState {
    let mut state = CellState::new();
    if let Some(first) = program
        .cell(cell)
        .and_then(|cell| cell.segments().first().copied())
    {
        state.seed_target(first.start_value);
    }
    state
}

/// One block's caller operations before its events, applied alike to the reference cell and the
/// one-cursor output: a ramp the caller reports, a jump deferred.
#[derive(Clone, Copy, Default)]
struct Ops {
    hold: Option<i64>,
    defer: Option<i64>,
}

/// The reference cell's events and the one-cursor output's over the same blocks, per block.
fn one_cursor_streams(
    program: &Program,
    blocks: &[(NodeSpan, u32, Ops)],
    mode: SeekMode,
) -> (Vec<Vec<Word>>, Vec<Vec<Word>>) {
    let cell = program.cell(0).expect("cell 0");
    let mut reference = seeded(program, 0);
    let mut composed =
        Composed::<f32>::new(program, &[(0, CursorKind::Value)], cell.completion).expect("bind");
    composed.seed(identity);
    let (mut expected, mut actual) = (Vec::new(), Vec::new());
    for &(span, jump, ops) in blocks {
        if let Some(completion) = ops.hold {
            reference.hold_until(completion);
            // A ramp the caller reports on one cell is the cell's and the output's.
            composed.state.hold_until(completion);
            if let Some(cursor) = composed.cursor_mut(0) {
                cursor.state_mut().hold_until(completion);
            }
        }
        if let Some(at) = ops.defer {
            reference.defer_jump(at);
            composed.defer_jump(at);
        }
        expected.push(
            CellEvents::new(cell, &mut reference, span, mode, jump)
                .map(|event| word(&event))
                .collect::<Vec<_>>(),
        );
        if let Some(cursor) = composed.cursor_mut(0) {
            cursor.set_jump(jump);
        }
        let mut events = Vec::new();
        composed.walk(program, span, mode, identity, |event| {
            events.push(composed_word(&event));
        });
        actual.push(events);
    }
    (expected, actual)
}

/// K1 gate 1, corpus half: one cursor under the identity composition gives exactly its cell's
/// events, block for block, over every case of the G5 corpus (its tables, cell rules, node-time
/// mappings, seeks of both modes and the ramps it reports).
///
/// Red if the merge, a resumed pull or the output's change-only, lagging or held-event rule
/// departs from one cell's.
#[test]
fn one_cursor_gives_its_cells_events_over_the_corpus() {
    for case in 0..CASE_COUNT {
        let spec = case_of(case);
        let program = ProgramBuilder::build(&[CellDescription {
            segments: &spec.table,
            grid_ramp: spec.ramp,
            grid: spec.ramp.max(64),
            jump: spec.jump,
            completion: spec.completion,
            no_restart: spec.no_restart,
        }])
        .expect("corpus cell");
        let blocks: Vec<_> = blocks(case)
            .into_iter()
            .map(|block| {
                (
                    block.span,
                    block.jump,
                    Ops {
                        hold: block.hold,
                        defer: None,
                    },
                )
            })
            .collect();
        let (expected, actual) = one_cursor_streams(&program, &blocks, spec.mode);
        let total: usize = expected.iter().map(Vec::len).sum();
        assert!(total > 10, "case {case} must give events: {total}");
        assert_eq!(actual, expected, "case {case}");
    }
}

/// A xorshift64* generator: the differential's tables and mappings, reproducible from a seed.
struct Rng(u64);

impl Rng {
    const fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    const fn below(&mut self, bound: u64) -> u64 {
        let bound = if bound == 0 { 1 } else { bound };
        self.next() % bound
    }

    const fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }

    fn value(&mut self, positive: bool) -> f32 {
        let value = (self.below(4_000) as f32 - 2_000.0) / 50.0;
        if positive { value.abs() + 0.5 } else { value }
    }
}

/// A random table: segments of every shape, with gaps, adjacent joins and equal-valued joins.
/// Draws whose discontinuities break the 64-sample spacing are refused by the builder, and the
/// caller draws again.
fn random_table(rng: &mut Rng) -> Vec<Segment> {
    let mut table = Vec::new();
    let mut time = rng.below(500);
    for _ in 0..1 + rng.below(12) {
        let length = 64 + rng.below(3_000);
        let shape = match rng.below(3) {
            0 => Shape::Step,
            1 => Shape::Linear,
            _ => Shape::Exponential,
        };
        let positive = shape == Shape::Exponential;
        let start = rng.value(positive);
        let end = if rng.chance(20) {
            start
        } else {
            rng.value(positive)
        };
        table.push(seg(time, time + length, start, end, shape));
        time += length + if rng.chance(50) { 0 } else { rng.below(800) };
    }
    table
}

/// A random cell rule over a random table, built, and a seek mode.
fn random_program(rng: &mut Rng) -> (Program, SeekMode) {
    loop {
        let table = random_table(rng);
        let ramp = if rng.chance(70) { 64 } else { 128 };
        let description = CellDescription {
            segments: &table,
            grid_ramp: ramp,
            grid: ramp,
            jump: if rng.chance(70) {
                JumpLength::Block
            } else {
                JumpLength::Fixed(64 * (1 + rng.below(8) as u32))
            },
            completion: if rng.chance(70) {
                Completion::End
            } else {
                Completion::After
            },
            no_restart: rng.chance(20),
        };
        let mode = if rng.chance(70) {
            SeekMode::Exact
        } else {
            SeekMode::Ramp
        };
        if let Ok(program) = ProgramBuilder::build(&[description]) {
            return (program, mode);
        }
    }
}

/// A random node-time mapping: blocks of a random quantum, seeks forward and backward at random
/// offsets, reported ramps and deferred jumps.
fn random_blocks(rng: &mut Rng) -> Vec<(NodeSpan, u32, Ops)> {
    let quantum = [1, 37, 64, 100, 128, 256, 300][rng.below(7) as usize];
    let mut node = rng.below(400) as i64 - 200;
    let mut out = Vec::new();
    for _ in 0..120 {
        let step = if rng.chance(6) {
            Some((
                rng.below(u64::from(quantum)) as u32,
                rng.below(16_000) as i64,
            ))
        } else {
            None
        };
        let jump = [0, 64, 240, 480, 960][rng.below(5) as usize];
        let hold = if rng.chance(8) {
            Some(node + rng.below(900) as i64)
        } else {
            None
        };
        let defer = if rng.chance(8) {
            Some(node + rng.below(900) as i64)
        } else {
            None
        };
        let Ok(span) = NodeSpan::new(node, step, quantum) else {
            continue;
        };
        out.push((span, jump, Ops { hold, defer }));
        node = match step {
            Some((offset, time)) => time + i64::from(quantum - offset),
            None => node + i64::from(quantum),
        };
    }
    out
}

/// K1 gate 1, generator half: one cursor gives exactly its cell's events over 400 random tables,
/// cell rules and mappings (seeks of both modes, reported ramps, deferred jumps).
///
/// Red if a resumed pull is not the uninterrupted cell's stream anywhere the generator reaches (a
/// seek inside a window, a deferral behind a run, a `no_restart` cell's re-deferral), or the
/// output's rules differ from the cell's.
#[test]
fn one_cursor_gives_its_cells_events_over_random_tables() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut total = 0;
    for draw in 0..400 {
        let (program, mode) = random_program(&mut rng);
        let blocks = random_blocks(&mut rng);
        let (expected, actual) = one_cursor_streams(&program, &blocks, mode);
        total += expected.iter().map(Vec::len).sum::<usize>();
        assert_eq!(actual, expected, "draw {draw}");
    }
    assert!(total > 10_000, "the generator must reach events: {total}");
}

/// K1 gate 2: with every curve but one flat, the output under a composition that passes the
/// moving curve through (a product with the flat curves at 1.0) gives exactly that curve's events,
/// whatever the flat curves' kinds and the moving curve's place in the table.
///
/// Red if a flat curve's cursor changes the merge (an event of its own, a deferral it starts, a
/// landing it lengthens).
#[test]
fn flat_curves_leave_the_moving_curves_events() {
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    let flat = [seg(0, 1 << 40, 1.0, 1.0, Shape::Step)];
    for draw in 0..150 {
        let (moving, mode) = random_program(&mut rng);
        let rule = moving.cell(0).expect("cell");
        let moving_cell = CellDescription {
            segments: rule.segments(),
            grid_ramp: rule.ramp,
            grid: rule.grid,
            jump: rule.jump,
            completion: rule.completion,
            no_restart: rule.no_restart,
        };
        // A flat curve lands a ramped seek over no ramp, so the landing keeps the moving one's.
        let flat_cell = CellDescription {
            jump: JumpLength::Fixed(0),
            ..fader_cell(&flat)
        };
        let program = ProgramBuilder::build(&[flat_cell, moving_cell]).expect("program");
        let place = rng.below(3) as usize;
        let cells: Vec<_> = (0..3)
            .map(|index| match index {
                _ if index == place => (1, CursorKind::Value),
                0 => (0, CursorKind::Gate),
                _ => (0, CursorKind::Value),
            })
            .collect();
        let mut composed = Composed::<f32>::new(&program, &cells, rule.completion).expect("bind");
        let product = |heard: Heard<'_>| (0..heard.len()).map(|index| heard.value(index)).product();
        composed.seed(product);
        let mut reference = seeded(&program, 1);
        let reference_cell = program.cell(1).expect("cell");
        for (span, jump, _) in random_blocks(&mut rng) {
            let expected: Vec<Word> =
                CellEvents::new(reference_cell, &mut reference, span, mode, jump)
                    .map(|event| word(&event))
                    .collect();
            for cursor in composed.cursors_mut() {
                cursor.set_jump(jump);
            }
            let mut actual = Vec::new();
            composed.walk(&program, span, mode, product, |event| {
                actual.push(composed_word(&event));
            });
            assert_eq!(actual, expected, "draw {draw}");
        }
    }
}

/// The non-held events of a block of 4,000 samples from node time 0, as `(offset, kind, ramp)`,
/// and the value of the one at `at`, if any.
fn retargets(
    program: &Program,
    composed: &mut Composed<f32>,
    compose: impl FnMut(Heard<'_>) -> f32,
    at: u32,
) -> (Vec<(u32, EventKind, u32)>, Option<f32>) {
    let span = NodeSpan::new(0, None, 4_000).expect("span");
    let mut out = Vec::new();
    let mut value = None;
    composed.walk(program, span, SeekMode::Exact, compose, |event| {
        if !event.held {
            out.push((event.offset, event.kind, event.ramp));
            if event.offset == at {
                value = Some(event.value);
            }
        }
    });
    (out, value)
}

/// The output `gain` while the mute curve (1 until 1,000, then 0: an unmute) is open, `0.0` while
/// it is closed, with the block lengths `[mute, gain]`.
fn gate_and(gain: &[Segment], lengths: [u32; 2]) -> (Program, Composed<f32>) {
    let mute = [seg(0, 1_000, 1.0, 0.0, Shape::Step)];
    let program = ProgramBuilder::build(&[fader_cell(&mute), fader_cell(gain)]).expect("program");
    let mut composed = Composed::new(
        &program,
        &[(0, CursorKind::Gate), (1, CursorKind::Value)],
        Completion::End,
    )
    .expect("bind");
    composed.seed(gated);
    for (cursor, jump) in composed.cursors_mut().iter_mut().zip(lengths) {
        cursor.set_jump(jump);
    }
    (program, composed)
}

fn gated(heard: Heard<'_>) -> f32 {
    if heard.value(0) == 0.0 {
        heard.value(1)
    } else {
        0.0
    }
}

/// K1 gate 3 (R19-3): a gain grid event inside the unmute's jump does not cut it. The unmute at
/// 1,000 jumps over the mute length (480) and nothing retargets the output before 1,480, where
/// the gain lands over its own length (256) to the curve at the landing's last frame; its grid
/// then moves the output again from the first grid point whose ramp ends after the landing's
/// (1,728), and on every grid point after it.
///
/// Red if a grid event inside another curve's jump retargets the output (cuts the fade), if the
/// held move is not landed at the jump's end, or if it lands over the gate's length.
#[test]
fn a_grid_event_inside_a_gate_jump_is_held_and_lands_at_its_end() {
    let ride = [seg(0, 8_000, 0.0, -24.0, Shape::Linear)];
    let (program, mut composed) = gate_and(&ride, [480, 256]);
    let (events, landing) = retargets(&program, &mut composed, gated, 1_480);
    let window: Vec<_> = events
        .iter()
        .copied()
        .filter(|&(offset, ..)| offset < 1_750)
        .collect();
    assert_eq!(
        window,
        [
            (1_000, EventKind::Jump, 480),
            (1_480, EventKind::Jump, 256),
            (1_728, EventKind::Grid, 64),
        ]
    );
    let ride_cell = program.cell(1).expect("cell");
    assert_eq!(
        landing.map(f32::to_bits),
        Some(crate::value_at(ride_cell.segments(), 1_480 + 255).to_bits())
    );
    let after: Vec<_> = events
        .iter()
        .filter(|&&(offset, ..)| offset > 1_728)
        .collect();
    assert!(after.len() > 30);
    assert!(
        after.iter().all(|&&(offset, kind, ramp)| offset % 64 == 0
            && kind == EventKind::Grid
            && ramp == 64)
    );
}

/// K1 gate 4 (R19-4): a gate jump and a value jump at one offset are one retarget over the longer
/// jump, whichever curve owns it.
///
/// Red if coincident events take one curve's length regardless, or emit two events at one offset.
#[test]
fn coincident_jumps_take_the_longest() {
    // The gain steps from -6 to -12 at 1,000, where the unmute is.
    let step = [
        seg(0, 1_000, -6.0, -6.0, Shape::Step),
        seg(1_000, 9_000, -12.0, -12.0, Shape::Step),
    ];
    for lengths in [[480, 960], [960, 480]] {
        let (program, mut composed) = gate_and(&step, lengths);
        let (events, value) = retargets(&program, &mut composed, gated, 1_000);
        assert_eq!(events, [(1_000, EventKind::Jump, 960)], "{lengths:?}");
        assert_eq!(value, Some(-12.0));
    }
}

/// One riding value cursor bound with the block length 480.
fn riding(program: &Program) -> Composed<f32> {
    let mut composed =
        Composed::<f32>::new(program, &[(0, CursorKind::Value)], Completion::End).expect("bind");
    composed.seed(identity);
    if let Some(cursor) = composed.cursor_mut(0) {
        cursor.set_jump(480);
    }
    composed
}

/// K1 gate 5 (#1711 D1 on the output): a jump deferred to 300 holds the riding curve's grid until
/// it fires: every grid event before 300 is held, and at 300 the output jumps over the block's
/// length. Cleared, the grid moves the output from its first point.
///
/// Red if the output retargets on a grid point before the deferral fires, or the deferral fires
/// nothing, or a cleared one still holds the grid.
#[test]
fn a_deferral_holds_the_grid_until_it_fires() {
    let ride = [seg(0, 8_000, 0.0, -24.0, Shape::Linear)];
    let program = ProgramBuilder::build(&[fader_cell(&ride)]).expect("program");
    let span = NodeSpan::new(0, None, 512).expect("span");
    let mut deferred = riding(&program);
    deferred.defer_jump(300);
    let mut events = Vec::new();
    deferred.walk(&program, span, SeekMode::Exact, identity, |event| {
        events.push((event.offset, event.kind, event.ramp, event.held));
    });
    assert_eq!(
        events.get(..6),
        Some(
            &[
                (0, EventKind::Grid, 64, true),
                (64, EventKind::Grid, 64, true),
                (128, EventKind::Grid, 64, true),
                (192, EventKind::Grid, 64, true),
                (256, EventKind::Grid, 64, true),
                (300, EventKind::Jump, 480, false),
            ][..]
        )
    );
    let mut cleared = riding(&program);
    cleared.defer_jump(300);
    cleared.clear_deferred_jump();
    let mut first = None;
    cleared.walk(&program, span, SeekMode::Exact, identity, |event| {
        first = first.or(Some((event.offset, event.kind, event.held)));
    });
    assert_eq!(first, Some((0, EventKind::Grid, false)));
}

/// R19-3 on a record: a column jump the caller starts ([`Composed::retarget`] over 200 samples
/// from 0, to the curve at its completion) holds the value grid until its first free sample,
/// where the curve lands over its own length; a zero-length retarget defers nothing.
///
/// Red if a caller's column jump does not hold the value grid, or holds it past its end.
#[test]
fn a_caller_retarget_holds_the_value_grid_until_its_end() {
    let ride = [seg(0, 8_000, 0.0, -24.0, Shape::Linear)];
    let program = ProgramBuilder::build(&[fader_cell(&ride)]).expect("program");
    for (length, first) in [
        (200, (200, EventKind::Jump, 480)),
        (0, (0, EventKind::Grid, 64)),
    ] {
        let mut composed = riding(&program);
        let target = composed.compose_toward(&program, 0, length, identity);
        assert_eq!(
            target.to_bits(),
            crate::value_at(&ride, i64::from(length.saturating_sub(1))).to_bits()
        );
        composed.retarget(target, 0, length);
        let (events, _) = retargets(&program, &mut composed, identity, 0);
        assert_eq!(events.first(), Some(&first), "record length {length}");
    }
}

/// R19-3 with a value jump that changes nothing (#1711 U1 fold 2): the unmute at 1,000 jumps over
/// 480 and defers the value cursor to 1,480. The gain is flat at -5.9921875 to 1,200, then linear
/// from -4 to -8 over 1,200..1,712; its discontinuity at 1,200 targets the curve at 1,455
/// (-5.9921875, its value already), so its jump emits nothing and replaces no ramp. The deferral
/// stays owed: the grid is held until 1,480, where the gain lands over its own length.
///
/// Red if a discontinuity whose jump emits nothing drops the deferral (the grid then cuts the
/// unmute's jump at 1,216 and the landing at 1,480 is skipped).
#[test]
fn an_unchanged_value_jump_keeps_the_gate_deferral() {
    let gain = [
        seg(0, 1_200, -5.992_187_5, -5.992_187_5, Shape::Step),
        seg(1_200, 1_712, -4.0, -8.0, Shape::Linear),
        seg(1_712, 9_000, -8.0, -8.0, Shape::Step),
    ];
    let (program, mut composed) = gate_and(&gain, [480, 256]);
    let (events, _) = retargets(&program, &mut composed, gated, 1_480);
    assert_eq!(
        events,
        [(1_000, EventKind::Jump, 480), (1_480, EventKind::Jump, 256)]
    );
}

fn summed(heard: Heard<'_>) -> f32 {
    heard.value(0) + heard.value(1)
}

/// #1655 V2 (R19-2), `with_held_grid`: a step of one value curve keeps its jump length while
/// another rides, both ways round. The step at 1,000 jumps over 480; the riding curve's grid
/// points whose ramps would end inside that jump (1,024 to 1,408) are held, and the first one
/// whose ramp ends after it (1,472) retargets again, by the one-cell rule. Without the rule (a
/// route column, R19-3) the riding curve's grid point at 1,024 cuts the jump short.
///
/// Red if a grid event inside another curve's jump retargets a held-grid output, or if the hold
/// outlasts the jump.
#[test]
fn a_step_keeps_its_jump_length_while_another_curve_rides_on_a_held_grid_output() {
    let ride = [seg(0, 8_000, 0.0, -24.0, Shape::Linear)];
    let step = [
        seg(0, 1_000, -6.0, -6.0, Shape::Step),
        seg(1_000, 9_000, -12.0, -12.0, Shape::Step),
    ];
    for order in [[&ride[..], &step[..]], [&step[..], &ride[..]]] {
        let program =
            ProgramBuilder::build(&[fader_cell(order[0]), fader_cell(order[1])]).expect("program");
        let bind = |held: bool| {
            let composed = Composed::<f32>::new(
                &program,
                &[(0, CursorKind::Value), (1, CursorKind::Value)],
                Completion::End,
            )
            .expect("bind");
            let mut composed = if held {
                composed.with_held_grid()
            } else {
                composed
            };
            composed.seed(summed);
            for cursor in composed.cursors_mut() {
                cursor.set_jump(480);
            }
            composed
        };
        let after_step = |events: &[(u32, EventKind, u32)]| {
            events
                .iter()
                .copied()
                .filter(|&(offset, _, _)| offset >= 1_000)
                .take(2)
                .collect::<Vec<_>>()
        };
        let (held, _) = retargets(&program, &mut bind(true), summed, 1_000);
        assert_eq!(
            after_step(&held),
            [(1_000, EventKind::Jump, 480), (1_472, EventKind::Grid, 64)]
        );
        let (free, _) = retargets(&program, &mut bind(false), summed, 1_000);
        assert_eq!(
            after_step(&free),
            [(1_000, EventKind::Jump, 480), (1_024, EventKind::Grid, 64)]
        );
    }
}
