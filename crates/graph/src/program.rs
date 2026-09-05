//! The executable form of a prepared graph: an [`ExecutionProgram`].
//!
//! `GraphSpec` is a *semantic* graph. It materialises seven `TrackStage` nodes per track and one
//! edge per hop because that is the vocabulary sends, taps, PDC and diagnostics are expressed in.
//! It is not a schedule. Executed literally -- one schedule item per node, one contribution buffer
//! per edge, one pairwise reduction per node -- a track with no effects costs seven node visits
//! and six full buffer copies per block to move its input to its output unchanged (#99 F2).
//!
//! Lowering keeps the semantic graph exactly as it is and *derives* an executable program from it:
//!
//! * node ids are interned to [`NodeIndex`], so nothing on the render path compares strings;
//! * a `TrackStage` boundary that is a pure pass-through becomes a **buffer alias** ([`Tap`]) with
//!   no schedule item at all -- observers still attach to it, immediately after the op that last
//!   wrote the buffer;
//! * a consumer with exactly one undelayed input reads its producer's buffer **in place**, so the
//!   copy disappears;
//! * a `Sum` -- the per-frame `balanced_pairwise_sum` -- exists only where fan-in is genuinely
//!   greater than one;
//! * buffers are liveness-coloured over *ops*, so the arena is proportional to the graph's live
//!   width rather than to its edge count.
//!
//! [`lower`] is a pure function of `(spec, schedule, levels, delays)`: the program cannot disagree
//! with the semantic graph, because it is computed from it. Everything is derived from sorted
//! inputs and is deterministic; nothing here reads a clock, a CPU or an environment.
//!
//! ## What lowering must not change
//!
//! Reduction shape is frozen by master plan #83 D9: pairwise, left-to-right, in stable edge-ID
//! order, the same tree in both executors. Lowering preserves it exactly -- `Op::inputs` is a
//! half-open range into [`ExecutionProgram::inputs`] filled in `spec.edges` order, which is sorted
//! by `GraphEdgeId`. A single-input "reduction" was already the identity (`values[0]`), so folding
//! it into an in-place read is bit-preserving by construction, not by tolerance.

use std::collections::BTreeMap;

use crate::{
    DependencyLevel, GraphEdgeId, GraphNodeId, GraphPortKind, GraphSpec, InsertedDelay, RackId,
    TrackStage,
};

/// Position of a node in `GraphSpec::nodes`, which is sorted by `GraphNodeId`.
pub type NodeIndex = u32;
/// Position of an op in [`ExecutionProgram::ops`].
pub type OpIndex = u32;

/// A physical audio buffer in the executor's arena.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BufferRef(pub u32);

/// A compensation delay applied to one input on the way into its consumer.
///
/// `line` indexes [`ExecutionProgram::delays`]; `staging` is the buffer the delayed copy lands in,
/// which is live only for the duration of the op that reads it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DelayRef {
    pub line: u32,
    pub staging: BufferRef,
}

/// One input of an op: where to read it, and whether it is delayed on the way in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputRef {
    pub buffer: BufferRef,
    pub delay: Option<DelayRef>,
}

/// A compensation delay line, in samples.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DelaySpec {
    pub samples: u64,
}

/// One executable step.
///
/// `inputs` is a half-open range into [`ExecutionProgram::inputs`], in stable edge-ID order (D9).
/// `in_place` means the op's single input already lives in `output` and must not be copied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Op {
    pub node: NodeIndex,
    pub level: u64,
    pub inputs: (u32, u32),
    pub sidechain: Option<InputRef>,
    pub output: BufferRef,
    pub in_place: bool,
}

impl Op {
    /// Number of main inputs. A `Sum` is needed only when this is greater than one.
    #[must_use]
    pub const fn input_count(&self) -> u32 {
        self.inputs.1 - self.inputs.0
    }
}

/// An elided node: it has no op, and its output is an alias of `buffer`.
///
/// `after_op` is the op that last wrote `buffer` before any consumer reads it. An observer bound
/// to an elided node fires there -- immediately after that op's own observers and before the next
/// op runs. Attaching it to the *consumer* instead would let it observe mutated data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tap {
    pub node: NodeIndex,
    pub buffer: BufferRef,
    pub after_op: OpIndex,
}

/// The executable form of a prepared graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionProgram {
    /// Level-major, node-id-sorted within a level: the sequential schedule minus elided nodes.
    pub ops: Box<[Op]>,
    pub inputs: Box<[InputRef]>,
    pub delays: Box<[DelaySpec]>,
    /// `NodeIndex` -> the buffer carrying that node's output.
    pub node_buffer: Box<[BufferRef]>,
    /// `NodeIndex` -> its op, or `None` when the node was elided into an alias.
    pub node_op: Box<[Option<OpIndex>]>,
    /// One entry per elided node.
    pub taps: Box<[Tap]>,
    /// Size of the coloured arena.
    pub buffers: u32,
    /// Buffer carrying the sole session output.
    pub output: BufferRef,
}

impl ExecutionProgram {
    /// Main inputs of one op, in stable edge-ID order.
    #[must_use]
    pub fn inputs_of(&self, op: &Op) -> &[InputRef] {
        &self.inputs[op.inputs.0 as usize..op.inputs.1 as usize]
    }
    /// Ops that still need a per-frame pairwise reduction: fan-in greater than one.
    #[must_use]
    pub fn reduction_count(&self) -> usize {
        self.ops.iter().filter(|op| op.input_count() > 1).count()
    }
    /// Inputs that carry a compensation delay, and therefore a staging buffer.
    #[must_use]
    pub fn delayed_input_count(&self) -> usize {
        self.inputs
            .iter()
            .filter(|input| input.delay.is_some())
            .count()
            + self
                .ops
                .iter()
                .filter(|op| op.sidechain.is_some_and(|side| side.delay.is_some()))
                .count()
    }
}

/// Why a spec could not be lowered. Every variant is an internal invariant violation: the compiler
/// produced these inputs itself, so a caller maps them to `graph.internal.invariant`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramError {
    /// An id in the schedule, the levels or an edge endpoint is not in `spec.nodes`.
    UnknownNode,
    /// `spec.nodes` is not sorted by id, so interning by binary search is not valid.
    SpecUnsorted,
    /// The schedule is not the concatenation of the levels, or is not a permutation of the nodes.
    ScheduleMismatch,
    /// An edge runs backwards in the schedule.
    NotTopological,
    /// A count did not fit its index type.
    Overflow,
}

/// Intern one node id. `spec.nodes` is sorted by id, so this is a binary search.
///
/// # Errors
/// Returns `None` when the id is not a node of this spec.
#[must_use]
pub fn node_index(spec: &GraphSpec, id: &GraphNodeId) -> Option<NodeIndex> {
    spec.nodes
        .binary_search_by(|node| node.id.cmp(id))
        .ok()
        .and_then(|index| u32::try_from(index).ok())
}

/// A stage boundary that carries signal without ever changing it.
///
/// These three are the *internal* rack boundaries. They are never effects, never bank members and
/// never appear in `required_bindings`, so nothing can bind a processor to them; their only role
/// is to be a stable observation and send-tap point. The other four stages
/// (`Input`, `PostInputBuiltins`, `PostFader`, `PostMatrix`) are all bindable and keep their ops.
const fn is_alias_candidate(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::TrackStage {
            stage: TrackStage::PostSimd1 | TrackStage::PostDynamic | TrackStage::PostSimd2PreFader,
            ..
        }
    )
}

/// A node whose output buffer is never returned to the free list.
///
/// The SIMD-rack effects and the post-input builtin stage, unchanged since #99: the same rule
/// `GraphExecutor::new` once hard-coded by re-buffering members after colouring. It is a
/// conservative classification by node *kind*, and it costs whatever it costs -- a dedicated
/// buffer cannot be consumed in place, so its consumer pays a copy.
///
/// It is **not** what makes a homogeneous bank safe, despite predating banks and looking like it
/// should be. A bank's hazard is a *window*, not a node; the window is handled by
/// [`bank_windows`], and [`lower`] records why extending this predicate to bank members was
/// measured and rejected (issue #169).
const fn is_dedicated(node: &GraphNodeId) -> bool {
    match node {
        GraphNodeId::Effect(id) => !matches!(id.rack, RackId::Dynamic),
        GraphNodeId::TrackStage { stage, .. } => matches!(stage, TrackStage::PostInputBuiltins),
        _ => false,
    }
}

/// The node whose storage a node's first main input reads, resolved through elided aliases.
///
/// This is the lowered program's `first_producer` stated on the semantic graph: the buffer an op's
/// first main input names is written by the op of the node this returns, because an elided stage
/// boundary owns no storage of its own and an in-place op keeps its producer's colour under its
/// own ownership. `runtime::op_dataflow` derives the same edge from the colouring; the two have to
/// agree, and `chainable_bank_groups` is the only place the lowering needs the answer before the
/// colouring exists.
fn first_main_producer(
    spec: &GraphSpec,
    elided: &[bool],
    main_in: &[Vec<usize>],
    node: usize,
) -> Option<usize> {
    let edge = *main_in.get(node)?.first()?;
    let mut source = node_index(spec, &spec.edges[edge].source.node)? as usize;
    while elided[source] {
        source = node_index(spec, &spec.edges[*main_in[source].first()?].source.node)? as usize;
    }
    Some(source)
}

/// Bank member lists with every pair of banks a cohort chain may fuse unioned into one entry.
///
/// A merged run renders as **one unit at its first slot's op position** (`runtime::cohort_runs`),
/// so the op range the schedule is permuted over is the union of every slot's ops -- not each
/// slot's own range. `bank_windows` needs that union, and this is where it is formed.
///
/// The pairing condition is exactly `runtime::chains_into`'s first clause, restated on the
/// semantic graph: two banks may fuse only when they cover the same number of lanes and, for
/// **every** lane `i`, the later bank's lane `i` reads the earlier bank's lane `i`. Everything
/// else `chains_into` demands -- sole readership, no observed alias, no sidechain, no delay, not
/// the session output -- can only *decline* a merge, and a window wider than the permutation that
/// actually happens is always safe. So this is a superset of what the runtime will do, computed
/// without the runtime's bindings, which the lowering does not have.
///
/// It is deliberately not "every bank connected to another bank by any edge". That coarser union
/// would fold whole racks of unrelated cohorts into one span on sessions where nothing can fuse,
/// and every op inside a span is an op whose physical slot may not be recycled: an over-wide
/// window is sound but it costs arena buffers, and this keeps the cost proportional to the merges
/// the runtime can actually take.
fn chainable_bank_groups(
    banks: &[Vec<GraphNodeId>],
    spec: &GraphSpec,
    elided: &[bool],
    main_in: &[Vec<usize>],
) -> Vec<Vec<GraphNodeId>> {
    // A bank with an id that is not a node of this spec pairs with nothing: the lane alignment
    // below is positional, so a silently dropped member would compare the wrong lanes. Such an
    // id is already documented as ignorable ("a stale id can only widen a window"), and leaving
    // its bank unpaired keeps that true.
    let interned: Vec<Option<Vec<usize>>> = banks
        .iter()
        .map(|members| {
            members
                .iter()
                .map(|id| node_index(spec, id).map(|index| index as usize))
                .collect::<Option<Vec<usize>>>()
        })
        .collect();
    let mut bank_lane: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for (bank, members) in interned.iter().enumerate() {
        for (lane, node) in members.iter().flatten().enumerate() {
            bank_lane.insert(*node, (bank, lane));
        }
    }
    let producer = |node: usize| first_main_producer(spec, elided, main_in, node);
    let mut parent: Vec<usize> = (0..banks.len()).collect();
    fn root(parent: &mut [usize], mut bank: usize) -> usize {
        while parent[bank] != bank {
            parent[bank] = parent[parent[bank]];
            bank = parent[bank];
        }
        bank
    }
    for (later, members) in interned.iter().enumerate() {
        let Some(members) = members else { continue };
        // The candidate predecessor is whichever bank owns lane 0's producer; a bank owns each of
        // its members exactly once, so there is at most one candidate and no search.
        let Some(first) = members.first().copied().and_then(producer) else {
            continue;
        };
        let Some((earlier, lane)) = bank_lane.get(&first).copied() else {
            continue;
        };
        let Some(Some(before)) = interned.get(earlier) else {
            continue;
        };
        if lane != 0 || earlier == later || before.len() != members.len() {
            continue;
        }
        if !members
            .iter()
            .zip(before.iter())
            .all(|(after, before)| producer(*after) == Some(*before))
        {
            continue;
        }
        let (a, b) = (root(&mut parent, earlier), root(&mut parent, later));
        parent[a] = b;
    }
    let mut grouped: BTreeMap<usize, Vec<GraphNodeId>> = BTreeMap::new();
    for (bank, members) in banks.iter().enumerate() {
        let group = root(&mut parent, bank);
        grouped
            .entry(group)
            .or_default()
            .extend(members.iter().cloned());
    }
    grouped.into_values().collect()
}

