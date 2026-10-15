//! Gates 2 to 4: the event rules, the per-block bound and quantum and seek invariance.

mod common;

use automation::{
    CellEvents, CellState, Completion, EventKind, JumpLength, NodeSpan, ProgramBuilder, SeekMode,
    Shape, value_at,
};
use common::{block, description, fader, run, seg};

fn span(start: i64, step: Option<(u32, i64)>, frames: u32) -> NodeSpan {
    NodeSpan::new(start, step, frames).unwrap()
}

/// Red if the grid follows render samples: a grid event at every node-time multiple of 64 and
/// none elsewhere, at quantum 100 (not a multiple of 64) from an off-grid start.
#[test]
fn grid_events_sit_on_node_time_multiples_of_64() {
    let program = fader(&[seg(0, 2048, 0.0, 2048.0, Shape::Linear)]);
    let mut state = CellState::new();
    let events = run(&program, &mut state, -37, 100, 25, SeekMode::Exact, 480);
    let times: Vec<i64> = events.iter().map(|(time, _)| *time).collect();
    assert!(times.iter().all(|time| time % 64 == 0), "{times:?}");
    let expected: Vec<i64> = (0..=32).map(|k| k * 64).collect();
    assert_eq!(times, expected, "the curve ends at 2048 and then holds");
    for (time, event) in &events {
        assert_eq!(event.kind, EventKind::Grid);
        assert_eq!(event.ramp, 64);
        assert_eq!(
            event.value,
            value_at(program.cell(0).unwrap().segments(), time + 63)
        );
    }
}

/// Red if a jump snaps to the grid, if a grid ramp that spans a discontinuity eats the jump
/// (change only would then drop it), or if the jump ramp is not the block's length.
#[test]
fn jump_at_off_grid_discontinuity_at_its_exact_offset() {
    let program = fader(&[
        seg(0, 100, 1.0, 1.0, Shape::Step),
        seg(100, 300, 5.0, 5.0, Shape::Step),
    ]);
    let mut state = CellState::new();
    let events = block(
        &program,
        &mut state,
        span(0, None, 128),
        SeekMode::Exact,
        10,
    );
    let shape: Vec<_> = events
        .iter()
        .map(|e| (e.offset, e.kind, e.ramp, e.value))
        .collect();
    assert_eq!(
        shape,
        [
            (0, EventKind::Grid, 64, 1.0),
            (100, EventKind::Jump, 10, 5.0)
        ]
    );
    let next = block(
        &program,
        &mut state,
        span(128, None, 128),
        SeekMode::Exact,
        10,
    );
    assert!(
        next.is_empty(),
        "the curve is flat after the jump: {next:?}"
    );
}

/// Red if a completion is off by one: `End` aims at `τ + n - 1`, `After` at `τ + n`.
#[test]
fn end_and_after_completions() {
    let table = [
        seg(0, 100, 7.0, 7.0, Shape::Step),
        seg(100, 1100, 0.0, 1000.0, Shape::Linear),
    ];
    for (completion, aim) in [(Completion::End, 109), (Completion::After, 110)] {
        let program = ProgramBuilder::build(&[description(
            &table,
            64,
            JumpLength::Block,
            completion,
            false,
        )])
        .unwrap();
        let mut state = CellState::new();
        let events = block(
            &program,
            &mut state,
            span(100, None, 28),
            SeekMode::Exact,
            10,
        );
        assert_eq!(events.len(), 1, "{events:?}");
        assert_eq!(events[0].kind, EventKind::Jump);
        assert_eq!(events[0].value, value_at(&table, aim));
    }
}

/// Red if a redundant retarget is emitted: a flat curve emits its first event only.
#[test]
fn change_only() {
    let program = fader(&[seg(0, 100_000, 3.0, 3.0, Shape::Linear)]);
    let mut state = CellState::new();
    let events = run(&program, &mut state, 0, 128, 50, SeekMode::Exact, 480);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].1.value, 3.0);
}

