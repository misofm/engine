//! The host-side source control table: one sorted set of producer endpoints per prepared session.
//!
//! Every host used to carry its own `ControlSource` struct, its own lookup (capi binary-searched,
//! host-web linearly scanned), and its own transcription of the region and generation rules. This
//! module is the single definition; the rules are capi's, which were the stricter pair.

use core::alloc::Layout;
use std::collections::BTreeMap;

use engine::SampleRateHz;
use engine::realtime::PreparedRenderPlan;
use session::SessionModel;
use source::{
    HostChunkError, HostChunkProvider, HostPlanarChunk, PublishedGeneration, SessionSeek,
    SessionSeekError, SourceAdmission, SourceCommand, SourceFrame, SourceGeneration,
    SourceSeekError, SourceSeekReport, SubmitReport, TimelineProducer,
};

use crate::PlanStateInventory;

/// The reserved ID that [`SourceControlSet::seek_report`] reads the session timeline's seek
/// report under (#1704 D5), on both hosts. A stable ID starts with `a`-`z`
/// (`session::STABLE_ID_MAXIMUM_BYTES` bounds its length), so an ID that starts with `$` never
/// names a source.
pub const TIMELINE_SEEK_REPORT_ID: &[u8] = b"$timeline";

/// One host-owned producer endpoint plus the immutable facts a submission is checked against.
pub(crate) struct ControlSource {
    id_offset: usize,
    id_bytes: usize,
    sample_rate_hz: u32,
    channel_count: u32,
    region_start: u64,
    region_end: u64,
    /// Where the ring's producer is (#2258 HC-5).
    provider: Producer<HostChunkProvider>,
    /// Whether the plan was prepared to carry this source's ring from its predecessor (#2273):
    /// fixed at preparation, whichever producer moves have happened since.
    carried: bool,
}

/// How preparation made one entry of a [`SourceControlSet`] (#2273). Passed by value to the
/// builder, which moves the producer into the entry; boxing it would add an allocation per source.
#[allow(clippy::large_enum_variant)]
pub(crate) enum PreparedProducer {
    /// The plan carries the ring of the plan before it: the entry starts vacant, and the producer
    /// arrives through [`SourceControlSet::adopt_persisting`].
    Carried,
    /// A projection (#2199): the ring is charged, not allocated, so there is no producer.
    Projected,
    /// A ring of the plan's own, with its producer.
    Fresh(HostChunkProvider),
}

/// Where one entry's producer is: one source's [`HostChunkProvider`], or the session timeline's
/// [`TimelineProducer`] (#2258 HC-5).
///
/// An entry is *vacant* in every state but [`Self::Owned`], and every call that feeds or reads the
/// producer refuses a vacant entry with [`SourceControlError::Vacated`]. The states differ in what
/// may fill the entry: [`SourceControlSet::adopt_persisting`] fills [`Self::Awaiting`] alone, and
/// [`SourceControlSet::reclaim_from_withdrawn`] fills [`Self::Handed`] alone; nothing fills the
/// others. Control-thread state: render reads the ring the producer feeds, never this.
pub(crate) enum Producer<P> {
    /// The plan carries the ring of the plan before it, and the ring's producer has not arrived:
    /// the predecessor's set still holds it, or never held one.
    Awaiting,
    /// This set holds the producer.
    Owned(P),
    /// The producer moved to another set of the session: to a successor's, by its
    /// [`SourceControlSet::adopt_persisting`], or back from a withdrawn successor's to the set it
    /// came from, by [`SourceControlSet::reclaim_from_withdrawn`].
    Handed,
    /// The producer moved into a successor's fresh entry by a donation ([`apply_donation`]): this
    /// set belongs to a withdrawn plan and drops with it, so nothing fills the entry again. No
    /// fill reads it, so it behaves as [`Self::Handed`] does (vacant, refused
    /// [`SourceControlError::Vacated`]) in a set that is dropped before any reclaim; it is kept
    /// apart only to document how the producer left.
    Donated,
    /// A projection (#2199): the plan charges the ring without allocating it, so there is no
    /// producer. A projection's set never leaves its preparation. Sources only: a projection's
    /// timeline is never prepared and stays [`Self::Awaiting`].
    Projected,
}

impl<P> Producer<P> {
    fn owned(&self) -> Option<&P> {
        match self {
            Self::Owned(producer) => Some(producer),
            _ => None,
        }
    }

    fn owned_mut(&mut self) -> Option<&mut P> {
        match self {
            Self::Owned(producer) => Some(producer),
            _ => None,
        }
    }

    /// Move the producer out of an [`Self::Owned`] entry, leaving it `left`; any other entry is
    /// left as it is and gives `None`.
    fn take_leaving(&mut self, left: Self) -> Option<P> {
        if !matches!(self, Self::Owned(_)) {
            return None;
        }
        match core::mem::replace(self, left) {
            Self::Owned(producer) => Some(producer),
            _ => None,
        }
    }
}

impl ControlSource {
    /// The frame a session seek to `timeline_sample` sends this source: the sample clamped to
    /// the source's region (#1642 D1).
    fn session_frame(&self, timeline_sample: u64) -> u64 {
        timeline_sample.min(self.region_end).max(self.region_start)
    }
}

/// Step 1 of [`SourceControlSet::seek_session`]: generation 0 is
/// [`SourceControlError::GenerationZero`], as for a per-source seek; a stale generation is the
/// word's [`SourceControlError::Seek`] refusal.
fn session_refusal(error: SourceSeekError) -> SourceControlError {
    match error {
        SourceSeekError::GenerationZero => SourceControlError::GenerationZero,
        error => SourceControlError::Seek(error),
    }
}

/// Step 2 of [`SourceControlSet::seek_session`], once the caller has the timeline's producer (a
/// vacant timeline is [`SourceControlError::Vacated`]): the timeline producer, then every source
/// producer in ID order, checks its command with nothing pushed; the first refusal is returned. A
/// vacant entry is [`SourceControlError::Vacated`]; a producer of another session's word is an
/// engine invariant ([`HostChunkError::InternalInvariant`]): every producer of the set is prepared
/// with the set's word.
fn check_session_producers(
    seek: &SessionSeek<'_>,
    timeline: &TimelineProducer,
    sources: &[ControlSource],
    timeline_sample: u64,
) -> Result<(), SourceControlError> {
    let refused = |error| match error {
        SessionSeekError::Seek(error) => SourceControlError::Seek(error),
        SessionSeekError::ForeignSession => {
            SourceControlError::Chunk(HostChunkError::InternalInvariant)
        }
    };
    seek.check_timeline(timeline, timeline_sample)
        .map_err(refused)?;
    for source in sources {
        seek.check_source(
            source.provider.owned().ok_or(SourceControlError::Vacated)?,
            source.session_frame(timeline_sample),
        )
        .map_err(refused)?;
    }
    Ok(())
}

/// A session seek that every producer of its set has accepted (steps 1 and 2 of
/// [`SourceControlSet::seek_session`]), holding the set's producers exclusively until it pushes
/// (#2258 HC-9). [`Self::check`] is its only constructor, so a seek reaches its pushes only with
/// every check passed, and nothing can change a producer between its check and its push.
///
/// Its push, [`Self::push_and_publish`], cannot be refused. For each producer, what the check read
/// and the push reads again is unchanged in between:
/// - the entry is still owned, and its producer still publishes to this session's word: the
///   borrow keeps every producer move ([`SourceControlSet::adopt_persisting`],
///   [`SourceControlSet::reclaim_from_withdrawn`], a donation) out;
/// - every rule but the slot (a nonzero generation above the producer's, the anchor, the frame
///   limit) reads the command, which is the same, and the producer's own state, which only its
///   own push changes; no push changes another producer, and none publishes;
/// - the one-slot command ring had room: only this producer fills it, and render only empties it.
///
/// So no state has some commands pushed and another refused: the seek is refused with nothing
/// pushed, or every command is pushed and the generation is published once.
struct CheckedSessionSeek<'s> {
    seek: SessionSeek<'s>,
    timeline: &'s mut TimelineProducer,
    sources: &'s mut [ControlSource],
    timeline_sample: u64,
}

