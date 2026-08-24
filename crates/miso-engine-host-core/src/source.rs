//! The host-side source control table: one sorted set of producer endpoints per prepared session.
//!
//! Every host used to carry its own `ControlSource` struct, its own lookup (capi binary-searched,
//! host-web linearly scanned), and its own transcription of the region and generation rules. This
//! module is the single definition; the rules are capi's, which were the stricter pair.

use core::alloc::Layout;

use miso_engine_core::SampleRateHz;
use miso_engine_source::{
    HostChunkError, HostChunkProvider, HostPlanarChunk, SourceCommand, SourceFrame,
    SourceGeneration, SourceSeekError, SubmitReport,
};

/// One host-owned producer endpoint plus the immutable facts a submission is checked against.
pub(crate) struct ControlSource {
    id_offset: usize,
    id_bytes: usize,
    sample_rate_hz: u32,
    channel_count: u32,
    region_start: u64,
    region_end: u64,
    provider: HostChunkProvider,
}

/// One borrowed planar chunk offered to a named source.
///
/// Borrowed rather than owned: the facade copies once, into the ring, and never retains the planes.
#[derive(Clone, Copy, Debug)]
pub struct SourceSubmission<'a> {
    /// Strictly increasing nonzero generation tag; `0` is rejected.
    pub generation: u64,
    /// Absolute first source frame of this chunk.
    pub start_frame: u64,
    /// Explicit sample rate of the decoded PCM, which must equal the source's declared rate.
    pub sample_rate_hz: u32,
    /// One borrowed plane per declared source channel.
    pub planes: &'a [&'a [f32]],
    /// Frames in each plane.
    pub frames: u32,
    /// Whether this chunk ends exactly at the mapped region end. The rule is symmetric: a chunk
    /// that ends at the region end must set it, and one that does not must not.
    pub end_of_region: bool,
}

/// A typed rejection of one source submission or seek.
///
/// Hosts map these onto their own result codes and diagnostic text; [`Self::diagnostic`] is the one
/// string table, so two hosts never disagree about what a rejection is called.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceControlError {
    /// No source in this session carries the supplied ID.
    UnknownSource,
    /// `start_frame + frames` does not fit in `u64`.
    RegionOverflow,
    /// The chunk or seek frame falls outside the source's mapped region.
    OutsideRegion,
    /// `end_of_region` disagrees with whether the chunk ends at the region end.
    EndOfRegionMismatch,
    /// Generation `0` is reserved and never valid.
    GenerationZero,
    /// The producer ring rejected the chunk.
    Chunk(HostChunkError),
    /// The producer ring rejected the seek.
    Seek(SourceSeekError),
}

impl SourceControlError {
    /// The stable diagnostic code for this rejection. This is the only place these strings exist.
    #[must_use]
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::UnknownSource => "source.id.unknown",
            Self::RegionOverflow => "source.region.overflow",
            Self::OutsideRegion => "source.region.outside",
            Self::EndOfRegionMismatch => "source.region.end_mismatch",
            Self::GenerationZero | Self::Seek(SourceSeekError::GenerationZero) => {
                "source.generation.zero"
            }
            Self::Chunk(HostChunkError::WrongSampleRate { .. }) => "source.rate.mismatch",
            Self::Chunk(HostChunkError::StaleGeneration { .. })
            | Self::Seek(SourceSeekError::GenerationNotStrictlyIncreasing { .. }) => {
                "source.generation.stale"
            }
            Self::Chunk(HostChunkError::ChannelCount { .. }) => "source.channels.mismatch",
            Self::Chunk(HostChunkError::PlaneLength { .. }) => "source.plane.length",
            Self::Chunk(HostChunkError::FrameCount { .. }) => "source.frames.shape",
            Self::Chunk(HostChunkError::NonContiguous { .. }) => "source.frame.noncontiguous",
            Self::Chunk(HostChunkError::EndOfRegionAlreadySubmitted) => "source.region.ended",
            Self::Chunk(HostChunkError::Full { .. }) => "source.backpressure",
            Self::Chunk(HostChunkError::InternalInvariant) => "source.internal",
            Self::Seek(SourceSeekError::Backpressure { .. }) => "source.seek.backpressure",
        }
    }

    /// Whether this rejection is bounded backpressure, which a host retries rather than reports.
    #[must_use]
    pub const fn is_backpressure(self) -> bool {
        matches!(
            self,
            Self::Chunk(HostChunkError::Full { .. })
                | Self::Seek(SourceSeekError::Backpressure { .. })
        )
    }

    /// Whether this rejection means an engine invariant failed rather than a caller mistake.
    #[must_use]
    pub const fn is_internal(self) -> bool {
        matches!(self, Self::Chunk(HostChunkError::InternalInvariant))
    }
}

