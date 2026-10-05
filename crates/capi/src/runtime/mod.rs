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
    PlanResources, ResourceFault, SourceFailure, SourceSubmission,
};
pub(crate) use core::mem::size_of;
pub(crate) use effect_contract::TailSamples;
pub(crate) use engine::realtime::RenderError;

pub(crate) use crate::{
    ABI_VERSION, CompileLimits, PlanResourceReport, RESULT_BACKPRESSURE, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, TAIL_FINITE, TAIL_INFINITE,
};

pub(crate) mod error;
#[cfg(test)]
mod live_peak_tests;
#[cfg(test)]
mod live_tests;
#[cfg(test)]
mod tests;

pub(crate) use error::*;

// What capi's runtime tests name through `use super::*`: the dependency items the moved modules
// imported here before #1309, and the control plane's `test-support` observers (D8).
#[cfg(test)]
pub(crate) use control_plane::{
    C_ABI_LIVE_LANES, CapiResources, CompiledModelAdmission, LIVE_QUEUE_DEPTH, LiveEpochResources,
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
        control_plane::compile_children::<CapiAdapter>(document, ControlLimits::from(limits))?;
    Ok(CompiledChildren {
        session,
        session_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
        plan,
    })
}

pub(crate) fn all_limits_nonzero(limits: CompileLimits) -> bool {
    limits.maximum_automation_spans_per_block != 0
        && [
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
        && limits.reserved0 == 0
        && limits.reserved == [0; 2]
        && all_limits_nonzero(limits)
}

/// The frozen ABI limits as the control plane's plain limits (#1309 D3): every numeric field, same
/// name and type, with `maximum_capi_retained_bytes` as `maximum_control_retained_bytes`. The
/// header words (`struct_size`, `reserved0`, `reserved`) are [`limits_are_valid`]'s alone.
impl From<CompileLimits> for ControlLimits {
    fn from(limits: CompileLimits) -> Self {
        Self {
            source_ring_frames: limits.source_ring_frames,
            maximum_automation_spans_per_block: limits.maximum_automation_spans_per_block,
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
        }
    }
}

/// The ABI report row the control plane stores, built once when a row is inserted (#1309 D4).
impl From<PlanResources> for PlanResourceReport {
    fn from(resources: PlanResources) -> Self {
        let (tail_kind, tail_samples) = match resources.output_tail {
            TailSamples::Finite(samples) => (TAIL_FINITE, samples),
            TailSamples::Infinite => (TAIL_INFINITE, 0),
        };
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
            output_tail: if report.tail_kind == TAIL_FINITE {
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
    fn distinct(output_tail: TailSamples) -> PlanResources {
        PlanResources {
            sample_rate_hz: 1,
            quantum_frames: 2,
            source_count: 3,
            track_count: 4,
            latency_samples: 5,
            output_tail,
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
