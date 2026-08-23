//! Safe control-plane orchestration behind the raw FFI boundary.

use core::{alloc::Layout, mem::size_of, num::NonZeroUsize};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::{
    SampleRateHz,
    realtime::{
        PlanExchangeConfig, PlanPublisher, PlanRetirer, PlanarBufferMut, PreparedRenderPlan,
        RealtimePlanOwner, RenderIo, RenderTime, plan_exchange, plan_exchange_resource_report,
    },
};
use miso_engine_effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry_v1, prepare_native_session_effects,
};
use miso_engine_effect_contract::TailSamples;
use miso_engine_graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, StableGraphId, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompileReport, GraphCompiler};
use miso_engine_protocol::{
    CommandFrameProcessError, ControllerRetainedCapacity, DecodeScratch, EncodeError,
    EventEgressError, MockProvider, PreparedCommandFrame, ProtocolCodec, ProtocolController,
    ProtocolControllerConfig, ProtocolLimits, ProtocolQueueConfig, ProtocolQueues,
    ProviderFeatures, ReplayCache, ReplayCacheConfig, SessionStore,
};
use miso_engine_session::{CompileCaps, CompiledSession, DiagnosticSet, parse_session_toml};
use miso_engine_source::{
    HostChunkError, HostChunkProvider, HostPlanarChunk, PcmSourceRing, PcmSourceRingConfig,
    SourceCommand, SourceFrame, SourceGeneration, SourceGraphSource, SourceGraphTrackMapping,
    SourceSeekError, prepare_graph_source_set,
};

use crate::{
    ABI_VERSION, CompileLimits, PlanResourceReport, RESULT_BACKPRESSURE, RESULT_INTERNAL,
    RESULT_INVALID_ARGUMENT, TAIL_FINITE, TAIL_INFINITE,
};

pub(crate) struct FixedBytes {
    bytes: Box<[u8]>,
    len: usize,
}

impl FixedBytes {
    fn try_new(capacity: u64) -> Result<Self, CompileFailure> {
        let capacity = usize::try_from(capacity).map_err(|_| failure("capi.resource.platform"))?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(capacity)
            .map_err(|_| failure("capi.resource.allocation"))?;
        bytes.resize(capacity, 0);
        Ok(Self {
            bytes: bytes.into_boxed_slice(),
            len: 0,
        })
    }

    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }

    pub(crate) fn set(&mut self, value: &[u8]) {
        let value = core::str::from_utf8(value).unwrap_or("capi.internal.utf8");
        self.len = value.len().min(self.bytes.len());
        while !value.is_char_boundary(self.len) {
            self.len -= 1;
        }
        self.bytes[..self.len].copy_from_slice(&value.as_bytes()[..self.len]);
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

struct ControlSource {
    id_offset: usize,
    id_bytes: usize,
    sample_rate_hz: u32,
    channel_count: u32,
    region_start: u64,
    region_end: u64,
    provider: HostChunkProvider,
}

struct ProviderEpoch {
    epoch: u64,
    source_ids: Box<[u8]>,
    sources: Box<[ControlSource]>,
}

/// Structural plans own independent source rings; buffered host state never crosses an epoch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StructuralSourceStatePolicy {
    ResetAtReplacementBoundary,
}

