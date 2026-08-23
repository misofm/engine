//! Bounded, prepared PCM source rings for just-in-time audio delivery.
//!
//! A source ring splits ownership between one non-render producer and one exclusive render
//! consumer. Transfer blocks, queues, and their PCM storage are all created by [`PcmSourceRing`]
//! before rendering starts. The consumer only moves prepared blocks, copies samples, and updates
//! local saturating counters.

#![allow(missing_docs)]

use core::{
    alloc::Layout,
    cmp,
    mem::{align_of, size_of},
    num::NonZeroUsize,
};
use std::fmt;

use miso_engine_core::{
    QuantumFrames, SampleRateHz,
    realtime::{
        Consumer, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError, bounded_spsc,
        bounded_spsc_move, bounded_spsc_retained_payload,
    },
};
use miso_engine_graph::{
    GraphNodeId, GraphPreparedSourceSet, GraphPreparedSourceSetDriver, GraphSourceInputClaim,
    GraphSourceSetResourceReport,
};

#[cfg(not(target_arch = "wasm32"))]
mod native_wave;

#[cfg(not(target_arch = "wasm32"))]
mod native_source;

#[cfg(not(target_arch = "wasm32"))]
use native_source::NativeSourceWorker;

#[cfg(not(target_arch = "wasm32"))]
pub use native_wave::{
    NativeDecodeReport, NativeWaveContainer, NativeWaveDecoder, NativeWaveEncoding,
    NativeWaveError, NativeWaveMetadata, NativeWaveParseCaps, NativeWaveRegion, parse_native_wave,
};

#[cfg(not(target_arch = "wasm32"))]
pub use native_source::{
    NativeResolvedAsset, NativeSessionPreparedSources, NativeSessionSourcePrepareCaps,
    NativeSessionSourcePrepareFailure, NativeSessionSourceResourceReport, NativeSourceController,
    NativeSourcePrepareCaps, NativeSourcePrepareError, NativeSourcePrepareRequest,
    NativeSourceResolver, NativeSourceResolverError, NativeSourceResourceReport,
    NativeSourceWorkerControlError, NativeSourceWorkerEvent, NativeSourceWorkerExit,
    PreparedNativeSource, prepare_native_session_sources, prepare_native_source,
};

#[cfg(all(not(target_arch = "wasm32"), feature = "test-support"))]
pub use native_source::{
    NativeSourceAllocationLayoutEntry, NativeWorkerAuditGate, native_source_allocation_layout,
    prepare_native_source_with_audit_gate,
};

/// A nonzero source-stream generation selected by an off-render controller.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceGeneration(pub u64);

impl SourceGeneration {
    /// Return the generation when it is nonzero.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    /// Whether this carrier is a valid prepared generation.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// An absolute decoded-source frame position.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceFrame(pub u64);

/// A bounded source-control command delivered to the render consumer at a block boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceCommand {
    /// Switch to a strictly newer decoded source generation at `frame`.
    Seek {
        /// The nonzero generation to make audible.
        generation: SourceGeneration,
        /// First decoded source frame for the new generation.
        frame: SourceFrame,
    },
}

/// Stable source diagnostic registry values.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SourceDiagnosticCode {
    AssetUnresolved,
    ContentIdentityMismatch,
    RateMismatch,
    ChannelsMismatch,
    RegionOutOfBounds,
    ContainerInvalid,
    FormatUnsupported,
    GenerationNonMonotonic,
    ResourceArithmeticOverflow,
    ResourceLimit,
    GraphBindingMismatch,
}

impl SourceDiagnosticCode {
    /// Stable dotted machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssetUnresolved => "source.asset.unresolved",
            Self::ContentIdentityMismatch => "source.content.identity_mismatch",
            Self::RateMismatch => "source.rate.mismatch",
            Self::ChannelsMismatch => "source.channels.mismatch",
            Self::RegionOutOfBounds => "source.region.out_of_bounds",
            Self::ContainerInvalid => "source.container.invalid",
            Self::FormatUnsupported => "source.format.unsupported",
            Self::GenerationNonMonotonic => "source.generation.non_monotonic",
            Self::ResourceArithmeticOverflow => "source.resource.arithmetic_overflow",
            Self::ResourceLimit => "source.resource.limit",
            Self::GraphBindingMismatch => "source.graph.binding_mismatch",
        }
    }
}

impl fmt::Display for SourceDiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A stable source declaration path, including its stable-ID selector.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SourceDiagnosticPath(String);

impl SourceDiagnosticPath {
    /// Construct the required source-ID diagnostic path.
    #[must_use]
    pub fn for_source(source_id: &str) -> Self {
        Self(format!("$.sources[id={source_id}]"))
    }

    /// Borrow the canonical path text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SourceDiagnosticPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One sorted, structured source-preparation diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDiagnostic {
    /// Stable registry reason.
    pub code: SourceDiagnosticCode,
    /// Source declaration path.
    pub path: SourceDiagnosticPath,
    /// Concise control-plane-only explanation.
    pub message: String,
}

impl SourceDiagnostic {
    /// Build one source diagnostic.
    #[must_use]
    pub fn new(code: SourceDiagnosticCode, path: SourceDiagnosticPath, message: &str) -> Self {
        Self {
            code,
            path,
            message: message.to_owned(),
        }
    }
}

/// Exact source-ring configuration prepared before rendering starts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PcmSourceRingConfig {
    /// Number of planar source channels.
    pub channel_count: u32,
    /// Fixed transfer and render quantum in frames.
    pub quantum_frames: QuantumFrames,
    /// Exact retained source-ring capacity in PCM frames.
    pub frame_capacity: u64,
    /// Initially active nonzero source generation.
    pub initial_generation: SourceGeneration,
}

/// Immutable shape shared by the two endpoints of one prepared source ring.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PcmSourceShape {
    pub channel_count: u32,
    pub quantum_frames: QuantumFrames,
    pub frame_capacity: u64,
    pub transfer_block_count: u64,
}

/// Exact source-owned allocation and queue accounting.
///
/// `pcm_payload_already_charged_bytes` is the exact session-owned source-ring PCM charge. It is
/// repeated as transfer-block samples and is intentionally excluded from `overhead_bytes`, so a
/// later graph/source preparation cannot charge it twice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceResourceReport {
    /// Number of preallocated transfer blocks.
    pub transfer_block_count: u64,
    /// PCM payload already charged by the session source declaration.
    pub pcm_payload_already_charged_bytes: u64,
    /// Data SPSC header plus slots.
    pub data_queue_bytes: u64,
    /// Recycle SPSC header plus slots.
    pub recycle_queue_bytes: u64,
    /// One-slot seek-command SPSC header plus slots.
    pub command_queue_bytes: u64,
    /// One `TransferBlock` allocation per prepared block, excluding its PCM allocation.
    pub transfer_block_metadata_bytes: u64,
    /// PCM bytes in prepared transfer blocks; exactly equals the session PCM charge.
    pub transfer_block_pcm_bytes: u64,
    /// Source allocations not already charged by the session declaration.
    pub overhead_bytes: u64,
    /// Exact source-owned bytes including the already charged PCM payload.
    pub total_engine_owned_bytes: u64,
    /// Largest exact engine-owned allocation request.
    pub largest_allocation_bytes: u64,
}

/// One exact retained allocation class, excluding allocator headers and page rounding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceRetainedAllocation {
    pub item_count: u64,
    pub bytes: u64,
    pub largest_allocation_bytes: u64,
    pub alignment_bytes: u64,
}

/// Enumerated source-set allocations retained after successful graph binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceSetRetainedResourceReport {
    pub source_entries: SourceRetainedAllocation,
    pub mappings: SourceRetainedAllocation,
    pub claims: SourceRetainedAllocation,
    pub driver: SourceRetainedAllocation,
    pub retirement_workers: SourceRetainedAllocation,
    pub source_planes: SourceRetainedAllocation,
    pub owned_stable_id_payloads: SourceRetainedAllocation,
}

impl SourceSetRetainedResourceReport {
    fn overhead_bytes(self) -> Option<u64> {
        [
            self.source_entries.bytes,
            self.mappings.bytes,
            self.claims.bytes,
            self.driver.bytes,
            self.retirement_workers.bytes,
            self.source_planes.bytes,
            self.owned_stable_id_payloads.bytes,
        ]
        .into_iter()
        .try_fold(0_u64, u64::checked_add)
    }

    fn largest_allocation_bytes(self) -> u64 {
        [
            self.source_entries.largest_allocation_bytes,
            self.mappings.largest_allocation_bytes,
            self.claims.largest_allocation_bytes,
            self.driver.largest_allocation_bytes,
            self.retirement_workers.largest_allocation_bytes,
            self.source_planes.largest_allocation_bytes,
            self.owned_stable_id_payloads.largest_allocation_bytes,
        ]
        .into_iter()
        .max()
        .unwrap_or(0)
    }
}

/// Preparation failure that occurs before a usable producer/consumer split exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PcmSourceRingError {
    ZeroChannelCount,
    ZeroQuantumFrames,
    CapacityBelowQuantum,
    CapacityNotQuantumMultiple,
    InitialGenerationZero,
    ArithmeticOverflow,
    PlatformSizeLimit,
    AllocationFailure,
    InternalQueueCapacity,
}

