//! Native resolver, preparation, and one-file-per-worker source delivery.
//!
//! This module is control/worker-only and cfg-excluded from browser Wasm. The started worker owns
//! the opened reader, decoder, prepared source producer, and all decoded staging storage; it never
//! shares those mutable objects with a render worker.

use core::{alloc::Layout, cell::Cell, marker::PhantomData, num::NonZeroUsize};
use std::{
    io::{Read, Seek},
    thread::{self, JoinHandle},
};

use miso_engine_core::{
    SampleRateHz,
    realtime::{
        Consumer, Producer, QueueEmpty, QueueFull, QueueGeneration, bounded_spsc,
        bounded_spsc_retained_payload,
    },
};
use miso_engine_graph::{GraphNodeId, StableGraphId, TrackStage};
use miso_engine_session::CompiledSession;

use crate::{
    HostChunkError, HostChunkProvider, NativeWaveDecoder, NativeWaveError, NativeWaveMetadata,
    NativeWaveParseCaps, NativeWaveRegion, PcmSourceConsumer, PcmSourceRing, PcmSourceRingConfig,
    PcmSourceRingError, SourceCommand, SourceDiagnostic, SourceDiagnosticCode,
    SourceDiagnosticPath, SourceFrame, SourceGeneration, SourceGraphSource,
    SourceGraphTrackMapping, SourceResourceReport, SourceSeekError, parse_native_wave,
    prepare_graph_source_set,
};

/// Opaque native resolver failure before a source asset can be parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceResolverError {
    /// The opaque locator could not be resolved to an opened native asset.
    Unresolved,
}

/// An opened native seekable asset and its resolver-observed opaque identity.
pub struct NativeResolvedAsset<R: Read + Seek + Send + 'static> {
    /// Identity bytes compared exactly with the declaration; their encoding remains opaque.
    pub observed_identity: Vec<u8>,
    /// Opened reader moved into the sole worker after preparation succeeds.
    pub reader: R,
}

/// Opaque native source resolver.
pub trait NativeSourceResolver {
    /// Opened asset type for one source worker.
    type Asset: Read + Seek + Send + 'static;

    /// Open `opaque_locator` without interpreting the locator or identity formats.
    fn resolve(
        &mut self,
        opaque_locator: &str,
    ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError>;
}

/// One fully declared native source preparation input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeSourcePrepareRequest {
    /// Opaque locator delivered unchanged to the resolver.
    pub locator: String,
    /// Opaque declared content identity compared byte-for-byte after resolution.
    pub declared_identity: Vec<u8>,
    /// Declared source sample rate; implicit conversion is forbidden.
    pub declared_sample_rate_hz: SampleRateHz,
    /// Render plan sample rate, which must exactly equal the source rate.
    pub engine_sample_rate_hz: SampleRateHz,
    /// Declared source channel count.
    pub declared_channel_count: u16,
    /// Exact finite decoded source region.
    pub region: NativeWaveRegion,
    /// Exact prepared ring storage shape.
    pub ring_config: PcmSourceRingConfig,
}

/// Fixed native source preparation caps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSourcePrepareCaps {
    /// Parser traversal and skipped-metadata limits.
    pub parser: NativeWaveParseCaps,
    /// Maximum fixed native decoder interleaved read scratch bytes.
    pub max_worker_read_scratch_bytes: u64,
    /// Maximum exact ring plus fixed native worker allocation total.
    pub max_total_engine_owned_bytes: u64,
    /// Maximum exact source-ring, decoder, or staging allocation.
    pub max_largest_allocation_bytes: u64,
    /// Bounded worker command queue item count.
    pub control_queue_items: NonZeroUsize,
}

/// Exact source preparation accounting, independent of asset duration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSourceResourceReport {
    /// Exact prepared source-ring accounting, including the session-charged PCM payload.
    pub ring: SourceResourceReport,
    /// One native decoder interleaved fixed read scratch allocation.
    pub decoder_read_scratch_bytes: u64,
    /// One worker-owned planar `[channel][quantum]` staging allocation.
    pub worker_planar_staging_bytes: u64,
    /// Exact bounded worker command item count.
    pub worker_control_queue_items: u64,
    /// Exact SPSC worker-command queue header and slot payload bytes.
    pub worker_control_queue_bytes: u64,
    /// Largest worker-command queue allocation request.
    pub worker_control_queue_largest_allocation_bytes: u64,
    /// Maximum required alignment among worker-command queue allocations.
    pub worker_control_queue_alignment_bytes: u64,
    /// Exact SPSC worker-event queue header and slot payload bytes.
    pub worker_event_queue_bytes: u64,
    /// Exact worker event queue item capacity: ready plus one snapshot-or-terminal slot.
    pub worker_event_queue_items: u64,
    /// Largest worker-event queue allocation request.
    pub worker_event_queue_largest_allocation_bytes: u64,
    /// Maximum required alignment among worker-event queue allocations.
    pub worker_event_queue_alignment_bytes: u64,
    /// Exact capacity-one SPSC worker-stop queue header and slot payload bytes.
    pub worker_stop_queue_bytes: u64,
    /// Exact worker stop queue item capacity.
    pub worker_stop_queue_items: u64,
    /// Largest worker-stop queue allocation request.
    pub worker_stop_queue_largest_allocation_bytes: u64,
    /// Maximum required alignment among worker-stop queue allocations.
    pub worker_stop_queue_alignment_bytes: u64,
    /// Exact source ring plus fixed decoder/staging allocation total.
    pub total_engine_owned_bytes: u64,
    /// Largest exact allocation among ring, decoder, staging, and native worker queues.
    pub largest_allocation_bytes: u64,
}

/// Fixed caps for preparing every declared native session source as one transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSessionSourcePrepareCaps {
    /// Per-source parser, ring, worker, and decoder caps.
    pub source: NativeSourcePrepareCaps,
    /// Maximum checked session-runtime plus source-overhead bytes after deduplicating ring PCM.
    pub max_combined_runtime_bytes: u64,
    /// Maximum checked allocation request across session, source, and graph source-set storage.
    pub max_largest_allocation_bytes: u64,
}

/// Exact one-time accounting for a prepared session source transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSessionSourceResourceReport {
    pub source_count: u64,
    pub session_runtime_bytes: u64,
    pub source_pcm_already_charged_bytes: u64,
    pub source_overhead_bytes: u64,
    /// Exact vector allocation retaining one non-render controller per native source.
    pub controller_records_bytes: u64,
    pub combined_runtime_bytes: u64,
    pub largest_allocation_bytes: u64,
}

/// One opaque, started native source before it is transactionally moved into a graph source set.
///
/// The native join owner is deliberately not exposed. Convert this object with
/// [`Self::into_graph_source`] only on a control/retirement path; that conversion moves the
/// consumer and join owner together into the graph-owned source entry.
pub struct PreparedNativeSource {
    controller: NativeSourceController,
    consumer: PcmSourceConsumer,
    resources: NativeSourceResourceReport,
    worker: NativeSourceWorker,
}

impl PreparedNativeSource {
    /// Borrow the sole non-render controller endpoint before graph publication.
    #[must_use]
    pub fn controller(&mut self) -> &mut NativeSourceController {
        &mut self.controller
    }

    /// Exact fixed source resource accounting prepared for this source.
    #[must_use]
    pub const fn resource_report(&self) -> NativeSourceResourceReport {
        self.resources
    }

    /// Move the controller and native source into their separate prepared ownership domains.
    #[must_use]
    pub fn into_graph_source(self) -> (NativeSourceController, SourceGraphSource) {
        let Self {
            controller,
            consumer,
            resources,
            worker,
        } = self;
        let additional_overhead_bytes = resources
            .total_engine_owned_bytes
            .checked_sub(resources.ring.total_engine_owned_bytes)
            .expect("native report includes ring");
        (
            controller,
            SourceGraphSource::with_native_worker(
                consumer,
                resources.ring,
                additional_overhead_bytes,
                resources.largest_allocation_bytes,
                worker,
            ),
        )
    }
}

/// A failed all-or-nothing session source preparation with sorted stable diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeSessionSourcePrepareFailure {
    pub diagnostics: Vec<SourceDiagnostic>,
}

/// Prepared graph source set plus separate non-render native controller endpoints.
pub struct NativeSessionPreparedSources {
    pub source_set: miso_engine_graph::GraphPreparedSourceSet,
    pub controllers: Vec<NativeSourceController>,
    pub resources: NativeSessionSourceResourceReport,
}