/// Red if a held event retargets the stage or the grid does not resume after the reported ramp.
#[test]
fn held_inside_a_reported_ramp_and_the_grid_resumes() {
    let program = fader(&[seg(0, 100_000, 0.0, 100_000.0, Shape::Linear)]);
    let mut state = CellState::new();
    state.hold_until(200);
    let events = run(&program, &mut state, 0, 256, 1, SeekMode::Exact, 480);
    let held: Vec<(i64, bool)> = events.iter().map(|(t, e)| (*t, e.held)).collect();
    assert_eq!(held, [(0, true), (64, true), (128, true), (192, false)]);
    assert_eq!(
        state.target(),
        Some(value_at(program.cell(0).unwrap().segments(), 255))
    );
    // The held values did not become the target; the first free one did.
    let last = events.last().unwrap().1;
    assert_eq!(Some(last.value), state.target());
}

/// Red if a held event inside a ramp is dropped as unchanged while the stage still remembers an
/// older held value.
#[test]
fn a_held_value_back_at_the_target_still_emits_once() {
    // The target 1.0 is set at 0 by a free grid event; the curve then rises and returns while a
    // reported ramp is in flight, with no discontinuity.
    let table = [
        seg(0, 64, 1.0, 1.0, Shape::Linear),
        seg(64, 128, 1.0, 4.0, Shape::Linear),
        seg(128, 192, 4.0, 1.0, Shape::Linear),
        seg(192, 10_000, 1.0, 1.0, Shape::Linear),
    ];
    let program = fader(&table);
    let mut state = CellState::new();
    let first = block(&program, &mut state, span(0, None, 1), SeekMode::Exact, 480);
    assert_eq!(first.len(), 1);
    state.hold_until(1000);
    let events = run(&program, &mut state, 1, 255, 1, SeekMode::Exact, 480);
    let held: Vec<(i64, f32, bool)> = events.iter().map(|(t, e)| (*t, e.value, e.held)).collect();
    assert_eq!(held.len(), 3, "{held:?}");
    assert!(held.iter().all(|&(_, _, h)| h));
    assert!(held[0].1 > 3.9 && held[1].1 > 1.0);
    assert_eq!(
        (held[2].0, held[2].1),
        (192, 1.0),
        "the refresh back at the target emits once"
    );
    assert_eq!(
        state.target(),
        Some(1.0),
        "held events never move the target"
    );
}

/// Red if a `no_restart` jump retargets inside the ramp it would restart, or is lost.
#[test]
fn no_restart_jump_waits_for_the_ramp_to_end() {
    let table = [
        seg(0, 100, 1.0, 1.0, Shape::Step),
        seg(100, 200, 2.0, 2.0, Shape::Step),
        seg(200, 100_000, 3.0, 3.0, Shape::Step),
    ];
    let program = ProgramBuilder::build(&[description(
        &table,
        128,
        JumpLength::Fixed(128),
        Completion::End,
        true,
    )])
    .unwrap();
    let mut state = CellState::new();
    let events = run(&program, &mut state, 0, 100, 5, SeekMode::Exact, 0);
    let shape: Vec<_> = events
        .iter()
        .map(|(t, e)| (*t, e.kind, e.value, e.held))
        .collect();
    // The grid ramp started at 0 runs to 127: the jump at 100 waits to 128 and aims at 2.0 (the
    // next discontinuity at 200 bounds its window); the jump at 200 comes after that ramp ends
    // at 255, at 256, and aims at 3.0.
    assert_eq!(
        shape,
        [
            (0, EventKind::Grid, 1.0, false),
            (128, EventKind::Jump, 2.0, false),
            (256, EventKind::Jump, 3.0, false),
        ]
    );
}

fn seek_table() -> automation::Program {
    fader(&[
        seg(0, 1000, 0.0, 10.0, Shape::Linear),
        seg(1000, 4000, 10.0, 50.0, Shape::Exponential),
    ])
}

