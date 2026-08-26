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
//! Audio lives in one [`DisjointArena`](miso_engine_core::realtime::DisjointArena) per prepared
//! plan: two planar `f32` planes of `buffers * frames` words, reached only through a checked
//! [`ArenaLeaseV1`]. The lease API is the one implementation of *where the audio is* as well as
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
//!   render spends one multiply and one [`Lane::fma`](miso_engine_lane::Lane::fma) per output word
//!   through [`mix2x2_block`].
//! * Compensation delays are two-segment slice exchanges through [`pdc_delay_block`]; there is no
//!   per-sample work and no `%` on the render path.
//! * The graph performs **no** sanitisation (D7). Input sanitisation is the input stage's, output
//!   finiteness is the bank boundary check.

use std::collections::BTreeMap;

use core::num::NonZeroUsize;

use miso_engine_core::realtime::{ArenaLeaseSetBuilder, ArenaLeaseV1, RenderError};

/// The arena reserves buffer zero as the always-zero silence slot, so every executor buffer is
/// offset by one.
pub(crate) const ARENA_BASE: u32 = 1;
use miso_engine_effect_contract::{
    BypassShunt, EffectControlLane, EffectProcessBlock, ObservationLaneV1, ObservationSampleV1,
    PreparedAutomationSpan, PreparedNativeEffect,
};
use miso_engine_lane::kernels::{mix2x2_block, pdc_delay_block, sum_into_block, sum2_block};
use miso_engine_rack::{BankChain, BankMembers};

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
pub(crate) type FrameLane = miso_engine_lane::Simd8;
/// See [`FrameLane`].
#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
pub(crate) type FrameLane = miso_engine_lane::Simd4;
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
fn reduce_plane(lease: &mut ArenaLeaseV1, plane: usize, out: u32, inputs: &[u32]) {
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

/// What an op does to its reduced output.
pub(crate) enum NodeKind {
    /// A stage boundary, a submix or the session output: the reduction is the whole node.
    Identity,
    /// A track input filled by the coordinator's source set: no reduction, no processing.
    SourceInput,
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
    observation: Option<Box<ObservationLaneV1>>,
}

impl ConsoleEffect {
    fn new(
        effect: GraphPreparedEffect,
        control: Box<EffectControlLane>,
        observation: Option<Box<ObservationLaneV1>>,
        frames: usize,
    ) -> Self {
        let capacity = effect.metadata.automation_capacity as usize;
        let latency = usize::try_from(effect.metadata.latency.0).unwrap_or(usize::MAX);
        Self {
            observation,
            spans: vec![
                PreparedAutomationSpan {
                    kind: miso_engine_effect_contract::AutomationSpanKind::Point,
                    channel: miso_engine_effect_contract::ParameterChannel::Both,
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
    observation: &mut ObservationLaneV1,
    processor: &dyn PreparedNativeEffect,
    first_sample: u64,
    frames: u64,
) {
    if !observation.any_armed() {
        return;
    }
    let mut sample = ObservationSampleV1 {
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
                .map_or(0, ObservationLaneV1::retained_bytes),
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
    lease: &'a mut ArenaLeaseV1,
    inputs: &'a [u32],
    outputs: &'a [u32],
}

impl BankMembers for ArenaMembers<'_> {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        self.lease.read_stereo(self.inputs[lane])
    }
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
        self.lease.write_stereo(self.outputs[lane])
    }
}

// REALTIME_POLICY_END

/// Ops, their audio and their delay lines: everything one executor (or one native parcel) owns.
pub(crate) struct Runtime {
    /// This runtime's checked view of the plan's shared arena.
    pub(crate) lease: ArenaLeaseV1,
    pub(crate) delays: Box<[CompensationDelay]>,
    pub(crate) units: Box<[RuntimeUnit]>,
    /// Scratch for a bank chain's gather-source buffers, sized to the widest bank at bind.
    bank_inputs: Box<[u32]>,
    /// Scratch for a bank chain's scatter-target buffers, sized to the widest bank at bind.
    bank_outputs: Box<[u32]>,
}

impl Runtime {
    pub(crate) fn new(
        lease: ArenaLeaseV1,
        delays: Vec<CompensationDelay>,
        units: Vec<RuntimeUnit>,
    ) -> Self {
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
            units: units.into_boxed_slice(),
            bank_inputs: vec![0; widest].into_boxed_slice(),
            bank_outputs: vec![0; widest].into_boxed_slice(),
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
            units,
            bank_inputs,
            bank_outputs,
        } = self;
        let delays: &mut [CompensationDelay] = delays;
        match &mut units[index] {
            RuntimeUnit::Op(op) => execute_op(op, lease, delays, first_sample),
            RuntimeUnit::Bank {
                members,
                lanes,
                chain,
            } => {
                let lanes = *lanes;
                // Only the first slot reduces graph inputs; the chain computes the rest -- and a
                // member whose whole reduction *is* the dedication copy does not even do that:
                // the gather reads its producer's buffer directly. See `bank_gather_source`.
                for (lane, member) in members[..lanes].iter_mut().enumerate() {
                    if let Some(source) = bank_gather_source(member) {
                        bank_inputs[lane] = source;
                    } else {
                        execute_op(member, lease, delays, first_sample)?;
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
    lease: &mut ArenaLeaseV1,
    delays: &mut [CompensationDelay],
    first_sample: u64,
) -> Result<(), RenderError> {
    let output = op.output;
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
    reduce_plane(lease, 0, output, &op.inputs);
    reduce_plane(lease, 1, output, &op.inputs);
    match &mut op.kind {
        NodeKind::SourceInput | NodeKind::Identity | NodeKind::BankMember => {}
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

fn observe(op: &mut RuntimeOp, lease: &ArenaLeaseV1, first_sample: u64) -> Result<(), RenderError> {
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

use miso_engine_effect_contract::BankWidth;
use miso_engine_rack::{
    AoSoaScratch, BankBlock, BankSlot, BankStage, ConsoleEffectBankStage, EffectBankStage,
};

use crate::{
    GraphNodeBinding, GraphNodeId, GraphPreparedBuiltinBank, GraphPreparedBuiltinBankProcessor,
    GraphPreparedEffectBank, GraphSpec, PreparedRoute, RouteTransform,
    program::{ExecutionProgram, Op},
};

/// Adapter that lets a compiler-owned builtin bank act as a chain slot.
struct BuiltinStage(Box<dyn GraphPreparedBuiltinBankProcessor>);
impl BankStage for BuiltinStage {
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(block.left, block.right, block.frames, block.first_sample)
    }
    fn qualification_counters(&self) -> [u64; 2] {
        self.0.qualification_counters()
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
    effect_observations: BTreeMap<crate::EffectNodeId, Box<ObservationLaneV1>>,
    pub(crate) bindings: BTreeMap<GraphNodeId, Option<Box<dyn GraphRuntimeProcessor>>>,
    pub(crate) observers: BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
    pub(crate) source_inputs: std::collections::BTreeSet<GraphNodeId>,
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
        effect_controls: Vec<crate::GraphEffectControlBindingV1>,
        effect_observations: Vec<crate::GraphEffectObservationBindingV1>,
        banks: Vec<GraphPreparedEffectBank>,
        builtin_banks: Vec<GraphPreparedBuiltinBank>,
        observers: Vec<GraphNodeObserverBinding>,
        bindings: Vec<GraphNodeBinding>,
        source_inputs: std::collections::BTreeSet<GraphNodeId>,
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
            NodeKind::SourceInput
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
            NodeKind::Route([
                transform.gain * transform.ll,
                transform.gain * transform.lr,
                transform.gain * transform.rl,
                transform.gain * transform.rr,
            ])
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
        for membership in run {
            let (slot_scratch, slot_active, stage) = self.stage_for(*membership, members);
            if scratch.is_none() {
                scratch = Some(slot_scratch);
                active = Some(slot_active);
            }
            stages.push(stage);
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
                let observations: Vec<Option<ObservationLaneV1>> = (0..width.lanes() as usize)
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

    /// The cohort chain a bound effect bank belongs to, or `None` for a builtin bank.
    fn cohort_of(&self, membership: Membership) -> Option<crate::GraphBankCohortV1> {
        match membership {
            Membership::Effect(index) => self.banks[index].as_ref().map(|bank| bank.cohort),
            Membership::Builtin(_) => None,
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
    let mut units = Vec::with_capacity(runs.len());
    for run in runs {
        let membership: Vec<Membership> =
            run.iter().filter_map(|index| grouped[*index].0).collect();
        let ops: Vec<usize> = run
            .iter()
            .flat_map(|index| grouped[*index].1.iter().copied())
            .collect();
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
    let mut builder = ArenaLeaseSetBuilder::new(
        NonZeroUsize::new(2).expect("stereo planes"),
        NonZeroUsize::new(frames.max(1)).expect("nonzero frames"),
    );
    let buffers: Vec<u32> = (0..program.buffers).map(|_| builder.reserve()).collect();
    builder.lease(0, buffers.clone(), buffers);
    let (_arena, mut leases) = builder
        .finish()
        .expect("one lease over one coloured arena is disjoint by construction");
    Runtime::new(leases.pop().expect("the sequential lease"), delays, units)
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
/// two may be rendered as consecutive slots of one chain (issue #181).
///
/// The merge replaces two planar/AoSoA round-trips with one: the chain gathers `earlier`'s member
/// outputs, runs both stages over the resident block, and scatters into `later`'s. The price is
/// that `earlier`'s output buffers are left holding the *chain's input* rather than the first
/// stage's output, and `later`'s ops never reduce. Every clause below is one way that could be
/// observed, and any one of them declines the merge:
///
/// * **Lane count.** The two slots must cover the same lanes, one op each.
/// * **`later` reads only `earlier`, undelayed and unmixed.** One main input, no sidechain, no
///   compensation-delay staging -- otherwise skipping `later`'s reduction would drop a summand or
///   a delay line. A sidechained slot already blocks banking (#96 F9); this re-checks it on the
///   lowered program rather than trusting the planner.
/// * **Nothing else reads `earlier`.** Exactly one reader, and it is `later`'s op. A send tap, a
///   meter or a second consumer would read the pre-stage signal.
/// * **No alias, no observer, and not the session output.** A `program::Tap` aliases an elided
///   node onto `earlier`'s buffer; an observer bound to `earlier` fires after the unit and would
///   see the chain's input; and the session output is read by the host.
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
        if producer.output == program.output
            || program
                .taps
                .iter()
                .any(|tap| tap.after_op as usize == *before)
        {
            return false;
        }
        let node = &spec.nodes[producer.node as usize].id;
        if parts.observers.contains_key(node) {
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
/// Two bank units are candidates when the cohort planner put them in the same group -- that is
/// the only place the "same lanes, consecutive slots of one rack chain" fact exists -- and the
/// merge happens only when [`chains_into`] can also prove it on the lowered program. The planner
/// knows the session's shape; the program knows what the colouring and the taps did with it, and
/// both have to agree.
fn cohort_runs(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    units: &[PlannedUnit],
) -> Vec<Vec<usize>> {
    let (readers, first_producer) = op_dataflow(program);
    // Candidate successors: within one cohort group, the next bound slot.
    let mut by_group: BTreeMap<u32, Vec<(u32, usize)>> = BTreeMap::new();
    for (index, (membership, _)) in units.iter().enumerate() {
        if let Some(cohort) = membership.and_then(|value| parts.cohort_of(value)) {
            by_group
                .entry(cohort.group)
                .or_default()
                .push((cohort.slot, index));
        }
    }
    let mut successor: BTreeMap<usize, usize> = BTreeMap::new();
    for slots in by_group.values_mut() {
        slots.sort_unstable();
        for pair in slots.windows(2) {
            let (earlier, later) = (pair[0].1, pair[1].1);
            if chains_into(
                program,
                spec,
                parts,
                &readers,
                &first_producer,
                &units[earlier].1,
                &units[later].1,
            ) {
                successor.insert(earlier, later);
            }
        }
    }
    let merged: std::collections::BTreeSet<usize> = successor.values().copied().collect();
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

    /// One lease that owns `buffers` buffers of one plane, as the sequential executor does.
    fn single_lease(frames: usize, buffers: usize) -> ArenaLeaseV1 {
        let mut builder = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(1).expect("one plane"),
            NonZeroUsize::new(frames).expect("frames"),
        );
        let owned: Vec<u32> = (0..buffers).map(|_| builder.reserve()).collect();
        builder.lease(0, owned.clone(), owned);
        let (_arena, mut leases) = builder.finish().expect("one disjoint lease");
        leases.pop().expect("the lease")
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

        // (b) Seeded corpora at several fan-ins, against the one-line scalar reference.
        let mut state = 0x6d69_736fu32;
        for count in [1usize, 2, 3, 5, 9, 64] {
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
            let expected_left = miso_engine_lane::softfma::unfused_multiply_add_via_f64(
                folded[1],
                r,
                folded[0] * l,
            );
            let expected_right = miso_engine_lane::softfma::unfused_multiply_add_via_f64(
                folded[3],
                r,
                folded[2] * l,
            );
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
