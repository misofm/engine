//! Structurally immutable prepared plan and bounded silence reference renderer.

#![allow(dead_code)] // State fields are intentionally prepared now for later issue-owned kernels.

use super::{BufferArena, BufferArenaError, PlanarBufferMut, PlanarBufferRef};
use crate::{QuantumFrames, SampleRateHz, is_launch_sample_rate};
use core::{cell::Cell, num::NonZeroUsize};

/// Exact rate, quantum, and external I/O shape accepted by a plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderEnvelope {
    /// Caller-selected engine rate; no implicit sample-rate conversion occurs.
    pub sample_rate: SampleRateHz,
    /// Fixed number of frames in every full render block.
    pub quantum: QuantumFrames,
    /// Optional external planar input channel count.
    pub input_channels: Option<NonZeroUsize>,
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
/// Render inputs and outputs. This only owns borrowed scalar/slice references.
pub struct RenderIo<'a> {
    /// Optional external planar input.
    pub input: Option<PlanarBufferRef<'a>>,
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
///   (`session_structural_symmetry_v1`, because the planner needs the class before any prepared
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
pub struct PlanUnitEligibilityV1 {
    /// This unit's position in the built runtime's execution order.
    pub unit: u32,
    /// `true` for a homogeneous bank chain, `false` for a single dispatched op.
    pub banked: bool,
    /// Stages this unit renders: a chain's bound slots, or 1 for a single op.
    pub stages: u32,
    /// Of those, the stages that sit **upstream of the fader/matrix seam** -- the stages a
    /// collapsed track would have computed once. Zero makes the unit's witness vacuous.
    pub upstream_of_seam_stages: u32,
    /// Lane -> the track this lane renders, in lane order. One entry per **active** lane; a
    /// single op renders one "lane", its own node. Empty for a stage that names no track (a route,
    /// a submix, the output, a compensation delay).
    pub lane_tracks: Box<[Box<str>]>,
    /// Lane -> whether that lane's whole runtime channel-symmetry witness holds. Same length and
    /// same order as [`lane_tracks`](Self::lane_tracks).
    ///
    /// Per lane rather than a count, because a count cannot be *localised*: a bank reporting seven
    /// eligible lanes of eight does not say which track lost the term, and "exactly that track and
    /// no other" is the claim every witness test in the tree makes.
    ///
    /// Deliberately **not** the whole answer: this is the runtime half, and it is source agnostic
    /// (`SOURCE` is decided on the control plane, by `session_structural_symmetry_v1`). Conjoin it
    /// with the structural half through `lane_tracks`.
    pub lane_eligible: Box<[bool]>,
}

impl PlanUnitEligibilityV1 {
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
    /// All-lanes-or-nothing, which is `miso_engine_rack::BankChain::all_lanes_symmetric`'s rule
    /// restated at the plan surface: masking the eligible lanes of a mixed cohort would save
    /// nothing, because the vector op runs every lane regardless. Making a cohort homogeneous is
    /// the *planner's* job (`CohortPoolClassV1`), not the dispatch's.
    #[must_use]
    pub fn all_lanes_eligible(&self) -> bool {
        !self.lane_eligible.is_empty() && self.lane_eligible.iter().all(|lane| *lane)
    }

    /// This unit renders nothing upstream of the seam, so its witness proves nothing.
    ///
    /// A fader or matrix bank is the case: `SEAM_SIDE_WITNESS` is an unconditional
    /// `ChannelSymmetryWitnessV1::SYMMETRIC`, so such a unit reports every lane eligible on every
    /// session, mono or not. That is correct as a statement about the seam and useless as
    /// collapse evidence, and a caller must test this before believing
    /// [`all_lanes_eligible`](Self::all_lanes_eligible).
    #[must_use]
    pub const fn witness_is_vacuous(&self) -> bool {
        self.upstream_of_seam_stages == 0
    }
}