/// Red if a seek step emits nothing, at a mid-block offset or at offset 0, or lands on the wrong
/// value or kind.
#[test]
fn set_at_a_seek_step_for_exact_and_a_ramped_jump_for_ramp() {
    let program = seek_table();
    let segments = program.cell(0).unwrap().segments();
    for offset in [0, 37] {
        let mut state = CellState::new();
        let a = block(
            &program,
            &mut state,
            span(0, Some((offset, 2500)), 128),
            SeekMode::Exact,
            480,
        );
        let set = a
            .iter()
            .find(|e| e.offset == offset)
            .expect("an event at the step");
        assert_eq!(set.kind, EventKind::Set, "offset {offset}");
        assert_eq!(set.ramp, 0);
        assert_eq!(set.value, value_at(segments, 2500));
        assert_eq!(a.iter().filter(|e| e.offset == offset).count(), 1);

        let mut state = CellState::new();
        let b = block(
            &program,
            &mut state,
            span(0, Some((offset, 2500)), 128),
            SeekMode::Ramp,
            480,
        );
        let jump = b
            .iter()
            .find(|e| e.offset == offset)
            .expect("an event at the step");
        assert_eq!(jump.kind, EventKind::Jump, "offset {offset}");
        assert_eq!(jump.ramp, 480);
        assert_eq!(jump.value, value_at(segments, 2500 + 479));
    }
}

/// Red if `Set` is dropped by change only: a seek to the value the target already holds still
/// emits.
#[test]
fn set_always_emits() {
    let program = fader(&[seg(0, 100_000, 3.0, 3.0, Shape::Linear)]);
    let mut state = CellState::new();
    let _ = block(&program, &mut state, span(0, None, 64), SeekMode::Exact, 0);
    let again = block(
        &program,
        &mut state,
        span(64, Some((10, 5000)), 64),
        SeekMode::Exact,
        0,
    );
    assert_eq!(again.iter().filter(|e| e.kind == EventKind::Set).count(), 1);
}

/// Red if a jump uses a stored length instead of the word the block passes.
#[test]
fn a_block_keyed_jump_ramps_over_the_blocks_length() {
    let program = fader(&[
        seg(0, 100, 1.0, 1.0, Shape::Step),
        seg(100, 400, 5.0, 5.0, Shape::Step),
        seg(400, 500, 9.0, 9.0, Shape::Step),
    ]);
    let mut state = CellState::new();
    let first = block(
        &program,
        &mut state,
        span(64, None, 100),
        SeekMode::Exact,
        100,
    );
    let second = block(
        &program,
        &mut state,
        span(300, None, 200),
        SeekMode::Exact,
        300,
    );
    let jump = |events: &[automation::Event]| {
        events
            .iter()
            .find(|e| e.kind == EventKind::Jump)
            .map(|e| e.ramp)
    };
    assert_eq!(jump(&first), Some(100));
    assert_eq!(jump(&second), Some(300));
}

/// A `Fixed` key ignores the block's word.
#[test]
fn a_fixed_key_ignores_the_blocks_length() {
    let table = [
        seg(0, 100, 1.0, 1.0, Shape::Step),
        seg(100, 400, 5.0, 5.0, Shape::Step),
    ];
    let program = ProgramBuilder::build(&[description(
        &table,
        64,
        JumpLength::Fixed(64),
        Completion::After,
        false,
    )])
    .unwrap();
    let mut state = CellState::new();
    let events = block(
        &program,
        &mut state,
        span(64, None, 100),
        SeekMode::Exact,
        999,
    );
    assert_eq!(
        events
            .iter()
            .find(|e| e.kind == EventKind::Jump)
            .map(|e| e.ramp),
        Some(64)
    );
}

/// Red if a span with a step outside the block is accepted.
#[test]
fn a_step_outside_the_block_is_refused() {
    assert!(NodeSpan::new(0, Some((128, 5)), 128).is_err());
    assert!(NodeSpan::new(0, Some((127, 5)), 128).is_ok());
}

fn adversarial() -> automation::Program {
    // A discontinuity every 64 samples, every segment moving.
    let table: Vec<_> = (0..400_u64)
        .map(|k| {
            let (a, b) = if k % 2 == 0 { (0.0, 1.0) } else { (3.0, 2.0) };
            seg(k * 64 + 32, k * 64 + 96, a, b, Shape::Linear)
        })
        .collect();
    fader(&table)
}

fn no_restart_adversarial() -> automation::Program {
    // The same discontinuity every 64 samples, on a delay-like cell whose jumps wait for its ramp.
    let table: Vec<_> = (0..400_u64)
        .map(|k| {
            let (a, b) = if k % 2 == 0 { (0.0, 1.0) } else { (3.0, 2.0) };
            seg(k * 64 + 32, k * 64 + 96, a, b, Shape::Linear)
        })
        .collect();
    ProgramBuilder::build(&[description(
        &table,
        32,
        JumpLength::Fixed(0),
        Completion::End,
        true,
    )])
    .unwrap()
}

