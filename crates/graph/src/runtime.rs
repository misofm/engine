//! One runtime model, shared by both executors.
//!
//! The sequential executor and the native dependency-wave executor used to carry two copies of
//! every piece of node semantics: two reductions, two route loops, two effect-block constructions,
//! two node builders, two bank loops. They now share this module and differ only in *where the
//! audio lives* -- the sequential executor colours one arena and reads its producers in place,
//! the native one gives every parcel its own arena and stages every edge across partitions
//! (#98 F7).
//!
//! Everything here is derived from the lowered [`ExecutionProgram`](crate::program) (#99 F2), so
//! the ops, their input order, the identity aliases and the buffer colouring are a pure function
//! of the semantic graph rather than a second opinion about it.
//!
//! ## Arena
//!
//! Audio lives in one [`DisjointArena`](engine::realtime::DisjointArena) per prepared
//! plan: two planar `f32` planes of `buffers * frames` words, reached only through a checked
//! [`ArenaLease`]. The lease API is the one implementation of *where the audio is* as well as
//! of what happens to it.
//!
//! The sequential executor holds a single lease over the whole coloured arena. A delayed edge
//! copies, and that copy is made by the consuming op through the [`RuntimeOp::staged`] list.
//!
//! ## Frozen arithmetic
//!
//! * Reductions are master plan #83 D9: stable edge-ID order, left-to-right, block-wide, through
//!   [`sum2_block`] and [`sum_into_block`]. Fan-in 0 zero-fills, fan-in 1 copies (which preserves
//!   `-0.0`), fan-in `n >= 2` is `out = in0 + in1` then `out += in_k` for each remaining input.
//! * Routes are D3: the linear gain is folded into the 2x2 coefficients **once, at bind**, and
//!   render spends one multiply and one [`Lane::fma`](lane::Lane::fma) per output word
//!   through [`mix2x2_block`].
//! * Compensation delays are two-segment slice exchanges through [`pdc_delay_block`]; there is no
//!   per-sample work and no `%` on the render path.
//! * The graph performs **no** sanitisation (D7). Input sanitisation is the input stage's, output
//!   finiteness is the bank boundary check.

use std::collections::BTreeMap;

use core::num::NonZeroUsize;

use engine::realtime::{ArenaLease, ArenaLeaseSetBuilder, RenderError};

/// The arena reserves buffer zero as the always-zero silence slot, so every executor buffer is
/// offset by one.
pub(crate) const ARENA_BASE: u32 = 1;
use effect_contract::{
    BypassShunt, ChannelSymmetryWitness, EffectControlLane, EffectProcessBlock, ObservationLane,
    ObservationSample, PreparedAutomationSpan, PreparedNativeEffect,
};
use lane::Lane;
use lane::kernels::{mix2x2_block, ordered_accumulate_block, pdc_delay_block, sum_into_block};
use rack::{BankChain, BankMembers, BankPlaneViews, FoldCohort};

use crate::{
    GraphBindingBlock, GraphNodeObserverBinding, GraphObservationBlock, GraphPreparedEffect,
    GraphRuntimeProcessor,
};

/// Lane type the block kernels are instantiated at to vectorise **over frames**.
///
/// Frames are independent, so this is purely a width choice: master plan #83 §4.2 pins every one
/// of these kernels to a width-independent result, and gate G2 proves it, so the rendered bits do
/// not depend on which arm of this `cfg` a target takes.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) type FrameLane = lane::Simd8;
/// See [`FrameLane`].
#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
pub(crate) type FrameLane = lane::Simd4;
/// See [`FrameLane`].
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
)))]
pub(crate) type FrameLane = f32;

// REALTIME_POLICY_BEGIN

/// D9 reduction of one plane: stable edge order, left-to-right, block-wide.
///
/// `inputs` are buffer indices in `spec.edges` order, which the compiler sorts by `GraphEdgeId`.
/// A single input is a copy -- or nothing at all when the op reads its producer in place, which is
/// how the lowering removes a pass-through's copy -- so `-0.0` survives; `a + b` then accumulates
/// left to right, exactly as the scalar reference `inputs.reduce(|a, b| a + b)` does.
#[inline]
fn reduce_plane(lease: &mut ArenaLease, plane: usize, out: u32, inputs: &[u32]) {
    match inputs {
        [] => lease.write(plane, out).fill(0.0),
        [single] => {
            if *single != out {
                let (output, input) = lease.write_read(plane, out, *single);
                output.copy_from_slice(input);
            }
        }
        [first, second, rest @ ..] => {
            reduce_many::<FrameLane>(lease, plane, out, *first, *second, rest);
        }
    }
}

#[inline(always)]
fn reduce_many<L: lane::Lane>(
    lease: &mut ArenaLease,
    plane: usize,
    out: u32,
    first: u32,
    second: u32,
    rest: &[u32],
) {
    let frames = lease.frames();
    let vectored = frames - frames % L::WIDTH;
    let mut index = 0;
    while index < vectored {
        let mut acc = {
            let source = lease.read(plane, first);
            L::load(&source[index..])
        };
        for input in std::iter::once(second).chain(rest.iter().copied()) {
            let value = {
                let source = lease.read(plane, input);
                L::load(&source[index..])
            };
            acc = acc.add(value);
        }
        acc.store(&mut lease.write(plane, out)[index..]);
        index += L::WIDTH;
    }
    while index < frames {
        let mut acc = <f32 as lane::Lane>::load(&lease.read(plane, first)[index..]);
        for input in std::iter::once(second).chain(rest.iter().copied()) {
            let value = <f32 as lane::Lane>::load(&lease.read(plane, input)[index..]);
            acc = acc.add(value);
        }
        acc.store(&mut lease.write(plane, out)[index..]);
        index += 1;
    }
}

// REALTIME_POLICY_END

/// Integer-sample plugin-delay compensation for one stereo edge.
///
/// The line is `samples` words per channel and the block is exchanged with it in at most two
/// contiguous segments per channel ([`pdc_delay_block`], #98 F3). A block longer than the line is
/// walked in `samples`-word segments, which is still slice-only: no per-sample loop and no `%`.
pub(crate) struct CompensationDelay {
    left: Box<[f32]>,
    right: Box<[f32]>,
    cursor: usize,
}

impl CompensationDelay {
    pub(crate) fn new(samples: usize) -> Self {
        Self {
            left: vec![0.0; samples].into_boxed_slice(),
            right: vec![0.0; samples].into_boxed_slice(),
            cursor: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn samples(&self) -> usize {
        self.left.len()
    }

    #[cfg(test)]
    pub(crate) fn reset(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
        self.cursor = 0;
    }
}

// REALTIME_POLICY_BEGIN
impl CompensationDelay {
    pub(crate) fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let samples = self.left.len();
        if samples == 0 {
            return;
        }
        let mut offset = 0;
        while offset < left.len() {
            let take = core::cmp::min(samples, left.len() - offset);
            let cursor = self.cursor;
            let mut left_cursor = cursor;
            pdc_delay_block(
                &mut self.left,
                &mut left_cursor,
                &mut left[offset..offset + take],
            );
            let mut right_cursor = cursor;
            pdc_delay_block(
                &mut self.right,
                &mut right_cursor,
                &mut right[offset..offset + take],
            );
            debug_assert_eq!(left_cursor, right_cursor);
            self.cursor = left_cursor;
            offset += take;
        }
    }
}
// REALTIME_POLICY_END

/// One track's **input-side time alignment**: PDC's ring shape and PDC's kernel, deliberately
/// none of PDC's accounting.
///
/// # Why this is not a `CompensationDelay`
///
/// `CompensationDelay` is one length for both lanes -- PDC computes a single per-edge skew, so its
/// two rings are the same size and share one cursor (its `debug_assert_eq!` on the two cursors is
/// that invariant written down). Track delay is declared **per lane** (`builtins.left.delay_samples`
/// and `builtins.right.delay_samples` are independent words, per the dual-mono law), so the two
/// rings can differ in length and their cursors advance independently. Rather than loosen PDC's
/// type -- and move bytes under every plan that has nothing to do with this feature -- the two
/// lanes get one independent ring and cursor each, driven by the same `pdc_delay_block` kernel.
///
/// # Why it is not latency
///
/// PDC equalizes *unrequested* arrival-time skew: `pdc::timings` computes every path's arrival
/// from declared node latency and inserts compensating delays to make them agree. A track delay is
/// the opposite -- a time shift the session asked for. Declaring it as node latency would make PDC
/// insert matching delays on every other path and cancel exactly the alignment the user wanted. So
/// a `TrackDelay` node contributes **zero** to `GraphNode.latency`, contributes nothing to
/// `TimingResult::total_delay`, and never appears in `inserted_delays`. Its bytes are charged to
/// the estimate separately (see `estimate::resource_estimate`); its samples are charged to nothing.
pub(crate) struct TrackDelayLine {
    left: Box<[f32]>,
    right: Box<[f32]>,
    left_cursor: usize,
    right_cursor: usize,
}

impl TrackDelayLine {
    pub(crate) fn new(left: usize, right: usize) -> Self {
        Self {
            left: vec![0.0; left].into_boxed_slice(),
            right: vec![0.0; right].into_boxed_slice(),
            left_cursor: 0,
            right_cursor: 0,
        }
    }

    /// This line's designed-word comparison: the two lanes' declared delays, compared exactly.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// `process` reads one designed word per lane -- the ring length, which **is**
    /// `delay_samples` for that lane. The cursors and the ring contents are running state, not
    /// designed words, and are excluded for the same reason `InputChainState` is excluded from the
    /// input stage's list.
    ///
    /// A track whose lanes declare different delays produces genuinely different left and right
    /// audio out of a single source channel, so it is not mono-collapsible. That verdict is also
    /// taken at prepare, from the session, by `session_structural_symmetry`; this is the same
    /// fact answered by the object that owns the rings.
    #[cfg(test)]
    pub(crate) const fn channels_agree(&self) -> bool {
        self.left.len() == self.right.len()
    }

    #[cfg(test)]
    pub(crate) const fn lane_samples(&self) -> [usize; 2] {
        [self.left.len(), self.right.len()]
    }
}

// REALTIME_POLICY_BEGIN
impl TrackDelayLine {
    pub(crate) fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        delay_lane(&mut self.left, &mut self.left_cursor, left);
        delay_lane(&mut self.right, &mut self.right_cursor, right);
    }
}

/// One lane of a track delay: the same two-segment swap `CompensationDelay::process` runs, over one
/// ring with its own cursor. No `%`, no per-sample loop, and a block longer than the ring is walked
/// in ring-sized takes exactly as the PDC caller loop walks it.
fn delay_lane(ring: &mut [f32], cursor: &mut usize, block: &mut [f32]) {
    let samples = ring.len();
    if samples == 0 {
        return;
    }
    let mut offset = 0;
    while offset < block.len() {
        let take = core::cmp::min(samples, block.len() - offset);
        pdc_delay_block(ring, cursor, &mut block[offset..offset + take]);
        offset += take;
    }
}
// REALTIME_POLICY_END

/// What an op does to its reduced output.
pub(crate) enum NodeKind {
    /// A stage boundary, a submix or the session output: the reduction is the whole node.
    Identity,
    /// A track input filled by the coordinator's source set: no reduction, no processing.
    SourceInput,
    /// A track input filled by the coordinator's source set, then time-aligned in place.
    ///
    /// This variant **subsumes** [`NodeKind::SourceInput`] rather than sitting beside it. An input
    /// node has no graph inputs and the coordinator has already written its output buffer for this
    /// block, so there is no reduction to run: `reduce_plane` over an empty input list would
    /// `fill(0.0)` straight over the source audio. `execute_op` therefore takes this arm in the
    /// same early-return position the `SourceInput` arm occupies, keeps that arm's fill semantics
    /// (there is no fill), and delays the buffer in place.
    ///
    /// A track that declares zero delay on both lanes is never lowered to this variant -- it falls
    /// through to `SourceInput` exactly as before, so the compiled program of an undelayed session
    /// is structurally identical to the program that session compiled to before this feature
    /// existed.
    TrackDelay {
        /// Index into the runtime's track-delay lines.
        line: u32,
        /// Whether the two lanes declared the **same** delay, cached from the lowering.
        ///
        /// The witness is asked without the delay lines in hand, and `TrackDelayLine` is where the
        /// truth lives; `the_node_witness_agrees_with_its_line` keeps the two from drifting.
        channels_agree: bool,
    },
    /// A host-supplied processor.
    Bound(Box<dyn GraphRuntimeProcessor>),
    /// A track-local prepared native effect.
    Effect(GraphPreparedEffect),
    /// A track-local prepared native effect that a live console drives (issue #140 A).
    ///
    /// A separate variant from [`NodeKind::Effect`] on purpose, in the shape #137 D1 fixed for
    /// `ConsoleMatrixProcessor`: the console-free arm keeps the exact `&[]` call and the exact
    /// storage it had before this issue, so "a session with no console renders the same bits and
    /// holds the same bytes" is a property of the code rather than a claim about it.
    ConsoleEffect(Box<ConsoleEffect>),
    /// A route's 2x2 matrix, with the route gain already folded in (D3).
    Route([f32; 4]),
    /// A homogeneous-bank member: the reduction gathers its input, the bank does the work.
    BankMember,
}

/// One prepared native effect plus everything its live-console channel needs (issue #140 A).
///
/// Sized once, at bind, from the effect's own prepared metadata: the staging window is exactly
/// `PreparedEffectMetadata::automation_capacity` spans and the shunt's delay line is exactly
/// `PreparedEffectMetadata::latency` samples. Render allocates nothing and frees nothing.
pub(crate) struct ConsoleEffect {
    pub(crate) effect: GraphPreparedEffect,
    control: Box<EffectControlLane>,
    /// `automation_capacity` spans; only `[..staged]` is ever handed to the effect.
    spans: Box<[PreparedAutomationSpan]>,
    /// Latency-preserving dry path, so live bypass keeps the effect's declared latency exactly
    /// and therefore leaves every compiled PDC route timing correct.
    shunt: BypassShunt,
    /// Issue #143 D3: this instance's observation taps, or `None` in a plan with no observation
    /// capacity. `None` is one null pointer and one predicted branch per block -- and, crucially,
    /// it is the *only* observation state such a plan holds.
    observation: Option<Box<ObservationLane>>,
}

impl ConsoleEffect {
    fn new(
        effect: GraphPreparedEffect,
        control: Box<EffectControlLane>,
        observation: Option<Box<ObservationLane>>,
        frames: usize,
    ) -> Self {
        let capacity = effect.metadata.automation_capacity as usize;
        let latency = usize::try_from(effect.metadata.latency.0).unwrap_or(usize::MAX);
        Self {
            observation,
            spans: vec![
                PreparedAutomationSpan {
                    kind: effect_contract::AutomationSpanKind::Point,
                    channel: effect_contract::ParameterChannel::Both,
                    parameter_index: 0,
                    start_sample: 0,
                    end_sample: 0,
                    start_value: 0.0,
                    end_value: 0.0,
                };
                capacity
            ]
            .into_boxed_slice(),
            shunt: BypassShunt::new(frames, latency),
            effect,
            control,
        }
    }
}

// REALTIME_POLICY_BEGIN
/// Publish every armed tap of one prepared instance, after `process` returned (issue #143 D2).
///
/// **After**, always. The reading is "the value at the end of the block", so taking it before
/// `process` would report the previous block's state against this block's window -- the exact
/// off-by-one #137's E1 caught on the command side, mirrored here (E3's red mutation).
///
/// The whole of level-2 zero is the `wants` call: an unarmed tap's effect state is never read,
/// never folded and never stored, and a plan with no capacity at all never reaches this function
/// because `observation` is `None`.
///
/// Issue #163 phase 4 item 6 adds the lane-level gate in front of that per-tap one. `wants` made
/// the *state read* free for an unarmed tap but still walked every declared tap of every driven
/// effect on every block, so a capable-but-unsubscribed plan paid O(taps) where #143 promises
/// "one predicted branch per driven effect per block". `any_armed` is that one branch, and it is
/// the branch its own doc comment has always described itself as. The per-tap `wants` stays: it
/// is what keeps an armed lane from reading an unarmed sibling tap, and the two gates are a
/// conjunction, never a replacement.
fn publish_observations(
    observation: &mut ObservationLane,
    processor: &dyn PreparedNativeEffect,
    first_sample: u64,
    frames: u64,
) {
    if !observation.any_armed() {
        return;
    }
    let mut sample = ObservationSample {
        left: 0.0,
        right: 0.0,
    };
    for tap in 0..observation.len() {
        if !observation.wants(tap) {
            continue;
        }
        let index = tap as u32;
        if processor.observe_resident(index, &mut sample) {
            observation.accumulate(tap, sample, first_sample, frames);
        }
    }
}
// REALTIME_POLICY_END

/// One delayed input, staged into a scratch buffer on the way in.
#[derive(Clone, Copy)]
pub(crate) struct StagedInput {
    /// Buffer the producer wrote.
    pub(crate) source: u32,
    /// Scratch buffer the delayed copy lands in; this is what the op reads.
    pub(crate) staging: u32,
    /// Index into [`Runtime::delays`].
    pub(crate) line: u32,
}

/// One executable step: reduce the inputs, do the node's work, then let the observers look.
pub(crate) struct RuntimeOp {
    /// Buffers to reduce, in stable edge-ID order (D9). These are the *effective* reads: a
    /// delayed input names its staging buffer, not the producer's.
    pub(crate) inputs: Box<[u32]>,
    /// Delayed inputs this op stages itself. Empty in the native executor, whose coordinator
    /// stages every edge between partitions.
    pub(crate) staged: Box<[StagedInput]>,
    pub(crate) sidechain: Option<u32>,
    pub(crate) output: u32,
    pub(crate) kind: NodeKind,
    /// This node's observers, by handle, followed by the observers of every alias that resolves
    /// to this op's output buffer, in schedule order (`program::Tap`).
    pub(crate) observers: Box<[GraphNodeObserverBinding]>,
}

/// One scheduling unit: a single op, or a whole homogeneous bank.
pub(crate) enum RuntimeUnit {
    Op(RuntimeOp),
    Bank {
        /// Slot major, `lanes` ops per slot. A single-slot chain has exactly `lanes` of them.
        ///
        /// Only the first slot's ops are *executed*: they reduce their graph inputs into their
        /// output buffers, which is what the chain then gathers. Every later slot's audio is
        /// computed by the chain itself, so running its op would overwrite the scatter with a
        /// copy of the chain's input. Later slots are still carried here because they still own
        /// their observers and their output buffers.
        members: Box<[RuntimeOp]>,
        /// Lanes per slot; `members.len()` is a whole multiple of it.
        lanes: usize,
        chain: BankChain,
        /// One entry per lane when this chain's epilogue folds its routes into the master bus,
        /// empty otherwise (issue #218). Decided once, at bind, by [`route_fold`].
        fold: Box<[FoldLane]>,
        /// The master buffer a folded lane accumulates into. Meaningless when `fold` is empty.
        master: u32,
    },
}

impl RuntimeUnit {
    pub(crate) fn qualification_counters(&self) -> [u64; 2] {
        match self {
            Self::Op(_) => [0, 0],
            Self::Bank { chain, .. } => chain.qualification_counters(),
        }
    }

    pub(crate) fn transposes(&self) -> u64 {
        match self {
            Self::Op(_) => 0,
            Self::Bank { chain, .. } => chain.transposes(),
        }
    }

    /// `[blocks rendered collapsed, cohorts that can collapse at all]` for this unit.
    ///
    /// A per-block count and a bind-time one, together, because either alone is unreadable: the
    /// block count says the collapse fired but not out of how many chains, and the cohort count
    /// says a chain *could* collapse but not that any block did. A single op contributes neither
    /// -- the per-instance path declines in this milestone.
    pub(crate) fn collapse_counters(&self) -> [u64; 2] {
        match self {
            Self::Op(_) => [0, 0],
            Self::Bank { chain, .. } => [chain.collapses(), u64::from(chain.can_collapse())],
        }
    }

    /// `[disengages, re-engages, agreement proofs]` for this unit (mono-collapse M3).
    ///
    /// The cycle's three edges. `collapse_counters` counts blocks, and a block count cannot tell a
    /// chain that collapsed throughout from one that collapsed, stopped and started again -- which
    /// is the transition M3 is about. See `BankChain::collapse_transitions`.
    pub(crate) fn collapse_transitions(&self) -> [u64; 3] {
        match self {
            Self::Op(_) => [0; 3],
            Self::Bank { chain, .. } => chain.collapse_transitions(),
        }
    }

    /// Force this unit's collapse off (or back on). Bind-time; see `BankChain::force_mono_collapse_off`.
    pub(crate) fn force_mono_collapse_off(&mut self, forced: bool) {
        if let Self::Bank { chain, .. } = self {
            chain.force_mono_collapse_off(forced);
        }
    }

    /// `[bank chains, bound bank slots]` this unit realises (issue #181).
    ///
    /// The two were the same number for every plan before a cohort chain could carry more than
    /// one slot, which is precisely why G5 could not tell "one round-trip per chain" from "one
    /// per slot". Reporting both is what lets the gate state the law it means.
    pub(crate) fn bank_shape(&self) -> [u64; 2] {
        match self {
            Self::Op(_) => [0, 0],
            Self::Bank { members, lanes, .. } => [1, (members.len() / lanes) as u64],
        }
    }

    /// Exact engine-owned observation bytes this unit retains (issue #143 R7).
    pub(crate) fn observation_retained_bytes(&self) -> usize {
        match self {
            Self::Op(op) => op.kind.observation_retained_bytes(),
            Self::Bank { members, chain, .. } => {
                members
                    .iter()
                    .map(|member| member.kind.observation_retained_bytes())
                    .sum::<usize>()
                    + chain.observation_retained_bytes()
            }
        }
    }

