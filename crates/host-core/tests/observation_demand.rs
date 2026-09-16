//! Preparation and admission gates for the native selected-meter owner.

use builtins::{MeterHandle, MeterMetricSet, MeterTap};
use core::{
    mem::size_of,
    num::{NonZeroU32, NonZeroU64, NonZeroUsize},
};
use graph::{GraphObservationActivationConfig, GraphObservationController};
use host_core::{
    HostConsoleRequest, HostMeterId, HostMeterRequest, HostObservationController,
    HostObservationPreparation, HostObservationPreparationConfig, HostPrepareCaps,
    HostPrepareReport, HostShapePolicy, HostSpectrumDemand, HostSpectrumMode,
    HostSpectrumReadError, ObservationRefusalReason, ObservationWorkCost, ObservationWorkLimits,
    PreparedHostMeter, SPECTRUM_WINDOW_FRAMES, SourceSubmission, SpectrumCaptureCollectionEntry,
    SpectrumCaptureCollectionRequest, SpectrumCaptureRequest, SpectrumChannels, SpectrumHop,
    SpectrumTarget, StartedRenderSession, compile_host_session, prepare_host_runtime,
    prepare_host_runtime_with_observation_demand,
    prepare_host_runtime_with_observation_demand_between_render_calls,
    prepare_host_runtime_with_observation_demand_between_render_calls_config,
    prepare_host_runtime_with_observation_demand_config, prepare_host_runtime_with_spectrum,
    spectrum_capture_resources_for,
};
#[cfg(feature = "test-support")]
use host_core::{
    SpectrumOperationCounts, test_only_reset_spectrum_operation_counts,
    test_only_spectrum_operation_counts,
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
        maximum_active_spectrum_captures: u64::MAX,
        maximum_capture_input_samples_per_block: u64::MAX,
        maximum_capture_copy_samples_per_block: u64::MAX,
        maximum_capture_publications_per_block: u64::MAX,
        maximum_capture_bytes_per_second: u64::MAX,
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

fn all_meters() -> Vec<HostMeterRequest> {
    (0..9)
        .map(|index| HostMeterRequest {
            track_id: format!("eq{index}").into(),
            tap: MeterTap::PostMatrix,
            metrics: MeterMetricSet::SAMPLE_PEAK,
        })
        .collect()
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

fn all_demand(meters: &[HostMeterRequest]) -> HostObservationPreparation<'_> {
    let mut demand = demand(meters);
    demand.activation.maximum_active_observers = meters.len();
    demand
}

fn common_admission_bytes(report: HostPrepareReport) -> u64 {
    let observation = report.observation_demand_resources;
    report.graph_session_plus_plan_bytes
        + report.session_model_bytes
        + report.effect_control_resources.total_bytes().unwrap()
        + report.spectrum_capture_retained_bytes
        + observation.owner_inline_bytes
        + observation.metadata_heap_bytes
        + observation
            .graph_activation
            .map_or(0, |graph| graph.retained_bytes - graph.runtime_state_bytes)
}

fn owner_ids(owner: &HostObservationController) -> Vec<HostMeterId> {
    owner.meters().iter().map(|meter| meter.id).collect()
}

fn render_source_block(
    render: &mut StartedRenderSession,
    sources: &mut host_core::SourceControlSet,
    block: usize,
) -> [f32; 128 * 2] {
    const QUANTUM: usize = 128;
    let left = [0.25_f32; QUANTUM];
    let right = [-0.25_f32; QUANTUM];
    sources
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: (block * QUANTUM) as u64,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: QUANTUM as u32,
                end_of_region: false,
            },
        )
        .expect("source block");
    let mut output = [0.0_f32; QUANTUM * 2];
    let _ = render
        .render_planar(&mut output, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
        .expect("render block");
    output
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
fn resident_observation_is_refused_and_controlled_spectrum_is_prepared() {
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
    let (host, handles, owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console, &demand)
            .expect("controlled spectrum demand is prepared");
    assert_eq!(owner.meters().len(), requests.len());
    assert!(handles.meters.is_empty());
    assert!(handles.effect_observations.is_empty());
    assert!(
        host.report
            .observation_demand_resources
            .graph_activation
            .is_some()
    );
    assert!(host.report.spectrum_capture_retained_bytes > 0);
    assert_eq!(
        owner.work(),
        ObservationWorkCost {
            retained_bytes: host.report.observation_demand_resources.reserved_bytes,
            transition_entry_visits_per_block: host
                .report
                .observation_demand_resources
                .graph_activation
                .expect("spectrum graph activation")
                .maximum_transition_entry_visits_per_block,
            ..ObservationWorkCost::ZERO
        }
    );
}

#[test]
fn controlled_spectrum_resource_cap_is_inclusive_and_prepares_exact_targets() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let spectrum = SpectrumCaptureCollectionRequest {
        entries: vec![SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Stereo,
        }],
        maximum_capture_bytes: u64::MAX,
    };
    let mut demand = demand(&[]);
    demand.spectrum = Some(&spectrum);
    prepare_host_runtime_with_observation_demand_between_render_calls(
        &compiled,
        &caps(),
        &console(),
        &demand,
    )
    .expect("serialized host prepares the same controlled spectrum catalog");
    let (host, handles, owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .expect("spectrum catalog");
    assert!(owner.meters().is_empty());
    assert!(handles.meters.is_empty());
    assert!(host.report.spectrum_capture_retained_bytes > 0);
    let retained = host.report.spectrum_capture_retained_bytes;
    let mut exact_caps = caps();
    exact_caps.maximum_graph_session_plus_plan_bytes = common_admission_bytes(host.report);
    prepare_host_runtime_with_observation_demand(&compiled, &exact_caps, &console(), &demand)
        .expect("common graph admission is inclusive");
    exact_caps.maximum_graph_session_plus_plan_bytes -= 1;
    assert!(
        prepare_host_runtime_with_observation_demand(&compiled, &exact_caps, &console(), &demand)
            .is_err(),
        "one below common graph admission"
    );

    let mut exact_spectrum = spectrum.clone();
    exact_spectrum.maximum_capture_bytes = retained;
    demand.spectrum = Some(&exact_spectrum);
    prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
        .expect("exact paired capture budget is inclusive");
    let mut below_spectrum = exact_spectrum.clone();
    below_spectrum.maximum_capture_bytes = retained - 1;
    demand.spectrum = Some(&below_spectrum);
    let refusal =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .err()
            .expect("one below paired capture budget");
    assert!(String::from_utf8_lossy(refusal.as_bytes()).contains("host.spectrum.capture_budget"));

    let explicit_hop = SpectrumHop::new(256).expect("supported explicit hop");
    demand.spectrum = Some(&exact_spectrum);
    let serialized_preparation =
        HostObservationPreparationConfig::new(demand).with_spectrum_hop(explicit_hop);
    let (serialized_host, _, serialized_owner) =
        prepare_host_runtime_with_observation_demand_between_render_calls_config(
            &compiled,
            &caps(),
            &console(),
            &serialized_preparation,
        )
        .expect("serialized explicit cadence keeps paired storage accounting");
    assert_eq!(
        serialized_owner
            .spectrum_cadence()
            .expect("serialized explicit cadence")
            .hop_frames(),
        explicit_hop.get()
    );
    drop(serialized_host);
    let explicit_preparation =
        HostObservationPreparationConfig::new(demand).with_spectrum_hop(explicit_hop);
    let (explicit_host, _, explicit_owner) = prepare_host_runtime_with_observation_demand_config(
        &compiled,
        &caps(),
        &console(),
        &explicit_preparation,
    )
    .expect("explicit cadence keeps paired storage accounting");
    assert_eq!(
        explicit_host.report.spectrum_capture_retained_bytes,
        retained
    );
    assert_eq!(
        explicit_owner
            .spectrum_cadence()
            .expect("explicit cadence")
            .hop_frames(),
        explicit_hop.get()
    );
    assert!(
        prepare_host_runtime_with_observation_demand_config(
            &compiled,
            &caps(),
            &console(),
            &explicit_preparation,
        )
        .is_ok(),
        "explicit cadence does not alter retained storage"
    );
    demand.spectrum = Some(&below_spectrum);
    let below_preparation =
        HostObservationPreparationConfig::new(demand).with_spectrum_hop(explicit_hop);
    assert!(
        prepare_host_runtime_with_observation_demand_config(
            &compiled,
            &caps(),
            &console(),
            &below_preparation,
        )
        .is_err(),
        "one below paired storage budget refuses with explicit cadence"
    );
}

