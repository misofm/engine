//! Preparation and admission gates for the native selected-meter owner.

use builtins::{MeterHandle, MeterMetricSet, MeterTap};
use core::{
    mem::size_of,
    num::{NonZeroU32, NonZeroU64, NonZeroUsize},
};
use graph::{GraphObservationActivationConfig, GraphObservationController};
use host_core::{
    HostConsoleRequest, HostMeterId, HostMeterRequest, HostObservationController,
    HostObservationPreparation, HostPrepareCaps, HostPrepareReport, HostShapePolicy,
    ObservationRefusalReason, ObservationWorkCost, ObservationWorkLimits, PreparedHostMeter,
    SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumChannels,
    SpectrumTarget, compile_host_session, prepare_host_runtime_with_observation_demand,
    prepare_host_runtime_with_observation_demand_between_render_calls,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 100,
        maximum_meter_items: 1_000,
        maximum_meter_bytes: 10_000_000,
    }
}

fn limits() -> ObservationWorkLimits {
    ObservationWorkLimits {
        maximum_active_meter_channels: u64::MAX,
        maximum_meter_samples_per_block: u64::MAX,
        maximum_meter_publications_per_block: u64::MAX,
        maximum_meter_publication_bytes_per_block: u64::MAX,
        maximum_active_spectrum_captures: 0,
        maximum_capture_input_samples_per_block: 0,
        maximum_capture_copy_samples_per_block: 0,
        maximum_capture_publications_per_block: 0,
        maximum_capture_bytes_per_second: 0,
        maximum_transition_entry_visits_per_block: u64::MAX,
        maximum_retained_bytes: u64::MAX,
    }
}

fn console() -> HostConsoleRequest {
    HostConsoleRequest {
        control_queue_depth: NonZeroUsize::new(2),
        meter_period_frames: NonZeroU32::new(128),
        meter_queue_depth: NonZeroUsize::new(2).unwrap(),
        // The explicit catalog is authoritative, including its tap.
        meter_tap: MeterTap::Input,
        master_track: Some(0),
        ..HostConsoleRequest::default()
    }
}

fn meters() -> [HostMeterRequest; 2] {
    ["eq7", "eq1"].map(|track| HostMeterRequest {
        track_id: track.into(),
        tap: MeterTap::PostMatrix,
        metrics: MeterMetricSet::SAMPLE_PEAK,
    })
}

fn demand(meters: &[HostMeterRequest]) -> HostObservationPreparation<'_> {
    HostObservationPreparation {
        meters,
        spectrum: None,
        work_limits: limits(),
        activation: GraphObservationActivationConfig {
            maximum_active_observers: 3,
            maximum_retained_bytes: u64::MAX,
        },
    }
}

fn common_admission_bytes(report: HostPrepareReport) -> u64 {
    let observation = report.observation_demand_resources;
    report.graph_session_plus_plan_bytes
        + report.session_model_bytes
        + report.effect_control_resources.total_bytes().unwrap()
        + observation.owner_inline_bytes
        + observation.metadata_heap_bytes
        + observation
            .graph_activation
            .map_or(0, |graph| graph.retained_bytes - graph.runtime_state_bytes)
}

fn owner_ids(owner: &HostObservationController) -> Vec<HostMeterId> {
    owner.meters().iter().map(|meter| meter.id).collect()
}

