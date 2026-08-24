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
//! [`ArenaLeaseV1`]. Both executors use the same lease API, so node semantics have exactly one
//! implementation of *where the audio is* as well as of what happens to it.
//!
//! The sequential executor holds a single lease over the whole coloured arena. The native
//! dependency-wave executor gives every parcel its own lease over the *same* arena: each op's
//! output has a globally unique buffer for the life of the plan, so a consumer reads its
//! producers' buffers in place, on the worker that needs them, instead of waiting for the
//! coordinator to copy every inter-parcel edge between waves (#100 F8, #98's F8 hand-off). Only
//! a delayed edge still copies, and that copy is made by the *consuming* parcel through the same
//! [`RuntimeOp::staged`] list the sequential executor uses.
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
use miso_engine_effect_contract::EffectProcessBlock;
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
    /// A route's 2x2 matrix, with the route gain already folded in (D3).
    Route([f32; 4]),
    /// A homogeneous-bank member: the reduction gathers its input, the bank does the work.
    BankMember,
}

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
        members: Box<[RuntimeOp]>,
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
}

// REALTIME_POLICY_BEGIN
/// Planar per-lane view over the arena slots a bank's members own.
struct ArenaMembers<'a> {
    lease: &'a mut ArenaLeaseV1,
    outputs: &'a [u32],
}