#[test]
fn spectrum_catalog_with_zero_capture_limit_is_prepared_but_dormant() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let spectrum = SpectrumCaptureCollectionRequest {
        entries: vec![SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Left,
        }],
        maximum_capture_bytes: u64::MAX,
    };
    let mut demand = demand(&[]);
    demand.spectrum = Some(&spectrum);
    demand.work_limits.maximum_active_spectrum_captures = 0;
    let (host, _handles, owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .expect("zero active capacity still prepares storage");
    assert!(
        host.report
            .observation_demand_resources
            .graph_activation
            .is_some()
    );
    assert_eq!(owner.work().active_spectrum_captures, 0);
    assert_eq!(owner.work().capture_input_samples_per_block, 0);
    assert_eq!(owner.work().capture_copy_samples_per_block, 0);
    assert_eq!(owner.work().capture_publications_per_block, 0);
    assert_eq!(owner.work().capture_bytes_per_second, 0);
}

#[cfg(feature = "test-support")]
#[test]
fn dormant_controlled_spectrum_does_no_capture_work_on_render() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let spectrum = SpectrumCaptureCollectionRequest {
        entries: vec![SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Stereo,
        }],
        maximum_capture_bytes: u64::MAX,
    };
    let mut demand = demand(&[]);
    demand.spectrum = Some(&spectrum);
    let (host, _handles, _owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .expect("controlled spectrum catalog");
    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    test_only_reset_spectrum_operation_counts();
    let _ = render_source_block(&mut render, &mut sources, 0);
    assert_eq!(
        test_only_spectrum_operation_counts(),
        SpectrumOperationCounts::default()
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

#[test]
fn receipts_are_real_bounded_and_stop_reuses_pending_empty_publication() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let demand = demand(&requests);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .unwrap();
    let ids = owner_ids(&owner);

    assert_eq!(owner.stop_all(), Ok(host_core::ObservationStop::Quiescent));
    assert_eq!(owner.try_applied(), None);
    let ordinary = owner.replace_meters(&ids[..1]).expect("ordinary admission");
    assert_eq!(
        owner.try_applied(),
        None,
        "paused owner cannot fabricate a receipt"
    );
    let removal = owner.stop_all().expect("reserved stop");
    let removal = match removal {
        host_core::ObservationStop::Pending(accepted) => accepted,
        host_core::ObservationStop::Quiescent => panic!("stop must publish removal"),
    };
    assert_eq!(removal.revision, ordinary.revision + 1);
    assert_eq!(
        owner.stop_all(),
        Ok(host_core::ObservationStop::Pending(removal))
    );

    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    builtins::test_only_reset_peak_samples();
    render_source_block(&mut render, &mut sources, 0);
    assert_eq!(builtins::test_only_peak_samples(), 0);
    let first = owner
        .try_applied()
        .expect("ordinary receipt at the first render boundary");
    let second = owner
        .try_applied()
        .expect("removal receipt at the same render boundary");
    assert_eq!(first.revision, ordinary.revision);
    assert_eq!(second.revision, removal.revision);
    assert_eq!(first.first_sample, 0);
    assert_eq!(second.first_sample, 0);
    assert_eq!(owner.try_applied(), None);
    assert_eq!(owner.stop_all(), Ok(host_core::ObservationStop::Quiescent));
}

#[test]
fn real_render_subset_removal_keeps_sibling_work_and_final_stop_is_quiescent() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let demand = demand(&requests);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .unwrap();
    let ids = owner_ids(&owner);
    let initial = owner.replace_meters(&ids).expect("select both meters");
    let (mut render, mut sources, _) = host.start_render_session().unwrap();

    builtins::test_only_reset_peak_samples();
    render_source_block(&mut render, &mut sources, 0);
    assert_eq!(
        owner.try_applied().expect("initial receipt").revision,
        initial.revision
    );
    assert_eq!(builtins::test_only_peak_samples(), 2 * 2 * 128);

    let subset = owner
        .remove_meters_to(&ids[..1])
        .expect("remove one meter and preserve its sibling");
    builtins::test_only_reset_peak_samples();
    render_source_block(&mut render, &mut sources, 1);
    let subset_applied = owner.try_applied().expect("subset receipt");
    assert_eq!(subset_applied.revision, subset.revision);
    assert_eq!(subset_applied.first_sample, 128);
    assert_eq!(builtins::test_only_peak_samples(), 2 * 128);

    let final_removal = owner.stop_all().expect("stop remaining meter");
    let final_removal = match final_removal {
        host_core::ObservationStop::Pending(accepted) => accepted,
        host_core::ObservationStop::Quiescent => panic!("active sibling requires removal"),
    };
    builtins::test_only_reset_peak_samples();
    render_source_block(&mut render, &mut sources, 2);
    let final_applied = owner.try_applied().expect("final removal receipt");
    assert_eq!(final_applied.revision, final_removal.revision);
    assert_eq!(final_applied.first_sample, 256);
    assert_eq!(builtins::test_only_peak_samples(), 0);
    assert_eq!(owner.stop_all(), Ok(host_core::ObservationStop::Quiescent));
}

