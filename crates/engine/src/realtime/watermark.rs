//! The applied-revision watermark (#1314, decision 15 D15-17).
//!
//! # What it answers
//!
//! "Which committed revision is audible, and since which render sample?" The control thread
//! commits a revision and returns at once; render applies it at some later block that the control
//! thread cannot predict (a live record is drained by whatever render call comes next, and a
//! rebuild applies when render adopts its candidate, which a paused host never reaches). So
//! completion is *observed*, never awaited: render publishes `(revision, first sample in effect,
//! outcome flags)` here, with saturating per-outcome counters, and any thread reads it.
//!
//! The revision is the highest one that is in effect together with every revision before it.
//! Revisions travel with their plans (each plan's own revision gate, #1502), and render takes its
//! block snapshot only from the gate of the plan it runs, so the watermark never reports a
//! revision before the plan that carries it renders.
//!
//! # Why this is a level, not a queue
//!
//! Revisions are monotone, so the newest one answers the host's question for every older one: a
//! watermark at `b` says every revision up to `b` is in effect. Nothing is queued, so nothing can
//! back up, be dropped or make render wait. Render overwrites the record in bounded time; a reader
//! that missed an advance loses nothing, because the counters accumulate and the revision only
//! grows. That is the acked-batch answer: no ack can precede a drop, because there is no queue to
//! drop from (#1314 D8).
//!
//! # The record
//!
//! The words travel through the engine's one single-writer seqlock (`seqlock.rs`, B15-R1): six
//! words, render the only writer, a reader bounded to `seqlock::MAXIMUM_READ_ATTEMPTS` attempts
//! before it reports [`WatermarkBusy`].

use super::seqlock::{
    PublishedWordsReader, PublishedWordsWriter, published_words, published_words_retained_bytes,
};

/// Outcome flag: the covered revisions completed exactly as committed.
pub const OUTCOME_EXACT: u64 = 1;
/// Outcome flag: a warm successor could not adopt and its revisions completed through the
/// transition (#1397, #1358).
pub const OUTCOME_TRANSITION_FALLBACK: u64 = 2;
/// Outcome flag: the covered revisions include ones folded in from candidates a newer candidate
/// replaced before render adopted them (#1310).
pub const OUTCOME_SUPERSEDED: u64 = 4;

/// How a candidate's own revisions complete when render adopts it (#1314 D1, D5).
///
/// Written only while control owns the candidate, before publication. In this slice every
/// candidate is [`Self::Exact`]; the only writer of [`Self::TransitionFallback`] is the
/// transition fallback (#1397, #1358).
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CandidateOutcome {
    /// The candidate's revisions complete as [`OUTCOME_EXACT`].
    #[default]
    Exact = 0,
    /// The candidate is the transition a warm successor fell back to; its revisions complete as
    /// [`OUTCOME_TRANSITION_FALLBACK`].
    TransitionFallback = 1,
}

/// One whole watermark: what a reader gets, and the only thing that crosses.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanWatermark {
    /// The highest committed revision in effect together with every revision before it.
    pub revision: u64,
    /// The absolute render sample of the first block in which `revision` was in effect: a live
    /// value's ramp starts at it, and a rebuilt plan renders from it. `0` for the initial revision.
    pub first_sample: u64,
    /// The OR of the `OUTCOME_*` flags over the revisions the last advance covered.
    pub outcome_flags: u64,
    /// Saturating count of revisions completed as [`OUTCOME_EXACT`].
    pub exact: u64,
    /// Saturating count of revisions completed as [`OUTCOME_TRANSITION_FALLBACK`].
    pub transition_fallback: u64,
    /// Saturating count of revisions completed as [`OUTCOME_SUPERSEDED`].
    pub superseded: u64,
}

/// A read gave up after `MAXIMUM_READ_ATTEMPTS` torn attempts; the caller retries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatermarkBusy;

/// The words of one watermark in the shared record, in [`PlanWatermark`]'s field order.
const WATERMARK_WORDS: usize = 6;

impl PlanWatermark {
    const fn to_words(self) -> [u64; WATERMARK_WORDS] {
        [
            self.revision,
            self.first_sample,
            self.outcome_flags,
            self.exact,
            self.transition_fallback,
            self.superseded,
        ]
    }

