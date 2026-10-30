//! capi's adapter over the portable control plane.
//!
//! The control plane itself -- the session store, the live classifier call, successor preparation,
//! plan publication and retirement, provider epochs -- is the `control-plane` crate, shared with
//! the browser (#1309). What stays here is what is capi's:
//!
//! | module | job |
//! |---|---|
//! | [`error`] | the session's bounded diagnostic storage, the plan's render codes, and capi's bytes for every failure the control plane reports |
//! | this module | [`CapiAdapter`], the limit and report conversions, and the children `compile_session` returns |
//!
//! The compile *pipeline* is not here at all: it is `host-core`, shared with every other host.

pub(crate) use control_plane::{
    CommandError, CompileFailure, ControlAdapter, ControlLimits, EventError, EventLane,
    PlanResources, ResourceFault, SessionSeekFailure, SourceFailure, SourceSubmission,
};
pub(crate) use core::mem::size_of;
pub(crate) use effect_contract::TailSamples;
pub(crate) use engine::realtime::RenderError;

pub(crate) use crate::{
    ABI_VERSION, CompileLimits, PlanResourceReport, RESULT_BACKPRESSURE, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, TAIL_FINITE,
};

pub(crate) mod error;
#[cfg(test)]
mod live_peak_tests;
#[cfg(test)]
mod live_tests;
#[cfg(test)]
mod mute_automation_tests;
#[cfg(test)]
mod spectrum_tests;
#[cfg(test)]
mod supersede_tests;
#[cfg(test)]
mod tests;

pub(crate) use error::*;

// What capi's runtime tests name through `use super::*`: the dependency items the moved modules
// imported here before #1309, and the control plane's `test-support` observers (D8).
#[cfg(test)]
pub(crate) use control_plane::{
    CapiResources, CompiledModelAdmission, LiveCatalogAdmission, LiveEpochResources,
    TestOwnerCounters, TestOwnerState, TestStructuralFaultPhase, test_lifecycle_counters,
    test_reset_lifecycle_observer, validate_live_peak,
};
#[cfg(test)]
pub(crate) use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo};
#[cfg(test)]
pub(crate) use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, SourceControlSet,
    prepare_host_runtime_with_live_lanes,
};
#[cfg(test)]
pub(crate) use protocol::{DecodeScratch, ProtocolCodec};
#[cfg(test)]
pub(crate) use std::sync::atomic::Ordering;

/// capi's transaction snapshot: the control plane's, with capi's report rows.
#[cfg(test)]
pub(crate) type TestTransactionSnapshot =
    control_plane::TestTransactionSnapshot<PlanResourceReport>;

/// The facade caps for capi's ABI limits, through the one conversion (#1309 D3).
#[cfg(test)]
pub(crate) fn prepare_caps(limits: CompileLimits) -> HostPrepareCaps {
    control_plane::prepare_caps(ControlLimits::from(limits))
}

/// capi as a control-plane adapter (#1309 D4, D5): its report table stores the frozen ABI
/// [`PlanResourceReport`], and each session keeps one `Session` and one `Plan` handle.
pub(crate) struct CapiAdapter;

impl ControlAdapter for CapiAdapter {
    type Row = PlanResourceReport;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[
        size_of::<crate::Session>() as u64,
        size_of::<crate::Plan>() as u64,
    ];
    /// `miso_engine_v1_submit_document_command` borrows the document beside the frame (#1386).
    const DOCUMENT_TRANSPORT: bool = true;
    /// The C ABI subscribes through the `Observe` edit (#1584 D7a): its plans bind an observation
    /// queue slot for every declared tap ([`CapiPreparer`]), and nothing else writes those queues.
    const OBSERVE_EDIT: bool = true;
    /// The control plane serves the C ABI's meter batches from its own delivery (#1583 R2).
    const METER_DELIVERY: bool = true;
    /// The C ABI reads spectrum through `miso_engine_v1_submit_command` (#1595 R1, R2): its
    /// sessions advertise the three spectrum commands and capability bit 16.
    const SPECTRUM: bool = true;
}

/// capi's preparation (issue #1400 D3): the live request with observation capacity for every
/// declared tap (#1584 D7), the lanes the control plane chose (with the effect lanes that
/// observation rides) and the session's spectrum preparation (#1593 D3), through host-core's one
/// session entry. It holds no lane constant: the lanes arrive as an argument (decision 17 C6).
pub(crate) struct CapiPreparer;