impl<'s> CheckedSessionSeek<'s> {
    /// Step 2: check every producer ([`check_session_producers`]), keeping them all for the push.
    fn check(
        seek: SessionSeek<'s>,
        timeline: &'s mut Producer<TimelineProducer>,
        sources: &'s mut [ControlSource],
        timeline_sample: u64,
    ) -> Result<Self, SourceControlError> {
        let timeline = timeline.owned_mut().ok_or(SourceControlError::Vacated)?;
        check_session_producers(&seek, timeline, sources, timeline_sample)?;
        Ok(Self {
            seek,
            timeline,
            sources,
            timeline_sample,
        })
    }

    /// Step 3: push every command, the timeline first, then the sources in ID order, none of them
    /// published; then publish the generation once, so render observes all of them at one block
    /// entry. Each push is one its check accepted (see the type), so none is refused. One refused
    /// anyway would mean a broken SPSC ring (render only empties the slot): the engine stops there
    /// (a panic is an engine defect, and a release build aborts) rather than seek some producers
    /// and not others.
    fn push_and_publish(self) {
        let Self {
            mut seek,
            timeline,
            sources,
            timeline_sample,
        } = self;
        if seek.push_timeline(timeline, timeline_sample).is_err() {
            unreachable!("the timeline accepted this seek, and nothing has pushed to it since");
        }
        for source in sources {
            let frame = source.session_frame(timeline_sample);
            let Some(provider) = source.provider.owned_mut() else {
                unreachable!("the check found every source owned");
            };
            if seek.push_source(provider, frame).is_err() {
                unreachable!("each source accepted this seek, and nothing has pushed to it since");
            }
        }
        seek.publish();
    }
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
    /// The supplied ID cannot name a source: it is empty, not UTF-8, or longer than
    /// [`session::STABLE_ID_MAXIMUM_BYTES`] ([`Self::unmatched_id`]), refused by every per-source
    /// call, so both hosts refuse a malformed ID with the same text.
    InvalidId,
    /// The source exists but this set holds no producer for it: the set belongs to a successor
    /// plan whose producer has not been handed over yet ([`SourceControlSet::adopt_persisting`]),
    /// or to a predecessor whose producer has already moved to its successor, by an adoption or a
    /// donation. Neither backpressure nor an engine invariant: the caller feeds the newest set.
    Vacated,
    /// Every bounded attempt to read the source's seek report landed inside render's publication
    /// of it (#1316 D6): backpressure, the caller reads again.
    ReportBusy,
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
            Self::Vacated => "source.ring.vacated",
            Self::Seek(SourceSeekError::AnchorUnaligned) => "source.seek.anchor_unaligned",
            Self::ReportBusy => "source.report.busy",
            Self::Seek(SourceSeekError::FrameOutOfRange) => "source.seek.frame_out_of_range",
            Self::InvalidId => "source.id.invalid",
        }
    }

    /// The refusal for a source ID that names no source (#1706 D4): [`Self::InvalidId`] for an
    /// empty, non-UTF-8 or over-[`session::STABLE_ID_MAXIMUM_BYTES`] ID, [`Self::UnknownSource`]
    /// for a well-formed one. Every declared source ID is a stable ID, so a malformed ID never
    /// names one: classifying after a failed lookup refuses exactly what checking first would.
    /// A host that looks a source up itself reports its miss through this, never its own text.
    #[must_use]
    pub fn unmatched_id(id: &[u8]) -> Self {
        if id.is_empty()
            || id.len() > session::STABLE_ID_MAXIMUM_BYTES
            || core::str::from_utf8(id).is_err()
        {
            Self::InvalidId
        } else {
            Self::UnknownSource
        }
    }

    /// Whether this rejection is bounded backpressure, which a host retries rather than reports.
    #[must_use]
    pub const fn is_backpressure(self) -> bool {
        matches!(
            self,
            Self::Chunk(HostChunkError::Full { .. })
                | Self::Seek(SourceSeekError::Backpressure { .. })
                | Self::ReportBusy
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
    /// The session timeline's producer (#1641 D2). Every successor carries the timeline, so a
    /// successor's starts [`Producer::Awaiting`] and [`Self::adopt_persisting`] moves its
    /// predecessor's in; a first plan's is [`Producer::Owned`] from preparation.
    timeline: Producer<TimelineProducer>,
    /// The session's published seek generation (#1642 B16-R2), which every producer here
    /// publishes to and every plan of the session loads at block entry: the same word in every
    /// set of the session.
    published: PublishedGeneration,
}

impl SourceControlSet {
    pub(crate) fn new(
        ids: Box<[u8]>,
        sources: Box<[ControlSource]>,
        published: PublishedGeneration,
    ) -> Self {
        Self {
            ids,
            sources,
            timeline: Producer::Awaiting,
            published,
        }
    }

    /// Give the set its plan's fresh timeline producer (#1641 D2). Preparation only.
    pub(crate) fn set_timeline(&mut self, producer: TimelineProducer) {
        self.timeline = Producer::Owned(producer);
    }

    /// The session timeline's producer, for a test that seeks the timeline directly (#1641
    /// gates). `None` while vacant. A host seeks the timeline with [`Self::seek_session`].
    #[cfg(feature = "test-support")]
    pub fn test_only_timeline_producer(&mut self) -> Option<&mut TimelineProducer> {
        self.timeline.owned_mut()
    }

    /// Whether the set's plan carries source `id`'s ring from the plan before it (#1272 D3): fixed
    /// when the set is prepared, so the answer is the same before and after
    /// [`Self::adopt_persisting`] or a donation moves a producer (#2273). `false` for an ID the
    /// set does not hold.
    #[must_use]
    pub fn carries(&self, id: &[u8]) -> bool {
        self.index_of(id)
            .is_some_and(|index| self.sources[index].carried)
    }

    /// Store `required(id)` on the `prime_required` flag (#1320 D4) of every ring whose producer
    /// this set holds (#1403 D3). A vacant entry has no producer here and is skipped: its ring's
    /// flag is reached through the set that holds the producer. Control thread only; no allocation.
    pub fn set_prime_flags(&self, mut required: impl FnMut(&[u8]) -> bool) {
        for source in &self.sources {
            if let Some(provider) = source.provider.owned() {
                provider.prime_flag().set(required(self.id(source)));
            }
        }
    }

    fn id(&self, source: &ControlSource) -> &[u8] {
        &self.ids[source.id_offset..source.id_offset + source.id_bytes]
    }

    fn index_of(&self, id: &[u8]) -> Option<usize> {
        self.sources
            .binary_search_by(|source| self.id(source).cmp(id))
            .ok()
    }

    /// The index of the source a host call names, or the core's refusal for an ID that names none
    /// ([`SourceControlError::unmatched_id`]).
    fn source_index(&self, id: &[u8]) -> Result<usize, SourceControlError> {
        self.index_of(id)
            .ok_or_else(|| SourceControlError::unmatched_id(id))
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
        let index = self.source_index(id)?;
        let source = &mut self.sources[index];
        let provider = source
            .provider
            .owned_mut()
            .ok_or(SourceControlError::Vacated)?;
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
        provider
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
    ///
    /// It lands when its PCM is ready (#2205, owner decision 2026-10-29): the playing generation
    /// plays on until the seek's runway, `R` blocks of `generation`, is queued
    /// ([`crate::source_seek_runway_blocks`], 50 ms capped at half the ring), or a block that
    /// ends the region is, then lands in one cut at `frame`, from its first sample. A seek to the
    /// region's end, or on a source that had no PCM before it, lands at the next block. It lands
    /// at the latest `R` blocks after render observed it; a source with no PCM then underruns.
    pub fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
    ) -> Result<(), SourceControlError> {
        self.queue_seek(id, generation, frame, None)
    }

    /// Queue one strictly increasing, generation-tagged seek whose source `frame` enters the graph
    /// in the block that reads source time from `anchor_sample` (issue #1274).
    ///
    /// `anchor_sample` is on the plan's source-read clock, which every plan swap continues and
    /// which equals the absolute render clock a host renders with until a plan that reads ahead of
    /// it, a warm successor (#1355 D1), is adopted; no host publishes one in this version (#1403). It
    /// names when `frame` enters the graph, and the output hears it the plan's latency later. It
    /// must be a multiple of the quantum, or the seek is refused with
    /// `source.seek.anchor_unaligned`. The region and generation rules are [`Self::seek`]'s.
    ///
    /// On success the source accepts the new generation's PCM from `frame` at once; submit it
    /// before the anchor block renders and the source plays it from exactly that block, even when
    /// the render pops that PCM before it pops the seek. Should the render reach the seek only
    /// after the anchor block has passed, it starts at `frame` plus the lateness, so it stays in
    /// time. Until the anchor block, PCM already queued for the playing generation keeps playing;
    /// when that runs out, the source renders silence and holds: the blocks count as held, not as
    /// underruns (#1318). [`Self::seek_report`] tells where the seek landed.
    pub fn seek_at(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
        anchor_sample: u64,
    ) -> Result<(), SourceControlError> {
        self.queue_seek(id, generation, frame, Some(anchor_sample))
    }

    /// Seek the session's timeline and every source to timeline sample `timeline_sample`, under
    /// one new `generation`, all or nothing (#1642 D1): the session seek both hosts call.
    ///
    /// The timeline goes to `timeline_sample` as given (up to [`source::TIMELINE_SAMPLE_LIMIT`]);
    /// each source to `min(timeline_sample, its region end)`. With `anchor_sample` every command is
    /// the anchored form of [`Self::seek_at`], with that anchor. The host then submits generation
    /// `generation` from each source's frame.
    ///
    /// 1. Generation 0 is [`SourceControlError::GenerationZero`]; a generation not above the
    ///    session's published generation ([`Self::published_generation`]) is stale
    ///    (`source.generation.stale`), so a source removed at a higher generation cannot let a
    ///    command through early.
    /// 2. The timeline producer and every source producer, in ID order, check their command with
    ///    no push: the first refusal is returned and nothing is pushed. A vacant entry is
    ///    [`SourceControlError::Vacated`]; a generation not above a producer's, a full slot, an
    ///    unaligned anchor and a timeline frame over the limit are that producer's
    ///    [`SourceControlError::Seek`] refusal.
    /// 3. Every command is pushed, the timeline first, then the sources in ID order, none of them
    ///    published; then the generation is published once (B16-R2), so render observes all of
    ///    them at one block entry or none. Step 2 holds every producer until its push, and only
    ///    this thread fills those slots while render only empties them, so no push can be
    ///    refused (#2258 HC-9): the seek is refused whole at step 1 or 2, or it lands whole.
    ///
    /// The plain form lands as [`Self::seek`] does, for every source and the timeline in one
    /// block (#2205): the first at which every source that waits has its runway queued, or the
    /// deadline. It never lands the sources one by one.
    ///
    /// Acknowledged only when every consumer holds the seek in its slot: no ack precedes a drop.
    pub fn seek_session(
        &mut self,
        generation: u64,
        timeline_sample: u64,
        anchor_sample: Option<u64>,
    ) -> Result<(), SourceControlError> {
        self.seek_session_landing(generation, timeline_sample, anchor_sample, false)
    }

    /// [`Self::seek_session`]'s plain form for a seek with no live audio to keep (#2205): an
    /// engine that renders only when asked, or a stopped transport. It lands for every source and
    /// the timeline at the first block that observes it, as a plain session seek did before
    /// #2205, without waiting for a runway; the host submits each source's PCM before that block.
    /// Every rule and refusal is [`Self::seek_session`]'s.
    pub fn seek_session_at_once(
        &mut self,
        generation: u64,
        timeline_sample: u64,
    ) -> Result<(), SourceControlError> {
        self.seek_session_landing(generation, timeline_sample, None, true)
    }

    fn seek_session_landing(
        &mut self,
        generation: u64,
        timeline_sample: u64,
        anchor_sample: Option<u64>,
        at_once: bool,
    ) -> Result<(), SourceControlError> {
        let mut seek = self
            .published
            .session_seek(generation, anchor_sample)
            .map_err(session_refusal)?;
        if at_once {
            seek.land_at_once();
        }
        CheckedSessionSeek::check(seek, &mut self.timeline, &mut self.sources, timeline_sample)?
            .push_and_publish();
        Ok(())
    }

    /// Every rule of [`Self::seek_session`] (steps 1 and 2) with nothing pushed and nothing
    /// published: the check a caller runs before it commits to the seek (B16-R9 declares the
    /// discontinuity between this check and [`Self::seek_session`], which checks again).
    pub fn check_session_seek(
        &self,
        generation: u64,
        timeline_sample: u64,
        anchor_sample: Option<u64>,
    ) -> Result<(), SourceControlError> {
        let seek = self
            .published
            .session_seek(generation, anchor_sample)
            .map_err(session_refusal)?;
        let timeline = self.timeline.owned().ok_or(SourceControlError::Vacated)?;
        check_session_producers(&seek, timeline, &self.sources, timeline_sample)
    }

    /// The session's published seek generation (#1642 B16-R2): the largest generation any seek of
    /// this session has published, 1 before the first. A session seek must be above it; a host
    /// that chooses one above the largest generation it has used on the session always is.
    #[must_use]
    pub fn published_generation(&self) -> u64 {
        self.published.generation().0
    }

    /// Where the named source's newest generation landed, as render last published it (#1316 D7):
    /// the generation in effect, the source-read sample of the first block it played in and the
    /// source frame that block started at, plus the ring's cumulative underrun and held frames.
    /// Generation `0` means the ring has not rendered a block yet.
    ///
    /// A plain seek lands at the first block at which its runway is queued, at the latest `R`
    /// blocks after render observed it, at its frame (#2205, [`Self::seek`]); an anchored one at
    /// its anchor block, at `frame` plus the lateness if render reached it late. So a host that
    /// reads `(g, s, f)` here knows the source plays frame `f + (t - s)` at source-read sample `t`.
    ///
    /// Bounded: a read that keeps landing inside render's publication is
    /// [`SourceControlError::ReportBusy`]. The record belongs to the ring, so it survives a plan
    /// swap that carries the source; a vacant entry is [`SourceControlError::Vacated`], as for
    /// [`Self::seek`].
    ///
    /// The reserved ID [`TIMELINE_SEEK_REPORT_ID`] reads the session timeline's report instead
    /// (#1704 D5), in the same form: its `source_frame` is the timeline sample the block started
    /// at, and its underrun and held counts are `0`. A vacant timeline is
    /// [`SourceControlError::Vacated`]. Only this call takes the reserved ID: [`Self::submit`],
    /// [`Self::seek`] and [`Self::seek_at`] refuse it as an unknown source, because the timeline
    /// is seeked by [`Self::seek_session`] alone.
    pub fn seek_report(&self, id: &[u8]) -> Result<SourceSeekReport, SourceControlError> {
        if id == TIMELINE_SEEK_REPORT_ID {
            return self
                .timeline
                .owned()
                .ok_or(SourceControlError::Vacated)?
                .seek_report()
                .map_err(|_| SourceControlError::ReportBusy);
        }
        let index = self.source_index(id)?;
        self.sources[index]
            .provider
            .owned()
            .ok_or(SourceControlError::Vacated)?
            .seek_report()
            .map_err(|_| SourceControlError::ReportBusy)
    }

    /// The named source's ring occupancy as its producer sees it now (issue #1387), for a host
    /// feed that sizes its submissions by it. Lock-free and allocation-free; any control thread.
    /// An unknown ID is the core's unmatched-ID refusal; a vacant entry is
    /// [`SourceControlError::Vacated`]. The reserved timeline ID names no ring, so it is unknown.
    pub fn admission(&self, id: &[u8]) -> Result<SourceAdmission, SourceControlError> {
        let index = self.source_index(id)?;
        Ok(self.sources[index]
            .provider
            .owned()
            .ok_or(SourceControlError::Vacated)?
            .admission())
    }

    fn queue_seek(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
        anchor_sample: Option<u64>,
    ) -> Result<(), SourceControlError> {
        let index = self.source_index(id)?;
        let source = &mut self.sources[index];
        let provider = source
            .provider
            .owned_mut()
            .ok_or(SourceControlError::Vacated)?;
        if !(source.region_start..=source.region_end).contains(&frame) {
            return Err(SourceControlError::OutsideRegion);
        }
        let generation =
            SourceGeneration::new(generation).ok_or(SourceControlError::GenerationZero)?;
        let frame = SourceFrame(frame);
        let command = match anchor_sample {
            None => SourceCommand::Seek { generation, frame },
            Some(anchor_sample) => SourceCommand::SeekAt {
                generation,
                frame,
                anchor_sample,
            },
        };
        provider.try_seek(command).map_err(SourceControlError::Seek)
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

    /// Move into this set the producer of every source it awaits (its plan carries the ring and
    /// the producer has not arrived), from `predecessor`'s entry with the same ID, which is left
    /// handed on. Returns how many source producers moved. An awaited timeline producer moves by
    /// the same rule (#1641 D2) and is not counted: every successor carries the timeline. No
    /// other entry is filled (#2258 HC-5): this set's own producers, and an entry that handed or
    /// donated its producer on, stay as they are.
    ///
    /// A host calls this after preparing a successor with
    /// [`crate::prepare_host_runtime_successor`] (or its live-control form) and after its last
    /// fallible step, so the move is never undone. Infallible and allocation-free: an awaiting
    /// entry whose ID the predecessor does not hold, or holds vacant itself, stays awaiting and
    /// refuses PCM with [`SourceControlError::Vacated`]. Keyed by source ID, never by index.
    ///
    /// The producers move before the plan swap that moves their consumers. Should that swap's
    /// carry report `PredecessorMismatch` -- unreachable on both hosts by construction (issue
    /// #1269 P11) -- the successor's vacant sources render `+0.0` from the swap block on, since
    /// their producers have already moved here and their rings stay with the retired plan. The
    /// timeline keeps the same contract: its producer has moved here too, and the successor's
    /// vacant timeline advances from timeline sample 0 and observes no seek. Its one exception
    /// (#1641 D9): nothing reports the vacant timeline itself; the swap's mismatch count is the
    /// only sign.
    pub fn adopt_persisting(&mut self, predecessor: &mut SourceControlSet) -> usize {
        let mut moved = 0;
        for index in 0..self.sources.len() {
            if !matches!(self.sources[index].provider, Producer::Awaiting) {
                continue;
            }
            let Some(from) = predecessor.index_of(self.id(&self.sources[index])) else {
                continue;
            };
            if let Some(provider) = predecessor.sources[from]
                .provider
                .take_leaving(Producer::Handed)
            {
                self.sources[index].provider = Producer::Owned(provider);
                moved += 1;
            }
        }
        if matches!(self.timeline, Producer::Awaiting)
            && let Some(timeline) = predecessor.timeline.take_leaving(Producer::Handed)
        {
            self.timeline = Producer::Owned(timeline);
        }
        debug_assert!(
            self.published.is_shared_with(&predecessor.published),
            "a successor's set shares its predecessor's published generation (#1642 B16-R2)"
        );
        moved
    }

    /// Take back from `withdrawn`, a successor of this set's plan withdrawn before render adopted
    /// it (#1310 D5 step 5), every producer this set handed it ([`Self::adopt_persisting`]) that
    /// it still holds, because this plan renders their rings until the plan that replaces it is
    /// adopted. Returns how many source producers moved; the timeline moves by the same rule and is
    /// not counted.
    ///
    /// Only an entry here that handed its producer on is filled, and only from an entry of
    /// `withdrawn` with the same ID that carries this plan's ring ([`Self::carries`]) and holds its
    /// producer (#2258 HC-5). So an entry that awaits its own predecessor's producer, or handed its
    /// producer to another set, never takes a producer of `withdrawn`'s own, whose ring drops with
    /// `withdrawn`. Every producer `withdrawn` holds of a ring it carries came from this set, so
    /// nothing is left behind. Infallible, allocation-free and keyed by source ID.
    pub fn reclaim_from_withdrawn(&mut self, withdrawn: &mut SourceControlSet) -> usize {
        let mut moved = 0;
        for index in 0..self.sources.len() {
            if !matches!(self.sources[index].provider, Producer::Handed) {
                continue;
            }
            let Some(from) = withdrawn.index_of(self.id(&self.sources[index])) else {
                continue;
            };
            let entry = &mut withdrawn.sources[from];
            if !entry.carried {
                continue;
            }
            if let Some(provider) = entry.provider.take_leaving(Producer::Handed) {
                self.sources[index].provider = Producer::Owned(provider);
                moved += 1;
            }
        }
        if matches!(self.timeline, Producer::Handed)
            && let Some(timeline) = withdrawn.timeline.take_leaving(Producer::Handed)
        {
            self.timeline = Producer::Owned(timeline);
        }
        debug_assert!(
            self.published.is_shared_with(&withdrawn.published),
            "a withdrawn successor's set shares this set's published generation (#1642 B16-R2)"
        );
        moved
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
        producer: PreparedProducer,
    ) {
        let (carried, provider) = match producer {
            PreparedProducer::Carried => (true, Producer::Awaiting),
            PreparedProducer::Projected => (false, Producer::Projected),
            PreparedProducer::Fresh(mut provider) => {
                // #2205: a live plain seek to the region's end has no PCM to wait for.
                provider.set_region_end(SourceFrame(region_end));
                (false, Producer::Owned(provider))
            }
        };
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
            carried,
        });
    }

    pub(crate) fn finish(mut self, published: PublishedGeneration) -> SourceControlSet {
        let ids = self.ids;
        self.sources.sort_unstable_by(|left, right| {
            ids[left.id_offset..left.id_offset + left.id_bytes]
                .cmp(&ids[right.id_offset..right.id_offset + right.id_bytes])
        });
        SourceControlSet::new(
            ids.into_boxed_slice(),
            self.sources.into_boxed_slice(),
            published,
        )
    }
}

