//! Structurally immutable prepared plan and bounded silence reference renderer.

use super::{
    BufferArena, BufferArenaError, GateKey, GatePin, LiveSnapshot, LiveStamp, PlanarBufferMut,
    RevisionGate, StampRefused,
};
use crate::{QuantumFrames, SampleRateHz, is_launch_sample_rate};
use core::{alloc::Layout, cell::Cell, num::NonZeroUsize, sync::atomic::AtomicUsize};
use std::sync::Arc;

/// Why an exclusive live-response capture was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotError {
    /// The active runtime has no owner that can provide the requested snapshot.
    Unsupported,
    /// The requested stable track is absent from the active plan.
    MissingTrack,
    /// Caller-owned snapshot storage cannot hold the selected owners or sections.
    Capacity,
    /// The owner or sink rejected a section shape or other immutable capture detail.
    InvalidShape,
    /// A retained owner returned a typed failure while its words were being copied.
    Owner,
}

/// Whether an owner supplied a response snapshot at this boundary.
///
/// A declared owner can be present in the graph while having no native response provider.  That
/// is an ordered exclusion in the returned snapshot, rather than a failed capture.  Other
/// failures remain [`ResponseSnapshotError`]s so a required provider cannot silently disappear.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotAvailability {
    /// The owner copied one or more response sections.
    Provided,
    /// The owner is present but declares no response provider for this query.
    DeclaredUnavailable,
}

/// Stable metadata for one owner emitted into a response snapshot sink.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotOwnerInfo<'a> {
    /// The selected track's stable graph identity.
    pub track_id: &'a str,
    /// The authoritative native provider identity, independent of the session node identity.
    pub native_id: &'a str,
    /// The owner's stable slot/effect identity.
    pub stable_id: &'a str,
    /// Native rack/stage identity, with zero reserved for the input filter owner.
    pub rack: u8,
    /// Signal-chain order among the selected track's response-capable owners.
    pub slot: u32,
    /// Owner-specific response kind.
    pub kind: u32,
    /// Bypass state at the same boundary as the copied words.
    pub bypassed: bool,
    /// Whether this owner supplied response sections or was retained as an ordered exclusion.
    pub availability: ResponseSnapshotAvailability,
}

/// One bounded section in an opaque response snapshot copy.
///
/// The engine owns this fixed-size record so an executor can hand borrowed sections to a caller
/// sink without exposing graph or effect-contract types. `word_count` is validated before the
/// record crosses the seam; unused words remain zero for deterministic caller storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotSection {
    /// Stable section identifier within the owner.
    pub id: u32,
    /// Owner-specific response kind.
    pub kind: u32,
    /// Whether this section is currently enabled.
    pub enabled: bool,
    /// Number of meaningful words in `words`.
    pub word_count: u8,
    /// Fixed bounded payload copied from the prepared owner.
    pub words: [u32; 7],
}

/// Caller-owned destination for an exclusive response snapshot.
///
/// The engine knows only this bounded callback surface. A host implements the sink with its own
/// preallocated section/identity buffers; graph and effect crates remain below the engine crate.
pub trait ResponseSnapshotSink {
    /// Copy one owner in actual graph order. The slices borrow fixed records owned by the
    /// prepared executor and are valid only for the duration of this callback.
    fn copy_owner(
        &mut self,
        owner: ResponseSnapshotOwnerInfo<'_>,
        left: &[ResponseSnapshotSection],
        right: &[ResponseSnapshotSection],
    ) -> Result<(), ResponseSnapshotError>;
}

/// Caller-owned request passed to the exclusive prepared-plan owner.
pub struct ResponseSnapshotRequest<'a> {
    /// Stable graph track identity to select.
    pub track_id: &'a str,
    /// Bounded destination that receives the immutable owner copy.
    pub sink: &'a mut dyn ResponseSnapshotSink,
}

/// Boundary metadata for one completed response capture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotCapture {
    /// The active plan's actual next sample at capture.
    pub captured_sample: u64,
    /// Number of response-capable owners copied in graph order.
    pub owners: u32,
}

/// Exact rate, quantum, and external output shape accepted by a plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderEnvelope {
    /// Caller-selected engine rate; no implicit sample-rate conversion occurs.
    pub sample_rate: SampleRateHz,
    /// Fixed number of frames in every full render block.
    pub quantum: QuantumFrames,
    /// Required external planar PCM output channel count.
    pub output_channels: NonZeroUsize,
}
impl RenderEnvelope {
    /// Validate a launch-supported rate and nonzero quantum.
    pub fn validate(self) -> Result<Self, RenderError> {
        if !is_launch_sample_rate(self.sample_rate) {
            return Err(RenderError::UnsupportedRate);
        }
        if self.quantum.0 == 0 {
            return Err(RenderError::InvalidEnvelope);
        }
        Ok(self)
    }
}
/// Immutable input accepted by the prepare phase.
pub struct PrepareRenderPlan<'a> {
    /// Control-plane identity used in deterministic traces.
    pub plan_id: u64,
    /// Immutable external render shape.
    pub envelope: RenderEnvelope,
    /// Internal planar scratch buffers allocated during preparation.
    pub scratch: &'a [super::PlanarBufferSpec],
}
/// The plan's frozen control-plane identity: its `plan_id` and validated [`RenderEnvelope`].
///
/// Both are fixed by [`PreparedRenderPlan::prepare`] and immutable for the plan's life. Graph
/// topology is deliberately *not* here -- it is owned by the [`PreparedPlanExecutor`] the plan
/// holds behind its dispatch seam -- so this stays a two-field identity record rather than
/// growing a schema.
#[derive(Debug)]
pub struct PreparedProgram {
    plan_id: u64,
    envelope: RenderEnvelope,
}
impl PreparedProgram {
    /// Control-plane plan identity.
    #[must_use]
    pub const fn plan_id(&self) -> u64 {
        self.plan_id
    }
    /// Immutable render envelope.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.envelope
    }
}
/// Render outputs. This only owns borrowed scalar/slice references.
pub struct RenderIo<'a> {
    /// External planar PCM output.
    pub output: PlanarBufferMut<'a>,
}