    const fn from_words(words: [u64; WATERMARK_WORDS]) -> Self {
        let [
            revision,
            first_sample,
            outcome_flags,
            exact,
            transition_fallback,
            superseded,
        ] = words;
        Self {
            revision,
            first_sample,
            outcome_flags,
            exact,
            transition_fallback,
            superseded,
        }
    }
}

/// Exact engine-owned bytes of the shared watermark record, with its shared-owner header.
pub(crate) fn watermark_retained_bytes() -> usize {
    published_words_retained_bytes::<WATERMARK_WORDS>()
}

/// Render's half: the only writer, with the render-local state an advance needs.
#[derive(Debug)]
pub(crate) struct WatermarkWriter {
    record: PublishedWordsWriter<WATERMARK_WORDS>,
    /// What the record holds, plus the spans of claims settled since the last advance (#1482
    /// D1); render is its only writer, so it never loads it back.
    current: PlanWatermark,
    /// The newest candidate claimed since the last advance, not yet settled (#1314 D5).
    pending: Option<Claim>,
    /// The `OUTCOME_*` flags of the claims settled since the last advance (#1482 D1).
    pending_flags: u64,
}

/// One claimed candidate, as render holds it until its revisions complete.
#[derive(Clone, Copy, Debug)]
struct Claim {
    /// The candidate's own structural revision (#1482 R1).
    revision: u64,
    superseded: u64,
    outcome: CandidateOutcome,
}

/// Any-thread reader handle. Cloning it shares the one record.
#[derive(Clone, Debug)]
pub struct PlanWatermarkReader {
    record: PublishedWordsReader<WATERMARK_WORDS>,
}

/// Make the record with the initial watermark `(initial_revision, 0, EXACT)` and every counter 0.
pub(crate) fn plan_watermark(initial_revision: u64) -> (WatermarkWriter, PlanWatermarkReader) {
    let current = PlanWatermark {
        revision: initial_revision,
        first_sample: 0,
        outcome_flags: OUTCOME_EXACT,
        exact: 0,
        transition_fallback: 0,
        superseded: 0,
    };
    let (record, reader) = published_words(current.to_words());
    let reader = PlanWatermarkReader { record: reader };
    (
        WatermarkWriter {
            record,
            current,
            pending: None,
            pending_flags: 0,
        },
        reader,
    )
}

// REALTIME_POLICY_BEGIN
impl WatermarkWriter {
    /// Render claimed the candidate of structural revision `revision`: hold its `superseded`
    /// count and its `outcome` for the first advance after the claim (#1314 D5).
    ///
    /// If an earlier claim is still held (two claims between two advances), it is settled first
    /// (#1482 D1): the revisions above the record's revision up to the earlier candidate's own
    /// revision complete with that candidate's `superseded` count and outcome, the local revision
    /// moves up to it, and its flags wait for the next advance. A settle stores nothing to the
    /// shared record. So each claim's revisions complete with that claim's own outcome, and none
    /// is lost to a block that published nothing.
    #[inline]
    pub(crate) fn note_claim(&mut self, revision: u64, superseded: u64, outcome: CandidateOutcome) {
        let claim = Claim {
            revision,
            superseded,
            outcome,
        };
        if let Some(earlier) = self.pending.replace(claim) {
            let covered = earlier.revision.saturating_sub(self.current.revision);
            self.pending_flags |= self.complete(covered, earlier);
            self.current.revision = self.current.revision.max(earlier.revision);
        }
    }

    /// Advance to `revision`, in effect from `first_sample`, if it is above the published one;
    /// otherwise write nothing. Returns the whole watermark it just published, or `None` when it
    /// wrote nothing (#1349 D1).
    ///
    /// An advance from `a` to `b` completes the `b - a` revisions that no settled claim already
    /// completed (#1482 D1). The held claim's `superseded` count goes to its counter with
    /// [`OUTCOME_SUPERSEDED`]; the rest goes to `exact`, or to `transition_fallback` if the held
    /// claim's outcome was the fallback, with its flag. With no claim held they are
    /// [`OUTCOME_EXACT`]. The published `outcome_flags` are the union of the advance's flags and
    /// every settled claim's.
    #[inline]
    pub(crate) fn advance(&mut self, revision: u64, first_sample: u64) -> Option<PlanWatermark> {
        if revision <= self.current.revision {
            return None;
        }
        let covered = revision - self.current.revision;
        let claim = self.pending.take().unwrap_or(Claim {
            revision,
            superseded: 0,
            outcome: CandidateOutcome::Exact,
        });
        let flags = core::mem::take(&mut self.pending_flags) | self.complete(covered, claim);
        self.current.revision = revision;
        self.current.first_sample = first_sample;
        self.current.outcome_flags = flags;
        self.store(self.current);
        Some(self.current)
    }