impl NativeSessionPreparedSources {
    /// Move the source set and its independent controllers without exposing worker join owners.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        miso_engine_graph::GraphPreparedSourceSet,
        Vec<NativeSourceController>,
        NativeSessionSourceResourceReport,
    ) {
        (self.source_set, self.controllers, self.resources)
    }
}

/// Resolution or source preparation rejection before any consumer is published.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourcePrepareError {
    /// Resolver could not open an opaque asset.
    Resolver(NativeSourceResolverError),
    /// Observed and declared opaque identities differ.
    ContentIdentityMismatch,
    /// Declared, observed, or engine sample rates differ.
    RateMismatch,
    /// Declared, observed, or ring channel counts differ.
    ChannelMismatch,
    /// Declared region is not within parsed source frames.
    RegionOutOfBounds,
    /// Prepared ring setup rejected its fixed shape.
    Ring(PcmSourceRingError),
    /// Native WAVE parsing/decoder preparation rejected the asset.
    Wave(NativeWaveError),
    /// One explicit worker resource cap rejected preparation.
    ResourceLimit,
    /// The off-render native worker could not be started.
    WorkerStart,
}

impl NativeSourcePrepareError {
    /// Stable source diagnostic code.
    #[must_use]
    pub const fn diagnostic_code(self) -> SourceDiagnosticCode {
        match self {
            Self::Resolver(_) => SourceDiagnosticCode::AssetUnresolved,
            Self::ContentIdentityMismatch => SourceDiagnosticCode::ContentIdentityMismatch,
            Self::RateMismatch => SourceDiagnosticCode::RateMismatch,
            Self::ChannelMismatch => SourceDiagnosticCode::ChannelsMismatch,
            Self::RegionOutOfBounds => SourceDiagnosticCode::RegionOutOfBounds,
            Self::Ring(error) => error.diagnostic_code(),
            Self::Wave(error) => error.diagnostic_code(),
            Self::ResourceLimit | Self::WorkerStart => SourceDiagnosticCode::ResourceLimit,
        }
    }
}

/// Worker lifecycle event delivered only through a bounded non-render channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerEvent {
    /// First PCM chunk or zero-frame end marker has been submitted to the prepared ring.
    SourceReady {
        native_decoder_sanitized_samples: u64,
    },
    /// Response to one explicit controller snapshot request.
    SanitationSnapshot {
        native_decoder_sanitized_samples: u64,
    },
    /// Final worker-local sanitation watermark, published before the worker exits.
    Terminal {
        native_decoder_sanitized_samples: u64,
    },
}

/// Worker terminal state returned after an off-render join.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerExit {
    /// Explicit stop or controller disconnect stopped the worker.
    Stopped,
    /// File/decoder work failed outside render.
    DecodeFailed(NativeWaveError),
    /// The native path violated shared host/ring submission semantics.
    SubmitFailed(HostChunkError),
    /// A queued source seek could not be delivered through the bounded source command state.
    SeekFailed(SourceSeekError),
}

/// Typed non-render worker command failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerControlError {
    /// Worker has already stopped or its command receiver disconnected.
    Stopped,
    /// A seek generation is zero.
    GenerationZero,
    /// A seek generation does not strictly increase from the last accepted command.
    GenerationNotStrictlyIncreasing {
        /// Last accepted generation.
        active: SourceGeneration,
        /// Rejected generation.
        requested: SourceGeneration,
    },
    /// A seek frame lies outside the prepared finite region.
    RegionOutOfBounds,
    /// Bounded worker command storage is full; the command was not accepted.
    Backpressure,
    /// Worker thread panicked; no render-thread recovery is attempted.
    WorkerPanicked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkerCommand {
    Seek {
        generation: SourceGeneration,
        frame: SourceFrame,
    },
    Wake,
    SnapshotSanitation,
}

#[derive(Clone, Copy)]
struct PendingBlock {
    generation: SourceGeneration,
    start_frame: SourceFrame,
    frames: u32,
    end_of_region: bool,
    native_decoder_sanitized_samples: u64,
}

/// Non-render endpoint for bounded native seek/wake commands and worker events.
pub struct NativeSourceController {
    commands: Producer<WorkerCommand>,
    events: Consumer<NativeSourceWorkerEvent>,
    observed_sanitation: u64,
    terminal_observed: bool,
    next_requested_generation: SourceGeneration,
    region: NativeWaveRegion,
    _not_sync: PhantomData<Cell<()>>,
}

impl NativeSourceController {
    /// Cumulative native decoder replacements observed through bounded prepared telemetry.
    #[must_use]
    pub fn native_decoder_sanitized_samples(&self) -> u64 {
        self.observed_sanitation
    }

    /// Request and wait for a synchronized native decoder sanitation watermark outside render.
    pub fn snapshot_native_decoder_sanitized_samples(
        &mut self,
    ) -> Result<u64, NativeSourceWorkerControlError> {
        self.try_send(WorkerCommand::SnapshotSanitation)?;
        loop {
            match self.wait_for_event()? {
                NativeSourceWorkerEvent::SanitationSnapshot { .. }
                | NativeSourceWorkerEvent::Terminal { .. } => {
                    return Ok(self.observed_sanitation);
                }
                NativeSourceWorkerEvent::SourceReady { .. } => {}
            }
        }
    }

    /// Queue a strictly increasing region-bounded source seek without blocking.
    pub fn try_seek(
        &mut self,
        command: SourceCommand,
    ) -> Result<(), NativeSourceWorkerControlError> {
        let SourceCommand::Seek { generation, frame } = command;
        if !generation.is_valid() {
            return Err(NativeSourceWorkerControlError::GenerationZero);
        }
        if generation <= self.next_requested_generation {
            return Err(
                NativeSourceWorkerControlError::GenerationNotStrictlyIncreasing {
                    active: self.next_requested_generation,
                    requested: generation,
                },
            );
        }
        validate_seek_frame(self.region, frame)?;
        self.try_send(WorkerCommand::Seek { generation, frame })?;
        self.next_requested_generation = generation;
        Ok(())
    }

    /// Wake a worker waiting for ring capacity after an off-render consumer-drain notification.
    pub fn try_wake(&mut self) -> Result<(), NativeSourceWorkerControlError> {
        self.try_send(WorkerCommand::Wake)
    }

    /// Wait outside render for the initial prepared source data event.
    pub fn wait_for_event(
        &mut self,
    ) -> Result<NativeSourceWorkerEvent, NativeSourceWorkerControlError> {
        loop {
            match self.events.try_pop() {
                Ok(event) => {
                    self.observe_event(event);
                    return Ok(event);
                }
                Err(QueueEmpty { .. }) if self.terminal_observed => {
                    return Err(NativeSourceWorkerControlError::Stopped);
                }
                Err(QueueEmpty { .. }) => thread::yield_now(),
            }
        }
    }

    fn try_send(&mut self, command: WorkerCommand) -> Result<(), NativeSourceWorkerControlError> {
        if self.terminal_observed {
            return Err(NativeSourceWorkerControlError::Stopped);
        }
        match self.commands.try_push(command) {
            Ok(()) => Ok(()),
            Err(QueueFull { .. }) => Err(NativeSourceWorkerControlError::Backpressure),
        }
    }

    fn observe_event(&mut self, event: NativeSourceWorkerEvent) {
        let value = match event {
            NativeSourceWorkerEvent::SourceReady {
                native_decoder_sanitized_samples,
            }
            | NativeSourceWorkerEvent::SanitationSnapshot {
                native_decoder_sanitized_samples,
            }
            | NativeSourceWorkerEvent::Terminal {
                native_decoder_sanitized_samples,
            } => native_decoder_sanitized_samples,
        };
        self.observed_sanitation = self.observed_sanitation.max(value);
        if matches!(event, NativeSourceWorkerEvent::Terminal { .. }) {
            self.terminal_observed = true;
        }
    }
}

/// Sole stop/join owner for one started native worker.
///
/// This token is intentionally moved only into the source-set driver. Its `Drop` implementation
/// runs on source-set/retired-plan reclamation, never from render.
pub(crate) struct NativeSourceWorker {
    join: Option<JoinHandle<NativeSourceWorkerExit>>,
    stopped: bool,
    stop: Producer<()>,
    _not_sync: PhantomData<Cell<()>>,
}

impl NativeSourceWorker {
    /// Stop and join the worker outside render.
    pub(crate) fn stop_and_join(
        &mut self,
    ) -> Result<NativeSourceWorkerExit, NativeSourceWorkerControlError> {
        if !self.stopped {
            self.stop
                .try_push(())
                .map_err(|_| NativeSourceWorkerControlError::Stopped)?;
            self.stopped = true;
        }
        let Some(join) = self.join.take() else {
            return Err(NativeSourceWorkerControlError::Stopped);
        };
        join.join()
            .map_err(|_| NativeSourceWorkerControlError::WorkerPanicked)
    }
}