/// One scheduling unit's collapse-eligibility row: the query surface the mono collapse's dispatch
/// reads (mono-collapse M1).
///
/// # Why a row per unit rather than a pair of totals
///
/// [`PreparedPlanExecutor::symmetry_counters`] is a **census** and says so; it answers "how many
/// lanes of this plan could collapse" and nothing more. The collapse decides per *cohort* -- the
/// unit of savable work is a whole plane pass over one bank chain's lane vector, and a SIMD op
/// executes every lane whether that lane needs it or not -- so the question the dispatch actually
/// asks is "does **this** chain's every active lane hold", and the census cannot answer it. This
/// can.
///
/// # The three joins this carries, and why each is here
///
/// * **Eligibility, per unit.** `eligible_lanes == lanes` is the cohort-level answer.
/// * **The track join.** The structural half of the witness is keyed by *track id*
///   (`session_structural_symmetry`, because the planner needs the class before any prepared
///   object exists) and the runtime half is keyed by anonymous *lanes*. Nothing else in the plan
///   relates the two, so a caller holding both halves could not conjoin them at all.
///   [`lane_tracks`](Self::lane_tracks) is that relation, in lane order.
/// * **The seam classification.** A fader or matrix bank's witness is
///   [vacuously symmetric](Self::witness_is_vacuous): the collapse duplicates its one computed
///   plane *into* those stages, so their per-channel words are free to differ and are deliberately
///   never checked. A caller that read such a unit's `eligible_lanes == lanes` as "this cohort may
///   collapse" would be reading an unconditional `true` as evidence.
///   [`upstream_of_seam_stages`](Self::upstream_of_seam_stages) is what makes that distinguishable
///   rather than a footnote.
///
/// Nothing on the render path builds or reads this. It is materialised on demand, off render,
/// after the render audit is disarmed, and it allocates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanUnitEligibility {
    /// This unit's position in the built runtime's execution order.
    pub unit: u32,
    /// `true` for a homogeneous bank chain, `false` for a single dispatched op.
    pub banked: bool,
    /// Issue #970: `true` only for a bank chain every lane of which begins at its track's
    /// `PostInputBuiltins` stage, the one unit the mono collapse (`arm_mono_collapse`) may arm on
    /// the structural join alone; `false` for every later chain of a split strip and for a single
    /// op. Since issue #987 a later chain whose every lane gathers a same-track bank producer is
    /// armed too, and collapses only on a block its producers rendered with bit-identical planes;
    /// [`armed_on_input`](Self::armed_on_input) marks it. A caller reading
    /// [`all_lanes_eligible`](Self::all_lanes_eligible) as collapse evidence tests this and
    /// [`witness_is_vacuous`](Self::witness_is_vacuous) first.
    pub gathers_track_input: bool,
    /// Issue #987: `true` for a later bank chain that the mono collapse armed on its producers:
    /// every lane gathers a same-track bank producer's planes, and the chain collapses only on a
    /// block whose producers rendered them bit-identical. `false` for an input-gathering chain,
    /// an unarmed chain and a single op. Evidence only.
    pub armed_on_input: bool,
    /// Stages this unit renders: a chain's bound slots, or 1 for a single op.
    pub stages: u32,
    /// Of those, the stages that sit **upstream of the fader/matrix seam** -- the stages a
    /// collapsed track would have computed once. Zero makes the unit's witness vacuous.
    pub upstream_of_seam_stages: u32,
    /// Lane -> the track this lane renders, in lane order. One entry per **active** lane; a
    /// single op renders one "lane", its own node, named by its strip: a track, or since #1200 a
    /// submix strip, whose stages are track stages named by the bus. Empty for a node that names no
    /// strip (a route, a graph-level submix node, the output, a compensation delay).
    pub lane_tracks: Box<[Box<str>]>,
    /// Lane -> whether that lane's whole runtime channel-symmetry witness holds. Same length and
    /// same order as [`lane_tracks`](Self::lane_tracks).
    ///
    /// Per lane rather than a count, because a count cannot be *localised*: a bank reporting seven
    /// eligible lanes of eight does not say which track lost the term, and "exactly that track and
    /// no other" is the claim every witness test in the tree makes.
    ///
    /// Deliberately **not** the whole answer: this is the runtime half, and it is source agnostic
    /// (`SOURCE` is decided on the control plane, by `session_structural_symmetry`). Conjoin it
    /// with the structural half through `lane_tracks`.
    pub lane_eligible: Box<[bool]>,
}

impl PlanUnitEligibility {
    /// Active lanes this unit renders.
    #[must_use]
    pub fn lanes(&self) -> u32 {
        u32::try_from(self.lane_eligible.len()).unwrap_or(u32::MAX)
    }

    /// Of those, the lanes whose whole runtime witness holds.
    #[must_use]
    pub fn eligible_lanes(&self) -> u32 {
        u32::try_from(self.lane_eligible.iter().filter(|lane| **lane).count()).unwrap_or(u32::MAX)
    }

    /// Every active lane of this unit is collapse-eligible.
    ///
    /// All-lanes-or-nothing, which is `rack::BankChain::all_lanes_symmetric`'s rule
    /// restated at the plan surface: masking the eligible lanes of a mixed cohort would save
    /// nothing, because the vector op runs every lane regardless. Making a cohort homogeneous is
    /// the *planner's* job (`CohortPoolClass`), not the dispatch's. Collapse also needs
    /// [`gathers_track_input`](Self::gathers_track_input), or for a later chain of a split strip
    /// an input its producers proved symmetric on that block (issue #987): such a chain reports
    /// every lane eligible whatever its input.
    #[must_use]
    pub fn all_lanes_eligible(&self) -> bool {
        !self.lane_eligible.is_empty() && self.lane_eligible.iter().all(|lane| *lane)
    }

    /// This unit renders nothing upstream of the seam, so its witness proves nothing.
    ///
    /// A fader or matrix bank is the case: `SEAM_SIDE_WITNESS` is an unconditional
    /// `ChannelSymmetryWitness::SYMMETRIC`, so such a unit reports every lane eligible on every
    /// session, mono or not. That is correct as a statement about the seam and useless as
    /// collapse evidence, and a caller must test this before believing
    /// [`all_lanes_eligible`](Self::all_lanes_eligible), together with
    /// [`gathers_track_input`](Self::gathers_track_input).
    #[must_use]
    pub const fn witness_is_vacuous(&self) -> bool {
        self.upstream_of_seam_stages == 0
    }
}