#[test]
fn reads_fence_state_without_draining_and_keep_unchanged_siblings_live() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let base_demand = demand(&requests);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &base_demand)
            .unwrap();
    let ids = owner_ids(&owner);
    let (_foreign_host, _, foreign_owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &base_demand)
            .unwrap();
    let foreign = foreign_owner.meters()[0].id;
    let mut unknown = ids[0];
    unknown.handle = MeterHandle(NonZeroU64::new(99).expect("unknown handle"));

    let first = owner
        .replace_meters(&ids[..1])
        .expect("first meter admission");
    assert_eq!(
        owner.try_read_meter(ids[0]),
        Err(host_core::ObservationReadError::PendingApplication)
    );
    assert_eq!(
        owner.try_read_meter(ids[1]),
        Err(host_core::ObservationReadError::Inactive)
    );
    assert_eq!(
        owner.try_read_meter(foreign),
        Err(host_core::ObservationReadError::WrongOwner)
    );
    assert_eq!(
        owner.try_read_meter(unknown),
        Err(host_core::ObservationReadError::NotPrepared)
    );

    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    builtins::test_only_reset_peak_samples();
    let _ = render_source_block(&mut render, &mut sources, 0);
    assert_eq!(builtins::test_only_peak_samples(), 2 * 128);
    // The first block has published a real queue record, but its application receipt has not been
    // reconciled yet. Every state error still leaves that record untouched.
    assert_eq!(
        owner.try_read_meter(ids[0]),
        Err(host_core::ObservationReadError::PendingApplication)
    );
    assert_eq!(
        owner.try_read_meter(ids[1]),
        Err(host_core::ObservationReadError::Inactive)
    );
    assert_eq!(
        owner.try_read_meter(foreign),
        Err(host_core::ObservationReadError::WrongOwner)
    );
    assert_eq!(
        owner.try_read_meter(unknown),
        Err(host_core::ObservationReadError::NotPrepared)
    );
    assert_eq!(
        owner.try_applied().expect("first receipt").revision,
        first.revision
    );

    // The sibling admission leaves ids[0]'s applied generation live while ids[1] waits for the
    // next boundary. The refusal below must preserve that still-readable queue entry.
    let sibling = owner.replace_meters(&ids).expect("sibling admission");
    assert_eq!(
        owner.try_read_meter(ids[1]),
        Err(host_core::ObservationReadError::PendingApplication)
    );
    assert_eq!(
        owner
            .replace_meters(&ids[..1])
            .expect_err("ordinary credit is occupied")
            .reason,
        ObservationRefusalReason::Backpressure
    );
    let previous = owner
        .try_read_meter(ids[0])
        .expect("unchanged sibling stays readable")
        .expect("first live window");
    assert_eq!(previous.snapshot.observation_generation, first.revision);
    assert_eq!(previous.snapshot.start_sample, 0);

    builtins::test_only_reset_peak_samples();
    let _ = render_source_block(&mut render, &mut sources, 1);
    assert_eq!(builtins::test_only_peak_samples(), 2 * 2 * 128);
    assert_eq!(
        owner.try_applied().expect("sibling receipt").revision,
        sibling.revision
    );
    let fresh = owner
        .try_read_meter(ids[1])
        .expect("fresh meter read")
        .expect("fresh activation window");
    assert_eq!(fresh.meter, ids[1]);
    assert_eq!(fresh.snapshot.observation_generation, sibling.revision);
    assert_eq!(fresh.snapshot.start_sample, 128);
    assert_eq!(owner.try_read_meter(ids[1]), Ok(None));
}

#[test]
fn selected_meter_churn_keeps_nine_track_bank_tail_pcm_bit_identical() {
    const BLOCKS: usize = 5;
    let compiled = compile_host_session(SESSION, &caps()).unwrap();

    let baseline = prepare_host_runtime(&compiled, &caps()).expect("baseline host");
    let (mut baseline_render, mut baseline_sources, _) =
        baseline.start_render_session().expect("baseline render");
    let baseline_outputs = (0..BLOCKS)
        .map(|block| render_source_block(&mut baseline_render, &mut baseline_sources, block))
        .collect::<Vec<_>>();
    assert!(
        baseline_outputs
            .iter()
            .flatten()
            .any(|sample| *sample != 0.0),
        "fixture must exercise nonzero PCM"
    );

    let requests = all_meters();
    let observation = all_demand(&requests);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("nine-track selected owner");
    let ids = owner_ids(&owner);
    let selections = [
        None,
        Some(vec![0]),
        Some((0..9).collect::<Vec<_>>()),
        Some(vec![8]),
        Some(Vec::new()),
    ];
    let expected_selected = [0, 1, 9, 1, 0];
    let (mut render, mut sources, _) = host.start_render_session().expect("selected render");
    for (block, (selection, selected)) in selections.iter().zip(expected_selected).enumerate() {
        if let Some(selection) = selection {
            let handles = selection
                .iter()
                .map(|index| ids[*index])
                .collect::<Vec<_>>();
            owner.replace_meters(&handles).expect("churn admission");
        }
        builtins::test_only_reset_peak_samples();
        let output = render_source_block(&mut render, &mut sources, block);
        assert_eq!(
            output.map(f32::to_bits),
            baseline_outputs[block].map(f32::to_bits),
            "selected observation changed PCM at block {block}"
        );
        assert_eq!(
            builtins::test_only_peak_samples(),
            2 * selected * 128,
            "selected meter sample sites at block {block}"
        );
        if selection.is_some() {
            let applied = owner.try_applied().expect("churn receipt");
            assert_eq!(applied.first_sample, (block * 128) as u64);
        }
    }
    assert_eq!(owner.stop_all(), Ok(host_core::ObservationStop::Quiescent));
}