    /// `[collapse-eligible lanes, lanes] ` this unit realises. Evidence and gates only.
    ///
    /// A bank counts its active lanes; a single op counts one "lane", its own node. Nothing on
    /// the render path calls this.
    pub(crate) fn symmetry_counters(&self) -> [u64; 2] {
        match self {
            Self::Op(op) => [u64::from(op.kind.channel_symmetry().eligible()), 1],
            Self::Bank { chain, .. } => chain.symmetry_counters(),
        }
    }

    /// One flag per active lane, in lane order (mono-collapse M1).
    ///
    /// The localisable form of [`symmetry_counters`](Self::symmetry_counters); folding this gives
    /// that, which is what keeps the census and the per-unit rows from ever disagreeing.
    pub(crate) fn lane_eligibility(&self) -> Vec<bool> {
        match self {
            Self::Op(op) => vec![op.kind.channel_symmetry().eligible()],
            Self::Bank { chain, .. } => chain.active_lane_eligibility(),
        }
    }

    /// `[observed stages, declared taps, armed taps]` for this unit (issue #143 E5).
    pub(crate) fn observation_binding_counts(&self) -> [u64; 3] {
        match self {
            Self::Op(op) => op.kind.observation_binding_counts(),
            Self::Bank { members, chain, .. } => {
                let mut total = chain.observation_binding_counts();
                for member in members.iter() {
                    let counts = member.kind.observation_binding_counts();
                    for (slot, value) in total.iter_mut().zip(counts) {
                        *slot = slot.saturating_add(value);
                    }
                }
                total
            }
        }
    }
}

impl NodeKind {
    /// This op's channel-symmetry witness.
    ///
    /// A bank member says nothing: its work is the chain's, and the chain's slots carry the
    /// witness for it. An identity, a source input and a route are not upstream-of-seam track
    /// work at all. Everything else answers for itself, and the two effect variants answer with
    /// the effect's own designed-word comparison -- plus, for a console-driven one, the live terms
    /// its drain maintains.
    pub(crate) fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        let designed = |symmetric: bool| {
            if symmetric {
                ChannelSymmetryWitness::SYMMETRIC
            } else {
                ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::DESIGNED)
            }
        };
        match self {
            // The one non-bank stage that can be asymmetric upstream of the seam: two lanes with
            // different declared delays turn one source channel into two different signals, so the
            // track is not collapsible. The same verdict is taken at prepare, from the session, by
            // `session_structural_symmetry` -- which is what actually arms the chain; this arm
            // is the plan's own evidence row agreeing with it.
            Self::TrackDelay { channels_agree, .. } => designed(*channels_agree),
            Self::Effect(effect) => designed(effect.processor.channel_symmetry()),
            Self::ConsoleEffect(console) => designed(console.effect.processor.channel_symmetry())
                .and(console.control.symmetry()),
            Self::Bound(processor) => processor.channel_symmetry(),
            // Not a per-track upstream stage: nothing here can make the two channels disagree,
            // and nothing here is collapsed.
            Self::Identity | Self::SourceInput | Self::Route(_) | Self::BankMember => {
                ChannelSymmetryWitness::SYMMETRIC
            }
        }
    }

    /// `[observed stages, declared taps, armed taps]` for one op.
    pub(crate) fn observation_binding_counts(&self) -> [u64; 3] {
        let Self::ConsoleEffect(console) = self else {
            return [0, 0, 0];
        };
        let Some(observation) = console.observation.as_deref() else {
            return [0, 0, 0];
        };
        let armed = (0..observation.len())
            .filter(|tap| observation.is_armed(*tap))
            .count() as u64;
        [1, observation.len() as u64, armed]
    }

    /// Exact engine-owned observation bytes this op retains. Zero for an unobserved op.
    pub(crate) fn observation_retained_bytes(&self) -> usize {
        match self {
            Self::ConsoleEffect(console) => console
                .observation
                .as_deref()
                .map_or(0, ObservationLane::retained_bytes),
            _ => 0,
        }
    }
}

// REALTIME_POLICY_BEGIN
/// Planar per-lane view over the arena slots a bank chain gathers from and scatters to.
///
/// The two lists differ only for a multi-slot cohort chain (issue #181): the chain gathers the
/// first slot's member outputs and scatters the *last* slot's, because the last slot's buffer is
/// what the rest of the graph reads. A single-slot chain passes the same list twice, which is the
/// in-place round-trip it has always done.
struct ArenaMembers<'a> {
    lease: &'a mut ArenaLease,
    inputs: &'a [u32],
    outputs: &'a [u32],
    /// One entry per lane when this chain's epilogue folds its routes, empty otherwise.
    fold: &'a [FoldLane],
    /// The buffer a folded lane's routed tile lands in. Meaningless when `fold` is empty.
    master: u32,
}

/// One folded lane's epilogue: the route's bind-folded 2x2, and how its tile meets the master.
///
/// `store` is the D9 association restated for a scatter-accumulate: the reduction this replaces is
/// `sum2_block(in0, in1)` then `sum_into_block` left to right, so the **first** contributor writes
/// the master and every later one adds into it. Writing then adding computes `in0 + in1` with the
/// same operation and the same rounding `sum2_block` does. Zero-filling first and accumulating
/// every contributor would not: `0.0 + (-0.0)` is `+0.0` where `in0 + in1` on a `-0.0` first
/// contributor is `-0.0`, and a fan-in-one master would lose its sign outright.
#[derive(Clone, Copy)]
pub(crate) struct FoldLane {
    /// The route's 2x2 with its linear gain already folded in (D3), exactly as `NodeKind::Route`
    /// carries it: the epilogue applies the same constants through the same `mix2x2_block`.
    coefficients: [f32; 4],
    /// This lane is the master's first contributor, so it stores rather than accumulates.
    store: bool,
}

impl BankMembers for ArenaMembers<'_> {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        self.lease.read_stereo(self.inputs[lane])
    }
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
        self.lease.write_stereo(self.outputs[lane])
    }
    fn distinct_planes_mut(&mut self, lanes: usize, frames: usize) -> Option<BankPlaneViews<'_>> {
        match lanes {
            4 => {
                let buffers: [u32; 4] = self.outputs.get(..4)?.try_into().ok()?;
                Some(BankPlaneViews::from_four(
                    self.lease.write_stereo_many(&buffers, frames)?,
                    frames,
                )?)
            }
            8 => {
                let buffers: [u32; 8] = self.outputs.get(..8)?.try_into().ok()?;
                Some(BankPlaneViews::from_eight(
                    self.lease.write_stereo_many(&buffers, frames)?,
                    frames,
                )?)
            }
            _ => None,
        }
    }
    /// The route and the master accumulation, in the lane's own transposed tile (issue #218).
    ///
    /// Three frozen facts make this the reduction it replaces rather than a re-derivation of it:
    ///
    /// * the tile is `frames` words long, exactly as `lease.write_stereo` hands the route op its
    ///   buffer, so `mix2x2_block::<FrameLane>` takes the same vector/tail split and emits the same
    ///   per-sample op order over the same constants;
    /// * the route stays its own arithmetic step. It is **not** merged into the matrix slot above
    ///   it: two 2x2s multiplied out is a different rounding, and D3 folds the *gain* into the
    ///   route's own coefficients and nothing else;
    /// * the master meets `sum_into_block::<FrameLane>`, the same kernel `reduce_plane`'s
    ///   left-to-right accumulation used, in the same order -- which `route_fold` proves rather
    ///   than assumes.
    ///
    /// The accumulation is two independent per-plane calls, so a mono collapse drops one of them
    /// and changes nothing else. The 2x2 above it is irreducibly cross-plane -- that is what a
    /// route *is* (D3) -- and is not something a plane-wise factoring could have separated.
    fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
        let fold = self.fold[lane];
        mix2x2_block::<FrameLane>(left, right, fold.coefficients);
        let (master_left, master_right) = self.lease.write_stereo(self.master);
        if fold.store {
            master_left.copy_from_slice(left);
            master_right.copy_from_slice(right);
        } else {
            sum_into_block::<FrameLane>(master_left, left);
            sum_into_block::<FrameLane>(master_right, right);
        }
    }

    fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
        let lane_ids = cohort.lane_ids();
        let count = lane_ids.len();
        let frames = cohort.frames();
        let stride = cohort.stride();
        let Some(max_lane) = lane_ids.iter().copied().max() else {
            return;
        };
        if lane_ids
            .iter()
            .enumerate()
            .any(|(index, lane)| lane_ids[..index].contains(lane))
        {
            return;
        }
        let Some(required) = max_lane
            .checked_add(1)
            .and_then(|lanes| lanes.checked_mul(stride))
        else {
            return;
        };
        if count == 0
            || count > 8
            || stride < frames
            || cohort.left().len() < required
            || cohort.right().len() < required
            || frames > self.lease.frames()
            || !self.lease.writes(self.master)
        {
            return;
        }
        let mut coefficients = [[0.0; 4]; 8];
        let mut stores = [false; 8];
        let mut ids = [0usize; 8];
        for (index, &lane) in lane_ids.iter().enumerate() {
            ids[index] = lane;
            let Some(fold) = self.fold.get(lane).copied() else {
                return;
            };
            if index != 0 && fold.store {
                return;
            }
            coefficients[index] = fold.coefficients;
            stores[index] = fold.store;
        }
        let mut left = cohort;
        for (index, coefficient) in coefficients[..count].iter().enumerate() {
            let Some((left_plane, right_plane)) = left.planes_mut(ids[index]) else {
                return;
            };
            mix2x2_block::<FrameLane>(left_plane, right_plane, *coefficient);
        }
        let mut left_inputs: [&[f32]; 8] = [&[]; 8];
        let mut right_inputs: [&[f32]; 8] = [&[]; 8];
        for index in 0..count {
            let start = ids[index] * stride;
            left_inputs[index] = &left.left()[start..start + frames];
            right_inputs[index] = &left.right()[start..start + frames];
        }
        let (master_left, master_right) = self.lease.write_stereo(self.master);
        let initial_store = stores[0];
        let valid_left = ordered_accumulate_block::<FrameLane>(
            &mut master_left[..frames],
            &left_inputs[..count],
            initial_store,
        );
        let valid_right = ordered_accumulate_block::<FrameLane>(
            &mut master_right[..frames],
            &right_inputs[..count],
            initial_store,
        );
        debug_assert!(valid_left && valid_right);
    }
}

// REALTIME_POLICY_END

/// The static half of one unit's [`PlanUnitEligibility`] row, fixed at bind.
///
/// Split from the dynamic half because the two move at different times and for different reasons.
/// Which track a lane renders, how many stages a unit has and which side of the seam each stage
/// sits on are decided when the plan is built and cannot change afterwards; how many of the unit's
/// lanes are *eligible* moves whenever a live-console record is drained. So the identity is
/// computed once, here, from the node ids the lowering already resolved -- and the counters are
/// pulled from the chain on demand.
pub(crate) struct UnitIdentity {
    pub(crate) banked: bool,
    pub(crate) stages: u32,
    pub(crate) upstream_of_seam_stages: u32,
    pub(crate) lane_tracks: Box<[Box<str>]>,
}

/// Which side of the fader/matrix seam one graph node's stage sits on.
///
/// The seam is `effect_contract::SeamSide`'s: the 2x2 matrix is the earliest
/// genuinely cross-channel operation in the strip and the fader is immediately before it, so
/// everything from `PostFader` on reads the plane a collapsed track duplicated and may legitimately
/// differ between the channels. It is read off `TrackStage` rather than off the processor, because
/// the *stage* is what decides it: a fader bank and an EQ bank are the same kind of object and only
/// their position in the strip separates them.
///
/// A node that is not per-track strip work at all -- a route, a submix, the output, a compensation
/// delay -- is **not** upstream: it is not a stage a collapse would have computed once, so counting
/// it as upstream would let a route op's unconditionally-symmetric witness read as collapse
/// evidence, which is precisely what the seam classification exists to prevent.
fn upstream_of_seam(node: &GraphNodeId) -> bool {
    match node {
        GraphNodeId::TrackStage { stage, .. } => !matches!(
            stage,
            crate::TrackStage::PostFader | crate::TrackStage::PostMatrix
        ),
        GraphNodeId::Effect(_) => true,
        GraphNodeId::Route { .. }
        | GraphNodeId::Submix { .. }
        | GraphNodeId::Output { .. }
        | GraphNodeId::CompensationDelay { .. } => false,
    }
}

/// The track one graph node renders, or `""` for a node that names none.
fn node_track(node: &GraphNodeId) -> Box<str> {
    match node {
        GraphNodeId::TrackStage { track_id, .. } => Box::from(track_id.as_str()),
        GraphNodeId::Effect(effect) => Box::from(effect.track_id.as_str()),
        GraphNodeId::Route { .. }
        | GraphNodeId::Submix { .. }
        | GraphNodeId::Output { .. }
        | GraphNodeId::CompensationDelay { .. } => Box::from(""),
    }
}

/// Ops, their audio and their delay lines: everything one executor (or one native parcel) owns.
pub(crate) struct Runtime {
    /// This runtime's checked view of the plan's shared arena.
    pub(crate) lease: ArenaLease,
    pub(crate) delays: Box<[CompensationDelay]>,
    /// Input-side track alignment lines, one per track that declared a nonzero delay on either
    /// lane. Empty on every session that declared none, which is what keeps an undelayed plan on
    /// the bytes and the program it had before this feature existed.
    pub(crate) track_delays: Box<[TrackDelayLine]>,
    pub(crate) units: Box<[RuntimeUnit]>,
    /// One row per unit, in `units` order: the bind-time half of the collapse-eligibility query.
    pub(crate) identity: Box<[UnitIdentity]>,
    /// Scratch for a bank chain's gather-source buffers, sized to the widest bank at bind.
    bank_inputs: Box<[u32]>,
    /// Scratch for a bank chain's scatter-target buffers, sized to the widest bank at bind.
    bank_outputs: Box<[u32]>,
    /// Lanes whose scatter this bind pointed at their consumer's buffer (issue #202 rec 3).
    redirects: u64,
    /// Lanes whose route and master accumulation this bind folded into their chain's epilogue
    /// (issue #218).
    folds: u64,
}

impl Runtime {
    /// Lanes whose scatter this bind pointed at their consumer's buffer (issue #202 rec 3).
    pub(crate) const fn scatter_redirects(&self) -> u64 {
        self.redirects
    }

    /// `[blocks rendered collapsed, cohorts that can collapse at all]`, over every unit.
    pub(crate) fn collapse_counters(&self) -> [u64; 2] {
        self.units.iter().fold([0, 0], |mut total, unit| {
            let counters = unit.collapse_counters();
            for (value, add) in total.iter_mut().zip(counters) {
                *value = value.saturating_add(add);
            }
            total
        })
    }

    /// `[disengages, re-engages, agreement proofs]`, over every unit.
    pub(crate) fn collapse_transitions(&self) -> [u64; 3] {
        self.units.iter().fold([0; 3], |mut total, unit| {
            let counters = unit.collapse_transitions();
            for (value, add) in total.iter_mut().zip(counters) {
                *value = value.saturating_add(add);
            }
            total
        })
    }

    /// Perform the collapse's structural join and arm every chain it admits.
    ///
    /// `eligible` answers, for one track id, whether that track's **structural** witness holds --
    /// the `SOURCE` term, which lives on the control plane and is keyed by track id. This is the
    /// only place in the engine where the two halves of the channel-symmetry witness meet: the
    /// runtime half is per lane and source agnostic, the structural half is per track, and
    /// `UnitIdentity::lane_tracks` is the relation between the two keys.
    ///
    /// All lanes or nothing, exactly as `BankChain::all_lanes_symmetric` is: a cohort with one
    /// two-source lane saves nothing by collapsing the others, because the vector op runs every
    /// lane regardless. Making a cohort homogeneous is the planner's job (`CohortPoolClass`).
    ///
    /// A unit whose lane list is empty arms nothing: a chain that names no track is one this join
    /// cannot speak for.
    pub(crate) fn arm_mono_collapse(&mut self, eligible: &dyn Fn(&str) -> bool) {
        for (unit, identity) in self.units.iter_mut().zip(self.identity.iter()) {
            let RuntimeUnit::Bank { chain, .. } = unit else {
                continue;
            };
            let tracks = &identity.lane_tracks;
            let armed = !tracks.is_empty()
                && tracks
                    .iter()
                    .all(|track| !track.is_empty() && eligible(track));
            chain.arm_mono_collapse(armed);
        }
    }

    /// Force every chain's collapse off (or back on). Bind-time, off the render thread.
    pub(crate) fn force_mono_collapse_off(&mut self, forced: bool) {
        for unit in self.units.iter_mut() {
            unit.force_mono_collapse_off(forced);
        }
    }

    /// Lanes whose route and master accumulation this bind folded into the chain's epilogue.
    pub(crate) const fn route_folds(&self) -> u64 {
        self.folds
    }

    pub(crate) fn new(
        lease: ArenaLease,
        delays: Vec<CompensationDelay>,
        track_delays: Vec<TrackDelayLine>,
        units: Vec<RuntimeUnit>,
        identity: Vec<UnitIdentity>,
        redirects: u64,
        folds: u64,
    ) -> Self {
        debug_assert_eq!(identity.len(), units.len());
        let widest = units
            .iter()
            .map(|unit| match unit {
                RuntimeUnit::Op(_) => 0,
                RuntimeUnit::Bank { lanes, .. } => *lanes,
            })
            .max()
            .unwrap_or(0);
        Self {
            lease,
            delays: delays.into_boxed_slice(),
            track_delays: track_delays.into_boxed_slice(),
            units: units.into_boxed_slice(),
            identity: identity.into_boxed_slice(),
            bank_inputs: vec![0; widest].into_boxed_slice(),
            bank_outputs: vec![0; widest].into_boxed_slice(),
            redirects,
            folds,
        }
    }

    // REALTIME_POLICY_BEGIN
    /// The audio of one buffer, for the source set to fill and for the host copy-out.
    pub(crate) fn buffer_mut(&mut self, buffer: u32) -> (&mut [f32], &mut [f32]) {
        self.lease.write_stereo(buffer)
    }

    /// The audio of one buffer, shared.
    pub(crate) fn buffer(&self, buffer: u32) -> (&[f32], &[f32]) {
        self.lease.read_stereo(buffer)
    }

    /// Runs unit `index`. Every producer this unit reads precedes it in `units`, or was written
    /// by a strictly earlier wave.
    pub(crate) fn execute(&mut self, index: usize, first_sample: u64) -> Result<(), RenderError> {
        let Self {
            lease,
            delays,
            track_delays,
            units,
            bank_inputs,
            bank_outputs,
            ..
        } = self;
        let delays: &mut [CompensationDelay] = delays;
        let track_delays: &mut [TrackDelayLine] = track_delays;
        match &mut units[index] {
            RuntimeUnit::Op(op) => execute_op(op, lease, delays, track_delays, first_sample),
            RuntimeUnit::Bank {
                members,
                lanes,
                chain,
                fold,
                master,
            } => {
                let lanes = *lanes;
                // Only the first slot reduces graph inputs; the chain computes the rest -- and a
                // member whose whole reduction *is* the dedication copy does not even do that:
                // the gather reads its producer's buffer directly. See `bank_gather_source`.
                for (lane, member) in members[..lanes].iter_mut().enumerate() {
                    if let Some(source) = bank_gather_source(member) {
                        bank_inputs[lane] = source;
                    } else {
                        execute_op(member, lease, delays, track_delays, first_sample)?;
                        bank_inputs[lane] = member.output;
                    }
                }
                let last = members.len() - lanes;
                for lane in 0..lanes {
                    bank_outputs[lane] = members[last + lane].output;
                }
                let frames = lease.frames();
                chain.run(
                    &mut ArenaMembers {
                        lease,
                        inputs: &bank_inputs[..lanes],
                        outputs: &bank_outputs[..lanes],
                        fold,
                        master: *master,
                    },
                    u32::try_from(frames).unwrap_or(u32::MAX),
                    first_sample,
                )
            }
        }
    }

    /// Runs the observers of unit `index`, in member order and then by handle.
    pub(crate) fn observe_unit(
        &mut self,
        index: usize,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let Self { lease, units, .. } = self;
        match &mut units[index] {
            RuntimeUnit::Op(op) => observe(op, lease, first_sample),
            RuntimeUnit::Bank { members, .. } => {
                for member in members.iter_mut() {
                    observe(member, lease, first_sample)?;
                }
                Ok(())
            }
        }
    }
    // REALTIME_POLICY_END
}

// REALTIME_POLICY_BEGIN
/// The buffer a first-slot bank member's gather may read **instead of** the member's own output,
/// or `None` when the member has to run.
///
/// The SIMD-rack effects sit in dedicated racks (`program::is_dedicated`), so their ops are never
/// `in_place` and `reduce_plane` memcpys the upstream output into a buffer whose only reader is
/// this bank's gather -- after which the chain's scatter fully overwrites that same buffer. The
/// copy is therefore pure cost, and pointing the gather at the producer removes it. `None` on any
/// doubt; every clause below is one way the copy could be load-bearing:
///
/// * **Nothing but the reduction.** [`NodeKind::BankMember`] is the kind whose `execute_op` body is
///   empty apart from `reduce_plane`, so skipping the op skips exactly the copy and nothing else.
/// * **One undelayed, unmixed input.** Two inputs is a sum and zero inputs is a `fill(0.0)`; either
///   way the gather source is a value no single buffer holds. A sidechain already blocks banking
///   (#96 F9) and a `staged` input owns a compensation delay line that must still be pumped.
/// * **Not already in place.** `inputs[0] == output` is the lowering having elided the copy
///   already, and redirecting the gather would change nothing.
///
/// What stays true of the member's own output buffer: for a single-slot chain it *is* the scatter
/// target, so every later reader -- an observer, a `program::Tap`'s observer, the session output --
/// sees exactly the bits it saw before. For a multi-slot cohort chain the scatter lands in the last
/// slot's buffer instead and this one is left holding the previous block's words, which is sound
/// for the reason `chains_into` already requires and checks: the first slot has exactly one reader
/// (the next slot's op, which never reduces), no tap, no observer, and is not the session output.
///
/// This does not touch `is_dedicated`, which stays a classification by node kind: dedication *by
/// bank membership* was measured and rejected, and `program::lower` records why (#169).
fn bank_gather_source(member: &RuntimeOp) -> Option<u32> {
    if !matches!(member.kind, NodeKind::BankMember)
        || !member.staged.is_empty()
        || member.sidechain.is_some()
    {
        return None;
    }
    match &*member.inputs {
        [single] if *single != member.output => Some(*single),
        _ => None,
    }
}