impl Drop for NativeSourceWorker {
    fn drop(&mut self) {
        let _ = self.stop_and_join();
    }
}

/// Resolve, validate, prepare, and start one native source worker transactionally.
/// ```compile_fail
/// use miso_engine_source::NativeSourceWorker;
/// ```
///
/// The worker join token is private: callers can only move it into a graph source entry through
/// [`PreparedNativeSource::into_graph_source`].
pub fn prepare_native_source<S: NativeSourceResolver>(
    resolver: &mut S,
    request: NativeSourcePrepareRequest,
    caps: NativeSourcePrepareCaps,
) -> Result<PreparedNativeSource, NativeSourcePrepareError> {
    let (controller, worker, consumer, resources) =
        prepare_native_source_parts(resolver, request, caps)?;
    Ok(PreparedNativeSource {
        controller,
        consumer,
        resources,
        worker,
    })
}

fn prepare_native_source_parts<S: NativeSourceResolver>(
    resolver: &mut S,
    request: NativeSourcePrepareRequest,
    caps: NativeSourcePrepareCaps,
) -> Result<
    (
        NativeSourceController,
        NativeSourceWorker,
        PcmSourceConsumer,
        NativeSourceResourceReport,
    ),
    NativeSourcePrepareError,
> {
    let mut asset = resolver
        .resolve(&request.locator)
        .map_err(NativeSourcePrepareError::Resolver)?;
    if asset.observed_identity != request.declared_identity {
        return Err(NativeSourcePrepareError::ContentIdentityMismatch);
    }
    let metadata = parse_native_wave(&mut asset.reader, caps.parser)
        .map_err(NativeSourcePrepareError::Wave)?;
    if metadata.sample_rate_hz != request.declared_sample_rate_hz
        || metadata.sample_rate_hz != request.engine_sample_rate_hz
    {
        return Err(NativeSourcePrepareError::RateMismatch);
    }
    if metadata.channel_count != request.declared_channel_count
        || request.ring_config.channel_count != u32::from(request.declared_channel_count)
    {
        return Err(NativeSourcePrepareError::ChannelMismatch);
    }
    validate_region(metadata, request.region)?;
    let report = source_resource_report(metadata, request.ring_config, caps)?;
    let quantum = usize::try_from(request.ring_config.quantum_frames.0)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let quantum = NonZeroUsize::new(quantum).ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let decoder = NativeWaveDecoder::prepare(asset.reader, metadata, request.region, quantum)
        .map_err(NativeSourcePrepareError::Wave)?;
    let staging_samples = usize::from(metadata.channel_count)
        .checked_mul(quantum.get())
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let mut planar_staging = Vec::new();
    planar_staging
        .try_reserve_exact(staging_samples)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    planar_staging.resize(staging_samples, 0.0);
    let (producer, consumer, ring_report) =
        PcmSourceRing::prepare_at_source_frame(request.ring_config, request.region.start_frame)
            .map_err(NativeSourcePrepareError::Ring)?;
    debug_assert_eq!(ring_report, report.ring);
    let provider = producer.into_host_chunk_provider(metadata.sample_rate_hz);
    let (command_sender, command_receiver) =
        bounded_spsc(caps.control_queue_items, QueueGeneration(11))
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let (event_sender, event_receiver) = bounded_spsc(
        NonZeroUsize::new(2).expect("two events"),
        QueueGeneration(12),
    )
    .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let initial_generation = request.ring_config.initial_generation;
    let (stop_sender, stop_receiver) =
        bounded_spsc(NonZeroUsize::new(1).expect("one stop"), QueueGeneration(13))
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let join = thread::Builder::new()
        .name("miso-engine-source".to_owned())
        .spawn(move || {
            run_worker(
                command_receiver,
                event_sender,
                provider,
                decoder,
                planar_staging.into_boxed_slice(),
                initial_generation,
                stop_receiver,
            )
        })
        .map_err(|_| NativeSourcePrepareError::WorkerStart)?;
    Ok((
        NativeSourceController {
            commands: command_sender,
            events: event_receiver,
            observed_sanitation: 0,
            terminal_observed: false,
            next_requested_generation: initial_generation,
            region: request.region,
            _not_sync: PhantomData,
        },
        NativeSourceWorker {
            join: Some(join),
            stopped: false,
            stop: stop_sender,
            _not_sync: PhantomData,
        },
        consumer,
        report,
    ))
}

/// Resolve every normalized session source once and publish no source set unless all preparation
/// and combined resource checks succeed. Started workers are stopped/joined on every failure.
pub fn prepare_native_session_sources<S: NativeSourceResolver>(
    session: &CompiledSession,
    resolver: &mut S,
    caps: NativeSessionSourcePrepareCaps,
) -> Result<NativeSessionPreparedSources, NativeSessionSourcePrepareFailure> {
    let model = session.normalized_model();
    let mut diagnostics = Vec::new();
    let mut controllers = Vec::with_capacity(model.sources.len());
    let mut graph_sources = Vec::with_capacity(model.sources.len());
    let mut reports = Vec::with_capacity(model.sources.len());

    for source in &model.sources {
        let request = NativeSourcePrepareRequest {
            locator: source.content.locator.clone(),
            declared_identity: source.content.identity.as_bytes().to_vec(),
            declared_sample_rate_hz: SampleRateHz(source.sample_rate_hz),
            engine_sample_rate_hz: session.sample_rate(),
            declared_channel_count: u16::from(source.mapping.channel_count),
            region: NativeWaveRegion {
                start_frame: SourceFrame(source.mapping.region.start_sample),
                length_frames: source.mapping.region.length_samples,
            },
            ring_config: PcmSourceRingConfig {
                channel_count: u32::from(source.mapping.channel_count),
                quantum_frames: session.quantum(),
                frame_capacity: model.limits.pcm_ring_frames,
                initial_generation: SourceGeneration(1),
            },
        };
        match prepare_native_source(resolver, request, caps.source) {
            Ok(prepared) => {
                let report = prepared.resource_report();
                let (controller, graph_source) = prepared.into_graph_source();
                graph_sources.push(graph_source);
                reports.push(report);
                controllers.push(controller);
            }
            Err(error) => diagnostics.push(native_prepare_diagnostic(source.id.as_str(), error)),
        }
    }
    if !diagnostics.is_empty() {
        sort_diagnostics(&mut diagnostics);
        return Err(NativeSessionSourcePrepareFailure { diagnostics });
    }

    let source_indexes: std::collections::BTreeMap<_, _> = model
        .sources
        .iter()
        .enumerate()
        .map(|(index, source)| (source.id.clone(), index))
        .collect();
    let mappings: Vec<_> = model
        .tracks
        .iter()
        .map(|track| SourceGraphTrackMapping {
            node: GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track.id.as_str()).expect("compiled stable ID"),
                stage: TrackStage::Input,
            },
            source_index: source_indexes[&track.source_id],
            left_channel: u32::from(track.left_source_channel),
            right_channel: u32::from(track.right_source_channel),
        })
        .collect();
    let source_set = match prepare_graph_source_set(
        miso_engine_core::realtime::RenderEnvelope {
            sample_rate: session.sample_rate(),
            quantum: session.quantum(),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("dual mono output"),
        },
        graph_sources,
        mappings,
    ) {
        Ok(source_set) => source_set,
        Err(_) => {
            return Err(NativeSessionSourcePrepareFailure {
                diagnostics: vec![SourceDiagnostic::new(
                    SourceDiagnosticCode::GraphBindingMismatch,
                    SourceDiagnosticPath::for_source(model.sources[0].id.as_str()),
                    "compiled track/source mappings could not be sealed for graph binding",
                )],
            });
        }
    };
    let graph_resources = source_set.resource_report();
    let controller_records_bytes =
        retained_array_bytes::<NativeSourceController>(controllers.len())
            .ok_or_else(|| resource_failure(model.sources[0].id.as_str()))?;
    let session_runtime_bytes = session.resource_estimate().requested_runtime_bytes;
    let combined_runtime_bytes = match session_runtime_bytes
        .checked_add(graph_resources.overhead_bytes)
        .and_then(|total| total.checked_add(controller_records_bytes))
    {
        Some(total) => total,
        None => {
            return Err(resource_failure(model.sources[0].id.as_str()));
        }
    };
    let largest_allocation_bytes = session
        .resource_estimate()
        .single_allocation_bytes
        .max(graph_resources.largest_allocation_bytes)
        .max(controller_records_bytes);
    if graph_resources.pcm_payload_already_charged_bytes
        != session.resource_estimate().source_ring_bytes
        || combined_runtime_bytes > model.limits.memory_bytes
        || combined_runtime_bytes > caps.max_combined_runtime_bytes
        || largest_allocation_bytes > caps.max_largest_allocation_bytes
    {
        return Err(resource_failure(model.sources[0].id.as_str()));
    }
    let source_count = u64::try_from(reports.len()).expect("source vector length fits u64");
    Ok(NativeSessionPreparedSources {
        source_set,
        controllers,
        resources: NativeSessionSourceResourceReport {
            source_count,
            session_runtime_bytes,
            source_pcm_already_charged_bytes: graph_resources.pcm_payload_already_charged_bytes,
            source_overhead_bytes: graph_resources.overhead_bytes,
            controller_records_bytes,
            combined_runtime_bytes,
            largest_allocation_bytes,
        },
    })
}