/// Engine-internal prepared-plan dispatch seam.
///
/// Implementations are constructed off the render thread and own all state needed to render a
/// complete block.  It is deliberately not a host callback interface.  Production
/// implementations are policy-limited to `graph`.
#[doc(hidden)]
pub trait PreparedPlanExecutor: Send {
    /// The heap bytes this value retains (#1074), exclusive of the value itself: every `Box`,
    /// `Vec`, `String` and `Arc` allocation reachable through its fields, a `Vec` or `String` at
    /// its capacity, an `Arc`'s allocation in full whoever else holds it. Whoever boxes the value
    /// adds `size_of_val` for the box. Computed from capacities on the control thread, never on
    /// the render path. Required, so a new owner cannot under-report by default.
    fn retained_bytes(&self) -> u64;
    /// Whether the exclusively owned source consumer supports between-block seek preparation.
    fn can_prepare_source_seek(&self, _source_index: usize) -> bool {
        false
    }
    /// Apply an admitted source seek without rendering or advancing any sample clock.
    fn prepare_source_seek(&mut self, _source_index: usize, _generation: u64, _frame: u64) -> bool {
        false
    }
    /// Copy one selected track's retained response owners at an already-established block
    /// boundary. The executor receives only the engine-owned sink surface, so this crate does not
    /// depend on graph or effect types.
    fn copy_response_snapshot(
        &self,
        _track_id: &str,
        _captured_sample: u64,
        _sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<u32, ResponseSnapshotError> {
        Err(ResponseSnapshotError::Unsupported)
    }
    /// This executor as `Any`, so a successor's [`adopt_predecessor`](Self::adopt_predecessor)
    /// can downcast its predecessor to a concrete type it knows how to take state from.
    ///
    /// `None` (the default) declares that this executor offers no state to a successor.
    fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
        None
    }
    /// This executor as shared `Any`, so the crate that built it can read its own executor type
    /// through a shared plan off the render thread (#1344 D3: a donation check reads a withdrawn
    /// candidate the control thread holds only by shared reference). `None` (the default) offers
    /// nothing.
    fn as_any(&self) -> Option<&dyn core::any::Any> {
        None
    }
    /// Take state from the plan this one displaces, in the block that makes this plan active.
    ///
    /// **Where it runs.** On the render thread, inside the render scope, in the one block that
    /// applies the swap: after this plan has adopted the predecessor's sample clock and before it
    /// renders that block. It never runs on a block whose swap is deferred, nor for a candidate
    /// dropped unapplied. A synchronous host makes the same hand-over through
    /// [`PreparedRenderPlan::adopt_predecessor_plan`].
    ///
    /// **Render-thread code.** No allocation, free, lock, syscall, log or unbounded loop: the
    /// hand-over may only copy or swap state between preallocated storage.
    ///
    /// **Afterwards.** The predecessor is pushed to retirement after this returns and is reclaimed
    /// off the render thread; it never renders again, so state may be moved out of it.
    ///
    /// **`block_start`.** The absolute sample of the block this plan renders first: the block
    /// `predecessor` would have rendered next. A predecessor that has not rendered its own first
    /// block yet may still hold work that block would have done, and this is its sample.
    ///
    /// The default takes nothing and reports [`CarryOutcome::NotRequested`].
    fn adopt_predecessor(
        &mut self,
        _predecessor: &mut dyn PreparedPlanExecutor,
        _block_start: u64,
    ) -> CarryOutcome {
        CarryOutcome::NotRequested
    }
    /// The observers this plan took from its predecessor in the block that applied its swap
    /// (#1327 D3): one record per carried observer, written by
    /// [`adopt_predecessor`](Self::adopt_predecessor) into preallocated storage and never changed
    /// after, since a plan is adopted at most once.
    ///
    /// **Where it runs.** Called on the render thread inside the render scope, right after the
    /// hand-over, by [`PreparedRenderPlan::adopt_predecessor_plan`] and by the plan exchange's swap
    /// block. Control-side code may read it too.
    ///
    /// **Render-thread code.** A borrow of existing storage: no allocation, free, lock, syscall,
    /// log or unbounded loop.
    ///
    /// The default is empty: this executor carried no observer.
    fn observer_carries(&self) -> &[ObserverCarry] {
        &[]
    }
    /// Whether this plan, running, is ready for its successor to adopt with a prime of
    /// `lead_blocks` blocks at the block that starts at `block_start` (#1311 D3).
    ///
    /// **Where it runs.** On the render thread, inside the render scope, at block entry, on the
    /// **active** plan (never on the candidate), and only for a candidate published
    /// [`super::PlanAdoption::Primed`] whose `not_before` the block has reached. Render claims
    /// the candidate only if this returns `true`, and adopts it in the same block.
    ///
    /// **Render-thread code.** No allocation, free, lock, syscall, log or unbounded loop. It may
    /// read atomics the control thread stores before it publishes the candidate: an answer that
    /// belongs to another publication goes with a mailbox word of another generation, so the
    /// claim it would allow fails.
    ///
    /// The default `false` declares that this plan cannot check readiness, so a `Primed`
    /// candidate is never adopted over it.
    fn prime_ready(&self, _block_start: u64, _lead_blocks: u32) -> bool {
        false
    }
    /// Read this plan's sources `offset` samples ahead of its render clock (#1396 D1): a block
    /// that renders from absolute sample `r` reads source time from `r + offset`, its
    /// source-read sample, on which an anchored seek is compared.
    ///
    /// **Where it runs.** On the control thread, on an unpublished plan, through
    /// [`PreparedRenderPlan::set_source_read_offset`]; never on the render thread.
    ///
    /// The default ignores the offset, which is right for an executor that reads no source.
    fn set_source_read_offset(&mut self, _offset: u64) {}
    /// Invalidate owner-side observers after a render refusal that happened before dispatch.
    ///
    /// This is an internal hook so a prepared executor can discard a partial capture without
    /// exposing graph types through the engine foundation seam.
    #[doc(hidden)]
    fn invalidate_observers(&mut self) {}
    /// Receive the block's live snapshot `S` (#1502 D2): the revision the plan's own gate held
    /// when the block began. Called once per block, on the render thread inside the render scope,
    /// after the block's output shape and clock checks and before [`render`](Self::render). A
    /// block those checks refuse never calls it.
    ///
    /// **Render-thread code.** No allocation, free, lock, syscall, log or unbounded loop. The
    /// default ignores the snapshot, which is right for an executor whose live drains still pop
    /// queues at their nodes (#1504 hands `S` to the drains).
    fn begin_live_block(&mut self, _snapshot: LiveSnapshot) {}
    /// Receive the block's strip meter level (#882): `false` asks the executor to skip every
    /// strip meter observer this block, so a block nobody meters does no strip meter work; taps
    /// and other observers run unchanged. Called once per block, from
    /// [`begin_block`](Self::begin_block).
    ///
    /// **Render-thread code.** One store: no allocation, free, lock, syscall, log or loop. The
    /// default ignores it, which is right for an executor that binds no strip meter.
    fn set_strip_meter_level(&mut self, _on: bool) {}
    /// The block's entry, once per block where [`begin_live_block`](Self::begin_live_block) is
    /// documented to run: the strip meter level ([`set_strip_meter_level`](Self::set_strip_meter_level),
    /// #882), then the live snapshot. One dispatch for both, so the render path's dynamic calls
    /// stay as they were; an executor overrides the two hooks, not this.
    fn begin_block(&mut self, snapshot: LiveSnapshot, strip_meters: bool) {
        self.set_strip_meter_level(strip_meters);
        self.begin_live_block(snapshot);
    }
    /// Render one already-validated block using only preallocated state, and report the
    /// block's source facts ([`ExecutorBlockReport`]) in the same call.
    fn render(
        &mut self,
        arena: &mut BufferArena,
        output: PlanarBufferMut<'_>,
        time: RenderTime,
    ) -> Result<ExecutorBlockReport, RenderError>;
    /// Bounded implementation-owned qualification counters, read only after rendering is
    /// disarmed. Production executors that do not expose counters retain the zero default.
    #[doc(hidden)]
    fn qualification_counters(&self) -> [u64; 2] {
        [0, 0]
    }
    /// Cumulative planar/AoSoA round-trips performed by every bank chain, read only after
    /// rendering is disarmed. Master plan §4.5 fixes this at one per bank chain per block.
    #[doc(hidden)]
    fn bank_transposes(&self) -> u64 {
        0
    }
    /// `[bank chains, bound bank slots]` the built runtime realises (issue #181).
    ///
    /// G5 fixes one round-trip per bank **chain** per block. While every chain carried one slot
    /// the two readings were numerically identical and the gate could not discriminate between
    /// them -- a runtime that had silently regressed to one chain per slot would still have
    /// passed. Reporting both makes the law checkable.
    #[doc(hidden)]
    fn bank_shape(&self) -> [u64; 2] {
        [0, 0]
    }
    /// `[collapse-eligible lanes, lanes]` the built runtime realises: the channel-symmetry census.
    ///
    /// A lane is eligible when every term of its channel-symmetry witness holds -- the two
    /// channels are fed by one source channel, every designed per-lane word every upstream stage's
    /// kernel reads compares bit-equal between them, no admitted record has written one channel's
    /// upstream word, no upstream stage is live-bypassed, and every restored payload's two
    /// sections compared byte-equal.
    ///
    /// **Nothing rendered reads this census.** The *terms* it counts are read on the render path
    /// since mono-collapse M2 -- that is what the collapse dispatches on -- but this pair of totals
    /// is a walk over the built runtime, for the same reason `observation_binding_counts` is: the
    /// honest way to check what a plan holds is to walk it. Read only after rendering is disarmed.
    ///
    /// # What this census counts, and what it must not be used for
    ///
    /// It is **monotone evidence** and nothing else: a number that must not go down when a session
    /// is made more symmetric, and must go down when one lane is desymmetrised. Every existing
    /// gate on it is of that shape, and it is sound for all of them.
    ///
    /// It is **not usable for pool sizing** -- for "how many lanes would a collapse actually
    /// save?" -- and the reason is in the denominator. A unit that is not per-track upstream work
    /// at all contributes to *both* halves: an `Identity`, a `SourceInput` and a `Route` each
    /// report `ChannelSymmetryWitness::SYMMETRIC` and count one "lane" apiece, because there is
    /// nothing about them that could make two channels disagree. So a 64-track plan's totals carry
    /// its 64 source inputs and (when the #218 fold declines) its 64 route ops alongside its 64
    /// bank lanes, and the ratio moves when the *plan shape* moves even though not one track's
    /// symmetry did. Two sessions' censuses are comparable only when their unit inventories are.
    ///
    /// The mono collapse's dispatch must read [`unit_eligibility`](Self::unit_eligibility)
    /// instead: it is per unit, it says whether a unit is banked, and it says how many of a unit's
    /// stages are upstream of the seam -- which is exactly the three things this pair of totals
    /// has folded away.
    #[doc(hidden)]
    fn symmetry_counters(&self) -> [u64; 2] {
        [0, 0]
    }
    /// One [`PlanUnitEligibility`] row per scheduling unit, in execution order.
    ///
    /// The per-cohort form of [`symmetry_counters`](Self::symmetry_counters); that method's
    /// documentation says what the census can and cannot be used for. Allocates, walks the built
    /// runtime, and is read only after rendering is disarmed.
    #[doc(hidden)]
    fn unit_eligibility(&self) -> Vec<PlanUnitEligibility> {
        Vec::new()
    }
    /// Bank-chain lanes whose scatter was pointed at their consumer's buffer (issue #202 rec 3).
    ///
    /// The optimisation removes a whole stereo block copy per admitted lane per block, and removes
    /// it by *not doing* something -- there is no output difference to observe and no timing
    /// difference a gate may rest on. A count of the lanes the bind admitted is the only honest way
    /// to state that it fired, which is the same reason `bank_shape` exists beside
    /// `bank_transposes`. Fixed at bind, so it does not move across blocks. Read only after
    /// rendering is disarmed.
    #[doc(hidden)]
    fn bank_scatter_redirects(&self) -> u64 {
        0
    }
    /// Bank-chain lanes whose route and master accumulation were folded into the chain's own
    /// epilogue (issue #218).
    ///
    /// The same kind of counter, and for the same reason: the fold removes a whole per-lane
    /// `mix2x2_block` pass and a whole pass of the master reduction by *not doing* them, so it
    /// moves no rendered bit and a digest gate cannot see whether it fired. Fixed at bind. Read
    /// only after rendering is disarmed.
    #[doc(hidden)]
    fn bank_route_folds(&self) -> u64 {
        0
    }
    /// `[blocks rendered with the mono collapse taken, cohorts that can take it at all]`.
    ///
    /// The mono collapse's counter, and it is a pair for the reason `bank_shape` is a pair: either
    /// number alone is unreadable. The first says the collapse *fired* -- it is a per-block count,
    /// so a plan that rendered `n` blocks with all `c` of its cohorts collapsed reports `n * c` --
    /// and the second says how many cohorts could have taken it, which is fixed at bind and is what
    /// "8 of 8 cohorts" is read off.
    ///
    /// A collapse moves no rendered bit -- that is the whole claim -- so a digest gate cannot see
    /// whether it fired, and a count is the only honest way to state that it did. The same reason
    /// `bank_scatter_redirects` and `bank_route_folds` exist. Read only after rendering is
    /// disarmed.
    #[doc(hidden)]
    fn bank_collapse_counters(&self) -> [u64; 2] {
        [0, 0]
    }
    /// `[disengages, re-engages, agreement proofs]`, over every bank chain (mono-collapse M3).
    ///
    /// The companion to [`bank_collapse_counters`](Self::bank_collapse_counters), and it answers
    /// the question that one cannot: a block count says how much of the session collapsed, never
    /// how many times it stopped and started. Read only after rendering is disarmed.
    #[doc(hidden)]
    fn bank_collapse_transitions(&self) -> [u64; 3] {
        [0; 3]
    }
    /// Force every bank chain's mono collapse off, or back on.
    ///
    /// The second arm of the paired mono measurement, and a kill switch. Bind-time and per plan,
    /// never a process-global: the measurement builds both arms in one process and alternates them
    /// per observation, so a global switch could not express it. Off the render thread.
    #[doc(hidden)]
    fn force_mono_collapse_off(&mut self, forced: bool) {
        let _ = forced;
    }
    /// Perform the mono collapse's structural join and arm the cohorts it admits.
    ///
    /// `eligible(track_id)` is the **structural** half of the channel-symmetry witness -- the
    /// `SOURCE` term, decided from the compiled session by
    /// `session_structural_symmetry`. Every chain is unarmed until this is called, and an
    /// unarmed chain never collapses: the runtime half a chain carries is source agnostic (see
    /// [`PlanUnitEligibility::lane_eligible`]) and would admit a track whose two channels read
    /// two different source channels.
    ///
    /// Off the render thread, once, after bind.
    #[doc(hidden)]
    fn arm_mono_collapse(&mut self, eligible: &dyn Fn(&str) -> bool) {
        let _ = eligible;
    }
    /// `[observed stages, declared taps, armed taps]`, walked over the **built** runtime.
    ///
    /// Issue #143 E5's structural gate. "A session that asked for no observation carries none" is
    /// a statement about the objects a plan actually holds, and the only honest way to check it is
    /// to walk them; comparing render output would pass even for a plan that bound every tap and
    /// happened to publish nothing. Read only after rendering is disarmed.
    #[doc(hidden)]
    fn observation_binding_counts(&self) -> [u64; 3] {
        [0, 0, 0]
    }
    /// Exact engine-owned observation bytes the built runtime retains (issue #143 R7).
    ///
    /// Zero for every plan that bound no tap, which is what makes `observation_retained_bytes == 0`
    /// a walked fact rather than a computed guess.
    #[doc(hidden)]
    fn observation_retained_bytes(&self) -> u64 {
        0
    }
    /// The accepted source frames this plan's rings hold that render never read (#1699 D1).
    /// Control thread only, after the plan is retired; render never calls it. Default 0 (no
    /// sources).
    fn unread_source_frames(&self) -> u64 {
        0
    }
}