#[test]
fn preparation_keeps_explicit_catalog_private_and_initially_idle() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let demand = demand(&requests);
    let mut previous_owner = None;
    for prepare in [
        prepare_host_runtime_with_observation_demand,
        prepare_host_runtime_with_observation_demand_between_render_calls,
    ] {
        builtins::test_only_reset_peak_samples();
        let (host, handles, owner) = prepare(&compiled, &caps(), &console(), &demand).unwrap();
        assert_eq!(builtins::test_only_peak_samples(), 0);
        assert!(!owner.is_closed());
        assert_ne!(previous_owner, Some(owner.owner()));
        previous_owner = Some(owner.owner());
        assert!(handles.meters.is_empty());
        assert!(handles.effect_observations.is_empty());
        assert_eq!(handles.master_track, Some(0));
        assert_eq!(handles.tracks.len(), 9);
        assert_eq!(handles.track_controls.len(), handles.tracks.len());
        assert!(!handles.effect_controls.is_empty());
        for (control, track) in handles.track_controls.iter().zip(&handles.tracks) {
            assert_eq!(&control.track_id, track);
        }
        assert_eq!(owner.meters().len(), requests.len());
        for (index, (actual, requested)) in owner.meters().iter().zip(&requests).enumerate() {
            assert_eq!(actual.id.owner, owner.owner());
            assert_eq!(actual.id.handle.0.get(), index as u64 + 1);
            assert_eq!(actual.track_id, requested.track_id);
            assert_eq!(actual.tap, requested.tap);
            assert_eq!(actual.metrics, requested.metrics);
            assert_eq!(actual.period_frames.get(), 128);
        }
        let resources = host.report.observation_demand_resources;
        let graph = resources.graph_activation.unwrap();
        assert_eq!(graph.maximum_transition_entry_visits_per_block, 12);
        assert_eq!(
            resources.owner_inline_bytes,
            (size_of::<HostObservationController>() - size_of::<GraphObservationController>())
                as u64,
        );
        // Two scalar identities per entry, five complete sets, one contiguous handle scratch.
        let selection_capacity = requests.len();
        let expected_metadata = size_of::<PreparedHostMeter>() * requests.len()
            + requests
                .iter()
                .map(|request| request.track_id.len())
                .sum::<usize>()
            + 5 * selection_capacity * size_of::<(usize, u64)>()
            + selection_capacity * size_of::<u64>();
        assert_eq!(resources.metadata_heap_bytes, expected_metadata as u64);
        assert_eq!(
            resources.reserved_bytes,
            host.report.builtin_meter_payload_bytes
                + graph.retained_bytes
                + resources.owner_inline_bytes
                + resources.metadata_heap_bytes
        );
        assert_eq!(
            owner.work(),
            ObservationWorkCost {
                transition_entry_visits_per_block: graph.maximum_transition_entry_visits_per_block,
                retained_bytes: resources.reserved_bytes,
                ..ObservationWorkCost::ZERO
            }
        );
    }
}

#[test]
fn empty_catalog_ignores_unused_activation_capacity_but_charges_its_owner() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let empty_spectrum = SpectrumCaptureCollectionRequest {
        entries: Vec::new(),
        maximum_capture_bytes: 0,
    };
    let mut demand = demand(&[]);
    demand.spectrum = Some(&empty_spectrum);
    demand.activation.maximum_active_observers = usize::MAX;
    demand.activation.maximum_retained_bytes = 0;
    demand.work_limits.maximum_retained_bytes = 0;
    demand.work_limits.maximum_transition_entry_visits_per_block = 0;
    let (host, handles, owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .unwrap();
    assert!(owner.meters().is_empty());
    assert!(handles.meters.is_empty());
    assert_eq!(owner.work(), ObservationWorkCost::ZERO);
    assert!(!owner.is_closed());
    let resources = host.report.observation_demand_resources;
    assert!(resources.graph_activation.is_none());
    assert_eq!(
        resources.owner_inline_bytes,
        size_of::<HostObservationController>() as u64
    );
    assert_eq!(resources.metadata_heap_bytes, 0);
    assert_eq!(resources.metadata_largest_allocation_bytes, 0);
    assert_eq!(resources.reserved_bytes, 0);
    let mut exact = caps();
    exact.maximum_graph_session_plus_plan_bytes = common_admission_bytes(host.report);
    assert!(
        prepare_host_runtime_with_observation_demand(&compiled, &exact, &console(), &demand)
            .is_ok()
    );
    exact.maximum_graph_session_plus_plan_bytes -= 1;
    assert!(
        prepare_host_runtime_with_observation_demand(&compiled, &exact, &console(), &demand)
            .is_err()
    );
}

