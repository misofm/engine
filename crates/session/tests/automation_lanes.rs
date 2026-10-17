//! One renderable curve per automated lane (#1626): the session refuses two entries that drive one
//! lane and two discontinuities of one entry closer than 64 samples, and accepts the shapes next
//! to them.

use session::{
    AutomationSegment, AutomationShape, CompileCaps, DiagnosticCode, ParameterChannel,
    ParameterUnit, Section, SessionModel, StableId, compile_session, parse_session_json,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("test ID")
}

fn fixture() -> SessionModel {
    parse_session_json(CANONICAL).expect("fixture parses")
}

fn segment(
    shape: AutomationShape,
    start: u64,
    end: u64,
    start_value: f32,
    end_value: f32,
) -> AutomationSegment {
    AutomationSegment {
        shape,
        start_sample: start,
        end_sample: end,
        start_value,
        end_value,
        unit: ParameterUnit::Db,
    }
}

/// The fixture with its automation replaced by one fader (`builtins` 5) entry per `(id, lane)`, in
/// the given declared order.
fn fader_rides(rides: &[(&str, ParameterChannel)]) -> SessionModel {
    let mut model = fixture();
    let template = model.automation[0].clone();
    model.automation = rides
        .iter()
        .map(|(name, lane)| {
            let mut entry = template.clone();
            entry.id = id(name);
            entry.target.section = Section::Builtins;
            entry.target.control = 5;
            entry.target.lane = *lane;
            entry
        })
        .collect();
    model
}

/// The fixture with its one entry's segments replaced.
fn with_segments(segments: Vec<AutomationSegment>) -> SessionModel {
    let mut model = fixture();
    model.automation[0].segments = segments;
    model
}

fn refusals(model: &SessionModel) -> Vec<(DiagnosticCode, String)> {
    match compile_session(model, caps()) {
        Ok(_) => Vec::new(),
        Err(error) => error
            .diagnostics()
            .iter()
            .map(|item| (item.code, item.path.to_string()))
            .collect(),
    }
}

fn duplicate_lane(position: usize) -> (DiagnosticCode, String) {
    (
        DiagnosticCode::AutomationDuplicateLane,
        format!("$.automation[{position}].target.lane"),
    )
}

use ParameterChannel::{Both, Left, Right};

/// A `left` and a `right` entry are two lanes; so are two controls of one section, and the same
/// control of two owners' sections.
#[test]
fn distinct_lanes_compile() {
    assert!(refusals(&fader_rides(&[("ride-l", Left), ("ride-r", Right)])).is_empty());

    let mut model = fader_rides(&[("ride-a", Both), ("ride-b", Both)]);
    model.automation[1].target.control = 6;
    assert!(refusals(&model).is_empty());
}

/// The entry with the greater `id` is refused whatever the declared order, and an entry is refused
/// once however many it overlaps.
#[test]
fn the_greater_id_is_refused_in_either_declared_order() {
    let in_order = fader_rides(&[("ride-a", Both), ("ride-b", Left)]);
    assert_eq!(refusals(&in_order), [duplicate_lane(1)]);
    let reversed = fader_rides(&[("ride-b", Left), ("ride-a", Both)]);
    assert_eq!(refusals(&reversed), [duplicate_lane(0)]);

    // `both` overlaps `left` and `right`: it is one refusal, and `left` with `right` is none.
    let overlapping_two = fader_rides(&[("ride-a", Left), ("ride-b", Right), ("ride-c", Both)]);
    assert_eq!(refusals(&overlapping_two), [duplicate_lane(2)]);

    let three = fader_rides(&[("ride-c", Both), ("ride-a", Both), ("ride-b", Both)]);
    assert_eq!(refusals(&three), [duplicate_lane(0), duplicate_lane(2)]);
}

/// An entry that is already refused for another reason is not also reported as a duplicate lane:
/// a repeated `id` is `id.duplicate` alone, and an entry whose target is refused reports that.
#[test]
fn an_entry_already_refused_is_not_reported_twice() {
    let mut repeated = fader_rides(&[("ride-a", Both)]);
    repeated.automation.push(repeated.automation[0].clone());
    assert_eq!(
        refusals(&repeated),
        [(DiagnosticCode::DuplicateId, "$.automation[1].id".to_owned())]
    );

    let mut missing = fader_rides(&[("ride-a", Both), ("ride-b", Both)]);
    missing.automation[1].target.owner = session::Owner::Strip(id("missing-track"));
    assert_eq!(
        refusals(&missing),
        [(
            DiagnosticCode::MissingEntityReference,
            "$.automation[1].target.owner_id".to_owned()
        )]
    );
}

/// Discontinuities exactly 64 samples apart compile; 63 do not.
#[test]
fn discontinuities_at_the_grid_period_compile() {
    let jumps_at = |second: u64| {
        with_segments(vec![
            segment(AutomationShape::Step, 0, 960, 0.0, 1.0),
            segment(AutomationShape::Linear, second, second + 480, 0.5, 0.0),
        ])
    };
    assert!(refusals(&jumps_at(1024)).is_empty());
    assert_eq!(
        refusals(&jumps_at(1023)),
        [(
            DiagnosticCode::AutomationDiscontinuitySpacing,
            "$.automation[0].segments[1].start_sample".to_owned()
        )]
    );
}

