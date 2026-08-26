//! Native resolver, preparation, and one-worker-per-prepared-set source delivery.
//!
//! This module is control/worker-only and cfg-excluded from browser Wasm. The started worker owns
//! the opened reader, decoder, prepared source producer, and all decoded staging storage; it never
//! shares those mutable objects with a render worker.

use core::{alloc::Layout, cell::Cell, marker::PhantomData, num::NonZeroUsize};
use std::{
    io::{Read, Seek},
    thread::{self, JoinHandle, Thread},
    time::Duration,
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

use crate::native_wave::{validate_region, validate_seek_frame};
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
    /// Maximum exact set-of-one source plus worker total; session preparation applies it to each
    /// base source and applies shared-worker limits through [`NativeSessionSourcePrepareCaps`].
    pub max_total_engine_owned_bytes: u64,
    /// Maximum set-of-one allocation; sessions apply this to base-source allocations and their
    /// session cap to the one shared typed boxed job array.
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
    /// Exact three-item worker event capacity: SourceReady, at most one synchronous Snapshot,
    /// and one slot reserved for Terminal.
    pub worker_event_queue_items: u64,
    /// Largest worker-event queue allocation request.
    pub worker_event_queue_largest_allocation_bytes: u64,
    /// Maximum required alignment among worker-event queue allocations.
    pub worker_event_queue_alignment_bytes: u64,
    /// Exact once-per-thread stop queue and typed boxed job-array accounting when folded.
    pub worker: NativeWorkerResourceReport,
    /// Exact base source plus folded once-per-thread stop queue and job-array total.
    pub total_engine_owned_bytes: u64,
    /// Largest exact ring, decoder, staging, queue, or typed boxed job-array allocation.
    pub largest_allocation_bytes: u64,
}

/// Exact once-per-thread native worker allocation accounting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeWorkerResourceReport {
    /// Exact capacity-one stop SPSC header and slot payload bytes.
    pub stop_queue_bytes: u64,
    /// Exact stop SPSC logical item capacity.
    pub stop_queue_items: u64,
    /// Largest stop SPSC allocation request.
    pub stop_queue_largest_allocation_bytes: u64,
    /// Maximum required alignment among stop SPSC allocations.
    pub stop_queue_alignment_bytes: u64,
    /// Exact one-allocation boxed source-job array bytes.
    pub job_array_bytes: u64,
    /// Exact number of source jobs owned by the thread.
    pub job_count: u64,
    /// Required alignment of the boxed source-job array.
    pub job_array_alignment_bytes: u64,
}

impl NativeWorkerResourceReport {
    const fn total_engine_owned_bytes(self) -> Option<u64> {
        self.stop_queue_bytes.checked_add(self.job_array_bytes)
    }

    const fn largest_allocation_bytes(self) -> u64 {
        if self.stop_queue_largest_allocation_bytes > self.job_array_bytes {
            self.stop_queue_largest_allocation_bytes
        } else {
            self.job_array_bytes
        }
    }
}

const EMPTY_WORKER_RESOURCE_REPORT: NativeWorkerResourceReport = NativeWorkerResourceReport {
    stop_queue_bytes: 0,
    stop_queue_items: 0,
    stop_queue_largest_allocation_bytes: 0,
    stop_queue_alignment_bytes: 1,
    job_array_bytes: 0,
    job_count: 0,
    job_array_alignment_bytes: 1,
};

/// One exact retained allocation request used by the test-support duration audit.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc(hidden)]
pub struct NativeSourceAllocationLayoutEntry {
    pub category: &'static str,
    pub requested_size_bytes: u64,
    pub alignment_bytes: u64,
    pub count: u64,
}

/// Enumerate the exact source-owned allocation requests for an accepted prepared source.
///
/// This control-plane-only test support is derived from the same concrete queue/block layouts the
/// preparation path allocates; it excludes asset bytes, allocator headers, thread stacks and RSS.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn native_source_allocation_layout(
    ring_config: PcmSourceRingConfig,
    caps: NativeSourcePrepareCaps,
    report: NativeSourceResourceReport,
) -> Result<Vec<NativeSourceAllocationLayoutEntry>, NativeSourcePrepareError> {
    fn push_queue<T: Send + 'static>(
        entries: &mut Vec<NativeSourceAllocationLayoutEntry>,
        category: &'static str,
        capacity: NonZeroUsize,
    ) -> Result<(), NativeSourcePrepareError> {
        let payload = bounded_spsc_retained_payload::<T>(capacity)
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
        entries.push(NativeSourceAllocationLayoutEntry {
            category,
            requested_size_bytes: u64::try_from(payload.ring_header_bytes)
                .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
            alignment_bytes: u64::try_from(payload.ring_header_align)
                .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
            count: 1,
        });
        entries.push(NativeSourceAllocationLayoutEntry {
            category,
            requested_size_bytes: u64::try_from(payload.slot_payload_bytes)
                .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
            alignment_bytes: u64::try_from(payload.slot_payload_align)
                .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
            count: 1,
        });
        Ok(())
    }

    let block_count = report.ring.transfer_block_count;
    let samples_per_block = u64::from(ring_config.channel_count)
        .checked_mul(u64::from(ring_config.quantum_frames.0))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let pcm_per_block = samples_per_block
        .checked_mul(u64::try_from(core::mem::size_of::<f32>()).expect("f32 size"))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let mut entries = Vec::with_capacity(16);
    push_queue::<Box<crate::TransferBlock>>(
        &mut entries,
        "ring.data_queue",
        NonZeroUsize::new(
            usize::try_from(block_count).map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
        )
        .ok_or(NativeSourcePrepareError::ResourceLimit)?,
    )?;
    push_queue::<Box<crate::TransferBlock>>(
        &mut entries,
        "ring.recycle_queue",
        NonZeroUsize::new(
            usize::try_from(block_count).map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
        )
        .ok_or(NativeSourcePrepareError::ResourceLimit)?,
    )?;
    push_queue::<SourceCommand>(
        &mut entries,
        "ring.seek_queue",
        NonZeroUsize::new(1).expect("one seek"),
    )?;
    entries.push(NativeSourceAllocationLayoutEntry {
        category: "ring.transfer_block_metadata",
        requested_size_bytes: u64::try_from(core::mem::size_of::<crate::TransferBlock>())
            .expect("platform size"),
        alignment_bytes: u64::try_from(core::mem::align_of::<crate::TransferBlock>())
            .expect("platform alignment"),
        count: block_count,
    });
    entries.push(NativeSourceAllocationLayoutEntry {
        category: "ring.transfer_block_pcm",
        requested_size_bytes: pcm_per_block,
        alignment_bytes: u64::try_from(core::mem::align_of::<f32>()).expect("f32 alignment"),
        count: block_count,
    });
    entries.push(NativeSourceAllocationLayoutEntry {
        category: "decoder.read_scratch",
        requested_size_bytes: report.decoder_read_scratch_bytes,
        alignment_bytes: 1,
        count: 1,
    });
    entries.push(NativeSourceAllocationLayoutEntry {
        category: "worker.planar_staging",
        requested_size_bytes: report.worker_planar_staging_bytes,
        alignment_bytes: u64::try_from(core::mem::align_of::<f32>()).expect("f32 alignment"),
        count: 1,
    });
    push_queue::<WorkerCommand>(
        &mut entries,
        "worker.command_queue",
        caps.control_queue_items,
    )?;
    push_queue::<NativeSourceWorkerEvent>(
        &mut entries,
        "worker.event_queue",
        WORKER_EVENT_QUEUE_ITEMS,
    )?;
    push_queue::<()>(
        &mut entries,
        "worker.stop_queue",
        NonZeroUsize::new(
            usize::try_from(report.worker.stop_queue_items)
                .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
        )
        .ok_or(NativeSourcePrepareError::ResourceLimit)?,
    )?;
    entries.push(NativeSourceAllocationLayoutEntry {
        category: "worker.job_array",
        requested_size_bytes: report.worker.job_array_bytes,
        alignment_bytes: report.worker.job_array_alignment_bytes,
        count: 1,
    });
    entries.sort();
    Ok(entries)
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
    /// Exact once-per-session worker stop queue plus typed boxed job-array bytes.
    pub worker_bytes: u64,
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
    worker_resources: NativeWorkerResourceReport,
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
    pub fn resource_report(&self) -> NativeSourceResourceReport {
        fold_worker_resources(self.resources, self.worker_resources)
            .expect("accepted worker resources fit the source report")
    }

    /// Move the controller and native source into their separate prepared ownership domains.
    #[must_use]
    pub fn into_graph_source(self) -> (NativeSourceController, SourceGraphSource) {
        let Self {
            controller,
            consumer,
            resources,
            worker_resources,
            worker,
        } = self;
        let resources = fold_worker_resources(resources, worker_resources)
            .expect("accepted worker resources fit the source report");
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

/// Per-source job lifecycle event delivered only through a bounded non-render channel.
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
    /// Final job-local sanitation watermark and exact terminal reason.
    Terminal {
        native_decoder_sanitized_samples: u64,
        exit: NativeSourceWorkerExit,
    },
}

/// Exact per-job terminal reason embedded in [`NativeSourceWorkerEvent::Terminal`].
///
/// The shared set worker join uses the same type for its set-level result: an explicit set stop
/// returns [`Self::Stopped`], while isolated per-job failures are observed through their events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerExit {
    /// Explicit set stop ended a live job or the shared worker.
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
    #[cfg(feature = "test-support")]
    AuditHold,
}

const CONTROL_POLL_WAIT: Duration = Duration::from_micros(100);
#[cfg(feature = "test-support")]
const AUDIT_WORKER_WAIT: Duration = Duration::from_millis(1);
// SourceReady and one synchronous Snapshot may be outstanding; the third slot is reserved so
// retirement can always publish Terminal without waiting for the controller to drain events.
const WORKER_EVENT_QUEUE_ITEMS: NonZeroUsize = NonZeroUsize::new(3).expect("three events");
const MIN_RENDER_WAIT_NANOS: u128 = 1_000_000;
const MAX_RENDER_WAIT_NANOS: u128 = 20_000_000;

/// Deterministic off-render hold/release gate for native-worker qualification only.
///
/// The worker acknowledges that it is held before the caller enters render, and acknowledges its
/// release before the caller renders resumed PCM. This is compiled only with `test-support`.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub struct NativeWorkerAuditGate {
    held: Consumer<()>,
    release: Producer<()>,
    resumed: Consumer<()>,
    worker: Thread,
    released: bool,
    _not_sync: PhantomData<Cell<()>>,
}

#[cfg(feature = "test-support")]
struct WorkerAuditGate {
    held: Producer<()>,
    release: Consumer<()>,
    resumed: Producer<()>,
}