/// What a successor plan took from the plan it displaced, at the block that applied the swap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CarryOutcome {
    /// No hand-over ran or the successor asked for none (also every block without an applied swap).
    NotRequested,
    /// The successor took state from its predecessor.
    Carried,
    /// The successor asked for state but its predecessor was not a shape it can take state from.
    PredecessorMismatch,
}

/// Which host-visible reader vector an [`ObserverCarry`] indexes (#1327 D3).
///
/// The kinds name vectors the host holds, in the order the plan's observer location table
/// lists the entries of that kind, so an entry's index is its reader's index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObserverKind {
    /// A strip meter's reader.
    Meter,
    /// An effect observation tap's reader.
    Observation,
    /// A spectrum capture's reader (#1395).
    Spectrum,
}

/// One observer the successor took from its predecessor in the swap block (#1327 D3): the
/// successor's observer at entry `successor` of its `kind` now holds the state that sat at entry
/// `predecessor` of the predecessor's, producer included, so a reader the host already holds for
/// the predecessor's entry must follow it to the successor's.
///
/// Plain data: the executor writes the record into preallocated storage in the swap block, and a
/// host pairs its readers from it with no allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObserverCarry {
    /// The reader vector both indices address.
    pub kind: ObserverKind,
    /// The entry index in the successor plan (this plan).
    pub successor: usize,
    /// The entry index in the predecessor plan.
    pub predecessor: usize,
}

/// Absolute sample time supplied by the host; no wall clock is used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderTime {
    /// Absolute sample index of this block's first frame.
    pub absolute_sample: u64,
}
/// Bounded render failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderError {
    /// The caller selected a sample rate outside the launch engine set.
    UnsupportedRate,
    /// The prepared envelope contains a zero or otherwise invalid dimension.
    InvalidEnvelope,
    /// External output does not match the prepared envelope.
    OutputShape,
    /// Advancing the absolute sample clock would overflow `u64`.
    TimeOverflow,
    /// A contiguous render was asked to start at a sample other than the one that follows the
    /// last rendered block. `expected` is the sample the plan is waiting for.
    TimeDiscontinuity {
        /// The absolute sample this plan's next contiguous block must start at.
        expected: u64,
    },
    /// Internal or external planar storage validation failed.
    Buffer(BufferArenaError),
}
impl From<BufferArenaError> for RenderError {
    fn from(value: BufferArenaError) -> Self {
        Self::Buffer(value)
    }
}
/// What an executor reports about a block it rendered successfully (#1537 D3), returned by
/// [`PreparedPlanExecutor::render`] itself, so the render path asks the executor nothing more.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExecutorBlockReport {
    /// Whether a source played zeros in this block because the host had not delivered PCM it
    /// owed (graph-wide, not per source). A held block and a block past the end of a region are
    /// not underruns. `false` for an executor without sources.
    pub source_underrun: bool,
}

/// Fixed report returned after every invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderReport {
    /// Identity of the plan that rendered the complete block.
    pub plan_id: u64,
    /// Absolute sample immediately after the rendered block.
    pub next_absolute_sample: u64,
    /// Number of PCM frames rendered.
    pub frames: u32,
    /// The block's live snapshot `S`, taken from the plan's own revision gate (#1502 D2): the
    /// newest committed revision whose live values the block applied.
    pub live_snapshot: LiveSnapshot,
    /// Whether a source underran in this block ([`ExecutorBlockReport::source_underrun`]);
    /// `false` for a plan without an executor.
    pub source_underrun: bool,
}

/// Why a block failed, and whether it failed after the live snapshot.
///
/// A block the output-shape or clock checks refuse takes no snapshot and applies nothing. A block
/// the executor fails after [`PreparedPlanExecutor::begin_live_block`] may have applied live
/// values without rendering output (#1502 D7), which the owner counts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BlockFailure {
    pub(crate) error: RenderError,
    pub(crate) after_snapshot: bool,
}

