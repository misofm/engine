//! Safe control-plane orchestration behind the raw FFI boundary.

use core::{alloc::Layout, mem::size_of};

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::{
    SampleRateHz,
    realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo, RenderTime},
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
    Backpressure, BackpressureQueueKind, Capabilities, CapabilityFlags, CommandHeader,
    DecodeScratch, DecodedCommandPayload, EncodeError, ExpectedRevision, MessageId, NonOkResponse,
    ProtocolCodec, ProtocolLimits, ProtocolVersion, RequestId, SessionRevision, StatusCode,
    SuccessResponsePayload, TypedNonOkResponseFrame, TypedSuccessResponseFrame,
};
use miso_engine_session::{CompileCaps, DiagnosticSet, compile_session, parse_session_toml};
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

#[derive(Clone, Copy, Default)]
struct ReplayEntryRecord {
    request_offset: u64,
    request_bytes: u64,
    response_offset: u64,
    response_bytes: u64,
    request_id: u64,
    occupied: u64,
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

pub(crate) struct SessionState {
    _canonical_session: Box<[u8]>,
    source_ids: Box<[u8]>,
    sources: Box<[ControlSource]>,
    codec: ProtocolCodec,
    revision: SessionRevision,
    quantum_frames: u32,
    decode_fields: Box<[u16]>,
    _decode_tail: Option<Box<[u8]>>,
    response_scratch: Box<[u8]>,
    replay_payload: Box<[u8]>,
    replay_entries: Box<[ReplayEntryRecord]>,
    replay_len: usize,
    replay_used: usize,
    highest_new_request_id: Option<RequestId>,
}

pub(crate) struct PlanState {
    pub(crate) _plan: PreparedRenderPlan,
    pub(crate) resources: PlanResourceReport,
    next_absolute_sample: u64,
}

pub(crate) struct CompiledChildren {
    pub(crate) session: SessionState,
    pub(crate) session_error: FixedBytes,
    pub(crate) plan: PlanState,
    pub(crate) plan_error: FixedBytes,
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
    retained: u64,
    largest: u64,
}

const CAPABILITY_COMMANDS: [u16; 1] = [MessageId::CapabilitiesGet as u16];

fn encode_capabilities_response(
    codec: ProtocolCodec,
    revision: SessionRevision,
    request_id: RequestId,
    replay_entries: u64,
    replay_bytes: u64,
    quantum_frames: u32,
    output: &mut [u8],
) -> Result<usize, EncodeError> {
    let limits = codec.limits();
    codec.encode_success_response_frame_into(
        &TypedSuccessResponseFrame {
            request_id,
            revision,
            payload: SuccessResponsePayload::Capabilities(Capabilities {
                minimum_version: ProtocolVersion::V1,
                maximum_version: ProtocolVersion::V1,
                maximum_frame_bytes: limits.max_frame_bytes as u64,
                maximum_tlvs: limits.max_tlv_count,
                maximum_string_bytes: limits.max_string_bytes as u64,
                maximum_nesting: limits.max_nesting,
                maximum_automation_records: miso_engine_protocol::AUTOMATION_BATCH_RECORDS as u16,
                control_command_slots: 0,
                control_command_bytes: 0,
                automation_batch_slots: 0,
                reliable_response_slots: 0,
                reliable_event_slots: 0,
                telemetry_slots: 0,
                replay_entries,
                replay_bytes,
                maximum_cached_response_bytes: limits.max_frame_bytes as u64,
                per_block_automation_density: 0,
                admission_quantum_frames: u64::from(quantum_frames),
                maximum_parameter_page_items: 0,
                maximum_diagnostic_page_items: 0,
                maximum_telemetry_handles: 0,
                maximum_transaction_edits: 0,
                supported_commands: &CAPABILITY_COMMANDS,
                supported_events: &[],
                flags: CapabilityFlags(0b111),
            }),
        },
        output,
    )
}

fn capi_resources(
    limits: CompileLimits,
    canonical_bytes: usize,
    source_count: usize,
    source_id_bytes: usize,
) -> Result<CapiResources, CompileFailure> {
    let rows = [
        checked_layout::<u8>(canonical_bytes)?,
        checked_layout::<ControlSource>(source_count)?,
        checked_layout::<u8>(source_id_bytes)?,
        checked_byte_layout(limits.maximum_diagnostic_bytes)?,
        checked_byte_layout(limits.maximum_diagnostic_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_byte_layout(limits.maximum_control_frame_bytes)?,
        checked_byte_layout(limits.maximum_replay_bytes)?,
        checked_layout::<ReplayEntryRecord>(
            usize::try_from(limits.maximum_replay_entries)
                .map_err(|_| failure("capi.resource.platform"))?,
        )?,
        checked_layout::<crate::Session>(1)?,
        checked_layout::<crate::Plan>(1)?,
    ];
    let retained = rows.into_iter().try_fold(0_u64, |total, row| {
        total
            .checked_add(row)
            .ok_or_else(|| failure("capi.resource.arithmetic"))
    })?;
    let largest = rows.into_iter().max().unwrap_or(0);
    Ok(CapiResources { retained, largest })
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

pub(crate) fn compile_children(
    toml: &str,
    limits: CompileLimits,
) -> Result<CompiledChildren, CompileFailure> {
    let model = parse_session_toml(toml).map_err(|value| session_diagnostics(&value))?;
    let track_count = u64::try_from(model.tracks.len()).map_err(|_| failure("capi.count"))?;
    let source_count = u64::try_from(model.sources.len()).map_err(|_| failure("capi.count"))?;
    let route_count = u64::try_from(model.routes.len()).map_err(|_| failure("capi.count"))?;
    let effect_count = count_effects(&model)?;
    if track_count > limits.maximum_tracks
        || source_count > limits.maximum_sources
        || route_count > limits.maximum_routes
        || effect_count > limits.maximum_effects
    {
        return Err(failure("capi.resource.count"));
    }

    let aggregate_ring_frames = source_count
        .checked_mul(u64::from(limits.source_ring_frames))
        .ok_or_else(|| failure("capi.resource.arithmetic"))?;
    let compiled = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: limits.maximum_graph_session_plus_plan_bytes,
            max_requested_runtime_bytes: limits.maximum_graph_session_plus_plan_bytes,
            max_single_allocation_bytes: limits.maximum_named_allocation_bytes,
            max_queue_items: u64::MAX,
            max_source_ring_frames: aggregate_ring_frames,
            max_source_ring_bytes: limits.maximum_source_total_bytes,
        },
    )
    .map_err(|value| session_diagnostics(&value))?;
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

    let canonical_session = compiled.canonical_toml().as_bytes().to_vec();
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
    let capi = capi_resources(
        limits,
        canonical_session.len(),
        compiled.source_count(),
        source_id_bytes,
    )?;
    if capi.retained > limits.maximum_capi_retained_bytes
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
        &compiled,
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
        &compiled,
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
    if graph_resources.session_plus_plan_bytes > limits.maximum_graph_session_plus_plan_bytes {
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
    if largest_named > limits.maximum_named_allocation_bytes
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
        capi_retained_bytes: capi.retained,
        largest_named_allocation_bytes: largest_named,
        reserved: [0; 4],
    };

    let replay_entry_count = usize::try_from(limits.maximum_replay_entries)
        .map_err(|_| failure("capi.resource.platform"))?;
    let mut replay_entries = Vec::new();
    replay_entries
        .try_reserve_exact(replay_entry_count)
        .map_err(|_| failure("capi.resource.allocation"))?;
    replay_entries.resize(replay_entry_count, ReplayEntryRecord::default());
    let control_bytes = usize::try_from(limits.maximum_control_frame_bytes)
        .map_err(|_| failure("capi.resource.platform"))?;
    let decode_field_count = control_bytes / size_of::<u16>();
    let maximum_tlvs = u32::try_from(decode_field_count).unwrap_or(u32::MAX);
    let codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: control_bytes,
        max_tlv_count: maximum_tlvs,
        max_string_bytes: control_bytes,
        max_nesting: 4,
    });
    let revision = SessionRevision(compiled.normalized_model().revision);
    let mut response_scratch = boxed_zeroed(limits.maximum_control_frame_bytes)?;
    encode_capabilities_response(
        codec,
        revision,
        RequestId::new(1).expect("one is a nonzero protocol request ID"),
        limits.maximum_replay_entries,
        limits.maximum_replay_bytes,
        compiled.quantum().0,
        &mut response_scratch,
    )
    .map_err(|_| failure("capi.control.frame_limit"))?;
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
            _canonical_session: canonical_session.into_boxed_slice(),
            source_ids: ids.into_boxed_slice(),
            sources: controls.into_boxed_slice(),
            codec,
            revision,
            quantum_frames: compiled.quantum().0,
            decode_fields: decode_fields.into_boxed_slice(),
            _decode_tail: decode_tail,
            response_scratch,
            replay_payload: boxed_zeroed(limits.maximum_replay_bytes)?,
            replay_entries: replay_entries.into_boxed_slice(),
            replay_len: 0,
            replay_used: 0,
            highest_new_request_id: None,
        },
        session_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
        plan: PlanState {
            _plan: bound.plan,
            resources,
            next_absolute_sample: 0,
        },
        plan_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
    })
}