/// The stated per-block bound: `2⌈q/64⌉ + 3`, plus one for a `no_restart` cell.
fn stated_bound(quantum: u32, no_restart: bool) -> usize {
    2 * quantum.div_ceil(64) as usize + 3 + usize::from(no_restart)
}

/// Red if a pathological table makes a block's work grow past the stated bound, or if
/// `max_events` (the capacity a successor sizes its buffer by) differs from it: a discontinuity
/// every 64 samples, seeks mid-block, and for the `no_restart` cell, caller ramps that defer jumps
/// across blocks.
#[test]
fn events_per_block_are_bounded() {
    for (program, no_restart) in [(adversarial(), false), (no_restart_adversarial(), true)] {
        let cell = program.cell(0).unwrap();
        for quantum in [1_u32, 63, 64, 100, 128, 1024] {
            let bound = stated_bound(quantum, no_restart);
            assert_eq!(cell.max_events(quantum) as usize, bound);
            let mut state = CellState::new();
            let mut most = 0;
            for index in 0..(20_000 / quantum).min(2_000) {
                let first = i64::from(index) * i64::from(quantum);
                // A seek in every fifth block, at a mid-block offset; a caller ramp in every
                // seventh.
                let step = (index % 5 == 2).then_some((quantum / 3, (first * 7) % 24_000));
                if index % 7 == 3 {
                    state.hold_until(first + 70);
                }
                let events = CellEvents::new(
                    cell,
                    &mut state,
                    span(first, step, quantum),
                    SeekMode::Exact,
                    480,
                )
                .collect::<Vec<_>>();
                assert!(events.windows(2).all(|w| w[0].offset < w[1].offset));
                assert!(events.iter().all(|e| e.offset < quantum));
                assert!(
                    events.len() <= bound,
                    "no_restart {no_restart}, q {quantum}: {} > {bound}",
                    events.len()
                );
                most = most.max(events.len());
            }
            // The tables stress the bound: a block holds more events than its grid points alone.
            if quantum >= 64 {
                assert!(
                    most > quantum.div_ceil(64) as usize,
                    "no_restart {no_restart}, q {quantum}: most {most}"
                );
            }
        }
    }
}

/// Jumps at 70 and 134, then a step to a ramp at 1010, on a `no_restart` cell with jump ramp 0.
fn deferral_table() -> automation::Program {
    let table = [
        seg(0, 70, 1.0, 1.0, Shape::Step),
        seg(70, 134, 2.0, 2.0, Shape::Step),
        seg(134, 1010, 3.0, 3.0, Shape::Step),
        seg(1010, 2000, 4.0, 5.0, Shape::Linear),
    ];
    ProgramBuilder::build(&[description(
        &table,
        32,
        JumpLength::Fixed(0),
        Completion::End,
        true,
    )])
    .unwrap()
}

/// The verifier's case for attempt 1's `+ 3` on a `no_restart` cell: a jump deferred from the
/// previous block, two discontinuities and a seek in one 64-sample block. Red if the bound or
/// `max_events` is lower than the six events this block yields (so `+ 4` is reached, not slack).
#[test]
fn a_carried_deferral_reaches_the_no_restart_bound() {
    let program = deferral_table();
    let cell = program.cell(0).unwrap();
    let mut state = CellState::new();
    state.hold_until(130);
    let _ = block(&program, &mut state, span(64, None, 64), SeekMode::Exact, 0);
    let events = block(
        &program,
        &mut state,
        span(128, Some((23, 1000)), 64),
        SeekMode::Exact,
        0,
    );
    let shape: Vec<_> = events.iter().map(|e| (e.offset, e.kind)).collect();
    assert_eq!(
        shape,
        [
            (0, EventKind::Grid),
            (3, EventKind::Jump),  // the jump at 70, deferred to 131
            (6, EventKind::Jump),  // the discontinuity at 134
            (23, EventKind::Set),  // the seek to 1000
            (33, EventKind::Jump), // the discontinuity at 1010
            (47, EventKind::Grid), // 1024
        ]
    );
    assert_eq!(events.len(), stated_bound(64, true));
    assert_eq!(cell.max_events(64) as usize, events.len());
}