impl PcmSourceRingError {
    /// The stable diagnostic code for capacity/preparation errors.
    #[must_use]
    pub const fn diagnostic_code(self) -> SourceDiagnosticCode {
        match self {
            Self::ArithmeticOverflow | Self::PlatformSizeLimit => {
                SourceDiagnosticCode::ResourceArithmeticOverflow
            }
            Self::ZeroChannelCount
            | Self::ZeroQuantumFrames
            | Self::CapacityBelowQuantum
            | Self::CapacityNotQuantumMultiple
            | Self::InitialGenerationZero
            | Self::AllocationFailure
            | Self::InternalQueueCapacity => SourceDiagnosticCode::ResourceLimit,
        }
    }
}

/// One checked host submission result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubmitReport {
    /// Source frames accepted from the supplied chunk.
    pub accepted_frames: u32,
    /// Saturating total accepted source frames for this producer endpoint.
    pub cumulative_written_frames: u64,
    /// Active generation after accepting the chunk.
    pub active_generation: SourceGeneration,
}

/// A borrowed planar host PCM chunk submitted outside render.
#[derive(Clone, Copy, Debug)]
pub struct HostPlanarChunk<'a> {
    /// Explicit sample rate of the host-provided decoded PCM.
    pub sample_rate_hz: SampleRateHz,
    /// Source stream generation carried by every frame in this chunk.
    pub generation: SourceGeneration,
    /// Absolute decoded source-frame position of the first supplied frame.
    pub start_frame: SourceFrame,
    /// One contiguous plane for every declared source channel.
    pub planes: &'a [&'a [f32]],
    /// Number of valid frames in each plane.
    pub frames: u32,
    /// Whether this is the sole final short block or a zero-frame end marker.
    pub end_of_region: bool,
}

/// Rejection of a borrowed host chunk; no prefix is accepted on any error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostChunkError {
    WrongSampleRate {
        expected: SampleRateHz,
        actual: SampleRateHz,
    },
    StaleGeneration {
        active: SourceGeneration,
        submitted: SourceGeneration,
    },
    ChannelCount {
        expected: u32,
        actual: usize,
    },
    PlaneLength {
        expected_frames: u32,
    },
    FrameCount {
        quantum_frames: u32,
        submitted_frames: u32,
        end_of_region: bool,
    },
    NonContiguous {
        expected: SourceFrame,
        actual: SourceFrame,
    },
    EndOfRegionAlreadySubmitted,
    Full {
        full_count: u64,
    },
    InternalInvariant,
}

impl HostChunkError {
    /// Stable diagnostics where a host error maps to a source registry reason.
    #[must_use]
    pub const fn diagnostic_code(self) -> Option<SourceDiagnosticCode> {
        match self {
            Self::WrongSampleRate { .. } => Some(SourceDiagnosticCode::RateMismatch),
            Self::ChannelCount { .. } => Some(SourceDiagnosticCode::ChannelsMismatch),
            Self::StaleGeneration { .. } => Some(SourceDiagnosticCode::GenerationNonMonotonic),
            Self::FrameCount { .. }
            | Self::PlaneLength { .. }
            | Self::NonContiguous { .. }
            | Self::EndOfRegionAlreadySubmitted
            | Self::Full { .. }
            | Self::InternalInvariant => None,
        }
    }
}

/// Rejection of a source seek before it reaches the render consumer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceSeekError {
    GenerationZero,
    GenerationNotStrictlyIncreasing {
        active: SourceGeneration,
        requested: SourceGeneration,
    },
    Backpressure {
        full_count: u64,
    },
}

impl SourceSeekError {
    /// Stable source registry reason for rejected generation changes.
    #[must_use]
    pub const fn diagnostic_code(self) -> SourceDiagnosticCode {
        SourceDiagnosticCode::GenerationNonMonotonic
    }
}

/// Failure to copy one prepared source quantum into caller-owned source planes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceReadError {
    ChannelCount { expected: u32, actual: usize },
    PlaneLength { expected_frames: u32 },
}

/// One render-owner result for a prepared source quantum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceReadReport {
    /// PCM frames copied from an accepted transfer block.
    pub copied_frames: u32,
    /// Unavailable in-region frames emitted as positive zero in this call.
    pub underrun_frames: u32,
    /// Whether this call contained one maximal missing in-region run.
    pub underrun_event: bool,
    /// Whether the source is at a declared end-of-region after this call.
    pub end_of_region: bool,
    /// Active generation used for this render quantum.
    pub active_generation: SourceGeneration,
    /// Saturating cumulative accepted PCM frame reads (silence is excluded).
    pub cumulative_read_frames: u64,
    /// Saturating cumulative missing in-region frame count.
    pub cumulative_underrun_frames: u64,
    /// Saturating cumulative missing-run count.
    pub cumulative_underrun_events: u64,
}

/// Owner-local producer telemetry copied only outside render.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceProducerTelemetry {
    pub active_generation: SourceGeneration,
    pub cumulative_written_frames: u64,
    pub data_full_count: u64,
    pub recycle_empty_count: u64,
    pub end_of_region_submitted: bool,
    /// Native-decoder replacements, or zero for a host-supplied producer.
    pub native_decoder_sanitized_samples: u64,
}

/// Owner-local render telemetry copied only after the render owner is disarmed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceConsumerTelemetry {
    pub active_generation: SourceGeneration,
    pub cumulative_read_frames: u64,
    pub stale_generation_discard_count: u64,
    pub underrun_frames: u64,
    pub underrun_events: u64,
    pub end_of_region: bool,
    /// Native-decoder replacements, or zero for a host-supplied consumer.
    pub native_decoder_sanitized_samples: u64,
}

struct TransferBlock {
    generation: SourceGeneration,
    start_frame: SourceFrame,
    frames: u32,
    end_of_region: bool,
    native_decoder_sanitized_samples: u64,
    samples: Box<[f32]>,
}

impl TransferBlock {
    fn try_new(sample_count: usize) -> Result<Self, PcmSourceRingError> {
        let mut samples = Vec::new();
        samples
            .try_reserve_exact(sample_count)
            .map_err(|_| PcmSourceRingError::AllocationFailure)?;
        samples.resize(sample_count, 0.0);
        Ok(Self {
            generation: SourceGeneration(1),
            start_frame: SourceFrame(0),
            frames: 0,
            end_of_region: false,
            native_decoder_sanitized_samples: 0,
            samples: samples.into_boxed_slice(),
        })
    }

    fn reset_metadata(&mut self) {
        self.generation = SourceGeneration(1);
        self.start_frame = SourceFrame(0);
        self.frames = 0;
        self.end_of_region = false;
        self.native_decoder_sanitized_samples = 0;
    }
}

/// Prepared source-ring constructor.
pub struct PcmSourceRing;

impl PcmSourceRing {
    /// Compute exact source-ring allocations without creating any queues or PCM blocks.
    pub fn resource_report(
        config: PcmSourceRingConfig,
    ) -> Result<SourceResourceReport, PcmSourceRingError> {
        let shape = PreparedShape::validate(config)?;
        let data_queue = queue_bytes::<Box<TransferBlock>>(shape.transfer_block_count)?;
        let recycle_queue = queue_bytes::<Box<TransferBlock>>(shape.transfer_block_count)?;
        let command_capacity = NonZeroUsize::new(1).expect("one command slot");
        let command_queue = queue_bytes::<SourceCommand>(command_capacity)?;
        let pcm = checked_u64_mul(config.frame_capacity, u64::from(config.channel_count))?
            .checked_mul(
                u64::try_from(size_of::<f32>())
                    .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            )
            .ok_or(PcmSourceRingError::ArithmeticOverflow)?;
        let metadata = checked_u64_mul(
            shape.transfer_block_count_u64,
            u64::try_from(size_of::<TransferBlock>())
                .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
        )?;
        let overhead = checked_u64_add(
            checked_u64_add(data_queue, recycle_queue)?,
            checked_u64_add(command_queue, metadata)?,
        )?;
        let total = checked_u64_add(pcm, overhead)?;
        let largest = [
            pcm / shape.transfer_block_count_u64,
            u64::try_from(size_of::<TransferBlock>())
                .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            largest_queue_allocation::<Box<TransferBlock>>(shape.transfer_block_count)?,
            largest_queue_allocation::<SourceCommand>(command_capacity)?,
        ]
        .into_iter()
        .max()
        .expect("nonempty exact allocation list");
        Ok(SourceResourceReport {
            transfer_block_count: shape.transfer_block_count_u64,
            pcm_payload_already_charged_bytes: pcm,
            data_queue_bytes: data_queue,
            recycle_queue_bytes: recycle_queue,
            command_queue_bytes: command_queue,
            transfer_block_metadata_bytes: metadata,
            transfer_block_pcm_bytes: pcm,
            overhead_bytes: overhead,
            total_engine_owned_bytes: total,
            largest_allocation_bytes: largest,
        })
    }

    /// Allocate the exact fixed source-ring resources and split producer/render ownership.
    pub fn prepare(
        config: PcmSourceRingConfig,
    ) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
    {
        Self::prepare_at_source_frame(config, SourceFrame(0))
    }

    /// Allocate a host-fed ring whose first accepted chunk begins at `initial_frame`.
    ///
    /// Shape validation, allocation, ownership, and resource accounting are identical to
    /// [`Self::prepare`]; only the initial absolute source position differs.
    pub fn prepare_host_region(
        config: PcmSourceRingConfig,
        initial_frame: SourceFrame,
    ) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
    {
        Self::prepare_at_source_frame(config, initial_frame)
    }