impl PlanState {
    pub(crate) fn quantum_frames(&self) -> u32 {
        self._plan.envelope().quantum.0
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
            ._plan
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime { absolute_sample },
            )
            .map_err(|_| ())?;
        self.next_absolute_sample = report.next_absolute_sample;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandError {
    Invalid,
    BufferTooSmall { required: u64 },
    Internal,
}

#[derive(Clone, Copy)]
enum ReplayRoute {
    Cached(usize),
    RequestIdReuse,
    ReplayExpired,
    Backpressure,
    Execute,
}

fn protocol_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_le_bytes(
        bytes.get(offset..offset + 2)?.try_into().ok()?,
    ))
}

fn protocol_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

fn protocol_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        bytes.get(offset..offset + 8)?.try_into().ok()?,
    ))
}

fn protocol_message_id(raw: u16) -> Option<MessageId> {
    Some(match raw {
        0x0001 => MessageId::CapabilitiesGet,
        0x0002 => MessageId::SessionSnapshotGet,
        0x0003 => MessageId::SessionTransactionApply,
        0x0004 => MessageId::ParameterMetadataGet,
        0x0005 => MessageId::ParameterStateGet,
        0x0006 => MessageId::AutomationEnqueue,
        0x0007 => MessageId::TransportGet,
        0x0008 => MessageId::TransportSet,
        0x0009 => MessageId::TelemetryConfigure,
        0x000a => MessageId::CountersGet,
        0x000b => MessageId::DiagnosticsGet,
        _ => return None,
    })
}