#[test]
fn stalled_reader_drains_only_the_entry_bounded_queue() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let observation = demand(&requests);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .unwrap();
    let ids = owner_ids(&owner);
    let accepted = owner.replace_meters(&ids[..1]).expect("meter admission");
    let (mut render, mut sources, _) = host.start_render_session().unwrap();

    for block in 0..3 {
        builtins::test_only_reset_peak_samples();
        let _ = render_source_block(&mut render, &mut sources, block);
        assert_eq!(builtins::test_only_peak_samples(), 2 * 128);
        if block == 0 {
            assert_eq!(
                owner.try_applied().expect("first receipt").revision,
                accepted.revision
            );
        }
    }

    // The configured queue has two slots. The third producer window is dropped by the engine;
    // this read may pop only the two records visible at entry and returns the newest of them.
    let newest = owner
        .try_read_meter(ids[0])
        .expect("bounded read")
        .expect("queued windows");
    assert_eq!(newest.snapshot.start_sample, 128);
    assert_eq!(newest.snapshot.window_sequence, 1);
    assert_eq!(owner.try_read_meter(ids[0]), Ok(None));

    builtins::test_only_reset_peak_samples();
    let _ = render_source_block(&mut render, &mut sources, 3);
    assert_eq!(builtins::test_only_peak_samples(), 2 * 128);
    let delivered_after_drop = owner
        .try_read_meter(ids[0])
        .expect("post-drop read")
        .expect("next bounded window");
    assert_eq!(delivered_after_drop.snapshot.start_sample, 3 * 128);
    assert_eq!(delivered_after_drop.snapshot.window_sequence, 3);
    assert_eq!(
        delivered_after_drop.snapshot.cumulative_dropped_snapshots,
        1
    );

    let removal = match owner.stop_all().expect("stop active meter") {
        host_core::ObservationStop::Pending(accepted) => accepted,
        host_core::ObservationStop::Quiescent => panic!("active meter requires removal"),
    };
    builtins::test_only_reset_peak_samples();
    let _ = render_source_block(&mut render, &mut sources, 4);
    assert_eq!(builtins::test_only_peak_samples(), 0);
    assert_eq!(
        owner.try_applied().expect("removal receipt").revision,
        removal.revision
    );
    assert_eq!(owner.stop_all(), Ok(host_core::ObservationStop::Quiescent));

    let reactivated = owner.replace_meters(&ids[..1]).expect("fresh activation");
    builtins::test_only_reset_peak_samples();
    let _ = render_source_block(&mut render, &mut sources, 5);
    assert_eq!(builtins::test_only_peak_samples(), 2 * 128);
    assert_eq!(
        owner.try_applied().expect("reactivation receipt").revision,
        reactivated.revision
    );
    let fresh = owner
        .try_read_meter(ids[0])
        .expect("fresh generation read")
        .expect("fresh generation window");
    assert_eq!(fresh.snapshot.observation_generation, reactivated.revision);
    assert_eq!(fresh.snapshot.start_sample, 5 * 128);
    assert_eq!(fresh.snapshot.window_sequence, 4);
    assert_eq!(fresh.snapshot.cumulative_dropped_snapshots, 1);
}

#[test]
fn dropping_the_renderer_closes_pending_observation_without_a_fake_receipt() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let demand = demand(&requests);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &demand)
            .unwrap();
    let ids = owner_ids(&owner);
    let accepted = owner.replace_meters(&ids[..1]).expect("ordinary admission");
    let (render, _sources, _) = host.start_render_session().unwrap();
    drop(render);

    assert!(owner.is_closed());
    assert_eq!(owner.try_applied(), None);
    assert_eq!(
        owner.try_read_meter(ids[0]),
        Err(host_core::ObservationReadError::Closed)
    );
    assert_eq!(
        owner.stop_all(),
        Err(host_core::ObservationRefusal {
            reason: ObservationRefusalReason::Closed,
            limit: None,
            requested: None,
            maximum: None,
        })
    );
    assert_eq!(accepted.revision, 1);
}

fn controlled_spectrum_request(
    entries: Vec<SpectrumCaptureCollectionEntry>,
) -> SpectrumCaptureCollectionRequest {
    SpectrumCaptureCollectionRequest {
        entries,
        maximum_capture_bytes: u64::MAX,
    }
}

#[test]
fn protected_explicit_hops_share_one_cadence_and_fence_replacement_epochs() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let request = controlled_spectrum_request(vec![
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Stereo,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
        },
    ]);

    for hop in SpectrumHop::SUPPORTED {
        let mut observation = demand(&[]);
        observation.spectrum = Some(&request);
        let preparation = HostObservationPreparationConfig::new(observation).with_spectrum_hop(hop);
        let (host, _, mut owner) = prepare_host_runtime_with_observation_demand_config(
            &compiled,
            &caps(),
            &console(),
            &preparation,
        )
        .expect("explicit protected cadence prepares");
        assert_eq!(
            owner.spectrum_cadence().map(|cadence| cadence.hop_frames()),
            Some(hop.get())
        );

        let target_a = HostSpectrumDemand {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Stereo,
            mode: HostSpectrumMode::Continuous,
        };
        let target_b = HostSpectrumDemand {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
            mode: HostSpectrumMode::Continuous,
        };
        let first = owner
            .replace_spectrum(&target_a)
            .expect("first explicit spectrum admission");
        let (mut render, mut sources, _) = host.start_render_session().unwrap();
        let initial_blocks =
            (SPECTRUM_WINDOW_FRAMES as u64 + 2 * u64::from(hop.get())).div_ceil(128) as usize;
        let mut first_windows = Vec::new();
        let mut first_applied = false;
        for block in 0..initial_blocks {
            render_source_block(&mut render, &mut sources, block);
            if !first_applied && let Some(receipt) = owner.try_applied() {
                assert_eq!(receipt.revision, first.revision);
                first_applied = true;
            }
            match owner.try_read_continuous_spectrum() {
                Ok(window) => first_windows.push(window),
                Err(HostSpectrumReadError::Pending | HostSpectrumReadError::Warming) => {}
                Err(error) => panic!("unexpected initial explicit read: {error:?}"),
            }
        }
        assert!(first_applied);
        assert!(first_windows.len() >= 3);
        for (sequence, observed) in first_windows.iter().take(3).enumerate() {
            let first_sample = sequence as u64 * u64::from(hop.get());
            assert_eq!(observed.observation_generation, first.revision);
            assert_eq!(observed.selection_epoch, 1);
            assert_eq!(observed.window.stream_epoch, 1);
            assert_eq!(observed.window.sequence, sequence as u64);
            assert_eq!(observed.window.first_sample, first_sample);
            assert_eq!(
                observed.window.end_sample(),
                Some(first_sample + SPECTRUM_WINDOW_FRAMES as u64)
            );
        }

        let replacement = owner
            .replace_spectrum(&target_b)
            .expect("explicit target replacement admission");
        assert_eq!(replacement.revision, first.revision + 1);
        assert_eq!(replacement.work, first.work);
        assert_eq!(owner.spectrum_cadence().unwrap().hop_frames(), hop.get());
        assert_eq!(owner.spectrum_state().selection_epoch, 2);
        assert_eq!(
            owner.try_read_continuous_spectrum().err(),
            Some(HostSpectrumReadError::PendingApplication)
        );

        let replacement_start_block = initial_blocks;
        let mut replacement_first_sample = None;
        let mut replacement_window = None;
        let mut replacement_applied = false;
        for block in replacement_start_block..replacement_start_block + 16 {
            render_source_block(&mut render, &mut sources, block);
            if !replacement_applied && let Some(receipt) = owner.try_applied() {
                assert_eq!(receipt.revision, replacement.revision);
                replacement_first_sample = Some(receipt.first_sample);
                replacement_applied = true;
            }
            match owner.try_read_continuous_spectrum() {
                Ok(window) => replacement_window = Some(window),
                Err(HostSpectrumReadError::Pending | HostSpectrumReadError::Warming) => {}
                Err(error) => panic!("unexpected replacement read: {error:?}"),
            }
        }
        let replacement_window = replacement_window.expect("replacement window");
        assert!(replacement_applied);
        assert_eq!(
            replacement_window.observation_generation,
            replacement.revision
        );
        assert_eq!(replacement_window.selection_epoch, 2);
        assert_eq!(replacement_window.window.stream_epoch, 1);
        assert_eq!(replacement_window.window.sequence, 0);
        assert_eq!(
            replacement_window.window.first_sample,
            replacement_first_sample.expect("replacement application sample")
        );
        assert_eq!(
            replacement_window.window.end_sample(),
            Some(replacement_window.window.first_sample + SPECTRUM_WINDOW_FRAMES as u64)
        );

        let restart = owner
            .restart_spectrum()
            .expect("explicit restart admission");
        assert_eq!(restart.revision, replacement.revision + 1);
        assert_eq!(restart.work, replacement.work);
        assert_eq!(owner.spectrum_cadence().unwrap().hop_frames(), hop.get());
        assert_eq!(owner.spectrum_state().selection_epoch, 2);
        let restart_start_block = replacement_start_block + 16;
        let mut restart_first_sample = None;
        let mut restart_window = None;
        let mut restart_applied = false;
        for block in restart_start_block..restart_start_block + 16 {
            render_source_block(&mut render, &mut sources, block);
            if !restart_applied && let Some(receipt) = owner.try_applied() {
                assert_eq!(receipt.revision, restart.revision);
                restart_first_sample = Some(receipt.first_sample);
                restart_applied = true;
            }
            match owner.try_read_continuous_spectrum() {
                Ok(window) => restart_window = Some(window),
                Err(HostSpectrumReadError::Pending | HostSpectrumReadError::Warming) => {}
                Err(error) => panic!("unexpected restart read: {error:?}"),
            }
        }
        let restart_window = restart_window.expect("restart window");
        assert!(restart_applied);
        assert_eq!(restart_window.observation_generation, restart.revision);
        assert_eq!(restart_window.selection_epoch, 2);
        assert_eq!(restart_window.window.stream_epoch, 1);
        assert_eq!(restart_window.window.sequence, 0);
        assert_eq!(
            restart_window.window.first_sample,
            restart_first_sample.expect("restart application sample")
        );
    }
}