/// Every host-owned source producer of one prepared session, sorted by ID for binary search.
///
/// The set is control-plane state: it is `Send` and moves to whichever thread feeds PCM, and it is
/// never touched from the render thread (the render side owns the matching consumers, which travel
/// inside the prepared plan).
pub struct SourceControlSet {
    ids: Box<[u8]>,
    sources: Box<[ControlSource]>,
}

impl SourceControlSet {
    pub(crate) fn new(ids: Box<[u8]>, sources: Box<[ControlSource]>) -> Self {
        Self { ids, sources }
    }

    fn id(&self, source: &ControlSource) -> &[u8] {
        &self.ids[source.id_offset..source.id_offset + source.id_bytes]
    }

    fn index_of(&self, id: &[u8]) -> Option<usize> {
        self.sources
            .binary_search_by(|source| self.id(source).cmp(id))
            .ok()
    }

    /// Submit one borrowed planar chunk to the named source.
    ///
    /// Preconditions are checked in a fixed order — rate, channel count, region, end-of-region
    /// symmetry, generation — before the ring sees the chunk, so a rejection always names the
    /// first rule that was broken.
    pub fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<SubmitReport, SourceControlError> {
        let index = self.index_of(id).ok_or(SourceControlError::UnknownSource)?;
        let source = &mut self.sources[index];
        let end = submission
            .start_frame
            .checked_add(u64::from(submission.frames))
            .ok_or(SourceControlError::RegionOverflow)?;
        if submission.sample_rate_hz != source.sample_rate_hz {
            return Err(SourceControlError::Chunk(HostChunkError::WrongSampleRate {
                expected: SampleRateHz(source.sample_rate_hz),
                actual: SampleRateHz(submission.sample_rate_hz),
            }));
        }
        if u32::try_from(submission.planes.len()).ok() != Some(source.channel_count) {
            return Err(SourceControlError::Chunk(HostChunkError::ChannelCount {
                expected: source.channel_count,
                actual: submission.planes.len(),
            }));
        }
        if submission.start_frame < source.region_start || end > source.region_end {
            return Err(SourceControlError::OutsideRegion);
        }
        if submission.end_of_region != (end == source.region_end) {
            return Err(SourceControlError::EndOfRegionMismatch);
        }
        let generation = SourceGeneration::new(submission.generation)
            .ok_or(SourceControlError::GenerationZero)?;
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
            .map_err(SourceControlError::Chunk)
    }