/// One side of a source donation (#1344 D3): a prepared plan the control thread holds, its source
/// producers, what it holds ([`crate::PreparedHost::inventory`]) and the session model it was
/// compiled from.
///
/// The successor side is a prepared plan not yet published; the donor side is a plan committed and
/// then withdrawn before render adopted it (an `UnadoptedCandidate`'s `plan()`). Neither is owned
/// by render, so nothing either side holds can change between [`check_donation`] and
/// [`apply_donation`]: holding the plan by reference is what makes it unpublished.
#[derive(Clone, Copy)]
pub struct DonationSide<'a> {
    /// The plan, which owns the source consumers.
    pub plan: &'a PreparedRenderPlan,
    /// The plan's source producers.
    pub sources: &'a SourceControlSet,
    /// The plan's own inventory: each source's ring configuration and index.
    pub inventory: &'a PlanStateInventory,
    /// The normalized model the plan was compiled from: each source's declaration.
    pub model: &'a SessionModel,
}

/// The mutable halves [`apply_donation`] moves between: a plan and its source producers.
pub struct DonationSideMut<'a> {
    /// The plan, which owns the source consumers.
    pub plan: &'a mut PreparedRenderPlan,
    /// The plan's source producers.
    pub sources: &'a mut SourceControlSet,
}