/// The control handle of one plan's revision gate (#1502 D1): the shared gate and the one
/// [`GatePin`] that publishes to it. [`PreparedRenderPlan::take_revision_gate`] hands it out once.
///
/// Only the holder publishes on the plan's gate. The plan's owner stamps writes to the plan's
/// cells before hand-out ([`PreparedRenderPlan::stamp_held_gate`]), and the holder after it; the
/// open stamp moves with the pin into the handle (#1277 D11). The handle moves between threads
/// only by ownership transfer, and only with no open stamp: every revision it stamped has been
/// published. The control plane keeps it in the plan's provider epoch, so a handle follows its
/// plan, its cells and the producers that write them.
pub struct PlanRevisionGate {
    gate: Arc<RevisionGate>,
    pin: GatePin,
}

impl core::fmt::Debug for PlanRevisionGate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PlanRevisionGate")
            .field("revision", &self.gate.revision())
            .field("pin", &self.pin)
            .finish()
    }
}

impl PlanRevisionGate {
    /// The revision carried by the newest publication on the gate.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.gate.revision()
    }

    /// The revision carried by the gate's last publication, or the revision it was built at.
    #[must_use]
    pub const fn last_published(&self) -> u64 {
        self.pin.last_published()
    }

    /// The gate's last publication: number 0 and the built-at revision before the first.
    #[must_use]
    pub const fn last_published_key(&self) -> GateKey {
        self.pin.last_published_key()
    }

    /// Stamp the writes of a transaction that commits as `revision` (#1432 `GatePin::stamp`). The
    /// stamp borrows the handle, so the revision cannot be published while a write holds it.
    ///
    /// # Errors
    ///
    /// Refuses, in every build, a revision at or below the last published one or above the gate's
    /// ceiling, and changes nothing. A caller takes the stamp as its last fallible step, maps the
    /// refusal to its internal-error refusal with nothing written, and publishes the stamp's
    /// revision.
    pub fn stamp(&mut self, revision: u64) -> Result<LiveStamp<'_>, StampRefused> {
        self.pin.stamp(revision)
    }

    /// Stamp the writes of a monitor publication under the committed `revision`: the next
    /// publication number, carrying `revision` again (#1603 `GatePin::stamp_monitor`). A monitor
    /// write spends no revision. The stamp borrows the handle, so the key cannot be published
    /// while a write holds it.
    ///
    /// # Errors
    ///
    /// Refuses, in every build and changing nothing, a `revision` other than the one the gate
    /// published last
    /// ([`NotPublishedRevision`](crate::realtime::StampRefusal::NotPublishedRevision)) and a gate
    /// with no publication number left
    /// ([`PublicationsExhausted`](crate::realtime::StampRefusal::PublicationsExhausted)). A caller
    /// takes the stamp as its last fallible step and publishes the stamp's key.
    pub fn stamp_monitor(&mut self, revision: u64) -> Result<LiveStamp<'_>, StampRefused> {
        self.pin.stamp_monitor(revision)
    }

    /// Publish `revision` on the plan's gate, as the commit's last write (#1432
    /// `GatePin::publish`): a new publication that carries `revision`. Render's next snapshot of the gate covers it.
    pub fn publish(&mut self, revision: u64) {
        self.pin.publish(&self.gate, revision);
    }

    /// Publish `key` on the plan's gate, as the commit's last write (#1603
    /// `GatePin::publish_key`): a caller reads `stamp.key()` into a local and publishes it, so a
    /// monitor publication and a model commit take the same call. Render's next snapshot of the
    /// gate covers it.
    pub fn publish_key(&mut self, key: GateKey) {
        self.pin.publish_key(&self.gate, key);
    }
}

#[repr(C)]
struct SharedGateAllocation {
    strong: AtomicUsize,
    weak: AtomicUsize,
    gate: RevisionGate,
}

/// Exact engine-owned bytes one plan's revision gate retains: the gate word with the shared-owner
/// header, one allocation (#1502 D1). The host-core preparation charges it to each prepared plan.
#[must_use]
pub fn plan_revision_gate_retained_bytes() -> u64 {
    u64::try_from(Layout::new::<SharedGateAllocation>().size())
        .expect("a gate allocation's size fits u64")
}
/// Heap-byte arithmetic for the plan's retained-bytes walk (#1074), shared by every crate that
/// implements a walked trait. Each helper answers the bytes one owner allocates, exclusive of the
/// owner itself, from lengths and capacities only: never from sample content, never on render.
#[doc(hidden)]
pub mod retained {
    use core::alloc::Layout;
    use std::sync::Arc;

    fn to_u64(bytes: usize) -> u64 {
        u64::try_from(bytes).unwrap_or(u64::MAX)
    }

    /// A `Box<[T]>` or a borrowed slice's backing allocation: `len * size_of::<T>()`.
    #[must_use]
    pub const fn slice<T>(values: &[T]) -> u64 {
        // Lossless: `usize` is at most 64 bits on every supported target.
        core::mem::size_of_val(values) as u64
    }

    /// A `Box<str>`'s allocation.
    #[must_use]
    pub fn str(value: &str) -> u64 {
        to_u64(value.len())
    }

    /// A `Vec<T>`'s allocation, at its capacity (spare capacity counts).
    #[must_use]
    pub fn vec<T>(values: &Vec<T>) -> u64 {
        to_u64(values.capacity().saturating_mul(core::mem::size_of::<T>()))
    }

    /// A `Box<T>` of a sized `T`: `size_of::<T>()` (zero for a zero-sized `T`).
    #[must_use]
    pub const fn boxed<T>(_value: &T) -> u64 {
        core::mem::size_of::<T>() as u64
    }

    /// An `Arc<T>`'s whole allocation (the two reference counts, then `T`, padded), charged in
    /// full whatever its strong count.
    #[must_use]
    pub fn arc<T: ?Sized>(value: &Arc<T>) -> u64 {
        arc_layout(Layout::for_value::<T>(value))
    }

    fn arc_layout(value: Layout) -> u64 {
        Layout::new::<[usize; 2]>()
            .extend(value)
            .map_or(u64::MAX, |(layout, _)| to_u64(layout.pad_to_align().size()))
    }
}

/// Exclusive realtime state. It is `Send` but intentionally `!Sync`.
///
/// A shared reference is refused at compile time. rustc reports E0277 (`Sync` is not implemented);
/// the fence carries no code because stable rustdoc does not check one.
///
/// ```compile_fail
/// use engine::realtime::PreparedRenderPlan;
/// fn requires<T: Sync>() {}
/// requires::<PreparedRenderPlan>();
/// ```
///
/// Moving it to the render thread is the allowed path. This twin differs from the doctest above in
/// the bound only, so a renamed item or a typo in the shared code turns it red (issue #1422 D2):
///
/// ```
/// use engine::realtime::PreparedRenderPlan;
/// fn requires<T: Send>() {}
/// requires::<PreparedRenderPlan>();
/// ```
pub struct PreparedRenderPlan {
    program: PreparedProgram,
    arena: BufferArena,
    rendered_blocks: u64,
    next_absolute_sample: u64,
    /// `ΣP`, how far this plan reads its sources ahead of its render clock (#1396 D1). Set on the
    /// control thread before publication; render reads it and never changes it.
    source_read_offset: u64,
    response_snapshot_valid: bool,
    executor: Option<Box<dyn PreparedPlanExecutor>>,
    /// This plan's revision gate (#1502 D1), built at revision 0 on the control thread. Render
    /// takes the block's live snapshot from it.
    gate: Arc<RevisionGate>,
    /// The pin of `gate` until [`Self::take_revision_gate`] hands the control handle out.
    gate_pin: Option<GatePin>,
    /// Host data that travels with the plan (issue #1400 D4). Render never reads or drops it, and
    /// calls it once per applied swap, through [`HostAttachment::adopt_predecessor`] in the swap
    /// block. It is set on the control thread before publication, and drops with the plan on the
    /// retirement path.
    host_attachment: Option<Box<dyn HostAttachment>>,
    /// The strip meter level this plan hands its executor each block (#882): `true` (the
    /// default) keeps every bound strip meter observing, so a plan with no host meter policy
    /// meters every block. Render reads it; [`Self::set_strip_meter_level`] stores it.
    strip_meters: bool,
    /// Whether this plan's executor carried a strip meter from its predecessor at its swap block
    /// (#882): an [`ObserverKind::Meter`] entry in its [`Self::observer_carries`], recorded by
    /// [`Self::carry_from`]. `false` until then.
    strip_meters_carried: bool,
    _not_sync: Cell<()>,
    #[cfg(test)]
    drop_observer: Option<DropObserver>,
}