fn correlatable_command_header(
    codec: ProtocolCodec,
    input: &[u8],
) -> Result<CommandHeader, CommandError> {
    if input.len() > codec.limits().max_frame_bytes {
        return Err(CommandError::Invalid);
    }
    let header = input
        .get(..miso_engine_protocol::OUTER_HEADER_BYTES)
        .ok_or(CommandError::Invalid)?;
    if header[..8] != *b"MISOCTL\0"
        || protocol_u16(header, 8) != Some(1)
        || protocol_u16(header, 12) != Some(miso_engine_protocol::OUTER_HEADER_BYTES as u16)
        || header[14] != 1
        || header[15] & !1 != 0
        || protocol_u16(header, 18) != Some(0)
        || protocol_u32(header, 44) != Some(0)
    {
        return Err(CommandError::Invalid);
    }
    let message_id = protocol_u16(header, 16)
        .and_then(protocol_message_id)
        .ok_or(CommandError::Invalid)?;
    let payload_len = protocol_u32(header, 20).ok_or(CommandError::Invalid)?;
    if miso_engine_protocol::OUTER_HEADER_BYTES
        .checked_add(usize::try_from(payload_len).map_err(|_| CommandError::Invalid)?)
        != Some(input.len())
    {
        return Err(CommandError::Invalid);
    }
    let request_id = protocol_u64(header, 24)
        .and_then(RequestId::new)
        .ok_or(CommandError::Invalid)?;
    let wire_revision = protocol_u64(header, 32).ok_or(CommandError::Invalid)?;
    let tlv_count = protocol_u32(header, 40).ok_or(CommandError::Invalid)?;
    if tlv_count > codec.limits().max_tlv_count {
        return Err(CommandError::Invalid);
    }
    let expected_revision = if header[15] & 1 != 0 {
        if wire_revision != 0 {
            return Err(CommandError::Invalid);
        }
        ExpectedRevision::Any
    } else {
        ExpectedRevision::Exact(SessionRevision(wire_revision))
    };
    Ok(CommandHeader {
        version: ProtocolVersion {
            major: 1,
            minor: protocol_u16(header, 10).ok_or(CommandError::Invalid)?,
        },
        message_id,
        request_id,
        expected_revision,
        payload_len,
        tlv_count,
    })
}