/// The op ranges over which a bank reorders the schedule, merged into disjoint spans.
///
/// A bank's window runs from its first member's op to its last: `runtime::units_of` emits the
/// whole bank at the first position, so every op in between executes in some other order than the
/// one colouring saw. A bank with fewer than two ops reorders nothing and contributes no window.
///
/// Overlapping windows are merged, because two banks that interleave at one level reorder each
/// other's ops as well as their own; the merged span is the range over which no slot may be
/// recycled. Returns `(defer_release, closes_here)`, both indexed by op:
///
/// * `defer_release[o]` -- a slot whose last reader is op `o` must be *held*, not freed, because
///   some op in the same span still executes after `o` does;
/// * `closes_here[o]` -- op `o` is the last op of a span, so everything held may be released once
///   `o` is behind us.
fn bank_windows(
    node_op: &[Option<OpIndex>],
    banks: &[Vec<GraphNodeId>],
    spec: &GraphSpec,
    op_count: usize,
) -> (Vec<bool>, Vec<bool>) {
    let mut spans: Vec<(usize, usize)> = Vec::with_capacity(banks.len());
    for bank in banks {
        let (mut first, mut last) = (usize::MAX, 0usize);
        let mut seen = 0usize;
        for id in bank {
            let Some(index) = node_index(spec, id) else {
                continue;
            };
            let Some(op) = node_op[index as usize] else {
                continue;
            };
            first = first.min(op as usize);
            last = last.max(op as usize);
            seen += 1;
        }
        if seen > 1 && first < last {
            spans.push((first, last));
        }
    }
    spans.sort_unstable();
    let mut defer_release = vec![false; op_count];
    let mut closes_here = vec![false; op_count];
    let mut merged: Vec<(usize, usize)> = Vec::with_capacity(spans.len());
    for (first, last) in spans {
        match merged.last_mut() {
            Some(open) if first <= open.1 => open.1 = open.1.max(last),
            _ => merged.push((first, last)),
        }
    }
    for (first, last) in merged {
        // The releasing op is the span's *reader*: a slot whose last reader is `last` is safe to
        // reuse afterwards, because the span is over by then. Hence the half-open range.
        for slot in defer_release.iter_mut().take(last).skip(first) {
            *slot = true;
        }
        closes_here[last] = true;
    }
    (defer_release, closes_here)
}

/// One logical buffer's lifetime, in op indices.
struct Lifetime {
    def_op: usize,
    last_use: usize,
    dedicated: bool,
}

/// Lower a semantic graph into an executable program.
///
/// `schedule` must be the concatenation of `levels`, `spec.nodes` must be sorted by id, and every
/// edge must run forwards in the schedule -- all three are properties the graph compiler
/// establishes before it calls this.
///
/// `banks` is one entry per homogeneous bank this plan will render, each listing that bank's
/// member nodes in any order and from any rack. It is the *plan's* answer, not a guess from the
/// node id, and it is used for one thing: the op ranges those banks reorder. Ids that are not
/// nodes of this spec are ignored -- a stale id can only widen a window, never unsound one.
///
/// ## Banks reorder the schedule, and colouring has to survive it (issue #169)
///
/// `runtime::units_of` emits a whole bank as **one unit at its first member's op position**, and
/// that unit gathers every member before the kernel runs. Over the op range from a bank's first
/// member to its last -- its *window* -- execution is therefore a permutation of the schedule
/// this function coloured: a member scheduled after the first member runs *earlier*, and a
/// non-member op scheduled between two members runs *later*.
///
/// A *single-slot* bank may not cross a dependency level (#96 F12) and ops are level-major, so
/// every op in such a window sits at one level and no op in it reads another's output. Each op in
/// the window therefore reads only values produced before the window and is read only after it.
/// Its inputs are live entering the window and its output is dead until the window ends, so the
/// whole soundness condition collapses to one sentence:
///
/// > **No physical slot may be recycled inside a bank window.**
///
/// ## A cohort chain's window spans levels, and the sentence still holds (issues #181, #202)
///
/// `runtime::cohort_runs` renders consecutive slots of one cohort chain as **one** unit, so a
/// window can cover slot `k` at level `L` and slot `k + 1` at level `L + 1`, and the clause "no op
/// in a window reads another's output" is false there: slot `k + 1`'s op names slot `k`'s buffer.
/// The argument has to be re-made rather than reused, and it comes out the same way:
///
/// * A later slot's op is **not executed**. The chain computes it over the resident AoSoA block,
///   so the read that would have gone through the arena never happens. What was an inter-op edge
///   inside the window becomes a value passed between two slots in registers and scratch.
/// * `runtime::chains_into` merges only when the later slot's op has exactly one main input, no
///   sidechain, and no compensation-delay staging, and when the earlier slot's output is read by
///   **nothing else** -- no second consumer, no observer, no *observed* alias, and not the session
///   output. So the earlier slot's buffer, which now holds the chain's *input* rather than that
///   slot's output, has no reader inside or outside the window that could tell.
/// * Every other op in the window is still read only after the window and written only before it,
///   exactly as above.
///
/// Issue #202 rec 2 widens which pairs may fuse -- across rack locations, and into the post-input
/// builtin bank -- and none of the argument above depends on where a slot sat. What it does change
/// is how far a window reaches: a chain that runs `builtins -> EQ -> compressor -> limiter` for one
/// cohort executes that whole strip at the cohort's *first* op position, so its window spans every
/// dependency level the strip crosses. The obligation is unchanged -- no physical slot may be
/// recycled inside it -- and it is simply held over a longer range, which costs arena buffers and
/// buys the round-trips.
///
/// `chainable_bank_groups` is what makes the window the merged one. It is handed one entry per
/// bound bank and unions the pairs that could fuse, using the same lane-wise producer/consumer
/// relation `chains_into` proves, so the span this function holds is a superset of the span the
/// runtime actually permutes. A window wider than the permutation is always safe; one narrower is
/// the defect this machinery exists to prevent.
///
/// `bank_windows` computes those ranges and pass 2 *holds* every slot freed inside one until
/// the window closes, instead of returning it to the free list where an op the bank hoists past
/// its releaser could take it. `a_bank_window_never_recycles_a_physical_slot` constructs the
/// smallest graph that reaches the defect and
/// `bank_window_hoisting_preserves_dataflow_on_random_graphs` interprets seeded graphs in the
/// order the executor actually runs them.
///
/// ## The rejected alternative: dedication by bank membership
///
/// The obvious-looking fix is to extend `is_dedicated` to bank members, so a member's output is
/// never returned to the free list. **It was implemented, measured and deliberately not taken.**
/// Do not re-propose it without new evidence, because:
///
/// * **It does not fix the defect.** Dedication governs what colouring *returns*; the hazard is
///   what colouring *takes*. `take` draws from the free list with no notion of a window, so a
///   slot released inside one still reaches a member hoisted past its releaser. Over the corpus
///   `bank_window_hoisting_preserves_dataflow_on_random_graphs` draws from, 285 of 3617 graphs
///   diverge today and the window hold takes that to zero.
/// * **On its own it makes matters worse** -- 528 divergences, up from 285. A dedicated member
///   cannot fold into its producer in place, so it allocates, and the slot it allocates may be
///   one a hoisted op still needs.
/// * **It is redundant once windows hold**, and it is not free: on
///   `fixtures/session/v1/console-sixty-four-track.json` it costs 64 arena buffers (193 -> 257)
///   and 64 stereo block copies per render block, one per dynamic member whose consumer can no
///   longer consume it in place. The window hold costs nothing there --
///   `banking_a_dynamic_rack_costs_no_arena_buffers` pins the 193.
///
/// The invariant the doc on `is_dedicated` used to claim for bank members -- "no op may consume
/// a member's buffer in place" -- is not needed and is not held. A member's consumer sits at a
/// strictly later dependency level, so it runs after the whole bank unit, including the
/// observers; overwriting a member's output there is safe.
///
/// # Errors
/// See [`ProgramError`]; every variant means the caller's own invariants were violated.
pub fn lower(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
    banks: &[Vec<GraphNodeId>],
) -> Result<ExecutionProgram, ProgramError> {
    lower_with(spec, schedule, levels, delays, banks, true)
}

/// [`lower`], with the cohort-chain window union switched off.
///
/// The only caller is the counterfactual arm of
/// `cohort_chain_merging_preserves_dataflow_on_random_graphs`: it measures how many seeded graphs a
/// per-bank window gets wrong, which is the measurement of what [`chainable_bank_groups`] buys.
/// There is no production path to it, and there must not be -- a per-bank window is unsound for a
/// merged chain.
#[cfg(test)]
fn lower_with_per_bank_windows(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
    banks: &[Vec<GraphNodeId>],
) -> Result<ExecutionProgram, ProgramError> {
    lower_with(spec, schedule, levels, delays, banks, false)
}