/// The one implementation of node semantics, shared by both executors.
fn execute_op(
    op: &mut RuntimeOp,
    lease: &mut ArenaLease,
    delays: &mut [CompensationDelay],
    track_delays: &mut [TrackDelayLine],
    first_sample: u64,
) -> Result<(), RenderError> {
    let output = op.output;
    if let NodeKind::TrackDelay { line, .. } = op.kind {
        // The delayed form of the `SourceInput` arm below, in the same position and for the same
        // reason: the coordinator's source set already wrote this node's output for this block, and
        // an input node has no graph inputs, so there is no reduction to run. Falling through would
        // reach `reduce_plane` with an empty input list, whose `[]` arm fills the buffer with `0.0`
        // -- straight over the source audio. The alignment therefore happens here, in place, and
        // this returns exactly where the undelayed arm returns.
        let (left, right) = lease.write_stereo(output);
        track_delays[line as usize].process(left, right);
        return Ok(());
    }
    if matches!(op.kind, NodeKind::SourceInput) {
        // The coordinator's source set already wrote this node's output for this block.
        return Ok(());
    }
    // A delayed edge is the only inter-parcel copy left, and the *consuming* parcel makes it.
    for staged in &op.staged {
        {
            let (destination, input) = lease.write_read(0, staged.staging, staged.source);
            destination.copy_from_slice(input);
        }
        {
            let (destination, input) = lease.write_read(1, staged.staging, staged.source);
            destination.copy_from_slice(input);
        }
        let (staged_left, staged_right) = lease.write_stereo(staged.staging);
        delays[staged.line as usize].process(staged_left, staged_right);
    }
    // The fan-in-zero fill is dead under a bound source (issue #218). `reduce_plane`'s `[]` arm
    // fills the buffer with `0.0` so that a node with no contributors renders silence; a bound
    // node with no graph inputs has a contributor, and it is the host's processor, which is
    // required to write every word of the block it is handed. Filling first and overwriting
    // second is two whole stereo blocks of stores per bound source per block -- 64 of them on the
    // intended fixture -- and not one of those words is ever read.
    //
    // The clause is exactly "no graph inputs *and* a host processor". A `NodeKind::Identity` with
    // no inputs is a submix nothing routes into and its fill **is** its audio; a `SourceInput` and
    // a `TrackDelay` are already skipped above; every other kind reduces first and processes in
    // place, so its fill is the value it processes.
    if !op.inputs.is_empty() || !matches!(op.kind, NodeKind::Bound(_)) {
        reduce_plane(lease, 0, output, &op.inputs);
        reduce_plane(lease, 1, output, &op.inputs);
    }
    match &mut op.kind {
        // `TrackDelay` returned above, before the reduction it must not run; it is named here only
        // because the match is exhaustive.
        NodeKind::TrackDelay { .. }
        | NodeKind::SourceInput
        | NodeKind::Identity
        | NodeKind::BankMember => {}
        NodeKind::Route(coefficients) => {
            let (out_left, out_right) = lease.write_stereo(output);
            mix2x2_block::<FrameLane>(out_left, out_right, *coefficients);
        }
        NodeKind::Bound(processor) => {
            let (out_left, out_right) = lease.write_stereo(output);
            processor.process(GraphBindingBlock {
                left: out_left,
                right: out_right,
                first_sample,
            })?;
        }
        NodeKind::Effect(effect) => {
            let quantum = effect.metadata.quantum;
            match op.sidechain {
                None => {
                    let (out_left, out_right) = lease.write_stereo(output);
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        None,
                        first_sample,
                        &[],
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
                Some(sidechain) => {
                    let ((out_left, out_right), (side_left, side_right)) =
                        lease.write_read_stereo(output, sidechain);
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        Some((side_left, side_right)),
                        first_sample,
                        &[],
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
            }
        }
        NodeKind::ConsoleEffect(console) => {
            // The drain runs before a single sample is touched, so an admitted record takes
            // effect on the first sample of this block -- the exact `applied_at_sample` the
            // control side was acknowledged with (#137 E1's rule, now for effects).
            // Issue #143 D3: the subscription and the parameter commands are drained by this one
            // call, so a batch that changes a threshold and arms its tap lands on one sample
            // timeline by construction rather than by two clocks agreeing.
            let staged = console.control.stage(
                &mut console.spans,
                first_sample,
                console.observation.as_deref_mut(),
            );
            // Preparation refuses a queue deeper than the effect's automation capacity, so a full
            // drain can never produce more distinct spans than the window holds. This is the
            // invariant, not a runtime policy: in release it costs nothing.
            debug_assert_eq!(staged.dropped, 0, "console staging window overflowed");
            let automation = &console.spans[..staged.staged];
            let bypassed = console.control.bypassed();
            // Issue #163 phase 4 item 4: the dry staging is read only by the `apply` below, and
            // only when this block is bypassed. `bypassed` is already decided here — the control
            // drain that could change it ran above — so the capture is skippable for an
            // un-bypassed block *unless* the shunt carries a latency line, which has to be fed on
            // every block whatever the bypass state. Both readers of `dry_*` are later in this
            // same block, so nothing crosses a block boundary and the skip moves no rendered bit.
            let capture_dry = bypassed || console.shunt.feeds_line();
            let effect = &mut console.effect;
            let quantum = effect.metadata.quantum;
            match op.sidechain {
                None => {
                    let (out_left, out_right) = lease.write_stereo(output);
                    if capture_dry {
                        console.shunt.capture(out_left, out_right);
                    }
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        None,
                        first_sample,
                        automation,
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
                Some(sidechain) => {
                    let ((out_left, out_right), (side_left, side_right)) =
                        lease.write_read_stereo(output, sidechain);
                    if capture_dry {
                        console.shunt.capture(out_left, out_right);
                    }
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        Some((side_left, side_right)),
                        first_sample,
                        automation,
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
            }
            if bypassed {
                let (out_left, out_right) = lease.write_stereo(output);
                console.shunt.apply(out_left, out_right);
            }
            // Issue #143: after `process`, and after the bypass shunt, so an observed value always
            // describes the block that was actually emitted.
            if let Some(observation) = console.observation.as_deref_mut() {
                publish_observations(
                    observation,
                    console.effect.processor.as_ref(),
                    first_sample,
                    lease.frames() as u64,
                );
            }
        }
    }
    Ok(())
}

fn observe(op: &mut RuntimeOp, lease: &ArenaLease, first_sample: u64) -> Result<(), RenderError> {
    if op.observers.is_empty() {
        return Ok(());
    }
    let (left, right) = lease.read_stereo(op.output);
    for observer in op.observers.iter_mut() {
        observer.observer.observe(GraphObservationBlock {
            left,
            right,
            first_sample,
        })?;
    }
    Ok(())
}

// REALTIME_POLICY_END

/// Ordered observers of one node, plus the observers of every alias resolving to its buffer.
pub(crate) fn take_observers(
    observers: &mut BTreeMap<crate::GraphNodeId, Vec<GraphNodeObserverBinding>>,
    nodes: impl Iterator<Item = crate::GraphNodeId>,
) -> Box<[GraphNodeObserverBinding]> {
    let mut collected = Vec::new();
    for node in nodes {
        if let Some(mut bound) = observers.remove(&node) {
            bound.sort_by_key(|observer| observer.handle);
            collected.append(&mut bound);
        }
    }
    collected.into_boxed_slice()
}

// ---------------------------------------------------------------------------------------------
// Build (control plane): one node builder, one bank builder, for both executors.
// ---------------------------------------------------------------------------------------------

use effect_contract::BankWidth;
use rack::{AoSoaScratch, BankBlock, BankSlot, BankStage, ConsoleEffectBankStage, EffectBankStage};

use crate::{
    GraphNodeBinding, GraphNodeId, GraphPreparedBuiltinBank, GraphPreparedBuiltinBankProcessor,
    GraphPreparedEffectBank, GraphSpec, PreparedRoute, RouteTransform, TrackStage,
    program::{ExecutionProgram, Op},
};

/// Adapter that lets a compiler-owned builtin bank act as a chain slot.
struct BuiltinStage(Box<dyn GraphPreparedBuiltinBankProcessor>);
impl BankStage for BuiltinStage {
    // REALTIME_POLICY_BEGIN
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(block.left, block.right, block.frames, block.first_sample)
    }
    // REALTIME_POLICY_END
    /// The drain, forwarded. `BankChain::run` calls this on every slot before it reads the
    /// collapse witness, which is the ordering the input bank's trim/polarity drain depends on.
    fn begin_block(&mut self, first_sample: u64) -> Result<(), RenderError> {
        self.0.begin_block(first_sample)
    }
    fn qualification_counters(&self) -> [u64; 2] {
        self.0.qualification_counters()
    }
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        self.0.lane_symmetry(lane)
    }
    fn seam_side(&self) -> effect_contract::SeamSide {
        self.0.seam_side()
    }
    fn supports_mono_collapse(&self) -> bool {
        self.0.supports_mono_collapse()
    }
    /// The one-plane body. `block.right` is the ungathered scratch and is not passed on: the
    /// builtin banks take a single plane, so a mis-wired collapse cannot reach a stale plane
    /// through this adapter at all.
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process_mono(block.left, block.frames, block.first_sample)
    }
    fn desymmetrize(&mut self) {
        self.0.desymmetrize();
    }
    fn channels_agree(&self) -> bool {
        self.0.channels_agree()
    }
}

/// A padded group's active mask **is** its membership: the planner emits `Some` members before
/// every `None`, so lane `i` is active exactly while `i < members`. The mask is therefore derived
/// here and stored nowhere (#86 F9's graph half).
pub(crate) fn trailing_active_mask(members: usize, width: BankWidth) -> Box<[bool]> {
    (0..width.lanes() as usize)
        .map(|lane| lane < members)
        .collect()
}

/// One chain over `slots`, in cascade order, sharing one resident block.
///
/// `active` is the chain's lane mask; every slot carries it too, because a cohort chain binds a
/// slot only when *every* lane of the group runs it (`banks::bind_rack_banks`, #96 F7). The rack
/// crate has supported multi-slot chains since it was written and unit-tests three of them; until
/// issue #181 nothing in the graph layer ever handed it more than one.
fn bank_chain(
    scratch: AoSoaScratch,
    active: Box<[bool]>,
    slots: Vec<Box<dyn BankStage>>,
) -> BankChain {
    let slots = slots
        .into_iter()
        .map(|stage| BankSlot {
            stage,
            active_lanes: active.clone(),
        })
        .collect();
    BankChain::new(scratch, active, slots).expect("validated bank shape")
}

/// Which unit a node belongs to, when it is a bank member.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Membership {
    Effect(usize),
    Builtin(usize),
}

/// Node index -> `(bank, member position)`, for every homogeneous-bank member of a plan.
pub(crate) type BankMembership = BTreeMap<u32, (Membership, usize)>;

/// One scheduling unit before it is built: its bank, if any, and the ops it owns.
pub(crate) type PlannedUnit = (Option<Membership>, Vec<usize>);

/// The one bank-membership map, built from the plan before either executor exists.
pub(crate) fn bank_membership(
    spec: &GraphSpec,
    banks: &[GraphPreparedEffectBank],
    builtin_banks: &[GraphPreparedBuiltinBank],
) -> BankMembership {
    let index_of =
        |node: &GraphNodeId| crate::program::node_index(spec, node).expect("validated member");
    let mut membership = BankMembership::new();
    for (bank, prepared) in banks.iter().enumerate() {
        for (member, id) in prepared.members.iter().enumerate() {
            membership.insert(
                index_of(&GraphNodeId::Effect(id.clone())),
                (Membership::Effect(bank), member),
            );
        }
    }
    for (bank, prepared) in builtin_banks.iter().enumerate() {
        for (member, id) in prepared.members.iter().enumerate() {
            membership.insert(index_of(id), (Membership::Builtin(bank), member));
        }
    }
    membership
}

/// Everything a bound plan hands the runtime, consumed exactly once per node.
pub(crate) struct RuntimeParts {
    pub(crate) routes: BTreeMap<GraphNodeId, RouteTransform>,
    pub(crate) effects: BTreeMap<GraphNodeId, GraphPreparedEffect>,
    /// Issue #140 A: live-console channels by effect node, taken by whichever owner renders that
    /// node -- the per-node `ConsoleEffect`, or the bank slot that holds the node's lane.
    effect_controls: BTreeMap<crate::EffectNodeId, Box<EffectControlLane>>,
    /// Issue #143 D3: observation lanes by effect node, taken by whichever owner renders that
    /// node. Empty for a plan with no observation capacity, so `node_kind` hands out `None`.
    effect_observations: BTreeMap<crate::EffectNodeId, Box<ObservationLane>>,
    pub(crate) bindings: BTreeMap<GraphNodeId, Option<Box<dyn GraphRuntimeProcessor>>>,
    pub(crate) observers: BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
    pub(crate) source_inputs: std::collections::BTreeSet<GraphNodeId>,
    /// Issue #210 phase 2: declared per-lane input delay, by track input node. Only tracks that
    /// declared a nonzero delay on at least one lane are present, so this map is empty -- and
    /// `node_kind` never leaves the `SourceInput` arm -- for every session that declares none.
    track_delays: BTreeMap<GraphNodeId, [u32; 2]>,
    /// The lines `node_kind` allocated, in the order it allocated them.
    track_delay_lines: Vec<TrackDelayLine>,
    banks: Vec<Option<GraphPreparedEffectBank>>,
    builtin_banks: Vec<Option<GraphPreparedBuiltinBank>>,
    membership: BankMembership,
    /// Render quantum, so a console-driven effect's staging and shunt are sized once, at bind.
    frames: usize,
}

impl RuntimeParts {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        spec: &GraphSpec,
        routes: Vec<PreparedRoute>,
        effects: Vec<GraphPreparedEffect>,
        effect_controls: Vec<crate::GraphEffectControlBinding>,
        effect_observations: Vec<crate::GraphEffectObservationBinding>,
        banks: Vec<GraphPreparedEffectBank>,
        builtin_banks: Vec<GraphPreparedBuiltinBank>,
        observers: Vec<GraphNodeObserverBinding>,
        bindings: Vec<GraphNodeBinding>,
        source_inputs: std::collections::BTreeSet<GraphNodeId>,
        track_delays: Vec<crate::PreparedTrackDelay>,
        frames: usize,
    ) -> Self {
        let membership = bank_membership(spec, &banks, &builtin_banks);
        let mut by_node: BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>> = BTreeMap::new();
        for observer in observers {
            by_node
                .entry(observer.node.clone())
                .or_default()
                .push(observer);
        }
        Self {
            routes: routes
                .into_iter()
                .map(|route| (route.node, route.transform))
                .collect(),
            effects: effects
                .into_iter()
                .map(|effect| (GraphNodeId::Effect(effect.id.clone()), effect))
                .collect(),
            effect_controls: effect_controls
                .into_iter()
                .map(|binding| (binding.node, binding.control))
                .collect(),
            effect_observations: effect_observations
                .into_iter()
                .map(|binding| (binding.node, binding.observation))
                .collect(),
            bindings: bindings
                .into_iter()
                .map(|binding| (binding.node, binding.processor))
                .collect(),
            observers: by_node,
            source_inputs,
            track_delays: track_delays
                .into_iter()
                .map(|delay| (delay.node, [delay.left_samples, delay.right_samples]))
                .collect(),
            track_delay_lines: Vec::new(),
            banks: banks.into_iter().map(Some).collect(),
            builtin_banks: builtin_banks.into_iter().map(Some).collect(),
            membership,
            frames,
        }
    }

    /// The one node-kind decision, shared by both executors.
    ///
    /// A route's linear gain is folded into its 2x2 coefficients here, once, at bind: render then
    /// spends two multiplies and one add per output word instead of re-applying the
    /// gain every frame (D3, #98 F4).
    fn node_kind(&mut self, node: &GraphNodeId, index: u32) -> NodeKind {
        if self.source_inputs.contains(node) {
            // The delay arm takes this branch's place rather than sitting after it: an input node
            // is a source input whether or not it is delayed, and `execute_op` must early-return
            // for both. A track with no declared delay is absent from the map and lowers to
            // `SourceInput`, byte for byte and op for op, exactly as it did before this feature.
            match self.track_delays.remove(node) {
                None => NodeKind::SourceInput,
                Some([left, right]) => {
                    let line =
                        u32::try_from(self.track_delay_lines.len()).expect("delay line index");
                    self.track_delay_lines
                        .push(TrackDelayLine::new(left as usize, right as usize));
                    NodeKind::TrackDelay {
                        line,
                        channels_agree: left == right,
                    }
                }
            }
        } else if self.membership.contains_key(&index) {
            NodeKind::BankMember
        } else if let Some(Some(processor)) = self.bindings.remove(node) {
            NodeKind::Bound(processor)
        } else if let Some(effect) = self.effects.remove(node) {
            let observation = self.effect_observations.remove(&effect.id);
            match self.effect_controls.remove(&effect.id) {
                // An observation lane is only ever created alongside a control channel -- a
                // subscription rides that queue -- so this arm is the unobserved, console-free
                // path it always was, byte for byte.
                None => NodeKind::Effect(effect),
                Some(control) => NodeKind::ConsoleEffect(Box::new(ConsoleEffect::new(
                    effect,
                    control,
                    observation,
                    self.frames,
                ))),
            }
        } else if let Some(transform) = self.routes.remove(node) {
            NodeKind::Route(folded_route(&transform))
        } else {
            NodeKind::Identity
        }
    }

    /// One chain for a whole cohort run, in slot order (issue #181).
    ///
    /// The scratch and the lane mask come from the run's first slot; every slot of a cohort
    /// covers the same lanes by construction, and `BankChain::new` re-checks it rather than
    /// trusting it.
    fn chain_for(&mut self, run: &[Membership], members: usize) -> BankChain {
        // Every slot arrives with its own `AoSoaScratch`, because a bound bank is prepared
        // without knowing whether it will end up sharing a chain. One chain has one resident
        // block, so the run keeps the first slot's scratch and drops the rest here, on the
        // control plane at bind. The compile-time estimate (`banks::effect_bank_resource`) still
        // charges one scratch per bound *slot*, so it now over-states what the plan retains by
        // one scratch per merged slot. Over-stating is the safe direction for a memory ceiling --
        // a plan admitted under the estimate always fits -- and it is left that way deliberately:
        // the estimate is computed before the lowered program exists, and whether a merge is
        // admissible is not knowable until it does.
        let mut scratch = None;
        let mut active: Option<Box<[bool]>> = None;
        let mut stages = Vec::with_capacity(run.len());
        let mut index = 0;
        while index < run.len() {
            let pair = if index + 1 < run.len() {
                match (run[index], run[index + 1]) {
                    (Membership::Builtin(a), Membership::Builtin(b)) => {
                        let left = self.builtin_banks[a].as_ref().expect("builtin owner");
                        let right = self.builtin_banks[b].as_ref().expect("builtin owner");
                        let same_tracks = left.members.len() == right.members.len()
                            && left.members.iter().zip(right.members.iter()).all(|(left, right)| {
                                matches!((left, right),
                                    (GraphNodeId::TrackStage { track_id: left_id, stage: TrackStage::PostFader },
                                     GraphNodeId::TrackStage { track_id: right_id, stage: TrackStage::PostMatrix })
                                    if left_id == right_id)
                            });
                        same_tracks
                            && left.backend == right.backend
                            && left.scratch.width() == right.scratch.width()
                            && left.scratch.quantum() == right.scratch.quantum()
                    }
                    _ => false,
                }
            } else {
                false
            };
            if pair {
                let a = match run[index] {
                    Membership::Builtin(i) => self.builtin_banks[i].take().expect("builtin owner"),
                    _ => unreachable!(),
                };
                let b = match run[index + 1] {
                    Membership::Builtin(i) => self.builtin_banks[i].take().expect("builtin owner"),
                    _ => unreachable!(),
                };
                let scratch_a = a.scratch;
                let active_a = trailing_active_mask(members, scratch_a.width());
                let factory = a.processor.pair_factory();
                let (slot_scratch, slot_active, stage) = match factory {
                    Some(factory) => match factory(a.processor, b.processor) {
                        Ok(processor) => (
                            scratch_a,
                            active_a,
                            Box::new(BuiltinStage(processor)) as Box<dyn BankStage>,
                        ),
                        Err((left, right)) => {
                            if scratch.is_none() {
                                scratch = Some(scratch_a);
                                active = Some(active_a);
                            }
                            stages.push(Box::new(BuiltinStage(left)) as Box<dyn BankStage>);
                            stages.push(Box::new(BuiltinStage(right)) as Box<dyn BankStage>);
                            index += 2;
                            continue;
                        }
                    },
                    None => {
                        if scratch.is_none() {
                            scratch = Some(scratch_a);
                            active = Some(active_a);
                        }
                        stages.push(Box::new(BuiltinStage(a.processor)) as Box<dyn BankStage>);
                        stages.push(Box::new(BuiltinStage(b.processor)) as Box<dyn BankStage>);
                        index += 2;
                        continue;
                    }
                };
                if scratch.is_none() {
                    scratch = Some(slot_scratch);
                    active = Some(slot_active);
                }
                stages.push(stage);
                index += 2;
                continue;
            }
            let (slot_scratch, slot_active, stage) = self.stage_for(run[index], members);
            if scratch.is_none() {
                scratch = Some(slot_scratch);
                active = Some(slot_active);
            }
            stages.push(stage);
            index += 1;
        }
        bank_chain(
            scratch.expect("a unit has at least one slot"),
            active.expect("a unit has at least one slot"),
            stages,
        )
    }

    fn stage_for(
        &mut self,
        membership: Membership,
        members: usize,
    ) -> (AoSoaScratch, Box<[bool]>, Box<dyn BankStage>) {
        match membership {
            Membership::Effect(index) => {
                let bank = self.banks[index].take().expect("one effect bank owner");
                let width = bank.scratch.width();
                let quantum = bank.scratch.quantum();
                // Every lane's channel is moved out of its own `GraphPreparedEffect` in lane
                // order, so the slot has exactly one drainer per lane and a lane the console does
                // not address stays `None` -- the bank's own `&[]` for that lane's offsets.
                let controls: Vec<Option<EffectControlLane>> = (0..width.lanes() as usize)
                    .map(|lane| {
                        bank.members
                            .get(lane)
                            .and_then(|member| self.effect_controls.remove(member))
                            .map(|control| *control)
                    })
                    .collect();
                // Issue #143: one observation lane per bank lane, moved out in the same lane order
                // so a slot has exactly one owner per lane and a lane nobody observes stays `None`.
                let observations: Vec<Option<ObservationLane>> = (0..width.lanes() as usize)
                    .map(|lane| {
                        bank.members
                            .get(lane)
                            .and_then(|member| self.effect_observations.remove(member))
                            .map(|observation| *observation)
                    })
                    .collect();
                if controls.iter().any(Option::is_some) {
                    let latency = usize::try_from(bank.processor.metadata().program_key.latency.0)
                        .unwrap_or(usize::MAX);
                    let stage = ConsoleEffectBankStage::new(
                        bank.processor,
                        width,
                        quantum,
                        controls,
                        observations,
                        latency,
                    )
                    .expect("validated width");
                    return (bank.scratch, bank.active_mask, Box::new(stage));
                }
                let stage =
                    EffectBankStage::new(bank.processor, width, quantum).expect("validated width");
                (bank.scratch, bank.active_mask, Box::new(stage))
            }
            Membership::Builtin(index) => {
                let bank = self.builtin_banks[index]
                    .take()
                    .expect("one builtin bank owner");
                let active = trailing_active_mask(members, bank.scratch.width());
                (bank.scratch, active, Box::new(BuiltinStage(bank.processor)))
            }
        }
    }
}