/// Why [`check_donation`] refuses (#1344 D5, D6). A refusal moves nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DonationError {
    /// A donor ring the donation would move has been consumed: its consumer has begun a block, so
    /// PCM the host was acked for may already be gone. A withdrawn candidate is never rendered
    /// (#1311), so only a plan rendered on the control thread (a synchronous host, a test) is
    /// refused this way.
    Consumed,
    /// The apply step could not perform a selected move: a producer is missing on one side, or
    /// the two plans' source entries cannot exchange (another quantum or channel count, or not
    /// graph plans).
    Unswappable,
    /// A successor ring the donation would replace has already been fed: its producer accepted
    /// PCM, an end of region or a seek. The donation drops that producer and moves its ring to the
    /// donor, so the host would lose what it was acked for. An unpublished successor is not
    /// reachable by a host submit, so only a caller that fed it by hand (a test) is refused.
    SuccessorFed,
}

/// The producer half of a [`DonationPlan`]: `(successor entry, donor entry)` positions in the two
/// [`SourceControlSet`]s.
#[derive(Debug, Eq, PartialEq)]
pub struct ProducerDonation(Box<[(usize, usize)]>);

/// The consumer half of a [`DonationPlan`]: `(successor source index, donor source index)` pairs
/// in the two plans' source sets.
#[derive(Debug, Eq, PartialEq)]
pub struct ConsumerDonation(Box<[(u32, u32)]>);