#[test]
fn observation_and_common_caps_are_inclusive() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let mut demand = demand(&requests);
    let (host, _, _) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .unwrap();
    let report = host.report;
    let resources = report.observation_demand_resources;
    let graph = resources.graph_activation.unwrap();
    demand.work_limits.maximum_retained_bytes = resources.reserved_bytes;
    demand.work_limits.maximum_transition_entry_visits_per_block =
        graph.maximum_transition_entry_visits_per_block;
    demand.activation.maximum_retained_bytes = graph.retained_bytes;
    let mut exact = caps();
    exact.maximum_graph_session_plus_plan_bytes = common_admission_bytes(report);
    exact.maximum_named_allocation_bytes = report
        .largest_engine_allocation_bytes
        .max(report.session_largest_allocation_bytes)
        .max(report.effect_control_resources.largest_allocation_bytes());
    assert!(
        prepare_host_runtime_with_observation_demand(&compiled, &exact, &console(), &demand)
            .is_ok()
    );
    for field in 0..5 {
        let mut below_caps = exact;
        let mut below_demand = demand;
        match field {
            0 => below_demand.work_limits.maximum_retained_bytes -= 1,
            1 => {
                below_demand
                    .work_limits
                    .maximum_transition_entry_visits_per_block -= 1
            }
            2 => below_demand.activation.maximum_retained_bytes -= 1,
            3 => below_caps.maximum_graph_session_plus_plan_bytes -= 1,
            4 => below_caps.maximum_named_allocation_bytes -= 1,
            _ => unreachable!(),
        }
        assert!(
            prepare_host_runtime_with_observation_demand(
                &compiled,
                &below_caps,
                &console(),
                &below_demand,
            )
            .is_err(),
            "one below cap {field} must refuse"
        );
    }
}

#[test]
fn unsupported_families_are_explicit_refusals() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let mut demand = demand(&requests);
    let mut console = console();
    console.observation_taps = 1;
    let error = prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console, &demand)
        .err()
        .expect("resident demand refused");
    assert!(
        String::from_utf8_lossy(error.as_bytes())
            .contains("host.observation.resident_not_supported")
    );
    console.observation_taps = 0;
    let spectrum = SpectrumCaptureCollectionRequest {
        entries: vec![SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Stereo,
        }],
        maximum_capture_bytes: u64::MAX,
    };
    demand.spectrum = Some(&spectrum);
    let error = prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console, &demand)
        .err()
        .expect("spectrum demand refused");
    assert!(
        String::from_utf8_lossy(error.as_bytes())
            .contains("host.observation.spectrum_not_supported")
    );
}

#[test]
fn paused_admission_accepts_without_render_work_and_preserves_ordinary_backpressure() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let base_demand = demand(&requests);
    let (_host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &base_demand)
            .unwrap();
    let ids = owner_ids(&owner);

    builtins::test_only_reset_peak_samples();
    bench_support::alloc::assert_installed();
    let allocation_mark = bench_support::alloc::current_thread_counters();
    let accepted = owner.replace_meters(&ids).expect("ordinary admission");
    assert_eq!(
        bench_support::alloc::current_thread_delta_since(allocation_mark),
        bench_support::alloc::Counters::default()
    );
    assert_eq!(accepted.revision, 1);
    assert_eq!(accepted.work.active_meter_channels, 4);
    assert_eq!(accepted.work.meter_samples_per_block, 4 * 128);
    assert_eq!(builtins::test_only_peak_samples(), 0);
    assert_eq!(owner.work(), accepted.work);

    let refusal = owner
        .replace_meters(&ids[..1])
        .expect_err("ordinary credit remains pending");
    assert_eq!(refusal.reason, ObservationRefusalReason::Backpressure);
    assert_eq!(owner.work(), accepted.work);
}

#[test]
fn reserved_removal_can_follow_one_ordinary_and_second_removal_is_refused() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let demand = demand(&requests);
    let (_host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .unwrap();
    let ids = owner_ids(&owner);

    let ordinary = owner.replace_meters(&ids).expect("ordinary admission");
    let removal = owner
        .remove_meters_to(&ids[..1])
        .expect("reserved removal may pass ordinary");
    assert_eq!(ordinary.revision, 1);
    assert_eq!(removal.revision, 2);
    assert_eq!(owner.work().active_meter_channels, 2);
    assert_eq!(owner.work().meter_samples_per_block, 2 * 128);

    let refusal = owner
        .remove_meters_to(&[])
        .expect_err("only one removal obligation is reserved");
    assert_eq!(refusal.reason, ObservationRefusalReason::Backpressure);
    let refusal = owner
        .replace_meters(&ids[..1])
        .expect_err("ordinary remains blocked while removal is pending");
    assert_eq!(refusal.reason, ObservationRefusalReason::Backpressure);
    assert_eq!(owner.work().active_meter_channels, 2);
}