    pub(crate) fn prepare_at_source_frame(
        config: PcmSourceRingConfig,
        initial_frame: SourceFrame,
    ) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
    {
        let shape = PreparedShape::validate(config)?;
        let report = Self::resource_report(config)?;
        let queue_generation = QueueGeneration(config.initial_generation.0);
        let (data_producer, data_consumer) =
            bounded_spsc_move(shape.transfer_block_count, queue_generation)
                .map_err(map_spsc_error)?;
        let (recycle_producer, recycle_consumer) =
            bounded_spsc_move(shape.transfer_block_count, queue_generation)
                .map_err(map_spsc_error)?;
        let (command_producer, command_consumer) = bounded_spsc(
            NonZeroUsize::new(1).expect("one command slot"),
            queue_generation,
        )
        .map_err(map_spsc_error)?;
        let mut consumer = PcmSourceConsumer {
            data_consumer,
            recycle_producer,
            command_consumer,
            shape: PcmSourceShape {
                channel_count: config.channel_count,
                quantum_frames: config.quantum_frames,
                frame_capacity: config.frame_capacity,
                transfer_block_count: shape.transfer_block_count_u64,
            },
            channel_count: config.channel_count,
            quantum_frames: config.quantum_frames.0,
            transfer_block_count: shape.transfer_block_count.get(),
            active_generation: config.initial_generation,
            next_frame: initial_frame,
            end_frame: None,
            end_of_region: false,
            current: None,
            deferred_recycle: None,
            cumulative_read_frames: 0,
            stale_generation_discard_count: 0,
            underrun_frames: 0,
            underrun_events: 0,
            native_decoder_sanitized_samples: 0,
        };
        for _ in 0..shape.transfer_block_count.get() {
            let block = Box::new(TransferBlock::try_new(shape.samples_per_block)?);
            consumer
                .recycle_producer
                .try_push(block)
                .map_err(|_| PcmSourceRingError::InternalQueueCapacity)?;
        }
        Ok((
            PcmSourceProducer {
                data_producer,
                recycle_consumer,
                command_producer,
                shape: consumer.shape,
                channel_count: config.channel_count,
                quantum_frames: config.quantum_frames.0,
                active_generation: config.initial_generation,
                next_write_frame: initial_frame,
                end_of_region_submitted: false,
                cumulative_written_frames: 0,
                deferred_block: None,
                native_decoder_sanitized_samples: 0,
            },
            consumer,
            report,
        ))
    }
}

struct PreparedShape {
    transfer_block_count: NonZeroUsize,
    transfer_block_count_u64: u64,
    samples_per_block: usize,
}

impl PreparedShape {
    fn validate(config: PcmSourceRingConfig) -> Result<Self, PcmSourceRingError> {
        if config.channel_count == 0 {
            return Err(PcmSourceRingError::ZeroChannelCount);
        }
        if config.quantum_frames.0 == 0 {
            return Err(PcmSourceRingError::ZeroQuantumFrames);
        }
        if config.frame_capacity < u64::from(config.quantum_frames.0) {
            return Err(PcmSourceRingError::CapacityBelowQuantum);
        }
        if !config
            .frame_capacity
            .is_multiple_of(u64::from(config.quantum_frames.0))
        {
            return Err(PcmSourceRingError::CapacityNotQuantumMultiple);
        }
        if !config.initial_generation.is_valid() {
            return Err(PcmSourceRingError::InitialGenerationZero);
        }
        let transfer_block_count_u64 = config.frame_capacity / u64::from(config.quantum_frames.0);
        let transfer_block_count = usize::try_from(transfer_block_count_u64)
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
        let transfer_block_count =
            NonZeroUsize::new(transfer_block_count).ok_or(PcmSourceRingError::PlatformSizeLimit)?;
        let samples_per_block = usize::try_from(config.channel_count)
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?
            .checked_mul(
                usize::try_from(config.quantum_frames.0)
                    .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            )
            .ok_or(PcmSourceRingError::ArithmeticOverflow)?;
        Ok(Self {
            transfer_block_count,
            transfer_block_count_u64,
            samples_per_block,
        })
    }
}

/// Exclusive non-render producer endpoint for one prepared source ring.
pub struct PcmSourceProducer {
    data_producer: Producer<Box<TransferBlock>>,
    recycle_consumer: Consumer<Box<TransferBlock>>,
    command_producer: Producer<SourceCommand>,
    shape: PcmSourceShape,
    channel_count: u32,
    quantum_frames: u32,
    active_generation: SourceGeneration,
    next_write_frame: SourceFrame,
    end_of_region_submitted: bool,
    cumulative_written_frames: u64,
    deferred_block: Option<Box<TransferBlock>>,
    native_decoder_sanitized_samples: u64,
}

/// Name for the prepared producer when it is used solely to control source seeks.
pub type SourceController = PcmSourceProducer;

impl PcmSourceProducer {
    /// Immutable prepared ring shape shared with the consumer endpoint.
    #[must_use]
    pub const fn shape(&self) -> PcmSourceShape {
        self.shape
    }

    /// Turn this producer into an explicit-rate host PCM boundary.
    #[must_use]
    pub fn into_host_chunk_provider(self, sample_rate_hz: SampleRateHz) -> HostChunkProvider {
        HostChunkProvider {
            producer: self,
            sample_rate_hz,
        }
    }

    /// Request a strictly newer generation; it becomes audible only on the next render block.
    pub fn try_seek(&mut self, command: SourceCommand) -> Result<(), SourceSeekError> {
        let SourceCommand::Seek { generation, frame } = command;
        if !generation.is_valid() {
            return Err(SourceSeekError::GenerationZero);
        }
        if generation <= self.active_generation {
            return Err(SourceSeekError::GenerationNotStrictlyIncreasing {
                active: self.active_generation,
                requested: generation,
            });
        }
        match self
            .command_producer
            .try_push(SourceCommand::Seek { generation, frame })
        {
            Ok(()) => {
                self.active_generation = generation;
                self.next_write_frame = frame;
                self.end_of_region_submitted = false;
                Ok(())
            }
            Err(QueueFull { full_count, .. }) => Err(SourceSeekError::Backpressure { full_count }),
        }
    }

    /// Exact configured source channel count.
    #[must_use]
    pub const fn channel_count(&self) -> u32 {
        self.channel_count
    }

    /// Exact configured source render quantum.
    #[must_use]
    pub const fn quantum_frames(&self) -> u32 {
        self.quantum_frames
    }

    /// Copy producer-local telemetry outside render.
    #[must_use]
    pub fn telemetry(&self) -> SourceProducerTelemetry {
        SourceProducerTelemetry {
            active_generation: self.active_generation,
            cumulative_written_frames: self.cumulative_written_frames,
            data_full_count: self.data_producer.full_count(),
            recycle_empty_count: self.recycle_consumer.empty_count(),
            end_of_region_submitted: self.end_of_region_submitted,
            native_decoder_sanitized_samples: self.native_decoder_sanitized_samples,
        }
    }

    fn take_recycled_block(&mut self) -> Result<Box<TransferBlock>, HostChunkError> {
        if let Some(block) = self.deferred_block.take() {
            match self.data_producer.try_push(block) {
                Ok(()) => {}
                Err(QueueFull {
                    value, full_count, ..
                }) => {
                    self.deferred_block = Some(value);
                    return Err(HostChunkError::Full { full_count });
                }
            }
        }
        self.recycle_consumer
            .try_pop()
            .map_err(|QueueEmpty { .. }| HostChunkError::Full {
                full_count: self.data_producer.full_count(),
            })
    }

    fn submit(&mut self, chunk: HostPlanarChunk<'_>) -> Result<SubmitReport, HostChunkError> {
        validate_host_chunk(self, chunk)?;
        let mut block = self.take_recycled_block()?;
        block.generation = chunk.generation;
        block.start_frame = chunk.start_frame;
        block.frames = chunk.frames;
        block.end_of_region = chunk.end_of_region;
        block.native_decoder_sanitized_samples = 0;
        let quantum = usize::try_from(self.quantum_frames).expect("prepared quantum fits usize");
        let frames = usize::try_from(chunk.frames).expect("u32 fits usize");
        for (channel, plane) in chunk.planes.iter().enumerate() {
            let offset = channel
                .checked_mul(quantum)
                .expect("prepared channel offset");
            block.samples[offset..offset + frames].copy_from_slice(&plane[..frames]);
        }
        self.publish_block(block, chunk.frames, chunk.end_of_region)
    }

    #[allow(dead_code)]
    fn submit_contiguous_planar(
        &mut self,
        generation: SourceGeneration,
        start_frame: SourceFrame,
        planar_quantum: &[f32],
        frames: u32,
        end_of_region: bool,
        native_decoder_sanitized_samples: u64,
    ) -> Result<SubmitReport, HostChunkError> {
        validate_submission_metadata(
            self,
            generation,
            start_frame,
            frames,
            end_of_region,
            self.channel_count,
        )?;
        let quantum = usize::try_from(self.quantum_frames).expect("prepared quantum fits usize");
        let expected_samples = usize::try_from(self.channel_count)
            .expect("u32 fits usize")
            .checked_mul(quantum)
            .expect("prepared planar samples");
        if planar_quantum.len() != expected_samples {
            return Err(HostChunkError::InternalInvariant);
        }
        let mut block = self.take_recycled_block()?;
        block.generation = generation;
        block.start_frame = start_frame;
        block.frames = frames;
        block.end_of_region = end_of_region;
        block.native_decoder_sanitized_samples = native_decoder_sanitized_samples;
        self.native_decoder_sanitized_samples = self
            .native_decoder_sanitized_samples
            .max(native_decoder_sanitized_samples);
        let frames = usize::try_from(frames).expect("u32 fits usize");
        for channel in 0..usize::try_from(self.channel_count).expect("u32 fits usize") {
            let offset = channel
                .checked_mul(quantum)
                .expect("prepared channel offset");
            block.samples[offset..offset + frames]
                .copy_from_slice(&planar_quantum[offset..offset + frames]);
        }
        self.publish_block(
            block,
            u32::try_from(frames).expect("source frames are u32"),
            end_of_region,
        )
    }