#[cfg(not(feature = "test-support"))]
struct WorkerAuditGate;

#[cfg(feature = "test-support")]
impl NativeWorkerAuditGate {
    /// Wait outside render until the worker has consumed a hold command.
    #[doc(hidden)]
    pub fn wait_until_held(&mut self) -> Result<(), NativeSourceWorkerControlError> {
        loop {
            match self.held.try_pop() {
                Ok(()) => return Ok(()),
                Err(QueueEmpty { .. }) => thread::sleep(CONTROL_POLL_WAIT),
            }
        }
    }

    /// Release one held worker outside render.
    #[doc(hidden)]
    pub fn release_and_wait(&mut self) -> Result<(), NativeSourceWorkerControlError> {
        if self.released {
            return Err(NativeSourceWorkerControlError::Stopped);
        }
        self.release
            .try_push(())
            .map_err(|_| NativeSourceWorkerControlError::Backpressure)?;
        self.worker.unpark();
        self.released = true;
        loop {
            match self.resumed.try_pop() {
                Ok(()) => {
                    self.released = false;
                    return Ok(());
                }
                Err(QueueEmpty { .. }) => thread::sleep(CONTROL_POLL_WAIT),
            }
        }
    }
}

#[cfg(feature = "test-support")]
impl WorkerAuditGate {
    fn hold(&mut self, stop: &mut Consumer<()>) -> bool {
        self.held
            .try_push(())
            .expect("audit controller consumes bounded hold acknowledgement");
        loop {
            match self.release.try_pop() {
                Ok(()) => return true,
                Err(QueueEmpty { .. }) if stop.try_pop().is_ok() => return false,
                Err(QueueEmpty { .. }) => thread::park_timeout(AUDIT_WORKER_WAIT),
            }
        }
    }

    fn acknowledge_resumed(&mut self) {
        self.resumed
            .try_push(())
            .expect("audit controller consumes bounded release acknowledgement");
    }
}

#[derive(Clone, Copy)]
struct PendingBlock {
    generation: SourceGeneration,
    start_frame: SourceFrame,
    frames: u32,
    end_of_region: bool,
    native_decoder_sanitized_samples: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingSeek {
    generation: SourceGeneration,
    frame: SourceFrame,
}

/// Complete per-source decode state prepared before any worker thread starts.
struct SourceJob<R: Read + Seek> {
    commands: Consumer<WorkerCommand>,
    events: Producer<NativeSourceWorkerEvent>,
    provider: HostChunkProvider,
    decoder: NativeWaveDecoder<R>,
    planar_staging: Box<[f32]>,
    generation: SourceGeneration,
    pending: Option<PendingBlock>,
    pending_seek: Option<PendingSeek>,
    end_submitted: bool,
    source_ready_sent: bool,
    sanitation: u64,
    render_wait: Duration,
    terminated: Option<NativeSourceWorkerExit>,
    #[cfg(feature = "test-support")]
    audit_gate: Option<WorkerAuditGate>,
    #[cfg(feature = "test-support")]
    audit_resume_after_submit: bool,
    #[cfg(feature = "test-support")]
    audit_hold_after_submit: bool,
}

struct UnstartedNativeSource<R: Read + Seek> {
    command_sender: Producer<WorkerCommand>,
    event_receiver: Consumer<NativeSourceWorkerEvent>,
    job: SourceJob<R>,
    consumer: PcmSourceConsumer,
    resources: NativeSourceResourceReport,
    caps: NativeSourcePrepareCaps,
    initial_generation: SourceGeneration,
    region: NativeWaveRegion,
}

/// Non-render endpoint for bounded native seek/wake commands and worker events.
pub struct NativeSourceController {
    commands: Producer<WorkerCommand>,
    events: Consumer<NativeSourceWorkerEvent>,
    observed_sanitation: u64,
    terminal_exit: Option<NativeSourceWorkerExit>,
    next_requested_generation: SourceGeneration,
    region: NativeWaveRegion,
    worker: Thread,
    _not_sync: PhantomData<Cell<()>>,
}

impl NativeSourceController {
    /// Cumulative native decoder replacements observed through bounded prepared telemetry.
    #[must_use]
    pub fn native_decoder_sanitized_samples(&self) -> u64 {
        self.observed_sanitation
    }

    /// Exact terminal reason after this controller has consumed its job Terminal event.
    #[must_use]
    pub const fn worker_exit(&self) -> Option<NativeSourceWorkerExit> {
        self.terminal_exit
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
        validate_seek_frame(self.region, frame)
            .map_err(|_| NativeSourceWorkerControlError::RegionOutOfBounds)?;
        self.try_send(WorkerCommand::Seek { generation, frame })?;
        self.next_requested_generation = generation;
        Ok(())
    }

    /// Wake a worker waiting for ring capacity after an off-render consumer-drain notification.
    pub fn try_wake(&mut self) -> Result<(), NativeSourceWorkerControlError> {
        self.try_send(WorkerCommand::Wake)
    }

    /// Ask the test-support worker gate to hold after its next submitted block.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn hold_worker_for_audit(&mut self) -> Result<(), NativeSourceWorkerControlError> {
        self.try_send(WorkerCommand::AuditHold)
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
                Err(QueueEmpty { .. }) if self.terminal_exit.is_some() => {
                    return Err(NativeSourceWorkerControlError::Stopped);
                }
                Err(QueueEmpty { .. }) => thread::sleep(CONTROL_POLL_WAIT),
            }
        }
    }

    fn try_send(&mut self, command: WorkerCommand) -> Result<(), NativeSourceWorkerControlError> {
        if self.terminal_exit.is_some() {
            return Err(NativeSourceWorkerControlError::Stopped);
        }
        match self.commands.try_push(command) {
            Ok(()) => {
                self.worker.unpark();
                Ok(())
            }
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
                ..
            } => native_decoder_sanitized_samples,
        };
        self.observed_sanitation = self.observed_sanitation.max(value);
        if let NativeSourceWorkerEvent::Terminal { exit, .. } = event {
            self.terminal_exit = Some(exit);
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
            if let Some(join) = self.join.as_ref() {
                join.thread().unpark();
            }
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
    let worker_resources = resources.worker;
    let resources = base_source_resources(resources).expect("set-of-one worker fold is reversible");
    Ok(PreparedNativeSource {
        controller,
        consumer,
        resources,
        worker_resources,
        worker,
    })
}

/// Prepare a native source with a deterministic audit-only worker hold/release gate.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn prepare_native_source_with_audit_gate<S: NativeSourceResolver>(
    resolver: &mut S,
    request: NativeSourcePrepareRequest,
    caps: NativeSourcePrepareCaps,
) -> Result<(PreparedNativeSource, NativeWorkerAuditGate), NativeSourcePrepareError> {
    let (held_sender, held_receiver) = bounded_spsc(
        NonZeroUsize::new(1).expect("one hold acknowledgement"),
        QueueGeneration(14),
    )
    .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let (release_sender, release_receiver) = bounded_spsc(
        NonZeroUsize::new(1).expect("one hold release"),
        QueueGeneration(15),
    )
    .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let (resumed_sender, resumed_receiver) = bounded_spsc(
        NonZeroUsize::new(1).expect("one release acknowledgement"),
        QueueGeneration(16),
    )
    .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let (controller, worker, consumer, resources) = prepare_native_source_parts_with_audit_gate(
        resolver,
        request,
        caps,
        Some(WorkerAuditGate {
            held: held_sender,
            release: release_receiver,
            resumed: resumed_sender,
        }),
    )?;
    let audit_worker = controller.worker.clone();
    let worker_resources = resources.worker;
    let resources = base_source_resources(resources).expect("set-of-one worker fold is reversible");
    Ok((
        PreparedNativeSource {
            controller,
            consumer,
            resources,
            worker_resources,
            worker,
        },
        NativeWorkerAuditGate {
            held: held_receiver,
            release: release_sender,
            resumed: resumed_receiver,
            worker: audit_worker,
            released: false,
            _not_sync: PhantomData,
        },
    ))
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
    prepare_native_source_parts_with_audit_gate(resolver, request, caps, None)
}

fn prepare_native_source_parts_with_audit_gate<S: NativeSourceResolver>(
    resolver: &mut S,
    request: NativeSourcePrepareRequest,
    caps: NativeSourcePrepareCaps,
    audit_gate: Option<WorkerAuditGate>,
) -> Result<
    (
        NativeSourceController,
        NativeSourceWorker,
        PcmSourceConsumer,
        NativeSourceResourceReport,
    ),
    NativeSourcePrepareError,
> {
    let prepared = prepare_native_source_job(resolver, request, caps, audit_gate)?;
    start_native_worker(prepared)
}

fn prepare_native_source_job<S: NativeSourceResolver>(
    resolver: &mut S,
    request: NativeSourcePrepareRequest,
    caps: NativeSourcePrepareCaps,
    #[cfg_attr(not(feature = "test-support"), allow(unused_variables))] audit_gate: Option<
        WorkerAuditGate,
    >,
) -> Result<UnstartedNativeSource<S::Asset>, NativeSourcePrepareError> {
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
    validate_region(metadata, request.region)
        .map_err(|_| NativeSourcePrepareError::RegionOutOfBounds)?;
    let report = source_resource_report(metadata, request.ring_config, caps)?;
    let (frames_per_read, decoder_read_scratch_bytes) =
        worker_decode_buffer_shape(metadata, request.ring_config, caps)?;
    debug_assert_eq!(
        report.decoder_read_scratch_bytes,
        decoder_read_scratch_bytes
    );
    let quantum = usize::try_from(request.ring_config.quantum_frames.0)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let quantum = NonZeroUsize::new(quantum).ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let decoder =
        NativeWaveDecoder::prepare(asset.reader, metadata, request.region, frames_per_read)
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
    let (event_sender, event_receiver) =
        bounded_spsc(WORKER_EVENT_QUEUE_ITEMS, QueueGeneration(12))
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let initial_generation = request.ring_config.initial_generation;
    let render_wait = worker_render_wait(
        ring_report.transfer_block_count,
        request.ring_config.quantum_frames.0,
        metadata.sample_rate_hz,
    );
    Ok(UnstartedNativeSource {
        command_sender,
        event_receiver,
        job: SourceJob {
            commands: command_receiver,
            events: event_sender,
            provider,
            decoder,
            planar_staging: planar_staging.into_boxed_slice(),
            generation: initial_generation,
            pending: None,
            pending_seek: None,
            end_submitted: false,
            source_ready_sent: false,
            sanitation: 0,
            render_wait,
            terminated: None,
            #[cfg(feature = "test-support")]
            audit_gate,
            #[cfg(feature = "test-support")]
            audit_resume_after_submit: false,
            #[cfg(feature = "test-support")]
            audit_hold_after_submit: false,
        },
        consumer,
        resources: report,
        caps,
        initial_generation,
        region: request.region,
    })
}