const STRUCTURAL_SOURCE_STATE_POLICY: StructuralSourceStatePolicy =
    StructuralSourceStatePolicy::ResetAtReplacementBoundary;

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TestStructuralFaultPhase {
    AfterProtocolPrepare,
    AfterResourceProjection,
    AfterRuntimePrepare,
    AfterAdmission,
    AfterPlanReservation,
    BeforeProtocolCommit,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TestOwnerCounters {
    pub(crate) current_provider_constructed: u64,
    pub(crate) current_provider_disposed: u64,
    pub(crate) candidate_provider_constructed: u64,
    pub(crate) candidate_provider_disposed: u64,
    pub(crate) candidate_provider_published: u64,
    pub(crate) current_plan_constructed: u64,
    pub(crate) current_plan_disposed: u64,
    pub(crate) candidate_plan_constructed: u64,
    pub(crate) candidate_plan_disposed: u64,
    pub(crate) candidate_plan_published: u64,
    pub(crate) token_constructed: u64,
    pub(crate) token_disposed: u64,
    pub(crate) replay_current_constructed: u64,
    pub(crate) replay_current_disposed: u64,
    pub(crate) replay_candidate_constructed: u64,
    pub(crate) replay_candidate_disposed: u64,
    pub(crate) replay_candidate_published: u64,
    pub(crate) reservation_constructed: u64,
    pub(crate) reservation_canceled: u64,
    pub(crate) reservation_committed: u64,
}

struct SharedPlanState {
    plan_alive: AtomicBool,
    active_epoch: AtomicU64,
    reports: Mutex<Vec<(u64, PlanResourceReport)>>,
    #[cfg(test)]
    render_sequence: AtomicU64,
    #[cfg(test)]
    render_sample: AtomicU64,
}

pub(crate) struct SessionState {
    controller: ProtocolController<MockProvider>,
    providers: ProviderEpoch,
    pending_providers: Vec<ProviderEpoch>,
    retired_providers: Vec<ProviderEpoch>,
    publisher: PlanPublisher,
    retirer: PlanRetirer,
    limits: CompileLimits,
    decode_fields: Box<[u16]>,
    _decode_tail: Option<Box<[u8]>>,
    response_scratch: Box<[u8]>,
    shared: Arc<SharedPlanState>,
    #[cfg(test)]
    observed_render_sequence: u64,
    #[cfg(test)]
    test_faults: [Option<TestStructuralFaultPhase>; 2],
    #[cfg(test)]
    test_fault_index: usize,
    #[cfg(test)]
    test_owner_counters: TestOwnerCounters,
}

pub(crate) struct PlanState {
    pub(crate) owner: RealtimePlanOwner,
    shared: Arc<SharedPlanState>,
    quantum_frames: u32,
    next_absolute_sample: u64,
}

pub(crate) struct CompiledChildren {
    pub(crate) session: SessionState,
    pub(crate) session_error: FixedBytes,
    pub(crate) plan: PlanState,
    pub(crate) plan_error: FixedBytes,
}

struct PreparedRuntime {
    source_ids: Box<[u8]>,
    sources: Box<[ControlSource]>,
    plan: PreparedRenderPlan,
    resources: PlanResourceReport,
}

#[derive(Debug)]
pub(crate) struct CompileFailure {
    pub(crate) diagnostics: Vec<u8>,
}

fn failure(code: &str) -> CompileFailure {
    CompileFailure {
        diagnostics: format!("{code}\t$\n").into_bytes(),
    }
}

fn session_diagnostics(diagnostics: &DiagnosticSet) -> CompileFailure {
    let mut bytes = Vec::new();
    for diagnostic in diagnostics.diagnostics() {
        bytes.extend_from_slice(diagnostic.code.as_str().as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(diagnostic.path.to_string().as_bytes());
        bytes.push(b'\n');
    }
    CompileFailure { diagnostics: bytes }
}

fn boxed_zeroed(bytes: u64) -> Result<Box<[u8]>, CompileFailure> {
    Ok(FixedBytes::try_new(bytes)?.bytes)
}

fn checked_layout<T>(count: usize) -> Result<u64, CompileFailure> {
    let layout = Layout::array::<T>(count).map_err(|_| failure("capi.resource.arithmetic"))?;
    u64::try_from(layout.size()).map_err(|_| failure("capi.resource.platform"))
}

fn checked_byte_layout(bytes: u64) -> Result<u64, CompileFailure> {
    checked_layout::<u8>(usize::try_from(bytes).map_err(|_| failure("capi.resource.platform"))?)
}

#[derive(Clone, Copy)]
struct CapiResources {
    active_retained: u64,
    epoch_retained: u64,
    prepared_protocol_retained: u64,
    largest: u64,
}

#[derive(Clone, Copy)]
struct CompiledModelAdmission {
    retained_bytes: u64,
    largest_allocation_bytes: u64,
}

fn compiled_model_admission(
    current: &CompiledSession,
    prospective: &CompiledSession,
) -> Result<CompiledModelAdmission, CompileFailure> {
    Ok(CompiledModelAdmission {
        retained_bytes: current
            .resource_estimate()
            .compiled_model_bytes
            .checked_add(prospective.resource_estimate().compiled_model_bytes)
            .ok_or_else(|| failure("capi.resource.arithmetic"))?,
        largest_allocation_bytes: current
            .resource_estimate()
            .single_allocation_bytes
            .max(prospective.resource_estimate().single_allocation_bytes),
    })
}

#[repr(C)]
struct SharedArcAllocation<T> {
    strong: core::sync::atomic::AtomicUsize,
    weak: core::sync::atomic::AtomicUsize,
    value: T,
}

fn checked_sum(rows: &[u64]) -> Result<u64, CompileFailure> {
    rows.iter().try_fold(0_u64, |total, row| {
        total
            .checked_add(*row)
            .ok_or_else(|| failure("capi.resource.arithmetic"))
    })
}

fn protocol_queue_config(
    limits: CompileLimits,
    quantum_frames: usize,
) -> Result<ProtocolQueueConfig, CompileFailure> {
    let one = NonZeroUsize::new(1).expect("one is nonzero");
    Ok(ProtocolQueueConfig {
        control_command_slots: one,
        control_command_bytes: NonZeroUsize::new(
            usize::try_from(limits.maximum_control_frame_bytes)
                .map_err(|_| failure("capi.resource.platform"))?,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        automation_batch_slots: one,
        reliable_response_slots: one,
        reliable_event_slots: NonZeroUsize::new(2).expect("two is nonzero"),
        telemetry_slots: one,
        per_block_automation_density: NonZeroUsize::new(
            limits.maximum_automation_spans_per_block as usize,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        quantum_frames: NonZeroUsize::new(quantum_frames)
            .ok_or_else(|| failure("capi.resource.limit"))?,
    })
}

fn capi_resources(
    limits: CompileLimits,
    canonical_bytes: usize,
    source_count: usize,
    source_id_bytes: usize,
    quantum_frames: usize,
) -> Result<CapiResources, CompileFailure> {
    let queue_config = protocol_queue_config(limits, quantum_frames)?;
    let queue = ProtocolQueues::resource_report_for_config(queue_config)
        .map_err(|_| failure("capi.resource.arithmetic"))?;
    let replay_config = ReplayCacheConfig {
        entries: NonZeroUsize::new(
            usize::try_from(limits.maximum_replay_entries)
                .map_err(|_| failure("capi.resource.platform"))?,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        bytes: NonZeroUsize::new(
            usize::try_from(limits.maximum_replay_bytes)
                .map_err(|_| failure("capi.resource.platform"))?,
        )
        .ok_or_else(|| failure("capi.resource.limit"))?,
        max_response_bytes: usize::try_from(limits.maximum_control_frame_bytes)
            .map_err(|_| failure("capi.resource.platform"))?,
    };
    let replay = ReplayCache::resource_report_for_config(replay_config)
        .map_err(|_| failure("capi.resource.arithmetic"))?;
    let exchange = plan_exchange_resource_report(PlanExchangeConfig {
        publication_capacity: NonZeroUsize::new(1).expect("one is nonzero"),
        retirement_capacity: NonZeroUsize::new(1).expect("one is nonzero"),
    })
    .map_err(|_| failure("capi.resource.arithmetic"))?;
    let epoch_rows = [
        checked_layout::<u8>(canonical_bytes)?,
        checked_layout::<ControlSource>(source_count)?,
        checked_layout::<u8>(source_id_bytes)?,
    ];
    let maximum_configuration_items = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure("capi.resource.platform"))?
        / size_of::<u16>();
    let fixed_allocation_rows = [
        checked_byte_layout(limits.maximum_diagnostic_bytes)?,
        checked_byte_layout(limits.maximum_diagnostic_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_layout::<SharedArcAllocation<AtomicU64>>(1)?,
        checked_layout::<SharedArcAllocation<SharedPlanState>>(1)?,
        checked_layout::<Option<miso_engine_protocol::Diagnostic>>(2)?,
        // ProtocolController and MockProvider each retain their own complete telemetry config.
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<miso_engine_protocol::CounterId>(maximum_configuration_items)?,
        checked_layout::<miso_engine_protocol::CounterValue>(maximum_configuration_items)?,
        checked_layout::<u32>(maximum_configuration_items)?,
        checked_layout::<miso_engine_protocol::CounterId>(maximum_configuration_items)?,
        checked_layout::<ProviderEpoch>(2)?,
        checked_layout::<(u64, PlanResourceReport)>(2)?,
        checked_layout::<crate::Session>(1)?,
        checked_layout::<crate::Plan>(1)?,
    ];
    let fixed_aggregate_rows = [
        queue.retained_payload_bytes,
        replay.retained_payload_bytes,
        exchange.retained_payload_bytes,
    ];
    let prepared_protocol_allocation_rows = [
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_layout::<miso_engine_protocol::PreparedStructuralCommand>(1)?,
    ];
    let prepared_protocol_aggregate_rows = [replay.retained_payload_bytes];
    let epoch_retained = checked_sum(&epoch_rows)?;
    let active_retained = checked_sum(&fixed_allocation_rows)?
        .checked_add(checked_sum(&fixed_aggregate_rows)?)
        .and_then(|value| value.checked_add(epoch_retained))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let prepared_protocol_retained = checked_sum(&prepared_protocol_allocation_rows)?
        .checked_add(checked_sum(&prepared_protocol_aggregate_rows)?)
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let largest = epoch_rows
        .into_iter()
        .chain(fixed_allocation_rows)
        .chain(prepared_protocol_allocation_rows)
        .chain([
            queue.largest_allocation_bytes,
            replay.largest_allocation_bytes,
            exchange.largest_allocation_bytes,
        ])
        .max()
        .unwrap_or(0);
    Ok(CapiResources {
        active_retained,
        epoch_retained,
        prepared_protocol_retained,
        largest,
    })
}

fn count_effects(model: &miso_engine_session::SessionTomlV1) -> Result<u64, CompileFailure> {
    model.tracks.iter().try_fold(0_u64, |total, track| {
        let count = track
            .simd1
            .effects
            .len()
            .checked_add(track.dynamic.effects.len())
            .and_then(|value| value.checked_add(track.simd2.effects.len()))
            .ok_or_else(|| failure("capi.resource.arithmetic"))?;
        total
            .checked_add(u64::try_from(count).map_err(|_| failure("capi.resource.platform"))?)
            .ok_or_else(|| failure("capi.resource.arithmetic"))
    })
}

fn compiled_capi_resources(
    compiled: &CompiledSession,
    limits: CompileLimits,
) -> Result<(CapiResources, usize), CompileFailure> {
    let source_id_bytes =
        compiled
            .normalized_model()
            .sources
            .iter()
            .try_fold(0_usize, |total, source| {
                total
                    .checked_add(source.id.as_str().len())
                    .ok_or_else(|| failure("capi.resource.arithmetic"))
            })?;
    Ok((
        capi_resources(
            limits,
            compiled.canonical_toml().len(),
            compiled.source_count(),
            source_id_bytes,
            compiled.quantum().0 as usize,
        )?,
        source_id_bytes,
    ))
}

fn validate_replacement_peak(
    current: PlanResourceReport,
    prospective: PlanResourceReport,
    prospective_capi: CapiResources,
    compiled_models: CompiledModelAdmission,
    limits: CompileLimits,
) -> Result<(), CompileFailure> {
    let combined = |left: u64, right: u64| {
        left.checked_add(right)
            .ok_or_else(|| failure("capi.resource.arithmetic"))
    };
    if combined(
        current.graph_session_plus_plan_bytes,
        prospective.graph_session_plus_plan_bytes,
    )?
    .checked_add(compiled_models.retained_bytes)
    .ok_or_else(|| failure("capi.resource.arithmetic"))?
        > limits.maximum_graph_session_plus_plan_bytes
    {
        return Err(failure("graph.resource.limit"));
    }
    if combined(current.source_total_bytes, prospective.source_total_bytes)?
        > limits.maximum_source_total_bytes
        || combined(
            current.source_overhead_bytes,
            prospective.source_overhead_bytes,
        )? > limits.maximum_source_overhead_bytes
    {
        return Err(failure("source.resource.limit"));
    }
    if combined(
        current.effect_scalar_state_bytes,
        prospective.effect_scalar_state_bytes,
    )? > limits.maximum_effect_state_bytes
        || combined(
            current.effect_scalar_scratch_bytes,
            prospective.effect_scalar_scratch_bytes,
        )? > limits.maximum_effect_scratch_bytes
    {
        return Err(failure("effect.resource.limit"));
    }
    if combined(
        current.builtin_retained_payload_bytes,
        prospective.builtin_retained_payload_bytes,
    )? > limits.maximum_builtin_retained_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    let capi_peak = current
        .capi_retained_bytes
        .checked_add(prospective_capi.epoch_retained)
        .and_then(|value| value.checked_add(prospective_capi.prepared_protocol_retained))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    if capi_peak > limits.maximum_capi_retained_bytes {
        return Err(failure("capi.resource.limit"));
    }
    if current
        .largest_named_allocation_bytes
        .max(prospective.largest_named_allocation_bytes)
        .max(prospective_capi.largest)
        .max(compiled_models.largest_allocation_bytes)
        > limits.maximum_named_allocation_bytes
    {
        return Err(failure("capi.resource.limit"));
    }
    Ok(())
}

fn all_limits_nonzero(limits: CompileLimits) -> bool {
    limits.source_ring_frames != 0
        && limits.maximum_automation_spans_per_block != 0
        && [
            limits.maximum_toml_bytes,
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
        && limits.reserved == [0; 4]
        && all_limits_nonzero(limits)
}

struct IdentityProcessor;

impl GraphRuntimeProcessor for IdentityProcessor {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

fn prepare_runtime(
    compiled: &CompiledSession,
    limits: CompileLimits,
) -> Result<PreparedRuntime, CompileFailure> {
    let model = compiled.normalized_model();
    let track_count = u64::try_from(model.tracks.len()).map_err(|_| failure("capi.count"))?;
    let source_count = u64::try_from(model.sources.len()).map_err(|_| failure("capi.count"))?;
    let route_count = u64::try_from(model.routes.len()).map_err(|_| failure("capi.count"))?;
    let effect_count = count_effects(model)?;
    if track_count > limits.maximum_tracks
        || source_count > limits.maximum_sources
        || route_count > limits.maximum_routes
        || effect_count > limits.maximum_effects
    {
        return Err(failure("capi.resource.count"));
    }

    if !matches!(compiled.sample_rate().0, 44_100 | 48_000 | 88_200 | 96_000) {
        return Err(failure("capi.sample_rate.unsupported"));
    }
    if limits.source_ring_frames < compiled.quantum().0
        || !limits
            .source_ring_frames
            .is_multiple_of(compiled.quantum().0)
    {
        return Err(failure("capi.source.ring_frames"));
    }

    let (capi, source_id_bytes) = compiled_capi_resources(compiled, limits)?;
    if capi.active_retained > limits.maximum_capi_retained_bytes
        || capi.largest > limits.maximum_named_allocation_bytes
    {
        return Err(failure("capi.resource.limit"));
    }

    let mut ids = Vec::new();
    ids.try_reserve_exact(source_id_bytes)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let mut controls = Vec::new();
    controls
        .try_reserve_exact(compiled.source_count())
        .map_err(|_| failure("capi.resource.allocation"))?;
    let mut graph_sources = Vec::new();
    graph_sources
        .try_reserve_exact(compiled.source_count())
        .map_err(|_| failure("capi.resource.allocation"))?;
    for source in &compiled.normalized_model().sources {
        if source.sample_rate_hz != compiled.sample_rate().0 {
            return Err(failure("source.rate.mismatch"));
        }
        let region_end = source
            .mapping
            .region
            .start_sample
            .checked_add(source.mapping.region.length_samples)
            .ok_or_else(|| failure("source.region.overflow"))?;
        let config = PcmSourceRingConfig {
            channel_count: u32::from(source.mapping.channel_count),
            quantum_frames: compiled.quantum(),
            frame_capacity: u64::from(limits.source_ring_frames),
            initial_generation: SourceGeneration(1),
        };
        let (producer, consumer, resources) = PcmSourceRing::prepare_host_region(
            config,
            SourceFrame(source.mapping.region.start_sample),
        )
        .map_err(|_| failure("source.resource.prepare"))?;
        let id_offset = ids.len();
        ids.extend_from_slice(source.id.as_str().as_bytes());
        controls.push(ControlSource {
            id_offset,
            id_bytes: source.id.as_str().len(),
            sample_rate_hz: source.sample_rate_hz,
            channel_count: u32::from(source.mapping.channel_count),
            region_start: source.mapping.region.start_sample,
            region_end,
            provider: producer.into_host_chunk_provider(SampleRateHz(source.sample_rate_hz)),
        });
        graph_sources.push(SourceGraphSource::new(consumer, resources, 0, 0));
    }
    controls.sort_unstable_by(|left, right| {
        ids[left.id_offset..left.id_offset + left.id_bytes]
            .cmp(&ids[right.id_offset..right.id_offset + right.id_bytes])
    });

    let mappings = compiled
        .normalized_model()
        .tracks
        .iter()
        .map(|track| {
            let source_index = compiled
                .source_index(&track.source_id)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or_else(|| failure("source.graph.mapping"))?;
            Ok(SourceGraphTrackMapping {
                node: GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(track.id.as_str())
                        .ok_or_else(|| failure("source.graph.mapping"))?,
                    stage: TrackStage::Input,
                },
                source_index,
                left_channel: u32::from(track.left_source_channel),
                right_channel: u32::from(track.right_source_channel),
            })
        })
        .collect::<Result<Vec<_>, CompileFailure>>()?;

    let registry = launch_native_effect_registry_v1().map_err(|_| failure("effect.registry"))?;
    let effects = prepare_native_session_effects(
        compiled,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: limits.maximum_effect_state_bytes,
            maximum_scratch_bytes: limits.maximum_effect_scratch_bytes,
            maximum_automation_spans_per_block: limits.maximum_automation_spans_per_block,
        },
    )
    .map_err(|diagnostics| {
        let mut bytes = Vec::new();
        for diagnostic in diagnostics.0 {
            bytes.extend_from_slice(diagnostic.code.as_bytes());
            bytes.push(b'\t');
            bytes.extend_from_slice(diagnostic.path.as_bytes());
            bytes.push(b'\n');
        }
        CompileFailure { diagnostics: bytes }
    })?;
    let (effect_state_bytes, effect_scratch_bytes) =
        effects
            .entries
            .iter()
            .try_fold((0_u64, 0_u64), |total, entry| {
                Ok::<_, CompileFailure>((
                    total
                        .0
                        .checked_add(
                            entry
                                .metadata
                                .state_sizes
                                .total()
                                .ok_or_else(|| failure("effect.resource.arithmetic"))?,
                        )
                        .ok_or_else(|| failure("effect.resource.arithmetic"))?,
                    total
                        .1
                        .checked_add(entry.metadata.scratch_bytes)
                        .ok_or_else(|| failure("effect.resource.arithmetic"))?,
                ))
            })?;
    if effect_state_bytes > limits.maximum_effect_state_bytes
        || effect_scratch_bytes > limits.maximum_effect_scratch_bytes
    {
        return Err(failure("effect.resource.limit"));
    }

    let builtins = prepare_session_builtins(
        compiled,
        &[],
        BuiltinCompileCaps {
            maximum_total_state_bytes: limits.maximum_builtin_retained_bytes,
            maximum_total_retained_payload_bytes: limits.maximum_builtin_retained_bytes,
            maximum_total_meter_items: limits.maximum_meter_items,
            maximum_total_meter_bytes: limits.maximum_meter_bytes,
            maximum_single_allocation_bytes: limits.maximum_named_allocation_bytes,
            maximum_meter_streams: limits.maximum_meter_streams,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .map_err(|diagnostics| {
        let mut bytes = Vec::new();
        for diagnostic in diagnostics.0 {
            bytes.extend_from_slice(diagnostic.code.as_bytes());
            bytes.push(b'\t');
            bytes.extend_from_slice(diagnostic.path.as_bytes());
            bytes.push(b'\n');
        }
        CompileFailure { diagnostics: bytes }
    })?;
    let builtin_resources = builtins.resource_report();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 1,
        effects,
        builtins,
        caps: GraphCompileCaps {
            maximum_nodes: u64::MAX,
            maximum_edges: u64::MAX,
            maximum_schedule_items: u64::MAX,
            maximum_dependency_levels: u64::MAX,
            maximum_audio_buffer_samples: u64::MAX,
            maximum_delay_samples_per_edge: u64::MAX,
            maximum_total_delay_samples: u64::MAX,
            maximum_graph_bytes: limits.maximum_graph_session_plus_plan_bytes,
            maximum_plan_bytes: limits.maximum_graph_session_plus_plan_bytes,
            maximum_single_allocation_bytes: limits.maximum_named_allocation_bytes,
            maximum_finite_tail_samples: u64::MAX,
        },
    })
    .map_err(|failure_value| {
        let mut bytes = Vec::new();
        for diagnostic in failure_value.diagnostics.diagnostics() {
            bytes.extend_from_slice(diagnostic.code.as_bytes());
            bytes.push(b'\t');
            bytes.extend_from_slice(diagnostic.path.as_bytes());
            bytes.push(b'\n');
        }
        CompileFailure { diagnostics: bytes }
    })?;
    let graph_report: GraphCompileReport = artifact.report().clone();
    let graph_resources = artifact.graph_resource_estimate().clone();
    let admitted_graph_and_model = graph_resources
        .session_plus_plan_bytes
        .checked_add(compiled.resource_estimate().compiled_model_bytes)
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    if admitted_graph_and_model > limits.maximum_graph_session_plus_plan_bytes {
        return Err(failure("graph.resource.limit"));
    }
    let source_set = prepare_graph_source_set(artifact.envelope(), graph_sources, mappings)
        .map_err(|_| failure("source.graph.prepare"))?;
    let source_resources = source_set.resource_report();
    if source_resources.total_engine_owned_bytes > limits.maximum_source_total_bytes
        || source_resources.overhead_bytes > limits.maximum_source_overhead_bytes
    {
        return Err(failure("source.resource.limit"));
    }
    let largest_named = graph_resources
        .largest_allocation_bytes
        .max(source_resources.largest_allocation_bytes)
        .max(builtin_resources.maximum_single_allocation_bytes)
        .max(capi.largest);
    if largest_named.max(compiled.resource_estimate().single_allocation_bytes)
        > limits.maximum_named_allocation_bytes
        || builtin_resources.engine_owned_retained_payload_bytes
            > limits.maximum_builtin_retained_bytes
    {
        return Err(failure("capi.resource.limit"));
    }

    let external_nodes = artifact
        .external_binding_nodes()
        .filter(|node| {
            !matches!(
                node,
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                }
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    let bindings = GraphRuntimeBindings {
        envelope: artifact.envelope(),
        nodes: external_nodes
            .into_iter()
            .map(|node| GraphNodeBinding::new(node, Box::new(IdentityProcessor)))
            .collect(),
        observers: Vec::new(),
    };
    let bound = artifact
        .into_bound_with_source_set(bindings, source_set)
        .map_err(|failure_value| failure(failure_value.code))?;
    if !bound.meter_consumers.is_empty() {
        return Err(failure("builtin.meter.unexpected"));
    }

    let (tail_kind, tail_samples) = match graph_report.output_tail {
        TailSamples::Finite(samples) => (TAIL_FINITE, samples),
        TailSamples::Infinite => (TAIL_INFINITE, 0),
    };
    let resources = PlanResourceReport {
        struct_size: crate::PLAN_RESOURCE_REPORT_SIZE,
        abi_version: ABI_VERSION,
        sample_rate_hz: compiled.sample_rate().0,
        quantum_frames: compiled.quantum().0,
        source_count,
        track_count,
        latency_samples: graph_report.output_latency.0,
        tail_kind,
        tail_samples,
        graph_session_plus_plan_bytes: graph_resources.session_plus_plan_bytes,
        graph_incremental_plan_bytes: graph_resources.incremental_plan_bytes,
        graph_metadata_bytes: graph_resources.graph_metadata_bytes,
        graph_delay_bytes: graph_resources.delay_bytes,
        effect_bank_scratch_bytes: graph_resources.effect_bank_scratch_bytes,
        effect_bank_runtime_buffer_bytes: graph_resources.effect_bank_runtime_buffer_bytes,
        effect_bank_metadata_bytes: graph_resources.effect_bank_metadata_bytes,
        builtin_bank_bytes: graph_resources.builtin_bank_bytes,
        builtin_bank_scratch_bytes: graph_resources.builtin_bank_scratch_bytes,
        source_pcm_payload_bytes: source_resources.pcm_payload_already_charged_bytes,
        source_overhead_bytes: source_resources.overhead_bytes,
        source_total_bytes: source_resources.total_engine_owned_bytes,
        effect_scalar_state_bytes: effect_state_bytes,
        effect_scalar_scratch_bytes: effect_scratch_bytes,
        builtin_processor_payload_bytes: builtin_resources.engine_owned_processor_payload_bytes,
        builtin_meter_payload_bytes: builtin_resources.engine_owned_meter_payload_bytes,
        builtin_retained_payload_bytes: builtin_resources.engine_owned_retained_payload_bytes,
        capi_retained_bytes: capi.active_retained,
        largest_named_allocation_bytes: largest_named,
        reserved: [0; 4],
    };

    Ok(PreparedRuntime {
        source_ids: ids.into_boxed_slice(),
        sources: controls.into_boxed_slice(),
        plan: bound.plan,
        resources,
    })
}

pub(crate) fn compile_children(
    toml: &str,
    limits: CompileLimits,
) -> Result<CompiledChildren, CompileFailure> {
    let model = parse_session_toml(toml).map_err(|value| session_diagnostics(&value))?;
    let source_count = u64::try_from(model.sources.len()).map_err(|_| failure("capi.count"))?;
    let aggregate_ring_frames = source_count
        .checked_mul(u64::from(limits.source_ring_frames))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let compile_caps = CompileCaps {
        max_compiled_model_bytes: limits.maximum_graph_session_plus_plan_bytes,
        max_requested_runtime_bytes: limits.maximum_graph_session_plus_plan_bytes,
        max_single_allocation_bytes: limits.maximum_named_allocation_bytes,
        max_queue_items: u64::MAX,
        max_source_ring_frames: aggregate_ring_frames,
        max_source_ring_bytes: limits.maximum_source_total_bytes,
    };
    let store =
        SessionStore::new(model, compile_caps).map_err(|value| session_diagnostics(&value))?;
    let runtime = prepare_runtime(store.compiled(), limits)?;

    let control_bytes = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure("capi.resource.platform"))?;
    let replay_bytes = usize::try_from(limits.maximum_replay_bytes)
        .map_err(|_| failure("capi.resource.platform"))?;
    let replay_entries = usize::try_from(limits.maximum_replay_entries)
        .map_err(|_| failure("capi.resource.platform"))?;
    let quantum_frames = usize::try_from(store.compiled().quantum().0)
        .map_err(|_| failure("capi.resource.platform"))?;
    let maximum_tlvs = u32::try_from(control_bytes / size_of::<u16>()).unwrap_or(u32::MAX);
    let codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: control_bytes,
        max_tlv_count: maximum_tlvs,
        max_string_bytes: control_bytes,
        max_nesting: 4,
    });
    let one = NonZeroUsize::new(1).expect("one is nonzero");
    let queues = ProtocolQueues::prepare(protocol_queue_config(limits, quantum_frames)?)
        .map_err(|_| failure("capi.protocol.queue"))?;
    let replay = ReplayCache::try_new(ReplayCacheConfig {
        entries: NonZeroUsize::new(replay_entries).ok_or_else(|| failure("capi.resource.limit"))?,
        bytes: NonZeroUsize::new(replay_bytes).ok_or_else(|| failure("capi.resource.limit"))?,
        max_response_bytes: control_bytes,
    })
    .map_err(|_| failure("capi.resource.allocation"))?;
    let retained_capacity = ControllerRetainedCapacity {
        meter_handles: maximum_tlvs as usize,
        counter_ids: maximum_tlvs as usize,
    };
    let provider = MockProvider::try_with_retained_capacity(retained_capacity)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let controller = ProtocolController::try_with_config_and_retained_capacity(
        store,
        queues,
        provider,
        replay,
        codec,
        ProtocolControllerConfig {
            maximum_transaction_edits: maximum_tlvs,
            maximum_response_diagnostics: u16::MAX,
            provider_features: ProviderFeatures::ALL,
        },
        retained_capacity,
    )
    .map_err(|_| failure("capi.resource.allocation"))?;

    let PreparedRuntime {
        source_ids,
        sources,
        plan,
        resources,
    } = runtime;
    let (publisher, owner, retirer) = plan_exchange(
        plan,
        PlanExchangeConfig {
            publication_capacity: one,
            retirement_capacity: one,
        },
    )
    .map_err(|_| failure("capi.plan.exchange"))?;
    let mut reports = Vec::new();
    reports
        .try_reserve_exact(2)
        .map_err(|_| failure("capi.resource.allocation"))?;
    reports.push((0, resources));
    let shared = Arc::new(SharedPlanState {
        plan_alive: AtomicBool::new(true),
        active_epoch: AtomicU64::new(0),
        reports: Mutex::new(reports),
        #[cfg(test)]
        render_sequence: AtomicU64::new(0),
        #[cfg(test)]
        render_sample: AtomicU64::new(0),
    });
    let mut pending_providers = Vec::new();
    pending_providers
        .try_reserve_exact(1)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let mut retired_providers = Vec::new();
    retired_providers
        .try_reserve_exact(1)
        .map_err(|_| failure("capi.resource.allocation"))?;
    let decode_field_count = control_bytes / size_of::<u16>();
    let mut decode_fields = Vec::new();
    decode_fields
        .try_reserve_exact(decode_field_count)
        .map_err(|_| failure("capi.resource.allocation"))?;
    decode_fields.resize(decode_field_count, 0);
    let decode_tail = if control_bytes.is_multiple_of(size_of::<u16>()) {
        None
    } else {
        Some(boxed_zeroed(1)?)
    };

    Ok(CompiledChildren {
        session: SessionState {
            controller,
            providers: ProviderEpoch {
                epoch: 0,
                source_ids,
                sources,
            },
            pending_providers,
            retired_providers,
            publisher,
            retirer,
            limits,
            decode_fields: decode_fields.into_boxed_slice(),
            _decode_tail: decode_tail,
            response_scratch: boxed_zeroed(limits.maximum_control_frame_bytes)?,
            shared: Arc::clone(&shared),
            #[cfg(test)]
            observed_render_sequence: 0,
            #[cfg(test)]
            test_faults: [None; 2],
            #[cfg(test)]
            test_fault_index: 0,
            #[cfg(test)]
            test_owner_counters: TestOwnerCounters {
                current_provider_constructed: 1,
                current_plan_constructed: 1,
                replay_current_constructed: 1,
                ..TestOwnerCounters::default()
            },
        },
        session_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
        plan: PlanState {
            owner,
            shared,
            quantum_frames: resources.quantum_frames,
            next_absolute_sample: 0,
        },
        plan_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
    })
}

impl PlanState {
    pub(crate) const fn quantum_frames(&self) -> u32 {
        self.quantum_frames
    }

    pub(crate) fn resources(&self) -> PlanResourceReport {
        let active = self.shared.active_epoch.load(Ordering::Acquire);
        self.shared
            .reports
            .lock()
            .expect("plan resource report lock is not poisoned")
            .iter()
            .find_map(|(epoch, report)| (*epoch == active).then_some(*report))
            .expect("active plan epoch retains its resource report")
    }

    pub(crate) const fn next_absolute_sample(&self) -> u64 {
        self.next_absolute_sample
    }

    pub(crate) fn render(
        &mut self,
        absolute_sample: u64,
        output: PlanarBufferMut<'_>,
    ) -> Result<(), ()> {
        if absolute_sample != self.next_absolute_sample {
            return Err(());
        }
        let report = self
            .owner
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime { absolute_sample },
            )
            .map_err(|_| ())?;
        self.shared
            .active_epoch
            .store(report.active_epoch.0, Ordering::Release);
        #[cfg(test)]
        {
            self.shared
                .render_sample
                .store(report.render.next_absolute_sample, Ordering::Release);
            self.shared.render_sequence.fetch_add(1, Ordering::AcqRel);
        }
        self.next_absolute_sample = report.render.next_absolute_sample;
        Ok(())
    }
}

impl Drop for PlanState {
    fn drop(&mut self) {
        self.shared.plan_alive.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
pub(crate) enum CommandError {
    Invalid,
    BufferTooSmall { required: u64 },
    Backpressure,
    CompileRejected(CompileFailure),
    Internal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EventLane {
    Reliable,
    Lossy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EventError {
    BufferTooSmall { required: u64 },
    Backpressure,
    Internal,
}

fn map_encode_error(error: EncodeError) -> CommandError {
    match error {
        EncodeError::OutputTooSmall { required } => CommandError::BufferTooSmall {
            required: u64::try_from(required).unwrap_or(u64::MAX),
        },
        EncodeError::MessageKindMismatch | EncodeError::LimitExceeded => CommandError::Internal,
    }
}

fn map_command_process_error(error: CommandFrameProcessError) -> CommandError {
    match error {
        CommandFrameProcessError::Uncorrelatable(_) => CommandError::Invalid,
        CommandFrameProcessError::Encode(error) => map_encode_error(error),
        CommandFrameProcessError::OutputReservationTooSmall { required } => {
            CommandError::BufferTooSmall {
                required: u64::try_from(required).unwrap_or(u64::MAX),
            }
        }
        CommandFrameProcessError::PreparedCommandOutstanding => CommandError::Backpressure,
        CommandFrameProcessError::Internal => CommandError::Internal,
    }
}

fn map_event_egress_error(error: EventEgressError) -> EventError {
    match error {
        EventEgressError::Encode(EncodeError::OutputTooSmall { required }) => {
            EventError::BufferTooSmall {
                required: u64::try_from(required).unwrap_or(u64::MAX),
            }
        }
        EventEgressError::ReliableQueueFull(_) => EventError::Backpressure,
        EventEgressError::Disabled
        | EventEgressError::DiagnosticStorageFull
        | EventEgressError::Encode(_) => EventError::Internal,
    }
}

impl SessionState {
    #[cfg(test)]
    pub(crate) fn test_set_structural_faults(
        &mut self,
        faults: [Option<TestStructuralFaultPhase>; 2],
    ) {
        self.test_faults = faults;
        self.test_fault_index = 0;
    }

    #[cfg(test)]
    pub(crate) const fn test_owner_counters(&self) -> TestOwnerCounters {
        self.test_owner_counters
    }

    #[cfg(test)]
    fn take_test_fault(&mut self, phase: TestStructuralFaultPhase) -> bool {
        if self
            .test_faults
            .get(self.test_fault_index)
            .copied()
            .flatten()
            == Some(phase)
        {
            self.test_fault_index += 1;
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    fn record_test_candidate_cancel(&mut self, runtime: bool, reservation: bool) {
        self.test_owner_counters.token_disposed += 1;
        self.test_owner_counters.replay_candidate_disposed += 1;
        if runtime {
            self.test_owner_counters.candidate_provider_disposed += 1;
            self.test_owner_counters.candidate_plan_disposed += 1;
        }
        if reservation {
            self.test_owner_counters.reservation_canceled += 1;
        }
    }

    #[cfg(test)]
    fn collect_render_activity(&mut self) {
        let sequence = self.shared.render_sequence.load(Ordering::Acquire);
        if sequence == self.observed_render_sequence {
            return;
        }
        self.observed_render_sequence = sequence;
        let sample = self.shared.render_sample.load(Ordering::Acquire);
        let revision = self.controller.session().revision();
        for (index, handle) in [1_u32, 2].into_iter().enumerate() {
            let _ = self.controller.stage_meter_batch_event(
                revision,
                miso_engine_protocol::SampleTime(sample),
                &[miso_engine_protocol::MeterRecord {
                    handle,
                    component: miso_engine_protocol::MeterComponent::Left,
                    flags: 1,
                    value: sequence as f32 + index as f32 * 0.25,
                }],
            );
        }
        let _ = self.controller.stage_counter_snapshot_event(
            revision,
            &miso_engine_protocol::CounterSnapshot {
                observed_sample: miso_engine_protocol::SampleTime(sample),
                values: vec![miso_engine_protocol::CounterValue {
                    id: miso_engine_protocol::CounterId::ControlCommandBackpressure,
                    value: sequence,
                }],
            },
        );
        let _ = self.controller.enqueue_diagnostic_event(
            revision,
            miso_engine_protocol::DiagnosticEvent {
                diagnostic: miso_engine_protocol::Diagnostic {
                    code: "capi.render.activity".to_owned(),
                    severity: miso_engine_protocol::DiagnosticSeverity::Info,
                    path: Vec::new(),
                    detail: None,
                    operation_index: None,
                    sample_time: Some(sample),
                    provider_sequence: Some(sequence),
                },
            },
        );
    }

    #[cfg(test)]
    pub(crate) fn test_install_parameter(
        &mut self,
        descriptor: miso_engine_protocol::ParameterDescriptor,
        state: miso_engine_protocol::ParameterStateRecord,
    ) {
        self.controller
            .install_conformance_parameter(descriptor, state);
    }

    #[cfg(test)]
    pub(crate) fn test_state_summary(&self) -> (u64, usize, u64, usize) {
        (
            self.controller.session().revision().0,
            self.controller.replay().len(),
            self.providers.epoch,
            self.pending_providers.len(),
        )
    }

    #[cfg(test)]
    pub(crate) fn test_enqueue_reliable(
        &mut self,
        event: miso_engine_protocol::ReliableSlot,
    ) -> Result<(), ()> {
        self.controller
            .queues_mut()
            .try_enqueue_event(event)
            .map_err(|_| ())
    }

    #[cfg(test)]
    pub(crate) fn test_telemetry_counters(&self) -> miso_engine_protocol::TelemetryCounters {
        self.controller.queues().telemetry_counters()
    }

    #[cfg(test)]
    pub(crate) fn test_set_capi_retained_limit(&mut self, bytes: u64) {
        self.limits.maximum_capi_retained_bytes = bytes;
    }

    #[cfg(test)]
    pub(crate) fn test_retained_capacities(&self) -> [usize; 7] {
        let controller = self.controller.retained_configuration_capacity();
        let (provider, provider_counters) = self.controller.provider().retained_capacities();
        let replay = self.controller.replay().retained_storage_capacities();
        [
            controller.meter_handles,
            controller.counter_ids,
            provider.meter_handles,
            provider.counter_ids,
            provider_counters,
            replay.0,
            replay.1,
        ]
    }

    fn active_resource_report(&self) -> Result<PlanResourceReport, CommandError> {
        let active = self.shared.active_epoch.load(Ordering::Acquire);
        self.shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?
            .iter()
            .find_map(|(epoch, report)| (*epoch == active).then_some(*report))
            .ok_or(CommandError::Internal)
    }

    fn synchronize_plan_epochs(&mut self) -> Result<(), CommandError> {
        let active_epoch = self.shared.active_epoch.load(Ordering::Acquire);
        if active_epoch != self.providers.epoch {
            let index = self
                .pending_providers
                .iter()
                .position(|provider| provider.epoch == active_epoch)
                .ok_or(CommandError::Internal)?;
            let next = self.pending_providers.remove(index);
            let previous = core::mem::replace(&mut self.providers, next);
            if self.retired_providers.len() == self.retired_providers.capacity() {
                return Err(CommandError::Internal);
            }
            self.retired_providers.push(previous);
        }

        while let Ok((retired_epoch, retired_plan)) = self.retirer.try_reclaim() {
            drop(retired_plan);
            #[cfg(test)]
            {
                self.test_owner_counters.current_plan_disposed += 1;
            }
            if let Some(index) = self
                .retired_providers
                .iter()
                .position(|provider| provider.epoch == retired_epoch.0)
            {
                self.retired_providers.remove(index);
                #[cfg(test)]
                {
                    self.test_owner_counters.current_provider_disposed += 1;
                }
            } else {
                return Err(CommandError::Internal);
            }
            let active = self.shared.active_epoch.load(Ordering::Acquire);
            let mut reports = self
                .shared
                .reports
                .lock()
                .map_err(|_| CommandError::Internal)?;
            if retired_epoch.0 != active
                && let Some(index) = reports
                    .iter()
                    .position(|(epoch, _)| *epoch == retired_epoch.0)
            {
                reports.remove(index);
            }
        }
        Ok(())
    }

    pub(crate) fn command(
        &mut self,
        request: &[u8],
        output_capacity: u64,
    ) -> Result<usize, CommandError> {
        self.synchronize_plan_epochs()?;
        #[cfg(test)]
        self.collect_render_activity();
        let output_capacity = usize::try_from(output_capacity).unwrap_or(usize::MAX);
        let prepared = self
            .controller
            .prepare_command_frame(
                request,
                &mut DecodeScratch::new(&mut self.decode_fields),
                output_capacity,
            )
            .map_err(map_command_process_error)?;
        match prepared {
            PreparedCommandFrame::Immediate(response) => response
                .write_into(&mut self.response_scratch)
                .map_err(map_encode_error),
            PreparedCommandFrame::Structural(prepared) => {
                #[cfg(test)]
                {
                    self.test_owner_counters.token_constructed += 1;
                    self.test_owner_counters.replay_candidate_constructed += 1;
                    if self.take_test_fault(TestStructuralFaultPhase::AfterProtocolPrepare) {
                        self.record_test_candidate_cancel(false, false);
                        return Err(CommandError::Backpressure);
                    }
                }
                if !self.shared.plan_alive.load(Ordering::Acquire) {
                    return Err(CommandError::Backpressure);
                }
                let response_len = prepared.response_len();
                if response_len > self.response_scratch.len() || response_len > output_capacity {
                    return Err(CommandError::BufferTooSmall {
                        required: u64::try_from(response_len).unwrap_or(u64::MAX),
                    });
                }
                let (prospective_capi, _) =
                    compiled_capi_resources(prepared.prospective_session().compiled(), self.limits)
                        .map_err(CommandError::CompileRejected)?;
                #[cfg(test)]
                if self.take_test_fault(TestStructuralFaultPhase::AfterResourceProjection) {
                    self.record_test_candidate_cancel(false, false);
                    return Err(CommandError::Backpressure);
                }
                let prepared_runtime = match STRUCTURAL_SOURCE_STATE_POLICY {
                    StructuralSourceStatePolicy::ResetAtReplacementBoundary => {
                        prepare_runtime(prepared.prospective_session().compiled(), self.limits)
                    }
                }
                .map_err(CommandError::CompileRejected)?;
                let PreparedRuntime {
                    source_ids,
                    sources,
                    plan,
                    resources,
                } = prepared_runtime;
                #[cfg(test)]
                {
                    self.test_owner_counters.candidate_provider_constructed += 1;
                    self.test_owner_counters.candidate_plan_constructed += 1;
                    if self.take_test_fault(TestStructuralFaultPhase::AfterRuntimePrepare) {
                        self.record_test_candidate_cancel(true, false);
                        return Err(CommandError::Backpressure);
                    }
                }
                validate_replacement_peak(
                    self.active_resource_report()?,
                    resources,
                    prospective_capi,
                    compiled_model_admission(
                        self.controller.session().compiled(),
                        prepared.prospective_session().compiled(),
                    )
                    .map_err(CommandError::CompileRejected)?,
                    self.limits,
                )
                .map_err(CommandError::CompileRejected)?;
                #[cfg(test)]
                if self.take_test_fault(TestStructuralFaultPhase::AfterAdmission) {
                    self.record_test_candidate_cancel(true, false);
                    return Err(CommandError::Backpressure);
                }
                if !self.pending_providers.is_empty() {
                    return Err(CommandError::Backpressure);
                }
                let reservation = self
                    .publisher
                    .reserve_replacement(plan)
                    .map_err(|_| CommandError::Backpressure)?;
                #[cfg(test)]
                {
                    self.test_owner_counters.reservation_constructed += 1;
                    if self
                        .test_faults
                        .get(self.test_fault_index)
                        .copied()
                        .flatten()
                        == Some(TestStructuralFaultPhase::AfterPlanReservation)
                    {
                        self.test_fault_index += 1;
                        self.test_owner_counters.token_disposed += 1;
                        self.test_owner_counters.replay_candidate_disposed += 1;
                        self.test_owner_counters.candidate_provider_disposed += 1;
                        self.test_owner_counters.candidate_plan_disposed += 1;
                        self.test_owner_counters.reservation_canceled += 1;
                        return Err(CommandError::Backpressure);
                    }
                }
                let epoch = reservation.epoch().0;
                let mut reports = self
                    .shared
                    .reports
                    .lock()
                    .map_err(|_| CommandError::Internal)?;
                if reports.len() == reports.capacity()
                    || self.pending_providers.len() == self.pending_providers.capacity()
                {
                    return Err(CommandError::Backpressure);
                }

                #[cfg(test)]
                if self
                    .test_faults
                    .get(self.test_fault_index)
                    .copied()
                    .flatten()
                    == Some(TestStructuralFaultPhase::BeforeProtocolCommit)
                {
                    self.test_fault_index += 1;
                    self.test_owner_counters.token_disposed += 1;
                    self.test_owner_counters.replay_candidate_disposed += 1;
                    self.test_owner_counters.candidate_provider_disposed += 1;
                    self.test_owner_counters.candidate_plan_disposed += 1;
                    self.test_owner_counters.reservation_canceled += 1;
                    return Err(CommandError::Backpressure);
                }

                let committed = self
                    .controller
                    .commit_prepared_structural(*prepared)
                    .map_err(|_| CommandError::Internal)?;
                self.pending_providers.push(ProviderEpoch {
                    epoch,
                    source_ids,
                    sources,
                });
                reports.push((epoch, resources));
                reservation.commit();
                #[cfg(test)]
                {
                    self.test_owner_counters.token_disposed += 1;
                    self.test_owner_counters.replay_candidate_published += 1;
                    self.test_owner_counters.replay_current_disposed += 1;
                    self.test_owner_counters.candidate_provider_published += 1;
                    self.test_owner_counters.candidate_plan_published += 1;
                    self.test_owner_counters.reservation_committed += 1;
                }
                Ok(committed
                    .write_into(&mut self.response_scratch)
                    .expect("prepared response capacity was admitted before protocol commit"))
            }
        }
    }

    pub(crate) fn command_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    pub(crate) fn dequeue_event(
        &mut self,
        lane: EventLane,
        output_capacity: u64,
    ) -> Result<Option<usize>, EventError> {
        self.synchronize_plan_epochs()
            .map_err(|error| match error {
                CommandError::Backpressure => EventError::Backpressure,
                _ => EventError::Internal,
            })?;
        #[cfg(test)]
        self.collect_render_activity();
        let capacity = usize::try_from(output_capacity)
            .unwrap_or(usize::MAX)
            .min(self.response_scratch.len());
        let output = &mut self.response_scratch[..capacity];
        let result = match lane {
            EventLane::Reliable => self.controller.dequeue_reliable_event_frame_into(output),
            EventLane::Lossy => self.controller.dequeue_lossy_event_frame_into(output),
        };
        result.map_err(map_event_egress_error)
    }

    pub(crate) fn event_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    fn source_id(&self, source: &ControlSource) -> &[u8] {
        &self.providers.source_ids[source.id_offset..source.id_offset + source.id_bytes]
    }

    fn source_index(&self, id: &[u8]) -> Option<usize> {
        self.providers
            .sources
            .binary_search_by(|source| self.source_id(source).cmp(id))
            .ok()
    }

    pub(crate) fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<miso_engine_source::SubmitReport, u32> {
        self.synchronize_plan_epochs()
            .map_err(|_| RESULT_INTERNAL)?;
        let index = self.source_index(id).ok_or(RESULT_INVALID_ARGUMENT)?;
        let source = &mut self.providers.sources[index];
        let end = submission
            .start_frame
            .checked_add(u64::from(submission.frames))
            .ok_or(RESULT_INVALID_ARGUMENT)?;
        if submission.sample_rate_hz != source.sample_rate_hz
            || u32::try_from(submission.planes.len()).ok() != Some(source.channel_count)
            || submission.start_frame < source.region_start
            || end > source.region_end
            || (submission.end_of_region && end != source.region_end)
            || (!submission.end_of_region && end == source.region_end)
        {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let generation =
            SourceGeneration::new(submission.generation).ok_or(RESULT_INVALID_ARGUMENT)?;
        source
            .provider
            .submit(HostPlanarChunk {
                sample_rate_hz: SampleRateHz(submission.sample_rate_hz),
                generation,
                start_frame: SourceFrame(submission.start_frame),
                planes: submission.planes,
                frames: submission.frames,
                end_of_region: submission.end_of_region,
            })
            .map_err(|error| match error {
                HostChunkError::Full { .. } => RESULT_BACKPRESSURE,
                HostChunkError::InternalInvariant => RESULT_INTERNAL,
                _ => RESULT_INVALID_ARGUMENT,
            })
    }

    pub(crate) fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), u32> {
        self.synchronize_plan_epochs()
            .map_err(|_| RESULT_INTERNAL)?;
        let index = self.source_index(id).ok_or(RESULT_INVALID_ARGUMENT)?;
        let source = &mut self.providers.sources[index];
        if !(source.region_start..=source.region_end).contains(&source_frame) {
            return Err(RESULT_INVALID_ARGUMENT);
        }
        let generation = SourceGeneration::new(generation).ok_or(RESULT_INVALID_ARGUMENT)?;
        source
            .provider
            .try_seek(SourceCommand::Seek {
                generation,
                frame: SourceFrame(source_frame),
            })
            .map_err(|error| match error {
                SourceSeekError::Backpressure { .. } => RESULT_BACKPRESSURE,
                _ => RESULT_INVALID_ARGUMENT,
            })
    }
}

pub(crate) struct SourceSubmission<'a> {
    pub(crate) generation: u64,
    pub(crate) start_frame: u64,
    pub(crate) sample_rate_hz: u32,
    pub(crate) planes: &'a [&'a [f32]],
    pub(crate) frames: u32,
    pub(crate) end_of_region: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_protocol::{
        CapabilityFlags, ExpectedRevision, RequestId, SessionRevision, StatusCode,
    };

    const SESSION: &str =
        include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");

    fn limits() -> CompileLimits {
        CompileLimits {
            struct_size: crate::COMPILE_LIMITS_SIZE,
            source_ring_frames: 1_024,
            maximum_automation_spans_per_block: 128,
            reserved0: 0,
            maximum_toml_bytes: 1_000_000,
            maximum_diagnostic_bytes: 4_096,
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
            maximum_capi_retained_bytes: 10_000_000,
            maximum_named_allocation_bytes: 100_000_000,
            maximum_meter_streams: 1,
            maximum_meter_items: 1,
            maximum_meter_bytes: 1,
            maximum_control_frame_bytes: 4_096,
            maximum_replay_bytes: 8_192,
            maximum_replay_entries: 16,
            reserved: [0; 4],
        }
    }

    fn generated_scratch_session() -> String {
        let mut model = parse_session_toml(SESSION).expect("accepted base session");
        for track in &mut model.tracks {
            let effect = &mut track.simd1.effects[0];
            effect.id = miso_engine_session::StableId::parse("soft-clip").expect("effect slot");
            effect.identity = miso_engine_session::EffectIdentity::Native {
                effect_id: miso_engine_session::StableId::parse("miso.soft-clip")
                    .expect("effect id"),
            };
            effect.params = vec![
                miso_engine_session::EffectParam {
                    parameter_id: 1,
                    channel: miso_engine_session::ParameterChannel::Left,
                    unit: miso_engine_session::ParameterUnit::Db,
                    value: -6.0,
                },
                miso_engine_session::EffectParam {
                    parameter_id: 1,
                    channel: miso_engine_session::ParameterChannel::Right,
                    unit: miso_engine_session::ParameterUnit::Db,
                    value: -6.0,
                },
            ];
        }
        miso_engine_session::canonical_session_toml(&model).expect("generated canonical session")
    }

    fn command_bytes(
        request_id: u64,
        payload: miso_engine_protocol::CommandPayload<'_>,
    ) -> Vec<u8> {
        command_bytes_at_revision(request_id, ExpectedRevision::Any, payload)
    }

    fn command_bytes_at_revision(
        request_id: u64,
        expected_revision: ExpectedRevision,
        payload: miso_engine_protocol::CommandPayload<'_>,
    ) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let frame = miso_engine_protocol::TypedCommandFrame {
            request_id: RequestId::new(request_id).expect("nonzero request"),
            expected_revision,
            payload,
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let len = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("typed command");
        bytes.truncate(len);
        bytes
    }

    fn typed_event_bytes(frame: &miso_engine_protocol::TypedEventFrame<'_>) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let len = codec
            .encode_event_frame_into(frame, &mut bytes)
            .expect("manual typed event oracle");
        bytes.truncate(len);
        bytes
    }

    fn pinned_hex(value: &str) -> Vec<u8> {
        assert!(value.len().is_multiple_of(2));
        value
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let digit = |value: u8| match value {
                    b'0'..=b'9' => value - b'0',
                    b'a'..=b'f' => value - b'a' + 10,
                    _ => panic!("pinned lowercase hexadecimal"),
                };
                digit(pair[0]) << 4 | digit(pair[1])
            })
            .collect()
    }

    const ALL_COMMAND_RESPONSE_VECTORS: [&str; 11] = [
        "4d49534f43544c00010000003000020001000000c801000001000000000000002a000000000000001b000000000000000100020102000000010000000000000002000201020000000000000000000000030002010200000001000000000000000400020102000000000000000000000005000401080000000010000000000000060003010400000000080000000000000700040108000000001000000000000008000101010000000400000000000000090002010200000000010000000000000a0004010800000001000000000000000b0004010800000000100000000000000c0004010800000001000000000000000d0004010800000001000000000000000e0004010800000002000000000000000f00040108000000010000000000000010000401080000001000000000000000110004010800000000200000000000001200040108000000001000000000000013000401080000008000000000000000140004010800000080000000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01160000000100020003000400050006000700080009000a000b0000001a000c010c000000018002801080208021803080000000001b00040108000000ff3f000000000000",
        "4d49534f43544c000100000030000200020000004000000002000000000000002a00000000000000040000000000000001000401080000003b250000000000000200040108000000000000000000000003000a0101000000730000000000000004000801010000000000000000000000",
        "4d49534f43544c000100000030000200040000002000000003000000000000002a0000000000000002000000000000000100030104000000000000000000000002000801010000000100000000000000",
        "4d49534f43544c00010000003000020005000d004800000004000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000",
        "4d49534f43544c00010000003000020006000d004800000005000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000",
        "4d49534f43544c000100000030000200070000003000000006000000000000002a000000000000000300000000000000010001010100000001000000000000000200040108000000000000000000000003000401080000000000000000000000",
        "4d49534f43544c000100000030000200080000003000000007000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000",
        "4d49534f43544c000100000030000200090000005000000008000000000000002a00000000000000060000000000000001000d01000000000200030104000000000000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
        "4d49534f43544c0001000000300002000a0000001000000009000000000000002a00000000000000010000000000000001000401080000000000000000000000",
        "4d49534f43544c0001000000300002000b000000200000000a000000000000002a0000000000000002000000000000000100040108000000000000000000000002000801010000000100000000000000",
        "4d49534f43544c00010000003000020003000000100000000b000000000000002b00000000000000010000000000000001000301040000000100000000000000",
    ];

    fn replacement_projection(
        children: &mut CompiledChildren,
        request: &[u8],
    ) -> (
        PlanResourceReport,
        PlanResourceReport,
        CapiResources,
        CompiledModelAdmission,
    ) {
        let current = children.plan.resources();
        let prepared = children
            .session
            .controller
            .prepare_command_frame(
                request,
                &mut DecodeScratch::new(&mut children.session.decode_fields),
                4_096,
            )
            .expect("structural projection");
        let PreparedCommandFrame::Structural(prepared) = prepared else {
            panic!("structural replacement projection")
        };
        let (production_capi, _) = compiled_capi_resources(
            prepared.prospective_session().compiled(),
            children.session.limits,
        )
        .expect("prospective CAPI resources");
        let prospective_capi = independent_capi_resources(
            prepared.prospective_session().compiled(),
            children.session.limits,
        );
        assert_eq!(
            production_capi.active_retained,
            prospective_capi.active_retained
        );
        assert_eq!(
            production_capi.epoch_retained,
            prospective_capi.epoch_retained
        );
        assert_eq!(
            production_capi.prepared_protocol_retained,
            prospective_capi.prepared_protocol_retained
        );
        assert_eq!(production_capi.largest, prospective_capi.largest);
        let prospective = prepare_runtime(
            prepared.prospective_session().compiled(),
            children.session.limits,
        )
        .expect("prospective runtime")
        .resources;
        let current_model = children
            .session
            .controller
            .session()
            .compiled()
            .resource_estimate();
        let prospective_model = prepared
            .prospective_session()
            .compiled()
            .resource_estimate();
        let models = CompiledModelAdmission {
            retained_bytes: current_model
                .compiled_model_bytes
                .checked_add(prospective_model.compiled_model_bytes)
                .expect("manual double-live model aggregate"),
            largest_allocation_bytes: current_model
                .single_allocation_bytes
                .max(prospective_model.single_allocation_bytes),
        };
        let production_models = compiled_model_admission(
            children.session.controller.session().compiled(),
            prepared.prospective_session().compiled(),
        )
        .expect("production compiled-model admission");
        assert_eq!(models.retained_bytes, production_models.retained_bytes);
        assert_eq!(
            models.largest_allocation_bytes,
            production_models.largest_allocation_bytes
        );
        drop(prepared);
        (current, prospective, prospective_capi, models)
    }

    fn independent_capi_resources(
        compiled: &CompiledSession,
        limits: CompileLimits,
    ) -> CapiResources {
        use core::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicUsize};

        #[repr(C)]
        struct RingMirror<T> {
            slots: Box<[UnsafeCell<MaybeUninit<T>>]>,
            slots_len: usize,
            logical_capacity: usize,
            generation: miso_engine_core::realtime::QueueGeneration,
            producer: AtomicUsize,
            consumer: AtomicUsize,
        }
        #[repr(C)]
        struct SharedRingMirror<T> {
            strong: AtomicUsize,
            weak: AtomicUsize,
            ring: RingMirror<T>,
        }
        #[repr(C)]
        struct SharedCounterMirror {
            strong: AtomicUsize,
            weak: AtomicUsize,
            value: AtomicUsize,
        }
        #[allow(dead_code)]
        struct SharedPlanStateMirror {
            plan_alive: AtomicBool,
            active_epoch: AtomicU64,
            reports: Mutex<Vec<(u64, PlanResourceReport)>>,
            render_sequence: AtomicU64,
            render_sample: AtomicU64,
        }
        #[repr(C)]
        struct DensityMirror {
            block: u64,
            starts: usize,
            occupied: bool,
        }
        #[repr(C)]
        struct IntervalMirror {
            record: miso_engine_protocol::AutomationRecord,
            occupied: bool,
        }
        #[allow(dead_code)]
        struct ReplayEntryMirror {
            request_id: RequestId,
            request_offset: usize,
            request_bytes: usize,
            response_offset: usize,
            response_bytes: usize,
            response_status: StatusCode,
            response_revision: SessionRevision,
            framed: bool,
        }
        #[allow(dead_code)]
        struct PublishedPlanMirror {
            epoch: miso_engine_core::realtime::PlanEpoch,
            plan: PreparedRenderPlan,
            retirement_reserved: bool,
        }
        #[allow(dead_code)]
        struct RetiredPlanMirror {
            epoch: miso_engine_core::realtime::PlanEpoch,
            plan: PreparedRenderPlan,
        }

        fn bytes<T>(count: usize) -> u64 {
            u64::try_from(Layout::array::<T>(count).expect("oracle layout").size())
                .expect("oracle platform")
        }
        fn sum(rows: &[u64]) -> u64 {
            rows.iter()
                .try_fold(0_u64, |total, value| total.checked_add(*value))
                .expect("oracle sum")
        }
        fn spsc<T>(capacity: usize) -> (u64, u64) {
            let header = bytes::<SharedRingMirror<T>>(1);
            let slots = bytes::<UnsafeCell<MaybeUninit<T>>>(
                capacity.checked_add(1).expect("oracle sentinel"),
            );
            (
                header.checked_add(slots).expect("oracle SPSC"),
                header.max(slots),
            )
        }

        let control_bytes = usize::try_from(limits.maximum_control_frame_bytes).expect("control");
        let configuration_items = control_bytes / size_of::<u16>();
        let source_id_bytes = compiled
            .normalized_model()
            .sources
            .iter()
            .map(|source| source.id.as_str().len())
            .sum::<usize>();
        let queue_pairs = [
            spsc::<miso_engine_protocol::ControlCommandSlot>(1),
            spsc::<miso_engine_protocol::AutomationBatchSlot>(1),
            spsc::<miso_engine_protocol::ReliableSlot>(1),
            spsc::<miso_engine_protocol::ReliableSlot>(2),
            spsc::<miso_engine_protocol::TelemetryRecord>(1),
            spsc::<miso_engine_protocol::CounterTelemetryRecord>(1),
        ];
        let queue_extra = [
            bytes::<Option<miso_engine_protocol::TelemetryRecord>>(1),
            bytes::<Option<miso_engine_protocol::CounterTelemetryRecord>>(1),
            bytes::<DensityMirror>(miso_engine_protocol::AUTOMATION_BATCH_RECORDS),
            bytes::<IntervalMirror>(miso_engine_protocol::AUTOMATION_BATCH_RECORDS),
            bytes::<SharedCounterMirror>(1),
        ];
        let queue_retained = queue_pairs
            .iter()
            .map(|pair| pair.0)
            .chain(queue_extra)
            .sum::<u64>();
        let queue_largest = queue_pairs
            .iter()
            .map(|pair| pair.1)
            .chain(queue_extra)
            .max()
            .expect("queue largest");
        let replay_entries = bytes::<ReplayEntryMirror>(limits.maximum_replay_entries as usize);
        let replay_retained = replay_entries
            .checked_add(limits.maximum_replay_bytes)
            .expect("replay aggregate");
        let replay_largest = replay_entries.max(limits.maximum_replay_bytes);
        let publication = spsc::<PublishedPlanMirror>(1);
        let retirement = spsc::<RetiredPlanMirror>(1);
        let counter = bytes::<SharedCounterMirror>(1);
        let exchange_retained = publication
            .0
            .checked_add(retirement.0)
            .and_then(|value| value.checked_add(counter.checked_mul(2)?))
            .expect("exchange aggregate");
        let exchange_largest = publication.1.max(retirement.1).max(counter);
        let epoch_rows = [
            bytes::<u8>(compiled.canonical_toml().len()),
            bytes::<ControlSource>(compiled.source_count()),
            bytes::<u8>(source_id_bytes),
        ];
        let fixed_allocations = [
            bytes::<u8>(limits.maximum_diagnostic_bytes as usize),
            bytes::<u8>(limits.maximum_diagnostic_bytes as usize),
            bytes::<u8>(control_bytes),
            bytes::<u8>(control_bytes),
            bytes::<SharedArcAllocation<AtomicU64>>(1),
            bytes::<SharedArcAllocation<SharedPlanStateMirror>>(1),
            bytes::<Option<miso_engine_protocol::Diagnostic>>(2),
            bytes::<u32>(configuration_items),
            bytes::<miso_engine_protocol::CounterId>(configuration_items),
            bytes::<u32>(configuration_items),
            bytes::<miso_engine_protocol::CounterId>(configuration_items),
            bytes::<miso_engine_protocol::CounterValue>(configuration_items),
            bytes::<ProviderEpoch>(2),
            bytes::<(u64, PlanResourceReport)>(2),
            bytes::<crate::Session>(1),
            bytes::<crate::Plan>(1),
        ];
        let fixed_aggregates = [queue_retained, replay_retained, exchange_retained];
        let prepared_allocations = [
            bytes::<u8>(control_bytes),
            bytes::<miso_engine_protocol::PreparedStructuralCommand>(1),
        ];
        let epoch_retained = sum(&epoch_rows);
        let active_retained = sum(&fixed_allocations)
            .checked_add(sum(&fixed_aggregates))
            .and_then(|value| value.checked_add(epoch_retained))
            .expect("active oracle");
        let prepared_protocol_retained = sum(&prepared_allocations)
            .checked_add(replay_retained)
            .expect("prepared oracle");
        let largest = epoch_rows
            .into_iter()
            .chain(fixed_allocations)
            .chain(prepared_allocations)
            .chain([queue_largest, replay_largest, exchange_largest])
            .max()
            .expect("largest oracle");
        CapiResources {
            active_retained,
            epoch_retained,
            prepared_protocol_retained,
            largest,
        }
    }

    fn replacement_requirement(
        row: &str,
        current: PlanResourceReport,
        prospective: PlanResourceReport,
        capi: CapiResources,
        models: CompiledModelAdmission,
    ) -> u64 {
        match row {
            "graph" => current
                .graph_session_plus_plan_bytes
                .checked_add(prospective.graph_session_plus_plan_bytes)
                .and_then(|value| value.checked_add(models.retained_bytes)),
            "source-total" => current
                .source_total_bytes
                .checked_add(prospective.source_total_bytes),
            "source-overhead" => current
                .source_overhead_bytes
                .checked_add(prospective.source_overhead_bytes),
            "effect-state" => current
                .effect_scalar_state_bytes
                .checked_add(prospective.effect_scalar_state_bytes),
            "effect-scratch" => current
                .effect_scalar_scratch_bytes
                .checked_add(prospective.effect_scalar_scratch_bytes),
            "builtin" => current
                .builtin_retained_payload_bytes
                .checked_add(prospective.builtin_retained_payload_bytes),
            "capi" => current
                .capi_retained_bytes
                .checked_add(capi.epoch_retained)
                .and_then(|value| value.checked_add(capi.prepared_protocol_retained)),
            "largest" => Some(
                current
                    .largest_named_allocation_bytes
                    .max(prospective.largest_named_allocation_bytes)
                    .max(capi.largest)
                    .max(models.largest_allocation_bytes),
            ),
            _ => unreachable!(),
        }
        .expect("bounded replacement requirement")
    }

    fn set_replacement_cap(limits: &mut CompileLimits, row: &str, value: u64) {
        match row {
            "graph" => limits.maximum_graph_session_plus_plan_bytes = value,
            "source-total" => limits.maximum_source_total_bytes = value,
            "source-overhead" => limits.maximum_source_overhead_bytes = value,
            "effect-state" => limits.maximum_effect_state_bytes = value,
            "effect-scratch" => limits.maximum_effect_scratch_bytes = value,
            "builtin" => limits.maximum_builtin_retained_bytes = value,
            "capi" => limits.maximum_capi_retained_bytes = value,
            "largest" => limits.maximum_named_allocation_bytes = value,
            _ => unreachable!(),
        }
    }

    fn generated_parity_session(track_count: usize, sample_rate_hz: u32) -> String {
        let mut model = parse_session_toml(SESSION).expect("accepted parity base");
        model.sample_rate_hz = sample_rate_hz;
        model.sources[0].sample_rate_hz = sample_rate_hz;
        model.sources[0].mapping.region.length_samples = 192;
        if track_count == 1 {
            model.tracks.truncate(1);
            model.routes.truncate(1);
        } else {
            assert_eq!(track_count, 10);
            let mut track = model.tracks[8].clone();
            track.id = miso_engine_session::StableId::parse("eq9").expect("tenth track");
            let effect = &mut track.simd1.effects[0];
            effect.id = miso_engine_session::StableId::parse("limiter").expect("limiter slot");
            effect.identity = miso_engine_session::EffectIdentity::Native {
                effect_id: miso_engine_session::StableId::parse("miso.true-peak-limiter")
                    .expect("limiter id"),
            };
            effect.params.clear();
            effect.bypass = true;
            let mut route = model.routes[8].clone();
            route.id = miso_engine_session::StableId::parse("eq9-main").expect("tenth route");
            let miso_engine_session::RouteSource::Track { track_id, .. } = &mut route.source else {
                panic!("track route")
            };
            *track_id = track.id.clone();
            model.tracks.push(track);
            model.routes.push(route);
        }
        miso_engine_session::canonical_session_toml(&model).expect("canonical parity session")
    }

    fn submit_c(
        session: *mut crate::Session,
        generation: u64,
        start_frame: u64,
        sample_rate_hz: u32,
        left: &[f32],
        right: &[f32],
        final_chunk: bool,
    ) {
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = crate::SourceChunk {
            struct_size: crate::SOURCE_CHUNK_SIZE,
            sample_rate_hz,
            generation,
            start_frame,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: left.len() as u32,
            end_of_region: u32::from(final_chunk),
            reserved0: 0,
        };
        let mut report = crate::SubmitReport {
            struct_size: crate::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        assert_eq!(left.len(), right.len());
        assert_eq!(
            crate::ffi::test_source_submit(session, b"fixture-source", &chunk, &mut report,),
            crate::RESULT_OK
        );
        assert_eq!(report.accepted_frames, left.len() as u64);
    }

    fn boxed_c_children(session: &str) -> (*mut crate::Session, *mut crate::Plan) {
        boxed_c_children_with_limits(session, limits())
    }

    fn boxed_c_children_with_limits(
        session: &str,
        limits: CompileLimits,
    ) -> (*mut crate::Session, *mut crate::Plan) {
        let children = compile_children(session, limits).expect("C children");
        (
            Box::into_raw(Box::new(crate::Session::new(
                children.session,
                children.session_error,
            ))),
            Box::into_raw(Box::new(crate::Plan::new(
                children.plan,
                children.plan_error,
            ))),
        )
    }

    fn command_c(session: *mut crate::Session, request: &[u8]) -> (u32, Vec<u8>) {
        let (result, _, storage) = command_c_capacity(session, request, 4_096);
        (result, storage)
    }

    fn command_c_capacity(
        session: *mut crate::Session,
        request: &[u8],
        capacity: usize,
    ) -> (u32, u64, Vec<u8>) {
        let mut storage = vec![0xa5_u8; capacity];
        let mut output = crate::BytesOut {
            struct_size: crate::BYTES_OUT_SIZE,
            reserved0: 0,
            data: if storage.is_empty() {
                core::ptr::null_mut()
            } else {
                storage.as_mut_ptr()
            },
            capacity_bytes: storage.len() as u64,
            required_bytes: u64::MAX,
        };
        let result = crate::ffi::test_submit_command(session, request, &mut output);
        if result == crate::RESULT_OK && output.required_bytes <= storage.len() as u64 {
            storage.truncate(output.required_bytes as usize);
        }
        (result, output.required_bytes, storage)
    }

    fn event_c(session: *mut crate::Session, lane: u32) -> (u32, Vec<u8>) {
        let (result, _, storage) = event_c_capacity(session, lane, 4_096);
        (result, storage)
    }

    fn event_c_capacity(
        session: *mut crate::Session,
        lane: u32,
        capacity: usize,
    ) -> (u32, u64, Vec<u8>) {
        let mut storage = vec![0xa5_u8; capacity];
        let mut output = crate::BytesOut {
            struct_size: crate::BYTES_OUT_SIZE,
            reserved0: 0,
            data: if storage.is_empty() {
                core::ptr::null_mut()
            } else {
                storage.as_mut_ptr()
            },
            capacity_bytes: storage.len() as u64,
            required_bytes: u64::MAX,
        };
        let result = crate::ffi::test_dequeue_event(session, lane, &mut output);
        if result == crate::RESULT_OK && output.required_bytes <= storage.len() as u64 {
            storage.truncate(output.required_bytes as usize);
        }
        (result, output.required_bytes, storage)
    }

    fn event_c_exact_retry(session: *mut crate::Session, lane: u32, oracle: &[u8]) {
        let (query_result, required, query) = event_c_capacity(session, lane, 0);
        assert_eq!(query_result, crate::RESULT_BUFFER_TOO_SMALL);
        assert_eq!(required, oracle.len() as u64);
        assert!(query.is_empty());
        let (short_result, short_required, short) =
            event_c_capacity(session, lane, oracle.len() - 1);
        assert_eq!(short_result, crate::RESULT_BUFFER_TOO_SMALL);
        assert_eq!(short_required, oracle.len() as u64);
        assert!(short.iter().all(|byte| *byte == 0xa5));
        let (exact_result, exact_required, exact) = event_c_capacity(session, lane, oracle.len());
        assert_eq!(exact_result, crate::RESULT_OK);
        assert_eq!(exact_required, oracle.len() as u64);
        assert_eq!(exact, oracle);
    }

    fn render_parity_shape(track_count: usize, sample_rate_hz: u32) {
        let session = generated_parity_session(track_count, sample_rate_hz);
        let mut direct = compile_children(&session, limits()).expect("direct children");
        let wrapped = compile_children(&session, limits()).expect("C children");
        let c_session = Box::into_raw(Box::new(crate::Session::new(
            wrapped.session,
            wrapped.session_error,
        )));
        let c_plan = Box::into_raw(Box::new(crate::Plan::new(wrapped.plan, wrapped.plan_error)));
        let quantum = 128_usize;
        let mut first_left = vec![0.0_f32; quantum];
        let mut first_right = vec![0.0_f32; quantum];
        first_left[0] = -0.0;
        first_right[0] = 0.0;
        first_left[1] = 0.25;
        first_right[1] = -0.5;
        let final_left = vec![0.125_f32; 64];
        let final_right = vec![-0.25_f32; 64];

        for block in 0..8_u64 {
            match block {
                0 | 3 => {
                    let generation = if block == 0 { 1 } else { 2 };
                    if block == 3 {
                        direct
                            .session
                            .seek(b"fixture-source", generation, 0)
                            .expect("direct seek");
                        assert_eq!(
                            crate::ffi::test_source_seek(
                                c_session,
                                b"fixture-source",
                                generation,
                                0,
                            ),
                            crate::RESULT_OK
                        );
                    }
                    direct
                        .session
                        .submit(
                            b"fixture-source",
                            SourceSubmission {
                                generation,
                                start_frame: 0,
                                sample_rate_hz,
                                planes: &[&first_left, &first_right],
                                frames: quantum as u32,
                                end_of_region: false,
                            },
                        )
                        .expect("direct full chunk");
                    submit_c(
                        c_session,
                        generation,
                        0,
                        sample_rate_hz,
                        &first_left,
                        &first_right,
                        false,
                    );
                }
                1 | 4 => {
                    let generation = if block == 1 { 1 } else { 2 };
                    direct
                        .session
                        .submit(
                            b"fixture-source",
                            SourceSubmission {
                                generation,
                                start_frame: 128,
                                sample_rate_hz,
                                planes: &[&final_left, &final_right],
                                frames: 64,
                                end_of_region: true,
                            },
                        )
                        .expect("direct partial final");
                    submit_c(
                        c_session,
                        generation,
                        128,
                        sample_rate_hz,
                        &final_left,
                        &final_right,
                        true,
                    );
                }
                _ => {}
            }

            let mut direct_pcm = vec![f32::NAN; quantum * 2];
            direct
                .plan
                .render(
                    block * quantum as u64,
                    PlanarBufferMut::try_new(&mut direct_pcm, 2, quantum, quantum)
                        .expect("direct output"),
                )
                .expect("direct render");
            let mut c_pcm = vec![f32::NAN; quantum * 2];
            let output = crate::PlanarOutput {
                struct_size: crate::PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: c_pcm.as_mut_ptr(),
                sample_capacity: c_pcm.len() as u64,
                frames: quantum as u32,
                plane_stride_samples: quantum as u32,
                reserved: [0; 2],
            };
            assert_eq!(
                crate::ffi::test_render(c_plan, block * quantum as u64, &output),
                crate::RESULT_OK
            );
            assert_eq!(
                c_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                direct_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "direct/C parity for {track_count} tracks at {sample_rate_hz} Hz block {block}"
            );
        }
        if track_count == 1 {
            crate::ffi::test_session_destroy(c_session);
            crate::ffi::test_plan_destroy(c_plan);
        } else {
            crate::ffi::test_plan_destroy(c_plan);
            crate::ffi::test_session_destroy(c_session);
        }
    }

    #[test]
    fn generated_session_prepares_independent_source_and_plan_ownership() {
        let mut children = compile_children(SESSION, limits()).unwrap_or_else(|failure| {
            panic!("compile: {}", String::from_utf8_lossy(&failure.diagnostics))
        });
        assert_eq!(children.plan.resources().sample_rate_hz, 48_000);
        assert_eq!(children.plan.resources().quantum_frames, 128);
        assert_eq!(children.plan.resources().source_count, 1);
        assert_eq!(children.plan.resources().track_count, 9);
        assert!(children.plan.resources().graph_session_plus_plan_bytes > 0);
        assert!(children.plan.resources().source_total_bytes > 0);
        assert!(children.plan.resources().effect_scalar_state_bytes > 0);
        assert!(children.plan.resources().builtin_retained_payload_bytes > 0);
        assert!(children.plan.resources().capi_retained_bytes > 0);
        assert!(children.plan.resources().largest_named_allocation_bytes > 0);

        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        let submitted = children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: 0,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .expect("first source block");
        assert_eq!(submitted.accepted_frames, 128);
        children
            .session
            .seek(b"fixture-source", 2, 48_000)
            .expect("inclusive end seek");
        children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 2,
                    start_frame: 48_000,
                    sample_rate_hz: 48_000,
                    planes: &[&[], &[]],
                    frames: 0,
                    end_of_region: true,
                },
            )
            .expect("zero-frame final marker");
    }

    #[test]
    fn exact_report_caps_accept_equal_and_reject_one_below() {
        let baseline = compile_children(SESSION, limits()).expect("baseline");
        let report = baseline.plan.resources();
        let model = baseline
            .session
            .controller
            .session()
            .compiled()
            .resource_estimate();

        for below in [
            (
                "graph",
                report
                    .graph_session_plus_plan_bytes
                    .checked_add(model.compiled_model_bytes)
                    .expect("active graph/model aggregate"),
            ),
            ("source-total", report.source_total_bytes),
            ("source-overhead", report.source_overhead_bytes),
            ("effect-state", report.effect_scalar_state_bytes),
            ("builtin", report.builtin_retained_payload_bytes),
            ("capi", report.capi_retained_bytes),
            (
                "largest",
                report
                    .largest_named_allocation_bytes
                    .max(model.single_allocation_bytes),
            ),
        ] {
            assert!(below.1 > 0, "{} row must be nonzero", below.0);
            let mut equal = limits();
            match below.0 {
                "graph" => equal.maximum_graph_session_plus_plan_bytes = below.1,
                "source-total" => equal.maximum_source_total_bytes = below.1,
                "source-overhead" => equal.maximum_source_overhead_bytes = below.1,
                "effect-state" => equal.maximum_effect_state_bytes = below.1,
                "builtin" => equal.maximum_builtin_retained_bytes = below.1,
                "capi" => equal.maximum_capi_retained_bytes = below.1,
                "largest" => equal.maximum_named_allocation_bytes = below.1,
                _ => unreachable!(),
            }
            compile_children(SESSION, equal).unwrap_or_else(|failure| {
                panic!(
                    "{} equal cap: {}",
                    below.0,
                    String::from_utf8_lossy(&failure.diagnostics)
                )
            });
            let mut constrained = limits();
            match below.0 {
                "graph" => constrained.maximum_graph_session_plus_plan_bytes = below.1 - 1,
                "source-total" => constrained.maximum_source_total_bytes = below.1 - 1,
                "source-overhead" => constrained.maximum_source_overhead_bytes = below.1 - 1,
                "effect-state" => constrained.maximum_effect_state_bytes = below.1 - 1,
                "builtin" => constrained.maximum_builtin_retained_bytes = below.1 - 1,
                "capi" => constrained.maximum_capi_retained_bytes = below.1 - 1,
                "largest" => constrained.maximum_named_allocation_bytes = below.1 - 1,
                _ => unreachable!(),
            }
            assert!(
                compile_children(SESSION, constrained).is_err(),
                "{} one-below cap must reject",
                below.0
            );
        }

        let scratch_session = generated_scratch_session();
        let scratch_report = compile_children(&scratch_session, limits())
            .expect("scratch baseline")
            .plan
            .resources();
        assert!(scratch_report.effect_scalar_scratch_bytes > 0);
        let mut equal = limits();
        equal.maximum_effect_scratch_bytes = scratch_report.effect_scalar_scratch_bytes;
        compile_children(&scratch_session, equal).expect("effect scratch equal cap");
        let mut below = limits();
        below.maximum_effect_scratch_bytes = scratch_report.effect_scalar_scratch_bytes - 1;
        assert!(
            compile_children(&scratch_session, below).is_err(),
            "effect scratch one-below cap must reject"
        );
    }

    #[test]
    fn replacement_peak_caps_accept_equal_and_reject_one_below_atomically() {
        let session = generated_scratch_session();
        let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("double-live-cap").expect("stable ID"),
        };
        let request = command_bytes_at_revision(
            1,
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        let mut projection = compile_children(&session, limits()).expect("projection children");
        let (current, prospective, capi, models) =
            replacement_projection(&mut projection, &request);
        assert_eq!(
            projection.session.controller.session().revision(),
            SessionRevision(42),
            "dropping the projection token must cancel it"
        );
        assert_eq!(projection.session.controller.replay().len(), 0);

        for row in [
            "graph",
            "source-total",
            "source-overhead",
            "effect-state",
            "effect-scratch",
            "builtin",
            "capi",
            "largest",
        ] {
            let required = replacement_requirement(row, current, prospective, capi, models);
            assert!(required > 0, "{row} replacement requirement");

            let mut exact = compile_children(&session, limits()).expect("exact children");
            set_replacement_cap(&mut exact.session.limits, row, required);
            let response_len = exact
                .session
                .command(&request, 4_096)
                .unwrap_or_else(|error| panic!("{row} exact replacement cap: {error:?}"));
            assert!(response_len > 0, "{row} exact response");
            assert_eq!(
                exact.session.controller.session().revision(),
                SessionRevision(43),
                "{row} exact cap commits"
            );
            assert_eq!(exact.session.pending_providers.len(), 1);

            let mut below = compile_children(&session, limits()).expect("one-below children");
            set_replacement_cap(&mut below.session.limits, row, required - 1);
            let canonical = below
                .session
                .controller
                .session()
                .canonical_snapshot()
                .as_bytes()
                .to_vec();
            let error = below
                .session
                .command(&request, 4_096)
                .expect_err("one-below replacement must reject");
            let CommandError::CompileRejected(failure) = error else {
                panic!("{row} one-below failure: {error:?}")
            };
            let expected_diagnostic = match row {
                "graph" => b"graph.resource.limit\t$\n".as_slice(),
                "source-total" | "source-overhead" => b"source.resource.limit\t$\n".as_slice(),
                "effect-state" | "effect-scratch" => b"effect.resource.limit\t$\n".as_slice(),
                "builtin" | "capi" => b"capi.resource.limit\t$\n".as_slice(),
                "largest" => b"capi.resource.limit\t$\n".as_slice(),
                _ => unreachable!(),
            };
            assert_eq!(failure.diagnostics, expected_diagnostic, "{row} diagnostic");
            assert_eq!(
                below
                    .session
                    .controller
                    .session()
                    .canonical_snapshot()
                    .as_bytes(),
                canonical,
                "{row} canonical rollback"
            );
            assert_eq!(
                below.session.controller.session().revision(),
                SessionRevision(42),
                "{row} revision rollback"
            );
            assert_eq!(below.session.controller.replay().len(), 0, "{row} replay");
            assert_eq!(below.session.providers.epoch, 0, "{row} provider epoch");
            assert!(
                below.session.pending_providers.is_empty(),
                "{row} providers"
            );
            assert_eq!(below.plan.owner.active_epoch().0, 0, "{row} plan epoch");
            assert_eq!(
                below
                    .session
                    .dequeue_event(EventLane::Reliable, 4_096)
                    .expect("reliable lane"),
                None,
                "{row} reliable events"
            );
        }
    }

    #[test]
    fn controller_command_is_canonical_replayed_and_supports_snapshot() {
        let mut children = compile_children(SESSION, limits()).expect("children");
        let capability = command_bytes(1, miso_engine_protocol::CommandPayload::CapabilitiesGet);
        assert!(matches!(
            children.session.command(&capability, 0),
            Err(CommandError::BufferTooSmall { required: 4_096 })
        ));
        assert!(children.session.controller.replay().is_empty());

        let first_len = children
            .session
            .command(&capability, 4_096)
            .expect("capability response");
        let first = children.session.command_response(first_len).to_vec();
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(&first, &mut DecodeScratch::new(&mut fields))
            .expect("typed capability response")
        {
            miso_engine_protocol::DecodedTypedResponseFrame::Success {
                header, payload, ..
            } => {
                assert_eq!(header.request_id.get(), 1);
                assert_eq!(header.revision, SessionRevision(42));
                let miso_engine_protocol::DecodedSuccessResponsePayload::Capabilities(value) =
                    payload
                else {
                    panic!("capability payload")
                };
                assert_eq!(value.supported_commands.len(), 22);
                assert_eq!(value.supported_events.len(), 12);
                assert_eq!(value.flags, CapabilityFlags((1 << 14) - 1));
                assert_eq!(value.replay_entries, 16);
                assert_eq!(value.replay_bytes, 8_192);
                assert_eq!(value.maximum_cached_response_bytes, 4_096);
            }
            _ => panic!("success response"),
        }
        let replay_len = children
            .session
            .command(&capability, first_len as u64)
            .expect("exact replay");
        assert_eq!(children.session.command_response(replay_len), first);
        assert_eq!(children.session.controller.replay().len(), 1);

        let snapshot = command_bytes(
            2,
            miso_engine_protocol::CommandPayload::SessionSnapshotGet(
                miso_engine_protocol::SessionSnapshotRequest {
                    offset: 0,
                    maximum_bytes: 1,
                },
            ),
        );
        let snapshot_len = children
            .session
            .command(&snapshot, 4_096)
            .expect("typed snapshot response");
        let snapshot = children.session.command_response(snapshot_len);
        let mut fields = [0_u16; 8];
        assert!(matches!(
            ProtocolCodec::default()
                .decode_typed_response(snapshot, &mut DecodeScratch::new(&mut fields))
                .expect("typed snapshot"),
            miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                if header.status == StatusCode::Ok && header.request_id.get() == 2
        ));
    }

    #[test]
    fn structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic() {
        let mut children = compile_children(SESSION, limits()).expect("children");
        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: 0,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .expect("old provider source block");
        let mut pcm = [0.0_f32; 256];
        children
            .plan
            .render(
                0,
                PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("old output"),
            )
            .expect("old plan block");
        assert!(pcm.iter().any(|sample| *sample != 0.0), "old provider PCM");
        let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("capi-replaced").expect("stable ID"),
        };
        let first_request = command_bytes_at_revision(
            1,
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        assert!(matches!(
            children.session.command(&first_request, 0),
            Err(CommandError::BufferTooSmall { required: 4_096 })
        ));
        assert_eq!(
            children.session.controller.session().revision(),
            SessionRevision(42)
        );
        assert_eq!(children.session.providers.epoch, 0);
        assert_eq!(children.plan.owner.active_epoch().0, 0);

        let first_len = children
            .session
            .command(&first_request, 4_096)
            .expect("first structural command");
        let first_response = children.session.command_response(first_len).to_vec();
        assert_eq!(
            children.session.controller.session().revision(),
            SessionRevision(43)
        );
        assert_eq!(children.session.providers.epoch, 0);
        assert_eq!(children.session.pending_providers[0].epoch, 1);
        assert_eq!(children.plan.owner.active_epoch().0, 0);
        assert_eq!(children.session.controller.replay().len(), 1);
        children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: 128,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .expect("submission remains routed to old committed provider before boundary");

        let required = match children.session.dequeue_event(EventLane::Reliable, 0) {
            Err(EventError::BufferTooSmall { required }) => required,
            other => panic!("expected reliable query length, got {other:?}"),
        };
        let event_len = children
            .session
            .dequeue_event(EventLane::Reliable, required)
            .expect("reliable retry")
            .expect("session event");
        let event = children.session.event_response(event_len).to_vec();
        let mut fields = [0_u16; 64];
        assert!(matches!(
            ProtocolCodec::default()
                .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
                .expect("session event"),
            miso_engine_protocol::DecodedTypedEventFrame {
                header,
                payload: miso_engine_protocol::DecodedEventPayload::SessionCommitted(_),
            } if header.revision == SessionRevision(43)
        ));
        assert_eq!(
            children
                .session
                .dequeue_event(EventLane::Reliable, 0)
                .expect("empty reliable lane"),
            None
        );

        let model = parse_session_toml(SESSION).expect("source-changing model");
        let mut mapping = model.sources[0].mapping.clone();
        mapping.region.length_samples = 512;
        let second_edit = miso_engine_protocol::SessionEditV1::SetSourceMapping {
            source_id: model.sources[0].id.clone(),
            mapping,
        };
        let second_request = command_bytes_at_revision(
            2,
            ExpectedRevision::Exact(SessionRevision(43)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &second_edit,
            )),
        );
        assert!(matches!(
            children.session.command(&second_request, 4_096),
            Err(CommandError::Backpressure)
        ));
        assert_eq!(
            children.session.controller.session().revision(),
            SessionRevision(43)
        );
        assert_eq!(children.session.controller.replay().len(), 1);

        pcm.fill(f32::NAN);
        assert_eq!(
            STRUCTURAL_SOURCE_STATE_POLICY,
            StructuralSourceStatePolicy::ResetAtReplacementBoundary
        );
        children
            .plan
            .render(
                128,
                PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
            )
            .expect("replacement boundary");
        assert!(
            pcm.iter().all(|sample| *sample == 0.0),
            "new provider follows the frozen structural source-state policy"
        );
        assert_eq!(children.plan.owner.active_epoch().0, 1);
        assert_eq!(children.session.providers.epoch, 0);
        children
            .session
            .synchronize_plan_epochs()
            .expect("control promotion and retirement");
        assert_eq!(children.session.providers.epoch, 1);
        assert!(children.session.pending_providers.is_empty());

        let second_len = children
            .session
            .command(&second_request, 4_096)
            .expect("source-changing replacement after reclaim");
        assert!(second_len > 0);
        assert_eq!(
            children.session.controller.session().revision(),
            SessionRevision(44)
        );
        assert_eq!(children.session.providers.sources[0].region_end, 48_000);
        assert_eq!(children.session.pending_providers[0].epoch, 2);
        assert_eq!(
            children.session.pending_providers[0].sources[0].region_end,
            512
        );
        children
            .plan
            .render(
                256,
                PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
            )
            .expect("second replacement boundary");
        children
            .session
            .synchronize_plan_epochs()
            .expect("second provider promotion and retirement");
        assert_eq!(children.session.providers.epoch, 2);
        assert_eq!(children.session.providers.sources[0].region_end, 512);
        assert!(children.session.pending_providers.is_empty());
        assert!(children.session.retired_providers.is_empty());
        children
            .session
            .seek(b"fixture-source", 2, 384)
            .expect("seek new source-changing provider");
        children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 2,
                    start_frame: 384,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: true,
                },
            )
            .expect("new source-changing provider PCM");
        pcm.fill(f32::NAN);
        children
            .plan
            .render(
                384,
                PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("new provider output"),
            )
            .expect("new provider render");
        assert!(
            pcm.iter().any(|sample| *sample != 0.0),
            "source-changing provider produces submitted PCM"
        );

        let replay_len = children
            .session
            .command(&first_request, first_len as u64)
            .expect("exact structural replay");
        assert_eq!(
            children.session.command_response(replay_len),
            first_response
        );
        assert_eq!(
            children.session.controller.session().revision(),
            SessionRevision(44)
        );
        assert!(children.session.pending_providers.is_empty());
    }

    #[test]
    fn capi_event_selector_encodes_all_six_protocol_event_families() {
        let mut children = compile_children(SESSION, limits()).expect("children");
        let revision = SessionRevision(42);
        let configuration = miso_engine_protocol::TelemetryConfiguration {
            meter_handles: vec![1, 2],
            meter_period_blocks: 1,
            counter_ids: vec![miso_engine_protocol::CounterId::ControlCommandBackpressure],
            counter_period_blocks: 1,
            diagnostics_enabled: true,
            minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        };
        let configure = command_bytes_at_revision(
            1,
            ExpectedRevision::Exact(revision),
            miso_engine_protocol::CommandPayload::TelemetryConfigure(&configuration),
        );
        children
            .session
            .command(&configure, 4_096)
            .expect("telemetry configuration");

        let request_id = RequestId::new(9).expect("request ID");
        let reliable = [
            miso_engine_protocol::ReliableSlot::session_committed(
                revision,
                1,
                request_id,
                SessionRevision(41),
                1,
            ),
            miso_engine_protocol::ReliableSlot::automation_canceled(
                revision,
                2,
                request_id,
                3,
                miso_engine_protocol::AutomationCancellationReason::ExplicitReconfiguration,
                1,
                Some(miso_engine_protocol::SampleTime(0)),
            ),
            miso_engine_protocol::ReliableSlot::transport_state(
                revision,
                3,
                miso_engine_protocol::TransportState::Playing,
                miso_engine_protocol::SampleTime(0),
                miso_engine_protocol::SampleTime(0),
                Some(request_id),
            ),
        ];
        let mut observed = Vec::new();
        for event in reliable {
            children
                .session
                .controller
                .queues_mut()
                .try_enqueue_event(event)
                .expect("reliable fixture event");
            let len = children
                .session
                .dequeue_event(EventLane::Reliable, 4_096)
                .expect("reliable egress")
                .expect("reliable event");
            let mut fields = [0_u16; 64];
            observed.push(
                ProtocolCodec::default()
                    .decode_typed_event(
                        children.session.event_response(len),
                        &mut DecodeScratch::new(&mut fields),
                    )
                    .expect("typed reliable event")
                    .header
                    .message_id,
            );
        }

        children
            .session
            .controller
            .enqueue_diagnostic_event(
                revision,
                miso_engine_protocol::DiagnosticEvent {
                    diagnostic: miso_engine_protocol::Diagnostic {
                        code: "capi.event".to_owned(),
                        severity: miso_engine_protocol::DiagnosticSeverity::Warning,
                        path: Vec::new(),
                        detail: None,
                        operation_index: None,
                        sample_time: Some(0),
                        provider_sequence: Some(1),
                    },
                },
            )
            .expect("diagnostic fixture");
        let diagnostic_len = children
            .session
            .dequeue_event(EventLane::Reliable, 4_096)
            .expect("diagnostic egress")
            .expect("diagnostic event");
        let mut fields = [0_u16; 64];
        observed.push(
            ProtocolCodec::default()
                .decode_typed_event(
                    children.session.event_response(diagnostic_len),
                    &mut DecodeScratch::new(&mut fields),
                )
                .expect("typed diagnostic")
                .header
                .message_id,
        );

        children
            .session
            .controller
            .stage_meter_batch_event(
                revision,
                miso_engine_protocol::SampleTime(4),
                &[miso_engine_protocol::MeterRecord {
                    handle: 1,
                    component: miso_engine_protocol::MeterComponent::Left,
                    flags: 1,
                    value: 0.5,
                }],
            )
            .expect("meter fixture");
        let meter_len = children
            .session
            .dequeue_event(EventLane::Lossy, 4_096)
            .expect("meter egress")
            .expect("meter event");
        let mut fields = [0_u16; 64];
        observed.push(
            ProtocolCodec::default()
                .decode_typed_event(
                    children.session.event_response(meter_len),
                    &mut DecodeScratch::new(&mut fields),
                )
                .expect("typed meter")
                .header
                .message_id,
        );

        children
            .session
            .controller
            .stage_counter_snapshot_event(
                revision,
                &miso_engine_protocol::CounterSnapshot {
                    observed_sample: miso_engine_protocol::SampleTime(4),
                    values: vec![miso_engine_protocol::CounterValue {
                        id: miso_engine_protocol::CounterId::ControlCommandBackpressure,
                        value: 7,
                    }],
                },
            )
            .expect("counter fixture");
        let counter_len = children
            .session
            .dequeue_event(EventLane::Lossy, 4_096)
            .expect("counter egress")
            .expect("counter event");
        let mut fields = [0_u16; 64];
        observed.push(
            ProtocolCodec::default()
                .decode_typed_event(
                    children.session.event_response(counter_len),
                    &mut DecodeScratch::new(&mut fields),
                )
                .expect("typed counter")
                .header
                .message_id,
        );

        assert_eq!(
            observed,
            [
                miso_engine_protocol::MessageId::SessionCommitted,
                miso_engine_protocol::MessageId::AutomationCanceled,
                miso_engine_protocol::MessageId::TransportState,
                miso_engine_protocol::MessageId::Diagnostic,
                miso_engine_protocol::MessageId::MeterBatch,
                miso_engine_protocol::MessageId::CounterSnapshot,
            ]
        );
    }

    #[test]
    fn all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes() {
        let (c_session, c_plan) = boxed_c_children(SESSION);
        let eager_capacities = crate::ffi::test_retained_capacities(c_session);
        let revision = SessionRevision(42);
        crate::ffi::test_install_parameter(
            c_session,
            miso_engine_protocol::ParameterDescriptor {
                handle: 1,
                track_id: "fixture".to_owned(),
                rack: miso_engine_protocol::ParameterRack::Dynamic,
                effect_id: "fixture".to_owned(),
                parameter_id: 1,
                channel: miso_engine_protocol::ParameterChannel::Left,
                value_kind: miso_engine_protocol::ParameterValueKind::F32,
                unit: miso_engine_protocol::ParameterUnit::Linear,
                domain: miso_engine_protocol::ParameterDomain::Continuous,
                minimum: Some(-1.0),
                maximum: Some(1.0),
                default: 0.0,
                mapping: miso_engine_protocol::ParameterMapping::Linear,
                automation_rate: miso_engine_protocol::ParameterAutomationRate::Sample,
                smoothing_samples: 0,
                flags: 3,
                display_name: None,
                display_unit: None,
                enum_choices: Vec::new(),
            },
            miso_engine_protocol::ParameterStateRecord {
                handle: 1,
                flags: 0,
                value: 0.0,
            },
        );
        let configuration = miso_engine_protocol::TelemetryConfiguration {
            meter_handles: vec![1],
            meter_period_blocks: 1,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: true,
            minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        };
        let configure = command_bytes_at_revision(
            1,
            ExpectedRevision::Exact(revision),
            miso_engine_protocol::CommandPayload::TelemetryConfigure(&configuration),
        );
        let (c_result, c_response) = command_c(c_session, &configure);
        assert_eq!(c_result, crate::RESULT_OK);
        assert!(!c_response.is_empty());

        let transport = command_bytes_at_revision(
            2,
            ExpectedRevision::Exact(revision),
            miso_engine_protocol::CommandPayload::TransportSet(
                miso_engine_protocol::TransportSetRequest {
                    state: miso_engine_protocol::TransportState::Playing,
                    position: Some(miso_engine_protocol::SampleTime(0)),
                },
            ),
        );
        assert_eq!(command_c(c_session, &transport).0, crate::RESULT_OK);
        let oracle = typed_event_bytes(&miso_engine_protocol::TypedEventFrame {
            revision,
            payload: miso_engine_protocol::EventPayload::TransportState(
                miso_engine_protocol::TransportStateEvent {
                    event_sequence: 1,
                    state: miso_engine_protocol::TransportState::Playing,
                    position: miso_engine_protocol::SampleTime(0),
                    effective_sample: miso_engine_protocol::SampleTime(0),
                    origin_request_id: Some(RequestId::new(2).expect("request")),
                },
            ),
        });
        event_c_exact_retry(c_session, crate::EVENT_LANE_RELIABLE, &oracle);

        let record = miso_engine_protocol::AutomationRecord {
            kind: miso_engine_protocol::AutomationKind::Point,
            handle: miso_engine_protocol::ParameterHandle(1),
            start: miso_engine_protocol::SampleTime(1),
            end: miso_engine_protocol::SampleTime(1),
            start_value: 0.0,
            end_value: 0.0,
        };
        let automation = command_bytes_at_revision(
            3,
            ExpectedRevision::Exact(revision),
            miso_engine_protocol::CommandPayload::AutomationEnqueue(
                miso_engine_protocol::AutomationEnqueue {
                    records: core::slice::from_ref(&record),
                },
            ),
        );
        let (automation_result, automation_response) = command_c(c_session, &automation);
        assert_eq!(automation_result, crate::RESULT_OK);
        let mut automation_fields = [0_u16; 64];
        let automation_decoded = ProtocolCodec::default()
            .decode_typed_response(
                &automation_response,
                &mut DecodeScratch::new(&mut automation_fields),
            )
            .expect("accepted automation response");
        assert!(matches!(
            automation_decoded,
            miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                if header.status == StatusCode::Ok
        ));
        let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("event-origin").expect("stable ID"),
        };
        let structural = command_bytes_at_revision(
            4,
            ExpectedRevision::Exact(revision),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        assert_eq!(command_c(c_session, &structural).0, crate::RESULT_OK);
        let expected_events = [
            miso_engine_protocol::TypedEventFrame {
                revision: SessionRevision(43),
                payload: miso_engine_protocol::EventPayload::SessionCommitted(
                    miso_engine_protocol::SessionCommitted {
                        event_sequence: 2,
                        origin_request_id: RequestId::new(4).expect("request"),
                        previous_revision: revision,
                        applied_operations: 1,
                    },
                ),
            },
            miso_engine_protocol::TypedEventFrame {
                revision: SessionRevision(43),
                payload: miso_engine_protocol::EventPayload::AutomationCanceled(
                    miso_engine_protocol::AutomationCanceled {
                        event_sequence: 3,
                        origin_request_id: RequestId::new(3).expect("request"),
                        canceled_records: 1,
                        reason: miso_engine_protocol::AutomationCancellationReason::RevisionChanged,
                        queue_generation: 2,
                        effective_sample: Some(miso_engine_protocol::SampleTime(0)),
                    },
                ),
            },
        ];
        for expected in &expected_events {
            let oracle = typed_event_bytes(expected);
            event_c_exact_retry(c_session, crate::EVENT_LANE_RELIABLE, &oracle);
        }

        let mut pcm = [f32::NAN; 256];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(c_plan, 0, &output),
            crate::RESULT_OK
        );
        let diagnostic = miso_engine_protocol::Diagnostic {
            code: "capi.render.activity".to_owned(),
            severity: miso_engine_protocol::DiagnosticSeverity::Info,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: Some(128),
            provider_sequence: Some(1),
        };
        let expected_diagnostic = typed_event_bytes(&miso_engine_protocol::TypedEventFrame {
            revision: SessionRevision(43),
            payload: miso_engine_protocol::EventPayload::Diagnostic(&diagnostic),
        });
        event_c_exact_retry(c_session, crate::EVENT_LANE_RELIABLE, &expected_diagnostic);

        let quiet_meter_configuration = miso_engine_protocol::TelemetryConfiguration {
            meter_handles: vec![1],
            meter_period_blocks: 1,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        };
        let disable_diagnostics = command_bytes_at_revision(
            5,
            ExpectedRevision::Exact(SessionRevision(43)),
            miso_engine_protocol::CommandPayload::TelemetryConfigure(&quiet_meter_configuration),
        );
        assert_eq!(
            command_c(c_session, &disable_diagnostics).0,
            crate::RESULT_OK
        );
        assert_eq!(
            crate::ffi::test_render(c_plan, 128, &output),
            crate::RESULT_OK
        );
        let expanded_meter_configuration = miso_engine_protocol::TelemetryConfiguration {
            meter_handles: vec![1, 2],
            meter_period_blocks: 1,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        };
        let expand_meters = command_bytes_at_revision(
            6,
            ExpectedRevision::Exact(SessionRevision(43)),
            miso_engine_protocol::CommandPayload::TelemetryConfigure(&expanded_meter_configuration),
        );
        assert_eq!(command_c(c_session, &expand_meters).0, crate::RESULT_OK);
        assert_eq!(
            crate::ffi::test_render(c_plan, 256, &output),
            crate::RESULT_OK
        );
        let collect_third_render =
            command_bytes(7, miso_engine_protocol::CommandPayload::CapabilitiesGet);
        assert_eq!(
            command_c(c_session, &collect_third_render).0,
            crate::RESULT_OK
        );
        assert_eq!(
            crate::ffi::test_telemetry_counters(c_session),
            miso_engine_protocol::TelemetryCounters {
                telemetry_coalesced: 1,
                telemetry_dropped: 1,
            }
        );
        for (sample, value) in [(128, 1.0), (384, 3.0)] {
            let record = miso_engine_protocol::MeterRecord {
                handle: 1,
                component: miso_engine_protocol::MeterComponent::Left,
                flags: 1,
                value,
            };
            let oracle = typed_event_bytes(&miso_engine_protocol::TypedEventFrame {
                revision: SessionRevision(43),
                payload: miso_engine_protocol::EventPayload::MeterBatch(
                    miso_engine_protocol::MeterBatch {
                        observed_sample: miso_engine_protocol::SampleTime(sample),
                        records: core::slice::from_ref(&record),
                    },
                ),
            });
            event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &oracle);
        }

        let counter_configuration = miso_engine_protocol::TelemetryConfiguration {
            meter_handles: Vec::new(),
            meter_period_blocks: 0,
            counter_ids: vec![miso_engine_protocol::CounterId::ControlCommandBackpressure],
            counter_period_blocks: 1,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        };
        let configure_counters = command_bytes_at_revision(
            8,
            ExpectedRevision::Exact(SessionRevision(43)),
            miso_engine_protocol::CommandPayload::TelemetryConfigure(&counter_configuration),
        );
        assert_eq!(
            command_c(c_session, &configure_counters).0,
            crate::RESULT_OK
        );
        assert_eq!(
            crate::ffi::test_render(c_plan, 384, &output),
            crate::RESULT_OK
        );
        let counters = miso_engine_protocol::CounterSnapshot {
            observed_sample: miso_engine_protocol::SampleTime(512),
            values: vec![miso_engine_protocol::CounterValue {
                id: miso_engine_protocol::CounterId::ControlCommandBackpressure,
                value: 4,
            }],
        };
        let oracle = typed_event_bytes(&miso_engine_protocol::TypedEventFrame {
            revision: SessionRevision(43),
            payload: miso_engine_protocol::EventPayload::CounterSnapshot(
                miso_engine_protocol::CounterSnapshotRef {
                    observed_sample: counters.observed_sample,
                    values: &counters.values,
                },
            ),
        });
        event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &oracle);

        assert_eq!(
            event_c(c_session, crate::EVENT_LANE_RELIABLE),
            (crate::RESULT_OK, Vec::new())
        );
        assert_eq!(
            event_c(c_session, crate::EVENT_LANE_LOSSY),
            (crate::RESULT_OK, Vec::new())
        );
        assert_eq!(
            crate::ffi::test_retained_capacities(c_session),
            eager_capacities
        );
        crate::ffi::test_session_destroy(c_session);
        crate::ffi::test_plan_destroy(c_plan);
    }

    #[test]
    fn plan_first_destroy_guards_structural_publication_without_visible_mutation() {
        let (c_session, c_plan) = boxed_c_children(SESSION);
        crate::ffi::test_plan_destroy(c_plan);
        let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("destroyed-plan").expect("stable ID"),
        };
        let request = command_bytes_at_revision(
            1,
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        let before = crate::ffi::test_session_state_summary(c_session);
        let (short_result, required, short_canary) = command_c_capacity(c_session, &request, 0);
        assert_eq!(short_result, crate::RESULT_BUFFER_TOO_SMALL);
        assert_eq!(required, 4_096);
        assert!(short_canary.is_empty());
        assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
        let (result, canary) = command_c(c_session, &request);
        assert_eq!(result, crate::RESULT_BACKPRESSURE);
        assert_eq!(canary, vec![0xa5; 4_096]);
        assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
        assert_eq!(
            event_c(c_session, crate::EVENT_LANE_RELIABLE),
            (crate::RESULT_OK, Vec::new())
        );
        crate::ffi::test_session_destroy(c_session);
    }

    #[test]
    fn every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits() {
        use TestStructuralFaultPhase::{
            AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterResourceProjection,
            AfterRuntimePrepare, BeforeProtocolCommit,
        };

        const PHASES: [TestStructuralFaultPhase; 6] = [
            AfterProtocolPrepare,
            AfterResourceProjection,
            AfterRuntimePrepare,
            AfterAdmission,
            AfterPlanReservation,
            BeforeProtocolCommit,
        ];
        fn accumulate_fault(counters: &mut TestOwnerCounters, phase: TestStructuralFaultPhase) {
            counters.token_constructed += 1;
            counters.token_disposed += 1;
            counters.replay_candidate_constructed += 1;
            counters.replay_candidate_disposed += 1;
            if matches!(
                phase,
                AfterRuntimePrepare | AfterAdmission | AfterPlanReservation | BeforeProtocolCommit
            ) {
                counters.candidate_provider_constructed += 1;
                counters.candidate_provider_disposed += 1;
                counters.candidate_plan_constructed += 1;
                counters.candidate_plan_disposed += 1;
            }
            if matches!(phase, AfterPlanReservation | BeforeProtocolCommit) {
                counters.reservation_constructed += 1;
                counters.reservation_canceled += 1;
            }
        }
        fn accumulate_success(counters: &mut TestOwnerCounters) {
            counters.token_constructed += 1;
            counters.token_disposed += 1;
            counters.replay_candidate_constructed += 1;
            counters.replay_candidate_published += 1;
            counters.replay_current_disposed += 1;
            counters.candidate_provider_constructed += 1;
            counters.candidate_provider_published += 1;
            counters.candidate_plan_constructed += 1;
            counters.candidate_plan_published += 1;
            counters.reservation_constructed += 1;
            counters.reservation_committed += 1;
        }

        for first in PHASES {
            for second in PHASES {
                let (c_session, c_plan) = boxed_c_children(SESSION);
                crate::ffi::test_set_structural_faults(c_session, [Some(first), Some(second)]);
                let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
                    session_id: miso_engine_session::StableId::parse("fault-matrix")
                        .expect("stable ID"),
                };
                let request = command_bytes_at_revision(
                    1,
                    ExpectedRevision::Exact(SessionRevision(42)),
                    miso_engine_protocol::CommandPayload::SessionTransactionApply(
                        core::slice::from_ref(&edit),
                    ),
                );
                let before = crate::ffi::test_session_state_summary(c_session);
                let mut expected = TestOwnerCounters {
                    current_provider_constructed: 1,
                    current_plan_constructed: 1,
                    replay_current_constructed: 1,
                    ..TestOwnerCounters::default()
                };
                for phase in [first, second] {
                    let (result, canary) = command_c(c_session, &request);
                    assert_eq!(result, crate::RESULT_BACKPRESSURE, "{first:?}/{second:?}");
                    assert_eq!(canary, vec![0xa5; 4_096]);
                    assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
                    assert_eq!(
                        event_c(c_session, crate::EVENT_LANE_RELIABLE),
                        (crate::RESULT_OK, Vec::new())
                    );
                    accumulate_fault(&mut expected, phase);
                    assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
                }

                let (result, response) = command_c(c_session, &request);
                assert_eq!(result, crate::RESULT_OK);
                assert_ne!(response, vec![0xa5; 4_096]);
                accumulate_success(&mut expected);
                assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
                assert_eq!(
                    crate::ffi::test_session_state_summary(c_session),
                    (43, 1, 0, 1)
                );

                let mut pcm = [f32::NAN; 256];
                let output = crate::PlanarOutput {
                    struct_size: crate::PLANAR_OUTPUT_SIZE,
                    channels: 2,
                    samples: pcm.as_mut_ptr(),
                    sample_capacity: pcm.len() as u64,
                    frames: 128,
                    plane_stride_samples: 128,
                    reserved: [0; 2],
                };
                assert_eq!(
                    crate::ffi::test_render(c_plan, 0, &output),
                    crate::RESULT_OK
                );
                assert_eq!(
                    event_c(c_session, crate::EVENT_LANE_RELIABLE).0,
                    crate::RESULT_OK
                );
                expected.current_provider_disposed += 1;
                expected.current_plan_disposed += 1;
                assert_eq!(crate::ffi::test_owner_counters(c_session), expected);

                let retry_edit = miso_engine_protocol::SessionEditV1::SetSessionId {
                    session_id: miso_engine_session::StableId::parse("fault-matrix-retry")
                        .expect("stable ID"),
                };
                let retry = command_bytes_at_revision(
                    2,
                    ExpectedRevision::Exact(SessionRevision(43)),
                    miso_engine_protocol::CommandPayload::SessionTransactionApply(
                        core::slice::from_ref(&retry_edit),
                    ),
                );
                assert_eq!(command_c(c_session, &retry).0, crate::RESULT_OK);
                assert_eq!(
                    crate::ffi::test_session_state_summary(c_session),
                    (44, 2, 1, 1),
                    "reclaim released publication and retirement credit"
                );
                crate::ffi::test_plan_destroy(c_plan);
                crate::ffi::test_session_destroy(c_session);
            }
        }
    }

    #[test]
    fn capi_controller_dispatches_every_advertised_command_family() {
        let (c_session, c_plan) = boxed_c_children(SESSION);
        let eager_capacities = crate::ffi::test_retained_capacities(c_session);
        assert!(eager_capacities.iter().all(|capacity| *capacity > 0));
        let codec = ProtocolCodec::default();
        let mut request_id = 0_u64;
        macro_rules! dispatch {
            ($expected:expr, $payload:expr, $status:expr, $revision:expr, $events:expr) => {{
                request_id += 1;
                let mut request = vec![0_u8; 4_096];
                let len = codec
                    .encode_command_frame_into(
                        &miso_engine_protocol::TypedCommandFrame {
                            request_id: RequestId::new(request_id).expect("request ID"),
                            expected_revision: $expected,
                            payload: $payload,
                        },
                        &mut request,
                    )
                    .expect("command frame");
                request.truncate(len);
                let pinned = pinned_hex(ALL_COMMAND_RESPONSE_VECTORS[request_id as usize - 1]);
                let (c_result, c_bytes) = command_c(c_session, &request);
                assert_eq!(c_result, crate::RESULT_OK, "C command {request_id}");
                assert_eq!(c_bytes, pinned, "pinned C bytes {request_id}");

                let (c_replay_result, c_replay) = command_c(c_session, &request);
                assert_eq!(c_replay_result, crate::RESULT_OK, "C replay {request_id}");
                assert_eq!(c_replay, pinned, "pinned replay bytes {request_id}");
                let mut fields = [0_u16; 512];
                let response = codec
                    .decode_typed_response(&c_bytes, &mut DecodeScratch::new(&mut fields))
                    .expect("typed response");
                let header = match response {
                    miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                    | miso_engine_protocol::DecodedTypedResponseFrame::NonOk { header, .. } => {
                        header
                    }
                };
                assert_eq!(header.status, $status, "accepted status {request_id}");
                assert_eq!(header.revision, SessionRevision($revision));
                let mut event_ids = Vec::new();
                loop {
                    let (event_result, c_event) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
                    assert_eq!(event_result, crate::RESULT_OK);
                    if c_event.is_empty() {
                        break;
                    }
                    let mut event_fields = [0_u16; 64];
                    event_ids.push(
                        codec
                            .decode_typed_event(
                                &c_event,
                                &mut DecodeScratch::new(&mut event_fields),
                            )
                            .expect("typed command event")
                            .header
                            .message_id,
                    );
                }
                assert_eq!(
                    event_c(c_session, crate::EVENT_LANE_RELIABLE),
                    (crate::RESULT_OK, Vec::new())
                );
                assert_eq!(event_ids.as_slice(), $events, "events {request_id}");
                let summary = crate::ffi::test_session_state_summary(c_session);
                assert_eq!(summary.0, $revision as u64);
                assert_eq!(summary.1, request_id as usize);
                assert_eq!(summary.2, 0, "pinned active provider epoch");
                assert_eq!(
                    summary.3,
                    usize::from(request_id == 11),
                    "pinned pending provider count"
                );
                header.message_id
            }};
        }

        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::CapabilitiesGet,
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::CapabilitiesGet
        );
        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::SessionSnapshotGet(
                    miso_engine_protocol::SessionSnapshotRequest {
                        offset: 0,
                        maximum_bytes: 1,
                    },
                ),
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::SessionSnapshotGet
        );
        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::ParameterMetadataGet(
                    miso_engine_protocol::ParameterMetadataRequest {
                        after_handle: 0,
                        limit: 1,
                    },
                ),
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::ParameterMetadataGet
        );
        let state = miso_engine_protocol::ParameterStateRequest { handles: vec![1] };
        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::ParameterStateGet(&state),
                StatusCode::NotFound,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::ParameterStateGet
        );
        let automation = [miso_engine_protocol::AutomationRecord {
            kind: miso_engine_protocol::AutomationKind::Point,
            handle: miso_engine_protocol::ParameterHandle(1),
            start: miso_engine_protocol::SampleTime(1),
            end: miso_engine_protocol::SampleTime(1),
            start_value: 0.0,
            end_value: 0.0,
        }];
        assert_eq!(
            dispatch!(
                ExpectedRevision::Exact(SessionRevision(42)),
                miso_engine_protocol::CommandPayload::AutomationEnqueue(
                    miso_engine_protocol::AutomationEnqueue {
                        records: &automation,
                    },
                ),
                StatusCode::NotFound,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::AutomationEnqueue
        );
        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::TransportGet,
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::TransportGet
        );
        assert_eq!(
            dispatch!(
                ExpectedRevision::Exact(SessionRevision(42)),
                miso_engine_protocol::CommandPayload::TransportSet(
                    miso_engine_protocol::TransportSetRequest {
                        state: miso_engine_protocol::TransportState::Playing,
                        position: Some(miso_engine_protocol::SampleTime(0)),
                    },
                ),
                StatusCode::Ok,
                42,
                &[miso_engine_protocol::MessageId::TransportState]
            ),
            miso_engine_protocol::MessageId::TransportSet
        );
        let telemetry = miso_engine_protocol::TelemetryConfiguration {
            meter_handles: Vec::new(),
            meter_period_blocks: 0,
            counter_ids: Vec::new(),
            counter_period_blocks: 0,
            diagnostics_enabled: false,
            minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
        };
        assert_eq!(
            dispatch!(
                ExpectedRevision::Exact(SessionRevision(42)),
                miso_engine_protocol::CommandPayload::TelemetryConfigure(&telemetry),
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::TelemetryConfigure
        );
        let counters = miso_engine_protocol::CountersRequest {
            all: true,
            ids: Vec::new(),
        };
        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::CountersGet(&counters),
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::CountersGet
        );
        assert_eq!(
            dispatch!(
                ExpectedRevision::Any,
                miso_engine_protocol::CommandPayload::DiagnosticsGet(
                    miso_engine_protocol::DiagnosticsRequest {
                        after_sequence: 0,
                        limit: 1,
                        minimum_severity: miso_engine_protocol::DiagnosticSeverity::Info,
                    },
                ),
                StatusCode::Ok,
                42,
                &[]
            ),
            miso_engine_protocol::MessageId::DiagnosticsGet
        );
        let structural = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("all-command-families")
                .expect("stable ID"),
        };
        assert_eq!(
            dispatch!(
                ExpectedRevision::Exact(SessionRevision(42)),
                miso_engine_protocol::CommandPayload::SessionTransactionApply(
                    core::slice::from_ref(&structural),
                ),
                StatusCode::Ok,
                43,
                &[miso_engine_protocol::MessageId::SessionCommitted]
            ),
            miso_engine_protocol::MessageId::SessionTransactionApply
        );
        assert_eq!(request_id, 11);
        assert_eq!(
            crate::ffi::test_session_state_summary(c_session),
            (43, 11, 0, 1),
            "pinned revision/replay/provider/pending vector"
        );
        assert_eq!(
            crate::ffi::test_retained_capacities(c_session),
            eager_capacities
        );
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
    }

    #[test]
    fn exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact() {
        let mut direct = compile_children(SESSION, limits()).expect("direct children");
        let (c_session, c_plan) = boxed_c_children(SESSION);
        let codec = ProtocolCodec::default();
        macro_rules! parity {
            ($request:expr, $status:expr) => {{
                let request = $request;
                let len = direct
                    .session
                    .command(&request, 4_096)
                    .expect("direct decision");
                let oracle = direct.session.command_response(len).to_vec();
                let (result, bytes) = command_c(c_session, &request);
                assert_eq!(result, crate::RESULT_OK);
                assert_eq!(bytes, oracle);
                let mut fields = [0_u16; 64];
                let decoded = codec
                    .decode_typed_response(&oracle, &mut DecodeScratch::new(&mut fields))
                    .expect("typed decision");
                let header = match decoded {
                    miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                    | miso_engine_protocol::DecodedTypedResponseFrame::NonOk { header, .. } => {
                        header
                    }
                };
                assert_eq!(header.status, $status);
                oracle
            }};
        }

        let first = command_bytes(1, miso_engine_protocol::CommandPayload::CapabilitiesGet);
        let first_bytes = parity!(first.clone(), StatusCode::Ok);
        assert_eq!(parity!(first.clone(), StatusCode::Ok), first_bytes);
        let conflict = command_bytes(1, miso_engine_protocol::CommandPayload::TransportGet);
        parity!(conflict, StatusCode::RequestIdReuse);
        for request_id in 2..=18 {
            parity!(
                command_bytes(
                    request_id,
                    miso_engine_protocol::CommandPayload::CapabilitiesGet
                ),
                StatusCode::Ok
            );
        }
        parity!(first, StatusCode::ReplayExpired);

        let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("pressure-one").expect("stable ID"),
        };
        let stale = command_bytes_at_revision(
            19,
            ExpectedRevision::Exact(SessionRevision(41)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        parity!(stale, StatusCode::RevisionConflict);

        for sequence in [1, 2] {
            let event = miso_engine_protocol::ReliableSlot::transport_state(
                SessionRevision(42),
                sequence,
                miso_engine_protocol::TransportState::Stopped,
                miso_engine_protocol::SampleTime(0),
                miso_engine_protocol::SampleTime(0),
                None,
            );
            direct
                .session
                .test_enqueue_reliable(event)
                .expect("fill direct reliable");
            crate::ffi::test_enqueue_reliable(c_session, event).expect("fill C reliable");
        }
        let event_full = command_bytes_at_revision(
            20,
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        parity!(event_full, StatusCode::Backpressure);
        for _ in 0..2 {
            let len = direct
                .session
                .dequeue_event(EventLane::Reliable, 4_096)
                .expect("reliable drain")
                .expect("retained full event");
            let oracle = direct.session.event_response(len).to_vec();
            let (result, bytes) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
            assert_eq!(result, crate::RESULT_OK);
            assert_eq!(bytes, oracle, "event-full command did not drop FIFO data");
        }

        let first_structural = command_bytes_at_revision(
            21,
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &edit,
            )),
        );
        parity!(first_structural, StatusCode::Ok);
        let len = direct
            .session
            .dequeue_event(EventLane::Reliable, 4_096)
            .expect("commit event")
            .expect("commit bytes");
        let oracle = direct.session.event_response(len).to_vec();
        assert_eq!(event_c(c_session, crate::EVENT_LANE_RELIABLE).1, oracle);

        let second_edit = miso_engine_protocol::SessionEditV1::SetSessionId {
            session_id: miso_engine_session::StableId::parse("pressure-two").expect("stable ID"),
        };
        let publication_full = command_bytes_at_revision(
            22,
            ExpectedRevision::Exact(SessionRevision(43)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &second_edit,
            )),
        );
        let before = crate::ffi::test_session_state_summary(c_session);
        crate::ffi::test_set_capi_retained_limit(c_session, 0);
        let (resource_result, resource_canary) = command_c(c_session, &publication_full);
        assert_eq!(resource_result, crate::RESULT_COMPILE_REJECTED);
        assert_eq!(resource_canary, vec![0xa5; 4_096]);
        assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
        crate::ffi::test_set_capi_retained_limit(c_session, limits().maximum_capi_retained_bytes);
        assert!(matches!(
            direct.session.command(&publication_full, 4_096),
            Err(CommandError::Backpressure)
        ));
        let (result, canary) = command_c(c_session, &publication_full);
        assert_eq!(result, crate::RESULT_BACKPRESSURE);
        assert_eq!(canary, vec![0xa5; 4_096]);
        assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
        assert_eq!(before.0, 43);
        assert_eq!(before.3, 1);

        let mut direct_pcm = [f32::NAN; 256];
        direct
            .plan
            .render(
                0,
                PlanarBufferMut::try_new(&mut direct_pcm, 2, 128, 128)
                    .expect("direct boundary output"),
            )
            .expect("direct boundary");
        let mut c_pcm = [f32::NAN; 256];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: c_pcm.as_mut_ptr(),
            sample_capacity: c_pcm.len() as u64,
            frames: 128,
            plane_stride_samples: 128,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(c_plan, 0, &output),
            crate::RESULT_OK
        );
        assert_eq!(
            c_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            direct_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>()
        );
        let retry = parity!(publication_full, StatusCode::Ok);
        assert!(!retry.is_empty());
        let after_retry = crate::ffi::test_session_state_summary(c_session);
        assert_eq!(after_retry.0, 44);
        assert_eq!(after_retry.2, 1);
        assert_eq!(after_retry.3, 1);

        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
    }

    #[test]
    fn direct_and_c_render_match_one_and_ten_tracks_across_launch_rates() {
        for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
            render_parity_shape(1, sample_rate_hz);
            render_parity_shape(10, sample_rate_hz);
        }
    }

    #[test]
    fn barrier_schedule_separates_one_source_producer_from_exclusive_render() {
        let mut model =
            parse_session_toml(&generated_parity_session(1, 48_000)).expect("concurrency session");
        model.sources[0].mapping.region.length_samples = 1_024;
        let session = miso_engine_session::canonical_session_toml(&model).expect("canonical");
        let children = compile_children(&session, limits()).expect("concurrent children");
        let session = Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        ))) as usize;
        let plan = Box::into_raw(Box::new(crate::Plan::new(
            children.plan,
            children.plan_error,
        ))) as usize;
        let submitted = std::sync::Arc::new(std::sync::Barrier::new(2));
        let consumed = std::sync::Arc::new(std::sync::Barrier::new(2));
        std::thread::scope(|scope| {
            let producer_submitted = submitted.clone();
            let producer_consumed = consumed.clone();
            scope.spawn(move || {
                let session = session as *mut crate::Session;
                let left = [0.25_f32; 128];
                let right = [-0.5_f32; 128];
                for block in 0..6_u64 {
                    let (generation, start_frame) = if block < 3 {
                        (1, block * 128)
                    } else {
                        if block == 3 {
                            assert_eq!(
                                crate::ffi::test_source_seek(session, b"fixture-source", 2, 512,),
                                crate::RESULT_OK
                            );
                        }
                        (2, 512 + (block - 3) * 128)
                    };
                    submit_c(
                        session,
                        generation,
                        start_frame,
                        48_000,
                        &left,
                        &right,
                        false,
                    );
                    producer_submitted.wait();
                    producer_consumed.wait();
                }
            });
            let render_submitted = submitted.clone();
            let render_consumed = consumed.clone();
            scope.spawn(move || {
                let plan = plan as *mut crate::Plan;
                let mut observed_signal = false;
                for block in 0..6_u64 {
                    render_submitted.wait();
                    let mut pcm = [f32::NAN; 256];
                    let output = crate::PlanarOutput {
                        struct_size: crate::PLANAR_OUTPUT_SIZE,
                        channels: 2,
                        samples: pcm.as_mut_ptr(),
                        sample_capacity: pcm.len() as u64,
                        frames: 128,
                        plane_stride_samples: 128,
                        reserved: [0; 2],
                    };
                    assert_eq!(
                        crate::ffi::test_render(plan, block * 128, &output),
                        crate::RESULT_OK
                    );
                    assert!(pcm.iter().all(|sample| sample.is_finite()));
                    observed_signal |= pcm.iter().any(|sample| *sample != 0.0);
                    render_consumed.wait();
                }
                assert!(observed_signal);
            });
        });
        crate::ffi::test_session_destroy(session as *mut crate::Session);
        crate::ffi::test_plan_destroy(plan as *mut crate::Plan);
    }
}