#[allow(clippy::too_many_lines)]
fn lower_with(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
    banks: &[Vec<GraphNodeId>],
    chain_windows: bool,
) -> Result<ExecutionProgram, ProgramError> {
    if spec.nodes.windows(2).any(|pair| pair[0].id >= pair[1].id) {
        return Err(ProgramError::SpecUnsorted);
    }
    let node_count = spec.nodes.len();
    if schedule.len() != node_count {
        return Err(ProgramError::ScheduleMismatch);
    }
    // The schedule is the concatenation of the levels, and each node carries its level.
    let mut level_of = vec![None; node_count];
    let mut cursor = 0usize;
    for level in levels {
        for id in &level.nodes {
            if schedule.get(cursor) != Some(id) {
                return Err(ProgramError::ScheduleMismatch);
            }
            let index = node_index(spec, id).ok_or(ProgramError::UnknownNode)? as usize;
            if level_of[index].is_some() {
                return Err(ProgramError::ScheduleMismatch);
            }
            level_of[index] = Some(level.level);
            cursor += 1;
        }
    }
    if cursor != node_count {
        return Err(ProgramError::ScheduleMismatch);
    }

    // Schedule position per node, and the interned schedule.
    let mut position = vec![usize::MAX; node_count];
    let mut order = Vec::with_capacity(node_count);
    for (slot, id) in schedule.iter().enumerate() {
        let index = node_index(spec, id).ok_or(ProgramError::UnknownNode)? as usize;
        position[index] = slot;
        order.push(index);
    }

    let delay_of: std::collections::BTreeMap<&GraphEdgeId, u64> = delays
        .iter()
        .map(|delay| (&delay.edge_id, delay.samples.0))
        .collect();

    // Inputs per node, in `spec.edges` order -- which is sorted by `GraphEdgeId` (D9).
    let mut main_in: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    let mut side_in: Vec<Option<usize>> = vec![None; node_count];
    for (edge_index, edge) in spec.edges.iter().enumerate() {
        let source = node_index(spec, &edge.source.node).ok_or(ProgramError::UnknownNode)? as usize;
        let destination =
            node_index(spec, &edge.destination.node).ok_or(ProgramError::UnknownNode)? as usize;
        if position[source] >= position[destination] {
            return Err(ProgramError::NotTopological);
        }
        match edge.destination.kind {
            GraphPortKind::MainInput => main_in[destination].push(edge_index),
            GraphPortKind::SidechainInput => side_in[destination] = Some(edge_index),
            GraphPortKind::MainOutput => return Err(ProgramError::NotTopological),
        }
    }

    let edge_delay = |edge_index: usize| {
        delay_of
            .get(&spec.edges[edge_index].id)
            .copied()
            .filter(|s| *s != 0)
    };

    // A stage boundary is elided when it is a pure alias: one main input, no sidechain, no PDC.
    let elided: Vec<bool> = (0..node_count)
        .map(|index| {
            is_alias_candidate(&spec.nodes[index].id)
                && main_in[index].len() == 1
                && side_in[index].is_none()
                && edge_delay(main_in[index][0]).is_none()
        })
        .collect();

    // A read is an edge whose *destination* is not elided: an edge into an elided node is an
    // alias, not a consumption, and a tap is not a reader either (it fires immediately after the
    // producing op, before any consumer). Counting either would block every in-place op in a
    // chain.
    let mut reads_of = vec![0usize; node_count];
    for edge in &spec.edges {
        let destination =
            node_index(spec, &edge.destination.node).ok_or(ProgramError::UnknownNode)? as usize;
        if elided[destination] {
            continue;
        }
        let mut source =
            node_index(spec, &edge.source.node).ok_or(ProgramError::UnknownNode)? as usize;
        // Resolve through the alias chain to the node that actually produces the storage.
        while elided[source] {
            source = node_index(spec, &spec.edges[main_in[source][0]].source.node)
                .ok_or(ProgramError::UnknownNode)? as usize;
        }
        reads_of[source] += 1;
    }

    // ---- pass 1: ops, logical buffers, aliases ---------------------------------------------
    let mut ops: Vec<Op> = Vec::with_capacity(node_count);
    let mut inputs: Vec<InputRef> = Vec::new();
    let mut delay_specs: Vec<DelaySpec> = Vec::new();
    let mut taps: Vec<Tap> = Vec::new();
    let mut node_op: Vec<Option<OpIndex>> = vec![None; node_count];
    let mut logical_of_node: Vec<Option<u32>> = vec![None; node_count];
    let mut after_op_of_node: Vec<Option<OpIndex>> = vec![None; node_count];
    let mut lifetimes: Vec<Lifetime> = Vec::new();
    // Which node currently owns a logical buffer, so an in-place chain keeps one reader count.
    let mut owner_of_logical: Vec<usize> = Vec::new();

    for &index in &order {
        let id = &spec.nodes[index].id;
        if elided[index] {
            let producer = node_index(spec, &spec.edges[main_in[index][0]].source.node)
                .ok_or(ProgramError::UnknownNode)? as usize;
            let buffer = logical_of_node[producer].ok_or(ProgramError::NotTopological)?;
            let after = after_op_of_node[producer].ok_or(ProgramError::NotTopological)?;
            logical_of_node[index] = Some(buffer);
            after_op_of_node[index] = Some(after);
            taps.push(Tap {
                node: u32::try_from(index).map_err(|_| ProgramError::Overflow)?,
                buffer: BufferRef(buffer),
                after_op: after,
            });
            continue;
        }

        let op_index = u32::try_from(ops.len()).map_err(|_| ProgramError::Overflow)?;
        let first_input = u32::try_from(inputs.len()).map_err(|_| ProgramError::Overflow)?;
        for &edge_index in &main_in[index] {
            let source = node_index(spec, &spec.edges[edge_index].source.node)
                .ok_or(ProgramError::UnknownNode)? as usize;
            let buffer = logical_of_node[source].ok_or(ProgramError::NotTopological)?;
            let delay = edge_delay(edge_index).map(|samples| {
                delay_specs.push(DelaySpec { samples });
                (delay_specs.len() - 1) as u32
            });
            inputs.push(InputRef {
                buffer: BufferRef(buffer),
                // The staging buffer is filled in by pass 2, which knows the physical arena.
                delay: delay.map(|line| DelayRef {
                    line,
                    staging: BufferRef(u32::MAX),
                }),
            });
            lifetimes[buffer as usize].last_use = ops.len();
        }
        let last_input = u32::try_from(inputs.len()).map_err(|_| ProgramError::Overflow)?;
        let sidechain = match side_in[index] {
            None => None,
            Some(edge_index) => {
                let source = node_index(spec, &spec.edges[edge_index].source.node)
                    .ok_or(ProgramError::UnknownNode)? as usize;
                let buffer = logical_of_node[source].ok_or(ProgramError::NotTopological)?;
                let delay = edge_delay(edge_index).map(|samples| {
                    delay_specs.push(DelaySpec { samples });
                    (delay_specs.len() - 1) as u32
                });
                lifetimes[buffer as usize].last_use = ops.len();
                Some(InputRef {
                    buffer: BufferRef(buffer),
                    delay: delay.map(|line| DelayRef {
                        line,
                        staging: BufferRef(u32::MAX),
                    }),
                })
            }
        };
        let dedicated = is_dedicated(id);
        // In place iff this op is the *only* reader of a single undelayed input, and neither end
        // of the aliasing is dedicated storage.
        let single = (last_input - first_input == 1)
            && sidechain.is_none()
            && inputs[first_input as usize].delay.is_none();
        let in_place = single && !dedicated && {
            let buffer = inputs[first_input as usize].buffer.0 as usize;
            let owner = owner_of_logical[buffer];
            reads_of[owner] == 1 && !lifetimes[buffer].dedicated
        };
        let output = if in_place {
            let buffer = inputs[first_input as usize].buffer.0;
            owner_of_logical[buffer as usize] = index;
            buffer
        } else {
            let buffer = u32::try_from(lifetimes.len()).map_err(|_| ProgramError::Overflow)?;
            lifetimes.push(Lifetime {
                def_op: ops.len(),
                last_use: ops.len(),
                dedicated,
            });
            owner_of_logical.push(index);
            buffer
        };
        lifetimes[output as usize].last_use = lifetimes[output as usize].last_use.max(ops.len());

        ops.push(Op {
            node: u32::try_from(index).map_err(|_| ProgramError::Overflow)?,
            level: level_of[index].ok_or(ProgramError::ScheduleMismatch)?,
            inputs: (first_input, last_input),
            sidechain,
            output: BufferRef(output),
            in_place,
        });
        node_op[index] = Some(op_index);
        logical_of_node[index] = Some(output);
        after_op_of_node[index] = Some(op_index);
    }

    // The sole session output survives the last op: the executor copies it out afterwards.
    let output_node = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
        .ok_or(ProgramError::UnknownNode)?;
    let output_logical = logical_of_node[output_node].ok_or(ProgramError::UnknownNode)?;
    lifetimes[output_logical as usize].last_use = ops.len();

    // ---- pass 2: liveness colouring over ops -------------------------------------------------
    let mut expire: Vec<Vec<u32>> = vec![Vec::new(); ops.len() + 1];
    for (buffer, life) in lifetimes.iter().enumerate() {
        expire[life.last_use].push(u32::try_from(buffer).map_err(|_| ProgramError::Overflow)?);
    }
    // Issue #202 rec 2: the window a merged cohort chain permutes is the union of every slot's
    // ops, and a chain may now fuse across rack locations and into a builtin bank, so the union is
    // derived from the graph's own dataflow rather than from the cohort planner's grouping.
    let chained = if chain_windows {
        chainable_bank_groups(banks, spec, &elided, &main_in)
    } else {
        banks.to_vec()
    };
    let (defer_release, closes_here) = bank_windows(&node_op, &chained, spec, ops.len());
    let mut physical = vec![u32::MAX; lifetimes.len()];
    let mut free: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    // Slots freed inside a bank window, released together once the window closes (#169).
    let mut held: Vec<u32> = Vec::new();
    let mut next_physical = 0u32;
    let mut staging_release: Vec<u32> = Vec::new();
    let take = |free: &mut std::collections::BTreeSet<u32>, next: &mut u32| -> u32 {
        if let Some(buffer) = free.pop_first() {
            buffer
        } else {
            let buffer = *next;
            *next += 1;
            buffer
        }
    };
    for op_index in 0..ops.len() {
        // Slots retire one op late: a buffer whose last reader is `op_index - 1` is only free
        // once that read has happened.
        if op_index > 0 {
            // The window that op `op_index - 1` sat in (if any) is behind us now, so the slots it
            // held are safe to hand out again.
            if closes_here[op_index - 1] {
                free.extend(held.drain(..));
            }
            // What op `op_index - 1` gave up. A dedicated buffer gives up nothing -- that is what
            // dedication is. A staging scratch belongs to the op that filled it, so it retires
            // with that op's outputs.
            //
            // Inside a bank window those slots are *held* rather than freed (#169): the bank
            // hoists some of the window's ops past op `op_index - 1`, so a slot it no longer
            // needs may be one they still do.
            let retired = expire[op_index - 1]
                .drain(..)
                .filter(|buffer| !lifetimes[*buffer as usize].dedicated)
                .map(|buffer| physical[buffer as usize])
                .chain(staging_release.drain(..));
            if defer_release[op_index - 1] {
                held.extend(retired);
            } else {
                free.extend(retired);
            }
        }
        // A delayed input stages into a scratch buffer that lives only for this op.
        let (first, last) = ops[op_index].inputs;
        for input in &mut inputs[first as usize..last as usize] {
            if let Some(delay) = &mut input.delay {
                let buffer = take(&mut free, &mut next_physical);
                delay.staging = BufferRef(buffer);
                staging_release.push(buffer);
            }
        }
        if let Some(delay) = ops[op_index]
            .sidechain
            .as_mut()
            .and_then(|side| side.delay.as_mut())
        {
            let buffer = take(&mut free, &mut next_physical);
            delay.staging = BufferRef(buffer);
            staging_release.push(buffer);
        }
        let output = ops[op_index].output.0;
        if lifetimes[output as usize].def_op == op_index && physical[output as usize] == u32::MAX {
            physical[output as usize] = take(&mut free, &mut next_physical);
        }
        ops[op_index].output = BufferRef(physical[output as usize]);
    }
    // Rewrite every logical reference to its physical buffer.
    for input in &mut inputs {
        input.buffer = BufferRef(physical[input.buffer.0 as usize]);
    }
    for op in &mut ops {
        if let Some(side) = &mut op.sidechain {
            side.buffer = BufferRef(physical[side.buffer.0 as usize]);
        }
    }
    let node_buffer: Vec<BufferRef> = (0..node_count)
        .map(|index| {
            logical_of_node[index]
                .map(|buffer| BufferRef(physical[buffer as usize]))
                .ok_or(ProgramError::UnknownNode)
        })
        .collect::<Result<_, _>>()?;
    for tap in &mut taps {
        tap.buffer = node_buffer[tap.node as usize];
    }

    let output = node_buffer[output_node];
    Ok(ExecutionProgram {
        ops: ops.into_boxed_slice(),
        inputs: inputs.into_boxed_slice(),
        delays: delay_specs.into_boxed_slice(),
        node_buffer: node_buffer.into_boxed_slice(),
        node_op: node_op.into_boxed_slice(),
        taps: taps.into_boxed_slice(),
        buffers: next_physical,
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GraphEdge, GraphNode, GraphPortId, StableGraphId};
    use effect_contract::{LatencySamples, TailSamples};

    fn gid(value: &str) -> StableGraphId {
        StableGraphId::parse(value).expect("static id")
    }
    fn stage_node(track: &str, stage: TrackStage) -> GraphNodeId {
        GraphNodeId::TrackStage {
            track_id: gid(track),
            stage,
        }
    }
    fn node(id: GraphNodeId) -> GraphNode {
        GraphNode {
            id,
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
        }
    }
    fn port(node: GraphNodeId, kind: GraphPortKind) -> GraphPortId {
        GraphPortId {
            node,
            kind,
            effect_port: None,
        }
    }
    fn main_edge(id: GraphEdgeId, source: GraphNodeId, destination: GraphNodeId) -> GraphEdge {
        GraphEdge {
            id,
            source: port(source, GraphPortKind::MainOutput),
            destination: port(destination, GraphPortKind::MainInput),
            path: "$".to_owned(),
        }
    }
    /// A spec plus the level-major schedule the graph compiler would emit for it.
    ///
    /// The compiler sorts nodes and edges by id and emits levels in ascending node-id order, so
    /// the fixtures do the same rather than assuming a hand-written order is canonical.
    fn build(
        mut nodes: Vec<GraphNode>,
        mut edges: Vec<GraphEdge>,
    ) -> (GraphSpec, Vec<GraphNodeId>, Vec<DependencyLevel>) {
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        let mut level_of: std::collections::BTreeMap<GraphNodeId, u64> =
            nodes.iter().map(|node| (node.id.clone(), 0_u64)).collect();
        // Longest-path levels; the fixtures are small, so iterate to a fixed point.
        for _ in 0..nodes.len() {
            for edge in &edges {
                let source = level_of[&edge.source.node];
                let destination = level_of.get_mut(&edge.destination.node).expect("node");
                *destination = (*destination).max(source + 1);
            }
        }
        let depth = level_of.values().copied().max().unwrap_or(0);
        let levels: Vec<DependencyLevel> = (0..=depth)
            .map(|level| DependencyLevel {
                level,
                nodes: nodes
                    .iter()
                    .filter(|node| level_of[&node.id] == level)
                    .map(|node| node.id.clone())
                    .collect(),
            })
            .filter(|level| !level.nodes.is_empty())
            .collect();
        let schedule: Vec<GraphNodeId> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let spec = GraphSpec {
            ports: Vec::new(),
            nodes,
            edges,
        };
        (spec, schedule, levels)
    }

    /// One track, no effects, one route, one output: the shape that cost seven schedule items and
    /// six full buffer copies per block before lowering.
    fn plain_track(track: &str, routes: &[&str]) -> (Vec<GraphNode>, Vec<GraphEdge>) {
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostSimd1,
            TrackStage::PostDynamic,
            TrackStage::PostSimd2PreFader,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let mut nodes: Vec<GraphNode> = stages
            .iter()
            .map(|stage| node(stage_node(track, *stage)))
            .collect();
        let mut edges: Vec<GraphEdge> = stages
            .windows(2)
            .map(|pair| {
                main_edge(
                    GraphEdgeId::TrackMain {
                        target: stage_node(track, pair[1]),
                    },
                    stage_node(track, pair[0]),
                    stage_node(track, pair[1]),
                )
            })
            .collect();
        nodes.push(node(GraphNodeId::Output {
            output_id: gid("out"),
        }));
        for route in routes {
            let route_node = GraphNodeId::Route {
                route_id: gid(route),
            };
            nodes.push(node(route_node.clone()));
            edges.push(main_edge(
                GraphEdgeId::RouteSource {
                    route_id: gid(route),
                },
                stage_node(track, TrackStage::PostMatrix),
                route_node.clone(),
            ));
            edges.push(main_edge(
                GraphEdgeId::RouteDestination {
                    route_id: gid(route),
                },
                route_node,
                GraphNodeId::Output {
                    output_id: gid("out"),
                },
            ));
        }
        (nodes, edges)
    }

    /// The headline #99 F2 result: nine semantic nodes and eight edges become six ops, three
    /// aliases and a two-buffer arena.
    ///
    /// Before lowering this graph executed nine schedule items, allocated one contribution buffer
    /// per edge (eight) plus nine coloured node outputs, copied every edge every block, and ran a
    /// one-element pairwise reduction at every node.
    #[test]
    fn chain_of_seven_stages_lowers_to_six_ops_three_taps_and_two_buffers() {
        let (nodes, edges) = plain_track("t", &["r"]);
        let (spec, schedule, levels) = build(nodes, edges);
        assert_eq!(spec.nodes.len(), 9);
        assert_eq!(spec.edges.len(), 8);
        let program = lower(&spec, &schedule, &levels, &[], &[]).expect("lowers");

        assert_eq!(program.ops.len(), 6);
        assert_eq!(program.taps.len(), 3);
        assert_eq!(program.buffers, 2);
        // Nothing needs a per-frame reduction: every op has exactly one input.
        assert_eq!(program.reduction_count(), 0);
        assert_eq!(program.delayed_input_count(), 0);

        let op_nodes: Vec<&GraphNodeId> = program
            .ops
            .iter()
            .map(|op| &spec.nodes[op.node as usize].id)
            .collect();
        assert_eq!(
            op_nodes,
            vec![
                &stage_node("t", TrackStage::Input),
                &stage_node("t", TrackStage::PostInputBuiltins),
                &stage_node("t", TrackStage::PostFader),
                &stage_node("t", TrackStage::PostMatrix),
                &GraphNodeId::Route { route_id: gid("r") },
                &GraphNodeId::Output {
                    output_id: gid("out")
                },
            ]
        );
        // Input has no producer; the builtin stage is bank-eligible so it must own its buffer;
        // everything downstream of it reads in place.
        let in_place: Vec<bool> = program.ops.iter().map(|op| op.in_place).collect();
        assert_eq!(in_place, vec![false, false, false, true, true, true]);

        // The three elided stages alias the builtin stage's buffer and observe right after it.
        for tap in &program.taps {
            assert_eq!(
                tap.buffer,
                program.node_buffer[program.ops[1].node as usize]
            );
            assert_eq!(tap.after_op, 1);
            assert!(program.node_op[tap.node as usize].is_none());
        }
        assert_eq!(program.output, program.ops[5].output);
    }

    /// Two routes off one tap: the shared buffer has two readers, so neither route may consume it
    /// in place, and the output op keeps a genuine two-input reduction.
    #[test]
    fn fan_out_blocks_in_place_and_fan_in_keeps_its_reduction() {
        let (nodes, edges) = plain_track("t", &["ra", "rb"]);
        let (spec, schedule, levels) = build(nodes, edges);
        let program = lower(&spec, &schedule, &levels, &[], &[]).expect("lowers");

        assert_eq!(program.taps.len(), 3);
        assert_eq!(program.reduction_count(), 1);
        let output_op = program.ops.last().expect("output op");
        assert_eq!(output_op.input_count(), 2);
        assert!(!output_op.in_place);
        // The two route inputs of the output are distinct buffers, in stable edge-ID order.
        let output_inputs = program.inputs_of(output_op);
        assert_ne!(output_inputs[0].buffer, output_inputs[1].buffer);

        let routes: Vec<&Op> = program
            .ops
            .iter()
            .filter(|op| matches!(spec.nodes[op.node as usize].id, GraphNodeId::Route { .. }))
            .collect();
        assert_eq!(routes.len(), 2);
        for route in routes {
            assert!(
                !route.in_place,
                "a route reading a two-reader buffer must not alias it"
            );
        }
    }

    #[test]
    fn sixty_four_plain_tracks_keep_one_ordered_non_aliasing_master_reduction() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for index in 0..64 {
            let track = format!("track{index:02}");
            let route = format!("route{index:02}");
            let (mut track_nodes, mut track_edges) = plain_track(&track, &[&route]);
            track_nodes.retain(|node| !matches!(node.id, GraphNodeId::Output { .. }));
            nodes.append(&mut track_nodes);
            edges.append(&mut track_edges);
        }
        nodes.push(node(GraphNodeId::Output {
            output_id: gid("out"),
        }));
        let (spec, schedule, levels) = build(nodes, edges);
        let program = lower(&spec, &schedule, &levels, &[], &[]).expect("plumbing lowers");
        let master = program.ops.last().expect("master output");
        let inputs = program.inputs_of(master);
        assert_eq!(inputs.len(), 64);
        assert!(!master.in_place);
        assert!(inputs.iter().all(|input| input.buffer != master.output));
        let expected: Vec<_> = (0..64)
            .map(|index| {
                let id = GraphNodeId::Route {
                    route_id: gid(&format!("route{index:02}")),
                };
                let node = spec
                    .nodes
                    .iter()
                    .position(|candidate| candidate.id == id)
                    .expect("route node");
                program.node_buffer[node]
            })
            .collect();
        assert_eq!(
            inputs.iter().map(|input| input.buffer).collect::<Vec<_>>(),
            expected
        );
    }

    /// A PDC edge stages into a scratch buffer that is returned to the arena after its op, and its
    /// consumer is never in place -- the delayed samples are not the producer's buffer.
    #[test]
    fn delayed_edge_gets_staging_buffer_and_blocks_in_place() {
        let (nodes, edges) = plain_track("t", &["r"]);
        let (spec, schedule, levels) = build(nodes, edges);
        let delayed = InsertedDelay {
            node: GraphNodeId::Route { route_id: gid("r") },
            edge_id: GraphEdgeId::RouteSource { route_id: gid("r") },
            samples: LatencySamples(64),
        };
        let program = lower(
            &spec,
            &schedule,
            &levels,
            std::slice::from_ref(&delayed),
            &[],
        )
        .expect("lowers");

        assert_eq!(program.delays.len(), 1);
        assert_eq!(program.delays[0].samples, 64);
        assert_eq!(program.delayed_input_count(), 1);
        let route_op = program
            .ops
            .iter()
            .find(|op| matches!(spec.nodes[op.node as usize].id, GraphNodeId::Route { .. }))
            .expect("route op");
        assert!(
            !route_op.in_place,
            "a delayed input cannot be consumed in place"
        );
        let input = program.inputs_of(route_op)[0];
        let delay = input.delay.expect("delayed input");
        assert_eq!(delay.line, 0);
        assert_ne!(delay.staging, input.buffer);
        assert!(delay.staging.0 < program.buffers);
    }

    /// An elided stage is exactly a stage with one undelayed, un-sidechained input: give the same
    /// stage a PDC edge and it keeps its op.
    #[test]
    fn a_delayed_stage_boundary_is_not_elided() {
        let (nodes, edges) = plain_track("t", &["r"]);
        let (spec, schedule, levels) = build(nodes, edges);
        let delayed = InsertedDelay {
            node: stage_node("t", TrackStage::PostSimd1),
            edge_id: GraphEdgeId::TrackMain {
                target: stage_node("t", TrackStage::PostSimd1),
            },
            samples: LatencySamples(8),
        };
        let program = lower(
            &spec,
            &schedule,
            &levels,
            std::slice::from_ref(&delayed),
            &[],
        )
        .expect("lowers");
        assert_eq!(program.taps.len(), 2);
        assert_eq!(program.ops.len(), 7);
        let index = node_index(&spec, &stage_node("t", TrackStage::PostSimd1)).expect("node");
        assert!(program.node_op[index as usize].is_some());
    }

    /// A dynamic-rack effect is not bank-eligible, so the two stage boundaries after it are pure
    /// aliases of *its* buffer and the fader downstream still reads that buffer in place.
    ///
    /// This is the fixture that makes the "a tap is not a reader" rule observable. In the
    /// effect-free chain every alias chain is rooted at the bank-eligible builtin stage, whose
    /// buffer is never consumed in place for an unrelated reason, so miscounting taps as readers
    /// changes nothing there. Here the root is an ordinary effect: count the two taps as readers
    /// and the fader stops aliasing, costing a buffer and a full copy per block.
    #[test]
    fn taps_are_not_readers_so_an_alias_chain_still_folds_into_its_producer() {
        let track = "t";
        let effect = GraphNodeId::Effect(crate::EffectNodeId {
            track_id: gid(track),
            rack: RackId::Dynamic,
            effect_id: gid("d"),
        });
        let (mut nodes, mut edges) = plain_track(track, &["r"]);
        nodes.push(node(effect.clone()));
        // Splice the effect between PostSimd1 and PostDynamic.
        edges.retain(|edge| {
            edge.id
                != GraphEdgeId::TrackMain {
                    target: stage_node(track, TrackStage::PostDynamic),
                }
        });
        edges.push(main_edge(
            GraphEdgeId::TrackMain {
                target: effect.clone(),
            },
            stage_node(track, TrackStage::PostSimd1),
            effect.clone(),
        ));
        edges.push(main_edge(
            GraphEdgeId::TrackMain {
                target: stage_node(track, TrackStage::PostDynamic),
            },
            effect.clone(),
            stage_node(track, TrackStage::PostDynamic),
        ));
        let (spec, schedule, levels) = build(nodes, edges);
        let program = lower(&spec, &schedule, &levels, &[], &[]).expect("lowers");

        let effect_index = node_index(&spec, &effect).expect("effect node");
        let effect_op = program.node_op[effect_index as usize].expect("effect keeps its op");
        assert!(!program.ops[effect_op as usize].in_place);

        // PostDynamic and PostSimd2PreFader alias the effect's buffer and observe right after it.
        let aliased: Vec<&Tap> = program
            .taps
            .iter()
            .filter(|tap| tap.buffer == program.node_buffer[effect_index as usize])
            .collect();
        assert_eq!(aliased.len(), 2);
        for tap in aliased {
            assert_eq!(tap.after_op, effect_op);
        }

        // The fader is the effect buffer's only real reader, so it consumes it in place.
        let fader_index =
            node_index(&spec, &stage_node(track, TrackStage::PostFader)).expect("fader");
        let fader_op = program.node_op[fader_index as usize].expect("fader keeps its op");
        assert!(
            program.ops[fader_op as usize].in_place,
            "an alias chain with one real reader must fold into its producer's buffer"
        );
        assert_eq!(
            program.ops[fader_op as usize].output,
            program.node_buffer[effect_index as usize]
        );
    }

    /// A symbolic value: what a buffer holds, as an expression over node outputs.
    ///
    /// `Sum` keeps its operand order, so a reduction whose inputs were reordered is a different
    /// value, not an equal one.
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Expr {
        Silence,
        Node(NodeIndex, Box<Expr>),
        Delayed(Box<Expr>, u64),
        Sum(Vec<Expr>),
    }

    /// Evaluate the *semantic* graph the naive way: every node consumes the ordered, individually
    /// delayed outputs of its incoming main edges. An identity stage boundary is transparent,
    /// because that is exactly what the executor's `RuntimeNodeKind::Identity` does with a
    /// single-input pairwise sum (`balanced_pairwise_sum` of one element returns it unchanged).
    fn evaluate_spec(
        spec: &GraphSpec,
        schedule: &[GraphNodeId],
        delays: &[InsertedDelay],
    ) -> Vec<Expr> {
        let mut value = vec![Expr::Silence; spec.nodes.len()];
        for id in schedule {
            let index = node_index(spec, id).expect("node") as usize;
            let mut operands = Vec::new();
            for edge in &spec.edges {
                if &edge.destination.node != id || edge.destination.kind != GraphPortKind::MainInput
                {
                    continue;
                }
                let source = node_index(spec, &edge.source.node).expect("node") as usize;
                let samples = delays
                    .iter()
                    .find(|delay| delay.edge_id == edge.id)
                    .map(|delay| delay.samples.0)
                    .filter(|samples| *samples != 0);
                operands.push(match samples {
                    Some(samples) => Expr::Delayed(Box::new(value[source].clone()), samples),
                    None => value[source].clone(),
                });
            }
            let combined = match operands.len() {
                0 => Expr::Silence,
                1 => operands.remove(0),
                _ => Expr::Sum(operands),
            };
            // An identity stage boundary carries its input unchanged whether or not lowering
            // elides it: elision removes the schedule item, never a transformation. Modelling it
            // as transparent on both sides is what makes the delayed case (which keeps its op)
            // comparable to the undelayed case (which becomes an alias).
            value[index] = if is_alias_candidate(id) {
                combined
            } else {
                Expr::Node(u32::try_from(index).expect("index"), Box::new(combined))
            };
        }
        value
    }

    /// Interpret the *program* over an arena of symbolic buffers and compare, node by node.
    ///
    /// This is the check that colouring is sound: if an op were given storage another op still
    /// needs, the later read returns the wrong expression and the comparison fails. It is also the
    /// check that in-place folding and aliasing preserve dataflow.
    fn assert_program_matches_spec(
        spec: &GraphSpec,
        schedule: &[GraphNodeId],
        delays: &[InsertedDelay],
        program: &ExecutionProgram,
    ) {
        let expected = evaluate_spec(spec, schedule, delays);
        let mut arena = vec![Expr::Silence; program.buffers as usize];
        let mut taps_by_op: std::collections::BTreeMap<OpIndex, Vec<&Tap>> =
            std::collections::BTreeMap::new();
        for tap in &program.taps {
            taps_by_op.entry(tap.after_op).or_default().push(tap);
        }
        for (op_index, op) in program.ops.iter().enumerate() {
            let mut operands = Vec::new();
            for input in program.inputs_of(op) {
                let value = arena[input.buffer.0 as usize].clone();
                operands.push(match input.delay {
                    Some(delay) => {
                        Expr::Delayed(Box::new(value), program.delays[delay.line as usize].samples)
                    }
                    None => value,
                });
            }
            let combined = match operands.len() {
                0 => Expr::Silence,
                1 => operands.remove(0),
                _ => Expr::Sum(operands),
            };
            if op.in_place {
                // The single input already lives in `output`; nothing is copied.
                assert_eq!(
                    op.output,
                    program.inputs_of(op)[0].buffer,
                    "an in-place op must write its own input buffer"
                );
            }
            // Same rule as the reference side: an identity stage boundary that kept its op
            // (because its input is delayed) still transforms nothing.
            let id = &spec.nodes[op.node as usize].id;
            arena[op.output.0 as usize] = if is_alias_candidate(id) {
                combined
            } else {
                Expr::Node(op.node, Box::new(combined))
            };
            assert_eq!(
                arena[op.output.0 as usize], expected[op.node as usize],
                "op {op_index} produced the wrong value"
            );
            // Every alias attached here must already read the value its node has semantically.
            for tap in taps_by_op.get(&(op_index as OpIndex)).into_iter().flatten() {
                assert_eq!(
                    arena[tap.buffer.0 as usize], expected[tap.node as usize],
                    "alias for node {} observes the wrong value",
                    tap.node
                );
            }
        }
        let output_node = spec
            .nodes
            .iter()
            .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
            .expect("output");
        assert_eq!(arena[program.output.0 as usize], expected[output_node]);
    }

    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    /// #99 F2, the load-bearing eval: over 300 seeded multi-track graphs with effects, fan-out,
    /// fan-in and PDC, the lowered program computes exactly the value the semantic graph does,
    /// and its arena is bounded by the graph's live width rather than its edge count.
    ///
    /// The comparison is symbolic, so it does not depend on either executor existing yet, and the
    /// reference side never looks at the program: it walks `spec`/`schedule`/`delays` naively,
    /// node by node, exactly as the pre-#99 render loop did.
    #[test]
    fn lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs() {
        let mut state = 0x5deb_c0de_1234_9e37_u64;
        for graph in 0..300_u32 {
            let track_count = (xorshift(&mut state) % 4) as usize + 1;
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            let mut route_index = 0usize;
            for track in 0..track_count {
                let name = format!("t{track:02}");
                let routes: Vec<String> = (0..(xorshift(&mut state) % 2) + 1)
                    .map(|_| {
                        route_index += 1;
                        format!("r{route_index:02}")
                    })
                    .collect();
                let borrowed: Vec<&str> = routes.iter().map(String::as_str).collect();
                let (track_nodes, track_edges) = plain_track(&name, &borrowed);
                // Splice an effect into one rack, sometimes bank-eligible and sometimes not.
                let rack = match xorshift(&mut state) % 3 {
                    0 => None,
                    1 => Some((RackId::Dynamic, TrackStage::PostDynamic)),
                    _ => Some((RackId::Simd1, TrackStage::PostSimd1)),
                };
                let mut track_nodes = track_nodes;
                let mut track_edges = track_edges;
                if let Some((rack, stage)) = rack {
                    let upstream = match stage {
                        TrackStage::PostSimd1 => TrackStage::PostInputBuiltins,
                        _ => TrackStage::PostSimd1,
                    };
                    let effect = GraphNodeId::Effect(crate::EffectNodeId {
                        track_id: gid(&name),
                        rack,
                        effect_id: gid("fx"),
                    });
                    track_nodes.push(node(effect.clone()));
                    track_edges.retain(|edge| {
                        edge.id
                            != GraphEdgeId::TrackMain {
                                target: stage_node(&name, stage),
                            }
                    });
                    track_edges.push(main_edge(
                        GraphEdgeId::TrackMain {
                            target: effect.clone(),
                        },
                        stage_node(&name, upstream),
                        effect.clone(),
                    ));
                    track_edges.push(main_edge(
                        GraphEdgeId::TrackMain {
                            target: stage_node(&name, stage),
                        },
                        effect,
                        stage_node(&name, stage),
                    ));
                }
                nodes.extend(track_nodes.into_iter().filter(|candidate| {
                    !matches!(candidate.id, GraphNodeId::Output { .. }) || track == 0
                }));
                edges.extend(track_edges);
            }
            let (spec, schedule, levels) = build(nodes, edges);
            // PDC on a random subset of edges.
            let mut delays: Vec<InsertedDelay> = Vec::new();
            for edge in &spec.edges {
                if !xorshift(&mut state).is_multiple_of(4) {
                    continue;
                }
                delays.push(InsertedDelay {
                    node: edge.destination.node.clone(),
                    edge_id: edge.id.clone(),
                    samples: LatencySamples(xorshift(&mut state) % 128 + 1),
                });
            }
            let program = lower(&spec, &schedule, &levels, &delays, &[])
                .unwrap_or_else(|error| panic!("graph {graph}: {error:?}"));

            assert_program_matches_spec(&spec, &schedule, &delays, &program);

            // Structure: every node is either an op or an alias, never both and never neither.
            assert_eq!(
                program.ops.len() + program.taps.len(),
                spec.nodes.len(),
                "graph {graph}: nodes are neither op nor alias"
            );
            for (index, op) in program.node_op.iter().enumerate() {
                let tapped = program.taps.iter().any(|tap| tap.node as usize == index);
                assert_eq!(op.is_none(), tapped, "graph {graph}: node {index}");
            }
            // Ops stay level-major, and within a level in ascending node id.
            assert!(
                program
                    .ops
                    .windows(2)
                    .all(|pair| (pair[0].level, pair[0].node) < (pair[1].level, pair[1].node)),
                "graph {graph}: ops are not level-major"
            );
            // Once a dedicated node has written its buffer, nothing else may write it.
            //
            // Dedication is forward-only, and deliberately so: inheriting storage a *dead* buffer
            // used earlier is fine and is what keeps the arena small. What it forbids is handing
            // the slot on once the node owns it. The symbolic interpreter above cannot see this,
            // because it compares values op by op and a recycled slot is only wrong once someone
            // reads it, so it is checked structurally here.
            //
            // This is *not* the bank invariant, despite `is_dedicated` predating banks and
            // covering most bank-eligible nodes. A bank's hazard is its reordering window; see
            // `bank_window_hoisting_preserves_dataflow_on_random_graphs` and `lower` (#169).
            let mut open: std::collections::BTreeMap<BufferRef, usize> =
                std::collections::BTreeMap::new();
            for (at, op) in program.ops.iter().enumerate() {
                if let Some(since) = open.get(&op.output) {
                    panic!(
                        "graph {graph}: op {at} writes buffer {:?}, held by a bank-eligible node \
                         since op {since}",
                        op.output
                    );
                }
                for input in program.inputs_of(op) {
                    if let Some(delay) = input.delay {
                        assert!(
                            !open.contains_key(&delay.staging),
                            "graph {graph}: op {at} stages PDC into open bank storage"
                        );
                    }
                }
                if is_dedicated(&spec.nodes[op.node as usize].id) {
                    open.insert(op.output, at);
                }
            }

            // Arena bound: at most one buffer per op, and strictly fewer than the pre-#99 model,
            // which allocated one contribution buffer per edge on top of a coloured output each.
            assert!(
                program.buffers as usize <= program.ops.len(),
                "graph {graph}: arena is larger than the op count"
            );
            assert!(
                (program.buffers as usize) < spec.edges.len() + spec.nodes.len(),
                "graph {graph}: arena is no smaller than the per-edge model it replaces"
            );
        }
    }

    /// The three rejections a caller maps to `graph.internal.invariant`.
    #[test]
    fn malformed_inputs_are_rejected_rather_than_lowered() {
        let (nodes, edges) = plain_track("t", &["r"]);
        let (spec, schedule, levels) = build(nodes, edges);
        assert_eq!(
            lower(&spec, &schedule[..schedule.len() - 1], &levels, &[], &[]),
            Err(ProgramError::ScheduleMismatch)
        );
        let mut swapped = schedule.clone();
        swapped.swap(0, 1);
        assert_eq!(
            lower(&spec, &swapped, &levels, &[], &[]),
            Err(ProgramError::ScheduleMismatch)
        );
        let mut unsorted = spec.clone();
        unsorted.nodes.swap(0, 1);
        assert_eq!(
            lower(&unsorted, &schedule, &levels, &[], &[]),
            Err(ProgramError::SpecUnsorted)
        );
    }

    // ---- issue #169: colouring across a bank's reordering window ---------------------------

    /// A track whose dynamic rack carries one effect, spliced between the two stage boundaries
    /// that surround it, plus one route to the shared output.
    fn dynamic_track(track: &str, route: &str) -> (Vec<GraphNode>, Vec<GraphEdge>) {
        let (mut nodes, mut edges) = plain_track(track, &[route]);
        let effect = GraphNodeId::Effect(crate::EffectNodeId {
            track_id: gid(track),
            rack: RackId::Dynamic,
            effect_id: gid("fx"),
        });
        nodes.push(node(effect.clone()));
        edges.retain(|edge| {
            edge.id
                != GraphEdgeId::TrackMain {
                    target: stage_node(track, TrackStage::PostDynamic),
                }
        });
        edges.push(main_edge(
            GraphEdgeId::TrackMain {
                target: effect.clone(),
            },
            stage_node(track, TrackStage::PostSimd1),
            effect.clone(),
        ));
        edges.push(main_edge(
            GraphEdgeId::TrackMain {
                target: stage_node(track, TrackStage::PostDynamic),
            },
            effect,
            stage_node(track, TrackStage::PostDynamic),
        ));
        (nodes, edges)
    }

    fn dynamic_effect(track: &str) -> GraphNodeId {
        GraphNodeId::Effect(crate::EffectNodeId {
            track_id: gid(track),
            rack: RackId::Dynamic,
            effect_id: gid("fx"),
        })
    }

    /// `node index -> (bank, lane)`, the shape `runtime::BankMembership` has.
    fn member_lanes(
        spec: &GraphSpec,
        banks: &[Vec<GraphNodeId>],
    ) -> std::collections::BTreeMap<u32, (usize, usize)> {
        let mut lanes = std::collections::BTreeMap::new();
        for (bank, members) in banks.iter().enumerate() {
            for (lane, id) in members.iter().enumerate() {
                lanes.insert(
                    node_index(spec, id).expect("member is a node"),
                    (bank, lane),
                );
            }
        }
        lanes
    }

    /// The op groups the executor will run, in the order it will run them.
    ///
    /// This mirrors `runtime::units_of` exactly, and it is the whole reason #169 exists: a bank
    /// becomes **one unit at its first member's position**, so a member scheduled later is hoisted
    /// forward and every non-member op between the members is deferred past the bank.
    fn units_in_runtime_order(
        program: &ExecutionProgram,
        lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
    ) -> Vec<Vec<usize>> {
        let mut units: Vec<Vec<usize>> = Vec::with_capacity(program.ops.len());
        let mut emitted: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
        for (index, op) in program.ops.iter().enumerate() {
            match lanes.get(&op.node) {
                None => units.push(vec![index]),
                Some((bank, _)) => {
                    if !emitted.insert(*bank) {
                        continue;
                    }
                    let mut members: Vec<(usize, usize)> = program
                        .ops
                        .iter()
                        .enumerate()
                        .filter_map(|(other, candidate)| {
                            lanes.get(&candidate.node).and_then(|(other_bank, lane)| {
                                (*other_bank == *bank).then_some((*lane, other))
                            })
                        })
                        .collect();
                    members.sort_unstable();
                    units.push(members.into_iter().map(|(_, op)| op).collect());
                }
            }
        }
        units
    }

    /// `runtime::op_dataflow`, restated for the interpreter: readers of each op, and the op that
    /// produced each op's first main input.
    fn op_dataflow_model(program: &ExecutionProgram) -> (Vec<Vec<usize>>, Vec<Option<usize>>) {
        let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
        let mut readers: Vec<Vec<usize>> = vec![Vec::new(); program.ops.len()];
        let mut first: Vec<Option<usize>> = vec![None; program.ops.len()];
        for (index, op) in program.ops.iter().enumerate() {
            for (position, input) in program.inputs_of(op).iter().enumerate() {
                if let Some(producer) = owner[input.buffer.0 as usize] {
                    readers[producer].push(index);
                    if position == 0 {
                        first[index] = Some(producer);
                    }
                }
            }
            if let Some(Some(producer)) = op.sidechain.map(|side| owner[side.buffer.0 as usize]) {
                readers[producer].push(index);
            }
            owner[op.output.0 as usize] = Some(index);
        }
        (readers, first)
    }

    /// `runtime::chains_into`, restated for the interpreter.
    ///
    /// The observer clauses are the two this model omits, and it omits them on purpose: a program
    /// -level fixture binds no observers at all, so both are vacuously satisfied. Omitting them
    /// makes the model *more* permissive than the runtime, which is the safe direction for an
    /// oracle -- it interprets at least every merge the runtime can take.
    fn chains_into_model(
        program: &ExecutionProgram,
        readers: &[Vec<usize>],
        first_producer: &[Option<usize>],
        earlier: &[usize],
        later: &[usize],
    ) -> bool {
        if earlier.len() != later.len() || earlier.is_empty() {
            return false;
        }
        earlier.iter().zip(later.iter()).all(|(before, after)| {
            let producer = &program.ops[*before];
            let consumer = &program.ops[*after];
            consumer.input_count() == 1
                && consumer.sidechain.is_none()
                && program.inputs_of(consumer)[0].delay.is_none()
                && first_producer[*after] == Some(*before)
                && readers[*before].len() == 1
                && readers[*before][0] == *after
                && producer.output != program.output
        })
    }

    /// `runtime::cohort_runs`, restated for the interpreter: the units of
    /// [`units_in_runtime_order`], grouped into the multi-slot chains the executor will build.
    ///
    /// Each entry is one rendered unit as a slot-major list: `run[slot][lane]` is an op index.
    fn runs_in_runtime_order(
        program: &ExecutionProgram,
        lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
    ) -> Vec<Vec<Vec<usize>>> {
        let units = units_in_runtime_order(program, lanes);
        let (readers, first_producer) = op_dataflow_model(program);
        let mut unit_of_op: Vec<Option<usize>> = vec![None; program.ops.len()];
        for (index, ops) in units.iter().enumerate() {
            if !lanes.contains_key(&program.ops[ops[0]].node) {
                continue;
            }
            for op in ops {
                unit_of_op[*op] = Some(index);
            }
        }
        let mut successor: std::collections::BTreeMap<usize, usize> =
            std::collections::BTreeMap::new();
        for (earlier, ops) in units.iter().enumerate() {
            if !lanes.contains_key(&program.ops[ops[0]].node) {
                continue;
            }
            let Some(later) = ops
                .first()
                .and_then(|lane| readers[*lane].first())
                .and_then(|reader| unit_of_op[*reader])
            else {
                continue;
            };
            if later != earlier
                && chains_into_model(program, &readers, &first_producer, ops, &units[later])
            {
                successor.insert(earlier, later);
            }
        }
        let merged: std::collections::BTreeSet<usize> = successor.values().copied().collect();
        let mut runs = Vec::with_capacity(units.len());
        for index in 0..units.len() {
            if merged.contains(&index) {
                continue;
            }
            let mut run = vec![units[index].clone()];
            let mut cursor = index;
            while let Some(next) = successor.get(&cursor) {
                run.push(units[*next].clone());
                cursor = *next;
            }
            runs.push(run);
        }
        runs
    }

    /// `runtime::bank_gather_source`, restated for the interpreter: the buffer a first-slot
    /// member's gather reads **instead of** the member's own output, or `None` when it must run.
    ///
    /// Modelling this is what closes the gap #194's verification flagged. A redirected member
    /// never writes its own output buffer during the gather, so an interpreter that always wrote
    /// it was describing a store the executor does not make -- and could therefore not tell that
    /// buffer apart from one another member's gather still needs.
    fn gather_source_model(program: &ExecutionProgram, op: &Op) -> Option<u32> {
        if op.sidechain.is_some() {
            return None;
        }
        match program.inputs_of(op) {
            [single] if single.delay.is_none() && single.buffer != op.output => {
                Some(single.buffer.0)
            }
            _ => None,
        }
    }

    /// `runtime::scatter_target`, restated for the interpreter.
    ///
    /// The observer clauses are omitted for the reason [`chains_into_model`] omits them: a
    /// program-level fixture binds none, so both are vacuously satisfied and omitting them makes
    /// the model at least as permissive as the runtime.
    fn scatter_target_model(
        program: &ExecutionProgram,
        lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
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
            || consumer_op.output == producer_op.output
            || producer_op.output == program.output
            || lanes.contains_key(&consumer_op.node)
        {
            return None;
        }
        let target = consumer_op.output;
        let names = |op: &Op| {
            let hit = |input: &InputRef| {
                input.buffer == target || input.delay.is_some_and(|delay| delay.staging == target)
            };
            op.output == target
                || program.inputs_of(op).iter().any(hit)
                || op.sidechain.as_ref().is_some_and(hit)
        };
        for (index, op) in program.ops.iter().enumerate().take(consumer).skip(first_op) {
            if !run.contains(&index) && names(op) {
                return None;
            }
        }
        Some(consumer)
    }

    /// `runtime::scatter_redirects`, restated for the interpreter: last-slot op -> the consumer op
    /// whose buffer the chain scatters into instead of its own.
    fn redirects_in_runtime_order(
        program: &ExecutionProgram,
        lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
        runs: &[Vec<Vec<usize>>],
    ) -> std::collections::BTreeMap<usize, usize> {
        let (readers, first_producer) = op_dataflow_model(program);
        let mut redirects = std::collections::BTreeMap::new();
        for run in runs {
            let Some(first_op) = run.iter().flatten().min().copied() else {
                continue;
            };
            let flat: Vec<usize> = run.iter().flatten().copied().collect();
            let last = run.last().expect("a run has at least one slot");
            if last.len() < 2 && !lanes.contains_key(&program.ops[last[0]].node) {
                continue;
            }
            let lane_targets: Vec<Option<usize>> = last
                .iter()
                .map(|producer| {
                    scatter_target_model(
                        program,
                        lanes,
                        &readers,
                        &first_producer,
                        &flat,
                        first_op,
                        *producer,
                    )
                })
                .collect();
            let scattered: Vec<BufferRef> = last
                .iter()
                .zip(lane_targets.iter())
                .map(|(producer, target)| program.ops[target.unwrap_or(*producer)].output)
                .collect();
            let distinct: std::collections::BTreeSet<_> = scattered.iter().collect();
            if distinct.len() != scattered.len() {
                continue;
            }
            for (producer, target) in last.iter().zip(lane_targets) {
                if let Some(consumer) = target {
                    redirects.insert(*producer, consumer);
                }
            }
        }
        redirects
    }

    /// Interpret the program the way the executor runs it -- banks hoisted -- and compare against
    /// the semantic graph, node by node.
    ///
    /// This is [`assert_program_matches_spec`]'s check moved from schedule order to *unit* order,
    /// which is the order that actually renders. It models the two things schedule-order
    /// interpretation cannot see: a bank gathers every member before its kernel runs, and a
    /// delayed input is physically written into its staging slot.
    fn divergence_in_runtime_order(
        spec: &GraphSpec,
        schedule: &[GraphNodeId],
        delays: &[InsertedDelay],
        program: &ExecutionProgram,
        lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
    ) -> Option<String> {
        let expected = evaluate_spec(spec, schedule, delays);
        let mut arena = vec![Expr::Silence; program.buffers as usize];
        // One op's reduction: stage every delayed input into its scratch, then combine.
        let gather = |arena: &mut Vec<Expr>, op: &Op| {
            let mut operands = Vec::new();
            for input in program.inputs_of(op) {
                let value = arena[input.buffer.0 as usize].clone();
                match input.delay {
                    Some(delay) => {
                        let staged = Expr::Delayed(
                            Box::new(value),
                            program.delays[delay.line as usize].samples,
                        );
                        arena[delay.staging.0 as usize] = staged.clone();
                        operands.push(staged);
                    }
                    None => operands.push(value),
                }
            }
            match operands.len() {
                0 => Expr::Silence,
                1 => operands.remove(0),
                _ => Expr::Sum(operands),
            }
        };
        let transform = |op: &Op, gathered: Expr| {
            if is_alias_candidate(&spec.nodes[op.node as usize].id) {
                gathered
            } else {
                Expr::Node(op.node, Box::new(gathered))
            }
        };
        let runs = runs_in_runtime_order(program, lanes);
        let redirect_of_lane = redirects_in_runtime_order(program, lanes, &runs);
        for run in runs {
            let first = &run[0];
            // A bank gathers *every* member before the kernel touches any of them, so a member's
            // storage must survive every other member's gather.
            let banked = first.len() > 1 || lanes.contains_key(&program.ops[first[0]].node);
            if !banked {
                let op = &program.ops[first[0]];
                // A redirected consumer does not reduce: the chain already scattered its input
                // into this very buffer, which is what `reduce_plane` does for a lone input that
                // is its own output.
                let gathered = if redirect_of_lane.values().any(|op| *op == first[0]) {
                    arena[op.output.0 as usize].clone()
                } else {
                    gather(&mut arena, op)
                };
                arena[op.output.0 as usize] = transform(op, gathered);
                if arena[op.output.0 as usize] != expected[op.node as usize] {
                    return Some(format!(
                        "op {} (node {}) rendered a foreign value",
                        first[0], op.node
                    ));
                }
                continue;
            }
            // Gather: the first slot's lanes, in lane order. A lane whose whole reduction is the
            // dedication copy is not executed at all -- the gather reads its producer's buffer and
            // its own output buffer is left alone (`runtime::bank_gather_source`).
            let mut resident: Vec<Expr> = Vec::with_capacity(first.len());
            for index in first {
                let op = &program.ops[*index];
                match gather_source_model(program, op) {
                    Some(source) => resident.push(arena[source as usize].clone()),
                    None => {
                        let gathered = gather(&mut arena, op);
                        arena[op.output.0 as usize] = gathered.clone();
                        resident.push(gathered);
                    }
                }
            }
            // Every slot of the chain runs over the resident block, in cascade order. A later
            // slot's op is never executed: its value is passed between slots, not through the
            // arena, so its input buffer is never read and only the *last* slot's output buffer is
            // written by the scatter.
            for slot in &run {
                for (lane, index) in slot.iter().enumerate() {
                    let op = &program.ops[*index];
                    resident[lane] = transform(op, resident[lane].clone());
                }
            }
            let last = run.last().expect("a run has at least one slot");
            for (lane, index) in last.iter().enumerate() {
                let op = &program.ops[*index];
                if resident[lane] != expected[op.node as usize] {
                    return Some(format!(
                        "op {index} (node {}) rendered a foreign value",
                        op.node
                    ));
                }
                // The scatter lands in the consumer's buffer when this lane was redirected, and
                // the last slot's own buffer is then never written at all.
                let target = redirect_of_lane
                    .get(index)
                    .map_or(op.output, |consumer| program.ops[*consumer].output);
                arena[target.0 as usize] = resident[lane].clone();
            }
        }
        let output_node = spec
            .nodes
            .iter()
            .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
            .expect("output");
        (arena[program.output.0 as usize] != expected[output_node])
            .then(|| "the session output rendered a foreign value".to_owned())
    }

    /// Buffers one op writes (its output, plus every staging slot it fills) and reads.
    fn touched(program: &ExecutionProgram, op: &Op) -> (Vec<u32>, Vec<u32>) {
        let mut writes = vec![op.output.0];
        let mut reads = Vec::new();
        for input in program.inputs_of(op) {
            reads.push(input.buffer.0);
            if let Some(delay) = input.delay {
                writes.push(delay.staging.0);
            }
        }
        if let Some(side) = op.sidechain {
            reads.push(side.buffer.0);
            if let Some(delay) = side.delay {
                writes.push(delay.staging.0);
            }
        }
        (writes, reads)
    }

    /// The #169 invariant, checked structurally: **no physical slot is recycled inside a bank
    /// window**.
    ///
    /// A window is the op range from a bank's first member to its last. Execution over that range
    /// is a permutation of the schedule -- members hoisted forward, non-members deferred -- so
    /// colouring is sound there only if no op in the window writes storage another op in the
    /// window reads or writes. An op writing its own input in place is the one exception, because
    /// that is one op, not two.
    fn assert_no_slot_is_recycled_inside_a_bank_window(
        program: &ExecutionProgram,
        spec: &GraphSpec,
        banks: &[Vec<GraphNodeId>],
        label: &str,
    ) {
        for members in banks {
            let positions: Vec<usize> = members
                .iter()
                .filter_map(|id| node_index(spec, id))
                .filter_map(|index| program.node_op[index as usize])
                .map(|op| op as usize)
                .collect();
            let (Some(first), Some(last)) = (positions.iter().min(), positions.iter().max()) else {
                continue;
            };
            for here in *first..=*last {
                let (writes, _) = touched(program, &program.ops[here]);
                for there in *first..=*last {
                    if there == here {
                        continue;
                    }
                    let (other_writes, other_reads) = touched(program, &program.ops[there]);
                    for slot in &writes {
                        assert!(
                            !other_writes.contains(slot),
                            "{label}: ops {here} and {there} both write buffer {slot} inside a \
                             bank window"
                        );
                        assert!(
                            !other_reads.contains(slot),
                            "{label}: op {here} writes buffer {slot}, which op {there} reads \
                             inside the same bank window"
                        );
                    }
                }
            }
        }
    }

    /// Issue #169, the minimal reproduction: a slot released inside a bank window was handed
    /// straight back to a member of that same bank.
    ///
    /// Three tracks, one dynamic effect each, and a bank over the *outer* two -- the shape #166
    /// made reachable, because a dynamic rack banks by cohort signature, so the tracks that share
    /// a compressor bank need not be adjacent. Track `t01`'s effect is not in the bank and sits
    /// between the two members in the schedule, so the executor defers it past the bank; its
    /// delayed input needs a staging slot, and colouring released that slot one op before `t02`'s
    /// member allocated its output.
    ///
    /// In schedule order that is correct: the staging slot is dead the moment `t01`'s op finishes.
    /// In *execution* order the bank runs first, `t02`'s member writes the slot, and then the
    /// deferred `t01` stages over the top of it -- so `t02` renders `t01`'s delayed input.
    ///
    /// The second half of the test is what keeps the first half honest: lowered with no bank
    /// declared -- which is exactly what the pre-#169 signature could express -- the same graph
    /// really does hand the slot over.
    #[test]
    fn a_bank_window_never_recycles_a_physical_slot() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for (index, track) in ["t00", "t01", "t02"].iter().enumerate() {
            let (track_nodes, track_edges) = dynamic_track(track, &format!("r{index:02}"));
            nodes.extend(track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || index == 0
            }));
            edges.extend(track_edges);
        }
        let (spec, schedule, levels) = build(nodes, edges);
        // The one PDC edge: into the effect of the track the bank does *not* contain.
        let delays = vec![InsertedDelay {
            node: dynamic_effect("t01"),
            edge_id: GraphEdgeId::TrackMain {
                target: dynamic_effect("t01"),
            },
            samples: LatencySamples(64),
        }];
        let banks = vec![vec![dynamic_effect("t00"), dynamic_effect("t02")]];
        let lanes = member_lanes(&spec, &banks);

        // Not vacuous: told nothing about the bank, colouring recycles the slot and the render
        // diverges. This is the pre-#169 behaviour, and the fixture exists to reach it.
        let unaware = lower(&spec, &schedule, &levels, &delays, &[]).expect("lowers");
        assert_program_matches_spec(&spec, &schedule, &delays, &unaware);
        let member = node_index(&spec, &dynamic_effect("t02")).expect("member");
        let outsider = node_index(&spec, &dynamic_effect("t01")).expect("outsider");
        let outsider_op = &unaware.ops[unaware.node_op[outsider as usize].expect("op") as usize];
        let staging = unaware.inputs_of(outsider_op)[0]
            .delay
            .expect("the outsider's input is delayed")
            .staging;
        assert_eq!(
            unaware.node_buffer[member as usize], staging,
            "the fixture must reach the collision it is here to pin"
        );
        assert!(
            divergence_in_runtime_order(&spec, &schedule, &delays, &unaware, &lanes).is_some(),
            "and that collision must actually change what renders"
        );

        // Told about the bank, the slot is held until the window closes.
        let program = lower(&spec, &schedule, &levels, &delays, &banks).expect("lowers");
        assert_program_matches_spec(&spec, &schedule, &delays, &program);
        assert_no_slot_is_recycled_inside_a_bank_window(&program, &spec, &banks, "minimal");
        assert_eq!(
            divergence_in_runtime_order(&spec, &schedule, &delays, &program, &lanes),
            None
        );
        // And it costs nothing here: holding a slot across the window changes *which* slot each
        // op gets, not how many exist. The window hold buys the collision away out of the
        // colouring's own slack -- which is why it, and not dedication, is the fix that shipped.
        assert_eq!((unaware.buffers, program.buffers), (7, 7));
    }

    /// Issue #169's structural half: **no physical slot is recycled inside a merged bank
    /// window**, which is the whole soundness condition (see [`lower`]).
    ///
    /// Four tracks and two interleaved cohorts -- the shape a dynamic rack produces when
    /// neighbouring tracks carry different chains, which is what #166 made reachable. Each bank's
    /// window contains the other's members, so the two merge into one span: what makes them
    /// unsafe is not either bank alone but that each reorders the other's ops.
    ///
    /// The overlap assertion is the non-vacuity guard. If the fixture ever stopped interleaving,
    /// the windows would stop merging and this would pass on a graph that no longer poses the
    /// question.
    #[test]
    fn no_slot_is_recycled_inside_a_merged_bank_window() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for (index, track) in ["t00", "t01", "t02", "t03"].iter().enumerate() {
            let (track_nodes, track_edges) = dynamic_track(track, &format!("r{index:02}"));
            nodes.extend(track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || index == 0
            }));
            edges.extend(track_edges);
        }
        let (spec, schedule, levels) = build(nodes, edges);
        let banks = vec![
            vec![dynamic_effect("t00"), dynamic_effect("t02")],
            vec![dynamic_effect("t01"), dynamic_effect("t03")],
        ];
        let program = lower(&spec, &schedule, &levels, &[], &banks).expect("lowers");
        assert_program_matches_spec(&spec, &schedule, &[], &program);

        // The two windows really do interleave, so `bank_windows` really does merge them.
        let window = |members: &[GraphNodeId]| {
            let ops: Vec<usize> = members
                .iter()
                .map(|id| {
                    let index = node_index(&spec, id).expect("member is a node");
                    program.node_op[index as usize].expect("a member is never elided") as usize
                })
                .collect();
            (
                *ops.iter().min().expect("first"),
                *ops.iter().max().expect("last"),
            )
        };
        let (first_start, first_end) = window(&banks[0]);
        let (second_start, second_end) = window(&banks[1]);
        assert!(
            first_start < second_start && second_start < first_end && first_end < second_end,
            "the cohorts must interleave: {first_start}..{first_end} and \
             {second_start}..{second_end}"
        );

        assert_no_slot_is_recycled_inside_a_bank_window(&program, &spec, &banks, "merged window");
        let lanes = member_lanes(&spec, &banks);
        assert_eq!(
            divergence_in_runtime_order(&spec, &schedule, &[], &program, &lanes),
            None
        );
    }

    /// Issue #169, the load-bearing eval: over seeded multi-track graphs with dynamic rack chains,
    /// interleaved cohorts, fan-out and PDC, the program renders the value the semantic graph does
    /// **in the order the executor actually runs it**, with banks hoisted to their first member.
    ///
    /// `lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs` checks the same thing
    /// in schedule order, which is not the order that renders once a bank exists. The `unaware`
    /// arm counts how many of these graphs the pre-#169 lowering got wrong, so the corpus cannot
    /// quietly stop constructing the hazard.
    #[test]
    fn bank_window_hoisting_preserves_dataflow_on_random_graphs() {
        let mut state = 0x1234_5678_9abc_def1_u64;
        let mut banked_graphs = 0usize;
        let mut unaware_divergences = 0usize;
        for graph in 0..4000_u32 {
            let track_count = (xorshift(&mut state) % 5) as usize + 2;
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            let mut cohort_of_track: Vec<usize> = Vec::new();
            let mut route_index = 0usize;
            for track in 0..track_count {
                let name = format!("t{track:02}");
                let routes: Vec<String> = (0..(xorshift(&mut state) % 2) + 1)
                    .map(|_| {
                        route_index += 1;
                        format!("r{route_index:02}")
                    })
                    .collect();
                let borrowed: Vec<&str> = routes.iter().map(String::as_str).collect();
                let (mut track_nodes, mut track_edges) = plain_track(&name, &borrowed);
                // A dynamic rack chain of one or two slots, and the cohort it belongs to.
                let slots = (xorshift(&mut state) % 2) as usize + 1;
                cohort_of_track.push((xorshift(&mut state) % 2) as usize);
                track_edges.retain(|edge| {
                    edge.id
                        != GraphEdgeId::TrackMain {
                            target: stage_node(&name, TrackStage::PostDynamic),
                        }
                });
                let mut upstream = stage_node(&name, TrackStage::PostSimd1);
                for slot in 0..slots {
                    let effect = GraphNodeId::Effect(crate::EffectNodeId {
                        track_id: gid(&name),
                        rack: RackId::Dynamic,
                        effect_id: gid(&format!("fx{slot}")),
                    });
                    track_nodes.push(node(effect.clone()));
                    track_edges.push(main_edge(
                        GraphEdgeId::TrackMain {
                            target: effect.clone(),
                        },
                        upstream,
                        effect.clone(),
                    ));
                    upstream = effect;
                }
                track_edges.push(main_edge(
                    GraphEdgeId::TrackMain {
                        target: stage_node(&name, TrackStage::PostDynamic),
                    },
                    upstream,
                    stage_node(&name, TrackStage::PostDynamic),
                ));
                nodes.extend(track_nodes.into_iter().filter(|candidate| {
                    !matches!(candidate.id, GraphNodeId::Output { .. }) || track == 0
                }));
                edges.extend(track_edges);
            }
            let (spec, schedule, levels) = build(nodes, edges);
            let mut delays: Vec<InsertedDelay> = Vec::new();
            for edge in &spec.edges {
                if !xorshift(&mut state).is_multiple_of(6) {
                    continue;
                }
                delays.push(InsertedDelay {
                    node: edge.destination.node.clone(),
                    edge_id: edge.id.clone(),
                    samples: LatencySamples(xorshift(&mut state) % 128 + 1),
                });
            }
            // Banks: dynamic effects, bucketed by (level, cohort), which is what the rack
            // compiler's cohort planner produces. Cohorts interleave by track, so one bank's
            // window contains the other's members.
            let mut buckets: std::collections::BTreeMap<(u64, usize), Vec<GraphNodeId>> =
                std::collections::BTreeMap::new();
            for level in &levels {
                for id in &level.nodes {
                    let GraphNodeId::Effect(effect) = id else {
                        continue;
                    };
                    let track: usize = effect.track_id.as_str()[1..].parse().expect("track");
                    buckets
                        .entry((level.level, cohort_of_track[track]))
                        .or_default()
                        .push(id.clone());
                }
            }
            let banks: Vec<Vec<GraphNodeId>> = buckets
                .into_values()
                .filter(|members| members.len() > 1)
                .collect();
            if banks.is_empty() {
                continue;
            }
            banked_graphs += 1;

            let program = lower(&spec, &schedule, &levels, &delays, &banks).expect("lowers");
            let lanes = member_lanes(&spec, &banks);
            assert_program_matches_spec(&spec, &schedule, &delays, &program);
            assert_no_slot_is_recycled_inside_a_bank_window(&program, &spec, &banks, "random");
            assert_eq!(
                divergence_in_runtime_order(&spec, &schedule, &delays, &program, &lanes),
                None,
                "graph {graph}"
            );

            let unaware = lower(&spec, &schedule, &levels, &delays, &[]).expect("lowers");
            if divergence_in_runtime_order(&spec, &schedule, &delays, &unaware, &lanes).is_some() {
                unaware_divergences += 1;
            }
        }
        // Both numbers are pinned, not bounded, because `lower`'s documentation quotes them: the
        // corpus is seeded and deterministic, so a change here means the corpus moved and the
        // claims that rest on it need re-measuring rather than re-pinning. The second number is
        // also the non-vacuity guard -- without it the arm above could pass on a corpus that no
        // longer constructs the hazard at all.
        assert_eq!(banked_graphs, 3617, "the banked corpus moved");
        assert_eq!(
            unaware_divergences, 285,
            "the number of graphs reaching the pre-#169 defect moved"
        );
    }

    /// Issue #202 rec 2, the load-bearing eval: over seeded multi-track graphs whose cohorts form
    /// **multi-slot chains**, the program renders the value the semantic graph does in the order
    /// the executor actually runs it -- chains merged, later slots never executed, and first-slot
    /// gathers redirected past their dedication copy.
    ///
    /// [`bank_window_hoisting_preserves_dataflow_on_random_graphs`] is the same eval for #169's
    /// per-bank hoisting, and its corpus gives each *track* an independent slot count, so a
    /// cohort's two levels rarely hold the same lane set and almost nothing merges. This corpus
    /// fixes the slot count per **cohort**, which is what a rack chain actually looks like: every
    /// lane of a cohort runs the same program, so slot `k` and slot `k + 1` cover the same lanes in
    /// the same order and `runtime::chains_into` can prove the fusion.
    ///
    /// Three arms, and the second and third are what make the first mean something:
    ///
    /// * **Merged.** Lowered with the bank member lists production passes, so
    ///   [`chainable_bank_groups`] unions the fusible pairs and the window spans the whole chain.
    ///   No graph may diverge.
    /// * **Narrow window.** The same lowering with [`chainable_bank_groups`] switched off
    ///   ([`lower_with_per_bank_windows`]), so every bank holds only its own span. That is
    ///   precisely the window set a merge-unaware lowering would hold, and the count of graphs it
    ///   gets wrong is the measurement of what the union buys.
    /// * **No window at all.** The pre-#169 arm, kept so the corpus cannot quietly stop
    ///   constructing a hazard at all.
    ///
    /// Every count is pinned rather than bounded, for the reason the #169 eval gives: the corpus is
    /// seeded and deterministic, so a change means the corpus moved and the claims resting on it
    /// need re-measuring rather than re-pinning.
    #[test]
    fn cohort_chain_merging_preserves_dataflow_on_random_graphs() {
        let mut state = 0x0fed_cba9_8765_4321_u64;
        let mut chained_graphs = 0usize;
        let mut merged_runs = 0usize;
        let mut redirected_lanes = 0usize;
        let mut narrow_divergences = 0usize;
        let mut unaware_divergences = 0usize;
        for graph in 0..4000_u32 {
            let track_count = (xorshift(&mut state) % 11) as usize + 2;
            // Four cohorts, each with its own chain length: every lane of a cohort runs the same
            // program, which is what makes slot `k` and slot `k + 1` the same lane set. Four
            // rather than two because a merged run hoists its whole chain past *every* other
            // cohort's ops at the same level, so the pressure on the colouring grows with the
            // cohort count -- the standing console fixture runs eight.
            let cohort_slots = [
                (xorshift(&mut state) % 3) as usize + 1,
                (xorshift(&mut state) % 3) as usize + 1,
                (xorshift(&mut state) % 3) as usize + 1,
                (xorshift(&mut state) % 3) as usize + 1,
            ];
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            let mut cohort_of_track: Vec<usize> = Vec::new();
            let mut route_index = 0usize;
            for track in 0..track_count {
                let name = format!("t{track:02}");
                let routes: Vec<String> = (0..(xorshift(&mut state) % 2) + 1)
                    .map(|_| {
                        route_index += 1;
                        format!("r{route_index:02}")
                    })
                    .collect();
                let borrowed: Vec<&str> = routes.iter().map(String::as_str).collect();
                let (mut track_nodes, mut track_edges) = plain_track(&name, &borrowed);
                let cohort = (xorshift(&mut state) % 4) as usize;
                cohort_of_track.push(cohort);
                track_edges.retain(|edge| {
                    edge.id
                        != GraphEdgeId::TrackMain {
                            target: stage_node(&name, TrackStage::PostDynamic),
                        }
                });
                let mut upstream = stage_node(&name, TrackStage::PostSimd1);
                // One cohort's members are dedicated storage and one's are not, which is the
                // difference that decides whether a member folds into its producer in place.
                // A member that folds allocates nothing, so a corpus of only those never reaches
                // the recycling hazard a merged window exists to hold off.
                let rack = if cohort.is_multiple_of(2) {
                    RackId::Simd1
                } else {
                    RackId::Dynamic
                };
                for slot in 0..cohort_slots[cohort] {
                    let effect = GraphNodeId::Effect(crate::EffectNodeId {
                        track_id: gid(&name),
                        rack,
                        effect_id: gid(&format!("fx{slot}")),
                    });
                    track_nodes.push(node(effect.clone()));
                    track_edges.push(main_edge(
                        GraphEdgeId::TrackMain {
                            target: effect.clone(),
                        },
                        upstream,
                        effect.clone(),
                    ));
                    upstream = effect;
                }
                track_edges.push(main_edge(
                    GraphEdgeId::TrackMain {
                        target: stage_node(&name, TrackStage::PostDynamic),
                    },
                    upstream,
                    stage_node(&name, TrackStage::PostDynamic),
                ));
                nodes.extend(track_nodes.into_iter().filter(|candidate| {
                    !matches!(candidate.id, GraphNodeId::Output { .. }) || track == 0
                }));
                edges.extend(track_edges);
            }
            let (spec, schedule, levels) = build(nodes, edges);
            let mut delays: Vec<InsertedDelay> = Vec::new();
            for edge in &spec.edges {
                if !xorshift(&mut state).is_multiple_of(6) {
                    continue;
                }
                delays.push(InsertedDelay {
                    node: edge.destination.node.clone(),
                    edge_id: edge.id.clone(),
                    samples: LatencySamples(xorshift(&mut state) % 128 + 1),
                });
            }
            let mut buckets: std::collections::BTreeMap<(u64, usize), Vec<GraphNodeId>> =
                std::collections::BTreeMap::new();
            for level in &levels {
                for id in &level.nodes {
                    let GraphNodeId::Effect(effect) = id else {
                        continue;
                    };
                    let track: usize = effect.track_id.as_str()[1..].parse().expect("track");
                    buckets
                        .entry((level.level, cohort_of_track[track]))
                        .or_default()
                        .push(id.clone());
                }
            }
            let banks: Vec<Vec<GraphNodeId>> = buckets
                .into_values()
                .filter(|members| members.len() > 1)
                .collect();
            if banks.is_empty() {
                continue;
            }
            chained_graphs += 1;

            let program = lower(&spec, &schedule, &levels, &delays, &banks).expect("lowers");
            let lanes = member_lanes(&spec, &banks);
            assert_program_matches_spec(&spec, &schedule, &delays, &program);
            assert_eq!(
                divergence_in_runtime_order(&spec, &schedule, &delays, &program, &lanes),
                None,
                "graph {graph}"
            );
            let realised = runs_in_runtime_order(&program, &lanes);
            merged_runs += realised.iter().filter(|run| run.len() > 1).count();
            let modelled = redirects_in_runtime_order(&program, &lanes, &realised);
            redirected_lanes += modelled.len();
            // The model is an oracle only while it and the runtime agree, and the model cannot
            // check that by itself. Issue #202's adversarial verification found the gap: shortening
            // `runtime::scatter_target`'s in-between scan by one op -- the unsound direction --
            // reddened nothing, while the same one-token change to `scatter_target_model` reddened
            // this corpus at graph 0. The corpus was building the hazard and then only ever asking
            // the model about it. So the corpus now drives the runtime's own clauses too, and every
            // clause it exercises has this eval as its red test on both sides of the pair.
            assert_eq!(
                crate::runtime::scatter_redirects_over_program(&program, &spec, &lanes, &realised),
                modelled,
                "graph {graph}: the runtime and the model disagree about which lanes redirect"
            );

            // The narrow-window arm: every bank's own span, and no union across banks.
            let narrow = lower_with_per_bank_windows(&spec, &schedule, &levels, &delays, &banks)
                .expect("lowers");
            if divergence_in_runtime_order(&spec, &schedule, &delays, &narrow, &lanes).is_some() {
                narrow_divergences += 1;
            }

            let unaware = lower(&spec, &schedule, &levels, &delays, &[]).expect("lowers");
            if divergence_in_runtime_order(&spec, &schedule, &delays, &unaware, &lanes).is_some() {
                unaware_divergences += 1;
            }
        }
        assert_eq!(chained_graphs, 3563, "the chained corpus moved");
        assert_eq!(
            merged_runs, 3752,
            "the number of realised multi-slot chains moved: if this ever falls to zero the arm \
             above is passing on a corpus where nothing merges"
        );
        assert_eq!(
            redirected_lanes, 1669,
            "the number of lanes whose scatter is redirected into their consumer moved (#202 \
             rec 3): a zero here would mean this eval never interprets one"
        );
        assert_eq!(
            narrow_divergences, 885,
            "the number of graphs a per-bank window gets wrong moved -- this is the measurement \
             of what the cohort-chain window union buys, and a zero here would mean the union is \
             held for nothing"
        );
        assert_eq!(
            unaware_divergences, 1665,
            "the number of graphs reaching the pre-#169 defect moved"
        );
    }
}