impl SessionState {
    fn replay_index(&self, request_id: RequestId) -> Option<usize> {
        self.replay_entries[..self.replay_len]
            .iter()
            .position(|entry| entry.occupied == 1 && entry.request_id == request_id.get())
    }

    fn evict_oldest_replay(&mut self) -> Result<(), CommandError> {
        let first = *self
            .replay_entries
            .first()
            .filter(|_| self.replay_len != 0)
            .ok_or(CommandError::Internal)?;
        let removed = usize::try_from(
            first
                .response_offset
                .checked_add(first.response_bytes)
                .ok_or(CommandError::Internal)?,
        )
        .map_err(|_| CommandError::Internal)?;
        if first.request_offset != 0 || removed > self.replay_used {
            return Err(CommandError::Internal);
        }
        self.replay_payload
            .copy_within(removed..self.replay_used, 0);
        for index in 1..self.replay_len {
            let mut entry = self.replay_entries[index];
            entry.request_offset = entry
                .request_offset
                .checked_sub(removed as u64)
                .ok_or(CommandError::Internal)?;
            entry.response_offset = entry
                .response_offset
                .checked_sub(removed as u64)
                .ok_or(CommandError::Internal)?;
            self.replay_entries[index - 1] = entry;
        }
        self.replay_len -= 1;
        self.replay_entries[self.replay_len] = ReplayEntryRecord::default();
        self.replay_used -= removed;
        Ok(())
    }

    fn replay_preflight(
        &mut self,
        request_id: RequestId,
        request: &[u8],
    ) -> Result<ReplayRoute, CommandError> {
        if let Some(index) = self.replay_index(request_id) {
            let entry = self.replay_entries[index];
            let start =
                usize::try_from(entry.request_offset).map_err(|_| CommandError::Internal)?;
            let bytes = usize::try_from(entry.request_bytes).map_err(|_| CommandError::Internal)?;
            let retained = self
                .replay_payload
                .get(start..start.checked_add(bytes).ok_or(CommandError::Internal)?)
                .ok_or(CommandError::Internal)?;
            return Ok(if retained == request {
                ReplayRoute::Cached(index)
            } else {
                ReplayRoute::RequestIdReuse
            });
        }
        if self
            .highest_new_request_id
            .is_some_and(|highest| request_id <= highest)
        {
            return Ok(ReplayRoute::ReplayExpired);
        }
        let reservation = request
            .len()
            .checked_add(self.response_scratch.len())
            .ok_or(CommandError::Internal)?;
        if reservation > self.replay_payload.len() {
            return Ok(ReplayRoute::Backpressure);
        }
        while self.replay_len >= self.replay_entries.len()
            || self.replay_used.saturating_add(reservation) > self.replay_payload.len()
        {
            if self.replay_len == 0 {
                return Ok(ReplayRoute::Backpressure);
            }
            self.evict_oldest_replay()?;
        }
        self.highest_new_request_id = Some(request_id);
        Ok(ReplayRoute::Execute)
    }

    fn encode_status(
        &mut self,
        header: CommandHeader,
        status: StatusCode,
        backpressure: Option<Backpressure>,
    ) -> Result<usize, CommandError> {
        let payload = NonOkResponse {
            diagnostics: Vec::new(),
            omitted_diagnostics: 0,
            backpressure,
        };
        self.codec
            .encode_non_ok_response_frame_into(
                &TypedNonOkResponseFrame {
                    request_id: header.request_id,
                    revision: self.revision,
                    message_id: header.message_id,
                    status,
                    payload: &payload,
                },
                &mut self.response_scratch,
            )
            .map_err(|_| CommandError::Internal)
    }

