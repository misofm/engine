//! Bounded, prepared PCM source rings for just-in-time audio delivery.
//!
//! A source ring splits ownership between one non-render producer and one exclusive render
//! consumer. Transfer blocks, queues, and their PCM storage are all created by [`PcmSourceRing`]
//! before rendering starts. The consumer only moves prepared blocks, copies or lends their samples
//! in place (zeroing a short block's tail once), and updates local saturating counters.

#![allow(missing_docs)]

use core::{alloc::Layout, cmp, mem::size_of, num::NonZeroUsize};
use std::fmt;

use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{
        Consumer, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError, bounded_spsc,
        bounded_spsc_move, bounded_spsc_retained_payload,
    },
};
use graph::{
    GraphNodeId, GraphObservationValidity, GraphPreparedSourceSet, GraphPreparedSourceSetDriver,
    GraphSourceInputClaim, GraphSourceSetResourceReport,
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
    /// Switch to a strictly newer generation so that `frame` enters the graph in the block that
    /// starts at absolute render sample `anchor_sample` (issue #1274).
    ///
    /// `anchor_sample` is on the plan's absolute render clock -- the first sample the graph hands
    /// its source set each block, which every plan swap continues -- and must be a multiple of the
    /// quantum. The output hears `frame` the plan's latency later. The render consumer holds the
    /// seek until the first block starting at or after the anchor; observed late, it starts at
    /// `frame + (block start - anchor_sample)`, so the source is in time either way. While the
    /// seek is held, already queued PCM of the playing generation keeps playing and the new
    /// generation's first submitted block waits as the pending block, so a host may prime the
    /// ring from `frame` as soon as the seek is accepted.
    SeekAt {
        /// The nonzero generation to make audible.
        generation: SourceGeneration,
        /// Decoded source frame that enters the graph at `anchor_sample`.
        frame: SourceFrame,
        /// Absolute render sample of the block `frame` enters in; a multiple of the quantum.
        anchor_sample: u64,
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

/// A stable path to the source collection or to one source declaration selected by stable ID.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SourceDiagnosticPath(String);

impl SourceDiagnosticPath {
    /// Construct the source-collection diagnostic path.
    #[must_use]
    pub fn for_sources_collection() -> Self {
        Self("$.sources".to_owned())
    }

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
///
/// The ring allocates `transfer_block_count + retained_block_count` blocks and sizes both queues
/// at that count (#917). The retained block is the one the render consumer holds outside both
/// queues at every block boundary (see [`PcmSourceConsumer`]); the session did not charge it, so
/// its PCM (`retained_block_pcm_bytes`), its metadata and its two queue slots are all overhead.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceResourceReport {
    /// Configured transfer blocks: `frame_capacity / quantum_frames`, the producer's admission
    /// depth.
    pub transfer_block_count: u64,
    /// Blocks allocated beyond the configured count for the consumer to retain; always 1.
    pub retained_block_count: u64,
    /// PCM payload already charged by the session source declaration.
    pub pcm_payload_already_charged_bytes: u64,
    /// Data SPSC header plus slots, sized at the allocated block count.
    pub data_queue_bytes: u64,
    /// Recycle SPSC header plus slots, sized at the allocated block count.
    pub recycle_queue_bytes: u64,
    /// One-slot seek-command SPSC header plus slots.
    pub command_queue_bytes: u64,
    /// One `TransferBlock` allocation per allocated block (configured plus retained), excluding
    /// its PCM allocation.
    pub transfer_block_metadata_bytes: u64,
    /// PCM bytes in the configured transfer blocks; exactly equals the session PCM charge.
    pub transfer_block_pcm_bytes: u64,
    /// PCM bytes in the retained block, charged to `overhead_bytes`.
    pub retained_block_pcm_bytes: u64,
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
    pub owned_stable_id_payloads: SourceRetainedAllocation,
}

impl SourceSetRetainedResourceReport {
    fn overhead_bytes(self) -> Option<u64> {
        [
            self.source_entries.bytes,
            self.mappings.bytes,
            self.claims.bytes,
            self.driver.bytes,
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
    /// A [`SourceCommand::SeekAt`] anchor that is not a multiple of the render quantum: no block
    /// starts there.
    AnchorUnaligned,
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
    /// Whether a seek generation was applied at this block boundary.
    pub generation_changed: bool,
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
        let data_queue = queue_bytes::<Box<TransferBlock>>(shape.allocated_block_count)?;
        let recycle_queue = queue_bytes::<Box<TransferBlock>>(shape.allocated_block_count)?;
        let command_capacity = NonZeroUsize::new(1).expect("one command slot");
        let command_queue = queue_bytes::<SourceCommand>(command_capacity)?;
        let pcm = checked_u64_mul(config.frame_capacity, u64::from(config.channel_count))?
            .checked_mul(
                u64::try_from(size_of::<f32>())
                    .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            )
            .ok_or(PcmSourceRingError::ArithmeticOverflow)?;
        // `frame_capacity` is a whole number of quanta, so this division is exact.
        let block_pcm = pcm / shape.transfer_block_count_u64;
        let retained_block_count = u64::try_from(RETAINED_TRANSFER_BLOCKS)
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
        let retained_pcm = checked_u64_mul(block_pcm, retained_block_count)?;
        let metadata = checked_u64_mul(
            shape.allocated_block_count_u64,
            u64::try_from(size_of::<TransferBlock>())
                .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
        )?;
        let overhead = checked_u64_add(
            checked_u64_add(
                checked_u64_add(data_queue, recycle_queue)?,
                checked_u64_add(command_queue, metadata)?,
            )?,
            retained_pcm,
        )?;
        let total = checked_u64_add(pcm, overhead)?;
        let largest = [
            block_pcm,
            u64::try_from(size_of::<TransferBlock>())
                .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            largest_queue_allocation::<Box<TransferBlock>>(shape.allocated_block_count)?,
            largest_queue_allocation::<SourceCommand>(command_capacity)?,
        ]
        .into_iter()
        .max()
        .expect("nonempty exact allocation list");
        Ok(SourceResourceReport {
            transfer_block_count: shape.transfer_block_count_u64,
            retained_block_count,
            pcm_payload_already_charged_bytes: pcm,
            data_queue_bytes: data_queue,
            recycle_queue_bytes: recycle_queue,
            command_queue_bytes: command_queue,
            transfer_block_metadata_bytes: metadata,
            transfer_block_pcm_bytes: pcm,
            retained_block_pcm_bytes: retained_pcm,
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
        // Both queues hold every allocated block, so no push to either can ever be refused: a
        // block being pushed is not in the queue it is pushed to.
        let (data_producer, data_consumer) =
            bounded_spsc_move(shape.allocated_block_count, queue_generation)
                .map_err(map_spsc_error)?;
        let (recycle_producer, recycle_consumer) =
            bounded_spsc_move(shape.allocated_block_count, queue_generation)
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
            played: None,
            played_frames: 0,
            retained_idle: Some(Box::new(TransferBlock::try_new(shape.samples_per_block)?)),
            deferred_recycle: None,
            cumulative_read_frames: 0,
            stale_generation_discard_count: 0,
            underrun_frames: 0,
            underrun_events: 0,
            native_decoder_sanitized_samples: 0,
            generation_changed: false,
            held_seek: None,
        };
        // The producer starts with exactly the configured blocks; the retained block above never
        // enters the recycle queue until the consumer has a played block to hold instead.
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

/// Transfer blocks the render consumer *retains* outside both queues at every block boundary:
/// the played block while one is readable, otherwise the same storage idle (#917). This is in
/// addition to the pre-fetched `current` block the consumer already held before #917 whenever its
/// `start_frame` is ahead of `next_frame` (a gap after a seek or an underrun), or a held anchored
/// seek's pending block (#1274), which is of another generation and may start behind `next_frame`.
const RETAINED_TRANSFER_BLOCKS: usize = 1;

struct PreparedShape {
    /// Configured blocks: the producer's admission depth.
    transfer_block_count: NonZeroUsize,
    transfer_block_count_u64: u64,
    /// Configured plus retained blocks: what is allocated and what each queue can hold.
    allocated_block_count: NonZeroUsize,
    allocated_block_count_u64: u64,
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
        let allocated_block_count = transfer_block_count
            .checked_add(RETAINED_TRANSFER_BLOCKS)
            .ok_or(PcmSourceRingError::ArithmeticOverflow)?;
        let allocated_block_count_u64 = u64::try_from(allocated_block_count.get())
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
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
            allocated_block_count,
            allocated_block_count_u64,
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

    /// Request a strictly newer generation; it becomes audible only on the next render block, or,
    /// for [`SourceCommand::SeekAt`], on the block its anchor names.
    ///
    /// Either command switches this producer to the new generation at once, so the host submits
    /// that generation's PCM from `frame` straight away.
    pub fn try_seek(&mut self, command: SourceCommand) -> Result<(), SourceSeekError> {
        let (generation, frame, anchor_sample) = match command {
            SourceCommand::Seek { generation, frame } => (generation, frame, None),
            SourceCommand::SeekAt {
                generation,
                frame,
                anchor_sample,
            } => (generation, frame, Some(anchor_sample)),
        };
        if !generation.is_valid() {
            return Err(SourceSeekError::GenerationZero);
        }
        if generation <= self.active_generation {
            return Err(SourceSeekError::GenerationNotStrictlyIncreasing {
                active: self.active_generation,
                requested: generation,
            });
        }
        if anchor_sample
            .is_some_and(|anchor| !anchor.is_multiple_of(u64::from(self.quantum_frames)))
        {
            return Err(SourceSeekError::AnchorUnaligned);
        }
        match self.command_producer.try_push(command) {
            Ok(()) => {
                self.active_generation = generation;
                self.next_write_frame = frame;
                self.end_of_region_submitted = false;
                Ok(())
            }
            Err(QueueFull { full_count, .. }) => Err(SourceSeekError::Backpressure { full_count }),
        }
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
        let quantum = usize::try_from(self.quantum_frames).expect("prepared quantum fits usize");
        let mut block = self.take_recycled_block()?;
        block.generation = chunk.generation;
        block.start_frame = chunk.start_frame;
        block.frames = chunk.frames;
        block.end_of_region = chunk.end_of_region;
        block.native_decoder_sanitized_samples = 0;
        let frames = usize::try_from(chunk.frames).expect("u32 fits usize");
        for (channel, plane) in chunk.planes.iter().enumerate() {
            let offset = channel
                .checked_mul(quantum)
                .expect("prepared channel offset");
            block.samples[offset..offset + frames].copy_from_slice(&plane[..frames]);
        }
        self.publish_block(block, chunk.frames, chunk.end_of_region)
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
///
/// # Played-block ownership (#917)
///
/// The played block is the consumer's from `begin_block` until the next `begin_block`,
/// `prepare_seek`, `end_block`, or drop. Until then [`Self::played_plane`] can borrow its planes
/// in place and [`Self::copy_channel`] can copy them, as often as the render needs.
///
/// Its storage stays with the consumer after that point too. The ring allocates one block beyond
/// the configured `transfer_block_count`, and at every block boundary the consumer retains exactly
/// one block outside both queues -- the played block while one is readable, otherwise the same
/// storage idle (after an underrun, the end of the region, `end_block` or `prepare_seek`) -- in
/// addition to the pre-fetched `current` block it already held before #917 at the boundaries
/// where `current.start_frame` is ahead of `next_frame`, or where `current` is a held anchored
/// seek's pending block (#1274), of another generation and possibly behind `next_frame`. The hold is therefore "what it held
/// before, plus one", which is what keeps the producer's admission sequence unchanged. The
/// idle storage goes back to the producer's recycle queue only inside `begin_block`, at the moment
/// a newer block becomes the played block. So:
///
/// * a block is never recycled while a `played_plane` borrow is live: the only release point takes
///   `&mut self`, and a block is only ever reached by the producer through the recycle queue;
/// * the producer's admission depth is the configured `transfer_block_count` at every block
///   boundary, exactly as when the consumer recycled the played block at the end of the render;
///   and the producer is no longer held one block short while a render reads its block;
/// * the extra block never reaches the data queue except through an ordinary producer
///   submission, whose ack still follows the data-queue push, and no push to either queue can be
///   refused because each queue can hold every allocated block.
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
    played: Option<Box<TransferBlock>>,
    played_frames: u32,
    /// The retained block while no block is played; `Some` exactly when `played` is `None`.
    retained_idle: Option<Box<TransferBlock>>,
    deferred_recycle: Option<Box<TransferBlock>>,
    cumulative_read_frames: u64,
    stale_generation_discard_count: u64,
    underrun_frames: u64,
    underrun_events: u64,
    native_decoder_sanitized_samples: u64,
    generation_changed: bool,
    /// An observed [`SourceCommand::SeekAt`] whose anchor block has not begun yet (issue #1274).
    held_seek: Option<HeldSeek>,
}

/// A [`SourceCommand::SeekAt`] the consumer has popped and holds until its anchor block.
#[derive(Clone, Copy, Debug)]
struct HeldSeek {
    generation: SourceGeneration,
    frame: SourceFrame,
    anchor_sample: u64,
}

/// When an observed anchored seek may apply.
#[derive(Clone, Copy)]
enum SeekClock {
    /// At once, as a plain seek to its frame: a caller that supplies no block time.
    Now,
    /// In the block starting at this absolute render sample, if it is at or past the anchor.
    At(u64),
    /// Not now: preparation between blocks, which has no render time.
    Hold,
}

impl PcmSourceConsumer {
    /// Apply an already-admitted seek on the exclusive consumer owner between blocks.
    /// Recycles stale storage and retains current-generation PCM without consuming a frame.
    /// Producers still only enqueue commands; this is not a shared controller handle.
    ///
    /// An anchored seek ([`SourceCommand::SeekAt`]) has no block time here: it is held, never
    /// applied or dropped, until the render reaches its anchor, and this returns `false`.
    pub fn prepare_seek(&mut self, generation: SourceGeneration, frame: SourceFrame) -> bool {
        self.end_block();
        self.flush_deferred_recycle();
        // The prepared source command queue has one slot. Observe exactly that admitted
        // command, then check the requested identity before granting readiness.
        self.observe_seek_at_block_boundary(SeekClock::Hold);
        // The held-seek test is not redundant: a caller may prepare the playing generation again
        // while a newer anchored seek is held, and that is not ready.
        if self.held_seek.is_some()
            || self.active_generation != generation
            || self.next_frame != frame
        {
            return false;
        }
        self.acquire_current_block(SeekClock::Hold);
        true
    }

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
        let report = self.begin_block();
        let copied = (|| {
            for (channel, output) in output_planes.iter_mut().enumerate() {
                self.copy_channel(
                    u32::try_from(channel).expect("prepared channel count fits u32"),
                    output,
                )?;
            }
            Ok(())
        })();
        self.end_block();
        copied.map(|()| report)
    }

    /// Advance the source state machine once and retain this quantum for channel fan-out reads.
    /// The previously played block stops being readable here. A short (end-of-region) block has
    /// its tail zeroed in place, once, so every [`Self::played_plane`] is a whole quantum.
    /// This render-path operation allocates nothing and never blocks.
    ///
    /// Without a block time an anchored seek applies at once, as a plain seek to its frame; the
    /// graph calls [`Self::begin_block_at`] instead.
    pub fn begin_block(&mut self) -> SourceReadReport {
        self.begin_block_with(SeekClock::Now)
    }

    /// [`Self::begin_block`] for the block that starts at absolute render sample `first_sample`.
    ///
    /// An anchored seek ([`SourceCommand::SeekAt`]) is held while `first_sample` is before its
    /// anchor; the block that starts at or past it applies the seek at `frame + (first_sample -
    /// anchor_sample)` (saturating). While it is held, queued PCM of the playing generation still
    /// plays, the new generation's first block is kept as the pending block, and with nothing
    /// playable the block underruns. A newer command replaces a held one. This render-path
    /// operation allocates nothing and never blocks.
    pub fn begin_block_at(&mut self, first_sample: u64) -> SourceReadReport {
        self.begin_block_with(SeekClock::At(first_sample))
    }

    fn begin_block_with(&mut self, clock: SeekClock) -> SourceReadReport {
        self.begin_block_observe(clock);
        self.begin_block_play(clock)
    }

    /// The first half of [`Self::begin_block_with`]: release the played block and observe the
    /// admitted command. The two halves are separate so a test can interleave a producer between
    /// the command pop and the data pop, the window `acquire_current_block` must close.
    fn begin_block_observe(&mut self, clock: SeekClock) {
        self.end_block();
        self.flush_deferred_recycle();
        self.generation_changed = false;
        self.observe_seek_at_block_boundary(clock);
    }

    /// The second half of [`Self::begin_block_with`]: acquire and play this block.
    fn begin_block_play(&mut self, clock: SeekClock) -> SourceReadReport {
        self.acquire_current_block(clock);
        let mut copied_frames = 0_u32;
        let mut underrun_frames = 0_u32;
        if self.end_reached() {
            self.end_of_region = true;
        } else if self.current_matches_next_frame() {
            let block = self.current.take().expect("matching current block");
            copied_frames = block.frames;
            self.played_frames = block.frames;
            self.next_frame =
                SourceFrame(self.next_frame.0.saturating_add(u64::from(block.frames)));
            self.cumulative_read_frames = self
                .cumulative_read_frames
                .saturating_add(u64::from(block.frames));
            if block.end_of_region {
                self.end_of_region = true;
            }
            self.play(block);
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
        SourceReadReport {
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.active_generation,
            generation_changed: self.generation_changed,
            cumulative_read_frames: self.cumulative_read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
        }
    }

    /// Copy one retained source channel without consuming the played quantum.
    /// This render-path operation allocates nothing and never blocks.
    pub fn copy_channel(
        &self,
        channel: u32,
        destination: &mut [f32],
    ) -> Result<(), SourceReadError> {
        if channel >= self.channel_count {
            return Err(SourceReadError::ChannelCount {
                expected: self.channel_count,
                actual: usize::try_from(channel)
                    .expect("u32 fits usize")
                    .saturating_add(1),
            });
        }
        let quantum = usize::try_from(self.quantum_frames).expect("u32 fits usize");
        if destination.len() != quantum {
            return Err(SourceReadError::PlaneLength {
                expected_frames: self.quantum_frames,
            });
        }
        // Zero only what the copy does not reach. The plane used to be filled with `0.0` in full
        // and then, in the whole-quantum case, overwritten in full by the `copy_from_slice`
        // below -- a dead 512-byte write per plane per track per block on the production source
        // path, which the FrozenGraphSource-bound benchmark never sees. The two regions are
        // disjoint, so which order they are written in cannot be observed.
        match self.played.as_ref() {
            Some(block) => {
                let frames = usize::try_from(self.played_frames).expect("u32 fits usize");
                let offset = usize::try_from(channel)
                    .expect("u32 fits usize")
                    .checked_mul(quantum)
                    .expect("prepared channel offset");
                destination[..frames].copy_from_slice(&block.samples[offset..offset + frames]);
                // Exactly the region the old full fill left standing: the underrun tail of a
                // short block, and nothing at all when `frames == quantum`.
                destination[frames..].fill(0.0);
            }
            // No retained quantum: the whole plane is the underrun, as before.
            None => destination.fill(0.0),
        }
        Ok(())
    }

    /// End the played quantum without blocking: afterwards [`Self::played_plane`] is `None` and
    /// [`Self::copy_channel`] writes silence. The block's storage stays with the consumer as its
    /// idle retained block and returns to the producer when the next block is played.
    pub fn end_block(&mut self) {
        self.played_frames = 0;
        let Some(mut block) = self.played.take() else {
            return;
        };
        block.reset_metadata();
        let displaced = self.retained_idle.replace(block);
        debug_assert!(
            displaced.is_none(),
            "a played block and an idle retained block never coexist"
        );
        if let Some(displaced) = displaced {
            // Unreachable by the ownership rule above; recycle rather than free on render.
            self.recycle_block(displaced);
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Borrow one channel plane of the played block in place, without copying or consuming it.
    ///
    /// The slice is exactly `quantum_frames` long: the played frames, then `+0.0` to the quantum
    /// on a short block. `None` when no block was played this quantum (underrun, end of region,
    /// after `end_block` or `prepare_seek`) or when `channel` is out of range. The borrow ends
    /// before the next `&mut self` call, which is the only place the block can be released.
    /// This render-path operation allocates nothing and never blocks.
    #[must_use]
    pub fn played_plane(&self, channel: u32) -> Option<&[f32]> {
        if channel >= self.channel_count {
            return None;
        }
        let block = self.played.as_ref()?;
        let quantum = usize::try_from(self.quantum_frames).ok()?;
        let offset = usize::try_from(channel).ok()?.checked_mul(quantum)?;
        block.samples.get(offset..offset.checked_add(quantum)?)
    }
    // REALTIME_POLICY_END

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

    /// Pop the admitted command, if any, then apply a held anchored seek whose block has come.
    fn observe_seek_at_block_boundary(&mut self, clock: SeekClock) {
        match self.command_consumer.try_pop() {
            Ok(SourceCommand::Seek { generation, frame }) => {
                self.held_seek = None;
                self.apply_seek(generation, frame);
            }
            Ok(SourceCommand::SeekAt {
                generation,
                frame,
                anchor_sample,
            }) => {
                // A newer command replaces a held one; the replaced seek's pending block is stale.
                self.held_seek = Some(HeldSeek {
                    generation,
                    frame,
                    anchor_sample,
                });
                let active = self.active_generation;
                if let Some(block) = self
                    .current
                    .take_if(|block| block.generation != active && block.generation != generation)
                {
                    self.discard_block(block);
                }
            }
            Err(_) => {}
        }
        let Some(held) = self.held_seek else {
            return;
        };
        let late_by = match clock {
            SeekClock::Now => 0,
            SeekClock::At(first_sample) if first_sample >= held.anchor_sample => {
                first_sample - held.anchor_sample
            }
            SeekClock::At(_) | SeekClock::Hold => return,
        };
        self.held_seek = None;
        self.apply_seek(
            held.generation,
            SourceFrame(held.frame.0.saturating_add(late_by)),
        );
    }

    /// Make `generation` audible from `frame`. A pending `current` block of that generation that
    /// does not start behind `frame` is kept -- an anchored seek's primed block -- and any other is
    /// discarded, noting the region end it carries.
    fn apply_seek(&mut self, generation: SourceGeneration, frame: SourceFrame) {
        self.generation_changed = true;
        self.active_generation = generation;
        self.next_frame = frame;
        self.end_frame = None;
        self.end_of_region = false;
        if let Some(block) = self.current.take() {
            if block.generation == generation && block.start_frame.0 >= frame.0 {
                if block.end_of_region {
                    self.note_end_frame(&block);
                }
                self.current = Some(block);
            } else {
                self.note_end_and_discard(block);
            }
        }
    }

    /// Pop toward the block to play. `clock` is the block's, for a seek observed here.
    fn acquire_current_block(&mut self, clock: SeekClock) {
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
            if self.is_unobserved_generation(block.generation) {
                // This block's seek was pushed after this block's command pop. The producer pushes
                // a seek's command before any of its PCM, on one thread, through release/acquire
                // queues, so the command is in the one-slot command queue now: observe it before
                // judging the block rather than discard accepted PCM (#1274 MINOR-3). Bounded:
                // at most one command pop per popped block.
                self.observe_seek_at_block_boundary(clock);
            }
            if block.generation == self.active_generation
                && block.start_frame.0 >= self.next_frame.0
            {
                if block.end_of_region {
                    self.note_end_frame(&block);
                }
                self.current = Some(block);
                break;
            }
            if self
                .held_seek
                .is_some_and(|held| held.generation == block.generation)
            {
                // A held anchored seek's first block: kept unexamined as the pending block until
                // the seek applies (its end, if it carries one, is noted then), and the blocks
                // behind it stay queued.
                self.current = Some(block);
                break;
            }
            self.note_end_and_discard(block);
        }
    }

    /// Whether `generation` is newer than every seek this consumer has observed.
    fn is_unobserved_generation(&self, generation: SourceGeneration) -> bool {
        generation > self.active_generation
            && self
                .held_seek
                .is_none_or(|held| generation > held.generation)
    }

    fn current_matches_next_frame(&self) -> bool {
        self.current.as_ref().is_some_and(|block| {
            block.generation == self.active_generation && block.start_frame == self.next_frame
        })
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

    /// Make `block` the played block, then hand the idle retained block back to the producer:
    /// the consumer keeps exactly one block outside the queues.
    fn play(&mut self, mut block: Box<TransferBlock>) {
        let quantum = usize::try_from(self.quantum_frames).expect("u32 fits usize");
        let frames = usize::try_from(block.frames).expect("u32 fits usize");
        if frames < quantum {
            // A short block is the region's last: zero each plane's tail in place, once, so a
            // borrowed plane is a whole quantum. `copy_channel` keeps its own tail fill.
            for plane in block.samples.chunks_exact_mut(quantum) {
                plane[frames..].fill(0.0);
            }
        }
        debug_assert!(
            self.played.is_none(),
            "begin_block ended the previous block"
        );
        self.played = Some(block);
        if let Some(idle) = self.retained_idle.take() {
            self.recycle_block(idle);
        }
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
        self.recycle_block(block);
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
    /// `None` for a vacant source ([`Self::vacant`]).
    consumer: Option<PcmSourceConsumer>,
    /// Exact channel count, of the consumer or the vacancy.
    channel_count: u32,
    /// `None` for a vacant source: it charges no PCM payload and no ring overhead.
    resources: Option<SourceResourceReport>,
    /// Fixed caller-owned bytes not represented by the ring report (every host passes zero).
    additional_overhead_bytes: u64,
    /// Largest such fixed allocation, if larger than the ring allocation (zero from every host).
    additional_largest_allocation_bytes: u64,
}

impl SourceGraphSource {
    /// Construct one host-decoded source.
    #[must_use]
    pub fn new(
        consumer: PcmSourceConsumer,
        resources: SourceResourceReport,
        additional_overhead_bytes: u64,
        additional_largest_allocation_bytes: u64,
    ) -> Self {
        Self {
            channel_count: consumer.channel_count(),
            consumer: Some(consumer),
            resources: Some(resources),
            additional_overhead_bytes,
            additional_largest_allocation_bytes,
        }
    }

    /// Construct a vacant source of `channel_count` channels: no ring, until a successor plan's
    /// carry program moves a predecessor's consumer into it at the swap block. Until then every
    /// claim on it renders `+0.0` and each block reports a source underrun.
    #[must_use]
    pub const fn vacant(channel_count: u32) -> Self {
        Self {
            consumer: None,
            channel_count,
            resources: None,
            additional_overhead_bytes: 0,
            additional_largest_allocation_bytes: 0,
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
    /// `None` while vacant: the entry renders `+0.0` and reports an underrun every block.
    consumer: Option<PcmSourceConsumer>,
    /// The channel count the mappings were validated against, kept for a vacant entry.
    channel_count: u32,
}

struct SourceGraphSourceSetDriver {
    sources: Box<[GraphSourceEntry]>,
    mappings: Box<[SourceGraphTrackMapping]>,
    quantum_frames: u32,
    block_validity: GraphObservationValidity,
    pending_generation_change: bool,
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
) -> Result<SourceSetRetainedResourceReport, SourceGraphSourceSetError> {
    let source_entries = allocation_class::<GraphSourceEntry>(sources.len())?;
    let mappings_report = allocation_class::<SourceGraphTrackMapping>(mappings.len())?;
    let claims = allocation_class::<GraphSourceInputClaim>(mappings.len())?;
    let driver = allocation_class::<SourceGraphSourceSetDriver>(1)?;
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
        owned_stable_id_payloads: ids,
    })
}

impl GraphPreparedSourceSetDriver for SourceGraphSourceSetDriver {
    fn can_prepare_source_seek(&self, source_index: usize) -> bool {
        self.sources
            .get(source_index)
            .is_some_and(|source| source.consumer.is_some())
    }

    fn prepare_source_seek(&mut self, source_index: usize, generation: u64, frame: u64) -> bool {
        let Some(generation) = SourceGeneration::new(generation) else {
            return false;
        };
        let prepared = self
            .sources
            .get_mut(source_index)
            .and_then(|source| source.consumer.as_mut())
            .is_some_and(|consumer| consumer.prepare_seek(generation, SourceFrame(frame)));
        if prepared {
            self.pending_generation_change = true;
        }
        prepared
    }

    fn claim_count(&self) -> usize {
        self.mappings.len()
    }

    fn begin_block(
        &mut self,
        first_sample: u64,
        frames: u32,
    ) -> Result<(), engine::realtime::RenderError> {
        if frames != self.quantum_frames {
            return Err(engine::realtime::RenderError::InvalidEnvelope);
        }
        self.block_validity = GraphObservationValidity {
            source_underrun: false,
            source_generation_changed: self.pending_generation_change,
        };
        self.pending_generation_change = false;
        for source in &mut self.sources {
            // A vacant entry has nothing to play: its block is an underrun.
            let Some(consumer) = source.consumer.as_mut() else {
                self.block_validity.source_underrun = true;
                continue;
            };
            // The block's absolute render sample is the clock an anchored seek waits on.
            let report = consumer.begin_block_at(first_sample);
            self.block_validity.source_underrun |= report.underrun_event;
            self.block_validity.source_generation_changed |= report.generation_changed;
        }
        // No claim will read this quantum, so nothing needs the played blocks past this point.
        if self.mappings.is_empty() {
            for consumer in self
                .sources
                .iter_mut()
                .filter_map(|source| source.consumer.as_mut())
            {
                consumer.end_block();
            }
        }
        Ok(())
    }

    fn observation_validity(&self) -> GraphObservationValidity {
        self.block_validity
    }

    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), engine::realtime::RenderError> {
        let (source_index, left_channel, right_channel) = self
            .mappings
            .get(claim_index)
            .map(|mapping| {
                (
                    mapping.source_index,
                    mapping.left_channel,
                    mapping.right_channel,
                )
            })
            .ok_or(engine::realtime::RenderError::InvalidEnvelope)?;
        let source = self
            .sources
            .get(source_index)
            .ok_or(engine::realtime::RenderError::InvalidEnvelope)?;
        let Some(consumer) = source.consumer.as_ref() else {
            // Vacant: the claim is silence, as an underrun is.
            left.fill(0.0);
            right.fill(0.0);
            return Ok(());
        };
        consumer
            .copy_channel(left_channel, left)
            .map_err(|_| engine::realtime::RenderError::InvalidEnvelope)?;
        // The played block is not released here: it stays readable in place until the next
        // `begin_block` or seek preparation, whichever claim copied last (#917).
        consumer
            .copy_channel(right_channel, right)
            .map_err(|_| engine::realtime::RenderError::InvalidEnvelope)
    }

    /// Every claim's planes are its source's played block, lent in place (issue #918).
    fn provides_played_planes(&self) -> bool {
        true
    }

    // REALTIME_POLICY_BEGIN
    /// Borrow the claim's `(left, right)` channel planes of its source's played block in place.
    ///
    /// `None` when the claim index is out of range or its source played no block this quantum
    /// (underrun, end of region, or no mappings at all, which releases in `begin_block`); on
    /// `None`, `copy_track_input` writes `+0.0` throughout, and on `Some` it copies these words.
    /// The planes stay valid until the next `begin_block` or seek preparation (#917): both take
    /// `&mut self`, so no borrow of a plane outlives them. A banked source gather in the graph
    /// reads them in place of the copy.
    fn played_planes(&self, claim_index: usize) -> Option<(&[f32], &[f32])> {
        let mapping = self.mappings.get(claim_index)?;
        let consumer = self.sources.get(mapping.source_index)?.consumer.as_ref()?;
        Some((
            consumer.played_plane(mapping.left_channel)?,
            consumer.played_plane(mapping.right_channel)?,
        ))
    }
    // REALTIME_POLICY_END

    fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
        let mut written = 0;
        for source in &self.sources {
            // A vacant entry reports zeros, so every source keeps its five telemetry slots.
            let values = source.consumer.as_ref().map_or([0; 5], |consumer| {
                let telemetry = consumer.telemetry();
                [
                    telemetry.cumulative_read_frames,
                    telemetry.stale_generation_discard_count,
                    telemetry.underrun_frames,
                    telemetry.underrun_events,
                    telemetry.native_decoder_sanitized_samples,
                ]
            });
            for value in values {
                let Some(slot) = output.get_mut(written) else {
                    return written;
                };
                *slot = value;
                written += 1;
            }
        }
        written
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
        Some(self)
    }

    fn source_vacancy(&self, source_index: usize) -> Option<bool> {
        self.sources
            .get(source_index)
            .map(|source| source.consumer.is_none())
    }

    fn vacant_source_count(&self) -> usize {
        self.sources
            .iter()
            .filter(|source| source.consumer.is_none())
            .count()
    }

    // REALTIME_POLICY_BEGIN
    /// Move each named predecessor consumer into this set's vacant entry with a swap, so the
    /// predecessor's entry becomes vacant and nothing is allocated, freed or dropped. Every move
    /// is checked first -- both indices in range, this entry vacant, the predecessor's occupied,
    /// the channel counts and quanta equal -- and any failure moves nothing. The predecessor's
    /// pending generation change (a seek prepared since its last block) is carried too.
    fn adopt_sources(
        &mut self,
        predecessor: &mut dyn GraphPreparedSourceSetDriver,
        moves: &[(u32, u32)],
    ) -> bool {
        let Some(predecessor) = predecessor
            .as_any_mut()
            .and_then(|any| any.downcast_mut::<Self>())
        else {
            return false;
        };
        let movable = predecessor.quantum_frames == self.quantum_frames
            && moves.iter().all(|&(successor, source)| {
                match (
                    self.sources.get(successor as usize),
                    predecessor.sources.get(source as usize),
                ) {
                    (Some(successor), Some(source)) => {
                        successor.consumer.is_none()
                            && source.consumer.is_some()
                            && successor.channel_count == source.channel_count
                    }
                    _ => false,
                }
            });
        if !movable {
            return false;
        }
        for &(successor, source) in moves {
            core::mem::swap(
                &mut self.sources[successor as usize].consumer,
                &mut predecessor.sources[source as usize].consumer,
            );
        }
        self.pending_generation_change |= predecessor.pending_generation_change;
        true
    }
    // REALTIME_POLICY_END
}

/// Seal one or more prepared source consumers into a graph-owned fan-out source set.
///
/// The wrapper owns exactly one consumer per source. The graph calls `begin_block` once, then each
/// mapping copies directly from the retained played block, which stays the consumer's until the
/// next `begin_block` or seek preparation (#917).
pub fn prepare_graph_source_set(
    envelope: engine::realtime::RenderEnvelope,
    sources: Vec<SourceGraphSource>,
    mappings: Vec<SourceGraphTrackMapping>,
) -> Result<GraphPreparedSourceSet, SourceGraphSourceSetError> {
    if sources.is_empty() {
        return Err(SourceGraphSourceSetError::EmptySources);
    }
    let retained = source_set_retained_resources(&sources, &mappings)?;
    let mut pcm_payload = 0_u64;
    let mut overhead = 0_u64;
    let mut largest = 0_u64;
    let mut entries = Vec::with_capacity(sources.len());
    for source in sources {
        // A vacant source owns no ring: it charges no PCM payload and no ring overhead.
        if let Some(resources) = source.resources {
            pcm_payload = pcm_payload
                .checked_add(resources.pcm_payload_already_charged_bytes)
                .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
            overhead = overhead
                .checked_add(resources.overhead_bytes)
                .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
            largest = largest.max(resources.largest_allocation_bytes);
        }
        overhead = overhead
            .checked_add(source.additional_overhead_bytes)
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        largest = largest.max(source.additional_largest_allocation_bytes);
        entries.push(GraphSourceEntry {
            consumer: source.consumer,
            channel_count: source.channel_count,
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
            sources: entries.into_boxed_slice(),
            mappings: mappings.into_boxed_slice(),
            quantum_frames: envelope.quantum.0,
            block_validity: GraphObservationValidity::CLEAR,
            pending_generation_change: false,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: SampleRateHz = SampleRateHz(48_000);

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
        assert_eq!(report.retained_block_count, 1);
        assert_eq!(report.pcm_payload_already_charged_bytes, 64);
        assert_eq!(report.transfer_block_pcm_bytes, 64);
        // #917: the retained block is one more 2 x 4-sample block, charged to overhead only.
        assert_eq!(report.retained_block_pcm_bytes, 32);
        let three = NonZeroUsize::new(3).expect("allocated blocks");
        let queue = queue_bytes::<Box<TransferBlock>>(three).expect("queue");
        assert_eq!(report.data_queue_bytes, queue);
        assert_eq!(report.recycle_queue_bytes, queue);
        let metadata = u64::try_from(size_of::<TransferBlock>()).expect("metadata size");
        assert_eq!(report.transfer_block_metadata_bytes, 3 * metadata);
        assert_eq!(
            report.overhead_bytes,
            report.data_queue_bytes
                + report.recycle_queue_bytes
                + report.command_queue_bytes
                + report.transfer_block_metadata_bytes
                + report.retained_block_pcm_bytes
        );
        // The memory delta over the pre-#917 two-block ring, which sized both queues and the
        // metadata at two blocks: one block's PCM, one block's metadata and one pointer slot in
        // each queue. Nothing moves into the session-charged PCM row.
        let two = NonZeroUsize::new(2).expect("configured blocks");
        let pre_change_overhead = 2 * queue_bytes::<Box<TransferBlock>>(two).expect("queue")
            + report.command_queue_bytes
            + 2 * metadata;
        let slot = u64::try_from(size_of::<Box<TransferBlock>>()).expect("slot size");
        assert_eq!(
            report.overhead_bytes - pre_change_overhead,
            32 + metadata + 2 * slot
        );
        assert_eq!(
            report.total_engine_owned_bytes,
            report.pcm_payload_already_charged_bytes + report.overhead_bytes
        );
        let largest_queue =
            largest_queue_allocation::<Box<TransferBlock>>(three).expect("largest queue");
        assert_eq!(
            report.largest_allocation_bytes,
            largest_queue.max(32).max(metadata)
        );
    }

    #[test]
    fn paused_seek_prepares_full_queues_without_consuming_target() {
        for retained in [false, true] {
            let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
            let mut host = producer.into_host_chunk_provider(RATE);
            let old = [0.25; 4];
            host.submit(chunk(1, 0, &[&old], 4, false)).unwrap();
            host.submit(chunk(1, 4, &[&old], 4, false)).unwrap();
            if retained {
                consumer.acquire_current_block(SeekClock::Now);
            }
            host.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(100),
            })
            .unwrap();
            assert!(consumer.prepare_seek(SourceGeneration(2), SourceFrame(100)));
            assert_eq!(consumer.next_frame, SourceFrame(100));
            assert_eq!(consumer.cumulative_read_frames, 0);
            assert_eq!(consumer.underrun_frames, 0);
            assert_eq!(consumer.underrun_events, 0);
            assert_eq!(consumer.stale_generation_discard_count, 2);
            let target = [1.0, 2.0, 3.0, 4.0];
            host.submit(chunk(2, 100, &[&target], 4, false)).unwrap();
            // Preparing again retains current-generation PCM and never consumes it.
            assert!(consumer.prepare_seek(SourceGeneration(2), SourceFrame(100)));
            assert!(!consumer.prepare_seek(SourceGeneration(1), SourceFrame(100)));
            let mut output = [0.0; 4];
            let report = consumer.read_block(&mut [&mut output]).unwrap();
            assert_eq!(output, target);
            assert_eq!(report.underrun_frames, 0);
            assert_eq!(consumer.next_frame, SourceFrame(104));
        }
    }

    fn seek_at(generation: u64, frame: u64, anchor_sample: u64) -> SourceCommand {
        SourceCommand::SeekAt {
            generation: SourceGeneration(generation),
            frame: SourceFrame(frame),
            anchor_sample,
        }
    }

    /// Render one block starting at `first_sample` and return its single channel.
    fn render_at(
        consumer: &mut PcmSourceConsumer,
        first_sample: u64,
    ) -> ([f32; 4], SourceReadReport) {
        let report = consumer.begin_block_at(first_sample);
        let mut output = [f32::NAN; 4];
        consumer.copy_channel(0, &mut output).unwrap();
        consumer.end_block();
        (output, report)
    }

    /// #1274 D1. Red if an anchor no block starts at is admitted, or if the refusal switches the
    /// producer's generation anyway.
    #[test]
    fn an_unaligned_anchor_is_refused_without_a_generation_switch() {
        let (producer, _consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        assert_eq!(
            host.try_seek(seek_at(2, 100, 6)),
            Err(SourceSeekError::AnchorUnaligned)
        );
        assert_eq!(host.telemetry().active_generation, SourceGeneration(1));
        host.submit(chunk(1, 0, &[&[0.5; 4]], 4, false))
            .expect("generation 1 still accepted");
        host.try_seek(seek_at(2, 100, 8)).expect("aligned anchor");
        host.submit(chunk(2, 100, &[&[0.5; 4]], 4, false))
            .expect("the new generation is accepted at once, from its frame");
    }

    /// #1274 D3. Red if a newer anchored seek does not replace a held one, or if the replaced
    /// seek's pending block is kept when the newer seek is observed rather than discarded: kept
    /// out of the queues until the newer anchor, it holds the producer one block short of priming
    /// the newer generation to its full admission depth before that anchor. (Its PCM cannot play
    /// either way: `apply_seek` and the generation compare already refuse it.)
    #[test]
    fn a_newer_anchored_seek_replaces_a_held_one() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 16)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        let depth = consumer.transfer_block_capacity();
        host.try_seek(seek_at(2, 100, 24)).unwrap();
        host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false)).unwrap();
        let (output, report) = render_at(&mut consumer, 0);
        assert_eq!(
            output, [0.0; 4],
            "held: nothing of the old generation queued"
        );
        assert_eq!(report.active_generation, SourceGeneration(1));
        host.try_seek(seek_at(3, 200, 8)).unwrap();
        host.submit(chunk(3, 200, &[&[3.0; 4]], 4, false)).unwrap();
        host.submit(chunk(3, 204, &[&[3.25; 4]], 4, false)).unwrap();
        let (output, report) = render_at(&mut consumer, 4);
        assert_eq!(output, [0.0; 4], "the replacement is held too");
        assert_eq!(report.active_generation, SourceGeneration(1));
        // Before its anchor the newer generation primes to the full admission depth.
        let values = [3.0, 3.25, 3.5, 3.75];
        for (block, value) in values.iter().enumerate().skip(2) {
            let start = 200 + 4 * u64::try_from(block).unwrap();
            host.submit(chunk(3, start, &[&[*value; 4]], 4, false))
                .expect("the replaced seek's pending block was recycled");
        }
        assert_eq!(values.len(), depth);
        for (block, value) in values.iter().enumerate() {
            let first_sample = 8 + 4 * u64::try_from(block).unwrap();
            let (output, report) = render_at(&mut consumer, first_sample);
            assert_eq!(
                output, [*value; 4],
                "the newer seek applies at its own anchor"
            );
            assert_eq!(report.generation_changed, block == 0);
            assert_eq!(report.active_generation, SourceGeneration(3));
        }
        let (output, _) = render_at(&mut consumer, 24);
        assert_eq!(output, [0.0; 4], "the replaced seek never applies");
        assert_eq!(consumer.active_generation, SourceGeneration(3));
    }

    /// #1274 D3. Red if the held generation's primed block plays before its anchor because it
    /// starts at the frame the old generation has reached (`current_matches_next_frame` must
    /// compare the generation), or if it is discarded rather than kept as the pending block.
    #[test]
    fn a_primed_block_waits_for_its_anchor() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 4, 8)).unwrap();
        host.submit(chunk(2, 4, &[&[2.0; 4]], 4, false)).unwrap();
        let (output, report) = render_at(&mut consumer, 0);
        assert_eq!((output, report.underrun_frames), ([0.0; 4], 4));
        // The old generation's underrun has reached frame 4, the primed block's start.
        let (output, report) = render_at(&mut consumer, 4);
        assert_eq!((output, report.underrun_frames), ([0.0; 4], 4));
        assert_eq!(report.active_generation, SourceGeneration(1));
        let (output, report) = render_at(&mut consumer, 8);
        assert_eq!((output, report.copied_frames), ([2.0; 4], 4));
        assert_eq!(report.active_generation, SourceGeneration(2));
    }

    /// #1274 D4. Red if seek preparation pops an anchored seek and drops it, losing an accepted
    /// seek, or applies it with no block time.
    #[test]
    fn prepare_seek_holds_an_anchored_seek() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&[1.0; 4]], 4, false)).unwrap();
        host.try_seek(seek_at(2, 100, 8)).unwrap();
        assert!(!consumer.prepare_seek(SourceGeneration(2), SourceFrame(100)));
        assert_eq!(consumer.active_generation, SourceGeneration(1));
        host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false)).unwrap();
        let (output, report) = render_at(&mut consumer, 4);
        assert_eq!(
            output, [1.0; 4],
            "the old generation plays while the seek is held"
        );
        assert_eq!(report.active_generation, SourceGeneration(1));
        let (output, report) = render_at(&mut consumer, 8);
        assert_eq!(output, [2.0; 4]);
        assert_eq!(report.active_generation, SourceGeneration(2));
    }

    /// #1274 D3. Red if a late anchored seek past the region end is not the end of the region:
    /// the pending block it was primed with must still report where the region ends.
    #[test]
    fn a_late_anchored_seek_past_the_region_end_is_the_end_of_region() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 100, 8)).unwrap();
        host.submit(chunk(2, 100, &[&[2.0, 2.0]], 2, true)).unwrap();
        let (_, report) = render_at(&mut consumer, 4);
        assert!(
            !report.end_of_region,
            "held, with its region-end block pending"
        );
        assert_eq!(report.active_generation, SourceGeneration(1));
        // Eight samples late: frame 108 is past the region's end at 102.
        let (output, report) = render_at(&mut consumer, 16);
        assert_eq!(output, [0.0; 4]);
        assert_eq!(report.active_generation, SourceGeneration(2));
        assert!(report.end_of_region);
        assert_eq!(report.copied_frames, 0);
        assert_eq!(report.underrun_frames, 0, "past the end is not an underrun");
    }

    /// #1274 MINOR-1. Red if an on-time apply keeps the primed block without noting the region
    /// end it carries: a stem whose primed block is its region's last, short block would then
    /// report an underrun on every block after it ends, and never reach the end of its region.
    #[test]
    fn an_on_time_anchored_seek_to_a_last_short_block_ends_its_region() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 100, 8)).unwrap();
        host.submit(chunk(2, 100, &[&[2.0, 2.0]], 2, true)).unwrap();
        let (_, report) = render_at(&mut consumer, 4);
        assert_eq!(report.active_generation, SourceGeneration(1));
        let (output, report) = render_at(&mut consumer, 8);
        assert_eq!(output, [2.0, 2.0, 0.0, 0.0]);
        assert_eq!(report.active_generation, SourceGeneration(2));
        assert_eq!(report.copied_frames, 2);
        assert!(report.end_of_region);
        let events = report.cumulative_underrun_events;
        for first_sample in [12, 16] {
            let (output, report) = render_at(&mut consumer, first_sample);
            assert_eq!(output, [0.0; 4]);
            assert!(report.end_of_region, "block {first_sample} is past the end");
            assert_eq!(report.underrun_frames, 0, "the end is not an underrun");
            assert_eq!(report.cumulative_underrun_events, events);
        }
    }

    /// Render one block with a producer step between its command pop and its data pop: the
    /// window in which a seek's PCM can reach the render before its command does.
    fn render_with_producer_in_window(
        consumer: &mut PcmSourceConsumer,
        clock: SeekClock,
        producer: impl FnOnce(),
    ) -> ([f32; 4], SourceReadReport) {
        consumer.begin_block_observe(clock);
        producer();
        let report = consumer.begin_block_play(clock);
        let mut output = [f32::NAN; 4];
        consumer.copy_channel(0, &mut output).unwrap();
        consumer.end_block();
        (output, report)
    }

    /// #1274 MINOR-3, the acked-batch question. Red if the render discards an accepted block of
    /// a newer generation because the block's seek command was pushed after the block boundary
    /// popped the command queue: the producer acked the PCM, and it must play (a plain seek at
    /// once, an anchored one at its anchor) rather than count as a stale discard.
    #[test]
    fn a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm() {
        for anchored in [false, true] {
            let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).unwrap();
            let mut host = producer.into_host_chunk_provider(RATE);
            let command = if anchored {
                seek_at(2, 100, 8)
            } else {
                SourceCommand::Seek {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(100),
                }
            };
            let (output, report) =
                render_with_producer_in_window(&mut consumer, SeekClock::At(4), || {
                    host.try_seek(command).unwrap();
                    host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false))
                        .expect("accepted");
                });
            if anchored {
                assert_eq!(output, [0.0; 4], "held until its anchor");
                assert_eq!(report.active_generation, SourceGeneration(1));
                let (output, report) = render_at(&mut consumer, 8);
                assert_eq!(output, [2.0; 4], "anchored: the acked block plays");
                assert!(report.generation_changed);
            } else {
                assert_eq!(output, [2.0; 4], "plain: the acked block plays at once");
                assert!(report.generation_changed);
            }
            assert_eq!(consumer.active_generation, SourceGeneration(2));
            assert_eq!(consumer.stale_generation_discard_count, 0);
            assert_eq!(consumer.underrun_events, u64::from(anchored));
        }
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
    fn retained_played_block_supports_repeat_fanout_and_auto_recycles_once() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(2, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let left_first = [1.0, 2.0, 3.0, 4.0];
        let right_first = [-1.0, -2.0, -3.0, -4.0];
        host.submit(chunk(1, 0, &[&left_first, &right_first], 4, false))
            .expect("first block");
        let left_final = [5.0, 6.0];
        let right_final = [-5.0, -6.0];
        host.submit(chunk(1, 4, &[&left_final, &right_final], 2, true))
            .expect("short EOF block");

        let first = consumer.begin_block();
        assert_eq!(first.copied_frames, 4);
        assert_eq!(first.cumulative_read_frames, 4);
        let mut left = [0.0; 4];
        consumer.copy_channel(0, &mut left).expect("left copy");
        assert_eq!(left, left_first);
        left.fill(99.0);
        consumer
            .copy_channel(0, &mut left)
            .expect("repeat left copy");
        assert_eq!(left, left_first);
        let mut right = [0.0; 4];
        consumer.copy_channel(1, &mut right).expect("right copy");
        assert_eq!(right, right_first);
        assert!(matches!(
            consumer.copy_channel(2, &mut right),
            Err(SourceReadError::ChannelCount { .. })
        ));
        assert!(matches!(
            consumer.copy_channel(0, &mut right[..3]),
            Err(SourceReadError::PlaneLength { .. })
        ));
        consumer
            .copy_channel(1, &mut right)
            .expect("played block survives invalid copies");
        assert_eq!(right, right_first);
        assert_eq!(consumer.telemetry().cumulative_read_frames, 4);

        let final_report = consumer.begin_block();
        assert_eq!(final_report.copied_frames, 2);
        assert_eq!(final_report.cumulative_read_frames, 6);
        assert!(final_report.end_of_region);
        consumer.copy_channel(0, &mut left).expect("short left");
        consumer.copy_channel(1, &mut right).expect("short right");
        assert_eq!(left, [5.0, 6.0, 0.0, 0.0]);
        assert_eq!(right, [-5.0, -6.0, 0.0, 0.0]);
        assert_eq!(consumer.telemetry().cumulative_read_frames, 6);

        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(100),
        })
        .expect("new generation");
        host.submit(chunk(2, 100, &[&left_first, &right_first], 4, false))
            .expect("initial begin of final block auto-recycled prior played block");

        consumer.end_block();
        assert!(consumer.played.is_none());
        assert_eq!(consumer.played_frames, 0);
        consumer.end_block();
        assert!(consumer.played.is_none());
        assert_eq!(consumer.telemetry().cumulative_read_frames, 6);
    }

    /// Values a skipped write would leave standing. `-0.0` is in the list because the fill writes
    /// `+0.0` and `-0.0 == 0.0`, so only a bit comparison can tell a fill that ran from one that
    /// did not; `NAN` is there because it is not equal to itself.
    const COPY_CHANNEL_POISON: [f32; 4] = [-0.0, f32::NAN, 1.0e30, -7.5];

    /// Compare every copied word with caller-supplied PCM and explicit positive-zero tails.
    fn assert_copy_channel(
        consumer: &PcmSourceConsumer,
        channel: u32,
        expected: &[f32],
        what: &str,
    ) {
        for poison in COPY_CHANNEL_POISON {
            let mut actual = vec![poison; expected.len()];
            consumer
                .copy_channel(channel, &mut actual)
                .expect("copy_channel");
            for (frame, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "{what}: poison {poison}, frame {frame}"
                );
            }
        }
    }

    /// Full PCM, short PCM with a zero tail, and silence overwrite every poisoned destination.
    #[test]
    fn copy_channel_preserves_pcm_and_zeroes_missing_frames() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(2, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);

        // No retained quantum at all: the whole plane is the underrun.
        assert_copy_channel(&consumer, 0, &[0.0; 4], "before any block");

        // `-0.0` and a subnormal-adjacent value in the payload, so a copy that was quietly
        // replaced by a zero fill could not pass by comparing equal.
        let left_full = [1.0, -0.0, f32::MIN_POSITIVE, 4.0];
        let right_full = [-1.0, -2.0, -3.0, -4.0];
        host.submit(chunk(1, 0, &[&left_full, &right_full], 4, false))
            .expect("whole-quantum block");
        let left_short = [5.0, -6.0];
        let right_short = [-5.0, 6.0];
        host.submit(chunk(1, 4, &[&left_short, &right_short], 2, true))
            .expect("short EOF block");

        // Whole quantum: every sample is the supplied PCM.
        let full = consumer.begin_block();
        assert_eq!(full.copied_frames, 4);
        assert_copy_channel(&consumer, 0, &left_full, "whole quantum, left");
        assert_copy_channel(&consumer, 1, &right_full, "whole quantum, right");

        // Short submission: the copied prefix, then a tail that must still come out `+0.0`.
        let short = consumer.begin_block();
        assert_eq!(short.copied_frames, 2);
        assert_copy_channel(&consumer, 0, &[5.0, -6.0, 0.0, 0.0], "short quantum, left");
        assert_copy_channel(&consumer, 1, &[-5.0, 6.0, 0.0, 0.0], "short quantum, right");

        // Past the end of the region: nothing is retained, so the whole plane is zeroed.
        let after = consumer.begin_block();
        assert!(after.end_of_region);
        assert_copy_channel(&consumer, 0, &[0.0; 4], "past the end of the region");
    }

    fn test_mapping(index: u32) -> SourceGraphTrackMapping {
        SourceGraphTrackMapping {
            node: GraphNodeId::TrackStage {
                track_id: graph::StableGraphId::parse(&format!("track{index}")).expect("track ID"),
                stage: graph::TrackStage::Input,
            },
            source_index: 0,
            left_channel: 0,
            right_channel: 0,
        }
    }

    fn test_driver(
        consumer: PcmSourceConsumer,
        mappings: Vec<SourceGraphTrackMapping>,
    ) -> SourceGraphSourceSetDriver {
        SourceGraphSourceSetDriver {
            sources: vec![GraphSourceEntry {
                channel_count: consumer.channel_count(),
                consumer: Some(consumer),
            }]
            .into_boxed_slice(),
            mappings: mappings.into_boxed_slice(),
            quantum_frames: 4,
            block_validity: GraphObservationValidity::CLEAR,
            pending_generation_change: false,
        }
    }

    #[test]
    fn graph_driver_forwards_underrun_and_seek_generation_facts() {
        let (producer, consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let mut driver = test_driver(consumer, vec![test_mapping(0)]);

        driver.begin_block(0, 4).expect("first block");
        assert_eq!(
            driver.observation_validity(),
            GraphObservationValidity {
                source_underrun: true,
                source_generation_changed: false,
            }
        );
        let samples = [1.0; 4];
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(100),
        })
        .expect("seek");
        assert!(driver.prepare_source_seek(0, 2, 100));
        host.submit(chunk(2, 100, &[&samples], 4, false))
            .expect("fresh block");
        driver.begin_block(4, 4).expect("second block");
        assert_eq!(
            driver.observation_validity(),
            GraphObservationValidity {
                source_underrun: false,
                source_generation_changed: true,
            }
        );
    }

    /// #917 restates the pre-change `..._last_claim_recycles_in_call_...` test for the hold rule:
    /// no claim path releases the played block, the next `begin_block` does, and the producer
    /// keeps its configured one-block depth while the render holds its block. The old incomplete
    /// leg asserted `Full` here, because the render then held one of the ring's only blocks.
    #[test]
    fn graph_driver_retains_played_blocks_to_next_begin_on_every_claim_path() {
        let samples = [1.0, 2.0, 3.0, 4.0];
        let later = [5.0, 6.0, 7.0, 8.0];

        let (producer, consumer, _) = PcmSourceRing::prepare(config(1, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&samples], 4, false))
            .expect("first block");
        let mut driver = test_driver(consumer, vec![test_mapping(0)]);
        driver.begin_block(0, 4).expect("complete begin");
        let mut left = [0.0; 4];
        let mut right = [0.0; 4];
        driver
            .copy_track_input(0, &mut left, &mut right)
            .expect("last claim");
        assert_eq!(left, samples);
        assert_eq!(right, samples);
        assert_eq!(
            driver.played_planes(0),
            Some((&samples[..], &samples[..])),
            "the last claim no longer releases the played block"
        );
        host.submit(chunk(1, 4, &[&later], 4, false))
            .expect("configured depth admitted while the render holds its block");
        assert!(matches!(
            host.submit(chunk(1, 8, &[&later], 4, false)),
            Err(HostChunkError::Full { .. })
        ));
        driver.begin_block(4, 4).expect("next begin");
        assert_eq!(driver.played_planes(0), Some((&later[..], &later[..])));
        host.submit(chunk(1, 8, &[&later], 4, false))
            .expect("next begin recycled the previous played block");

        let (producer, consumer, _) = PcmSourceRing::prepare(config(1, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&samples], 4, false))
            .expect("first block");
        let mut driver = test_driver(consumer, vec![test_mapping(0), test_mapping(1)]);
        driver.begin_block(0, 4).expect("incomplete begin");
        driver
            .copy_track_input(0, &mut left, &mut right)
            .expect("first of two claims");
        host.submit(chunk(1, 4, &[&samples], 4, false))
            .expect("configured depth admitted while a claim is still missing");
        assert!(matches!(
            host.submit(chunk(1, 8, &[&samples], 4, false)),
            Err(HostChunkError::Full { .. })
        ));
        driver
            .begin_block(4, 4)
            .expect("next begin releases the missing claim's block");
        host.submit(chunk(1, 8, &[&samples], 4, false))
            .expect("missing claim's block recycled on next begin");

        let (producer, consumer, _) = PcmSourceRing::prepare(config(1, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&samples], 4, false))
            .expect("first block");
        let mut driver = test_driver(consumer, vec![test_mapping(0)]);
        driver.begin_block(0, 4).expect("error begin");
        assert_eq!(
            driver.copy_track_input(0, &mut left[..3], &mut right),
            Err(engine::realtime::RenderError::InvalidEnvelope)
        );
        assert_eq!(
            driver.played_planes(0),
            Some((&samples[..], &samples[..])),
            "a failed claim copy does not release the played block"
        );
        host.submit(chunk(1, 4, &[&later], 4, false))
            .expect("configured depth admitted after a failed claim");
        assert!(matches!(
            host.submit(chunk(1, 8, &[&later], 4, false)),
            Err(HostChunkError::Full { .. })
        ));
        driver
            .begin_block(4, 4)
            .expect("next begin releases the errored claim's block");
        assert_eq!(driver.played_planes(0), Some((&later[..], &later[..])));
        host.submit(chunk(1, 8, &[&later], 4, false))
            .expect("errored claim's block recycled on next begin");
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
        assert!(report.generation_changed);
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
        assert_eq!(
            SourceDiagnosticPath::for_sources_collection().as_str(),
            "$.sources"
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

    #[test]
    fn one_quantum_ring_shape_matches_its_report_and_reads_back_planar() {
        let (producer, mut consumer, report) =
            PcmSourceRing::prepare(config(2, 4, 4)).expect("ring");
        assert_eq!(producer.shape(), consumer.shape());
        assert_eq!(producer.shape().frame_capacity, 4);
        // The shape and the report keep the configured count; the retained block is separate.
        assert_eq!(consumer.shape().transfer_block_count, 1);
        assert_eq!(consumer.transfer_block_capacity(), 1);
        assert_eq!(report.transfer_block_count, 1);
        assert_eq!(report.retained_block_count, 1);
        let mut host = producer.into_host_chunk_provider(RATE);
        let left_in = [1.0, 2.0, 3.0, 4.0];
        let right_in = [-1.0, -2.0, -3.0, -4.0];
        host.submit(chunk(1, 0, &[&left_in, &right_in], 4, true))
            .expect("planar submission");
        let mut left = [0.0; 4];
        let mut right = [0.0; 4];
        let report = consumer
            .read_block(&mut [&mut left, &mut right])
            .expect("read");
        assert_eq!(report.copied_frames, 4);
        assert_eq!(left, left_in);
        assert_eq!(right, right_in);
    }

    #[test]
    fn rejected_host_submission_publishes_nothing_and_leaves_the_producer_untouched() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(100),
        })
        .expect("seek");
        let before = host.telemetry();
        // A chunk stamped before the admitted seek is stale; validation precedes any publication.
        let stale = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(
            host.submit(chunk(1, 0, &[&stale], 4, false)),
            Err(HostChunkError::StaleGeneration {
                active: SourceGeneration(2),
                submitted: SourceGeneration(1),
            })
        );
        // A short chunk that does not end the region has the wrong frame count.
        let short = [1.0, 2.0, 3.0];
        assert_eq!(
            host.submit(chunk(2, 100, &[&short], 3, false)),
            Err(HostChunkError::FrameCount {
                quantum_frames: 4,
                submitted_frames: 3,
                end_of_region: false,
            })
        );
        assert_eq!(host.telemetry(), before);
        let fresh = [5.0, 6.0, 7.0, 8.0];
        host.submit(chunk(2, 100, &[&fresh], 4, true))
            .expect("fresh submission");
        // The render sees only the fresh block: nothing rejected was published, so there is no
        // stale block to discard ahead of it.
        let mut output = [7.0_f32; 4];
        let report = consumer.read_block(&mut [&mut output]).expect("fresh read");
        assert_eq!(report.active_generation, SourceGeneration(2));
        assert_eq!(report.copied_frames, 4);
        assert_eq!(output, fresh);
        assert_eq!(consumer.telemetry().stale_generation_discard_count, 0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn one_four_channel_source_fans_out_to_three_inputs_in_the_sequential_executor() {
        use core::num::NonZeroUsize;
        use effect_contract::{LatencySamples, TailSamples};
        use engine::{
            QuantumFrames,
            realtime::{PlanarBufferMut, RenderEnvelope, RenderIo, RenderTime},
        };
        use graph::{
            DependencyLevel, GraphEdge, GraphEdgeId, GraphNode, GraphPortId, GraphPortKind,
            GraphResourceEstimate, GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec,
            PreparedGraphPlan, PreparedGraphPlanParts, StableGraphId, TrackStage,
        };

        struct Noop;
        impl GraphRuntimeProcessor for Noop {
            fn process(
                &mut self,
                _block: graph::GraphBindingBlock<'_>,
            ) -> Result<(), engine::realtime::RenderError> {
                Ok(())
            }
        }

        let envelope = RenderEnvelope {
            sample_rate: RATE,
            quantum: QuantumFrames(2),
            output_channels: NonZeroUsize::new(2).expect("two"),
        };
        let track = |id| graph::GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(id).expect("track ID"),
            stage: TrackStage::Input,
        };
        let inputs = [track("a"), track("b"), track("c")];
        let output = graph::GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };
        let edge = |source: graph::GraphNodeId, route: &str| GraphEdge {
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
                track_delays: Vec::new(),
                effects: Vec::new(),
                effect_controls: Vec::new(),
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers: Vec::new(),
                effect_observations: Vec::new(),
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
            nodes: vec![graph::GraphNodeBinding::new(output.clone(), Box::new(Noop))],
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
                    graph::GraphNodeBinding::new(output.clone(), Box::new(Noop)),
                    graph::GraphNodeBinding::new(inputs[0].clone(), Box::new(Noop)),
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
        let mut sequential_pcm = [0.0_f32; 4];
        sequential
            .render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut sequential_pcm, 2, 2, 2).expect("output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("render");
        assert_eq!(sequential_pcm, [10.0, 10.0, 10.0, 10.0]);
    }

    fn plane_bits(plane: &[f32]) -> Vec<u32> {
        plane.iter().map(|sample| sample.to_bits()).collect()
    }

    #[test]
    fn seek_preparation_preserves_configured_depth_and_prefetches_without_consuming() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(1, 4, 12)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(100),
        })
        .expect("seek");
        assert!(consumer.prepare_seek(SourceGeneration(2), SourceFrame(100)));
        let blocks = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ];
        for (index, samples) in blocks.iter().enumerate() {
            host.submit(chunk(2, 100 + index as u64 * 4, &[samples], 4, false))
                .expect("configured admission");
        }
        assert!(matches!(
            host.submit(chunk(2, 112, &[&blocks[0]], 4, false)),
            Err(HostChunkError::Full { .. })
        ));
        // Prefetching the first queued block retains its storage without opening an admission.
        assert!(consumer.prepare_seek(SourceGeneration(2), SourceFrame(100)));
        assert_eq!(consumer.telemetry().cumulative_read_frames, 0);
        assert!(matches!(
            host.submit(chunk(2, 112, &[&blocks[0]], 4, false)),
            Err(HostChunkError::Full { .. })
        ));
        let mut output = [f32::NAN; 4];
        let report = consumer
            .read_block(&mut [&mut output])
            .expect("first target block");
        assert_eq!(output, blocks[0]);
        assert_eq!(report.copied_frames, 4);
        assert_eq!(report.cumulative_read_frames, 4);
        assert_eq!(report.active_generation, SourceGeneration(2));
    }

    /// Gate 1, "at every point" (#917): while the render borrows the played planes, the producer
    /// still admits exactly the configured number of blocks, and filling the ring to
    /// backpressure never touches the borrowed storage. Only the next `begin_block` releases it.
    ///
    /// Red mutation: drop the retained block from the allocation -- the producer admits one
    /// block fewer while the planes are borrowed.
    #[test]
    fn played_planes_stay_intact_while_the_producer_fills_its_configured_depth() {
        for configured in 1_u64..=3 {
            let (producer, mut consumer, report) =
                PcmSourceRing::prepare(config(2, 4, 4 * configured)).expect("ring");
            assert_eq!(report.transfer_block_count, configured);
            let mut host = producer.into_host_chunk_provider(RATE);
            let first_left = [1.0, 2.0, 3.0, 4.0];
            let first_right = [-1.0, -2.0, -3.0, -4.0];
            host.submit(chunk(1, 0, &[&first_left, &first_right], 4, false))
                .expect("first block");
            let played = consumer.begin_block();
            assert_eq!(played.copied_frames, 4);
            let left = consumer.played_plane(0).expect("left plane");
            let right = consumer.played_plane(1).expect("right plane");
            // Mid-render: the producer admits exactly `configured` further blocks, then `Full`.
            let poison = [f32::NAN; 4];
            let mut admitted = 0_u64;
            let mut start = 4;
            while host
                .submit(chunk(1, start, &[&poison, &poison], 4, false))
                .is_ok()
            {
                admitted += 1;
                start += 4;
                assert!(admitted <= configured, "admitted past the configured depth");
            }
            assert_eq!(
                admitted, configured,
                "configured depth while the render holds"
            );
            assert_eq!(plane_bits(left), first_left.map(f32::to_bits));
            assert_eq!(plane_bits(right), first_right.map(f32::to_bits));
            // The next boundary releases the held block and exactly one admission opens.
            let next = consumer.begin_block();
            assert_eq!(next.copied_frames, 4);
            assert!(
                consumer
                    .played_plane(0)
                    .expect("next plane")
                    .iter()
                    .all(|sample| sample.is_nan())
            );
            host.submit(chunk(1, start, &[&poison, &poison], 4, false))
                .expect("one admission after the release");
            assert!(matches!(
                host.submit(chunk(1, start + 4, &[&poison, &poison], 4, false)),
                Err(HostChunkError::Full { .. })
            ));
        }
    }

    /// Gate 2 (#917): `played_plane` on a short block is the played frames followed by exact
    /// `+0.0` words to the quantum, written in place over storage that held poison; it is `None`
    /// before any block, on an out-of-range channel, past the end of the region and after
    /// `end_block`; and `copy_channel` emits the same supplied PCM and positive-zero tail.
    ///
    /// Red mutation: drop the in-place tail fill in `play` -- the borrowed tail keeps the poison.
    #[test]
    fn played_plane_is_the_quantum_with_a_zeroed_short_tail_and_none_without_a_block() {
        let (producer, mut consumer, _) = PcmSourceRing::prepare(config(2, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        assert!(consumer.played_plane(0).is_none(), "nothing played yet");
        assert_copy_channel(&consumer, 0, &[0.0; 4], "before any block");

        // Rotate every allocated block (one configured, one retained) through a poisoned full
        // quantum, so the short block below lands in storage whose tail is not zero.
        let poison = [-0.0, f32::NAN, 1.0e30, -7.5];
        for (index, start) in [0_u64, 4].into_iter().enumerate() {
            host.submit(chunk(1, start, &[&poison, &poison], 4, false))
                .expect("poison block");
            let report = consumer.begin_block();
            assert_eq!(report.copied_frames, 4, "poison block {index}");
            assert_eq!(
                plane_bits(consumer.played_plane(1).expect("poison plane")),
                poison.map(f32::to_bits)
            );
        }

        let left_short = [5.0, -0.0];
        let right_short = [f32::MIN_POSITIVE, -6.0];
        host.submit(chunk(1, 8, &[&left_short, &right_short], 2, true))
            .expect("short EOF block");
        let short = consumer.begin_block();
        assert_eq!(short.copied_frames, 2);
        assert!(short.end_of_region);
        for (channel, played) in [(0_u32, left_short), (1, right_short)] {
            let plane = consumer.played_plane(channel).expect("short plane");
            assert_eq!(plane.len(), 4, "a whole quantum");
            let bits = plane_bits(plane);
            assert_eq!(bits[..2], played.map(f32::to_bits));
            assert_eq!(bits[2..], [0_u32; 2], "exact +0.0 tail words");
            // Repeat borrows see the same words: nothing was consumed.
            assert_eq!(
                plane_bits(consumer.played_plane(channel).expect("repeat plane")),
                bits
            );
            assert_copy_channel(
                &consumer,
                channel,
                &[played[0], played[1], 0.0, 0.0],
                "short block",
            );
        }
        assert!(consumer.played_plane(2).is_none(), "out-of-range channel");

        // `end_block` ends the quantum: no plane, and the copy writes the whole-plane silence.
        consumer.end_block();
        assert!(consumer.played_plane(0).is_none());
        assert_copy_channel(&consumer, 0, &[0.0; 4], "after end_block");

        // Past the end of the region nothing is played.
        let after = consumer.begin_block();
        assert!(after.end_of_region);
        assert_eq!(after.copied_frames, 0);
        assert!(consumer.played_plane(0).is_none());
        assert!(consumer.played_plane(1).is_none());
        assert_copy_channel(&consumer, 1, &[0.0; 4], "past the end of the region");

        // An underrun (in-region, nothing queued) plays nothing either.
        let (_producer, mut starved, _) = PcmSourceRing::prepare(config(1, 4, 8)).expect("ring");
        let underrun = starved.begin_block();
        assert!(underrun.underrun_event);
        assert!(starved.played_plane(0).is_none());
    }

    /// The driver maps each claim's `(left_channel, right_channel)` onto the played block, holds
    /// through every claim copy, and releases on the next `begin_block`, on a seek preparation,
    /// and at once when there are no claims (#917).
    #[test]
    fn graph_driver_played_planes_map_claims_until_the_next_begin_or_seek() {
        let channels: [[f32; 4]; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [10.0, 20.0, 30.0, 40.0],
            [100.0, 200.0, 300.0, 400.0],
            [1000.0, 2000.0, 3000.0, 4000.0],
        ];
        let planes: Vec<&[f32]> = channels.iter().map(|plane| &plane[..]).collect();
        let (producer, consumer, _) = PcmSourceRing::prepare(config(4, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &planes, 4, false))
            .expect("first block");
        let mut first = test_mapping(0);
        first.right_channel = 1;
        let mut second = test_mapping(1);
        second.left_channel = 3;
        second.right_channel = 2;
        let mut driver = test_driver(consumer, vec![first, second]);
        assert!(driver.played_planes(0).is_none(), "nothing played yet");

        driver.begin_block(0, 4).expect("begin");
        assert_eq!(
            driver.played_planes(0),
            Some((&channels[0][..], &channels[1][..]))
        );
        assert_eq!(
            driver.played_planes(1),
            Some((&channels[3][..], &channels[2][..]))
        );
        assert!(driver.played_planes(2).is_none(), "out-of-range claim");
        let (mut left, mut right) = ([0.0; 4], [0.0; 4]);
        for claim in 0..2 {
            driver
                .copy_track_input(claim, &mut left, &mut right)
                .expect("claim copy");
        }
        assert_eq!(
            driver.played_planes(1),
            Some((&channels[3][..], &channels[2][..])),
            "held after every claim copied"
        );

        // Nothing queued: the next begin releases and plays nothing.
        driver.begin_block(4, 4).expect("underrun begin");
        assert!(driver.played_planes(0).is_none());
        assert!(driver.observation_validity().source_underrun);

        // A seek preparation releases a played block.
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(100),
        })
        .expect("seek");
        host.submit(chunk(2, 100, &planes, 4, false))
            .expect("post-seek block");
        host.submit(chunk(2, 104, &planes, 4, false))
            .expect("second post-seek block");
        driver.begin_block(8, 4).expect("post-seek begin");
        assert!(driver.played_planes(0).is_some());
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(200),
        })
        .expect("second seek");
        assert!(driver.prepare_source_seek(0, 3, 200));
        assert!(
            driver.played_planes(0).is_none(),
            "seek preparation releases"
        );

        // No claims: `begin_block` releases at once.
        let (producer, consumer, _) = PcmSourceRing::prepare(config(1, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&channels[0]], 4, false))
            .expect("first block");
        let mut driver = test_driver(consumer, Vec::new());
        driver.begin_block(0, 4).expect("zero-claim begin");
        assert!(
            driver.sources[0]
                .consumer
                .as_ref()
                .expect("occupied")
                .played_plane(0)
                .is_none()
        );
    }

    /// Slice #1271: a successor graph plan takes the predecessor's source consumer at the swap
    /// block, through a vacant entry and an installed carry program.
    #[cfg(not(target_arch = "wasm32"))]
    mod carry {
        use super::*;
        use core::num::NonZeroUsize;
        use effect_contract::{LatencySamples, TailSamples};
        use engine::{
            QuantumFrames,
            realtime::{
                CarryOutcome, PlanExchangeConfig, PlanarBufferMut, PreparedRenderPlan,
                RenderEnvelope, RenderIo, SwapOutcome, plan_exchange,
            },
        };
        use graph::{
            DependencyLevel, GraphCarryInstallError, GraphCarryProgram, GraphEdge, GraphEdgeId,
            GraphNode, GraphNodeId, GraphNodeObserverBinding, GraphObservationBlock,
            GraphObservationValidity, GraphPortId, GraphPortKind, GraphResourceEstimate,
            GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor, GraphSpec,
            PreparedGraphPlan, PreparedGraphPlanParts, StableGraphId, TrackStage,
        };
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU8, Ordering};

        const FRAMES: u32 = 4;
        const BLOCKS: usize = 8;
        const SWAP_BLOCK: usize = 4;

        type Block = [u32; 2 * FRAMES as usize];

        struct Noop;
        impl GraphRuntimeProcessor for Noop {
            fn process(
                &mut self,
                _block: graph::GraphBindingBlock<'_>,
            ) -> Result<(), engine::realtime::RenderError> {
                Ok(())
            }
        }

        /// The last block's source facts: bit 0 underrun, bit 1 generation change, bit 2 seen.
        struct Validity(Arc<AtomicU8>);
        impl GraphRuntimeObserver for Validity {
            fn observe(
                &mut self,
                _block: GraphObservationBlock<'_>,
            ) -> Result<(), engine::realtime::RenderError> {
                Ok(())
            }

            fn observe_with_validity(
                &mut self,
                _block: GraphObservationBlock<'_>,
                validity: GraphObservationValidity,
            ) -> Result<(), engine::realtime::RenderError> {
                let bits = u8::from(validity.source_underrun)
                    | u8::from(validity.source_generation_changed) << 1
                    | 4;
                self.0.store(bits, Ordering::Relaxed);
                Ok(())
            }
        }

        /// `(source_underrun, source_generation_changed)` of the last observed block.
        fn take_validity(cell: &AtomicU8) -> (bool, bool) {
            let bits = cell.swap(0, Ordering::Relaxed);
            assert_ne!(bits & 4, 0, "the observer saw no block");
            (bits & 1 != 0, bits & 2 != 0)
        }

        fn envelope() -> RenderEnvelope {
            RenderEnvelope {
                sample_rate: RATE,
                quantum: QuantumFrames(FRAMES),
                output_channels: NonZeroUsize::new(2).expect("two"),
            }
        }

        fn track(id: &str) -> GraphNodeId {
            GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(id).expect("track ID"),
                stage: TrackStage::Input,
            }
        }

        fn output() -> GraphNodeId {
            GraphNodeId::Output {
                output_id: StableGraphId::parse("main").expect("output ID"),
            }
        }

        fn inputs() -> [GraphNodeId; 2] {
            [track("a"), track("b")]
        }

        /// Two track inputs summed into the output: the fan-out test's shape with two claims.
        fn graph_plan() -> PreparedGraphPlan {
            let edge = |source: GraphNodeId, route: &str| GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse(route).expect("route ID"),
                },
                source: GraphPortId {
                    node: source,
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: output(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: format!("$.{route}"),
            };
            let [a, b] = inputs();
            let schedule = vec![a.clone(), b.clone(), output()];
            PreparedGraphPlan::new(PreparedGraphPlanParts {
                plan_id: 1,
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
                    edges: vec![edge(a.clone(), "a"), edge(b.clone(), "b")],
                },
                sequential_schedule: schedule.clone(),
                dependency_levels: vec![
                    DependencyLevel {
                        level: 0,
                        nodes: vec![a, b],
                    },
                    DependencyLevel {
                        level: 1,
                        nodes: vec![output()],
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
                envelope: envelope(),
                required_bindings: schedule,
                routes: Vec::new(),
                track_delays: Vec::new(),
                effects: Vec::new(),
                effect_controls: Vec::new(),
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers: Vec::new(),
                effect_observations: Vec::new(),
            })
        }

        /// Claim `a` reads channels `(0, 1)` and claim `b` reads `(1, 0)`, both of source 0.
        fn mappings() -> Vec<SourceGraphTrackMapping> {
            let [a, b] = inputs();
            vec![
                SourceGraphTrackMapping {
                    node: a,
                    source_index: 0,
                    left_channel: 0,
                    right_channel: 1,
                },
                SourceGraphTrackMapping {
                    node: b,
                    source_index: 0,
                    left_channel: 1,
                    right_channel: 0,
                },
            ]
        }

        /// Bind a graph plan over `source`, observing the output's source facts into `validity`.
        fn bind(source: SourceGraphSource, validity: &Arc<AtomicU8>) -> PreparedRenderPlan {
            let set =
                prepare_graph_source_set(envelope(), vec![source], mappings()).expect("source set");
            let bindings = GraphRuntimeBindings {
                envelope: envelope(),
                nodes: vec![graph::GraphNodeBinding::new(output(), Box::new(Noop))],
                observers: vec![GraphNodeObserverBinding::new(
                    output(),
                    1,
                    Box::new(Validity(Arc::clone(validity))),
                )],
            };
            match graph_plan().bind_with_source_set(bindings, set) {
                Ok(plan) => plan,
                Err(failure) => panic!("bind failed: {}", failure.code),
            }
        }

        /// Channel 0 is `1 + frame` and channel 1 is `0.25 * (1 + frame)`: no exact-zero sample,
        /// and every frame distinct.
        fn frame_samples(start: u64) -> [[f32; FRAMES as usize]; 2] {
            let first = core::array::from_fn(|frame| (start as f32) + frame as f32 + 1.0);
            [first, first.map(|value: f32| 0.25 * value)]
        }

        /// A two-channel ring holding all `BLOCKS` blocks of generation 1, from frame 0.
        fn filled_source() -> (HostChunkProvider, SourceGraphSource) {
            let config = PcmSourceRingConfig {
                channel_count: 2,
                quantum_frames: QuantumFrames(FRAMES),
                frame_capacity: BLOCKS as u64 * u64::from(FRAMES),
                initial_generation: SourceGeneration(1),
            };
            let (producer, consumer, resources) = PcmSourceRing::prepare(config).expect("ring");
            let mut host = producer.into_host_chunk_provider(RATE);
            for block in 0..BLOCKS as u64 {
                let planes = frame_samples(block * u64::from(FRAMES));
                host.submit(chunk(
                    1,
                    block * u64::from(FRAMES),
                    &[&planes[0], &planes[1]],
                    FRAMES,
                    false,
                ))
                .expect("source PCM");
            }
            (host, SourceGraphSource::new(consumer, resources, 0, 0))
        }

        fn io(output: &mut [f32; 2 * FRAMES as usize]) -> RenderIo<'_> {
            RenderIo {
                output: PlanarBufferMut::try_new(output, 2, FRAMES as usize, FRAMES as usize)
                    .expect("output"),
            }
        }

        fn bits(output: &[f32; 2 * FRAMES as usize]) -> Block {
            output.map(f32::to_bits)
        }

        fn plan_block(plan: &mut PreparedRenderPlan) -> Block {
            let mut output = [f32::NAN; 2 * FRAMES as usize];
            let sample = plan.next_absolute_sample();
            plan.render_contiguous(io(&mut output), sample)
                .expect("render");
            bits(&output)
        }

        /// One unswapped plan over the filled ring, for `BLOCKS` blocks.
        fn reference() -> Vec<Block> {
            let validity = Arc::new(AtomicU8::new(0));
            let (_host, source) = filled_source();
            let mut plan = bind(source, &validity);
            let blocks: Vec<Block> = (0..BLOCKS).map(|_| plan_block(&mut plan)).collect();
            assert!(
                blocks
                    .iter()
                    .flatten()
                    .all(|word| f32::from_bits(*word) != 0.0)
            );
            blocks
        }

        fn exchange_config() -> PlanExchangeConfig {
            PlanExchangeConfig {
                publication_capacity: NonZeroUsize::new(1).expect("one"),
                retirement_capacity: NonZeroUsize::new(1).expect("one"),
            }
        }

        /// Plan A over the filled ring for `SWAP_BLOCK` blocks through a plan exchange, then the
        /// vacant plan B, given the program `program(identity of A)`, for the rest. Returns every
        /// block, the swap block's carry outcome, and each block's `(underrun, generation)` facts.
        fn swap(
            program: impl FnOnce(u64) -> Option<GraphCarryProgram>,
        ) -> (Vec<Block>, CarryOutcome, Vec<(bool, bool)>) {
            let validity = Arc::new(AtomicU8::new(0));
            let (_host, source) = filled_source();
            let mut predecessor = bind(source, &validity);
            let identity = graph::plan_identity(&mut predecessor).expect("graph plan");
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(predecessor, exchange_config()).expect("exchange");
            let mut blocks = Vec::new();
            let mut facts = Vec::new();
            let mut carry = CarryOutcome::NotRequested;
            let mut successor = Some(bind(SourceGraphSource::vacant(2), &validity));
            let mut program = Some(program);
            for block in 0..BLOCKS {
                if block == SWAP_BLOCK {
                    let mut plan = successor.take().expect("successor");
                    if let Some(program) = (program.take().expect("program"))(identity) {
                        graph::install_carry_program(&mut plan, program).expect("install");
                    }
                    assert!(publisher.publish(plan).is_ok());
                }
                let mut output = [f32::NAN; 2 * FRAMES as usize];
                let sample = owner.next_absolute_sample();
                let report = owner
                    .render_contiguous(io(&mut output), sample)
                    .expect("render");
                let expected_swap = if block == SWAP_BLOCK {
                    carry = report.carry;
                    SwapOutcome::Applied
                } else {
                    assert_eq!(report.carry, CarryOutcome::NotRequested);
                    SwapOutcome::None
                };
                assert_eq!(report.swap, expected_swap, "block {block}");
                blocks.push(bits(&output));
                facts.push(take_validity(&validity));
            }
            (blocks, carry, facts)
        }

        fn carry_source_zero(predecessor: u64) -> GraphCarryProgram {
            GraphCarryProgram {
                predecessor,
                sources: vec![(0, 0)].into_boxed_slice(),
            }
        }

        /// Gate 1. Red if the consumer is not moved into the vacant entry at the swap block (the
        /// successor would render silence there) or the move lands a block late.
        #[test]
        fn a_carried_consumer_continues_the_predecessor_audio_gap_free() {
            let reference = reference();
            let (blocks, carry, facts) = swap(|identity| Some(carry_source_zero(identity)));
            assert_eq!(blocks, reference);
            assert_eq!(carry, CarryOutcome::Carried);
            assert!(
                facts.iter().all(|&fact| fact == (false, false)),
                "{facts:?}"
            );
        }

        /// Gate 2. Red if a mismatched predecessor still hands its ring over (the successor
        /// would play the predecessor's audio) or a vacant entry renders anything but `+0.0`
        /// without flagging an underrun.
        #[test]
        fn a_vacant_source_without_a_matching_carry_renders_silence_and_an_underrun() {
            let reference = reference();
            let silence: Block = [0.0_f32.to_bits(); 2 * FRAMES as usize];
            let outcomes = [
                (CarryOutcome::NotRequested, None),
                (CarryOutcome::PredecessorMismatch, Some(1_u64)),
            ];
            for (expected, offset) in outcomes {
                let (blocks, carry, facts) = swap(|identity| {
                    offset.map(|offset| carry_source_zero(identity.wrapping_add(offset)))
                });
                assert_eq!(carry, expected);
                assert_eq!(blocks[..SWAP_BLOCK], reference[..SWAP_BLOCK]);
                assert!(blocks[SWAP_BLOCK..].iter().all(|block| *block == silence));
                assert!(
                    facts[..SWAP_BLOCK]
                        .iter()
                        .all(|&fact| fact == (false, false))
                );
                assert!(
                    facts[SWAP_BLOCK..]
                        .iter()
                        .all(|&fact| fact == (true, false))
                );
            }
            // A predecessor index the predecessor does not have refuses the whole move.
            let (blocks, carry, _) = swap(|identity| {
                Some(GraphCarryProgram {
                    predecessor: identity,
                    sources: vec![(0, 1)].into_boxed_slice(),
                })
            });
            assert_eq!(carry, CarryOutcome::PredecessorMismatch);
            assert!(blocks[SWAP_BLOCK..].iter().all(|block| *block == silence));
        }

        /// Gate 3. Red if the carry drops the predecessor's pending generation change: the
        /// successor's first block would claim continuity across a seek.
        #[test]
        fn a_seek_prepared_before_the_swap_is_reported_in_the_successors_first_block() {
            let validity = Arc::new(AtomicU8::new(0));
            let config = PcmSourceRingConfig {
                channel_count: 2,
                quantum_frames: QuantumFrames(FRAMES),
                frame_capacity: 4 * u64::from(FRAMES),
                initial_generation: SourceGeneration(1),
            };
            let (producer, consumer, resources) = PcmSourceRing::prepare(config).expect("ring");
            let mut host = producer.into_host_chunk_provider(RATE);
            let first = frame_samples(0);
            host.submit(chunk(1, 0, &[&first[0], &first[1]], FRAMES, false))
                .expect("first block");
            let mut predecessor =
                bind(SourceGraphSource::new(consumer, resources, 0, 0), &validity);
            plan_block(&mut predecessor);
            assert_eq!(take_validity(&validity), (false, false));

            const SEEK: u64 = 1_000;
            host.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(SEEK),
            })
            .expect("seek");
            let sought = [frame_samples(SEEK), frame_samples(SEEK + u64::from(FRAMES))];
            for (index, planes) in sought.iter().enumerate() {
                let start = SEEK + index as u64 * u64::from(FRAMES);
                host.submit(chunk(2, start, &[&planes[0], &planes[1]], FRAMES, false))
                    .expect("sought block");
            }
            assert!(predecessor.prepare_source_seek(0, 2, SEEK));

            let identity = graph::plan_identity(&mut predecessor).expect("graph plan");
            let mut successor = bind(SourceGraphSource::vacant(2), &validity);
            graph::install_carry_program(&mut successor, carry_source_zero(identity))
                .expect("install");
            assert_eq!(
                successor.adopt_predecessor_plan(&mut predecessor),
                CarryOutcome::Carried
            );
            for (index, planes) in sought.iter().enumerate() {
                let block = plan_block(&mut successor);
                let sum: Vec<u32> = (0..FRAMES as usize)
                    .map(|frame| (planes[0][frame] + planes[1][frame]).to_bits())
                    .collect();
                assert_eq!(block[..FRAMES as usize], sum[..], "left, block {index}");
                assert_eq!(block[FRAMES as usize..], sum[..], "right, block {index}");
                assert_eq!(
                    take_validity(&validity),
                    (false, index == 0),
                    "block {index}"
                );
            }
        }

        /// Gate 4. Red if install accepts a program that names a source index the successor does
        /// not have, a source that already owns a ring, or one index twice.
        #[test]
        fn install_refuses_out_of_range_occupied_and_repeated_sources() {
            let validity = Arc::new(AtomicU8::new(0));
            let mut vacant = bind(SourceGraphSource::vacant(2), &validity);
            let (_host, source) = filled_source();
            let mut occupied = bind(source, &validity);
            let program = |sources: &[(u32, u32)]| GraphCarryProgram {
                predecessor: 1,
                sources: sources.into(),
            };
            assert_eq!(
                graph::install_carry_program(&mut vacant, program(&[(1, 0)])),
                Err(GraphCarryInstallError::SourceIndexOutOfRange)
            );
            assert_eq!(
                graph::install_carry_program(&mut occupied, program(&[(0, 0)])),
                Err(GraphCarryInstallError::SourceNotVacant)
            );
            assert_eq!(
                graph::install_carry_program(&mut vacant, program(&[(0, 0), (0, 1)])),
                Err(GraphCarryInstallError::DuplicateSourceIndex)
            );
            let mut plain = PreparedRenderPlan::prepare(engine::realtime::PrepareRenderPlan {
                plan_id: 1,
                envelope: envelope(),
                scratch: &[],
            })
            .expect("plain plan");
            assert_eq!(
                graph::install_carry_program(&mut plain, program(&[(0, 0)])),
                Err(GraphCarryInstallError::NotAGraphPlan)
            );
            assert_eq!(graph::plan_identity(&mut plain), None);
            // A refused install charges nothing; an accepted one charges its move table.
            assert_eq!(graph::carry_program_retained_bytes(&mut vacant), 0);
            graph::install_carry_program(&mut vacant, program(&[(0, 0)])).expect("install");
            assert_eq!(
                graph::carry_program_retained_bytes(&mut vacant),
                core::mem::size_of::<(u32, u32)>() as u64
            );
            let vacant_identity = graph::plan_identity(&mut vacant).expect("identity");
            let occupied_identity = graph::plan_identity(&mut occupied).expect("identity");
            assert_ne!(vacant_identity, occupied_identity);
            assert!(vacant_identity != 0 && occupied_identity != 0);
        }

        /// D1. Red if a vacant source charges ring bytes or is not counted apart.
        #[test]
        fn a_vacant_source_charges_no_ring_and_is_counted_apart() {
            let (_host, occupied_source) = filled_source();
            let ring = occupied_source.resources.expect("ring report");
            let occupied = prepare_graph_source_set(envelope(), vec![occupied_source], mappings())
                .expect("occupied set");
            let vacant = prepare_graph_source_set(
                envelope(),
                vec![SourceGraphSource::vacant(2)],
                mappings(),
            )
            .expect("vacant set");
            let (occupied_report, vacant_report) =
                (occupied.resource_report(), vacant.resource_report());
            assert_eq!(vacant_report.pcm_payload_already_charged_bytes, 0);
            assert_eq!(
                occupied_report.overhead_bytes - vacant_report.overhead_bytes,
                ring.overhead_bytes
            );
            assert_eq!(
                vacant_report.total_engine_owned_bytes,
                vacant_report.overhead_bytes
            );
            assert_eq!(
                (occupied.vacant_source_count(), vacant.vacant_source_count()),
                (0, 1)
            );
            // Mappings are still checked against a vacant source's channel count.
            assert!(matches!(
                prepare_graph_source_set(
                    envelope(),
                    vec![SourceGraphSource::vacant(1)],
                    mappings()
                ),
                Err(SourceGraphSourceSetError::ChannelIndex)
            ));
        }
    }
}