    fn publish_block(
        &mut self,
        block: Box<TransferBlock>,
        frames: u32,
        end_of_region: bool,
    ) -> Result<SubmitReport, HostChunkError> {
        match self.data_producer.try_push(block) {
            Ok(()) => {
                self.next_write_frame =
                    SourceFrame(self.next_write_frame.0.saturating_add(u64::from(frames)));
                self.cumulative_written_frames = self
                    .cumulative_written_frames
                    .saturating_add(u64::from(frames));
                if end_of_region {
                    self.end_of_region_submitted = true;
                }
                Ok(SubmitReport {
                    accepted_frames: frames,
                    cumulative_written_frames: self.cumulative_written_frames,
                    active_generation: self.active_generation,
                })
            }
            Err(QueueFull {
                value, full_count, ..
            }) => {
                self.deferred_block = Some(value);
                Err(HostChunkError::Full { full_count })
            }
        }
    }
}

/// Explicit-rate host PCM submission boundary for mobile and browser embedding.
pub struct HostChunkProvider {
    producer: PcmSourceProducer,
    sample_rate_hz: SampleRateHz,
}

impl HostChunkProvider {
    /// Submit one borrowed planar chunk atomically; errors accept no prefix.
    pub fn submit(&mut self, chunk: HostPlanarChunk<'_>) -> Result<SubmitReport, HostChunkError> {
        if chunk.sample_rate_hz != self.sample_rate_hz {
            return Err(HostChunkError::WrongSampleRate {
                expected: self.sample_rate_hz,
                actual: chunk.sample_rate_hz,
            });
        }
        self.producer.submit(chunk)
    }

    /// Submit a bounded generation-tagged seek request.
    pub fn try_seek(&mut self, command: SourceCommand) -> Result<(), SourceSeekError> {
        self.producer.try_seek(command)
    }

    /// Copy producer telemetry outside render.
    #[must_use]
    pub fn telemetry(&self) -> SourceProducerTelemetry {
        self.producer.telemetry()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    pub(crate) fn submit_native_planar(
        &mut self,
        generation: SourceGeneration,
        start_frame: SourceFrame,
        planar_quantum: &[f32],
        frames: u32,
        end_of_region: bool,
        native_decoder_sanitized_samples: u64,
    ) -> Result<SubmitReport, HostChunkError> {
        self.producer.submit_contiguous_planar(
            generation,
            start_frame,
            planar_quantum,
            frames,
            end_of_region,
            native_decoder_sanitized_samples,
        )
    }
}

fn validate_host_chunk(
    producer: &PcmSourceProducer,
    chunk: HostPlanarChunk<'_>,
) -> Result<(), HostChunkError> {
    validate_submission_metadata(
        producer,
        chunk.generation,
        chunk.start_frame,
        chunk.frames,
        chunk.end_of_region,
        u32::try_from(chunk.planes.len()).map_err(|_| HostChunkError::InternalInvariant)?,
    )?;
    if chunk
        .planes
        .iter()
        .any(|plane| plane.len() != usize::try_from(chunk.frames).expect("u32 fits usize"))
    {
        return Err(HostChunkError::PlaneLength {
            expected_frames: chunk.frames,
        });
    }
    Ok(())
}

fn validate_submission_metadata(
    producer: &PcmSourceProducer,
    generation: SourceGeneration,
    start_frame: SourceFrame,
    frames: u32,
    end_of_region: bool,
    channel_count: u32,
) -> Result<(), HostChunkError> {
    if generation != producer.active_generation {
        return Err(HostChunkError::StaleGeneration {
            active: producer.active_generation,
            submitted: generation,
        });
    }
    if channel_count != producer.channel_count {
        return Err(HostChunkError::ChannelCount {
            expected: producer.channel_count,
            actual: usize::try_from(channel_count).expect("u32 fits usize"),
        });
    }
    if frames > producer.quantum_frames
        || (frames < producer.quantum_frames && !end_of_region)
        || (frames == 0 && !end_of_region)
    {
        return Err(HostChunkError::FrameCount {
            quantum_frames: producer.quantum_frames,
            submitted_frames: frames,
            end_of_region,
        });
    }
    if start_frame != producer.next_write_frame {
        return Err(HostChunkError::NonContiguous {
            expected: producer.next_write_frame,
            actual: start_frame,
        });
    }
    if producer.end_of_region_submitted {
        return Err(HostChunkError::EndOfRegionAlreadySubmitted);
    }
    Ok(())
}

/// Exclusive render-owner consumer endpoint for one prepared source ring.
pub struct PcmSourceConsumer {
    data_consumer: Consumer<Box<TransferBlock>>,
    recycle_producer: Producer<Box<TransferBlock>>,
    command_consumer: Consumer<SourceCommand>,
    shape: PcmSourceShape,
    channel_count: u32,
    quantum_frames: u32,
    transfer_block_count: usize,
    active_generation: SourceGeneration,
    next_frame: SourceFrame,
    end_frame: Option<SourceFrame>,
    end_of_region: bool,
    current: Option<Box<TransferBlock>>,
    deferred_recycle: Option<Box<TransferBlock>>,
    cumulative_read_frames: u64,
    stale_generation_discard_count: u64,
    underrun_frames: u64,
    underrun_events: u64,
    native_decoder_sanitized_samples: u64,
}

impl PcmSourceConsumer {
    /// Immutable prepared ring shape shared with the producer endpoint.
    #[must_use]
    pub const fn shape(&self) -> PcmSourceShape {
        self.shape
    }

    /// Copy one exact render quantum into planar caller storage without allocation or blocking.
    pub fn read_block(
        &mut self,
        output_planes: &mut [&mut [f32]],
    ) -> Result<SourceReadReport, SourceReadError> {
        self.validate_output_shape(output_planes)?;
        self.flush_deferred_recycle();
        self.observe_seek_at_block_boundary();
        for output in output_planes.iter_mut() {
            output.fill(0.0);
        }
        self.acquire_current_block();
        let mut copied_frames = 0_u32;
        let mut underrun_frames = 0_u32;
        if self.end_reached() {
            self.end_of_region = true;
        } else if self.current_matches_next_frame() {
            copied_frames = self.copy_current_block(output_planes);
        } else {
            let available_until_end = self
                .end_frame
                .map(|end| end.0.saturating_sub(self.next_frame.0))
                .unwrap_or(u64::from(self.quantum_frames));
            underrun_frames = u32::try_from(cmp::min(
                u64::from(self.quantum_frames),
                available_until_end,
            ))
            .expect("bounded by quantum");
            self.next_frame = SourceFrame(
                self.next_frame
                    .0
                    .saturating_add(u64::from(self.quantum_frames)),
            );
            if underrun_frames != 0 {
                self.underrun_frames = self
                    .underrun_frames
                    .saturating_add(u64::from(underrun_frames));
                self.underrun_events = self.underrun_events.saturating_add(1);
            }
            if self.end_reached() {
                self.end_of_region = true;
            }
        }
        Ok(SourceReadReport {
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.active_generation,
            cumulative_read_frames: self.cumulative_read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
        })
    }

    /// Copy one exact render quantum into preallocated contiguous planar storage.
    ///
    /// Planes are laid out consecutively, each with `quantum_frames()` samples. This is the
    /// coordinator-facing variant used by graph source fan-out and performs no allocation.
    pub fn read_block_contiguous(
        &mut self,
        output: &mut [f32],
    ) -> Result<SourceReadReport, SourceReadError> {
        let expected = usize::try_from(self.channel_count)
            .expect("u32 fits usize")
            .checked_mul(usize::try_from(self.quantum_frames).expect("u32 fits usize"))
            .expect("prepared source shape");
        if output.len() != expected {
            return Err(SourceReadError::PlaneLength {
                expected_frames: self.quantum_frames,
            });
        }
        self.flush_deferred_recycle();
        self.observe_seek_at_block_boundary();
        output.fill(0.0);
        self.acquire_current_block();
        let mut copied_frames = 0_u32;
        let mut underrun_frames = 0_u32;
        if self.end_reached() {
            self.end_of_region = true;
        } else if self.current_matches_next_frame() {
            copied_frames = self.copy_current_block_contiguous(output);
        } else {
            let available_until_end = self
                .end_frame
                .map(|end| end.0.saturating_sub(self.next_frame.0))
                .unwrap_or(u64::from(self.quantum_frames));
            underrun_frames = u32::try_from(cmp::min(
                u64::from(self.quantum_frames),
                available_until_end,
            ))
            .expect("bounded by quantum");
            self.next_frame = SourceFrame(
                self.next_frame
                    .0
                    .saturating_add(u64::from(self.quantum_frames)),
            );
            if underrun_frames != 0 {
                self.underrun_frames = self
                    .underrun_frames
                    .saturating_add(u64::from(underrun_frames));
                self.underrun_events = self.underrun_events.saturating_add(1);
            }
            if self.end_reached() {
                self.end_of_region = true;
            }
        }
        Ok(SourceReadReport {
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.active_generation,
            cumulative_read_frames: self.cumulative_read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
        })
    }