fn native_prepare_diagnostic(source_id: &str, error: NativeSourcePrepareError) -> SourceDiagnostic {
    SourceDiagnostic::new(
        error.diagnostic_code(),
        SourceDiagnosticPath::for_source(source_id),
        "native source resolution or preparation rejected the declared source",
    )
}

fn resource_failure(source_id: &str) -> NativeSessionSourcePrepareFailure {
    NativeSessionSourcePrepareFailure {
        diagnostics: vec![SourceDiagnostic::new(
            SourceDiagnosticCode::ResourceLimit,
            SourceDiagnosticPath::for_source(source_id),
            "session plus graph source overhead exceeds a declared preparation resource limit",
        )],
    }
}

fn sort_diagnostics(diagnostics: &mut [SourceDiagnostic]) {
    diagnostics.sort_by(|left, right| left.path.cmp(&right.path).then(left.code.cmp(&right.code)));
}

fn source_resource_report(
    metadata: NativeWaveMetadata,
    ring_config: PcmSourceRingConfig,
    caps: NativeSourcePrepareCaps,
) -> Result<NativeSourceResourceReport, NativeSourcePrepareError> {
    let ring =
        PcmSourceRing::resource_report(ring_config).map_err(NativeSourcePrepareError::Ring)?;
    let decoder_read_scratch_bytes = u64::from(ring_config.quantum_frames.0)
        .checked_mul(u64::from(metadata.block_align_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let worker_planar_staging_bytes = u64::from(ring_config.quantum_frames.0)
        .checked_mul(u64::from(metadata.channel_count))
        .and_then(|samples| {
            samples.checked_mul(u64::try_from(core::mem::size_of::<f32>()).expect("f32 size"))
        })
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    if decoder_read_scratch_bytes > caps.max_worker_read_scratch_bytes {
        return Err(NativeSourcePrepareError::ResourceLimit);
    }
    let worker_control_queue = exact_queue_resources::<WorkerCommand>(caps.control_queue_items)?;
    let worker_event_queue = exact_queue_resources::<NativeSourceWorkerEvent>(
        NonZeroUsize::new(2).expect("two events"),
    )?;
    let worker_stop_queue = exact_queue_resources::<()>(NonZeroUsize::new(1).expect("one stop"))?;
    let total_engine_owned_bytes = ring
        .total_engine_owned_bytes
        .checked_add(decoder_read_scratch_bytes)
        .and_then(|total| total.checked_add(worker_planar_staging_bytes))
        .and_then(|total| total.checked_add(worker_control_queue.total_bytes))
        .and_then(|total| total.checked_add(worker_event_queue.total_bytes))
        .and_then(|total| total.checked_add(worker_stop_queue.total_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let largest_allocation_bytes = ring
        .largest_allocation_bytes
        .max(decoder_read_scratch_bytes)
        .max(worker_planar_staging_bytes)
        .max(worker_control_queue.largest_allocation_bytes)
        .max(worker_event_queue.largest_allocation_bytes)
        .max(worker_stop_queue.largest_allocation_bytes);
    if total_engine_owned_bytes > caps.max_total_engine_owned_bytes
        || largest_allocation_bytes > caps.max_largest_allocation_bytes
    {
        return Err(NativeSourcePrepareError::ResourceLimit);
    }
    Ok(NativeSourceResourceReport {
        ring,
        decoder_read_scratch_bytes,
        worker_planar_staging_bytes,
        worker_control_queue_items: u64::try_from(caps.control_queue_items.get())
            .expect("usize fits u64"),
        worker_control_queue_bytes: worker_control_queue.total_bytes,
        worker_control_queue_largest_allocation_bytes: worker_control_queue
            .largest_allocation_bytes,
        worker_control_queue_alignment_bytes: worker_control_queue.alignment_bytes,
        worker_event_queue_bytes: worker_event_queue.total_bytes,
        worker_event_queue_items: 2,
        worker_event_queue_largest_allocation_bytes: worker_event_queue.largest_allocation_bytes,
        worker_event_queue_alignment_bytes: worker_event_queue.alignment_bytes,
        worker_stop_queue_bytes: worker_stop_queue.total_bytes,
        worker_stop_queue_items: 1,
        worker_stop_queue_largest_allocation_bytes: worker_stop_queue.largest_allocation_bytes,
        worker_stop_queue_alignment_bytes: worker_stop_queue.alignment_bytes,
        total_engine_owned_bytes,
        largest_allocation_bytes,
    })
}

fn retained_array_bytes<T>(count: usize) -> Option<u64> {
    let layout = Layout::array::<T>(count).ok()?;
    u64::try_from(layout.size()).ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExactQueueResources {
    total_bytes: u64,
    largest_allocation_bytes: u64,
    alignment_bytes: u64,
}

fn exact_queue_resources<T: Send + 'static>(
    capacity: NonZeroUsize,
) -> Result<ExactQueueResources, NativeSourcePrepareError> {
    let payload = bounded_spsc_retained_payload::<T>(capacity)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let total_bytes = u64::try_from(
        payload
            .total_bytes()
            .ok_or(NativeSourcePrepareError::ResourceLimit)?,
    )
    .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    Ok(ExactQueueResources {
        total_bytes,
        largest_allocation_bytes: u64::try_from(payload.largest_allocation_bytes())
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
        alignment_bytes: u64::try_from(payload.ring_header_align.max(payload.slot_payload_align))
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
    })
}

fn validate_region(
    metadata: NativeWaveMetadata,
    region: NativeWaveRegion,
) -> Result<(), NativeSourcePrepareError> {
    let end = region
        .start_frame
        .0
        .checked_add(region.length_frames)
        .ok_or(NativeSourcePrepareError::RegionOutOfBounds)?;
    if end > metadata.total_frames {
        return Err(NativeSourcePrepareError::RegionOutOfBounds);
    }
    Ok(())
}

fn validate_seek_frame(
    region: NativeWaveRegion,
    frame: SourceFrame,
) -> Result<(), NativeSourceWorkerControlError> {
    let end = region
        .start_frame
        .0
        .checked_add(region.length_frames)
        .ok_or(NativeSourceWorkerControlError::RegionOutOfBounds)?;
    if frame.0 < region.start_frame.0 || frame.0 > end {
        return Err(NativeSourceWorkerControlError::RegionOutOfBounds);
    }
    Ok(())
}

fn run_worker<R: Read + Seek>(
    mut commands: Consumer<WorkerCommand>,
    mut events: Producer<NativeSourceWorkerEvent>,
    mut provider: HostChunkProvider,
    mut decoder: NativeWaveDecoder<R>,
    mut planar_staging: Box<[f32]>,
    mut generation: SourceGeneration,
    mut stop: Consumer<()>,
) -> NativeSourceWorkerExit {
    let mut pending: Option<PendingBlock> = None;
    let mut end_submitted = false;
    let mut source_ready_sent = false;
    let mut sanitation = 0_u64;
    let exit = loop {
        if stop.try_pop().is_ok() {
            break NativeSourceWorkerExit::Stopped;
        }
        if let Ok(command) = commands.try_pop() {
            match apply_command(command, &mut provider, &mut decoder, &mut generation) {
                Ok(CommandResult::Seek) => {
                    pending = None;
                    end_submitted = false;
                    continue;
                }
                Ok(CommandResult::Wake) => {}
                Ok(CommandResult::Snapshot) => {
                    publish_worker_event(
                        &mut events,
                        NativeSourceWorkerEvent::SanitationSnapshot {
                            native_decoder_sanitized_samples: sanitation,
                        },
                    );
                }
                Err(exit) => break exit,
            }
        }
        if let Some(block) = pending.take() {
            match provider.submit_native_planar(
                block.generation,
                block.start_frame,
                &planar_staging,
                block.frames,
                block.end_of_region,
                block.native_decoder_sanitized_samples,
            ) {
                Ok(_) => {
                    end_submitted = block.end_of_region;
                    if !source_ready_sent {
                        publish_worker_event(
                            &mut events,
                            NativeSourceWorkerEvent::SourceReady {
                                native_decoder_sanitized_samples: sanitation,
                            },
                        );
                        source_ready_sent = true;
                    }
                    continue;
                }
                Err(HostChunkError::Full { .. }) => {
                    pending = Some(block);
                    thread::yield_now();
                    continue;
                }
                Err(error) => break NativeSourceWorkerExit::SubmitFailed(error),
            }
        }
        if end_submitted {
            thread::yield_now();
            continue;
        }
        let start_frame = decoder.next_source_frame();
        let decoded = match decoder.decode_quantum_into_planar(&mut planar_staging) {
            Ok(report) => report,
            Err(error) => break NativeSourceWorkerExit::DecodeFailed(error),
        };
        // The decoder report is already cumulative across decodes and seeks. Preserve that
        // watermark directly; adding successive reports would double-count earlier blocks.
        sanitation = sanitation.max(decoded.sanitized_sample_count);
        pending = Some(PendingBlock {
            generation,
            start_frame,
            frames: decoded.decoded_frames,
            end_of_region: decoded.end_of_region,
            native_decoder_sanitized_samples: sanitation,
        });
    };
    publish_worker_event(
        &mut events,
        NativeSourceWorkerEvent::Terminal {
            native_decoder_sanitized_samples: sanitation,
        },
    );
    exit
}

fn publish_worker_event(
    events: &mut Producer<NativeSourceWorkerEvent>,
    event: NativeSourceWorkerEvent,
) {
    let mut pending = event;
    loop {
        match events.try_push(pending) {
            Ok(()) => return,
            Err(QueueFull { value, .. }) => {
                pending = value;
                thread::yield_now();
            }
        }
    }
}

enum CommandResult {
    Wake,
    Seek,
    Snapshot,
}

fn apply_command<R: Read + Seek>(
    command: WorkerCommand,
    provider: &mut HostChunkProvider,
    decoder: &mut NativeWaveDecoder<R>,
    generation: &mut SourceGeneration,
) -> Result<CommandResult, NativeSourceWorkerExit> {
    match command {
        WorkerCommand::Wake => Ok(CommandResult::Wake),
        WorkerCommand::SnapshotSanitation => Ok(CommandResult::Snapshot),
        WorkerCommand::Seek {
            generation: requested,
            frame,
        } => {
            decoder
                .seek_to_source_frame(frame)
                .map_err(NativeSourceWorkerExit::DecodeFailed)?;
            provider
                .try_seek(SourceCommand::Seek {
                    generation: requested,
                    frame,
                })
                .map_err(NativeSourceWorkerExit::SeekFailed)?;
            *generation = requested;
            Ok(CommandResult::Seek)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use crate::{HostPlanarChunk, PcmSourceRing, QuantumFrames, SourceReadReport};
    use miso_engine_session::{CompileCaps, StableId, compile_session, parse_session_toml};

    struct Resolver {
        asset: Option<NativeResolvedAsset<Cursor<Vec<u8>>>>,
    }

    impl NativeSourceResolver for Resolver {
        type Asset = Cursor<Vec<u8>>;

        fn resolve(
            &mut self,
            opaque_locator: &str,
        ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError> {
            if opaque_locator != "opaque:stem" {
                return Err(NativeSourceResolverError::Unresolved);
            }
            self.asset
                .take()
                .ok_or(NativeSourceResolverError::Unresolved)
        }
    }

    fn resolver(samples: &[f32], identity: &[u8]) -> Resolver {
        resolver_wave(float32_wave(samples), identity)
    }

    fn resolver_wave(wave: Vec<u8>, identity: &[u8]) -> Resolver {
        Resolver {
            asset: Some(NativeResolvedAsset {
                observed_identity: identity.to_vec(),
                reader: Cursor::new(wave),
            }),
        }
    }

    fn request(region: NativeWaveRegion) -> NativeSourcePrepareRequest {
        NativeSourcePrepareRequest {
            locator: "opaque:stem".to_owned(),
            declared_identity: b"exact-identity".to_vec(),
            declared_sample_rate_hz: SampleRateHz(48_000),
            engine_sample_rate_hz: SampleRateHz(48_000),
            declared_channel_count: 1,
            region,
            ring_config: PcmSourceRingConfig {
                channel_count: 1,
                quantum_frames: QuantumFrames(4),
                frame_capacity: 8,
                initial_generation: SourceGeneration(1),
            },
        }
    }

    fn caps() -> NativeSourcePrepareCaps {
        NativeSourcePrepareCaps {
            parser: NativeWaveParseCaps {
                max_chunk_count: 8,
                max_skipped_metadata_bytes: 32,
            },
            max_worker_read_scratch_bytes: 64,
            max_total_engine_owned_bytes: u64::MAX,
            max_largest_allocation_bytes: u64::MAX,
            control_queue_items: NonZeroUsize::new(2).expect("two"),
        }
    }

    fn session_caps() -> NativeSessionSourcePrepareCaps {
        NativeSessionSourcePrepareCaps {
            source: NativeSourcePrepareCaps {
                parser: NativeWaveParseCaps {
                    max_chunk_count: 8,
                    max_skipped_metadata_bytes: 32,
                },
                max_worker_read_scratch_bytes: 2_048,
                max_total_engine_owned_bytes: u64::MAX,
                max_largest_allocation_bytes: u64::MAX,
                control_queue_items: NonZeroUsize::new(2).expect("two"),
            },
            max_combined_runtime_bytes: u64::MAX,
            max_largest_allocation_bytes: u64::MAX,
        }
    }

    fn compiled_source_session() -> CompiledSession {
        let mut session =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("session");
        session.sources[0].mapping.region.length_samples = 4;
        compile_session(
            &session,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled")
    }

    struct SessionResolver {
        assets: Vec<NativeResolvedAsset<Cursor<Vec<u8>>>>,
        calls: usize,
    }

    impl NativeSourceResolver for SessionResolver {
        type Asset = Cursor<Vec<u8>>;

        fn resolve(
            &mut self,
            _opaque_locator: &str,
        ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError> {
            self.calls = self.calls.saturating_add(1);
            if self.assets.is_empty() {
                Err(NativeSourceResolverError::Unresolved)
            } else {
                Ok(self.assets.remove(0))
            }
        }
    }

    fn session_resolver(identity: &[u8]) -> SessionResolver {
        SessionResolver {
            assets: vec![NativeResolvedAsset {
                observed_identity: identity.to_vec(),
                reader: Cursor::new(stereo_float32_wave(&[0.0; 8])),
            }],
            calls: 0,
        }
    }

    #[test]
    fn resolver_preparation_validates_identity_rate_channels_region_and_fixed_caps() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut wrong_identity = resolver(&[0.0; 4], b"other");
        let identity_error =
            match prepare_native_source(&mut wrong_identity, request(region), caps()) {
                Ok(_) => panic!("identity should reject"),
                Err(error) => error,
            };
        assert_eq!(
            identity_error,
            NativeSourcePrepareError::ContentIdentityMismatch
        );
        assert_eq!(
            identity_error.diagnostic_code(),
            SourceDiagnosticCode::ContentIdentityMismatch
        );

        let mut wrong_rate = resolver(&[0.0; 4], b"exact-identity");
        let mut rate_request = request(region);
        rate_request.engine_sample_rate_hz = SampleRateHz(44_100);
        let rate_error = match prepare_native_source(&mut wrong_rate, rate_request, caps()) {
            Ok(_) => panic!("rate should reject"),
            Err(error) => error,
        };
        assert_eq!(rate_error, NativeSourcePrepareError::RateMismatch);

        let mut wrong_channels = resolver(&[0.0; 4], b"exact-identity");
        let mut channel_request = request(region);
        channel_request.declared_channel_count = 2;
        let channel_error =
            match prepare_native_source(&mut wrong_channels, channel_request, caps()) {
                Ok(_) => panic!("channels should reject"),
                Err(error) => error,
            };
        assert_eq!(channel_error, NativeSourcePrepareError::ChannelMismatch);

        let mut wrong_region = resolver(&[0.0; 4], b"exact-identity");
        let region_error = match prepare_native_source(
            &mut wrong_region,
            request(NativeWaveRegion {
                start_frame: SourceFrame(3),
                length_frames: 2,
            }),
            caps(),
        ) {
            Ok(_) => panic!("region should reject"),
            Err(error) => error,
        };
        assert_eq!(region_error, NativeSourcePrepareError::RegionOutOfBounds);

        let mut capped = resolver(&[0.0; 4], b"exact-identity");
        let mut too_small = caps();
        too_small.max_worker_read_scratch_bytes = 15;
        let cap_error = match prepare_native_source(&mut capped, request(region), too_small) {
            Ok(_) => panic!("read scratch cap should reject"),
            Err(error) => error,
        };
        assert_eq!(cap_error, NativeSourcePrepareError::ResourceLimit);
    }

    #[test]
    fn native_worker_and_host_provider_produce_identical_prepared_ring_pcm() {
        let samples = [0.25, -0.5, 0.75, -1.0];
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(&samples, b"exact-identity");
        let (mut controller, mut worker, mut native_consumer, report) =
            prepare_native_source_parts(&mut native_resolver, request(region), caps())
                .expect("prepare");
        assert_eq!(report.decoder_read_scratch_bytes, 16);
        assert_eq!(report.worker_planar_staging_bytes, 16);
        assert!(matches!(
            controller.wait_for_event().expect("ready"),
            NativeSourceWorkerEvent::SourceReady { .. }
        ));
        let native = read_one(&mut native_consumer);

        let (producer, mut host_consumer, _) = PcmSourceRing::prepare(PcmSourceRingConfig {
            channel_count: 1,
            quantum_frames: QuantumFrames(4),
            frame_capacity: 8,
            initial_generation: SourceGeneration(1),
        })
        .expect("ring");
        let mut host = producer.into_host_chunk_provider(SampleRateHz(48_000));
        host.submit(HostPlanarChunk {
            sample_rate_hz: SampleRateHz(48_000),
            generation: SourceGeneration(1),
            start_frame: SourceFrame(0),
            planes: &[&samples],
            frames: 4,
            end_of_region: true,
        })
        .expect("host submit");
        let host_pcm = read_one(&mut host_consumer);
        assert_eq!(native, host_pcm);
        assert_eq!(
            native_consumer.telemetry().native_decoder_sanitized_samples,
            0
        );
        assert_eq!(host.telemetry().native_decoder_sanitized_samples, 0);
        assert_eq!(
            host_consumer.telemetry().native_decoder_sanitized_samples,
            0
        );
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
    }

    #[test]
    fn controller_snapshot_and_terminal_watermarks_are_exact_and_monotonic() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(
            &[f32::INFINITY, f32::from_bits(1), -0.0, 0.25],
            b"exact-identity",
        );
        let (mut controller, mut worker, mut consumer, _) =
            prepare_native_source_parts(&mut native_resolver, request(region), caps())
                .expect("prepare");
        assert!(matches!(
            controller.wait_for_event().expect("ready"),
            NativeSourceWorkerEvent::SourceReady {
                native_decoder_sanitized_samples: 2
            }
        ));
        assert_eq!(controller.native_decoder_sanitized_samples(), 2);
        assert_eq!(
            controller
                .snapshot_native_decoder_sanitized_samples()
                .expect("snapshot"),
            2
        );
        let _ = read_one(&mut consumer);
        assert_eq!(consumer.telemetry().native_decoder_sanitized_samples, 2);
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal {
                native_decoder_sanitized_samples: 2
            }
        ));
        assert_eq!(controller.native_decoder_sanitized_samples(), 2);
    }

    #[test]
    fn multiblock_native_watermark_does_not_readd_the_cumulative_decoder_report() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 8,
        };
        let mut native_resolver = resolver(
            &[
                f32::INFINITY,
                0.25,
                f32::from_bits(1),
                -0.25,
                f32::NAN,
                0.5,
                -0.5,
                0.0,
            ],
            b"exact-identity",
        );
        let (mut controller, mut worker, mut consumer, _) =
            prepare_native_source_parts(&mut native_resolver, request(region), caps())
                .expect("prepare");
        controller.wait_for_event().expect("ready");
        let _ = read_one(&mut consumer);
        assert_eq!(consumer.telemetry().native_decoder_sanitized_samples, 2);
        let _ = read_one(&mut consumer);
        assert_eq!(consumer.telemetry().native_decoder_sanitized_samples, 3);
        assert_eq!(
            controller
                .snapshot_native_decoder_sanitized_samples()
                .expect("cumulative snapshot"),
            3
        );
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal {
                native_decoder_sanitized_samples: 3
            }
        ));
    }

    #[test]
    fn native_queue_layout_and_per_source_caps_use_exact_requests() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut initial_resolver = resolver(&[0.0; 4], b"exact-identity");
        let initial = prepare_native_source(&mut initial_resolver, request(region), caps())
            .expect("initial preparation");
        let report = initial.resource_report();
        let control = exact_queue_resources::<WorkerCommand>(caps().control_queue_items)
            .expect("control queue layout");
        let events = exact_queue_resources::<NativeSourceWorkerEvent>(
            NonZeroUsize::new(2).expect("two events"),
        )
        .expect("event queue layout");
        let stop = exact_queue_resources::<()>(NonZeroUsize::new(1).expect("one stop"))
            .expect("stop queue layout");
        assert_eq!(report.worker_control_queue_bytes, control.total_bytes);
        assert_eq!(
            report.worker_control_queue_largest_allocation_bytes,
            control.largest_allocation_bytes
        );
        assert_eq!(
            report.worker_control_queue_alignment_bytes,
            control.alignment_bytes
        );
        assert_eq!(report.worker_event_queue_bytes, events.total_bytes);
        assert_eq!(
            report.worker_event_queue_largest_allocation_bytes,
            events.largest_allocation_bytes
        );
        assert_eq!(
            report.worker_event_queue_alignment_bytes,
            events.alignment_bytes
        );
        assert_eq!(report.worker_stop_queue_bytes, stop.total_bytes);
        assert_eq!(
            report.worker_stop_queue_largest_allocation_bytes,
            stop.largest_allocation_bytes
        );
        assert_eq!(
            report.worker_stop_queue_alignment_bytes,
            stop.alignment_bytes
        );
        let exact_largest = report
            .ring
            .largest_allocation_bytes
            .max(report.decoder_read_scratch_bytes)
            .max(report.worker_planar_staging_bytes)
            .max(control.largest_allocation_bytes)
            .max(events.largest_allocation_bytes)
            .max(stop.largest_allocation_bytes);
        assert_eq!(report.largest_allocation_bytes, exact_largest);
        drop(initial);

        let mut exact_caps = caps();
        exact_caps.max_total_engine_owned_bytes = report.total_engine_owned_bytes;
        exact_caps.max_largest_allocation_bytes = report.largest_allocation_bytes;
        let mut exact_resolver = resolver(&[0.0; 4], b"exact-identity");
        let exact = prepare_native_source(&mut exact_resolver, request(region), exact_caps)
            .expect("exact per-source caps");
        assert_eq!(exact.resource_report(), report);
        drop(exact);

        let mut total_short_caps = exact_caps;
        total_short_caps.max_total_engine_owned_bytes = report
            .total_engine_owned_bytes
            .checked_sub(1)
            .expect("nonzero source total");
        let mut total_short_resolver = resolver(&[0.0; 4], b"exact-identity");
        assert!(matches!(
            prepare_native_source(&mut total_short_resolver, request(region), total_short_caps),
            Err(NativeSourcePrepareError::ResourceLimit)
        ));

        let mut largest_short_caps = exact_caps;
        largest_short_caps.max_largest_allocation_bytes = report
            .largest_allocation_bytes
            .checked_sub(1)
            .expect("nonzero largest allocation");
        let mut largest_short_resolver = resolver(&[0.0; 4], b"exact-identity");
        assert!(matches!(
            prepare_native_source(
                &mut largest_short_resolver,
                request(region),
                largest_short_caps
            ),
            Err(NativeSourcePrepareError::ResourceLimit)
        ));
    }

    #[test]
    fn controller_first_drop_does_not_detach_the_retirement_stop_owner() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(&[0.25; 4], b"exact-identity");
        let (controller, mut worker, _consumer, _) =
            prepare_native_source_parts(&mut native_resolver, request(region), caps())
                .expect("prepare");
        drop(controller);
        assert_eq!(
            worker.stop_and_join().expect("retirement stop and join"),
            NativeSourceWorkerExit::Stopped
        );
    }

    #[test]
    fn worker_seek_stop_wake_and_join_are_bounded_and_typed() {
        let mut native_resolver = resolver(&[0.0; 8], b"exact-identity");
        let mut lifecycle_caps = caps();
        lifecycle_caps.control_queue_items = NonZeroUsize::new(3).expect("three");
        let (mut controller, mut worker, _consumer, _) = prepare_native_source_parts(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 8,
            }),
            lifecycle_caps,
        )
        .expect("prepare");
        controller.wait_for_event().expect("ready");
        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(4),
            })
            .expect("seek");
        assert!(matches!(
            controller.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(4),
            }),
            Err(NativeSourceWorkerControlError::GenerationNotStrictlyIncreasing { .. })
        ));
        assert!(matches!(
            controller.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(3),
                frame: SourceFrame(9),
            }),
            Err(NativeSourceWorkerControlError::RegionOutOfBounds)
        ));
        let _ = controller.try_wake();
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
    }

    #[test]
    fn compiled_session_sources_prepare_once_and_publish_one_graph_source_set() {
        let session = compiled_source_session();
        let mut resolver = session_resolver(b"sha256:demo");
        let prepared = prepare_native_session_sources(&session, &mut resolver, session_caps())
            .expect("session source preparation");
        assert_eq!(resolver.calls, 1);
        assert_eq!(prepared.resources.source_count, 1);
        assert_eq!(prepared.source_set.claims().len(), 1);
        assert_eq!(
            prepared.resources.source_pcm_already_charged_bytes,
            session.resource_estimate().source_ring_bytes
        );
        let (source_set, _controllers, _resources) = prepared.into_parts();
        drop(source_set);
    }

    #[test]
    fn native_sanitation_reaches_after_disarm_source_set_telemetry_and_drop_retires_worker() {
        use core::num::NonZeroUsize;
        use miso_engine_core::realtime::RenderEnvelope;

        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(
            &[f32::INFINITY, f32::from_bits(1), -0.0, 0.25],
            b"exact-identity",
        );
        let (mut controller, worker, mut consumer, report) =
            prepare_native_source_parts(&mut native_resolver, request(region), caps())
                .expect("prepare");
        controller.wait_for_event().expect("ready");
        let (decoded, _) = read_one(&mut consumer);
        assert_eq!(decoded[0].to_bits(), 0.0_f32.to_bits());
        assert_eq!(decoded[1].to_bits(), 0.0_f32.to_bits());
        assert_eq!(controller.native_decoder_sanitized_samples(), 2);

        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two"),
        };
        let node = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("voice").expect("ID"),
            stage: TrackStage::Input,
        };
        let source_set = prepare_graph_source_set(
            envelope,
            vec![SourceGraphSource::with_native_worker(
                consumer,
                report.ring,
                report
                    .total_engine_owned_bytes
                    .checked_sub(report.ring.total_engine_owned_bytes)
                    .expect("native overhead"),
                report.largest_allocation_bytes,
                worker,
            )],
            vec![SourceGraphTrackMapping {
                node,
                source_index: 0,
                left_channel: 0,
                right_channel: 0,
            }],
        )
        .expect("source set");
        let mut telemetry = [0_u64; 5];
        assert_eq!(source_set.copy_after_disarm_telemetry(&mut telemetry), 5);
        assert_eq!(telemetry[4], 2);
        drop(source_set);
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal {
                native_decoder_sanitized_samples: 2
            }
        ));
        assert_eq!(
            controller.try_wake(),
            Err(NativeSourceWorkerControlError::Stopped)
        );
    }

    #[test]
    fn retired_graph_plan_is_the_native_worker_join_owner() {
        use core::num::NonZeroUsize;
        use miso_engine_core::realtime::{
            PlanExchangeConfig, PlanarBufferMut, PrepareRenderPlan, RenderEnvelope, RenderIo,
            RenderTime, SwapOutcome, plan_exchange,
        };
        use miso_engine_effect_contract::{LatencySamples, TailSamples};
        use miso_engine_graph::{
            DependencyLevel, GraphEdge, GraphEdgeId, GraphNode, GraphPortId, GraphPortKind,
            GraphResourceEstimate, GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec,
            PreparedGraphPlan, PreparedGraphPlanParts,
        };

        struct Noop;
        impl GraphRuntimeProcessor for Noop {
            fn process(
                &mut self,
                _block: miso_engine_graph::GraphBindingBlock<'_>,
            ) -> Result<(), miso_engine_core::realtime::RenderError> {
                Ok(())
            }
        }

        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two"),
        };
        let input = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("voice").expect("ID"),
            stage: TrackStage::Input,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let mut native_resolver = resolver(&[0.25; 4], b"exact-identity");
        let (mut controller, worker, consumer, report) = prepare_native_source_parts(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 4,
            }),
            caps(),
        )
        .expect("prepare");
        controller.wait_for_event().expect("ready");
        let source_set = prepare_graph_source_set(
            envelope,
            vec![SourceGraphSource::with_native_worker(
                consumer,
                report.ring,
                report
                    .total_engine_owned_bytes
                    .checked_sub(report.ring.total_engine_owned_bytes)
                    .expect("native overhead"),
                report.largest_allocation_bytes,
                worker,
            )],
            vec![SourceGraphTrackMapping {
                node: input.clone(),
                source_index: 0,
                left_channel: 0,
                right_channel: 0,
            }],
        )
        .expect("source set");
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 40,
            spec: GraphSpec {
                nodes: vec![
                    GraphNode {
                        id: input.clone(),
                        latency: LatencySamples(0),
                        tail: TailSamples::Finite(0),
                    },
                    GraphNode {
                        id: output.clone(),
                        latency: LatencySamples(0),
                        tail: TailSamples::Finite(0),
                    },
                ],
                ports: Vec::new(),
                edges: vec![GraphEdge {
                    id: GraphEdgeId::RouteSource {
                        route_id: StableGraphId::parse("voice-route").expect("ID"),
                    },
                    source: GraphPortId {
                        node: input.clone(),
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: output.clone(),
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: "$.routes[0]".to_owned(),
                }],
            },
            sequential_schedule: vec![input.clone(), output.clone()],
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![output.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: GraphResourceEstimate {
                logical_nodes: 0,
                materialized_nodes: 0,
                edges: 0,
                schedule_items: 0,
                dependency_levels: 0,
                reductions: 0,
                routes: 0,
                effects: 0,
                audio_buffer_samples: 0,
                total_delay_samples: 0,
                delay_bytes: 0,
                graph_metadata_bytes: 0,
                declared_effect_bytes: 0,
                builtin_bank_bytes: 0,
                builtin_bank_scratch_bytes: 0,
                builtin_bank_count: 0,
                largest_allocation_bytes: 0,
                incremental_plan_bytes: 0,
                session_plus_plan_bytes: 0,
            },
            envelope,
            required_bindings: vec![input, output.clone()],
            routes: Vec::new(),
            effects: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let old = match graph.bind_with_source_set(
            GraphRuntimeBindings {
                envelope,
                nodes: vec![miso_engine_graph::GraphNodeBinding::new(
                    output,
                    Box::new(Noop),
                )],
                observers: Vec::new(),
            },
            source_set,
        ) {
            Ok(plan) => plan,
            Err(failure) => panic!("bind failed: {}", failure.code),
        };
        let replacement =
            miso_engine_core::realtime::PreparedRenderPlan::prepare(PrepareRenderPlan {
                plan_id: 41,
                envelope,
                scratch: &[],
                parameter_defaults: &[],
                event_capacity: 0,
            })
            .expect("replacement");
        let (mut publisher, mut owner, mut retirer) = plan_exchange(
            old,
            PlanExchangeConfig {
                publication_capacity: NonZeroUsize::new(1).expect("one"),
                retirement_capacity: NonZeroUsize::new(1).expect("one"),
            },
        )
        .expect("exchange");
        assert!(publisher.publish(replacement).is_ok());
        let mut output_pcm = [0.0_f32; 8];
        assert_eq!(
            owner
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut output_pcm, 2, 4, 4).expect("output"),
                    },
                    RenderTime { absolute_sample: 0 },
                )
                .expect("render")
                .swap,
            SwapOutcome::Applied
        );
        let (_, retired) = retirer.try_reclaim().expect("retired plan");
        assert_ne!(
            controller.try_wake(),
            Err(NativeSourceWorkerControlError::Stopped)
        );
        drop(retired);
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal { .. }
        ));
        assert_eq!(
            controller.try_wake(),
            Err(NativeSourceWorkerControlError::Stopped)
        );
    }

    #[test]
    fn compiled_session_source_mismatch_and_cap_fail_without_publication() {
        let session = compiled_source_session();
        let mut identity_resolver = session_resolver(b"wrong");
        let identity = match prepare_native_session_sources(
            &session,
            &mut identity_resolver,
            session_caps(),
        ) {
            Ok(_) => panic!("identity mismatch unexpectedly prepared"),
            Err(failure) => failure,
        };
        assert_eq!(identity.diagnostics.len(), 1);
        assert_eq!(
            identity.diagnostics[0].code,
            SourceDiagnosticCode::ContentIdentityMismatch
        );

        let mut capped_resolver = session_resolver(b"sha256:demo");
        let mut caps = session_caps();
        caps.max_combined_runtime_bytes = 0;
        let capped = match prepare_native_session_sources(&session, &mut capped_resolver, caps) {
            Ok(_) => panic!("combined cap unexpectedly prepared"),
            Err(failure) => failure,
        };
        assert_eq!(
            capped.diagnostics[0].code,
            SourceDiagnosticCode::ResourceLimit
        );
    }

    #[test]
    fn combined_retained_cap_accepts_exactly_and_rejects_one_byte_short() {
        let session = compiled_source_session();
        let mut initial_resolver = session_resolver(b"sha256:demo");
        let initial =
            prepare_native_session_sources(&session, &mut initial_resolver, session_caps())
                .expect("uncapped preparation");
        let exact_total = initial.resources.combined_runtime_bytes;
        assert!(initial.resources.controller_records_bytes > 0);
        drop(initial);

        let mut exact_caps = session_caps();
        exact_caps.max_combined_runtime_bytes = exact_total;
        let mut exact_resolver = session_resolver(b"sha256:demo");
        let exact = prepare_native_session_sources(&session, &mut exact_resolver, exact_caps)
            .expect("exact retained cap");
        assert_eq!(exact.resources.combined_runtime_bytes, exact_total);
        drop(exact);

        let mut short_caps = session_caps();
        short_caps.max_combined_runtime_bytes = exact_total.checked_sub(1).expect("nonzero total");
        let mut short_resolver = session_resolver(b"sha256:demo");
        let short = match prepare_native_session_sources(&session, &mut short_resolver, short_caps)
        {
            Ok(_) => panic!("one byte short cap unexpectedly prepared"),
            Err(failure) => failure,
        };
        assert_eq!(
            short.diagnostics[0].code,
            SourceDiagnosticCode::ResourceLimit
        );
    }

    #[test]
    fn launch_rates_prepare_and_extended_rate_mismatch_is_rejected() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        for rate_hz in [44_100, 48_000, 88_200, 96_000] {
            let mut native_resolver =
                resolver_wave(float32_wave_at_rate(&[0.0; 4], rate_hz), b"exact-identity");
            let mut prepare_request = request(region);
            prepare_request.declared_sample_rate_hz = SampleRateHz(rate_hz);
            prepare_request.engine_sample_rate_hz = SampleRateHz(rate_hz);
            let prepared = prepare_native_source(&mut native_resolver, prepare_request, caps())
                .expect("launch rate preparation");
            drop(prepared);
        }

        let mut extended_resolver =
            resolver_wave(float32_wave_at_rate(&[0.0; 4], 192_000), b"exact-identity");
        let mut extended_request = request(region);
        extended_request.declared_sample_rate_hz = SampleRateHz(192_000);
        let mismatch = match prepare_native_source(&mut extended_resolver, extended_request, caps())
        {
            Ok(_) => panic!("extended source unexpectedly prepared with implicit conversion"),
            Err(error) => error,
        };
        assert_eq!(mismatch, NativeSourcePrepareError::RateMismatch);
    }

    #[test]
    fn retained_layout_grid_is_checked_without_source_duration_storage() {
        for count in [1_usize, 4, 65_537] {
            let entries =
                crate::allocation_class::<crate::GraphSourceEntry>(count).expect("entry layout");
            let mappings =
                crate::allocation_class::<SourceGraphTrackMapping>(count).expect("mapping layout");
            let claims = crate::allocation_class::<miso_engine_graph::GraphSourceInputClaim>(count)
                .expect("claim layout");
            let controllers =
                retained_array_bytes::<NativeSourceController>(count).expect("controller layout");
            assert_eq!(entries.item_count, u64::try_from(count).expect("count"));
            assert_eq!(mappings.item_count, entries.item_count);
            assert_eq!(claims.item_count, entries.item_count);
            assert_eq!(
                controllers,
                u64::try_from(size_of::<NativeSourceController>())
                    .expect("size")
                    .checked_mul(entries.item_count)
                    .expect("checked grid")
            );
        }
    }

    #[test]
    fn compiled_session_source_rollback_stops_earlier_worker() {
        let mut session_toml =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("session");
        session_toml.sources[0].mapping.region.length_samples = 4;
        let mut second = session_toml.sources[0].clone();
        second.id = StableId::parse("voice2").expect("ID");
        second.content.locator = "host:voice2".to_owned();
        session_toml.sources.push(second);
        let session = compile_session(
            &session_toml,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled");
        let mut resolver = session_resolver(b"sha256:demo");
        let failure = match prepare_native_session_sources(&session, &mut resolver, session_caps())
        {
            Ok(_) => panic!("second resolver call unexpectedly prepared"),
            Err(failure) => failure,
        };
        assert_eq!(resolver.calls, 2);
        assert_eq!(failure.diagnostics.len(), 1);
        assert_eq!(
            failure.diagnostics[0].code,
            SourceDiagnosticCode::AssetUnresolved
        );
        assert_eq!(failure.diagnostics[0].path.as_str(), "$.sources[id=voice2]");
    }

    fn read_one(consumer: &mut PcmSourceConsumer) -> ([f32; 4], SourceReadReport) {
        let mut output = [0.0; 4];
        let report = {
            let mut planes = [&mut output[..]];
            consumer.read_block(&mut planes).expect("read")
        };
        (output, report)
    }

    fn float32_wave(samples: &[f32]) -> Vec<u8> {
        float32_wave_at_rate(samples, 48_000)
    }

    fn float32_wave_at_rate(samples: &[f32], sample_rate_hz: u32) -> Vec<u8> {
        let mut format = Vec::new();
        format.extend_from_slice(&3_u16.to_le_bytes());
        format.extend_from_slice(&1_u16.to_le_bytes());
        format.extend_from_slice(&sample_rate_hz.to_le_bytes());
        format.extend_from_slice(
            &sample_rate_hz
                .checked_mul(4)
                .expect("float32 byte rate")
                .to_le_bytes(),
        );
        format.extend_from_slice(&4_u16.to_le_bytes());
        format.extend_from_slice(&32_u16.to_le_bytes());
        let mut data = Vec::new();
        for sample in samples {
            data.extend_from_slice(&sample.to_le_bytes());
        }
        let mut wave = Vec::new();
        wave.extend_from_slice(b"RIFF");
        wave.extend_from_slice(&0_u32.to_le_bytes());
        wave.extend_from_slice(b"WAVE");
        append_chunk(&mut wave, b"fmt ", &format);
        append_chunk(&mut wave, b"data", &data);
        let riff_size = u32::try_from(wave.len() - 8).expect("len");
        wave[4..8].copy_from_slice(&riff_size.to_le_bytes());
        wave
    }

    fn stereo_float32_wave(samples: &[f32]) -> Vec<u8> {
        let mut format = Vec::new();
        format.extend_from_slice(&3_u16.to_le_bytes());
        format.extend_from_slice(&2_u16.to_le_bytes());
        format.extend_from_slice(&48_000_u32.to_le_bytes());
        format.extend_from_slice(&384_000_u32.to_le_bytes());
        format.extend_from_slice(&8_u16.to_le_bytes());
        format.extend_from_slice(&32_u16.to_le_bytes());
        let mut data = Vec::new();
        for sample in samples {
            data.extend_from_slice(&sample.to_le_bytes());
        }
        let mut wave = Vec::new();
        wave.extend_from_slice(b"RIFF");
        wave.extend_from_slice(&0_u32.to_le_bytes());
        wave.extend_from_slice(b"WAVE");
        append_chunk(&mut wave, b"fmt ", &format);
        append_chunk(&mut wave, b"data", &data);
        let riff_size = u32::try_from(wave.len() - 8).expect("len");
        wave[4..8].copy_from_slice(&riff_size.to_le_bytes());
        wave
    }

    fn append_chunk(out: &mut Vec<u8>, id: &[u8; 4], payload: &[u8]) {
        out.extend_from_slice(id);
        out.extend_from_slice(&u32::try_from(payload.len()).expect("len").to_le_bytes());
        out.extend_from_slice(payload);
        if payload.len() % 2 == 1 {
            out.push(0);
        }
    }
}