/// Red if a deferral fires inside a caller ramp reported between blocks (attempt 2 checked the lane
/// only for a fresh discontinuity): the jump at 70, deferred to 131, meets a caller ramp that runs
/// to 200, so it and the discontinuity at 134 wait for the lane and fire once at 201, the ramp's
/// last sample plus one, aimed at the curve there.
#[test]
fn a_deferral_waits_for_a_caller_ramp_reported_after_it() {
    let program = deferral_table();
    let mut state = CellState::new();
    state.hold_until(130);
    let _ = block(&program, &mut state, span(64, None, 64), SeekMode::Exact, 0);
    state.hold_until(200);
    let second = block(
        &program,
        &mut state,
        span(128, None, 64),
        SeekMode::Exact,
        0,
    );
    assert!(
        second.iter().all(|e| e.kind == EventKind::Grid && e.held),
        "{second:?}"
    );
    let third = block(
        &program,
        &mut state,
        span(192, None, 64),
        SeekMode::Exact,
        0,
    );
    let jumps: Vec<_> = third
        .iter()
        .filter(|e| e.kind == EventKind::Jump)
        .map(|e| (192 + i64::from(e.offset), e.value))
        .collect();
    assert_eq!(jumps, [(201, 3.0)], "{third:?}");

    // The same deferral carried behind a forward span is due at the run's first sample (500),
    // where the caller's ramp still runs to 600: it waits for 601.
    let mut state = CellState::new();
    state.hold_until(130);
    let _ = block(&program, &mut state, span(64, None, 64), SeekMode::Exact, 0);
    state.hold_until(600);
    let skipped = block(
        &program,
        &mut state,
        span(500, None, 64),
        SeekMode::Exact,
        0,
    );
    assert!(
        skipped.iter().all(|e| e.kind != EventKind::Jump),
        "{skipped:?}"
    );
    let _ = block(
        &program,
        &mut state,
        span(564, None, 32),
        SeekMode::Exact,
        0,
    );
    let due = block(
        &program,
        &mut state,
        span(596, None, 64),
        SeekMode::Exact,
        0,
    );
    let jumps: Vec<_> = due
        .iter()
        .filter(|e| e.kind == EventKind::Jump)
        .map(|e| (596 + i64::from(e.offset), e.value))
        .collect();
    assert_eq!(jumps, [(601, 3.0)], "{due:?}");
}

/// Red if a deferral the caller skipped past (node time moved forward with no seek step) gives an
/// offset outside the block (attempt 1 wrapped a negative distance to `4294966927`), or is lost:
/// it is due at the span's first sample, aimed at the curve there.
#[test]
fn a_deferral_behind_a_forward_span_is_due_at_its_start() {
    let program = deferral_table();
    let mut state = CellState::new();
    state.hold_until(130);
    let first = block(&program, &mut state, span(64, None, 64), SeekMode::Exact, 0);
    assert!(first.iter().all(|e| e.kind == EventKind::Grid && e.held));
    let events = block(
        &program,
        &mut state,
        span(500, None, 64),
        SeekMode::Exact,
        0,
    );
    assert!(events.iter().all(|e| e.offset < 64), "{events:?}");
    let shape: Vec<_> = events.iter().map(|e| (e.offset, e.kind, e.value)).collect();
    assert_eq!(shape, [(0, EventKind::Jump, 3.0)]);
}

/// Red if change only drops a real jump because the grid ramp in flight aims at the same value:
/// the grid ramp from 0 aims at `value(31) = 32`, the curve jumps from its left limit 33 to
/// `value(32) = 32`, and the ramp (completing at 64) has not reached 32 at sample 32.
#[test]
fn a_jump_to_the_in_flight_target_still_lands_at_its_time() {
    let table = [
        seg(0, 32, 1.0, 33.0, Shape::Linear),
        seg(32, 500, 32.0, 100.0, Shape::Linear),
    ];
    let program = ProgramBuilder::build(&[description(
        &table,
        64,
        JumpLength::Fixed(0),
        Completion::After,
        false,
    )])
    .unwrap();
    assert_eq!(program.cell(0).unwrap().discontinuities(), &[32]);
    let mut state = CellState::new();
    let events = block(&program, &mut state, span(0, None, 128), SeekMode::Exact, 0);
    let shape: Vec<_> = events
        .iter()
        .map(|e| (e.offset, e.kind, e.ramp, e.value))
        .collect();
    assert_eq!(shape[0], (0, EventKind::Grid, 64, 32.0));
    assert_eq!(shape[1], (32, EventKind::Jump, 0, 32.0));
}

