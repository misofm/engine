//! Issue #2199: a session's retained rows, projected before preparation allocates any effect
//! processor, bank, plan, source ring or live-control lane, against what the prepared host reports.
//!
//! `project_host_session` runs the preparation to its projection and stops there. The rows a host
//! admits on (the graph, the builtins, the sources, the effect producers) are the prepared report's
//! own; the control rows (the send lanes, the carry, the state inventory) bound it from above.

use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostPrepareReport, HostProjection,
    HostProjectionRequest, HostShapePolicy, LiveValuesSuperseded, compile_host_session,
    launch_maximum_observation_taps, prepare_host_runtime_with_live_lanes, project_host_session,
};
use lane::Backend;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: 64,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_meter_items: u64::MAX,
        maximum_meter_bytes: u64::MAX,
    }
}

/// The projection of `session` under `lanes` and live controls with `observation_taps`, and the
/// report of its preparation.
fn projected_and_prepared(
    session: &str,
    lanes: HostLiveLanes,
    observation_taps: u32,
) -> (HostProjection, Prepared) {
    let compiled = compile_host_session(session, &caps()).expect("the fixture compiles");
    let live = HostLiveControlRequest {
        meter_window_blocks: 1,
        observation_taps,
        ..HostLiveControlRequest::default()
    };
    let superseded = LiveValuesSuperseded::default();
    let projection = project_host_session(
        &compiled,
        &caps(),
        HostProjectionRequest {
            live_controls: Some(&live),
            selected_meters: None,
            backend: Backend::current(),
            spectrum: None,
            successor: None,
            lanes,
            superseded: Some(&superseded),
        },
    )
    .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let (host, handles) =
        prepare_host_runtime_with_live_lanes(&compiled, &caps(), &live, lanes, &superseded)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let prepared = Prepared {
        report: host.report,
        strip_heap_bytes: handles
            .strip_controls
            .iter()
            .map(host_core::TrackControlProducer::retained_heap_bytes)
            .sum(),
        observation_handles: handles.effect_observations.len(),
        observation_slots: handles
            .effect_controls
            .iter()
            .map(host_core::EffectControlProducer::observe_capacity)
            .sum(),
        #[cfg(feature = "control-provider")]
        catalog: host.control_catalog.resources(),
    };
    (projection, prepared)
}

/// What a prepared host reports, with the rows the control plane reads off its handles.
struct Prepared {
    report: HostPrepareReport,
    strip_heap_bytes: u64,
    observation_handles: usize,
    observation_slots: usize,
    #[cfg(feature = "control-provider")]
    catalog: host_core::CatalogResources,
}

/// For the nine-track fixture and the 64-track console, with every lane and every observation tap
/// and with the fader and matrix lanes alone, the projection's graph, builtins, sources, source control set and effect
/// producer rows are the report's, and its send-lane, carry and inventory rows are at least the
/// report's.
///
/// Test value: red when the projection a host admits a session on (`project_host_session`, the
/// browser boot's and the control plane's) differs from the preparation it projects: a row the
/// projection leaves out or charges at another layout (an effect lane planned at another shape, a
/// source ring left uncharged, a builtin row prepared under other requests) would admit a session
/// whose prepared rows exceed the budget it was admitted on. Preparation refuses its own report's
/// disagreement; this holds the separate projection entry to it.
#[test]
fn a_projection_is_its_preparations_rows() {
    for (name, session) in [
        (
            "nine-track",
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json"),
        ),
        (
            "console",
            include_str!("../../../fixtures/session/v1/console-sixty-four-track.json"),
        ),
    ] {
        for (lanes, taps) in [
            (HostLiveLanes::ALL, launch_maximum_observation_taps()),
            (HostLiveLanes::FADER_AND_MATRIX, 0),
        ] {
            let (projection, prepared) = projected_and_prepared(session, lanes, taps);
            let report = prepared.report;
            assert_eq!(
                (
                    projection.graph_session_plus_plan_bytes,
                    projection.builtin_retained_payload_bytes,
                    projection.source_total_bytes,
                    projection.control_retained_bytes,
                    projection.effect_control_resources,
                    projection.session_model_bytes,
                    projection.strip_heap_bytes,
                    projection.observation_handles,
                    projection.observation_slots,
                ),
                (
                    report.graph_session_plus_plan_bytes,
                    report.builtin_retained_payload_bytes,
                    report.source_total_bytes,
                    report.control_retained_bytes,
                    report.effect_control_resources,
                    report.session_model_bytes,
                    prepared.strip_heap_bytes,
                    prepared.observation_handles,
                    prepared.observation_slots,
                ),
                "{name}, {lanes:?}: the exact rows"
            );
            #[cfg(feature = "control-provider")]
            assert_eq!(
                projection.catalog, prepared.catalog,
                "{name}, {lanes:?}: the provider catalog"
            );
            assert!(
                projection.route_control_bytes >= report.route_control_resources.total_bytes
                    && projection.carry_bytes >= report.carry_program_retained_bytes
                    && projection.inventory_bytes >= report.inventory_retained_bytes,
                "{name}, {lanes:?}: the ceilings, {projection:?} against {report:?}"
            );
        }
    }
}