#[test]
fn typed_identity_subset_capacity_and_work_refusals_preserve_state() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let base_demand = demand(&requests);

    let (_host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &base_demand)
            .unwrap();
    let ids = owner_ids(&owner);
    let initial_work = owner.work();
    let mut unknown = ids[0];
    unknown.handle = MeterHandle(NonZeroU64::new(99).unwrap());
    assert_eq!(
        owner
            .replace_meters(&[unknown])
            .expect_err("unknown prepared handle")
            .reason,
        ObservationRefusalReason::NotPrepared
    );
    assert_eq!(
        owner
            .replace_meters(&[ids[0], ids[0]])
            .expect_err("duplicate handle")
            .reason,
        ObservationRefusalReason::InvalidRequest
    );
    assert_eq!(owner.work(), initial_work);

    let (_foreign_host, _, foreign_owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &base_demand)
            .unwrap();
    assert_eq!(
        owner
            .replace_meters(&[foreign_owner.meters()[0].id])
            .expect_err("foreign owner handle")
            .reason,
        ObservationRefusalReason::WrongOwner
    );
    assert_eq!(owner.work(), initial_work);

    let ordinary = owner.replace_meters(&ids).expect("establish accepted set");
    let removal = owner
        .remove_meters_to(&[ids[0]])
        .expect("reserved removal follows ordinary");
    assert_eq!(removal.work.active_meter_channels, 2);
    assert_eq!(owner.work().active_meter_channels, 2);

    let (_subset_host, _, mut subset_owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &base_demand)
            .unwrap();
    let subset_ids = owner_ids(&subset_owner);
    subset_owner
        .replace_meters(&subset_ids[..1])
        .expect("subset baseline admission");
    let subset_refusal = subset_owner
        .remove_meters_to(&subset_ids[1..])
        .expect_err("removal must be a subset of the accepted set");
    assert_eq!(subset_refusal.reason, ObservationRefusalReason::Conflict);
    assert_eq!(subset_owner.work().active_meter_channels, 2);

    // A fresh owner keeps each refusal independent of the two occupied publication records.
    let mut limited_demand = demand(&requests);
    limited_demand.activation.maximum_active_observers = 1;
    let (_capacity_host, _, mut capacity_owner) = prepare_host_runtime_with_observation_demand(
        &compiled,
        &caps(),
        &console(),
        &limited_demand,
    )
    .unwrap();
    let capacity_ids = owner_ids(&capacity_owner);
    let capacity_refusal = capacity_owner
        .replace_meters(&capacity_ids)
        .expect_err("controlled capacity");
    assert_eq!(capacity_refusal.reason, ObservationRefusalReason::Capacity);
    assert_eq!(capacity_owner.work().active_meter_channels, 0);
    assert_eq!(capacity_owner.work().transition_entry_visits_per_block, 4);

    let mut limited_demand = demand(&requests);
    limited_demand.work_limits.maximum_active_meter_channels = 2;
    let (_work_host, _, mut work_owner) = prepare_host_runtime_with_observation_demand(
        &compiled,
        &caps(),
        &console(),
        &limited_demand,
    )
    .unwrap();
    let work_ids = owner_ids(&work_owner);
    let work_refusal = work_owner
        .replace_meters(&work_ids)
        .expect_err("meter-channel work budget");
    assert_eq!(work_refusal.reason, ObservationRefusalReason::WorkBudget);
    assert_eq!(work_refusal.limit, Some("maximum_active_meter_channels"));
    assert_eq!(work_owner.work().active_meter_channels, 0);
    assert_eq!(work_owner.work().transition_entry_visits_per_block, 12);

    // The accepted and latest pending records remain the values established before a refused
    // request. The ordinary and removal records are both still occupied here.
    assert_eq!(ordinary.revision, 1);
}