/// What [`check_donation`] found movable, one pair per donated source, in successor source order.
/// It borrows neither side.
#[derive(Debug, Eq, PartialEq)]
pub struct DonationPlan {
    /// The producers `donate_producers` swaps.
    pub producers: ProducerDonation,
    /// The consumers `donate_consumers` swaps.
    pub consumers: ConsumerDonation,
}

impl DonationPlan {
    /// Sources the plan donates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.consumers.0.len()
    }

    /// Whether the plan donates nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.consumers.0.is_empty()
    }
}

/// Check a donation of fresh source rings from a withdrawn plan to its replacement (#1344 D3,
/// D5). Read-only: it moves nothing and has no side effect.
///
/// It selects every source that both plans created fresh (each holds the source's ring: neither
/// prepared it vacant to carry from the plan render runs) with the same ID, the same declaration
/// (content, channels, bit depth, frames) and the same ring configuration. A source whose ID
/// matches but whose declaration or ring differs is a source the transaction redeclared, as the
/// carry rule treats it ([`crate::SuccessorBase`]): it is not selected and keeps its own fresh
/// ring. It then checks everything the apply needs, so the apply cannot fail: both producers
/// present, both consumers exchangeable, and every donated consumer unconsumed since its
/// generation start (it has never begun a block), so every acked chunk, the generation and any
/// held seek are still in its ring.
///
/// A caller runs it before the protocol commit, and [`apply_donation`] after.
pub fn check_donation(
    successor: DonationSide<'_>,
    donor: DonationSide<'_>,
) -> Result<DonationPlan, DonationError> {
    fn declarations(model: &SessionModel) -> BTreeMap<&str, &session::Source> {
        model
            .sources
            .iter()
            .map(|source| (source.id.as_str(), source))
            .collect()
    }
    let successor_declarations = declarations(successor.model);
    let donor_declarations = declarations(donor.model);
    let mut producers = Vec::new();
    let mut consumers = Vec::new();
    for row in successor.inventory.sources() {
        let Some(given) = donor.inventory.source(&row.id) else {
            continue;
        };
        let fresh =
            |side: &DonationSide<'_>, index: u32| side.plan.can_prepare_source_seek(index as usize);
        if !fresh(&successor, row.index)
            || !fresh(&donor, given.index)
            || row.ring != given.ring
            || successor_declarations.get(&*row.id) != donor_declarations.get(&*row.id)
            || !successor_declarations.contains_key(&*row.id)
        {
            continue;
        }
        let (Some(into), Some(from)) = (
            successor.sources.index_of(row.id.as_bytes()),
            donor.sources.index_of(given.id.as_bytes()),
        ) else {
            return Err(DonationError::Unswappable);
        };
        let (Some(fresh_producer), Some(_)) = (
            successor.sources.sources[into].provider.owned(),
            donor.sources.sources[from].provider.owned(),
        ) else {
            return Err(DonationError::Unswappable);
        };
        if !unfed(fresh_producer) {
            return Err(DonationError::SuccessorFed);
        }
        producers.push((into, from));
        consumers.push((row.index, given.index));
    }
    graph::can_donate_sources(successor.plan, donor.plan, &consumers).map_err(|refusal| {
        match refusal {
            graph::GraphDonationRefusal::Consumed => DonationError::Consumed,
            graph::GraphDonationRefusal::NotAGraphPlan | graph::GraphDonationRefusal::Mismatch => {
                DonationError::Unswappable
            }
        }
    })?;
    Ok(DonationPlan {
        producers: ProducerDonation(producers.into_boxed_slice()),
        consumers: ConsumerDonation(consumers.into_boxed_slice()),
    })
}

/// Whether a replacement would drop PCM the host was acked for on a ring the withdrawn plan
/// `donor` created (#1310, root rulings of 2026-10-16 on the acked-batch rule, M-A of fold 3).
///
/// True when some source X of `donor` meets all three:
///
/// * **Fresh in the donor.** The donor's plan owns X's ring (`can_prepare_source_seek` on X's
///   inventory index): the donor added X, or redeclared it against the plan it was prepared to
///   displace. A source the donor carries unchanged is prepared vacant (its ring is the running
///   plan's, carried at the swap), so it is never fresh here: its chunks are in the ring the
///   running plan renders, and the replacement treats it as a direct swap from the running plan
///   would. This is the same freshness test `check_donation` applies to the donor.
/// * **Fed.** The donor's producer for X has acked anything since preparation: a chunk, a seek or
///   an end of region (it is no longer `unfed`).
/// * **Not donated.** `donation`, the plan `check_donation` returned for this donor and the
///   replacement, has no pair whose donor index is X's. So the replacement does not keep X as the
///   donor declares it: it reverts X to the running plan's declaration, redeclares it another way,
///   or removes it. The donor's ring and its acked chunks would drop with the donor.
///
/// The caller refuses the replacement before its commit as retryable backpressure: once render
/// adopts the donor, X's ring is the running plan's and the same edit commits on it. Read-only;
/// it allocates nothing. Every case stays refused until an issue gives the replacement a way to
/// read those chunks or the owner rules that a removed source's chunks may drop. #1698 keeps the
/// reverted case too: the host restarted its feed of X against the donor, so the replacement
/// starts X fresh ([`crate::PlanStateInventory::across_withdrawn`]) and cannot read the donor's
/// chunks into the running plan's continuing X.
#[must_use]
pub fn drops_acked_fresh_source(donor: DonationSide<'_>, donation: &DonationPlan) -> bool {
    donor.inventory.sources().iter().any(|row| {
        donor.plan.can_prepare_source_seek(row.index as usize)
            && !donation
                .consumers
                .0
                .iter()
                .any(|&(_, from)| from == row.index)
            && donor
                .sources
                .index_of(row.id.as_bytes())
                .and_then(|index| donor.sources.sources[index].provider.owned())
                .is_some_and(|provider| !unfed(provider))
    })
}

/// Whether a fresh producer is exactly as preparation made it: generation 1 (the prepared initial
/// generation), nothing written, no end of region. Such a producer has acked nothing, so dropping
/// it (with the ring it feeds) loses nothing the host is owed.
fn unfed(provider: &HostChunkProvider) -> bool {
    let telemetry = provider.telemetry();
    telemetry.active_generation == SourceGeneration(1)
        && telemetry.cumulative_written_frames == 0
        && !telemetry.end_of_region_submitted
}