#[test]
fn protected_explicit_work_limits_are_inclusive_and_transactional() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Stereo,
    }]);
    let hop = SpectrumHop::new(256).expect("supported explicit hop");
    let mut observation = demand(&[]);
    observation.spectrum = Some(&request);
    let preparation = HostObservationPreparationConfig::new(observation).with_spectrum_hop(hop);

    let (host, _, mut owner) = prepare_host_runtime_with_observation_demand_config(
        &compiled,
        &caps(),
        &console(),
        &preparation,
    )
    .expect("explicit protected owner");
    let spectrum_demand = HostSpectrumDemand {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let expected = owner
        .replace_spectrum(&spectrum_demand)
        .expect("unlimited explicit admission")
        .work;
    assert_eq!(
        expected.capture_copy_samples_per_block,
        2 * 128 + (2 + 4) * SPECTRUM_WINDOW_FRAMES as u64
    );
    assert_eq!(owner.spectrum_cadence().unwrap().hop_frames(), hop.get());
    drop(host);

    for field in 0..5 {
        let mut exact = preparation;
        exact.demand = observation;
        match field {
            0 => {
                exact.demand.work_limits.maximum_active_spectrum_captures =
                    expected.active_spectrum_captures
            }
            1 => {
                exact
                    .demand
                    .work_limits
                    .maximum_capture_input_samples_per_block =
                    expected.capture_input_samples_per_block
            }
            2 => {
                exact
                    .demand
                    .work_limits
                    .maximum_capture_copy_samples_per_block =
                    expected.capture_copy_samples_per_block
            }
            3 => {
                exact
                    .demand
                    .work_limits
                    .maximum_capture_publications_per_block =
                    expected.capture_publications_per_block
            }
            4 => {
                exact.demand.work_limits.maximum_capture_bytes_per_second =
                    expected.capture_bytes_per_second
            }
            _ => unreachable!(),
        }
        let (_exact_host, _, mut exact_owner) =
            prepare_host_runtime_with_observation_demand_config(
                &compiled,
                &caps(),
                &console(),
                &exact,
            )
            .expect("exact spectrum work limit prepares");
        let accepted = exact_owner
            .replace_spectrum(&spectrum_demand)
            .expect("exact spectrum work limit is inclusive");
        assert_eq!(accepted.work, expected);

        let mut below = exact;
        match field {
            0 => below.demand.work_limits.maximum_active_spectrum_captures -= 1,
            1 => {
                below
                    .demand
                    .work_limits
                    .maximum_capture_input_samples_per_block -= 1
            }
            2 => {
                below
                    .demand
                    .work_limits
                    .maximum_capture_copy_samples_per_block -= 1
            }
            3 => {
                below
                    .demand
                    .work_limits
                    .maximum_capture_publications_per_block -= 1
            }
            4 => below.demand.work_limits.maximum_capture_bytes_per_second -= 1,
            _ => unreachable!(),
        }
        let (_refused_host, _, mut refused_owner) =
            prepare_host_runtime_with_observation_demand_config(
                &compiled,
                &caps(),
                &console(),
                &below,
            )
            .expect("admission-only spectrum limit prepares");
        let before_work = refused_owner.work();
        let before_state = refused_owner.spectrum_state();
        let refusal = refused_owner
            .replace_spectrum(&spectrum_demand)
            .expect_err("one below spectrum work limit refuses");
        assert_eq!(refusal.reason, ObservationRefusalReason::WorkBudget);
        if field == 2 {
            assert_eq!(
                refusal.limit,
                Some("maximum_capture_copy_samples_per_block")
            );
            assert_eq!(refusal.requested, Some(12_544));
            assert_eq!(refusal.maximum, Some(12_543));
        }
        assert_eq!(refused_owner.work(), before_work);
        assert_eq!(refused_owner.spectrum_state(), before_state);
        assert_eq!(
            refused_owner.spectrum_cadence().unwrap().hop_frames(),
            hop.get()
        );
    }
}