    fn complete_replay(
        &mut self,
        request_id: RequestId,
        request: &[u8],
        response_bytes: usize,
    ) -> Result<(), CommandError> {
        let request_offset = self.replay_used;
        let response_offset = request_offset
            .checked_add(request.len())
            .ok_or(CommandError::Internal)?;
        let end = response_offset
            .checked_add(response_bytes)
            .ok_or(CommandError::Internal)?;
        if self.replay_len >= self.replay_entries.len() || end > self.replay_payload.len() {
            return Err(CommandError::Internal);
        }
        self.replay_payload[request_offset..response_offset].copy_from_slice(request);
        self.replay_payload[response_offset..end]
            .copy_from_slice(&self.response_scratch[..response_bytes]);
        self.replay_entries[self.replay_len] = ReplayEntryRecord {
            request_offset: request_offset as u64,
            request_bytes: request.len() as u64,
            response_offset: response_offset as u64,
            response_bytes: response_bytes as u64,
            request_id: request_id.get(),
            occupied: 1,
        };
        self.replay_len += 1;
        self.replay_used = end;
        Ok(())
    }

    pub(crate) fn command(
        &mut self,
        request: &[u8],
        output_capacity: u64,
    ) -> Result<usize, CommandError> {
        let header = correlatable_command_header(self.codec, request)?;
        let is_new = self.replay_index(header.request_id).is_none()
            && self
                .highest_new_request_id
                .is_none_or(|highest| header.request_id > highest);
        if is_new && output_capacity < self.response_scratch.len() as u64 {
            return Err(CommandError::BufferTooSmall {
                required: self.response_scratch.len() as u64,
            });
        }
        let route = self.replay_preflight(header.request_id, request)?;
        let response_bytes = match route {
            ReplayRoute::Cached(index) => {
                let entry = self.replay_entries[index];
                let start =
                    usize::try_from(entry.response_offset).map_err(|_| CommandError::Internal)?;
                let bytes =
                    usize::try_from(entry.response_bytes).map_err(|_| CommandError::Internal)?;
                let end = start.checked_add(bytes).ok_or(CommandError::Internal)?;
                let retained = self
                    .replay_payload
                    .get(start..end)
                    .ok_or(CommandError::Internal)?;
                self.response_scratch[..bytes].copy_from_slice(retained);
                bytes
            }
            ReplayRoute::RequestIdReuse => {
                self.encode_status(header, StatusCode::RequestIdReuse, None)?
            }
            ReplayRoute::ReplayExpired => {
                self.encode_status(header, StatusCode::ReplayExpired, None)?
            }
            ReplayRoute::Backpressure => {
                let requested_bytes = request.len().saturating_add(self.response_scratch.len());
                self.encode_status(
                    header,
                    StatusCode::Backpressure,
                    Some(Backpressure {
                        queue_kind: BackpressureQueueKind::ReplayCache,
                        capacity: self.replay_entries.len() as u64,
                        occupancy: self.replay_len as u64,
                        requested_items: 1,
                        generation: None,
                        retry_boundary: None,
                        requested_bytes: Some(requested_bytes as u64),
                        available_bytes: Some(
                            self.replay_payload.len().saturating_sub(self.replay_used) as u64,
                        ),
                    }),
                )?
            }
            ReplayRoute::Execute => {
                let decoded = self.codec.decode_typed_command(
                    request,
                    &mut DecodeScratch::new(&mut self.decode_fields),
                );
                let bytes = match decoded {
                    Ok(decoded) => match decoded.payload {
                        DecodedCommandPayload::CapabilitiesGet => encode_capabilities_response(
                            self.codec,
                            self.revision,
                            header.request_id,
                            self.replay_entries.len() as u64,
                            self.replay_payload.len() as u64,
                            self.quantum_frames,
                            &mut self.response_scratch,
                        )
                        .map_err(|_| CommandError::Internal)?,
                        _ => self.encode_status(header, StatusCode::UnsupportedMessage, None)?,
                    },
                    Err(error) => self.encode_status(header, error.status(), None)?,
                };
                self.complete_replay(header.request_id, request, bytes)?;
                bytes
            }
        };
        if output_capacity < response_bytes as u64 {
            return Err(CommandError::BufferTooSmall {
                required: response_bytes as u64,
            });
        }
        Ok(response_bytes)
    }