impl control_plane::RuntimePreparer for CapiPreparer {
    fn prepare(
        &self,
        compiled: &host_core::CompiledSession,
        caps: &host_core::HostPrepareCaps,
        lanes: host_core::HostLiveLanes,
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<host_core::SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<
        (
            host_core::PreparedHost,
            host_core::HostLiveControlHandles,
            Option<host_core::PreparedSpectrumCaptures>,
        ),
        host_core::PrepareDiagnostics,
    > {
        let (live, lanes) = capi_request(lanes);
        host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
            compiled, caps, &live, lanes, spectrum, successor, superseded,
        )
    }

    /// Issue #2199: the same preparation, projected.
    fn project(
        &self,
        compiled: &host_core::CompiledSession,
        caps: &host_core::HostPrepareCaps,
        lanes: host_core::HostLiveLanes,
        spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<host_core::SuccessorBase<'_>>,
        superseded: &host_core::LiveValuesSuperseded,
    ) -> Result<host_core::HostProjection, host_core::PrepareDiagnostics> {
        let (live, lanes) = capi_request(lanes);
        host_core::project_host_session(
            compiled,
            caps,
            host_core::HostProjectionRequest {
                live_controls: Some(&live),
                selected_meters: None,
                backend: lane::Backend::current(),
                spectrum,
                successor,
                lanes,
                superseded: Some(superseded),
            },
        )
    }
}

/// capi's live-control request and the lanes it attaches over the control plane's `lanes`.
fn capi_request(
    lanes: host_core::HostLiveLanes,
) -> (host_core::HostLiveControlRequest, host_core::HostLiveLanes) {
    // #1584 D7: an observation queue slot for every declared tap of every effect, so an
    // `Observe` edit always has a queue. Observation rides the effect lanes (#1254 D2), so a
    // session whose live rows chose none gets them, as the browser's does.
    // #1587a: the strip meters' queues are as deep as the output meter's, so a service step
    // that lags the render by up to that many windows still pairs every strip window with
    // its output window.
    let live = host_core::HostLiveControlRequest {
        meter_queue_depth: core::num::NonZeroUsize::new(host_core::METER_QUEUE_DEPTH)
            .unwrap_or(core::num::NonZeroUsize::MIN),
        observation_taps: host_core::launch_maximum_observation_taps(),
        ..host_core::HostLiveControlRequest::default()
    };
    let lanes = host_core::HostLiveLanes {
        effects: lanes.effects || live.observation_taps != 0,
        ..lanes
    };
    (live, lanes)
}

/// The control-protocol session capi's `Session` handle owns.
pub(crate) type SessionState = control_plane::SessionState<CapiAdapter>;
/// The render-thread plan capi's `Plan` handle owns.
pub(crate) type PlanState = control_plane::PlanState<CapiAdapter>;
/// The any-thread resource query capi's `Plan` handle holds beside the render state.
pub(crate) type PlanQueries = control_plane::PlanQueries<CapiAdapter>;

/// The children `miso_engine_v1_compile_session` returns: the control plane's session and plan,
/// and the session's own diagnostic storage.
pub(crate) struct CompiledChildren {
    pub(crate) session: SessionState,
    pub(crate) session_error: FixedBytes,
    pub(crate) plan: PlanState,
}

/// Compiles `document` under capi's validated `limits`: converts them once (#1309 D3), runs the
/// control plane's compile, and builds the session's diagnostic storage after it, as the last
/// allocation of the compile.
pub(crate) fn compile_children(
    document: &str,
    limits: CompileLimits,
) -> Result<CompiledChildren, CompileRejection> {
    let control_plane::CompiledChildren { session, plan } =
        control_plane::compile_children::<CapiAdapter>(
            document,
            ControlLimits::from(limits),
            Box::new(CapiPreparer),
        )?;
    Ok(CompiledChildren {
        session,
        session_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
        plan,
    })
}

pub(crate) fn all_limits_nonzero(limits: CompileLimits) -> bool {
    [
        limits.maximum_document_bytes,
        limits.maximum_diagnostic_bytes,
        limits.maximum_tracks,
        limits.maximum_sources,
        limits.maximum_routes,
        limits.maximum_effects,
        limits.maximum_graph_session_plus_plan_bytes,
        limits.maximum_source_total_bytes,
        limits.maximum_source_overhead_bytes,
        limits.maximum_effect_state_bytes,
        limits.maximum_effect_scratch_bytes,
        limits.maximum_builtin_retained_bytes,
        limits.maximum_capi_retained_bytes,
        limits.maximum_named_allocation_bytes,
        limits.maximum_meter_streams,
        limits.maximum_meter_items,
        limits.maximum_meter_bytes,
        limits.maximum_control_frame_bytes,
        limits.maximum_replay_bytes,
        limits.maximum_replay_entries,
    ]
    .into_iter()
    .all(|value| value != 0)
}