/// Which nodes alias each op's output buffer, in schedule order (`program::Tap`).
fn taps_by_op(program: &ExecutionProgram, spec: &GraphSpec) -> BTreeMap<u32, Vec<GraphNodeId>> {
    let mut by_op: BTreeMap<u32, Vec<GraphNodeId>> = BTreeMap::new();
    for tap in &program.taps {
        by_op
            .entry(tap.after_op)
            .or_default()
            .push(spec.nodes[tap.node as usize].id.clone());
    }
    by_op
}

/// The buffer-identity half of serialized scalar fader/matrix admission.
///
/// The composite receives one in-place block at the fader slot. It can preserve the later matrix
/// op only when that op's reduction was already a self-copy: one undelayed input, the same input
/// and output buffer as the fader, and the lowering's own `in_place` witness.
fn scalar_pair_is_in_place(program: &ExecutionProgram, fader: usize, matrix: usize) -> bool {
    let fader = &program.ops[fader];
    let matrix = &program.ops[matrix];
    let inputs = program.inputs_of(matrix);
    matches!(inputs, [input] if input.delay.is_none()
        && input.buffer == fader.output
        && matrix.output == fader.output
        && matrix.in_place)
}

/// Groups the program's ops into units: a bank's members become one unit at the first member's
/// position, which the level-major schedule proves is after every member's producers (#98 F1).
///
/// Returns, for each unit, the ops it owns and the bank membership it belongs to.
pub(crate) fn units_of(
    program: &ExecutionProgram,
    membership_of: &BankMembership,
) -> Vec<PlannedUnit> {
    let mut units: Vec<PlannedUnit> = Vec::with_capacity(program.ops.len());
    let mut emitted: BTreeMap<usize, usize> = BTreeMap::new();
    let mut op_of_node: BTreeMap<u32, usize> = BTreeMap::new();
    for (index, op) in program.ops.iter().enumerate() {
        op_of_node.insert(op.node, index);
    }
    for (index, op) in program.ops.iter().enumerate() {
        match membership_of.get(&op.node) {
            None => units.push((None, vec![index])),
            Some((membership, _)) => {
                let key = match membership {
                    Membership::Effect(bank) => *bank,
                    Membership::Builtin(bank) => bank + program.ops.len(),
                };
                if emitted.contains_key(&key) {
                    continue;
                }
                emitted.insert(key, units.len());
                let mut members: Vec<(usize, usize)> = program
                    .ops
                    .iter()
                    .enumerate()
                    .filter_map(|(other, candidate)| {
                        membership_of.get(&candidate.node).and_then(
                            |(other_membership, position)| {
                                (*other_membership == *membership).then_some((*position, other))
                            },
                        )
                    })
                    .collect();
                members.sort_unstable();
                units.push((
                    Some(*membership),
                    members.into_iter().map(|(_, op)| op).collect(),
                ));
            }
        }
    }
    units
}

/// Builds the sequential executor's runtime: one coloured arena, producers read in place.
pub(crate) fn build_sequential(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: RuntimeParts,
    frames: usize,
) -> Runtime {
    let mut parts = parts;
    // The arena reserves buffer 0 as the always-zero silence slot, so a coloured buffer `b` is
    // arena buffer `b + ARENA_BASE`.
    let arena = |buffer: u32| buffer + ARENA_BASE;
    let taps = taps_by_op(program, spec);
    let grouped = units_of(program, &parts.membership.clone());
    let delays = program
        .delays
        .iter()
        .map(|line| CompensationDelay::new(line.samples as usize))
        .collect();
    // Issue #181: consecutive slots of one cohort chain become one unit with one chain, so the
    // pair pays one planar/AoSoA round-trip per block where it used to pay two.
    let runs = cohort_runs(program, spec, &parts, &grouped);
    let run_units: Vec<(Vec<Membership>, Vec<usize>)> = runs
        .iter()
        .map(|run| {
            (
                run.iter().filter_map(|index| grouped[*index].0).collect(),
                run.iter()
                    .flat_map(|index| grouped[*index].1.iter().copied())
                    .collect(),
            )
        })
        .collect();
    // Issue #218: decided here, before `build_op` consumes `parts.observers` and before any op is
    // built, because it decides which ops are built at all.
    let fold = route_fold(program, spec, &parts, &run_units);
    let folded_runs: std::collections::BTreeSet<usize> = fold
        .as_ref()
        .map(|fold| fold.runs.iter().map(|(run, _)| *run).collect())
        .unwrap_or_default();
    let retired: std::collections::BTreeSet<usize> = fold
        .as_ref()
        .map_or_else(Default::default, |fold| fold.retired.clone());
    // Issue #202 rec 3: decided here, before `build_op` consumes `parts.observers`, because two of
    // the clauses are about what is bound to a node rather than about the program.
    //
    // A folded chain is excluded: the redirect points a lane's scatter at its consumer's buffer,
    // and a folded lane has no scatter to point anywhere -- its tile goes to the epilogue and its
    // consumer no longer runs. Excluding it keeps the two counters honest as well as the code:
    // `bank_scatter_redirects` reports the lanes that still relocate a scatter, not the lanes the
    // fold made the question moot for.
    let redirects: Vec<ScatterRedirect> = scatter_redirects(
        program,
        spec,
        &parts.membership,
        &parts.observers,
        &run_units,
    )
    .into_iter()
    .filter(|(run, _, _)| !folded_runs.contains(run))
    .collect();

    // Serialized scalar fader/matrix pairing is decided while both original owners and the
    // lowered graph are still available.  The schedule is intentionally left untouched: the
    // matrix binding becomes an identity at its original slot, while the composite runs from the
    // fader slot and the existing reduction/observer boundaries remain in place.
    let (readers, first_producer) = op_dataflow(program);
    for pair in run_units.windows(2) {
        let (first_membership, first_ops) = &pair[0];
        let (second_membership, second_ops) = &pair[1];
        if !first_membership.is_empty()
            || !second_membership.is_empty()
            || first_ops.len() != 1
            || second_ops.len() != 1
        {
            continue;
        }
        let first = first_ops[0];
        let second = second_ops[0];
        if second != first.saturating_add(1)
            || program.inputs_of(&program.ops[first]).is_empty()
            || !scalar_pair_is_in_place(program, first, second)
        {
            continue;
        }
        let first_node = &spec.nodes[program.ops[first].node as usize].id;
        let second_node = &spec.nodes[program.ops[second].node as usize].id;
        let (
            GraphNodeId::TrackStage {
                track_id: first_track,
                stage: TrackStage::PostFader,
            },
            GraphNodeId::TrackStage {
                track_id: second_track,
                stage: TrackStage::PostMatrix,
            },
        ) = (first_node, second_node)
        else {
            continue;
        };
        if first_track != second_track
            || !chains_into(
                program,
                spec,
                &parts,
                &readers,
                &first_producer,
                &[first],
                &[second],
            )
        {
            continue;
        }
        let Some(Some(fader)) = parts.bindings.remove(first_node) else {
            continue;
        };
        let Some(Some(matrix)) = parts.bindings.remove(second_node) else {
            parts.bindings.insert(first_node.clone(), Some(fader));
            continue;
        };
        let Some(factory) = fader.scalar_pair_factory() else {
            parts.bindings.insert(first_node.clone(), Some(fader));
            parts.bindings.insert(second_node.clone(), Some(matrix));
            continue;
        };
        match factory(fader, matrix) {
            Ok(composite) => {
                parts.bindings.insert(first_node.clone(), Some(composite));
                parts.bindings.insert(second_node.clone(), None);
            }
            Err((fader, matrix)) => {
                parts.bindings.insert(first_node.clone(), Some(fader));
                parts.bindings.insert(second_node.clone(), Some(matrix));
            }
        }
    }
    // Where each op's `RuntimeOp` ended up, so a redirect can neutralise the consumer's reduction.
    let mut op_slot: Vec<Option<(usize, usize)>> = vec![None; program.ops.len()];
    // Run unit -> the unit index it was emitted at, for the chains the fold arms.
    let mut unit_of_run: Vec<Option<usize>> = vec![None; run_units.len()];
    let mut units = Vec::with_capacity(run_units.len());
    // The bind-time half of the collapse-eligibility query, one row per emitted unit. Built here
    // rather than by a later walk because this is the only place the unit's ops and the spec's
    // node ids are both in hand: `RuntimeOp` deliberately carries no node id, and reconstructing
    // one from the arena buffers afterwards would be a second opinion about which lane is which.
    let mut identity: Vec<UnitIdentity> = Vec::with_capacity(run_units.len());
    for (run, (membership, ops)) in run_units.iter().enumerate() {
        // A retired route op is absorbed by its cohort's epilogue: no unit, no dispatch, no
        // reduction, no `mix2x2_block` pass of its own.
        if ops.iter().all(|index| retired.contains(index)) {
            continue;
        }
        let membership = membership.clone();
        unit_of_run[run] = Some(units.len());
        {
            // `ops` is slot major with `lanes` ops per slot (`units_of` sorts by the member's
            // position within its bank), so slot `s`'s lane `l` is `ops[s * lanes + l]` and lane
            // `l`'s track is read off the first slot. A plain op is one stage over one "lane".
            let stages = membership.len().max(1);
            let lanes = ops.len() / stages;
            let node_of = |index: usize| &spec.nodes[program.ops[index].node as usize].id;
            identity.push(UnitIdentity {
                banked: !membership.is_empty(),
                stages: u32::try_from(stages).unwrap_or(u32::MAX),
                upstream_of_seam_stages: u32::try_from(
                    (0..stages)
                        .filter(|slot| upstream_of_seam(node_of(ops[slot * lanes])))
                        .count(),
                )
                .unwrap_or(u32::MAX),
                lane_tracks: (0..lanes)
                    .map(|lane| node_track(node_of(ops[lane])))
                    .collect(),
            });
        }
        for (member, index) in ops.iter().enumerate() {
            op_slot[*index] = Some((units.len(), member));
        }
        let members: Vec<RuntimeOp> = ops
            .iter()
            .map(|index| {
                let op = &program.ops[*index];
                let mut inputs = Vec::with_capacity(op.input_count() as usize);
                let mut staged = Vec::new();
                for input in program.inputs_of(op) {
                    match input.delay {
                        None => inputs.push(arena(input.buffer.0)),
                        Some(delay) => {
                            inputs.push(arena(delay.staging.0));
                            staged.push(StagedInput {
                                source: arena(input.buffer.0),
                                staging: arena(delay.staging.0),
                                line: delay.line,
                            });
                        }
                    }
                }
                let sidechain = op.sidechain.map(|side| match side.delay {
                    None => arena(side.buffer.0),
                    Some(delay) => {
                        staged.push(StagedInput {
                            source: arena(side.buffer.0),
                            staging: arena(delay.staging.0),
                            line: delay.line,
                        });
                        arena(delay.staging.0)
                    }
                });
                build_op(
                    &mut parts,
                    spec,
                    op,
                    inputs,
                    staged,
                    sidechain,
                    arena(op.output.0),
                    taps.get(&u32::try_from(*index).expect("op index"))
                        .map(Vec::as_slice),
                )
            })
            .collect();
        units.push(finish_unit(&mut parts, &membership, members));
    }
    apply_scatter_redirects(program, &redirects, &op_slot, &mut units);
    let folds = apply_route_fold(fold.as_ref(), &unit_of_run, &op_slot, arena, &mut units);
    let mut builder = ArenaLeaseSetBuilder::new(
        NonZeroUsize::new(2).expect("stereo planes"),
        NonZeroUsize::new(frames.max(1)).expect("nonzero frames"),
    );
    let buffers: Vec<u32> = (0..program.buffers).map(|_| builder.reserve()).collect();
    builder.lease(0, buffers.clone(), buffers);
    let (_arena, mut leases) = builder
        .finish()
        .expect("one lease over one coloured arena is disjoint by construction");
    Runtime::new(
        leases.pop().expect("the sequential lease"),
        delays,
        // Allocated by `node_kind` as it lowered each delayed input node, so the line indices the
        // ops carry and this vector's order are the same walk.
        core::mem::take(&mut parts.track_delay_lines),
        units,
        identity,
        redirects.len() as u64,
        folds,
    )
}

/// Arm every admitted chain's epilogue and neutralise the reduction it performed.
///
/// Three edits, and they are each other's counterparts. The chain is told which lanes to hand to
/// `fold_plane` and given the master buffer and the per-lane constants; the retired route ops were
/// never built into units at all; and the master op's inputs become its own output, which is the
/// shape `reduce_plane` already treats as "nothing to copy", so the op still runs -- with whatever
/// kind it has, a host binding included -- over the sum its cohorts' epilogues already wrote.
///
/// Returns the number of lanes armed, which is the only honest way to state that the fold fired:
/// like the scatter redirect it optimises by *not doing* something, so there is no output
/// difference to observe and no timing difference a gate may rest on.
fn apply_route_fold(
    fold: Option<&RouteFold>,
    unit_of_run: &[Option<usize>],
    op_slot: &[Option<(usize, usize)>],
    arena: impl Fn(u32) -> u32,
    units: &mut [RuntimeUnit],
) -> u64 {
    let Some(fold) = fold else {
        return 0;
    };
    let master = arena(fold.master.0);
    let mut armed = 0_u64;
    for (run, lanes) in &fold.runs {
        let Some(unit) = unit_of_run[*run] else {
            continue;
        };
        let RuntimeUnit::Bank {
            chain,
            fold: slot,
            master: destination,
            ..
        } = &mut units[unit]
        else {
            // Unreachable by construction: `route_fold` only ever names a banked run unit.
            debug_assert!(false, "only a bank chain carries a folded epilogue");
            continue;
        };
        let width = chain.width().lanes() as usize;
        let mut mask = vec![false; width].into_boxed_slice();
        for lane in 0..lanes.len().min(width) {
            mask[lane] = true;
        }
        if chain.arm_fold(mask).is_err() {
            // Unreachable by construction: the mask is the chain's own rendered lanes, which are
            // exactly its active ones. Left inert rather than half-armed.
            debug_assert!(false, "a chain's rendered lanes are its active lanes");
            continue;
        }
        armed += lanes.len() as u64;
        *destination = master;
        *slot = lanes.clone().into_boxed_slice();
    }
    if armed == 0 {
        return 0;
    }
    if let Some((unit, _)) = op_slot[fold.master_op] {
        match &mut units[unit] {
            RuntimeUnit::Op(op) => op.inputs = vec![master].into_boxed_slice(),
            // Unreachable by construction: `route_fold` declines a banked master outright, because
            // such a master's reduction is its chain's gather.
            RuntimeUnit::Bank { .. } => debug_assert!(false, "a banked master never folds"),
        }
    }
    armed
}

/// One chain's scatter redirect: `(run, lane, consumer op)` for every lane whose scatter may land
/// in its consumer's buffer instead of the last slot's own (issue #202 rec 3).
type ScatterRedirect = (usize, usize, usize);

/// The buffers a chain's scatter may land in **instead of** the last slot's own outputs.
///
/// The mirror image of [`bank_gather_source`], on the other side of the round trip. The gather
/// side removed the copy a *member's* reduction made into a dedicated bank buffer; this removes
/// the copy the *consumer's* reduction makes out of one. On the 64-track intended strip that copy
/// is `reduce_plane` memcpying a whole stereo block from the limiter's dedicated buffer into the
/// fader op, once per track per block -- 64 stereo block copies that exist only because
/// `program::is_dedicated` refuses the in-place fold (`program::lower`, the `in_place` clause).
///
/// The redirect makes the consumer behave exactly as an `in_place` op would: the chain scatters
/// straight into the consumer's buffer, and the consumer's reduction becomes the no-op
/// `reduce_plane` already performs for a single input that is its own output. Nothing else about
/// the consumer changes -- it still runs, in its own unit, at its own position.
///
/// This does **not** touch `program::is_dedicated`. Dedication *by bank membership* was measured
/// and rejected (`program::lower` records why, #169), and this is not that: `is_dedicated` stays
/// exactly the classification by node kind it has always been, and what moves is where one chain
/// scatters. It is likewise not the #194 "scatter straight into the planes" null, which was about
/// staging the transpose itself and not about which buffer the scatter targets.
///
/// `None` on any doubt. Every clause is one way the redirect could be observed:
///
/// * **Sole readership.** The last slot's output must have exactly one reader, and that reader is
///   the consumer. A second reader -- a send, a meter, a sidechain source, all of which
///   `op_dataflow` counts -- would read a buffer the scatter no longer fills.
/// * **The consumer's reduction is a pure copy.** One main input, no sidechain, no
///   compensation-delay staging, and that input is the last slot's output. Two inputs is a sum and
///   zero is a `fill(0.0)`; either way the consumer's buffer is not simply the chain's output, and
///   neutralising its reduction would drop a summand. A delayed input owns a line that must still
///   be pumped.
/// * **Not already in place.** `consumer.output == producer.output` is the lowering having elided
///   the copy already, and redirecting the scatter would change nothing.
/// * **The consumer is not a bank member.** A banked consumer's own gather may already read the
///   producer's buffer directly ([`bank_gather_source`]), and redirecting the scatter away from it
///   would hand that gather the previous block's words. The two redirects are each other's only
///   incompatibility, so this is where they are kept apart.
/// * **Not the session output.** The host copies the session output out of its buffer after the
///   last unit; leaving it stale would silence the render.
/// * **No observer, and no observed alias, on the producer.** After the redirect the last slot's
///   own buffer is never written, so anything that reads it reads the previous block. That is the
///   same pair of clauses `chains_into` carries, keyed the same way -- `parts.observers` is keyed
///   by node, so the alias check names the elided stage node and not the producing one.
/// * **Nothing between the scatter and the consumer names the consumer's buffer.** This is the one
///   clause with no counterpart on the gather side, and it is the load-bearing one. The scatter now
///   writes the consumer's buffer at the *chain's* position, which is earlier -- often much
///   earlier -- than the consumer's own op, and the colouring only owns that physical slot from the
///   consumer's op onwards. Whatever held the slot before then is still live for the ops in
///   between. So every op in `[chain's first op, consumer's op)` that is not part of the chain is
///   checked, and any mention of the buffer -- as an output, an input, a sidechain or a staging
///   slot -- declines. The chain's own ops are excluded because they all run *before* the scatter:
///   the first slot's reductions are the gather, and no later slot's op is executed at all.
///
///   Ops outside that range cannot be a hazard. One before the chain's first op belongs to a unit
///   emitted at or before that op, so it has already run. One at or after the consumer's op that
///   names the buffer is a legitimate reader of the value the consumer produces, and a bank that
///   hoisted it past the consumer would be the #169 defect the bank window already forbids.
fn scatter_redirects(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    bank_membership: &BankMembership,
    observers: &BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
    run_units: &[(Vec<Membership>, Vec<usize>)],
) -> Vec<ScatterRedirect> {
    let (readers, first_producer) = op_dataflow(program);
    let mut redirects = Vec::new();
    for (run, (membership, ops)) in run_units.iter().enumerate() {
        if membership.is_empty() || ops.is_empty() {
            continue;
        }
        let lanes = ops.len() / membership.len();
        let last = ops.len() - lanes;
        let Some(first_op) = ops.iter().min().copied() else {
            continue;
        };
        let mut chain: Vec<(usize, Option<usize>)> = Vec::with_capacity(lanes);
        for lane in 0..lanes {
            let producer = ops[last + lane];
            chain.push((
                producer,
                scatter_target(
                    program,
                    spec,
                    bank_membership,
                    observers,
                    &readers,
                    &first_producer,
                    ops,
                    first_op,
                    producer,
                ),
            ));
        }
        // A chain scatters every lane in one pass, so its targets have to stay pairwise distinct.
        // They can stop being so: a consumer's buffer is coloured after its own producer's is
        // released, so one lane's consumer may have been handed the *physical slot* another lane's
        // last slot still scatters into. Two lanes would then write one buffer in the same pass.
        // The colouring is what decides this and it is not knowable lane by lane, so it is checked
        // over the whole chain, and a collision declines every redirect in it rather than picking a
        // winner.
        let scattered: Vec<crate::program::BufferRef> = chain
            .iter()
            .map(|(producer, target)| program.ops[target.unwrap_or(*producer)].output)
            .collect();
        let distinct: std::collections::BTreeSet<_> = scattered.iter().collect();
        if distinct.len() != scattered.len() {
            continue;
        }
        for (lane, (_, target)) in chain.into_iter().enumerate() {
            if let Some(consumer) = target {
                redirects.push((run, last + lane, consumer));
            }
        }
    }
    redirects
}