    /// Exact source channel count.
    #[must_use]
    pub const fn channel_count(&self) -> u32 {
        self.channel_count
    }

    /// Exact configured source ring capacity in transfer blocks.
    #[must_use]
    pub const fn transfer_block_capacity(&self) -> usize {
        self.transfer_block_count
    }

    /// Copy render-owner telemetry after its plan/source set is disarmed.
    #[must_use]
    pub fn telemetry(&self) -> SourceConsumerTelemetry {
        SourceConsumerTelemetry {
            active_generation: self.active_generation,
            cumulative_read_frames: self.cumulative_read_frames,
            stale_generation_discard_count: self.stale_generation_discard_count,
            underrun_frames: self.underrun_frames,
            underrun_events: self.underrun_events,
            end_of_region: self.end_of_region,
            native_decoder_sanitized_samples: self.native_decoder_sanitized_samples,
        }
    }

    fn validate_output_shape(
        &self,
        output_planes: &mut [&mut [f32]],
    ) -> Result<(), SourceReadError> {
        if output_planes.len() != usize::try_from(self.channel_count).expect("u32 fits usize") {
            return Err(SourceReadError::ChannelCount {
                expected: self.channel_count,
                actual: output_planes.len(),
            });
        }
        if output_planes.iter().any(|plane| {
            plane.len() != usize::try_from(self.quantum_frames).expect("u32 fits usize")
        }) {
            return Err(SourceReadError::PlaneLength {
                expected_frames: self.quantum_frames,
            });
        }
        Ok(())
    }

    fn observe_seek_at_block_boundary(&mut self) {
        let Ok(SourceCommand::Seek { generation, frame }) = self.command_consumer.try_pop() else {
            return;
        };
        self.active_generation = generation;
        self.next_frame = frame;
        self.end_frame = None;
        self.end_of_region = false;
        if let Some(block) = self.current.take() {
            self.discard_block(block);
        }
    }

    fn acquire_current_block(&mut self) {
        if self.current.is_some() {
            return;
        }
        for _ in 0..self.transfer_block_count {
            let Ok(block) = self.data_consumer.try_pop() else {
                break;
            };
            self.native_decoder_sanitized_samples = self
                .native_decoder_sanitized_samples
                .max(block.native_decoder_sanitized_samples);
            if block.generation != self.active_generation || block.start_frame.0 < self.next_frame.0
            {
                self.note_end_and_discard(block);
                continue;
            }
            if block.end_of_region {
                self.note_end_frame(&block);
            }
            self.current = Some(block);
            break;
        }
    }

    fn current_matches_next_frame(&self) -> bool {
        self.current
            .as_ref()
            .is_some_and(|block| block.start_frame == self.next_frame)
    }

    fn copy_current_block(&mut self, output_planes: &mut [&mut [f32]]) -> u32 {
        let Some(mut block) = self.current.take() else {
            return 0;
        };
        let frames = usize::try_from(block.frames).expect("u32 fits usize");
        let quantum = usize::try_from(self.quantum_frames).expect("u32 fits usize");
        for (channel, output) in output_planes.iter_mut().enumerate() {
            let offset = channel
                .checked_mul(quantum)
                .expect("prepared channel offset");
            output[..frames].copy_from_slice(&block.samples[offset..offset + frames]);
        }
        self.next_frame = SourceFrame(self.next_frame.0.saturating_add(u64::from(block.frames)));
        self.cumulative_read_frames = self
            .cumulative_read_frames
            .saturating_add(u64::from(block.frames));
        if block.end_of_region {
            self.end_of_region = true;
        }
        block.reset_metadata();
        self.recycle_block(block);
        u32::try_from(frames).expect("source block frame count is u32")
    }

    fn copy_current_block_contiguous(&mut self, output: &mut [f32]) -> u32 {
        let Some(mut block) = self.current.take() else {
            return 0;
        };
        let frames = usize::try_from(block.frames).expect("u32 fits usize");
        let quantum = usize::try_from(self.quantum_frames).expect("u32 fits usize");
        for channel in 0..usize::try_from(self.channel_count).expect("u32 fits usize") {
            let offset = channel
                .checked_mul(quantum)
                .expect("prepared channel offset");
            output[offset..offset + frames]
                .copy_from_slice(&block.samples[offset..offset + frames]);
        }
        self.next_frame = SourceFrame(self.next_frame.0.saturating_add(u64::from(block.frames)));
        self.cumulative_read_frames = self
            .cumulative_read_frames
            .saturating_add(u64::from(block.frames));
        if block.end_of_region {
            self.end_of_region = true;
        }
        block.reset_metadata();
        self.recycle_block(block);
        u32::try_from(frames).expect("source block frame count is u32")
    }

    fn note_end_and_discard(&mut self, block: Box<TransferBlock>) {
        if block.end_of_region && block.generation == self.active_generation {
            self.note_end_frame(&block);
        }
        self.discard_block(block);
    }

    fn note_end_frame(&mut self, block: &TransferBlock) {
        self.end_frame = Some(SourceFrame(
            block.start_frame.0.saturating_add(u64::from(block.frames)),
        ));
    }

    fn discard_block(&mut self, mut block: Box<TransferBlock>) {
        self.stale_generation_discard_count = self.stale_generation_discard_count.saturating_add(1);
        block.reset_metadata();
        self.recycle_block(block);
    }

    fn recycle_block(&mut self, block: Box<TransferBlock>) {
        match self.recycle_producer.try_push(block) {
            Ok(()) => {}
            Err(QueueFull { value, .. }) => {
                self.deferred_recycle = Some(value);
            }
        }
    }

    fn flush_deferred_recycle(&mut self) {
        let Some(block) = self.deferred_recycle.take() else {
            return;
        };
        match self.recycle_producer.try_push(block) {
            Ok(()) => {}
            Err(QueueFull { value, .. }) => self.deferred_recycle = Some(value),
        }
    }