/// Whether every producer pair resolves to an occupied entry on both sides and the successor's
/// producer is still [`unfed`]: the producer half's precondition, re-read by [`apply_donation`]
/// before anything moves. A successor producer fed since [`check_donation`] has acked PCM into a
/// ring the donation would hand to the donor, so it refuses the whole donation.
fn producers_resolve(
    successor: &SourceControlSet,
    donor: &SourceControlSet,
    producers: &ProducerDonation,
) -> bool {
    producers.0.iter().all(|&(into, from)| {
        successor
            .sources
            .get(into)
            .is_some_and(|source| source.provider.owned().is_some_and(unfed))
            && donor
                .sources
                .get(from)
                .is_some_and(|source| source.provider.owned().is_some())
    })
}

/// The producer half of [`apply_donation`] (#1344 D3): move each checked pair's donor producer
/// into the successor's entry, so the successor feeds the donated ring. The donor's entry is left
/// [`Producer::Donated`], so a later submit or seek to the donor is refused with
/// [`SourceControlError::Vacated`]
/// and never acked into a ring nobody plays. The successor's own fresh producer, which
/// [`check_donation`] proved unfed, is dropped; its ring moves to the donor with
/// [`donate_consumers`] and drops with it. Infallible on a plan [`check_donation`] returned for
/// these sets; allocates nothing. Returns how many moved. Crate-private: only [`apply_donation`]
/// runs it, after re-reading its precondition.
pub(crate) fn donate_producers(
    successor: &mut SourceControlSet,
    donor: &mut SourceControlSet,
    producers: ProducerDonation,
) -> usize {
    let mut moved = 0;
    for &(into, from) in &producers.0 {
        if let (Some(into), Some(from)) =
            (successor.sources.get_mut(into), donor.sources.get_mut(from))
            && let Some(provider) = from.provider.take_leaving(Producer::Donated)
        {
            debug_assert!(
                into.provider.owned().is_some_and(unfed),
                "a checked successor producer is unfed"
            );
            into.provider = Producer::Owned(provider);
            moved += 1;
        }
    }
    debug_assert_eq!(moved, producers.0.len(), "a checked producer pair resolves");
    moved
}

/// The consumer half of [`apply_donation`] (#1344 D3, D4): swap each checked pair's whole source
/// entries between the two plans (`graph::donate_sources`), ring, queued blocks, generation, read
/// position and held seek. Infallible on a plan [`check_donation`] returned for these plans; on
/// the control thread, on two plans neither of which render owns. Returns how many moved.
/// Crate-private: only [`apply_donation`] runs it, after re-reading its precondition.
pub(crate) fn donate_consumers(
    successor: &mut PreparedRenderPlan,
    donor: &mut PreparedRenderPlan,
    consumers: ConsumerDonation,
) -> usize {
    if consumers.0.is_empty() {
        return 0;
    }
    let moved = graph::donate_sources(successor, donor, &consumers.0);
    debug_assert!(moved, "a checked consumer donation swaps");
    if moved { consumers.0.len() } else { 0 }
}

/// Apply a donation [`check_donation`] returned (#1344 D3). Returns how many sources moved. The
/// donor ends with the successor's fresh, unfed rings and drops them with itself; its donated
/// producer entries are vacant.
///
/// Neither side can have changed since the check: the successor is unpublished and the donor
/// withdrawn, both held by the control thread. Call it after the protocol commit, when the
/// donor is never republished.
///
/// The order is chosen so that no failure can lose PCM the host was acked for, even if a caller
/// breaks that contract (passes sets or plans other than the ones checked, or feeds a successor
/// producer between the check and the apply). First it re-reads, without moving anything, both
/// halves' preconditions: every producer pair resolves, every successor producer is still unfed,
/// and the consumer pairs pass [`graph::can_donate_sources`]. If any fails, nothing moves and it
/// returns `0`: the donor keeps its fed rings and their producers, together, and the successor
/// keeps its own. Then
/// `donate_consumers`, which is all or nothing; only if it moved does `donate_producers` run.
/// So a donated ring and its producer are never separated.
pub fn apply_donation(
    successor: DonationSideMut<'_>,
    donor: DonationSideMut<'_>,
    plan: DonationPlan,
) -> usize {
    let DonationPlan {
        producers,
        consumers,
    } = plan;
    let checked = producers.0.len() == consumers.0.len()
        && producers_resolve(successor.sources, donor.sources, &producers)
        && graph::can_donate_sources(successor.plan, donor.plan, &consumers.0).is_ok();
    if !checked {
        return 0;
    }
    let consumers = donate_consumers(successor.plan, donor.plan, consumers);
    if consumers == 0 {
        return 0;
    }
    let producers = donate_producers(successor.sources, donor.sources, producers);
    debug_assert_eq!(producers, consumers, "both halves move every pair");
    consumers
}

#[cfg(test)]
mod tests {
    //! #1642 D1: the session seek, on a hand-built set of two sources and the timeline.

    use core::num::NonZeroUsize;

    use engine::QuantumFrames;
    use source::{
        PcmSourceConsumer, PcmSourceRing, PcmSourceRingConfig, SourceTimeline,
        TIMELINE_SAMPLE_LIMIT, TimelineConsumer,
    };

    use super::*;

    const Q: u32 = 4;
    const LONG: &[u8] = b"long";
    const SHORT: &[u8] = b"short";

    /// A set of `long` (10,000 frames) and `short` (4,000 frames) and the timeline, all sharing
    /// one published generation, and their consumers, rendered here by hand.
    struct Rig {
        set: SourceControlSet,
        long: PcmSourceConsumer,
        short: PcmSourceConsumer,
        timeline: TimelineConsumer,
        next: u64,
    }

    impl Rig {
        fn new() -> Self {
            let word = PublishedGeneration::new(SourceGeneration(1));
            let ring = PcmSourceRingConfig {
                channel_count: 1,
                quantum_frames: QuantumFrames(Q),
                frame_capacity: 16,
                initial_generation: SourceGeneration(1),
                seek_runway_blocks: 0,
            };
            let mut builder = ControlSourceBuilder::with_capacity(9, 2).expect("builder");
            let mut consumers = Vec::new();
            for (id, frames) in [("long", 10_000), ("short", 4_000)] {
                let (producer, consumer, _) =
                    PcmSourceRing::prepare_host_region(ring, SourceFrame(0), word.clone())
                        .expect("ring");
                builder.push(
                    id,
                    48_000,
                    1,
                    0,
                    frames,
                    PreparedProducer::Fresh(
                        producer.into_host_chunk_provider(SampleRateHz(48_000)),
                    ),
                );
                consumers.push(consumer);
            }
            let mut set = builder.finish(word.clone());
            let (producer, timeline, _) =
                SourceTimeline::prepare(QuantumFrames(Q), NonZeroUsize::MIN, word)
                    .expect("timeline");
            set.set_timeline(producer);
            let short = consumers.pop().expect("short");
            let long = consumers.pop().expect("long");
            Self {
                set,
                long,
                short,
                timeline,
                next: 0,
            }
        }

        /// Render one block of every consumer; returns its source-read sample.
        fn block(&mut self) -> u64 {
            let sample = self.next;
            self.timeline.begin_block_at(sample);
            self.long.begin_block_at(sample);
            self.short.begin_block_at(sample);
            self.next += u64::from(Q);
            sample
        }

        /// `(generation, source frame)` each source's report names, then the timeline at `sample`.
        fn landed(&self, sample: u64) -> ((u64, u64), (u64, u64), i64) {
            let report = |id| {
                let report = self.set.seek_report(id).expect("report");
                (report.generation, report.source_frame)
            };
            (
                report(LONG),
                report(SHORT),
                self.timeline.timeline_at(sample),
            )
        }

        /// Whether every producer still takes a seek of `generation`: no slot is full.
        fn every_slot_empty(&self, generation: u64) -> bool {
            let command = SourceCommand::Seek {
                generation: SourceGeneration(generation),
                frame: SourceFrame(0),
            };
            self.set
                .timeline
                .owned()
                .is_some_and(|timeline| timeline.check_seek(command).is_ok())
                && self.set.sources.iter().all(|source| {
                    source
                        .provider
                        .owned()
                        .is_some_and(|provider| provider.check_seek(command).is_ok())
                })
        }
    }