#[test]
fn protected_explicit_spectrum_is_pcm_bit_exact() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let target = SpectrumTarget::TrackPostMatrix("eq1".into());
    let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
        target: target.clone(),
        channels: SpectrumChannels::Stereo,
    }]);
    let mut observation = demand(&[]);
    observation.spectrum = Some(&request);
    let hop = SpectrumHop::new(256).expect("supported explicit hop");
    let preparation = HostObservationPreparationConfig::new(observation).with_spectrum_hop(hop);
    let baseline_host = prepare_host_runtime(&compiled, &caps()).expect("baseline host");
    let (selected_host, _, mut owner) = prepare_host_runtime_with_observation_demand_config(
        &compiled,
        &caps(),
        &console(),
        &preparation,
    )
    .expect("explicit protected host");
    let selected = HostSpectrumDemand {
        target,
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let accepted = owner
        .replace_spectrum(&selected)
        .expect("explicit spectrum admission");
    let (mut baseline, mut baseline_sources, _) = baseline_host.start_render_session().unwrap();
    let (mut render, mut sources, _) = selected_host.start_render_session().unwrap();
    for block in 0..16 {
        let baseline_output = render_source_block(&mut baseline, &mut baseline_sources, block);
        let selected_output = render_source_block(&mut render, &mut sources, block);
        for (baseline_sample, selected_sample) in baseline_output.iter().zip(selected_output.iter())
        {
            assert_eq!(
                baseline_sample.to_bits(),
                selected_sample.to_bits(),
                "explicit protected spectrum changed PCM at block {block}"
            );
        }
        if block == 0 {
            assert_eq!(
                owner.try_applied().expect("spectrum application").revision,
                accepted.revision
            );
        }
    }
    let window = owner
        .try_read_continuous_spectrum()
        .expect("explicit first spectrum window");
    assert_eq!(window.observation_generation, accepted.revision);
    assert_eq!(window.selection_epoch, 1);
    assert_eq!(window.window.first_sample, 0);
    assert_eq!(
        window.window.end_sample(),
        Some(SPECTRUM_WINDOW_FRAMES as u64)
    );
}

#[test]
fn continuous_spectrum_replaces_restarts_and_refuses_protected_one_shot_atomically() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Stereo,
    }]);
    let mut observation = demand(&[]);
    observation.spectrum = Some(&request);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("controlled spectrum owner");
    let target = SpectrumTarget::TrackPostMatrix("eq1".into());
    let continuous = HostSpectrumDemand {
        target: target.clone(),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let first = owner
        .replace_spectrum(&continuous)
        .expect("first continuous admission");
    assert_eq!(first.revision, 1);
    assert_eq!(first.work.active_spectrum_captures, 1);

    let one_shot = HostSpectrumDemand {
        target: SpectrumTarget::TrackPostMatrix("unknown".into()),
        channels: SpectrumChannels::Left,
        mode: HostSpectrumMode::OneShot,
    };
    assert_eq!(
        owner.replace_spectrum(&one_shot),
        Err(host_core::ObservationRefusal {
            reason: ObservationRefusalReason::InvalidRequest,
            limit: None,
            requested: None,
            maximum: None,
        })
    );
    assert_eq!(owner.work(), first.work);

    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    render_source_block(&mut render, &mut sources, 0);
    assert_eq!(owner.try_applied().expect("first application").revision, 1);

    let restart = owner
        .restart_spectrum()
        .expect("same-target restart admission");
    assert_eq!(restart.revision, 2);
    assert_eq!(restart.work, first.work);
    let removal = match owner.stop_spectrum().expect("reserved spectrum stop") {
        host_core::ObservationStop::Pending(accepted) => accepted,
        host_core::ObservationStop::Quiescent => panic!("active spectrum requires a stop"),
    };
    assert_eq!(removal.revision, 3);
    assert_eq!(
        owner.stop_spectrum(),
        Ok(host_core::ObservationStop::Pending(removal))
    );
    render_source_block(&mut render, &mut sources, 1);
    assert_eq!(
        owner.try_applied().expect("restart application").revision,
        2
    );
    assert_eq!(owner.try_applied().expect("stop application").revision, 3);
    assert_eq!(
        owner.stop_spectrum(),
        Ok(host_core::ObservationStop::Quiescent)
    );

    #[cfg(feature = "test-support")]
    {
        test_only_reset_spectrum_operation_counts();
        render_source_block(&mut render, &mut sources, 2);
        assert_eq!(
            test_only_spectrum_operation_counts(),
            SpectrumOperationCounts::default(),
            "applied spectrum stop must remove graph dispatch"
        );
    }
}

#[test]
fn spectrum_and_meter_complete_sets_preserve_the_sibling_family() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let requests = meters();
    let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Left,
    }]);
    let mut observation = demand(&requests);
    observation.spectrum = Some(&request);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("meter-plus-spectrum owner");
    let ids = owner_ids(&owner);
    let target = HostSpectrumDemand {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Left,
        mode: HostSpectrumMode::Continuous,
    };
    let spectrum = owner.replace_spectrum(&target).expect("spectrum admission");
    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    render_source_block(&mut render, &mut sources, 0);
    assert_eq!(owner.try_applied().expect("spectrum receipt").revision, 1);

    let meter_only = owner
        .replace_meters(&ids[..1])
        .expect("meter replacement retains spectrum");
    assert_eq!(meter_only.work.active_spectrum_captures, 1);
    assert_eq!(meter_only.work.active_meter_channels, 2);
    render_source_block(&mut render, &mut sources, 1);
    assert_eq!(owner.try_applied().expect("meter receipt").revision, 2);

    let spectrum_stop = match owner.stop_spectrum().expect("spectrum-only stop") {
        host_core::ObservationStop::Pending(accepted) => accepted,
        host_core::ObservationStop::Quiescent => panic!("spectrum remains active"),
    };
    assert_eq!(spectrum_stop.work.active_spectrum_captures, 0);
    assert_eq!(spectrum_stop.work.active_meter_channels, 2);
    render_source_block(&mut render, &mut sources, 2);
    assert_eq!(
        owner.try_applied().expect("spectrum stop receipt").revision,
        spectrum_stop.revision
    );
    assert_eq!(owner.work().active_spectrum_captures, 0);
    assert_eq!(owner.work().active_meter_channels, 2);
    assert_ne!(spectrum.work.active_spectrum_captures, 0);
}

#[test]
fn pending_spectrum_start_and_reserved_stop_reconcile_both_receipts() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Stereo,
    }]);
    let mut observation = demand(&[]);
    observation.spectrum = Some(&request);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("spectrum owner");
    let demand = HostSpectrumDemand {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let start = owner.replace_spectrum(&demand).expect("spectrum start");
    let stop = match owner.stop_spectrum().expect("reserved stop") {
        host_core::ObservationStop::Pending(accepted) => accepted,
        host_core::ObservationStop::Quiescent => panic!("pending spectrum start must be stopped"),
    };
    assert_eq!(stop.revision, start.revision + 1);
    assert_eq!(
        owner.stop_spectrum(),
        Ok(host_core::ObservationStop::Pending(stop))
    );
    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    render_source_block(&mut render, &mut sources, 0);
    assert_eq!(
        owner.try_applied().expect("start receipt").revision,
        start.revision
    );
    assert_eq!(
        owner.try_applied().expect("stop receipt").revision,
        stop.revision
    );
    assert_eq!(
        owner.stop_spectrum(),
        Ok(host_core::ObservationStop::Quiescent)
    );
}

