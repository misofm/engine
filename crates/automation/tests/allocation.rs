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