/// # Which clauses a mutation makes red (the house ledger, measured)
///
/// Dropping a clause and running the graph and graph-compiler suites gives:
///
/// * **sole readership** -> `a_send_from_the_last_slots_alias_declines_that_lanes_scatter_redirect`
/// * **one main input** -> `id_ordered_bank_plan_rejects_transactionally_and_returned_ownership_\
///   is_reusable`
/// * **the consumer is not a bank member** -> `misaligned_lane_sets_decline_the_merge` and two
///   others
/// * **no observed alias** ->
///   `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect`
/// * **no observer on the producer** ->
///   `a_meter_on_a_bank_member_declines_that_lanes_scatter_redirect`
/// * **nothing in between names the buffer** -> seven session-level tests, including the fixture's
///   own `the_intended_strip_fuses_the_whole_signal_path_into_one_chain_per_cohort`, *and* --
///   for the scan's **end boundary**, which none of those seven pin --
///   `cohort_chain_merging_preserves_dataflow_on_random_graphs` through
///   [`scatter_redirects_over_program`]. Shortening the scan by a single op reddens it at graph 24.
/// * **no compensation delay on the consumer's input** -> the same differential eval, at graph 0.
///   No *compiled session* reaches a delayed consumer of a chain's last slot -- PDC is inserted on
///   route edges between tracks, never between a rack slot and the fader it feeds -- so this clause
///   has no session-level red test and never will. The corpus builds the case 38,725 times, so the
///   eval is where it is defended.
///
/// Four clauses have **no** red test, and each is kept for a stated reason rather than a measured
/// one. Saying so is the point of writing the ledger down:
///
/// * **no sidechain on the consumer** is conservative and nothing more. A sidechained consumer's
///   *reduction* is still a pure copy and the redirect does not touch which buffer its
///   `write_read_stereo` names, so no hazard is known here. It is kept because
///   [`bank_gather_source`] carries the same clause and a reader comparing the two should not have
///   to work out why one side omits it.
/// * **`first_producer` agrees** is redundant given the two above it: if the producer's sole reader
///   is this consumer and the consumer has exactly one main input, that input is the producer's
///   buffer. It re-derives the fact from the colouring rather than inferring it.
/// * **not already in place** is an early-out: if the consumer already writes the producer's
///   buffer, the redirect's target *is* that buffer and nothing changes.
/// * **not the session output** is subsumed by the clause above wherever it can fire -- an output
///   node folded onto its producer in place satisfies both -- and is kept because "the host reads
///   this buffer after the last unit" is the thing being defended, not "the output op is in place".
///
/// **Pairwise-distinct scatter targets is defensive by construction, not merely unreached.** The
/// adversarial verification of issue #202 closed this one: for a bank that satisfies the contract
/// the collision cannot arise at all. A chain's last slot is `program::is_dedicated` storage, and a
/// dedicated buffer is never returned to the free list, so no consumer can ever be coloured onto a
/// slot another lane is still scattering into; and where two lanes' consumers *are* ordered such
/// that one could be, the earlier lane has already declined on another clause. The guard is
/// therefore a construction check rather than a hazard defence -- it costs one `BTreeSet` per chain
/// at bind and it is what makes "a chain scatters every lane in one pass" a checked fact rather
/// than an inherited assumption. It is deliberately **not** presented as a measured clause.
///
/// The consumer's *own* observers are deliberately **not** a clause. The redirect changes nothing
/// about what the consumer's buffer holds by the time its observers run, so there is nothing there
/// for one to see.
///
/// The consumer whose buffer one lane's scatter may land in, or `None`. See [`scatter_redirects`]
/// for what each clause is defending.
#[allow(clippy::too_many_arguments)]
fn scatter_target(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    membership: &BankMembership,
    observers: &BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    run: &[usize],
    first_op: usize,
    producer: usize,
) -> Option<usize> {
    if readers[producer].len() != 1 {
        return None;
    }
    let consumer = readers[producer][0];
    let producer_op = &program.ops[producer];
    let consumer_op = &program.ops[consumer];
    if consumer_op.input_count() != 1
        || consumer_op.sidechain.is_some()
        || program.inputs_of(consumer_op)[0].delay.is_some()
        || first_producer[consumer] != Some(producer)
    {
        return None;
    }
    if consumer_op.output == producer_op.output || producer_op.output == program.output {
        return None;
    }
    if membership.contains_key(&consumer_op.node) {
        return None;
    }
    let node = &spec.nodes[producer_op.node as usize].id;
    if observers.contains_key(node) {
        return None;
    }
    if program
        .taps
        .iter()
        .filter(|tap| tap.after_op as usize == producer)
        .any(|tap| observers.contains_key(&spec.nodes[tap.node as usize].id))
    {
        return None;
    }
    let target = consumer_op.output;
    for (index, op) in program.ops.iter().enumerate().take(consumer).skip(first_op) {
        if run.contains(&index) {
            continue;
        }
        if op_names_buffer(program, op, target) {
            return None;
        }
    }
    Some(consumer)
}

/// Whether `op` mentions `buffer` at all: as its output, a main input, a sidechain, or the staging
/// slot of either.
fn op_names_buffer(program: &ExecutionProgram, op: &Op, buffer: crate::program::BufferRef) -> bool {
    let staging_or_buffer = |input: &crate::program::InputRef| {
        input.buffer == buffer || input.delay.is_some_and(|delay| delay.staging == buffer)
    };
    op.output == buffer
        || program.inputs_of(op).iter().any(staging_or_buffer)
        || op.sidechain.as_ref().is_some_and(staging_or_buffer)
}

/// The **runtime's own** scatter-redirect decision, driven over a program that has no bindings.
///
/// `program::tests::cohort_chain_merging_preserves_dataflow_on_random_graphs` interprets the
/// executor through a *model* of [`scatter_target`]'s clauses. A model is an oracle only while it
/// and the thing it models agree, and the model cannot check that by itself: the adversarial
/// verification of issue #202 found that shortening [`scatter_target`]'s in-between scan by a
/// single op -- `take(consumer)` to `take(consumer - 1)`, which is the unsound direction -- reddens
/// nothing anywhere, while the same one-token change to the model reddens the corpus at once. The
/// corpus was building the hazard at exactly that boundary and then only ever asking the model
/// about it.
///
/// This is the missing half. The corpus now drives *this* function, and the two answers must be
/// equal graph for graph, so every clause the corpus exercises has the corpus as its red test on
/// both sides of the model/runtime pair -- the same correspondence #194 established on the gather
/// side.
///
/// Returns `producer op -> consumer op` for every admitted lane. A program-level fixture binds no
/// observers, which is why the two observer clauses take an empty map here: they are covered by
/// the session-level tests in the graph compiler, and this eval covers the rest.
#[cfg(test)]
pub(crate) fn scatter_redirects_over_program(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    lanes: &BTreeMap<u32, (usize, usize)>,
    runs: &[Vec<Vec<usize>>],
) -> BTreeMap<usize, usize> {
    let bank_membership: BankMembership = lanes
        .iter()
        .map(|(node, (bank, lane))| (*node, (Membership::Effect(*bank), *lane)))
        .collect();
    let observers = BTreeMap::new();
    // `build_sequential` builds this shape from `units_of`; here it is built from the model's runs,
    // so the only thing that differs between the two sides is which copy of the clauses answers.
    // A plain unit carries an empty membership list, which is what marks it as not a bank.
    let run_units: Vec<(Vec<Membership>, Vec<usize>)> = runs
        .iter()
        .map(|run| {
            let last = run.last().expect("a run has at least one slot");
            let banked = last.len() > 1 || lanes.contains_key(&program.ops[last[0]].node);
            let membership = if banked {
                vec![Membership::Effect(0); run.len()]
            } else {
                Vec::new()
            };
            (membership, run.iter().flatten().copied().collect())
        })
        .collect();
    scatter_redirects(program, spec, &bank_membership, &observers, &run_units)
        .into_iter()
        .map(|(run, member, consumer)| (run_units[run].1[member], consumer))
        .collect()
}

/// Point each admitted lane's scatter at its consumer's buffer and neutralise the consumer's
/// reduction.
///
/// Two edits, and they are each other's counterpart: the member's `output` is what
/// `Runtime::execute` reads to build the chain's scatter list, and the consumer's single input
/// becomes its own output, which is the shape `reduce_plane` already treats as "nothing to copy".
/// The consumer therefore runs exactly as an `in_place` op does, which is what it would have been
/// had its producer not been dedicated storage.
fn apply_scatter_redirects(
    program: &ExecutionProgram,
    redirects: &[ScatterRedirect],
    op_slot: &[Option<(usize, usize)>],
    units: &mut [RuntimeUnit],
) {
    for (run, member, consumer) in redirects.iter().copied() {
        let target = ARENA_BASE + program.ops[consumer].output.0;
        let RuntimeUnit::Bank { members, .. } = &mut units[run] else {
            continue;
        };
        members[member].output = target;
        let Some((unit, _)) = op_slot[consumer] else {
            continue;
        };
        match &mut units[unit] {
            RuntimeUnit::Op(op) => op.inputs = vec![target].into_boxed_slice(),
            // Unreachable by construction: `scatter_target` declines a banked consumer outright,
            // because such a consumer's own gather may already read the producer's buffer
            // (`bank_gather_source`) and the two redirects would then disagree about which buffer
            // holds this block's audio. Left inert rather than applying, so that if that clause is
            // ever loosened this arm does nothing instead of doing the wrong thing.
            RuntimeUnit::Bank { .. } => debug_assert!(false, "a banked consumer never redirects"),
        }
    }
}

/// A route's 2x2 with its linear gain folded in, once, at bind (D3, #98 F4).
///
/// One derivation, two callers: [`RuntimeParts::node_kind`] builds `NodeKind::Route` from it and
/// [`route_fold`] builds `FoldLane` from it, so a chain's epilogue cannot apply constants that
/// differ from the ones the route op it replaced would have applied.
const fn folded_route(transform: &RouteTransform) -> [f32; 4] {
    [
        transform.gain * transform.ll,
        transform.gain * transform.lr,
        transform.gain * transform.rl,
        transform.gain * transform.rr,
    ]
}

/// The route constants `node_kind` *would* hand this node, asked without consuming anything.
///
/// [`RuntimeParts::node_kind`] takes the node's binding, its prepared effect and its route out of
/// `parts` as it answers, so it can be asked exactly once and only while its op is being built.
/// The fold has to know before any op is built -- it decides which ops are built at all -- so this
/// restates the same cascade as a query, in the same precedence order, and returns `None` for
/// every arm that is not a plain route. A node that a host bound, that a bank owns, that a source
/// set fills or that carries a prepared effect is not a route however the session named it.
fn plain_route_gains(parts: &RuntimeParts, node: &GraphNodeId, index: u32) -> Option<[f32; 4]> {
    if parts.source_inputs.contains(node)
        || parts.membership.contains_key(&index)
        || matches!(parts.bindings.get(node), Some(Some(_)))
        || parts.effects.contains_key(node)
    {
        return None;
    }
    parts.routes.get(node).map(folded_route)
}

/// Whether anything can *see* the buffer op `index` writes other than by reading it as an input.
///
/// Two ways, and they are the pair [`chains_into`] and [`scatter_target`] already carry: an
/// observer bound to the producing node, and an observer bound to an elided node whose alias
/// resolves to this op's buffer (`program::Tap`). `parts.observers` is keyed by node, so the second
/// has to name the alias node rather than the producing one.
fn observed(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    index: usize,
) -> bool {
    let node = &spec.nodes[program.ops[index].node as usize].id;
    if parts.observers.contains_key(node) {
        return true;
    }
    program
        .taps
        .iter()
        .filter(|tap| tap.after_op as usize == index)
        .any(|tap| {
            parts
                .observers
                .contains_key(&spec.nodes[tap.node as usize].id)
        })
}

/// One chain's folded epilogue: which run unit it is, and one entry per rendered lane.
type FoldedRun = (usize, Vec<FoldLane>);

/// One candidate chain mid-proof: its run unit, and `(route op, folded 2x2)` for every lane.
type FoldCandidate = (usize, Vec<(usize, [f32; 4])>);

/// What [`route_fold`] admitted: the chains that fold, the route ops that stop running, and the
/// reduction that was replaced.
struct RouteFold {
    /// Folded chains in render order; the order the master is accumulated in.
    runs: Vec<FoldedRun>,
    /// Route ops the epilogues absorbed. These are not built into units at all.
    retired: std::collections::BTreeSet<usize>,
    /// The op whose reduction the epilogues performed. It still runs; its inputs become its own
    /// output, which is the no-op `reduce_plane` already performs for a single input that is its
    /// own output.
    master_op: usize,
    /// The buffer that op writes, in the lowering's numbering.
    master: crate::program::BufferRef,
}

/// The producer of each of op `target`'s main inputs, in `inputs` order.
///
/// The same last-writer walk [`op_dataflow`] does, stopped at `target` and kept *per input
/// position* rather than for position zero alone -- which is exactly the sequence the association
/// proof in [`route_fold`] has to compare against.
fn input_producers(program: &ExecutionProgram, target: usize) -> Vec<Option<usize>> {
    let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
    for (index, op) in program.ops.iter().enumerate() {
        if index == target {
            return program
                .inputs_of(op)
                .iter()
                .map(|input| owner[input.buffer.0 as usize])
                .collect();
        }
        owner[op.output.0 as usize] = Some(index);
    }
    Vec::new()
}

/// One lane's route, if that lane's chain may absorb it: `(route op, folded 2x2)`.
///
/// `producer` is the chain's last slot for this lane. Every clause is one way absorbing the route
/// could be observed; see [`route_fold`] for the ledger.
fn foldable_lane(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    producer: usize,
) -> Option<(usize, [f32; 4])> {
    if readers[producer].len() != 1 {
        return None;
    }
    let route = readers[producer][0];
    let producer_op = &program.ops[producer];
    let route_op = &program.ops[route];
    if route_op.input_count() != 1
        || route_op.sidechain.is_some()
        || program.inputs_of(route_op)[0].delay.is_some()
        || first_producer[route] != Some(producer)
    {
        return None;
    }
    if producer_op.output == program.output || route_op.output == program.output {
        return None;
    }
    if observed(program, spec, parts, producer) || observed(program, spec, parts, route) {
        return None;
    }
    let node = &spec.nodes[route_op.node as usize].id;
    let gains = plain_route_gains(parts, node, route_op.node)?;
    Some((route, gains))
}

