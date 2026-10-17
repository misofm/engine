//! Gate 5: event generation allocates nothing.

mod common;

use automation::{CellEvents, CellState, NodeSpan, SeekMode, Shape};
use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use common::{fader, seg};

/// Red if generating the events of 10,000 blocks, seeks and all, allocates or frees.
#[test]
fn ten_thousand_blocks_allocate_nothing() {
    assert_installed();
    let table: Vec<_> = (0..300_u64)
        .map(|k| {
            seg(
                k * 700,
                k * 700 + 700,
                (k % 5) as f32,
                ((k * 3) % 7) as f32 + 1.0,
                Shape::Linear,
            )
        })
        .collect();
    let program = fader(&table);
    let cell = program.cell(0).unwrap();
    let mut state = CellState::new();
    // Warm: first touches of thread statics.
    let warm = CellEvents::new(
        cell,
        &mut state,
        NodeSpan::new(0, None, 128).unwrap(),
        SeekMode::Exact,
        480,
    );
    let _ = warm.count();

    let mark = current_thread_counters();
    let mut events = 0_usize;
    for index in 0..10_000_i64 {
        let step = (index % 97 == 0).then_some((31, (index * 211) % 200_000));
        let span = NodeSpan::new(128 + index * 128, step, 128).unwrap();
        let mode = if index % 2 == 0 {
            SeekMode::Exact
        } else {
            SeekMode::Ramp
        };
        events += CellEvents::new(cell, &mut state, span, mode, 480).count();
    }
    let delta = current_thread_delta_since(mark);
    assert!(events > 1_000, "the loop must produce events: {events}");
    assert_eq!((delta.allocations, delta.deallocations), (0, 0));
}

/// #1654 K1: the merged walk of a gate and two riding curves over 10,000 blocks, seeks, column
/// jumps, a caller retarget and deferrals all, allocates and frees nothing.
///
/// Red if a block's walk allocates (a per-block buffer of the cursors' events or values instead of
/// the table bound at bind).
#[test]
fn the_merged_walk_allocates_nothing() {
    use automation::{Completion, Composed, CursorKind, Heard, JumpLength, ProgramBuilder};
    assert_installed();
    let mute: Vec<_> = (0..400_u64)
        .map(|k| seg(k * 3_000, k * 3_000 + 1_500, 1.0, 0.0, Shape::Step))
        .collect();
    let ride: Vec<_> = (0..300_u64)
        .map(|k| {
            seg(
                k * 700,
                k * 700 + 700,
                (k % 5) as f32,
                ((k * 3) % 7) as f32,
                Shape::Linear,
            )
        })
        .collect();
    let cell =
        |segments| common::description(segments, 64, JumpLength::Block, Completion::End, false);
    let program = ProgramBuilder::build(&[cell(&mute), cell(&ride), cell(&ride)]).unwrap();
    let mut composed = Composed::<(f32, bool)>::new(
        &program,
        &[
            (0, CursorKind::Gate),
            (1, CursorKind::Value),
            (2, CursorKind::Value),
        ],
        Completion::End,
    )
    .unwrap();
    let compose = |heard: Heard<'_>| (heard.value(1) * heard.value(2), heard.value(0) != 0.0);
    composed.seed(compose);
    for (cursor, jump) in composed.cursors_mut().iter_mut().zip([480, 256, 960]) {
        cursor.set_jump(jump);
    }
    // Warm: first touches of thread statics.
    composed.walk(
        &program,
        NodeSpan::new(0, None, 128).unwrap(),
        SeekMode::Exact,
        compose,
        |_| {},
    );

    let mark = current_thread_counters();
    let mut events = 0_usize;
    for index in 0..10_000_i64 {
        let start = 128 + index * 128;
        if index % 131 == 0 {
            let target = composed.compose_toward(&program, start, 300, compose);
            composed.retarget(target, start, 300);
        }
        if index % 173 == 0 {
            composed.defer_jump(start + 200);
        }
        let step = (index % 97 == 0).then_some((31, (index * 211) % 200_000));
        let span = NodeSpan::new(start, step, 128).unwrap();
        composed.walk(&program, span, SeekMode::Exact, compose, |_| events += 1);
    }
    let delta = current_thread_delta_since(mark);
    assert!(events > 1_000, "the loop must produce events: {events}");
    assert_eq!((delta.allocations, delta.deallocations), (0, 0));
}
