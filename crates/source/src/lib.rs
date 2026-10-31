//! Bounded, prepared PCM source rings for just-in-time audio delivery.
//!
//! A source ring splits ownership between one non-render producer and one exclusive render
//! consumer. Transfer blocks, queues, and their PCM storage are all created by [`PcmSourceRing`]
//! before rendering starts. The consumer only moves prepared blocks, copies or lends their samples
//! in place (zeroing a short block's tail once), and updates local saturating counters.

#![allow(missing_docs)]

use core::{
    alloc::Layout,
    cmp,
    mem::size_of,
    num::NonZeroUsize,
    sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
};
use std::{fmt, sync::Arc};

use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{
        Consumer, Producer, PublishedWordsReader, PublishedWordsWriter, QueueEmpty, QueueFull,
        QueueGeneration, SpscError, bounded_spsc, bounded_spsc_move, bounded_spsc_retained_payload,
        published_words, published_words_retained_bytes,
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
    /// reads source time from `anchor_sample` (issue #1274).
    ///
    /// `anchor_sample` is on the plan's source-read clock -- the absolute sample of source time a
    /// block reads, which the graph hands its source set each block and every plan swap continues;
    /// equal to the render clock in this version (#1316 D1) -- and must be a multiple of the
    /// quantum. The output hears `frame` the plan's latency later. The render consumer holds the
    /// seek until the first block starting at or after the anchor; observed late, it starts at
    /// `frame + (block start - anchor_sample)`, so the source is in time either way. While the seek
    /// is held, already queued PCM of the playing generation keeps playing and the new generation's
    /// first submitted block waits as the pending block, so a host may prime the ring from `frame`
    /// as soon as the seek is accepted.
    SeekAt {
        /// The nonzero generation to make audible.
        generation: SourceGeneration,
        /// Decoded source frame that enters the graph at `anchor_sample`.
        frame: SourceFrame,
        /// Source-read sample of the block `frame` enters in; a multiple of the quantum.
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
    /// The seek runway `R` in blocks (#2205): a live plain seek lands at the first block at which
    /// `R` blocks of its generation are queued (or its region ends), and at the latest `R` blocks
    /// after render observed it. At most half the ring ([`PcmSourceRingError::SeekRunwayAboveHalfRing`]).
    /// A host prepares [`seek_runway_blocks`]; `0` lands a plain seek at the next block.
    pub seek_runway_blocks: u32,
}

/// The audio a live plain seek waits for at most, and the runway it waits for (#2205, owner
/// decision 2026-10-29): 50 ms, in blocks.
pub const SEEK_RUNWAY_MS: u32 = 50;

/// The seek runway `R = D` of one ring (#2205, root ruling 2): `min(ceil(50 ms / Q),
/// transfer_block_count / 2)` blocks, at `sample_rate_hz` and `quantum_frames`. A zero quantum is
/// `0`. One constant for every host: the C ABI and the browser prepare it, and the browser's
/// control Worker derives its admission headroom from the same value.
#[must_use]
pub const fn seek_runway_blocks(
    sample_rate_hz: u32,
    quantum_frames: u32,
    transfer_block_count: u64,
) -> u32 {
    if quantum_frames == 0 {
        return 0;
    }
    let frames = (sample_rate_hz as u64 * SEEK_RUNWAY_MS as u64).div_ceil(1_000);
    let blocks = frames.div_ceil(quantum_frames as u64);
    let cap = transfer_block_count / 2;
    let runway = if blocks < cap { blocks } else { cap };
    if runway > u32::MAX as u64 {
        u32::MAX
    } else {
        runway as u32
    }
}

/// Immutable shape shared by the two endpoints of one prepared source ring.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PcmSourceShape {
    pub channel_count: u32,
    pub quantum_frames: QuantumFrames,
    pub frame_capacity: u64,
    pub transfer_block_count: u64,
    /// The seek runway and deadline `R = D` in blocks (#2205), fixed at prepare.
    pub seek_runway_blocks: u32,
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
    /// The ring's shared seek report (#1316 D2): one record, with its shared-owner header.
    pub seek_report_bytes: u64,
    /// The ring's shared `prime_required` flag (#1320 D4): one `AtomicBool`, with its
    /// shared-owner header.
    pub prime_required_bytes: u64,
    /// The ring's shared seek-readiness word (#2205): one `AtomicU64`, with its shared-owner
    /// header.
    pub seek_readiness_bytes: u64,
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
    /// The seek runway is above half the ring's transfer blocks (#2205).
    SeekRunwayAboveHalfRing,
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
            | Self::InternalQueueCapacity
            | Self::SeekRunwayAboveHalfRing => SourceDiagnosticCode::ResourceLimit,
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
    /// A timeline seek to a sample above [`TIMELINE_SAMPLE_LIMIT`] (#1640 D4): past it a
    /// timeline value would stop being an exact `i64` for every reader.
    FrameOutOfRange,
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
    /// In-region frames emitted as positive zero because an anchored seek is held: the host asked
    /// the source to wait, so the frames are not an underrun.
    pub held_frames: u32,
    /// Whether this call was a held block (`held_frames != 0`).
    pub held: bool,
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
    /// Saturating cumulative held in-region frame count.
    pub cumulative_held_frames: u64,
    /// Saturating cumulative held block count.
    pub cumulative_held_blocks: u64,
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
    /// Frames rendered as silence while an anchored seek was held (not underruns).
    pub held_frames: u64,
    /// Blocks rendered as silence while an anchored seek was held (not underruns).
    pub held_blocks: u64,
    pub end_of_region: bool,
    /// Native-decoder replacements, or zero for a host-supplied consumer.
    pub native_decoder_sanitized_samples: u64,
}

/// Where the ring's newest generation landed, as render published it (#1316 D2, D3).
///
/// A level, not a queue: render overwrites it at the block a seek applies at, and whenever a
/// cumulative counter moves. Before the ring's first block it reads generation `0`, meaning "not
/// yet rendered".
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SourceSeekReport {
    /// The generation in effect; `0` until the ring's first block.
    pub generation: u64,
    /// The plan's source-read sample of the first block `generation` played in: the clock an
    /// anchored seek's `anchor_sample` is on.
    pub first_sample: u64,
    /// The source frame that block started reading at; a late anchored seek's includes its
    /// lateness.
    pub source_frame: u64,
    /// Saturating cumulative missing in-region frames (true underruns).
    pub cumulative_underrun_frames: u64,
    /// Saturating cumulative held in-region frames (an anchored seek waiting, #1318).
    pub cumulative_held_frames: u64,
}

/// The words of a [`SourceSeekReport`] in the ring's shared record.
const SEEK_REPORT_WORDS: usize = 5;

impl SourceSeekReport {
    const fn to_words(self) -> [u64; SEEK_REPORT_WORDS] {
        [
            self.generation,
            self.first_sample,
            self.source_frame,
            self.cumulative_underrun_frames,
            self.cumulative_held_frames,
        ]
    }

    const fn from_words(words: [u64; SEEK_REPORT_WORDS]) -> Self {
        let [
            generation,
            first_sample,
            source_frame,
            cumulative_underrun_frames,
            cumulative_held_frames,
        ] = words;
        Self {
            generation,
            first_sample,
            source_frame,
            cumulative_underrun_frames,
            cumulative_held_frames,
        }
    }
}

/// A seek-report read landed inside render's publication on every bounded attempt; retry later.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceSeekReportBusy;

/// The producer's view of its ring's occupancy (issue #1387): what a host-side feed sizes its
/// submissions by. Read from the producer's endpoints alone -- each queue's cursors, the
/// partner's with `Acquire` -- so it takes no lock, allocates nothing and is safe on the control
/// thread while render runs; render may only lower `queued` and raise `room` after the read.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SourceAdmission {
    /// Blocks published and not yet taken by render: chunks queued ahead of the played one, of
    /// any generation (a seek's stale chunks stay counted until render pops the seek).
    pub queued: u64,
    /// Chunks a submit would accept now: the recycled blocks the producer holds storage for,
    /// zero while a block whose push failed waits to be published first.
    pub room: u64,
}

struct TransferBlock {
    generation: SourceGeneration,
    start_frame: SourceFrame,
    frames: u32,
    end_of_region: bool,
    native_decoder_sanitized_samples: u64,
    /// The producer's monotone last-non-zero frame of this block's generation as it stood after
    /// copying this block (issue #2133, [`PcmSourceProducer::last_nonzero`]): the absolute source
    /// frame of the newest frame, in this block or an earlier one of its generation, holding a word
    /// whose bits are not `+0.0` on any channel. `None` when there is none. The block's every word
    /// is exact `+0.0` when this is below [`Self::start_frame`].
    last_nonzero: Option<SourceFrame>,
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
            last_nonzero: Some(SourceFrame(u64::MAX)),
            samples: samples.into_boxed_slice(),
        })
    }

    fn reset_metadata(&mut self) {
        self.generation = SourceGeneration(1);
        self.start_frame = SourceFrame(0);
        self.frames = 0;
        self.end_of_region = false;
        self.native_decoder_sanitized_samples = 0;
        // A recycled block claims nothing until a submit stamps it again: "not silent".
        self.last_nonzero = Some(SourceFrame(u64::MAX));
    }

    /// Whether every word this block lends or copies is exact `+0.0` (issue #2133): its frames by
    /// the producer's stamp, and the tail past `frames`, which `PcmSourceConsumer::play` zeroes.
    /// One compare; never reads a word.
    fn is_positive_zero(&self) -> bool {
        self.last_nonzero
            .is_none_or(|frame| frame.0 < self.start_frame.0)
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
        let seek_report = u64::try_from(published_words_retained_bytes::<SEEK_REPORT_WORDS>())
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
        let prime_required = u64::try_from(prime_flag_retained_bytes())
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
        let seek_readiness = u64::try_from(seek_readiness_retained_bytes())
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
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
            checked_u64_add(
                checked_u64_add(retained_pcm, seek_report)?,
                checked_u64_add(prime_required, seek_readiness)?,
            )?,
        )?;
        let total = checked_u64_add(pcm, overhead)?;
        let largest = [
            block_pcm,
            u64::try_from(size_of::<TransferBlock>())
                .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            largest_queue_allocation::<Box<TransferBlock>>(shape.allocated_block_count)?,
            largest_queue_allocation::<SourceCommand>(command_capacity)?,
            seek_report,
            prime_required,
            seek_readiness,
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
            seek_report_bytes: seek_report,
            prime_required_bytes: prime_required,
            seek_readiness_bytes: seek_readiness,
            transfer_block_metadata_bytes: metadata,
            transfer_block_pcm_bytes: pcm,
            retained_block_pcm_bytes: retained_pcm,
            overhead_bytes: overhead,
            total_engine_owned_bytes: total,
            largest_allocation_bytes: largest,
        })
    }

    /// Allocate the exact fixed source-ring resources and split producer/render ownership, for a
    /// ring that is a session of its own: it allocates its own published generation (#1642
    /// B16-R2), at `config.initial_generation`, which its producer publishes to and its consumer
    /// loads. A ring of a multi-source session is prepared with [`Self::prepare_host_region`] and
    /// the session's word. The word's bytes are the session's, so no row of the report counts
    /// them.
    pub fn prepare(
        config: PcmSourceRingConfig,
    ) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
    {
        Self::prepare_at_source_frame(
            config,
            SourceFrame(0),
            PublishedGeneration::new(config.initial_generation),
        )
    }

    /// Allocate a host-fed ring of the session whose published generation is `published` (#1642
    /// B16-R2), whose first accepted chunk begins at `initial_frame`. Every seek of its producer
    /// publishes to `published`, and its consumer observes no command above it.
    ///
    /// Shape validation, allocation, ownership, and resource accounting are identical to
    /// [`Self::prepare`]; only the initial absolute source position and the word differ.
    pub fn prepare_host_region(
        config: PcmSourceRingConfig,
        initial_frame: SourceFrame,
        published: PublishedGeneration,
    ) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
    {
        Self::prepare_at_source_frame(config, initial_frame, published)
    }

    pub(crate) fn prepare_at_source_frame(
        config: PcmSourceRingConfig,
        initial_frame: SourceFrame,
        published: PublishedGeneration,
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
        // Generation 0 until the first block: "not yet rendered" (#1316 D3).
        let (seek_report_writer, seek_report_reader) =
            published_words(SourceSeekReport::default().to_words());
        // #1320 D4: no warm successor carries a new ring.
        let prime_required = Arc::new(AtomicBool::new(false));
        // #2205: no seek pushed and no block published yet.
        let seek_readiness = Arc::new(AtomicU64::new(0));
        let mut consumer = PcmSourceConsumer {
            data_consumer,
            recycle_producer,
            command_consumer,
            shape: PcmSourceShape {
                channel_count: config.channel_count,
                quantum_frames: config.quantum_frames,
                frame_capacity: config.frame_capacity,
                transfer_block_count: shape.transfer_block_count_u64,
                seek_runway_blocks: config.seek_runway_blocks,
            },
            channel_count: config.channel_count,
            quantum_frames: config.quantum_frames.0,
            transfer_block_count: shape.transfer_block_count.get(),
            seek: SeekState::new(config.initial_generation),
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
            held_frames: 0,
            held_blocks: 0,
            native_decoder_sanitized_samples: 0,
            generation_changed: false,
            begun_block: false,
            seek_report: seek_report_writer,
            published_seek_report: SourceSeekReport::default(),
            next_source_read_sample: 0,
            prime_required: Arc::clone(&prime_required),
            seek_readiness: Arc::clone(&seek_readiness),
            session_landing: SessionLanding::NONE,
            // Every entry point sets it before it is read; until then it fences at the initial
            // generation, which holds every command.
            observed_published: config.initial_generation.0,
            published: published.clone(),
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
                seek: SeekProducer {
                    command_producer,
                    active_generation: config.initial_generation,
                    quantum_frames: config.quantum_frames.0,
                    frame_limit: None,
                    published,
                },
                shape: consumer.shape,
                channel_count: config.channel_count,
                quantum_frames: config.quantum_frames.0,
                next_write_frame: initial_frame,
                end_of_region_submitted: false,
                cumulative_written_frames: 0,
                deferred_block: None,
                native_decoder_sanitized_samples: 0,
                seek_report: seek_report_reader,
                prime_required,
                seek_readiness,
                readiness: 0,
                region_end: None,
                last_nonzero: None,
                last_nonzero_generation: config.initial_generation,
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
        if u64::from(config.seek_runway_blocks) > transfer_block_count_u64 / 2 {
            return Err(PcmSourceRingError::SeekRunwayAboveHalfRing);
        }
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
    /// The command producer and the generation in effect: the seek producer every
    /// source-transport producer shares (#1640 D1).
    seek: SeekProducer,
    shape: PcmSourceShape,
    channel_count: u32,
    quantum_frames: u32,
    next_write_frame: SourceFrame,
    end_of_region_submitted: bool,
    cumulative_written_frames: u64,
    deferred_block: Option<Box<TransferBlock>>,
    native_decoder_sanitized_samples: u64,
    /// The reader of the ring's seek report, which the consumer publishes (#1316 D2).
    seek_report: PublishedWordsReader<SEEK_REPORT_WORDS>,
    /// The ring's `prime_required` flag, shared with the consumer (#1320 D4).
    prime_required: Arc<AtomicBool>,
    /// The ring's seek-readiness word (#2205), shared with the consumer: this producer is its
    /// only writer. See [`SeekReadiness`].
    seek_readiness: Arc<AtomicU64>,
    /// What the readiness word holds; this producer is its only writer, so it never loads it.
    readiness: u64,
    /// The region's end, if the host set it ([`HostChunkProvider::set_region_end`]): a seek to it
    /// or past it is ready at once, because no PCM of its generation can follow (#2205).
    region_end: Option<SourceFrame>,
    /// Issue #2133: the absolute source frame of the newest submitted frame of
    /// [`Self::last_nonzero_generation`] holding a word whose bits are not `+0.0` (a `-0.0` word
    /// counts: conservative, cost only), or `None`. Monotone within a generation; a chunk of
    /// another generation (a seek) resets it before its own words are folded in. Advanced by the
    /// submit's copy on the control thread and stamped into each block, so render learns a
    /// block's silence by one compare.
    last_nonzero: Option<SourceFrame>,
    /// The generation [`Self::last_nonzero`] counts.
    last_nonzero_generation: SourceGeneration,
}

/// The control thread's handle on one ring's `prime_required` flag (#1320 D4).
///
/// Control sets it for the rings a warm successor carries, before the `Release` that publishes
/// the candidate; render loads it after its `Acquire` of the plan mailbox, so the flag needs no
/// ordering of its own. A handle is an `Arc` clone taken off render.
#[derive(Clone, Debug)]
pub struct PcmSourcePrimeFlag(Arc<AtomicBool>);

impl PcmSourcePrimeFlag {
    /// Mark the ring as one a warm successor carries (`true`) or not (`false`).
    pub fn set(&self, required: bool) {
        self.0.store(required, Ordering::Relaxed);
    }
}

/// Exact bytes of the ring's shared `prime_required` allocation: `Arc`'s `{ strong, weak, data }`.
fn prime_flag_retained_bytes() -> usize {
    #[repr(C)]
    struct SharedAllocation {
        strong: AtomicUsize,
        weak: AtomicUsize,
        flag: AtomicBool,
    }
    Layout::new::<SharedAllocation>().size()
}

/// Exact bytes of the ring's shared seek-readiness allocation: `Arc`'s `{ strong, weak, data }`.
fn seek_readiness_retained_bytes() -> usize {
    #[repr(C)]
    struct SharedAllocation {
        strong: AtomicUsize,
        weak: AtomicUsize,
        word: AtomicU64,
    }
    Layout::new::<SharedAllocation>().size()
}

/// The ring's seek-readiness word (#2205): what render needs to land a held live plain seek, in
/// one `AtomicU64` the producer writes (`Release`) and render reads (`Acquire`).
///
/// * bits 0..32, the **epoch**: the producer's accepted seek pushes, wrapped. Render compares it
///   with its own command pops, wrapped: equal exactly when the word describes the newest seek
///   render has popped. The command queue has one slot, so the producer is never more than one
///   push ahead and the wrap never aliases. A word of another epoch is not ready.
/// * bit 32, **fed**: a block of any generation was ever published.
/// * bit 33, **waits**: the ring was fed before the epoch's seek was pushed. A seek on a ring
///   never fed before it has no old audio to play on and nothing it waits for, so it is ready at
///   once (root ruling 4): an empty boot or a stem still loading does not hold a seek.
/// * bit 34, **ended**: the epoch's seek has no more PCM to come: a block that ends the region
///   was published, or the seek named the region's end or past it.
/// * bits 35..64, **blocks**: the epoch's published blocks, saturating.
struct SeekReadiness;

impl SeekReadiness {
    const EPOCH_MASK: u64 = 0xFFFF_FFFF;
    const FED: u64 = 1 << 32;
    const WAITS: u64 = 1 << 33;
    const ENDED: u64 = 1 << 34;
    const BLOCK_SHIFT: u32 = 35;
    const BLOCK_MAX: u64 = (1 << 29) - 1;

    /// The word after a seek push whose epoch is `epoch`: no block yet, fed kept.
    const fn after_seek(word: u64, epoch: u64, ended: bool) -> u64 {
        let fed = word & Self::FED;
        let waits = if fed != 0 { Self::WAITS } else { 0 };
        let ended = if ended { Self::ENDED } else { 0 };
        fed | waits | ended | (epoch & Self::EPOCH_MASK)
    }

    /// The word after one more block of the epoch.
    const fn after_block(word: u64, end_of_region: bool) -> u64 {
        let blocks = word >> Self::BLOCK_SHIFT;
        let blocks = if blocks < Self::BLOCK_MAX {
            blocks + 1
        } else {
            blocks
        };
        let ended = if end_of_region { Self::ENDED } else { 0 };
        (word & ((1 << Self::BLOCK_SHIFT) - 1)) | Self::FED | ended | (blocks << Self::BLOCK_SHIFT)
    }

    /// Whether `word` makes the seek render popped as its `epoch`-th command ready for a runway
    /// of `runway` blocks.
    const fn ready(word: u64, epoch: u64, runway: u32) -> bool {
        (word & Self::EPOCH_MASK) == (epoch & Self::EPOCH_MASK)
            && (word & Self::WAITS == 0
                || word & Self::ENDED != 0
                || (word >> Self::BLOCK_SHIFT) >= runway as u64)
    }
}

/// One session's published seek generation (#1642 B16-R2): the fence that makes every command of
/// one session seek visible to render at the same block entry.
///
/// One word per session, allocated on the control thread when the session's first plan is
/// prepared, and shared by every source producer, the timeline producer and the source-set driver
/// of every plan of that session: a successor's preparation takes its predecessor's word, so a
/// fresh and a carried consumer of one plan are fenced by the same object.
///
/// * Every seek push is followed by [`Self::publish`] of its generation, a `Release`
///   read-modify-write to `max(word, g)`: a per-source seek right after its one push, a session
///   seek only after its last push.
/// * Render loads the word once per block entry with `Acquire`, before the timeline and the
///   sources, and each consumer leaves in its queue (by `peek`, never `pop`) a command whose
///   generation is above the loaded value. The command keeps its slot, so nothing is dropped and a
///   later push to that slot is still backpressure; it is observed at the first block whose load
///   covers it.
///
/// A load that returns at least `g` read the publication of `g` or a later one of the same
/// control thread, so every push sequenced before it is visible: either every consumer of a
/// session seek takes it at a block entry or none does. A word never decreases.
///
/// A second word beside it names the newest **plain session seek** (#2205): the generation a
/// source set lands for every source and the timeline in one block, once every source has its
/// runway ([`SessionSeek::publish`]).
#[derive(Clone)]
pub struct PublishedGeneration(Arc<SessionWords>);

/// The session's words (#1642 B16-R2, #2205).
struct SessionWords {
    /// The published seek generation.
    published: AtomicU64,
    /// The newest plain session seek's generation, `0` before the first. Stored before the
    /// `Release` publish of the same generation, so a block entry whose load covers it reads it.
    session: AtomicU64,
    /// The newest plain session seek that lands at once ([`SessionSeek::land_at_once`]), `0`
    /// before the first: a seek with no live audio to keep. Stored before the same publish.
    immediate: AtomicU64,
}

impl PublishedGeneration {
    /// A new session's word at `initial`, the generation every prepared ring starts at. Control
    /// thread only: it allocates.
    #[must_use]
    pub fn new(initial: SourceGeneration) -> Self {
        Self(Arc::new(SessionWords {
            published: AtomicU64::new(initial.0),
            session: AtomicU64::new(0),
            immediate: AtomicU64::new(0),
        }))
    }

    /// Exact bytes of the words' shared allocation: `Arc`'s `{ strong, weak, data }`.
    #[must_use]
    pub fn retained_bytes() -> usize {
        #[repr(C)]
        struct SharedAllocation {
            strong: AtomicUsize,
            weak: AtomicUsize,
            published: AtomicU64,
            session: AtomicU64,
            immediate: AtomicU64,
        }
        Layout::new::<SharedAllocation>().size()
    }

    /// The newest generation published on this session. Control thread: its only writer.
    #[must_use]
    pub fn generation(&self) -> SourceGeneration {
        SourceGeneration(self.0.published.load(Ordering::Relaxed))
    }

    /// Publish `generation` after the last push it covers: the word becomes `max(word,
    /// generation)` with `Release`, so a block entry whose load covers it sees every such push.
    pub fn publish(&self, generation: SourceGeneration) {
        self.0.published.fetch_max(generation.0, Ordering::Release);
    }

    /// Whether `self` and `other` are the same session's word.
    #[must_use]
    pub fn is_shared_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    /// The block entry's load (render): one `Acquire` load, wait-free.
    fn load_at_block_entry(&self) -> u64 {
        self.0.published.load(Ordering::Acquire)
    }

    /// Name `generation` the newest plain session seek; before its publish (control thread).
    fn name_session_seek(&self, generation: SourceGeneration, at_once: bool) {
        if at_once {
            self.0.immediate.fetch_max(generation.0, Ordering::Relaxed);
        }
        self.0.session.fetch_max(generation.0, Ordering::Relaxed);
    }

    /// Whether the plain session seek `generation` lands at once (render): it is the newest one
    /// named to land at once. One `Relaxed` load, ordered by the block entry's `Acquire`.
    fn session_seek_lands_at_once(&self, generation: u64) -> bool {
        generation != 0 && self.0.immediate.load(Ordering::Relaxed) == generation
    }

    /// The newest plain session seek a block entry whose load returned `published` may land
    /// (render): the session word when the load covers it, else `None` (a newer seek is between
    /// its two stores, and its commands are not observed in this block). One `Relaxed` load,
    /// ordered by the entry's `Acquire`.
    fn session_seek_at_entry(&self, published: u64) -> Option<u64> {
        let session = self.0.session.load(Ordering::Relaxed);
        (session != 0 && session <= published).then_some(session)
    }
}

/// Identity: two words are equal when they are one session's.
impl PartialEq for PublishedGeneration {
    fn eq(&self, other: &Self) -> bool {
        self.is_shared_with(other)
    }
}

impl Eq for PublishedGeneration {}

impl fmt::Debug for PublishedGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PublishedGeneration")
            .field(&self.0.published.load(Ordering::Relaxed))
            .finish()
    }
}

/// One session seek (#1642 D1, B16-R2): the only path that pushes a seek without publishing it.
///
/// [`PublishedGeneration::session_seek`] opens it for one generation and anchor, after the
/// session rules; [`Self::check_source`] and [`Self::check_timeline`] check one producer's command
/// with nothing pushed; [`Self::push_source`] and [`Self::push_timeline`] push one, unpublished, so
/// render observes none of them yet. [`Self::publish`], or the drop, publishes the generation once
/// if anything was pushed, so render observes every pushed command at one block entry, and never
/// leaves one held in its slot. A caller checks every producer before it pushes to any, so the
/// seek is all or nothing.
///
/// Every producer must be of this word's session: one of another session is
/// [`SessionSeekError::ForeignSession`], checked and pushed nothing.
pub struct SessionSeek<'a> {
    published: &'a PublishedGeneration,
    generation: SourceGeneration,
    anchor_sample: Option<u64>,
    pushed: bool,
    at_once: bool,
}

/// A refusal of one [`SessionSeek`] check or push.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionSeekError {
    /// The producer's own refusal.
    Seek(SourceSeekError),
    /// The producer publishes to another session's word: render would never observe its seek
    /// under this one.
    ForeignSession,
}

impl PublishedGeneration {
    /// Open a session seek to `generation`, plain or, with `anchor_sample`, anchored (#1642 D1).
    /// Generation 0 is [`SourceSeekError::GenerationZero`]; a generation not above this word is
    /// stale ([`SourceSeekError::GenerationNotStrictlyIncreasing`] against the word), so a
    /// source removed at a higher generation cannot let a command through early. Opening it
    /// changes nothing.
    pub fn session_seek(
        &self,
        generation: u64,
        anchor_sample: Option<u64>,
    ) -> Result<SessionSeek<'_>, SourceSeekError> {
        let generation =
            SourceGeneration::new(generation).ok_or(SourceSeekError::GenerationZero)?;
        let published = self.generation();
        if generation <= published {
            return Err(SourceSeekError::GenerationNotStrictlyIncreasing {
                active: published,
                requested: generation,
            });
        }
        Ok(SessionSeek {
            published: self,
            generation,
            anchor_sample,
            pushed: false,
            at_once: false,
        })
    }
}