fn start_native_worker<R: Read + Seek + Send + 'static>(
    prepared: UnstartedNativeSource<R>,
) -> Result<
    (
        NativeSourceController,
        NativeSourceWorker,
        PcmSourceConsumer,
        NativeSourceResourceReport,
    ),
    NativeSourcePrepareError,
> {
    let UnstartedNativeSource {
        command_sender,
        event_receiver,
        job,
        consumer,
        resources,
        caps,
        initial_generation,
        region,
    } = prepared;
    let (worker, worker_resources, worker_thread) =
        start_native_workers(vec![job], Some((resources, caps)))?;
    let resources = fold_worker_resources(resources, worker_resources)
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    Ok((
        native_source_controller(
            command_sender,
            event_receiver,
            initial_generation,
            region,
            worker_thread,
        ),
        worker,
        consumer,
        resources,
    ))
}

fn native_source_controller(
    commands: Producer<WorkerCommand>,
    events: Consumer<NativeSourceWorkerEvent>,
    initial_generation: SourceGeneration,
    region: NativeWaveRegion,
    worker: Thread,
) -> NativeSourceController {
    NativeSourceController {
        commands,
        events,
        observed_sanitation: 0,
        terminal_exit: None,
        next_requested_generation: initial_generation,
        region,
        worker,
        _not_sync: PhantomData,
    }
}