    fn end_reached(&self) -> bool {
        self.end_frame.is_some_and(|end| self.next_frame.0 >= end.0)
    }
}

fn queue_bytes<T>(capacity: NonZeroUsize) -> Result<u64, PcmSourceRingError> {
    let payload = bounded_spsc_retained_payload::<T>(capacity).map_err(map_spsc_error)?;
    u64::try_from(
        payload
            .total_bytes()
            .ok_or(PcmSourceRingError::ArithmeticOverflow)?,
    )
    .map_err(|_| PcmSourceRingError::PlatformSizeLimit)
}

fn largest_queue_allocation<T>(capacity: NonZeroUsize) -> Result<u64, PcmSourceRingError> {
    let payload = bounded_spsc_retained_payload::<T>(capacity).map_err(map_spsc_error)?;
    u64::try_from(payload.largest_allocation_bytes())
        .map_err(|_| PcmSourceRingError::PlatformSizeLimit)
}

fn checked_u64_add(left: u64, right: u64) -> Result<u64, PcmSourceRingError> {
    left.checked_add(right)
        .ok_or(PcmSourceRingError::ArithmeticOverflow)
}

fn checked_u64_mul(left: u64, right: u64) -> Result<u64, PcmSourceRingError> {
    left.checked_mul(right)
        .ok_or(PcmSourceRingError::ArithmeticOverflow)
}

const fn map_spsc_error(error: SpscError) -> PcmSourceRingError {
    match error {
        SpscError::CapacityOverflow => PcmSourceRingError::ArithmeticOverflow,
    }
}

/// One render-owned source endpoint moved into the graph fan-out wrapper.
pub struct SourceGraphSource {
    consumer: PcmSourceConsumer,
    resources: SourceResourceReport,
    /// Fixed native worker/decoder bytes not represented by the ring report.
    additional_overhead_bytes: u64,
    /// Largest fixed worker/decoder allocation, if larger than the ring allocation.
    additional_largest_allocation_bytes: u64,
    #[cfg(not(target_arch = "wasm32"))]
    retirement_worker: Option<NativeSourceWorker>,
}

impl SourceGraphSource {
    /// Construct one host-decoded source with no native worker retirement owner.
    #[must_use]
    pub fn new(
        consumer: PcmSourceConsumer,
        resources: SourceResourceReport,
        additional_overhead_bytes: u64,
        additional_largest_allocation_bytes: u64,
    ) -> Self {
        Self {
            consumer,
            resources,
            additional_overhead_bytes,
            additional_largest_allocation_bytes,
            #[cfg(not(target_arch = "wasm32"))]
            retirement_worker: None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_native_worker(
        consumer: PcmSourceConsumer,
        resources: SourceResourceReport,
        additional_overhead_bytes: u64,
        additional_largest_allocation_bytes: u64,
        retirement_worker: NativeSourceWorker,
    ) -> Self {
        Self {
            consumer,
            resources,
            additional_overhead_bytes,
            additional_largest_allocation_bytes,
            retirement_worker: Some(retirement_worker),
        }
    }
}

/// One immutable source-channel mapping to a graph track-input node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceGraphTrackMapping {
    pub node: GraphNodeId,
    pub source_index: usize,
    pub left_channel: u32,
    pub right_channel: u32,
}

/// Rejection while sealing source consumers and mappings for graph fan-out.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceGraphSourceSetError {
    EmptySources,
    SourceIndex,
    ChannelIndex,
    ArithmeticOverflow,
}

struct GraphSourceEntry {
    consumer: PcmSourceConsumer,
    channel_count: u32,
    planes: Box<[f32]>,
}

struct SourceGraphSourceSetDriver {
    /// Declared first so native workers stop/join before source consumers are dropped.
    #[cfg(not(target_arch = "wasm32"))]
    _retirement_workers: Box<[NativeSourceWorker]>,
    sources: Box<[GraphSourceEntry]>,
    mappings: Box<[SourceGraphTrackMapping]>,
    quantum_frames: u32,
}

fn allocation_class<T>(
    count: usize,
) -> Result<SourceRetainedAllocation, SourceGraphSourceSetError> {
    let layout =
        Layout::array::<T>(count).map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?;
    let count = u64::try_from(count).map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?;
    let bytes =
        u64::try_from(layout.size()).map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?;
    Ok(SourceRetainedAllocation {
        item_count: count,
        bytes,
        largest_allocation_bytes: bytes,
        alignment_bytes: u64::try_from(layout.align()).expect("alignment fits u64"),
    })
}

fn source_set_retained_resources(
    sources: &[SourceGraphSource],
    mappings: &[SourceGraphTrackMapping],
    quantum: usize,
) -> Result<SourceSetRetainedResourceReport, SourceGraphSourceSetError> {
    let source_entries = allocation_class::<GraphSourceEntry>(sources.len())?;
    let mappings_report = allocation_class::<SourceGraphTrackMapping>(mappings.len())?;
    let claims = allocation_class::<GraphSourceInputClaim>(mappings.len())?;
    let driver = allocation_class::<SourceGraphSourceSetDriver>(1)?;
    #[cfg(not(target_arch = "wasm32"))]
    let retirement_workers = allocation_class::<NativeSourceWorker>(
        sources
            .iter()
            .filter(|source| source.retirement_worker.is_some())
            .count(),
    )?;
    #[cfg(target_arch = "wasm32")]
    let retirement_workers = SourceRetainedAllocation {
        item_count: 0,
        bytes: 0,
        largest_allocation_bytes: 0,
        alignment_bytes: 1,
    };
    let mut planes = SourceRetainedAllocation {
        item_count: 0,
        bytes: 0,
        largest_allocation_bytes: 0,
        alignment_bytes: u64::try_from(align_of::<f32>()).expect("alignment fits u64"),
    };
    for source in sources {
        let samples = usize::try_from(source.consumer.channel_count())
            .map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?
            .checked_mul(quantum)
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        let class = allocation_class::<f32>(samples)?;
        planes.item_count = planes.item_count.saturating_add(1);
        planes.bytes = planes
            .bytes
            .checked_add(class.bytes)
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        planes.largest_allocation_bytes = planes
            .largest_allocation_bytes
            .max(class.largest_allocation_bytes);
    }
    let mut ids = SourceRetainedAllocation {
        item_count: 0,
        bytes: 0,
        largest_allocation_bytes: 0,
        alignment_bytes: 1,
    };
    for mapping in mappings {
        let GraphNodeId::TrackStage { track_id, .. } = &mapping.node else {
            return Err(SourceGraphSourceSetError::SourceIndex);
        };
        let bytes = u64::try_from(track_id.as_str().len())
            .map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?;
        ids.item_count = ids.item_count.saturating_add(2);
        ids.bytes = ids
            .bytes
            .checked_add(
                bytes
                    .checked_mul(2)
                    .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?,
            )
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        ids.largest_allocation_bytes = ids.largest_allocation_bytes.max(bytes);
    }
    Ok(SourceSetRetainedResourceReport {
        source_entries,
        mappings: mappings_report,
        claims,
        driver,
        retirement_workers,
        source_planes: planes,
        owned_stable_id_payloads: ids,
    })
}

impl GraphPreparedSourceSetDriver for SourceGraphSourceSetDriver {
    fn claim_count(&self) -> usize {
        self.mappings.len()
    }