impl SessionSeek<'_> {
    /// The seek's generation.
    #[must_use]
    pub const fn generation(&self) -> SourceGeneration {
        self.generation
    }

    /// The command this seek sends a producer at `frame`.
    #[must_use]
    pub const fn command(&self, frame: u64) -> SourceCommand {
        match self.anchor_sample {
            None => SourceCommand::Seek {
                generation: self.generation,
                frame: SourceFrame(frame),
            },
            Some(anchor_sample) => SourceCommand::SeekAt {
                generation: self.generation,
                frame: SourceFrame(frame),
                anchor_sample,
            },
        }
    }

    fn own(&self, seek: &SeekProducer) -> Result<(), SessionSeekError> {
        if seek.published == *self.published {
            Ok(())
        } else {
            Err(SessionSeekError::ForeignSession)
        }
    }

    /// Check `provider`'s command at `frame`, pushing nothing ([`PcmSourceProducer::check_seek`]).
    pub fn check_source(
        &self,
        provider: &HostChunkProvider,
        frame: u64,
    ) -> Result<(), SessionSeekError> {
        self.own(&provider.producer.seek)?;
        provider
            .check_seek(self.command(frame))
            .map_err(SessionSeekError::Seek)
    }

    /// Check the timeline's command at `frame`, pushing nothing ([`TimelineProducer::check_seek`]).
    pub fn check_timeline(
        &self,
        timeline: &TimelineProducer,
        frame: u64,
    ) -> Result<(), SessionSeekError> {
        self.own(&timeline.seek)?;
        timeline
            .check_seek(self.command(frame))
            .map_err(SessionSeekError::Seek)
    }

    /// Push `provider`'s command at `frame`, unpublished; its rules are
    /// [`PcmSourceProducer::try_seek`]'s, and the host submits this generation's PCM from `frame`.
    pub fn push_source(
        &mut self,
        provider: &mut HostChunkProvider,
        frame: u64,
    ) -> Result<(), SessionSeekError> {
        self.own(&provider.producer.seek)?;
        provider
            .producer
            .push_seek(self.command(frame))
            .map_err(SessionSeekError::Seek)?;
        self.pushed = true;
        Ok(())
    }

    /// Push the timeline's command at `frame`, unpublished; its rules are
    /// [`TimelineProducer::try_seek`]'s.
    pub fn push_timeline(
        &mut self,
        timeline: &mut TimelineProducer,
        frame: u64,
    ) -> Result<(), SessionSeekError> {
        self.own(&timeline.seek)?;
        timeline
            .seek
            .push(self.command(frame))
            .map_err(SessionSeekError::Seek)?;
        self.pushed = true;
        Ok(())
    }

    /// Publish the generation, if anything was pushed: render observes every pushed command at
    /// the first block entry whose load covers it. The drop does the same.
    ///
    /// A plain session seek is also named the session's seek (#2205): a source set lands it for
    /// every source and the timeline in one block, the first at which every source holding it
    /// has its runway, or at the deadline. An anchored one keeps its anchor's contract.
    pub fn publish(self) {}

    /// Make this plain session seek land at once (#2205): a seek with no live audio to keep (an
    /// engine that renders only when asked, or a stopped transport). The source set lands it at
    /// the first block that observes it, as before #2205, whatever its sources hold. No effect on
    /// an anchored seek.
    pub fn land_at_once(&mut self) {
        self.at_once = true;
    }
}