    /// Test value: red if a source is not clamped to its region, the timeline is, a consumer
    /// takes the seek in another block, or the anchored form applies before its anchor.
    #[test]
    fn both_forms_move_the_timeline_and_every_source_clamped_to_its_region() {
        let mut rig = Rig::new();
        rig.block();
        rig.set.seek_session(2, 6_000, None).expect("session seek");
        assert_eq!(rig.set.published_generation(), 2);
        let sample = rig.block();
        assert_eq!(rig.landed(sample), ((2, 6_000), (2, 4_000), 6_000));

        let anchor = rig.next + 2 * u64::from(Q);
        rig.set
            .seek_session(3, 1_000, Some(anchor))
            .expect("anchored session seek");
        for _ in 0..2 {
            let sample = rig.block();
            let ((long, _), (short, _), _) = rig.landed(sample);
            assert_eq!((long, short), (2, 2), "held before the anchor");
        }
        let sample = rig.block();
        assert_eq!(sample, anchor);
        assert_eq!(rig.landed(sample), ((3, 1_000), (3, 1_000), 1_000));
        // The timeline is not clamped to any region.
        rig.set
            .seek_session(4, 50_000, None)
            .expect("past every region");
        let sample = rig.block();
        assert_eq!(rig.landed(sample), ((4, 10_000), (4, 4_000), 50_000));
    }

    /// Test value: red if any slot is pushed before every check passes (a partial seek). Each
    /// stale case here is also a live producer's own refusal, so the word's rule is
    /// `a_generation_not_above_the_word_is_stale_when_every_producer_takes_it`'s.
    #[test]
    fn a_refusal_pushes_nothing_and_publishes_nothing() {
        let mut rig = Rig::new();
        let stale = SourceControlError::Seek(SourceSeekError::GenerationNotStrictlyIncreasing {
            active: SourceGeneration(1),
            requested: SourceGeneration(1),
        });
        assert_eq!(
            rig.set.seek_session(0, 0, None),
            Err(SourceControlError::GenerationZero)
        );
        assert_eq!(rig.set.seek_session(1, 0, None), Err(stale));
        assert_eq!(stale.diagnostic(), "source.generation.stale");
        assert_eq!(
            rig.set.seek_session(2, 0, Some(1)),
            Err(SourceControlError::Seek(SourceSeekError::AnchorUnaligned))
        );
        assert_eq!(
            rig.set.seek_session(2, TIMELINE_SAMPLE_LIMIT + 1, None),
            Err(SourceControlError::Seek(SourceSeekError::FrameOutOfRange))
        );
        assert_eq!(rig.set.published_generation(), 1);
        assert!(rig.every_slot_empty(2));

        // One full slot refuses all: the short source holds a per-source seek.
        rig.set.seek(SHORT, 2, 10).expect("per-source seek");
        assert_eq!(
            rig.set.published_generation(),
            2,
            "a per-source seek publishes"
        );
        let refused = rig.set.seek_session(3, 0, None);
        assert!(
            matches!(
                refused,
                Err(SourceControlError::Seek(
                    SourceSeekError::Backpressure { .. }
                ))
            ),
            "{refused:?}"
        );
        assert_eq!(rig.set.published_generation(), 2);
        // The timeline's and the long source's slots stayed empty.
        rig.set
            .seek(LONG, 3, 20)
            .expect("the long source's slot is empty");
        let sample = rig.block();
        let (long, short, timeline) = rig.landed(sample);
        assert_eq!((long, short, timeline), ((3, 20), (2, 10), 0));

        // A per-source seek at 5 makes a session seek at 4 stale.
        rig.set.seek(LONG, 5, 0).expect("per-source seek");
        assert_eq!(
            rig.set.seek_session(4, 0, None),
            Err(SourceControlError::Seek(
                SourceSeekError::GenerationNotStrictlyIncreasing {
                    active: SourceGeneration(5),
                    requested: SourceGeneration(4),
                }
            ))
        );
    }

    /// Test value: red if the session seek is not refused against the session's word when every
    /// live producer would take it (the case of a source removed at a higher generation), or the
    /// check and the seek disagree on it.
    #[test]
    fn a_generation_not_above_the_word_is_stale_when_every_producer_takes_it() {
        let mut rig = Rig::new();
        rig.set.published.publish(SourceGeneration(10));
        assert!(rig.every_slot_empty(5), "every live producer takes 5");
        let stale = Err(SourceControlError::Seek(
            SourceSeekError::GenerationNotStrictlyIncreasing {
                active: SourceGeneration(10),
                requested: SourceGeneration(5),
            },
        ));
        assert_eq!(rig.set.check_session_seek(5, 0, None), stale);
        assert_eq!(rig.set.seek_session(5, 0, None), stale);
        assert!(rig.every_slot_empty(5), "nothing pushed");
        assert_eq!(rig.set.published_generation(), 10, "nothing published");
        assert_eq!(rig.set.check_session_seek(11, 0, None), Ok(()));
        assert!(rig.every_slot_empty(11), "a check pushes nothing");
        assert_eq!(
            rig.set.published_generation(),
            10,
            "a check publishes nothing"
        );
        rig.set.seek_session(11, 0, None).expect("above the word");
        assert_eq!(rig.set.published_generation(), 11);
    }

    /// Test value: red if a vacant entry is skipped instead of refusing the whole seek.
    #[test]
    fn a_vacant_entry_refuses_the_whole_seek() {
        let mut rig = Rig::new();
        let timeline = core::mem::replace(&mut rig.set.timeline, Producer::Handed);
        assert_eq!(
            rig.set.seek_session(2, 0, None),
            Err(SourceControlError::Vacated)
        );
        rig.set.timeline = timeline;
        let provider = core::mem::replace(&mut rig.set.sources[1].provider, Producer::Awaiting);
        assert_eq!(
            rig.set.seek_session(2, 0, None),
            Err(SourceControlError::Vacated)
        );
        rig.set.sources[1].provider = provider;
        assert!(rig.every_slot_empty(2));
        assert_eq!(rig.set.published_generation(), 1);
        rig.set
            .seek_session(2, 0, None)
            .expect("every entry occupied");
    }

    /// A set of mono sources `0..1_000` frames long, each prepared as `entries` names it, on
    /// `word`. The fresh rings' consumers are dropped: these tests move producers, never render.
    fn hand_built(word: &PublishedGeneration, entries: &[(&str, bool)]) -> SourceControlSet {
        let ring = PcmSourceRingConfig {
            channel_count: 1,
            quantum_frames: QuantumFrames(Q),
            frame_capacity: 16,
            initial_generation: SourceGeneration(1),
            seek_runway_blocks: 0,
        };
        let id_bytes = entries.iter().map(|(id, _)| id.len()).sum();
        let mut builder =
            ControlSourceBuilder::with_capacity(id_bytes, entries.len()).expect("builder");
        for &(id, carried) in entries {
            let producer = if carried {
                PreparedProducer::Carried
            } else {
                let (producer, _, _) =
                    PcmSourceRing::prepare_host_region(ring, SourceFrame(0), word.clone())
                        .expect("ring");
                PreparedProducer::Fresh(producer.into_host_chunk_provider(SampleRateHz(48_000)))
            };
            builder.push(id, 48_000, 1, 0, 1_000, producer);
        }
        builder.finish(word.clone())
    }

