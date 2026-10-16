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
    HostChunkError, HostChunkProvider, HostPlanarChunk, SourceCommand, SourceFrame,
    SourceGeneration, SourceSeekError, SubmitReport,
};

use crate::PlanStateInventory;

/// One host-owned producer endpoint plus the immutable facts a submission is checked against.
pub(crate) struct ControlSource {
    id_offset: usize,
    id_bytes: usize,
    sample_rate_hz: u32,
    channel_count: u32,
    region_start: u64,
    region_end: u64,
    /// `None` while vacant: the plan was prepared to carry this source's ring from its
    /// predecessor, and the producer arrives through [`SourceControlSet::adopt_persisting`].
    provider: Option<HostChunkProvider>,
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
    /// The source exists but this set holds no producer for it: the set belongs to a successor
    /// plan whose producer has not been handed over yet ([`SourceControlSet::adopt_persisting`]),
    /// or to a predecessor whose producer has already moved to its successor. Neither
    /// backpressure nor an engine invariant: the caller feeds the newest set.
    Vacated,
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
        let provider = source
            .provider
            .as_mut()
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
    pub fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
    ) -> Result<(), SourceControlError> {
        self.queue_seek(id, generation, frame, None)
    }

    /// Queue one strictly increasing, generation-tagged seek whose source `frame` enters the graph
    /// in the block that starts at absolute render sample `anchor_sample` (issue #1274).
    ///
    /// `anchor_sample` is on the plan's absolute render clock, the one a host renders with and
    /// every plan swap continues; it names when `frame` enters the graph, and the output hears it
    /// the plan's latency later. It must be a multiple of the quantum, or the seek is refused with
    /// `source.seek.anchor_unaligned`. The region and generation rules are [`Self::seek`]'s.
    ///
    /// On success the source accepts the new generation's PCM from `frame` at once; submit it
    /// before the anchor block renders and the source plays it from exactly that block, even when
    /// the render pops that PCM before it pops the seek. Should the render reach the seek only
    /// after the anchor block has passed, it starts at `frame` plus the lateness, so it stays in
    /// time. Until the anchor block, PCM already queued for the playing generation keeps playing;
    /// when that runs out, the source renders silence and reports an underrun.
    pub fn seek_at(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
        anchor_sample: u64,
    ) -> Result<(), SourceControlError> {
        self.queue_seek(id, generation, frame, Some(anchor_sample))
    }

    fn queue_seek(
        &mut self,
        id: &[u8],
        generation: u64,
        frame: u64,
        anchor_sample: Option<u64>,
    ) -> Result<(), SourceControlError> {
        let index = self.index_of(id).ok_or(SourceControlError::UnknownSource)?;
        let source = &mut self.sources[index];
        let provider = source
            .provider
            .as_mut()
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

    /// Move into this set the producer of every source it holds vacant, from `predecessor`'s
    /// entry with the same ID, leaving that entry vacant. Returns how many producers moved.
    ///
    /// A host calls this after preparing a successor with
    /// [`crate::prepare_host_runtime_successor`] (or its live-control form) and after its last
    /// fallible step, so the move is never undone. Infallible and allocation-free: a vacant
    /// entry whose ID the predecessor does not hold, or holds vacant itself, stays vacant and
    /// refuses PCM with [`SourceControlError::Vacated`]. Keyed by source ID, never by index.
    ///
    /// The producers move before the plan swap that moves their consumers. Should that swap's
    /// carry report `PredecessorMismatch` -- unreachable on both hosts by construction (issue
    /// #1269 P11) -- the successor's vacant sources render `+0.0` from the swap block on, since
    /// their producers have already moved here and their rings stay with the retired plan.
    pub fn adopt_persisting(&mut self, predecessor: &mut SourceControlSet) -> usize {
        let mut moved = 0;
        for index in 0..self.sources.len() {
            if self.sources[index].provider.is_some() {
                continue;
            }
            let Some(from) = predecessor.index_of(self.id(&self.sources[index])) else {
                continue;
            };
            if let Some(provider) = predecessor.sources[from].provider.take() {
                self.sources[index].provider = Some(provider);
                moved += 1;
            }
        }
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
        provider: Option<HostChunkProvider>,
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
            successor.sources.sources[into].provider.as_ref(),
            donor.sources.sources[from].provider.as_ref(),
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
/// it allocates nothing. Issue TBD42 lifts the reverted case only (it carries X from the running
/// plan); the others stay refused until an issue gives the replacement a way to read those chunks
/// or the owner rules that a removed source's chunks may drop.
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
                .and_then(|index| donor.sources.sources[index].provider.as_ref())
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
            .is_some_and(|source| source.provider.as_ref().is_some_and(unfed))
            && donor
                .sources
                .get(from)
                .is_some_and(|source| source.provider.is_some())
    })
}

/// The producer half of [`apply_donation`] (#1344 D3): move each checked pair's donor producer
/// into the successor's entry, so the successor feeds the donated ring. The donor's entry is left
/// vacant, so a later submit or seek to the donor is refused with [`SourceControlError::Vacated`]
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
            && let Some(provider) = from.provider.take()
        {
            debug_assert!(
                into.provider.as_ref().is_some_and(unfed),
                "a checked successor producer is unfed"
            );
            into.provider = Some(provider);
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