/// Only a jump counts: a segment that starts at the value the lane holds, a `step` whose two
/// values are equal, and a step end met by a segment starting at once are not extra jumps.
#[test]
fn a_continuous_boundary_is_not_a_discontinuity() {
    // The ramp starts at the step's end value, held across the gap: one jump (at 960), then the
    // ramp's end holds into a segment that starts 1 sample later at the held value.
    let continuous = with_segments(vec![
        segment(AutomationShape::Step, 0, 960, 0.0, 1.0),
        segment(AutomationShape::Linear, 1000, 1010, 1.0, 0.25),
        segment(AutomationShape::Linear, 1011, 1500, 0.25, 0.0),
    ]);
    assert!(refusals(&continuous).is_empty());

    // A step with equal values never jumps, even when stacked tightly.
    let equal = with_segments(vec![
        segment(AutomationShape::Step, 0, 10, 0.5, 0.5),
        segment(AutomationShape::Step, 10, 20, 0.5, 0.5),
        segment(AutomationShape::Step, 20, 30, 0.5, 0.5),
    ]);
    assert!(refusals(&equal).is_empty());

    // A step whose end a segment meets at once is one boundary at that sample: the lane jumps
    // 0 to 0.5 at 100, once, and the next jump is far enough away.
    let one_boundary = with_segments(vec![
        segment(AutomationShape::Step, 0, 100, 0.0, 1.0),
        segment(AutomationShape::Linear, 100, 200, 0.5, 0.5),
        segment(AutomationShape::Linear, 300, 400, 0.0, 0.0),
    ]);
    assert!(refusals(&one_boundary).is_empty());

    // The same step end met by a segment that starts at the value the step held: no jump at 100,
    // so a jump 10 samples later is the first and only one.
    let held_across = with_segments(vec![
        segment(AutomationShape::Step, 0, 100, 0.0, 1.0),
        segment(AutomationShape::Step, 100, 110, 0.0, 0.0),
        segment(AutomationShape::Linear, 111, 200, 0.7, 0.0),
    ]);
    assert!(refusals(&held_across).is_empty());
}

/// Jumps compare by bits (`0.0` against `-0.0`), and every pair closer than 64 is named.
#[test]
fn close_jumps_are_named_per_pair() {
    let signed_zero = with_segments(vec![
        segment(AutomationShape::Linear, 0, 10, 0.0, 0.0),
        segment(AutomationShape::Linear, 20, 30, -0.0, -0.0),
        segment(AutomationShape::Linear, 40, 50, 0.0, 0.0),
        segment(AutomationShape::Linear, 60, 70, -0.0, -0.0),
    ]);
    // Jumps at 20, 40 and 60: the later one of each close pair is named, the first is not.
    assert_eq!(
        refusals(&signed_zero),
        [
            (
                DiagnosticCode::AutomationDiscontinuitySpacing,
                "$.automation[0].segments[2].start_sample".to_owned()
            ),
            (
                DiagnosticCode::AutomationDiscontinuitySpacing,
                "$.automation[0].segments[3].start_sample".to_owned()
            ),
        ]
    );

    // A step's end is a jump too, and names its own `end_sample`.
    let step_end = with_segments(vec![
        segment(AutomationShape::Linear, 0, 10, 0.0, 0.0),
        segment(AutomationShape::Linear, 20, 30, 1.0, 1.0),
        segment(AutomationShape::Step, 40, 50, 1.0, 0.0),
    ]);
    assert_eq!(
        refusals(&step_end),
        [(
            DiagnosticCode::AutomationDiscontinuitySpacing,
            "$.automation[0].segments[2].end_sample".to_owned()
        )]
    );
}

/// The spacing rule reads only a well-formed sequence: out-of-order segments report their order
/// and nothing more.
#[test]
fn spacing_never_reports_on_a_malformed_sequence() {
    let malformed = with_segments(vec![
        segment(AutomationShape::Linear, 500, 600, 0.0, 1.0),
        segment(AutomationShape::Linear, 100, 200, 0.0, 1.0),
    ]);
    assert_eq!(
        refusals(&malformed),
        [(
            DiagnosticCode::AutomationOutOfOrder,
            "$.automation[0].segments[1].start_sample".to_owned()
        )]
    );
}

/// The segment time bound (`automation::TIME_LIMIT`, `2^53`) is the validator's whatever the row:
/// an insert row (not rendered today) and the fader row (rendered, #1646) are refused the same way
/// at the same path, and `2^53 - 1` is accepted on both. Red if the bound is made to depend on the
/// row, which would let a validated fader session reach the program builder's refusal.
#[test]
fn the_segment_time_bound_does_not_depend_on_the_row() {
    let limit = automation::TIME_LIMIT;
    let end_path = "$.automation[0].segments[0].end_sample".to_owned();
    let models: [fn(Vec<AutomationSegment>) -> SessionModel; 2] = [with_segments, |segments| {
        let mut model = fader_rides(&[("ride", ParameterChannel::Both)]);
        model.automation[0].segments = segments;
        model
    }];
    for build in models {
        let accepted = build(vec![segment(
            AutomationShape::Step,
            limit - 2,
            limit - 1,
            0.0,
            0.0,
        )]);
        assert_eq!(refusals(&accepted), Vec::new());
        let refused = build(vec![segment(AutomationShape::Step, 0, limit, 0.0, 0.0)]);
        assert_eq!(
            refusals(&refused),
            vec![(DiagnosticCode::NumericOutOfSchemaRange, end_path.clone())]
        );
    }
}