    /// Queue one strictly increasing, generation-tagged absolute seek on the named source.
    pub fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
    ) -> Result<(), SourceControlError> {
        let index = self.index_of(id).ok_or(SourceControlError::UnknownSource)?;
        let source = &mut self.sources[index];
        if !(source.region_start..=source.region_end).contains(&frame) {
            return Err(SourceControlError::OutsideRegion);
        }
        let generation =
            SourceGeneration::new(generation).ok_or(SourceControlError::GenerationZero)?;
        source
            .provider
            .try_seek(SourceCommand::Seek {
                generation,
                frame: SourceFrame(frame),
            })
            .map_err(SourceControlError::Seek)
    }

    /// The mapped source region of the named source, as `start..end` in source frames.
    ///
    /// A host reports the region a submission must fall inside; the facade enforces it.
    #[must_use]
    pub fn region(&self, id: &[u8]) -> Option<core::ops::Range<u64>> {
        let index = self.index_of(id)?;
        let source = &self.sources[index];
        Some(source.region_start..source.region_end)
    }

    /// Number of sources in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Whether the session declared no sources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Total bytes of source ID text retained by the set.
    #[must_use]
    pub fn id_bytes(&self) -> usize {
        self.ids.len()
    }

    /// Length in bytes of the longest source ID, for hosts that stage IDs in a fixed buffer.
    #[must_use]
    pub fn longest_id_bytes(&self) -> usize {
        self.sources
            .iter()
            .map(|source| source.id_bytes)
            .max()
            .unwrap_or(0)
    }

    /// Bytes this set retains: the endpoint table plus the ID arena.
    ///
    /// A host that must decide *before* preparing whether the set fits its cap projects the same
    /// number from [`control_table_bytes`] and [`source_id_arena_bytes`]; the two agree by
    /// construction and `retained_bytes_projection_matches_the_live_set` proves it.
    #[must_use]
    pub fn retained_bytes(&self) -> Option<u64> {
        // Measured from the live boxes, never from the projection, so the two are independent
        // witnesses of the same number.
        let table = u64::try_from(core::mem::size_of_val(&*self.sources)).ok()?;
        let ids = u64::try_from(core::mem::size_of_val(&*self.ids)).ok()?;
        table.checked_add(ids)
    }
}

/// Bytes one [`SourceControlSet`] endpoint table of `source_count` entries occupies.
///
/// This is the mirror a host's pre-flight resource projection reads. It is a function of the
/// facade's own private struct layout, so a host never spells that layout itself and a host's
/// projection cannot drift when the struct changes.
#[must_use]
pub fn control_table_bytes(source_count: usize) -> Option<u64> {
    let layout = Layout::array::<ControlSource>(source_count).ok()?;
    u64::try_from(layout.size()).ok()
}

/// Bytes the source ID arena of `total_id_bytes` occupies.
#[must_use]
pub fn source_id_arena_bytes(total_id_bytes: usize) -> Option<u64> {
    let layout = Layout::array::<u8>(total_id_bytes).ok()?;
    u64::try_from(layout.size()).ok()
}

pub(crate) struct ControlSourceBuilder {
    ids: Vec<u8>,
    sources: Vec<ControlSource>,
}

impl ControlSourceBuilder {
    /// Reserve exactly the ID arena and endpoint table one session needs, or report the failed
    /// reservation; the builder never grows afterwards.
    pub(crate) fn with_capacity(id_bytes: usize, source_count: usize) -> Result<Self, ()> {
        let mut ids = Vec::new();
        ids.try_reserve_exact(id_bytes).map_err(|_| ())?;
        let mut sources = Vec::new();
        sources.try_reserve_exact(source_count).map_err(|_| ())?;
        Ok(Self { ids, sources })
    }

    pub(crate) fn push(
        &mut self,
        id: &str,
        sample_rate_hz: u32,
        channel_count: u32,
        region_start: u64,
        region_end: u64,
        provider: HostChunkProvider,
    ) {
        let id_offset = self.ids.len();
        self.ids.extend_from_slice(id.as_bytes());
        self.sources.push(ControlSource {
            id_offset,
            id_bytes: id.len(),
            sample_rate_hz,
            channel_count,
            region_start,
            region_end,
            provider,
        });
    }

    pub(crate) fn finish(mut self) -> SourceControlSet {
        let ids = self.ids;
        self.sources.sort_unstable_by(|left, right| {
            ids[left.id_offset..left.id_offset + left.id_bytes]
                .cmp(&ids[right.id_offset..right.id_offset + right.id_bytes])
        });
        SourceControlSet::new(ids.into_boxed_slice(), self.sources.into_boxed_slice())
    }
}