#[test]
fn one_shot_refusal_precedes_inert_owner_lookup() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let observation = demand(&[]);
    let (_host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("inert owner");
    let demand = HostSpectrumDemand {
        target: SpectrumTarget::TrackPostMatrix("unknown".into()),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::OneShot,
    };
    assert_eq!(
        owner.replace_spectrum(&demand),
        Err(host_core::ObservationRefusal {
            reason: ObservationRefusalReason::InvalidRequest,
            limit: None,
            requested: None,
            maximum: None,
        })
    );
    assert_eq!(
        owner.stop_spectrum(),
        Ok(host_core::ObservationStop::Quiescent)
    );
}

#[test]
fn continuous_reads_fence_pending_and_preserve_generation_selection_and_sibling_identity() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let request = controlled_spectrum_request(vec![
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("eq1".into()),
            channels: SpectrumChannels::Stereo,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
        },
    ]);
    let meter_requests = meters();
    let mut observation = demand(&meter_requests);
    observation.spectrum = Some(&request);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("controlled spectrum owner");
    let meter_ids = owner_ids(&owner);
    let target_a = HostSpectrumDemand {
        target: SpectrumTarget::TrackPostMatrix("eq1".into()),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let first = owner
        .replace_spectrum(&target_a)
        .expect("first spectrum admission");
    assert_eq!(first.revision, 1);
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::PendingApplication)
    );

    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    for block in 0..16 {
        render_source_block(&mut render, &mut sources, block);
    }
    // The completed queue item is still present while its graph application receipt is pending.
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::PendingApplication)
    );
    assert_eq!(
        owner.try_applied().expect("first receipt").revision,
        first.revision
    );
    let first_window = owner
        .try_read_continuous_spectrum()
        .expect("first continuous window");
    assert_eq!(first_window.owner, owner.owner());
    assert_eq!(first_window.observation_generation, first.revision);
    assert_eq!(first_window.selection_epoch, 1);
    assert_eq!(first_window.window.stream_epoch, 1);
    assert_eq!(first_window.window.sequence, 0);
    assert_eq!(first_window.window.first_sample, 0);
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::Pending)
    );

    // A meter-only publication keeps the spectrum observer in the complete set. With no new
    // complete window yet, the applied sibling publication leaves its reader pending.
    let sibling = owner
        .replace_meters(&meter_ids[..1])
        .expect("sibling meter admission");
    assert_eq!(sibling.revision, 2);
    for block in 16..17 {
        render_source_block(&mut render, &mut sources, block);
    }
    assert_eq!(
        owner.try_applied().expect("sibling receipt").revision,
        sibling.revision
    );
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::Pending)
    );

    let target_b = HostSpectrumDemand {
        target: SpectrumTarget::Output("main-out".into()),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let second = owner
        .replace_spectrum(&target_b)
        .expect("target replacement admission");
    assert_eq!(second.revision, 3);
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::PendingApplication)
    );
    for block in 17..33 {
        render_source_block(&mut render, &mut sources, block);
    }
    assert_eq!(
        owner
            .try_applied()
            .expect("target replacement receipt")
            .revision,
        second.revision
    );
    let second_window = owner
        .try_read_continuous_spectrum()
        .expect("replacement continuous window");
    assert_eq!(second_window.observation_generation, second.revision);
    assert_eq!(second_window.selection_epoch, 2);
    assert_eq!(second_window.window.stream_epoch, 1);
    assert_eq!(second_window.window.sequence, 0);
    assert_eq!(second_window.window.first_sample, 2_176);

    let restart = owner.restart_spectrum().expect("same-target restart");
    assert_eq!(restart.revision, 4);
    assert_eq!(restart.work, second.work);
    for block in 33..49 {
        render_source_block(&mut render, &mut sources, block);
    }
    assert_eq!(
        owner.try_applied().expect("restart receipt").revision,
        restart.revision
    );
    let restarted_window = owner
        .try_read_continuous_spectrum()
        .expect("restarted continuous window");
    assert_eq!(restarted_window.observation_generation, restart.revision);
    assert_eq!(restarted_window.selection_epoch, 2);
    assert_eq!(restarted_window.window.stream_epoch, 1);
    assert_eq!(restarted_window.window.sequence, 0);
    assert_eq!(restarted_window.window.first_sample, 4_224);
}

#[test]
fn continuous_reads_wrap_failure_and_gap_epochs_and_close_with_renderer() {
    let compiled = compile_host_session(SESSION, &caps()).unwrap();
    let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
        target: SpectrumTarget::Output("main-out".into()),
        channels: SpectrumChannels::Stereo,
    }]);
    let mut observation = demand(&[]);
    observation.spectrum = Some(&request);
    let (host, _, mut owner) =
        prepare_host_runtime_with_observation_demand(&compiled, &caps(), &console(), &observation)
            .expect("controlled spectrum owner");
    let demand = HostSpectrumDemand {
        target: SpectrumTarget::Output("main-out".into()),
        channels: SpectrumChannels::Stereo,
        mode: HostSpectrumMode::Continuous,
    };
    let accepted = owner.replace_spectrum(&demand).expect("spectrum admission");
    let (mut render, mut sources, _) = host.start_render_session().unwrap();
    for block in 0..16 {
        render_source_block(&mut render, &mut sources, block);
    }
    assert_eq!(
        owner.try_applied().expect("spectrum receipt").revision,
        accepted.revision
    );
    let _ = owner
        .try_read_continuous_spectrum()
        .expect("initial window");

    // A real source generation boundary invalidates a partial continuous window while keeping
    // the render owner usable for the next epoch. Input NaNs are sanitized by the audio contract,
    // so use the existing generation-tagged seek seam for this fault fixture.
    for block in 16..24 {
        render_source_block(&mut render, &mut sources, block);
    }
    sources
        .seek(b"fixture-source", 2, 2_048)
        .expect("queue generation change");
    assert!(render.prepare_source_seek(0, 2, 2_048));
    let fresh_left = [0.75_f32; 128];
    let fresh_right = [-0.25_f32; 128];
    sources
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 2,
                start_frame: 2_048,
                sample_rate_hz: 48_000,
                planes: &[&fresh_left, &fresh_right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("fresh generation block");
    let mut output = [0.0_f32; 128 * 2];
    render
        .render_planar(&mut output, 2, 128, 128, 3_072)
        .expect("render after seek");
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::Failed {
            owner: owner.owner(),
            observation_generation: accepted.revision,
            stream_epoch: 2,
        })
    );

    // Two complete post-failure windows overfill the one-record queue. The first read reports the
    // native gap and leaves the one queued record for the next read.
    for block in 25..57 {
        let first_sample = (block * 128) as u64;
        let source_start = 2_176 + ((block - 25) * 128) as u64;
        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 2,
                    start_frame: source_start,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .expect("post-failure source block");
        let mut output = [0.0_f32; 128 * 2];
        render
            .render_planar(&mut output, 2, 128, 128, first_sample)
            .expect("post-failure render block");
    }
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::Gap {
            owner: owner.owner(),
            observation_generation: accepted.revision,
            stream_epoch: 2,
            dropped_captures: 1,
        })
    );
    let recovered = owner
        .try_read_continuous_spectrum()
        .expect("queued post-gap window remains available");
    assert_eq!(recovered.observation_generation, accepted.revision);
    assert_eq!(recovered.window.stream_epoch, 2);
    assert_eq!(recovered.window.sequence, 0);

    drop(render);
    assert_eq!(
        owner.try_read_continuous_spectrum().err(),
        Some(HostSpectrumReadError::Closed)
    );
}