/// Engine-internal prepared-plan dispatch seam.
///
/// Implementations are constructed off the render thread and own all state needed to render a
/// complete block.  It is deliberately not a host callback interface.  Production
/// implementations are policy-limited to `miso-engine-graph`.
#[doc(hidden)]
pub trait PreparedPlanExecutor: Send {
    /// Render one already-validated block using only preallocated state.
    fn render(
        &mut self,
        arena: &mut BufferArena,
        input: Option<PlanarBufferRef<'_>>,
        output: PlanarBufferMut<'_>,
        time: RenderTime,
    ) -> Result<(), RenderError>;
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
    /// **Nothing rendered reads this.** It is the census of a control-plane bit, walked over the
    /// built runtime for the same reason `observation_binding_counts` is: the honest way to check
    /// what a plan holds is to walk it. Read only after rendering is disarmed.
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
    /// report `ChannelSymmetryWitnessV1::SYMMETRIC` and count one "lane" apiece, because there is
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
    /// One [`PlanUnitEligibilityV1`] row per scheduling unit, in execution order.
    ///
    /// The per-cohort form of [`symmetry_counters`](Self::symmetry_counters); that method's
    /// documentation says what the census can and cannot be used for. Allocates, walks the built
    /// runtime, and is read only after rendering is disarmed.
    #[doc(hidden)]
    fn unit_eligibility(&self) -> Vec<PlanUnitEligibilityV1> {
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
    /// Copy cumulative auxiliary-worker audit snapshots after render is disarmed.
    #[doc(hidden)]
    fn copy_worker_audit_snapshots(&self, _output: &mut [super::audit::AuditSnapshot]) -> usize {
        0
    }
    /// Give up an executor-owned resource at the block-boundary swap.
    ///
    /// This exists so a persistent auxiliary worker pool outlives the plan that used it: the
    /// retiring executor hands its worker lease to the replacement instead of stopping and
    /// respawning threads on every structural change. It runs on the render thread, so it is a
    /// move and nothing else: no allocation, no drop, no wait.
    #[doc(hidden)]
    fn take_handover(&mut self) -> Option<ExecutorHandover> {
        None
    }
    /// Accept a hand-over, or refuse it by returning it unchanged.
    ///
    /// A refused hand-over is given back to the retiring executor by
    /// [`RealtimePlanOwner::enter_block`], so it is dropped only when the retired plan is
    /// reclaimed off the render thread. Implementations never drop the value here.
    #[doc(hidden)]
    fn accept_handover(&mut self, handover: ExecutorHandover) -> Option<ExecutorHandover> {
        Some(handover)
    }
    /// Bounded implementation-owned dispatch counters, read only after rendering is disarmed.
    #[doc(hidden)]
    fn dispatch_counters(&self) -> [u64; 4] {
        [0; 4]
    }
}

/// One executor-owned resource moved between prepared plans at a block boundary.
///
/// The box is allocated at bind and only ever *moved* on the render thread. `Box<dyn Any>` moves
/// and `downcast` are pointer operations; neither allocates nor frees.
#[doc(hidden)]
pub type ExecutorHandover = Box<dyn core::any::Any + Send>;
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
    /// External input is absent or does not match the prepared envelope.
    InputShape,
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
/// Fixed report returned after every invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderReport {
    /// Identity of the plan that rendered the complete block.
    pub plan_id: u64,
    /// Absolute sample immediately after the rendered block.
    pub next_absolute_sample: u64,
    /// Number of PCM frames rendered.
    pub frames: u32,
}
/// Exclusive realtime state. It is `Send` but intentionally `!Sync`.
///
/// ```compile_fail
/// use miso_engine_core::realtime::PreparedRenderPlan;
/// fn requires_sync<T: Sync>() {}
/// requires_sync::<PreparedRenderPlan>();
/// ```
pub struct PreparedRenderPlan {
    program: PreparedProgram,
    arena: BufferArena,
    rendered_blocks: u64,
    next_absolute_sample: u64,
    executor: Option<Box<dyn PreparedPlanExecutor>>,
    _not_sync: Cell<()>,
    #[cfg(test)]
    drop_observer: Option<DropObserver>,
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
        Ok(Self {
            program: PreparedProgram {
                plan_id: request.plan_id,
                envelope,
            },
            arena: BufferArena::try_new(request.scratch)?,
            rendered_blocks: 0,
            next_absolute_sample: 0,
            executor: None,
            _not_sync: Cell::new(()),
            #[cfg(test)]
            drop_observer: None,
        })
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
    /// Only [`super::RealtimePlanOwner`] calls this, at a block boundary, so a plan swap does not
    /// restart the host's clock at zero.
    pub(crate) const fn adopt_absolute_sample(&mut self, absolute_sample: u64) {
        self.next_absolute_sample = absolute_sample;
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
    /// Read bounded executor dispatch counters outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn dispatch_counters(&self) -> [u64; 4] {
        assert!(
            !super::audit::is_render_scope_active(),
            "dispatch counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0; 4], PreparedPlanExecutor::dispatch_counters)
    }
    /// The plan's internal executor, for the block-boundary hand-over in `plan_exchange`.
    pub(crate) fn executor_mut(&mut self) -> Option<&mut (dyn PreparedPlanExecutor + 'static)> {
        self.executor.as_deref_mut()
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
    /// See [`PlanUnitEligibilityV1`] for what a row carries and for the two things a caller must
    /// check before reading one as evidence.
    #[doc(hidden)]
    #[must_use]
    pub fn unit_eligibility(&self) -> Vec<PlanUnitEligibilityV1> {
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
    /// Copy cumulative auxiliary-worker audit snapshots in stable worker order.
    #[doc(hidden)]
    pub fn copy_worker_audit_snapshots(&self, output: &mut [super::audit::AuditSnapshot]) -> usize {
        assert!(
            !super::audit::is_render_scope_active(),
            "worker audit snapshots are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, |executor| executor.copy_worker_audit_snapshots(output))
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
    /// Render a fixed full quantum. The reference implementation writes silence only.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, RenderError> {
        super::audit::in_render_scope(|| self.render_inner(io, time))
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
            return Err(RenderError::TimeDiscontinuity {
                expected: self.next_absolute_sample,
            });
        }
        self.render(io, RenderTime { absolute_sample })
    }

    pub(crate) fn render_inner(
        &mut self,
        mut io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, RenderError> {
        let envelope = self.program.envelope;
        let frames = envelope.quantum.0 as usize;
        if io.output.frames() != frames || io.output.channels() != envelope.output_channels.get() {
            return Err(RenderError::OutputShape);
        }
        match (io.input, envelope.input_channels) {
            (None, None) => {}
            (Some(input), Some(channels))
                if input.frames() == frames && input.channels() == channels.get() => {}
            _ => return Err(RenderError::InputShape),
        }
        let next = time
            .absolute_sample
            .checked_add(u64::from(envelope.quantum.0))
            .ok_or(RenderError::TimeOverflow)?;
        if let Some(executor) = &mut self.executor {
            executor.render(&mut self.arena, io.input, io.output, time)?;
        } else {
            let mut channel = 0;
            while channel < envelope.output_channels.get() {
                io.output.plane_mut(channel)?.fill(0.0);
                channel += 1;
            }
        }
        self.rendered_blocks = self.rendered_blocks.saturating_add(1);
        self.next_absolute_sample = next;
        Ok(RenderReport {
            plan_id: self.program.plan_id,
            next_absolute_sample: next,
            frames: envelope.quantum.0,
        })
    }
    // REALTIME_POLICY_END
}