/// Host data a plan carries from its preparation to its retirement (issue #1400 D4).
///
/// The engine only stores it and drops it with the plan. Render never touches it: the control
/// plane reaches it through [`PreparedRenderPlan::host_attachment_mut`] before the plan is
/// published, and a host that renders through the plan's owner reads it there. `Send` because
/// the plan moves to the render thread; the drop runs wherever the plan is reclaimed, which is
/// off the render thread.
pub trait HostAttachment: Send + 'static {
    /// The attachment as `Any`, for [`PreparedRenderPlan::host_attachment_mut`]'s downcast.
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
    /// Take state from the attachment of the plan this one displaces, in the block that applies the
    /// swap (#1381 D3).
    ///
    /// **Where it runs.** On the render thread, inside the render scope, from
    /// `PreparedRenderPlan::carry_from`: after the executor hand-over, and only when both plans
    /// carry an attachment. `carries` is the successor executor's
    /// [`PreparedPlanExecutor::observer_carries`] (empty when it has none), so an attachment that
    /// holds a host's readers pairs them with the observers the executors carried.
    ///
    /// **Render-thread code.** No allocation, free, lock, syscall, log or unbounded loop: it may
    /// only move or swap state between preallocated storage. `predecessor` retires after this
    /// returns and never renders again, so state may be moved out of it.
    ///
    /// The default takes nothing.
    fn adopt_predecessor(
        &mut self,
        _predecessor: &mut dyn HostAttachment,
        _carries: &[ObserverCarry],
    ) {
    }
}

#[cfg(test)]
struct DropObserver {
    plan_id: u64,
    observer: std::sync::Arc<std::sync::Mutex<Vec<(u64, std::thread::ThreadId)>>>,
}