fn digest_table() -> automation::Program {
    fader(&[
        seg(100, 2000, 1.0, 9.0, Shape::Linear),
        seg(2300, 2900, 2.0, 2.0, Shape::Step),
        seg(2900, 9000, 4.0, 400.0, Shape::Exponential),
        seg(9000, 9100, 3.0, 5.0, Shape::Step),
    ])
}

/// `(node time, kind, ramp, value bits, held)` of every event.
fn trace(
    program: &automation::Program,
    state: &mut CellState,
    start: i64,
    quantum: u32,
    blocks: u32,
    mode: SeekMode,
) -> Vec<(i64, EventKind, u32, u32, bool)> {
    run(program, state, start, quantum, blocks, mode, 480)
        .into_iter()
        .map(|(t, e)| (t, e.kind, e.ramp, e.value.to_bits(), e.held))
        .collect()
}

/// Red if event values depend on the quantum: the same table at quantum 128 and at quantum 100
/// (blocks that straddle the grid differently) gives the same events. The blocks hold no seek, so
/// the seek mode does not enter.
#[test]
fn quantum_invariance() {
    let program = digest_table();
    let a = trace(
        &program,
        &mut CellState::new(),
        -128,
        128,
        90,
        SeekMode::Exact,
    );
    let b = trace(
        &program,
        &mut CellState::new(),
        -128,
        100,
        116,
        SeekMode::Exact,
    );
    let end = 11_000;
    let a: Vec<_> = a.into_iter().filter(|e| e.0 < end).collect();
    let b: Vec<_> = b.into_iter().filter(|e| e.0 < end).collect();
    assert_eq!(a, b);
    assert!(a.len() > 100);
}

/// Red if the events after a seek depend on the render sample the seek landed at, or on what
/// came before it (blocker B1).
#[test]
fn seek_invariance() {
    let program = digest_table();
    let target = 2750_i64;
    for mode in [SeekMode::Exact, SeekMode::Ramp] {
        let mut results = Vec::new();
        // The same timeline sample reached at two render positions, after different histories.
        for (quantum, history, offset) in [(128_u32, 7_u32, 37_u32), (100, 11, 91), (128, 3, 0)] {
            let mut state = CellState::new();
            let _ = trace(&program, &mut state, 0, quantum, history, mode);
            let first = i64::from(quantum) * i64::from(history);
            let step = Some((offset, target));
            let landing = CellEvents::new(
                program.cell(0).unwrap(),
                &mut state,
                span(first, step, quantum),
                mode,
                480,
            )
            .collect::<Vec<_>>();
            let landed_at = landing.iter().find(|e| e.offset == offset).unwrap();
            assert!(matches!(landed_at.kind, EventKind::Set | EventKind::Jump));
            // The rest of the landing block and the following blocks, in node time.
            let mut after: Vec<(i64, EventKind, u32, u32, bool)> = landing
                .iter()
                .filter(|e| e.offset > offset)
                .map(|e| {
                    (
                        target + i64::from(e.offset - offset),
                        e.kind,
                        e.ramp,
                        e.value.to_bits(),
                        e.held,
                    )
                })
                .collect();
            let landed_block_end = target + i64::from(quantum - offset);
            after.extend(trace(
                &program,
                &mut state,
                landed_block_end,
                quantum,
                40,
                mode,
            ));
            after.retain(|e| e.0 < target + 4000);
            results.push((landed_at.value.to_bits(), landed_at.ramp, after));
        }
        assert!(results[0].2.len() > 20);
        assert!(results.windows(2).all(|w| w[0] == w[1]), "{mode:?}");
    }
}
