//! Safe control-plane orchestration behind the raw FFI boundary.

use core::alloc::Layout;

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::{SampleRateHz, realtime::PreparedRenderPlan};
use miso_engine_effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry_v1, prepare_native_session_effects,
};
use miso_engine_effect_contract::TailSamples;
use miso_engine_graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, StableGraphId, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompileReport, GraphCompiler};
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
    _request_offset: u64,
    _request_bytes: u64,
    _response_offset: u64,
    _response_bytes: u64,
    _request_id: u64,
    _occupied: u64,
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
    _request_scratch: Box<[u8]>,
    _response_scratch: Box<[u8]>,
    _replay_payload: Box<[u8]>,
    _replay_entries: Box<[ReplayEntryRecord]>,
}

pub(crate) struct PlanState {
    pub(crate) _plan: PreparedRenderPlan,
    pub(crate) resources: PlanResourceReport,
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

    Ok(CompiledChildren {
        session: SessionState {
            _canonical_session: canonical_session.into_boxed_slice(),
            source_ids: ids.into_boxed_slice(),
            sources: controls.into_boxed_slice(),
            _request_scratch: boxed_zeroed(limits.maximum_control_frame_bytes)?,
            _response_scratch: boxed_zeroed(limits.maximum_control_frame_bytes)?,
            _replay_payload: boxed_zeroed(limits.maximum_replay_bytes)?,
            _replay_entries: replay_entries.into_boxed_slice(),
        },
        session_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
        plan: PlanState {
            _plan: bound.plan,
            resources,
        },
        plan_error: FixedBytes::try_new(limits.maximum_diagnostic_bytes)?,
    })
}

impl SessionState {
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
}