/// Fold every cohort chain's routes and the master reduction into the chains' own epilogues.
///
/// The shape this replaces, on the intended 64-track strip, is one route op per track -- a whole
/// stereo block of `mix2x2_block` over a buffer the chain had just scattered -- followed by a
/// 63-pass `sum_into_block` reduction over those 64 buffers. The shape it renders instead is one
/// pass: each lane's tile is routed where the transpose left it and goes straight into the master,
/// the first contributor storing and the rest accumulating.
///
/// # The association proof, which is the whole of the correctness argument
///
/// D9 fixes the reduction as `sum2_block(in0, in1)` then `sum_into_block` left to right over
/// `spec.edges` order, and a floating-point sum is not associative, so "the same summands" is not
/// "the same bits". The epilogues accumulate in **chain execution order**, which is a different
/// sequence written down in a different place. They are only the same reduction if the two
/// sequences are equal, and this function proves that rather than assuming it: it walks the lowered
/// program, resolves the producer of *every* input position of the reduction (`input_producers`),
/// and requires that list to equal, element for element and in order, the route ops of the folded
/// chains taken in render order, lane by lane. Anything else -- a contributor that is not a folded
/// lane, a lane out of place, a cohort whose planner ordered its lanes differently from the edge
/// order -- fails the equality and declines the whole fold. This mirrors how `chains_into` proves
/// lane alignment on the lowered program instead of trusting two planners to agree.
///
/// Equality of the *whole* list is also what makes coverage total: a master with one contributor
/// that is not a folded lane cannot be folded at all, because there is no position in the chains'
/// order at which that contributor's summand could be inserted.
///
/// # Which clauses a mutation makes red (the house ledger, measured)
///
/// Dropping a clause and running the graph, graph-compiler and console-workload suites gives:
///
/// * **the association order** -> `route_ids_ordered_against_the_cohorts_decline_the_route_fold`,
///   plus `a_leased_stage_meter_declines_the_merge_and_still_meters` and
///   `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect`. Keeping the length
///   check and dropping the element-wise comparison is the unsound direction, and it is the one
///   measured.
/// * **no observer on the route/output path** -> `the_folded_master_is_the_reductions_own_bits`.
///   That test's oracle is a post-matrix meter, so dropping this clause destroys the oracle *and*
///   the plan it was oracle for; it goes red either way, which is what the ledger records.
/// * **the opening chain's ops are excluded from the in-between scan** -> every fold in the tree
///   stops firing and `every_standing_workload_folds_one_route_per_track` goes red. That is the
///   conservative direction, and it is worth pinning: the colouring gives the session output the
///   physical slot of track zero's *input* buffer on every console fixture, so without the
///   exclusion the standing fixture never folds at all.
/// * **the first contributor stores** ->
///   `the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign`.
/// * **the whole fold** -> `every_standing_workload_folds_one_route_per_track`, on a count. There
///   is no output difference to see: that is the point of the counter.
///
/// Four clauses have **no** red test, and each is kept for a stated reason rather than a measured
/// one. Saying so is the point of writing the ledger down:
///
/// * **sole readership of a chain's last slot.** Genuinely load-bearing -- a folded lane stops
///   writing that buffer, so a second reader would carry the previous block -- but *shadowed* in
///   every session a compiler can build. A second route from the same tap adds a summand the
///   master's input list has, so the association proof declines on length first; a sidechain from
///   that tap is read by an op scheduled *before* the route, so `readers[producer][0]` is not a
///   route and the plain-route clause declines instead. Dropping the clause reddens nothing, and
///   that is reported rather than dressed up.
/// * **nothing in between names the master.** No compiled session reaches the hazard, and the
///   reason is structural: the master's colour is the first colour the lowering frees, which is
///   track zero's input buffer, and track zero is always in the *opening* cohort -- whose ops the
///   scan excludes because they all precede the first master write. A later cohort naming the
///   master's slot is expressible in a lowered program and not in a session, exactly as
///   `scatter_target`'s compensation-delay clause is.
/// * **one master op for the whole plan.** A session whose tracks reduce into several submixes
///   could fold each submix separately; this folds one reduction or none. The proof would have to
///   be run per master and the chains partitioned between them, and no fixture in the tree needs
///   it. Dropping it is shadowed by the association proof's length check.
/// * **the master buffer is distinct from every folded buffer.** The colouring cannot hand the
///   master a slot a folded lane still writes -- a chain's last slot is `program::is_dedicated`
///   storage and is never returned to the free list -- so this is a construction check, in the
///   same sense as `scatter_redirects`' pairwise-distinct guard, and is deliberately not presented
///   as a hazard defence.
fn route_fold(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    run_units: &[(Vec<Membership>, Vec<usize>)],
) -> Option<RouteFold> {
    let (readers, first_producer) = op_dataflow(program);
    // (run unit, one (route op, folded 2x2) per rendered lane), in render order.
    let mut candidates: Vec<FoldCandidate> = Vec::new();
    for (run, (membership, ops)) in run_units.iter().enumerate() {
        if membership.is_empty() || ops.is_empty() {
            continue;
        }
        let lanes = ops.len() / membership.len();
        let last = ops.len() - lanes;
        let mut folded = Vec::with_capacity(lanes);
        for lane in 0..lanes {
            match foldable_lane(
                program,
                spec,
                parts,
                &readers,
                &first_producer,
                ops[last + lane],
            ) {
                Some(lane_fold) => folded.push(lane_fold),
                // A chain folds every lane or none: a half-folded chain would have to keep its
                // scatter for the rest, and its unfolded lanes' routes would then have to be
                // inserted into the master's order somewhere the chains' order has no room for.
                None => {
                    folded.clear();
                    break;
                }
            }
        }
        if folded.is_empty() {
            continue;
        }
        candidates.push((run, folded));
    }
    let first_route = candidates.first()?.1.first()?.0;
    if readers[first_route].len() != 1 {
        return None;
    }
    let master_op = readers[first_route][0];
    // Every candidate must reduce into that one master, and only into it.
    candidates.retain(|(_, lanes)| {
        lanes
            .iter()
            .all(|(route, _)| readers[*route].len() == 1 && readers[*route][0] == master_op)
    });
    if candidates.is_empty() {
        return None;
    }
    let master = &program.ops[master_op];
    if master.sidechain.is_some()
        || program
            .inputs_of(master)
            .iter()
            .any(|input| input.delay.is_some())
        || parts.membership.contains_key(&master.node)
        || parts
            .source_inputs
            .contains(&spec.nodes[master.node as usize].id)
    {
        return None;
    }
    // The association proof: the reduction's contributors, in its own edge order, are exactly the
    // folded lanes in render order.
    let ordered: Vec<usize> = candidates
        .iter()
        .flat_map(|(_, lanes)| lanes.iter().map(|(route, _)| *route))
        .collect();
    let producers = input_producers(program, master_op);
    if producers.len() != ordered.len()
        || producers
            .iter()
            .zip(ordered.iter())
            .any(|(producer, route)| *producer != Some(*route))
    {
        return None;
    }
    let target = master.output;
    // A folded lane's own buffers stop being written, so the master must not be one of them.
    if candidates.iter().any(|(_, lanes)| {
        lanes
            .iter()
            .any(|(route, _)| program.ops[*route].output == target)
    }) {
        return None;
    }
    // The epilogues write the master at each chain's position, which is earlier -- often much
    // earlier -- than the reduction they replaced. Everything scheduled in between must therefore
    // leave the buffer alone: a reader would see a partial sum, and a writer would clobber one.
    //
    // The window is over *units*, not op indices, because a unit is what runs. Three exclusions,
    // each for a reason:
    //
    // * **the opening folded chain's own unit.** Every op it owns runs before its chain does, and
    //   its chain's scatter is the first write to the master, so nothing it names can be a hazard.
    //   This is load-bearing rather than tidy: the colouring reuses the session output's physical
    //   slot for track zero's *input* buffer on every console fixture in the tree, so the opening
    //   cohort's lane-zero gather reads the master's slot -- before the master exists -- on every
    //   block. Excluding only the retired routes declines the whole fold on the standing fixture.
    // * **every unit at or after the master's.** The master op is the value they are entitled to
    //   read, and the master's own unit holds nothing but the master op (a banked master is
    //   declined above).
    // * **the retired routes**, which no longer run at all.
    //
    // Everything else is checked, chain members included: a later cohort's first-slot reduction
    // runs *after* the opening cohort's scatter, and its gather source is one of the inputs
    // `op_names_buffer` counts, so a cohort whose gather read the master's slot declines here.
    let retired: std::collections::BTreeSet<usize> = ordered.iter().copied().collect();
    let opening = candidates.first().expect("a non-empty candidate list").0;
    let master_run = run_units
        .iter()
        .position(|(_, ops)| ops.contains(&master_op))?;
    if master_run <= opening {
        return None;
    }
    for (run, (_, ops)) in run_units.iter().enumerate() {
        if run <= opening || run >= master_run {
            continue;
        }
        if ops
            .iter()
            .filter(|index| !retired.contains(index))
            .any(|index| op_names_buffer(program, &program.ops[*index], target))
        {
            return None;
        }
    }
    let mut store = true;
    let runs = candidates
        .into_iter()
        .map(|(run, lanes)| {
            let fold = lanes
                .into_iter()
                .map(|(_, coefficients)| {
                    let lane = FoldLane {
                        coefficients,
                        store,
                    };
                    store = false;
                    lane
                })
                .collect();
            (run, fold)
        })
        .collect();
    Some(RouteFold {
        runs,
        retired,
        master_op,
        master: target,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_op(
    parts: &mut RuntimeParts,
    spec: &GraphSpec,
    op: &Op,
    inputs: Vec<u32>,
    staged: Vec<StagedInput>,
    sidechain: Option<u32>,
    output: u32,
    aliases: Option<&[GraphNodeId]>,
) -> RuntimeOp {
    let node = spec.nodes[op.node as usize].id.clone();
    let kind = parts.node_kind(&node, op.node);
    let observers = take_observers(
        &mut parts.observers,
        core::iter::once(node).chain(aliases.unwrap_or(&[]).iter().cloned()),
    );
    RuntimeOp {
        inputs: inputs.into_boxed_slice(),
        staged: staged.into_boxed_slice(),
        sidechain,
        output,
        kind,
        observers,
    }
}

fn finish_unit(
    parts: &mut RuntimeParts,
    run: &[Membership],
    mut members: Vec<RuntimeOp>,
) -> RuntimeUnit {
    if run.is_empty() {
        return RuntimeUnit::Op(members.pop().expect("one op per plain unit"));
    }
    let lanes = members.len() / run.len();
    let chain = parts.chain_for(run, lanes);
    RuntimeUnit::Bank {
        members: members.into_boxed_slice(),
        lanes,
        chain,
        fold: Box::default(),
        master: 0,
    }
}

/// For each op, the ops that read its output, and the op that produced its first main input.
///
/// The lowering gives an op output a *coloured* buffer and colours are reused, so "which ops name
/// this buffer" is not the question -- "which ops name it while this op is still its last writer"
/// is. Walking `program.ops` in schedule order and remembering the last writer of each colour
/// answers exactly that: liveness colouring never reassigns a colour while a consumer still needs
/// it, so the last writer at the moment a consumer is reached is that consumer's producer. This
/// reuses #98/#99's colouring rather than forming a second opinion from the semantic graph.
fn op_dataflow(program: &ExecutionProgram) -> (Vec<Vec<usize>>, Vec<Option<usize>>) {
    let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
    let mut readers: Vec<Vec<usize>> = vec![Vec::new(); program.ops.len()];
    let mut first_producer: Vec<Option<usize>> = vec![None; program.ops.len()];
    for (index, op) in program.ops.iter().enumerate() {
        for (position, input) in program.inputs_of(op).iter().enumerate() {
            if let Some(producer) = owner[input.buffer.0 as usize] {
                readers[producer].push(index);
                if position == 0 {
                    first_producer[index] = Some(producer);
                }
            }
        }
        if let Some(Some(producer)) = op.sidechain.map(|side| owner[side.buffer.0 as usize]) {
            readers[producer].push(index);
        }
        owner[op.output.0 as usize] = Some(index);
    }
    (readers, first_producer)
}

/// `true` when unit `later`'s ops are exactly the lane-wise consumers of unit `earlier`'s, so the
/// two may be rendered as consecutive slots of one chain (issue #181, widened by #202 rec 2).
///
/// The merge replaces two planar/AoSoA round-trips with one: the chain gathers `earlier`'s member
/// outputs, runs both stages over the resident block, and scatters into `later`'s. The price is
/// that `earlier`'s output buffers are left holding the *chain's input* rather than the first
/// stage's output, and `later`'s ops never reduce. Every clause below is one way that could be
/// observed, and any one of them declines the merge:
///
/// * **Lane count and lane order.** The two slots must cover the same lanes, one op each, *in the
///   same order*: the `first_producer` clause below is checked lane by lane, so two banks whose
///   planners disagreed about which track sits in which lane can never fuse. This is the whole of
///   the lane-alignment obligation and it is proved on the lowered program rather than assumed of
///   the planners.
/// * **`later` reads only `earlier`, undelayed and unmixed.** One main input, no sidechain, no
///   compensation-delay staging -- otherwise skipping `later`'s reduction would drop a summand or
///   a delay line. A sidechained slot already blocks banking (#96 F9); this re-checks it on the
///   lowered program rather than trusting the planner.
/// * **Nothing else reads `earlier`.** Exactly one reader, and it is `later`'s op. A send, a
///   second consumer or a sidechain source would read the pre-stage signal; `op_dataflow` counts
///   sidechain reads, so a sidechained consumer of `earlier` is one of these and not an omission.
/// * **No observer, and not the session output.** An observer bound to `earlier` fires after the
///   unit and would see the chain's input; the session output is read by the host.
/// * **No *observed* alias.** A `program::Tap` aliases an elided stage boundary onto `earlier`'s
///   buffer. The alias is a name, not a read -- an edge out of it resolves to `earlier` and is
///   already counted as a second reader above -- so a tap on its own is not a reason to decline.
///   What can read one is an **observer bound to the alias node**, which is how a leased stage
///   meter reaches `PostSimd1`, `PostDynamic` or `PostSimd2PreFader`
///   (`builtins::MeterTap`). `parts.observers` is keyed by *node*, so the check has to
///   name the alias node and not the producing one; keying it on the producer would miss exactly
///   the meter it exists to protect.
///
/// # The perf cliff this last clause buys, stated out loud
///
/// Issue #181 declined on the presence of a tap alone, which is why the intended 64-track strip
/// stopped at the `simd1`/`simd2` boundary: the three elided rack-boundary stages put a tap on the
/// compressor and the limiter never fused. Nothing planar reads those aliases in a session that
/// leases no stage meter, so the refusal was paying for an observer that was not there. It is now
/// paid only when the observer is: **leasing a meter at `PostSimd1`, `PostDynamic` or
/// `PostSimd2PreFader` costs that track's cohort one extra planar/AoSoA round-trip per block**,
/// because its chain can no longer span the stage the meter reads. That is the intended trade --
/// the meter must see post-compressor audio, and a merged chain would hand it the chain's input --
/// and `a_leased_stage_meter_declines_the_merge_and_still_meters` pins both halves of it.
///
/// Effect observation (`ObservationLane`) is *not* such an observer and must not be confused
/// with one: it reads the effect's own resident state through `observe_resident`, never a planar
/// stage buffer, so an armed console lane neither declines the merge nor is disturbed by one.
fn chains_into(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    earlier: &[usize],
    later: &[usize],
) -> bool {
    if earlier.len() != later.len() || earlier.is_empty() {
        return false;
    }
    for (before, after) in earlier.iter().zip(later.iter()) {
        let producer = &program.ops[*before];
        let consumer = &program.ops[*after];
        if consumer.input_count() != 1
            || consumer.sidechain.is_some()
            || program.inputs_of(consumer)[0].delay.is_some()
            || first_producer[*after] != Some(*before)
        {
            return false;
        }
        if readers[*before].len() != 1 || readers[*before][0] != *after {
            return false;
        }
        if producer.output == program.output {
            return false;
        }
        let node = &spec.nodes[producer.node as usize].id;
        if parts.observers.contains_key(node) {
            return false;
        }
        if program
            .taps
            .iter()
            .filter(|tap| tap.after_op as usize == *before)
            .any(|tap| {
                parts
                    .observers
                    .contains_key(&spec.nodes[tap.node as usize].id)
            })
        {
            return false;
        }
    }
    true
}

/// Groups the planned units into cohort runs: consecutive slots of one chain become one unit.
///
/// Returns one entry per rendered unit, in render order, each listing the planned-unit indices it
/// covers. A unit that merges with nothing is a run of one, which is what every unit was before
/// issue #181.
///
/// # Candidacy is the program's dataflow, not the planner's grouping (issue #202 rec 2)
///
/// Issue #181 asked the cohort planner which bound slots came out of one group and offered only
/// those pairs to [`chains_into`]. That is a strictly narrower question than the one the merge
/// actually needs answered, and it left three quarters of the intended strip's round-trips on the
/// table: `plan_bank_groups` pools per `RackLocation`, so no candidate ever crossed a rack
/// boundary, and a builtin bank has no cohort group at all, so the `builtins -> simd1` boundary
/// was not even expressible. On the 64-track intended fixture that is 8 groups x {builtins, simd1,
/// simd2} = 24 chains where 8 will do.
///
/// The candidate successor of a bank unit is therefore taken from the lowered program itself: the
/// unit that owns the op reading lane 0's output. `chains_into` then has to prove the whole
/// lane-wise relation anyway, so nothing is trusted to the planners -- least of all that two banks
/// planned by two different planners agree about which track sits in which lane. Where the merge
/// is admissible the planners' lane orders coincide *because the proof says so*, and where they do
/// not the merge is simply declined.
///
/// Two structural facts make the run construction below well formed:
///
/// * **The successor relation is injective.** `chains_into` requires
///   `first_producer[later[i]] == Some(earlier[i])` for every lane, so two different predecessors
///   would have to share lane 0's op -- that is, be the same unit. No unit is ever appended to two
///   runs.
/// * **A successor is always later in unit order.** Every lane of `later` is scheduled after the
///   matching lane of `earlier`, and a unit is emitted at its members' minimum op index, so the
///   minimum over `later` strictly exceeds the minimum over `earlier`. The runs therefore have no
///   cycles and stay in render order.
///
/// The op range a merged run permutes is held by `program::lower`'s bank window, which forms the
/// same union from the same lane-wise relation (`program::chainable_bank_groups`).
fn cohort_runs(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    units: &[PlannedUnit],
) -> Vec<Vec<usize>> {
    let (readers, first_producer) = op_dataflow(program);
    let mut unit_of_op: Vec<Option<usize>> = vec![None; program.ops.len()];
    for (index, (membership, ops)) in units.iter().enumerate() {
        if membership.is_none() {
            continue;
        }
        for op in ops {
            unit_of_op[*op] = Some(index);
        }
    }
    let mut successor: BTreeMap<usize, usize> = BTreeMap::new();
    for (earlier, (membership, ops)) in units.iter().enumerate() {
        if membership.is_none() {
            continue;
        }
        // The sole reader of lane 0 names the only unit this one can possibly chain into.
        // `chains_into` re-checks sole readership for every lane, so a `first()` here is a lookup
        // and not a decision.
        let Some(later) = ops
            .first()
            .and_then(|lane| readers[*lane].first())
            .and_then(|reader| unit_of_op[*reader])
        else {
            continue;
        };
        if later == earlier {
            continue;
        }
        if chains_into(
            program,
            spec,
            parts,
            &readers,
            &first_producer,
            ops,
            &units[later].1,
        ) {
            successor.insert(earlier, later);
        }
    }
    let merged: std::collections::BTreeSet<usize> = successor.values().copied().collect();
    debug_assert_eq!(
        merged.len(),
        successor.len(),
        "the successor relation is injective, so no unit joins two runs"
    );
    let mut runs = Vec::with_capacity(units.len());
    for index in 0..units.len() {
        if merged.contains(&index) {
            continue;
        }
        let mut run = vec![index];
        let mut cursor = index;
        while let Some(next) = successor.get(&cursor) {
            run.push(*next);
            cursor = *next;
        }
        runs.push(run);
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::{BufferRef, DelayRef, InputRef};
    use core::any::Any;
    use lane::kernels::sum2_block;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn synthetic_distinct_matrix_destination_is_the_scalar_pair_identity_decline() {
        // This is a deliberately synthetic lowered program. It isolates the defensive identity
        // gate; #476 owns the separate question of whether production lowering can emit it.
        let mut program = ExecutionProgram {
            ops: vec![
                Op {
                    node: 0,
                    level: 0,
                    inputs: (0, 1),
                    sidechain: None,
                    output: BufferRef(1),
                    in_place: true,
                },
                Op {
                    node: 1,
                    level: 1,
                    inputs: (1, 2),
                    sidechain: None,
                    output: BufferRef(2),
                    in_place: false,
                },
            ]
            .into_boxed_slice(),
            inputs: vec![
                InputRef {
                    buffer: BufferRef(1),
                    delay: None,
                },
                InputRef {
                    buffer: BufferRef(1),
                    delay: None,
                },
            ]
            .into_boxed_slice(),
            delays: Box::new([]),
            node_buffer: vec![BufferRef(1), BufferRef(2)].into_boxed_slice(),
            node_op: vec![Some(0), Some(1)].into_boxed_slice(),
            taps: Box::new([]),
            buffers: 3,
            output: BufferRef(2),
        };
        assert!(
            !scalar_pair_is_in_place(&program, 0, 1),
            "an otherwise valid single undelayed edge declines on its distinct destination"
        );

        program.ops[1].output = BufferRef(1);
        program.ops[1].in_place = true;
        program.node_buffer[1] = BufferRef(1);
        assert!(
            scalar_pair_is_in_place(&program, 0, 1),
            "the same fixture admits only after the exact identity facts are restored"
        );
        program.inputs[1].delay = Some(DelayRef {
            line: 0,
            staging: BufferRef(2),
        });
        assert!(
            !scalar_pair_is_in_place(&program, 0, 1),
            "the explicit undelayed-input guard remains independent"
        );

        struct FaderOwner(Arc<AtomicUsize>);
        impl GraphRuntimeProcessor for FaderOwner {
            fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                self.0.fetch_add(1, Ordering::Relaxed);
                for sample in block.left.iter_mut().chain(block.right.iter_mut()) {
                    *sample *= 2.0;
                }
                Ok(())
            }
        }
        struct FailingMatrixOwner {
            calls: Arc<AtomicUsize>,
            queued: Arc<AtomicUsize>,
        }
        impl GraphRuntimeProcessor for FailingMatrixOwner {
            fn process(&mut self, _: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                self.calls.fetch_add(1, Ordering::Relaxed);
                self.queued.fetch_sub(1, Ordering::Relaxed);
                Err(RenderError::InvalidEnvelope)
            }
        }

        // Execute the original declined shape. The later reduction copies into its distinct
        // destination before its owner reports the first error; the fader source remains the
        // completed earlier state. This is the boundary a one-buffer early composite cannot own.
        let fader_calls = Arc::new(AtomicUsize::new(0));
        let matrix_calls = Arc::new(AtomicUsize::new(0));
        let queued = Arc::new(AtomicUsize::new(2));
        let mut fader = RuntimeOp {
            inputs: vec![ARENA_BASE].into_boxed_slice(),
            staged: Box::new([]),
            sidechain: None,
            output: ARENA_BASE,
            kind: NodeKind::Bound(Box::new(FaderOwner(Arc::clone(&fader_calls)))),
            observers: Box::new([]),
        };
        let mut matrix = RuntimeOp {
            inputs: vec![ARENA_BASE].into_boxed_slice(),
            staged: Box::new([]),
            sidechain: None,
            output: ARENA_BASE + 1,
            kind: NodeKind::Bound(Box::new(FailingMatrixOwner {
                calls: Arc::clone(&matrix_calls),
                queued: Arc::clone(&queued),
            })),
            observers: Box::new([]),
        };
        let mut lease = stereo_lease(2, 2);
        lease
            .write_stereo(ARENA_BASE)
            .0
            .copy_from_slice(&[0.25, -0.5]);
        lease
            .write_stereo(ARENA_BASE)
            .1
            .copy_from_slice(&[-0.75, 1.0]);
        lease.write_stereo(ARENA_BASE + 1).0.fill(91.0);
        lease.write_stereo(ARENA_BASE + 1).1.fill(-91.0);
        execute_op(&mut fader, &mut lease, &mut [], &mut [], 0).expect("earlier fader");
        assert_eq!(
            execute_op(&mut matrix, &mut lease, &mut [], &mut [], 0),
            Err(RenderError::InvalidEnvelope)
        );
        assert_eq!(lease.read_stereo(ARENA_BASE).0, &[0.5, -1.0]);
        assert_eq!(lease.read_stereo(ARENA_BASE).1, &[-1.5, 2.0]);
        assert_eq!(lease.read_stereo(ARENA_BASE + 1).0, &[0.5, -1.0]);
        assert_eq!(lease.read_stereo(ARENA_BASE + 1).1, &[-1.5, 2.0]);
        assert_eq!(fader_calls.load(Ordering::Relaxed), 1);
        assert_eq!(matrix_calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            queued.load(Ordering::Relaxed),
            1,
            "later failure retains its next record"
        );
    }

    struct DecliningPairOwner(Arc<AtomicUsize>);
    fn decline_pair(
        left: crate::BuiltinProcessor,
        right: crate::BuiltinProcessor,
    ) -> Result<crate::BuiltinProcessor, (crate::BuiltinProcessor, crate::BuiltinProcessor)> {
        Err((left, right))
    }
    impl GraphPreparedBuiltinBankProcessor for DecliningPairOwner {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn pair_factory(&self) -> Option<crate::BuiltinPairFactory> {
            Some(decline_pair)
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            _: u32,
            _: u64,
        ) -> Result<(), RenderError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            for sample in left.iter_mut().chain(right.iter_mut()) {
                *sample += 1.0;
            }
            Ok(())
        }
    }
    struct PlainPairOwner(Arc<AtomicUsize>);
    impl GraphPreparedBuiltinBankProcessor for PlainPairOwner {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            _: u32,
            _: u64,
        ) -> Result<(), RenderError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            for sample in left.iter_mut().chain(right.iter_mut()) {
                *sample *= 2.0;
            }
            Ok(())
        }
    }

    #[test]
    fn a_declined_first_pair_retains_the_first_slots_scratch() {
        let track = crate::StableGraphId::parse("decline").expect("id");
        let fader = GraphNodeId::TrackStage {
            track_id: track.clone(),
            stage: TrackStage::PostFader,
        };
        let matrix = GraphNodeId::TrackStage {
            track_id: track,
            stage: TrackStage::PostMatrix,
        };
        let spec = GraphSpec {
            nodes: vec![
                crate::GraphNode {
                    id: fader.clone(),
                    latency: effect_contract::LatencySamples(0),
                    tail: effect_contract::TailSamples::Finite(0),
                },
                crate::GraphNode {
                    id: matrix.clone(),
                    latency: effect_contract::LatencySamples(0),
                    tail: effect_contract::TailSamples::Finite(0),
                },
            ],
            ports: Vec::new(),
            edges: Vec::new(),
        };
        let bank = |member, processor: Box<dyn GraphPreparedBuiltinBankProcessor>| {
            GraphPreparedBuiltinBank {
                backend: lane::Backend::Simd4,
                members: vec![member].into_boxed_slice(),
                processor,
                scratch: AoSoaScratch::new(effect_contract::BankWidth::Four, 8).expect("scratch"),
            }
        };
        let first_calls = Arc::new(AtomicUsize::new(0));
        let second_calls = Arc::new(AtomicUsize::new(0));
        let mut parts = RuntimeParts::new(
            &spec,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![
                bank(
                    fader,
                    Box::new(DecliningPairOwner(Arc::clone(&first_calls))),
                ),
                bank(matrix, Box::new(PlainPairOwner(Arc::clone(&second_calls)))),
            ],
            Vec::new(),
            Vec::new(),
            Default::default(),
            Vec::new(),
            8,
        );
        let mut chain = parts.chain_for(&[Membership::Builtin(0), Membership::Builtin(1)], 1);
        assert!(
            parts.builtin_banks.iter().all(Option::is_none),
            "both original owners moved once"
        );
        const FRAMES: usize = 2;
        let mut lease = stereo_lease(FRAMES, 3);
        lease.write_stereo(1).0.copy_from_slice(&[1.0, 2.0]);
        lease.write_stereo(1).1.copy_from_slice(&[-1.0, -2.0]);
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &[1],
            outputs: &[2],
            fold: &[],
            master: 0,
        };
        chain
            .run(&mut members, FRAMES as u32, 0)
            .expect("declined chain render");
        assert_eq!(
            first_calls.load(Ordering::Relaxed),
            1,
            "first returned owner executes"
        );
        assert_eq!(
            second_calls.load(Ordering::Relaxed),
            1,
            "second returned owner executes"
        );
        assert_eq!(members.lease.read_stereo(2).0, &[4.0, 6.0]);
        assert_eq!(members.lease.read_stereo(2).1, &[0.0, -2.0]);
    }

    /// The node's cached witness and the line's own answer are the same fact (#210 phase 2).
    ///
    /// `NodeKind::channel_symmetry` is asked without the delay lines in hand, so the lowering
    /// caches the verdict on the variant. This is what keeps the cache and the rings from drifting.
    #[test]
    fn the_node_witness_agrees_with_its_line() {
        for (left, right) in [(0_usize, 0_usize), (1, 1), (480, 480), (0, 1), (480, 481)] {
            let line = TrackDelayLine::new(left, right);
            assert_eq!(line.lane_samples(), [left, right]);
            assert_eq!(line.channels_agree(), left == right);
            let kind = NodeKind::TrackDelay {
                line: 0,
                channels_agree: line.channels_agree(),
            };
            assert_eq!(
                kind.channel_symmetry().eligible(),
                left == right,
                "the op witness for {left}/{right}"
            );
        }
    }

    /// A lane's ring is exactly a pure `N`-sample shift, whatever the block partitioning.
    ///
    /// Blocks shorter than, equal to and longer than the ring, and a ring of one -- the three cases
    /// `delay_lane`'s take loop exists for. FP-free by construction: the kernel swaps words, so
    /// this is an exact equality over a sequence that includes signed zeros and a NaN, none of
    /// which a ring is allowed to alter.
    #[test]
    fn a_delay_line_is_a_pure_shift_at_every_partitioning() {
        for delay in [1_usize, 3, 8, 32] {
            for block in [1_usize, 4, 8, 31, 64] {
                let source: Vec<f32> = (0..128)
                    .map(|index| match index {
                        5 => -0.0,
                        9 => f32::NAN,
                        other => other as f32 + 0.5,
                    })
                    .collect();
                let mut line = TrackDelayLine::new(delay, delay);
                let mut got = Vec::new();
                for chunk in source.chunks(block) {
                    let mut left = chunk.to_vec();
                    let mut right = chunk.to_vec();
                    line.process(&mut left, &mut right);
                    assert_eq!(
                        left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        "equal-length lanes must stay in step"
                    );
                    got.extend(left);
                }
                for (index, value) in got.iter().enumerate() {
                    let want = if index < delay {
                        0.0_f32
                    } else {
                        source[index - delay]
                    };
                    assert_eq!(
                        value.to_bits(),
                        want.to_bits(),
                        "delay {delay}, block {block}, sample {index}"
                    );
                }
            }
        }
    }

    /// Deterministic 32-bit LCG, frozen here so the corpora do not depend on host RNG state.
    fn lcg(state: &mut u32) -> f32 {
        *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f32::from(((*state >> 16) & 0xffff) as i16) / 3_276.8
    }

    /// Lays `inputs` out as one lease over `inputs.len() + 1` buffers and reduces into the first.
    fn reduce_case(frames: usize, inputs: &[Vec<f32>]) -> Vec<f32> {
        let mut lease = single_lease(frames, inputs.len() + 1);
        let refs: Vec<u32> = (2..=inputs.len() as u32 + 1).collect();
        // The output slot starts at a sentinel, never at zero: a reduction that forgets to write
        // it -- the fan-in-zero fill in particular -- must not be able to pass by accident.
        lease.write(0, 1).fill(f32::from_bits(0x7f7f_7f7f));
        for (index, input) in inputs.iter().enumerate() {
            lease.write(0, refs[index]).copy_from_slice(input);
        }
        reduce_plane(&mut lease, 0, 1, &refs);
        lease.read(0, 1).to_vec()
    }

    fn old_reduce_case(frames: usize, inputs: &[Vec<f32>]) -> Vec<f32> {
        let mut lease = single_lease(frames, inputs.len() + 1);
        let refs: Vec<u32> = (2..=inputs.len() as u32 + 1).collect();
        lease.write(0, 1).fill(f32::from_bits(0x7f7f_7f7f));
        for (index, input) in inputs.iter().enumerate() {
            lease.write(0, refs[index]).copy_from_slice(input);
        }
        old_reduce_plane(&mut lease, 0, 1, &refs);
        lease.read(0, 1).to_vec()
    }

    /// Frozen pre-RT-3 oracle: the old two-kernel left-associated reduction.
    fn old_reduce_plane(lease: &mut ArenaLease, plane: usize, out: u32, inputs: &[u32]) {
        match inputs {
            [] => lease.write(plane, out).fill(0.0),
            [single] => {
                if *single != out {
                    let (output, input) = lease.write_read(plane, out, *single);
                    output.copy_from_slice(input);
                }
            }
            [first, second, rest @ ..] => {
                {
                    let (output, a, b) = lease.write_read2(plane, out, *first, *second);
                    sum2_block::<FrameLane>(output, a, b);
                }
                for next in rest {
                    let (output, input) = lease.write_read(plane, out, *next);
                    sum_into_block::<FrameLane>(output, input);
                }
            }
        }
    }

    fn assert_width_matches_old<L: Lane>() {
        #[derive(Clone, Copy, Debug)]
        enum Family {
            Finite,
            NegativeZero,
            SmallNormal,
            Subnormal,
            Infinity,
            Nan,
        }

        let _fp_env = lane::fpenv::CanonicalFpEnv::enter();
        for frames in [
            1,
            L::WIDTH.saturating_sub(1).max(1),
            L::WIDTH,
            L::WIDTH + 1,
            L::WIDTH * 3 + 1,
            128,
        ] {
            for family in [
                Family::Finite,
                Family::NegativeZero,
                Family::SmallNormal,
                Family::Subnormal,
                Family::Infinity,
                Family::Nan,
            ] {
                let inputs: Vec<Vec<f32>> = (0..9)
                    .map(|input| {
                        (0..frames)
                            .map(|frame| match family {
                                Family::Finite => match (frame % 2, input) {
                                    (0, 0) => 2.0,
                                    (0, 1) => -0.5,
                                    (0, 2) => 0.25,
                                    (1, 0) => 16_777_216.0,
                                    (1, 1) => 1.0,
                                    (1, 2) => -16_777_216.0,
                                    _ => 0.0,
                                },
                                Family::NegativeZero => -0.0,
                                Family::SmallNormal => {
                                    if input < 2 {
                                        f32::MIN_POSITIVE
                                    } else {
                                        0.0
                                    }
                                }
                                Family::Subnormal => {
                                    if input < 2 {
                                        f32::from_bits(1)
                                    } else {
                                        0.0
                                    }
                                }
                                Family::Infinity => match (frame % 2, input) {
                                    (0, 0) => f32::INFINITY,
                                    (0, 1) => 1.0,
                                    (1, 0) => f32::NEG_INFINITY,
                                    (1, 1) => -1.0,
                                    _ => 0.0,
                                },
                                Family::Nan => {
                                    if input == 0 {
                                        f32::from_bits(0x7fc0_4201)
                                    } else {
                                        0.0
                                    }
                                }
                            })
                            .collect()
                    })
                    .collect();
                let mut actual = single_lease(frames, 11);
                let mut old = single_lease(frames, 11);
                let ids: Vec<u32> = (2..11).collect();
                for (index, values) in inputs.iter().enumerate() {
                    actual.write(0, ids[index]).copy_from_slice(values);
                    old.write(0, ids[index]).copy_from_slice(values);
                }
                reduce_many::<L>(&mut actual, 0, 1, ids[0], ids[1], &ids[2..]);
                {
                    let (output, first, second) = old.write_read2(0, 1, ids[0], ids[1]);
                    sum2_block::<L>(output, first, second);
                }
                for id in &ids[2..] {
                    let (output, input) = old.write_read(0, 1, *id);
                    sum_into_block::<L>(output, input);
                }
                for (frame, expected) in old.read(0, 1).iter().enumerate() {
                    let expected_bits = match family {
                        Family::Finite if frame % 2 == 0 => 1.75_f32.to_bits(),
                        Family::Finite => 0.0_f32.to_bits(),
                        Family::NegativeZero => (-0.0_f32).to_bits(),
                        Family::SmallNormal => 0x0100_0000,
                        Family::Subnormal => 2,
                        Family::Infinity if frame % 2 == 0 => f32::INFINITY.to_bits(),
                        Family::Infinity => f32::NEG_INFINITY.to_bits(),
                        Family::Nan => {
                            assert!(expected.is_nan(), "old NaN family output at frame {frame}");
                            expected.to_bits()
                        }
                    };
                    assert_eq!(
                        expected.to_bits(),
                        expected_bits,
                        "old {family:?} category at width {} frames {frames} frame {frame}",
                        L::WIDTH
                    );
                }
                assert_eq!(
                    actual
                        .read(0, 1)
                        .iter()
                        .map(|x| x.to_bits())
                        .collect::<Vec<_>>(),
                    old.read(0, 1)
                        .iter()
                        .map(|x| x.to_bits())
                        .collect::<Vec<_>>(),
                    "{family:?} width {} frames {frames}",
                    L::WIDTH
                );
            }
        }
    }

    #[test]
    fn every_lane_width_matches_the_frozen_old_kernel_on_hostile_values() {
        assert_width_matches_old::<f32>();
        assert_width_matches_old::<lane::Simd4>();
        assert_width_matches_old::<lane::Simd8>();
    }

    #[test]
    fn reduction_preserves_repeated_silence_self_and_unrelated_buffers() {
        const FRAMES: usize = 5;
        let build = || stereo_lease(FRAMES, 6);
        let mut actual = build();
        let mut old = build();
        for lease in [&mut actual, &mut old] {
            lease
                .write(0, 2)
                .copy_from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0]);
            lease
                .write(1, 2)
                .copy_from_slice(&[-1.0, -2.0, -3.0, -4.0, -5.0]);
            lease.write(0, 3).fill(99.0);
            lease.write(1, 3).fill(-99.0);
            lease.write(0, 4).fill(0.5);
            lease.write(1, 4).fill(-0.25);
            lease.write(0, 1).fill(f32::from_bits(0x7fc0_4202));
            lease.write(1, 1).fill(f32::from_bits(0xffc0_4202));
            lease.write(0, 5).fill(f32::from_bits(0x7fc0_4203));
            lease.write(1, 5).fill(f32::from_bits(0xffc0_4203));
        }
        let ids = [2, 2, 0, 3, 4];
        for plane in 0..2 {
            reduce_plane(&mut actual, plane, 1, &ids);
            old_reduce_plane(&mut old, plane, 1, &ids);
        }
        for plane in 0..2 {
            assert_eq!(
                actual
                    .read(plane, 1)
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                old.read(plane, 1)
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            assert!(
                actual
                    .read(plane, 5)
                    .iter()
                    .all(|x| x.to_bits() == if plane == 0 { 0x7fc0_4203 } else { 0xffc0_4203 })
            );
        }
        let mut self_alias = single_lease(2, 2);
        self_alias
            .write(0, 2)
            .copy_from_slice(&[-0.0, f32::from_bits(0x7fc0_4204)]);
        reduce_plane(&mut self_alias, 0, 2, &[2]);
        reduce_plane(&mut self_alias, 0, 2, &[2]);
        assert_eq!(
            self_alias
                .read(0, 2)
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            vec![(-0.0f32).to_bits(), 0x7fc0_4204]
        );
    }

    /// One lease that owns `buffers` buffers of one plane, as the sequential executor does.
    fn single_lease(frames: usize, buffers: usize) -> ArenaLease {
        let mut builder = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(1).expect("one plane"),
            NonZeroUsize::new(frames).expect("frames"),
        );
        let owned: Vec<u32> = (0..buffers).map(|_| builder.reserve()).collect();
        builder.lease(0, owned.clone(), owned);
        let (_arena, mut leases) = builder.finish().expect("one disjoint lease");
        leases.pop().expect("the lease")
    }

    /// One stereo lease over `buffers` buffers, the shape `build_sequential` builds.
    fn stereo_lease(frames: usize, buffers: usize) -> ArenaLease {
        let mut builder = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(2).expect("stereo planes"),
            NonZeroUsize::new(frames).expect("frames"),
        );
        let owned: Vec<u32> = (0..buffers).map(|_| builder.reserve()).collect();
        builder.lease(0, owned.clone(), owned);
        let (_arena, mut leases) = builder.finish().expect("one disjoint lease");
        leases.pop().expect("the lease")
    }

    /// The compatibility callback is deliberately unusable here: a regression to per-lane
    /// dispatch must fail rather than quietly producing the same sum.
    #[test]
    fn all_active_folded_bank_chain_dispatches_the_real_graph_cohort() {
        struct Identity;
        impl BankStage for Identity {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
        }
        struct Probe<'a> {
            inner: ArenaMembers<'a>,
            cohorts: usize,
        }
        impl BankMembers for Probe<'_> {
            fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
                self.inner.plane(lane)
            }
            fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
                self.inner.plane_mut(lane)
            }
            fn fold_plane(&mut self, _lane: usize, _left: &mut [f32], _right: &mut [f32]) {
                panic!("cohort dispatch regressed to fold_plane")
            }
            fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
                self.cohorts += 1;
                self.inner.fold_cohort(cohort);
            }
        }
        const FRAMES: usize = 2;
        let mut lease = stereo_lease(FRAMES, 10);
        let inputs = [2, 3, 4, 5];
        let outputs = [6, 7, 8, 9];
        let coefficients = [1.0, 0.0, 0.0, 1.0];
        for (lane, input) in inputs.iter().copied().enumerate() {
            let (left, right) = lease.write_stereo(input);
            left.fill(lane as f32 + 1.0);
            right.fill(-(lane as f32 + 1.0));
        }
        let mut oracle_lease = stereo_lease(FRAMES, 5);
        let oracle_routes = [
            ARENA_BASE + 1,
            ARENA_BASE + 2,
            ARENA_BASE + 3,
            ARENA_BASE + 4,
        ];
        for (lane, route) in oracle_routes.iter().copied().enumerate() {
            let (left, right) = oracle_lease.write_stereo(route);
            left.fill(lane as f32 + 1.0);
            right.fill(-(lane as f32 + 1.0));
            mix2x2_block::<FrameLane>(left, right, coefficients);
        }
        old_reduce_plane(&mut oracle_lease, 0, ARENA_BASE, &oracle_routes);
        old_reduce_plane(&mut oracle_lease, 1, ARENA_BASE, &oracle_routes);
        let (oracle_left, oracle_right) = oracle_lease.read_stereo(ARENA_BASE);
        let expected_left = oracle_left.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        let expected_right = oracle_right.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        let fold: Vec<FoldLane> = (0..4)
            .map(|lane| FoldLane {
                coefficients,
                store: lane == 0,
            })
            .collect();
        let active = vec![true; 4].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, FRAMES as u32).expect("scratch"),
            active.clone(),
            vec![BankSlot {
                stage: Box::new(Identity),
                active_lanes: active.clone(),
            }],
        )
        .expect("chain");
        chain.arm_fold(active).expect("fold mask");
        let mut members = Probe {
            inner: ArenaMembers {
                lease: &mut lease,
                inputs: &inputs,
                outputs: &outputs,
                fold: &fold,
                master: ARENA_BASE,
            },
            cohorts: 0,
        };
        chain.run(&mut members, FRAMES as u32, 0).expect("run");
        assert_eq!(members.cohorts, 1);
        let (left, right) = members.inner.lease.read_stereo(ARENA_BASE);
        assert_eq!(
            left.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            expected_left
        );
        assert_eq!(
            right.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            expected_right
        );
    }

    /// RT-1: real graph arena members gather one buffer set and scatter directly into another.
    #[test]
    fn arena_members_direct_scatter_preserves_redirected_identity_bits() {
        struct Identity;
        impl BankStage for Identity {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
        }

        const FRAMES: usize = 11;
        let mut lease = stereo_lease(FRAMES, 8);
        let inputs = [1, 2, 3, 4];
        let outputs = [5, 6, 7, 8];
        let mut expected = Vec::new();
        for (lane, input) in inputs.iter().copied().enumerate() {
            let left: Vec<f32> = (0..FRAMES)
                .map(|frame| f32::from_bits(0x7fc0_0000 | ((lane * FRAMES + frame) as u32 + 1)))
                .collect();
            let right: Vec<f32> = left
                .iter()
                .map(|word| f32::from_bits(word.to_bits() ^ 0x8000_3990))
                .collect();
            lease.write(0, input).copy_from_slice(&left);
            lease.write(1, input).copy_from_slice(&right);
            expected.push((left, right));
        }
        for output in outputs {
            lease.write(0, output).fill(f32::from_bits(0x7f80_3991));
            lease.write(1, output).fill(f32::from_bits(0x7f80_3992));
        }

        let active = vec![true; 4].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, FRAMES as u32).expect("scratch"),
            active.clone(),
            vec![BankSlot {
                stage: Box::new(Identity),
                active_lanes: active,
            }],
        )
        .expect("chain");
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &inputs,
            outputs: &outputs,
            fold: &[],
            master: 0,
        };
        chain
            .run(&mut members, FRAMES as u32, 0)
            .expect("direct graph run");

        for (lane, output) in outputs.iter().copied().enumerate() {
            let (left, right) = lease.read_stereo(output);
            assert_eq!(
                left.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
                expected[lane]
                    .0
                    .iter()
                    .map(|word| word.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                right.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
                expected[lane]
                    .1
                    .iter()
                    .map(|word| word.to_bits())
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(chain.transposes(), 1);
    }

    /// The folded epilogue is the route op followed by the D9 reduction, word for word.
    ///
    /// The oracle is the shape this replaces, built out of the very kernels the route op and
    /// `reduce_plane` call: `mix2x2_block` per contributor into its own buffer, then
    /// `sum2_block(first, second)` and `sum_into_block` for the rest, left to right. Nothing here
    /// is a restatement of the epilogue's arithmetic -- both sides are the production kernels, and
    /// what is under test is that the epilogue puts them in the same order.
    ///
    /// Block lengths that are not a multiple of the lane width are included, because the epilogue
    /// hands `mix2x2_block` a *staging* slice where the route op handed it an arena buffer: if the
    /// two lengths could differ the vector/tail split would differ with them, and the bits with it.
    ///
    /// Red mutations: accumulate the first contributor instead of storing it -- the `-0.0` case
    /// below fails; reverse the lane order -- the multi-contributor cases fail.
    #[test]
    fn a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit() {
        let coefficients = [
            [0.5_f32, -0.25, 0.125, 0.75],
            [1.0, 0.0, 0.0, 1.0],
            [-0.3, 0.9, 0.4, -0.6],
        ];
        for frames in [1_usize, 3, 7, 63, 64, 65, 128] {
            let mut state = 0x0bad_f00du32;
            let tiles: Vec<(Vec<f32>, Vec<f32>)> = (0..coefficients.len())
                .map(|_| {
                    (
                        (0..frames).map(|_| lcg(&mut state)).collect(),
                        (0..frames).map(|_| lcg(&mut state)).collect(),
                    )
                })
                .collect();

            // The oracle: one route buffer per contributor, then the D9 left-to-right reduction.
            let mut lease = stereo_lease(frames, coefficients.len() + 1);
            // The arena reserves buffer zero as the always-zero silence slot.
            let master = ARENA_BASE;
            let routes: Vec<u32> =
                (ARENA_BASE + 1..=ARENA_BASE + coefficients.len() as u32).collect();
            for (index, buffer) in routes.iter().enumerate() {
                let (left, right) = lease.write_stereo(*buffer);
                left.copy_from_slice(&tiles[index].0);
                right.copy_from_slice(&tiles[index].1);
                mix2x2_block::<FrameLane>(left, right, coefficients[index]);
            }
            old_reduce_plane(&mut lease, 0, master, &routes);
            old_reduce_plane(&mut lease, 1, master, &routes);
            let (oracle_left, oracle_right) = lease.read_stereo(master);
            let oracle: (Vec<u32>, Vec<u32>) = (
                oracle_left.iter().map(|value| value.to_bits()).collect(),
                oracle_right.iter().map(|value| value.to_bits()).collect(),
            );

            // The epilogue: the same tiles, in an opening cohort and a continuation cohort.
            let mut folded_lease = stereo_lease(frames, 1);
            let fold: Vec<FoldLane> = coefficients
                .iter()
                .enumerate()
                .map(|(index, coefficients)| FoldLane {
                    coefficients: *coefficients,
                    store: index == 0,
                })
                .collect();
            let mut members = ArenaMembers {
                lease: &mut folded_lease,
                inputs: &[],
                outputs: &[],
                fold: &fold,
                master,
            };
            let mut staged_left: Vec<f32> = tiles
                .iter()
                .flat_map(|tile| tile.0.iter().copied())
                .collect();
            let mut staged_right: Vec<f32> = tiles
                .iter()
                .flat_map(|tile| tile.1.iter().copied())
                .collect();
            members.fold_cohort(
                FoldCohort::new(&[0, 1], &mut staged_left, &mut staged_right, frames, frames)
                    .expect("valid opening cohort"),
            );
            members.fold_cohort(
                FoldCohort::new(&[2], &mut staged_left, &mut staged_right, frames, frames)
                    .expect("valid continuation cohort"),
            );
            let (folded_left, folded_right) = folded_lease.read_stereo(master);
            assert_eq!(
                (
                    folded_left
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    folded_right
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                ),
                oracle,
                "{frames} frames: the epilogue is not the route plus the reduction"
            );
        }
    }

    #[test]
    fn a_later_folded_cohort_continues_from_the_live_master_in_d9_order() {
        for frames in [1_usize, 3, 8, 11] {
            let master = ARENA_BASE;
            let fold = [
                FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: true,
                },
                FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: false,
                },
                FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: false,
                },
            ];
            let mut lease = stereo_lease(frames, 1);
            let mut left = [
                vec![16_777_216.0; frames],
                vec![1.0; frames],
                vec![-16_777_216.0; frames],
            ]
            .concat();
            let mut right = [
                vec![-16_777_216.0; frames],
                vec![-1.0; frames],
                vec![16_777_216.0; frames],
            ]
            .concat();
            let mut members = ArenaMembers {
                lease: &mut lease,
                inputs: &[],
                outputs: &[],
                fold: &fold,
                master,
            };
            members.fold_cohort(
                FoldCohort::new(&[0], &mut left, &mut right, frames, frames)
                    .expect("opening cohort"),
            );
            members.fold_cohort(
                FoldCohort::new(&[1, 2], &mut left, &mut right, frames, frames)
                    .expect("continuation cohort"),
            );
            let (actual_left, actual_right) = lease.read_stereo(master);
            assert!(
                actual_left
                    .iter()
                    .all(|sample| sample.to_bits() == 0.0_f32.to_bits())
            );
            assert!(
                actual_right
                    .iter()
                    .all(|sample| sample.to_bits() == 0.0_f32.to_bits())
            );
            assert_eq!(
                (16_777_216.0_f32 + (1.0 - 16_777_216.0)).to_bits(),
                1.0_f32.to_bits()
            );
        }
    }

    #[test]
    fn malformed_folded_cohorts_are_rejected_before_route_or_master_mutation() {
        const FRAMES: usize = 4;
        let fold = [
            FoldLane {
                coefficients: [2.0, 0.0, 0.0, 2.0],
                store: true,
            },
            FoldLane {
                coefficients: [3.0, 0.0, 0.0, 3.0],
                store: true,
            },
        ];
        for ids in [&[0, 1][..], &[2][..]] {
            let mut lease = stereo_lease(FRAMES, 1);
            let (master_left, master_right) = lease.write_stereo(ARENA_BASE);
            master_left.fill(19.0);
            master_right.fill(-23.0);
            let mut left = vec![5.0_f32; FRAMES * 3];
            let mut right = vec![-7.0_f32; FRAMES * 3];
            let before_left: Vec<u32> = left.iter().map(|x| x.to_bits()).collect();
            let before_right: Vec<u32> = right.iter().map(|x| x.to_bits()).collect();
            let mut members = ArenaMembers {
                lease: &mut lease,
                inputs: &[],
                outputs: &[],
                fold: &fold,
                master: ARENA_BASE,
            };
            members.fold_cohort(
                FoldCohort::new(ids, &mut left, &mut right, FRAMES, FRAMES)
                    .expect("representable invalid graph metadata"),
            );
            assert_eq!(
                left.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                before_left
            );
            assert_eq!(
                right.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                before_right
            );
            let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
            assert!(
                master_left
                    .iter()
                    .all(|x| x.to_bits() == 19.0_f32.to_bits())
            );
            assert!(
                master_right
                    .iter()
                    .all(|x| x.to_bits() == (-23.0_f32).to_bits())
            );
        }

        let mut lease = stereo_lease(FRAMES, 1);
        let (master_left, master_right) = lease.write_stereo(ARENA_BASE);
        master_left.fill(29.0);
        master_right.fill(-31.0);
        let mut left = vec![11.0_f32; FRAMES + 1];
        let mut right = vec![-13.0_f32; FRAMES + 1];
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: ARENA_BASE,
        };
        members.fold_cohort(
            FoldCohort::new(&[0], &mut left, &mut right, FRAMES + 1, FRAMES + 1)
                .expect("shape is valid at the public boundary"),
        );
        assert!(left.iter().all(|x| x.to_bits() == 11.0_f32.to_bits()));
        assert!(right.iter().all(|x| x.to_bits() == (-13.0_f32).to_bits()));
        let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
        assert!(
            master_left
                .iter()
                .all(|x| x.to_bits() == 29.0_f32.to_bits())
        );
        assert!(
            master_right
                .iter()
                .all(|x| x.to_bits() == (-31.0_f32).to_bits())
        );
    }

    /// The first contributor **stores**: a master whose only summand is `-0.0` stays `-0.0`.
    ///
    /// This is the law correction 1 of the plan states, and it is an absolute property rather than
    /// a comparison, because both arms of a differential move together under the mutation that
    /// breaks it. Zero-filling the master and accumulating every contributor computes
    /// `0.0 + (-0.0)`, which is `+0.0`; `sum2_block` computes `t0 + t1` directly, and a fan-in-one
    /// reduction is a *copy*, which is why `reduce_plane` has never been `fold(0.0, +)` either.
    ///
    /// Red mutation: replace the `store` arm's `copy_from_slice` with `sum_into_block` -- the
    /// master is `+0.0` (bits 0) where `-0.0` (bits 0x8000_0000) is required.
    #[test]
    fn the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign() {
        const FRAMES: usize = 8;
        let mut lease = stereo_lease(FRAMES, 1);
        // The arena starts at `+0.0`, which is exactly the value a zero-fill would leave.
        assert!(
            lease
                .read_stereo(ARENA_BASE)
                .0
                .iter()
                .all(|value| *value == 0.0)
        );
        let fold = [FoldLane {
            // Identity, so the route stage cannot itself manufacture a sign: `0.0 * (-0.0)` is
            // `-0.0` and `-0.0 + -0.0` is `-0.0`.
            coefficients: [1.0, 0.0, 0.0, 1.0],
            store: true,
        }];
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: ARENA_BASE,
        };
        let mut left = vec![-0.0_f32; FRAMES];
        let mut right = vec![-0.0_f32; FRAMES];
        members.fold_plane(0, &mut left, &mut right);
        let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
        for frame in 0..FRAMES {
            assert_eq!(
                master_left[frame].to_bits(),
                0x8000_0000,
                "frame {frame}: the first contributor's sign was lost on the left"
            );
            assert_eq!(
                master_right[frame].to_bits(),
                0x8000_0000,
                "frame {frame}: the first contributor's sign was lost on the right"
            );
        }

        let mut cohort_lease = stereo_lease(FRAMES, 1);
        let (poison_left, poison_right) = cohort_lease.write_stereo(ARENA_BASE);
        poison_left.fill(17.0);
        poison_right.fill(-19.0);
        let mut cohort_members = ArenaMembers {
            lease: &mut cohort_lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: ARENA_BASE,
        };
        let mut cohort_left = vec![-0.0_f32; FRAMES];
        let mut cohort_right = vec![-0.0_f32; FRAMES];
        cohort_members.fold_cohort(
            FoldCohort::new(&[0], &mut cohort_left, &mut cohort_right, FRAMES, FRAMES)
                .expect("valid signed-zero cohort"),
        );
        assert!(
            cohort_left
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
        assert!(
            cohort_right
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
        let (master_left, master_right) = cohort_lease.read_stereo(ARENA_BASE);
        assert!(
            master_left
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
        assert!(
            master_right
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
    }

    /// The fan-in-zero fill is skipped under a bound source and kept everywhere else.
    ///
    /// Both directions, because only one of them is the optimisation:
    ///
    /// * A **bound** node with no graph inputs is written entirely by its host processor, so the
    ///   `fill(0.0)` in front of it is two stereo blocks of dead stores. The buffer is left holding
    ///   the previous block's words on the way in, which is what the trait contract now says.
    /// * An **identity** node with no graph inputs is a submix nothing routes into, and the fill
    ///   *is* its audio. Dropping the `NodeKind::Bound` guard reddens this arm at once.
    #[test]
    fn the_fan_in_zero_fill_is_dead_only_under_a_bound_source() {
        /// A host source that writes every word it is handed, as the contract requires.
        struct Fill(f32);
        impl crate::GraphRuntimeProcessor for Fill {
            fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                block.left.fill(self.0);
                block.right.fill(-self.0);
                Ok(())
            }
        }
        const FRAMES: usize = 16;
        const STALE: f32 = 1.5;

        for (case, kind, expected) in [
            (
                "a bound source overwrites the block, so the fill is dead",
                NodeKind::Bound(Box::new(Fill(0.25))),
                (0.25_f32, -0.25_f32),
            ),
            (
                "an identity node with no contributors renders silence, and the fill is that",
                NodeKind::Identity,
                (0.0, 0.0),
            ),
        ] {
            let mut lease = stereo_lease(FRAMES, 1);
            lease.write(0, ARENA_BASE).fill(STALE);
            lease.write(1, ARENA_BASE).fill(STALE);
            let mut op = RuntimeOp {
                inputs: Box::new([]),
                staged: Box::new([]),
                sidechain: None,
                output: ARENA_BASE,
                kind,
                observers: Box::new([]),
            };
            execute_op(&mut op, &mut lease, &mut [], &mut [], 0).expect("op");
            let (left, right) = lease.read_stereo(ARENA_BASE);
            assert!(
                left.iter().all(|value| *value == expected.0)
                    && right.iter().all(|value| *value == expected.1),
                "{case}"
            );
        }
    }

    /// The dedication copy is skipped for exactly the shape that is only a copy, and for nothing
    /// else.
    ///
    /// Red mutations (`tests/MUTATIONS.md`): drop the `staged` clause -- the delayed case admits
    /// and the delay line stops being pumped; drop the `sidechain` clause -- the sidechained case
    /// admits; accept two inputs -- the summed case admits and a summand is lost; accept
    /// `inputs[0] == output` -- the already-in-place case admits, which is a no-op redirect that
    /// hides the clause; widen past `BankMember` -- the `Effect` case admits and the effect's own
    /// processing is skipped along with the copy.
    #[test]
    fn bank_gather_source_admits_only_the_dedication_copy() {
        fn member(
            inputs: Vec<u32>,
            staged: Vec<StagedInput>,
            sidechain: Option<u32>,
            output: u32,
            kind: NodeKind,
        ) -> RuntimeOp {
            RuntimeOp {
                inputs: inputs.into_boxed_slice(),
                staged: staged.into_boxed_slice(),
                sidechain,
                output,
                kind,
                observers: Box::new([]),
            }
        }

        // The shape the SIMD racks actually produce: a dedicated member whose whole body is one
        // copy from its producer.
        assert_eq!(
            bank_gather_source(&member(vec![7], Vec::new(), None, 9, NodeKind::BankMember)),
            Some(7),
            "a single undelayed input into a distinct dedicated buffer is the copy"
        );

        // Every refusal, one clause each.
        for (case, op) in [
            (
                "no input at all is a fill(0.0), not a copy",
                member(Vec::new(), Vec::new(), None, 9, NodeKind::BankMember),
            ),
            (
                "two inputs are a sum; skipping it would drop a summand",
                member(vec![7, 8], Vec::new(), None, 9, NodeKind::BankMember),
            ),
            (
                "a staged input owns a delay line that must still be pumped",
                member(
                    vec![7],
                    vec![StagedInput {
                        source: 6,
                        staging: 7,
                        line: 0,
                    }],
                    None,
                    9,
                    NodeKind::BankMember,
                ),
            ),
            (
                "a sidechain is a second read the gather has no port for",
                member(vec![7], Vec::new(), Some(8), 9, NodeKind::BankMember),
            ),
            (
                "already in place: there is no copy left to skip",
                member(vec![9], Vec::new(), None, 9, NodeKind::BankMember),
            ),
            (
                "a kind that does work of its own must run",
                member(vec![7], Vec::new(), None, 9, NodeKind::Identity),
            ),
            (
                "a route does work of its own too",
                member(
                    vec![7],
                    Vec::new(),
                    None,
                    9,
                    NodeKind::Route([1.0, 0.0, 0.0, 1.0]),
                ),
            ),
        ] {
            assert_eq!(bank_gather_source(&op), None, "{case}");
        }
    }

    /// E5. Master plan #83 D9: the block reduction is bit-for-bit the scalar left-to-right
    /// reference `inputs.reduce(|a, b| a + b)`, at every fan-in and for every frame.
    ///
    /// Red mutations (`tests/MUTATIONS.md`): restore the balanced pairwise tree -- (a) fails;
    /// use `fold(0.0, +)` in the reference -- (c) fails; reverse the input order -- (b) fails.
    #[test]
    fn reduction_is_left_to_right_bit_identical_to_scalar_reference() {
        // (a) The classic discriminator: 1 + 2^-24 + 2^-24 + 0 is 1.0 left to right, but
        // 1 + 2^-23 = 1 + 2^-23 as a balanced tree.
        let frames = 128;
        let epsilon = f32::from_bits(0x3380_0000); // 2^-24
        let inputs = vec![
            vec![1.0f32; frames],
            vec![epsilon; frames],
            vec![epsilon; frames],
            vec![0.0f32; frames],
        ];
        for sample in reduce_case(frames, &inputs) {
            assert_eq!(
                sample.to_bits(),
                1.0f32.to_bits(),
                "left-to-right must round away the second epsilon"
            );
        }

        let order = [16_777_216.0_f32, 1.0, -16_777_216.0];
        let old = (order[0] + order[1]) + order[2];
        let wrong = order[0] + (order[1] + order[2]);
        assert_ne!(old.to_bits(), wrong.to_bits());
        let got = reduce_case(1, &order.map(|value| vec![value]));
        let old_kernel = old_reduce_case(1, &order.map(|value| vec![value]));
        assert_eq!(old_kernel[0].to_bits(), old.to_bits());
        assert_eq!(got[0].to_bits(), old_kernel[0].to_bits());

        let many = [
            16_777_216.0_f32,
            1.0,
            -16_777_216.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            16_777_216.0,
            1.0,
            -16_777_216.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ];
        let old = many.into_iter().reduce(|a, b| a + b).expect("inputs");
        let first = many[..8]
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .expect("group");
        let second = many[8..]
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .expect("group");
        let wrong_subtotal = first + second;
        assert_ne!(old.to_bits(), wrong_subtotal.to_bits());
        let inputs = many.map(|value| vec![value]);
        let old_kernel = old_reduce_case(1, &inputs);
        assert_eq!(old_kernel[0].to_bits(), old.to_bits());
        assert_eq!(
            reduce_case(1, &inputs)[0].to_bits(),
            old_kernel[0].to_bits()
        );

        // (b) Seeded corpora at several fan-ins, against the one-line scalar reference.
        let mut state = 0x6d69_736fu32;
        for count in [1usize, 2, 3, 5, 8, 9, 64, 65, 129] {
            for frames in [1usize, 7, 64, 128, 512] {
                let inputs: Vec<Vec<f32>> = (0..count)
                    .map(|_| (0..frames).map(|_| lcg(&mut state)).collect())
                    .collect();
                let reduced = reduce_case(frames, &inputs);
                for frame in 0..frames {
                    let reference = inputs
                        .iter()
                        .map(|input| input[frame])
                        .reduce(|a, b| a + b)
                        .unwrap_or(0.0);
                    assert_eq!(
                        reduced[frame].to_bits(),
                        reference.to_bits(),
                        "fan-in {count}, {frames} frames, frame {frame}"
                    );
                }
            }
        }

        // (c) Signed zero. `reduce` preserves it; `fold(0.0, +)` would not.
        assert_eq!(
            reduce_case(1, &[vec![-0.0f32]])[0].to_bits(),
            (-0.0f32).to_bits()
        );
        assert_eq!(
            reduce_case(1, &[vec![-0.0f32], vec![-0.0f32]])[0].to_bits(),
            (-0.0f32).to_bits()
        );
        assert_eq!(
            reduce_case(1, &[vec![-0.0f32], vec![0.0f32]])[0].to_bits(),
            0.0f32.to_bits()
        );
        // Fan-in zero is a zero fill, not a copy.
        assert_eq!(reduce_case(4, &[]), vec![0.0f32; 4]);
    }

    /// E5 continued: an in-place single input is the identity, not a copy through a scratch.
    #[test]
    fn a_single_in_place_input_is_left_untouched() {
        let mut lease = single_lease(2, 2);
        lease.write(0, 1).copy_from_slice(&[1.0, 2.0]);
        lease.write(0, 2).copy_from_slice(&[3.0, 4.0]);
        reduce_plane(&mut lease, 0, 2, &[2]);
        assert_eq!(lease.read(0, 1), &[1.0, 2.0]);
        assert_eq!(lease.read(0, 2), &[3.0, 4.0]);
    }

    /// E7. The two-segment slice PDC is bit-for-bit a per-sample ring delay, and the result does
    /// not depend on how the stream is partitioned into blocks (master plan #83 D5).
    ///
    /// Red mutations (`tests/MUTATIONS.md`): drop the cursor advance between segments; swap the
    /// two `pdc_delay_block` segments; carry one cursor across both channels instead of restarting
    /// the right channel at the block's cursor.
    #[test]
    fn compensation_delay_is_partition_invariant_and_matches_per_sample_reference() {
        const SAMPLES: usize = 4_096;
        let mut state = 0x0a05_1970u32;
        let signal: Vec<f32> = (0..SAMPLES).map(|_| lcg(&mut state)).collect();
        for delay_samples in [1usize, 3, 37, 127, 128, 129, 600] {
            // The independent oracle: a per-sample `VecDeque` ring, three lines, no slice algebra.
            let per_sample = |input: &[f32]| -> Vec<f32> {
                let mut ring: std::collections::VecDeque<f32> =
                    std::iter::repeat_n(0.0, delay_samples).collect();
                input
                    .iter()
                    .map(|sample| {
                        ring.push_back(*sample);
                        ring.pop_front().expect("non-empty ring")
                    })
                    .collect()
            };
            let negated: Vec<f32> = signal.iter().map(|sample| -sample).collect();
            let reference = per_sample(&signal);
            let reference_right = per_sample(&negated);
            let mut baseline: Option<Vec<u32>> = None;
            for block in [1usize, 7, 64, 128, 512] {
                let mut delay = CompensationDelay::new(delay_samples);
                assert_eq!(delay.samples(), delay_samples);
                let mut left = signal.clone();
                let mut right = negated.clone();
                for offset in (0..SAMPLES).step_by(block) {
                    let end = (offset + block).min(SAMPLES);
                    let (head, tail) = left.split_at_mut(end);
                    let _ = tail;
                    let (right_head, right_tail) = right.split_at_mut(end);
                    let _ = right_tail;
                    delay.process(&mut head[offset..], &mut right_head[offset..]);
                }
                for (index, sample) in left.iter().enumerate() {
                    assert_eq!(
                        sample.to_bits(),
                        reference[index].to_bits(),
                        "delay {delay_samples}, block {block}, sample {index}"
                    );
                    assert_eq!(right[index].to_bits(), reference_right[index].to_bits());
                }
                let bits: Vec<u32> = left.iter().map(|sample| sample.to_bits()).collect();
                match &baseline {
                    None => baseline = Some(bits),
                    Some(first) => assert_eq!(&bits, first, "partition invariance at {block}"),
                }
                delay.reset();
                let mut zero = vec![0.0f32; delay_samples];
                let mut zero_right = vec![0.0f32; delay_samples];
                delay.process(&mut zero, &mut zero_right);
                assert!(
                    zero.iter().all(|sample| *sample == 0.0),
                    "reset clears the line"
                );
            }
        }
    }

    /// E10. The route is two multiplies plus one add per output word, with the gain
    /// folded into the coefficients at bind (D3). The oracle is
    /// `softfma::unfused_multiply_add_via_f64`, which computes the same multiply-add through `f64` with
    /// no dependence on `mix2x2_block`'s vector body; the exact product and the innocuous double
    /// rounding of the sum make it bit-identical to the `f32` expression (issue #163 phase 2).
    ///
    /// Red mutation (`tests/MUTATIONS.md`): compute `gain * (ll * l + lr * r)` instead.
    #[test]
    fn route_applies_folded_gain_with_frozen_op_order() {
        const FRAMES: usize = 4_096;
        let transform = (0.707_9f32, 0.9f32, -0.1f32, 0.2f32, 0.8f32);
        let folded = [
            transform.0 * transform.1,
            transform.0 * transform.2,
            transform.0 * transform.3,
            transform.0 * transform.4,
        ];
        let mut state = 0x1234_5678u32;
        let left_in: Vec<f32> = (0..FRAMES).map(|_| lcg(&mut state)).collect();
        let right_in: Vec<f32> = (0..FRAMES).map(|_| lcg(&mut state)).collect();
        let mut left = left_in.clone();
        let mut right = right_in.clone();
        mix2x2_block::<FrameLane>(&mut left, &mut right, folded);
        for frame in 0..FRAMES {
            let (l, r) = (left_in[frame], right_in[frame]);
            let expected_left =
                lane::softfma::unfused_multiply_add_via_f64(folded[1], r, folded[0] * l);
            let expected_right =
                lane::softfma::unfused_multiply_add_via_f64(folded[3], r, folded[2] * l);
            assert_eq!(
                left[frame].to_bits(),
                expected_left.to_bits(),
                "frame {frame} left"
            );
            assert_eq!(
                right[frame].to_bits(),
                expected_right.to_bits(),
                "frame {frame} right"
            );
        }
        // Block lengths that are not a multiple of the lane width, so the scalar tail is walked
        // and has to agree with the vector body word for word.
        for block in [1usize, 3, 7, 63, 65, 129, 511] {
            let mut tail_left = left_in[..block].to_vec();
            let mut tail_right = right_in[..block].to_vec();
            mix2x2_block::<FrameLane>(&mut tail_left, &mut tail_right, folded);
            assert_eq!(
                (
                    tail_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
                    tail_right.iter().map(|s| s.to_bits()).collect::<Vec<_>>()
                ),
                (
                    left[..block]
                        .iter()
                        .map(|s| s.to_bits())
                        .collect::<Vec<_>>(),
                    right[..block]
                        .iter()
                        .map(|s| s.to_bits())
                        .collect::<Vec<_>>()
                ),
                "block {block}: the scalar tail must match the vector body"
            );
        }
        // The fold is not the unfolded form: this is the bit change #98 F4 declares.
        let unfolded: Vec<u32> = (0..FRAMES)
            .map(|frame| {
                (transform.0 * (transform.1 * left_in[frame] + transform.2 * right_in[frame]))
                    .to_bits()
            })
            .collect();
        assert_ne!(
            left.iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            unfolded,
            "the folded route is a distinct rounding, not a coincidence"
        );
    }
}