    fn begin_block(
        &mut self,
        _first_sample: u64,
        frames: u32,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        if frames != self.quantum_frames {
            return Err(miso_engine_core::realtime::RenderError::InvalidEnvelope);
        }
        for source in &mut self.sources {
            source
                .consumer
                .read_block_contiguous(&mut source.planes)
                .map_err(|_| miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        }
        Ok(())
    }

    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        let mapping = self
            .mappings
            .get(claim_index)
            .ok_or(miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        let source = self
            .sources
            .get(mapping.source_index)
            .ok_or(miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        let quantum = usize::try_from(self.quantum_frames)
            .map_err(|_| miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        if left.len() != quantum || right.len() != quantum {
            return Err(miso_engine_core::realtime::RenderError::InvalidEnvelope);
        }
        let left_channel = usize::try_from(mapping.left_channel)
            .map_err(|_| miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        let right_channel = usize::try_from(mapping.right_channel)
            .map_err(|_| miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        if left_channel >= usize::try_from(source.channel_count).expect("u32 fits usize")
            || right_channel >= usize::try_from(source.channel_count).expect("u32 fits usize")
        {
            return Err(miso_engine_core::realtime::RenderError::InvalidEnvelope);
        }
        let left_offset = left_channel
            .checked_mul(quantum)
            .ok_or(miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        let right_offset = right_channel
            .checked_mul(quantum)
            .ok_or(miso_engine_core::realtime::RenderError::InvalidEnvelope)?;
        left.copy_from_slice(&source.planes[left_offset..left_offset + quantum]);
        right.copy_from_slice(&source.planes[right_offset..right_offset + quantum]);
        Ok(())
    }

    fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
        let mut written = 0;
        for source in &self.sources {
            let telemetry = source.consumer.telemetry();
            for value in [
                telemetry.cumulative_read_frames,
                telemetry.stale_generation_discard_count,
                telemetry.underrun_frames,
                telemetry.underrun_events,
                telemetry.native_decoder_sanitized_samples,
            ] {
                let Some(slot) = output.get_mut(written) else {
                    return written;
                };
                *slot = value;
                written += 1;
            }
        }
        written
    }
}

/// Seal one or more prepared source consumers into a graph-owned fan-out source set.
///
/// The wrapper owns exactly one consumer and preallocated contiguous source planes per source.
/// The graph calls `begin_block` once, then copies each declared mapping without allowing native
/// workers to observe a consumer or ring.
pub fn prepare_graph_source_set(
    envelope: miso_engine_core::realtime::RenderEnvelope,
    sources: Vec<SourceGraphSource>,
    mappings: Vec<SourceGraphTrackMapping>,
) -> Result<GraphPreparedSourceSet, SourceGraphSourceSetError> {
    if sources.is_empty() {
        return Err(SourceGraphSourceSetError::EmptySources);
    }
    let quantum = usize::try_from(envelope.quantum.0)
        .map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?;
    let retained = source_set_retained_resources(&sources, &mappings, quantum)?;
    let mut pcm_payload = 0_u64;
    let mut overhead = 0_u64;
    let mut largest = 0_u64;
    let mut entries = Vec::with_capacity(sources.len());
    #[cfg(not(target_arch = "wasm32"))]
    let mut workers = Vec::with_capacity(
        usize::try_from(retained.retirement_workers.item_count)
            .map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?,
    );
    for source in sources {
        let channel_count = source.consumer.channel_count();
        let samples = usize::try_from(channel_count)
            .map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?
            .checked_mul(quantum)
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        pcm_payload = pcm_payload
            .checked_add(source.resources.pcm_payload_already_charged_bytes)
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        overhead = overhead
            .checked_add(source.resources.overhead_bytes)
            .and_then(|value| value.checked_add(source.additional_overhead_bytes))
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        largest = largest
            .max(source.resources.largest_allocation_bytes)
            .max(source.additional_largest_allocation_bytes);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(worker) = source.retirement_worker {
            workers.push(worker);
        }
        entries.push(GraphSourceEntry {
            consumer: source.consumer,
            channel_count,
            planes: vec![0.0; samples].into_boxed_slice(),
        });
    }
    overhead = overhead
        .checked_add(
            retained
                .overhead_bytes()
                .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?,
        )
        .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
    largest = largest.max(retained.largest_allocation_bytes());
    for mapping in &mappings {
        let source = entries
            .get(mapping.source_index)
            .ok_or(SourceGraphSourceSetError::SourceIndex)?;
        if mapping.left_channel >= source.channel_count
            || mapping.right_channel >= source.channel_count
        {
            return Err(SourceGraphSourceSetError::ChannelIndex);
        }
    }
    let total = pcm_payload
        .checked_add(overhead)
        .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
    let claims = mappings
        .iter()
        .map(|mapping| GraphSourceInputClaim {
            node: mapping.node.clone(),
        })
        .collect();
    Ok(GraphPreparedSourceSet::new(
        envelope,
        claims,
        GraphSourceSetResourceReport {
            pcm_payload_already_charged_bytes: pcm_payload,
            overhead_bytes: overhead,
            total_engine_owned_bytes: total,
            largest_allocation_bytes: largest,
        },
        Box::new(SourceGraphSourceSetDriver {
            #[cfg(not(target_arch = "wasm32"))]
            _retirement_workers: workers.into_boxed_slice(),
            sources: entries.into_boxed_slice(),
            mappings: mappings.into_boxed_slice(),
            quantum_frames: envelope.quantum.0,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: SampleRateHz = SampleRateHz(48_000);

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn set_driver_declares_worker_tokens_before_source_consumers() {
        let source = include_str!("lib.rs");
        let start = source
            .find("struct SourceGraphSourceSetDriver")
            .expect("set driver declaration");
        let body = &source[start..];
        let workers = body
            .find("_retirement_workers:")
            .expect("worker token field");
        let sources = body.find("sources:").expect("consumer field");
        assert!(
            workers < sources,
            "worker tokens must drop before source consumers"
        );
    }

    fn config(channels: u32, quantum: u32, capacity: u64) -> PcmSourceRingConfig {
        PcmSourceRingConfig {
            channel_count: channels,
            quantum_frames: QuantumFrames(quantum),
            frame_capacity: capacity,
            initial_generation: SourceGeneration(1),
        }
    }

    fn chunk<'a>(
        generation: u64,
        start: u64,
        planes: &'a [&'a [f32]],
        frames: u32,
        end: bool,
    ) -> HostPlanarChunk<'a> {
        HostPlanarChunk {
            sample_rate_hz: RATE,
            generation: SourceGeneration(generation),
            start_frame: SourceFrame(start),
            planes,
            frames,
            end_of_region: end,
        }
    }

    #[test]
    fn report_separates_session_pcm_from_source_overhead() {
        let report = PcmSourceRing::resource_report(config(2, 4, 8)).expect("report");
        assert_eq!(report.transfer_block_count, 2);
        assert_eq!(report.pcm_payload_already_charged_bytes, 64);
        assert_eq!(report.transfer_block_pcm_bytes, 64);
        assert!(report.overhead_bytes > 0);
        assert_eq!(
            report.total_engine_owned_bytes,
            report.pcm_payload_already_charged_bytes + report.overhead_bytes
        );
        assert!(report.largest_allocation_bytes >= 32);
    }

    #[test]
    fn prepare_rejects_invalid_fixed_ring_shape() {
        assert!(matches!(
            PcmSourceRing::prepare(config(0, 4, 4)),
            Err(PcmSourceRingError::ZeroChannelCount)
        ));
        assert!(matches!(
            PcmSourceRing::prepare(config(2, 0, 4)),
            Err(PcmSourceRingError::ZeroQuantumFrames)
        ));
        assert!(matches!(
            PcmSourceRing::prepare(config(2, 4, 6)),
            Err(PcmSourceRingError::CapacityNotQuantumMultiple)
        ));
    }

    #[test]
    fn host_region_preparation_preserves_resources_and_absolute_ownership() {
        let config = config(1, 4, 8);
        let (_, _, zero_report) =
            PcmSourceRing::prepare_host_region(config, SourceFrame(0)).expect("zero origin");
        let (_, _, one_report) =
            PcmSourceRing::prepare_host_region(config, SourceFrame(1)).expect("one origin");
        let (producer, mut consumer, session_report) =
            PcmSourceRing::prepare_host_region(config, SourceFrame(48_123))
                .expect("session origin");
        assert_eq!(zero_report, PcmSourceRing::resource_report(config).unwrap());
        assert_eq!(one_report, zero_report);
        assert_eq!(session_report, zero_report);

        let mut host = producer.into_host_chunk_provider(RATE);
        let samples = [1.0, 2.0, 3.0, 4.0];
        let planes = [&samples[..]];
        assert!(matches!(
            host.submit(chunk(1, 0, &planes, 4, false)),
            Err(HostChunkError::NonContiguous {
                expected: SourceFrame(48_123),
                actual: SourceFrame(0),
            })
        ));
        assert_eq!(host.telemetry().cumulative_written_frames, 0);
        host.submit(chunk(1, 48_123, &planes, 4, false))
            .expect("first absolute chunk after rejection");
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(72_001),
        })
        .expect("newer seek");
        let sought = [9.0, 8.0, 7.0, 6.0];
        host.submit(chunk(2, 72_001, &[&sought], 4, true))
            .expect("post-seek chunk");
        let mut output = [0.0; 4];
        let mut planes = [&mut output[..]];
        let report = consumer.read_block(&mut planes).expect("consumer retained");
        assert_eq!(report.active_generation, SourceGeneration(2));
        assert_eq!(output, sought);
    }

    #[test]
    fn host_submission_is_fifo_wraparound_and_never_accepts_a_prefix() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(2, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let left_a = [1.0, 2.0, 3.0, 4.0];
        let right_a = [11.0, 12.0, 13.0, 14.0];
        let planes_a = [&left_a[..], &right_a[..]];
        host.submit(chunk(1, 0, &planes_a, 4, false))
            .expect("first");
        let left_b = [5.0, 6.0, 7.0, 8.0];
        let right_b = [15.0, 16.0, 17.0, 18.0];
        let planes_b = [&left_b[..], &right_b[..]];
        assert!(matches!(
            host.submit(chunk(1, 4, &planes_b, 4, false)),
            Err(HostChunkError::Full { .. })
        ));
        assert_eq!(host.telemetry().cumulative_written_frames, 4);
        let mut left_out = [99.0; 4];
        let mut right_out = [99.0; 4];
        let report = {
            let mut output = [&mut left_out[..], &mut right_out[..]];
            consumer.read_block(&mut output).expect("render")
        };
        assert_eq!(report.copied_frames, 4);
        assert_eq!(left_out, left_a);
        assert_eq!(right_out, right_a);
        host.submit(chunk(1, 4, &planes_b, 4, false)).expect("wrap");
        {
            let mut output = [&mut left_out[..], &mut right_out[..]];
            consumer.read_block(&mut output).expect("second render");
        }
        assert_eq!(left_out, left_b);
        assert_eq!(right_out, right_b);
    }

    #[test]
    fn underrun_is_positive_zero_and_eof_is_not_an_underrun() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let mut output_plane = [-1.0; 4];
        let missing = {
            let mut output = [&mut output_plane[..]];
            consumer.read_block(&mut output).expect("missing")
        };
        assert_eq!(output_plane.map(f32::to_bits), [0; 4]);
        assert_eq!(missing.underrun_frames, 4);
        assert!(missing.underrun_event);
        let delayed = [0.0, 0.0, 0.0, 0.0];
        host.submit(chunk(1, 0, &[&delayed], 4, false))
            .expect("late source frame");
        let final_plane = [0.25, -0.5];
        let planes = [&final_plane[..]];
        host.submit(chunk(1, 4, &planes, 2, true)).expect("final");
        let eof = {
            let mut output = [&mut output_plane[..]];
            consumer.read_block(&mut output).expect("eof")
        };
        assert_eq!(eof.copied_frames, 2);
        assert_eq!(eof.underrun_frames, 0);
        assert!(eof.end_of_region);
        assert_eq!(output_plane[..2], final_plane);
        assert_eq!(
            output_plane[2..]
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            [0; 2]
        );
        let after_eof = {
            let mut output = [&mut output_plane[..]];
            consumer.read_block(&mut output).expect("after eof")
        };
        assert_eq!(after_eof.underrun_frames, 0);
        assert_eq!(after_eof.cumulative_underrun_events, 1);
    }

    #[test]
    fn seek_switches_at_boundary_and_discards_older_queued_audio() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 12)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let old_a = [1.0, 1.0, 1.0, 1.0];
        let old_b = [2.0, 2.0, 2.0, 2.0];
        host.submit(chunk(1, 0, &[&old_a], 4, false))
            .expect("old a");
        host.submit(chunk(1, 4, &[&old_b], 4, false))
            .expect("old b");
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(100),
        })
        .expect("seek");
        let fresh = [9.0, 8.0, 7.0, 6.0];
        host.submit(chunk(2, 100, &[&fresh], 4, true))
            .expect("fresh");
        let mut output_plane = [0.0; 4];
        let mut output = [&mut output_plane[..]];
        let report = consumer.read_block(&mut output).expect("new generation");
        assert_eq!(report.active_generation, SourceGeneration(2));
        assert_eq!(output_plane, fresh);
        assert_eq!(consumer.telemetry().stale_generation_discard_count, 2);
        assert!(matches!(
            host.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(0),
            }),
            Err(SourceSeekError::GenerationNotStrictlyIncreasing { .. })
        ));
    }

    #[test]
    fn registry_and_host_shape_errors_are_stable() {
        assert_eq!(
            SourceDiagnosticCode::RateMismatch.as_str(),
            "source.rate.mismatch"
        );
        assert_eq!(
            SourceDiagnosticPath::for_source("lead.vocal").as_str(),
            "$.sources[id=lead.vocal]"
        );
        let (producer, _consumer, _) = PcmSourceRing::prepare(config(2, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let mono = [0.0; 4];
        assert!(matches!(
            host.submit(chunk(1, 0, &[&mono], 4, false)),
            Err(HostChunkError::ChannelCount {
                expected: 2,
                actual: 1
            })
        ));
        assert!(matches!(
            host.submit(HostPlanarChunk {
                sample_rate_hz: SampleRateHz(44_100),
                ..chunk(1, 0, &[&mono, &mono], 4, false)
            }),
            Err(HostChunkError::WrongSampleRate { .. })
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn prepared_contiguous_native_submission_matches_planar_ring_shape() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(2, 4, 4)).expect("ring");
        assert_eq!(producer.shape(), consumer.shape());
        assert_eq!(producer.shape().frame_capacity, 4);
        assert_eq!(consumer.shape().transfer_block_count, 1);
        let mut provider = producer.into_host_chunk_provider(RATE);
        let planar_quantum = [1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0];
        provider
            .submit_native_planar(
                SourceGeneration(1),
                SourceFrame(0),
                &planar_quantum,
                4,
                true,
                0,
            )
            .expect("native planar submit");
        let mut left = [0.0; 4];
        let mut right = [0.0; 4];
        let mut output = [&mut left[..], &mut right[..]];
        let report = consumer.read_block(&mut output).expect("read");
        assert_eq!(report.copied_frames, 4);
        assert_eq!(left, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(right, [-1.0, -2.0, -3.0, -4.0]);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn stamped_native_watermark_survives_seek_stale_discard_and_saturates() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 12)).expect("ring");
        let mut provider = producer.into_host_chunk_provider(RATE);
        let old = [1.0_f32; 4];
        provider
            .submit_native_planar(SourceGeneration(1), SourceFrame(0), &old, 4, false, 7)
            .expect("old native block");
        provider
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(100),
            })
            .expect("seek");
        let fresh = [2.0_f32; 4];
        provider
            .submit_native_planar(
                SourceGeneration(2),
                SourceFrame(100),
                &fresh,
                4,
                false,
                u64::MAX,
            )
            .expect("fresh native block");
        let wrapped = [3.0_f32; 4];
        provider
            .submit_native_planar(
                SourceGeneration(2),
                SourceFrame(104),
                &wrapped,
                4,
                true,
                u64::MAX,
            )
            .expect("wrapped native block");
        let mut output = [0.0_f32; 4];
        consumer
            .read_block(&mut [&mut output])
            .expect("fresh read after stale discard");
        assert_eq!(output, fresh);
        assert_eq!(consumer.telemetry().stale_generation_discard_count, 1);
        assert_eq!(
            consumer.telemetry().native_decoder_sanitized_samples,
            u64::MAX
        );
        consumer
            .read_block(&mut [&mut output])
            .expect("wrapped read");
        assert_eq!(output, wrapped);
        assert_eq!(
            consumer.telemetry().native_decoder_sanitized_samples,
            u64::MAX
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn one_four_channel_source_fans_out_to_three_inputs_in_sequential_and_native_fallback() {
        use core::num::NonZeroUsize;
        use miso_engine_core::{
            QuantumFrames,
            realtime::{PlanarBufferMut, RenderEnvelope, RenderIo, RenderTime},
        };
        use miso_engine_effect_contract::{LatencySamples, TailSamples};
        use miso_engine_graph::{
            DependencyLevel, GraphEdge, GraphEdgeId, GraphNode, GraphPortId, GraphPortKind,
            GraphResourceEstimate, GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec,
            NativeGraphBindConfigV1, NativeGraphRenderModeV1, NativeSchedulerConfigV1,
            PreparedGraphPlan, PreparedGraphPlanParts, StableGraphId, TrackStage,
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
            sample_rate: RATE,
            quantum: QuantumFrames(2),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two"),
        };
        let track = |id| miso_engine_graph::GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(id).expect("track ID"),
            stage: TrackStage::Input,
        };
        let inputs = [track("a"), track("b"), track("c")];
        let output = miso_engine_graph::GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };
        let edge = |source: miso_engine_graph::GraphNodeId, route: &str| GraphEdge {
            id: GraphEdgeId::RouteSource {
                route_id: StableGraphId::parse(route).expect("route ID"),
            },
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: output.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: format!("$.{route}"),
        };
        let schedule = vec![
            inputs[0].clone(),
            inputs[1].clone(),
            inputs[2].clone(),
            output.clone(),
        ];
        let make_plan = || {
            PreparedGraphPlan::new(PreparedGraphPlanParts {
                plan_id: 10,
                spec: GraphSpec {
                    nodes: schedule
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: LatencySamples(0),
                            tail: TailSamples::Finite(0),
                        })
                        .collect(),
                    ports: Vec::new(),
                    edges: vec![
                        edge(inputs[0].clone(), "a"),
                        edge(inputs[1].clone(), "b"),
                        edge(inputs[2].clone(), "c"),
                    ],
                },
                sequential_schedule: schedule.clone(),
                dependency_levels: vec![
                    DependencyLevel {
                        level: 0,
                        nodes: inputs.to_vec(),
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
                required_bindings: [
                    inputs[0].clone(),
                    inputs[1].clone(),
                    inputs[2].clone(),
                    output.clone(),
                ]
                .to_vec(),
                routes: Vec::new(),
                effects: Vec::new(),
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers: Vec::new(),
            })
        };
        let mapping = |node, left_channel, right_channel| SourceGraphTrackMapping {
            node,
            source_index: 0,
            left_channel,
            right_channel,
        };
        let normal_mappings = || {
            vec![
                mapping(inputs[0].clone(), 0, 1),
                mapping(inputs[1].clone(), 3, 2),
                mapping(inputs[2].clone(), 0, 2),
            ]
        };
        let make_source_set = |mappings| {
            let config = PcmSourceRingConfig {
                channel_count: 4,
                quantum_frames: QuantumFrames(2),
                frame_capacity: 2,
                initial_generation: SourceGeneration(1),
            };
            let (producer, consumer, resources) = PcmSourceRing::prepare(config).expect("ring");
            let mut host = producer.into_host_chunk_provider(RATE);
            let c0 = [1.0_f32, 1.0];
            let c1 = [2.0_f32, 2.0];
            let c2 = [4.0_f32, 4.0];
            let c3 = [8.0_f32, 8.0];
            host.submit(HostPlanarChunk {
                sample_rate_hz: RATE,
                generation: SourceGeneration(1),
                start_frame: SourceFrame(0),
                planes: &[&c0, &c1, &c2, &c3],
                frames: 2,
                end_of_region: true,
            })
            .expect("source PCM");
            prepare_graph_source_set(
                envelope,
                vec![SourceGraphSource::new(consumer, resources, 0, 0)],
                mappings,
            )
            .expect("source set")
        };
        let bindings = || GraphRuntimeBindings {
            envelope,
            nodes: vec![miso_engine_graph::GraphNodeBinding::new(
                output.clone(),
                Box::new(Noop),
            )],
            observers: Vec::new(),
        };
        let assert_transactional_rejection = |source_set, graph_bindings| match make_plan()
            .bind_with_source_set(graph_bindings, source_set)
        {
            Ok(_) => panic!("invalid source claims unexpectedly bound"),
            Err(failure) => {
                assert_eq!(failure.code, "source.graph.binding_mismatch");
                assert!(!failure.source_set.claims().is_empty());
            }
        };
        assert_transactional_rejection(
            make_source_set(vec![
                mapping(inputs[0].clone(), 0, 1),
                mapping(inputs[1].clone(), 3, 2),
            ]),
            bindings(),
        );
        assert_transactional_rejection(
            make_source_set(vec![
                mapping(inputs[0].clone(), 0, 1),
                mapping(inputs[1].clone(), 3, 2),
                mapping(inputs[2].clone(), 0, 2),
                mapping(track("unexpected"), 0, 1),
            ]),
            bindings(),
        );
        assert_transactional_rejection(
            make_source_set(vec![
                mapping(inputs[0].clone(), 0, 1),
                mapping(inputs[0].clone(), 3, 2),
                mapping(inputs[1].clone(), 3, 2),
                mapping(inputs[2].clone(), 0, 2),
            ]),
            bindings(),
        );
        assert_transactional_rejection(
            make_source_set(normal_mappings()),
            GraphRuntimeBindings {
                envelope,
                nodes: vec![
                    miso_engine_graph::GraphNodeBinding::new(output.clone(), Box::new(Noop)),
                    miso_engine_graph::GraphNodeBinding::new(inputs[0].clone(), Box::new(Noop)),
                ],
                observers: Vec::new(),
            },
        );
        let mut sequential = match make_plan()
            .bind_with_source_set(bindings(), make_source_set(normal_mappings()))
        {
            Ok(plan) => plan,
            Err(failure) => panic!("sequential bind failed: {}", failure.code),
        };
        let mut native = match make_plan().bind_native_with_source_set(
            bindings(),
            NativeGraphBindConfigV1 {
                render_mode: NativeGraphRenderModeV1::SingleThread,
                scheduler: NativeSchedulerConfigV1::new(NonZeroUsize::new(1).expect("lane"), false),
                maximum_retained_bytes: 1 << 20,
            },
            make_source_set(normal_mappings()),
        ) {
            Ok(plan) => plan.into_plan(),
            Err(failure) => panic!("native bind failed: {}", failure.code),
        };
        let mut sequential_pcm = [0.0_f32; 4];
        let mut native_pcm = [0.0_f32; 4];
        for plan_and_pcm in [
            (&mut sequential, &mut sequential_pcm),
            (&mut native, &mut native_pcm),
        ] {
            plan_and_pcm
                .0
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(plan_and_pcm.1, 2, 2, 2).expect("output"),
                    },
                    RenderTime { absolute_sample: 0 },
                )
                .expect("render");
        }
        assert_eq!(sequential_pcm, [10.0, 10.0, 10.0, 10.0]);
        assert_eq!(
            native_pcm.map(f32::to_bits),
            sequential_pcm.map(f32::to_bits)
        );
    }
}