pub(crate) fn limits_are_valid(limits: CompileLimits) -> bool {
    limits.struct_size == crate::COMPILE_LIMITS_SIZE
        && limits.reserved1 == 0
        && limits.reserved0 == 0
        && limits.reserved == [0; 2]
        && all_limits_nonzero(limits)
}

/// The frozen ABI limits as the control plane's plain limits (#1309 D3): every numeric field, same
/// name and type, with `maximum_capi_retained_bytes` as `maximum_control_retained_bytes`. The
/// header words (`struct_size`, `reserved1`, `reserved0`, `reserved`) are `limits_are_valid`'s alone.
impl From<CompileLimits> for ControlLimits {
    fn from(limits: CompileLimits) -> Self {
        Self {
            source_ring_frames: limits.source_ring_frames,
            maximum_document_bytes: limits.maximum_document_bytes,
            maximum_diagnostic_bytes: limits.maximum_diagnostic_bytes,
            maximum_tracks: limits.maximum_tracks,
            maximum_sources: limits.maximum_sources,
            maximum_routes: limits.maximum_routes,
            maximum_effects: limits.maximum_effects,
            maximum_graph_session_plus_plan_bytes: limits.maximum_graph_session_plus_plan_bytes,
            maximum_source_total_bytes: limits.maximum_source_total_bytes,
            maximum_source_overhead_bytes: limits.maximum_source_overhead_bytes,
            maximum_effect_state_bytes: limits.maximum_effect_state_bytes,
            maximum_effect_scratch_bytes: limits.maximum_effect_scratch_bytes,
            maximum_builtin_retained_bytes: limits.maximum_builtin_retained_bytes,
            maximum_control_retained_bytes: limits.maximum_capi_retained_bytes,
            maximum_named_allocation_bytes: limits.maximum_named_allocation_bytes,
            maximum_meter_streams: limits.maximum_meter_streams,
            maximum_meter_items: limits.maximum_meter_items,
            maximum_meter_bytes: limits.maximum_meter_bytes,
            maximum_control_frame_bytes: limits.maximum_control_frame_bytes,
            maximum_replay_bytes: limits.maximum_replay_bytes,
            maximum_replay_entries: limits.maximum_replay_entries,
            maximum_submixes: limits.maximum_submixes,
            maximum_vcas: limits.maximum_vcas,
            // The compile limits carry no word for it: the same bound as every host.
            maximum_vca_reach_pairs: host_core::MAXIMUM_VCA_REACH_PAIRS,
            maximum_replacement_peak_bytes: u64::MAX,
        }
    }
}

/// The ABI report row the control plane stores, built once when a row is inserted (#1309 D4).
impl From<PlanResources> for PlanResourceReport {
    fn from(resources: PlanResources) -> Self {
        let (tail_kind, tail_samples) = control_plane::tail_words(resources.output_tail_every_peak);
        Self {
            struct_size: crate::PLAN_RESOURCE_REPORT_SIZE,
            abi_version: ABI_VERSION,
            sample_rate_hz: resources.sample_rate_hz,
            quantum_frames: resources.quantum_frames,
            source_count: resources.source_count,
            track_count: resources.track_count,
            latency_samples: resources.latency_samples,
            tail_kind,
            tail_samples,
            graph_session_plus_plan_bytes: resources.graph_session_plus_plan_bytes,
            graph_incremental_plan_bytes: resources.graph_incremental_plan_bytes,
            graph_metadata_bytes: resources.graph_metadata_bytes,
            graph_delay_bytes: resources.graph_delay_bytes,
            effect_bank_scratch_bytes: resources.effect_bank_scratch_bytes,
            effect_bank_runtime_buffer_bytes: resources.effect_bank_runtime_buffer_bytes,
            effect_bank_metadata_bytes: resources.effect_bank_metadata_bytes,
            builtin_bank_bytes: resources.builtin_bank_bytes,
            builtin_bank_scratch_bytes: resources.builtin_bank_scratch_bytes,
            source_pcm_payload_bytes: resources.source_pcm_payload_bytes,
            source_overhead_bytes: resources.source_overhead_bytes,
            source_total_bytes: resources.source_total_bytes,
            effect_scalar_state_bytes: resources.effect_scalar_state_bytes,
            effect_scalar_scratch_bytes: resources.effect_scalar_scratch_bytes,
            builtin_processor_payload_bytes: resources.builtin_processor_payload_bytes,
            builtin_meter_payload_bytes: resources.builtin_meter_payload_bytes,
            builtin_retained_payload_bytes: resources.builtin_retained_payload_bytes,
            capi_retained_bytes: resources.control_retained_bytes,
            largest_named_allocation_bytes: resources.largest_named_allocation_bytes,
            reserved: [0; 4],
        }
    }
}