fn start_native_workers<R: Read + Seek + Send + 'static>(
    jobs: Vec<SourceJob<R>>,
    limits: Option<(NativeSourceResourceReport, NativeSourcePrepareCaps)>,
) -> Result<(NativeSourceWorker, NativeWorkerResourceReport, Thread), NativeSourcePrepareError> {
    if jobs.is_empty() {
        return Err(NativeSourcePrepareError::ResourceLimit);
    }
    let job_count = jobs.len();
    let job_layout = Layout::array::<SourceJob<R>>(job_count)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let stop_resources = exact_queue_resources::<()>(NonZeroUsize::new(1).expect("one stop"))?;
    let worker_resources = NativeWorkerResourceReport {
        stop_queue_bytes: stop_resources.total_bytes,
        stop_queue_items: 1,
        stop_queue_largest_allocation_bytes: stop_resources.largest_allocation_bytes,
        stop_queue_alignment_bytes: stop_resources.alignment_bytes,
        job_array_bytes: u64::try_from(job_layout.size())
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
        job_count: u64::try_from(job_count).map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
        job_array_alignment_bytes: u64::try_from(job_layout.align())
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?,
    };
    if let Some((base, caps)) = limits {
        let folded = fold_worker_resources(base, worker_resources)
            .ok_or(NativeSourcePrepareError::ResourceLimit)?;
        if folded.total_engine_owned_bytes > caps.max_total_engine_owned_bytes
            || folded.largest_allocation_bytes > caps.max_largest_allocation_bytes
        {
            return Err(NativeSourcePrepareError::ResourceLimit);
        }
    }
    let jobs = jobs.into_boxed_slice();
    let (stop_sender, stop_receiver) =
        bounded_spsc(NonZeroUsize::new(1).expect("one stop"), QueueGeneration(13))
            .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let join = thread::Builder::new()
        .name("miso-engine-source".to_owned())
        .spawn(move || run_workers(jobs, stop_receiver))
        .map_err(|_| NativeSourcePrepareError::WorkerStart)?;
    let worker_thread = join.thread().clone();
    Ok((
        NativeSourceWorker {
            join: Some(join),
            stopped: false,
            stop: stop_sender,
            _not_sync: PhantomData,
        },
        worker_resources,
        worker_thread,
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
    let Some(first_source) = model.sources.first() else {
        return Err(NativeSessionSourcePrepareFailure {
            diagnostics: vec![SourceDiagnostic::new(
                SourceDiagnosticCode::GraphBindingMismatch,
                SourceDiagnosticPath::for_sources_collection(),
                "native graph source-set preparation requires at least one source",
            )],
        });
    };
    let first_source_id = first_source.id.as_str();
    let mut diagnostics = Vec::new();
    let mut prepared_sources = Vec::with_capacity(model.sources.len());

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
        match prepare_native_source_job(resolver, request, caps.source, None) {
            Ok(prepared) => prepared_sources.push(prepared),
            Err(error) => diagnostics.push(native_prepare_diagnostic(source.id.as_str(), error)),
        }
    }
    if !diagnostics.is_empty() {
        sort_diagnostics(&mut diagnostics);
        return Err(NativeSessionSourcePrepareFailure { diagnostics });
    }

    let source_count =
        u64::try_from(prepared_sources.len()).expect("source vector length fits u64");
    let mut jobs = Vec::with_capacity(prepared_sources.len());
    let mut endpoints = Vec::with_capacity(prepared_sources.len());
    for prepared in prepared_sources {
        let UnstartedNativeSource {
            command_sender,
            event_receiver,
            job,
            consumer,
            resources,
            initial_generation,
            region,
            ..
        } = prepared;
        debug_assert_eq!(resources.worker, EMPTY_WORKER_RESOURCE_REPORT);
        jobs.push(job);
        endpoints.push((
            command_sender,
            event_receiver,
            consumer,
            resources,
            initial_generation,
            region,
        ));
    }
    let (worker, worker_resources, worker_thread) = match start_native_workers(jobs, None) {
        Ok(started) => started,
        Err(error) => {
            return Err(NativeSessionSourcePrepareFailure {
                diagnostics: vec![native_prepare_diagnostic(first_source_id, error)],
            });
        }
    };
    let mut worker = Some(worker);
    let mut controllers = Vec::with_capacity(endpoints.len());
    let mut graph_sources = Vec::with_capacity(endpoints.len());
    for (index, (commands, events, consumer, resources, initial_generation, region)) in
        endpoints.into_iter().enumerate()
    {
        controllers.push(native_source_controller(
            commands,
            events,
            initial_generation,
            region,
            worker_thread.clone(),
        ));
        let additional_overhead_bytes = resources
            .total_engine_owned_bytes
            .checked_sub(resources.ring.total_engine_owned_bytes)
            .expect("base source report includes ring");
        if index == 0 {
            graph_sources.push(SourceGraphSource::with_native_worker(
                consumer,
                resources.ring,
                additional_overhead_bytes,
                resources.largest_allocation_bytes,
                worker.take().expect("first source owns set worker"),
            ));
        } else {
            graph_sources.push(SourceGraphSource::new(
                consumer,
                resources.ring,
                additional_overhead_bytes,
                resources.largest_allocation_bytes,
            ));
        }
    }
    debug_assert!(worker.is_none());

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
                    SourceDiagnosticPath::for_source(first_source_id),
                    "compiled track/source mappings could not be sealed for graph binding",
                )],
            });
        }
    };
    let graph_resources = source_set.resource_report();
    let controller_records_bytes =
        retained_array_bytes::<NativeSourceController>(controllers.len())
            .ok_or_else(|| resource_failure(first_source_id))?;
    let session_runtime_bytes = session.resource_estimate().requested_runtime_bytes;
    let worker_bytes = worker_resources
        .total_engine_owned_bytes()
        .ok_or_else(|| resource_failure(first_source_id))?;
    let combined_runtime_bytes = match session_runtime_bytes
        .checked_add(graph_resources.overhead_bytes)
        .and_then(|total| total.checked_add(controller_records_bytes))
        .and_then(|total| total.checked_add(worker_bytes))
    {
        Some(total) => total,
        None => {
            return Err(resource_failure(first_source_id));
        }
    };
    let largest_allocation_bytes = session
        .resource_estimate()
        .single_allocation_bytes
        .max(graph_resources.largest_allocation_bytes)
        .max(controller_records_bytes)
        .max(worker_resources.largest_allocation_bytes());
    if graph_resources.pcm_payload_already_charged_bytes
        != session.resource_estimate().source_ring_bytes
        || combined_runtime_bytes > model.limits.memory_bytes
        || combined_runtime_bytes > caps.max_combined_runtime_bytes
        || largest_allocation_bytes > caps.max_largest_allocation_bytes
    {
        return Err(resource_failure(first_source_id));
    }
    Ok(NativeSessionPreparedSources {
        source_set,
        controllers,
        resources: NativeSessionSourceResourceReport {
            source_count,
            session_runtime_bytes,
            source_pcm_already_charged_bytes: graph_resources.pcm_payload_already_charged_bytes,
            source_overhead_bytes: graph_resources.overhead_bytes,
            controller_records_bytes,
            worker_bytes,
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
    let (_, decoder_read_scratch_bytes) = worker_decode_buffer_shape(metadata, ring_config, caps)?;
    let worker_planar_staging_bytes = u64::from(ring_config.quantum_frames.0)
        .checked_mul(u64::from(metadata.channel_count))
        .and_then(|samples| {
            samples.checked_mul(u64::try_from(core::mem::size_of::<f32>()).expect("f32 size"))
        })
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let worker_control_queue = exact_queue_resources::<WorkerCommand>(caps.control_queue_items)?;
    let worker_event_queue =
        exact_queue_resources::<NativeSourceWorkerEvent>(WORKER_EVENT_QUEUE_ITEMS)?;
    let total_engine_owned_bytes = ring
        .total_engine_owned_bytes
        .checked_add(decoder_read_scratch_bytes)
        .and_then(|total| total.checked_add(worker_planar_staging_bytes))
        .and_then(|total| total.checked_add(worker_control_queue.total_bytes))
        .and_then(|total| total.checked_add(worker_event_queue.total_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let largest_allocation_bytes = ring
        .largest_allocation_bytes
        .max(decoder_read_scratch_bytes)
        .max(worker_planar_staging_bytes)
        .max(worker_control_queue.largest_allocation_bytes)
        .max(worker_event_queue.largest_allocation_bytes);
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
        worker_event_queue_items: u64::try_from(WORKER_EVENT_QUEUE_ITEMS.get())
            .expect("usize fits u64"),
        worker_event_queue_largest_allocation_bytes: worker_event_queue.largest_allocation_bytes,
        worker_event_queue_alignment_bytes: worker_event_queue.alignment_bytes,
        worker: EMPTY_WORKER_RESOURCE_REPORT,
        total_engine_owned_bytes,
        largest_allocation_bytes,
    })
}

fn worker_decode_buffer_shape(
    metadata: NativeWaveMetadata,
    ring_config: PcmSourceRingConfig,
    caps: NativeSourcePrepareCaps,
) -> Result<(NonZeroUsize, u64), NativeSourcePrepareError> {
    const TARGET_READ_BYTES: u64 = 65_536;

    let quantum_frames = u64::from(ring_config.quantum_frames.0);
    let quantum_bytes = quantum_frames
        .checked_mul(u64::from(metadata.block_align_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    if quantum_bytes == 0 || quantum_bytes > caps.max_worker_read_scratch_bytes {
        return Err(NativeSourcePrepareError::ResourceLimit);
    }
    let target_bytes = caps
        .max_worker_read_scratch_bytes
        .min(TARGET_READ_BYTES)
        .max(quantum_bytes);
    let quanta_per_read = target_bytes / quantum_bytes;
    let frames_per_read = quantum_frames
        .checked_mul(quanta_per_read)
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let scratch_bytes = frames_per_read
        .checked_mul(u64::from(metadata.block_align_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let frames_per_read = usize::try_from(frames_per_read)
        .ok()
        .and_then(NonZeroUsize::new)
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    Ok((frames_per_read, scratch_bytes))
}

fn fold_worker_resources(
    mut base: NativeSourceResourceReport,
    worker: NativeWorkerResourceReport,
) -> Option<NativeSourceResourceReport> {
    base.total_engine_owned_bytes = base
        .total_engine_owned_bytes
        .checked_add(worker.total_engine_owned_bytes()?)?;
    base.largest_allocation_bytes = base
        .largest_allocation_bytes
        .max(worker.largest_allocation_bytes());
    base.worker = worker;
    Some(base)
}

fn base_source_resources(
    mut folded: NativeSourceResourceReport,
) -> Option<NativeSourceResourceReport> {
    folded.total_engine_owned_bytes = folded
        .total_engine_owned_bytes
        .checked_sub(folded.worker.total_engine_owned_bytes()?)?;
    folded.worker = EMPTY_WORKER_RESOURCE_REPORT;
    folded.largest_allocation_bytes = folded
        .ring
        .largest_allocation_bytes
        .max(folded.decoder_read_scratch_bytes)
        .max(folded.worker_planar_staging_bytes)
        .max(folded.worker_control_queue_largest_allocation_bytes)
        .max(folded.worker_event_queue_largest_allocation_bytes);
    Some(folded)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Idle {
    Progress,
    WaitingForRender,
    WaitingForCommand,
}

fn worker_render_wait(
    transfer_block_count: u64,
    quantum_frames: u32,
    sample_rate_hz: SampleRateHz,
) -> Duration {
    let ring_frames = u128::from(transfer_block_count) * u128::from(quantum_frames);
    let half_playback_nanos = ring_frames * 1_000_000_000 / u128::from(sample_rate_hz.0) / 2;
    let bounded_nanos = half_playback_nanos.clamp(MIN_RENDER_WAIT_NANOS, MAX_RENDER_WAIT_NANOS);
    Duration::from_nanos(u64::try_from(bounded_nanos).expect("bounded render wait fits u64"))
}

fn service_job<R: Read + Seek>(
    job: &mut SourceJob<R>,
    #[cfg_attr(not(feature = "test-support"), allow(unused_variables))] stop: &mut Consumer<()>,
) -> Result<Idle, NativeSourceWorkerExit> {
    let mut idle = Idle::WaitingForCommand;
    let mut latest_seek = None;
    for _ in 0..job.commands.capacity() {
        let Ok(command) = job.commands.try_pop() else {
            break;
        };
        idle = Idle::Progress;
        match command {
            WorkerCommand::Seek {
                generation: requested,
                frame,
            } => {
                let observed = PendingSeek {
                    generation: requested,
                    frame,
                };
                if latest_seek
                    .is_none_or(|latest: PendingSeek| observed.generation > latest.generation)
                {
                    latest_seek = Some(observed);
                }
            }
            WorkerCommand::Wake => {}
            WorkerCommand::SnapshotSanitation => publish_worker_event(
                &mut job.events,
                NativeSourceWorkerEvent::SanitationSnapshot {
                    native_decoder_sanitized_samples: job.sanitation,
                },
            ),
            #[cfg(feature = "test-support")]
            WorkerCommand::AuditHold => job.audit_hold_after_submit = true,
        }
    }
    if let Some(latest) = latest_seek {
        job.pending_seek = Some(latest);
    }
    if let Some(seek) = job.pending_seek {
        match job.provider.try_seek(SourceCommand::Seek {
            generation: seek.generation,
            frame: seek.frame,
        }) {
            Ok(()) => {
                job.decoder
                    .seek_to_source_frame(seek.frame)
                    .map_err(NativeSourceWorkerExit::DecodeFailed)?;
                job.generation = seek.generation;
                job.pending = None;
                job.end_submitted = false;
                job.pending_seek = None;
            }
            Err(SourceSeekError::Backpressure { .. }) => {
                return Ok(if idle == Idle::Progress {
                    Idle::Progress
                } else {
                    Idle::WaitingForRender
                });
            }
            Err(error) => return Err(NativeSourceWorkerExit::SeekFailed(error)),
        }
    }
    if let Some(block) = job.pending.take() {
        match job.provider.submit_native_planar(
            block.generation,
            block.start_frame,
            &job.planar_staging,
            block.frames,
            block.end_of_region,
            block.native_decoder_sanitized_samples,
        ) {
            Ok(_) => {
                #[cfg(feature = "test-support")]
                if job.audit_resume_after_submit {
                    if let Some(gate) = job.audit_gate.as_mut() {
                        gate.acknowledge_resumed();
                    }
                    job.audit_resume_after_submit = false;
                }
                #[cfg(feature = "test-support")]
                if job.audit_hold_after_submit {
                    let Some(gate) = job.audit_gate.as_mut() else {
                        return Err(NativeSourceWorkerExit::Stopped);
                    };
                    if !gate.hold(stop) {
                        return Err(NativeSourceWorkerExit::Stopped);
                    }
                    job.audit_hold_after_submit = false;
                    job.audit_resume_after_submit = true;
                }
                job.end_submitted = block.end_of_region;
                if !job.source_ready_sent {
                    publish_worker_event(
                        &mut job.events,
                        NativeSourceWorkerEvent::SourceReady {
                            native_decoder_sanitized_samples: job.sanitation,
                        },
                    );
                    job.source_ready_sent = true;
                }
                return Ok(Idle::Progress);
            }
            Err(HostChunkError::Full { .. }) => {
                job.pending = Some(block);
                return Ok(if idle == Idle::Progress {
                    Idle::Progress
                } else {
                    Idle::WaitingForRender
                });
            }
            Err(error) => return Err(NativeSourceWorkerExit::SubmitFailed(error)),
        }
    }
    if job.end_submitted {
        return Ok(idle);
    }
    let start_frame = job.decoder.next_source_frame();
    let channels = usize::from(job.decoder.metadata().channel_count);
    let frames = job.planar_staging.len() / channels;
    let decoded = job
        .decoder
        .decode_planar(&mut job.planar_staging, frames)
        .map_err(NativeSourceWorkerExit::DecodeFailed)?;
    job.sanitation = job.sanitation.max(decoded.sanitized_sample_count);
    job.pending = Some(PendingBlock {
        generation: job.generation,
        start_frame,
        frames: decoded.decoded_frames,
        end_of_region: decoded.end_of_region,
        native_decoder_sanitized_samples: job.sanitation,
    });
    Ok(Idle::Progress)
}

fn publish_terminal<R: Read + Seek>(job: &mut SourceJob<R>, exit: NativeSourceWorkerExit) {
    publish_worker_event(
        &mut job.events,
        NativeSourceWorkerEvent::Terminal {
            native_decoder_sanitized_samples: job.sanitation,
            exit,
        },
    );
    job.terminated = Some(exit);
}

fn run_workers<R: Read + Seek>(
    mut jobs: Box<[SourceJob<R>]>,
    mut stop: Consumer<()>,
) -> NativeSourceWorkerExit {
    loop {
        if stop.try_pop().is_ok() {
            for job in &mut jobs {
                if job.terminated.is_none() {
                    publish_terminal(job, NativeSourceWorkerExit::Stopped);
                }
            }
            return NativeSourceWorkerExit::Stopped;
        }
        let mut idle = Idle::WaitingForCommand;
        let mut render_wait = Duration::MAX;
        for job in &mut jobs {
            if job.terminated.is_some() {
                continue;
            }
            match service_job(job, &mut stop) {
                Ok(Idle::Progress) => idle = Idle::Progress,
                Ok(Idle::WaitingForRender) => {
                    if idle != Idle::Progress {
                        idle = Idle::WaitingForRender;
                    }
                    render_wait = render_wait.min(job.render_wait);
                }
                Ok(Idle::WaitingForCommand) => {}
                Err(NativeSourceWorkerExit::Stopped) => {
                    for live in &mut jobs {
                        if live.terminated.is_none() {
                            publish_terminal(live, NativeSourceWorkerExit::Stopped);
                        }
                    }
                    return NativeSourceWorkerExit::Stopped;
                }
                Err(exit) => {
                    publish_terminal(job, exit);
                    idle = Idle::Progress;
                }
            }
        }
        match idle {
            Idle::Progress => {}
            Idle::WaitingForRender => thread::park_timeout(render_wait),
            Idle::WaitingForCommand => thread::park(),
        }
    }
}

fn publish_worker_event(
    events: &mut Producer<NativeSourceWorkerEvent>,
    event: NativeSourceWorkerEvent,
) {
    events.try_push(event).expect(
        "three event slots reserve Terminal behind SourceReady and one synchronous Snapshot",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor, SeekFrom};

    use crate::{HostPlanarChunk, PcmSourceRing, QuantumFrames, SourceReadReport};
    use miso_engine_session::{CompileCaps, StableId, compile_session, parse_session_toml};

    #[test]
    fn native_worker_idle_paths_do_not_use_active_spin_primitives() {
        let source = include_str!("native_source.rs");
        let yield_primitive = ["yield", "_now"].concat();
        let spin_primitive = ["spin", "_loop"].concat();
        assert!(
            !source.contains(&yield_primitive),
            "native worker/control paths must park or sleep instead of yielding"
        );
        assert!(
            !source.contains(&spin_primitive),
            "native worker/control paths must park or sleep instead of spinning"
        );
    }

    #[test]
    fn render_wait_is_half_the_prepared_ring_and_bounded() {
        assert_eq!(
            worker_render_wait(8, 128, SampleRateHz(48_000)),
            Duration::from_nanos(10_666_666)
        );
        assert_eq!(
            worker_render_wait(1, 1, SampleRateHz(96_000)),
            Duration::from_millis(1)
        );
        assert_eq!(
            worker_render_wait(64, 128, SampleRateHz(44_100)),
            Duration::from_millis(20)
        );
    }

    #[test]
    fn prepared_source_job_is_inert_until_the_single_start_boundary() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(&[0.25; 4], b"exact-identity");
        let mut prepared =
            prepare_native_source_job(&mut native_resolver, request(region), caps(), None)
                .expect("prepare without spawning");

        assert!(prepared.event_receiver.try_pop().is_err());
        assert!(prepared.job.pending.is_none());
        assert!(prepared.job.pending_seek.is_none());
        assert!(!prepared.job.end_submitted);
        assert!(!prepared.job.source_ready_sent);
        assert_eq!(prepared.job.sanitation, 0);
        assert_eq!(prepared.resources.worker, EMPTY_WORKER_RESOURCE_REPORT);
        assert!(matches!(
            start_native_workers::<Cursor<Vec<u8>>>(Vec::new(), None),
            Err(NativeSourcePrepareError::ResourceLimit)
        ));
    }

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

        let mut rounded = resolver(&[0.0; 4], b"exact-identity");
        let mut rounded_caps = caps();
        rounded_caps.max_worker_read_scratch_bytes = 50;
        let rounded = prepare_native_source_job(&mut rounded, request(region), rounded_caps, None)
            .expect("whole-quantum read buffer below cap");
        assert_eq!(rounded.resources.decoder_read_scratch_bytes, 48);
        assert_eq!(rounded.job.planar_staging.len(), 4);
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
        assert_eq!(report.decoder_read_scratch_bytes, 64);
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
                native_decoder_sanitized_samples: 2,
                exit: NativeSourceWorkerExit::Stopped,
            }
        ));
        assert_eq!(controller.native_decoder_sanitized_samples(), 2);
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn reserved_event_slot_preserves_terminal_after_ready_and_snapshot() {
        let samples: Vec<f32> = (0_u16..20).map(f32::from).collect();
        let mut native_resolver = resolver(&samples, b"exact-identity");
        let (prepared, mut gate) = prepare_native_source_with_audit_gate(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 20,
            }),
            caps(),
        )
        .expect("prepare with reserved terminal event slot");
        let PreparedNativeSource {
            mut controller,
            mut consumer,
            mut worker,
            ..
        } = prepared;

        controller
            .hold_worker_for_audit()
            .expect("request deterministic worker hold");
        let _ = read_one(&mut consumer);
        gate.wait_until_held()
            .expect("worker held after source ready");
        controller
            .try_send(WorkerCommand::SnapshotSanitation)
            .expect("queue the one synchronous snapshot");
        let _ = read_one(&mut consumer);
        gate.release_and_wait().expect("snapshot command consumed");

        assert_eq!(
            worker.stop_and_join().expect("prompt retirement join"),
            NativeSourceWorkerExit::Stopped
        );
        assert!(matches!(
            controller.wait_for_event().expect("source ready"),
            NativeSourceWorkerEvent::SourceReady { .. }
        ));
        assert!(matches!(
            controller.wait_for_event().expect("snapshot"),
            NativeSourceWorkerEvent::SanitationSnapshot { .. }
        ));
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal { .. }
        ));
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
                native_decoder_sanitized_samples: 3,
                exit: NativeSourceWorkerExit::Stopped,
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
        let base = base_source_resources(report).expect("remove folded worker resources");
        assert_eq!(base.worker, EMPTY_WORKER_RESOURCE_REPORT);
        assert_eq!(
            fold_worker_resources(base, report.worker).expect("refold worker resources"),
            report
        );
        let control = exact_queue_resources::<WorkerCommand>(caps().control_queue_items)
            .expect("control queue layout");
        let events = exact_queue_resources::<NativeSourceWorkerEvent>(WORKER_EVENT_QUEUE_ITEMS)
            .expect("event queue layout");
        let stop = exact_queue_resources::<()>(NonZeroUsize::new(1).expect("one stop"))
            .expect("stop queue layout");
        assert_eq!(
            size_of::<PendingSeek>(),
            size_of::<SourceGeneration>() + size_of::<SourceFrame>()
        );
        assert!(!core::mem::needs_drop::<PendingSeek>());
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
        assert_eq!(report.worker_event_queue_items, 3);
        assert_eq!(
            report.worker_event_queue_largest_allocation_bytes,
            events.largest_allocation_bytes
        );
        assert_eq!(
            report.worker_event_queue_alignment_bytes,
            events.alignment_bytes
        );
        assert_eq!(report.worker.stop_queue_bytes, stop.total_bytes);
        assert_eq!(
            report.worker.stop_queue_largest_allocation_bytes,
            stop.largest_allocation_bytes
        );
        assert_eq!(
            report.worker.stop_queue_alignment_bytes,
            stop.alignment_bytes
        );
        let job_layout = Layout::array::<SourceJob<Cursor<Vec<u8>>>>(1).expect("one job layout");
        assert_eq!(report.worker.job_count, 1);
        assert_eq!(
            report.worker.job_array_bytes,
            u64::try_from(job_layout.size()).expect("job bytes")
        );
        assert_eq!(
            report.worker.job_array_alignment_bytes,
            u64::try_from(job_layout.align()).expect("job alignment")
        );
        let exact_largest = report
            .ring
            .largest_allocation_bytes
            .max(report.decoder_read_scratch_bytes)
            .max(report.worker_planar_staging_bytes)
            .max(control.largest_allocation_bytes)
            .max(events.largest_allocation_bytes)
            .max(stop.largest_allocation_bytes)
            .max(report.worker.job_array_bytes);
        assert_eq!(report.largest_allocation_bytes, exact_largest);
        #[cfg(feature = "test-support")]
        {
            let layout =
                native_source_allocation_layout(request(region).ring_config, caps(), report)
                    .expect("exact folded allocation layout");
            assert_eq!(
                layout
                    .iter()
                    .try_fold(0_u64, |total, entry| {
                        total.checked_add(
                            entry
                                .requested_size_bytes
                                .checked_mul(entry.count)
                                .expect("layout category bytes"),
                        )
                    })
                    .expect("layout sum"),
                report.total_engine_owned_bytes
            );
            assert_eq!(
                layout
                    .iter()
                    .map(|entry| entry.requested_size_bytes)
                    .max()
                    .unwrap_or(0),
                report.largest_allocation_bytes
            );
            assert!(layout.iter().any(|entry| {
                entry.category == "worker.job_array"
                    && entry.alignment_bytes == report.worker.job_array_alignment_bytes
            }));
        }
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

    #[cfg(feature = "test-support")]
    #[test]
    fn single_worker_seek_resumes_contiguously_at_the_exact_frame() {
        let samples: Vec<f32> = (0_u16..20).map(f32::from).collect();
        let mut native_resolver = resolver(&samples, b"exact-identity");
        let (prepared, mut gate) = prepare_native_source_with_audit_gate(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 20,
            }),
            caps(),
        )
        .expect("prepare");
        let PreparedNativeSource {
            mut controller,
            mut consumer,
            mut worker,
            ..
        } = prepared;
        controller.wait_for_event().expect("ready");
        controller
            .hold_worker_for_audit()
            .expect("hold after next submission");
        let (initial, initial_report) = read_one(&mut consumer);
        gate.wait_until_held().expect("worker held");

        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(12),
            })
            .expect("single seek");
        controller
            .hold_worker_for_audit()
            .expect("hold after first seek block");
        let (while_held, held_report) = read_one(&mut consumer);
        gate.release_and_wait().expect("first seek block submitted");
        gate.wait_until_held().expect("worker held a second time");

        let (first, first_report) = read_one(&mut consumer);
        gate.release_and_wait()
            .expect("second seek block submitted");
        let (second, second_report) = read_one(&mut consumer);

        assert_eq!(initial, [0.0, 1.0, 2.0, 3.0]);
        assert_eq!(initial_report.active_generation, SourceGeneration(1));
        assert_eq!(while_held, [4.0, 5.0, 6.0, 7.0]);
        assert_eq!(held_report.active_generation, SourceGeneration(1));
        assert_eq!(first, [12.0, 13.0, 14.0, 15.0]);
        assert_eq!(first_report.active_generation, SourceGeneration(2));
        assert_eq!(first_report.copied_frames, 4);
        assert!(!first_report.end_of_region);
        assert_eq!(second, [16.0, 17.0, 18.0, 19.0]);
        assert_eq!(second_report.copied_frames, 4);
        assert!(second_report.end_of_region);
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm() {
        let mut samples: Vec<f32> = (0_u16..28).map(f32::from).collect();
        samples[13] = f32::NAN;
        samples[20] = f32::INFINITY;
        let mut native_resolver = resolver(&samples, b"exact-identity");
        let mut seek_caps = caps();
        seek_caps.control_queue_items = NonZeroUsize::new(4).expect("four commands");
        let mut seek_request = request(NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 28,
        });
        seek_request.ring_config.frame_capacity = 4;
        let (prepared, mut gate) =
            prepare_native_source_with_audit_gate(&mut native_resolver, seek_request, seek_caps)
                .expect("prepare");
        let PreparedNativeSource {
            mut controller,
            mut consumer,
            mut worker,
            ..
        } = prepared;
        assert!(matches!(
            controller.wait_for_event().expect("ready"),
            NativeSourceWorkerEvent::SourceReady { .. }
        ));

        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(8),
            })
            .expect("occupy provider seek slot");
        sync_worker(&mut controller);
        sync_worker(&mut controller);

        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(3),
                frame: SourceFrame(13),
            })
            .expect("retained intermediate seek");
        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(4),
                frame: SourceFrame(20),
            })
            .expect("replacement latest seek");
        sync_worker(&mut controller);
        sync_worker(&mut controller);
        assert!(controller.events.try_pop().is_err());
        controller.try_wake().expect("worker remains live");
        sync_worker(&mut controller);
        controller
            .hold_worker_for_audit()
            .expect("hold after latest first block");
        sync_worker(&mut controller);

        let (while_pending, pending_report) = read_one(&mut consumer);

        gate.wait_until_held()
            .expect("latest exact-frame block submitted");
        let (latest, latest_report) = read_one(&mut consumer);

        gate.release_and_wait()
            .expect("latest contiguous block submitted");
        let (contiguous, end_report) = read_one(&mut consumer);

        assert_eq!(while_pending, [0.0; 4]);
        assert_eq!(pending_report.active_generation, SourceGeneration(2));
        assert_eq!(pending_report.copied_frames, 0);
        assert_eq!(latest, [0.0, 21.0, 22.0, 23.0]);
        assert_eq!(latest_report.active_generation, SourceGeneration(4));
        assert_eq!(latest_report.copied_frames, 4);
        assert!(!latest_report.end_of_region);
        assert_eq!(contiguous, [24.0, 25.0, 26.0, 27.0]);
        assert_eq!(end_report.active_generation, SourceGeneration(4));
        assert_eq!(end_report.copied_frames, 4);
        assert!(end_report.end_of_region);
        assert_eq!(consumer.telemetry().stale_generation_discard_count, 1);
        assert_eq!(consumer.telemetry().native_decoder_sanitized_samples, 1);
        assert_eq!(
            controller
                .snapshot_native_decoder_sanitized_samples()
                .expect("latest sanitation"),
            1
        );
        assert_eq!(
            worker.stop_and_join().expect("stop live worker"),
            NativeSourceWorkerExit::Stopped
        );
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal {
                native_decoder_sanitized_samples: 1,
                exit: NativeSourceWorkerExit::Stopped,
            }
        ));
    }

    #[test]
    fn seek_continuation_pcm_reads_are_preceded_by_audit_acknowledgements() {
        let source = include_str!("native_source.rs");
        let cases = [
            (
                "fn single_worker_seek_resumes_contiguously_at_the_exact_frame()",
                ["let (first,", "let (second,"],
            ),
            (
                "fn worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm()",
                ["let (latest,", "let (contiguous,"],
            ),
        ];
        for (function, reads) in cases {
            let start = source.find(function).expect("seek continuation test");
            let tail = &source[start..];
            let end = tail.find("\n    #[test]\n").unwrap_or(tail.len());
            let body = &tail[..end];
            for read in reads {
                let read_offset = body.find(read).expect("nonzero PCM read");
                let prefix = &body[..read_offset];
                let previous_end = prefix.rfind(';').expect("preceding statement");
                let previous_start = prefix[..previous_end]
                    .rfind(';')
                    .map_or(0, |offset| offset + 1);
                let preceding = &prefix[previous_start..=previous_end];
                assert!(preceding.contains("gate."));
                assert!(!preceding.contains("sync_worker"));
                assert!(!preceding.contains("snapshot_native_decoder"));
            }
        }
        let held_reads = [
            (
                "fn single_worker_seek_resumes_contiguously_at_the_exact_frame()",
                "let (initial,",
            ),
            (
                "fn single_worker_seek_resumes_contiguously_at_the_exact_frame()",
                "let (while_held,",
            ),
            (
                "fn single_worker_seek_resumes_contiguously_at_the_exact_frame()",
                "let (first,",
            ),
            (
                "fn worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm()",
                "let (while_pending,",
            ),
            (
                "fn worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm()",
                "let (latest,",
            ),
        ];
        for (function, read) in held_reads {
            let start = source.find(function).expect("seek continuation test");
            let body = &source[start..];
            let read_offset = body.find(read).expect("read while hold may become active");
            let after_read = &body[read_offset..];
            let release = after_read
                .find("gate.release_and_wait()")
                .expect("held worker release");
            assert!(!after_read[..release].contains("assert"));
        }
    }

    #[test]
    fn provider_seek_admission_precedes_decoder_reposition() {
        let source = include_str!("native_source.rs");
        let start = source.find("fn service_job").expect("service_job");
        let tail = &source[start..];
        let end = tail.find("\nfn publish_terminal").expect("service_job end");
        let body = &tail[..end];
        let provider_seek = body
            .find("job.provider.try_seek")
            .expect("provider seek admission");
        let accepted = body[provider_seek..]
            .find("Ok(()) => {")
            .map(|offset| provider_seek + offset)
            .expect("provider accepted arm");
        let decoder_seek = body
            .find(".seek_to_source_frame(seek.frame)")
            .expect("decoder reposition");
        let backpressure = body
            .find("Err(SourceSeekError::Backpressure")
            .expect("provider backpressure arm");
        let seek_failed = body
            .find("NativeSourceWorkerExit::SeekFailed")
            .expect("non-backpressure seek failure");
        assert!(provider_seek < accepted && accepted < decoder_seek);
        assert!(decoder_seek < backpressure && backpressure < seek_failed);
        assert_eq!(body.matches(".seek_to_source_frame(").count(), 1);
        assert_eq!(
            body.matches("NativeSourceWorkerExit::SeekFailed").count(),
            1
        );
        assert!(
            !body[..accepted].contains(".seek_to_source_frame("),
            "decoder must retain its current position until provider seek admission succeeds"
        );
    }

    #[test]
    fn pending_seek_stops_without_render_drain_and_controller_backpressure_does_not_advance() {
        let (command_sender, mut command_receiver) = bounded_spsc(
            NonZeroUsize::new(1).expect("one command"),
            QueueGeneration(101),
        )
        .expect("command queue");
        let (_event_sender, event_receiver) = bounded_spsc(
            NonZeroUsize::new(1).expect("one event"),
            QueueGeneration(102),
        )
        .expect("event queue");
        let mut bounded_controller = NativeSourceController {
            commands: command_sender,
            events: event_receiver,
            observed_sanitation: 0,
            terminal_exit: None,
            next_requested_generation: SourceGeneration(1),
            region: NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 16,
            },
            worker: thread::current(),
            _not_sync: PhantomData,
        };
        bounded_controller.try_wake().expect("fill command queue");
        assert_eq!(
            bounded_controller.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(8),
            }),
            Err(NativeSourceWorkerControlError::Backpressure)
        );
        assert_eq!(command_receiver.try_pop(), Ok(WorkerCommand::Wake));
        bounded_controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(8),
            })
            .expect("rejected generation was not advanced");

        let mut native_resolver = resolver(&[0.0; 24], b"exact-identity");
        let mut seek_caps = caps();
        seek_caps.control_queue_items = NonZeroUsize::new(3).expect("three commands");
        let (mut controller, mut worker, _consumer, _) = prepare_native_source_parts(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 24,
            }),
            seek_caps,
        )
        .expect("prepare");
        controller.wait_for_event().expect("ready");
        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(4),
            })
            .expect("occupy provider slot");
        sync_worker(&mut controller);
        sync_worker(&mut controller);
        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(3),
                frame: SourceFrame(16),
            })
            .expect("retained seek");
        sync_worker(&mut controller);
        assert_eq!(
            worker.stop_and_join().expect("pending stop"),
            NativeSourceWorkerExit::Stopped
        );
        assert!(matches!(
            controller.wait_for_event().expect("terminal"),
            NativeSourceWorkerEvent::Terminal { .. }
        ));
    }

    #[test]
    fn decoder_failure_after_accepted_seek_keeps_typed_terminal() {
        let wave = float32_wave(&(0_u16..28).map(f32::from).collect::<Vec<_>>());
        let data_offset = 44_u64;
        let reader = FailAtRead {
            cursor: Cursor::new(wave),
            fail_at: data_offset + 20 * 4,
        };
        let mut native_resolver = FailingResolver {
            asset: Some(NativeResolvedAsset {
                observed_identity: b"exact-identity".to_vec(),
                reader,
            }),
        };
        let (mut controller, mut worker, _consumer, _) = prepare_native_source_parts(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 28,
            }),
            caps(),
        )
        .expect("prepare");
        controller.wait_for_event().expect("ready");
        controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(20),
            })
            .expect("seek to failing read");
        sync_worker(&mut controller);
        // The synchronous snapshot may itself consume Terminal and retain its exact exit.
        let exit = if let Some(exit) = controller.worker_exit() {
            exit
        } else {
            match controller.wait_for_event().expect("typed decoder terminal") {
                NativeSourceWorkerEvent::Terminal { exit, .. } => exit,
                event => panic!("expected typed decoder terminal, got {event:?}"),
            }
        };
        assert_eq!(
            exit,
            NativeSourceWorkerExit::DecodeFailed(NativeWaveError::Io(io::ErrorKind::Other))
        );
        assert_eq!(
            worker.stop_and_join().expect("stop isolated decoder job"),
            NativeSourceWorkerExit::Stopped
        );
    }

    #[test]
    fn terminal_event_carries_decode_failure_and_other_sources_continue() {
        let samples: Vec<f32> = (0_u16..20).map(f32::from).collect();
        let wave = float32_wave(&samples);
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 20,
        };
        let mut failed_resolver = FailingResolver {
            asset: Some(NativeResolvedAsset {
                observed_identity: b"exact-identity".to_vec(),
                reader: FailAtRead {
                    cursor: Cursor::new(wave.clone()),
                    fail_at: 44,
                },
            }),
        };
        let mut healthy_resolver = FailingResolver {
            asset: Some(NativeResolvedAsset {
                observed_identity: b"exact-identity".to_vec(),
                reader: FailAtRead {
                    cursor: Cursor::new(wave),
                    fail_at: u64::MAX,
                },
            }),
        };
        let failed = prepare_native_source_job(&mut failed_resolver, request(region), caps(), None)
            .expect("prepare failing job without starting");
        let healthy =
            prepare_native_source_job(&mut healthy_resolver, request(region), caps(), None)
                .expect("prepare healthy job without starting");
        let UnstartedNativeSource {
            command_sender: failed_commands,
            event_receiver: failed_events,
            job: failed_job,
            ..
        } = failed;
        let UnstartedNativeSource {
            command_sender: healthy_commands,
            event_receiver: healthy_events,
            job: healthy_job,
            mut consumer,
            initial_generation,
            region,
            ..
        } = healthy;
        let (mut worker, _worker_resources, worker_thread) =
            start_native_workers(vec![failed_job, healthy_job], None)
                .expect("start one set worker");
        let mut failed_controller = native_source_controller(
            failed_commands,
            failed_events,
            SourceGeneration(1),
            region,
            worker_thread.clone(),
        );
        let mut healthy_controller = native_source_controller(
            healthy_commands,
            healthy_events,
            initial_generation,
            region,
            worker_thread,
        );
        assert_eq!(
            failed_controller.worker.id(),
            healthy_controller.worker.id(),
            "both jobs must be serviced by the same set worker"
        );
        assert_eq!(failed_controller.worker_exit(), None);

        assert!(matches!(
            failed_controller.wait_for_event().expect("failed terminal"),
            NativeSourceWorkerEvent::Terminal {
                exit: NativeSourceWorkerExit::DecodeFailed(NativeWaveError::Io(
                    io::ErrorKind::Other
                )),
                ..
            }
        ));
        assert_eq!(
            failed_controller.worker_exit(),
            Some(NativeSourceWorkerExit::DecodeFailed(NativeWaveError::Io(
                io::ErrorKind::Other
            )))
        );
        assert!(matches!(
            healthy_controller.wait_for_event().expect("healthy ready"),
            NativeSourceWorkerEvent::SourceReady { .. }
        ));
        for quantum in 0_u16..4 {
            let (pcm, report) = read_one(&mut consumer);
            let first = quantum * 4;
            assert_eq!(
                pcm,
                [
                    f32::from(first),
                    f32::from(first + 1),
                    f32::from(first + 2),
                    f32::from(first + 3),
                ],
                "healthy peer quantum {quantum} after failed peer termination"
            );
            assert_eq!(report.copied_frames, 4);
            assert_eq!(report.underrun_frames, 0);
            healthy_controller
                .try_wake()
                .expect("healthy peer remains live after failed peer termination");
            sync_worker(&mut healthy_controller);
        }
        healthy_controller
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(4),
            })
            .expect("healthy peer remains seekable");
        sync_worker(&mut healthy_controller);

        assert_eq!(
            worker.stop_and_join().expect("stop set worker"),
            NativeSourceWorkerExit::Stopped
        );
        assert!(matches!(
            healthy_controller
                .wait_for_event()
                .expect("healthy terminal"),
            NativeSourceWorkerEvent::Terminal {
                exit: NativeSourceWorkerExit::Stopped,
                ..
            }
        ));
        assert_eq!(
            healthy_controller.worker_exit(),
            Some(NativeSourceWorkerExit::Stopped)
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
    fn compiled_multi_source_session_uses_one_exact_shared_worker() {
        let mut session_toml =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("session");
        session_toml.sources[0].mapping.region.length_samples = 4;
        let mut second = session_toml.sources[0].clone();
        second.id = StableId::parse("voice2").expect("second source ID");
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
        .expect("compiled two-source session");
        let asset = || NativeResolvedAsset {
            observed_identity: b"sha256:demo".to_vec(),
            reader: Cursor::new(stereo_float32_wave(&[0.0; 8])),
        };
        let mut resolver = SessionResolver {
            assets: vec![asset(), asset()],
            calls: 0,
        };
        let prepared = prepare_native_session_sources(&session, &mut resolver, session_caps())
            .expect("prepare one shared session worker");
        assert_eq!(resolver.calls, 2);
        assert_eq!(prepared.resources.source_count, 2);
        assert_eq!(prepared.controllers.len(), 2);
        assert_eq!(
            prepared.controllers[0].worker.id(),
            prepared.controllers[1].worker.id(),
            "all session controllers must target one shared worker"
        );
        let stop = exact_queue_resources::<()>(NonZeroUsize::new(1).expect("one stop"))
            .expect("shared stop resources");
        let jobs = Layout::array::<SourceJob<Cursor<Vec<u8>>>>(2).expect("two-job array");
        let expected_worker_bytes = stop
            .total_bytes
            .checked_add(u64::try_from(jobs.size()).expect("job-array bytes"))
            .expect("worker bytes");
        assert_eq!(prepared.resources.worker_bytes, expected_worker_bytes);
        assert_eq!(
            prepared.resources.combined_runtime_bytes,
            prepared
                .resources
                .session_runtime_bytes
                .checked_add(prepared.resources.source_overhead_bytes)
                .and_then(|total| {
                    total.checked_add(prepared.resources.controller_records_bytes)
                })
                .and_then(|total| total.checked_add(prepared.resources.worker_bytes))
                .expect("combined runtime")
        );

        let (source_set, mut controllers, _) = prepared.into_parts();
        for controller in &mut controllers {
            assert!(matches!(
                controller.wait_for_event().expect("shared worker ready"),
                NativeSourceWorkerEvent::SourceReady { .. }
            ));
        }
        drop(source_set);
        for controller in &mut controllers {
            assert!(matches!(
                controller.wait_for_event().expect("shared worker terminal"),
                NativeSourceWorkerEvent::Terminal {
                    exit: NativeSourceWorkerExit::Stopped,
                    ..
                }
            ));
            assert_eq!(
                controller.try_wake(),
                Err(NativeSourceWorkerControlError::Stopped)
            );
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn idle_decode_thread_cpu_is_bounded_and_one_thread_serves_a_set() {
        use core::num::NonZeroUsize;
        use miso_engine_core::realtime::{PlanarBufferMut, RenderEnvelope, RenderIo, RenderTime};
        use miso_engine_effect_contract::{LatencySamples, TailSamples};
        use miso_engine_graph::{
            DependencyLevel, GraphEdge, GraphEdgeId, GraphNode, GraphPortId, GraphPortKind,
            GraphResourceEstimate, GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec,
            PreparedGraphPlan, PreparedGraphPlanParts,
        };

        const SOURCE_COUNT: usize = 8;
        const QUANTUM: u32 = 128;
        const REGION_FRAMES: u64 = QUANTUM as u64 * 9;

        struct Noop;
        impl GraphRuntimeProcessor for Noop {
            fn process(
                &mut self,
                _block: miso_engine_graph::GraphBindingBlock<'_>,
            ) -> Result<(), miso_engine_core::realtime::RenderError> {
                Ok(())
            }
        }

        let mut session_toml =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("session");
        session_toml.limits.memory_bytes = 64 * 1024 * 1024;
        session_toml.sources[0].mapping.region.length_samples = REGION_FRAMES;
        let source_template = session_toml.sources[0].clone();
        let track_template = session_toml.tracks[0].clone();
        for index in 1..SOURCE_COUNT {
            let mut source = source_template.clone();
            source.id = StableId::parse(&format!("voice{index}")).expect("unique source stable ID");
            source.content.locator = format!("host:voice{index}");
            let mut track = track_template.clone();
            track.id = StableId::parse(&format!("vocal{index}")).expect("unique track stable ID");
            track.source_id = source.id.clone();
            session_toml.sources.push(source);
            session_toml.tracks.push(track);
        }
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
        .expect("compiled eight-source session");
        let wave = stereo_float32_wave(&vec![0.25; REGION_FRAMES as usize * 2]);
        let mut resolver = SessionResolver {
            assets: (0..SOURCE_COUNT)
                .map(|_| NativeResolvedAsset {
                    observed_identity: b"sha256:demo".to_vec(),
                    reader: Cursor::new(wave.clone()),
                })
                .collect(),
            calls: 0,
        };
        let prepared = prepare_native_session_sources(&session, &mut resolver, session_caps())
            .expect("prepare eight sources on one worker");
        assert_eq!(resolver.calls, SOURCE_COUNT);
        assert_eq!(prepared.controllers.len(), SOURCE_COUNT);
        let rust_thread_id = prepared.controllers[0].worker.id();
        assert!(
            prepared
                .controllers
                .iter()
                .all(|controller| controller.worker.id() == rust_thread_id),
            "all eight controllers must share one Rust worker thread ID"
        );
        let (source_set, mut controllers, _) = prepared.into_parts();
        for controller in &mut controllers {
            assert!(matches!(
                controller.wait_for_event().expect("source ready"),
                NativeSourceWorkerEvent::SourceReady { .. }
            ));
        }

        let tid = wait_for_only_source_worker_task(Duration::from_secs(10));
        let render_wait_ticks = measure_exclusive_source_worker_ticks(
            tid,
            Duration::from_millis(500),
            Duration::from_secs(10),
        );
        assert!(
            render_wait_ticks <= 5,
            "ring-full source-set worker consumed {render_wait_ticks} CPU ticks in 500 ms"
        );

        let envelope = RenderEnvelope {
            sample_rate: session.sample_rate(),
            quantum: session.quantum(),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two outputs"),
        };
        let model = session.normalized_model();
        assert_eq!(model.sources.len(), SOURCE_COUNT);
        assert_eq!(model.tracks.len(), SOURCE_COUNT);
        for (source, track) in model.sources.iter().zip(&model.tracks) {
            assert_eq!(
                track.source_id, source.id,
                "each unique compiled track must retain its intended unique source mapping"
            );
        }
        let input_nodes: Vec<_> = model
            .tracks
            .iter()
            .map(|track| GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track.id.as_str()).expect("compiled track ID"),
                stage: TrackStage::Input,
            })
            .collect();
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };
        assert_eq!(source_set.claims().len(), SOURCE_COUNT);
        assert!(
            source_set
                .claims()
                .iter()
                .map(|claim| &claim.node)
                .eq(input_nodes.iter()),
            "the source set must expose each of the eight compiled track-input claims exactly once"
        );
        let mut nodes: Vec<_> = input_nodes
            .iter()
            .cloned()
            .map(|id| GraphNode {
                id,
                latency: LatencySamples(0),
                tail: TailSamples::Finite(0),
            })
            .collect();
        nodes.push(GraphNode {
            id: output.clone(),
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
        });
        let edges: Vec<_> = input_nodes
            .iter()
            .enumerate()
            .map(|(index, input)| GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse(&format!("source-route{index}"))
                        .expect("route ID"),
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
                path: format!("$.routes[{index}]"),
            })
            .collect();
        let mut schedule = input_nodes.clone();
        schedule.push(output.clone());
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 124,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule.clone(),
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: input_nodes.clone(),
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
                effect_bank_count: 0,
                effect_bank_scratch_bytes: 0,
                effect_bank_runtime_buffer_bytes: 0,
                effect_bank_metadata_bytes: 0,
                builtin_bank_bytes: 0,
                builtin_bank_scratch_bytes: 0,
                builtin_bank_count: 0,
                largest_allocation_bytes: 0,
                incremental_plan_bytes: 0,
                session_plus_plan_bytes: 0,
            },
            envelope,
            required_bindings: schedule,
            routes: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
            effect_observations: Vec::new(),
        });
        let mut plan = match graph.bind_with_source_set(
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
            Err(failure) => panic!("eight-source bind failed: {}", failure.code),
        };
        let mut output_pcm = vec![0.0_f32; usize::from(2_u16) * QUANTUM as usize];
        let mut render = |block: u64| {
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(
                        &mut output_pcm,
                        2,
                        QUANTUM as usize,
                        QUANTUM as usize,
                    )
                    .expect("output shape"),
                },
                RenderTime {
                    absolute_sample: block * u64::from(QUANTUM),
                },
            )
            .expect("source-set render");
            let expected = if block < REGION_FRAMES / u64::from(QUANTUM) {
                2.0_f32
            } else {
                0.0_f32
            };
            assert!(
                output_pcm
                    .iter()
                    .all(|sample| sample.to_bits() == expected.to_bits()),
                "block {block} must mix all eight quarter-scale claims, then expose positive-zero EOF"
            );
        };
        render(0);
        for controller in &mut controllers {
            sync_worker(controller);
        }
        // The final snapshot is published before that job submits its pending block. A second
        // scheduler pass proves all eight final EOF blocks are in their rings before rendering.
        sync_worker(&mut controllers[0]);
        for block in 1..=9 {
            render(block);
        }
        for controller in &mut controllers {
            sync_worker(controller);
        }

        let eof_ticks = measure_exclusive_source_worker_ticks(
            tid,
            Duration::from_millis(500),
            Duration::from_secs(10),
        );
        assert!(
            eof_ticks <= 1,
            "EOF-inactive source-set worker consumed {eof_ticks} CPU ticks in 500 ms"
        );
        println!(
            "eight-source shared worker tid={tid} ring_full_ticks={render_wait_ticks} eof_ticks={eof_ticks}"
        );
        drop(plan);
        for controller in &mut controllers {
            assert!(matches!(
                controller.wait_for_event().expect("stopped terminal"),
                NativeSourceWorkerEvent::Terminal {
                    exit: NativeSourceWorkerExit::Stopped,
                    ..
                }
            ));
        }
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
                native_decoder_sanitized_samples: 2,
                exit: NativeSourceWorkerExit::Stopped,
            }
        ));
        assert_eq!(
            controller.try_wake(),
            Err(NativeSourceWorkerControlError::Stopped)
        );
    }

    #[test]
    fn graph_set_driver_retains_all_native_workers_until_set_drop() {
        use core::mem::size_of;
        use miso_engine_core::realtime::RenderEnvelope;

        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut first_resolver = resolver(&[0.25; 4], b"exact-identity");
        let mut second_resolver = resolver(&[0.5; 4], b"exact-identity");
        let first = prepare_native_source(&mut first_resolver, request(region), caps())
            .expect("prepare first native source");
        let second = prepare_native_source(&mut second_resolver, request(region), caps())
            .expect("prepare second native source");
        let (mut first_controller, first_source) = first.into_graph_source();
        let (mut second_controller, second_source) = second.into_graph_source();
        first_controller.wait_for_event().expect("first ready");
        second_controller.wait_for_event().expect("second ready");
        let sources = vec![first_source, second_source];
        let retained = crate::source_set_retained_resources(&sources, &[])
            .expect("exact set retained resources");
        assert_eq!(retained.retirement_workers.item_count, 2);
        assert_eq!(
            retained.retirement_workers.bytes,
            u64::try_from(size_of::<NativeSourceWorker>() * 2).expect("worker box bytes")
        );

        let source_set = prepare_graph_source_set(
            RenderEnvelope {
                sample_rate: SampleRateHz(48_000),
                quantum: QuantumFrames(4),
                input_channels: None,
                output_channels: NonZeroUsize::new(2).expect("two outputs"),
            },
            sources,
            Vec::new(),
        )
        .expect("seal set-owned workers");
        assert!(
            first_controller.events.try_pop().is_err(),
            "first worker terminated before graph-set drop"
        );
        assert!(
            second_controller.events.try_pop().is_err(),
            "second worker terminated before graph-set drop"
        );
        first_controller.try_wake().expect("first remains live");
        second_controller.try_wake().expect("second remains live");

        drop(source_set);
        assert!(matches!(
            first_controller.wait_for_event().expect("first terminal"),
            NativeSourceWorkerEvent::Terminal {
                exit: NativeSourceWorkerExit::Stopped,
                ..
            }
        ));
        assert!(matches!(
            second_controller.wait_for_event().expect("second terminal"),
            NativeSourceWorkerEvent::Terminal {
                exit: NativeSourceWorkerExit::Stopped,
                ..
            }
        ));
        assert_eq!(
            first_controller.try_wake(),
            Err(NativeSourceWorkerControlError::Stopped)
        );
        assert_eq!(
            second_controller.try_wake(),
            Err(NativeSourceWorkerControlError::Stopped)
        );
    }

    #[test]
    fn invalid_graph_mapping_stops_carried_worker_before_consumer_cleanup() {
        use miso_engine_core::realtime::RenderEnvelope;

        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(&[0.25; 4], b"exact-identity");
        let prepared = prepare_native_source(&mut native_resolver, request(region), caps())
            .expect("prepare native source carrier");
        let (mut controller, source) = prepared.into_graph_source();
        controller.wait_for_event().expect("source ready");

        let error = match prepare_graph_source_set(
            RenderEnvelope {
                sample_rate: SampleRateHz(48_000),
                quantum: QuantumFrames(4),
                input_channels: None,
                output_channels: NonZeroUsize::new(2).expect("two outputs"),
            },
            vec![source],
            vec![SourceGraphTrackMapping {
                node: GraphNodeId::Output {
                    output_id: StableGraphId::parse("main").expect("output ID"),
                },
                source_index: 0,
                left_channel: 0,
                right_channel: 0,
            }],
        ) {
            Ok(_) => panic!("invalid mapping unexpectedly sealed"),
            Err(error) => error,
        };
        assert_eq!(error, crate::SourceGraphSourceSetError::SourceIndex);
        assert!(matches!(
            controller.wait_for_event().expect("cleanup terminal"),
            NativeSourceWorkerEvent::Terminal {
                exit: NativeSourceWorkerExit::Stopped,
                ..
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
                effect_bank_count: 0,
                effect_bank_scratch_bytes: 0,
                effect_bank_runtime_buffer_bytes: 0,
                effect_bank_metadata_bytes: 0,
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
            effect_controls: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
            effect_observations: Vec::new(),
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
    fn compiled_sourceless_session_returns_collection_diagnostic_without_resolving() {
        let mut session_toml =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("session");
        session_toml.automation.clear();
        session_toml.routes.clear();
        session_toml.tracks.clear();
        session_toml.sources.clear();
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
        .expect("compiled sourceless session");
        let mut resolver = session_resolver(b"sha256:demo");

        let failure = match prepare_native_session_sources(&session, &mut resolver, session_caps())
        {
            Ok(_) => panic!("sourceless native session unexpectedly prepared"),
            Err(failure) => failure,
        };

        assert_eq!(resolver.calls, 0);
        assert_eq!(failure.diagnostics.len(), 1);
        assert_eq!(
            failure.diagnostics[0].code,
            SourceDiagnosticCode::GraphBindingMismatch
        );
        assert_eq!(failure.diagnostics[0].path.as_str(), "$.sources");
        assert_eq!(
            failure.diagnostics[0].message,
            "native graph source-set preparation requires at least one source"
        );
    }

    #[test]
    fn compiled_session_source_failure_collects_sorted_diagnostics_before_worker_start() {
        let mut session_toml =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("session");
        session_toml.sources[0].mapping.region.length_samples = 4;
        let mut second = session_toml.sources[0].clone();
        second.id = StableId::parse("voice2").expect("ID");
        second.content.locator = "host:voice2".to_owned();
        let mut third = second.clone();
        third.id = StableId::parse("alpha").expect("ID");
        third.content.locator = "host:alpha".to_owned();
        session_toml.sources.push(second);
        session_toml.sources.push(third);
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
        assert_eq!(resolver.calls, 3);
        assert_eq!(failure.diagnostics.len(), 2);
        assert!(
            failure
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code == SourceDiagnosticCode::AssetUnresolved)
        );
        assert_eq!(failure.diagnostics[0].path.as_str(), "$.sources[id=voice2]");
        assert_eq!(failure.diagnostics[1].path.as_str(), "$.sources[id=voice]");

        let source = include_str!("native_source.rs");
        let start = source
            .find("pub fn prepare_native_session_sources")
            .expect("session preparation function");
        let body = &source[start..];
        let inert = body
            .find("prepare_native_source_job")
            .expect("inert job preparation");
        let reject = body
            .find("if !diagnostics.is_empty()")
            .expect("diagnostic rejection");
        let spawn = body.find("start_native_workers").expect("single set start");
        assert!(
            inert < reject && reject < spawn,
            "all inert jobs must reject before the sole worker start"
        );
    }

    fn read_one(consumer: &mut PcmSourceConsumer) -> ([f32; 4], SourceReadReport) {
        let mut output = [0.0; 4];
        let report = {
            let mut planes = [&mut output[..]];
            consumer.read_block(&mut planes).expect("read")
        };
        (output, report)
    }

    #[cfg(target_os = "linux")]
    fn source_worker_tasks() -> Vec<u32> {
        let mut tids = Vec::new();
        for entry in std::fs::read_dir("/proc/self/task").expect("read process tasks") {
            let entry = entry.expect("task entry");
            let Ok(tid) = entry.file_name().to_string_lossy().parse::<u32>() else {
                continue;
            };
            let Ok(name) = std::fs::read_to_string(entry.path().join("comm")) else {
                continue;
            };
            if name.trim_end() == "miso-engine-sou" {
                tids.push(tid);
            }
        }
        tids.sort_unstable();
        tids
    }

    #[cfg(target_os = "linux")]
    fn wait_for_only_source_worker_task(timeout: Duration) -> u32 {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            let tids = source_worker_tasks();
            if let [tid] = tids.as_slice() {
                return *tid;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "expected exactly one miso-engine-sou task, observed {tids:?}"
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    #[cfg(target_os = "linux")]
    fn source_worker_cpu_ticks(tid: u32) -> u64 {
        let stat = std::fs::read_to_string(format!("/proc/self/task/{tid}/stat"))
            .expect("read source worker stat");
        let close = stat.rfind(')').expect("stat command terminator");
        let fields: Vec<_> = stat[close + 1..].split_whitespace().collect();
        let user = fields
            .get(11)
            .expect("stat utime")
            .parse::<u64>()
            .expect("numeric utime");
        let system = fields
            .get(12)
            .expect("stat stime")
            .parse::<u64>()
            .expect("numeric stime");
        user.checked_add(system).expect("worker CPU ticks")
    }

    #[cfg(target_os = "linux")]
    fn measure_exclusive_source_worker_ticks(
        tid: u32,
        interval: Duration,
        timeout: Duration,
    ) -> u64 {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            while source_worker_tasks().as_slice() != [tid] {
                assert!(
                    std::time::Instant::now() < deadline,
                    "parallel source workers did not clear before CPU sampling"
                );
                thread::sleep(Duration::from_millis(10));
            }
            let start = source_worker_cpu_ticks(tid);
            thread::sleep(interval);
            if source_worker_tasks().as_slice() == [tid] {
                return source_worker_cpu_ticks(tid).saturating_sub(start);
            }
            assert!(
                std::time::Instant::now() < deadline,
                "parallel source workers remained active across CPU sampling"
            );
        }
    }

    fn sync_worker(controller: &mut NativeSourceController) {
        controller
            .snapshot_native_decoder_sanitized_samples()
            .expect("worker synchronization");
    }

    struct FailAtRead {
        cursor: Cursor<Vec<u8>>,
        fail_at: u64,
    }

    impl Read for FailAtRead {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.cursor.position() >= self.fail_at {
                return Err(io::Error::other("injected read failure"));
            }
            self.cursor.read(buffer)
        }
    }

    impl Seek for FailAtRead {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            self.cursor.seek(position)
        }
    }

    struct FailingResolver {
        asset: Option<NativeResolvedAsset<FailAtRead>>,
    }

    impl NativeSourceResolver for FailingResolver {
        type Asset = FailAtRead;

        fn resolve(
            &mut self,
            _opaque_locator: &str,
        ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError> {
            self.asset
                .take()
                .ok_or(NativeSourceResolverError::Unresolved)
        }
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