#[test]
fn protected_track_spectrum_preserves_pcm_and_counts_resident_final_partial_and_large_quantum() {
    use engine::realtime::audit;

    for (quantum, source_ring_frames) in [(192_usize, 1_152_u32), (4_096, 8_192)] {
        let document = SESSION.replace(
            "\"quantum_frames\": 128",
            &format!("\"quantum_frames\": {quantum}"),
        );
        let mut test_caps = caps();
        test_caps.source_ring_frames = source_ring_frames;
        let compiled = compile_host_session(&document, &test_caps).expect("quantum fixture");
        let baseline_host = prepare_host_runtime(&compiled, &test_caps).expect("baseline host");
        let target = SpectrumTarget::TrackPostMatrix("eq1".into());
        let oracle_request = SpectrumCaptureRequest {
            target: target.clone(),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        };
        let (oracle_host, mut oracle_capture) =
            prepare_host_runtime_with_spectrum(&compiled, &test_caps, &oracle_request)
                .expect("one-shot target oracle host");
        oracle_capture.arm().expect("one-shot target oracle arm");
        let (mut oracle_render, mut oracle_sources, _) = oracle_host
            .start_render_session()
            .expect("one-shot target oracle render");
        let request = controlled_spectrum_request(vec![SpectrumCaptureCollectionEntry {
            target: target.clone(),
            channels: SpectrumChannels::Stereo,
        }]);
        let mut observation = demand(&[]);
        observation.spectrum = Some(&request);
        let (selected_host, _, mut owner) = prepare_host_runtime_with_observation_demand(
            &compiled,
            &test_caps,
            &console(),
            &observation,
        )
        .expect("protected track spectrum host");
        let selected = HostSpectrumDemand {
            target: target.clone(),
            channels: SpectrumChannels::Stereo,
            mode: HostSpectrumMode::Continuous,
        };
        let accepted = owner
            .replace_spectrum(&selected)
            .expect("spectrum admission");
        let blocks = SPECTRUM_WINDOW_FRAMES.div_ceil(quantum);
        for block in 0..blocks {
            let first_sample = (block * quantum) as u64;
            let left = vec![0.25_f32; quantum];
            let right = vec![-0.5_f32; quantum];
            oracle_sources
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: first_sample,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: quantum as u32,
                        end_of_region: false,
                    },
                )
                .expect("oracle source block");
            let mut oracle_pcm = vec![f32::from_bits(0x7fc0_3990); quantum * 2];
            oracle_render
                .render_planar(&mut oracle_pcm, 2, quantum, quantum, first_sample)
                .expect("oracle render block");
        }
        // The source is constant in every block. An independent one-shot capture at the same
        // resident target supplies the exact per-sample target oracle, including its fixed filter
        // startup state, for the continuous resident capture below.
        let oracle_window = oracle_capture
            .try_read()
            .expect("one-shot target oracle window");
        let (mut baseline_render, mut baseline_sources, _) = baseline_host
            .start_render_session()
            .expect("baseline render");
        let (mut render, mut sources, _) = selected_host
            .start_render_session()
            .expect("selected render");
        #[cfg(feature = "test-support")]
        test_only_reset_spectrum_operation_counts();
        for block in 0..blocks {
            let first_sample = (block * quantum) as u64;
            let left = vec![0.25_f32; quantum];
            let right = vec![-0.5_f32; quantum];
            for sources in [&mut baseline_sources, &mut sources] {
                sources
                    .submit(
                        b"fixture-source",
                        SourceSubmission {
                            generation: 1,
                            start_frame: first_sample,
                            sample_rate_hz: 48_000,
                            planes: &[&left, &right],
                            frames: quantum as u32,
                            end_of_region: false,
                        },
                    )
                    .expect("source block");
            }
            let mut baseline_pcm = vec![f32::from_bits(0x7fc0_3990); quantum * 2];
            let mut selected_pcm = vec![f32::from_bits(0x7fc0_3990); quantum * 2];
            let reports = audit::in_render_scope(|| {
                let baseline_report = baseline_render.render_planar(
                    &mut baseline_pcm,
                    2,
                    quantum,
                    quantum,
                    first_sample,
                );
                let selected_report =
                    render.render_planar(&mut selected_pcm, 2, quantum, quantum, first_sample);
                (baseline_report, selected_report, audit::snapshot())
            });
            assert!(reports.0.is_ok(), "baseline render: {:?}", reports.0);
            assert!(reports.1.is_ok(), "selected render: {:?}", reports.1);
            assert_eq!(reports.2.allocations, 0, "spectrum render allocated");
            assert_eq!(reports.2.deallocations, 0, "spectrum render freed");
            assert_eq!(
                baseline_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                selected_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "protected spectrum changed PCM at quantum {quantum} block {block}"
            );
            if block == 0 {
                assert_eq!(
                    owner.try_applied().expect("spectrum receipt").revision,
                    accepted.revision
                );
            }
        }
        let window = owner
            .try_read_continuous_spectrum()
            .expect("protected window");
        assert_eq!(window.observation_generation, accepted.revision);
        assert_eq!(window.window.first_sample, 0);
        assert_eq!(
            window.window.left, oracle_window.left,
            "resident continuous left payload differs from the constant-source target oracle at quantum {quantum}"
        );
        assert_eq!(
            window.window.right, oracle_window.right,
            "resident continuous right payload differs from the constant-source target oracle at quantum {quantum}"
        );
        #[cfg(feature = "test-support")]
        {
            let counts = test_only_spectrum_operation_counts();
            assert_eq!(counts.validation_samples, (2 * blocks * quantum) as u64);
            assert_eq!(
                counts.selected_storage_writes,
                (2 * blocks * quantum) as u64
            );
            assert_eq!(
                counts.chronological_reconstruction_writes,
                (2 * SPECTRUM_WINDOW_FRAMES) as u64
            );
            assert_eq!(
                counts.owned_record_sample_copies,
                (2 * SPECTRUM_WINDOW_FRAMES) as u64
            );
            assert_eq!(
                counts.queue_payload_transfer_samples,
                (2 * SPECTRUM_WINDOW_FRAMES) as u64
            );
            assert_eq!(counts.payload_constructions, 1);
            assert_eq!(counts.publication_attempts, 1);
        }
    }
}