impl Drop for SessionSeek<'_> {
    fn drop(&mut self) {
        if self.pushed {
            if self.anchor_sample.is_none() {
                self.published
                    .name_session_seek(self.generation, self.at_once);
            }
            self.published.publish(self.generation);
        }
    }
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
        let generation = self.push_seek(command)?;
        self.seek.published.publish(generation);
        Ok(())
    }

    fn push_seek(&mut self, command: SourceCommand) -> Result<SourceGeneration, SourceSeekError> {
        let generation = self.seek.push(command)?;
        let (SourceCommand::Seek { frame, .. } | SourceCommand::SeekAt { frame, .. }) = command;
        self.next_write_frame = frame;
        self.end_of_region_submitted = false;
        // #2205: a new epoch with no block, before any publish of the generation.
        let ended = self.region_end.is_some_and(|end| frame.0 >= end.0);
        self.store_readiness(SeekReadiness::after_seek(
            self.readiness,
            self.seek.command_producer.success_count(),
            ended,
        ));
        Ok(generation)
    }

    fn store_readiness(&mut self, word: u64) {
        self.readiness = word;
        self.seek_readiness.store(word, Ordering::Release);
    }

    /// Whether [`Self::try_seek`] would accept `command` now, pushing nothing and changing
    /// nothing: every rule of [`Self::try_seek`], with a full command slot reported as the
    /// backpressure its push would return (#1640 D1). An all-or-nothing caller checks every
    /// producer before it pushes to any. Only this producer pushes to its slot and the consumer
    /// only empties it, so an accepted check stays accepted until this producer pushes.
    pub fn check_seek(&self, command: SourceCommand) -> Result<(), SourceSeekError> {
        self.seek.check_seek(command)
    }

    /// The ring's seek report as render last published it (#1316 D2): bounded retries, then
    /// [`SourceSeekReportBusy`]. Any thread; it never waits for render.
    pub fn seek_report(&self) -> Result<SourceSeekReport, SourceSeekReportBusy> {
        self.seek_report
            .read()
            .map(SourceSeekReport::from_words)
            .map_err(|_| SourceSeekReportBusy)
    }

    /// The ring's occupancy as this producer sees it now (issue #1387); see [`SourceAdmission`].
    #[must_use]
    pub fn admission(&self) -> SourceAdmission {
        let room = if self.deferred_block.is_some() {
            0
        } else {
            self.recycle_consumer
                .queued_len()
                .min(self.data_producer.available_capacity())
        };
        SourceAdmission {
            queued: self.data_producer.queued_len() as u64,
            room: room as u64,
        }
    }

    /// A handle on the ring's `prime_required` flag (#1320 D4). Control only; never on render.
    #[must_use]
    pub fn prime_flag(&self) -> PcmSourcePrimeFlag {
        PcmSourcePrimeFlag(Arc::clone(&self.prime_required))
    }

    /// Copy producer-local telemetry outside render.
    #[must_use]
    pub fn telemetry(&self) -> SourceProducerTelemetry {
        SourceProducerTelemetry {
            active_generation: self.seek.active_generation,
            cumulative_written_frames: self.cumulative_written_frames,
            data_full_count: self.data_producer.full_count(),
            recycle_empty_count: self.recycle_consumer.empty_count(),
            end_of_region_submitted: self.end_of_region_submitted,
            native_decoder_sanitized_samples: self.native_decoder_sanitized_samples,
        }
    }
    // REALTIME_POLICY_BEGIN
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
    // REALTIME_POLICY_END
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
        if chunk.generation != self.last_nonzero_generation {
            self.last_nonzero = None;
            self.last_nonzero_generation = chunk.generation;
        }
        let mut newest = None;
        for (channel, plane) in chunk.planes.iter().enumerate() {
            let offset = channel
                .checked_mul(quantum)
                .expect("prepared channel offset");
            let words = &plane[..frames];
            block.samples[offset..offset + frames].copy_from_slice(words);
            if let Some(frame) = words.iter().rposition(|word| word.to_bits() != 0) {
                newest = newest.max(Some(frame));
            }
        }
        if let Some(frame) = newest {
            let frame = SourceFrame(chunk.start_frame.0.saturating_add(frame as u64));
            self.last_nonzero = self.last_nonzero.max(Some(frame));
        }
        block.last_nonzero = self.last_nonzero;
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
                // #2205: after the block's push, so render that reads this count finds the
                // blocks queued.
                self.store_readiness(SeekReadiness::after_block(self.readiness, end_of_region));
                Ok(SubmitReport {
                    accepted_frames: frames,
                    cumulative_written_frames: self.cumulative_written_frames,
                    active_generation: self.seek.active_generation,
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

    /// Check a seek request without pushing it; see [`PcmSourceProducer::check_seek`].
    pub fn check_seek(&self, command: SourceCommand) -> Result<(), SourceSeekError> {
        self.producer.check_seek(command)
    }

    /// Copy producer telemetry outside render.
    #[must_use]
    pub fn telemetry(&self) -> SourceProducerTelemetry {
        self.producer.telemetry()
    }

    /// The ring's seek report; see [`PcmSourceProducer::seek_report`].
    pub fn seek_report(&self) -> Result<SourceSeekReport, SourceSeekReportBusy> {
        self.producer.seek_report()
    }

    /// The ring's occupancy; see [`PcmSourceProducer::admission`].
    #[must_use]
    pub fn admission(&self) -> SourceAdmission {
        self.producer.admission()
    }

    /// The ring's `prime_required` flag; see [`PcmSourceProducer::prime_flag`].
    #[must_use]
    pub fn prime_flag(&self) -> PcmSourcePrimeFlag {
        self.producer.prime_flag()
    }

    /// Name the region's end frame (#2205): a later seek to `end` or past it has no PCM to wait
    /// for, so a live plain seek there lands at the next block. Control thread; set once, when
    /// the host knows the region.
    pub fn set_region_end(&mut self, end: SourceFrame) {
        self.producer.region_end = Some(end);
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
    if generation != producer.seek.active_generation {
        return Err(HostChunkError::StaleGeneration {
            active: producer.seek.active_generation,
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
/// The played block is the consumer's from `begin_block_at` until the next `begin_block_at`,
/// `prepare_seek`, `end_block`, or drop. Until then [`Self::played_plane`] can borrow its planes
/// in place and [`Self::copy_channel`] can copy them, as often as the render needs.
///
/// Its storage stays with the consumer after that point too. The ring allocates one block beyond
/// the configured `transfer_block_count`, and at every block boundary the consumer retains exactly
/// one block outside both queues -- the played block while one is readable, otherwise the same
/// storage idle (after an underrun, the end of the region, `end_block` or `prepare_seek`) -- in
/// addition to the pre-fetched `current` block it already held before #917 at the boundaries
/// where `current.start_frame` is ahead of `next_frame`, or where `current` is a held anchored
/// seek's pending block (#1274), of another generation and possibly behind `next_frame`. The hold
/// is therefore "what it held before, plus one", which is what keeps the producer's admission
/// sequence unchanged. The idle storage goes back to the producer's recycle queue only inside
/// `begin_block_at`, at the moment a newer block becomes the played block. So:
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
    /// The generation in effect and the held anchored seek: the seek state every source-transport
    /// consumer shares (#1640 D1).
    seek: SeekState,
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
    held_frames: u64,
    held_blocks: u64,
    native_decoder_sanitized_samples: u64,
    generation_changed: bool,
    /// Whether any block has begun on this consumer (#1344 D5): render-owned, set at block entry
    /// and never cleared.
    begun_block: bool,
    /// The ring's seek report: this consumer is its only writer (#1316 D2). It belongs to the
    /// ring, so it moves with the consumer across a plan swap (D5).
    seek_report: PublishedWordsWriter<SEEK_REPORT_WORDS>,
    /// What the seek report holds; render is its only writer, so it never loads it back.
    published_seek_report: SourceSeekReport,
    /// The source-read sample of the block after the last one begun: the clock a
    /// [`SeekClock::Hold`] block (a prime) reports on, so its blocks are consecutive quanta.
    next_source_read_sample: u64,
    /// The ring's `prime_required` flag, which control sets (#1320 D4).
    prime_required: Arc<AtomicBool>,
    /// The ring's seek-readiness word (#2205), which the producer writes.
    seek_readiness: Arc<AtomicU64>,
    /// The source set's landing decision for this block's plain session seek (#2205), set at the
    /// start of each block's play; [`SessionLanding::NONE`] on a consumer of its own.
    session_landing: SessionLanding,
    /// The session's published generation this block observes commands under (#1642 B16-R2): the
    /// block entry's load of [`Self::published`], set by every entry point that observes a
    /// command, the source-set driver's one load for the whole set. Render-owned.
    observed_published: u64,
    /// The session's published generation (#1642 B16-R2), the word this ring's producer publishes
    /// to. A consumer on its own loads it at each block entry; the source-set driver loads the
    /// same word once for every consumer of the set.
    published: PublishedGeneration,
}

/// A seek the consumer has popped and holds: an anchored one until its anchor block, a live
/// plain one until its runway is queued (#2205).
#[derive(Clone, Copy, Debug)]
struct HeldSeek {
    generation: SourceGeneration,
    frame: SourceFrame,
    landing: Landing,
}

/// When a held seek lands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Landing {
    /// [`SourceCommand::SeekAt`]: at the block that reads from the anchor, on the plan's
    /// source-read clock, at `frame` plus any lateness.
    Anchor(u64),
    /// A plain [`SourceCommand::Seek`] observed at a block (#2205): at the first block at which
    /// its source has its runway, or at the deadline `R` blocks after `since`, the source-read
    /// sample of the block that observed it; at `frame`, from its first sample. A plain session
    /// seek lands where its source set decides, for every source and the timeline in one block.
    WhenReady { since: u64 },
}

/// A source set's decision for its plain session seek in one block (#2205).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SessionLanding {
    /// The plain session seek's generation, `0` for none.
    generation: u64,
    /// Whether it lands in this block, for every consumer that holds it.
    lands: bool,
}

impl SessionLanding {
    /// No session seek decided by a set: each held plain seek lands on its own runway.
    const NONE: Self = Self {
        generation: 0,
        lands: false,
    };
}

/// When an observed anchored seek may apply.
#[derive(Clone, Copy)]
enum SeekClock {
    /// In the block that reads source time from `source_read_sample`, if it is at or past the
    /// anchor.
    At {
        /// The plan's source-read clock: the absolute sample of source time this block reads;
        /// equal to the render clock in this version (#1316 D1).
        source_read_sample: u64,
    },
    /// Not now: preparation between blocks, which has no render time.
    Hold,
}

/// The seek state machine every source-transport consumer shares (#1640 D1): the
/// [`PcmSourceConsumer`] and the [`TimelineConsumer`] each own one, so no seek rule is written
/// twice. It owns the generation in effect and the held anchored seek; [`Self::observe`] takes one
/// popped command against a [`SeekClock`] and says what to do now. Render-owned, `Copy`, with no
/// allocation.
#[derive(Clone, Copy, Debug)]
struct SeekState {
    /// The generation in effect.
    active_generation: SourceGeneration,
    /// An observed [`SourceCommand::SeekAt`] whose anchor block has not begun yet (issue #1274).
    held_seek: Option<HeldSeek>,
}

/// What one [`SeekState::observe`] decided.
#[derive(Clone, Copy, Debug)]
struct SeekObservation {
    /// An anchored seek this observation started to hold, replacing any held before it.
    newly_held: Option<NewlyHeldSeek>,
    /// The seek to apply in this block, lateness included. The seek state has already made its
    /// generation the one in effect.
    apply: Option<AppliedSeek>,
}

/// An anchored seek [`SeekState::observe`] started to hold.
#[derive(Clone, Copy, Debug)]
struct NewlyHeldSeek {
    /// The generation in effect when the command was popped, before any seek of this observation
    /// applied.
    active: SourceGeneration,
    /// The held seek's generation.
    generation: SourceGeneration,
}

/// A seek to apply now: its generation and the frame the block starts at.
#[derive(Clone, Copy, Debug)]
struct AppliedSeek {
    generation: SourceGeneration,
    frame: SourceFrame,
}

impl SeekState {
    const fn new(initial_generation: SourceGeneration) -> Self {
        Self {
            active_generation: initial_generation,
            held_seek: None,
        }
    }
    // REALTIME_POLICY_BEGIN
    /// Take one popped command (or none) at a block boundary. Under [`SeekClock::Hold`] a plain
    /// seek clears any held seek and applies at once (a prepared seek, #1320); under
    /// [`SeekClock::At`] it is held to land when ready ([`Landing::WhenReady`], #2205). An
    /// anchored seek is held to its anchor. Either held seek replaces the one held before it.
    /// Allocation-free and wait-free.
    fn take(&mut self, command: Option<SourceCommand>, clock: SeekClock) -> SeekObservation {
        let (generation, frame, landing) = match (command, clock) {
            (Some(SourceCommand::Seek { generation, frame }), SeekClock::Hold) => {
                self.held_seek = None;
                self.active_generation = generation;
                return SeekObservation {
                    newly_held: None,
                    apply: Some(AppliedSeek { generation, frame }),
                };
            }
            (
                Some(SourceCommand::Seek { generation, frame }),
                SeekClock::At { source_read_sample },
            ) => (
                generation,
                frame,
                Landing::WhenReady {
                    since: source_read_sample,
                },
            ),
            (
                Some(SourceCommand::SeekAt {
                    generation,
                    frame,
                    anchor_sample,
                }),
                _,
            ) => (generation, frame, Landing::Anchor(anchor_sample)),
            (None, _) => {
                return SeekObservation {
                    newly_held: None,
                    apply: None,
                };
            }
        };
        let newly_held = Some(NewlyHeldSeek {
            active: self.active_generation,
            generation,
        });
        self.held_seek = Some(HeldSeek {
            generation,
            frame,
            landing,
        });
        SeekObservation {
            newly_held,
            apply: None,
        }
    }

    /// Land the held seek if its block has come. An anchored seek lands in the first block whose
    /// [`SeekClock::At`] sample is at or past its anchor, at `frame + lateness` (saturating), and
    /// never on [`SeekClock::Hold`]. Admission keeps every anchor on the `quantum_frames` grid and
    /// render clocks advance by whole quanta, so the lateness is a whole number of quanta; a debug
    /// build asserts it (#1350 D3). A held plain seek lands at `frame` when `ready` (the caller's
    /// runway, deadline or set decision, #2205), and always on [`SeekClock::Hold`]. Allocation-free
    /// and wait-free.
    fn land(&mut self, clock: SeekClock, quantum_frames: u32, ready: bool) -> Option<AppliedSeek> {
        let held = self.held_seek?;
        let frame = match (held.landing, clock) {
            (Landing::Anchor(anchor_sample), SeekClock::At { source_read_sample })
                if source_read_sample >= anchor_sample =>
            {
                let late_by = source_read_sample - anchor_sample;
                debug_assert!(
                    late_by.is_multiple_of(u64::from(quantum_frames)),
                    "a late anchored seek must be late by whole quanta, or no producer block \
                     matches next_frame and the source starves"
                );
                SourceFrame(held.frame.0.saturating_add(late_by))
            }
            (Landing::Anchor(_), _) => return None,
            (Landing::WhenReady { .. }, SeekClock::Hold) => held.frame,
            (Landing::WhenReady { .. }, SeekClock::At { .. }) if ready => held.frame,
            (Landing::WhenReady { .. }, SeekClock::At { .. }) => return None,
        };
        self.held_seek = None;
        self.active_generation = held.generation;
        Some(AppliedSeek {
            generation: held.generation,
            frame,
        })
    }

    /// The held plain seek's generation and the sample its deadline passes at, `R` blocks of
    /// `quantum_frames` after the block that observed it (#2205); `None` for no held seek or an
    /// anchored one.
    fn when_ready(
        &self,
        runway_blocks: u32,
        quantum_frames: u32,
    ) -> Option<(SourceGeneration, u64)> {
        let held = self.held_seek?;
        let Landing::WhenReady { since } = held.landing else {
            return None;
        };
        let deadline = since
            .saturating_add(u64::from(runway_blocks).saturating_mul(u64::from(quantum_frames)));
        Some((held.generation, deadline))
    }

    /// Whether a held seek may land in a block that starts before `sample`: an anchored one
    /// anchored before it, a late one included, or a live plain one, which may land at any
    /// block (#2205).
    fn holds_anchor_before(&self, sample: u64) -> bool {
        self.held_seek.is_some_and(|held| match held.landing {
            Landing::Anchor(anchor_sample) => anchor_sample < sample,
            Landing::WhenReady { .. } => true,
        })
    }

    /// The seek half of a prime's readiness, which every source-transport consumer shares
    /// (#1320 D3, #1641 D5): no command is queued (`queued` is the command queue's count at entry)
    /// and no held anchored seek is anchored before the end of the window of `lead_blocks + 1`
    /// blocks of `quantum_frames` from `first_sample`. An overflowing window is not clear.
    fn prime_window_clear(
        &self,
        queued: usize,
        quantum_frames: u32,
        first_sample: u64,
        lead_blocks: u32,
    ) -> bool {
        if queued != 0 {
            return false;
        }
        let Some(window_end) = u64::from(lead_blocks)
            .checked_add(1)
            .and_then(|blocks| blocks.checked_mul(u64::from(quantum_frames)))
            .and_then(|window| first_sample.checked_add(window))
        else {
            return false;
        };
        // A held anchor is always past the last block's sample, so this is every held seek that
        // would apply at one of the blocks, a late one at `first_sample` included.
        !self.holds_anchor_before(window_end)
    }

    /// Whether the held anchored seek, if any, is of `generation`.
    fn holds_generation(&self, generation: SourceGeneration) -> bool {
        self.held_seek
            .is_some_and(|held| held.generation == generation)
    }

    /// Whether `generation` is newer than every seek this state has observed.
    fn is_unobserved(&self, generation: SourceGeneration) -> bool {
        generation > self.active_generation
            && self
                .held_seek
                .is_none_or(|held| generation > held.generation)
    }

    /// The pop of the observe step every source-transport consumer shares (#1642 B16-R2): the
    /// command at the head of `queue`, popped, when its generation is at most `published`, the
    /// block entry's load of the session's published generation; otherwise `None`, and the
    /// command stays in its slot for the first block whose load covers it. It peeks first and
    /// pops only what it peeked (only this consumer pops), so a command pushed after the peek is
    /// never taken here. Nothing is dropped. Wait-free: at most two loads and one pop.
    fn take_published(
        queue: &mut Consumer<SourceCommand>,
        published: u64,
    ) -> Option<SourceCommand> {
        let generation = match queue.peek(0) {
            Some(
                &(SourceCommand::Seek { generation, .. }
                | SourceCommand::SeekAt { generation, .. }),
            ) => generation,
            None => return None,
        };
        if generation.0 > published {
            return None;
        }
        queue.try_pop().ok()
    }
    // REALTIME_POLICY_END
}

/// The seek producer every source-transport producer shares (#1640 D1): the
/// [`PcmSourceProducer`] and the [`TimelineProducer`] each own one, so no seek admission rule is
/// written twice. It owns the one-slot command producer and the generation in effect.
struct SeekProducer {
    command_producer: Producer<SourceCommand>,
    /// The newest generation this producer accepted.
    active_generation: SourceGeneration,
    /// The render quantum an anchor must be a multiple of.
    quantum_frames: u32,
    /// The largest frame a seek may name, if any: [`TIMELINE_SAMPLE_LIMIT`] for a timeline,
    /// `None` for a PCM source.
    frame_limit: Option<u64>,
    /// The session's published generation (#1642 B16-R2): a seek on its own publishes to it
    /// after its push, a session seek ([`SessionSeek`]) after its last push.
    published: PublishedGeneration,
}

impl SeekProducer {
    /// Every admission rule but the slot: a nonzero, strictly newer generation, an anchor that is
    /// a quantum multiple and a frame within the limit. Returns the command's generation.
    fn check_rules(&self, command: SourceCommand) -> Result<SourceGeneration, SourceSeekError> {
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
        if self.frame_limit.is_some_and(|limit| frame.0 > limit) {
            return Err(SourceSeekError::FrameOutOfRange);
        }
        Ok(generation)
    }

    /// Every rule of [`Self::try_seek`] with nothing pushed or changed. A full slot is the
    /// [`SourceSeekError::Backpressure`] the push would return, carrying the current full count
    /// (a check counts no refusal).
    fn check_seek(&self, command: SourceCommand) -> Result<(), SourceSeekError> {
        self.check_rules(command)?;
        if self.command_producer.available_capacity() == 0 {
            return Err(SourceSeekError::Backpressure {
                full_count: self.command_producer.full_count(),
            });
        }
        Ok(())
    }

    /// The rules of [`Self::check_seek`], then the push, with nothing published; the generation
    /// switches only when the push lands. A full slot changes nothing but the queue's full count.
    /// Returns the pushed command's generation, which the caller publishes after its last push.
    fn push(&mut self, command: SourceCommand) -> Result<SourceGeneration, SourceSeekError> {
        let generation = self.check_rules(command)?;
        match self.command_producer.try_push(command) {
            Ok(()) => {
                self.active_generation = generation;
                Ok(generation)
            }
            Err(QueueFull { full_count, .. }) => Err(SourceSeekError::Backpressure { full_count }),
        }
    }

    /// [`Self::push`], then publish the pushed generation: one seek on its own.
    fn try_seek(&mut self, command: SourceCommand) -> Result<(), SourceSeekError> {
        let generation = self.push(command)?;
        self.published.publish(generation);
        Ok(())
    }
}

/// Whether an acquire may observe the admitted command when it meets a block of an unobserved
/// generation (#1320 D3).
#[derive(Clone, Copy, Eq, PartialEq)]
enum CommandObservation {
    /// Every ordinary block and `prepare_seek`: pop the block, then observe its seek.
    Allowed,
    /// A prime: stop before such a block and leave it and its command queued.
    Withheld,
}

impl PcmSourceConsumer {
    /// The heap bytes this consumer retains (#1074), exclusive of itself: its three rings in
    /// full (the host's endpoints drop first), every transfer block it or its two block rings
    /// hold now (box and PCM), its seek-report record and its prime flag. Not the session's
    /// published generation, which the session owns. A block the host's producer
    /// holds is the host's. Control thread only.
    ///
    /// It counts the transfer blocks queued in the two block rings at the moment it reads their
    /// cursors, so only a quiescent walk is meaningful: no block in flight between the host's
    /// producer and this consumer (no submit, pop or recycle racing it). A block that moves while
    /// the walk reads can be counted on both sides or on neither.
    pub(crate) fn retained_bytes(&self) -> u64 {
        use engine::realtime::retained::arc;
        let samples = u64::from(self.channel_count).saturating_mul(u64::from(self.quantum_frames));
        let block = (size_of::<TransferBlock>() as u64)
            .saturating_add(samples.saturating_mul(size_of::<f32>() as u64));
        let held = [
            &self.current,
            &self.played,
            &self.retained_idle,
            &self.deferred_recycle,
        ]
        .into_iter()
        .filter(|slot| slot.is_some())
        .count();
        let blocks = held
            .saturating_add(self.data_consumer.queued_len())
            .saturating_add(self.recycle_producer.queued_len());
        self.data_consumer
            .retained_bytes()
            .saturating_add(self.recycle_producer.retained_bytes())
            .saturating_add(self.command_consumer.retained_bytes())
            .saturating_add((blocks as u64).saturating_mul(block))
            .saturating_add(self.seek_report.retained_bytes())
            .saturating_add(arc(&self.prime_required))
            .saturating_add(arc(&self.seek_readiness))
    }
    /// Apply an already-admitted seek on the exclusive consumer owner between blocks.
    /// Recycles stale storage and retains current-generation PCM without consuming a frame.
    /// Producers still only enqueue commands; this is not a shared controller handle.
    ///
    /// An anchored seek ([`SourceCommand::SeekAt`]) has no block time here: it is held, never
    /// applied or dropped, until the render reaches its anchor, and this returns `false`. That
    /// holds for a seek a producer pushes while this runs, too: if the acquire's re-observe
    /// (#1274 MINOR-3) takes a newer seek, plain or anchored, this returns `false`.
    pub fn prepare_seek(&mut self, generation: SourceGeneration, frame: SourceFrame) -> bool {
        let published = self.published.load_at_block_entry();
        self.prepare_seek_published(generation, frame, published)
    }

    /// [`Self::prepare_seek`] under the session's published generation `published` (#1642
    /// B16-R2): the source-set driver's form, which observes no command above it.
    fn prepare_seek_published(
        &mut self,
        generation: SourceGeneration,
        frame: SourceFrame,
        published: u64,
    ) -> bool {
        self.observed_published = published;
        self.end_block();
        self.flush_deferred_recycle();
        // The prepared source command queue has one slot. Observe exactly that admitted
        // command, then check the requested identity before granting readiness.
        self.observe_seek_at_block_boundary(SeekClock::Hold);
        // The held-seek test is not redundant: a caller may prepare the playing generation again
        // while a newer anchored seek is held, and that is not ready.
        if self.seek.held_seek.is_some()
            || self.seek.active_generation != generation
            || self.next_frame != frame
        {
            return false;
        }
        self.acquire_current_block(SeekClock::Hold, CommandObservation::Allowed);
        // A producer running concurrently may push a newer seek and its block between the command
        // pop above and the data pop in the acquire; the acquire's re-observe takes that seek.
        // Readiness is granted for `generation` alone, so check again.
        self.seek.held_seek.is_none() && self.seek.active_generation == generation
    }

    /// Apply the admitted plain seek, if any, on the exclusive consumer owner between blocks
    /// (#2213), under the session's published generation `published`: the form of
    /// [`Self::prepare_seek_published`] for an owner that does not know the seek's identity, a C
    /// ABI host whose render is stopped. The seek is observed under [`SeekClock::Hold`]: a plain
    /// seek in the command slot, or one held for its runway, applies now, and every queued block
    /// of an older generation is discarded and recycled, so the producer has room for the new
    /// generation before render runs again. An anchored seek has no block time here: it stays
    /// held and nothing is discarded for it.
    ///
    /// Returns the applied seek's generation and the frame the consumer reads from next, or
    /// `None` when no seek applied. Allocation-free and wait-free; the acquire is bounded by the
    /// ring's capacity.
    fn prepare_admitted_seek(&mut self, published: u64) -> Option<(SourceGeneration, SourceFrame)> {
        self.observed_published = published;
        self.end_block();
        self.flush_deferred_recycle();
        // Generations strictly increase, so a seek applied here always changes the active one.
        let before = self.seek.active_generation;
        self.observe_seek_at_block_boundary(SeekClock::Hold);
        if self.seek.active_generation == before {
            return None;
        }
        self.acquire_current_block(SeekClock::Hold, CommandObservation::Allowed);
        Some((self.seek.active_generation, self.next_frame))
    }

    /// Immutable prepared ring shape shared with the producer endpoint.
    #[must_use]
    pub const fn shape(&self) -> PcmSourceShape {
        self.shape
    }

    /// Advance the source state machine once and retain this quantum for channel fan-out reads,
    /// for the block that reads source time from `source_read_sample`: the plan's source-read
    /// clock, the absolute sample of source time this block reads (#1316 D1):
    /// the render clock plus the plan's source-read offset (#1396 D1). The offset is 0 unless a
    /// plan is prepared to read ahead, so in this version the two clocks are equal. The previously
    /// played block stops being readable here. A short (end-of-region) block has its tail zeroed in
    /// place, once, so every [`Self::played_plane`] is a whole quantum.
    ///
    /// An anchored seek ([`SourceCommand::SeekAt`]) is held while `source_read_sample` is before
    /// its anchor; the block that starts at or past it applies the seek at `frame +
    /// (source_read_sample - anchor_sample)` (saturating). While it is held, queued PCM of the
    /// playing generation still plays, the new generation's first block is kept as the pending
    /// block, and with nothing playable the block is **held**, not an underrun (the host asked the
    /// source to wait; underrun means the host owed PCM and did not deliver it). A newer command
    /// replaces a held one. This render-path operation allocates nothing and never blocks.
    ///
    /// The block publishes the ring's seek report when a seek applies at it, at the ring's first
    /// block and when a cumulative counter moves ([`SourceSeekReport`], #1316 D3).
    pub fn begin_block_at(&mut self, source_read_sample: u64) -> SourceReadReport {
        let published = self.published.load_at_block_entry();
        self.begin_block_at_published(source_read_sample, published)
    }

    /// [`Self::begin_block_at`] under the session's published generation `published` (#1642
    /// B16-R2): the source-set driver's form. A command whose generation is above `published`
    /// stays in its slot, and a block of its generation stays queued behind it, for a later block.
    fn begin_block_at_published(
        &mut self,
        source_read_sample: u64,
        published: u64,
    ) -> SourceReadReport {
        self.observed_published = published;
        self.begin_block_with(SeekClock::At { source_read_sample })
    }

    fn begin_block_with(&mut self, clock: SeekClock) -> SourceReadReport {
        self.begun_block = true;
        self.begin_block_observe(clock);
        self.begin_block_play(clock, CommandObservation::Allowed, SessionLanding::NONE)
    }

    /// The first half of [`Self::begin_block_with`]: release the played block and take the
    /// admitted command. The two halves are separate so a source set can decide its session
    /// seek between them (#2205), and so a test can interleave a producer between the command pop
    /// and the data pop, the window `acquire_current_block` must close.
    fn begin_block_observe(&mut self, clock: SeekClock) {
        self.end_block();
        self.flush_deferred_recycle();
        self.generation_changed = false;
        self.take_seek_command(clock);
    }

    /// The second half of [`Self::begin_block_with`]: land a held seek whose block has come, under
    /// the set's `session` decision (#2205), then acquire and play this block. A prime
    /// ([`CommandObservation::Withheld`]) lands nothing.
    fn begin_block_play(
        &mut self,
        clock: SeekClock,
        observation: CommandObservation,
        session: SessionLanding,
    ) -> SourceReadReport {
        if observation == CommandObservation::Allowed {
            self.session_landing = session;
            self.land_held_seek(clock);
        }
        self.acquire_current_block(clock, observation);
        // Every seek this block can apply has applied by now (the observe step or the acquire's
        // re-observe), and nothing below has advanced: this is the frame the block reads from.
        let source_frame = self.next_frame;
        let source_read_sample = match clock {
            SeekClock::At { source_read_sample } => source_read_sample,
            SeekClock::Hold => self.next_source_read_sample,
        };
        self.next_source_read_sample =
            source_read_sample.saturating_add(u64::from(self.quantum_frames));
        let mut copied_frames = 0_u32;
        let mut underrun_frames = 0_u32;
        let mut held_frames = 0_u32;
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
            let missing_frames = u32::try_from(cmp::min(
                u64::from(self.quantum_frames),
                available_until_end,
            ))
            .expect("bounded by quantum");
            self.next_frame = SourceFrame(
                self.next_frame
                    .0
                    .saturating_add(u64::from(self.quantum_frames)),
            );
            // After the acquire step, which can set or apply a held seek (`acquire_current_block`):
            // missing PCM while an anchored seek is still held is a planned wait, not starvation.
            if self.seek.held_seek.is_some() {
                held_frames = missing_frames;
                if held_frames != 0 {
                    self.held_frames = self.held_frames.saturating_add(u64::from(held_frames));
                    self.held_blocks = self.held_blocks.saturating_add(1);
                }
            } else {
                underrun_frames = missing_frames;
                if underrun_frames != 0 {
                    self.underrun_frames = self
                        .underrun_frames
                        .saturating_add(u64::from(underrun_frames));
                    self.underrun_events = self.underrun_events.saturating_add(1);
                }
            }
            if self.end_reached() {
                self.end_of_region = true;
            }
        }
        self.publish_seek_report(source_read_sample, source_frame);
        SourceReadReport {
            copied_frames,
            underrun_frames,
            underrun_event: underrun_frames != 0,
            held_frames,
            held: held_frames != 0,
            end_of_region: self.end_of_region,
            active_generation: self.seek.active_generation,
            generation_changed: self.generation_changed,
            cumulative_read_frames: self.cumulative_read_frames,
            cumulative_underrun_frames: self.underrun_frames,
            cumulative_underrun_events: self.underrun_events,
            cumulative_held_frames: self.held_frames,
            cumulative_held_blocks: self.held_blocks,
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

    /// Whether every word [`Self::played_plane`] lends and [`Self::copy_channel`] writes this
    /// quantum is exact `+0.0` on every channel (issue #2133): no block was played (an underrun,
    /// a held seek, the end of the region: the consumer emits the `+0.0` itself), or the played
    /// block's last-non-zero stamp is below its first frame (a short block's tail is zeroed by
    /// `play`). One compare: it reads no sample. `false` never drops audio; it only costs time.
    /// This render-path operation allocates nothing and never blocks.
    #[must_use]
    pub fn played_block_silent(&self) -> bool {
        self.played
            .as_ref()
            .is_none_or(|block| block.is_positive_zero())
    }
    // REALTIME_POLICY_END

    /// Whether a warm successor carries this ring and so needs it primed (#1320 D4). Control sets
    /// it through [`PcmSourcePrimeFlag`] before it publishes that successor; render reads it
    /// after its `Acquire` of the plan mailbox, so the `Relaxed` load is ordered by that.
    #[must_use]
    pub fn prime_required(&self) -> bool {
        self.prime_required.load(Ordering::Relaxed)
    }

    /// Exact source channel count.
    #[must_use]
    pub const fn channel_count(&self) -> u32 {
        self.channel_count
    }

    /// Whether a block has ever begun on this consumer (#1344 D5). Until the first block it has
    /// popped no PCM and played nothing, so every acked chunk, the generation and any seek are
    /// still in its ring or held by it: the consumer can move to another plan whole with nothing
    /// lost. A seek prepared between blocks ([`Self::prepare_seek`]) is not a block.
    #[must_use]
    pub const fn has_begun_block(&self) -> bool {
        self.begun_block
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
            active_generation: self.seek.active_generation,
            cumulative_read_frames: self.cumulative_read_frames,
            stale_generation_discard_count: self.stale_generation_discard_count,
            underrun_frames: self.underrun_frames,
            underrun_events: self.underrun_events,
            held_frames: self.held_frames,
            held_blocks: self.held_blocks,
            end_of_region: self.end_of_region,
            native_decoder_sanitized_samples: self.native_decoder_sanitized_samples,
        }
    }

    /// The accepted frames render has not read (#1699 D1): the frames of the popped-but-unplayed
    /// pending block plus those of every block still queued, a stale-generation block included
    /// (R2). The played block is read already and does not count. Pops nothing. Control thread,
    /// once the consumer's plan is retired: the count it is summed into is
    /// `SOURCE_FRAMES_DISCARDED`.
    #[must_use]
    pub fn unread_frames(&self) -> u64 {
        let pending = self
            .current
            .as_ref()
            .map_or(0, |block| u64::from(block.frames));
        (0..self.data_consumer.available_at_entry())
            .map_while(|index| self.data_consumer.peek(index))
            .fold(pending, |total, block| {
                total.saturating_add(u64::from(block.frames))
            })
    }

    // REALTIME_POLICY_BEGIN
    /// Publish the ring's seek report if this block changed it (#1316 D3): a new generation in
    /// effect (the ring's first block, or one a seek applied at) takes this block's
    /// `source_read_sample` and `source_frame`; a moved cumulative counter rewrites the counters
    /// alone. Otherwise nothing is written. Wait-free, allocation-free.
    fn publish_seek_report(&mut self, source_read_sample: u64, source_frame: SourceFrame) {
        let mut report = self.published_seek_report;
        if report.generation != self.seek.active_generation.0 {
            report.generation = self.seek.active_generation.0;
            report.first_sample = source_read_sample;
            report.source_frame = source_frame.0;
        }
        report.cumulative_underrun_frames = self.underrun_frames;
        report.cumulative_held_frames = self.held_frames;
        if report != self.published_seek_report {
            self.published_seek_report = report;
            self.seek_report.publish(&report.to_words());
        }
    }

    /// Pop the admitted command, if any, then apply a held anchored seek whose block has come. A
    /// command above the block's published generation stays in its slot ([`SeekState::take_published`]).
    ///
    /// The seek rules are the shared [`SeekState::observe`]; the source's own work runs on its
    /// result: a newly held anchored seek makes the pending block of a replaced generation stale,
    /// and a seek to apply repositions the PCM.
    fn observe_seek_at_block_boundary(&mut self, clock: SeekClock) {
        self.take_seek_command(clock);
        self.land_held_seek(clock);
    }

    /// The take half of [`Self::observe_seek_at_block_boundary`]: pop the admitted command and
    /// hold it, or, a plain seek under [`SeekClock::Hold`], apply it.
    fn take_seek_command(&mut self, clock: SeekClock) {
        let command =
            SeekState::take_published(&mut self.command_consumer, self.observed_published);
        let observation = self.seek.take(command, clock);
        if let Some(held) = observation.newly_held {
            // A newer command replaces a held one; the replaced seek's pending block is stale.
            if let Some(block) = self.current.take_if(|block| {
                block.generation != held.active && block.generation != held.generation
            }) {
                self.discard_block(block);
            }
        }
        if let Some(seek) = observation.apply {
            self.apply_seek(seek.generation, seek.frame);
        }
    }

    /// The land half of [`Self::observe_seek_at_block_boundary`]: apply the held seek if its
    /// block has come ([`SeekState::land`]).
    fn land_held_seek(&mut self, clock: SeekClock) {
        let ready = self.held_plain_seek_lands(clock);
        if let Some(seek) = self.seek.land(clock, self.quantum_frames, ready) {
            self.apply_seek(seek.generation, seek.frame);
        }
    }

    /// Whether the held live plain seek lands in the block of `clock` (#2205): the set's decision
    /// for its plain session seek; otherwise its runway is queued or its deadline has come.
    fn held_plain_seek_lands(&self, clock: SeekClock) -> bool {
        let SeekClock::At { source_read_sample } = clock else {
            return true;
        };
        let Some((generation, deadline)) = self
            .seek
            .when_ready(self.shape.seek_runway_blocks, self.quantum_frames)
        else {
            return false;
        };
        if generation.0 == self.session_landing.generation {
            return self.session_landing.lands;
        }
        source_read_sample >= deadline || self.runway_queued()
    }

    /// Whether the newest seek this consumer popped has its runway queued (#2205): `R` blocks of
    /// its generation published, or its region's end, or a ring never fed before the seek. One
    /// `Acquire` load, ordered after the producer's block pushes it counts.
    fn runway_queued(&self) -> bool {
        let runway = self.shape.seek_runway_blocks;
        runway == 0
            || SeekReadiness::ready(
                self.seek_readiness.load(Ordering::Acquire),
                self.command_consumer.success_count(),
                runway,
            )
    }

    /// This consumer's part in a set's decision for the plain session seek `generation` at
    /// `source_read_sample` (#2205): `None` when it does not hold it; otherwise whether its runway
    /// is queued and whether its deadline has come.
    fn session_seek_hold(&self, generation: u64, source_read_sample: u64) -> Option<(bool, bool)> {
        let (held, deadline) = self
            .seek
            .when_ready(self.shape.seek_runway_blocks, self.quantum_frames)?;
        (held.0 == generation).then(|| (self.runway_queued(), source_read_sample >= deadline))
    }
    // REALTIME_POLICY_END
    /// Make `generation` audible from `frame`; the seek state has already made it the generation
    /// in effect. A pending `current` block of that generation that does not start behind `frame`
    /// is kept -- an anchored seek's primed block -- and any other is discarded, noting the region
    /// end it carries.
    fn apply_seek(&mut self, generation: SourceGeneration, frame: SourceFrame) {
        self.generation_changed = true;
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
    // REALTIME_POLICY_BEGIN
    /// Whether this consumer would play each of its next `lead_blocks + 1` blocks from queued PCM,
    /// with no underrun, no discard and no command (#1320 D2): the readiness check render makes
    /// in the block before it adopts a warm successor whose sources read `lead_blocks` quanta
    /// further ahead.
    ///
    /// `first_sample` is the source-read sample of the block render is about to run on the active
    /// plan. `true` exactly when the command queue is empty, no held anchored seek is anchored
    /// before `first_sample + (lead_blocks + 1) * quantum`, and for each of those blocks either
    /// the region's end is reached or the next block in queue order (the pending block first, then
    /// the data queue) is of the active generation and starts exactly where the consumer reads.
    /// A short block must carry `end_of_region`.
    ///
    /// `false` (a gap, another generation, a block the consumer would discard, a held seek's
    /// pending block, too few blocks queued) only delays adoption: each clears as render keeps
    /// playing. Read-only: it changes no state, and costs one command-queue load and at most
    /// `lead_blocks + 1` data-queue loads, all `Acquire`, with no allocation, lock or syscall.
    /// Only render changes the consumer and the producer can only add blocks or a command, so the
    /// answer holds for the rest of the render callback for [`Self::prime_block_at`], which
    /// observes no command.
    #[must_use]
    pub fn prime_ready(&self, first_sample: u64, lead_blocks: u32) -> bool {
        if !self.seek.prime_window_clear(
            self.command_consumer.available_at_entry(),
            self.quantum_frames,
            first_sample,
            lead_blocks,
        ) {
            return false;
        }
        let mut frame = self.next_frame.0;
        let mut end = self.end_frame.map(|end| end.0);
        let mut pending = self.current.as_deref();
        let mut queued = 0_usize;
        for _ in 0..=lead_blocks {
            if end.is_some_and(|end| frame >= end) {
                // At the region's end a block reads nothing and moves no frame: so are the rest.
                return true;
            }
            let block = if let Some(block) = pending.take() {
                block
            } else {
                let Some(block) = self.data_consumer.peek(queued) else {
                    return false;
                };
                queued += 1;
                &**block
            };
            if block.generation != self.seek.active_generation
                || block.start_frame.0 != frame
                || (block.frames < self.quantum_frames && !block.end_of_region)
            {
                return false;
            }
            frame = frame.saturating_add(u64::from(block.frames));
            if block.end_of_region {
                end = Some(frame);
            }
        }
        true
    }

    /// Replay the next block exactly as [`Self::begin_block_at`]`(source_read_sample)` would play
    /// it, without observing any command (#1320 D3): a warm successor's prime, in its adoption
    /// block.
    ///
    /// It releases the played block and plays, discards and recycles as an ordinary block does,
    /// but neither pops the command queue nor applies a held seek, and its acquire stops before a
    /// block of a generation it has not observed, so the command queue and any held seek are
    /// unchanged. The played planes are lent by [`Self::played_plane`] until the next `&mut self`
    /// call, as for any block.
    ///
    /// Precondition: [`Self::prime_ready`]`(first_sample, k)` was `true` in this render callback,
    /// and the calls are `prime_block_at(first_sample + j * quantum)` for `j = 0..k`, in order.
    /// Then no call underruns: each block matches the frame the consumer reads, or the region has
    /// ended. Allocation-free and wait-free.
    pub fn prime_block_at(&mut self, source_read_sample: u64) -> SourceReadReport {
        let commands_popped = self.command_consumer.success_count();
        self.begun_block = true;
        self.end_block();
        self.flush_deferred_recycle();
        self.generation_changed = false;
        // `Hold` reports on the consumer's own clock; this block's sample is the caller's.
        self.next_source_read_sample = source_read_sample;
        let report = self.begin_block_play(
            SeekClock::Hold,
            CommandObservation::Withheld,
            SessionLanding::NONE,
        );
        debug_assert!(
            report.underrun_frames == 0 && report.held_frames == 0,
            "a prime after a true readiness neither underruns nor holds"
        );
        debug_assert_eq!(
            self.command_consumer.success_count(),
            commands_popped,
            "a prime observes no command"
        );
        report
    }

    /// Pop toward the block to play. `clock` is the block's, for a seek observed here.
    ///
    /// The call pops at most the blocks queued when it begins (`available`), never the blocks the
    /// host publishes while it runs: a count taken at entry, not the ring's capacity, bounds it, so
    /// a host that keeps the ring fed cannot hold the render here. Each block is peeked before it
    /// is popped, with at most one seek re-observe in the call.
    ///
    /// A block of a generation this consumer has not observed is never popped: with
    /// [`CommandObservation::Withheld`] (a prime) the call stops before it, so it never observes a
    /// command; otherwise it re-observes the command queue once, and if the block's seek is still
    /// unobserved (its command is above the block's published generation, #1642 B16-R2) it stops
    /// before it too. Either way the block stays queued with its command, and plays at the block
    /// that observes the seek: no acked PCM is discarded for a seek render has not observed yet.
    fn acquire_current_block(&mut self, clock: SeekClock, observation: CommandObservation) {
        if self.current.is_some() {
            return;
        }
        let available = self
            .data_consumer
            .available_at_entry()
            .min(self.transfer_block_count);
        let mut reobserved = false;
        for _ in 0..available {
            let Some(generation) = self.data_consumer.peek(0).map(|block| block.generation) else {
                break;
            };
            if self.is_unobserved_generation(generation) {
                if observation == CommandObservation::Withheld || reobserved {
                    return;
                }
                // This block's seek was pushed after this block's command pop. The host pushes a
                // seek's command before any of its PCM, on one thread, through release/acquire
                // queues, so the command is in the one-slot command queue now: observe it before
                // judging the block rather than discard accepted PCM (#1274 MINOR-3). A counted
                // block was published after its seek's command was pushed, so that command was
                // pushed before the count too. Only `observe_seek_at_block_boundary` pops the
                // one-slot queue (its callers are this acquire, `prepare_seek` and
                // `begin_block_observe`, all on this consumer's thread), so at the count at most
                // one command is pushed and not yet popped, and at most one generation among the
                // counted blocks is unobserved. This one re-observe takes it, unless the session
                // has not published its generation at this block's entry: then the block waits.
                reobserved = true;
                self.observe_seek_at_block_boundary(clock);
                if self.is_unobserved_generation(generation) {
                    return;
                }
            }
            let Ok(block) = self.data_consumer.try_pop() else {
                break;
            };
            self.note_sanitized_samples(&block);
            if self.settle_block(block) {
                return;
            }
        }
    }

    /// Keep `block` as the pending block (`true`) when it is the active generation at or after
    /// `next_frame`, or a held seek's first block; otherwise discard it (`false`). Pops nothing.
    fn settle_block(&mut self, block: Box<TransferBlock>) -> bool {
        if block.generation == self.seek.active_generation
            && block.start_frame.0 >= self.next_frame.0
        {
            if block.end_of_region {
                self.note_end_frame(&block);
            }
            self.current = Some(block);
            return true;
        }
        if self.seek.holds_generation(block.generation) {
            // A held anchored seek's first block: kept unexamined as the pending block until
            // the seek applies (its end, if it carries one, is noted then), and the blocks
            // behind it stay queued.
            self.current = Some(block);
            return true;
        }
        self.note_end_and_discard(block);
        false
    }
    // REALTIME_POLICY_END
    /// Carry the decoder's sanitized-sample count of `block` into the consumer's running maximum.
    fn note_sanitized_samples(&mut self, block: &TransferBlock) {
        self.native_decoder_sanitized_samples = self
            .native_decoder_sanitized_samples
            .max(block.native_decoder_sanitized_samples);
    }

    /// Whether `generation` is newer than every seek this consumer has observed.
    fn is_unobserved_generation(&self, generation: SourceGeneration) -> bool {
        self.seek.is_unobserved(generation)
    }

    fn current_matches_next_frame(&self) -> bool {
        self.current.as_ref().is_some_and(|block| {
            block.generation == self.seek.active_generation && block.start_frame == self.next_frame
        })
    }

    fn note_end_and_discard(&mut self, block: Box<TransferBlock>) {
        if block.end_of_region && block.generation == self.seek.active_generation {
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

/// The largest timeline sample a seek may name (#1640 D4): `2^53`, about 2,970 years at 96 kHz.
/// It leaves `2^63 - 2^53` samples of headroom, so every timeline value and every node time
/// derived from it stays an exact `i64` for any run a host can render.
pub const TIMELINE_SAMPLE_LIMIT: u64 = 1 << 53;

/// Exact fixed bytes of one prepared timeline (#1640 D5), row by row: the allocations it owns.
/// #1641 adds them to the source set's overhead. The [`TimelineConsumer`] value itself has no row:
/// it is inline in its owner (the source set's driver), whose own allocation counts it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimelineResourceReport {
    /// The history's entries, `K`.
    pub history_blocks: u64,
    /// One-slot seek-command SPSC header plus slot: a ring's `command_queue_bytes` formula.
    pub command_queue_bytes: u64,
    /// The largest single allocation of the command queue (#1641 D7: a carried queue's row).
    pub command_queue_largest_allocation_bytes: u64,
    /// The history allocation: `16 * K` bytes.
    pub history_bytes: u64,
    /// The timeline's seek-report record (#1704 D4): one shared allocation, a ring's
    /// `seek_report_bytes` formula, so it is also its own largest-allocation row. A vacant
    /// timeline's record arrives with its carry, as its command queue does.
    pub seek_report_bytes: u64,
}

/// Prepared timeline constructor: the session's playhead as a source consumer with no PCM and no
/// ring (#1640).
pub struct SourceTimeline;

impl SourceTimeline {
    /// Compute the exact timeline bytes without creating anything.
    pub fn resource_report(
        quantum_frames: QuantumFrames,
        history_blocks: NonZeroUsize,
    ) -> Result<TimelineResourceReport, PcmSourceRingError> {
        if quantum_frames.0 == 0 {
            return Err(PcmSourceRingError::ZeroQuantumFrames);
        }
        let command_queue = queue_bytes::<SourceCommand>(NonZeroUsize::MIN)?;
        let history = u64::try_from(
            Layout::array::<TimelineEntry>(history_blocks.get())
                .map_err(|_| PcmSourceRingError::ArithmeticOverflow)?
                .size(),
        )
        .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
        let queue_largest = largest_queue_allocation::<SourceCommand>(NonZeroUsize::MIN)?;
        let seek_report = u64::try_from(published_words_retained_bytes::<SEEK_REPORT_WORDS>())
            .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?;
        Ok(TimelineResourceReport {
            history_blocks: u64::try_from(history_blocks.get())
                .map_err(|_| PcmSourceRingError::PlatformSizeLimit)?,
            command_queue_bytes: command_queue,
            command_queue_largest_allocation_bytes: queue_largest,
            history_bytes: history,
            seek_report_bytes: seek_report,
        })
    }

    /// Allocate a timeline at generation 1, timeline sample 0, with a history of `history_blocks`
    /// entries, of the session whose published generation is `published` (#1642 B16-R2), and
    /// split producer and render ownership. Every seek of the producer publishes to `published`,
    /// and the consumer observes no command above it. Control thread only.
    pub fn prepare(
        quantum_frames: QuantumFrames,
        history_blocks: NonZeroUsize,
        published: PublishedGeneration,
    ) -> Result<(TimelineProducer, TimelineConsumer, TimelineResourceReport), PcmSourceRingError>
    {
        let report = Self::resource_report(quantum_frames, history_blocks)?;
        let initial_generation = SourceGeneration(1);
        let (command_producer, command_consumer) =
            bounded_spsc(NonZeroUsize::MIN, QueueGeneration(initial_generation.0))
                .map_err(map_spsc_error)?;
        // Generation 0 until the first block, as a ring's (#1704 D1).
        let (seek_report_writer, seek_report_reader) =
            published_words(SourceSeekReport::default().to_words());
        let consumer = TimelineConsumer::with_history(
            quantum_frames,
            history_blocks,
            Some((command_consumer, seek_report_writer)),
            published.clone(),
        )?;
        Ok((
            TimelineProducer {
                seek_report: seek_report_reader,
                seek: SeekProducer {
                    command_producer,
                    active_generation: initial_generation,
                    quantum_frames: quantum_frames.0,
                    frame_limit: Some(TIMELINE_SAMPLE_LIMIT),
                    published,
                },
            },
            consumer,
            report,
        ))
    }

    /// Allocate a vacant timeline (#1641 D3): its own history of `history_blocks` entries and no
    /// command queue and no seek-report record (#1704 D3), for a successor plan that carries its
    /// predecessor's timeline at the swap block. Until a carry fills it, it advances from timeline
    /// sample 0 and observes no seek, so render stays defined. Its report's command-queue and
    /// seek-report rows name the queue and record the carry brings, which the successor does not
    /// allocate. `published` is the session's published generation (#1642 B16-R2), the word of
    /// the timeline the carry brings. Control thread only.
    pub fn prepare_vacant(
        quantum_frames: QuantumFrames,
        history_blocks: NonZeroUsize,
        published: PublishedGeneration,
    ) -> Result<(TimelineConsumer, TimelineResourceReport), PcmSourceRingError> {
        let report = Self::resource_report(quantum_frames, history_blocks)?;
        let consumer =
            TimelineConsumer::with_history(quantum_frames, history_blocks, None, published)?;
        Ok((consumer, report))
    }
}

/// The timeline's seek producer (#1640 D4): the shared seek producer and nothing else. Plain and
/// anchored seeks, a generation strictly above the current one, an anchor that is a quantum
/// multiple, a frame at most [`TIMELINE_SAMPLE_LIMIT`], one slot. Control thread only.
pub struct TimelineProducer {
    seek: SeekProducer,
    /// The timeline's seek report (#1704 D1); its consumer is the only writer.
    seek_report: PublishedWordsReader<SEEK_REPORT_WORDS>,
}

impl TimelineProducer {
    /// Request a strictly newer generation at `frame`, a timeline sample. It applies on the next
    /// block, or for [`SourceCommand::SeekAt`] on the block its anchor names. A full slot is
    /// [`SourceSeekError::Backpressure`] and changes nothing. An accepted seek applies unless a
    /// newer accepted seek replaces it first (#1274): it is never dropped silently.
    pub fn try_seek(&mut self, command: SourceCommand) -> Result<(), SourceSeekError> {
        self.seek.try_seek(command)
    }

    /// Whether [`Self::try_seek`] would accept `command` now, pushing nothing and changing nothing
    /// (#1640 D1), so an all-or-nothing caller checks every producer before it pushes to any.
    pub fn check_seek(&self, command: SourceCommand) -> Result<(), SourceSeekError> {
        self.seek.check_seek(command)
    }

    /// The newest generation this producer accepted.
    #[must_use]
    pub const fn active_generation(&self) -> SourceGeneration {
        self.seek.active_generation
    }

    /// The timeline's seek report as render last published it (#1704), in a ring's form: the
    /// generation in effect (`0` before the first block), the source-read sample of the first
    /// block it played in, and the timeline sample that block started at; the underrun and held
    /// counts are always `0`. Bounded retries, then [`SourceSeekReportBusy`]. Any thread; it
    /// never waits for render.
    pub fn seek_report(&self) -> Result<SourceSeekReport, SourceSeekReportBusy> {
        self.seek_report
            .read()
            .map(SourceSeekReport::from_words)
            .map_err(|_| SourceSeekReportBusy)
    }
}

/// One history entry: the block that read source time from `source_read_sample` started at
/// timeline sample `timeline`. 16 bytes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct TimelineEntry {
    source_read_sample: u64,
    timeline: u64,
}

/// The node-time mapping of one node block (#1640 D3) and a seek inside it: the graph's types,
/// so the source driver's `timeline_block` (#1647a) hands render the reader's own value.
pub use graph::{TimelineSpan, TimelineStep};

/// The session's playhead (#1640 D2): a source consumer with no PCM, no data queue and no
/// recycle queue. It starts at generation 1, timeline sample 0, takes plain and anchored seeks
/// through the shared seek state (`SeekState`), advances one quantum on every block and keeps a
/// fixed history of `K` blocks, newest last, so a reader can map any source-read sample a node of
/// the plan can still be processing to its timeline sample. Every value is an exact integer.
pub struct TimelineConsumer {
    seek: SeekState,
    /// `None` while vacant (#1641 D3): a successor's timeline before its carry.
    command_consumer: Option<Consumer<SourceCommand>>,
    /// The timeline's seek report (#1704 D1): this consumer is its only writer. `None` while
    /// vacant; a carry moves it with the command queue (D3).
    seek_report: Option<PublishedWordsWriter<SEEK_REPORT_WORDS>>,
    /// What the seek report holds; render is its only writer, so it never loads it back.
    published_seek_report: SourceSeekReport,
    quantum_frames: u32,
    /// The timeline sample of the next block.
    position: u64,
    /// `capacity` entries; the `len` valid ones are a ring that starts at index `oldest`.
    history: Box<[TimelineEntry]>,
    capacity: NonZeroUsize,
    oldest: usize,
    len: usize,
    /// The timeline the oldest entry holds if no seek applied at it: the evicted entry's timeline
    /// plus a quantum, or the starting position while nothing is evicted. With it the oldest entry
    /// reports a seek as every other entry does.
    oldest_expected_timeline: u64,
    /// The session's published generation (#1642 B16-R2): a timeline on its own loads it at each
    /// block entry; the source-set driver loads the same word once for the whole set. A carry
    /// moves the command queue, never the word: both sets are the session's.
    published: PublishedGeneration,
}

impl TimelineConsumer {
    /// The heap bytes this timeline retains (#1074): its command ring and seek-report record in
    /// full and its history. Not the session's published generation, which the session owns.
    /// Control thread only.
    fn retained_bytes(&self) -> u64 {
        use engine::realtime::retained::slice;
        self.command_consumer
            .as_ref()
            .map_or(0, engine::realtime::Consumer::retained_bytes)
            .saturating_add(
                self.seek_report
                    .as_ref()
                    .map_or(0, |writer| writer.retained_bytes()),
            )
            .saturating_add(slice(&self.history))
    }
    /// A timeline at generation 1, timeline sample 0, with an empty history of `history_blocks`
    /// entries. Control thread only.
    fn with_history(
        quantum_frames: QuantumFrames,
        history_blocks: NonZeroUsize,
        endpoints: Option<(
            Consumer<SourceCommand>,
            PublishedWordsWriter<SEEK_REPORT_WORDS>,
        )>,
        published: PublishedGeneration,
    ) -> Result<Self, PcmSourceRingError> {
        let (command_consumer, seek_report) = endpoints.unzip();
        let mut history = Vec::new();
        history
            .try_reserve_exact(history_blocks.get())
            .map_err(|_| PcmSourceRingError::AllocationFailure)?;
        history.resize(history_blocks.get(), TimelineEntry::default());
        Ok(Self {
            seek: SeekState::new(SourceGeneration(1)),
            command_consumer,
            seek_report,
            published_seek_report: SourceSeekReport::default(),
            quantum_frames: quantum_frames.0,
            position: 0,
            history: history.into_boxed_slice(),
            capacity: history_blocks,
            oldest: 0,
            len: 0,
            oldest_expected_timeline: 0,
            published,
        })
    }

    /// Whether this timeline is vacant: it has no command queue, because it waits for a carry
    /// (#1641 D3) or a carry has moved its queue to a successor.
    #[must_use]
    pub const fn is_vacant(&self) -> bool {
        self.command_consumer.is_none()
    }

    /// The quantum every block advances the timeline by.
    #[must_use]
    pub const fn quantum_frames(&self) -> u32 {
        self.quantum_frames
    }

    /// The history's entries, `K`.
    #[must_use]
    pub const fn history_capacity(&self) -> usize {
        self.capacity.get()
    }

    /// The timeline sample the next block starts at.
    #[must_use]
    pub const fn position(&self) -> u64 {
        self.position
    }

    /// The generation in effect.
    #[must_use]
    pub const fn active_generation(&self) -> SourceGeneration {
        self.seek.active_generation
    }
    // REALTIME_POLICY_BEGIN
    /// Begin the block that reads source time from `source_read_sample`: observe one command
    /// through the shared seek state (`SeekClock::At`), set the position to a seek's frame if
    /// one applies, record `(source_read_sample, position)` in the history, then advance one
    /// quantum. It runs on every block, so the timeline never stops. Allocation-free, lock-free
    /// and bounded.
    pub fn begin_block_at(&mut self, source_read_sample: u64) {
        let published = self.published.load_at_block_entry();
        self.begin_block_at_published(source_read_sample, published);
    }

    /// [`Self::begin_block_at`] under the session's published generation `published` (#1642
    /// B16-R2), the source-set driver's form: a command above it stays in its slot.
    fn begin_block_at_published(&mut self, source_read_sample: u64, published: u64) {
        self.take_published(source_read_sample, published);
        self.land_and_advance(source_read_sample, SessionLanding::NONE);
    }

    /// The take half of a block (#2205): pop and hold one command. A vacant timeline has no
    /// queue and observes nothing.
    fn take_published(&mut self, source_read_sample: u64, published: u64) {
        let command = match self.command_consumer.as_mut() {
            Some(consumer) => SeekState::take_published(consumer, published),
            None => None,
        };
        let observation = self
            .seek
            .take(command, SeekClock::At { source_read_sample });
        debug_assert!(
            observation.apply.is_none(),
            "a block's take holds every seek"
        );
    }

    /// The land half of a block (#2205): land the held seek if its block has come, set the
    /// position to its frame, then advance. A held plain seek is the set's `session` decision
    /// when it is the session seek, and lands at once otherwise: a timeline has no PCM to wait
    /// for.
    fn land_and_advance(&mut self, source_read_sample: u64, session: SessionLanding) {
        let ready = match self.seek.when_ready(0, self.quantum_frames) {
            Some((generation, _)) if generation.0 == session.generation => session.lands,
            _ => true,
        };
        if let Some(seek) = self.seek.land(
            SeekClock::At { source_read_sample },
            self.quantum_frames,
            ready,
        ) {
            self.position = seek.frame.0;
        }
        self.advance(source_read_sample);
    }

    /// Whether a prime of `lead_blocks + 1` blocks from `first_sample` can run (#1641 D5,
    /// B16-R3): the source consumer's non-PCM test, written once in
    /// `SeekState::prime_window_clear`. The command queue is empty and no held anchored seek
    /// falls before the prime window's end, so a prime never skips a seek. A timeline has no PCM
    /// to run out of, so that is the whole test.
    #[must_use]
    pub fn prime_ready(&self, first_sample: u64, lead_blocks: u32) -> bool {
        let queued = self
            .command_consumer
            .as_ref()
            .map_or(0, Consumer::available_at_entry);
        self.seek
            .prime_window_clear(queued, self.quantum_frames, first_sample, lead_blocks)
    }

    /// Advance one block from `source_read_sample` as [`Self::begin_block_at`] would, without
    /// observing any command (#1641 D5): a warm successor's prime. It neither pops the command
    /// queue nor applies a held seek; it records the history entry and advances one quantum.
    /// Precondition, as for [`PcmSourceConsumer::prime_block_at`]: [`Self::prime_ready`] was
    /// `true` for the window in this render callback. Allocation-free and wait-free.
    pub fn prime_block_at(&mut self, source_read_sample: u64) {
        self.advance(source_read_sample);
    }

    /// Take `predecessor`'s timeline at a carry (#1641 D3): swap the seek state, the command
    /// consumer, the seek-report writer and its published copy (#1704 D3) and the position, and
    /// copy the predecessor's newest history entries into this
    /// timeline's own history. Nothing is allocated or freed; the predecessor is left vacant.
    /// Both timelines have the same quantum.
    fn adopt_from(&mut self, predecessor: &mut Self) {
        core::mem::swap(&mut self.seek, &mut predecessor.seek);
        core::mem::swap(
            &mut self.command_consumer,
            &mut predecessor.command_consumer,
        );
        core::mem::swap(&mut self.seek_report, &mut predecessor.seek_report);
        core::mem::swap(
            &mut self.published_seek_report,
            &mut predecessor.published_seek_report,
        );
        core::mem::swap(&mut self.position, &mut predecessor.position);
        self.copy_history_from(predecessor);
    }

    /// Record `(source_read_sample, position)`, publish the seek report if a new generation is
    /// in effect (#1704 D2), and advance one quantum. A primed and a rendered block both count as
    /// played, as for a ring's report. A vacant timeline publishes nothing.
    fn advance(&mut self, source_read_sample: u64) {
        let generation = self.seek.active_generation.0;
        if self.published_seek_report.generation != generation
            && let Some(writer) = self.seek_report.as_mut()
        {
            let report = SourceSeekReport {
                generation,
                first_sample: source_read_sample,
                source_frame: self.position,
                ..SourceSeekReport::default()
            };
            self.published_seek_report = report;
            writer.publish(&report.to_words());
        }
        self.record(TimelineEntry {
            source_read_sample,
            timeline: self.position,
        });
        self.position = self.position.saturating_add(u64::from(self.quantum_frames));
    }

    /// The timeline sample at `source_read_sample` (#1640 D3). The entries are searched newest
    /// first for the block that contains the sample. With none, the newest entry that starts
    /// before the sample extrapolates forward, and a sample before every entry extrapolates back
    /// from the oldest one (negative before a fresh plan's first block). With no entry yet, the
    /// position stands at source-read sample 0. Exact; saturates at the `i64` range.
    #[must_use]
    pub fn timeline_at(&self, source_read_sample: u64) -> i64 {
        self.resolve(i128::from(source_read_sample)).1
    }

    /// The node-time mapping of the node block of `frames` samples from
    /// `first_source_read_sample` (#1640 D3): its `start`, and a step at the first sample `s` of
    /// the block where the timeline is not the sample before it plus one (`timeline_at(s) !=
    /// timeline_at(s - 1) + 1`): a seek, or a source-read clock step-back whose newer entry
    /// covers or uncovers an older one (#1641 D4). A step can only fall where an entry starts or
    /// ends, so those are the samples checked. When the node block starts exactly at a step the
    /// step is `(0, start)`. A node block with at most one step, every block
    /// a host renders without a step-back inside it, agrees with `timeline_at` on every sample;
    /// of several, the first is reported. Read-only and allocation-free, with at most
    /// `6K² + 10K + 1` entry reads.
    ///
    /// The first sample is signed (#1647a, R17-5): a node of input arrival `a(n)` reads the block
    /// from `r + ΣP - a(n)`, which is negative while `r + ΣP < a(n)`; such a sample extrapolates
    /// back from the oldest entry, so a fresh plan's node reads `-a(n)` at render sample 0. Every
    /// `i128` value is taken: the arithmetic is exact (a `u64` entry against an `i128` sample),
    /// a value past the `i64` range saturates to it, and nothing wraps or panics.
    #[must_use]
    pub fn timeline_block(&self, first_source_read_sample: i128, frames: u32) -> TimelineSpan {
        let start = self.resolve(first_source_read_sample).1;
        let quantum = u64::from(self.quantum_frames);
        let mut step: Option<TimelineStep> = None;
        for age in 0..self.len {
            let Some(entry) = self.entry(age) else {
                break;
            };
            for sample in [
                entry.source_read_sample,
                entry.source_read_sample.saturating_add(quantum),
            ] {
                let Some(offset) = i128::from(sample)
                    .checked_sub(first_source_read_sample)
                    .and_then(|offset| u32::try_from(offset).ok())
                    .filter(|&offset| offset < frames)
                else {
                    continue;
                };
                if step.is_some_and(|earlier| earlier.offset <= offset) {
                    continue;
                }
                if let Some(timeline) = self.step_at(sample) {
                    step = Some(TimelineStep { offset, timeline });
                }
            }
        }
        TimelineSpan { start, step }
    }

    /// The timeline value at `sample` when the timeline steps there: when it is not the value at
    /// `sample - 1` plus one. When no held entry starts at or before `sample - 1` (that value is
    /// extrapolated back from the oldest entry, or `sample` is 0), the entry that starts at
    /// `sample` is also a step when it is a seek, read against `oldest_expected_timeline` for the
    /// oldest entry: the history no longer holds the block it stepped from.
    fn step_at(&self, sample: u64) -> Option<i64> {
        let (resolving, timeline) = self.resolve(i128::from(sample));
        let stepped = match sample.checked_sub(1) {
            Some(previous) => {
                self.resolve(i128::from(previous)).1.checked_add(1) != Some(timeline)
                    || (!self.starts_at_or_before(previous)
                        && resolving.is_some_and(|age| self.is_seek_entry(age)))
            }
            None => resolving.is_some_and(|age| self.is_seek_entry(age)),
        };
        stepped.then_some(timeline)
    }

    /// Whether a held entry starts at or before `sample`.
    fn starts_at_or_before(&self, sample: u64) -> bool {
        (0..self.len)
            .filter_map(|age| self.entry(age))
            .any(|entry| entry.source_read_sample <= sample)
    }

    /// Replace this history with the newest `min(len_other, K_self)` entries of `other`, oldest
    /// first (#1641's carry): a bounded copy with no allocation. Both timelines have the same
    /// quantum.
    pub fn copy_history_from(&mut self, other: &Self) {
        let count = other.len.min(self.capacity.get());
        for index in 0..count {
            let age = count - 1 - index;
            if let (Some(entry), Some(slot)) = (other.entry(age), self.history.get_mut(index)) {
                *slot = entry;
            }
        }
        self.oldest = 0;
        self.len = count;
        self.oldest_expected_timeline = match other.entry(count) {
            Some(older) => older
                .timeline
                .saturating_add(u64::from(other.quantum_frames)),
            None => other.oldest_expected_timeline,
        };
    }

    /// Append `entry` as the newest, evicting the oldest when the history is full.
    fn record(&mut self, entry: TimelineEntry) {
        if self.len < self.capacity.get() {
            let index = self.oldest.saturating_add(self.len) % self.capacity;
            if let Some(slot) = self.history.get_mut(index) {
                *slot = entry;
            }
            self.len += 1;
            return;
        }
        if let Some(slot) = self.history.get_mut(self.oldest) {
            self.oldest_expected_timeline =
                slot.timeline.saturating_add(u64::from(self.quantum_frames));
            *slot = entry;
        }
        self.oldest = self.oldest.saturating_add(1) % self.capacity;
    }

    /// The entry `age` blocks before the newest (age 0 is the newest), if held.
    fn entry(&self, age: usize) -> Option<TimelineEntry> {
        let back = self.len.checked_sub(1)?.checked_sub(age)?;
        let index = self.oldest.checked_add(back)? % self.capacity;
        self.history.get(index).copied()
    }

    /// Whether the entry of `age` is a seek: its timeline is not its previous entry's plus a
    /// quantum (for the oldest entry, not `oldest_expected_timeline`).
    fn is_seek_entry(&self, age: usize) -> bool {
        let Some(entry) = self.entry(age) else {
            return false;
        };
        let expected = match age.checked_add(1).and_then(|older| self.entry(older)) {
            Some(previous) => previous
                .timeline
                .saturating_add(u64::from(self.quantum_frames)),
            None => self.oldest_expected_timeline,
        };
        entry.timeline != expected
    }

    /// The age of the entry whose block contains `sample` (`None` when the value is
    /// extrapolated), and the timeline value at `sample`.
    fn resolve(&self, sample: i128) -> (Option<usize>, i64) {
        let quantum = i128::from(self.quantum_frames);
        let mut before = None;
        for age in 0..self.len {
            let Some(entry) = self.entry(age) else {
                break;
            };
            if let Some(offset) = sample
                .checked_sub(i128::from(entry.source_read_sample))
                .filter(|&offset| offset >= 0)
            {
                if offset < quantum {
                    return (Some(age), offset_timeline(entry, sample));
                }
                if before.is_none() {
                    before = Some(entry);
                }
            }
        }
        let anchor = before
            .or_else(|| {
                self.len
                    .checked_sub(1)
                    .and_then(|oldest| self.entry(oldest))
            })
            .unwrap_or(TimelineEntry {
                source_read_sample: 0,
                timeline: self.position,
            });
        (None, offset_timeline(anchor, sample))
    }
}

/// `entry.timeline + (sample - entry.source_read_sample)`, exact, saturated to the `i64` range.
/// `sample` is signed (#1647a): it may lie before source-read sample 0. The `i128` sum saturates
/// at the `i128` range first, which lies far outside the `i64` one, so no input wraps.
fn offset_timeline(entry: TimelineEntry, sample: i128) -> i64 {
    let value = i128::from(entry.timeline)
        .saturating_add(sample)
        .saturating_sub(i128::from(entry.source_read_sample));
    i64::try_from(value).unwrap_or(if value < 0 { i64::MIN } else { i64::MAX })
}
// REALTIME_POLICY_END

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

    /// A source of `channel_count` channels whose ring is not allocated, charged the ring
    /// `resources` reports (issue #2199): what [`project_session_graph_source_set`] projects a
    /// prepared ring's charge from. Projection only; it never reaches a source set.
    #[must_use]
    pub const fn projected(channel_count: u32, resources: SourceResourceReport) -> Self {
        Self {
            consumer: None,
            channel_count,
            resources: Some(resources),
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
    SourceIndex,
    ChannelIndex,
    ArithmeticOverflow,
    /// The timeline's quantum is not the envelope's (#1641 D1).
    TimelineQuantum,
    /// The timeline or an occupied source was prepared with another session's published
    /// generation than the set's (#1642 B16-R2): its seeks would publish to a word render does
    /// not load.
    SessionWord,
}

struct GraphSourceEntry {
    /// `None` while vacant: the entry renders `+0.0` and reports an underrun every block.
    consumer: Option<PcmSourceConsumer>,
    /// The channel count the mappings were validated against, kept for a vacant entry.
    channel_count: u32,
}

struct SourceGraphSourceSetDriver {
    /// The session's playhead (#1641 D1), inline: the driver's own allocation counts its value.
    timeline: TimelineConsumer,
    /// The session's published generation (#1642 B16-R2), loaded once per block entry: the word
    /// of the timeline and of every consumer of the set, checked at sealing.
    published: PublishedGeneration,
    sources: Box<[GraphSourceEntry]>,
    mappings: Box<[SourceGraphTrackMapping]>,
    quantum_frames: u32,
    block_validity: GraphObservationValidity,
    pending_generation_change: bool,
    /// The newest plain session seek a block entry of this set has seen (#2205), `0` for none:
    /// render-owned, carried to a successor at a swap.
    session_seek: u64,
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
    /// The timeline, each source's consumer, and the source and mapping tables with the
    /// mappings' node IDs (#1074). Not the session's published generation: the session owns it
    /// and outlives every plan (host-core's plan inventory holds and charges it).
    fn retained_bytes(&self) -> u64 {
        use engine::realtime::retained::slice;
        let sources = self
            .sources
            .iter()
            .fold(slice(&self.sources), |total, source| {
                total.saturating_add(
                    source
                        .consumer
                        .as_ref()
                        .map_or(0, PcmSourceConsumer::retained_bytes),
                )
            });
        let mappings = self
            .mappings
            .iter()
            .fold(slice(&self.mappings), |total, mapping| {
                total.saturating_add(mapping.node.retained_bytes())
            });
        self.timeline
            .retained_bytes()
            .saturating_add(sources)
            .saturating_add(mappings)
    }
    fn can_prepare_source_seek(&self, source_index: usize) -> bool {
        self.sources
            .get(source_index)
            .is_some_and(|source| source.consumer.is_some())
    }

    fn prepare_source_seek(&mut self, source_index: usize, generation: u64, frame: u64) -> bool {
        let Some(generation) = SourceGeneration::new(generation) else {
            return false;
        };
        let published = self.published_at_entry();
        let prepared = self
            .sources
            .get_mut(source_index)
            .and_then(|source| source.consumer.as_mut())
            .is_some_and(|consumer| {
                consumer.prepare_seek_published(generation, SourceFrame(frame), published)
            });
        if prepared {
            self.pending_generation_change = true;
        }
        prepared
    }

    fn prepare_admitted_seeks(&mut self) -> usize {
        let published = self.published_at_entry();
        let mut prepared = 0;
        for consumer in self
            .sources
            .iter_mut()
            .filter_map(|source| source.consumer.as_mut())
        {
            if consumer.prepare_admitted_seek(published).is_some() {
                prepared += 1;
            }
        }
        if prepared != 0 {
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
        // #1642 B16-R2: one load of the session's published generation, before the timeline and
        // the sources, so every consumer observes the same session seeks in this block.
        let published = self.published_at_entry();
        if let Some(session) = self.published.session_seek_at_entry(published) {
            self.session_seek = session;
        }
        // #1641 D1: the timeline first, at the sample the sources read, so it observes a seek in
        // the same block as a source given the same seek. It applies no state to the sources.
        // The graph passes the plan's source-read sample, which an anchored seek waits on: the
        // render clock plus the plan's source-read offset (#1316 D1, #1396 D1).
        let clock = SeekClock::At {
            source_read_sample: first_sample,
        };
        self.timeline.take_published(first_sample, published);
        for consumer in self
            .sources
            .iter_mut()
            .filter_map(|source| source.consumer.as_mut())
        {
            consumer.begun_block = true;
            consumer.observed_published = published;
            consumer.begin_block_observe(clock);
        }
        // #2205: the plain session seek lands for every consumer that holds it, and the timeline,
        // in one block: the first at which each of them has its runway, or the first at which a
        // deadline has come. A vacant entry holds nothing.
        let mut all_ready = true;
        let mut expired = false;
        for consumer in self
            .sources
            .iter()
            .filter_map(|source| source.consumer.as_ref())
        {
            if let Some((ready, late)) = consumer.session_seek_hold(self.session_seek, first_sample)
            {
                all_ready &= ready;
                expired |= late;
            }
        }
        // A plain session seek with no live audio to keep lands at once.
        let session = SessionLanding {
            generation: self.session_seek,
            lands: all_ready
                || expired
                || self.published.session_seek_lands_at_once(self.session_seek),
        };
        self.timeline.land_and_advance(first_sample, session);
        for source in &mut self.sources {
            // A vacant entry has nothing to play: its block is an underrun.
            let Some(consumer) = source.consumer.as_mut() else {
                self.block_validity.source_underrun = true;
                continue;
            };
            let report = consumer.begin_block_play(clock, CommandObservation::Allowed, session);
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

    /// The claim's source played no block, its played block is stamped silent, or the entry is
    /// vacant (issue #2133): every word `copy_track_input` writes or `played_planes` lends is
    /// exact `+0.0`. The fact was computed where the host's chunk was copied in, on the control
    /// thread; no host states it.
    fn played_block_silent(&self, claim_index: usize) -> bool {
        let Some(source) = self
            .mappings
            .get(claim_index)
            .and_then(|mapping| self.sources.get(mapping.source_index))
        else {
            return false;
        };
        source
            .consumer
            .as_ref()
            .is_none_or(PcmSourceConsumer::played_block_silent)
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

    /// A vacant entry contributes 0: its ring moved to the successor.
    fn unread_source_frames(&self) -> u64 {
        self.sources
            .iter()
            .filter_map(|source| source.consumer.as_ref())
            .fold(0, |total, consumer| {
                total.saturating_add(consumer.unread_frames())
            })
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
    /// the channel counts, quanta and session words (#1642 B16-R2) equal -- and any failure moves
    /// nothing. The predecessor's
    /// pending generation change (a seek prepared since its last block) is carried too.
    ///
    /// A vacant timeline takes an occupied predecessor's (#1641 D3): its seek state, command
    /// consumer and position by swap and its newest history entries by copy, into this
    /// timeline's own history. Any other pair of timelines stays as it is and does not refuse
    /// the source moves: a timeline vacant on both sides keeps advancing from its own position.
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
            && predecessor.published == self.published
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
        if self.timeline.is_vacant() && !predecessor.timeline.is_vacant() {
            self.timeline.adopt_from(&mut predecessor.timeline);
        }
        self.pending_generation_change |= predecessor.pending_generation_change;
        self.session_seek = self.session_seek.max(predecessor.session_seek);
        true
    }

    /// The timeline sample at `source_read_sample` (#1641 D8), from the timeline's history.
    fn timeline_at(&self, source_read_sample: u64) -> Option<i64> {
        Some(self.timeline.timeline_at(source_read_sample))
    }

    /// The node-time mapping of the node block of `frames` samples from the signed
    /// `first_source_read_sample` (#1647a), from the timeline's history: render's per-block read.
    fn timeline_block(&self, first_source_read_sample: i128, frames: u32) -> Option<TimelineSpan> {
        Some(
            self.timeline
                .timeline_block(first_source_read_sample, frames),
        )
    }
    // REALTIME_POLICY_END

    fn as_any(&self) -> Option<&dyn core::any::Any> {
        Some(self)
    }

    fn source_has_begun_block(&self, source_index: usize) -> Option<bool> {
        self.sources
            .get(source_index)?
            .consumer
            .as_ref()
            .map(PcmSourceConsumer::has_begun_block)
    }

    /// Every pair names an occupied entry on both sides, of equal channel count, no index twice on
    /// either side, and the two sets share one quantum and one session word (#1642 B16-R2).
    fn can_swap_sources(
        &self,
        other: &dyn GraphPreparedSourceSetDriver,
        pairs: &[(u32, u32)],
    ) -> bool {
        other
            .as_any()
            .and_then(|any| any.downcast_ref::<Self>())
            .is_some_and(|other| self.swappable(other, pairs))
    }

    /// Exchange each pair's entries whole after [`Self::can_swap_sources`]'s check. The other
    /// set's pending generation change (a seek prepared on a donated consumer) comes with its
    /// consumers, as in [`Self::adopt_sources`].
    fn swap_sources(
        &mut self,
        other: &mut dyn GraphPreparedSourceSetDriver,
        pairs: &[(u32, u32)],
    ) -> bool {
        let Some(other) = other
            .as_any_mut()
            .and_then(|any| any.downcast_mut::<Self>())
        else {
            return false;
        };
        if !self.swappable(other, pairs) {
            return false;
        }
        for &(own, theirs) in pairs {
            core::mem::swap(
                &mut self.sources[own as usize],
                &mut other.sources[theirs as usize],
            );
        }
        // Set-wide, so over-approximate: a seek prepared on a donor entry this donation does not
        // move still flags this set's next block. Safe: the flag only marks that block's
        // observation validity as crossing a generation change, which an observer reads as a
        // discontinuity; a false mark costs one block of conservative observation and never
        // hides a real change, and this set's own flag is kept. It is also unreachable in
        // production, where `prepare_source_seek` runs only on the plan render runs (the active
        // plan of the exchange) and a donor is a withdrawn candidate that never was. A per-entry
        // flag would put a new read into `begin_block` and `adopt_sources` on render for no
        // audible gain.
        if !pairs.is_empty() {
            self.pending_generation_change |= other.pending_generation_change;
        }
        true
    }

    // REALTIME_POLICY_BEGIN
    /// The timeline's prime check and every flagged consumer's (#1355 D4, #1320 D2, #1641 D5):
    /// a ring no warm successor carries is not asked, so it never holds the adoption back.
    fn prime_ready(&self, source_read_sample: u64, lead_blocks: u32) -> bool {
        self.timeline.prime_ready(source_read_sample, lead_blocks)
            && self
                .sources
                .iter()
                .filter_map(|source| source.consumer.as_ref())
                .filter(|consumer| consumer.prime_required())
                .all(|consumer| consumer.prime_ready(source_read_sample, lead_blocks))
    }

    /// One prime block (#1355 D5): every move's successor entry must be occupied (the carry has
    /// moved it in), or nothing is primed. Then the timeline, if it is not vacant, and each moved
    /// consumer replay the block from `source_read_sample` (#1320 D3, #1641 D5).
    fn prime_block_at(&mut self, source_read_sample: u64, moves: &[(u32, u32)]) -> bool {
        if !moves.iter().all(|&(successor, _)| {
            self.sources
                .get(successor as usize)
                .is_some_and(|source| source.consumer.is_some())
        }) {
            return false;
        }
        if !self.timeline.is_vacant() {
            self.timeline.prime_block_at(source_read_sample);
        }
        for &(successor, _) in moves {
            if let Some(consumer) = self.sources[successor as usize].consumer.as_mut() {
                let _report = consumer.prime_block_at(source_read_sample);
            }
        }
        true
    }
    // REALTIME_POLICY_END
}

impl SourceGraphSourceSetDriver {
    /// The block entry's load of the session's published generation (#1642 B16-R2). One
    /// `Acquire` load, wait-free.
    fn published_at_entry(&self) -> u64 {
        self.published.load_at_block_entry()
    }

    /// [`GraphPreparedSourceSetDriver::can_swap_sources`] against a set of this type.
    fn swappable(&self, other: &Self, pairs: &[(u32, u32)]) -> bool {
        other.quantum_frames == self.quantum_frames
            && other.published == self.published
            && pairs.iter().enumerate().all(|(index, &(own, theirs))| {
                let fresh = pairs[..index].iter().all(|&(earlier_own, earlier_theirs)| {
                    earlier_own != own && earlier_theirs != theirs
                });
                match (
                    self.sources.get(own as usize),
                    other.sources.get(theirs as usize),
                ) {
                    (Some(own), Some(theirs)) => {
                        fresh
                            && own.consumer.is_some()
                            && theirs.consumer.is_some()
                            && own.channel_count == theirs.channel_count
                    }
                    _ => false,
                }
            })
    }
}

/// Seal one or more prepared source consumers of one session into a graph-owned fan-out source
/// set, for one plan of that session.
///
/// The wrapper owns exactly one consumer per source and the session's timeline (#1641 D1). The
/// graph calls `begin_block` once, which begins the timeline's block and then each source's, then
/// each mapping copies directly from the retained played block, which stays the consumer's until
/// the next `begin_block` or seek preparation (#917).
///
/// The driver loads `published`, the session's published generation (#1642 B16-R2), once per
/// block entry and passes it to the timeline's and every source's observe step, so a session seek
/// reaches every consumer at one block. `published` is the word every producer of the session
/// publishes to, the same object for every plan of the session; the timeline and every occupied
/// source must be prepared with it, or the set is [`SourceGraphSourceSetError::SessionWord`]. It
/// is the session's, not this set's: no row of the report counts it.
///
/// The timeline's fixed bytes join the set's overhead and largest allocation (#1641 D7): its
/// history, and for an occupied timeline its command queue and its seek-report record (#1704 D4).
/// A vacant timeline's queue and record arrive with its carry, so its host counts them with the
/// carried source overhead. The consumer value is inline in the driver, whose own allocation
/// already counts it.
/// The resource report [`prepare_session_graph_source_set`] gives a set of `sources` and
/// `mappings` over a timeline of `timeline_capacity` history blocks, vacant or not (issue #2199):
/// the one arithmetic behind it, so a host projects the set's charge before it allocates a ring.
/// A source made by [`SourceGraphSource::projected`] is charged as the ring it stands for.
///
/// # Errors
///
/// `ArithmeticOverflow` when a sum overflows, `SourceIndex` for a mapping off a track input.
pub fn project_session_graph_source_set(
    quantum: QuantumFrames,
    timeline_capacity: NonZeroUsize,
    timeline_vacant: bool,
    sources: &[SourceGraphSource],
    mappings: &[SourceGraphTrackMapping],
) -> Result<GraphSourceSetResourceReport, SourceGraphSourceSetError> {
    let retained = source_set_retained_resources(sources, mappings)?;
    let timeline_report = SourceTimeline::resource_report(quantum, timeline_capacity)
        .map_err(|_| SourceGraphSourceSetError::ArithmeticOverflow)?;
    let mut pcm_payload = 0_u64;
    let mut overhead = timeline_report.history_bytes;
    let mut largest = timeline_report.history_bytes;
    if !timeline_vacant {
        overhead = overhead
            .checked_add(timeline_report.command_queue_bytes)
            .and_then(|bytes| bytes.checked_add(timeline_report.seek_report_bytes))
            .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
        largest = largest
            .max(timeline_report.command_queue_largest_allocation_bytes)
            .max(timeline_report.seek_report_bytes);
    }
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
    }
    overhead = overhead
        .checked_add(
            retained
                .overhead_bytes()
                .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?,
        )
        .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
    largest = largest.max(retained.largest_allocation_bytes());
    let total = pcm_payload
        .checked_add(overhead)
        .ok_or(SourceGraphSourceSetError::ArithmeticOverflow)?;
    Ok(GraphSourceSetResourceReport {
        pcm_payload_already_charged_bytes: pcm_payload,
        overhead_bytes: overhead,
        total_engine_owned_bytes: total,
        largest_allocation_bytes: largest,
    })
}

pub fn prepare_session_graph_source_set(
    envelope: engine::realtime::RenderEnvelope,
    sources: Vec<SourceGraphSource>,
    timeline: TimelineConsumer,
    published: PublishedGeneration,
    mappings: Vec<SourceGraphTrackMapping>,
) -> Result<GraphPreparedSourceSet, SourceGraphSourceSetError> {
    if timeline.quantum_frames != envelope.quantum.0 {
        return Err(SourceGraphSourceSetError::TimelineQuantum);
    }
    let foreign = |consumer: &PcmSourceConsumer| consumer.published != published;
    if timeline.published != published
        || sources
            .iter()
            .filter_map(|source| source.consumer.as_ref())
            .any(foreign)
    {
        return Err(SourceGraphSourceSetError::SessionWord);
    }
    let report = project_session_graph_source_set(
        envelope.quantum,
        timeline.capacity,
        timeline.is_vacant(),
        &sources,
        &mappings,
    )?;
    let entries: Vec<GraphSourceEntry> = sources
        .into_iter()
        .map(|source| GraphSourceEntry {
            consumer: source.consumer,
            channel_count: source.channel_count,
        })
        .collect();
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
    let claims = mappings
        .iter()
        .map(|mapping| GraphSourceInputClaim {
            node: mapping.node.clone(),
        })
        .collect();
    Ok(GraphPreparedSourceSet::new(
        envelope,
        claims,
        report,
        Box::new(SourceGraphSourceSetDriver {
            timeline,
            published,
            sources: entries.into_boxed_slice(),
            mappings: mappings.into_boxed_slice(),
            quantum_frames: envelope.quantum.0,
            block_validity: GraphObservationValidity::CLEAR,
            pending_generation_change: false,
            session_seek: 0,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: SampleRateHz = SampleRateHz(48_000);

    std::thread_local! {
        static SESSION: PublishedGeneration = PublishedGeneration::new(SourceGeneration(1));
    }

    /// The test's session word (#1642 B16-R2): every ring, timeline and set one test prepares
    /// shares it, as every plan of one session does. Each test runs on its own thread, so each
    /// test is one session.
    fn session() -> PublishedGeneration {
        SESSION.with(Clone::clone)
    }

    /// A ring of the test's session, starting at frame 0.
    fn session_ring(
        config: PcmSourceRingConfig,
    ) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
    {
        PcmSourceRing::prepare_host_region(config, SourceFrame(0), session())
    }

    /// [`prepare_session_graph_source_set`] under the test's session word.
    fn session_source_set(
        envelope: engine::realtime::RenderEnvelope,
        sources: Vec<SourceGraphSource>,
        timeline: TimelineConsumer,
        mappings: Vec<SourceGraphTrackMapping>,
    ) -> Result<GraphPreparedSourceSet, SourceGraphSourceSetError> {
        prepare_session_graph_source_set(envelope, sources, timeline, session(), mappings)
    }

    /// An occupied timeline of one history entry at `quantum` frames, for a hand-built source set.
    fn test_timeline(quantum: u32) -> TimelineConsumer {
        SourceTimeline::prepare(QuantumFrames(quantum), NonZeroUsize::MIN, session())
            .expect("timeline")
            .1
    }

    fn config(channels: u32, quantum: u32, capacity: u64) -> PcmSourceRingConfig {
        PcmSourceRingConfig {
            channel_count: channels,
            quantum_frames: QuantumFrames(quantum),
            frame_capacity: capacity,
            initial_generation: SourceGeneration(1),
            seek_runway_blocks: 0,
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
                + report.seek_report_bytes
                + report.prime_required_bytes
                + report.seek_readiness_bytes
        );
        // #1316 D2: the seek report is one shared record of a sequence word and five report
        // words, behind `Arc`'s two counters, on every target.
        assert_eq!(
            report.seek_report_bytes,
            u64::try_from(2 * size_of::<usize>() + 6 * size_of::<u64>()).expect("record size")
        );
        // #1320 D4: the `prime_required` flag is one `AtomicBool` behind `Arc`'s two counters,
        // padded to the counters' alignment: three words on every target.
        assert_eq!(
            report.prime_required_bytes,
            u64::try_from(3 * size_of::<usize>()).expect("flag size")
        );
        // #2205: the seek-readiness word is one `AtomicU64` behind `Arc`'s two counters.
        assert_eq!(
            report.seek_readiness_bytes,
            u64::try_from(2 * size_of::<usize>() + size_of::<u64>()).expect("word size")
        );
        // The memory delta over the pre-#917 two-block ring, which sized both queues and the
        // metadata at two blocks: one block's PCM, one block's metadata and one pointer slot in
        // each queue. Nothing moves into the session-charged PCM row.
        let two = NonZeroUsize::new(2).expect("configured blocks");
        let pre_change_overhead = 2 * queue_bytes::<Box<TransferBlock>>(two).expect("queue")
            + report.command_queue_bytes
            + report.seek_report_bytes
            + report.prime_required_bytes
            + report.seek_readiness_bytes
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
            largest_queue
                .max(32)
                .max(metadata)
                .max(report.seek_report_bytes)
                .max(report.prime_required_bytes)
                .max(report.seek_readiness_bytes)
        );
    }

    /// Issue #2133 gate 1: the played block's silence is the producer's monotone last-non-zero
    /// frame against the block's first frame. A `+0.0` chunk plays silent; one `-0.0` word in the
    /// last frame of the right plane makes its block not silent, and the next `+0.0` block is
    /// silent again; an underrun plays silent; after a seek back to frame 0 the counter starts
    /// over, so a `+0.0` chunk of the new generation plays silent; and a short final `+0.0` block,
    /// whose tail the ring zeroes itself, plays silent.
    ///
    /// Test value: red when a block with a non-zero word is reported silent (a counter that
    /// ignores the right plane or the last frame, or a stamp read against the wrong frame), or
    /// when the counter survives a seek and keeps a silent block audible.
    #[test]
    fn a_played_block_is_silent_iff_its_producer_copied_no_nonzero_word() {
        let (producer, mut consumer, _) = session_ring(config(2, 4, 16)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        let zero = [0.0_f32; 4];
        let late = [0.0, 0.0, 0.0, -0.0];
        host.submit(chunk(1, 0, &[&zero, &zero], 4, false)).unwrap();
        host.submit(chunk(1, 4, &[&zero, &late], 4, false)).unwrap();
        host.submit(chunk(1, 8, &[&zero, &zero], 4, false)).unwrap();
        let mut plays = Vec::new();
        for block in 0..4_u64 {
            let report = consumer.begin_block_at(block * 4);
            plays.push((report.copied_frames, consumer.played_block_silent()));
        }
        assert_eq!(
            plays,
            [(4, true), (4, false), (4, true), (0, true)],
            "a +0.0 block, the -0.0 word, +0.0 again, an underrun"
        );
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(0),
        })
        .unwrap();
        host.submit(chunk(2, 0, &[&zero, &zero], 4, false)).unwrap();
        host.submit(chunk(2, 4, &[&zero[..2], &zero[..2]], 2, true))
            .unwrap();
        let mut plays = Vec::new();
        for block in 4..6_u64 {
            let report = consumer.begin_block_at(block * 4);
            plays.push((report.copied_frames, consumer.played_block_silent()));
        }
        assert_eq!(
            plays,
            [(4, true), (2, true)],
            "after the seek: frame 0 again, then the short final block"
        );
    }

    /// #2205 gate 4: with a live runway prepared, a plain seek prepared between blocks still
    /// applies at once (`SeekClock::Hold`), not when its runway is queued.
    #[test]
    fn paused_seek_prepares_full_queues_without_consuming_target() {
        for retained in [false, true] {
            let (producer, mut consumer, _) = session_ring(PcmSourceRingConfig {
                seek_runway_blocks: 1,
                ..config(1, 4, 8)
            })
            .unwrap();
            let mut host = producer.into_host_chunk_provider(RATE);
            let old = [0.25; 4];
            host.submit(chunk(1, 0, &[&old], 4, false)).unwrap();
            host.submit(chunk(1, 4, &[&old], 4, false)).unwrap();
            if retained {
                consumer.acquire_current_block(
                    SeekClock::At {
                        source_read_sample: 0,
                    },
                    CommandObservation::Allowed,
                );
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
            let report = read_at(&mut consumer, 0, &mut [&mut output]);
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
        let mut output = [f32::NAN; 4];
        let report = read_at(consumer, first_sample, &mut [&mut output]);
        (output, report)
    }

    /// The product read path for one block starting at `first_sample`: begin the block, copy
    /// every channel into `planes`, end the block.
    fn read_at(
        consumer: &mut PcmSourceConsumer,
        first_sample: u64,
        planes: &mut [&mut [f32]],
    ) -> SourceReadReport {
        let report = consumer.begin_block_at(first_sample);
        copy_planes_and_end(consumer, planes);
        report
    }

    /// Copy every channel of the begun block into `planes`, then end the block.
    fn copy_planes_and_end(consumer: &mut PcmSourceConsumer, planes: &mut [&mut [f32]]) {
        for (channel, plane) in planes.iter_mut().enumerate() {
            consumer
                .copy_channel(u32::try_from(channel).unwrap(), plane)
                .unwrap();
        }
        consumer.end_block();
    }

    /// #1274 D1. Red if an anchor no block starts at is admitted, or if the refusal switches the
    /// producer's generation anyway.
    #[test]
    fn an_unaligned_anchor_is_refused_without_a_generation_switch() {
        let (producer, _consumer, _) = session_ring(config(1, 4, 8)).unwrap();
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
        let (producer, mut consumer, _) = session_ring(config(1, 4, 16)).unwrap();
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
        assert_eq!(consumer.seek.active_generation, SourceGeneration(3));
    }

    /// #1274 D3. Red if the held generation's primed block plays before its anchor because it
    /// starts at the frame the old generation has reached (`current_matches_next_frame` must
    /// compare the generation), or if it is discarded rather than kept as the pending block.
    #[test]
    fn a_primed_block_waits_for_its_anchor() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 4, 8)).unwrap();
        host.submit(chunk(2, 4, &[&[2.0; 4]], 4, false)).unwrap();
        let (output, report) = render_at(&mut consumer, 0);
        assert_eq!(
            (output, report.held_frames, report.underrun_frames),
            ([0.0; 4], 4, 0)
        );
        // The old generation's held wait has reached frame 4, the primed block's start.
        let (output, report) = render_at(&mut consumer, 4);
        assert_eq!(
            (output, report.held_frames, report.underrun_frames),
            ([0.0; 4], 4, 0)
        );
        assert_eq!(report.active_generation, SourceGeneration(1));
        let (output, report) = render_at(&mut consumer, 8);
        assert_eq!((output, report.copied_frames), ([2.0; 4], 4));
        assert_eq!(report.active_generation, SourceGeneration(2));
    }

    /// #1274 D4. Red if seek preparation pops an anchored seek and drops it, losing an accepted
    /// seek, or applies it with no block time.
    #[test]
    fn prepare_seek_holds_an_anchored_seek() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&[1.0; 4]], 4, false)).unwrap();
        host.try_seek(seek_at(2, 100, 8)).unwrap();
        assert!(!consumer.prepare_seek(SourceGeneration(2), SourceFrame(100)));
        assert_eq!(consumer.seek.active_generation, SourceGeneration(1));
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
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
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

    /// #1699 D1. Red if `unread_frames` misses the popped-but-unplayed pending block, counts a
    /// short end block as a whole quantum, or counts a block render already played.
    #[test]
    fn unread_frames_count_the_pending_and_queued_blocks_not_the_played_one() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 16)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&[1.0; 4]], 4, false)).unwrap();
        assert_eq!(consumer.unread_frames(), 4);
        let (output, _) = render_at(&mut consumer, 0);
        assert_eq!(output, [1.0; 4]);
        assert_eq!(consumer.unread_frames(), 0, "the played block is read");
        host.try_seek(seek_at(2, 100, 8)).unwrap();
        host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false)).unwrap();
        host.submit(chunk(2, 104, &[&[2.0; 4]], 4, false)).unwrap();
        host.submit(chunk(2, 108, &[&[2.0, 2.0]], 2, true)).unwrap();
        // The held seek's first block is popped and pending; two blocks stay queued.
        let (_, report) = render_at(&mut consumer, 4);
        assert_eq!(report.held_frames, 4);
        assert_eq!(consumer.unread_frames(), 4 + 4 + 2);
        let (output, _) = render_at(&mut consumer, 8);
        assert_eq!(output, [2.0; 4]);
        assert_eq!(consumer.unread_frames(), 4 + 2);
        // R2: a further seek replaces generation 2; its two queued blocks are stale but still
        // accepted and unread, and they count with the new generation's first block.
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(3),
            frame: SourceFrame(200),
        })
        .unwrap();
        host.submit(chunk(3, 200, &[&[3.0; 4]], 4, false)).unwrap();
        assert_eq!(consumer.unread_frames(), 4 + 2 + 4);
    }

    /// #1274 MINOR-1. Red if an on-time apply keeps the primed block without noting the region
    /// end it carries: a stem whose primed block is its region's last, short block would then
    /// report an underrun on every block after it ends, and never reach the end of its region.
    #[test]
    fn an_on_time_anchored_seek_to_a_last_short_block_ends_its_region() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
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
    /// window in which a seek's PCM can reach the render before its command does. The block
    /// loads the session's word at its entry, before the producer step, as every entry does.
    fn render_with_producer_in_window(
        consumer: &mut PcmSourceConsumer,
        clock: SeekClock,
        producer: impl FnOnce(),
    ) -> ([f32; 4], SourceReadReport) {
        consumer.observed_published = consumer.published.load_at_block_entry();
        consumer.begin_block_observe(clock);
        producer();
        let report =
            consumer.begin_block_play(clock, CommandObservation::Allowed, SessionLanding::NONE);
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
            // Each run is a session of its own whose word already covers generation 2 (another
            // producer of the session published a later one), so the block entry's fence admits
            // the seek pushed in its window: the re-observe's case under the fence (#1642 B16-R2).
            let word = PublishedGeneration::new(SourceGeneration(3));
            let (producer, mut consumer, _) =
                PcmSourceRing::prepare_host_region(config(1, 4, 8), SourceFrame(0), word).unwrap();
            let mut host = producer.into_host_chunk_provider(RATE);
            let command = if anchored {
                seek_at(2, 100, 8)
            } else {
                SourceCommand::Seek {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(100),
                }
            };
            let (output, report) = render_with_producer_in_window(
                &mut consumer,
                SeekClock::At {
                    source_read_sample: 4,
                },
                || {
                    host.try_seek(command).unwrap();
                    host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false))
                        .expect("accepted");
                },
            );
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
            assert_eq!(consumer.seek.active_generation, SourceGeneration(2));
            assert_eq!(consumer.stale_generation_discard_count, 0);
            // The anchored run waited one block for its anchor: held, never an underrun.
            assert_eq!(consumer.underrun_events, 0);
            assert_eq!(consumer.held_blocks, u64::from(anchored));
        }

        // An anchored seek whose anchor has already passed when the window's re-observe takes
        // it applies at once, at `frame` plus the lateness, by the block's own clock: block 8
        // plays frame 104, and only the skipped block at frame 100 is discarded. The word covers
        // generation 2 at the block's entry, as above.
        let (producer, mut consumer, _) = PcmSourceRing::prepare_host_region(
            config(1, 4, 16),
            SourceFrame(0),
            PublishedGeneration::new(SourceGeneration(3)),
        )
        .unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        let (output, report) = render_with_producer_in_window(
            &mut consumer,
            SeekClock::At {
                source_read_sample: 8,
            },
            || {
                host.try_seek(seek_at(2, 100, 4)).unwrap();
                host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false))
                    .expect("accepted");
                host.submit(chunk(2, 104, &[&[2.5; 4]], 4, false))
                    .expect("accepted");
            },
        );
        assert_eq!(
            output, [2.5; 4],
            "late by one block: frame 104 plays at block 8"
        );
        assert!(report.generation_changed);
        assert_eq!(consumer.seek.active_generation, SourceGeneration(2));
        assert_eq!(
            consumer.stale_generation_discard_count, 1,
            "the late-skipped block"
        );
    }

    /// A host that keeps the ring fed while the render acquires its block (#1447). Every block
    /// the host publishes is stale, so each is discarded, and a block's acquire pops only what was
    /// queued when it began: the next block takes the one the host published during it. Red if
    /// the acquire is bounded by the ring's capacity instead of the count at its entry: it then
    /// pops again whenever the host refills the queue between two passes, and chases the host.
    /// The test races a real host thread, so it is red on most runs of that defect, not on every
    /// run: a green run on a reverted bound is not proof the bound holds.
    #[test]
    fn the_acquire_does_not_pop_a_block_published_after_its_count() {
        const BLOCKS: usize = 200_000;
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
        assert!(consumer.transfer_block_count > 1);
        consumer.next_frame = SourceFrame(u64::MAX / 2);
        let host = producer.into_host_chunk_provider(RATE);
        let started = std::time::Instant::now();
        bench_support::producer::render_while_producing(
            BLOCKS,
            host,
            |host| {
                let queue = &host.producer.data_producer;
                if queue.available_capacity() == queue.capacity() {
                    let start = host.producer.next_write_frame.0;
                    let _submitted = host.submit(chunk(1, start, &[&[0.5; 4]], 4, false));
                }
            },
            |host| {
                let queue = &host.producer.data_producer;
                queue.available_capacity() < queue.capacity()
            },
            |block| {
                let before = consumer.stale_generation_discard_count;
                consumer.begin_block_at(4 * u64::try_from(block).unwrap());
                let discarded = consumer.stale_generation_discard_count - before;
                assert!(
                    discarded <= 1,
                    "block {block}: the acquire popped {discarded} blocks, one was queued at its entry"
                );
            },
        );
        eprintln!("{BLOCKS} blocks in {:?}", started.elapsed());
    }

    /// #1274 follow-ups NIT-2. Red if `prepare_seek` grants readiness for the playing generation
    /// at its own position while a newer anchored seek is held: the caller would take the old
    /// generation as prepared while the source is about to jump.
    #[test]
    fn prepare_seek_of_the_playing_generation_is_not_ready_while_a_seek_is_held() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 16)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 100, 64)).unwrap();
        render_at(&mut consumer, 0);
        assert!(consumer.seek.held_seek.is_some());
        let next = consumer.next_frame;
        assert!(!consumer.prepare_seek(SourceGeneration(1), next));
        assert!(
            consumer.seek.held_seek.is_some(),
            "the held seek survives preparation"
        );
    }

    #[test]
    fn prepare_rejects_invalid_fixed_ring_shape() {
        assert!(matches!(
            session_ring(config(0, 4, 4)),
            Err(PcmSourceRingError::ZeroChannelCount)
        ));
        assert!(matches!(
            session_ring(config(2, 0, 4)),
            Err(PcmSourceRingError::ZeroQuantumFrames)
        ));
        assert!(matches!(
            session_ring(config(2, 4, 6)),
            Err(PcmSourceRingError::CapacityNotQuantumMultiple)
        ));
    }

    #[test]
    fn host_region_preparation_preserves_resources_and_absolute_ownership() {
        let config = config(1, 4, 8);
        let (_, _, zero_report) =
            PcmSourceRing::prepare_host_region(config, SourceFrame(0), session())
                .expect("zero origin");
        let (_, _, one_report) =
            PcmSourceRing::prepare_host_region(config, SourceFrame(1), session())
                .expect("one origin");
        let (producer, mut consumer, session_report) =
            PcmSourceRing::prepare_host_region(config, SourceFrame(48_123), session())
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
        let report = read_at(&mut consumer, 0, &mut planes);
        assert_eq!(report.active_generation, SourceGeneration(2));
        assert_eq!(output, sought);
    }

    #[test]
    fn retained_played_block_supports_repeat_fanout_and_auto_recycles_once() {
        let (producer, mut consumer, _) = session_ring(config(2, 4, 8)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let left_first = [1.0, 2.0, 3.0, 4.0];
        let right_first = [-1.0, -2.0, -3.0, -4.0];
        host.submit(chunk(1, 0, &[&left_first, &right_first], 4, false))
            .expect("first block");
        let left_final = [5.0, 6.0];
        let right_final = [-5.0, -6.0];
        host.submit(chunk(1, 4, &[&left_final, &right_final], 2, true))
            .expect("short EOF block");

        let first = consumer.begin_block_at(0);
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

        let final_report = consumer.begin_block_at(4);
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
            timeline: test_timeline(4),
            published: session(),
            sources: vec![GraphSourceEntry {
                channel_count: consumer.channel_count(),
                consumer: Some(consumer),
            }]
            .into_boxed_slice(),
            mappings: mappings.into_boxed_slice(),
            quantum_frames: 4,
            block_validity: GraphObservationValidity::CLEAR,
            pending_generation_change: false,
            session_seek: 0,
        }
    }

    /// #1318 gate 3. Red if the graph source driver still folds a held block into the graph-wide
    /// `source_underrun` flag (a planned wait would mark every spectrum window as starved), or if
    /// the fix swallows a true underrun on another source in the same block.
    #[test]
    fn graph_driver_does_not_flag_a_held_block_as_an_underrun() {
        let (producer, held, _) = session_ring(config(1, 4, 8)).expect("held ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        let (_other_producer, starved, _) = session_ring(config(1, 4, 8)).expect("starved ring");
        host.try_seek(seek_at(2, 100, 16)).expect("anchored seek");
        let mut driver = test_driver(held, vec![test_mapping(0)]);
        driver.begin_block(0, 4).expect("held block");
        assert!(
            !driver.observation_validity().source_underrun,
            "a held block is a planned wait"
        );
        assert!(
            !driver.sources[0]
                .consumer
                .as_ref()
                .unwrap()
                .seek
                .held_seek
                .is_none()
        );
        let telemetry = driver.sources[0].consumer.as_ref().unwrap().telemetry();
        assert_eq!((telemetry.held_blocks, telemetry.underrun_events), (1, 0));

        // A second source with nothing queued and no held seek is a true underrun in the same
        // block, and sets the flag.
        driver.sources = vec![
            GraphSourceEntry {
                channel_count: 1,
                consumer: driver.sources[0].consumer.take(),
            },
            GraphSourceEntry {
                channel_count: 1,
                consumer: Some(starved),
            },
        ]
        .into_boxed_slice();
        driver.mappings = vec![
            test_mapping(0),
            SourceGraphTrackMapping {
                source_index: 1,
                ..test_mapping(1)
            },
        ]
        .into_boxed_slice();
        driver.begin_block(4, 4).expect("held and underrun block");
        assert!(driver.observation_validity().source_underrun);
        let telemetry = driver.sources[0].consumer.as_ref().unwrap().telemetry();
        assert_eq!((telemetry.held_blocks, telemetry.underrun_events), (2, 0));
        let telemetry = driver.sources[1].consumer.as_ref().unwrap().telemetry();
        assert_eq!((telemetry.held_blocks, telemetry.underrun_events), (0, 1));
    }

    /// #1318 gate 1. Red if a block waiting for its anchor still counts as an underrun, or if
    /// "held" outlives the anchor (the block at the anchor applied the seek: a host that owes PCM
    /// then and delivers none is starved, not waiting).
    #[test]
    fn a_wait_for_an_anchor_is_held_and_starvation_after_it_is_an_underrun() {
        for delivered in [true, false] {
            let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
            let mut host = producer.into_host_chunk_provider(RATE);
            host.try_seek(seek_at(2, 100, 16)).unwrap();
            if delivered {
                host.submit(chunk(2, 100, &[&[2.0; 4]], 4, false)).unwrap();
            }
            for block in 0..4_u64 {
                let (output, report) = render_at(&mut consumer, 4 * block);
                assert_eq!(output, [0.0; 4]);
                assert!(report.held, "block {block}");
                assert_eq!(report.held_frames, 4);
                assert!(!report.underrun_event, "block {block}");
                assert_eq!(report.underrun_frames, 0);
                assert_eq!(report.cumulative_held_blocks, block + 1);
                assert_eq!(report.cumulative_held_frames, 4 * (block + 1));
                assert_eq!(report.cumulative_underrun_events, 0);
                assert_eq!(report.cumulative_underrun_frames, 0);
            }
            let (output, report) = render_at(&mut consumer, 16);
            assert_eq!(report.active_generation, SourceGeneration(2));
            assert!(report.generation_changed);
            assert!(!report.held, "the seek applied at its anchor");
            assert_eq!(report.held_frames, 0);
            assert_eq!(report.cumulative_held_blocks, 4);
            if delivered {
                assert_eq!(output, [2.0; 4]);
                assert_eq!(report.copied_frames, 4);
                assert!(!report.underrun_event);
                assert_eq!(report.cumulative_underrun_events, 0);
            } else {
                assert_eq!(output, [0.0; 4]);
                assert!(report.underrun_event, "the host owed PCM at the anchor");
                assert_eq!(report.underrun_frames, 4);
                assert_eq!(report.cumulative_underrun_events, 1);
            }
            let telemetry = consumer.telemetry();
            assert_eq!((telemetry.held_blocks, telemetry.held_frames), (4, 16));
        }
    }

    /// #1318 gate 2. Red if held is decided by `held_seek` alone: the playing generation's
    /// queued PCM still plays while a seek is held, and those blocks are neither held nor
    /// underruns.
    fn seek_report(
        generation: u64,
        first_sample: u64,
        source_frame: u64,
        cumulative_underrun_frames: u64,
        cumulative_held_frames: u64,
    ) -> SourceSeekReport {
        SourceSeekReport {
            generation,
            first_sample,
            source_frame,
            cumulative_underrun_frames,
            cumulative_held_frames,
        }
    }

    /// #1316 D3. Red if the report moves at the seek call (the producer's generation) rather than
    /// at the block that applies the seek, if it names the block after that one, if a late
    /// anchored seek reports its frame without the lateness, or if a block that applied nothing
    /// rewrites the sample and frame.
    #[test]
    fn the_seek_report_names_the_block_and_frame_each_generation_started_at() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 16)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        assert_eq!(host.seek_report(), Ok(SourceSeekReport::default()));
        for block in 0..2_u64 {
            host.submit(chunk(1, 4 * block, &[&[1.0; 4]], 4, false))
                .unwrap();
        }
        let _ = render_at(&mut consumer, 0);
        assert_eq!(host.seek_report(), Ok(seek_report(1, 0, 0, 0, 0)));
        let _ = render_at(&mut consumer, 4);
        assert_eq!(host.seek_report(), Ok(seek_report(1, 0, 0, 0, 0)));

        // A plain seek lands at the next block.
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(400),
        })
        .unwrap();
        host.submit(chunk(2, 400, &[&[2.0; 4]], 4, false)).unwrap();
        assert_eq!(
            host.seek_report(),
            Ok(seek_report(1, 0, 0, 0, 0)),
            "not at the call"
        );
        let (output, _) = render_at(&mut consumer, 8);
        assert_eq!(output, [2.0; 4]);
        assert_eq!(host.seek_report(), Ok(seek_report(2, 8, 400, 0, 0)));

        // An anchored seek render reaches one block late starts at its frame plus the lateness.
        host.try_seek(seek_at(3, 96, 12)).unwrap();
        host.submit(chunk(3, 96, &[&[-3.0; 4]], 4, false)).unwrap();
        host.submit(chunk(3, 100, &[&[3.0; 4]], 4, false)).unwrap();
        let (output, _) = render_at(&mut consumer, 16);
        assert_eq!(output, [3.0; 4]);
        assert_eq!(host.seek_report(), Ok(seek_report(3, 16, 100, 0, 0)));
    }

    /// #1350 D3: a held anchored seek that a block reaches off the quantum grid would apply at a
    /// frame no producer block starts at, so the source would starve; a debug build stops there.
    /// Test value: red if the assertion is removed, or computed on a clock other than the block's
    /// source-read sample.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "late by whole quanta")]
    fn an_off_grid_late_anchored_seek_panics_in_a_debug_build() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 0, 64)).unwrap();
        consumer.begin_block_at(65);
    }

    /// #1316 gate 5. Red if the report carries a block's own held or underrun frames instead of
    /// the cumulative counters, or if a counter-only publication rewrites the generation's sample
    /// and frame.
    #[test]
    fn the_seek_report_carries_the_cumulative_held_and_underrun_frames() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.try_seek(seek_at(2, 100, 16)).unwrap();
        for block in 0..4_u64 {
            let _ = render_at(&mut consumer, 4 * block);
            assert_eq!(
                host.seek_report(),
                Ok(seek_report(1, 0, 0, 0, 4 * (block + 1))),
                "held block {block}"
            );
        }
        // The anchor block applies the seek; the host owed PCM and delivered none.
        let _ = render_at(&mut consumer, 16);
        assert_eq!(host.seek_report(), Ok(seek_report(2, 16, 100, 4, 16)));
        let _ = render_at(&mut consumer, 20);
        assert_eq!(host.seek_report(), Ok(seek_report(2, 16, 100, 8, 16)));
    }

    #[test]
    fn queued_pcm_of_the_playing_generation_plays_while_a_seek_is_held() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
        let mut host = producer.into_host_chunk_provider(RATE);
        host.submit(chunk(1, 0, &[&[1.0; 4]], 4, false)).unwrap();
        host.submit(chunk(1, 4, &[&[1.5; 4]], 4, false)).unwrap();
        host.try_seek(seek_at(2, 100, 16)).unwrap();
        for (block, expected) in [(0_u64, 1.0), (1, 1.5)] {
            let (output, report) = render_at(&mut consumer, 4 * block);
            assert_eq!(output, [expected; 4]);
            assert_eq!(report.copied_frames, 4);
            assert!(!report.held && !report.underrun_event, "block {block}");
            assert_eq!(report.cumulative_held_blocks, 0);
        }
        for block in 2..4_u64 {
            let (output, report) = render_at(&mut consumer, 4 * block);
            assert_eq!(output, [0.0; 4]);
            assert!(report.held && !report.underrun_event, "block {block}");
            assert_eq!(report.cumulative_held_blocks, block - 1);
        }
        assert_eq!(consumer.telemetry().underrun_events, 0);
    }

    /// #917 restates the pre-change `..._last_claim_recycles_in_call_...` test for the hold rule:
    /// no claim path releases the played block, the next `begin_block` does, and the producer
    /// keeps its configured one-block depth while the render holds its block. The old incomplete
    /// leg asserted `Full` here, because the render then held one of the ring's only blocks.
    #[test]
    fn graph_driver_retains_played_blocks_to_next_begin_on_every_claim_path() {
        let samples = [1.0, 2.0, 3.0, 4.0];
        let later = [5.0, 6.0, 7.0, 8.0];

        let (producer, consumer, _) = session_ring(config(1, 4, 4)).expect("ring");
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

        let (producer, consumer, _) = session_ring(config(1, 4, 4)).expect("ring");
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

        let (producer, consumer, _) = session_ring(config(1, 4, 4)).expect("ring");
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
        let (producer, _consumer, _) = session_ring(config(2, 4, 4)).expect("ring");
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
        let (producer, mut consumer, report) = session_ring(config(2, 4, 4)).expect("ring");
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
        let report = read_at(&mut consumer, 0, &mut [&mut left, &mut right]);
        assert_eq!(report.copied_frames, 4);
        assert_eq!(left, left_in);
        assert_eq!(right, right_in);
    }

    fn plane_bits(plane: &[f32]) -> Vec<u32> {
        plane.iter().map(|sample| sample.to_bits()).collect()
    }

    #[test]
    fn seek_preparation_preserves_configured_depth_and_prefetches_without_consuming() {
        let (producer, mut consumer, _) = session_ring(config(1, 4, 12)).expect("ring");
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
        let report = read_at(&mut consumer, 0, &mut [&mut output]);
        assert_eq!(output, blocks[0]);
        assert_eq!(report.copied_frames, 4);
        assert_eq!(report.cumulative_read_frames, 4);
        assert_eq!(report.active_generation, SourceGeneration(2));
    }

    /// Gate 1, "at every point" (#917): while the render borrows the played planes, the producer
    /// still admits exactly the configured number of blocks, and filling the ring to
    /// backpressure never touches the borrowed storage. Only the next `begin_block_at` releases it.
    ///
    /// Red mutation: drop the retained block from the allocation -- the producer admits one
    /// block fewer while the planes are borrowed.
    #[test]
    fn played_planes_stay_intact_while_the_producer_fills_its_configured_depth() {
        for configured in 1_u64..=3 {
            let (producer, mut consumer, report) =
                session_ring(config(2, 4, 4 * configured)).expect("ring");
            assert_eq!(report.transfer_block_count, configured);
            let mut host = producer.into_host_chunk_provider(RATE);
            let first_left = [1.0, 2.0, 3.0, 4.0];
            let first_right = [-1.0, -2.0, -3.0, -4.0];
            host.submit(chunk(1, 0, &[&first_left, &first_right], 4, false))
                .expect("first block");
            let played = consumer.begin_block_at(0);
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
            let next = consumer.begin_block_at(4);
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
        let (producer, mut consumer, _) = session_ring(config(2, 4, 4)).expect("ring");
        let mut host = producer.into_host_chunk_provider(RATE);
        assert!(consumer.played_plane(0).is_none(), "nothing played yet");
        assert_copy_channel(&consumer, 0, &[0.0; 4], "before any block");

        // Rotate every allocated block (one configured, one retained) through a poisoned full
        // quantum, so the short block below lands in storage whose tail is not zero.
        let poison = [-0.0, f32::NAN, 1.0e30, -7.5];
        for (index, start) in [0_u64, 4].into_iter().enumerate() {
            host.submit(chunk(1, start, &[&poison, &poison], 4, false))
                .expect("poison block");
            let report = consumer.begin_block_at(start);
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
        let short = consumer.begin_block_at(8);
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
        let after = consumer.begin_block_at(12);
        assert!(after.end_of_region);
        assert_eq!(after.copied_frames, 0);
        assert!(consumer.played_plane(0).is_none());
        assert!(consumer.played_plane(1).is_none());
        assert_copy_channel(&consumer, 1, &[0.0; 4], "past the end of the region");

        // An underrun (in-region, nothing queued) plays nothing either.
        let (_producer, mut starved, _) = session_ring(config(1, 4, 8)).expect("ring");
        let underrun = starved.begin_block_at(0);
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
        let (producer, consumer, _) = session_ring(config(4, 4, 8)).expect("ring");
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
        let (producer, consumer, _) = session_ring(config(1, 4, 4)).expect("ring");
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
        use effect_contract::LatencySamples;
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
            fn retained_bytes(&self) -> u64 {
                0
            }
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
            fn retained_bytes(&self) -> u64 {
                engine::realtime::retained::arc(&self.0)
            }
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
                            bound: effect_contract::NodeTailBound::ZERO,
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
                    effect_bank_processor_bytes: 0,
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
            bind_with(vec![source], mappings(), validity)
        }

        /// Bind a graph plan over `sources` read through `mappings`.
        fn bind_with(
            sources: Vec<SourceGraphSource>,
            mappings: Vec<SourceGraphTrackMapping>,
            validity: &Arc<AtomicU8>,
        ) -> PreparedRenderPlan {
            let set = session_source_set(envelope(), sources, test_timeline(FRAMES), mappings)
                .expect("source set");
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
                seek_runway_blocks: 0,
            };
            let (producer, consumer, resources) = session_ring(config).expect("ring");
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
                retirement_capacity: NonZeroUsize::new(1).expect("one"),
            }
        }

        /// Plan A over the filled ring for `SWAP_BLOCK` blocks through a plan exchange, then the
        /// vacant plan B, given the program `program(identity of A)`, for the rest. Returns every
        /// block, the swap block's carry outcome, and each block's `(underrun, generation)` facts.
        /// What [`swap`] returns: each block's bits, the carry outcome, each block's validity
        /// facts and the source's seek report after each block.
        type SwapRun = (
            Vec<Block>,
            CarryOutcome,
            Vec<(bool, bool)>,
            Vec<SourceSeekReport>,
        );

        /// Renders `BLOCKS` blocks, swapping at `SWAP_BLOCK`.
        fn swap(program: impl FnOnce(u64) -> Option<GraphCarryProgram>) -> SwapRun {
            let validity = Arc::new(AtomicU8::new(0));
            let (host, source) = filled_source();
            let mut predecessor = bind(source, &validity);
            let identity = graph::plan_identity(&mut predecessor).expect("graph plan");
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(predecessor, exchange_config()).expect("exchange");
            let mut blocks = Vec::new();
            let mut facts = Vec::new();
            let mut reports = Vec::new();
            let mut carry = CarryOutcome::NotRequested;
            let mut successor = Some(bind(SourceGraphSource::vacant(2), &validity));
            let mut program = Some(program);
            for block in 0..BLOCKS {
                if block == SWAP_BLOCK {
                    let mut plan = successor.take().expect("successor");
                    if let Some(program) = (program.take().expect("program"))(identity) {
                        graph::install_carry_program(&mut plan, program).expect("install");
                    }
                    assert!(publisher.publish(plan, 0).is_ok());
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
                reports.push(host.seek_report().expect("a quiescent read"));
            }
            (blocks, carry, facts, reports)
        }

        fn carry_source_zero(predecessor: u64) -> GraphCarryProgram {
            GraphCarryProgram {
                predecessor,
                sources: vec![(0, 0)].into_boxed_slice(),
                timeline: false,
            }
        }

        /// Gate 1. Red if the consumer is not moved into the vacant entry at the swap block (the
        /// successor would render silence there) or the move lands a block late.
        #[test]
        fn a_carried_consumer_continues_the_predecessor_audio_gap_free() {
            let reference = reference();
            let (blocks, carry, facts, reports) =
                swap(|identity| Some(carry_source_zero(identity)));
            assert_eq!(blocks, reference);
            assert_eq!(carry, CarryOutcome::Carried);
            // #1316 gate 4: the seek report is the ring's, so it moves with the consumer and
            // reads the same before, at and after the swap block.
            assert!(
                reports.iter().all(|report| *report
                    == SourceSeekReport {
                        generation: 1,
                        ..SourceSeekReport::default()
                    }),
                "{reports:?}"
            );
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
                let (blocks, carry, facts, _) = swap(|identity| {
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
            let (blocks, carry, _, _) = swap(|identity| {
                Some(GraphCarryProgram {
                    predecessor: identity,
                    sources: vec![(0, 1)].into_boxed_slice(),
                    timeline: false,
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
                seek_runway_blocks: 0,
            };
            let (producer, consumer, resources) = session_ring(config).expect("ring");
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
                timeline: false,
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
            let occupied = session_source_set(
                envelope(),
                vec![occupied_source],
                test_timeline(FRAMES),
                mappings(),
            )
            .expect("occupied set");
            let vacant = session_source_set(
                envelope(),
                vec![SourceGraphSource::vacant(2)],
                test_timeline(FRAMES),
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
                session_source_set(
                    envelope(),
                    vec![SourceGraphSource::vacant(1)],
                    test_timeline(FRAMES),
                    mappings()
                ),
                Err(SourceGraphSourceSetError::ChannelIndex)
            ));
        }

        /// Claim `a` reads source 0 and claim `b` reads source 1, both as `(0, 1)`.
        fn two_source_mappings() -> Vec<SourceGraphTrackMapping> {
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
                    source_index: 1,
                    left_channel: 0,
                    right_channel: 1,
                },
            ]
        }

        fn silence() -> Block {
            [0.0_f32.to_bits(); 2 * FRAMES as usize]
        }

        /// Follow-up MINOR 1. Red if the swap block checks and moves one source at a time: a
        /// valid first move would land before the second is refused, so a carry reported as a
        /// mismatch would still play source 0.
        #[test]
        fn a_refused_second_move_moves_nothing() {
            let validity = Arc::new(AtomicU8::new(0));
            let (_first, first) = filled_source();
            let (_second, second) = filled_source();
            let mut predecessor = bind_with(vec![first, second], two_source_mappings(), &validity);
            let identity = graph::plan_identity(&mut predecessor).expect("graph plan");
            let mut successor = bind_with(
                vec![SourceGraphSource::vacant(2), SourceGraphSource::vacant(2)],
                two_source_mappings(),
                &validity,
            );
            graph::install_carry_program(
                &mut successor,
                GraphCarryProgram {
                    predecessor: identity,
                    sources: vec![(0, 0), (1, 5)].into_boxed_slice(),
                    timeline: false,
                },
            )
            .expect("install");
            assert_eq!(
                successor.adopt_predecessor_plan(&mut predecessor),
                CarryOutcome::PredecessorMismatch
            );
            assert_eq!(plan_block(&mut successor), silence());
        }

        /// Follow-up MINOR 2. Red if install checks only the successor index for repeats: two
        /// moves from one predecessor source would both pass the swap-block check, the second
        /// swap would trade two vacancies, and the carry would report success with successor
        /// source 1 silent.
        #[test]
        fn install_refuses_a_repeated_predecessor_index() {
            let validity = Arc::new(AtomicU8::new(0));
            let mut successor = bind_with(
                vec![SourceGraphSource::vacant(2), SourceGraphSource::vacant(2)],
                two_source_mappings(),
                &validity,
            );
            assert_eq!(
                graph::install_carry_program(
                    &mut successor,
                    GraphCarryProgram {
                        predecessor: 1,
                        sources: vec![(0, 0), (1, 0)].into_boxed_slice(),
                        timeline: false,
                    },
                ),
                Err(GraphCarryInstallError::DuplicateSourceIndex)
            );
        }

        /// Follow-up MINOR 3 and NIT. Red if a vacant claim leaves the caller's buffers as they
        /// were (a reused input buffer would replay stale audio) or a vacant entry accepts a seek.
        #[test]
        fn a_vacant_claim_writes_positive_zero_over_dirty_buffers_and_refuses_seek() {
            let [a, _] = inputs();
            let mut driver = SourceGraphSourceSetDriver {
                timeline: test_timeline(FRAMES),
                published: session(),
                sources: vec![GraphSourceEntry {
                    consumer: None,
                    channel_count: 2,
                }]
                .into_boxed_slice(),
                mappings: vec![SourceGraphTrackMapping {
                    node: a,
                    source_index: 0,
                    left_channel: 0,
                    right_channel: 1,
                }]
                .into_boxed_slice(),
                quantum_frames: FRAMES,
                block_validity: GraphObservationValidity::CLEAR,
                pending_generation_change: false,
                session_seek: 0,
            };
            assert!(!driver.can_prepare_source_seek(0));
            assert!(!driver.prepare_source_seek(0, 2, 0));
            driver.begin_block(0, FRAMES).expect("begin");
            let mut left = [f32::NAN; FRAMES as usize];
            let mut right = [-1.0_f32; FRAMES as usize];
            driver
                .copy_track_input(0, &mut left, &mut right)
                .expect("copy");
            assert!(left.iter().chain(&right).all(|word| word.to_bits() == 0));
        }

        /// Follow-up MINOR 7. Red if the swap block stops comparing channel counts (a one-channel
        /// consumer would land under two-channel mappings and fail render) or quantum sizes.
        #[test]
        fn the_swap_block_refuses_other_channel_counts_and_quanta() {
            let validity = Arc::new(AtomicU8::new(0));
            let [a, b] = inputs();
            let config = PcmSourceRingConfig {
                channel_count: 1,
                quantum_frames: QuantumFrames(FRAMES),
                frame_capacity: BLOCKS as u64 * u64::from(FRAMES),
                initial_generation: SourceGeneration(1),
                seek_runway_blocks: 0,
            };
            let (producer, consumer, resources) = session_ring(config).expect("ring");
            let mut host = producer.into_host_chunk_provider(RATE);
            for block in 0..BLOCKS as u64 {
                let planes = frame_samples(block * u64::from(FRAMES));
                host.submit(chunk(
                    1,
                    block * u64::from(FRAMES),
                    &[&planes[0]],
                    FRAMES,
                    false,
                ))
                .expect("source PCM");
            }
            let mono = |node: GraphNodeId, right_channel| SourceGraphTrackMapping {
                node,
                source_index: 0,
                left_channel: 0,
                right_channel,
            };
            let mut predecessor = bind_with(
                vec![SourceGraphSource::new(consumer, resources, 0, 0)],
                vec![mono(a.clone(), 0), mono(b.clone(), 0)],
                &validity,
            );
            let identity = graph::plan_identity(&mut predecessor).expect("graph plan");
            let mut successor = bind_with(
                vec![SourceGraphSource::vacant(2)],
                vec![mono(a, 1), mono(b, 1)],
                &validity,
            );
            graph::install_carry_program(&mut successor, carry_source_zero(identity))
                .expect("install");
            assert_eq!(
                successor.adopt_predecessor_plan(&mut predecessor),
                CarryOutcome::PredecessorMismatch
            );
            assert_eq!(plan_block(&mut successor), silence());

            // The quantum guard, at the driver: plans of one envelope cannot disagree, so only
            // a driver built by hand reaches it.
            let (_host, source) = filled_source();
            let driver =
                |sources: Vec<GraphSourceEntry>, quantum_frames| SourceGraphSourceSetDriver {
                    timeline: test_timeline(quantum_frames),
                    published: session(),
                    sources: sources.into_boxed_slice(),
                    mappings: Vec::new().into_boxed_slice(),
                    quantum_frames,
                    block_validity: GraphObservationValidity::CLEAR,
                    pending_generation_change: false,
                    session_seek: 0,
                };
            let mut predecessor = driver(
                vec![GraphSourceEntry {
                    consumer: source.consumer,
                    channel_count: 2,
                }],
                FRAMES,
            );
            let vacant = || {
                vec![GraphSourceEntry {
                    consumer: None,
                    channel_count: 2,
                }]
            };
            let mut other_quantum = driver(vacant(), 2 * FRAMES);
            assert!(!other_quantum.adopt_sources(&mut predecessor, &[(0, 0)]));
            assert!(other_quantum.sources[0].consumer.is_none());
            let mut same_quantum = driver(vacant(), FRAMES);
            assert!(same_quantum.adopt_sources(&mut predecessor, &[(0, 0)]));
        }

        /// Batch 16a fold M3 (#1641 D3): the timeline moves only from an occupied predecessor
        /// into a vacant successor, and no pairing of the two timelines refuses the source moves.
        ///
        /// Test value: red if a timeline vacant on both sides refuses the carry (every source of
        /// that swap would render silence), or a timeline moves in any other pairing.
        #[test]
        fn the_source_moves_carry_whatever_the_two_timelines_hold() {
            let vacant_timeline = || {
                SourceTimeline::prepare_vacant(QuantumFrames(FRAMES), NonZeroUsize::MIN, session())
                    .expect("vacant timeline")
                    .0
            };
            for (successor_vacant, predecessor_vacant) in
                [(true, true), (true, false), (false, true), (false, false)]
            {
                let pairing = (successor_vacant, predecessor_vacant);
                let (_host, entry) = occupied(2);
                let mut predecessor = swap_driver(vec![entry], FRAMES);
                if predecessor_vacant {
                    predecessor.timeline = vacant_timeline();
                }
                let mut successor = swap_driver(
                    vec![GraphSourceEntry {
                        consumer: None,
                        channel_count: 2,
                    }],
                    FRAMES,
                );
                if successor_vacant {
                    successor.timeline = vacant_timeline();
                }
                assert!(
                    successor.adopt_sources(&mut predecessor, &[(0, 0)]),
                    "{pairing:?}"
                );
                assert!(successor.sources[0].consumer.is_some(), "{pairing:?}");
                let moved = successor_vacant && !predecessor_vacant;
                assert_eq!(
                    successor.timeline.is_vacant(),
                    successor_vacant && !moved,
                    "{pairing:?}"
                );
                assert_eq!(
                    predecessor.timeline.is_vacant(),
                    predecessor_vacant || moved,
                    "{pairing:?}"
                );
            }
        }

        /// A hand-built driver over `entries` at `quantum_frames`.
        fn swap_driver(
            entries: Vec<GraphSourceEntry>,
            quantum_frames: u32,
        ) -> SourceGraphSourceSetDriver {
            SourceGraphSourceSetDriver {
                timeline: test_timeline(quantum_frames),
                published: session(),
                sources: entries.into_boxed_slice(),
                mappings: Vec::new().into_boxed_slice(),
                quantum_frames,
                block_validity: GraphObservationValidity::CLEAR,
                pending_generation_change: false,
                session_seek: 0,
            }
        }

        fn occupied(channel_count: u32) -> (HostChunkProvider, GraphSourceEntry) {
            let (host, source) = filled_source();
            (
                host,
                GraphSourceEntry {
                    consumer: source.consumer,
                    channel_count,
                },
            )
        }

        fn vacant_entry() -> GraphSourceEntry {
            GraphSourceEntry {
                consumer: None,
                channel_count: 2,
            }
        }

        /// Which entries own a consumer.
        fn occupancy(driver: &SourceGraphSourceSetDriver) -> Vec<bool> {
            driver
                .sources
                .iter()
                .map(|entry| entry.consumer.is_some())
                .collect()
        }

        /// #1344 fold m2. Each D4 refusal of `swappable`, through `can_swap_sources` and
        /// `swap_sources`: red if any one condition is dropped (a swap would then move a vacancy,
        /// put a consumer under mappings of another channel count, swap one entry twice so a pair
        /// trades back, index out of range, or cross quanta), or if a refused swap moves anything.
        #[test]
        fn a_source_swap_refuses_every_unswappable_pair_and_moves_nothing() {
            struct Foreign;
            impl GraphPreparedSourceSetDriver for Foreign {
                fn retained_bytes(&self) -> u64 {
                    0
                }
                fn claim_count(&self) -> usize {
                    0
                }
                fn begin_block(
                    &mut self,
                    _first_sample: u64,
                    _frames: u32,
                ) -> Result<(), engine::realtime::RenderError> {
                    Ok(())
                }
                fn copy_track_input(
                    &mut self,
                    _claim_index: usize,
                    _left: &mut [f32],
                    _right: &mut [f32],
                ) -> Result<(), engine::realtime::RenderError> {
                    Ok(())
                }
            }
            let mut hosts = Vec::new();
            let mut set = |entries: Vec<(u32, bool)>, quantum| {
                let entries = entries
                    .into_iter()
                    .map(|(channels, full)| {
                        if full {
                            let (host, entry) = occupied(channels);
                            hosts.push(host);
                            entry
                        } else {
                            GraphSourceEntry {
                                channel_count: channels,
                                ..vacant_entry()
                            }
                        }
                    })
                    .collect();
                swap_driver(entries, quantum)
            };
            let full = (2, true);
            let empty = (2, false);
            let mono = (1, true);
            type Case = (
                &'static str,
                SourceGraphSourceSetDriver,
                SourceGraphSourceSetDriver,
                Vec<(u32, u32)>,
            );
            let cases: Vec<Case> = vec![
                (
                    "own index out of range",
                    set(vec![full], FRAMES),
                    set(vec![full], FRAMES),
                    vec![(1, 0)],
                ),
                (
                    "other index out of range",
                    set(vec![full], FRAMES),
                    set(vec![full], FRAMES),
                    vec![(0, 1)],
                ),
                (
                    "own entry vacant",
                    set(vec![empty], FRAMES),
                    set(vec![full], FRAMES),
                    vec![(0, 0)],
                ),
                (
                    "other entry vacant",
                    set(vec![full], FRAMES),
                    set(vec![empty], FRAMES),
                    vec![(0, 0)],
                ),
                (
                    "channel counts differ",
                    set(vec![full], FRAMES),
                    set(vec![mono], FRAMES),
                    vec![(0, 0)],
                ),
                (
                    "quanta differ",
                    set(vec![full], FRAMES),
                    set(vec![full], 2 * FRAMES),
                    vec![(0, 0)],
                ),
                (
                    "own index repeated",
                    set(vec![full], FRAMES),
                    set(vec![full, full], FRAMES),
                    vec![(0, 0), (0, 1)],
                ),
                (
                    "other index repeated",
                    set(vec![full, full], FRAMES),
                    set(vec![full], FRAMES),
                    vec![(0, 0), (1, 0)],
                ),
            ];
            for (what, mut own, mut other, pairs) in cases {
                let before = (occupancy(&own), occupancy(&other));
                assert!(!own.can_swap_sources(&other, &pairs), "{what}: refused");
                assert!(!own.swap_sources(&mut other, &pairs), "{what}: not swapped");
                assert_eq!(
                    (occupancy(&own), occupancy(&other)),
                    before,
                    "{what}: nothing moved"
                );
            }
            // A driver of another type is refused both ways.
            let mut own = set(vec![full], FRAMES);
            assert!(!own.can_swap_sources(&Foreign, &[(0, 0)]));
            assert!(!own.swap_sources(&mut Foreign, &[(0, 0)]));
            assert_eq!(occupancy(&own), [true]);
            // Not vacuous: a checked pair swaps both entries whole.
            let mut own = set(vec![full, empty], FRAMES);
            let mut other = set(vec![empty, full], FRAMES);
            assert!(own.can_swap_sources(&other, &[(0, 1)]));
            assert!(own.swap_sources(&mut other, &[(0, 1)]));
            assert_eq!(
                (occupancy(&own), occupancy(&other)),
                (vec![true, false], vec![false, true])
            );
            let mut own = set(vec![(2, true), (1, true)], FRAMES);
            let mut other = set(vec![(1, true), (2, false)], FRAMES);
            assert!(own.can_swap_sources(&other, &[(1, 0)]));
            assert!(own.swap_sources(&mut other, &[(1, 0)]));
            assert_eq!(occupancy(&own), [true, true]);
        }

        /// #1344 fold m2. Red if `swap_sources` drops the other set's pending generation change
        /// (a seek prepared on a donated consumer since its last block): the successor's first
        /// block would not report the generation change the consumer already took. An empty
        /// donation takes nothing.
        #[test]
        fn a_source_swap_carries_a_prepared_seek_into_the_next_block() {
            let swapped = |pairs: &[(u32, u32)]| {
                let (_own_host, own_entry) = occupied(2);
                let (producer, consumer, _) = session_ring(PcmSourceRingConfig {
                    channel_count: 2,
                    quantum_frames: QuantumFrames(FRAMES),
                    frame_capacity: BLOCKS as u64 * u64::from(FRAMES),
                    initial_generation: SourceGeneration(1),
                    seek_runway_blocks: 0,
                })
                .expect("ring");
                let mut host = producer.into_host_chunk_provider(RATE);
                let mut own = swap_driver(vec![own_entry], FRAMES);
                let mut other = swap_driver(
                    vec![GraphSourceEntry {
                        consumer: Some(consumer),
                        channel_count: 2,
                    }],
                    FRAMES,
                );
                host.try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(0),
                })
                .expect("seek");
                let planes = frame_samples(0);
                host.submit(chunk(2, 0, &[&planes[0], &planes[1]], FRAMES, false))
                    .expect("generation 2 PCM");
                assert!(other.prepare_source_seek(0, 2, 0), "the seek is ready");
                assert!(own.swap_sources(&mut other, pairs));
                own.begin_block(0, FRAMES).expect("begin");
                own.observation_validity().source_generation_changed
            };
            assert!(swapped(&[(0, 0)]), "the donated seek is reported");
            assert!(!swapped(&[]), "an empty donation takes nothing");
        }
    }

    /// #1320 gate 2: readiness at its exact boundaries, on a two-channel ring of quantum 128
    /// with `k = 3`. Every readiness call is wrapped by [`prime_ready_checked`], which asserts the
    /// call changed nothing a twin that made no call would show.
    mod prime_readiness {
        use super::*;

        const Q: u32 = 128;
        const K: u32 = 3;

        fn ring() -> (HostChunkProvider, PcmSourceConsumer) {
            let (producer, consumer, _) =
                session_ring(config(2, Q, 16 * u64::from(Q))).expect("ring");
            (producer.into_host_chunk_provider(RATE), consumer)
        }

        fn sample(frame: u64, channel: u64) -> f32 {
            #[allow(clippy::cast_precision_loss)]
            let value = (frame * 2 + channel) as f32;
            value
        }

        /// Submit one block of `frames` encoded frames through the validating host path.
        fn submit(
            host: &mut HostChunkProvider,
            generation: u64,
            start: u64,
            frames: u32,
            end: bool,
        ) {
            let planes: [Vec<f32>; 2] = core::array::from_fn(|channel| {
                (0..u64::from(frames))
                    .map(|frame| sample(start + frame, channel as u64))
                    .collect()
            });
            host.submit(chunk(
                generation,
                start,
                &[&planes[0], &planes[1]],
                frames,
                end,
            ))
            .expect("valid block");
        }

        /// Publish one block with no submission check: a gap or a stale block the host path
        /// refuses, as a ring the consumer advanced past would hold it.
        fn push_raw(host: &mut HostChunkProvider, generation: u64, start: u64) {
            let producer = &mut host.producer;
            let mut block = producer.take_recycled_block().expect("free block");
            block.generation = SourceGeneration(generation);
            block.start_frame = SourceFrame(start);
            block.frames = Q;
            block.end_of_region = false;
            assert!(
                producer.data_producer.try_push(block).is_ok(),
                "queue space"
            );
        }

        #[derive(Debug, PartialEq)]
        struct Twin {
            telemetry: SourceConsumerTelemetry,
            next_frame: SourceFrame,
            end_frame: Option<SourceFrame>,
            data_queued: usize,
            recycle_free: usize,
            commands_queued: usize,
            commands_popped: u64,
            data_popped: u64,
            held: Option<(SourceGeneration, SourceFrame, Landing)>,
            current: Option<(SourceGeneration, SourceFrame)>,
            next_source_read_sample: u64,
            seek_report: SourceSeekReport,
        }

        fn twin(consumer: &PcmSourceConsumer) -> Twin {
            Twin {
                telemetry: consumer.telemetry(),
                next_frame: consumer.next_frame,
                end_frame: consumer.end_frame,
                data_queued: consumer.data_consumer.available_at_entry(),
                recycle_free: consumer.recycle_producer.available_capacity(),
                commands_queued: consumer.command_consumer.available_at_entry(),
                commands_popped: consumer.command_consumer.success_count(),
                data_popped: consumer.data_consumer.success_count(),
                held: consumer
                    .seek
                    .held_seek
                    .map(|held| (held.generation, held.frame, held.landing)),
                current: consumer
                    .current
                    .as_ref()
                    .map(|block| (block.generation, block.start_frame)),
                next_source_read_sample: consumer.next_source_read_sample,
                seek_report: consumer.published_seek_report,
            }
        }

        fn prime_ready_checked(consumer: &PcmSourceConsumer, first_sample: u64, k: u32) -> bool {
            let before = twin(consumer);
            let ready = consumer.prime_ready(first_sample, k);
            assert_eq!(twin(consumer), before, "readiness changed the consumer");
            ready
        }

        /// One ordinary block at `first_sample`.
        fn ordinary(consumer: &mut PcmSourceConsumer, first_sample: u64) -> SourceReadReport {
            let report = consumer.begin_block_at(first_sample);
            consumer.end_block();
            report
        }

        #[test]
        fn k_plus_one_contiguous_blocks_are_ready_and_k_are_not() {
            let (mut host, consumer) = ring();
            for block in 0..3 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            assert!(!prime_ready_checked(&consumer, 0, K), "only k blocks");
            assert!(
                prime_ready_checked(&consumer, 0, K - 1),
                "k blocks for k - 1"
            );
            submit(&mut host, 1, 3 * u64::from(Q), Q, false);
            assert!(prime_ready_checked(&consumer, 0, K), "k + 1 blocks");
            assert!(!prime_ready_checked(&consumer, 0, K + 1), "one block past");
        }

        #[test]
        fn a_held_current_block_is_block_zero() {
            let (mut host, mut consumer) = ring();
            for block in 0..3 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            // `prepare_seek` of the playing generation pops block 0 into `current`.
            assert!(consumer.prepare_seek(SourceGeneration(1), SourceFrame(0)));
            assert!(consumer.current.is_some());
            assert!(
                !prime_ready_checked(&consumer, 0, K),
                "current plus two queued"
            );
            assert!(prime_ready_checked(&consumer, 0, K - 1));
            submit(&mut host, 1, 3 * u64::from(Q), Q, false);
            assert!(
                prime_ready_checked(&consumer, 0, K),
                "current plus three queued"
            );
        }

        #[test]
        fn a_region_ending_inside_the_window_is_ready_only_with_its_end_block() {
            let (mut host, consumer) = ring();
            submit(&mut host, 1, 0, Q, false);
            assert!(
                !prime_ready_checked(&consumer, 0, K),
                "the end block is missing"
            );
            submit(&mut host, 1, u64::from(Q), Q / 2, true);
            assert!(
                prime_ready_checked(&consumer, 0, K),
                "a short end block at j = 1"
            );
            assert!(
                prime_ready_checked(&consumer, 0, K + 8),
                "the end holds past it"
            );
        }

        #[test]
        fn a_short_block_without_its_end_is_not_ready() {
            let (mut host, consumer) = ring();
            submit(&mut host, 1, 0, Q, false);
            let producer = &mut host.producer;
            let mut block = producer.take_recycled_block().expect("free block");
            block.generation = SourceGeneration(1);
            block.start_frame = SourceFrame(u64::from(Q));
            block.frames = Q / 2;
            block.end_of_region = false;
            assert!(
                producer.data_producer.try_push(block).is_ok(),
                "queue space"
            );
            assert!(!prime_ready_checked(&consumer, 0, K));
            assert!(
                prime_ready_checked(&consumer, 0, 0),
                "block 0 alone is whole"
            );
        }

        #[test]
        fn a_one_quantum_gap_at_j_two_is_not_ready() {
            let (mut host, consumer) = ring();
            push_raw(&mut host, 1, 0);
            push_raw(&mut host, 1, u64::from(Q));
            push_raw(&mut host, 1, 3 * u64::from(Q));
            push_raw(&mut host, 1, 4 * u64::from(Q));
            assert!(!prime_ready_checked(&consumer, 0, K), "a gap at j = 2");
            assert!(
                prime_ready_checked(&consumer, 0, 1),
                "the window before the gap"
            );
        }

        #[test]
        fn a_queued_plain_seek_is_not_ready_until_an_ordinary_block_applies_it() {
            let (mut host, mut consumer) = ring();
            for block in 0..4 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            assert!(prime_ready_checked(&consumer, 0, K));
            host.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(5_000),
            })
            .expect("seek");
            assert!(!prime_ready_checked(&consumer, 0, K), "a queued seek");
            for block in 0..5 {
                submit(&mut host, 2, 5_000 + block * u64::from(Q), Q, false);
            }
            assert!(!prime_ready_checked(&consumer, 0, K), "still queued");
            let report = ordinary(&mut consumer, 0);
            assert!(report.generation_changed && report.copied_frames == Q);
            assert!(
                prime_ready_checked(&consumer, u64::from(Q), K),
                "k + 1 new blocks"
            );
            assert!(!prime_ready_checked(&consumer, u64::from(Q), K + 1));
        }

        #[test]
        fn a_queued_anchored_seek_is_not_ready_until_it_applies() {
            let (mut host, mut consumer) = ring();
            for block in 0..4 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            // Anchored at the second block: the first block observes and holds it, the second
            // applies it.
            host.try_seek(seek_at(2, 5_000, u64::from(Q)))
                .expect("seek");
            assert!(
                !prime_ready_checked(&consumer, 0, K),
                "a queued anchored seek"
            );
            for block in 0..5 {
                submit(&mut host, 2, 5_000 + block * u64::from(Q), Q, false);
            }
            let first = ordinary(&mut consumer, 0);
            assert!(!first.generation_changed && first.copied_frames == Q);
            assert!(consumer.seek.held_seek.is_some());
            assert!(
                !prime_ready_checked(&consumer, u64::from(Q), K),
                "held, anchored at the next block"
            );
            let second = ordinary(&mut consumer, u64::from(Q));
            assert!(second.generation_changed && second.copied_frames == Q);
            assert!(prime_ready_checked(&consumer, 2 * u64::from(Q), K));
        }

        #[test]
        fn a_far_anchored_seek_once_held_leaves_the_window_ready() {
            let (mut host, mut consumer) = ring();
            for block in 0..5 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            host.try_seek(seek_at(2, 5_000, 64 * u64::from(Q)))
                .expect("seek");
            assert!(!prime_ready_checked(&consumer, 0, K), "queued");
            ordinary(&mut consumer, 0);
            assert!(consumer.seek.held_seek.is_some());
            assert!(
                prime_ready_checked(&consumer, u64::from(Q), K),
                "held past the window"
            );
        }

        #[test]
        fn a_held_seek_anchored_inside_the_window_or_late_is_not_ready() {
            let (mut host, mut consumer) = ring();
            for block in 0..6 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            ordinary(&mut consumer, 0);
            let first_sample = u64::from(Q);
            let window_end = first_sample + u64::from(K + 1) * u64::from(Q);
            assert!(prime_ready_checked(&consumer, first_sample, K));
            let hold = |consumer: &mut PcmSourceConsumer, anchor_sample| {
                consumer.seek.held_seek = Some(HeldSeek {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(9_000),
                    landing: Landing::Anchor(anchor_sample),
                });
            };
            hold(&mut consumer, window_end - 1);
            assert!(
                !prime_ready_checked(&consumer, first_sample, K),
                "last frame inside"
            );
            hold(&mut consumer, window_end);
            assert!(
                prime_ready_checked(&consumer, first_sample, K),
                "first frame past"
            );
            // Held because the last block began at `first_sample - q`, before the anchor; it
            // applies late at `first_sample`.
            hold(&mut consumer, first_sample - 1);
            assert!(!prime_ready_checked(&consumer, first_sample, K), "late");
        }

        #[test]
        fn stale_blocks_ahead_are_not_ready_until_ordinary_blocks_discard_them() {
            let (mut host, mut consumer) = ring();
            // The consumer reads generation 2 from frame 0; two generation-1 blocks are queued
            // ahead of its PCM.
            consumer.seek.active_generation = SourceGeneration(2);
            push_raw(&mut host, 1, 0);
            push_raw(&mut host, 1, u64::from(Q));
            for block in 0..5 {
                push_raw(&mut host, 2, block * u64::from(Q));
            }
            assert!(!prime_ready_checked(&consumer, 0, K), "stale blocks ahead");
            let report = ordinary(&mut consumer, 0);
            assert_eq!(report.copied_frames, Q);
            assert_eq!(consumer.telemetry().stale_generation_discard_count, 2);
            assert!(prime_ready_checked(&consumer, u64::from(Q), K), "discarded");
        }

        #[test]
        fn a_block_behind_the_read_frame_is_not_ready() {
            let (mut host, mut consumer) = ring();
            // An underrun moved the read frame a quantum past the host's next block.
            ordinary(&mut consumer, 0);
            for block in 0..5 {
                submit(&mut host, 1, block * u64::from(Q), Q, false);
            }
            assert!(
                !prime_ready_checked(&consumer, u64::from(Q), K),
                "block 0 is behind"
            );
            ordinary(&mut consumer, u64::from(Q));
            assert!(prime_ready_checked(&consumer, 2 * u64::from(Q), K - 1));
        }
    }

    /// #1640 gate 1: the timeline consumer and producer, at quantum 128.
    mod timeline {
        use super::*;

        const Q: u32 = 128;
        const QU: u64 = Q as u64;

        fn timeline(history_blocks: usize) -> (TimelineProducer, TimelineConsumer) {
            let (producer, consumer, _) = SourceTimeline::prepare(
                QuantumFrames(Q),
                NonZeroUsize::new(history_blocks).unwrap(),
                session(),
            )
            .unwrap();
            (producer, consumer)
        }

        fn seek(generation: u64, frame: u64) -> SourceCommand {
            SourceCommand::Seek {
                generation: SourceGeneration(generation),
                frame: SourceFrame(frame),
            }
        }

        /// Run blocks `from..to` (block indices) on the source-read clock; return the timeline
        /// value each block started at.
        fn run(consumer: &mut TimelineConsumer, from: u64, to: u64) -> Vec<u64> {
            (from..to)
                .map(|block| {
                    consumer.begin_block_at(block * QU);
                    let start = consumer.timeline_at(block * QU);
                    assert_eq!(consumer.position(), u64::try_from(start).unwrap() + QU);
                    u64::try_from(start).unwrap()
                })
                .collect()
        }

        /// Begin one block at each source-read sample of `samples`, in order.
        fn blocks_at(consumer: &mut TimelineConsumer, samples: &[u64]) {
            for &sample in samples {
                consumer.begin_block_at(sample);
            }
        }

        /// The span's value at sample `i` of its node block.
        fn span_value(span: TimelineSpan, i: u32) -> i64 {
            match span.step {
                Some(step) if i >= step.offset => step.timeline + i64::from(i - step.offset),
                _ => span.start + i64::from(i),
            }
        }

        /// #1647a (R17-5): the signed reader at the limits. A node block may start before
        /// source-read sample 0 (`r + ΣP < a(n)`) and is read back from the oldest entry, so a
        /// fresh plan's node of arrival 300 reads `-300` at render sample 0, and a block that
        /// crosses sample 0 has no step. First samples below `i64::MIN`, at `i64::MIN`, at 0, at
        /// `i64::MAX`, above it (up to `i128::MAX`), and blocks that cross each of these and the
        /// `u64` end of the source-read clock, give saturated `i64` values and no panic.
        ///
        /// Test value: red if the signed first sample wraps, panics or saturates a value inside
        /// the `i64` range, or if a block that crosses sample 0 reports a step.
        #[test]
        fn the_signed_reader_extrapolates_back_and_saturates_at_the_limits() {
            let (_producer, mut consumer) = timeline(4);
            blocks_at(&mut consumer, &[0]);
            let plain = |start| TimelineSpan { start, step: None };
            assert_eq!(consumer.timeline_block(-300, Q), plain(-300));
            assert_eq!(consumer.timeline_block(-40, Q), plain(-40));
            assert_eq!(consumer.timeline_block(0, Q), plain(0));
            let span = consumer.timeline_block(-40, Q);
            for i in 0..Q {
                let sample = -40 + i64::from(i);
                assert_eq!(span_value(span, i), sample, "sample {i} across 0");
            }
            let min = i128::from(i64::MIN);
            let max = i128::from(i64::MAX);
            for (first, start) in [
                (i128::MIN, i64::MIN),
                (min - 1, i64::MIN),
                (min, i64::MIN),
                (min + 40, i64::MIN + 40),
                (max - 40, i64::MAX - 40),
                (max, i64::MAX),
                (max + 1, i64::MAX),
                (i128::from(u64::MAX), i64::MAX),
                (i128::MAX - 40, i64::MAX),
                (i128::MAX, i64::MAX),
            ] {
                assert_eq!(consumer.timeline_block(first, Q), plain(start), "{first}");
            }
            // A history at the `u64` end of the source-read clock: an entry whose block ends
            // past `u64::MAX` and node blocks that cross that end.
            let (_producer, mut end) = timeline(4);
            blocks_at(&mut end, &[u64::MAX - 100]);
            assert_eq!(end.timeline_at(u64::MAX), 100);
            assert_eq!(end.timeline_block(i128::from(u64::MAX) - 40, Q), plain(60));
            assert_eq!(end.timeline_block(i128::from(u64::MAX), Q), plain(100));
            assert_eq!(end.timeline_block(i128::from(u64::MAX) + 1, Q), plain(101));
            assert_eq!(end.timeline_block(i128::MAX, Q), plain(i64::MAX));
            assert_eq!(end.timeline_block(i128::MIN, Q), plain(i64::MIN));
            // Sample -1 lies `u64::MAX - 99` before the entry: past the `i64` range, saturated.
            assert_eq!(end.timeline_block(-1, Q), plain(i64::MIN));
        }

        /// Batch 16a fold 2, M1 sweep: `K` 1 to 5, a step-back of 64, 128, 200, 256 or 384
        /// samples after four plain blocks, and a node block from every fourth sample.
        ///
        /// Test value: red if any node block with at most one step disagrees with `timeline_at`
        /// on any sample (a missed or a misplaced step, whatever entry is the oldest).
        #[test]
        fn every_node_block_with_one_step_agrees_with_timeline_at_after_a_step_back() {
            for history in 1..=5 {
                for back in [64, 128, 200, 256, 384] {
                    let (_producer, mut consumer) = timeline(history);
                    blocks_at(
                        &mut consumer,
                        &[0, QU, 2 * QU, 3 * QU, 4 * QU - back, 5 * QU - back],
                    );
                    for first in (0..6 * QU).step_by(4) {
                        let steps = (0..Q)
                            .filter(|&i| {
                                let sample = first + u64::from(i);
                                sample > 0
                                    && consumer.timeline_at(sample)
                                        != consumer.timeline_at(sample - 1) + 1
                            })
                            .count();
                        if steps > 1 {
                            continue;
                        }
                        let span = consumer.timeline_block(i128::from(first), Q);
                        for i in 0..Q {
                            assert_eq!(
                                span_value(span, i),
                                consumer.timeline_at(first + u64::from(i)),
                                "K {history}, back {back}, block from {first}, sample {i}"
                            );
                        }
                    }
                }
            }
        }

        /// Batch 16a fold MINOR 4, `K` of 3 and 4: the reader's search order.
        ///
        /// Test value: red if `timeline_at` searches the history oldest first (an overlap reads
        /// the older entry), if `timeline_block` reports any step but the first of two, or if an
        /// eviction does not update the oldest entry's expected timeline (a plain oldest entry
        /// would read as a seek).
        #[test]
        fn the_reader_reads_the_newest_overlap_and_reports_the_first_step() {
            let (mut producer, mut consumer) = timeline(4);
            blocks_at(&mut consumer, &[0, QU, 2 * QU]);
            producer.try_seek(seek(2, 10_000)).unwrap();
            // A seek at 384, then a step-back of 64 samples: the history holds (256, 256),
            // (384, 10_000), (512, 10_128) and (448, 10_256).
            blocks_at(&mut consumer, &[3 * QU, 4 * QU, 448]);
            assert_eq!(
                consumer.timeline_at(460),
                10_268,
                "the newest overlapping entry"
            );
            assert_eq!(consumer.timeline_at(520), 10_328);
            assert_eq!(
                consumer.timeline_at(580),
                10_196,
                "after the newest entry ends"
            );
            // Two steps in one node block: the seek at 384, then the step-back at 448.
            assert_eq!(
                consumer.timeline_block(380, Q),
                TimelineSpan {
                    start: 380,
                    step: Some(TimelineStep {
                        offset: 4,
                        timeline: 10_000,
                    }),
                }
            );
            assert_eq!(
                consumer.timeline_block(400, Q).step,
                Some(TimelineStep {
                    offset: 48,
                    timeline: 10_256,
                })
            );

            // Evictions keep the oldest entry's expectation: a plain oldest entry is no step.
            let (_producer, mut plain) = timeline(3);
            run(&mut plain, 0, 6);
            assert_eq!(plain.timeline_block(i128::from(3 * QU), Q).step, None);
            assert_eq!(plain.timeline_block(i128::from(3 * QU - 40), Q).step, None);
        }

        /// Batch 16a fold MINOR 3: a vacant timeline that no carry fills (#1641 D3).
        ///
        /// Test value: red if a vacant timeline stands still, starts anywhere but sample 0, or
        /// records no history (its reader would extrapolate from a position it never held).
        #[test]
        fn a_vacant_timeline_never_adopted_advances_from_zero() {
            let (mut consumer, _) = SourceTimeline::prepare_vacant(
                QuantumFrames(Q),
                NonZeroUsize::new(2).unwrap(),
                session(),
            )
            .unwrap();
            assert!(consumer.is_vacant());
            assert!(consumer.prime_ready(0, 3), "nothing is queued");
            assert_eq!(run(&mut consumer, 0, 4), vec![0, 128, 256, 384]);
            assert_eq!(consumer.timeline_at(3 * QU - 1), 383);
            assert_eq!(consumer.active_generation(), SourceGeneration(1));
        }

        #[test]
        fn an_anchored_seek_is_held_until_its_anchor_and_applies_with_its_lateness() {
            let anchor = 4 * QU;
            // On time: held while the block sample is below the anchor, the timeline advancing.
            let (mut producer, mut consumer) = timeline(4);
            producer
                .try_seek(SourceCommand::SeekAt {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(50_000),
                    anchor_sample: anchor,
                })
                .unwrap();
            assert_eq!(run(&mut consumer, 0, 4), vec![0, 128, 256, 384]);
            assert_eq!(consumer.active_generation(), SourceGeneration(1));
            assert!(consumer.seek.held_seek.is_some());
            assert_eq!(run(&mut consumer, 4, 6), vec![50_000, 50_128]);
            assert_eq!(consumer.active_generation(), SourceGeneration(2));

            // Observed late at anchor + 256: it applies at frame + 256.
            let (mut producer, mut consumer) = timeline(4);
            producer
                .try_seek(SourceCommand::SeekAt {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(50_000),
                    anchor_sample: anchor,
                })
                .unwrap();
            consumer.begin_block_at(anchor + 256);
            assert_eq!(consumer.timeline_at(anchor + 256), 50_256);
        }

        #[test]
        fn a_newer_seek_plain_or_anchored_replaces_a_held_one() {
            let held = |generation: u64, frame: u64, anchor_sample: u64| SourceCommand::SeekAt {
                generation: SourceGeneration(generation),
                frame: SourceFrame(frame),
                anchor_sample,
            };
            // Anchored over anchored: the first anchor passes with nothing applied.
            let (mut producer, mut consumer) = timeline(4);
            producer.try_seek(held(2, 20_000, 2 * QU)).unwrap();
            consumer.begin_block_at(0);
            producer.try_seek(held(3, 30_000, 4 * QU)).unwrap();
            assert_eq!(run(&mut consumer, 1, 4), vec![128, 256, 384]);
            assert_eq!(run(&mut consumer, 4, 5), vec![30_000]);
            assert_eq!(consumer.active_generation(), SourceGeneration(3));

            // Plain over anchored: applies at once and the held seek never applies.
            let (mut producer, mut consumer) = timeline(4);
            producer.try_seek(held(2, 20_000, 3 * QU)).unwrap();
            consumer.begin_block_at(0);
            producer.try_seek(seek(3, 7_000)).unwrap();
            assert_eq!(run(&mut consumer, 1, 5), vec![7_000, 7_128, 7_256, 7_384]);
            assert!(consumer.seek.held_seek.is_none());
            assert_eq!(consumer.active_generation(), SourceGeneration(3));
        }

        #[test]
        fn the_history_reads_both_sides_of_a_seek_and_extrapolates_below_its_oldest_entry() {
            let (mut producer, mut consumer) = timeline(4);
            run(&mut consumer, 0, 6);
            producer.try_seek(seek(2, 10_000)).unwrap();
            run(&mut consumer, 6, 8);
            // The window holds blocks 4..8: 512, 640, then the seek, 10_000, 10_128.
            assert_eq!(consumer.timeline_at(6 * QU - 1), 767);
            assert_eq!(consumer.timeline_at(6 * QU), 10_000);
            assert_eq!(consumer.timeline_at(7 * QU + 5), 10_133);
            // Below the oldest entry (block 4): back from it.
            assert_eq!(consumer.timeline_at(4 * QU - 1), 511);
            assert_eq!(consumer.timeline_at(0), 0);

            // A fresh plan's first block reads negative node times before it.
            let (_producer, mut fresh) = timeline(2);
            fresh.begin_block_at(256);
            assert_eq!(fresh.timeline_at(256 - 40), -40);
        }

        #[test]
        fn a_node_block_reports_the_step_of_a_seek_inside_it_or_at_its_first_sample() {
            let (mut producer, mut consumer) = timeline(4);
            run(&mut consumer, 0, 3);
            producer.try_seek(seek(2, 10_000)).unwrap();
            run(&mut consumer, 3, 5);
            // Straddling: the node block starts 40 samples before the seek's block.
            assert_eq!(
                consumer.timeline_block(i128::from(3 * QU - 40), Q),
                TimelineSpan {
                    start: 3 * 128 - 40,
                    step: Some(TimelineStep {
                        offset: 40,
                        timeline: 10_000,
                    }),
                }
            );
            // Exactly at the seek's block: the step is at offset 0.
            assert_eq!(
                consumer.timeline_block(i128::from(3 * QU), Q),
                TimelineSpan {
                    start: 10_000,
                    step: Some(TimelineStep {
                        offset: 0,
                        timeline: 10_000,
                    }),
                }
            );
            // A plain advance is no step, inside or at the first sample.
            assert_eq!(
                consumer.timeline_block(i128::from(4 * QU - 40), Q),
                TimelineSpan {
                    start: 10_088,
                    step: None,
                }
            );
            assert_eq!(consumer.timeline_block(i128::from(2 * QU), Q).step, None);
        }

        #[test]
        fn a_seek_in_the_oldest_entry_is_still_a_step() {
            let (mut producer, mut consumer) = timeline(2);
            run(&mut consumer, 0, 2);
            producer.try_seek(seek(2, 9_000)).unwrap();
            // Block 2 seeks; block 3 evicts block 1, so block 2 becomes the oldest entry.
            run(&mut consumer, 2, 4);
            assert_eq!(
                consumer.timeline_block(i128::from(2 * QU), Q).step,
                Some(TimelineStep {
                    offset: 0,
                    timeline: 9_000,
                })
            );
            assert_eq!(consumer.timeline_block(i128::from(3 * QU), Q).step, None);
        }

        #[test]
        fn copying_a_history_keeps_its_newest_entries_in_order() {
            let (mut producer, mut large) = timeline(6);
            run(&mut large, 0, 4);
            producer.try_seek(seek(2, 10_000)).unwrap();
            run(&mut large, 4, 8);
            let snapshot = |consumer: &TimelineConsumer| {
                (0..consumer.len)
                    .rev()
                    .map(|age| consumer.entry(age).unwrap())
                    .map(|entry| (entry.source_read_sample, entry.timeline))
                    .collect::<Vec<_>>()
            };
            // From a larger K: the newest three, oldest first, and the seek still a step.
            let (_p, mut small) = timeline(3);
            run(&mut small, 0, 1);
            small.copy_history_from(&large);
            assert_eq!(
                snapshot(&small),
                vec![(5 * QU, 10_128), (6 * QU, 10_256), (7 * QU, 10_384)]
            );
            assert_eq!(small.timeline_at(5 * QU - 1), 10_127);
            assert_eq!(small.timeline_block(i128::from(5 * QU), Q).step, None);
            // From a smaller K: all three, in order, then the copy keeps recording after them.
            let (_p, mut larger) = timeline(5);
            larger.copy_history_from(&small);
            assert_eq!(snapshot(&larger), snapshot(&small));
            // The copy's oldest entry is a plain advance from the entry before it.
            assert!(!larger.is_seek_entry(2));
            larger.begin_block_at(8 * QU);
            assert_eq!(snapshot(&larger).len(), 4);
            // A copy that keeps the seek's entry as its oldest still reports it.
            let (_p, mut five) = timeline(5);
            five.copy_history_from(&large);
            assert_eq!(snapshot(&five).first(), Some(&(3 * QU, 384)));
            let (_p, mut four) = timeline(4);
            four.copy_history_from(&large);
            assert_eq!(snapshot(&four).first(), Some(&(4 * QU, 10_000)));
            assert_eq!(
                four.timeline_block(i128::from(4 * QU), Q).step,
                Some(TimelineStep {
                    offset: 0,
                    timeline: 10_000,
                })
            );
        }

        #[test]
        fn the_resource_report_states_each_row() {
            let report =
                SourceTimeline::resource_report(QuantumFrames(Q), NonZeroUsize::new(5).unwrap())
                    .unwrap();
            let ring = PcmSourceRing::resource_report(config(1, Q, QU)).unwrap();
            assert_eq!(report.history_blocks, 5);
            assert_eq!(report.history_bytes, 80);
            assert_eq!(report.command_queue_bytes, ring.command_queue_bytes);
            assert_eq!(
                report.command_queue_largest_allocation_bytes,
                largest_queue_allocation::<SourceCommand>(NonZeroUsize::MIN).unwrap()
            );
            assert!(report.command_queue_largest_allocation_bytes <= report.command_queue_bytes);
            assert_eq!(
                SourceTimeline::resource_report(QuantumFrames(0), NonZeroUsize::MIN),
                Err(PcmSourceRingError::ZeroQuantumFrames)
            );
        }

        /// #1640 D1 on the PCM side: `check_seek` forwards to the shared seek producer, agrees
        /// with `try_seek` and pushes nothing.
        #[test]
        fn a_source_check_seek_agrees_with_its_try_seek_and_pushes_nothing() {
            let (producer, mut consumer, _) = session_ring(config(1, 4, 8)).unwrap();
            let mut host = producer.into_host_chunk_provider(RATE);
            assert_eq!(
                host.check_seek(seek_at(2, 100, 6)),
                Err(SourceSeekError::AnchorUnaligned)
            );
            assert_eq!(host.check_seek(seek(2, 100)), Ok(()));
            assert_eq!(host.telemetry().active_generation, SourceGeneration(1));
            consumer.begin_block_at(0);
            assert_eq!(consumer.telemetry().active_generation, SourceGeneration(1));
            host.try_seek(seek(2, 100)).unwrap();
            assert!(matches!(
                host.check_seek(seek(3, 0)),
                Err(SourceSeekError::Backpressure { .. })
            ));
            // A source seek has no frame limit.
            consumer.begin_block_at(4);
            assert_eq!(host.check_seek(seek(3, u64::MAX)), Ok(()));
        }
    }

    /// #1642 B16-R2: the publish fence a session seek is made visible to render through.
    mod published {
        use super::*;

        fn seek(generation: u64, frame: u64) -> SourceCommand {
            SourceCommand::Seek {
                generation: SourceGeneration(generation),
                frame: SourceFrame(frame),
            }
        }

        /// A set of one mono source and an occupied timeline at quantum 4, fenced by `word`, with
        /// both producers sharing it.
        fn fenced(
            word: &PublishedGeneration,
        ) -> (
            HostChunkProvider,
            TimelineProducer,
            SourceGraphSourceSetDriver,
        ) {
            let (producer, consumer, _) =
                PcmSourceRing::prepare_host_region(config(1, 4, 16), SourceFrame(0), word.clone())
                    .expect("ring");
            let (timeline_producer, timeline, _) =
                SourceTimeline::prepare(QuantumFrames(4), NonZeroUsize::MIN, word.clone())
                    .expect("timeline");
            let driver = SourceGraphSourceSetDriver {
                timeline,
                published: word.clone(),
                sources: vec![GraphSourceEntry {
                    channel_count: consumer.channel_count(),
                    consumer: Some(consumer),
                }]
                .into_boxed_slice(),
                mappings: vec![test_mapping(0)].into_boxed_slice(),
                quantum_frames: 4,
                block_validity: GraphObservationValidity::CLEAR,
                pending_generation_change: false,
                session_seek: 0,
            };
            (
                producer.into_host_chunk_provider(RATE),
                timeline_producer,
                driver,
            )
        }

        fn telemetry(driver: &SourceGraphSourceSetDriver) -> SourceConsumerTelemetry {
            driver.sources[0]
                .consumer
                .as_ref()
                .expect("occupied")
                .telemetry()
        }

        /// Test value: red if a consumer pops a command above the block's published generation (a
        /// session seek could split across two blocks), drops it or its PCM instead of leaving
        /// them queued, or does not take both at the first block after the publication.
        #[test]
        fn an_unpublished_seek_and_its_pcm_wait_in_their_slots_until_published() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut host, mut timeline, mut driver) = fenced(&word);
            driver.begin_block(0, 4).expect("block 0");
            let mut session_seek = word.session_seek(2, None).expect("open");
            session_seek
                .push_source(&mut host, 100)
                .expect("source push");
            session_seek
                .push_timeline(&mut timeline, 1_000)
                .expect("timeline push");
            let plane = [100.0_f32, 101.0, 102.0, 103.0];
            host.submit(chunk(2, 100, &[&plane], 4, false))
                .expect("generation 2 PCM");
            assert_eq!(word.generation(), SourceGeneration(1), "nothing published");

            driver.begin_block(4, 4).expect("block 1");
            let held = telemetry(&driver);
            assert_eq!(held.active_generation, SourceGeneration(1));
            assert_eq!(held.cumulative_read_frames, 0);
            assert_eq!(held.stale_generation_discard_count, 0, "the PCM waits");
            assert_eq!(driver.timeline_at(4), Some(4), "the timeline runs on");
            // Both commands still hold their slots.
            assert!(matches!(
                host.check_seek(seek(3, 0)),
                Err(SourceSeekError::Backpressure { .. })
            ));
            assert!(matches!(
                timeline.check_seek(seek(3, 0)),
                Err(SourceSeekError::Backpressure { .. })
            ));

            session_seek.publish();
            assert_eq!(word.generation(), SourceGeneration(2));
            driver.begin_block(8, 4).expect("block 2");
            let applied = telemetry(&driver);
            assert_eq!(applied.active_generation, SourceGeneration(2));
            assert_eq!(applied.cumulative_read_frames, 4, "the waiting PCM plays");
            assert_eq!(applied.stale_generation_discard_count, 0);
            assert_eq!(driver.timeline_at(8), Some(1_000));
            assert!(driver.observation_validity().source_generation_changed);
        }

        /// Test value: red if a seek on its own does not publish its generation (render would
        /// hold it forever) or publishing lowers the word.
        #[test]
        fn a_seek_on_its_own_publishes_its_generation_and_the_word_never_falls() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut host, mut timeline, mut driver) = fenced(&word);
            host.try_seek(seek(5, 40)).expect("source seek");
            assert_eq!(word.generation(), SourceGeneration(5));
            timeline.try_seek(seek(3, 700)).expect("timeline seek");
            assert_eq!(word.generation(), SourceGeneration(5), "max, not last");
            driver.begin_block(0, 4).expect("block 0");
            assert_eq!(telemetry(&driver).active_generation, SourceGeneration(5));
            assert_eq!(driver.timeline_at(0), Some(700));
        }

        /// Test value: red if a session seek opens at or below its word, publishes when nothing
        /// was pushed (a refused seek would make its generation stale), or pushes to a producer
        /// of another session (render would never observe that seek under this word).
        #[test]
        fn a_session_seek_opens_above_its_word_and_pushes_only_its_own_producers() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut host, mut timeline, _driver) = fenced(&word);
            assert_eq!(
                word.session_seek(0, None).err(),
                Some(SourceSeekError::GenerationZero)
            );
            word.publish(SourceGeneration(4));
            assert_eq!(
                word.session_seek(4, None).err(),
                Some(SourceSeekError::GenerationNotStrictlyIncreasing {
                    active: SourceGeneration(4),
                    requested: SourceGeneration(4),
                })
            );
            assert_eq!(host.check_seek(seek(4, 0)), Ok(()), "the producer takes 4");

            let other = PublishedGeneration::new(SourceGeneration(1));
            let (mut foreign_host, mut foreign_timeline, _) = fenced(&other);
            let mut session_seek = word.session_seek(5, Some(8)).expect("open");
            assert_eq!(
                session_seek.command(3),
                SourceCommand::SeekAt {
                    generation: SourceGeneration(5),
                    frame: SourceFrame(3),
                    anchor_sample: 8,
                }
            );
            assert_eq!(
                session_seek.check_source(&foreign_host, 0),
                Err(SessionSeekError::ForeignSession)
            );
            assert_eq!(
                session_seek.push_source(&mut foreign_host, 0),
                Err(SessionSeekError::ForeignSession)
            );
            assert_eq!(
                session_seek.push_timeline(&mut foreign_timeline, 0),
                Err(SessionSeekError::ForeignSession)
            );
            assert_eq!(
                session_seek.check_timeline(&timeline, 0),
                Ok(()),
                "its own producers check"
            );
            drop(session_seek);
            assert_eq!(
                word.generation(),
                SourceGeneration(4),
                "nothing pushed, nothing published"
            );
            assert_eq!(
                foreign_host.check_seek(seek(2, 0)),
                Ok(()),
                "the foreign slot is empty"
            );
            assert_eq!(foreign_timeline.check_seek(seek(2, 0)), Ok(()));

            // A refused push publishes nothing either; one that lands publishes on drop.
            let mut session_seek = word.session_seek(5, Some(1)).expect("open");
            assert_eq!(
                session_seek.push_source(&mut host, 0),
                Err(SessionSeekError::Seek(SourceSeekError::AnchorUnaligned))
            );
            drop(session_seek);
            assert_eq!(word.generation(), SourceGeneration(4));
            let mut session_seek = word.session_seek(5, None).expect("open");
            session_seek.push_timeline(&mut timeline, 0).expect("push");
            drop(session_seek);
            assert_eq!(word.generation(), SourceGeneration(5), "the drop publishes");
        }

        /// Test value: red if a set seals, or takes a carry, with a consumer of another session's
        /// word: that consumer's producer would publish to a word the set's render never loads.
        #[test]
        fn a_set_takes_only_consumers_of_its_session() {
            let envelope = engine::realtime::RenderEnvelope {
                sample_rate: RATE,
                quantum: QuantumFrames(4),
                output_channels: NonZeroUsize::MIN,
            };
            let word = PublishedGeneration::new(SourceGeneration(1));
            let other = PublishedGeneration::new(SourceGeneration(1));
            let timeline = |word: &PublishedGeneration| {
                SourceTimeline::prepare(QuantumFrames(4), NonZeroUsize::MIN, word.clone())
                    .expect("timeline")
                    .1
            };
            let source = |word: &PublishedGeneration| {
                let (_, consumer, resources) = PcmSourceRing::prepare_host_region(
                    config(1, 4, 16),
                    SourceFrame(0),
                    word.clone(),
                )
                .expect("ring");
                SourceGraphSource::new(consumer, resources, 0, 0)
            };
            let seal = |sources, timeline| {
                prepare_session_graph_source_set(
                    envelope,
                    sources,
                    timeline,
                    word.clone(),
                    vec![test_mapping(0)],
                )
                .err()
            };
            assert_eq!(seal(vec![source(&word)], timeline(&word)), None);
            assert_eq!(
                seal(vec![source(&other)], timeline(&word)),
                Some(SourceGraphSourceSetError::SessionWord)
            );
            assert_eq!(
                seal(vec![source(&word)], timeline(&other)),
                Some(SourceGraphSourceSetError::SessionWord)
            );
            assert_eq!(
                seal(vec![SourceGraphSource::vacant(1)], timeline(&word)),
                None,
                "a vacant entry has no word"
            );

            let (_, _, mut predecessor) = fenced(&other);
            let (_, _, mut successor) = fenced(&word);
            successor.sources[0].consumer = None;
            assert!(!successor.adopt_sources(&mut predecessor, &[(0, 0)]));
            assert!(!successor.swappable(&predecessor, &[]));
            assert!(successor.sources[0].consumer.is_none(), "nothing moved");
            let (_, _, mut same_session) = fenced(&word);
            assert!(successor.adopt_sources(&mut same_session, &[(0, 0)]));
        }
    }

    /// #2205: a live plain seek lands when its runway is queued, bounded by the deadline; a plain
    /// session seek lands every source and the timeline in one block.
    mod seek_landing {
        use super::*;

        /// Quantum 4, a ring of 8 blocks, a runway (and deadline) of 3 blocks: a seek observed at
        /// sample 4 holds to sample 16 at the latest, so a runway queued by sample 12 lands a
        /// block before the deadline and the two landings are told apart.
        const Q: u64 = 4;

        fn landing_config() -> PcmSourceRingConfig {
            PcmSourceRingConfig {
                seek_runway_blocks: 3,
                ..config(1, 4, 32)
            }
        }

        /// The value every frame of one block carries: its generation and first frame.
        #[allow(clippy::cast_precision_loss)]
        fn value(generation: u64, frame: u64) -> f32 {
            (generation * 1_000 + frame) as f32
        }

        fn feed(host: &mut HostChunkProvider, generation: u64, frame: u64) {
            host.submit(chunk(
                generation,
                frame,
                &[&[value(generation, frame); 4]],
                4,
                false,
            ))
            .expect("block");
        }

        fn plain(generation: u64, frame: u64) -> SourceCommand {
            SourceCommand::Seek {
                generation: SourceGeneration(generation),
                frame: SourceFrame(frame),
            }
        }

        /// The value one played block carries, or `None` for no played block.
        fn played(consumer: &PcmSourceConsumer) -> Option<f32> {
            consumer.played_plane(0).map(|plane| plane[0])
        }

        /// Gate 1. Test value: red when a live plain seek applies at the next block with nothing
        /// of its generation queued (today: the old audio stops and the block underruns), or when
        /// it lands later than the block at which its runway is queued (a readiness check that
        /// never sees the runway lands at the deadline, a block later), or at `frame` plus the
        /// wait instead of at `frame`.
        #[test]
        fn a_live_plain_seek_plays_the_old_generation_until_its_runway_is_queued() {
            let (producer, mut consumer, _) = session_ring(landing_config()).expect("ring");
            let mut host = producer.into_host_chunk_provider(RATE);
            for frame in [0, 4, 8, 12] {
                feed(&mut host, 1, frame);
            }
            consumer.begin_block_at(0);
            host.try_seek(plain(2, 100)).expect("seek");
            let mut blocks = Vec::new();
            let feeds: [&[u64]; 4] = [&[100], &[104, 108], &[], &[]];
            for (block, submit) in (1..=4).zip(feeds) {
                let report = consumer.begin_block_at(block * Q);
                blocks.push((
                    played(&consumer),
                    report.active_generation.0,
                    report.underrun_frames,
                ));
                for &frame in submit {
                    feed(&mut host, 2, frame);
                }
            }
            assert_eq!(
                blocks,
                [
                    (Some(value(1, 4)), 1, 0),
                    (Some(value(1, 8)), 1, 0),
                    (Some(value(2, 100)), 2, 0),
                    (Some(value(2, 104)), 2, 0),
                ],
                "old audio, old audio with one block queued, then the target from its first frame"
            );
            let report = host.seek_report().expect("report");
            assert_eq!(
                (report.generation, report.first_sample, report.source_frame),
                (2, 3 * Q, 100)
            );
            assert_eq!(report.cumulative_underrun_frames, 0);
        }

        /// Test value: red when a live plain seek with no more PCM to come waits for a runway it
        /// can never get: a seek to the region's end ([`HostChunkProvider::set_region_end`]), or
        /// one whose only block ends the region, would hold its source silent to the deadline.
        #[test]
        fn a_live_plain_seek_with_no_more_pcm_to_come_lands_at_the_next_block() {
            for to_end in [true, false] {
                let (producer, mut consumer, _) = session_ring(landing_config()).expect("ring");
                let mut host = producer.into_host_chunk_provider(RATE);
                host.set_region_end(SourceFrame(200));
                for frame in [0, 4, 8, 12] {
                    feed(&mut host, 1, frame);
                }
                consumer.begin_block_at(0);
                if to_end {
                    host.try_seek(plain(2, 200)).expect("seek to the end");
                } else {
                    host.try_seek(plain(2, 198)).expect("seek");
                    host.submit(chunk(2, 198, &[&[value(2, 198); 2]], 2, true))
                        .expect("the region's last block");
                }
                let report = consumer.begin_block_at(Q);
                assert_eq!(report.active_generation.0, 2, "to the end: {to_end}");
                if !to_end {
                    assert_eq!(report.copied_frames, 2, "the region's last two frames");
                }
            }
        }

        /// Two mono sources and an occupied timeline at quantum 4 with a runway of 2, fenced by
        /// one session word; both sources play generation 1 from frame 0, four blocks queued.
        fn two_sources(
            word: &PublishedGeneration,
        ) -> (
            [HostChunkProvider; 2],
            TimelineProducer,
            SourceGraphSourceSetDriver,
        ) {
            let mut hosts = Vec::new();
            let mut entries = Vec::new();
            for _ in 0..2 {
                let (producer, consumer, _) = PcmSourceRing::prepare_host_region(
                    landing_config(),
                    SourceFrame(0),
                    word.clone(),
                )
                .expect("ring");
                let mut host = producer.into_host_chunk_provider(RATE);
                for frame in [0, 4, 8, 12] {
                    feed(&mut host, 1, frame);
                }
                hosts.push(host);
                entries.push(GraphSourceEntry {
                    channel_count: 1,
                    consumer: Some(consumer),
                });
            }
            let (timeline_producer, timeline, _) =
                SourceTimeline::prepare(QuantumFrames(4), NonZeroUsize::MIN, word.clone())
                    .expect("timeline");
            let driver = SourceGraphSourceSetDriver {
                timeline,
                published: word.clone(),
                sources: entries.into_boxed_slice(),
                mappings: vec![test_mapping(0)].into_boxed_slice(),
                quantum_frames: 4,
                block_validity: GraphObservationValidity::CLEAR,
                pending_generation_change: false,
                session_seek: 0,
            };
            let hosts: [HostChunkProvider; 2] = hosts.try_into().ok().expect("two hosts");
            (hosts, timeline_producer, driver)
        }

        /// A plain session seek to timeline sample 100 at generation 2.
        fn session_seek_to_100(
            word: &PublishedGeneration,
            hosts: &mut [HostChunkProvider; 2],
            timeline: &mut TimelineProducer,
        ) {
            let mut seek = word.session_seek(2, None).expect("open");
            seek.push_timeline(timeline, 100).expect("timeline");
            for host in hosts.iter_mut() {
                seek.push_source(host, 100).expect("source");
            }
            seek.publish();
        }

        /// One block of the set: each source's played value, generation and underrun, and the
        /// timeline sample the block starts at.
        fn set_block(
            driver: &mut SourceGraphSourceSetDriver,
            block: u64,
        ) -> ([(Option<f32>, u64, u64); 2], i64) {
            driver.begin_block(block * Q, 4).expect("block");
            let sources = core::array::from_fn(|index| {
                let consumer = driver.sources[index].consumer.as_ref().expect("occupied");
                let telemetry = consumer.telemetry();
                (
                    played(consumer),
                    telemetry.active_generation.0,
                    telemetry.underrun_frames,
                )
            });
            (sources, driver.timeline_at(block * Q).expect("timeline"))
        }

        /// Gate 2. Test value: red when a plain session seek lands before every source holding
        /// it has its runway (today: at the next block, with the slow source underrunning), lands
        /// its sources or the timeline in different blocks, or waits past the block at which the
        /// last runway is queued (a block before the deadline).
        #[test]
        fn a_plain_session_seek_lands_every_source_and_the_timeline_once_all_are_ready() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut hosts, mut timeline, mut driver) = two_sources(&word);
            set_block(&mut driver, 0);
            session_seek_to_100(&word, &mut hosts, &mut timeline);
            for frame in [100, 104, 108] {
                feed(&mut hosts[0], 2, frame);
            }
            let mut blocks = vec![set_block(&mut driver, 1)];
            feed(&mut hosts[1], 2, 100);
            blocks.push(set_block(&mut driver, 2));
            feed(&mut hosts[1], 2, 104);
            feed(&mut hosts[1], 2, 108);
            blocks.push(set_block(&mut driver, 3));
            blocks.push(set_block(&mut driver, 4));
            let old = |frame| (Some(value(1, frame)), 1, 0);
            let new = |frame| (Some(value(2, frame)), 2, 0);
            assert_eq!(
                blocks,
                [
                    ([old(4), old(4)], 4),
                    ([old(8), old(8)], 8),
                    ([new(100), new(100)], 100),
                    ([new(104), new(104)], 104),
                ],
                "both sources and the timeline land together, at the first block both are ready"
            );
        }

        /// Gate 3. Test value: red when a session seek waits past its deadline for a source that
        /// publishes nothing, or lands the ready source apart from the timeline at it.
        #[test]
        fn a_plain_session_seek_lands_at_its_deadline_with_the_slow_source_underrunning() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut hosts, mut timeline, mut driver) = two_sources(&word);
            set_block(&mut driver, 0);
            session_seek_to_100(&word, &mut hosts, &mut timeline);
            for frame in [100, 104, 108] {
                feed(&mut hosts[0], 2, frame);
            }
            let blocks: Vec<_> = (1..=4).map(|block| set_block(&mut driver, block)).collect();
            let old = |frame| (Some(value(1, frame)), 1, 0);
            assert_eq!(
                blocks,
                [
                    ([old(4), old(4)], 4),
                    ([old(8), old(8)], 8),
                    ([old(12), old(12)], 12),
                    ([(Some(value(2, 100)), 2, 0), (None, 2, 4)], 100),
                ],
                "held for the deadline's three blocks, then all land; the slow source underruns"
            );
        }

        /// Test value (#2205, the never-fed rule, root ruling 4): red when a live plain seek on a
        /// ring that had no PCM before it waits for a runway (an empty boot or a stem still
        /// loading would hold every seek to the deadline), or when one on a ring fed before it
        /// does not wait.
        #[test]
        fn a_live_plain_seek_waits_only_on_a_ring_fed_before_it() {
            for fed_before in [false, true] {
                let (producer, mut consumer, _) = session_ring(landing_config()).expect("ring");
                let mut host = producer.into_host_chunk_provider(RATE);
                if fed_before {
                    feed(&mut host, 1, 0);
                    consumer.begin_block_at(0);
                }
                host.try_seek(plain(2, 100)).expect("seek");
                feed(&mut host, 2, 100);
                let report = consumer.begin_block_at(Q);
                let expected = if fed_before { 1 } else { 2 };
                assert_eq!(
                    report.active_generation.0, expected,
                    "fed before the seek: {fed_before}"
                );
            }
        }

        /// Test value (#2205): red when a plain session seek named to land at once
        /// ([`SessionSeek::land_at_once`], a seek with no live audio to keep) waits for a runway:
        /// the old generation would play on after a stopped or offline seek.
        #[test]
        fn a_session_seek_named_to_land_at_once_lands_at_the_next_block() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut hosts, mut timeline, mut driver) = two_sources(&word);
            set_block(&mut driver, 0);
            let mut seek = word.session_seek(2, None).expect("open");
            seek.push_timeline(&mut timeline, 100).expect("timeline");
            for host in &mut hosts {
                seek.push_source(host, 100).expect("source");
            }
            seek.land_at_once();
            seek.publish();
            feed(&mut hosts[0], 2, 100);
            let block = set_block(&mut driver, 1);
            assert_eq!(
                block,
                ([(Some(value(2, 100)), 2, 0), (None, 2, 4)], 100),
                "every source and the timeline land at once; no old audio plays"
            );
        }

        /// A ring of its own session word, filled with generation 1 from frame 0 until the
        /// producer refuses a block; the number of blocks queued.
        fn full_ring(word: &PublishedGeneration) -> (HostChunkProvider, PcmSourceConsumer, u64) {
            let (producer, consumer, _) =
                PcmSourceRing::prepare_host_region(landing_config(), SourceFrame(0), word.clone())
                    .expect("ring");
            let mut host = producer.into_host_chunk_provider(RATE);
            let mut queued = 0;
            while host
                .submit(chunk(
                    1,
                    queued * Q,
                    &[&[value(1, queued * Q); 4]],
                    4,
                    false,
                ))
                .is_ok()
            {
                queued += 1;
            }
            (host, consumer, queued)
        }

        /// #2213. Test value: red when preparing an admitted seek between blocks pops nothing, or
        /// discards the old generation's blocks without recycling them: the full ring stays full,
        /// the target's submit is refused, and the block after the prepare underruns.
        #[test]
        fn a_prepared_admitted_seek_frees_a_full_ring_for_its_generation() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut host, mut consumer, queued) = full_ring(&word);
            let mut seek = word.session_seek(2, None).expect("open");
            seek.push_source(&mut host, 100).expect("source");
            seek.land_at_once();
            seek.publish();
            assert_eq!(
                consumer.prepare_admitted_seek(word.load_at_block_entry()),
                Some((SourceGeneration(2), SourceFrame(100)))
            );
            assert_eq!(consumer.stale_generation_discard_count, queued);
            for frame in [100, 104, 108] {
                feed(&mut host, 2, frame);
            }
            let report = consumer.begin_block_at(0);
            assert_eq!(
                (
                    played(&consumer),
                    report.active_generation.0,
                    report.underrun_frames
                ),
                (Some(value(2, 100)), 2, 0),
                "the target plays from its first frame"
            );
        }

        /// #2213. Test value: red when preparing an admitted seek lands an anchored seek, which
        /// has no block time between blocks, or drops the old generation's PCM for it.
        #[test]
        fn a_prepared_anchored_seek_stays_held() {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let (mut host, mut consumer, _) = full_ring(&word);
            let mut seek = word.session_seek(2, Some(8)).expect("open");
            seek.push_source(&mut host, 100).expect("source");
            seek.publish();
            assert_eq!(
                consumer.prepare_admitted_seek(word.load_at_block_entry()),
                None
            );
            assert_eq!(consumer.stale_generation_discard_count, 0);
            assert!(consumer.seek.held_seek.is_some(), "still held");
            let report = consumer.begin_block_at(0);
            assert_eq!(
                (played(&consumer), report.active_generation.0),
                (Some(value(1, 0)), 1),
                "the old generation plays to the anchor"
            );
        }
    }
}