    pub(crate) fn command_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    fn source_id(&self, source: &ControlSource) -> &[u8] {
        &self.source_ids[source.id_offset..source.id_offset + source.id_bytes]
    }

    fn source_index(&self, id: &[u8]) -> Option<usize> {
        self.sources
            .binary_search_by(|source| self.source_id(source).cmp(id))
            .ok()
    }

    pub(crate) fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<miso_engine_source::SubmitReport, u32> {
        let index = self.source_index(id).ok_or(RESULT_INVALID_ARGUMENT)?;
        let source = &mut self.sources[index];
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
        let index = self.source_index(id).ok_or(RESULT_INVALID_ARGUMENT)?;
        let source = &mut self.sources[index];
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
        let codec = ProtocolCodec::default();
        let frame = miso_engine_protocol::TypedCommandFrame {
            request_id: RequestId::new(request_id).expect("nonzero request"),
            expected_revision: ExpectedRevision::Any,
            payload,
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let len = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("typed command");
        bytes.truncate(len);
        bytes
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
        assert_eq!(children.plan.resources.sample_rate_hz, 48_000);
        assert_eq!(children.plan.resources.quantum_frames, 128);
        assert_eq!(children.plan.resources.source_count, 1);
        assert_eq!(children.plan.resources.track_count, 9);
        assert!(children.plan.resources.graph_session_plus_plan_bytes > 0);
        assert!(children.plan.resources.source_total_bytes > 0);
        assert!(children.plan.resources.effect_scalar_state_bytes > 0);
        assert!(children.plan.resources.builtin_retained_payload_bytes > 0);
        assert!(children.plan.resources.capi_retained_bytes > 0);
        assert!(children.plan.resources.largest_named_allocation_bytes > 0);

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
        let report = baseline.plan.resources;

        for below in [
            ("graph", report.graph_session_plus_plan_bytes),
            ("source-total", report.source_total_bytes),
            ("source-overhead", report.source_overhead_bytes),
            ("effect-state", report.effect_scalar_state_bytes),
            ("builtin", report.builtin_retained_payload_bytes),
            ("capi", report.capi_retained_bytes),
            ("largest", report.largest_named_allocation_bytes),
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
            .resources;
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
    fn capability_command_is_canonical_replayed_and_typed_unsupported() {
        let mut children = compile_children(SESSION, limits()).expect("children");
        let capability = command_bytes(1, miso_engine_protocol::CommandPayload::CapabilitiesGet);
        assert_eq!(
            children.session.command(&capability, 0),
            Err(CommandError::BufferTooSmall { required: 4_096 })
        );
        assert_eq!(children.session.replay_len, 0);
        assert_eq!(children.session.highest_new_request_id, None);

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
                assert_eq!(value.supported_commands, [1, 0]);
                assert!(value.supported_events.is_empty());
                assert_eq!(value.flags, CapabilityFlags(0b111));
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
        assert_eq!(children.session.replay_len, 1);

        let unsupported = command_bytes(
            2,
            miso_engine_protocol::CommandPayload::SessionSnapshotGet(
                miso_engine_protocol::SessionSnapshotRequest {
                    offset: 0,
                    maximum_bytes: 1,
                },
            ),
        );
        let unsupported_len = children
            .session
            .command(&unsupported, 4_096)
            .expect("typed unsupported response");
        let unsupported = children.session.command_response(unsupported_len);
        let mut fields = [0_u16; 8];
        assert!(matches!(
            ProtocolCodec::default()
                .decode_typed_response(unsupported, &mut DecodeScratch::new(&mut fields))
                .expect("typed unsupported"),
            miso_engine_protocol::DecodedTypedResponseFrame::NonOk { header, .. }
                if header.status == StatusCode::UnsupportedMessage
                    && header.request_id.get() == 2
        ));
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
