//! Stored automation is held to the row it names by every host's preparation (#1566 D7): host-core
//! reports `controls::automation_diagnostics` beside the effect diagnostics, so a session that the
//! row rule refuses does not prepare, and the diagnostic is the rule's own.

use host_core::{HostPrepareCaps, HostShapePolicy, prepare_host_session};
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, Owner, ParameterChannel,
    ParameterUnit, Section, SessionModel, StableId, canonical_session_json, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// The fixture with one stored automation on the first track's console EQ, `control` on the left.
fn with_eq_automation(control: u32, unit: ParameterUnit) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.automation = vec![Automation {
        id: id("ride"),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[0].id.clone()),
            section: Section::Console(id("eq")),
            control,
            lane: ParameterChannel::Left,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 960,
            start_value: 1.0,
            end_value: 1.0,
            unit,
        }],
    }];
    model
}

fn prepare(model: &SessionModel) -> Result<(), String> {
    let document = canonical_session_json(model).expect("canonical");
    prepare_host_session(&document, &caps())
        .map(|_| ())
        .map_err(|failure| String::from_utf8_lossy(failure.as_bytes()).into_owned())
}

/// Gate 5 (host-core arm): the same row rule refuses the session with its own code and path, and
/// the block-rate parameter prepares.
///
/// Red if host preparation stops running the row rule (the session prepares and its automation is
/// acked and never heard), or reports a code other than the rule's.
#[test]
fn preparation_refuses_an_automation_the_row_refuses() {
    // `band-1-enabled` is a prepared parameter.
    assert_eq!(
        prepare(&with_eq_automation(1, ParameterUnit::Linear)),
        Err("control.automation.target\t$.automation[id=ride].target\n".to_owned())
    );
    // `band-1-gain`, in the wrong unit: the segment is the fault.
    assert_eq!(
        prepare(&with_eq_automation(4, ParameterUnit::Hz)),
        Err("control.domain\t$.automation[id=ride].segments[0]\n".to_owned())
    );
    assert_eq!(prepare(&with_eq_automation(4, ParameterUnit::Db)), Ok(()));
}
