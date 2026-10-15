//! The builder: typed refusals, the discontinuity list and the exact byte report.

mod common;

use automation::{BuildError, CellDescription, Completion, JumpLength, ProgramBuilder, Shape};
use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use common::{description, seg};

fn build(segments: &[automation::Segment]) -> Result<automation::Program, BuildError> {
    ProgramBuilder::build(&[description(
        segments,
        64,
        JumpLength::Block,
        Completion::End,
        false,
    )])
}

/// Red if the builder accepts a description that breaks one of the invariants the evaluator and
/// the event rules rely on.
#[test]
fn refusals_are_typed() {
    assert_eq!(build(&[]).unwrap_err(), BuildError::NoSegments { cell: 0 });
    assert_eq!(
        build(&[seg(5, 5, 1.0, 1.0, Shape::Step)]).unwrap_err(),
        BuildError::Segment {
            cell: 0,
            segment: 0
        }
    );
    assert_eq!(
        build(&[seg(0, 1 << 53, 1.0, 1.0, Shape::Step)]).unwrap_err(),
        BuildError::Segment {
            cell: 0,
            segment: 0
        }
    );
    assert_eq!(
        build(&[seg(0, 10, f32::NAN, 1.0, Shape::Linear)]).unwrap_err(),
        BuildError::Segment {
            cell: 0,
            segment: 0
        }
    );
    assert_eq!(
        build(&[
            seg(0, 10, 1.0, 1.0, Shape::Step),
            seg(9, 20, 1.0, 1.0, Shape::Step)
        ])
        .unwrap_err(),
        BuildError::Order {
            cell: 0,
            segment: 1
        }
    );
    assert_eq!(
        build(&[seg(0, 10, 0.0, 1.0, Shape::Exponential)]).unwrap_err(),
        BuildError::NonPositiveExponential {
            cell: 0,
            segment: 0
        }
    );
    assert_eq!(
        build(&[seg(0, 10, 1.0, -1.0, Shape::Exponential)]).unwrap_err(),
        BuildError::NonPositiveExponential {
            cell: 0,
            segment: 0
        }
    );
    let table = [seg(0, 10, 1.0, 1.0, Shape::Step)];
    for (ramp, grid) in [
        (64, 128),
        (64, 0),
        (128, 64),
        (100, 100),
        (300, 300),
        (10, 32),
    ] {
        let cell = CellDescription {
            grid,
            ..description(&table, ramp, JumpLength::Block, Completion::End, false)
        };
        assert_eq!(
            ProgramBuilder::build(&[cell]).unwrap_err(),
            BuildError::Grid { cell: 0 },
            "ramp {ramp} grid {grid}"
        );
    }
}

/// Red if the discontinuity list misses a hold-rule step, invents one at a continuous join, or
/// counts an equal-valued boundary.
#[test]
fn discontinuities_are_the_steps_of_the_hold_rule() {
    let table = [
        seg(100, 200, 1.0, 3.0, Shape::Linear), // first start: nothing before it to differ from
        seg(200, 300, 3.0, 4.0, Shape::Linear), // continuous join
        seg(300, 400, 6.0, 6.0, Shape::Step),   // start 6 differs from the held 4
        seg(400, 500, 6.0, 8.0, Shape::Linear), // step end 6 -> 6 held: no jump; join at 6
        seg(600, 700, 8.0, 8.0, Shape::Step),   // gap holds 8: start equal, end equal
        seg(700, 800, 9.0, 1.0, Shape::Step),   // 8 -> 9 at 700; step end 9 -> 1 at 800
    ];
    let program = build(&table).unwrap();
    let cell = program.cell(0).unwrap();
    assert_eq!(cell.discontinuities(), &[300, 700, 800]);
}

/// Red if `bytes` is not what the build leaves live, or if the build allocates more than once per
/// array.
#[test]
fn bytes_report_equals_the_live_bytes() {
    assert_installed();
    let a = [
        seg(0, 100, 1.0, 2.0, Shape::Step),
        seg(100, 200, 5.0, 5.0, Shape::Linear),
    ];
    let b = [seg(0, 10, 1.0, 1.0, Shape::Step)];
    let cells = [
        description(&a, 64, JumpLength::Block, Completion::End, false),
        description(&b, 128, JumpLength::Fixed(128), Completion::After, true),
    ];
    let reported = ProgramBuilder::bytes(&cells);
    let mark = current_thread_counters();
    let program = ProgramBuilder::build(&cells).unwrap();
    let delta = current_thread_delta_since(mark);
    assert_eq!(delta.requested_bytes - delta.released_bytes, reported);
    assert_eq!(delta.allocations, 3, "one reservation per array");
    assert_eq!(delta.reallocations, 0);
    assert!(reported > 0);
    drop(program);
}

/// Red if the builder accepts two discontinuities closer than the session's 64-sample spacing
/// (#1626 D3), which the per-block event bound rests on, or refuses a pair exactly 64 apart.
#[test]
fn closer_discontinuities_are_refused() {
    let table = |second: u64| {
        [
            seg(0, 100, 1.0, 1.0, Shape::Step),
            seg(100, second, 2.0, 2.0, Shape::Step),
            seg(second, 1000, 3.0, 3.0, Shape::Step),
        ]
    };
    assert_eq!(
        build(&table(163)).unwrap_err(),
        BuildError::Spacing { cell: 0, time: 163 }
    );
    assert_eq!(
        build(&table(164))
            .unwrap()
            .cell(0)
            .unwrap()
            .discontinuities(),
        &[100, 164]
    );
    // A step's own end counts: 2 -> 5 at 150 is 50 after the start at 100.
    assert_eq!(
        build(&[
            seg(0, 100, 1.0, 1.0, Shape::Step),
            seg(100, 150, 2.0, 5.0, Shape::Step),
        ])
        .unwrap_err(),
        BuildError::Spacing { cell: 0, time: 150 }
    );
}