impl BankMembers for ArenaMembers<'_> {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        self.lease.read_stereo(self.outputs[lane])
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
    /// Scratch for a bank's member output buffers, sized to the widest bank at bind.
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
                RuntimeUnit::Bank { members, .. } => members.len(),
            })
            .max()
            .unwrap_or(0);
        Self {
            lease,
            delays: delays.into_boxed_slice(),
            units: units.into_boxed_slice(),
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
            bank_outputs,
        } = self;
        let delays: &mut [CompensationDelay] = delays;
        match &mut units[index] {
            RuntimeUnit::Op(op) => execute_op(op, lease, delays, first_sample),
            RuntimeUnit::Bank { members, chain } => {
                for member in members.iter_mut() {
                    execute_op(member, lease, delays, first_sample)?;
                }
                for (lane, member) in members.iter().enumerate() {
                    bank_outputs[lane] = member.output;
                }
                let frames = lease.frames();
                chain.run(
                    &mut ArenaMembers {
                        lease,
                        outputs: &bank_outputs[..members.len()],
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
use miso_engine_rack::{AoSoaScratch, BankBlock, BankSlot, BankStage, EffectBankStage};

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

fn bank_chain(scratch: AoSoaScratch, active: Box<[bool]>, stage: Box<dyn BankStage>) -> BankChain {
    BankChain::new(
        scratch,
        active.clone(),
        vec![BankSlot {
            stage,
            active_lanes: active,
        }],
    )
    .expect("validated bank shape")
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

/// One dependency wave before it is built: its level and its units, in stable key order.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type PlannedWave = (u64, Vec<PlannedUnit>);

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
    pub(crate) bindings: BTreeMap<GraphNodeId, Option<Box<dyn GraphRuntimeProcessor>>>,
    pub(crate) observers: BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
    pub(crate) source_inputs: std::collections::BTreeSet<GraphNodeId>,
    banks: Vec<Option<GraphPreparedEffectBank>>,
    builtin_banks: Vec<Option<GraphPreparedBuiltinBank>>,
    membership: BankMembership,
}

impl RuntimeParts {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        spec: &GraphSpec,
        routes: Vec<PreparedRoute>,
        effects: Vec<GraphPreparedEffect>,
        banks: Vec<GraphPreparedEffectBank>,
        builtin_banks: Vec<GraphPreparedBuiltinBank>,
        observers: Vec<GraphNodeObserverBinding>,
        bindings: Vec<GraphNodeBinding>,
        source_inputs: std::collections::BTreeSet<GraphNodeId>,
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
            bindings: bindings
                .into_iter()
                .map(|binding| (binding.node, binding.processor))
                .collect(),
            observers: by_node,
            source_inputs,
            banks: banks.into_iter().map(Some).collect(),
            builtin_banks: builtin_banks.into_iter().map(Some).collect(),
            membership,
        }
    }

    /// The one node-kind decision, shared by both executors.
    ///
    /// A route's linear gain is folded into its 2x2 coefficients here, once, at bind: render then
    /// spends one multiply and one fused multiply-add per output word instead of re-applying the
    /// gain every frame (D3, #98 F4).
    fn node_kind(&mut self, node: &GraphNodeId, index: u32) -> NodeKind {
        if self.source_inputs.contains(node) {
            NodeKind::SourceInput
        } else if self.membership.contains_key(&index) {
            NodeKind::BankMember
        } else if let Some(Some(processor)) = self.bindings.remove(node) {
            NodeKind::Bound(processor)
        } else if let Some(effect) = self.effects.remove(node) {
            NodeKind::Effect(effect)
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

    fn chain_for(&mut self, membership: Membership, members: usize) -> BankChain {
        match membership {
            Membership::Effect(index) => {
                let bank = self.banks[index].take().expect("one effect bank owner");
                let width = bank.scratch.width();
                let quantum = bank.scratch.quantum();
                let stage =
                    EffectBankStage::new(bank.processor, width, quantum).expect("validated width");
                bank_chain(bank.scratch, bank.active_mask, Box::new(stage))
            }
            Membership::Builtin(index) => {
                let bank = self.builtin_banks[index]
                    .take()
                    .expect("one builtin bank owner");
                let active = trailing_active_mask(members, bank.scratch.width());
                bank_chain(bank.scratch, active, Box::new(BuiltinStage(bank.processor)))
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

#[cfg(not(target_arch = "wasm32"))]
/// The op that produces node `index`'s audio: its own op, or -- for an elided stage boundary --
/// the op whose output buffer it aliases.
fn producing_op(program: &ExecutionProgram, index: u32) -> Option<u32> {
    program.node_op[index as usize].or_else(|| {
        program
            .taps
            .iter()
            .find(|tap| tap.node == index)
            .map(|tap| tap.after_op)
    })
}

/// The op that produced each of an op's inputs, recovered from the lowering's own colouring.
///
/// The native executor gives every op output a globally unique arena buffer, so it has to map the
/// lowering's *coloured* input buffer back to the op that wrote it. Walking `program.ops` in
/// schedule order and remembering the last writer of each colour does exactly that: liveness
/// colouring never reassigns a colour while a consumer still needs it, so the last writer at the
/// moment a consumer is reached is that consumer's producer. This reuses #98/#99's colouring
/// rather than forming a second opinion from the semantic graph.
#[cfg(not(target_arch = "wasm32"))]
struct OpProducers {
    main: Vec<Vec<usize>>,
    sidechain: Vec<Option<usize>>,
}

#[cfg(not(target_arch = "wasm32"))]
fn op_producers(program: &ExecutionProgram) -> OpProducers {
    let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
    let mut main = Vec::with_capacity(program.ops.len());
    let mut sidechain = Vec::with_capacity(program.ops.len());
    for (index, op) in program.ops.iter().enumerate() {
        main.push(
            program
                .inputs_of(op)
                .iter()
                .map(|input| {
                    owner[input.buffer.0 as usize].expect("every input has a producing op")
                })
                .collect(),
        );
        sidechain.push(op.sidechain.map(|side| {
            owner[side.buffer.0 as usize].expect("every sidechain has a producing op")
        }));
        owner[op.output.0 as usize] = Some(index);
    }
    OpProducers { main, sidechain }
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
    let mut units = Vec::with_capacity(grouped.len());
    for (membership, ops) in grouped {
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
        units.push(finish_unit(&mut parts, membership, members));
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
    membership: Option<Membership>,
    mut members: Vec<RuntimeOp>,
) -> RuntimeUnit {
    match membership {
        None => RuntimeUnit::Op(members.pop().expect("one op per plain unit")),
        Some(membership) => {
            let chain = parts.chain_for(membership, members.len());
            RuntimeUnit::Bank {
                members: members.into_boxed_slice(),
                chain,
            }
        }
    }
}

/// Where one op's output lives in the native executor: which wave, which parcel, which buffer.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NodeLocation {
    pub(crate) wave: usize,
    pub(crate) partition: usize,
    pub(crate) buffer: u32,
}

/// One consumer read that must fall back to silence while its producing partition is trapped.
///
/// After a worker misses its deadline the coordinator does not own its parcel any more, so its
/// output buffers are radioactive: nothing may read them and nothing may write them until the
/// worker finally returns the parcel. The executor mutes exactly these reads instead (arena
/// invariant I4) and un-mutes them when the parcel is reaped.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MutedRead {
    pub(crate) wave: usize,
    pub(crate) partition: usize,
    pub(crate) buffer: u32,
}

/// The partitioning of one wave, chosen by the cost-weighted split at bind.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct WaveLayout {
    /// Unit indices of the wave, in the order the partitions cover them.
    pub(crate) unit_order: Vec<usize>,
    /// `(first_unit, end_unit, partition_id)` over `unit_order`.
    pub(crate) ranges: Vec<(usize, usize, usize)>,
}

/// The native executor's layout: one shared arena, one lease per parcel, no coordinator copies.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct NativeRuntime {
    pub(crate) levels: Vec<u64>,
    pub(crate) parcels: Vec<Vec<Runtime>>,
    /// Per `(wave, partition)`, every consumer read to mute while that partition is trapped.
    pub(crate) trapped_edges: Vec<Vec<Vec<MutedRead>>>,
    pub(crate) locations: BTreeMap<GraphNodeId, NodeLocation>,
    pub(crate) output: NodeLocation,
}

/// Units of the program grouped into waves: one entry per non-empty dependency level.
///
/// A level whose nodes are all elided stage boundaries produces no op and therefore no wave; the
/// remaining levels keep their order, so `source.wave < destination.wave` still holds for every
/// edge.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn waves_of(
    program: &ExecutionProgram,
    membership_of: &BankMembership,
) -> Vec<PlannedWave> {
    let mut waves: Vec<PlannedWave> = Vec::new();
    for (membership, ops) in units_of(program, membership_of) {
        let level = program.ops[ops[0]].level;
        match waves.last_mut() {
            Some((last, units)) if *last == level => units.push((membership, ops)),
            _ => waves.push((level, vec![(membership, ops)])),
        }
    }
    waves
}

/// Builds the native executor's runtime: one plan-wide disjoint arena, destination pulls.
///
/// Every op output gets a buffer that no other lease may ever write (arena invariant I1), so a
/// consumer names its producers' buffers directly and reads them in place on its own worker. The
/// coordinator copies nothing between waves; only a delayed edge still copies, into a staging
/// buffer the consuming parcel owns and stages itself.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn build_native(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    mut parts: RuntimeParts,
    frames: usize,
    layout: &[WaveLayout],
) -> NativeRuntime {
    let taps = taps_by_op(program, spec);
    let producers = op_producers(program);
    let waves = waves_of(program, &parts.membership.clone());
    let mut builder = ArenaLeaseSetBuilder::new(
        NonZeroUsize::new(2).expect("stereo planes"),
        NonZeroUsize::new(frames.max(1)).expect("nonzero frames"),
    );
    let mut op_arena_output = vec![0_u32; program.ops.len()];
    let mut op_location: Vec<Option<NodeLocation>> = vec![None; program.ops.len()];
    let mut locations = BTreeMap::new();
    let mut levels = Vec::with_capacity(waves.len());
    let mut built: Vec<Vec<(Vec<CompensationDelay>, Vec<RuntimeUnit>)>> =
        Vec::with_capacity(waves.len());
    let mut trapped_edges: Vec<Vec<Vec<MutedRead>>> = Vec::with_capacity(waves.len());
    // Arena buffer -> the partition that writes it, so a trapped partition's consumers are known.
    let mut buffer_owner: Vec<(usize, usize)> = vec![(0, 0)];
    for (wave_index, (level, units)) in waves.iter().enumerate() {
        levels.push(*level);
        let wave_layout = &layout[wave_index];
        let mut wave_built = Vec::with_capacity(wave_layout.ranges.len());
        trapped_edges.push(vec![Vec::new(); wave_layout.ranges.len()]);
        for (first_unit, end_unit, partition) in &wave_layout.ranges {
            let mut delays: Vec<CompensationDelay> = Vec::new();
            let mut writes: Vec<u32> = Vec::new();
            let mut reads: Vec<u32> = Vec::new();
            let mut members_built = Vec::with_capacity(end_unit - first_unit);
            for ordered in &wave_layout.unit_order[*first_unit..*end_unit] {
                let (membership, ops) = &units[*ordered];
                let mut members = Vec::with_capacity(ops.len());
                for op_index in ops {
                    let op = &program.ops[*op_index];
                    let node = op.node as usize;
                    let mut inputs = Vec::with_capacity(op.input_count() as usize);
                    let mut staged = Vec::new();
                    let mut pull = |source_op: usize,
                                    delay: Option<crate::program::DelayRef>,
                                    delays: &mut Vec<CompensationDelay>,
                                    writes: &mut Vec<u32>,
                                    reads: &mut Vec<u32>,
                                    staged: &mut Vec<StagedInput>|
                     -> u32 {
                        let source = op_arena_output[source_op];
                        reads.push(source);
                        match delay {
                            None => source,
                            Some(delay) => {
                                let staging = builder.reserve();
                                writes.push(staging);
                                delays.push(CompensationDelay::new(
                                    program.delays[delay.line as usize].samples as usize,
                                ));
                                staged.push(StagedInput {
                                    source,
                                    staging,
                                    line: u32::try_from(delays.len() - 1).expect("delay index"),
                                });
                                staging
                            }
                        }
                    };
                    for (position, input) in program.inputs_of(op).iter().enumerate() {
                        let buffer = pull(
                            producers.main[*op_index][position],
                            input.delay,
                            &mut delays,
                            &mut writes,
                            &mut reads,
                            &mut staged,
                        );
                        inputs.push(buffer);
                    }
                    let sidechain = op.sidechain.map(|side| {
                        pull(
                            producers.sidechain[*op_index].expect("sidechain source"),
                            side.delay,
                            &mut delays,
                            &mut writes,
                            &mut reads,
                            &mut staged,
                        )
                    });
                    let output = builder.reserve();
                    writes.push(output);
                    if buffer_owner.len() <= output as usize {
                        // Staging buffers may have been reserved in between; they are read only
                        // by the parcel that owns them, so their entry is never consulted.
                        buffer_owner.resize(output as usize + 1, (0, 0));
                    }
                    buffer_owner[output as usize] = (wave_index, *partition);
                    op_arena_output[*op_index] = output;
                    let location = NodeLocation {
                        wave: wave_index,
                        partition: *partition,
                        buffer: output,
                    };
                    op_location[*op_index] = Some(location);
                    locations.insert(spec.nodes[node].id.clone(), location);
                    members.push(build_op(
                        &mut parts,
                        spec,
                        op,
                        inputs,
                        staged,
                        sidechain,
                        output,
                        taps.get(&u32::try_from(*op_index).expect("op index"))
                            .map(Vec::as_slice),
                    ));
                }
                members_built.push(finish_unit(&mut parts, *membership, members));
            }
            // Every read of a buffer another partition owns is a mute candidate. Partition zero is
            // the coordinator's own lane and can never be trapped, so it is never a source here.
            for source in &reads {
                let (producer_wave, producer_partition) = buffer_owner[*source as usize];
                if producer_partition == 0
                    || (producer_wave == wave_index && producer_partition == *partition)
                {
                    continue;
                }
                trapped_edges[producer_wave][producer_partition].push(MutedRead {
                    wave: wave_index,
                    partition: *partition,
                    buffer: *source,
                });
            }
            builder.lease(wave_index, writes, reads);
            wave_built.push((delays, members_built));
        }
        built.push(wave_built);
    }
    let (_arena, leases) = builder
        .finish()
        .expect("unique per-op buffers and strictly earlier reads are disjoint by construction");
    let mut leases = leases.into_iter();
    let parcels = built
        .into_iter()
        .map(|wave| {
            wave.into_iter()
                .map(|(delays, units)| {
                    Runtime::new(leases.next().expect("one lease per parcel"), delays, units)
                })
                .collect()
        })
        .collect();
    let output_node = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
        .expect("validated single output");
    let output = op_location[producing_op(program, u32::try_from(output_node).expect("node index"))
        .expect("output op") as usize]
        .expect("output located");
    NativeRuntime {
        levels,
        parcels,
        trapped_edges,
        locations,
        output,
    }
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

    /// E10. The route is one multiply plus one fused multiply-add per output word, with the gain
    /// folded into the coefficients at bind (D3). The oracle is `softfma::fma_f32_via_f64`, the
    /// lane crate's exact software FMA -- one `f64` product and one correctly rounded narrowing,
    /// with no dependence on `mix2x2_block`'s vector body, and proven bit-identical to hardware
    /// FMA on every backend by master plan gate G3.
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
                miso_engine_lane::softfma::fma_f32_via_f64(folded[1], r, folded[0] * l);
            let expected_right =
                miso_engine_lane::softfma::fma_f32_via_f64(folded[3], r, folded[2] * l);
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