#[cfg(test)]
impl Drop for DropObserver {
    fn drop(&mut self) {
        self.observer
            .lock()
            .expect("drop observer lock")
            .push((self.plan_id, std::thread::current().id()));
    }
}
impl PreparedRenderPlan {
    /// Compile fixed storage and immutable program outside the render plane.
    pub fn prepare(request: PrepareRenderPlan<'_>) -> Result<Self, RenderError> {
        let envelope = request.envelope.validate()?;
        // Revision 0, below every revision the plan will stamp (#1432: a gate built at `r` cannot
        // take a stamp of `r`).
        let (gate, gate_pin) = RevisionGate::pair(0);
        Ok(Self {
            program: PreparedProgram {
                plan_id: request.plan_id,
                envelope,
            },
            arena: BufferArena::try_new(request.scratch)?,
            rendered_blocks: 0,
            next_absolute_sample: 0,
            source_read_offset: 0,
            response_snapshot_valid: true,
            executor: None,
            gate: Arc::new(gate),
            gate_pin: Some(gate_pin),
            host_attachment: None,
            strip_meters: true,
            strip_meters_carried: false,
            _not_sync: Cell::new(()),
            #[cfg(test)]
            drop_observer: None,
        })
    }
    /// Attach host data to this plan (issue #1400 D4), on the control thread before the plan is
    /// published. A second call replaces the first, which drops here, on the calling thread.
    pub fn set_host_attachment(&mut self, value: Box<dyn HostAttachment>) {
        self.host_attachment = Some(value);
    }
    /// Whether the plan carries a host attachment of any type. Control side only.
    #[must_use]
    pub fn has_host_attachment(&self) -> bool {
        self.host_attachment.is_some()
    }
    /// The attachment as a `T`, or `None` when the plan carries none or carries another type.
    /// Control side only: render never calls this.
    pub fn host_attachment_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.host_attachment
            .as_mut()?
            .as_any_mut()
            .downcast_mut::<T>()
    }
    /// Hand out this plan's one revision-gate handle (#1502 D1): the first call returns it, every
    /// later call `None`. The plan keeps the shared gate for render; only the holder of the
    /// handle publishes on it.
    pub fn take_revision_gate(&mut self) -> Option<PlanRevisionGate> {
        let pin = self.gate_pin.take()?;
        Some(PlanRevisionGate {
            gate: Arc::clone(&self.gate),
            pin,
        })
    }
    /// Stamp a write to this plan's cells on the plan's own pin, while the plan still holds its
    /// handle (#1277 D11): the same stamp [`PlanRevisionGate::stamp`] gives, for the plan's owner
    /// before [`Self::take_revision_gate`] hands the handle out. A preparation that writes cell
    /// values into the plan it is building (a successor's carry retargets) stamps them here.
    ///
    /// The open stamp stays in the pin, so it moves with the pin into the handle: the holder
    /// stamps the same revision again as its last fallible step (the pin allows a stamp at the
    /// open revision) and publishes it. A plan that is dropped with an open stamp has published
    /// nothing, so nothing it stamped is ever visible.
    ///
    /// `None` once the handle was taken, and nothing is stamped. Otherwise the result is exactly
    /// [`GatePin::stamp`]'s: a refusal in every build for a revision at or below the last
    /// published one or above [`GATE_REVISION_MAX`](crate::realtime::GATE_REVISION_MAX).
    pub fn stamp_held_gate(
        &mut self,
        revision: u64,
    ) -> Option<Result<LiveStamp<'_>, StampRefused>> {
        Some(self.gate_pin.as_mut()?.stamp(revision))
    }
    /// Whether `handle` is the control handle of this plan's gate, by pointer identity.
    pub(crate) fn owns_revision_gate(&self, handle: &PlanRevisionGate) -> bool {
        Arc::ptr_eq(&self.gate, &handle.gate)
    }
    /// Construct a plan with an already validated internal executor.
    ///
    /// The executor is owned by the plan, so it is destroyed only when the retired plan is
    /// reclaimed off the render thread.  No publication behavior is changed here.
    #[doc(hidden)]
    pub fn prepare_with_executor(
        request: PrepareRenderPlan<'_>,
        executor: Box<dyn PreparedPlanExecutor>,
    ) -> Result<Self, RenderError> {
        let mut plan = Self::prepare(request)?;
        plan.executor = Some(executor);
        Ok(plan)
    }
    /// The absolute sample the next *contiguous* block must start at.
    ///
    /// Zero before the first render; after every successful render it is the previous block's
    /// `next_absolute_sample`. The clock lives here, in the plan, so that no host has to keep its
    /// own copy and drift from it -- both the C ABI and the browser host used to.
    #[must_use]
    pub const fn next_absolute_sample(&self) -> u64 {
        self.next_absolute_sample
    }
    /// Adopt a continuing timeline, for a plan that replaces a running one.
    ///
    /// [`super::RealtimePlanOwner`] (at an applied swap) and
    /// [`PreparedRenderPlan::adopt_predecessor_plan`] (for a synchronous host) call this, at a
    /// block boundary, so a plan swap does not restart the host's clock at zero.
    pub(crate) const fn adopt_absolute_sample(&mut self, absolute_sample: u64) {
        self.next_absolute_sample = absolute_sample;
    }
    /// How far this plan reads its sources ahead of its render clock, `ΣP` (#1396 D1): a block
    /// that renders from absolute sample `r` reads source time from `r + ΣP`, the plan's
    /// source-read clock, on which every anchored seek is compared. 0 (the default) reads sources
    /// on the render clock.
    #[must_use]
    pub const fn source_read_offset(&self) -> u64 {
        self.source_read_offset
    }
    /// Set [`Self::source_read_offset`] and hand it to the executor
    /// ([`PreparedPlanExecutor::set_source_read_offset`]). A control-thread call on an
    /// unpublished plan: once published, a plan's offset never changes, and a swap does not carry
    /// it (each plan is prepared with its own, #1396 D2).
    pub fn set_source_read_offset(&mut self, offset: u64) {
        self.source_read_offset = offset;
        if let Some(executor) = self.executor.as_deref_mut() {
            executor.set_source_read_offset(offset);
        }
    }
    /// Immutable structural program.
    #[must_use]
    pub const fn program(&self) -> &PreparedProgram {
        &self.program
    }
    /// Immutable external render envelope.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.program.envelope
    }
    /// Preallocated internal planar arena.
    #[must_use]
    pub fn arena(&self) -> &BufferArena {
        &self.arena
    }
    /// Read bounded executor qualification counters outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn qualification_counters(&self) -> [u64; 2] {
        assert!(
            !super::audit::is_render_scope_active(),
            "qualification counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0, 0], PreparedPlanExecutor::qualification_counters)
    }
    /// Prevalidate source preparation before a host admits the producer-side seek.
    /// Requires the plan owner; it provides no concurrent control-side consumer handle.
    pub fn can_prepare_source_seek(&self, source_index: usize) -> bool {
        self.executor
            .as_ref()
            .is_some_and(|executor| executor.can_prepare_source_seek(source_index))
    }

    /// Prepare an admitted seek on the exclusive render owner between blocks.
    /// Does not render, mutate topology, or advance the plan's sample clock.
    pub fn prepare_source_seek(
        &mut self,
        source_index: usize,
        generation: u64,
        frame: u64,
    ) -> bool {
        self.executor
            .as_mut()
            .is_some_and(|executor| executor.prepare_source_seek(source_index, generation, frame))
    }

    /// Copy one selected track's retained response owners at the current exclusive boundary.
    ///
    /// The call neither drains queued controls nor advances render state. It is valid before the
    /// first render at sample zero and after a successful render; a failed render poisons this
    /// boundary until a later successful block establishes a new one.
    pub fn copy_response_snapshot(
        &mut self,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotCapture, ResponseSnapshotError> {
        if !self.response_snapshot_valid {
            return Err(ResponseSnapshotError::Owner);
        }
        let captured_sample = self.next_absolute_sample;
        let Some(executor) = self.executor.as_deref() else {
            return Err(ResponseSnapshotError::Unsupported);
        };
        let owners =
            executor.copy_response_snapshot(request.track_id, captured_sample, request.sink)?;
        Ok(ResponseSnapshotCapture {
            captured_sample,
            owners,
        })
    }

    /// Walk the built runtime for observation bindings, outside the render scope (issue #143 E5).
    #[doc(hidden)]
    #[must_use]
    pub fn observation_binding_counts(&self) -> [u64; 3] {
        assert!(
            !super::audit::is_render_scope_active(),
            "observation binding counts are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0; 3], PreparedPlanExecutor::observation_binding_counts)
    }
    /// The heap bytes this plan retains (#1074), exclusive of the plan value itself (its owner
    /// holds it by value): the buffer arena's three tables, the executor's box and everything the
    /// executor walks ([`PreparedPlanExecutor::retained_bytes`]), and the revision gate's shared
    /// allocation, charged in full whoever else holds it. The program and the gate pin own no
    /// heap.
    ///
    /// Excluded: the [`HostAttachment`]. It is host data the plan only carries (#1400 D4), and no
    /// host that compares this walk with an allocator binds one (capi binds none; host-web's
    /// render companions are host-owned and host-web has no plan oracle). Reopen if one gets an
    /// oracle. The test-only drop observer is excluded too: it exists only under `cfg(test)`,
    /// where no oracle reads this walk.
    ///
    /// Control thread only, from capacities and lengths; render never calls it.
    #[doc(hidden)]
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "plan retained bytes are sealed until the render audit is disarmed"
        );
        let executor = self.executor.as_deref().map_or(0, |executor| {
            u64::try_from(core::mem::size_of_val(executor))
                .unwrap_or(u64::MAX)
                .saturating_add(executor.retained_bytes())
        });
        self.arena
            .retained_bytes()
            .saturating_add(executor)
            .saturating_add(retained::arc(&self.gate))
    }
    /// Walk the built runtime for retained observation bytes, outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn observation_retained_bytes(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "observation retained bytes are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, PreparedPlanExecutor::observation_retained_bytes)
    }
    /// The accepted source frames this plan's rings hold that render never read (#1699 D1).
    /// Control thread only, once the plan is retired; render never calls it.
    #[must_use]
    pub fn unread_source_frames(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "unread source frames are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, PreparedPlanExecutor::unread_source_frames)
    }
    /// Read the cumulative bank transpose count outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn bank_transposes(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank transpose counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, PreparedPlanExecutor::bank_transposes)
    }
    /// Read `[bank chains, bound bank slots]` outside the render scope (issue #181).
    #[doc(hidden)]
    #[must_use]
    pub fn bank_shape(&self) -> [u64; 2] {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank shape counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0, 0], PreparedPlanExecutor::bank_shape)
    }
    /// Read `[collapse-eligible lanes, lanes]` outside the render scope: the symmetry census.
    #[doc(hidden)]
    #[must_use]
    pub fn symmetry_counters(&self) -> [u64; 2] {
        assert!(
            !super::audit::is_render_scope_active(),
            "symmetry counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0, 0], PreparedPlanExecutor::symmetry_counters)
    }
    /// One collapse-eligibility row per scheduling unit, outside the render scope.
    ///
    /// See [`PlanUnitEligibility`] for what a row carries and for the two things a caller must
    /// check before reading one as evidence.
    #[doc(hidden)]
    #[must_use]
    pub fn unit_eligibility(&self) -> Vec<PlanUnitEligibility> {
        assert!(
            !super::audit::is_render_scope_active(),
            "unit eligibility is sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or_else(Vec::new, PreparedPlanExecutor::unit_eligibility)
    }
    /// Read the admitted scatter-redirect count outside the render scope (issue #202 rec 3).
    #[doc(hidden)]
    #[must_use]
    pub fn bank_scatter_redirects(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank scatter redirects are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, PreparedPlanExecutor::bank_scatter_redirects)
    }
    /// Read the admitted route-fold lane count outside the render scope (issue #218).
    #[doc(hidden)]
    #[must_use]
    pub fn bank_route_folds(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank route folds are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, PreparedPlanExecutor::bank_route_folds)
    }
    /// Read `[collapsed blocks, collapsible cohorts]` outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn bank_collapse_counters(&self) -> [u64; 2] {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank collapse counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0, 0], PreparedPlanExecutor::bank_collapse_counters)
    }
    /// Read `[disengages, re-engages, agreement proofs]` outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn bank_collapse_transitions(&self) -> [u64; 3] {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank collapse transitions are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0; 3], PreparedPlanExecutor::bank_collapse_transitions)
    }
    /// Perform the mono collapse's structural join and arm the cohorts it admits.
    ///
    /// See [`PreparedPlanExecutor::arm_mono_collapse`]. Until a caller does this, no chain of this
    /// plan collapses -- which is the safe default and the reason the join is explicit.
    #[doc(hidden)]
    pub fn arm_mono_collapse(&mut self, eligible: &dyn Fn(&str) -> bool) {
        assert!(
            !super::audit::is_render_scope_active(),
            "the mono collapse join is sealed while the render audit is armed"
        );
        if let Some(executor) = self.executor.as_deref_mut() {
            executor.arm_mono_collapse(eligible);
        }
    }
    /// Force every bank chain's mono collapse off, or back on. Bind-time, outside render.
    #[doc(hidden)]
    pub fn force_mono_collapse_off(&mut self, forced: bool) {
        assert!(
            !super::audit::is_render_scope_active(),
            "the mono collapse switch is sealed while the render audit is armed"
        );
        if let Some(executor) = self.executor.as_deref_mut() {
            executor.force_mono_collapse_off(forced);
        }
    }
    #[cfg(test)]
    pub(crate) fn set_drop_observer(
        &mut self,
        observer: std::sync::Arc<std::sync::Mutex<Vec<(u64, std::thread::ThreadId)>>>,
    ) {
        self.drop_observer = Some(DropObserver {
            plan_id: self.program.plan_id,
            observer,
        });
    }
    // REALTIME_POLICY_BEGIN
    /// Set the strip meter level the next rendered blocks hand the executor
    /// ([`PreparedPlanExecutor::set_strip_meter_level`], #882). `false` skips every strip meter
    /// observer until it is set back; the plan's owner calls it before each block. One store, so
    /// a render-thread caller may call it between blocks.
    pub fn set_strip_meter_level(&mut self, on: bool) {
        self.strip_meters = on;
    }

    /// The strip meter level ([`Self::set_strip_meter_level`]); `true` by default.
    #[must_use]
    pub const fn strip_meter_level(&self) -> bool {
        self.strip_meters
    }

    /// Render a fixed full quantum. The reference implementation writes silence only.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, RenderError> {
        let result = super::audit::in_render_scope(|| self.render_inner(io, time))
            .map_err(|failure| failure.error);
        if result.is_err()
            && let Some(executor) = self.executor.as_deref_mut()
        {
            executor.invalidate_observers();
        }
        result
    }

    /// Render the block that must start at [`Self::next_absolute_sample`].
    ///
    /// The continuity rule is the engine's, not each host's transcription of it: a host that
    /// renders a quantum at a time calls this and never compares sample counters itself.
    pub fn render_contiguous(
        &mut self,
        io: RenderIo<'_>,
        absolute_sample: u64,
    ) -> Result<RenderReport, RenderError> {
        if absolute_sample != self.next_absolute_sample {
            if let Some(executor) = self.executor.as_deref_mut() {
                executor.invalidate_observers();
            }
            return Err(RenderError::TimeDiscontinuity {
                expected: self.next_absolute_sample,
            });
        }
        self.render(io, RenderTime { absolute_sample })
    }

    /// Hand a directly owned predecessor's clock and state to this plan, for a synchronous host.
    ///
    /// This plan adopts `predecessor`'s [`next_absolute_sample`](Self::next_absolute_sample), then,
    /// when both plans have executors, runs [`PreparedPlanExecutor::adopt_predecessor`] -- the same
    /// hand-over [`super::RealtimePlanOwner`] makes at an applied swap. Call it at a block
    /// boundary, before this plan renders, and render `predecessor` no more. The hand-over runs
    /// inside a render scope (arming one when called outside), so the realtime audit sees it in
    /// every host.
    ///
    /// A predecessor with a different [`RenderEnvelope`] (rate, quantum or output channels) is
    /// not a predecessor this plan can continue: it reports
    /// [`CarryOutcome::PredecessorMismatch`] and leaves this plan's clock and state untouched,
    /// the same envelope rule the exchange path enforces at publication.
    pub fn adopt_predecessor_plan(&mut self, predecessor: &mut Self) -> CarryOutcome {
        if self.program.envelope != predecessor.program.envelope {
            return CarryOutcome::PredecessorMismatch;
        }
        let block_start = predecessor.next_absolute_sample;
        self.adopt_absolute_sample(block_start);
        super::audit::in_render_scope(|| self.carry_from(predecessor, block_start))
    }

    /// This plan's executor as `Any` ([`PreparedPlanExecutor::as_any_mut`]), so the crate that
    /// built it can reach its own executor type off the render thread -- to read its identity or
    /// install a hand-over program before publication. `None` without an executor or when the
    /// executor offers none.
    #[doc(hidden)]
    pub fn executor_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
        self.executor.as_deref_mut()?.as_any_mut()
    }

    /// This plan's executor as shared `Any` ([`PreparedPlanExecutor::as_any`]): the read-only twin
    /// of [`Self::executor_any_mut`], for a check off the render thread that must not take the
    /// plan mutably. `None` without an executor or when the executor offers none.
    #[doc(hidden)]
    #[must_use]
    pub fn executor_any(&self) -> Option<&dyn core::any::Any> {
        self.executor.as_deref()?.as_any()
    }

    /// The executor's [`PreparedPlanExecutor::prime_ready`]; `false` without an executor.
    pub(crate) fn prime_ready(&self, block_start: u64, lead_blocks: u32) -> bool {
        self.executor
            .as_deref()
            .is_some_and(|executor| executor.prime_ready(block_start, lead_blocks))
    }

    /// The observers this plan took from its predecessor at its swap block
    /// ([`PreparedPlanExecutor::observer_carries`]); empty without an executor or a carry.
    #[must_use]
    pub fn observer_carries(&self) -> &[ObserverCarry] {
        self.executor
            .as_deref()
            .map_or(&[], |executor| executor.observer_carries())
    }

    /// Run the executor hand-over from `predecessor`, if both plans have executors, then the
    /// attachment hand-over ([`HostAttachment::adopt_predecessor`]), if both plans have an
    /// attachment, with the successor executor's observer carries. `block_start` is the sample
    /// of the block this plan renders first.
    pub(crate) fn carry_from(&mut self, predecessor: &mut Self, block_start: u64) -> CarryOutcome {
        let outcome = match (
            self.executor.as_deref_mut(),
            predecessor.executor.as_deref_mut(),
        ) {
            (Some(successor), Some(predecessor)) => {
                successor.adopt_predecessor(predecessor, block_start)
            }
            _ => CarryOutcome::NotRequested,
        };
        let carries = self
            .executor
            .as_deref()
            .map_or(&[][..], |executor| executor.observer_carries());
        self.strip_meters_carried = carries
            .iter()
            .any(|carry| carry.kind == ObserverKind::Meter);
        if let (Some(successor), Some(predecessor)) = (
            self.host_attachment.as_deref_mut(),
            predecessor.host_attachment.as_deref_mut(),
        ) {
            successor.adopt_predecessor(predecessor, carries);
        }
        outcome
    }

    /// Whether this plan's swap block carried a strip meter from its predecessor (#882).
    pub(crate) const fn strip_meters_carried(&self) -> bool {
        self.strip_meters_carried
    }

    /// Render one block through the plan's own revision gate (#1502 D2). After the output-shape
    /// and clock checks and before the executor renders, it takes the block's live snapshot `S`
    /// from this plan's gate and hands it to the executor; the report carries it. A block those
    /// checks refuse takes none.
    pub(crate) fn render_inner(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, BlockFailure> {
        self.response_snapshot_valid = false;
        let envelope = self.program.envelope;
        let frames = envelope.quantum.0 as usize;
        let before = |error: RenderError| BlockFailure {
            error,
            after_snapshot: false,
        };
        if io.output.frames() != frames || io.output.channels() != envelope.output_channels.get() {
            return Err(before(RenderError::OutputShape));
        }
        let next = time
            .absolute_sample
            .checked_add(u64::from(envelope.quantum.0))
            .ok_or_else(|| before(RenderError::TimeOverflow))?;
        let live_snapshot = self.gate.snapshot();
        let block = self
            .render_block(io, time, live_snapshot)
            .map_err(|error| BlockFailure {
                error,
                after_snapshot: true,
            })?;
        self.rendered_blocks = self.rendered_blocks.saturating_add(1);
        self.next_absolute_sample = next;
        self.response_snapshot_valid = true;
        Ok(RenderReport {
            plan_id: self.program.plan_id,
            next_absolute_sample: next,
            frames: envelope.quantum.0,
            live_snapshot,
            source_underrun: block.source_underrun,
        })
    }

    /// The part of a block that follows its live snapshot.
    fn render_block(
        &mut self,
        mut io: RenderIo<'_>,
        time: RenderTime,
        live_snapshot: LiveSnapshot,
    ) -> Result<ExecutorBlockReport, RenderError> {
        if let Some(executor) = &mut self.executor {
            executor.begin_block(live_snapshot, self.strip_meters);
            return executor.render(&mut self.arena, io.output, time);
        }
        {
            let mut channel = 0;
            while channel < self.program.envelope.output_channels.get() {
                io.output.plane_mut(channel)?.fill(0.0);
                channel += 1;
            }
        }
        Ok(ExecutorBlockReport::default())
    }
    // REALTIME_POLICY_END
}