    /// Test value: red if the give-back of a withdrawn candidate's producers fills a running
    /// entry by vacancy alone, so an entry that never handed its producer to the withdrawn plan
    /// (one still awaiting its own predecessor's, or one whose producer went elsewhere) takes a
    /// producer of the withdrawn plan's that is not its own: a fresh one, whose ring drops with
    /// that plan, so the running set would ack PCM into a ring nobody renders. Each of the two
    /// give-back rules (the entry handed its producer on; the withdrawn entry carries the ring) is
    /// needed by one case. An adoption, which fills awaited entries alone, does not refill a
    /// handed-on one either. The timeline's own branches follow the same rules; production never
    /// gives one back (every successor takes the timeline), so only this case reaches them.
    #[test]
    fn a_give_back_returns_only_the_producers_the_withdrawn_set_took() {
        let word = PublishedGeneration::new(SourceGeneration(1));
        // P0 renders `x` from its own ring and carries `y`, whose producer never arrived.
        let mut running = hand_built(&word, &[("x", false), ("y", true)]);
        // A carries `x` from P0 and redeclared `y`: its ring and producer are A's own.
        let mut withdrawn = hand_built(&word, &[("x", true), ("y", false)]);
        assert_eq!(withdrawn.adopt_persisting(&mut running), 1, "A takes x");
        assert_eq!(running.admission(b"x"), Err(SourceControlError::Vacated));
        // A is withdrawn: P0 takes back what it handed on, and only that.
        assert_eq!(
            running.reclaim_from_withdrawn(&mut withdrawn),
            1,
            "only x returns"
        );
        assert!(running.admission(b"x").is_ok());
        assert_eq!(running.admission(b"y"), Err(SourceControlError::Vacated));
        assert!(withdrawn.admission(b"y").is_ok(), "A keeps its own y");

        // P0 handed `x` to another set; the withdrawn A redeclared `x`, so its producer is A's own.
        let mut running = hand_built(&word, &[("x", false)]);
        let mut other = hand_built(&word, &[("x", true)]);
        let mut withdrawn = hand_built(&word, &[("x", false)]);
        assert_eq!(other.adopt_persisting(&mut running), 1);
        assert_eq!(
            running.reclaim_from_withdrawn(&mut withdrawn),
            0,
            "A's x stays"
        );
        assert_eq!(
            running.adopt_persisting(&mut withdrawn),
            0,
            "x is not awaited"
        );
        assert_eq!(running.admission(b"x"), Err(SourceControlError::Vacated));
        assert!(withdrawn.admission(b"x").is_ok());

        // P0 awaits `y`; A carries a `y` whose producer did not come from P0.
        let mut running = hand_built(&word, &[("y", true)]);
        let mut withdrawn = hand_built(&word, &[("y", true)]);
        let mut other = hand_built(&word, &[("y", false)]);
        assert_eq!(withdrawn.adopt_persisting(&mut other), 1);
        assert_eq!(
            running.reclaim_from_withdrawn(&mut withdrawn),
            0,
            "P0 handed no y"
        );
        assert!(withdrawn.admission(b"y").is_ok());

        // The timeline, by the same rules (every successor carries it, so no `carried` flag).
        let owning = |word: &PublishedGeneration| {
            let mut set = hand_built(word, &[]);
            let (producer, _, _) =
                SourceTimeline::prepare(QuantumFrames(Q), NonZeroUsize::MIN, word.clone())
                    .expect("timeline");
            set.set_timeline(producer);
            set
        };
        let holds = |set: &SourceControlSet| {
            set.seek_report(TIMELINE_SEEK_REPORT_ID) != Err(SourceControlError::Vacated)
        };
        let mut running = owning(&word);
        let mut withdrawn = hand_built(&word, &[]);
        withdrawn.adopt_persisting(&mut running);
        assert!(
            holds(&withdrawn) && !holds(&running),
            "A takes the timeline"
        );
        let mut other = owning(&word);
        running.adopt_persisting(&mut other);
        assert!(!holds(&running), "a handed-on timeline is not awaited");
        let mut awaiting = hand_built(&word, &[]);
        awaiting.reclaim_from_withdrawn(&mut other);
        assert!(!holds(&awaiting), "an awaited timeline was handed nothing");
        running.reclaim_from_withdrawn(&mut withdrawn);
        assert!(
            holds(&running) && !holds(&withdrawn),
            "the timeline returns"
        );
    }

    /// #1644 D2: the browser feed drain tells a stale-generation refusal from every other
    /// `invalidArgument` by the first bytes of the session diagnostic, so no other code may begin
    /// with `source.generation.stale` or be a proper prefix of it. `slot` is an exhaustive match,
    /// so a new variant does not compile until it is listed here.
    #[test]
    fn no_other_diagnostic_collides_with_the_stale_generation_code() {
        use SourceControlError as E;
        const fn slot(error: SourceControlError) -> usize {
            match error {
                E::UnknownSource => 0,
                E::RegionOverflow => 1,
                E::OutsideRegion => 2,
                E::EndOfRegionMismatch => 3,
                E::GenerationZero => 4,
                E::Chunk(HostChunkError::WrongSampleRate { .. }) => 5,
                E::Chunk(HostChunkError::StaleGeneration { .. }) => 6,
                E::Chunk(HostChunkError::ChannelCount { .. }) => 7,
                E::Chunk(HostChunkError::PlaneLength { .. }) => 8,
                E::Chunk(HostChunkError::FrameCount { .. }) => 9,
                E::Chunk(HostChunkError::NonContiguous { .. }) => 10,
                E::Chunk(HostChunkError::EndOfRegionAlreadySubmitted) => 11,
                E::Chunk(HostChunkError::Full { .. }) => 12,
                E::Chunk(HostChunkError::InternalInvariant) => 13,
                E::Seek(SourceSeekError::GenerationZero) => 14,
                E::Seek(SourceSeekError::GenerationNotStrictlyIncreasing { .. }) => 15,
                E::Seek(SourceSeekError::Backpressure { .. }) => 16,
                E::Seek(SourceSeekError::AnchorUnaligned) => 17,
                E::Seek(SourceSeekError::FrameOutOfRange) => 18,
                E::InvalidId => 19,
                E::Vacated => 20,
                E::ReportBusy => 21,
            }
        }
        let generation = SourceGeneration(1);
        let frame = SourceFrame(0);
        let every = [
            E::UnknownSource,
            E::RegionOverflow,
            E::OutsideRegion,
            E::EndOfRegionMismatch,
            E::GenerationZero,
            E::Chunk(HostChunkError::WrongSampleRate {
                expected: SampleRateHz(48_000),
                actual: SampleRateHz(44_100),
            }),
            E::Chunk(HostChunkError::StaleGeneration {
                active: generation,
                submitted: generation,
            }),
            E::Chunk(HostChunkError::ChannelCount {
                expected: 2,
                actual: 1,
            }),
            E::Chunk(HostChunkError::PlaneLength { expected_frames: 4 }),
            E::Chunk(HostChunkError::FrameCount {
                quantum_frames: 4,
                submitted_frames: 3,
                end_of_region: false,
            }),
            E::Chunk(HostChunkError::NonContiguous {
                expected: frame,
                actual: frame,
            }),
            E::Chunk(HostChunkError::EndOfRegionAlreadySubmitted),
            E::Chunk(HostChunkError::Full { full_count: 1 }),
            E::Chunk(HostChunkError::InternalInvariant),
            E::Seek(SourceSeekError::GenerationZero),
            E::Seek(SourceSeekError::GenerationNotStrictlyIncreasing {
                active: generation,
                requested: generation,
            }),
            E::Seek(SourceSeekError::Backpressure { full_count: 1 }),
            E::Seek(SourceSeekError::AnchorUnaligned),
            E::Seek(SourceSeekError::FrameOutOfRange),
            E::InvalidId,
            E::Vacated,
            E::ReportBusy,
        ];
        let slots: Vec<usize> = every.iter().map(|error| slot(*error)).collect();
        assert_eq!(
            slots,
            (0..every.len()).collect::<Vec<_>>(),
            "every variant once, in order"
        );
        const STALE: &str = "source.generation.stale";
        for error in every {
            let code = error.diagnostic();
            if code == STALE {
                continue;
            }
            assert!(
                !code.starts_with(STALE),
                "{error:?} begins with the stale code: {code}"
            );
            assert!(
                !STALE.starts_with(code),
                "{error:?} is a prefix of the stale code: {code}"
            );
        }
    }
}