/// A stored ABI row read back for the replacement and live admissions (#1309 D4). Lossless for
/// every row [`From<PlanResources>`] builds: `TAIL_FINITE` is `TailSamples::Finite(tail_samples)`
/// and `TAIL_INFINITE` is `TailSamples::Infinite`. No other tail kind is ever stored.
impl From<PlanResourceReport> for PlanResources {
    fn from(report: PlanResourceReport) -> Self {
        Self {
            sample_rate_hz: report.sample_rate_hz,
            quantum_frames: report.quantum_frames,
            source_count: report.source_count,
            track_count: report.track_count,
            latency_samples: report.latency_samples,
            output_tail_every_peak: if report.tail_kind == TAIL_FINITE {
                TailSamples::Finite(report.tail_samples)
            } else {
                TailSamples::Infinite
            },
            graph_session_plus_plan_bytes: report.graph_session_plus_plan_bytes,
            graph_incremental_plan_bytes: report.graph_incremental_plan_bytes,
            graph_metadata_bytes: report.graph_metadata_bytes,
            graph_delay_bytes: report.graph_delay_bytes,
            effect_bank_scratch_bytes: report.effect_bank_scratch_bytes,
            effect_bank_runtime_buffer_bytes: report.effect_bank_runtime_buffer_bytes,
            effect_bank_metadata_bytes: report.effect_bank_metadata_bytes,
            builtin_bank_bytes: report.builtin_bank_bytes,
            builtin_bank_scratch_bytes: report.builtin_bank_scratch_bytes,
            source_pcm_payload_bytes: report.source_pcm_payload_bytes,
            source_overhead_bytes: report.source_overhead_bytes,
            source_total_bytes: report.source_total_bytes,
            effect_scalar_state_bytes: report.effect_scalar_state_bytes,
            effect_scalar_scratch_bytes: report.effect_scalar_scratch_bytes,
            builtin_processor_payload_bytes: report.builtin_processor_payload_bytes,
            builtin_meter_payload_bytes: report.builtin_meter_payload_bytes,
            builtin_retained_payload_bytes: report.builtin_retained_payload_bytes,
            control_retained_bytes: report.capi_retained_bytes,
            largest_named_allocation_bytes: report.largest_named_allocation_bytes,
        }
    }
}

#[cfg(test)]
mod report_row_tests {
    //! #1309 D4: the control plane reads capi's stored rows back for its replacement and live
    //! admissions, so the row conversion must be lossless.

    use super::*;

    /// Every field distinct and nonzero, so a dropped, swapped or mis-mapped field changes the
    /// round trip.
    fn distinct(output_tail_every_peak: TailSamples) -> PlanResources {
        PlanResources {
            sample_rate_hz: 1,
            quantum_frames: 2,
            source_count: 3,
            track_count: 4,
            latency_samples: 5,
            output_tail_every_peak,
            graph_session_plus_plan_bytes: 7,
            graph_incremental_plan_bytes: 8,
            graph_metadata_bytes: 9,
            graph_delay_bytes: 10,
            effect_bank_scratch_bytes: 11,
            effect_bank_runtime_buffer_bytes: 12,
            effect_bank_metadata_bytes: 13,
            builtin_bank_bytes: 14,
            builtin_bank_scratch_bytes: 15,
            source_pcm_payload_bytes: 16,
            source_overhead_bytes: 17,
            source_total_bytes: 18,
            effect_scalar_state_bytes: 19,
            effect_scalar_scratch_bytes: 20,
            builtin_processor_payload_bytes: 21,
            builtin_meter_payload_bytes: 22,
            builtin_retained_payload_bytes: 23,
            control_retained_bytes: 24,
            largest_named_allocation_bytes: 25,
        }
    }

    #[test]
    fn a_stored_report_row_reads_back_as_the_resources_it_was_built_from() {
        for resources in [
            distinct(TailSamples::Finite(6)),
            distinct(TailSamples::Infinite),
        ] {
            assert_eq!(
                PlanResources::from(PlanResourceReport::from(resources)),
                resources
            );
        }
    }
}