    /// Count `covered` revisions completed by `claim` into the local counters: its `superseded`
    /// count as superseded, the rest with its outcome. Returns the flags they set.
    #[inline]
    fn complete(&mut self, covered: u64, claim: Claim) -> u64 {
        let rest = covered.saturating_sub(claim.superseded);
        let mut flags = 0;
        if claim.superseded != 0 {
            flags |= OUTCOME_SUPERSEDED;
            self.current.superseded = self.current.superseded.saturating_add(claim.superseded);
        }
        if rest != 0 {
            match claim.outcome {
                CandidateOutcome::Exact => {
                    flags |= OUTCOME_EXACT;
                    self.current.exact = self.current.exact.saturating_add(rest);
                }
                CandidateOutcome::TransitionFallback => {
                    flags |= OUTCOME_TRANSITION_FALLBACK;
                    self.current.transition_fallback =
                        self.current.transition_fallback.saturating_add(rest);
                }
            }
        }
        flags
    }

    /// Overwrite the record with `watermark`. Wait-free: two counter stores and six word stores.
    #[inline]
    fn store(&mut self, watermark: PlanWatermark) {
        self.record.publish(&watermark.to_words());
    }
}
// REALTIME_POLICY_END

impl PlanWatermarkReader {
    /// The newest whole watermark, or [`WatermarkBusy`] if every one of the bounded attempts
    /// landed inside a publication. Never a partial record: the words are taken between two equal
    /// even counter reads, so what comes back is exactly one publication.
    pub fn read(&self) -> Result<PlanWatermark, WatermarkBusy> {
        self.record
            .read()
            .map(PlanWatermark::from_words)
            .map_err(|_| WatermarkBusy)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CandidateOutcome, OUTCOME_EXACT, OUTCOME_SUPERSEDED, OUTCOME_TRANSITION_FALLBACK,
        plan_watermark,
    };

    /// #1482 gate 1: two claims between two advances. Each claim's revisions complete with that
    /// claim's own outcome and `superseded` count, and the one advance publishes the union of
    /// their flags.
    #[test]
    fn two_claims_between_advances_complete_each_span_with_its_own_outcome() {
        let r = 7;
        let s = 4096;
        let (mut writer, reader) = plan_watermark(r);
        writer.note_claim(r + 2, 0, CandidateOutcome::Exact);
        writer.note_claim(r + 5, 0, CandidateOutcome::TransitionFallback);
        let published = writer.advance(r + 5, s).expect("an advance");
        assert_eq!(reader.read(), Ok(published));
        assert_eq!((published.revision, published.first_sample), (r + 5, s));
        assert_eq!(
            published.outcome_flags,
            OUTCOME_EXACT | OUTCOME_TRANSITION_FALLBACK
        );
        assert_eq!(
            (
                published.exact,
                published.transition_fallback,
                published.superseded
            ),
            (2, 3, 0)
        );

        // With `superseded` on both claims, each count lands on its own span.
        let (mut writer, reader) = plan_watermark(r);
        writer.note_claim(r + 3, 1, CandidateOutcome::TransitionFallback);
        writer.note_claim(r + 7, 2, CandidateOutcome::Exact);
        let published = writer.advance(r + 8, s).expect("an advance");
        assert_eq!(reader.read(), Ok(published));
        assert_eq!(
            published.outcome_flags,
            OUTCOME_EXACT | OUTCOME_TRANSITION_FALLBACK | OUTCOME_SUPERSEDED
        );
        assert_eq!(
            (
                published.superseded,
                published.transition_fallback,
                published.exact
            ),
            (3, 2, 3)
        );
    }
}
